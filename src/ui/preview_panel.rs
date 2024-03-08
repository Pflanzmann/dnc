use crate::UiState;

pub fn preview_panel(ui: &mut egui::Ui, ui_state: &mut UiState) {
    egui::CentralPanel::default().show_inside(ui, |ui| {
        ui.vertical_centered(|ui| {
            ui.heading("Preview");
        });

        if let Some(item) = &ui_state.preview_item {
            ui.label(&item.name);
            ui.label(&item.kind);
            ui.label(&item.rarity);
            ui.label(&item.description);
            ui.label(&item.flavor);
        }
    });
}
