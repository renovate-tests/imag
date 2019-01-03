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

use std::path::PathBuf;

use clap::{Arg, ArgMatches, App, SubCommand};

use libimagstore::storeid::StoreId;
use libimagstore::storeid::IntoStoreId;
use libimagrt::runtime::IdPathProvider;
use libimagerror::trace::MapErrTrace;

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("create-category")
                    .about("Create a new category")
                    .version("0.1")
                    .arg(Arg::with_name("create-category-name")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("The name of the new category")
                         .value_name("NAME"))
                   )

        .subcommand(SubCommand::with_name("delete-category")
                    .about("Delete a new category")
                    .version("0.1")
                    .arg(Arg::with_name("delete-category-name")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("The name of the category to delete")
                         .value_name("NAME"))
                   )

        .subcommand(SubCommand::with_name("list-categories")
                    .about("Show all category names")
                    .version("0.1"))

        .subcommand(SubCommand::with_name("list-category")
                    .about("List all entries for a category")
                    .version("0.1")
                    .arg(Arg::with_name("list-category-name")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("The name of the category to list all entries for")
                         .value_name("NAME"))
                   )

        .subcommand(SubCommand::with_name("set")
                    .about("Set the category of entries")
                    .version("0.1")
                    .arg(Arg::with_name("set-name")
                         .index(1)
                         .takes_value(true)
                         .required(true)
                         .multiple(false)
                         .help("The name of the category to list all entries for")
                         .value_name("NAME"))

                    .arg(Arg::with_name("set-ids")
                         .index(2)
                         .takes_value(true)
                         .required(false)
                         .multiple(true)
                         .help("The entries to set the category for")
                         .value_name("ID"))
                   )

        .subcommand(SubCommand::with_name("get")
                    .about("Get the category of the entry")
                    .version("0.1")
                    .arg(Arg::with_name("get-ids")
                         .index(1)
                         .takes_value(true)
                         .required(false)
                         .multiple(true)
                         .help("The id of the Entry to get the category for")
                         .value_name("ID"))
                   )
}

pub struct PathProvider;
impl IdPathProvider for PathProvider {
    fn get_ids(matches: &ArgMatches) -> Vec<StoreId> {
        match matches.subcommand() {
            ("create-category", _) => {
                error!("Command does not get IDs as input");
                ::std::process::exit(1)
            },

            ("delete-category", _) => {
                error!("Command does not get IDs as input");
                ::std::process::exit(1)
            },

            ("list-categories", _) => {
                error!("Command does not get IDs as input");
                ::std::process::exit(1)
            },

            ("list-category", _) => {
                error!("Command does not get IDs as input");
                ::std::process::exit(1)
            },

            ("set", Some(subm)) => {
                subm.values_of("set-ids")
                    .ok_or_else(|| {
                        error!("No StoreId found");
                        ::std::process::exit(1)
                    })
                    .unwrap()
                    .into_iter()
                    .map(PathBuf::from)
                    .map(|pb| pb.into_storeid())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err_trace_exit_unwrap(1)
            },

            ("get", Some(subm)) => {
                subm.values_of("get-ids")
                    .ok_or_else(|| {
                        error!("No StoreId found");
                        ::std::process::exit(1)
                    })
                    .unwrap()
                    .into_iter()
                    .map(PathBuf::from)
                    .map(|pb| pb.into_storeid())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err_trace_exit_unwrap(1)
            },

            (other, _) => {
                    error!("Not a known command: {}", other);
                    ::std::process::exit(1)
            }
        }
    }
}
