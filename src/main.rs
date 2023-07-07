mod database;
mod logs;
mod vector;
use crate::database::Database;
use crate::logs::{LogEntry, Operation};
use crate::vector::Vector3D;

fn main() {
    let mut db = Database::new("log.txt".to_string());

    let v1 = Vector3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let v2 = Vector3D {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };
    let v3 = Vector3D {
        x: 7.0,
        y: 8.0,
        z: 9.0,
    };

    // Insert vectors
    db.set(v1.clone());
    db.set(v2.clone());
    db.set(v3.clone());

    // Check contents of the log
    db.flush_log().expect("Unable to flush log");
    // At this point, your log file should contain three insert operations

    // Update a vector
    let v1_updated = Vector3D {
        x: 10.0,
        y: 11.0,
        z: 12.0,
    };
    db.update(0, &v1_updated);

    // Delete a vector
    db.delete(2);

    // Check contents of the log again
    db.flush_log().expect("Unable to flush log");
    // Your log file should now additionally contain an update and a delete operation
}
