use ansi_term::Color;
use anyhow::anyhow;
use comrak::{markdown_to_html, Options};
use frontmatter::Frontmatter;
use handlebars::Handlebars;
use lightningcss::stylesheet::{MinifyOptions, ParserOptions, PrinterOptions, StyleSheet};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    net::SocketAddr,
    path::{Path, PathBuf},
};
use thiserror::Error;
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

#[derive(Serialize, Deserialize, Debug)]
pub struct Settings {}

impl Settings {
    fn new() -> Self {
        Self {}
    }
}

#[derive(Debug)]
pub struct Wingman {
    sourcecode: PathBuf,
    target: PathBuf,
    pub settings: Settings,
    router: axum::Router,
}

impl Default for Wingman {
    fn default() -> Self {
        let cwd = crate::cwd();
        let srcpth = Path::new("www");
        let source_path = &cwd.join(srcpth);
        let out_path = &cwd.join("_site");

        let router: axum::Router<()> =
            axum::Router::new().nest_service("/", ServeDir::new(&out_path));

        // I hate having to clone shit.
        // Makes sense though since the originals are about to drop.
        Self {
            sourcecode: source_path.clone(),
            target: out_path.clone(),
            settings: Settings::new(),
            router,
        }
    }
}

impl Wingman {
    /// Starts development webserver for Wingman project.
    pub async fn serve(self, port: &u16) -> anyhow::Result<()> {
        if !self.target.exists() {
            return Err(anyhow!(
                "Cannot serve nonexistant directory. ({})",
                self.target.display()
            ));
        }

        let addr = SocketAddr::from(([127, 0, 0, 1], *port));
        let listener = tokio::net::TcpListener::bind(addr).await?;
        println!("Serving on http://localhost:{}", port);
        axum::serve(listener, self.router.layer(TraceLayer::new_for_http())).await?;
        Ok(())
    }

    pub fn init(&self, force: bool) -> anyhow::Result<()> {
        let cwd = crate::cwd();
        if !is_empty(&cwd) && !force {
            return Err(anyhow!("Dir is full, and no --force flag passed"));
        }

        self.create_project_structure()?;
        Ok(())
    }

    fn create_project_structure(&self) -> anyhow::Result<()> {
        let cwd = crate::cwd();

        fs::create_dir_all(&cwd.join(&self.sourcecode).join("static"))?;
        fs::create_dir_all(&cwd.join("templates").join("partials"))?;
        fs::create_dir_all(&cwd.join(&self.target).join("static"))?;

        let page_tmpl = include_str!("../example/templates/page.hbs");
        let nav_partial = include_str!("../example/templates/partials/nav.hbs");
        let page_css = include_str!("../example/www/static/page.css");

        fs::write(&cwd.join("templates/page.hbs"), page_tmpl)?;
        fs::write(&cwd.join("templates/partials/nav.hbs"), nav_partial)?;
        fs::write(&cwd.join("www/static/page.css"), page_css)?;

        Ok(())
    }

    pub fn build(&self, watch: bool) -> anyhow::Result<()> {
        if watch {
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
                        | notify::EventKind::Modify(_)
                        | notify::EventKind::Remove(_) => {
                            for path in event.paths {
                                // println!("Rendering {}", &path.display());
                                if let Err(e) = Self::render_file(path) {
                                    match e.downcast_ref::<WingmanError>() {
                                        // This might not work? When I run tests, it prints regardless.
                                        Some(WingmanError::InputNotExist(_))
                                        | Some(WingmanError::InputNotFile(_)) => continue,
                                        _ => eprintln!("{}", Color::Red.paint(e.to_string())),
                                    }
                                }
                            }
                        }
                        // notify::EventKind::Other => todo!(),
                        _ => {}
                    },
                    Err(e) => eprintln!("watch error: {:?}", e),
                }
            }
        } else {
            for entry in walkdir::WalkDir::new(&self.sourcecode) {
                let entry = entry?;
                if entry.path().is_file() {
                    if let Err(e) = Self::render_file(entry.path()) {
                        eprintln!("{e}");
                    }
                }
            }
        }
        Ok(())
    }

    fn render_file<'i, P: AsRef<Path>>(p: P) -> anyhow::Result<()> {
        if !p.as_ref().exists() {
            return Err(anyhow!(WingmanError::InputNotExist(PathBuf::from(
                p.as_ref().to_string_lossy().to_string()
            ))));
        } else if !p.as_ref().is_file() {
            return Err(anyhow!(WingmanError::InputNotFile(PathBuf::from(
                p.as_ref().to_string_lossy().to_string()
            ))));
        }

        let cwd = crate::cwd();
        let www = &cwd.join("www");
        let site = &cwd.join("_site");

        let mut destination_pb = PathBuf::from(p.as_ref().to_string_lossy().to_string().replacen(
            &www.to_string_lossy().to_string(),
            &site.to_string_lossy().to_string(),
            1,
        ));

        if p.as_ref().extension().is_some_and(|x| x == "md") {
            let file = fs::read_to_string(&p)?;
            let mut fm = Frontmatter::new(&file)?;

            let mut html_opts = Options::default();
            html_opts.extension.footnotes = true;
            html_opts.extension.strikethrough = true;
            html_opts.extension.multiline_block_quotes = true;
            let html = markdown_to_html(&fm.content, &html_opts);
            fm.content = html;

            // create the handlebars registry
            let mut handlebars = Handlebars::new();

            // We could make these optional. Just warn users or something.
            assert!(handlebars
                .register_template_string("page", include_str!("../example/templates/page.hbs"))
                .is_ok());

            assert!(handlebars
                .register_partial("nav", include_str!("../example/templates/partials/nav.hbs"))
                .is_ok());

            let out = handlebars.render("page", &fm)?;

            if !destination_pb.set_extension("html") {
                let msg = format!(
                    "could not change {} extension to .html",
                    p.as_ref().display()
                );

                return Err(anyhow!(msg));
            }

            fs::write(&destination_pb, out)?;
            let mut style = Color::White.normal();
            style.background = Some(Color::Red);
            println!("{}: {}", style.paint(" HTML "), &destination_pb.display());
        } else if p.as_ref().extension().is_some_and(|x| x == "css") {
            let css: String = fs::read_to_string(&p)?;
            // Parse a style sheet from a string.
            let mut stylesheet = match StyleSheet::parse(&css, ParserOptions::default()) {
                Ok(s) => s,
                Err(e) => panic!("{e}"),
            };

            // Minify the stylesheet.
            stylesheet.minify(MinifyOptions::default())?;

            // Serialize it to a string.
            let res = stylesheet
                .to_css(PrinterOptions {
                    minify: true,
                    ..Default::default()
                })
                .unwrap();
            fs::write(&destination_pb, res.code)?;
            let mut style = Color::White.normal();
            style.background = Some(Color::Blue);

            println!("{}: {}", style.paint(" CSS "), &destination_pb.display());
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
}
