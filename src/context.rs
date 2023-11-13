#[derive(Clone)]
pub struct Context {
    pub config: crate::Config,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            config: crate::Config::default(),
        }
    }
}
