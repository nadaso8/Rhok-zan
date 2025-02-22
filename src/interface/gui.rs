// This module is the basis for the Rhok'zan graphical user interface
// code pretaining to command line execution can be found in main.

use crate::sim::*;
use circuit::operation::SignalID;
use eframe::egui::{self, lerp};
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
    prompt: String,                                         // new commands from the user
    autofocus_prompt: bool,                                 // automattically assign focus to prompt
    sim_stack: Vec<(String, circuit::operation::SignalID)>, // a stack of handles to previously allocated nodes
    sim_graph: circuit::builder::Module,                    // current hardware description
    src_txt: String,                                        // WSV list of builder commands
    viewer_state: ViewerState,                              // what to show in the main viewer
    warnings: Vec<(String, f32)>,
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
                    if warning.1 < 0.05 {
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
                for handle in &self.sim_stack {
                    ui.label(format!(" - {} {}", handle.1 .0, handle.0));
                }

                ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                    ui.add_space(5.0);
                    let prompt_line = ui.add(
                        egui::widgets::text_edit::TextEdit::multiline(&mut self.prompt)
                            .desired_rows(1)
                            .return_key(None),
                    );
                    if prompt_line.has_focus() && ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
                        match parse_cmd(&self.prompt) {
                            Ok((_, cmd)) => match cmd {
                                CMD::Allocate { name } => {
                                    let handle = self.sim_graph.rz_alloc();
                                    self.sim_stack.push((name, handle));
                                    self.prompt = "".to_string();
                                }
                                CMD::DefineNot { val } => {
                                    if let Some((_name, handle)) = self.sim_stack.pop() {
                                        if let Result::Err(err) =
                                            self.sim_graph.mk_not(handle, SignalID(val))
                                        {
                                            self.warnings.push((err, 1.0));
                                        }
                                    } else {
                                        self.warnings.push((
                                            "Empty Stack: Allocate a location?".to_string(),
                                            1.0,
                                        ));
                                    }
                                }
                                CMD::DefineAnd { lhs, rhs } => {
                                    if let Some((_name, handle)) = self.sim_stack.pop() {
                                        if let Result::Err(err) = self.sim_graph.mk_and(
                                            handle,
                                            SignalID(lhs),
                                            SignalID(rhs),
                                        ) {
                                            self.warnings.push((err, 1.0));
                                        }
                                    } else {
                                        self.warnings.push((
                                            "Empty Stack: Allocate a location?".to_string(),
                                            1.0,
                                        ));
                                    }
                                }
                                CMD::DefineOr { lhs, rhs } => {
                                    if let Some((_name, handle)) = self.sim_stack.pop() {
                                        if let Result::Err(err) = self.sim_graph.mk_or(
                                            handle,
                                            SignalID(lhs),
                                            SignalID(rhs),
                                        ) {
                                            self.warnings.push((err, 1.0));
                                        }
                                    } else {
                                        self.warnings.push((
                                            "Empty Stack: Allocate a location?".to_string(),
                                            1.0,
                                        ));
                                    }
                                }
                                CMD::DefineNand { lhs, rhs } => {
                                    if let Some((_name, handle)) = self.sim_stack.pop() {
                                        if let Result::Err(err) = self.sim_graph.mk_nand(
                                            handle,
                                            SignalID(lhs),
                                            SignalID(rhs),
                                        ) {
                                            self.warnings.push((err, 1.0));
                                        }
                                    } else {
                                        self.warnings.push((
                                            "Empty Stack: Allocate a location?".to_string(),
                                            1.0,
                                        ));
                                    }
                                }
                                CMD::DefineNor { lhs, rhs } => {
                                    if let Some((_name, handle)) = self.sim_stack.pop() {
                                        if let Result::Err(err) = self.sim_graph.mk_nor(
                                            handle,
                                            SignalID(lhs),
                                            SignalID(rhs),
                                        ) {
                                            self.warnings.push((err, 1.0));
                                        }
                                    } else {
                                        self.warnings.push((
                                            "Empty Stack: Allocate a location?".to_string(),
                                            1.0,
                                        ));
                                    }
                                }
                                CMD::DefineXor { lhs, rhs } => {
                                    if let Some((_name, handle)) = self.sim_stack.pop() {
                                        if let Result::Err(err) = self.sim_graph.mk_xor(
                                            handle,
                                            SignalID(lhs),
                                            SignalID(rhs),
                                        ) {
                                            self.warnings.push((err, 1.0));
                                        }
                                    } else {
                                        self.warnings.push((
                                            "Empty Stack: Allocate a location?".to_string(),
                                            1.0,
                                        ));
                                    }
                                }
                                CMD::DefineXnor { lhs, rhs } => {
                                    if let Some((_name, handle)) = self.sim_stack.pop() {
                                        if let Result::Err(err) = self.sim_graph.mk_xnor(
                                            handle,
                                            SignalID(lhs),
                                            SignalID(rhs),
                                        ) {
                                            self.warnings.push((err, 1.0));
                                        }
                                    } else {
                                        self.warnings.push((
                                            "Empty Stack: Allocate a location?".to_string(),
                                            1.0,
                                        ));
                                    }
                                }
                                _ => todo!("unhandled CMD type"),
                            },
                            Err(err) => self
                                .warnings
                                .push((format!("Invalid Command: {}", err), 1.0)),
                        }
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
            sim_stack: Vec::new(),
            sim_graph: circuit::builder::Module::new(),
            src_txt: String::new(),
            viewer_state: ViewerState::Graph,
            warnings: Vec::new(),
        }
    }
}

mod tiny_parse {
    use nom::{
        branch::*,
        bytes::complete::tag,
        character::complete::{alpha1, multispace0, multispace1, usize},
        sequence::{delimited, pair, preceded, tuple},
        Err, IResult, Parser,
    };
    pub enum CMD {
        // Graph manipulaiton
        Allocate { name: String },
        DefineInput { name: String, pattern: usize },
        DefineOutput { name: String, val: usize },
        DefineNot { val: usize },
        DefineAnd { lhs: usize, rhs: usize },
        DefineOr { lhs: usize, rhs: usize },
        DefineNand { lhs: usize, rhs: usize },
        DefineNor { lhs: usize, rhs: usize },
        DefineXor { lhs: usize, rhs: usize },
        DefineXnor { lhs: usize, rhs: usize },

        // utility
        Save { file_name: String },
        Load { file_name: String },
        Compile,
        Run,

        // navigation
        Graph,
        Src,
        Test,
    }

    pub fn parse_cmd(cmd_txt: &str) -> IResult<&str, CMD> {
        alt((
            parse_alloc,
            parse_not,
            parse_and,
            parse_or,
            parse_nand,
            parse_nor,
            parse_xor,
            parse_xnor,
        ))
        .parse(cmd_txt)
    }

    /// produces a valid allocate cmd paired with the remaining unmatched text on match
    fn parse_alloc(i: &str) -> IResult<&str, CMD> {
        match preceded(pair(multispace0, tag("alloc")), parse_name).parse(i) {
            Ok((remainder, name)) => Ok((
                remainder,
                CMD::Allocate {
                    name: name.to_string(),
                },
            )),
            Err(err) => IResult::Err(err),
        }
    }

    fn parse_input(i: &str) -> IResult<&str, CMD> {
        todo!()
    }

    fn parse_output(i: &str) -> IResult<&str, CMD> {
        todo!()
    }

    fn parse_not(i: &str) -> IResult<&str, CMD> {
        match preceded(pair(multispace0, tag("not")), preceded(multispace1, usize)).parse(i) {
            Ok((remainder, val)) => Ok((remainder, CMD::DefineNot { val })),
            Err(err) => IResult::Err(err),
        }
    }

    fn parse_and(i: &str) -> IResult<&str, CMD> {
        match preceded(
            pair(multispace0, tag("and")),
            pair(preceded(multispace1, usize), preceded(multispace1, usize)),
        )
        .parse(i)
        {
            Ok((remainder, (lhs, rhs))) => Ok((remainder, CMD::DefineAnd { lhs, rhs })),
            Err(err) => IResult::Err(err),
        }
    }

    fn parse_or(i: &str) -> IResult<&str, CMD> {
        match preceded(
            pair(multispace0, tag("or")),
            pair(preceded(multispace1, usize), preceded(multispace1, usize)),
        )
        .parse(i)
        {
            Ok((remainder, (lhs, rhs))) => Ok((remainder, CMD::DefineOr { lhs, rhs })),
            Err(err) => IResult::Err(err),
        }
    }

    fn parse_nand(i: &str) -> IResult<&str, CMD> {
        match preceded(
            pair(multispace0, tag("nand")),
            pair(preceded(multispace1, usize), preceded(multispace1, usize)),
        )
        .parse(i)
        {
            Ok((remainder, (lhs, rhs))) => Ok((remainder, CMD::DefineNand { lhs, rhs })),
            Err(err) => IResult::Err(err),
        }
    }

    fn parse_nor(i: &str) -> IResult<&str, CMD> {
        match preceded(
            pair(multispace0, tag("nor")),
            pair(preceded(multispace1, usize), preceded(multispace1, usize)),
        )
        .parse(i)
        {
            Ok((remainder, (lhs, rhs))) => Ok((remainder, CMD::DefineNor { lhs, rhs })),
            Err(err) => IResult::Err(err),
        }
    }

    fn parse_xor(i: &str) -> IResult<&str, CMD> {
        match preceded(
            pair(multispace0, tag("xor")),
            pair(preceded(multispace1, usize), preceded(multispace1, usize)),
        )
        .parse(i)
        {
            Ok((remainder, (lhs, rhs))) => Ok((remainder, CMD::DefineXor { lhs, rhs })),
            Err(err) => IResult::Err(err),
        }
    }

    fn parse_xnor(i: &str) -> IResult<&str, CMD> {
        match preceded(
            pair(multispace0, tag("xnor")),
            pair(preceded(multispace1, usize), preceded(multispace1, usize)),
        )
        .parse(i)
        {
            Ok((remainder, (lhs, rhs))) => Ok((remainder, CMD::DefineXnor { lhs, rhs })),
            Err(err) => IResult::Err(err),
        }
    }

    fn parse_save(i: &str) -> IResult<&str, CMD> {
        todo!()
    }

    fn parse_load(i: &str) -> IResult<&str, CMD> {
        todo!()
    }

    fn parse_compile(i: &str) -> IResult<&str, CMD> {
        todo!()
    }

    fn parse_run(i: &str) -> IResult<&str, CMD> {
        todo!()
    }

    fn parse_graph(i: &str) -> IResult<&str, CMD> {
        todo!()
    }

    fn parse_src(i: &str) -> IResult<&str, CMD> {
        todo!()
    }

    fn parse_test(i: &str) -> IResult<&str, CMD> {
        todo!()
    }

    /// a series of alpha characters delimited by whitespace
    /// may be an empty string
    fn parse_name(i: &str) -> IResult<&str, &str> {
        alt((delimited(multispace1, alpha1, multispace0), multispace0)).parse(i)
    }
}
