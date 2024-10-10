use std::usize;

// This module is the basis for the Rhok'zan graphical user interface
// code pretaining to command line execution can be found in main.
use eframe::{
    egui::{self, epaint::Hsva, vec2, Key, Sense, Stroke, TextEdit},
    glow::FALSE,
};
use egui::Button;

use crate::main;

/// start gui
pub fn start_gui() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_maximized(true),
        ..Default::default()
    };

    eframe::run_native(
        "Rhok'zan",
        options,
        Box::new(|cc| Box::<RzRuntime>::default()),
    )?;

    return Result::Ok(());
}

/// UI data which persists over multiple frames and cannot be computed each frame must go here.
struct RzRuntime {
    command: String,
    current_pannel: PannelOptions,
    gld: Box<[crate::sim::circuit::operation::Operation]>,
}

/// Default values for UI data must go here
impl Default for RzRuntime {
    fn default() -> Self {
        use crate::sim::circuit::{operation::*, signal::*};
        const TPI: usize = 8;
        let default_gld = Box::new([
            Operation::Input(InputHandler {
                handler: Box::new(|index, tick| match (tick / (TPI as u128 * 2)) % (2) {
                    0 => Signal::False,
                    _ => Signal::True,
                }),
            }),
            Operation::Input(InputHandler {
                handler: Box::new(|index, tick| match (tick / (TPI as u128 * 4)) % (2) {
                    0 => Signal::False,
                    _ => Signal::True,
                }),
            }),
            Operation::Nor(SignalID(0), SignalID(3)),
            Operation::Nor(SignalID(1), SignalID(2)),
            Operation::Output(
                SignalID(2),
                OutputHandler {
                    handler: Box::new(|index, tick, signal| {
                        if tick % TPI as u128 == 0 {
                            println!("Index: {} is {} on Tick: {}", index, signal, tick)
                        };
                        return;
                    }),
                },
            ),
            Operation::Output(
                SignalID(3),
                OutputHandler {
                    handler: Box::new(|index, tick, signal| {
                        if tick % TPI as u128 == 0 {
                            println!("Index: {} is {} on Tick: {}", index, signal, tick)
                        };
                        return;
                    }),
                },
            ),
        ]);
        let starting_pannel = PannelOptions::Simulate;

        Self {
            command: "".to_string(),
            current_pannel: starting_pannel,
            gld: default_gld,
        }
    }
}

impl eframe::App for RzRuntime {
    /// Code for immediate mode redering of the UI should go here.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Main ui components
        egui::CentralPanel::default().show(ctx, |main_view| {
            // pannel select menu
            main_view.horizontal_top(|pannel_select| {
                for pannel in [PannelOptions::Design, PannelOptions::Simulate] {
                    let pannel_name = &str::to_uppercase((&pannel).into());
                    if pannel == self.current_pannel {
                        self.current_pannel = match pannel_select
                            .label(pannel_name)
                            .highlight()
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            true => pannel,
                            false => self.current_pannel,
                        }
                    } else {
                        self.current_pannel = match pannel_select
                            .label(pannel_name)
                            .on_hover_cursor(egui::CursorIcon::PointingHand)
                            .clicked()
                        {
                            true => pannel,
                            false => self.current_pannel,
                        }
                    }
                }
            });

            match self.current_pannel {
                PannelOptions::Design => main_view.label("this area is under construction"),
                PannelOptions::Simulate => main_view.label("boop"),
            }
        });
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PannelOptions {
    Design,
    Simulate,
}

impl From<&PannelOptions> for &str {
    fn from(value: &PannelOptions) -> Self {
        match value {
            PannelOptions::Design => "design",
            PannelOptions::Simulate => "simulate",
        }
    }
}
