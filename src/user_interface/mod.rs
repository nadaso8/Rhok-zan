// This module is the basis for the Rhok'zan graphical user interface
// code pretaining to command line execution can be found in main.

use std::default;

use eframe::egui;

use crate::sim::circuit::operation::Operation;

/// start gui
fn start_gui() -> Result<(), eframe::Error> {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((320.0, 240.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Rhok'zan",
        opts, 
        Box::new(|cc| {
            
        })
    );

    return Result::Ok(());
}

struct rz_gui {
    graph: Box<[Operation]>,
    source: String
}

impl Default for rz_gui {
    fn default() -> Self {
        Self{ 
            graph: Box::new([]),
            source: "".to_string()
        }
    }
}