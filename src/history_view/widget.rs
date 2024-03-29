use super::HistoryView;
use crate::db_col_view::ColViewMes;
use crate::{app::message_handling::GuiMes, db_col_view::widget::DbColView, style::header};
use iced::widget::{component, text_editor, Component};
use iced::Alignment;
use iced::{
    widget::{Column, Row},
    Element, Length,
};

impl<'a> Component<GuiMes> for HistoryView<'a> {
    type State = ();

    type Event = GuiMes;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        Some(event)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        Row::new()
            .push(DbColView::new(
                "Year",
                vec![("New History Item".to_string(), Some(ColViewMes::New))],
                GuiMes::YearViewUpd,
                &self.state.year_view_state,
            ))
            .push(DbColView::new(
                "Day",
                vec![],
                GuiMes::DayViewUpd,
                &self.state.day_view_state,
            ))
            .push(DbColView::new(
                "Timestamp",
                vec![],
                GuiMes::HistoryTimestampViewUpd,
                &self.state.timestamp_view_state,
            ))
            .push(self.content_view())
            .align_items(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<'a> HistoryView<'a> {
    fn content_view(&self) -> Column<'_, GuiMes> {
        Column::new()
            .push(header("Content"))
            .push(text_editor(&self.state.current_content))
            .padding(5)
            .spacing(5)
            .width(Length::Fill)
    }
}

impl<'a> From<HistoryView<'a>> for Element<'a, GuiMes> {
    fn from(entity_view: HistoryView<'a>) -> Self {
        component(entity_view)
    }
}
