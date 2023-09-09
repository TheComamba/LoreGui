use super::SqlGui;
use crate::{
    db_col_view::ColViewMes, errors::LoreGuiError, relationship_view::RelationshipViewState,
};
use lorecore::sql::lore_database::LoreDatabase;

impl SqlGui {
    pub(super) fn update_parent_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let state = &mut self.relationship_view_state;
        match event {
            ColViewMes::New => (),
            ColViewMes::SearchFieldUpd(text) => state.parent_view_state.set_search_text(text),
            ColViewMes::Selected(_index, parent) => {
                state.parent_view_state.set_selected(parent);
                state.update_children(&self.lore_database)?;
                state.update_role(&self.lore_database)?;
            }
        };
        Ok(())
    }

    pub(super) fn update_child_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let state = &mut self.relationship_view_state;
        match event {
            ColViewMes::New => (),
            ColViewMes::SearchFieldUpd(text) => state.child_view_state.set_search_text(text),
            ColViewMes::Selected(_index, child) => {
                state.child_view_state.set_selected(child);
                state.update_parents(&self.lore_database)?;
                state.update_role(&self.lore_database)?;
            }
        };
        Ok(())
    }
}

impl RelationshipViewState {
    pub(super) fn reset(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        self.reset_selections();
        self.update_parents(db)?;
        self.update_children(db)?;
        Ok(())
    }

    fn reset_selections(&mut self) {
        self.parent_view_state.set_selected_none();
        self.child_view_state.set_selected_none();
        self.current_role = None;
    }

    fn update_parents(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let child = self.child_view_state.get_selected();
        match db {
            Some(db) => self.parent_view_state.set_entries(
                db.get_parents(&child.as_ref())
                    .map_err(LoreGuiError::LoreCoreError)?,
            ),
            None => self.parent_view_state.set_entries(vec![]),
        }
        Ok(())
    }

    fn update_children(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let parent = self.parent_view_state.get_selected();
        match db {
            Some(db) => self.child_view_state.set_entries(
                db.get_children(&parent.as_ref())
                    .map_err(LoreGuiError::LoreCoreError)?,
            ),
            None => self.child_view_state.set_entries(vec![]),
        }
        Ok(())
    }

    fn update_role(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let parent = match self.parent_view_state.get_selected() {
            Some(parent) => parent,
            None => {
                self.current_role = None;
                return Ok(());
            }
        };
        let child = match self.child_view_state.get_selected() {
            Some(child) => child,
            None => {
                self.current_role = None;
                return Ok(());
            }
        };
        match db {
            Some(db) => {
                self.current_role = db
                    .get_relationship_role(parent, child)
                    .map_err(LoreGuiError::LoreCoreError)?
            }
            None => self.current_role = None,
        }
        Ok(())
    }
}
