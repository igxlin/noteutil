pub mod journal;

mod note;
pub use note::Note;

mod context;
pub use context::Context;

mod config;
pub use config::Config;

pub mod date;

mod html;
pub mod http;

pub mod lsp;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
