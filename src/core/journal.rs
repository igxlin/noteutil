use std::path::Path;
use std::path::PathBuf;

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
) -> Vec<PathBuf> {
    let daily_path = date.format("%Y-%m-%d.md").to_string();
    let weekly_path = date.format("%Y-w%U.md").to_string();
    let monthly_path = date.format("%Y-%m.md").to_string();
    let yearly_path = date.format("%Y.md").to_string();

    let periods: &Vec<Period>;
    if args_periods.is_empty() {
        periods = &ALL_PERIODS;
    } else {
        periods = args_periods;
    }

    let _paths: Vec<String> = periods
        .iter()
        .map(|period| match period {
            Period::Daily => daily_path.clone(),
            Period::Weekly => weekly_path.clone(),
            Period::Monthly => monthly_path.clone(),
            Period::Yearly => yearly_path.clone(),
        })
        .collect();

    _paths
        .iter()
        .map(|p| root_dir.join("journals").join(Path::new(p)))
        .collect()
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
            ),
        );
        Ok(())
    }
}
