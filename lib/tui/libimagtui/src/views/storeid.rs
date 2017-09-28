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

use error::Result;
use util::StoreHandle;

use libimagstore::storeid::StoreId;

use cursive::view::View;
use cursive::views::TextView;
use cursive::event::Event;
use cursive::event::EventResult;
use cursive::Printer;
use cursive::vec::Vec2;
use cursive::direction::Direction;

pub struct StoreIdView {
    store: StoreHandle,
    sid: String,
    textview: TextView,
}

impl StoreIdView {

    fn build(store: StoreHandle, sid: StoreId) -> Result<StoreIdView> {
        let text = sid.to_str()?;
        Ok(StoreIdView {
            store: store,
            sid: text.clone(),
            textview: TextView::new(text),
        })
    }
}

impl View for StoreIdView {

    fn draw(&self, printer: &Printer) {
        self.textview.draw(printer)
    }

    fn on_event(&mut self, e: Event) -> EventResult {
        // Construct a Menu on what to do with the Storeid:
        //  * Load it from the store and show it
        //  * Check whether it exists
        //  * Yank its content
        unimplemented!()
    }

    fn required_size(&mut self, constraint: Vec2) -> Vec2 {
        (self.sid.len(), 1).into()
    }

    fn take_focus(&mut self, source: Direction) -> bool {
        true
    }
}

