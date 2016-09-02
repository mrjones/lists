extern crate std;
extern crate mysql;

use model::List;
use model::User;
use util::to_vector;

pub fn fetch_all_lists(user: &User, conn: &mysql::Pool) -> Result<Vec<List>, mysql::Error> {
    return to_vector::<List>(
        try!(conn.prep_exec("SELECT lists.lists.id, lists.lists.name FROM lists.list_users LEFT JOIN lists.lists ON lists.list_users.list_id = lists.lists.id WHERE lists.list_users.user_id = ?", (user.id,))));
}

pub fn fetch_all_users(conn: &mysql::Pool) -> Result<Vec<User>, mysql::Error> {
    return to_vector::<User>(
        try!(conn.prep_exec("SELECT id, name FROM lists.users", ())))
}
