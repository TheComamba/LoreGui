use iced::widget::text_editor;
use lorecore::{
    sql::lore_database::LoreDatabase,
    types::{descriptor::Descriptor, label::Label},
};

use crate::{
    db_col_view::{entry::DbColViewEntry, ColViewMes},
    dialog::{
        confirmation::ConfirmationDialog,
        new_descriptor::{NewDescriptorData, NewDescriptorDialog},
        new_entity::{NewEntityData, NewEntityDialog},
        relabel_entity::{RelabelEntityData, RelabelEntityDialog},
        rename_descriptor::{RenameDescriptorData, RenameDescriptorDialog},
    },
    entity_view::{EntityViewMessage, EntityViewState},
    errors::LoreGuiError,
};

use super::{message_handling::GuiMes, SqlGui};

impl SqlGui {
    pub(super) fn update_entity_view(
        &mut self,
        event: EntityViewMessage,
    ) -> Result<(), LoreGuiError> {
        match event {
            EntityViewMessage::NewEntity => self.dialog = Some(Box::new(NewEntityDialog::new())),
            EntityViewMessage::RelabelEntity(data) => {
                self.dialog = Some(Box::new(RelabelEntityDialog::new(data)))
            }
            EntityViewMessage::DeleteEntity(label) => {
                let message = format!("Do you really want to delete {}?", label);
                let on_confirm = GuiMes::DeleteEntity(label);
                self.dialog = Some(Box::new(ConfirmationDialog::new(message, on_confirm)))
            }
            EntityViewMessage::NewDescriptor(label) => {
                self.dialog = Some(Box::new(NewDescriptorDialog::new(label.clone())))
            }
            EntityViewMessage::RenameDescriptor(data) => {
                self.dialog = Some(Box::new(RenameDescriptorDialog::new(data)))
            }
            EntityViewMessage::DeleteDescriptor(label, descriptor) => {
                let message = format!(
                    "Do you really want to delete {}'s descriptor {}?",
                    label, descriptor
                );
                let on_confirm = GuiMes::DeleteDescriptor(label, descriptor);
                self.dialog = Some(Box::new(ConfirmationDialog::new(message, on_confirm)))
            }
            EntityViewMessage::LabelViewUpd(event) => self.update_label_view(event)?,
            EntityViewMessage::DescriptorViewUpd(event) => self.update_descriptor_view(event)?,
        };
        Ok(())
    }

    pub(super) fn update_label_view(
        &mut self,
        event: ColViewMes<Label>,
    ) -> Result<(), LoreGuiError> {
        let state = &mut self.entity_view_state;
        match event {
            ColViewMes::SearchFieldUpd(text) => {
                state.label_view_state.set_search_text(text);
                state.update_labels(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, label) => {
                state.label_view_state.set_selected(label);
                state
                    .descriptor_view_state
                    .set_selected(DbColViewEntry::NONE);
                state.update_descriptors(&self.lore_database)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_descriptor_view(
        &mut self,
        event: ColViewMes<Descriptor>,
    ) -> Result<(), LoreGuiError> {
        let state = &mut self.entity_view_state;
        match event {
            ColViewMes::SearchFieldUpd(text) => {
                state.descriptor_view_state.set_search_text(text);
                state.update_descriptors(&self.lore_database)?;
            }
            ColViewMes::Selected(_index, descriptor) => {
                state.descriptor_view_state.set_selected(descriptor);
                state.update_description(&self.lore_database)?;
            }
        };
        Ok(())
    }

    pub(super) fn write_new_entity(&mut self, data: NewEntityData) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let label = data.get_label().clone();
        data.write_to_database(db)?;
        self.update_label_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.update_label_view(ColViewMes::Selected(0, DbColViewEntry(Some(label))))?;
        self.dialog = None;
        Ok(())
    }

    pub(super) fn relable_entity(&mut self, data: RelabelEntityData) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let new_label = data.get_label().clone();
        data.update_label_in_database(db)?;
        self.update_label_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.update_label_view(ColViewMes::Selected(0, DbColViewEntry(Some(new_label))))?;
        self.dialog = None;
        Ok(())
    }

    pub(super) fn delete_entity(&mut self, label: Label) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        db.delete_entity(label)?;
        self.update_label_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.update_label_view(ColViewMes::Selected(0, DbColViewEntry::NONE))?;
        self.dialog = None;
        Ok(())
    }

    pub(super) fn write_new_descriptor(
        &mut self,
        data: NewDescriptorData,
    ) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let descriptor = data.get_descriptor().clone();
        data.write_to_database(db)?;
        self.update_descriptor_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.update_descriptor_view(ColViewMes::Selected(0, DbColViewEntry(Some(descriptor))))?;
        self.dialog = None;
        Ok(())
    }

    pub(super) fn change_descriptor(
        &mut self,
        data: RenameDescriptorData,
    ) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        let descriptor = data.get_descriptor().clone();
        data.update_descriptor_in_database(db)?;
        self.update_descriptor_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.update_descriptor_view(ColViewMes::Selected(0, DbColViewEntry(Some(descriptor))))?;
        self.dialog = None;
        Ok(())
    }

    pub(super) fn delete_descriptor(
        &mut self,
        label: Label,
        descriptor: Descriptor,
    ) -> Result<(), LoreGuiError> {
        let db = self
            .lore_database
            .as_ref()
            .ok_or(LoreGuiError::NoDatabase)?;
        db.delete_entity_column((label, descriptor))?;
        self.update_descriptor_view(ColViewMes::SearchFieldUpd(String::new()))?;
        self.update_descriptor_view(ColViewMes::Selected(0, DbColViewEntry::NONE))?;
        self.dialog = None;
        Ok(())
    }
}

impl EntityViewState {
    pub(super) fn reset_selections(
        &mut self,
        db: &Option<LoreDatabase>,
    ) -> Result<(), LoreGuiError> {
        self.label_view_state.set_selected(DbColViewEntry::NONE);
        self.descriptor_view_state
            .set_selected(DbColViewEntry::NONE);
        self.current_description = text_editor::Content::with_text("");
        self.update_labels(db)?;
        Ok(())
    }

    fn update_labels(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let labels = self
            .get_current_labels(db)?
            .into_iter()
            .map(|l| DbColViewEntry(Some(l)))
            .collect();
        self.label_view_state.set_entries(labels);
        self.update_descriptors(db)?;
        Ok(())
    }

    fn update_descriptors(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let descriptors = self
            .get_current_descriptors(db)?
            .into_iter()
            .map(|d| DbColViewEntry(Some(d)))
            .collect();
        self.descriptor_view_state.set_entries(descriptors);
        self.update_description(db)?;
        Ok(())
    }

    fn update_description(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let description = self.get_current_description(db)?;
        self.current_description = text_editor::Content::with_text(description.to_str());
        Ok(())
    }
}
