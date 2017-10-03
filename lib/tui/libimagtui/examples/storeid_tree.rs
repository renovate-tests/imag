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

extern crate clap;
extern crate cursive;
#[macro_use] extern crate version;
#[macro_use] extern crate log;
extern crate libimagtui;
extern crate libimagstore;
extern crate libimagrt;
extern crate libimagerror;

use clap::{Arg, App};
use cursive::Cursive;
use cursive::views::Panel;
use cursive::view::Identifiable;

use std::process::exit;

use libimagtui::storeid_explorer::Explorer;
use libimagstore::store::Store;
use libimagrt::setup::generate_runtime_setup;
use libimagerror::trace::MapErrTrace;

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
}

fn main() {
    let rt = generate_runtime_setup("imag-example-tui",
                                    &version!()[..],
                                    "TUI example for showing a tree of storeids",
                                    build_ui);

    let explorer = Explorer::new(rt.store()).unwrap();
    let mut siv = Cursive::new();
    siv.add_global_callback('q', |_| exit(0));

    siv.add_layer(Panel::new(explorer.with_id("tree")));

    siv.run();
}

