use super::{SqlGui, ViewType};
use crate::{db_col_view::ColViewMes, dialog::new_entity::NewEntityData, errors::LoreGuiError};

#[derive(Debug, Clone)]
pub(crate) enum GuiMes {
    ViewSelected(ViewType),
    NewDatabase,
    OpenDatabase,
    EntityLabelViewUpd(ColViewMes),
    DescriptorViewUpd(ColViewMes),
    YearViewUpd(ColViewMes),
    DayViewUpd(ColViewMes),
    HistoryLabelViewUpd(ColViewMes),
    ParentViewUpd(ColViewMes),
    ChildViewUpd(ColViewMes),
    DialogClosed,
    EntitySubmit(NewEntityData),
}

impl SqlGui {
    pub(super) fn handle_message(&mut self, message: GuiMes) -> Result<(), LoreGuiError> {
        match message {
            GuiMes::ViewSelected(view) => self.selected_view = view,
            GuiMes::NewDatabase => self.new_database_from_dialog()?,
            GuiMes::OpenDatabase => self.open_database_from_dialog()?,
            GuiMes::EntityLabelViewUpd(event) => self.update_label_view(event)?,
            GuiMes::DescriptorViewUpd(event) => self.update_descriptor_view(event)?,
            GuiMes::YearViewUpd(event) => self.update_year_view(event)?,
            GuiMes::DayViewUpd(event) => self.update_day_view(event)?,
            GuiMes::HistoryLabelViewUpd(event) => self.update_history_label_view(event)?,
            GuiMes::ParentViewUpd(event) => self.update_parent_view(event)?,
            GuiMes::ChildViewUpd(event) => self.update_child_view(event)?,
            GuiMes::DialogClosed => self.dialog = None,
            GuiMes::EntitySubmit(data) => {
                data.write_to_database(&self.lore_database)?;
                self.dialog = None;
            }
        }
        Ok(())
    }
}
