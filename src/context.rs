use cargo_metadata::{camino::Utf8PathBuf, Metadata};
use git_repository as git;
use git_repository::prelude::CacheAccessExt;

pub struct Context {
    pub root: Utf8PathBuf,
    pub meta: Metadata,
    pub git_easy: git::Easy,
    pub crate_names: Vec<String>,
}

impl Context {
    pub fn new(crate_names: Vec<String>) -> anyhow::Result<Self> {
        let meta = cargo_metadata::MetadataCommand::new().exec()?;
        let root = meta.workspace_root.clone();
        let repo = git::discover(&root)?;
        Ok(Context {
            root,
            git_easy: repo.into_easy().apply_environment()?,
            meta,
            crate_names: fill_in_root_crate_if_needed(crate_names)?,
        })
    }
}

fn fill_in_root_crate_if_needed(crate_names: Vec<String>) -> anyhow::Result<Vec<String>> {
    Ok(if crate_names.is_empty() {
        let current_dir = std::env::current_dir()?;
        let manifest = current_dir.join("Cargo.toml");
        let dir_name = current_dir
            .file_name()
            .expect("a valid directory with a name")
            .to_str()
            .expect("directory is UTF8 representable");
        let crate_name = if manifest.is_file() {
            let manifest = cargo_toml::Manifest::from_path(manifest)?;
            manifest.package.map_or(dir_name.to_owned(), |p| p.name)
        } else {
            dir_name.to_owned()
        };
        log::warn!(
            "Using '{}' as crate name as no one was provided. Specify one if this isn't correct",
            crate_name
        );
        vec![crate_name]
    } else {
        crate_names
    })
}
