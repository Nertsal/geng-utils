use geng::{
    prelude::{Deserialize, Serialize},
    Window,
};

/// A convenience type that combines [Key](geng::Key) and [MouseButton](geng::MouseButton).
/// Can often be used for specifying controls where only press/release plays a role.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventKey {
    Key(geng::Key),
    Mouse(geng::MouseButton),
}

impl From<&EventKey> for EventKey {
    fn from(value: &EventKey) -> Self {
        *value
    }
}

impl From<geng::Key> for EventKey {
    fn from(value: geng::Key) -> Self {
        Self::Key(value)
    }
}

impl From<&geng::Key> for EventKey {
    fn from(value: &geng::Key) -> Self {
        Self::Key(*value)
    }
}

impl From<geng::MouseButton> for EventKey {
    fn from(value: geng::MouseButton) -> Self {
        Self::Mouse(value)
    }
}

impl From<&geng::MouseButton> for EventKey {
    fn from(value: &geng::MouseButton) -> Self {
        Self::Mouse(*value)
    }
}

impl EventKey {
    /// Check whether the key is pressed.
    pub fn is_pressed(self, window: &geng::Window) -> bool {
        match self {
            Self::Key(key) => window.is_key_pressed(key),
            Self::Mouse(button) => window.is_button_pressed(button),
        }
    }

    /// Check whether the event corresponds to the press of the key.
    pub fn is_event_press(self, event: &geng::Event) -> bool {
        match (&self, event) {
            (Self::Key(self_key), geng::Event::KeyPress { key }) => self_key == key,
            (Self::Mouse(self_button), geng::Event::MousePress { button, .. }) => {
                self_button == button
            }
            _ => false,
        }
    }

    /// Check whether the event corresponds to the release of the key.
    pub fn is_event_release(self, event: &geng::Event) -> bool {
        match (&self, event) {
            (Self::Key(self_key), geng::Event::KeyRelease { key }) => self_key == key,
            (Self::Mouse(self_button), geng::Event::MouseRelease { button, .. }) => {
                self_button == button
            }
            _ => false,
        }
    }
}

/// Check whether at least one of the keys is pressed.
pub fn is_key_pressed(
    window: &Window,
    keys: impl IntoIterator<Item = impl Into<EventKey>>,
) -> bool {
    keys.into_iter()
        .any(|key| Into::<EventKey>::into(key).is_pressed(window))
}

/// Check whether the event corresponds to the press of at least one of the keys.
pub fn is_event_press(
    event: &geng::Event,
    keys: impl IntoIterator<Item = impl Into<EventKey>>,
) -> bool {
    keys.into_iter()
        .any(|key| Into::<EventKey>::into(key).is_event_press(event))
}

/// Check whether the event corresponds to the release of at least one of the keys.
pub fn is_event_release(
    event: &geng::Event,
    keys: impl IntoIterator<Item = impl Into<EventKey>>,
) -> bool {
    keys.into_iter()
        .any(|key| Into::<EventKey>::into(key).is_event_release(event))
}
