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

use filters::filter::Filter;

use libimagstore::storeid::StoreId;

pub struct IsInCollection<A: AsRef<str>>(Vec<A>);

impl<A: AsRef<str>> IsInCollection<A> {
    pub fn new(v: Vec<A>) -> Self {
        IsInCollection(v)
    }
}

impl<A: AsRef<str>> Filter<StoreId> for IsInCollection<A> {

    fn filter(&self, sid: &StoreId) -> bool {
        sid.is_in_collection(&self.0)
    }

}

