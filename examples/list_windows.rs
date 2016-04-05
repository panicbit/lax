extern crate lax;

use lax::prelude::*;

fn main() {
    let conn = Connection::new().expect("connection");
    let screen = conn.preferred_screen();
    let mut children = screen.children_refs();

    for w in children.as_mut() {
        let attrs = w.attributes().expect("attributes");
        println!("0x{:08X}: {:?}", w.id(), attrs.map_state);
    }
}
