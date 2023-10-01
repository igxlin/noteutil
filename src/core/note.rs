use std::path::PathBuf;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum JournalPeriod {
    All,
    Daily,
    Weekly,
}

pub struct Filter {
    paths: Vec<PathBuf>,
    period: JournalPeriod,
}

impl Filter {
    pub fn new() -> Self {
        Self {
            paths: Vec::new(),
            period: JournalPeriod::All,
        }
    }

    pub fn period(mut self, period: JournalPeriod) -> Self {
        self.period = period;
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
        let patterns: Vec<String>;
        let daily_patterns = vec![date.format("%Y-%m-%d").to_string()];
        let weekly_patterns = vec![date.format("%Y-w%U").to_string()];
        match self.period {
            JournalPeriod::Daily => {
                patterns = daily_patterns;
            }
            JournalPeriod::Weekly => {
                patterns = weekly_patterns;
            }
            JournalPeriod::All => {
                let mut pat = Vec::new();
                pat.extend_from_slice(&daily_patterns);
                pat.extend_from_slice(&weekly_patterns);
                patterns = pat;
            }
        }

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
