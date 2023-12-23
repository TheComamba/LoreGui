use lorecore::sql::{lore_database::LoreDatabase, search_text::RelationshipSearchParams};

use super::SqlGui;
use crate::{
    db_col_view::ColViewMes, errors::LoreGuiError, relationship_view::RelationshipViewState,
};

impl SqlGui {
    pub(super) fn update_parent_view(&mut self, event: ColViewMes) -> Result<(), LoreGuiError> {
        let state = &mut self.relationship_view_state;
        match event {
            ColViewMes::New => (),
            ColViewMes::SearchFieldUpd(text) => {
                state.parent_view_state.set_search_text(text);
                state.update_parents(&self.lore_database)?;
            }
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
            ColViewMes::SearchFieldUpd(text) => {
                state.child_view_state.set_search_text(text);
                state.update_children(&self.lore_database)?;
            }
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
    pub(super) fn reset_selections(
        &mut self,
        db: &Option<LoreDatabase>,
    ) -> Result<(), LoreGuiError> {
        self.parent_view_state.set_selected_none();
        self.child_view_state.set_selected_none();
        self.current_role = None;
        self.update_parents(db)?;
        self.update_children(db)?;
        Ok(())
    }

    fn update_parents(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let parents = self.get_parents(db)?;
        self.parent_view_state.set_entries(parents);
        Ok(())
    }

    fn get_parents(&self, db: &Option<LoreDatabase>) -> Result<Vec<String>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let child = self
            .child_view_state
            .get_selected()
            .as_ref()
            .map(|c| (c.as_str(), true));
        let search_text = self.parent_view_state.get_search_text().map(|t| (t, false));
        let search_params = RelationshipSearchParams::new(search_text, child);
        let relationships = db
            .get_relationships(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        let parents = relationships.iter().map(|rel| rel.parent.clone()).collect();
        Ok(parents)
    }

    fn update_children(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        let children = self.get_children(db)?;
        self.child_view_state.set_entries(children);
        Ok(())
    }

    fn get_children(&self, db: &Option<LoreDatabase>) -> Result<Vec<String>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(vec![]),
        };
        let parent = self
            .parent_view_state
            .get_selected()
            .as_ref()
            .map(|p| (p.as_str(), true));
        let search_text = self.child_view_state.get_search_text().map(|t| (t, false));
        let search_params = RelationshipSearchParams::new(parent, search_text);
        let relationships = db
            .get_relationships(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        let children = relationships.iter().map(|rel| rel.child.clone()).collect();
        Ok(children)
    }

    fn update_role(&mut self, db: &Option<LoreDatabase>) -> Result<(), LoreGuiError> {
        self.current_role = self.get_role(db)?;
        Ok(())
    }

    fn get_role(&self, db: &Option<LoreDatabase>) -> Result<Option<String>, LoreGuiError> {
        let db = match db {
            Some(db) => db,
            None => return Ok(None),
        };
        let parent = match self.parent_view_state.get_selected() {
            Some(parent) => parent,
            None => return Ok(None),
        };
        let child = match self.child_view_state.get_selected() {
            Some(child) => child,
            None => return Ok(None),
        };
        let search_params = RelationshipSearchParams::new(
            Some((parent.as_str(), true)),
            Some((child.as_str(), true)),
        );
        let relationships = db
            .get_relationships(search_params)
            .map_err(LoreGuiError::LoreCoreError)?;
        if relationships.len() > 1 {
            return Err(LoreGuiError::InputError(
                "Multiple relationships found".to_string(),
            ));
        }
        let role = match relationships.first() {
            Some(relationship) => relationship.role.clone(),
            None => None,
        };
        Ok(role)
    }
}
