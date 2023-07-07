use crate::logs::{LogEntry, Operation};
use crate::vector::Vector3D;
use ordered_float::OrderedFloat;
use serde_json::to_string;
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufWriter, Write};

pub struct Database {
    data: Vec<Vector3D>,
    log: Vec<LogEntry>,
    log_path: String,
    magnitude_index: BTreeMap<OrderedFloat<f64>, Vec<usize>>,
}

impl Database {
    pub fn new(log_path: String) -> Database {
        Database {
            data: Vec::new(),
            log: Vec::new(),
            log_path,
            magnitude_index: BTreeMap::new(),
        }
    }

    // Get
    pub fn get(&self, index: usize) -> Option<&Vector3D> {
        self.data.get(index)
    }
    pub fn log_operation(&mut self, id: usize, operation: Operation, vector: Vector3D) {
        let log_entry = LogEntry {
            id,
            operation,
            vector,
        };
        self.log.push(log_entry);
    }

    // Set/Add
    pub fn set(&mut self, vector: Vector3D) {
        let magnitude = OrderedFloat(vector.magnitude());
        let index = self.data.len();
        self.data.push(vector.clone());
        self.magnitude_index
            .entry(magnitude)
            .or_insert_with(Vec::new)
            .push(index);
        self.log_operation(index, Operation::Insert, vector);
    }

    // Update
    pub fn update(&mut self, index: usize, vector: &Vector3D) -> bool {
        // Rest of your code...
        self.log_operation(index, Operation::Update, vector.clone());
        true
    }

    // Delete
    pub fn delete(&mut self, index: usize) -> bool {
        // Rest of your code...
        self.log_operation(index, Operation::Delete, self.data[index].clone());
        true
    }

    pub fn flush_log(&mut self) -> std::io::Result<()> {
        let file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(&self.log_path)?;

        let mut writer = BufWriter::new(file);
        for entry in &self.log {
            let serialized = to_string(entry)?;
            writer.write_all(serialized.as_bytes())?;
            writer.write_all(b"\n")?;
        }
        self.log.clear();
        Ok(())
    }
}
