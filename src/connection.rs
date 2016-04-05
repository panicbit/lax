use prelude::*;
use std::cmp::max;
use xcb;

use xcb::ConnResult;

pub struct Connection {
    conn: xcb::Connection,
    preferred_screen: u8,
}

impl Connection {
    pub fn new() -> ConnResult<Connection> {
        Self::init(None)
    }

    pub fn with_display(display: &str) -> ConnResult<Connection> {
        Self::init(Some(display))
    }

    fn init(display: Option<&str>) -> ConnResult<Connection> {
        let (conn, preferred_screen) = try!(xcb::Connection::connect(display));
        Ok(Connection {
            conn: conn,
            preferred_screen: max(0, preferred_screen) as u8
        })
    }

    pub fn preferred_screen(&self) -> Screen {
        Screen::from(self, self.preferred_screen_index()).expect("preferred screen")
    }

    pub fn preferred_screen_index(&self) -> u8 {
        self.preferred_screen
    }

    pub fn as_xcb(&self) -> &xcb::Connection {
        &self.conn
    }
}
