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

