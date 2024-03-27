use iced::{
    widget::{component, Button, Column, Component, Text, TextInput},
    Element,
};
use lorecore::sql::lore_database::LoreDatabase;

use crate::{app::message_handling::GuiMes, errors::LoreGuiError};

use super::Dialog;

#[derive(Debug, Clone)]
pub(crate) struct RedateHistoryDialog {
    data: RedateHistoryData,
}

impl RedateHistoryDialog {
    pub(crate) fn new(data: RedateHistoryData) -> Self {
        RedateHistoryDialog { data }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RedateHistoryData {
    pub(self) timestamp: i64,
    pub(self) old_year: i32,
    pub(self) old_day: Option<i32>,
    pub(self) new_year: i32,
    pub(self) new_day: Option<i32>,
}

impl RedateHistoryData {
    pub(crate) fn new(timestamp: i64, old_year: i32, old_day: Option<i32>) -> Self {
        RedateHistoryData {
            timestamp,
            new_year: old_year,
            new_day: old_day.clone(),
            old_year,
            old_day,
        }
    }

    // Add your logic to update the history in the database
}

impl Dialog for RedateHistoryDialog {
    fn header(&self) -> String {
        format!(
            "Redate history for entity: year {}, day {} ({})",
            self.data.old_year,
            self.data.old_day.unwrap_or(0),
            self.data.timestamp
        )
    }

    fn body<'a>(&self) -> Element<'a, GuiMes> {
        component(self.clone())
    }
}

impl Component<GuiMes> for RedateHistoryDialog {
    type State = ();

    type Event = RedateHistoryMes;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        match event {
            RedateHistoryMes::YearUpd(year) => {
                self.data.new_year = year;
                None
            }
            RedateHistoryMes::DayUpd(day) => {
                self.data.new_day = Some(day);
                None
            }
            RedateHistoryMes::Submit => Some(GuiMes::RedateHistoryItem(self.data.to_owned())),
        }
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        let year_input = TextInput::new("", &self.data.new_year.to_string())
            .on_input(|input| RedateHistoryMes::YearUpd(input.parse().unwrap_or_default()));
        let day_string = match self.data.new_day {
            Some(day) => day.to_string(),
            None => String::new(),
        };
        let day_input = TextInput::new("", &day_string)
            .on_input(|input| RedateHistoryMes::DayUpd(input.parse().unwrap_or_default()));
        let submit_button = Button::new("Redate").on_press(RedateHistoryMes::Submit);
        Column::new()
            .push(Text::new("Year:"))
            .push(year_input)
            .push(Text::new("Day (optional):"))
            .push(day_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum RedateHistoryMes {
    YearUpd(i32),
    DayUpd(i32),
    Submit,
}