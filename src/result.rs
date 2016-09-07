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
                write!(f, "Missing parameter: {}", param_name);
            },
            ListsError::InvalidParam => {
                write!(f, "InvalidParam");  // TODO: param_name?
            },
            ListsError::DatabaseError(ref err) => {
                write!(f, "Database Error: {}", err);
            }
            ListsError::DoesNotExist => {
                write!(f, "Does Not Exist");
            },
            ListsError::Unknown => {
                write!(f, "Unknown Error");
            },
            ListsError::InconsistentDatabase(ref more) => {
                write!(f, "Inconsistent Dababase: {}", more);
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
