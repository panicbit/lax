use prelude::*;
use window::WindowIterator;
use xcb;
use monster::incubation::OwningRefMut;

pub struct Screen<'a> {
    conn: &'a Connection,
    xcb: OwningRefMut<xcb::Setup<'a>, xcb::Screen<'a>>,
    index: u8,
}

impl <'a> Screen<'a> {
    pub fn from(conn: &'a Connection, index: u8) -> Option<Screen<'a>> {
        let setup = conn.as_xcb().get_setup();

        if index >= setup.roots_len() {
            return None;
        }

        let screen = OwningRefMut::new(Box::new(setup), |setup| {
            setup.roots().nth(index as usize).expect("screen")
        });

        Some(Screen {
            conn: conn,
            xcb: screen,
            index: index,
        })
    }

    pub fn root_ref(&self) -> WindowRef<'a> {
        WindowRef::from(self.conn, self.xcb.root())
    }

    pub fn children_refs(&self) -> Result<WindowIterator<'a>, xcb::GenericError> {
        WindowRef::from(self.conn, self.xcb.root()).children_refs()
    }
}
