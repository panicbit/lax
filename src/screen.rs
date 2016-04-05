use prelude::*;
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

    pub fn children_refs(&self) -> WindowIterator<'a> {
        let tree = xcb::query_tree(self.conn.as_xcb(), self.xcb.root());
        let tree = tree.get_reply().expect("tree");
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
