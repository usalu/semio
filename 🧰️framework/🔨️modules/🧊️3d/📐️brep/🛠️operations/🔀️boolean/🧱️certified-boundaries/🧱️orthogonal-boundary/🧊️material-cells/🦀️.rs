//! 🧊️ Exact reusable rectilinear material over a bounded arrangement.

use super::*;

const PLANES: usize = 8;
const INTERVALS: usize = PLANES - 1;
const CELLS: usize = INTERVALS * INTERVALS * INTERVALS;

#[derive(Clone, Copy)]
/// 💎️ Identifies only the actual material cells of a certified closed orthogonal solid.
pub struct OrthogonalCells { axes: Axes, occupied: u32 }

semio_framework_value::artifact_retire_leaf!(OrthogonalCells);

impl OrthogonalCells {
    pub(super) fn new(axes: Axes, occupied: u32) -> Self { Self { axes, occupied } }

    /// 📦️ Encodes one finite stock box in the same exact material representation.
    pub fn from_box(bounds: Bounds) -> Result<Self, ValueError> {
        if !ordered(bounds) { return Err(unsupported()); }
        Ok(Self::new(std::array::from_fn(|axis| [bounds[0][axis], bounds[1][axis], f64::NAN, f64::NAN]), 1))
    }

    /// 📏️ Reads the single-box result only when no interior boundary remains.
    pub fn rectangular(&self) -> Option<Bounds> {
        (self.occupied == 1 && self.axes.iter().all(|planes| axis_count(planes) == 2))
            .then(|| std::array::from_fn(|side| self.axes.map(|planes| planes[side])))
    }

    /// 📍️ Measures the signed Euclidean distance to exposed faces of the closed material.
    pub fn signed_boundary_distance(&self, point: [f64; 3]) -> f64 {
        let mut remaining = self.occupied;
        let mut inside = false;
        let mut distance = f64::INFINITY;
        while remaining != 0 {
            let cell = decode(remaining.trailing_zeros() as usize, 3);
            remaining &= remaining - 1;
            let bounds: Bounds = std::array::from_fn(|side| std::array::from_fn(|axis| self.axes[axis][cell[axis] + side]));
            let gap: [f64; 3] = std::array::from_fn(|i| (bounds[0][i] - point[i]).max(point[i] - bounds[1][i]).max(0.));
            inside |= gap == [0.; 3];
            for (axis, outward) in (0..3).flat_map(|axis| [false, true].map(|side| (axis, side))) {
                if adjacent_cell(cell, axis, outward).is_some_and(|neighbor| self.occupied & (1 << neighbor) != 0) { continue; }
                let mut components = gap;
                components[axis] = (point[axis] - bounds[usize::from(outward)][axis]).abs();
                let [x, y, z] = components;
                distance = distance.min(x.hypot(y).hypot(z));
            }
        }
        if inside { -distance } else { distance }
    }

    fn contains(&self, low: [f64; 3], high: [f64; 3]) -> bool {
        let mut cells = [0; 3];
        for axis in 0..3 {
            let planes = &self.axes[axis];
            let count = axis_count(planes);
            let Some(cell) = planes[..count].partition_point(|&value| value <= low[axis]).checked_sub(1) else { return false; };
            if cell + 1 == count || high[axis] > planes[cell + 1] { return false; }
            cells[axis] = cell;
        }
        self.occupied & (1 << address(cells, 3)) != 0
    }

    /// 🔀️ Classifies the common grid, removes irrelevant planes and certifies one closed result.
    pub fn combine(self, other: Self, operation: BooleanOp) -> Result<OrthogonalBoundary, ValueError> {
        let mut counts = [0; 3];
        let planes: [[f64; PLANES]; 3] = std::array::from_fn(|axis| {
            let mut planes = [f64::NAN; PLANES];
            for value in self.axes[axis].into_iter().chain(other.axes[axis]).filter(|value| value.is_finite()) {
                if planes[..counts[axis]].contains(&value) { continue; }
                planes[counts[axis]] = value;
                counts[axis] += 1;
            }
            planes[..counts[axis]].sort_unstable_by(f64::total_cmp);
            planes
        });
        let mut bounds = [[INTERVALS; 3], [0; 3]];
        let occupied: [bool; CELLS] = std::array::from_fn(|slot| {
            let cell = decode(slot, INTERVALS);
            if (0..3).any(|axis| cell[axis] + 1 >= counts[axis]) { return false; }
            let limits = |end| std::array::from_fn(|axis| planes[axis][cell[axis] + end]);
            let (low, high) = (limits(0), limits(1));
            let (a, b) = (self.contains(low, high), other.contains(low, high));
            let keep = match operation { BooleanOp::Unite => a || b, BooleanOp::Cut => a && !b, BooleanOp::Intersect => a && b };
            if keep {
                for axis in 0..3 {
                    bounds[0][axis] = bounds[0][axis].min(cell[axis]);
                    bounds[1][axis] = bounds[1][axis].max(cell[axis] + 1);
                }
            }
            keep
        });
        if bounds[1][0] == 0 { return Err(empty_result()); }
        let mut slots = [[0; 4]; 3];
        let mut axes = [[f64::NAN; 4]; 3];
        for axis in 0..3 {
            let first = bounds[0][axis];
            let last = bounds[1][axis];
            let breaks = |plane| {
                occupied.iter().enumerate().any(|(slot, &active)| {
                    let mut cell = decode(slot, INTERVALS);
                    if cell[axis] != plane { return false; }
                    cell[axis] -= 1;
                    active != occupied[address(cell, INTERVALS)]
                })
            };
            for (next, plane) in std::iter::once(first).chain((first + 1..last).filter(|&plane| breaks(plane))).chain(std::iter::once(last)).enumerate() {
                let target = axes[axis].get_mut(next).ok_or_else(|| ValueError::literal(
                    semio_framework_value::ValueRefusalKind::OwnershipLimit, "original orthogonal Boolean exceeds four material planes per axis",
                ))?;
                *target = planes[axis][plane];
                slots[axis][next] = plane;
            }
        }
        OrthogonalBoundary::from_cells(axes, |_, cell| occupied[address(std::array::from_fn(|axis| slots[axis][cell[axis]]), INTERVALS)])
    }
}
