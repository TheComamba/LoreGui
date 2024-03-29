use super::RelationshipView;
use crate::{app::message_handling::GuiMes, db_col_view::widget::DbColView, style::header};
use iced::widget::{component, Component};
use iced::Alignment;
use iced::{
    widget::{Column, Row, Text},
    Element, Length,
};

impl<'a> Component<GuiMes> for RelationshipView<'a> {
    type State = ();

    type Event = GuiMes;

    fn update(&mut self, _state: &mut Self::State, event: Self::Event) -> Option<GuiMes> {
        Some(event)
    }

    fn view(&self, _state: &Self::State) -> Element<'_, Self::Event> {
        Row::new()
            .push(DbColView::new(
                "Parent",
                vec![],
                GuiMes::ParentViewUpd,
                &self.state.parent_view_state,
            ))
            .push(DbColView::new(
                "Child",
                vec![],
                GuiMes::ChildViewUpd,
                &self.state.child_view_state,
            ))
            .push(self.role_view())
            .align_items(Alignment::Start)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

impl<'a> RelationshipView<'a> {
    fn role_view(&self) -> Element<'a, GuiMes> {
        let mut col = Column::new().push(header("Role"));
        if let Some(role) = self.state.current_role.as_ref() {
            col = col.push(Text::new(role));
        }
        col.padding(5).spacing(5).width(Length::Fill).into()
    }
}

impl<'a> From<RelationshipView<'a>> for Element<'a, GuiMes> {
    fn from(entity_view: RelationshipView<'a>) -> Self {
        component(entity_view)
    }
}
