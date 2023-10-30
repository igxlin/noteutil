use std::path::Path;
use std::path::PathBuf;

use crate::core::config::Config;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Period {
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

lazy_static::lazy_static! {
    pub static ref ALL_PERIODS: Vec<Period> = vec![
        Period::Daily,
        Period::Weekly,
        Period::Monthly,
        Period::Yearly,
    ];
}

pub fn paths(
    date: chrono::NaiveDate,
    args_periods: &Vec<Period>,
    root_dir: PathBuf,
    config: &Config,
) -> Vec<PathBuf> {
    let path_format = &config.journal.path;
    let formats = [
        &path_format.daily,
        &path_format.weekly,
        &path_format.monthly,
        &path_format.yearly,
    ];

    let [daily_path, weekly_path, monthly_path, yearly_path] =
        formats.map(|format| date.format(format.as_str()).to_string());

    let periods: &Vec<Period>;
    if args_periods.is_empty() {
        periods = &ALL_PERIODS;
    } else {
        periods = args_periods;
    }

    let _paths: Vec<&str> = periods
        .iter()
        .map(|period| match period {
            Period::Daily => daily_path.as_str(),
            Period::Weekly => weekly_path.as_str(),
            Period::Monthly => monthly_path.as_str(),
            Period::Yearly => yearly_path.as_str(),
        })
        .collect();

    _paths.iter().map(|p| root_dir.join(Path::new(p))).collect()
}

#[cfg(test)]
mod paths_tests {
    use super::*;

    #[test]
    fn no_periods() -> Result<(), anyhow::Error> {
        assert_eq!(
            vec![
                "./journals/2023-10-21.md",
                "./journals/2023-w42.md",
                "./journals/2023-10.md",
                "./journals/2023.md"
            ]
            .iter()
            .map(|p| Path::new(p).to_path_buf())
            .collect::<Vec<PathBuf>>(),
            paths(
                chrono::NaiveDate::from_ymd_opt(2023, 10, 21).unwrap(),
                &Vec::new(),
                Path::new(".").to_path_buf(),
                &Config::default(),
            ),
        );

        Ok(())
    }

    #[test]
    fn one_period() -> Result<(), anyhow::Error> {
        assert_eq!(
            vec!["./journals/2023-10-21.md"]
                .iter()
                .map(|p| Path::new(p).to_path_buf())
                .collect::<Vec<PathBuf>>(),
            paths(
                chrono::NaiveDate::from_ymd_opt(2023, 10, 21).unwrap(),
                &vec![Period::Daily],
                Path::new(".").to_path_buf(),
                &Config::default(),
            ),
        );
        Ok(())
    }
}
