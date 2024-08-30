use bevy::prelude::*;

#[allow(dead_code)]
#[derive(Event, Clone, Debug)]
pub enum WebEvent {
    Drop(String, Vec<u8>),
}

pub struct WebPlugin {
    #[allow(dead_code)]
    pub dom_drop_element_id: String,
}
impl Plugin for WebPlugin {
    fn build(&self, app: &mut App) {
        #[cfg(not(target_family = "wasm"))]
        {
            app.add_event::<WebEvent>();
        }

        #[cfg(target_family = "wasm")]
        {
            use bevy_crossbeam_event::{CrossbeamEventApp, CrossbeamEventSender};

            app.add_crossbeam_event::<WebEvent>();

            let sender = app
                .world()
                .get_resource::<CrossbeamEventSender<WebEvent>>()
                .unwrap()
                .clone();

            super::channel::set_sender(sender);

            super::web::register_drop(&self.dom_drop_element_id).unwrap();
        }
    }
}
