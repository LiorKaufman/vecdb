use crate::vector::Vector3D;
use ordered_float::OrderedFloat;
use std::collections::BTreeMap;

pub struct Database {
    data: Vec<Vector3D>,
    magnitude_index: BTreeMap<OrderedFloat<f64>, Vec<usize>>,
}

impl Database {
    pub fn new() -> Database {
        Database {
            data: Vec::new(),
            magnitude_index: BTreeMap::new(),
        }
    }

    // Get
    pub fn get(&self, index: usize) -> Option<&Vector3D> {
        self.data.get(index)
    }
    // Set/Add
    pub fn set(&mut self, vector: Vector3D) {
        let magnitude = OrderedFloat(vector.magnitude());
        let index = self.data.len();
        self.data.push(vector);
        self.magnitude_index
            .entry(magnitude)
            .or_insert_with(Vec::new)
            .push(index);
    }

    // Update
    pub fn update(&mut self, index: usize, vector: &Vector3D) -> bool {
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
            true
        } else {
            false
        }
    }

    // Delete
    pub fn delete(&mut self, index: usize) -> bool {
        if let Some(old_vector) = self.data.get(index) {
            let old_magnitude = OrderedFloat(old_vector.magnitude());
            if let Some(indices) = self.magnitude_index.get_mut(&old_magnitude) {
                indices.retain(|&i| i != index);
            }
            self.data.remove(index);
            true
        } else {
            false
        }
    }

    // Get by magnitude
    pub fn get_by_magnitude(&self, magnitude: f64) -> Option<Vec<&Vector3D>> {
        self.magnitude_index
            .get(&OrderedFloat(magnitude))
            .map(|indices| indices.iter().filter_map(|&i| self.data.get(i)).collect())
    }
}
