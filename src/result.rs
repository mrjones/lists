extern crate std;
extern crate mysql;

pub type ListsResult<T> = Result<T, ListsError>;

#[derive(Debug)]
pub enum ListsError {
    MissingParam(String),
    InvalidParam,
    DatabaseError(mysql::Error),
    DoesNotExist,
    InconsistentDatabase(String),

    Unknown,
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
            ListsError::DoesNotExist => {
                try!(write!(f, "Does Not Exist"));
            },
            ListsError::Unknown => {
                try!(write!(f, "Unknown Error"));
            },
            ListsError::InconsistentDatabase(ref more) => {
                try!(write!(f, "Inconsistent Dababase: {}", more));
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
            ListsError::DoesNotExist => "DoesNotExist",
            ListsError::Unknown => "Unknown",
            ListsError::InconsistentDatabase(_) => "InconsistentDababase",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            ListsError::DatabaseError(ref err) => return Some(err),
            _ => return None,
        }
    }
}

//impl std::fmt::Display for ListsError {
//    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//        return f.write_str(self.str());
//    }
//}
