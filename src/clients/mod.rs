pub mod client_action;
pub mod client_state;
pub mod client_type;
pub mod util;

use xcb_util::ewmh;

use crate::clients::{
    client_action::ClientAction,
    client_state::ClientState,
    client_type::ClientType,
};

use self::util::ClientPadding;

/// Represents the ID of the client. Typically the `event.window()`, `event.child()` or
/// `event.event()` in XCB events.
pub type ClientID = u32;

#[derive(Clone)]
pub struct Client {
    /// Represents the ID of the client. Typically the `event.window()`, `event.child()` or
    /// `event.event()` in XCB events.
    pub wid: ClientID,

    /// The `_NET_WM_PID` of the client, also known as the process ID.
    pub wm_pid: Option<u32>,

    /// The `WM_CLASS` of the client.
    pub wm_class: Option<String>,

    pub padding: ClientPadding,

    is_controlled: bool,

    // TODO: docs
    r#type: ClientType,
    
    /// Represents the list of current `xcb::WM_STATE` atoms of the client.
    /// Each state must be unique in the vector.
    ///
    /// The importance of states is from last to first, as the latest pushed states
    /// are treated with more privileges. For example, a client with `states` equal to
    /// `[ClientState::Fullscreen, ClientState::Maximized]` must be drawn as maximized.
    /// When removing `ClientState::Maximized` from the list, the client must be drawn as fullscreen.
    ///
    /// Some functions that returns the state may sometimes return `ClientState::Tile`. This state
    /// is special and is never included in the list; it simply indicates that the client
    /// doesn't have any configured state.
    ///
    /// Refer to: https://specifications.freedesktop.org/wm-spec/wm-spec-1.3.html#idm46201142858672
    states: Vec<ClientState>,

    /// Represents the list of current `_NET_ALLOWED_ACTIONS` atoms of the client.
    /// Each action must be unique in the vector.
    ///
    /// Refer to: https://specifications.freedesktop.org/wm-spec/wm-spec-1.3.html#idm46201142837824
    allowed_actions: Vec<ClientAction>,
}

impl Client {
    pub fn new(wid: ClientID) -> Self {
        Client { 
            wid,
            is_controlled: true,
            padding: ClientPadding { top: 0, bottom: 0, left: 0, right: 0 },
            r#type: ClientType::Normal,
            states: vec![],
            allowed_actions: vec![],
            wm_class: None,
            wm_pid: None,
        }
    }

    /// Maps a window.
    pub fn map(&self, conn: &ewmh::Connection) {
        xcb::map_window(conn, self.wid);
    }

    /// Unmaps a window.
    pub fn unmap(&self, conn: &ewmh::Connection) {
        xcb::unmap_window(conn, self.wid);
    }

    /// Returns whether the client needs control.
    #[inline]
    pub fn is_controlled(&self) -> bool {
        self.is_controlled
    }

    pub fn set_border(&self, conn: &ewmh::Connection, color: u32) {
        xcb::change_window_attributes(
            conn,
            self.wid,
            &[(xcb::CW_BORDER_PIXEL, color)],
        );
    }

    pub fn set_input_focus(&self, conn: &ewmh::Connection) {
        xcb::set_input_focus(
            conn,
            xcb::INPUT_FOCUS_PARENT as u8,
            self.wid,
            xcb::CURRENT_TIME
        );
    }

    /// Sends a destroy notification to the window manager with the client's window ID.
    pub fn kill(&self, conn: &ewmh::Connection) {
        xcb::destroy_window(conn, self.wid);
    }

    /// TODO: rename it!
    pub fn enable_event_mask(&self, conn: &ewmh::Connection) {
        xcb::change_window_attributes(
            conn,
            self.wid,
            &[(
                xcb::CW_EVENT_MASK,
                xcb::EVENT_MASK_PROPERTY_CHANGE
                | xcb::EVENT_MASK_STRUCTURE_NOTIFY,
            )],
        );
    }
}
