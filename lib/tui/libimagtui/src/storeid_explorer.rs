//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; version
// 2.1 of the License.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
//

use std::ops::{Deref, DerefMut};
use std::path::PathBuf;

use libimagstore::store::Store;
use libimagstore::store::StoreObject;
use libimagstore::storeid::StoreId;

use error::Result;

use cursive_tree_view::TreeView;
use cursive_tree_view::Placement;

use cursive::Printer;
use cursive::event::Event;
use cursive::event::EventResult;
use cursive::vec::Vec2;
use cursive::view::Selector;
use cursive::view::View;
use cursive::direction::Direction;

pub struct Explorer(TreeView<StoreId>);

impl Explorer {

    pub fn new(store: &Store) -> Result<Explorer> {
        let mut tv = TreeView::new();

        for element in store.walk("") { // hack to walk all collections
            let entry = match element {
                StoreObject::Id(sid)        => try!(sid.into_pathbuf()),
                StoreObject::Collection(pb) => pb,
            };

            let id    = try!(StoreId::new_baseless(PathBuf::from(entry)));
            tv.insert_item(id, Placement::LastChild, 0);
        }

        Ok(Explorer(tv))
    }

}

impl Deref for Explorer {
    type Target = TreeView<StoreId>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Explorer {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl View for Explorer {

    fn draw(&self, printer: &Printer) {
        self.0.draw(printer)
    }

    fn on_event(&mut self, e: Event) -> EventResult {
        self.0.on_event(e)
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        self.0.required_size(constraint)
    }

    fn needs_relayout(&self) -> bool {
        self.0.needs_relayout()
    }

    fn layout(&mut self, v: Vec2) {
        self.0.layout(v)
    }

    //fn find_any<'a>(&mut self, s: &Selector, f: Box<FnMut(&mut ::std::any::Any) + 'a>) {
    //    self.0.find_any(s, f)
    //}

    fn focus_view(&mut self, s: &Selector) -> ::std::result::Result<(), ()> {
        self.0.focus_view(s)
    }

    fn take_focus(&mut self, source: Direction) -> bool {
        self.0.take_focus(source)
    }

}

