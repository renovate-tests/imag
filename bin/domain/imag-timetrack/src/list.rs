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

use std::str::FromStr;

use chrono::NaiveDateTime;
use filters::filter::Filter;
use prettytable::Table;
use prettytable::row::Row;
use prettytable::cell::Cell;

use libimagerror::trace::trace_error;
use libimagerror::trace::MapErrTrace;
use libimagerror::iter::TraceIterator;
use libimagstore::store::FileLockEntry;
use libimagtimetrack::timetrackingstore::TimeTrackStore;
use libimagtimetrack::timetracking::TimeTracking;
use libimagtimetrack::error::Result;

use libimagrt::runtime::Runtime;

pub fn list(rt: &Runtime) -> i32 {
    let (_, cmd) = rt.cli().subcommand();
    let cmd = cmd.unwrap(); // checked in main()

    let start = match cmd.value_of("start-time").map(::chrono::naive::NaiveDateTime::from_str) {
        None         => None,
        Some(Ok(dt)) => Some(dt),
        Some(Err(e)) => {
            trace_error(&e);
            None
        }
    };
    let end = match cmd.value_of("end-time").map(::chrono::naive::NaiveDateTime::from_str) {
        None         => None,
        Some(Ok(dt)) => Some(dt),
        Some(Err(e)) => {
            trace_error(&e);
            None
        }
    };

    let list_not_ended = cmd.is_present("list-not-ended");

    list_impl(rt, start, end, list_not_ended)
}

pub fn list_impl(rt: &Runtime,
                 start: Option<NaiveDateTime>,
                 end: Option<NaiveDateTime>,
                 list_not_ended: bool)
    -> i32
{

    let start_time_filter = |timetracking: &FileLockEntry| {
        start.map(|s| match timetracking.get_start_datetime() {
            Ok(Some(dt)) => dt >= s,
            Ok(None)     => {
                warn!("Funny things are happening: Timetracking has no start time");
                false
            }
            Err(e) => {
                trace_error(&e);
                false
            }
        })
        .unwrap_or(true)
    };

    let end_time_filter = |timetracking: &FileLockEntry| {
        end.map(|s| match timetracking.get_end_datetime() {
            Ok(Some(dt)) => dt <= s,
            Ok(None)     => list_not_ended,
            Err(e)       => {
                trace_error(&e);
                false
            }
        })
        .unwrap_or(true)
    };

    let filter = start_time_filter.and(end_time_filter);

    let mut table = Table::new();
    table.set_titles(Row::new(["Tag", "Start", "End"].into_iter().map(|s| Cell::new(s)).collect()));

    let mut stdout = ::std::io::stdout();

    rt.store()
        .get_timetrackings()
        .and_then(|iter| {
            iter.trace_unwrap()
                .filter(|e| filter.filter(e))
                .fold(Ok(table), |acc: Result<_>, e| {
                    acc.and_then(|mut tab: Table| {
                        debug!("Processing {:?}", e.get_location());

                        let tag   = e.get_timetrack_tag()?;
                        debug!(" -> tag = {:?}", tag);

                        let start = e.get_start_datetime()?;
                        debug!(" -> start = {:?}", start);

                        let end   = e.get_end_datetime()?;
                        debug!(" -> end = {:?}", end);

                        let v = match (start, end) {
                            (None, _)          => vec![String::from(tag.as_str()), String::from(""), String::from("")],
                            (Some(s), None)    => {
                                vec![
                                    String::from(tag.as_str()),
                                    format!("{}", s),
                                    String::from(""),
                                ]
                            },
                            (Some(s), Some(e)) => {
                                vec![
                                    String::from(tag.as_str()),
                                    format!("{}", s),
                                    format!("{}", e),
                                ]
                            },
                        };

                        let cells : Vec<Cell> = v
                            .into_iter()
                            .map(|s| Cell::new(&s))
                            .collect();
                        tab.add_row(Row::new(cells));

                        Ok(tab)
                    })
                })?
                .print(&mut stdout)
                .map_err(|_| String::from("Failed printing table").into())
        })
        .map(|_| 0)
        .map_err_trace()
        .unwrap_or(1)
}

