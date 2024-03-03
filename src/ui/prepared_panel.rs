use eframe::emath::Vec2;
use eframe::epaint::Color32;
use egui::{Margin, RichText, ScrollArea};

use crate::UiState;

pub fn prepared_panel(ui: &mut egui::Ui, ui_state: &mut UiState) {
    let width = ui.ctx().screen_rect().width();

    egui::SidePanel::left("prepared_panel")
        .resizable(true)
        .default_width(300.0)
        .width_range(80.0..=width * 0.3f32)
        .frame(egui::Frame {
            fill: Color32::WHITE,
            inner_margin: Margin::symmetric(0.0, 0.0),
            outer_margin: Margin::symmetric(0.0, 0.0),
            ..Default::default()
        })
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Chosen items");
            });

            ScrollArea::vertical()
                .id_source("prepared_scroll_area")
                .auto_shrink([false; 2])
                .show_viewport(ui, |ui, viewport| {
                    let mut item_to_delete: Option<usize> = None;

                    for (index, item) in ui_state.prepared_items.iter_mut().enumerate() {
                        ui.group(|ui| {
                            let width = ui.available_width();
                            ui.set_width(width);

                            ui.label(RichText::new(&item.name)
                                .color(Color32::from_rgb(0, 0, 0))
                                .size(16.0)
                            );

                            ui.add_space(5.0);

                            ui.horizontal(|ui| {
                                if ui.button("Delete").clicked() {
                                    item_to_delete = Some(index);
                                }

                                if ui.button("Preview").clicked() {
                                    ui_state.preview_item = Some(item.clone());
                                }
                            });
                        });

                        ui.add_space(5f32);
                    }

                    if let Some(index) = item_to_delete {
                        ui_state.remove_prepared_items(index);
                    }
                });
        });
}
