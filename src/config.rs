use std::path::Path;

#[derive(serde::Deserialize)]
#[serde(default)]
pub struct Config {
    pub journal: Journal,
}

#[derive(serde::Deserialize)]
#[serde(default)]
pub struct Journal {
    pub path: JournalPath,
}

impl Default for Journal {
    fn default() -> Self {
        Self {
            path: JournalPath::default(),
        }
    }
}

#[derive(serde::Deserialize)]
#[serde(default)]
pub struct JournalPath {
    pub daily: String,
    pub weekly: String,
    pub monthly: String,
    pub yearly: String,
}

impl Default for JournalPath {
    fn default() -> Self {
        Self {
            daily: String::from("journals/%Y-%m-%d.md"),
            weekly: String::from("journals/%Y-w%U.md"),
            monthly: String::from("journals/%Y-%m.md"),
            yearly: String::from("journals/%Y.md"),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            journal: Journal::default(),
        }
    }
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Self, anyhow::Error> {
        let s = std::fs::read_to_string(&path)?;
        Self::from_str(&s)
    }

    pub fn from_str(s: &str) -> Result<Self, anyhow::Error> {
        let config: Config = toml::from_str(&s)?;
        Ok(config)
    }
}
