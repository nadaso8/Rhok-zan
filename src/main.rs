mod back_end;
mod front_end;
mod middle_end;
/// for now we just no questions asked run the gui
/// eventualy I will make a proper cli and possibly
/// tui interface
fn main() -> std::io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = front_end::Tui::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
