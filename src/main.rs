mod database;
mod logs;
mod vector;
use crate::database::Database;
use crate::vector::Vector3D;

fn main() {
    let log_path = String::from("./log.txt");
    let mut db = Database::new(log_path.clone());

    // Attempt to load previous data
    match db.load_from_log() {
        Ok(_) => println!("Loaded data from log"),
        Err(e) => println!("Couldn't load data from log: {}", e),
    }

    // Define some vectors
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

    // Add some vectors
    db.set(&v1, true);
    db.set(&v2, true);
    db.set(&v3, true);

    // Try retrieving a vector
    let retrieved_vector = db.get(1).unwrap();
    println!("Retrieved vector: {:?}", retrieved_vector);

    // Update a vector
    let v_update = retrieved_vector.cross(&v2);

    if db.update(0, &v_update, true) {
        println!("Updated vector at index 0");
    } else {
        println!("Couldn't update vector at index 0");
    }

    // Delete a vector
    if db.delete(2, true) {
        println!("Deleted vector at index 2");
    } else {
        println!("Couldn't delete vector at index 2");
    }

    // Flush log
    match db.flush_log() {
        Ok(_) => println!("Flushed log"),
        Err(e) => println!("Error flushing log: {}", e),
    }
}
