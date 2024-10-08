use iced::widget::text_editor;
use lorecore::{
    extractions::{extract_days, extract_years},
    sql::{lore_database::LoreDatabase, search_params::HistoryItemSearchParams},
    types::*,
};

use crate::{
    db_col_view::{entry::DbColViewEntry, ColViewMes},
    dialog::redate_history::RedateHistoryData,
    editor::EditorState,
    errors::LoreGuiError,
};

use super::db_col_view::state::DbColViewState;

pub(crate) mod widget;

pub(super) struct HistoryViewState {
    pub(super) year_view_state: DbColViewState<Year>,
    pub(super) day_view_state: DbColViewState<Day>,
    pub(super) timestamp_view_state: DbColViewState<Timestamp>,
    pub(super) current_content: EditorState,
}

#[derive(Debug, Clone)]
pub(super) enum HistoryViewMessage {
    NewHistoryItem,
    RedateHistoryItem(RedateHistoryData),
    DeleteHistoryItem(Timestamp),
    YearViewUpdate(ColViewMes<Year>),
    DayViewUpdate(ColViewMes<Day>),
    HistoryTimestampViewUpdate(ColViewMes<Timestamp>),
    ContentUpdate(text_editor::Action),
    ContentDiscard,
    ContentSave,
}

impl HistoryViewState {
    pub(super) fn new() -> Self {
        Self {
            year_view_state: DbColViewState::default(),
            day_view_state: DbColViewState::default(),
            timestamp_view_state: DbColViewState::default(),
            current_content: EditorState::default(),
        }
    }

    pub(super) fn get_current_years(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<Year>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let year = self.year_view_state.get_search_int()?.map(|y| y.into());
        let search_params = HistoryItemSearchParams::new(year, None, None, None);
        let history_items = db.read_history_items(search_params)?;
        let years = extract_years(&history_items);
        Ok(years)
    }

    pub(super) fn get_current_days(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<Day>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let year = match self.get_selected_year() {
            Some(year) => Some(year),
            None => return Ok(vec![]),
        };

        let day = self.day_view_state.get_search_int()?.map(|d| d.into());
        let search_params = HistoryItemSearchParams::new(year, day, None, None);
        let history_items = db.read_history_items(search_params)?;
        Ok(extract_days(&history_items))
    }

    pub(super) fn get_current_timestamps(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<Vec<Timestamp>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let year = match self.get_selected_year() {
            Some(year) => Some(year),
            None => return Ok(vec![]),
        };
        let day = self.get_selected_day();

        let search_params = HistoryItemSearchParams::new(year, day, None, None);
        let history_items = db.read_history_items(search_params)?;
        let timestamps = history_items
            .iter()
            .map(|item| item.timestamp)
            .collect::<Vec<Timestamp>>();
        Ok(timestamps)
    }

    pub(super) fn get_current_content(
        &self,
        db: &Option<LoreDatabase>,
    ) -> Result<HistoryItemContent, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok("".into()),
        };
        let timestamp = match self.get_selected_timestamp() {
            Some(timestamp) => timestamp,
            None => return Ok("".into()),
        };

        let search_params = HistoryItemSearchParams::new(None, None, Some(timestamp), None);
        let history_items = db.read_history_items(search_params)?;
        if history_items.len() > 1 {
            return Err(LoreGuiError::MultipleResults);
        }
        let content = match history_items.first() {
            Some(item) => item.content.clone(),
            None => "".into(),
        };
        Ok(content)
    }

    pub(super) fn get_selected_year(&self) -> Option<Year> {
        self.year_view_state.get_selected().0
    }

    pub(super) fn set_selected_year(&mut self, year: Option<Year>) {
        self.year_view_state.set_selected(DbColViewEntry(year));
    }

    pub(super) fn get_selected_day(&self) -> Option<Day> {
        self.day_view_state.get_selected().0
    }

    pub(super) fn set_selected_day(&mut self, day: Option<Day>) {
        self.day_view_state.set_selected(DbColViewEntry(day));
    }

    pub(super) fn get_selected_timestamp(&self) -> Option<Timestamp> {
        self.timestamp_view_state.get_selected().0
    }

    pub(super) fn set_selected_timestamp(&mut self, timestamp: Option<Timestamp>) {
        self.timestamp_view_state
            .set_selected(DbColViewEntry(timestamp));
    }

    pub(super) fn get_content_text(&self) -> String {
        self.current_content.get_text()
    }

    pub(super) fn set_content_text(&mut self, text: &str) {
        self.current_content = EditorState::new(text);
    }
}

impl Default for HistoryViewState {
    fn default() -> Self {
        Self::new()
    }
}
