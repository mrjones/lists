extern crate std;
extern crate mysql;

use model::DbObject;
use model::FullItem;
use model::FullLinkAnnotation;
use model::FullList;
use model::Annotation;
use model::Item;
use model::List;
use model::User;
use util::to_vector;

pub struct Db {
    pub conn: Box<mysql::Pool>
}

pub type DbResult<T> = Result<T, mysql::Error>;

type DbItem = Item;
type DbAnnotation = Annotation;

impl Db {
    pub fn fetch_all_lists(&self, user: &User) -> DbResult<Vec<List>> {
        return to_vector::<List>(
            try!(self.conn.prep_exec("SELECT lists.lists.id, lists.lists.name FROM lists.list_users LEFT JOIN lists.lists ON lists.list_users.list_id = lists.lists.id WHERE lists.list_users.user_id = ?", (user.id,))));
    }

    pub fn fetch_all_users(&self, ) -> DbResult<Vec<User>> {
        return to_vector::<User>(
            try!(self.conn.prep_exec("SELECT id, name FROM lists.users", ())))
    }

    pub fn lookup_user(&self, id: i64) -> DbResult<User> {
        let mut result = try!(self.conn.prep_exec("SELECT id, name FROM lists.users WHERE id = ?", (id,)));
        let row = result.next().expect("reading row");
        let user = User::from_row(try!(row));
        assert!(result.next().is_none(), "Duplicate user id!");
        return Ok(user);
    }

    pub fn lookup_list(&self, list_id: i64) -> DbResult<FullList> {
        let mut list_result = try!(self.conn.prep_exec("SELECT id, name FROM lists.lists WHERE id = ?", (list_id,)));
        let row = list_result.next().expect("reading row");
        let list = List::from_row(try!(row));
        assert!(list_result.next().is_none(), "Duplicate list");
        
        let db_items = try!(to_vector::<DbItem>(
            try!(self.conn.prep_exec("SELECT id, name, description FROM lists.items WHERE list_id = ?", (list_id,)))));

        let db_annotations = try!(to_vector::<DbAnnotation>(
            try!(self.conn.prep_exec("SELECT lists.item_annotations.id, lists.items.id, lists.item_annotations.kind, lists.item_annotations.body FROM lists.items JOIN lists.item_annotations ON lists.items.id = lists.item_annotations.item_id WHERE lists.items.list_id = ?", (list_id,)))));

        let mut full_items : Vec<FullItem> = vec![];
        
        for db_item in db_items {
            full_items.push(FullItem{
                id: db_item.id,
                name: db_item.name,
                description: db_item.description,
                link_annotations: vec![],
            });
        }
        

        for db_annotation in db_annotations {
            let index = full_items.binary_search_by_key(
                &db_annotation.item_id, |item| item.id)
                .expect("dangling annotation");

            if db_annotation.kind == "LINK" {
                full_items[index].link_annotations.push(
                    FullLinkAnnotation{url: db_annotation.body});
            }
        }

        return Ok(FullList {
            name: list.name,
            items: full_items,
        });
    }

    pub fn add_item(&self, list_id: i64, name: &str, description: &str) -> DbResult<()> {
        let _ = try!(self.conn.prep_exec("INSERT INTO lists.items (list_id, name, description) VALUES (?, ?, ?)", (list_id, name, description)));
        return Ok(());
    }
}
