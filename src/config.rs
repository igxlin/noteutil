use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

#[derive(serde::Deserialize, Debug)]
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

#[derive(serde::Deserialize, Debug)]
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

#[derive(serde::Deserialize, Debug)]
#[serde(default)]
pub struct Config {
    pub root_dir: PathBuf,
    pub journal: Journal,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            root_dir: PathBuf::from("."),
            journal: Journal::default(),
        }
    }
}

impl Config {
    pub fn from_file(path: &Path) -> Result<Self, Box<dyn Error>> {
        let s = std::fs::read_to_string(&path)?;
        Self::from_str(&s)
    }

    pub fn from_str(s: &str) -> Result<Self, Box<dyn Error>> {
        let config: Config = toml::from_str(&s)?;
        Ok(config)
    }

    pub fn from_default_locations() -> Result<Self, Box<dyn Error>> {
        // TODO: Support other platforms
        let user_home = std::env::var("HOME")?;
        let path = Path::new(&user_home).join(".config/noteutil/config.toml");
        if path.exists() {
            return Self::from_file(&path);
        }

        Ok(Self::default())
    }
}
