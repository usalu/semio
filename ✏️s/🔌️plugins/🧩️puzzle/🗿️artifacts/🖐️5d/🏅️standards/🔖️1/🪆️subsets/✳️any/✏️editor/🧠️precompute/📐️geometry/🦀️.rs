//! 📐️ Puzzle 5d spatial index over grip world positions: a uniform hash grid whose cell edge is the query
//! radius, so a radius query reads the 27 cells around its point and a nearest query grows ring by ring.
//! Inserting, removing and querying cost O(entries in the touched cells), never O(document).

use std::collections::HashMap;

/// 🧊️ A uniform 3d hash grid of points carrying a small copyable payload.
#[derive(Clone, Debug)]
pub struct Puzzle5dPointGrid<T> {
    cell: f64,
    cells: HashMap<[i64; 3], Vec<([f64; 3], T)>>,
    len: usize,
}

impl<T: Copy + PartialEq> Puzzle5dPointGrid<T> {
    /// 📏️ A grid whose cells are `cell` units wide; a non-positive or non-finite edge falls back to one unit.
    pub fn new(cell: f64) -> Self {
        Self { cell: if cell.is_finite() && cell > 0.0 { cell } else { 1.0 }, cells: HashMap::new(), len: 0 }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn key(&self, point: [f64; 3]) -> [i64; 3] {
        point.map(|axis| (axis / self.cell).floor() as i64)
    }

    pub fn insert(&mut self, point: [f64; 3], value: T) {
        let key = self.key(point);
        self.cells.entry(key).or_default().push((point, value));
        self.len += 1;
    }

    /// ✂️ Removes one entry equal to `(point, value)`; `false` when absent.
    pub fn remove(&mut self, point: [f64; 3], value: T) -> bool {
        let key = self.key(point);
        let Some(entries) = self.cells.get_mut(&key) else { return false };
        let Some(index) = entries.iter().position(|entry| entry.0 == point && entry.1 == value) else { return false };
        entries.swap_remove(index);
        if entries.is_empty() {
            self.cells.remove(&key);
        }
        self.len -= 1;
        true
    }

    fn ring(&self, center: [i64; 3], ring: i64) -> impl Iterator<Item = &([f64; 3], T)> {
        (-ring..=ring).flat_map(move |dx| (-ring..=ring).flat_map(move |dy| (-ring..=ring).map(move |dz| [dx, dy, dz]))).filter(move |offset| offset.iter().any(|axis| axis.abs() == ring)).filter_map(move |offset| self.cells.get(&[center[0] + offset[0], center[1] + offset[1], center[2] + offset[2]])).flatten()
    }

    /// 🎯️ Every entry within `radius` of `point` (inclusive), in no particular order.
    pub fn within(&self, point: [f64; 3], radius: f64) -> impl Iterator<Item = ([f64; 3], T)> + '_ {
        let reach = (radius.max(0.0) / self.cell).ceil() as i64;
        let center = self.key(point);
        let squared = radius * radius;
        (0..=reach).flat_map(move |ring| self.ring(center, ring)).filter(move |entry| distance_squared(entry.0, point) <= squared).copied()
    }

    /// 🧲️ The entry nearest to `point` within `max_radius`, or `None`. Ties keep the first entry found.
    pub fn nearest(&self, point: [f64; 3], max_radius: f64) -> Option<([f64; 3], T)> {
        let reach = (max_radius.max(0.0) / self.cell).ceil() as i64;
        let center = self.key(point);
        let mut best: Option<(f64, ([f64; 3], T))> = None;
        for ring in 0..=reach {
            if best.is_some_and(|(squared, _)| squared.sqrt() <= (ring - 1).max(0) as f64 * self.cell) {
                break;
            }
            for entry in self.ring(center, ring) {
                let squared = distance_squared(entry.0, point);
                if squared <= max_radius * max_radius && best.is_none_or(|(current, _)| squared < current) {
                    best = Some((squared, *entry));
                }
            }
        }
        best.map(|(_, entry)| entry)
    }
}

pub fn distance_squared(first: [f64; 3], second: [f64; 3]) -> f64 {
    (0..3).map(|axis| (first[axis] - second[axis]) * (first[axis] - second[axis])).sum()
}

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️unit/🦀️.rs"]
mod tests;
//#endregion 🧪️Tests
