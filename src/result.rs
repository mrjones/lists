extern crate hyper;
extern crate mysql;
extern crate rustc_serialize;
extern crate std;

pub type ListsResult<T> = Result<T, ListsError>;

#[derive(Debug)]
pub enum ListsError {
    MissingParam(String),
    InvalidParam,
    DatabaseError(mysql::Error),
    IoError(std::io::Error),
    HyperError(hyper::error::Error),  // This seems wrong
    DoesNotExist,
    JsonDecodeError(rustc_serialize::json::DecoderError),
    JsonEncodeError(rustc_serialize::json::EncoderError),
    SystemTimeError(std::time::SystemTimeError),
    
    #[allow(dead_code)]
    Unknown(String),
}

impl std::fmt::Display for ListsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            ListsError::MissingParam(ref param_name) => {
                try!(write!(f, "Missing parameter: {}", param_name));
            },
            ListsError::InvalidParam => {
                try!(write!(f, "InvalidParam"));  // TODO: param_name?
            },
            ListsError::DatabaseError(ref err) => {
                try!(write!(f, "Database Error: {}", err));
            }
            ListsError::Unknown(ref more) => {
                try!(write!(f, "Unknown Error: {}", more));
            },
            ListsError::IoError(ref err) => {
                try!(write!(f, "IO Error: {}", err));
            },
            ListsError::HyperError(ref err) => {
                try!(write!(f, "HTTP Error: {}", err));
            },
            ListsError::DoesNotExist => {
                try!(write!(f, "Does not exist"));
            },
            ListsError::JsonDecodeError(ref err) => {
                try!(write!(f, "Json Decode Error: {}", err));
            },
            ListsError::JsonEncodeError(ref err) => {
                try!(write!(f, "Json Encode Error: {}", err));
            },
            ListsError::SystemTimeError(ref err) => {
                try!(write!(f, "SystemTime Error: {}", err));
            }

        }

        return Ok(());
    }
}

impl std::error::Error for ListsError {
    fn description(&self) -> &str {
        match *self {
            ListsError::MissingParam(_) => "MissingParam", 
            ListsError::InvalidParam => "InvalidParam",
            ListsError::DatabaseError(_) => "DatabaseError",
            ListsError::Unknown(_) => "Unknown",
            ListsError::IoError(_) => "IoError",
            ListsError::HyperError(_) => "HttpError",
            ListsError::DoesNotExist => "DoesNotExist",
            ListsError::JsonDecodeError(_) => "JsonDecodeError",
            ListsError::JsonEncodeError(_) => "JsonEncodeError",
            ListsError::SystemTimeError(_) => "SystemTimeError",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            ListsError::DatabaseError(ref err) => return Some(err),
            _ => return None,
        }
    }
}

impl std::convert::From<std::io::Error> for ListsError {
    fn from(err: std::io::Error) -> ListsError {
        return ListsError::IoError(err);
    }
}

impl std::convert::From<hyper::error::Error> for ListsError {
    fn from(err: hyper::error::Error) -> ListsError {
        return ListsError::HyperError(err);
    }
}

impl std::convert::From<rustc_serialize::json::DecoderError> for ListsError {
    fn from(err: rustc_serialize::json::DecoderError) -> ListsError {
        return ListsError::JsonDecodeError(err);
    }
}

impl std::convert::From<rustc_serialize::json::EncoderError> for ListsError {
    fn from(err: rustc_serialize::json::EncoderError) -> ListsError {
        return ListsError::JsonEncodeError(err);
    }
}


impl std::convert::From<std::time::SystemTimeError> for ListsError {
    fn from(err: std::time::SystemTimeError) -> ListsError {
        return ListsError::SystemTimeError(err);
    }
}


//impl std::fmt::Display for ListsError {
//    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//        return f.write_str(self.str());
//    }
//}
