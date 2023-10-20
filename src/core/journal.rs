use std::path::Path;
use std::path::PathBuf;

#[derive(Clone, Debug, clap::ValueEnum)]
pub enum Period {
    All,
    Daily,
    Weekly,
    Monthly,
    Yearly,
}

pub fn paths(date: chrono::NaiveDate, period: &Period, root_dir: PathBuf) -> Vec<PathBuf> {
    let daily_path = date.format("%Y-%m-%d.md").to_string();
    let weekly_path = date.format("%Y-w%U.md").to_string();
    let monthly_path = date.format("%Y-%m.md").to_string();
    let yearly_path = date.format("%Y.md").to_string();

    let _paths = match period {
        Period::Daily => {
            vec![daily_path]
        }
        Period::Weekly => {
            vec![weekly_path]
        }
        Period::Monthly => {
            vec![monthly_path]
        }
        Period::Yearly => {
            vec![yearly_path]
        }
        Period::All => {
            vec![daily_path, weekly_path, monthly_path, yearly_path]
        }
    };

    _paths
        .iter()
        .map(|p| root_dir.join("journals").join(Path::new(p)))
        .collect()
}
