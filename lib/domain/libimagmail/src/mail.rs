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

use std::path::Path;
use std::fs::File;
use std::io::Read;
use std::fs::OpenOptions;

use libimagstore::store::Store;
use libimagstore::storeid::StoreId;
use libimagstore::store::FileLockEntry;
use libimagentryref::reference::Ref;
use libimagentryref::refstore::RefStore;
use libimagentryref::refstore::UniqueRefPathGenerator;
use libimagerror::errors::ErrorMsg as EM;

use email::MimeMessage;
use email::results::ParsingResult as EmailParsingResult;

use failure::Fallible as Result;
use failure::ResultExt;
use failure::Error;
use failure::err_msg;

struct UniqueMailRefGenerator;
impl UniqueRefPathGenerator for UniqueMailRefGenerator {
    /// The collection the `StoreId` should be created for
    fn collection() -> &'static str {
        "mail"
    }

    /// A function which should generate a unique string for a Path
    fn unique_hash<A: AsRef<Path>>(path: A) -> Result<String> {
        use filters::filter::Filter;
        use email::Header;

        let mut s = String::new();
        let _     = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(path)?
            .read_to_string(&mut s)?;

        MimeMessage::parse(&s)
            .context(err_msg("Error creating ref"))
            .map_err(Error::from)
            .and_then(|mail| {
                let has_key = |hdr: &Header, exp: &str| hdr.name == exp;

                let subject_filter = |hdr: &Header| has_key(hdr, "Subject");
                let from_filter    = |hdr: &Header| has_key(hdr, "From");
                let to_filter      = |hdr: &Header| has_key(hdr, "To");

                let filter = subject_filter.or(from_filter).or(to_filter);

                let mut v : Vec<String> = vec![];
                for hdr in mail.headers.iter().filter(|item| filter.filter(item)) {
                    let s = hdr
                        .get_value()
                        .context(err_msg("Ref creation error"))?;

                    v.push(s);
                }
                let s : String = v.join("");
                Ok(s)
            })
    }

    /// Postprocess the generated `StoreId` object
    fn postprocess_storeid(sid: StoreId) -> Result<StoreId> {
        Ok(sid)
    }
}

struct Buffer(String);

impl Buffer {
    pub fn parsed(&self) -> EmailParsingResult<MimeMessage> {
        MimeMessage::parse(&self.0)
    }
}

impl From<String> for Buffer {
    fn from(data: String) -> Buffer {
        Buffer(data)
    }
}

pub struct Mail<'a>(FileLockEntry<'a>, Buffer);

impl<'a> Mail<'a> {

    /// Imports a mail from the Path passed
    pub fn import_from_path<P: AsRef<Path>>(store: &Store, p: P) -> Result<Mail> {
        debug!("Importing Mail from path");
        store.retrieve_ref::<UniqueMailRefGenerator, P>(p)
            .and_then(|reference| {
                debug!("Build reference file: {:?}", reference);
                reference.get_path()
                    .context(err_msg("Ref handling error"))
                    .map_err(Error::from)
                    .and_then(|path| File::open(path).context(EM::IO).map_err(Error::from))
                    .and_then(|mut file| {
                        let mut s = String::new();
                        file.read_to_string(&mut s)
                            .map(|_| s)
                            .context(EM::IO)
                            .map_err(Error::from)
                    })
                    .map(Buffer::from)
                    .map(|buffer| Mail(reference, buffer))
            })
    }

    /// Opens a mail by the passed hash
    pub fn open<S: AsRef<str>>(store: &Store, hash: S) -> Result<Option<Mail>> {
        debug!("Opening Mail by Hash");
        store.get_ref::<UniqueMailRefGenerator, S>(hash)
            .context(err_msg("Fetch by hash error"))
            .context(err_msg("Fetch error"))
            .map_err(Error::from)
            .and_then(|o| match o {
                Some(r) => Mail::from_fle(r).map(Some),
                None => Ok(None),
            })
    }

    /// Implement me as TryFrom as soon as it is stable
    pub fn from_fle(fle: FileLockEntry<'a>) -> Result<Mail<'a>> {
        fle.get_path()
            .context(err_msg("Ref handling error"))
            .map_err(Error::from)
            .and_then(|path| File::open(path).context(EM::IO).map_err(Error::from))
            .and_then(|mut file| {
                let mut s = String::new();
                file.read_to_string(&mut s)
                    .map(|_| s)
                    .context(EM::IO)
                    .map_err(Error::from)
            })
            .map(Buffer::from)
            .map(|buffer| Mail(fle, buffer))
    }

    pub fn get_field(&self, field: &str) -> Result<Option<String>> {
        debug!("Getting field in mail: {:?}", field);
        self.1
            .parsed()
            .context(err_msg("Mail parsing error"))
            .map_err(Error::from)
            .map(|parsed| {
                parsed.headers
                    .iter()
                    .filter(|hdr| hdr.name == field)
                    .nth(0)
                    .and_then(|field| field.get_value().ok())
            })
    }

    pub fn get_from(&self) -> Result<Option<String>> {
        self.get_field("From")
    }

    pub fn get_to(&self) -> Result<Option<String>> {
        self.get_field("To")
    }

    pub fn get_subject(&self) -> Result<Option<String>> {
        self.get_field("Subject")
    }

    pub fn get_message_id(&self) -> Result<Option<String>> {
        self.get_field("Message-ID")
    }

    pub fn get_in_reply_to(&self) -> Result<Option<String>> {
        self.get_field("In-Reply-To")
    }

}
