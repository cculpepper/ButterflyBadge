use butterfly_gui::ButterflyApp;

use eframe::egui::{Pos2, Vec2};

fn main() {
    let options = eframe::NativeOptions {
        resizable: true,
        initial_window_size: Some(Vec2 { x: 1200., y: 1000. }),
        initial_window_pos: Some(Pos2 { x: 0., y: 0. }),
        ..eframe::NativeOptions::default()
    };

    eframe::run_native(
        "Butterfly Visualization",
        options,
        Box::new(|_cc| Box::new(ButterflyApp::default())),
    ).unwrap();
}
