use anyhow::{Context, anyhow};
use comrak::{markdown_to_html, Options};
use frontmatter::Frontmatter;
use handlebars::Handlebars;
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use nu_ansi_term::Color;
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    net::SocketAddr,
    path::{Component, Path, PathBuf},
};
use thiserror::Error;

use std::time::Instant;
use tower_http::{services::ServeDir, trace::TraceLayer};
mod frontmatter;

pub fn is_empty<P: AsRef<Path>>(p: P) -> bool {
    let is_empty = match p.as_ref().read_dir() {
        Ok(iter) => {
            let mut dir_iter = iter;
            dir_iter.next().is_none()
        }
        Err(e) => {
            eprintln!("{e}");
            false
        }
    };

    is_empty
}

pub fn cwd() -> PathBuf {
    let cwd = env::current_dir().unwrap_or(PathBuf::from("."));
    cwd
}

/// Maps a file under `source_root` onto the matching location under `target_root`.
///
/// This is deliberately component-based. The previous string `replacen` left the
/// path untouched whenever the prefix didn't match, which silently wrote build
/// output back into the source tree, and it carried `..` segments straight
/// through into the target. Both are rejected here instead.
fn destination_for(source_root: &Path, target_root: &Path, path: &Path) -> anyhow::Result<PathBuf> {
    let outside = || anyhow!(WingmanError::OutsideSourceTree(path.to_path_buf()));

    let relative = path.strip_prefix(source_root).map_err(|_| outside())?;

    if relative.components().any(|c| c == Component::ParentDir) {
        return Err(outside());
    }

    Ok(target_root.join(relative))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {}

impl Settings {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct Wingman<'a> {
    sourcecode: PathBuf,
    target: PathBuf,
    pub settings: Settings,
    router: axum::Router,
    engine: Handlebars<'a>,
}

impl Default for Wingman<'_> {
    fn default() -> Self {
        let cwd = crate::cwd();
        let srcpth = Path::new("www");
        let source_path = &cwd.join(srcpth);
        let out_path = &cwd.join("_site");

        let router: axum::Router<()> =
            axum::Router::new().nest_service("/", ServeDir::new(&out_path));

        // We could make these optional. Just warn users or something.

        // I hate having to clone shit.
        // Makes sense though since the originals are about to drop.
        Self {
            sourcecode: source_path.clone(),
            target: out_path.clone(),
            settings: Settings::new(),
            router,
            engine: Handlebars::new(),
        }
    }
}

impl Wingman<'_> {
    /// Starts development webserver for Wingman project.
    pub async fn serve(self, port: &u16) -> anyhow::Result<()> {
        if !self.target.exists() {
            return Err(
                anyhow!(WingmanError::InputNotExist(self.target.to_path_buf())).context(format!(
                    "Cannot serve nonexistant directory. ({})",
                    self.target.display()
                )),
            );
        }

        let addr = SocketAddr::from(([127, 0, 0, 1], *port));
        let listener = tokio::net::TcpListener::bind(addr).await?;
        println!("Serving on http://localhost:{}", port);
        axum::serve(listener, self.router.layer(TraceLayer::new_for_http())).await?;
        Ok(())
    }

    pub fn init(&mut self, force: bool) -> anyhow::Result<()> {
        let cwd = crate::cwd();
        if !is_empty(&cwd) && !force {
            return Err(anyhow!("Dir is full, and no --force flag passed"));
        }

        self.create_project_structure()?;
        self.reload_engine()?;
        Ok(())
    }

    fn create_project_structure(&self) -> anyhow::Result<()> {
        let cwd = crate::cwd();

        fs::create_dir_all(&cwd.join(&self.sourcecode).join("static"))?;
        fs::create_dir_all(&cwd.join("templates").join("partials"))?;
        fs::create_dir_all(&cwd.join(&self.target).join("static"))?;

        let page_tmpl = include_str!("../templates/page.hbs");
        let nav_partial = include_str!("../templates/partials/nav.hbs");
        let page_css = include_str!("../templates/static/page.css");

        fs::write(&cwd.join("templates/page.hbs"), page_tmpl)?;
        fs::write(&cwd.join("templates/partials/nav.hbs"), nav_partial)?;
        fs::write(&cwd.join("www/static/page.css"), page_css)?;
        let index_md = Frontmatter::default().meta;
        let index_yml = serde_yaml::to_string(&index_md)?;

        fs::write(
            &cwd.join("www/index.md"),
            format!("---\n\n{index_yml}\n\n---"),
        )?;
        Ok(())
    }

    pub async fn build(&mut self, watch: bool) -> anyhow::Result<()> {
        if !&self.sourcecode.exists() || !self.target.exists() {
            return Err(anyhow!("Directories ./www and ./_site weren't found."));
        }

        if watch {
            self.reload_engine()?;
            println!("Watching ./www for changes");
            let (tx, rx) = std::sync::mpsc::channel();
            let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

            watcher.watch(&self.sourcecode, RecursiveMode::Recursive)?;

            for res in rx {
                match res {
                    Ok(event) => match event.kind {
                        // notify::EventKind::Any => todo!(),
                        notify::EventKind::Access(_)
                        | notify::EventKind::Create(_)
                        | notify::EventKind::Modify(_) => {
                            for path in event.paths {
                                // println!("Rendering {}", &path.display());
                                if let Err(e) = &self.render_file(path).await {
                                    match e.downcast_ref::<WingmanError>() {
                                        // This might not work? When I run tests, it prints regardless.
                                        Some(WingmanError::InputNotExist(_))
                                        | Some(WingmanError::InputNotFile(_))
                                        | Some(WingmanError::OutsideSourceTree(_)) => continue,
                                        _ => eprintln!(
                                            "{:#?}: {}",
                                            &event.kind,
                                            Color::Red.paint(e.to_string())
                                        ),
                                    }
                                }
                                // A bad template shouldn't kill a long-running watcher.
                                if let Err(e) = self.reload_engine() {
                                    eprintln!("{}", Color::Red.paint(format!("{e:#}")));
                                }
                            }
                        }
                        // TODO: remove from production when dev is deleted, and maybe remove trace dependencies?
                        notify::EventKind::Remove(_) => {}
                        // notify::EventKind::Other => todo!(),
                        _ => {}
                    },
                    Err(e) => eprintln!("watch error: {:?}", e),
                }
            }
        } else {
            self.reload_engine()?;
            let start = Instant::now();
            let mut handles = vec![];
            for entry in walkdir::WalkDir::new(&self.sourcecode) {
                let entry = entry?;
                let e_path = entry.path().to_path_buf();
                if entry.path().is_file() {
                    let handle = self.render_file(e_path);
                    handles.push(handle);
                    //     if let Err(e) = self.render_file(entry.path()).await {
                    //         eprintln!("{e}");
                    //     }
                }
            }

            let results = futures::future::join_all(handles).await;
            let count = results.len();

            // `results` is now a vector of the results of each future.
            // You can iterate over it and handle each result as needed.
            for result in results {
                match result {
                    Ok(_) => {}
                    Err(e) => eprintln!("{e}"),
                    // match e.downcast_ref::<WingmanError>() {
                    //     // This might not work? When I run tests, it prints regardless.
                    //     Some(WingmanError::InputNotExist(_))
                    //     | Some(WingmanError::InputNotFile(_)) => continue,
                    //     _ => eprintln!("{}", Color::Red.paint(e.to_string())),
                    // },
                }
            }

            let elapsed = Instant::now().duration_since(start);
            println!(
                "Built {} files in {:?}",
                Color::Cyan.paint(count.to_string()),
                elapsed
            )
        }
        Ok(())
    }

    async fn render_file<P: AsRef<Path>>(&self, p: P) -> anyhow::Result<()> {
        if !p.as_ref().exists() {
            return Err(anyhow!(WingmanError::InputNotExist(PathBuf::from(
                p.as_ref().to_string_lossy().to_string()
            ))));
        } else if !p.as_ref().is_file() {
            return Err(anyhow!(WingmanError::InputNotFile(PathBuf::from(
                p.as_ref().to_string_lossy().to_string()
            ))));
        }

        let mut destination_pb = destination_for(&self.sourcecode, &self.target, p.as_ref())?;

        if p.as_ref().extension().is_some_and(|x| x == "md") {
            let file = fs::read_to_string(&p)?;
            let mut fm = Frontmatter::new(&file)?;

            let mut html_opts = Options::default();
            html_opts.extension.footnotes = true;
            html_opts.extension.strikethrough = true;
            html_opts.extension.multiline_block_quotes = true;
            let html = markdown_to_html(&fm.content, &html_opts);
            fm.content = html;

            let out = self.engine.render("page", &fm)?;

            if !destination_pb.set_extension("html") {
                let msg = format!(
                    "could not change {} extension to .html",
                    p.as_ref().display()
                );

                return Err(anyhow!(msg));
            }

            if let Some(parent) = destination_pb.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(&destination_pb, out)?;
            let mut style = Color::White.normal();
            style.background = Some(Color::Red);
            println!("{}: {}", style.paint(" HTML "), &destination_pb.display());
        } else if p.as_ref().extension().is_some_and(|x| x == "css") {
            let css: String = fs::read_to_string(&p)?;
            // Parse a style sheet from a string.
            // A malformed stylesheet is bad input, not a bug in Wingman: report it and
            // keep going instead of taking the whole build (or watcher) down with us.
            let mut stylesheet = match StyleSheet::parse(&css, ParserOptions::default()) {
                Ok(s) => s,
                Err(e) => {
                    return Err(anyhow!("failed to parse {}: {e}", p.as_ref().display()));
                }
            };

            // Minify the stylesheet.
            stylesheet.minify(MinifyOptions::default())?;

            // Serialize it to a string.
            let res = stylesheet.to_css(PrinterOptions {
                minify: true,
                ..Default::default()
            })?;

            if let Some(parent) = destination_pb.parent() {
                fs::create_dir_all(parent)?;
            }

            fs::write(&destination_pb, res.code)?;
            let mut style = Color::White.normal();
            style.background = Some(Color::Blue);

            println!("{}: {}", style.paint(" CSS "), &destination_pb.display());
        }

        Ok(())
    }

    fn reload_engine(&mut self) -> anyhow::Result<()> {
        // BUG: Just realized that if you add a new template or partial after starting the program, Wingman won't refresh
        // HBS and it'll have to be restarted.
        let target_dir = crate::cwd().join("templates");
        if target_dir.exists() {
            let mut paths: Vec<PathBuf> = vec![];

            for entry in walkdir::WalkDir::new(&target_dir)
                .into_iter()
                .filter_map(|e| e.ok())
            {
                // println!("{}", &entry.path().display());

                if entry
                    .path()
                    .extension()
                    .is_some_and(|x| x == "hbs" || x == "handlebars")
                {
                    paths.push(entry.path().to_path_buf())
                }
            }

            for entry in paths {
                let name = entry
                    .file_stem()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default();
                // A broken template is user input. Surface the parse error instead of
                // asserting on it -- the assert both aborted the process and threw away
                // the one thing that explains what went wrong.
                if entry.starts_with(&target_dir.join("partials")) {
                    let source = fs::read_to_string(&entry)
                        .with_context(|| format!("failed to load partial {name}"))?;
                    self.engine
                        .register_partial(name, source)
                        .with_context(|| format!("failed to register partial {name}"))?;
                } else {
                    let source = fs::read_to_string(&entry)
                        .with_context(|| format!("failed to load template {name}"))?;
                    self.engine
                        .register_template_string(name, source)
                        .with_context(|| format!("failed to register template {name}"))?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum WingmanError {
    #[error("input does not exist")]
    InputNotExist(PathBuf),

    #[error("input is not a file")]
    InputNotFile(PathBuf),

    #[error("input resolves outside the source directory")]
    OutsideSourceTree(PathBuf),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_source_paths_into_the_target_tree() {
        let dest = destination_for(
            Path::new("/site/www"),
            Path::new("/site/_site"),
            Path::new("/site/www/blog/post.md"),
        )
        .expect("path inside the source tree should map");

        assert_eq!(dest, PathBuf::from("/site/_site/blog/post.md"));
    }

    #[test]
    fn rejects_paths_outside_the_source_tree() {
        // Nothing under the source root, so there is no output location for it.
        // Previously this fell through and wrote back over the input path.
        let err = destination_for(
            Path::new("/site/www"),
            Path::new("/site/_site"),
            Path::new("/etc/passwd"),
        );

        assert!(err.is_err());
    }

    #[test]
    fn rejects_traversal_out_of_the_target_tree() {
        // Textually prefixed by the source root, but the `..` segments walk back
        // out again -- joining this onto the target would escape it entirely.
        let err = destination_for(
            Path::new("/site/www"),
            Path::new("/site/_site"),
            Path::new("/site/www/../../etc/cron.d/payload.md"),
        );

        assert!(err.is_err());
    }
}
