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

use std::path::Path;

use failure::Fallible as Result;

use libimagentryref::hasher::Hasher;

pub struct MailHasher;

impl Hasher for MailHasher {
    const NAME: &'static str = "MailHasher";

    /// hash the file at path `path`
    ///
    /// TODO: This is the expensive implementation. We use the message Id as hash, which is
    /// convenient and _should_ be safe
    ///
    /// TODO: Confirm that this approach is right
    fn hash<P: AsRef<Path>>(path: P) -> Result<String> {
        ::util::get_message_id_for_mailfile(path)
    }
}
