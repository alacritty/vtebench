// Copyright 2016 Joe Wilm
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
use std::io;

use failure::Fail;

pub type Result<T> = ::std::result::Result<T, Error>;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),

    #[fail(display = "{}", _0)]
    Terminfo(#[cause] terminfo::Error),
}

impl From<io::Error> for Error {
    fn from(val: io::Error) -> Error {
        Error::Io(val)
    }
}

impl From<terminfo::Error> for Error {
    fn from(val: terminfo::Error) -> Error {
        Error::Terminfo(val)
    }
}
