use interface::gui::gui_main;

mod interface;
mod sim;
mod synth;

/// for now we just no questions asked run the gui
/// eventualy I will make a proper cli and possibly
/// tui interface
fn main() {
    gui_main().unwrap();
}
