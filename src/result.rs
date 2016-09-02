extern crate std;

pub type ListsResult<T> = Result<T, ListsError>;

#[derive(Debug, Clone)]
pub enum ListsError {
    MissingParam(String),
    InvalidParam,
    DatabaseError,
    DoesNotExist,
    InconsistentDatabase(String),

    Unknown,
}

impl ListsError {
    fn str(&self) -> &str {
        match *self {
            // TODO(mrjones): print out which param is actually missing?
            ListsError::MissingParam(_) => "MissingParam", 
            ListsError::InvalidParam => "InvalidParam",
            ListsError::DatabaseError => "DatabaseError",
            ListsError::DoesNotExist => "DoesNotExist",
            ListsError::Unknown => "Unknown",
            ListsError::InconsistentDatabase(_) => "InconsistentDababase",
        }
    }
}



impl std::error::Error for ListsError {
    fn description(&self) -> &str {
        return self.str();
    }

    fn cause(&self) -> Option<&std::error::Error> {
        return None;
    }
}

impl std::fmt::Display for ListsError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        return f.write_str(self.str());
    }
}
