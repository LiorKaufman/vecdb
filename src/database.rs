use crate::logs::{LogEntry, Operation};
use crate::vector::Vector3D;
use ordered_float::OrderedFloat;
use serde_json::{from_str, to_string};
use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, BufWriter, Write};

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

    // Set/Add
    pub fn set(&mut self, vector: &Vector3D, log_operation: bool) {
        let magnitude = OrderedFloat(vector.magnitude());
        let index = self.data.len();
        self.data.push(vector.clone());
        self.magnitude_index
            .entry(magnitude)
            .or_insert_with(Vec::new)
            .push(index);

        if log_operation {
            self.log_operation(index, Operation::Insert, vector.clone());
        }
    }

    // Update
    pub fn update(&mut self, index: usize, vector: &Vector3D, log_operation: bool) -> bool {
        if let Some(old_vector) = self.data.get(index) {
            let old_magnitude = OrderedFloat(old_vector.magnitude());
            if let Some(indices) = self.magnitude_index.get_mut(&old_magnitude) {
                indices.retain(|&i| i != index);
            }
        }

        if let Some(element) = self.data.get_mut(index) {
            let new_magnitude = OrderedFloat(vector.magnitude());
            *element = vector.clone();
            self.magnitude_index
                .entry(new_magnitude)
                .or_insert_with(Vec::new)
                .push(index);

            if log_operation {
                self.log_operation(index, Operation::Update, vector.clone());
            }

            true
        } else {
            false
        }
    }

    // Delete
    pub fn delete(&mut self, index: usize, log_operation: bool) -> bool {
        if let Some(old_vector) = self.data.get(index) {
            let old_magnitude = OrderedFloat(old_vector.magnitude());
            if let Some(indices) = self.magnitude_index.get_mut(&old_magnitude) {
                indices.retain(|&i| i != index);
            }

            if log_operation {
                self.log_operation(index, Operation::Delete, old_vector.clone());
            }

            self.data.remove(index);
            true
        } else {
            false
        }
    }

    pub fn log_operation(&mut self, id: usize, operation: Operation, vector: Vector3D) {
        let log_entry = LogEntry {
            id,
            operation,
            vector,
        };
        self.log.push(log_entry);
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

    pub fn load_from_log(&mut self) -> std::io::Result<()> {
        let file = File::open(&self.log_path)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let log_entry: LogEntry = from_str(&line)?;

            match log_entry.operation {
                Operation::Insert => {
                    self.set(&log_entry.vector, false);
                }
                Operation::Update => {
                    self.update(log_entry.id, &log_entry.vector, false);
                }
                Operation::Delete => {
                    self.delete(log_entry.id, false);
                }
            }
        }

        Ok(())
    }
}
