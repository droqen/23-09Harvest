use ambient_api::prelude::*;

#[main]
pub fn main() {
    ambient_api::core::messages::Frame::subscribe(|_| {
        let (delta, input) = input::get_delta();
        if delta.keys.contains(&KeyCode::Q) {
            println!("Q - on");
        }
        if delta.keys_released.contains(&KeyCode::Q) {
            println!("Q - off");
        }
    });
}
