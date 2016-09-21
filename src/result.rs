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
    JsonError(rustc_serialize::json::DecoderError),
    
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
            ListsError::JsonError(ref err) => {
                try!(write!(f, "Json Error: {}", err));
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
            ListsError::JsonError(_) => "JsonError",
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
        return ListsError::JsonError(err);
    }
}


//impl std::fmt::Display for ListsError {
//    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//        return f.write_str(self.str());
//    }
//}
