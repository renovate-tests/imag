//
// imag - the personal information management suite for the commandline
// Copyright (C) 2015-2018 the imag contributors
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

#[derive(Debug, Clone, Eq, PartialEq, Fail)]
pub enum ErrorMsg {
    #[fail(display = "IO Error")]
    IO,

    #[fail(display = "Locking error")]
    LockError,

    #[fail(display = "UTF8 error")]
    UTF8Error,

    #[fail(display = "Error in external process")]
    ExternalProcessError,

    #[fail(display = "File Error")]
    FileError,

    #[fail(display = "File not copied")]
    FileNotCopied,

    #[fail(display = "File not created")]
    FileNotCreated,

    #[fail(display = "File not found")]
    FileNotFound,

    #[fail(display = "Fail not removed")]
    FileNotRemoved,

    #[fail(display = "Fail not renamed")]
    FileNotRenamed,

    #[fail(display = "File not seeked")]
    FileNotSeeked,

    #[fail(display = "File not written")]
    FileNotWritten,

    #[fail(display = "Directory not created")]
    DirNotCreated,


    #[fail(display = "Formatting error")]
    FormatError,


    #[fail(display = "ID is locked")]
    IdLocked,

    #[fail(display = "Error while converting values")]
    ConversionError,


    #[fail(display = "Entry exists already: {}", _0)]
    EntryAlreadyExists(String),

    #[fail(display = "Entry not found: {}", _0)]
    EntryNotFound(String),

    #[fail(display = "Entry header error")]
    EntryHeaderError,

    #[fail(display = "Entry header type error")]
    EntryHeaderTypeError,

    #[fail(display = "Entry header type error at '{}', expected '{}'", _0, _1)]
    EntryHeaderTypeError2(&'static str, &'static str),

    #[fail(display = "Entry header read error")]
    EntryHeaderReadError,

    #[fail(display = "Entry header write error")]
    EntryHeaderWriteError,

    #[fail(display = "Entry header field missing: {}", _0)]
    EntryHeaderFieldMissing(&'static str),


    #[fail(display = "Toml deserialization error")]
    TomlDeserError,

    #[fail(display = "Toml querying error")]
    TomlQueryError,

}

