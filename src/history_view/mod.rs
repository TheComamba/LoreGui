use lorecore::sql::history::HistoryItem;

use super::db_col_view::state::DbColViewState;

mod widget;

pub(super) struct HistoryView<'a> {
    state: &'a HistoryViewState,
}

impl<'a> HistoryView<'a> {
    pub(super) fn new(state: &'a HistoryViewState) -> Self {
        Self { state }
    }
}

pub(super) struct HistoryViewState {
    history_items: Vec<HistoryItem>,
    pub(super) year_view_state: DbColViewState,
    pub(super) day_view_state: DbColViewState,
    pub(super) timestamp_view_state: DbColViewState,
    pub(super) current_content: String,
}

impl HistoryViewState {
    pub(super) fn new(years: Vec<i32>) -> Self {
        let years = years.iter().map(|y| y.to_string()).collect();
        Self {
            history_items: vec![],
            year_view_state: DbColViewState::new(years),
            day_view_state: DbColViewState::default(),
            timestamp_view_state: DbColViewState::default(),
            current_content: String::new(),
        }
    }

    pub(super) fn get_all_years(&self) -> Vec<i32> {
        let mut years: Vec<i32> = self.history_items.iter().map(|item| item.year).collect();
        years.sort();
        years.dedup();
        years
    }

    pub(super) fn get_days(&self, year: i32) -> Vec<Option<i32>> {
        let mut days: Vec<Option<i32>> = self
            .history_items
            .iter()
            .filter(|item| item.year == year)
            .map(|item| item.day)
            .collect();
        days.sort();
        days.dedup();
        days
    }

    pub(super) fn get_timestamps(&self, year: i32, day: Option<i32>) -> Vec<i64> {
        let mut timestamps: Vec<i64> = self
            .history_items
            .iter()
            .filter(|item| item.year == year)
            .filter(|item| item.day == day)
            .map(|item| item.timestamp)
            .collect();
        timestamps.sort();
        timestamps.dedup();
        timestamps
    }

    pub(super) fn get_content(&self, timestamp: i64) -> String {
        self.history_items
            .iter()
            .find(|item| item.timestamp == timestamp)
            .map(|item| item.content.clone())
            .unwrap_or(String::new())
    }
}

impl Default for HistoryViewState {
    fn default() -> Self {
        Self::new(vec![])
    }
}
