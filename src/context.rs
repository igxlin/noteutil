use std::path::PathBuf;

pub struct Context {
    pub config: crate::Config,
    pub root_dir: PathBuf,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            config: crate::Config::default(),
            root_dir: PathBuf::new(),
        }
    }
}
