use xcb;
use monster::incubation::OwningRefMut;
use prelude::*;

mod attributes;
pub use self::attributes::Attributes;

mod tree;
pub use self::tree::Tree;

#[derive(Copy,Clone)]
pub struct WindowRef<'a> {
    conn: &'a Connection,
    id: xcb::Window
}

impl <'a> WindowRef<'a> {
    pub fn from(conn: &'a Connection, id: xcb::Window) -> WindowRef<'a> {
        WindowRef {
            conn: conn,
            id: id
        }
    }

    pub fn id(&self) -> xcb::Window {
        self.id
    }

    pub fn attributes(&self) -> Result<Attributes, xcb::GenericError> {
        Attributes::from(self)
    }

    pub fn focus(&self, revert_focus: RevertFocus) {
        xcb::set_input_focus(
            self.conn.as_xcb(),
            revert_focus as u8,
            self.id(),
            xcb::TIME_CURRENT_TIME
        );
    }

    pub fn to_owned() -> Window {
        unimplemented!()
    }

    pub fn parent_ref(&self) -> Result<WindowRef<'a>, xcb::GenericError> {
        self.tree().map(|t| t.parent_ref())
    }

    pub fn children_refs(&self) -> Result<WindowIterator<'a>, xcb::GenericError> {
        self.tree().map(|t| t.children_refs())
    }

    pub fn tree(&self) -> Result<Tree<'a>, xcb::GenericError> {
        Tree::from(self)
    }
}

pub type WindowIterator<'a> = OwningRefMut<
    xcb::QueryTreeReply,
    Box<Iterator<Item=WindowRef<'a>> + 'a>
>;

pub struct Window {
    id: xcb::Window,
    map_state: MapState
}

impl Window {
    pub fn from_ref(window: WindowRef) {
        unimplemented!()
    }
}


#[derive(Debug,Eq,PartialEq)]
#[repr(u8)]
pub enum MapState {
    Unmapped   = 0,
    Unviewable = 1,
    Viewable   = 2,
}

impl MapState {
    fn from_xcb(map_state: xcb::MapState) -> MapState {
        match map_state {
            xcb::MAP_STATE_UNMAPPED   => MapState::Unmapped,
            xcb::MAP_STATE_UNVIEWABLE => MapState::Unviewable,
            xcb::MAP_STATE_VIEWABLE   => MapState::Viewable,
            _ => panic!("unknown map state")
        }
    }
}

#[derive(Debug,Eq,PartialEq)]
#[repr(u8)]
pub enum RevertFocus {
    None        = 0,
    PointerRoot = 1,
    Parent      = 2,
}
