extern crate mysql;
extern crate std;

use model::DbObject;

pub fn to_vector<T: DbObject>(query_result: mysql::QueryResult) -> mysql::error::Result<Vec<T>> {
    return query_result.map(|row_result| {
        return Ok(T::from_row(try!(row_result)));
    }).collect();
}
