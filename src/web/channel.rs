use bevy_crossbeam_event::CrossbeamEventSender;
use std::sync::OnceLock;

use super::WebEvent;

static SENDER: OnceLock<Option<CrossbeamEventSender<WebEvent>>> = OnceLock::new();

//TODO: error logging
pub fn send_event(e: WebEvent) {
    SENDER
        .get()
        .expect("invalid sender lock")
        .as_ref()
        .expect("sender not found")
        .send(e);
}

pub fn set_sender(sender: CrossbeamEventSender<WebEvent>) {
    while SENDER.set(Some(sender.clone())).is_err() {}
}
