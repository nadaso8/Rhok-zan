// This module is the basis for the Rhok'zan graphical user interface
// code pretaining to command line execution can be found in main.

use crate::sim::*;
use eframe::egui;

/// start gui
pub fn gui_main() -> Result<(), eframe::Error> {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native("Rhokzan", opts, Box::new(|cc| Ok(Box::<GUI>::default())))?;

    return Result::Ok(());
}

/// Stores the necesarry state for our UI interface
#[derive(Debug)]
struct GUI {
    design_under_test: egui_graphs::Graph,
}

enum WindowType {
    Schematic { module: usize },
    WaveformViewer {},
}

impl eframe::App for GUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        todo!()
    }
}

impl Default for GUI {
    fn default() -> Self {
        Self {}
    }
}
