use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use eframe::epaint::Color32;
use egui::{Margin, RichText};

use crate::models::item::Item;
use crate::UiState;

pub fn preview_panel(ui: &mut egui::Ui, ui_state: &mut UiState) {
    let width = ui.ctx().screen_rect().width();

    egui::SidePanel::right("preview_panel")
        .frame(egui::Frame {
            fill: Color32::WHITE,
            inner_margin: Margin::symmetric(0.0, 0.0),
            outer_margin: Margin::symmetric(0.0, 0.0),
            ..Default::default()
        })
        .default_width(300.0)
        .width_range(80.0..=width * 0.3f32)
        .show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Preview");
            });

            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label("Name");
                    ui.label("Type");
                    ui.label("Rarity");
                    ui.label("Description");
                    ui.label("Flavor");
                });

                ui.vertical(|ui| {
                    if let Some(item) = &ui_state.preview_item {
                        ui.label(&item.name);

                        ui.label(&item.kind);

                        ui.label(&item.rarity);

                        ui.label(&item.description);

                        ui.label(&item.flavor);
                    }
                });
            });
        });
}
