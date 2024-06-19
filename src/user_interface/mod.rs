// This module is the basis for the Rhok'zan graphical user interface
// code pretaining to command line execution can be found in main.

use eframe::egui::{
    self,
    Sense,
    Key,
    epaint::Hsva
};
use egui::{vec2, Stroke, TextEdit};
use crate::sim::circuit::operation::Operation;

/// start gui
pub fn start_gui() -> Result<(), eframe::Error> {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size((480.0, 360.0)),
        ..Default::default()
    };

    eframe::run_native(
        "Rhok'zan",
        opts, 
        Box::new(|cc| {
            Box::<RzGui>::default()
        })
    ).unwrap();

    return Result::Ok(());
}

enum RzPannel {
    Edit{
        source_built: bool
    },
    Sim{

    }, 
}

impl Default for RzPannel {
    fn default() -> Self {
        Self::Edit { source_built: false }
    }
}

enum RzViewer {
    Gld(Box<[Operation]>),
    Rtl(),
    None
}
impl Default for RzViewer {
    fn default() -> Self {Self::None}
}

struct RzGui {
    ui_pannel: RzPannel,
    graph: RzViewer,
    source: String,
    cmd: String,
    cmd_interupt: bool,
    show_side_pannel: bool,
}

impl Default for RzGui {
    fn default() -> Self {
        Self {   
            ui_pannel: RzPannel::default(),
            graph: RzViewer::default(),
            source: "".to_string(),
            cmd: "".to_string(),
            cmd_interupt: false,
            show_side_pannel: false
        }
    }
}

impl eframe::App for RzGui {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        match self.cmd_interupt {
            true => {

            },
            false => {
                egui::CentralPanel::default().show(ctx, |ui|{
                    match self.ui_pannel {
                        RzPannel::Edit { mut source_built } => {
                            ui.vertical_centered_justified(|edit_ui| {
                                edit_ui.heading("Rhok'zan Editor");
                                edit_ui.horizontal_centered(|editor| {
                                    let padding = vec2(12.0, 20.0);
                                    let (text, graph) = editor.clip_rect().shrink2(padding).split_left_right_at_fraction(0.33);
                                    if editor.add_sized(
                                        (text.width(), text.height()), 
                                        TextEdit::multiline(&mut self.source)
                                            .code_editor()
                                            .clip_text(false)
                                            .desired_width(text.width())
                                    ).changed() {
                                        source_built = false;
                                    }

                                    let graph_render = editor.allocate_painter(graph.size(), Sense::hover()).1;
                                    graph_render.rect_stroke(graph_render.clip_rect(), 1.0, Stroke::new(3.0, Hsva::new(0.0, 0.0, 1.0, 0.1)));
                                });
                            });
                        }
                        RzPannel::Sim {  } => {
                            ui.horizontal_centered(|ui| ui.heading("Rhok'zan Simulator"));
                            todo!()
                        }
                    }

                });

                // keyboard controll for side pannel visibility
                if ctx.input(|kb| kb.key_pressed(Key::Escape)) {
                    self.show_side_pannel = !self.show_side_pannel;
                }
                egui::SidePanel::left("Menu").default_width(160.0).show_animated(ctx,
                    self.show_side_pannel, 
                    |menu| {
                        menu.vertical_centered_justified(|menu| {
                            menu.heading("Rhok'zan Menu");
                            if menu.button("HELP").clicked() {
                                todo!("display help")
                            }
                            if menu.button("SAVE").clicked() {
                                todo!("save source")
                            }
                            if menu.button("LOAD").clicked() {
                                todo!("load source")
                            }
                            match self.ui_pannel {
                                RzPannel::Edit {source_built} => {
                                    if menu.button("SIM").clicked() {
                                        todo!("build graph source if needed, and enter sim enviroment")
                                    }
                                    if menu.button("BUILD").clicked() {
                                        todo!("call interpreter on source, and set source built flag")
                                    }
                                },
                                RzPannel::Sim { }=> {
                                    if menu.button("EDIT").clicked() {
                                        todo!("enter edit environment")
                                    }
                                },
                            }
                        });
                    }
                );
            }
        }

    }

/*
    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {

    }

    fn save(&mut self, _storage: &mut dyn eframe::Storage) {

    }

    fn auto_save_interval(&self) -> std::time::Duration {
   
    }
*/
}

#[cfg(test)]
mod test {
    use super::start_gui;

    #[test]
    fn dummyui() {
        start_gui().expect("ui failed to start or crashed");
    }
}