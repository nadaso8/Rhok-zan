use user_interface::start_gui;

mod user_interface;
mod interpreter;
mod sim;

fn main() -> Result<(),&'static str> {
    match start_gui(){
        Ok(_) => Result::Ok(()),
        Err(_) => Result::Err("ui failed to start or crashed during operation")
    }
}
