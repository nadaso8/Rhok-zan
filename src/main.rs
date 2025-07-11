mod back_end;
mod front_end;
mod middle_end;

use front_end::gui_main;

/// for now we just no questions asked run the gui
/// eventualy I will make a proper cli and possibly
/// tui interface
fn main() {
    gui_main().unwrap();
}
