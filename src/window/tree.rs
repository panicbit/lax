use xcb;
use ::Connection;
use super::{WindowRef, WindowIterator};
use monster::incubation::OwningRefMut;

pub struct Tree<'a> {
    conn: &'a Connection,
    xcb: xcb::QueryTreeReply,
}

impl <'a> Tree<'a> {
    pub fn from(window: &WindowRef<'a>) -> Result<Tree<'a>, xcb::GenericError> {
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
