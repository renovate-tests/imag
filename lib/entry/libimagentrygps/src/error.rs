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

error_chain! {
    types {
        GPSError, GPSErrorKind, ResultExt, Result;
    }

    errors {
        StoreReadError {
            description("Store read error")
            display("Store read error")
        }

        StoreWriteError {
            description("Store write error")
            display("Store write error")
        }

        HeaderWriteError {
            description("Couldn't write Header for annotation")
            display("Couldn't write Header for annotation")
        }

        HeaderReadError {
            description("Couldn't read Header of Entry")
            display("Couldn't read Header of Entry")
        }

        HeaderTypeError {
            description("Header field has unexpected type")
            display("Header field has unexpected type")
        }

        TypeError {
            description("Type Error")
            display("Type Error")
        }

        DegreeMissing {
            description("'degree' value missing")
            display("'degree' value missing")
        }

        MinutesMissing {
            description("'minutes' value missing")
            display("'minutes' value missing")
        }

        SecondsMissing {
            description("'seconds' value missing")
            display("'seconds' value missing")
        }

        LongitudeMissing {
            description("'longitude' value missing")
            display("'longitude' value missing")
        }

        LatitudeMissing {
            description("'latitude' value missing")
            display("'latitude' value missing")
        }

        NumberConversionError {
            description("Cannot convert number to fit into variable")
            display("Cannot convert number to fit into variable")
        }
    }
}

