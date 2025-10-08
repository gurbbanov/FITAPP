#![allow(warnings)]
use egui::{self};

mod app;
mod ui;
mod models;
mod muscles;
mod tools;

fn main() -> Result<(), eframe::Error> {
        let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_min_inner_size([600.0, 900.0])
            .with_max_inner_size([600.0, 1000.0])
            .with_fullscreen(false)
            .with_fullsize_content_view(false)
            .with_maximized(false)
            // .with_resizable(false)
            .with_maximize_button(false),
        ..Default::default()
    };

    let app = app::AppRuntime::new(&egui::Context::default());

    eframe::run_native("fitness app", options, Box::new(|cc| Ok(Box::new(app))))
}
