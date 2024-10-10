use user_interface::start_gui;

mod compiler;
mod sim;
mod user_interface;

fn main() -> Result<(), &'static str> {
    match start_gui() {
        Ok(_) => Result::Ok(()),
        Err(_) => Result::Err("ui failed to start or crashed during operation"),
    }
}
