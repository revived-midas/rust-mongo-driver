//! MongoDB Errors and Error Codes.
use bson::{self, oid};
use coll::error::{WriteException, BulkWriteException};
use rustc_serialize::hex;
use std::{error, fmt, io, result, sync};

/// A type for results generated by MongoDB related functions, where the Err type is mongodb::Error.
pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum MaliciousServerErrorType {
    InvalidRnonce,
    InvalidServerSignature,
    NoServerSignature,
}

impl fmt::Display for MaliciousServerErrorType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MaliciousServerErrorType::InvalidRnonce => fmt.write_str("The server returned an invalid rnonce during authentication"),
            MaliciousServerErrorType::InvalidServerSignature => fmt.write_str("The server returned an invalid signature during authentication"),
            MaliciousServerErrorType::NoServerSignature => fmt.write_str("The server did not sign its reponse during authentication"),
        }
    }
}

/// The error type for MongoDB operations.
#[derive(Debug)]
pub enum Error {
    /// I/O operation errors of `Read`, `Write`, `Seek`, and associated traits.
    IoError(io::Error),
    /// A BSON struct could not be encoded.
    EncoderError(bson::EncoderError),
    /// A BSON struct could not be decoded.
    DecoderError(bson::DecoderError),
    /// An ObjectId could not be generated.
    OIDError(oid::Error),
    /// A hexadecimal string could not be converted to bytes.
    FromHexError(hex::FromHexError),
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
    /// The application failed to secure a mutex due to a poisoned lock.
    PoisonLockError,
    /// A server error with a given code.
    CodedError(ErrorCode),
    /// The client was unable to emit the events to the listeners due to a poisoned lock;
    /// all event listeners were dropped, so they will have to be registered again. If the
    /// client is unable to emit a failure result, the error it failed to report is bundled
    /// into the `EventListenerError`.
    EventListenerError(Option<Box<Error>>),
    /// The server that the client is attempting to authenticate to does not actually have
    /// the user's authentication information stored.
    MaliciousServerError(MaliciousServerErrorType),
    /// A standard error with a string description;
    /// a more specific error should generally be used.
    DefaultError(String),
}

impl<'a> From<Error> for io::Error {
    fn from(err: Error) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err)
    }
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

impl From<oid::Error> for Error {
    fn from(err: oid::Error) -> Error {
        Error::OIDError(err)
    }
}

impl From<hex::FromHexError> for Error {
    fn from(err: hex::FromHexError) -> Error {
        Error::FromHexError(err)
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl<T> From<sync::PoisonError<T>> for Error {
    fn from(_: sync::PoisonError<T>) -> Error {
        Error::PoisonLockError
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::WriteError(ref inner) => inner.fmt(fmt),
            Error::BulkWriteError(ref inner) => inner.fmt(fmt),
            Error::EncoderError(ref inner) => inner.fmt(fmt),
            Error::DecoderError(ref inner) => inner.fmt(fmt),
            Error::OIDError(ref inner) => inner.fmt(fmt),
            Error::FromHexError(ref inner) => inner.fmt(fmt),
            Error::IoError(ref inner) => inner.fmt(fmt),
            Error::ArgumentError(ref inner) => inner.fmt(fmt),
            Error::OperationError(ref inner) => inner.fmt(fmt),
            Error::ResponseError(ref inner) => inner.fmt(fmt),
            Error::CursorNotFoundError => write!(fmt, "No cursor found for cursor operation."),
            Error::PoisonLockError => write!(fmt, "Socket lock poisoned while attempting to access."),
            Error::CodedError(ref err) => write!(fmt, "{}", err),
            Error::EventListenerError(ref err) => match *err {
                Some(ref e) => write!(fmt, "Unable to emit failure due to poisoned lock; failure: {}", e),
                None => write!(fmt, "Unable to emit failure due to poisoned lock")
            },
            Error::MaliciousServerError(ref err) => write!(fmt, "{}", err),
            Error::DefaultError(ref inner) => inner.fmt(fmt),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::WriteError(ref inner) => inner.description(),
            Error::BulkWriteError(ref inner) => inner.description(),
            Error::EncoderError(ref inner) => inner.description(),
            Error::DecoderError(ref inner) => inner.description(),
            Error::OIDError(ref inner) => inner.description(),
            Error::FromHexError(ref inner) => inner.description(),
            Error::IoError(ref inner) => inner.description(),
            Error::ArgumentError(ref inner) => &inner,
            Error::OperationError(ref inner) => &inner,
            Error::ResponseError(ref inner) => &inner,
            Error::CursorNotFoundError => "No cursor found for cursor operation.",
            Error::PoisonLockError => "Socket lock poisoned while attempting to access.",
            Error::CodedError(ref err) => err.to_str(),
            Error::EventListenerError(ref err) => match *err {
                Some(_) => "Due to a poisoned lock on the listeners, unable to emit failure",
                None => "Due to a poisoned lock on the listeners, unable to emit event"
            },
            Error::MaliciousServerError(ref err) => match *err {
                MaliciousServerErrorType::InvalidRnonce => "The server returned an invalid rnonce during authentication",
                MaliciousServerErrorType::InvalidServerSignature => "The server returned an invalid signature during authentication",
                MaliciousServerErrorType::NoServerSignature => "The server did not sign its reponse during authentication",
            },
            Error::DefaultError(ref inner) => &inner,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::WriteError(ref inner) => Some(inner),
            Error::BulkWriteError(ref inner) => Some(inner),
            Error::EncoderError(ref inner) => Some(inner),
            Error::DecoderError(ref inner) => Some(inner),
            Error::OIDError(ref inner) => Some(inner),
            Error::FromHexError(ref inner) => Some(inner),
            Error::IoError(ref inner) => Some(inner),
            Error::ArgumentError(_) => None,
            Error::OperationError(_) => None,
            Error::ResponseError(_) => None,
            Error::CursorNotFoundError => None,
            Error::PoisonLockError => None,
            Error::CodedError(_) => None,
            Error::EventListenerError(_) => None,
            Error::MaliciousServerError(_) => None,
            Error::DefaultError(_) => None,
        }
    }
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum ErrorCode {
    OK = 0,
    InternalError = 1,
    BadValue = 2,
    OBSOLETE_DuplicateKey = 3,
    NoSuchKey = 4,
    GraphContainsCycle = 5,
    HostUnreachable = 6,
    HostNotFound = 7,
    UnknownError = 8,
    FailedToParse = 9,
    CannotMutateObject = 10,
    UserNotFound = 11,
    UnsupportedFormat = 12,
    Unauthorized = 13,
    TypeMismatch = 14,
    Overflow = 15,
    InvalidLength = 16,
    ProtocolError = 17,
    AuthenticationFailed = 18,
    CannotReuseObject = 19,
    IllegalOperation = 20,
    EmptyArrayOperation = 21,
    InvalidBSON = 22,
    AlreadyInitialized = 23,
    LockTimeout = 24,
    RemoteValidationError = 25,
    NamespaceNotFound = 26,
    IndexNotFound = 27,
    PathNotViable = 28,
    NonExistentPath = 29,
    InvalidPath = 30,
    RoleNotFound = 31,
    RolesNotRelated = 32,
    PrivilegeNotFound = 33,
    CannotBackfillArray = 34,
    UserModificationFailed = 35,
    RemoteChangeDetected = 36,
    FileRenameFailed = 37,
    FileNotOpen = 38,
    FileStreamFailed = 39,
    ConflictingUpdateOperators = 40,
    FileAlreadyOpen = 41,
    LogWriteFailed = 42,
    CursorNotFound = 43,
    UserDataInconsistent = 45,
    LockBusy = 46,
    NoMatchingDocument = 47,
    NamespaceExists = 48,
    InvalidRoleModification = 49,
    ExceededTimeLimit = 50,
    ManualInterventionRequired = 51,
    DollarPrefixedFieldName = 52,
    InvalidIdField = 53,
    NotSingleValueField = 54,
    InvalidDBRef = 55,
    EmptyFieldName = 56,
    DottedFieldName = 57,
    RoleModificationFailed = 58,
    CommandNotFound = 59,
    DatabaseNotFound = 60,
    ShardKeyNotFound = 61,
    OplogOperationUnsupported = 62,
    StaleShardVersion = 63,
    WriteConcernFailed = 64,
    MultipleErrorsOccurred = 65,
    ImmutableField = 66,
    CannotCreateIndex = 67,
    IndexAlreadyExists = 68,
    AuthSchemaIncompatible = 69,
    ShardNotFound = 70,
    ReplicaSetNotFound = 71,
    InvalidOptions = 72,
    InvalidNamespace = 73,
    NodeNotFound = 74,
    WriteConcernLegacyOK = 75,
    NoReplicationEnabled = 76,
    OperationIncomplete = 77,
    CommandResultSchemaViolation = 78,
    UnknownReplWriteConcern = 79,
    RoleDataInconsistent = 80,
    NoWhereParseContext = 81,
    NoProgressMade = 82,
    RemoteResultsUnavailable = 83,
    DuplicateKeyValue = 84,
    IndexOptionsConflict = 85,
    IndexKeySpecsConflict = 86,
    CannotSplit = 87,
    SplitFailed = 88,
    NetworkTimeout = 89,
    CallbackCanceled = 90,
    ShutdownInProgress = 91,
    SecondaryAheadOfPrimary = 92,
    InvalidReplicaSetConfig = 93,
    NotYetInitialized = 94,
    NotSecondary = 95,
    OperationFailed = 96,
    NoProjectionFound = 97,
    DBPathInUse = 98,
    WriteConcernNotDefined = 99,
    CannotSatisfyWriteConcern = 100,
    OutdatedClient = 101,
    IncompatibleAuditMetadata = 102,
    NewReplicaSetConfigurationIncompatible = 103,
    NodeNotElectable = 104,
    IncompatibleShardingMetadata = 105,
    DistributedClockSkewed = 106,
    LockFailed = 107,
    InconsistentReplicaSetNames = 108,
    ConfigurationInProgress = 109,
    CannotInitializeNodeWithData = 110,
    NotExactValueField = 111,
    WriteConflict = 112,
    InitialSyncFailure = 113,
    InitialSyncOplogSourceMissing = 114,
    CommandNotSupported = 115,
    DocTooLargeForCapped = 116,
    ConflictingOperationInProgress = 117,
    NamespaceNotSharded = 118,
    InvalidSyncSource = 119,
    OplogStartMissing = 120,
    DocumentValidationFailure = 121,
    OBSOLETE_ReadAfterOptimeTimeout = 122,
    NotAReplicaSet = 123,
    IncompatibleElectionProtocol = 124,
    CommandFailed = 125,
    RPCProtocolNegotiationFailed = 126,
    UnrecoverableRollbackError = 127,
    LockNotFound = 128,
    LockStateChangeFailed = 129,
    SymbolNotFound = 130,
    RLPInitializationFailed = 131,
    ConfigServersInconsistent = 132,
    FailedToSatisfyReadPreference = 133,
    XXX_TEMP_NAME_ReadCommittedCurrentlyUnavailable = 134,
    StaleTerm = 135,
    CappedPositionLost = 136,
    IncompatibleShardingConfigVersion = 137,
    RemoteOplogStale = 138,
    JSInterpreterFailure = 139,
    NotMaster = 10107,
    DuplicateKey = 11000,
    InterruptedAtShutdown = 11600,
    Interrupted = 11601,
    BackgroundOperationInProgressForDatabase = 12586,
    BackgroundOperationInProgressForNamespace = 12587,
    PrepareConfigsFailedCode = 13104,
    DatabaseDifferCase = 13297,
    ShardKeyTooBig = 13334,
    SendStaleConfig = 13388,
    NotMasterNoSlaveOkCode = 13435,
    NotMasterOrSecondaryCode = 13436,
    OutOfDiskSpace = 14031,
    KeyTooLong = 17280,
    MaxError
}

impl ErrorCode {
    pub fn is_network_error(&self) -> bool {
        *self == ErrorCode::HostUnreachable ||
        *self == ErrorCode::HostNotFound ||
        *self == ErrorCode::NetworkTimeout
    }

    pub fn is_interruption(&self) -> bool {
        *self == ErrorCode::Interrupted ||
        *self == ErrorCode::InterruptedAtShutdown ||
        *self == ErrorCode::ExceededTimeLimit
    }

    pub fn is_index_creation_error(&self) -> bool {
        *self == ErrorCode::CannotCreateIndex ||
        *self == ErrorCode::IndexOptionsConflict ||
        *self == ErrorCode::IndexKeySpecsConflict ||
        *self == ErrorCode::IndexAlreadyExists
    }

    fn to_str(&self) -> &str {
        match *self {
            ErrorCode::OK => "OK",
            ErrorCode::InternalError => "InternalError",
            ErrorCode::BadValue => "BadValue",
            ErrorCode::OBSOLETE_DuplicateKey => "OBSOLETE_DuplicateKey",
            ErrorCode::NoSuchKey => "NoSuchKey",
            ErrorCode::GraphContainsCycle => "GraphContainsCycle",
            ErrorCode::HostUnreachable => "HostUnreachable",
            ErrorCode::HostNotFound => "HostNotFound",
            ErrorCode::UnknownError => "UnknownError",
            ErrorCode::FailedToParse => "FailedToParse",
            ErrorCode::CannotMutateObject => "CannotMutateObject",
            ErrorCode::UserNotFound => "UserNotFound",
            ErrorCode::UnsupportedFormat => "UnsupportedFormat",
            ErrorCode::Unauthorized => "Unauthorized",
            ErrorCode::TypeMismatch => "TypeMismatch",
            ErrorCode::Overflow => "Overflow",
            ErrorCode::InvalidLength => "InvalidLength",
            ErrorCode::ProtocolError => "ProtocolError",
            ErrorCode::AuthenticationFailed => "AuthenticationFailed",
            ErrorCode::CannotReuseObject => "CannotReuseObject",
            ErrorCode::IllegalOperation => "IllegalOperation",
            ErrorCode::EmptyArrayOperation => "EmptyArrayOperation",
            ErrorCode::InvalidBSON => "InvalidBSON",
            ErrorCode::AlreadyInitialized => "AlreadyInitialized",
            ErrorCode::LockTimeout => "LockTimeout",
            ErrorCode::RemoteValidationError => "RemoteValidationError",
            ErrorCode::NamespaceNotFound => "NamespaceNotFound",
            ErrorCode::IndexNotFound => "IndexNotFound",
            ErrorCode::PathNotViable => "PathNotViable",
            ErrorCode::NonExistentPath => "NonExistentPath",
            ErrorCode::InvalidPath => "InvalidPath",
            ErrorCode::RoleNotFound => "RoleNotFound",
            ErrorCode::RolesNotRelated => "RolesNotRelated",
            ErrorCode::PrivilegeNotFound => "PrivilegeNotFound",
            ErrorCode::CannotBackfillArray => "CannotBackfillArray",
            ErrorCode::UserModificationFailed => "UserModificationFailed",
            ErrorCode::RemoteChangeDetected => "RemoteChangeDetected",
            ErrorCode::FileRenameFailed => "FileRenameFailed",
            ErrorCode::FileNotOpen => "FileNotOpen",
            ErrorCode::FileStreamFailed => "FileStreamFailed",
            ErrorCode::ConflictingUpdateOperators => "ConflictingUpdateOperators",
            ErrorCode::FileAlreadyOpen => "FileAlreadyOpen",
            ErrorCode::LogWriteFailed => "LogWriteFailed",
            ErrorCode::CursorNotFound => "CursorNotFound",
            ErrorCode::UserDataInconsistent => "UserDataInconsistent",
            ErrorCode::LockBusy => "LockBusy",
            ErrorCode::NoMatchingDocument => "NoMatchingDocument",
            ErrorCode::NamespaceExists => "NamespaceExists",
            ErrorCode::InvalidRoleModification => "InvalidRoleModification",
            ErrorCode::ExceededTimeLimit => "ExceededTimeLimit",
            ErrorCode::ManualInterventionRequired => "ManualInterventionRequired",
            ErrorCode::DollarPrefixedFieldName => "DollarPrefixedFieldName",
            ErrorCode::InvalidIdField => "InvalidIdField",
            ErrorCode::NotSingleValueField => "NotSingleValueField",
            ErrorCode::InvalidDBRef => "InvalidDBRef",
            ErrorCode::EmptyFieldName => "EmptyFieldName",
            ErrorCode::DottedFieldName => "DottedFieldName",
            ErrorCode::RoleModificationFailed => "RoleModificationFailed",
            ErrorCode::CommandNotFound => "CommandNotFound",
            ErrorCode::DatabaseNotFound => "DatabaseNotFound",
            ErrorCode::ShardKeyNotFound => "ShardKeyNotFound",
            ErrorCode::OplogOperationUnsupported => "OplogOperationUnsupported",
            ErrorCode::StaleShardVersion => "StaleShardVersion",
            ErrorCode::WriteConcernFailed => "WriteConcernFailed",
            ErrorCode::MultipleErrorsOccurred => "MultipleErrorsOccurred",
            ErrorCode::ImmutableField => "ImmutableField",
            ErrorCode::CannotCreateIndex => "CannotCreateIndex",
            ErrorCode::IndexAlreadyExists => "IndexAlreadyExists",
            ErrorCode::AuthSchemaIncompatible => "AuthSchemaIncompatible",
            ErrorCode::ShardNotFound => "ShardNotFound",
            ErrorCode::ReplicaSetNotFound => "ReplicaSetNotFound",
            ErrorCode::InvalidOptions => "InvalidOptions",
            ErrorCode::InvalidNamespace => "InvalidNamespace",
            ErrorCode::NodeNotFound => "NodeNotFound",
            ErrorCode::WriteConcernLegacyOK => "WriteConcernLegacyOK",
            ErrorCode::NoReplicationEnabled => "NoReplicationEnabled",
            ErrorCode::OperationIncomplete => "OperationIncomplete",
            ErrorCode::CommandResultSchemaViolation => "CommandResultSchemaViolation",
            ErrorCode::UnknownReplWriteConcern => "UnknownReplWriteConcern",
            ErrorCode::RoleDataInconsistent => "RoleDataInconsistent",
            ErrorCode::NoWhereParseContext => "NoWhereParseContext",
            ErrorCode::NoProgressMade => "NoProgressMade",
            ErrorCode::RemoteResultsUnavailable => "RemoteResultsUnavailable",
            ErrorCode::DuplicateKeyValue => "DuplicateKeyValue",
            ErrorCode::IndexOptionsConflict => "IndexOptionsConflict",
            ErrorCode::IndexKeySpecsConflict => "IndexKeySpecsConflict",
            ErrorCode::CannotSplit => "CannotSplit",
            ErrorCode::SplitFailed => "SplitFailed",
            ErrorCode::NetworkTimeout => "NetworkTimeout",
            ErrorCode::CallbackCanceled => "CallbackCanceled",
            ErrorCode::ShutdownInProgress => "ShutdownInProgress",
            ErrorCode::SecondaryAheadOfPrimary => "SecondaryAheadOfPrimary",
            ErrorCode::InvalidReplicaSetConfig => "InvalidReplicaSetConfig",
            ErrorCode::NotYetInitialized => "NotYetInitialized",
            ErrorCode::NotSecondary => "NotSecondary",
            ErrorCode::OperationFailed => "OperationFailed",
            ErrorCode::NoProjectionFound => "NoProjectionFound",
            ErrorCode::DBPathInUse => "DBPathInUse",
            ErrorCode::WriteConcernNotDefined => "WriteConcernNotDefined",
            ErrorCode::CannotSatisfyWriteConcern => "CannotSatisfyWriteConcern",
            ErrorCode::OutdatedClient => "OutdatedClient",
            ErrorCode::IncompatibleAuditMetadata => "IncompatibleAuditMetadata",
            ErrorCode::NewReplicaSetConfigurationIncompatible => "NewReplicaSetConfigurationIncompatible",
            ErrorCode::NodeNotElectable => "NodeNotElectable",
            ErrorCode::IncompatibleShardingMetadata => "IncompatibleShardingMetadata",
            ErrorCode::DistributedClockSkewed => "DistributedClockSkewed",
            ErrorCode::LockFailed => "LockFailed",
            ErrorCode::InconsistentReplicaSetNames => "InconsistentReplicaSetNames",
            ErrorCode::ConfigurationInProgress => "ConfigurationInProgress",
            ErrorCode::CannotInitializeNodeWithData => "CannotInitializeNodeWithData",
            ErrorCode::NotExactValueField => "NotExactValueField",
            ErrorCode::WriteConflict => "WriteConflict",
            ErrorCode::InitialSyncFailure => "InitialSyncFailure",
            ErrorCode::InitialSyncOplogSourceMissing => "InitialSyncOplogSourceMissing",
            ErrorCode::CommandNotSupported => "CommandNotSupported",
            ErrorCode::DocTooLargeForCapped => "DocTooLargeForCapped",
            ErrorCode::ConflictingOperationInProgress => "ConflictingOperationInProgress",
            ErrorCode::NamespaceNotSharded => "NamespaceNotSharded",
            ErrorCode::InvalidSyncSource => "InvalidSyncSource",
            ErrorCode::OplogStartMissing => "OplogStartMissing",
            ErrorCode::DocumentValidationFailure => "DocumentValidationFailure",
            ErrorCode::OBSOLETE_ReadAfterOptimeTimeout => "OBSOLETE_ReadAfterOptimeTimeout",
            ErrorCode::NotAReplicaSet => "NotAReplicaSet",
            ErrorCode::IncompatibleElectionProtocol => "IncompatibleElectionProtocol",
            ErrorCode::CommandFailed => "CommandFailed",
            ErrorCode::RPCProtocolNegotiationFailed => "RPCProtocolNegotiationFailed",
            ErrorCode::UnrecoverableRollbackError => "UnrecoverableRollbackError",
            ErrorCode::LockNotFound => "LockNotFound",
            ErrorCode::LockStateChangeFailed => "LockStateChangeFailed",
            ErrorCode::SymbolNotFound => "SymbolNotFound",
            ErrorCode::RLPInitializationFailed => "RLPInitializationFailed",
            ErrorCode::ConfigServersInconsistent => "ConfigServersInconsistent",
            ErrorCode::FailedToSatisfyReadPreference => "FailedToSatisfyReadPreference",
            ErrorCode::XXX_TEMP_NAME_ReadCommittedCurrentlyUnavailable => "XXX_TEMP_NAME_ReadCommittedCurrentlyUnavailable",
            ErrorCode::StaleTerm => "StaleTerm",
            ErrorCode::CappedPositionLost => "CappedPositionLost",
            ErrorCode::IncompatibleShardingConfigVersion => "IncompatibleShardingConfigVersion",
            ErrorCode::RemoteOplogStale => "RemoteOplogStale",
            ErrorCode::JSInterpreterFailure => "JSInterpreterFailure",
            ErrorCode::NotMaster => "NotMaster",
            ErrorCode::DuplicateKey => "DuplicateKey",
            ErrorCode::InterruptedAtShutdown => "InterruptedAtShutdown",
            ErrorCode::Interrupted => "Interrupted",
            ErrorCode::BackgroundOperationInProgressForDatabase => "BackgroundOperationInProgressForDatabase",
            ErrorCode::BackgroundOperationInProgressForNamespace => "BackgroundOperationInProgressForNamespace",
            ErrorCode::PrepareConfigsFailedCode => "PrepareConfigsFailedCode",
            ErrorCode::DatabaseDifferCase => "DatabaseDifferCase",
            ErrorCode::ShardKeyTooBig => "ShardKeyTooBig",
            ErrorCode::SendStaleConfig => "SendStaleConfig",
            ErrorCode::NotMasterNoSlaveOkCode => "NotMasterNoSlaveOkCode",
            ErrorCode::NotMasterOrSecondaryCode => "NotMasterOrSecondaryCode",
            ErrorCode::OutOfDiskSpace => "OutOfDiskSpace",
            ErrorCode::KeyTooLong => "KeyTooLong",
            ErrorCode::MaxError => "MaxError",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.write_str(self.to_str())
    }
}
