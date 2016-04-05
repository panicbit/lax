use xcb;
use monster::incubation::OwningRefMut;
use prelude::*;

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
        let attrs = xcb::get_window_attributes(self.conn.as_xcb(), self.id);
        let attrs = try!(attrs.get_reply());
        Ok(Attributes::from_xcb(attrs))
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

pub struct Tree<'a> {
    conn: &'a Connection,
    xcb: xcb::QueryTreeReply,
}

impl <'a> Tree<'a> {
    fn from(window: &WindowRef<'a>) -> Result<Tree<'a>, xcb::GenericError> {
        let tree = xcb::query_tree(window.conn.as_xcb(), window.id());
        let tree = try!(tree.get_reply());

        Ok(Tree {
            conn: window.conn,
            xcb: tree
        })
    }

    pub fn parent_ref(&self) -> WindowRef<'a> {
        WindowRef::from(self.conn, self.xcb.parent())
    }

    pub fn children_refs(self) -> WindowIterator<'a> {
        let tree = self.xcb;
        let conn = self.conn;

        OwningRefMut::new(Box::new(tree), |tree| Box::new(
            tree.children().iter().map(move |&id|
                WindowRef::from(conn, id)
            )
        ) as Box<_>)
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
pub struct Attributes {
    pub map_state: MapState
}

impl Attributes {
    fn from_xcb(attrs: xcb::GetWindowAttributesReply) -> Attributes {
        Attributes {
            map_state: MapState::from_xcb(attrs.map_state() as u32)
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
