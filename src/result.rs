use std::io;
use terminfo;

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
