use super::SqlGui;
use crate::{
    db_col_view::{state::DbColViewState, ColViewMes},
    dialog::new_entity::NewEntityDialog,
    entity_view::EntityViewState,
    errors::LoreGuiError,
};
use lorecore::sql::{entity::EntityColumn, lore_database::LoreDatabase};

impl SqlGui {
    pub(super) fn update_label_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let state = &mut self.entity_view_state;
        match event {
            ColViewMes::New => self.dialog = Some(Box::new(NewEntityDialog::new())),
            ColViewMes::SearchFieldUpd(text) => state.label_view_state.set_search_text(text),
            ColViewMes::Selected(_index, label) => {
                state.label_view_state.set_selected(label);
                state.descriptor_view_state.set_selected_none();
                state.update_descriptors(&self.lore_database)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_descriptor_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let state = &mut self.entity_view_state;
        match event {
            ColViewMes::New => (),
            ColViewMes::SearchFieldUpd(text) => state.descriptor_view_state.set_search_text(text),
            ColViewMes::Selected(_index, descriptor) => {
                state.descriptor_view_state.set_selected(descriptor);
                state.update_description(&self.lore_database)?;
            }
        };
        Ok(())
    }
}

impl EntityViewState {
    pub(super) fn reset(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        self.reset_selections();
        self.update_labels(db)?;
        Ok(())
    }

    fn reset_selections(&mut self) {
        self.label_view_state.set_selected_none();
        self.descriptor_view_state.set_selected_none();
        self.current_description = None;
    }

    fn update_labels(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        match db {
            Some(db) => self.label_view_state.set_entries(
                db.get_all_entity_labels()
                    .map_err(LoreGuiError::LoreCoreError)?,
            ),
            None => self.label_view_state = DbColViewState::new(),
        }
        self.update_descriptors(db)?;
        Ok(())
    }

    fn update_descriptors(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let label = self.label_view_state.get_selected();
        match db {
            Some(db) => self.descriptor_view_state.set_entries(
                db.get_descriptors(&label.as_ref())
                    .map_err(LoreGuiError::LoreCoreError)?,
            ),
            None => self.descriptor_view_state = DbColViewState::new(),
        }
        self.update_description(db)?;
        Ok(())
    }

    fn update_description(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let label = match self.label_view_state.get_selected() {
            Some(label) => label,
            None => {
                self.current_description = None;
                return Ok(());
            }
        };
        let descriptor = match self.descriptor_view_state.get_selected() {
            Some(descriptor) => descriptor,
            None => {
                self.current_description = None;
                return Ok(());
            }
        };
        match db {
            Some(db) => {
                self.current_description = db
                    .get_description(label, descriptor)
                    .map_err(LoreGuiError::LoreCoreError)?
            }
            None => self.current_description = None,
        }
        Ok(())
    }

    fn new_descriptor(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let label = match self.label_view_state.get_selected().as_ref() {
            Some(label) => label.clone(),
            None => {
                return Err(LoreGuiError::InputError(
                    "No label selected for which to create new descriptor.".to_string(),
                ));
            }
        };
        let descriptor = self.descriptor_view_state.get_search_text().clone();
        if descriptor.is_empty() {
            return Err(LoreGuiError::InputError(
                "Cannot create empty descriptor.".to_string(),
            ));
        }
        let description = None;
        let new_col = EntityColumn {
            label,
            descriptor,
            description,
        };
        match db {
            Some(db) => db
                .write_entity_columns(vec![new_col])
                .map_err(LoreGuiError::LoreCoreError)?,
            None => {
                return Err(LoreGuiError::InputError(
                    "No database loaded to which to add descriptor.".to_string(),
                ));
            }
        };
        self.update_descriptors(db)?;
        Ok(())
    }
}
