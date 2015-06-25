use bson;
use byteorder;

use client::coll::error::{WriteException, BulkWriteException};
use std::{error, fmt, io, result, sync};

/// A type for results generated by MongoDB related functions, where the Err type is mongodb::Error.
pub type Result<T> = result::Result<T, Error>;

/// The error type for MongoDB operations.
#[derive(Debug)]
pub enum Error {
    /// I/O operation errors of `Read`, `Write`, `Seek`, and associated traits.
    IoError(io::Error),
    /// A BSON struct could not be encoded.
    EncoderError(bson::EncoderError),
    /// A BSON struct could not be decoded.
    DecoderError(bson::DecoderError),
    /// A single-write operation failed.
    WriteError(WriteException),
    /// A bulk-write operation failed due to one or more lower-level write-related errors.
    BulkWriteError(BulkWriteException),
    /// An invalid function or operational argument was provided.
    ArgumentError(String),
    /// A database operation failed to send or receive a reply.
    OperationError(String),
    /// A database operation returned an invalid reply.
    ResponseError(String),
    /// A cursor operation failed to return a cursor.
    CursorNotFoundError,
    /// The application failed to secure the client connection socket due to a poisoned lock.
    PoisonLockError,
    /// A standard error with a string description;
    /// a more specific error should generally be used.
    DefaultError(String),
}

impl<'a> From<&'a str> for Error {
    fn from(s: &str) -> Error {
        Error::DefaultError(s.to_owned())
    }
}

impl From<String> for Error {
    fn from(s: String) -> Error {
        Error::DefaultError(s.to_owned())
    }
}

impl From<WriteException> for Error {
    fn from(err: WriteException) -> Error {
        Error::WriteError(err)
    }
}

impl From<BulkWriteException> for Error {
    fn from(err: BulkWriteException) -> Error {
        Error::BulkWriteError(err)
    }
}

impl From<bson::EncoderError> for Error {
    fn from(err: bson::EncoderError) -> Error {
        Error::EncoderError(err)
    }
}

impl From<bson::DecoderError> for Error {
    fn from(err: bson::DecoderError) -> Error {
        Error::DecoderError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<byteorder::Error> for Error {
    fn from(err: byteorder::Error) -> Error {
        Error::IoError(From::from(err))
    }
}

impl<T> From<sync::PoisonError<T>> for Error {
    fn from(_: sync::PoisonError<T>) -> Error {
        Error::PoisonLockError
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            &Error::WriteError(ref inner) => inner.fmt(fmt),
            &Error::BulkWriteError(ref inner) => inner.fmt(fmt),
            &Error::EncoderError(ref inner) => inner.fmt(fmt),
            &Error::DecoderError(ref inner) => inner.fmt(fmt),
            &Error::IoError(ref inner) => inner.fmt(fmt),
            &Error::ArgumentError(ref inner) => inner.fmt(fmt),
            &Error::OperationError(ref inner) => inner.fmt(fmt),
            &Error::ResponseError(ref inner) => inner.fmt(fmt),
            &Error::CursorNotFoundError => write!(fmt, "No cursor found for cursor operation."),
            &Error::PoisonLockError => write!(fmt, "Socket lock poisoned while attempting to access."),
            &Error::DefaultError(ref inner) => inner.fmt(fmt),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match self {
            &Error::WriteError(ref inner) => inner.description(),
            &Error::BulkWriteError(ref inner) => inner.description(),
            &Error::EncoderError(ref inner) => inner.description(),
            &Error::DecoderError(ref inner) => inner.description(),
            &Error::IoError(ref inner) => inner.description(),
            &Error::ArgumentError(ref inner) => &inner,
            &Error::OperationError(ref inner) => &inner,
            &Error::ResponseError(ref inner) => &inner,
            &Error::CursorNotFoundError => "No cursor found for cursor operation.",
            &Error::PoisonLockError => "Socket lock poisoned while attempting to access.",
            &Error::DefaultError(ref inner) => &inner,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match self {
            &Error::WriteError(ref inner) => Some(inner),
            &Error::BulkWriteError(ref inner) => Some(inner),
            &Error::EncoderError(ref inner) => Some(inner),
            &Error::DecoderError(ref inner) => Some(inner),
            &Error::IoError(ref inner) => Some(inner),
            &Error::ArgumentError(_) => None,
            &Error::OperationError(_) => None,
            &Error::ResponseError(_) => None,
            &Error::CursorNotFoundError => None,
            &Error::PoisonLockError => None,
            &Error::DefaultError(_) => None,
        }
    }
}