//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 Matthias Beyer <mail@beyermatthias.de> and contributors
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

use clap::{Arg, App, SubCommand};

use libimagutil::cli_validators::*;

pub fn build_ui<'a>(app: App<'a, 'a>) -> App<'a, 'a> {
    app
        .subcommand(SubCommand::with_name("weight")
                   .about("Weight tracking tool")
                   .version("0.1")
                   .subcommand(SubCommand::with_name("add")
                              .about("Add weight (default: for today")
                              .version("0.1")

                              .arg(Arg::with_name("value")
                                   .index(1)
                                   .takes_value(true)
                                   .required(true)
                                   .multiple(false)
                                   .value_name("VALUE")
                                   .help("Value for tracking"))

                              .arg(Arg::with_name("unit")
                                   .index(2)
                                   .takes_value(true)
                                   .required(false)
                                   .multiple(false)
                                   .value_name("UNIT")
                                   .help("Unit for tracking (default from config or 'kg')"))

                              .arg(Arg::with_name("datetime")
                                   .long("datetime")
                                   .takes_value(true)
                                   .required(false)
                                   .multiple(false)
                                   .value_name("DATETIME")
                                   .help("Set time for tracking"))
                              )

                   .subcommand(SubCommand::with_name("list")
                              .about("List trackings for a certain timerange")
                              .version("0.1")

                              .arg(Arg::with_name("timerange")
                                   .index(1)
                                   .takes_value(true)
                                   .required(false)
                                   .multiple(false)
                                   .value_name("timerange")
                                   .help("Timerange to list (default: '1month', future dates will not be printed)"))

                              .arg(Arg::with_name("format")
                                   .long("format")
                                   .short("f")
                                   .takes_value(true)
                                   .required(false)
                                   .multiple(false)
                                   .value_name("FORMAT")
                                   .help("Override listing format, either with format string or config-defined format (config from health.weight.list_format)"))
                              .arg(Arg::with_name("future-prediction")
                                   .long("predict")
                                   .takes_value(false)
                                   .required(false)
                                   .multiple(false)
                                   .help("Try to predict further development"))

                              )
                   .subcommand(SubCommand::with_name("graph")
                              .about("Print graph about weight in certain timeframe")
                              .version("0.1")
                              .arg(Arg::with_name("timerange")
                                   .index(1)
                                   .takes_value(true)
                                   .required(false)
                                   .multiple(false)
                                   .value_name("timerange")
                                   .help("Timerange to list (default: '1month', future dates will not be printed)"))

                              .arg(Arg::with_name("future-prediction")
                                   .long("predict")
                                   .takes_value(false)
                                   .required(false)
                                   .multiple(false)
                                   .help("Try to predict further development"))
                              )
                   )

        .subcommand(SubCommand::with_name("workout")
                   .about("Workout tracking and planning tool")
                   .version("0.1")
                   .subcommand(SubCommand::with_name("define")
                              .about("Definr a new workout and schedule it")
                              .version("0.1")
                              // uses imag-calendar to create calendar entries for when workout is
                              // scheduled
                              // uses imag-todo to create todo entries.for the day the workout is
                              // scheduled
                              // uses kairos for re-scheduling
                              // links all created entries to a "workout" entry for that date and
                              // links this workout entry to an overall entry for this type of
                              // workout
                              //
                              // todo and calendar can be set to "off" via flags
                              )
                   .subcommand(SubCommand::with_name("track")
                              .about("Add a tracking of a completed workout")
                              .version("0.1")
                              // Finds the closest occurence for the workout type passed and marks
                              // it as done (imag-todo) or/and moves the calendar entry
                              // (inag-calendar) to the
                              // current time (or to the optionally passed time) so that it is in
                              // the past.
                              //
                              // It also records notes (via imag-diary, the diary name can be
                              // configured)
                              //
                              )
                   .subcommand(SubCommand::with_name("list")
                              .about("List trackings")
                              .version("0.1")
                              .arg(Arg::with_name("timerange")
                                   .index(1)
                                   .takes_value(true)
                                   .required(false)
                                   .multiple(false)
                                   .value_name("timerange")
                                   .help("Timerange to list (default: '1month', future dates will not be printed)"))

                              .arg(Arg::with_name("future-prediction")
                                   .long("predict")
                                   .takes_value(false)
                                   .required(false)
                                   .multiple(false)
                                   .help("Try to predict further development"))
                              )
                   )
        .subcommand(SubCommand::with_name("diet")
                   .about("Diet tracking and planning tool")
                   .version("0.1")
                   )
}
