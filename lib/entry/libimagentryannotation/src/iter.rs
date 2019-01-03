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

use toml_query::read::TomlValueReadTypeExt;

use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;
use libimagstore::storeid::StoreIdIterator;
use libimagerror::errors::ErrorMsg as EM;

use failure::Fallible as Result;
use failure::ResultExt;
use failure::Error;
use failure::err_msg;

#[derive(Debug)]
pub struct AnnotationIter<'a>(StoreIdIterator, &'a Store);

impl<'a> AnnotationIter<'a> {

    pub fn new(iter: StoreIdIterator, store: &'a Store) -> AnnotationIter<'a> {
        AnnotationIter(iter, store)
    }

}

impl<'a> Iterator for AnnotationIter<'a> {
    type Item = Result<FileLockEntry<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.0.next() {
                None         => return None, // iterator consumed
                Some(Err(e)) => return Some(Err(e).map_err(Error::from)),
                Some(Ok(id)) => match self.1.get(id) {
                    Err(e) => {
                        return Some(Err(e)
                                    .context(err_msg("Store read error"))
                                    .map_err(Error::from))
                    },
                    Ok(Some(entry)) => {
                        match entry
                            .get_header()
                            .read_bool("annotation.is_annotation")
                            .context(EM::EntryHeaderReadError)
                            .map_err(Error::from)
                        {
                            Ok(None)        => continue, // not an annotation
                            Ok(Some(false)) => continue,
                            Ok(Some(true))  => return Some(Ok(entry)),
                            Err(e)          => return Some(Err(e)),
                        }
                    },
                    Ok(None) => continue,
                }
            }
        }
    }

}

