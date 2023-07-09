use geng::prelude::*;
use geng_utils::key::{self as key_utils, EventKey};

const KEYS_JUMP: [EventKey; 2] = [
    EventKey::Key(geng::Key::Space),
    EventKey::Mouse(geng::MouseButton::Left),
];

fn main() {
    logger::init();
    geng::setup_panic_handler();
    Geng::run("EventKey example", |geng| async move {
        let mut events = geng.window().events();
        while let Some(event) = events.next().await {
            if key_utils::is_event_press(&event, KEYS_JUMP) {
                log::info!("Pressed jump key");
            }
            if key_utils::is_event_release(&event, KEYS_JUMP) {
                log::info!("Released jump key");
            }
        }
    })
}
