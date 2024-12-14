#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;
use egui::Color32;
use egui::Style;
use egui::Theme;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([750.0, 450.0]),
        ..Default::default()
    };
    eframe::run_native(
        "rat",
        options,
        Box::new(|cc| {
            setup_custom_style(&cc.egui_ctx);
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Ok(Box::<MyApp>::default())
        }),
    )
}

#[derive(Default)]
struct MyApp {}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.image(egui::include_image!("rat.png"));
        });
    }
}

fn setup_custom_style(ctx: &egui::Context) {
    ctx.style_mut_of(Theme::Light, white_bg);
    ctx.style_mut_of(Theme::Dark, white_bg);
}

fn white_bg(style: &mut Style) {
    style.visuals.panel_fill = Color32::WHITE;
}
