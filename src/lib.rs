use anyhow::anyhow;
use comrak::{markdown_to_html, Options};
use frontmatter::Frontmatter;
use handlebars::Handlebars;
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    path::{Path, PathBuf},
};

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

#[derive(Serialize, Deserialize, Debug)]
pub struct Wingman {
    sourcecode: PathBuf,
    target: PathBuf,
    pub settings: Settings,
}

impl Default for Wingman {
    fn default() -> Self {
        let cwd = crate::cwd();
        let srcpth = Path::new("www");
        let source_path = &cwd.join(srcpth);
        let out_path = &cwd.join("_site");

        // I hate having to clone shit.
        // Makes sense though since the originals are about to drop.
        Self {
            sourcecode: source_path.clone(),
            target: out_path.clone(),
            settings: Settings::new(),
        }
    }
}

impl Wingman {
    pub fn init(&self, force: bool) -> anyhow::Result<()> {
        let cwd = crate::cwd();
        if !is_empty(&cwd) {
            if force {
                self.create_project_structure()?;
            }
            return Err(anyhow!("ErrNoForceDirFull"));
        }

        self.create_project_structure()?;
        Ok(())
    }

    fn create_project_structure(&self) -> anyhow::Result<()> {
        let cwd = crate::cwd();

        fs::create_dir_all(&cwd.join(&self.sourcecode))?;
        fs::create_dir_all(&cwd.join(&self.target))?;

        Ok(())
    }

    pub fn build(&self, watch: bool) -> anyhow::Result<()> {
        if watch {
            println!("Watch enabled");
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
                                println!("Rendering {}", &path.display());
                                if let Err(e) = Self::render_file(path) {
                                    eprintln!("{e}");
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
            todo!("Nothing to see here, yet!")
        }
        Ok(())
    }

    fn render_file<P: AsRef<Path>>(p: P) -> anyhow::Result<()> {
        if !p.as_ref().exists() {
            return Err(anyhow!("File {} doesn't exist", p.as_ref().display()));
        } else if !p.as_ref().is_file() {
            return Err(anyhow!("Path must be a file: {}", p.as_ref().display()));
        }

        let file = fs::read_to_string(&p)?;
        let mut fm = Frontmatter::new(&file)?;

        let html = markdown_to_html(&fm.body, &Options::default());
        fm.body = html;
        // create the handlebars registry
        let mut handlebars = Handlebars::new();

        // We could make these optional. Just warn users or something.
        assert!(handlebars
            .register_template_string("page", include_str!("../example/templates/page.hbs"))
            .is_ok());

        assert!(handlebars
            .register_partial("nav", include_str!("../example/templates/partials/nav.hbs"))
            .is_ok());

        // println!("{:#?}", fm);

        let out = handlebars.render("page", &fm)?;
        let cwd = crate::cwd();

        let www = &cwd.join("www");
        let site = &cwd.join("_site");

        let destination_pb = PathBuf::from(p.as_ref().to_string_lossy().to_string().replacen(
            &www.to_string_lossy().to_string(),
            &site.to_string_lossy().to_string(),
            1,
        ));

        fs::write(destination_pb, out)?;

        Ok(())
    }
}
