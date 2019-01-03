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

//! The Ref object is a helper over the link functionality, so one is able to create references to
//! files outside of the imag store.

use std::path::Path;
use std::path::PathBuf;

use libimagentryutil::isa::Is;
use libimagentryutil::isa::IsKindHeaderPathProvider;
use libimagstore::store::Entry;
use libimagerror::errors::ErrorMsg as EM;

use toml::Value;
use toml_query::read::TomlValueReadExt;
use toml_query::delete::TomlValueDeleteExt;
use toml_query::insert::TomlValueInsertExt;
use failure::Fallible as Result;
use failure::Error;

use refstore::UniqueRefPathGenerator;

pub trait Ref {

    /// Check whether the underlying object is actually a ref
    fn is_ref(&self) -> Result<bool>;

    /// Get the stored hash.
    ///
    /// Does not need a `UniqueRefPathGenerator` as it reads the hash stored in the header
    fn get_hash(&self) -> Result<&str>;

    /// Make this object a ref
    fn make_ref<P: AsRef<Path>>(&mut self, hash: String, path: P) -> Result<()>;

    /// Get the referenced path.
    ///
    /// Does not need a `UniqueRefPathGenerator` as it reads the path stored in the header.
    fn get_path(&self) -> Result<PathBuf>;

    /// Check whether the referenced file still matches its hash
    fn hash_valid<RPG: UniqueRefPathGenerator>(&self) -> Result<bool>;

    fn remove_ref(&mut self) -> Result<()>;

    /// Alias for `r.fs_link_exists() && r.deref().is_file()`
    fn is_ref_to_file(&self) -> Result<bool> {
        self.get_path().map(|p| p.is_file())
    }

    /// Alias for `r.fs_link_exists() && r.deref().is_dir()`
    fn is_ref_to_dir(&self) -> Result<bool> {
        self.get_path().map(|p| p.is_dir())
    }

    /// Alias for `!Ref::fs_link_exists()`
    fn is_dangling(&self) -> Result<bool> {
        self.get_path().map(|p| !p.exists())
    }

}

provide_kindflag_path!(pub IsRef, "ref.is_ref");

impl Ref for Entry {

    /// Check whether the underlying object is actually a ref
    fn is_ref(&self) -> Result<bool> {
        self.is::<IsRef>().map_err(Error::from)
    }

    fn get_hash(&self) -> Result<&str> {
        self.get_header()
            .read("ref.hash")
            .map_err(Error::from)?
            .ok_or_else(|| Error::from(EM::EntryHeaderFieldMissing("ref.hash")))
            .and_then(|v| {
                v.as_str().ok_or_else(|| Error::from(EM::EntryHeaderTypeError2("ref.hash", "string")))
            })
    }

    fn make_ref<P: AsRef<Path>>(&mut self, hash: String, path: P) -> Result<()> {
        let path_str : String = path
            .as_ref()
            .to_str()
            .map(String::from)
            .ok_or_else(|| EM::UTF8Error)?;

        let _   = self.set_isflag::<IsRef>()?;
        let hdr = self.get_header_mut();
        hdr.insert("ref.path", Value::String(String::from(path_str)))?;
        hdr.insert("ref.hash", Value::String(hash))?;

        Ok(())
    }

    fn get_path(&self) -> Result<PathBuf> {
        self.get_header()
            .read("ref.path")
            .map_err(Error::from)?
            .ok_or_else(|| Error::from(EM::EntryHeaderFieldMissing("ref.path")))
            .and_then(|v| {
                v.as_str()
                    .ok_or_else(|| EM::EntryHeaderTypeError2("ref.path", "string"))
                    .map_err(Error::from)
            })
            .map(PathBuf::from)
    }

    fn hash_valid<RPG: UniqueRefPathGenerator>(&self) -> Result<bool> {
        self.get_path()
            .map(PathBuf::from)
            .map_err(Error::from)
            .and_then(|pb| RPG::unique_hash(pb))
            .and_then(|h| Ok(h == self.get_hash()?))
    }

    fn remove_ref(&mut self) -> Result<()> {
        let hdr = self.get_header_mut();
        let _   = hdr.delete("ref.hash")?;
        let _   = hdr.delete("ref.path")?;
        let _   = hdr.delete("ref")?;
        Ok(())
    }

}

