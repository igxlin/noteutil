use crate::core::journal::Period as JournalPeriod;
use std::sync::Arc;

pub struct Context {
    pub default_journal_periods: Arc<Vec<JournalPeriod>>,
}

impl Default for Context {
    fn default() -> Self {
        Self {
            default_journal_periods: Arc::new(vec![
                JournalPeriod::Daily,
                JournalPeriod::Weekly,
                JournalPeriod::Monthly,
                JournalPeriod::Yearly,
            ]),
        }
    }
}
