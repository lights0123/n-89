//! # Main error handling
//! This module handles the error handling for the whole crate. It is responsible for converting
//! errors from C functions into Rust ones.

use cstr_core::NulError;
use failure::Fail;

/// The main Error type
#[derive(Fail, Clone, Debug, Eq, PartialEq)]
pub enum Error {
    /// A string containing a null-character was passed to a function that needs to convert the
    /// string to a C-style string.
    #[fail(display = "cstr_core error: {}.", _0)]
    NullError(NulError),
    #[fail(display = "Breakpoint has been encountered")]
    Breakpoint,
    #[fail(display = "Trace")]
    Trace,
    #[fail(display = "CantOpen")]
    CantOpen,
    #[fail(display = "InvalidImage")]
    InvalidImage,
    #[fail(display = "InvalidUpgrade")]
    InvalidUpgrade,
    #[fail(display = "NoImage")]
    NoImage,
    #[fail(display = "InvalidRomSize")]
    InvalidRomSize,
    #[fail(display = "NotTiFile")]
    NotTiFile,
    #[fail(display = "Malloc")]
    Malloc,
    #[fail(display = "CantOpenDir")]
    CantOpenDir,
    #[fail(display = "CantUpgrade")]
    CantUpgrade,
    #[fail(display = "InvalidRom")]
    InvalidRom,
    #[fail(display = "CantOpenState")]
    CantOpenState,
    #[fail(display = "RevisionMatch")]
    RevisionMatch,
    #[fail(display = "HeaderMatch")]
    HeaderMatch,
    #[fail(display = "StateMatch")]
    StateMatch,
    #[fail(display = "Other: {}", _0)]
    Other(i32),
}

impl From<i32> for Error {
    fn from(err: i32) -> Self {
        match err {
            1 => Error::Breakpoint,
            2 => Error::Trace,
            768 => Error::CantOpen,
            770 => Error::InvalidImage,
            771 => Error::InvalidUpgrade,
            772 => Error::NoImage,
            774 => Error::InvalidRomSize,
            775 => Error::NotTiFile,
            776 => Error::Malloc,
            777 => Error::CantOpenDir,
            778 => Error::CantUpgrade,
            779 => Error::InvalidRom,
            780 => Error::CantOpenState,
            781 => Error::RevisionMatch,
            782 => Error::HeaderMatch,
            783 => Error::StateMatch,
            other => Error::Other(other),
        }
    }
}

impl From<NulError> for Error {
    fn from(err: NulError) -> Self {
        Error::NullError(err)
    }
}

pub type Result<T> = core::result::Result<T, Error>;
