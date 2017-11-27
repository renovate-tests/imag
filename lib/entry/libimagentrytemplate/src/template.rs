//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015, 2016 Matthias Beyer <mail@beyermatthias.de> and contributors
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

use std::result::Result as RResult;
use std::str::FromStr;

use toml::Value;

use libimagstore::store::Store;
use libimagstore::store::FileLockEntry;

use error::*;

/// A trait for describing a template in the store
pub trait Template {

    /// Check whether the store entry is actually a template
    fn is_template(&self) -> Result<bool>;

    /// Get the name of the template
    fn name(&self) -> Result<String>;

    /// Get the fields which are required by the template for an instance
    fn required_fields(&self) -> Result<Vec<Field>>;

    /// Get the optional fields for an instanceof this template
    fn optional_fields(&self) -> Result<Vec<Field>>;

    /// Get all fields which are available in the template
    ///
    /// Does nothing (does not call the `InstanceGenerator`) if `Self::is_template()`,
    /// `Self::required_fields()` or `Self::optional_fields()` returns an `Err(_)`.
    fn all_fields<'a>(&self) -> Result<Vec<Field>> {
        let _     = self.is_template()?;
        let mut r = self.required_fields()?;
        let mut o = self.optional_fields()?;

        r.append(&mut o);
        Ok(r)
    }

    /// Make an instance of the template
    ///
    /// The `ig` InstanceGenerator is used to make the name, the required fields and the optional
    /// fields into a `Instance` object which can then be converted to an actual FileLockEntry.
    ///
    /// Does nothing (does not call the `InstanceGenerator`) if `Self::is_template()`,
    /// `Self::name()`, `Self::required_fields()` or `Self::optional_fields()` returns an `Err(_)`.
    fn mk_instance<'a>(&self, ig: &mut InstanceGenerator, store: &'a Store)
        -> Result<FileLockEntry<'a>>
    {
        let _    = self.is_template()?;
        let name = self.name()?;
        let reqf = self.required_fields()?;
        let optf = self.optional_fields()?;
        ig.create_instance(store, name, reqf, optf)
    }
}

pub trait InstanceGenerator {
    fn create_instance<'a>(
        &mut self,
        &'a Store,
        template_name: String,
        required_fields: Vec<Field>,
        optional_fields: Vec<Field>
    ) -> Result<FileLockEntry<'a>>;
}

pub enum FieldType {
    Bool,
    Int,
    Float,
    String,
}

impl FieldType {
    fn instance_from_str(&self, s: &str) -> Result<Value> {
        unimplemented!()
    }
}

pub struct Field {
    name: String,
    ty: FieldType,
}

impl Field {

    pub fn field_name(&self) -> &String {
        &self.name
    }

    pub fn field_type(&self) -> &FieldType {
        &self.ty
    }

    pub fn mk_value(&self, s: &str) -> Result<Value> {
        self.ty.instance_from_str(s)
    }

}

