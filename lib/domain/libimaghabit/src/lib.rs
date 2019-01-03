//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2019 Matthias Beyer <mail@beyermatthias.de> and contributors
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

#![deny(
    dead_code,
    non_camel_case_types,
    non_snake_case,
    path_statements,
    trivial_numeric_casts,
    unstable_features,
    unused_allocation,
    unused_import_braces,
    unused_imports,
    unused_must_use,
    unused_mut,
    unused_qualifications,
    while_true,
)]

extern crate chrono;
extern crate toml;
extern crate toml_query;
extern crate kairos;
#[macro_use] extern crate log;
#[macro_use] extern crate failure;

#[macro_use] extern crate libimagstore;
extern crate libimagerror;
extern crate libimagentryedit;
extern crate libimagentrylink;
#[macro_use] extern crate libimagentryutil;
extern crate libimagutil;

module_entry_path_mod!("habit");

pub mod habit;
pub mod instance;
pub mod iter;
pub mod store;
pub mod util;

