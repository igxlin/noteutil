use std::error::Error;
use std::path::Path;
use std::path::PathBuf;

use markdown::mdast;

use crate::core::journal;

#[derive(Debug)]
pub struct Note {
    pub path: PathBuf,
    links: Vec<Link>,
}

#[derive(Debug, PartialEq)]
struct Link {
    title: Option<String>,
    url: String,
}

impl Note {
    pub fn link_to(&self, path: &Path) -> bool {
        for link in &self.links {
            if Path::new(link.url.as_str()) == path {
                return true;
            }
        }

        return false;
    }

    pub fn build(path: &Path) -> Result<Self, Box<dyn Error>> {
        let mut note = Self {
            path: PathBuf::from(path),
            links: Vec::new(),
        };
        note.parse(std::fs::read_to_string(path)?.as_str())?;

        Ok(note)
    }

    fn parse(&mut self, content: &str) -> Result<(), Box<dyn Error>> {
        let node = markdown::to_mdast(content, &markdown::ParseOptions::default())?;
        self.parse_node(&node);
        Ok(())
    }

    fn parse_link(&mut self, link: &mdast::Link) {
        let title = link.children.first().and_then(|node| match &node {
            mdast::Node::Text(text) => Some(text.value.clone()),
            _ => None,
        });

        let url = if !Self::is_filesystem_url(link.url.as_str()) {
            link.url.clone()
        } else {
            let dirpath = self.path.parent().unwrap();
            String::from(dirpath.join(link.url.as_str()).to_str().unwrap())
        };

        self.links.push(Link {
            title: title,
            url: url,
        });
    }

    fn is_filesystem_url(url: &str) -> bool {
        // TODO: maybe use regex to filter any $.*://
        !url.starts_with("https://") && !url.starts_with("http://")
    }

    fn parse_node(&mut self, node: &mdast::Node) {
        match node {
            mdast::Node::Link(link) => self.parse_link(link),
            _ => {}
        }

        if let Some(children) = node.children() {
            children.iter().for_each(|node| self.parse_node(&node));
        }
    }
}

pub struct Filter<'filter> {
    paths: Vec<PathBuf>,
    periods: &'filter Vec<journal::Period>,
}

impl<'filter> Filter<'filter> {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            periods: &journal::ALL_PERIODS,
        }
    }

    pub fn periods(mut self, periods: &'filter Vec<journal::Period>) -> Self {
        self.periods = periods;
        self
    }

    pub fn add(mut self, pathbuf: &PathBuf) -> Self {
        if !pathbuf.exists() {
            log::error!("Invalid path: {}", pathbuf.display());
        }

        if pathbuf.is_dir() {
            let paths = std::fs::read_dir(pathbuf).unwrap();
            for path in paths {
                if let Ok(entry) = path {
                    self = self.add(&entry.path());
                }
            }
        } else if pathbuf.is_file() {
            self.paths.push(pathbuf.clone());
        }

        self
    }

    pub fn journal_only(mut self) -> Self {
        let mut paths = Vec::new();
        for path in self.paths {
            if path.to_str().unwrap().contains("journals/") {
                paths.push(path);
            }
        }

        self.paths = paths;
        self
    }

    pub fn date(mut self, date: &chrono::NaiveDate) -> Self {
        let daily_pattern = date.format("%Y-%m-%d").to_string();
        let weekly_pattern = date.format("%Y-w%U").to_string();
        let monthly_pattern = date.format("%Y-%m").to_string();
        let yearly_pattern = date.format("%Y").to_string();

        let patterns: Vec<String> = self
            .periods
            .iter()
            .map(|period| match period {
                journal::Period::Daily => daily_pattern.clone(),
                journal::Period::Weekly => weekly_pattern.clone(),
                journal::Period::Monthly => monthly_pattern.clone(),
                journal::Period::Yearly => yearly_pattern.clone(),
            })
            .collect();

        let mut paths = Vec::new();
        for path in self.paths {
            let mut match_at_least_one = false;
            for pattern in &patterns {
                if path.to_str().unwrap().contains(pattern.as_str()) {
                    match_at_least_one = true;
                }
            }

            if match_at_least_one {
                paths.push(path);
            }
        }
        self.paths = paths;

        self
    }

    pub fn notes(&self) -> Vec<PathBuf> {
        self.paths.clone()
    }
}

#[cfg(test)]
mod note_tests {
    use std::io::Write;

    use super::*;

    fn temp_mdfile() -> Result<tempfile::NamedTempFile, Box<dyn Error>> {
        let mut file = tempfile::NamedTempFile::new()?;
        file.write_all(
            "# Title
This is a [link title](link_url).
"
            .as_bytes(),
        )?;
        Ok(file)
    }

    #[test]
    fn test_note_parse() -> Result<(), Box<dyn Error>> {
        let mdfile = temp_mdfile()?;
        let note = Note::build(mdfile.path())?;

        assert_eq!(
            note.links,
            vec![Link {
                title: Some(String::from("link title")),
                url: String::from("link_url"),
            }]
        );

        Ok(())
    }
}

#[cfg(test)]
mod filter_tests {
    use super::Filter;
    use rand::distributions::{Alphanumeric, DistString};
    use std::path::PathBuf;

    fn rand_string() -> String {
        Alphanumeric.sample_string(&mut rand::thread_rng(), 16)
    }

    #[test]
    fn add() -> Result<(), std::io::Error> {
        let path = rand_string();
        assert_eq!(
            Filter::new().add(&PathBuf::from(path.as_str())).notes(),
            Vec::<PathBuf>::new()
        );

        let dir = tempfile::tempdir()?;
        let mut files = Vec::new();
        for _ in 0..5 {
            let filepath = dir.path().join(rand_string().as_str());
            if let Ok(_) = std::fs::File::create(filepath.clone()) {
                files.push(filepath);
            }
        }

        let notes = Filter::new().add(&dir.path().to_path_buf());
        assert_eq!(notes.notes(), files);

        Ok(())
    }

    #[test]
    fn journal_only() -> Result<(), anyhow::Error> {
        let dir = tempfile::tempdir()?;
        std::fs::create_dir_all(dir.path().join("journals")).expect("failed to create dir");

        let paths = ["journals/test.md", "test.md"].map(|p| {
            let p = dir.path().join(p);
            std::fs::File::create(&p).expect("failed to create file");
            p
        });
        let expected = ["journals/test.md"].map(|p| dir.path().join(p));

        let mut filter = Filter::new();
        for path in paths {
            filter = filter.add(&path);
        }

        let notes = filter.journal_only().notes();
        assert_eq!(notes, expected);

        Ok(())
    }
}
