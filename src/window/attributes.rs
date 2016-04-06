use xcb;
use super::{MapState, WindowRef};

#[derive(Debug,Eq,PartialEq)]
pub struct Attributes {
    pub map_state: MapState,
    pub override_redirect: bool,
}

impl Attributes {
    pub fn from<'a>(window: &WindowRef<'a>) -> Result<Attributes, xcb::GenericError> {
        let attrs = xcb::get_window_attributes(window.conn.as_xcb(), window.id());
        let attrs = try!(attrs.get_reply());
        Ok(Attributes::from_xcb(attrs))
    }

    fn from_xcb(attrs: xcb::GetWindowAttributesReply) -> Attributes {
        Attributes {
            map_state: MapState::from_xcb(attrs.map_state() as u32),
            override_redirect: attrs.override_redirect()
        }
    }
}
