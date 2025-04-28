// This module is the basis for the Rhok'zan graphical user interface
// code pretaining to command line execution can be found in main.
mod tiny_parse;
pub mod vizualizer;
use std::io::{Read, Write};

use crate::sim::*;
use circuit::{operation::SignalID, signal::Signal};
use eframe::egui::{self, lerp};
use nom::Parser;
use tiny_parse::*;

/// start gui
pub fn gui_main() -> Result<(), eframe::Error> {
    let opts = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default(),
        ..Default::default()
    };

    eframe::run_native("Rhokzan", opts, Box::new(|cc| Ok(Box::<GUI>::default())))?;

    return Result::Ok(());
}

#[derive(Debug)]
enum ViewerState {
    Graph,
    IOWaveforms,
    Src,
}

/// Stores the necesarry state for our UI interface
#[derive(Debug)]
struct GUI {
    //Interface related
    autofocus_prompt: bool,       // automattically assign focus to prompt
    prompt: String,               // new commands from the user
    warnings: Vec<(String, f32)>, // list of warnings to display

    // Design related
    module_stack: Vec<(String, circuit::operation::SignalID)>, // a stack of handles to previously allocated nodes
    module_desc: circuit::builder::Module,                     // current hardware description
    src_txt: String,                                           // WSV list of builder commands

    // vizualization related
    viewer_state: ViewerState, // what to show in the main viewer
}

impl eframe::App for GUI {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::TopBottomPanel::bottom("warn").show_animated(
            ctx,
            (!self.warnings.is_empty()),
            |ui| {
                let mut gc = 0;
                for warning in &mut self.warnings {
                    let color = egui::Color32::WHITE.gamma_multiply(warning.1);
                    ui.colored_label(color, warning.0.clone());
                    warning.1 = lerp(warning.1..=0.0, 0.02);
                    if warning.1 < 0.1 {
                        gc += 1;
                    }
                }
                if gc > 0 {
                    self.warnings.drain(0..=gc - 1);
                }
            },
        );

        egui::SidePanel::left("cmd").show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.heading("STACK");
                for handle in &self.module_stack {
                    ui.label(format!(" - {} {}", handle.1 .0, handle.0));
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(5.0);
                    let prompt_line = ui.add(
                        egui::widgets::text_edit::TextEdit::multiline(&mut self.prompt)
                            .desired_rows(1)
                            .return_key(None)
                            .frame(false),
                    );
                    if prompt_line.has_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                        self.handle_cmd();
                        self.autofocus_prompt = true; // set to true so prompt re-aquires focus
                    } else if !prompt_line.has_focus() && self.autofocus_prompt {
                        prompt_line.request_focus();
                        self.autofocus_prompt = false; // set to false so tab navigation is not broken
                    }

                    ui.separator();
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| match &self.viewer_state {
            ViewerState::Src => {
                ui.add_sized(
                    ui.available_size(),
                    egui::TextEdit::multiline(&mut self.src_txt),
                );
            }
            ViewerState::Graph => {}
            ViewerState::IOWaveforms => {}
        });
    }
}

impl Default for GUI {
    fn default() -> Self {
        Self {
            prompt: String::new(),
            autofocus_prompt: true,
            module_stack: Vec::new(),
            module_desc: circuit::builder::Module::new(),
            src_txt: String::new(),
            viewer_state: ViewerState::Graph,
            warnings: Vec::new(),
        }
    }
}

impl GUI {
    /// Tries to parse a command from prompt and handles the command if one is parsed
    fn handle_cmd(&mut self) {
        let err_empty_stack = "Empty Stack: Allocate a location?"; // msg when user tries to pop from an empty stack

        match parse_cmd(&self.prompt) {
            Ok((_, cmd)) => {
                match cmd {
                    CMD::Allocate { name } => {
                        let handle = self.module_desc.rz_alloc();
                        self.module_stack.push((name, handle));
                        self.prompt = "".to_string();
                    }
                    CMD::DefineNot { val } => {
                        if let Some((_name, handle)) = self.module_stack.pop() {
                            if let Result::Err(err) = self.module_desc.mk_not(handle, SignalID(val))
                            {
                                self.warnings.push((err, 1.0));
                            }
                        } else {
                            self.warnings.push((err_empty_stack.to_string(), 1.0));
                        }
                    }
                    CMD::DefineAnd { lhs, rhs } => {
                        if let Some((_name, handle)) = self.module_stack.pop() {
                            if let Result::Err(err) =
                                self.module_desc
                                    .mk_and(handle, SignalID(lhs), SignalID(rhs))
                            {
                                self.warnings.push((err, 1.0));
                            }
                        } else {
                            self.warnings.push((err_empty_stack.to_string(), 1.0));
                        }
                    }
                    CMD::DefineOr { lhs, rhs } => {
                        if let Some((_name, handle)) = self.module_stack.pop() {
                            if let Result::Err(err) =
                                self.module_desc.mk_or(handle, SignalID(lhs), SignalID(rhs))
                            {
                                self.warnings.push((err, 1.0));
                            }
                        } else {
                            self.warnings.push((err_empty_stack.to_string(), 1.0));
                        }
                    }
                    CMD::DefineNand { lhs, rhs } => {
                        if let Some((_name, handle)) = self.module_stack.pop() {
                            if let Result::Err(err) =
                                self.module_desc
                                    .mk_nand(handle, SignalID(lhs), SignalID(rhs))
                            {
                                self.warnings.push((err, 1.0));
                            }
                        } else {
                            self.warnings.push((err_empty_stack.to_string(), 1.0));
                        }
                    }
                    CMD::DefineNor { lhs, rhs } => {
                        if let Some((_name, handle)) = self.module_stack.pop() {
                            if let Result::Err(err) =
                                self.module_desc
                                    .mk_nor(handle, SignalID(lhs), SignalID(rhs))
                            {
                                self.warnings.push((err, 1.0));
                            }
                        } else {
                            self.warnings.push((err_empty_stack.to_string(), 1.0));
                        }
                    }
                    CMD::DefineXor { lhs, rhs } => {
                        if let Some((_name, handle)) = self.module_stack.pop() {
                            if let Result::Err(err) =
                                self.module_desc
                                    .mk_xor(handle, SignalID(lhs), SignalID(rhs))
                            {
                                self.warnings.push((err, 1.0));
                            }
                        } else {
                            self.warnings.push((err_empty_stack.to_string(), 1.0));
                        }
                    }
                    CMD::DefineXnor { lhs, rhs } => {
                        if let Some((_name, handle)) = self.module_stack.pop() {
                            if let Result::Err(err) =
                                self.module_desc
                                    .mk_xnor(handle, SignalID(lhs), SignalID(rhs))
                            {
                                self.warnings.push((err, 1.0));
                            }
                        } else {
                            self.warnings.push((err_empty_stack.to_string(), 1.0));
                        }
                    }
                    CMD::Save { file_name } => match std::fs::File::create_new(file_name) {
                        Ok(mut file) => match file.write(self.src_txt.as_bytes()) {
                            Err(err) => self.warnings.push((format!("{err}"), 1.0)),
                            _ => (),
                        },
                        Err(err) => self.warnings.push((format!("{err}"), 1.0)),
                    },
                    CMD::Load { file_name } => match std::fs::File::open(file_name) {
                        Ok(mut file) => match file.read_to_string(&mut self.src_txt) {
                            Err(err) => self.warnings.push((format!("{err}"), 1.0)),
                            _ => (),
                        },
                        Err(err) => self.warnings.push((format!("{err}"), 1.0)),
                    },
                    CMD::Compile => {
                        self.compile_src();
                    }
                    CMD::Src => {
                        self.viewer_state = ViewerState::Src;
                    }
                    CMD::Graph => {
                        self.viewer_state = ViewerState::Graph;
                    }
                    CMD::Test => {
                        self.viewer_state = ViewerState::IOWaveforms;
                    }
                    _ => todo!("unhandled CMD type"),
                };
                self.prompt = "".to_string();
            }
            Err(err) => self
                .warnings
                .push((format!("Invalid Command: {}", err), 1.0)),
        }
    }

    /// handles a series of commands related to graph construction warning the user if any invalid command types are found
    fn compile_src(&mut self) {
        self.module_desc = crate::sim::circuit::builder::Module::new(); // clear existing module before compilation.
        let err_empty_stack = "Empty Stack: Allocate a location?"; // msg when user tries to pop from an empty stack

        let cmds =
            match nom::multi::separated_list0(nom::character::complete::line_ending, parse_cmd)
                .parse(&self.src_txt)
            {
                Ok((r, cmds)) => cmds,
                Err(err) => {
                    self.warnings.push((format!("{err}"), 1.0)); // I might not be properly outputting parse errors here since I seem to be getting a silent fail
                    return; // early return since cmds was not properly produced
                }
            };
        for cmd in cmds {
            match cmd {
                CMD::Allocate { name } => {
                    let handle = self.module_desc.rz_alloc();
                    self.module_stack.push((name, handle));
                    self.prompt = "".to_string();
                }
                CMD::DefineInput { name, pattern } => {
                    let expr = |id, tick| -> Signal {
                        // need to decide on how I visualize output from the simulation first
                        todo!()
                    };
                    if let Some((_name, handle)) = self.module_stack.pop() {
                        if let Result::Err(err) = self.module_desc.mk_input(handle, expr) {
                            self.warnings.push((err, 1.0));
                        }
                    } else {
                        self.warnings.push((err_empty_stack.to_string(), 1.0));
                    }
                }
                CMD::DefineOutput { name, val } => {
                    let expr = |id, tick, signal| {
                        // need to decide on how I visualize output from the simulation first
                        todo!()
                    };

                    if let Some((_name, handle)) = self.module_stack.pop() {
                        if let Result::Err(err) =
                            self.module_desc.mk_output(handle, SignalID(val), expr)
                        {
                            self.warnings.push((err, 1.0));
                        }
                    } else {
                        self.warnings.push((err_empty_stack.to_string(), 1.0));
                    }
                }
                CMD::DefineNot { val } => {
                    if let Some((_name, handle)) = self.module_stack.pop() {
                        if let Result::Err(err) = self.module_desc.mk_not(handle, SignalID(val)) {
                            self.warnings.push((err, 1.0));
                        }
                    } else {
                        self.warnings.push((err_empty_stack.to_string(), 1.0));
                    }
                }
                CMD::DefineAnd { lhs, rhs } => {
                    if let Some((_name, handle)) = self.module_stack.pop() {
                        if let Result::Err(err) =
                            self.module_desc
                                .mk_and(handle, SignalID(lhs), SignalID(rhs))
                        {
                            self.warnings.push((err, 1.0));
                        }
                    } else {
                        self.warnings.push((err_empty_stack.to_string(), 1.0));
                    }
                }
                CMD::DefineOr { lhs, rhs } => {
                    if let Some((_name, handle)) = self.module_stack.pop() {
                        if let Result::Err(err) =
                            self.module_desc.mk_or(handle, SignalID(lhs), SignalID(rhs))
                        {
                            self.warnings.push((err, 1.0));
                        }
                    } else {
                        self.warnings.push((err_empty_stack.to_string(), 1.0));
                    }
                }
                CMD::DefineNand { lhs, rhs } => {
                    if let Some((_name, handle)) = self.module_stack.pop() {
                        if let Result::Err(err) =
                            self.module_desc
                                .mk_nand(handle, SignalID(lhs), SignalID(rhs))
                        {
                            self.warnings.push((err, 1.0));
                        }
                    } else {
                        self.warnings.push((err_empty_stack.to_string(), 1.0));
                    }
                }
                CMD::DefineNor { lhs, rhs } => {
                    if let Some((_name, handle)) = self.module_stack.pop() {
                        if let Result::Err(err) =
                            self.module_desc
                                .mk_nor(handle, SignalID(lhs), SignalID(rhs))
                        {
                            self.warnings.push((err, 1.0));
                        }
                    } else {
                        self.warnings.push((err_empty_stack.to_string(), 1.0));
                    }
                }
                CMD::DefineXor { lhs, rhs } => {
                    if let Some((_name, handle)) = self.module_stack.pop() {
                        if let Result::Err(err) =
                            self.module_desc
                                .mk_xor(handle, SignalID(lhs), SignalID(rhs))
                        {
                            self.warnings.push((err, 1.0));
                        }
                    } else {
                        self.warnings.push((err_empty_stack.to_string(), 1.0));
                    }
                }
                CMD::DefineXnor { lhs, rhs } => {
                    if let Some((_name, handle)) = self.module_stack.pop() {
                        if let Result::Err(err) =
                            self.module_desc
                                .mk_xnor(handle, SignalID(lhs), SignalID(rhs))
                        {
                            self.warnings.push((err, 1.0));
                        }
                    } else {
                        self.warnings.push((err_empty_stack.to_string(), 1.0));
                    }
                }
                CMD::Save { .. } => self
                    .warnings
                    .push((format!("Ignored Invalid Command: save"), 1.0)),

                CMD::Load { .. } => self
                    .warnings
                    .push((format!("Ignored Invalid Command: load"), 1.0)),
                CMD::Src => self
                    .warnings
                    .push((format!("Ignored Invalid Command: src"), 1.0)),
                CMD::Graph => self
                    .warnings
                    .push((format!("Ignored Invalid Command: graph"), 1.0)),
                CMD::Test => self
                    .warnings
                    .push((format!("Ignored Invalid Command: test"), 1.0)),
                _ => todo!("unhandled CMD type"),
            };
            self.prompt = "".to_string();
        }
    }
}
