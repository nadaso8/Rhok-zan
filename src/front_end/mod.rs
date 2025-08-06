// This module is the basis for the Rhok'zan graphical user interface
// code pretaining to command line execution can be found in main.
use eframe::egui;

/// Start the GUI
///
/// This function exists as an entrypoint to start the GUI
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
struct GUI {}

impl eframe::App for GUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        eframe::egui::CentralPanel::default().show(ctx, |main_pannel| {});
    }
}

impl Default for GUI {
    fn default() -> Self {
        Self {}
    }
}
