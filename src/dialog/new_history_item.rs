use iced::{
    widget::{Button, Column, Text, TextInput},
    Element,
};
use lorecore::{
    sql::lore_database::LoreDatabase,
    types::{
        day::Day, history::HistoryItem, history_item_content::HistoryItemContent,
        history_item_properties::HistoryItemProperties, year::Year,
    },
};

use crate::{app::message_handling::GuiMessage, errors::LoreGuiError};

use super::{Dialog, DialogUpdate};

#[derive(Clone, Debug)]
pub(crate) struct NewHistoryDialog {
    data: NewHistoryData,
}

impl NewHistoryDialog {
    pub(crate) fn new() -> Self {
        NewHistoryDialog {
            data: NewHistoryData {
                year: 0.into(),
                day: Day::NONE,
                content: "".into(),
                properties: HistoryItemProperties::none(),
            },
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct NewHistoryData {
    pub(self) year: Year,
    pub(self) day: Day,
    pub(self) content: HistoryItemContent,
    pub(self) properties: HistoryItemProperties,
}

impl NewHistoryData {
    pub(crate) fn write_to_database(self, db: &LoreDatabase) -> Result<(), LoreGuiError> {
        let item = HistoryItem {
            timestamp: lorecore::timestamp::current_timestamp(),
            year: self.year,
            day: self.day,
            content: self.content,
            properties: self.properties,
        };
        db.write_history_items(vec![item])?;
        Ok(())
    }

    pub(crate) fn year(&self) -> &Year {
        &self.year
    }

    pub(crate) fn day(&self) -> &Day {
        &self.day
    }

    #[cfg(test)]
    pub(crate) fn content(&self) -> &HistoryItemContent {
        &self.content
    }

    #[cfg(test)]
    pub(crate) fn properties(&self) -> &HistoryItemProperties {
        &self.properties
    }
}

impl Dialog for NewHistoryDialog {
    fn header(&self) -> String {
        "Create new history item".to_string()
    }

    fn body(&self) -> Element<'_, GuiMessage> {
        let year_input = TextInput::new("", &self.data.year.to_string())
            .on_input(|i| GuiMessage::DialogUpdate(DialogUpdate::Year(i.try_into())));
        let day_string = format!("{}", self.data.day);
        let day_input = TextInput::new("", &day_string)
            .on_input(|i| GuiMessage::DialogUpdate(DialogUpdate::Day(i.try_into())));
        let content_input = TextInput::new("", self.data.content.to_str())
            .on_input(|i| GuiMessage::DialogUpdate(DialogUpdate::Content(i.into())));
        let submit_button = Button::new("Create").on_press(GuiMessage::DialogSubmit);
        Column::new()
            .push(Text::new("Year:"))
            .push(year_input)
            .push(Text::new("Day:"))
            .push(day_input)
            .push(Text::new("Content:"))
            .push(content_input)
            .push(submit_button)
            .padding(5)
            .spacing(5)
            .into()
    }

    fn update(&mut self, message: DialogUpdate) {
        match message {
            DialogUpdate::Year(Ok(year)) => self.data.year = year,
            DialogUpdate::Day(Ok(day)) => self.data.day = day,
            DialogUpdate::Content(content) => self.data.content = content,
            _ => (),
        }
    }

    fn submit(&self) -> GuiMessage {
        GuiMessage::NewHistoryItem(self.data.clone())
    }
}
