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

use clap::{Arg, App, ArgMatches, SubCommand};

use libimagstore::storeid::StoreId;
use libimagstore::storeid::IntoStoreId;
use libimagrt::runtime::IdPathProvider;
use libimagerror::trace::MapErrTrace;

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("deref")
                    .about("'Dereference a ref. This prints the Path(es) of the referenced file(s)")
                    .version("0.1")
                    .arg(Arg::with_name("ID")
                         .index(1)
                         .takes_value(true)
                         .required(false)
                         .multiple(true)
                         .help("The id of the store entry to dereference.")
                         .value_name("ID"))

                    .arg(Arg::with_name("ignore-noref")
                         .long("ignore-noref")
                         .takes_value(false)
                         .required(false)
                         .multiple(false)
                         .help("Ignore store entries which are not refs and do not print error message"))
                    )

        .subcommand(SubCommand::with_name("remove")
                .about("Remove a reference from an entry")
                .version("0.1")
                .arg(Arg::with_name("ID")
                     .index(1)
                     .takes_value(true)
                     .required(false)
                     .multiple(true)
                     .help("Remove the reference from this store entry")
                     .value_name("ENTRIES"))

                .arg(Arg::with_name("ignore-noref")
                     .long("ignore-noref")
                     .takes_value(false)
                     .required(false)
                     .multiple(false)
                     .help("Ignore store entries which are not refs and do not print error message"))
                )

        .subcommand(SubCommand::with_name("create")
                .about("Create a reference to a file")
                .version("0.1")
                .arg(Arg::with_name("ID")
                     .index(1)
                     .takes_value(true)
                     .required(true)
                     .multiple(false)
                     .help("Create a reference with that ID in the store. If the store id exists, it will be made into a reference.")
                     .value_name("ID"))

                .arg(Arg::with_name("path")
                     .index(2)
                     .takes_value(true)
                     .required(true)
                     .multiple(false)
                     .help("The path to refer to. If there is no basepath configuration in the config file for the path this file is located at, the operation will error.")
                     .value_name("ID"))

                .arg(Arg::with_name("force")
                     .long("force")
                     .takes_value(false)
                     .required(false)
                     .multiple(false)
                     .help("Use force to override existing references"))
                )
}

pub struct PathProvider;
impl IdPathProvider for PathProvider {
    fn get_ids(matches: &ArgMatches) -> Vec<StoreId> {
        match matches.subcommand() {
            ("deref", Some(subm)) => {
                subm.values_of("ID")
                    .ok_or_else(|| {
                        error!("No StoreId found");
                        ::std::process::exit(1)
                    })
                    .unwrap()
                    .into_iter()
                    .map(PathBuf::from)
                    .map(|pb| pb.into_storeid())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err_trace_exit_unwrap()
            },

            ("remove", Some(subm)) => {
                subm.values_of("ID")
                    .ok_or_else(|| {
                        error!("No StoreId found");
                        ::std::process::exit(1)
                    })
                    .unwrap()
                    .into_iter()
                    .map(PathBuf::from)
                    .map(|pb| pb.into_storeid())
                    .collect::<Result<Vec<_>, _>>()
                    .map_err_trace_exit_unwrap()
            },

            ("create", _) => {
                error!("Command does not get IDs as input");
                ::std::process::exit(1)
            },


            (other, _) => {
                error!("Not a known command: {}", other);
                ::std::process::exit(1)
            }
        }
    }
}
