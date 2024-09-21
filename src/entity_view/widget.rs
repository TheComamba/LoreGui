use super::{EntityViewMessage, EntityViewState};
use crate::db_col_view;
use crate::dialog::relabel_entity::RelabelEntityData;
use crate::dialog::rename_descriptor::RenameDescriptorData;
use crate::{app::message_handling::GuiMessage, style::header};
use iced::widget::button;
use iced::{
    widget::{text_editor, Column, Row},
    Alignment, Element, Length,
};

pub(crate) fn new(state: &EntityViewState) -> Element<'_, GuiMessage> {
    Column::new()
        .push(label_buttons(state))
        .push(descriptor_buttons(state))
        .push(col_views(state))
        .into()
}

fn label_buttons(state: &EntityViewState) -> Row<'_, GuiMessage> {
    let new_entity =
        button("New Entity").on_press(GuiMessage::EntityViewUpd(EntityViewMessage::NewEntity));
    let mut relabel_entity = button("Relabel Entity");
    let mut delete_entity = button("Delete Entity");
    if let Some(label) = &state.label_view_state.get_selected().0 {
        let relabel_entity_data = RelabelEntityData::new(label.clone());
        relabel_entity = relabel_entity.on_press(GuiMessage::EntityViewUpd(
            EntityViewMessage::RelabelEntity(relabel_entity_data),
        ));
        delete_entity = delete_entity.on_press(GuiMessage::EntityViewUpd(
            EntityViewMessage::DeleteEntity(label.clone()),
        ));
    }
    Row::new()
        .push(new_entity)
        .push(relabel_entity)
        .push(delete_entity)
        .spacing(5)
        .padding(5)
}

fn descriptor_buttons(state: &EntityViewState) -> Row<'_, GuiMessage> {
    let mut new_descriptor = button("New Descriptor");
    let mut rename_descriptor = button("Rename Descriptor");
    let mut delete_descriptor = button("Delete Descriptor");
    if let Some(label) = &state.label_view_state.get_selected().0 {
        new_descriptor = new_descriptor.on_press(GuiMessage::EntityViewUpd(
            EntityViewMessage::NewDescriptor(label.clone()),
        ));
        if let Some(descriptor) = &state.descriptor_view_state.get_selected().0 {
            let rename_descriptor_data =
                RenameDescriptorData::new(label.clone(), descriptor.clone());
            rename_descriptor = rename_descriptor.on_press(GuiMessage::EntityViewUpd(
                EntityViewMessage::RenameDescriptor(rename_descriptor_data),
            ));
            delete_descriptor = delete_descriptor.on_press(GuiMessage::EntityViewUpd(
                EntityViewMessage::DeleteDescriptor(label.clone(), descriptor.clone()),
            ));
        }
    }
    Row::new()
        .push(new_descriptor)
        .push(rename_descriptor)
        .push(delete_descriptor)
        .spacing(5)
        .padding(5)
}

fn col_views(state: &EntityViewState) -> Row<'_, GuiMessage> {
    Row::new()
        .push(db_col_view::widget::new(
            "Label",
            |m| GuiMessage::EntityViewUpd(EntityViewMessage::LabelViewUpdate(m)),
            &state.label_view_state,
        ))
        .push(db_col_view::widget::new(
            "Descriptor",
            |m| GuiMessage::EntityViewUpd(EntityViewMessage::DescriptorViewUpdate(m)),
            &state.descriptor_view_state,
        ))
        .push(desription_view(state))
        .align_y(Alignment::Start)
        .width(Length::Fill)
        .height(Length::Fill)
}

fn desription_view(state: &EntityViewState) -> Column<'_, GuiMessage> {
    let description_editor = text_editor(&state.current_description)
        .on_action(|a| GuiMessage::EntityViewUpd(EntityViewMessage::DescriptionUpdate(a)));
    Column::new()
        .push(header("Description"))
        .push(description_editor)
        .padding(5)
        .spacing(5)
        .width(Length::Fill)
}
