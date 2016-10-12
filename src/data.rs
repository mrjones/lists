extern crate chrono;
extern crate std;
extern crate mysql;

use model::DbObject;
use model::DbWorkQueueLease;
use model::DbWorkQueueTask;
use model::FullList;
use model::Annotation;
use model::AutoAnnotation;
use model::Item;
use model::List;
use model::User;

use result::ListsError;
use result::ListsResult;

pub struct Db {
    pub conn: Box<mysql::Pool>
}

pub type DbResult<T> = Result<T, mysql::Error>;

type DbItem = Item;
type DbAnnotation = Annotation;

macro_rules! dbtry {
    ($result:expr) => (match $result {
        ::std::result::Result::Ok(val) => val,
        ::std::result::Result::Err(err) => return ::std::result::Result::Err(
            ListsError::DatabaseError(err)),
    })
}

fn extract_one_or_none<T: DbObject>(result: &mut mysql::QueryResult) -> ListsResult<Option<T>> {
    match result.next() {
        Some(row) => {
            let obj = T::from_row(dbtry!(row));
            assert!(result.next().is_none(), "Duplicate entry");
            return Ok(Some(obj));
        },
        None => return Ok(None),
    }
}

fn extract_one<T: DbObject>(result: &mut mysql::QueryResult) -> ListsResult<T> {
    match try!(extract_one_or_none(result)) {
        Some(result) => return Ok(result),
        None => return Err(ListsError::DoesNotExist),
    }
}

fn to_vec<T: DbObject>(query_result: mysql::QueryResult) -> ListsResult<Vec<T>> {
    return query_result.map(|row_result| {
        return Ok(T::from_row(dbtry!(row_result)));
    }).collect();
}


impl Db {
    pub fn fetch_all_lists(&self, user: &User) -> ListsResult<Vec<List>> {
        return to_vec::<List>(
            dbtry!(self.conn.prep_exec("SELECT lists.lists.id, lists.lists.name FROM lists.list_users LEFT JOIN lists.lists ON lists.list_users.list_id = lists.lists.id WHERE lists.list_users.user_id = ?", (user.id,))));
    }

    pub fn fetch_all_users(&self, ) -> ListsResult<Vec<User>> {
        return to_vec::<User>(
            dbtry!(self.conn.prep_exec("SELECT id, name FROM lists.users", ())))
    }

    pub fn lookup_user(&self, id: i64) -> ListsResult<User> {
        let mut result = dbtry!(self.conn.prep_exec("SELECT id, name FROM lists.users WHERE id = ?", (id,)));
        return extract_one(&mut result);
    }

    // TODO(mrjones): Clean up the return type
    pub fn lookup_list(&self, list_id: i64) ->
        ListsResult<(FullList, Vec<Item>, Vec<Annotation>, Vec<AutoAnnotation>)> {
        let mut list_result = dbtry!(self.conn.prep_exec("SELECT id, name FROM lists.lists WHERE id = ?", (list_id,)));
        let list = try!(extract_one::<List>(&mut list_result));
        
        let db_items = try!(to_vec::<DbItem>(
            dbtry!(self.conn.prep_exec("SELECT id, name, description FROM lists.items WHERE list_id = ?", (list_id,)))));

        let db_annotations = try!(to_vec::<DbAnnotation>(
            dbtry!(self.conn.prep_exec("SELECT lists.item_annotations.id, lists.items.id, lists.item_annotations.kind, lists.item_annotations.body FROM lists.items JOIN lists.item_annotations ON lists.items.id = lists.item_annotations.item_id WHERE lists.items.list_id = ?", (list_id,)))));

        let db_auto_annotations = try!(to_vec::<AutoAnnotation>(
            dbtry!(self.conn.prep_exec("SELECT lists.item_auto_annotations.id, lists.items.id, lists.item_auto_annotations.parent_id, lists.item_auto_annotations.kind, lists.item_auto_annotations.body, lists.item_auto_annotations.mtime FROM lists.items JOIN lists.item_auto_annotations ON lists.items.id = lists.item_auto_annotations.item_id WHERE lists.items.list_id = ?", (list_id,)))));

        return Ok((FullList {
            name: list.name,
            items: vec![],
        }, db_items.clone(), db_annotations, db_auto_annotations));
    }
    
    pub fn lookup_list_item(&self, list_id: i64, item_id: i64) -> ListsResult<(Item, Vec<Annotation>, Vec<AutoAnnotation>)> {
        let mut item_result =
            dbtry!(self.conn.prep_exec("SELECT id, name, description FROM lists.items WHERE list_id = ? AND id = ?", (list_id, item_id)));
        let db_item = try!(extract_one::<DbItem>(&mut item_result));

        let db_annotations = try!(to_vec::<DbAnnotation>(
            dbtry!(self.conn.prep_exec("SELECT lists.item_annotations.id, lists.items.id, lists.item_annotations.kind, lists.item_annotations.body FROM lists.items JOIN lists.item_annotations ON lists.items.id = lists.item_annotations.item_id WHERE lists.items.list_id = ? AND lists.items.id = ?", (list_id,item_id)))));

        let db_auto_annotations = try!(to_vec::<AutoAnnotation>(
            dbtry!(self.conn.prep_exec("SELECT lists.item_auto_annotations.id, lists.items.id, lists.item_auto_annotations.parent_id, lists.item_auto_annotations.kind, lists.item_auto_annotations.body, lists.item_auto_annotations.mtime FROM lists.items JOIN lists.item_auto_annotations ON lists.items.id = lists.item_auto_annotations.item_id WHERE lists.items.list_id = ? AND lists.items.id = ?", (list_id,item_id)))));

        return Ok((db_item, db_annotations, db_auto_annotations));
    }

    pub fn fetch_list_accessors(&self, list_id: i64) -> ListsResult<Vec<User>> {
        return to_vec::<User>(
            dbtry!(self.conn.prep_exec("SELECT lists.users.id, lists.users.name FROM lists.list_users LEFT JOIN lists.users ON lists.list_users.user_id = lists.users.id WHERE lists.list_users.list_id = ?", (list_id,))));
    }

    pub fn add_list(&self, list_name: &str, owner_id: i64) -> ListsResult<List> {
        let mut conn = self.conn.get_conn().unwrap();
        let _ = dbtry!(conn.prep_exec("INSERT INTO lists.lists (name) VALUES (?)", (list_name,)));

        let ret;
        {
            let mut list_result = dbtry!(conn.prep_exec("SELECT id, name FROM lists.lists WHERE id = LAST_INSERT_ID()", ()));
            ret = extract_one::<List>(&mut list_result);
        }

        let _ = dbtry!(conn.prep_exec("INSERT INTO lists.list_users (user_id, list_id) VALUES (?, LAST_INSERT_ID())", (owner_id, )));

        return ret;
    }
    
    pub fn add_item(&self, list_id: i64, name: &str, description: &str) -> ListsResult<Item> {
        let mut conn = self.conn.get_conn().unwrap();
        let _ = dbtry!(conn.prep_exec("INSERT INTO lists.items (list_id, name, description) VALUES (?, ?, ?)", (list_id, name, description)));

        let mut result = dbtry!(conn.prep_exec("SELECT id, name, description FROM lists.items WHERE id = LAST_INSERT_ID()", ()));
        return extract_one::<DbItem>(&mut result);
    }

    pub fn delete_list(&self, list_id: i64) -> ListsResult<()> {
        let _ = dbtry!(self.conn.prep_exec("DELETE FROM lists.item_auto_annotations WHERE item_id IN (SELECT id FROM lists.items WHERE list_id = ?)", (list_id,)));
        let _ = dbtry!(self.conn.prep_exec("DELETE FROM lists.item_annotations WHERE item_id IN (SELECT id FROM lists.items WHERE list_id = ?)", (list_id,)));
        let _ = dbtry!(self.conn.prep_exec("DELETE FROM lists.items WHERE list_id = ?", (list_id,)));
        let _ = dbtry!(self.conn.prep_exec("DELETE FROM lists.list_users WHERE list_id = ?", (list_id,)));
        let _ = dbtry!(self.conn.prep_exec("DELETE FROM lists.lists WHERE id = ?", (list_id,)));
        return Ok(());        
    }
    
    pub fn delete_item(&self, item_id: i64) -> ListsResult<()> {
        let _ = dbtry!(self.conn.prep_exec("DELETE FROM lists.item_auto_annotations WHERE item_id = ?", (item_id,)));
        let _ = dbtry!(self.conn.prep_exec("DELETE FROM lists.item_annotations WHERE item_id = ?", (item_id,)));
        let _ = dbtry!(self.conn.prep_exec("DELETE FROM lists.items WHERE id = ?", (item_id,)));
        return Ok(());
    }

    pub fn add_annotation(&self, item_id: i64, kind: &str, body: &str) -> ListsResult<Annotation> {
        let mut conn = self.conn.get_conn().unwrap();
        // TODO: create a "kind" enum
        let _ = dbtry!(conn.prep_exec("INSERT INTO lists.item_annotations (item_id, kind, body) VALUES (?, ?, ?)", (item_id, kind, body)));

        let mut result = dbtry!(conn.prep_exec("SELECT id, item_id, kind, body FROM lists.item_annotations WHERE id = LAST_INSERT_ID()", ()));
        return extract_one::<Annotation>(&mut result);
    }

    pub fn add_auto_annotation(&self, item_id: i64, parent_id: i64, kind: &str, body: &[u8]) -> ListsResult<AutoAnnotation> {
        let mut conn = self.conn.get_conn().unwrap();
        let _ = dbtry!(conn.prep_exec("INSERT INTO lists.item_auto_annotations (item_id, parent_id, kind, body) VALUES (?, ?, ?, ?)", (item_id, parent_id, kind, body)));
        let mut result = dbtry!(conn.prep_exec("SELECT id, item_id, parent_id, kind, body, mtime FROM lists.item_auto_annotations WHERE id = LAST_INSERT_ID()", ()));
        return extract_one::<AutoAnnotation>(&mut result);
    }

    pub fn add_user_to_list(&self, list_id: i64, user_id: i64) -> ListsResult<()> {
        let mut conn = self.conn.get_conn().unwrap();
        let _ = dbtry!(conn.prep_exec("INSERT INTO lists.list_users (list_id, user_id) VALUES (?, ?)", (list_id, user_id)));

        return Ok(());
    }

    pub fn remove_user_from_list(&self, list_id: i64, user_id: i64) -> ListsResult<()> {
        let mut conn = self.conn.get_conn().unwrap();
        let _ = dbtry!(conn.prep_exec("DELETE FROM lists.list_users WHERE list_id = ? AND user_id = ?", (list_id, user_id)));

        return Ok(());
    }


    pub fn enqueue_work(&self, queue_name: &str, payload: &[u8]) -> ListsResult<()> {
        let mut conn = self.conn.get_conn().unwrap();
        let _ = dbtry!(conn.prep_exec("INSERT INTO lists.work_queue (queue_name, payload) VALUES (?, ?)", (queue_name, payload))); 

        return Ok(());
    }

    pub fn dequeue_work(&self, queue_name: &str, lease_duration: std::time::Duration) -> ListsResult<Option<DbWorkQueueLease>> {
        let mut conn = self.conn.get_conn().unwrap();
        let mut txn = dbtry!(conn.start_transaction(true, None, None));

        let task;
        {
            // TODO: ignore (and clean up) expired leases.
            let mut result = dbtry!(txn.prep_exec("SELECT lists.work_queue.id, lists.work_queue.payload FROM lists.work_queue LEFT OUTER JOIN lists.work_leases ON lists.work_queue.id = lists.work_leases.item_id WHERE lists.work_leases.id IS NULL AND lists.work_queue.queue_name = ? ORDER BY id ASC LIMIT 1 FOR UPDATE", (queue_name,)));

            match try!(extract_one_or_none::<DbWorkQueueTask>(&mut result)) {
                None => return Ok(None),
                Some(t) => task = t,
            }
        }

        let expiration = std::time::SystemTime::now() + lease_duration;
        let epoch_expiration = expiration.duration_since(std::time::UNIX_EPOCH).unwrap();
        {
            let _ = dbtry!(txn.prep_exec("INSERT INTO lists.work_leases (item_id, epoch_expiration) VALUES (?, ?)", (task.id, epoch_expiration)));
        }

        let lease;
        {
            let mut lease_result = dbtry!(txn.prep_exec("SELECT lists.work_leases.id, lists.work_queue.payload, lists.work_leases.epoch_expiration FROM lists.work_leases JOIN lists.work_queue ON lists.work_queue.id = lists.work_leases.item_id WHERE lists.work_leases.id = LAST_INSERT_ID()", ()));
            lease = try!(extract_one(&mut lease_result));
        }
        let _ = dbtry!(txn.commit());

        return Ok(Some(lease));
    }

    pub fn extend_lease(&self, id: i64, lease_duration: std::time::Duration) -> ListsResult<()> {
        let expiration = std::time::SystemTime::now() + lease_duration;
        let epoch_expiration = expiration.duration_since(std::time::UNIX_EPOCH).unwrap();

        let mut conn = self.conn.get_conn().unwrap();
        let _ = dbtry!(conn.prep_exec("UPDATE lists.work_queue SET epoch_expiration = ? WHERE id = ?", (epoch_expiration, id))); 

        return Ok(());
    }

    pub fn abort_lease(&self, id: i64) -> ListsResult<()> {
        let _ = dbtry!(self.conn.prep_exec("DELETE FROM lists.work_leases WHERE id =?", (id,)));
        return Ok(());
    }

    pub fn finish_task(&self, id: i64) -> ListsResult<()> {
        let mut conn = self.conn.get_conn().unwrap();
        let mut txn = dbtry!(conn.start_transaction(true, None, None));
        let item_id;
        {
            let mut result = dbtry!(txn.prep_exec("SELECT item_id FROM lists.work_leases WHERE id = ?", (id,)));
            item_id = try!(extract_one::<i64>(&mut result));
        }
        {
            let _ = dbtry!(txn.prep_exec("DELETE FROM lists.work_leases WHERE id = ?", (id,)));
        }
        {
            let _ = dbtry!(txn.prep_exec("DELETE FROM lists.work_queue WHERE id = ?", (item_id,)));
        }
        let _ = dbtry!(txn.commit());

        return Ok(());
    }
}

impl DbObject for i64 {
    fn from_row(row: mysql::Row) -> i64 {
        return mysql::from_row(row);
    }
}
