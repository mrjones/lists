extern crate std;

use data;
use result; 

pub struct Task {
    id: i64,
    payload: Vec<u8>,
}

pub trait WorkQueue {
    fn enqueue(&self, payload: &[u8]) -> result::ListsResult<()>;
    fn dequeue(&self) -> Option<(Task, std::time::SystemTime)>;
    fn extend_lease(&self, id: i64) -> result::ListsResult<()>;
    fn finish(&self, id: i64) -> result::ListsResult<()>;
    fn abort(&self, id: i64) -> result::ListsResult<()>;
}

pub struct DbWorkQueue {
    lease_duration: std::time::Duration,
    queue_name: String,
    db: std::sync::Arc<std::sync::Mutex<data::Db>>,
}

impl DbWorkQueue {
    pub fn new(
        lease_duration: std::time::Duration,
        queue_name: &str,
        db: std::sync::Arc<std::sync::Mutex<data::Db>>) -> DbWorkQueue {
        return DbWorkQueue{
            lease_duration: lease_duration,
            queue_name: queue_name.to_string(),
            db: db,
        }
    }
}

impl WorkQueue for DbWorkQueue {
    fn enqueue(&self, payload: &[u8]) -> result::ListsResult<()> {
        return self.db.lock().unwrap().enqueue_work(&self.queue_name, payload);
    }

    fn dequeue(&self) -> Option<(Task, std::time::SystemTime)> {
        match self.db.lock().unwrap().dequeue_work(
            &self.queue_name, self.lease_duration).unwrap_or(None) {
            None => return None,
            Some(lease) => return Some((Task{
                id: lease.id,
                payload: lease.payload
            }, lease.expiration)),
        }
    }

    fn extend_lease(&self, id: i64) -> result::ListsResult<()> {
        return self.db.lock().unwrap().extend_lease(id, self.lease_duration);
    }

    fn abort(&self, id: i64) -> result::ListsResult<()> {
        return self.db.lock().unwrap().abort_lease(id);
    }

    fn finish(&self, id: i64) -> result::ListsResult<()> {
        return self.db.lock().unwrap().finish_task(id);
    }
}
