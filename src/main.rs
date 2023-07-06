mod database;
mod vector;

use crate::vector::Vector3D;
use database::Database;
fn main() {
    // Create a new Database
    let mut db = Database::new();

    // Create a new Vector3D
    let v1 = Vector3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };

    // Add the Vector3D to the Database
    db.set(v1);

    // Retrieve the Vector3D from the Database
    if let Some(vector) = db.get(0) {
        println!("Retrieved vector: {}", vector);
    }

    // Update the Vector3D in the Database
    let v2 = Vector3D {
        x: 4.0,
        y: 5.0,
        z: 6.0,
    };
    if db.update(0, &v2) {
        println!("Updated vector at index 0");
        if let Some(vector) = db.get(0) {
            println!("Retrieved updated vector: {}", vector);
        }
    }

    // Delete the Vector3D from the Database
    if db.delete(0) {
        println!("Deleted vector at index 0");
    }

    // Use the magnitude index to retrieve vectors
    let magnitude = v2.magnitude();
    if let Some(vectors) = db.get_by_magnitude(magnitude) {
        for vector in vectors {
            println!("Retrieved vector by magnitude: {}", vector);
        }
    }
}
