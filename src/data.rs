extern crate std;
extern crate mysql;

use model::DbObject;
use model::List;
use model::User;
use util::to_vector;

pub struct Db {
    pub conn: Box<mysql::Pool>
}

impl Db {
    pub fn fetch_all_lists(&self, user: &User) -> Result<Vec<List>, mysql::Error> {
        return to_vector::<List>(
            try!(self.conn.prep_exec("SELECT lists.lists.id, lists.lists.name FROM lists.list_users LEFT JOIN lists.lists ON lists.list_users.list_id = lists.lists.id WHERE lists.list_users.user_id = ?", (user.id,))));
    }

    pub fn fetch_all_users(&self, ) -> Result<Vec<User>, mysql::Error> {
        return to_vector::<User>(
            try!(self.conn.prep_exec("SELECT id, name FROM lists.users", ())))
    }

    pub fn lookup_user(&self, id: i64) -> Result<User, mysql::Error> {
        let mut result = try!(self.conn.prep_exec("SELECT id, name FROM lists.users WHERE id = ?", (id,)));
        let row = result.next().expect("reading row");
        let user = User::from_row(try!(row));
        assert!(result.next().is_none(), "Duplicate user id!");
        return Ok(user);
    }
}
