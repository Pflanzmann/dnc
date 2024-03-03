use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

use eframe::epaint::Color32;
use egui::{RichText, Window};

use crate::models::item::Item;
use crate::UiState;

pub fn edit_panel(ui: &mut egui::Ui, ui_state: &mut UiState) -> bool {
    if let Some(item) = ui_state.editing_item.clone() {
        if ui_state.input1.is_empty() &&
            ui_state.input2.is_empty() &&
            ui_state.input3.is_empty() &&
            ui_state.input4.is_empty() &&
            ui_state.input5.is_empty()
        {
            ui_state.input1 = item.name;
            ui_state.input2 = item.kind;
            ui_state.input3 = item.rarity;
            ui_state.input4 = item.description;
            ui_state.input5 = item.flavor;

            ui_state.editing_item = None;
        }
    }

    let width = ui.ctx().screen_rect().width();

    let screen_rect = ui.ctx().available_rect();
    let window_size = egui::vec2(450.0, 600.0); // Your desired window size
    let window_position = screen_rect.center() - window_size / 1.5;

    let mut open = true;
    let mut close = false;

    Window::new("Edit Item")
        .resizable(false)
        .default_size(window_size)
        .default_pos(window_position)
        .open(&mut open)
        .collapsible(false)
        .show(ui.ctx(), |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("New Item");
            });

            let space_between_boxes = 25f32;

            ui.label("Name");
            ui.add(egui::TextEdit::multiline(&mut ui_state.input1).desired_width(f32::INFINITY));
            ui.add_space(space_between_boxes);

            ui.label("Type");
            ui.add(egui::TextEdit::multiline(&mut ui_state.input2).desired_width(f32::INFINITY));
            ui.add_space(space_between_boxes);

            ui.label("Rarity");
            ui.add(egui::TextEdit::multiline(&mut ui_state.input3).desired_width(f32::INFINITY));
            ui.add_space(space_between_boxes);

            ui.label("Description");
            ui.add(egui::TextEdit::multiline(&mut ui_state.input4).desired_width(f32::INFINITY));
            ui.add_space(space_between_boxes);

            ui.label("Flavor");
            ui.add(egui::TextEdit::multiline(&mut ui_state.input5).desired_width(f32::INFINITY));
            ui.add_space(space_between_boxes);

            ui.horizontal(|ui| {
                if ui.add(egui::Button::new(
                    RichText::new("Store")
                        .color(Color32::from_rgb(0, 0, 0))
                        .size(24.0))
                ).clicked() {
                    let new_item = Item::new(
                        ui_state.input1.clone(),
                        ui_state.input2.clone(),
                        ui_state.input3.clone(),
                        ui_state.input4.clone(),
                        ui_state.input5.clone(),
                    );

                    ui_state.input1 = String::new();
                    ui_state.input2 = String::new();
                    ui_state.input3 = String::new();
                    ui_state.input4 = String::new();
                    ui_state.input5 = String::new();

                    ui_state.push_stored_item(new_item);
                    close = true
                }

                if ui.add(egui::Button::new(
                    RichText::new("Reset")
                        .color(Color32::from_rgb(0, 0, 0))
                        .size(24.0))
                ).clicked() {
                    ui_state.input1 = String::new();
                    ui_state.input2 = String::new();
                    ui_state.input3 = String::new();
                    ui_state.input4 = String::new();
                    ui_state.input5 = String::new();
                }
            });
        });

    if close {
        false
    } else {
        open
    }
}
