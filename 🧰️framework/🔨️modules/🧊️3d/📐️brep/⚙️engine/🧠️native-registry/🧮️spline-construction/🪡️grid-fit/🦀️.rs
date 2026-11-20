//! 🪡️ Original fitting and storage for a small tensor-product interpolating NURBS surface.

use super::*;
use semio_framework_3d_brep_curve::bspline::{KnotVector, homogeneous_jet_at_span_original};
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress as Progress, RetirementDemand, ValueError, ValueRefusalKind};
use semio_framework_value::retained_clone::retained_clone_grant_admits;
use std::mem::size_of;

#[path = "🌈️coons/🦀️.rs"]
mod coons;
pub use coons::CoonsSurfaceBuild;

#[path = "➿️curve/🦀️.rs"]
mod curve;
pub use curve::CurveSplineBuild;

const MAXIMUM_EXTENT: usize = 8;
const ZERO: Pnt3 = Pnt3 { x: 0., y: 0., z: 0. };
type Line = [Pnt3; MAXIMUM_EXTENT];
type Matrix = [[f64; MAXIMUM_EXTENT]; MAXIMUM_EXTENT];

#[derive(Clone, Copy)]
enum Stage {
    Reading(usize),
    Parameterizing { axis: usize, line: usize, index: usize },
    Normalize { axis: usize, line: usize },
    ReserveKnots(usize),
    Knots { axis: usize, index: usize },
    Matrix { axis: usize, index: usize },
    Pivot { axis: usize, column: usize },
    Eliminate { axis: usize, column: usize, row: usize },
    Forward { axis: usize, line: usize, row: usize },
    Backward { axis: usize, line: usize, row: usize },
    ReserveOuter(usize),
    ReserveRow { output: usize, row: usize },
    Populate(usize),
    Ready,
}

#[derive(Clone, Copy)]
struct Fit {
    extent: [usize; 2],
    degree: [usize; 2],
    samples: [Line; MAXIMUM_EXTENT],
    temporary: [Line; MAXIMUM_EXTENT],
    controls: [Line; MAXIMUM_EXTENT],
    parameters: [[f64; MAXIMUM_EXTENT]; 2],
    matrices: [Matrix; 2],
    permutation: [[usize; MAXIMUM_EXTENT]; 2],
    increments: [f64; MAXIMUM_EXTENT],
    total: f64,
    right: [[f64; 3]; MAXIMUM_EXTENT],
    stage: Stage,
    ready_controls: bool,
}

semio_framework_value::artifact_retire_leaf!(Fit);

/// 📐️ Binds the fitted coefficients and owned tensor-product buffers to a not-yet-published key.
#[derive(semio_framework_value::RetireOwned)]
pub struct GridSurfaceBuild {
    label: PersistentLabel,
    fit: Fit,
    u_knots: KnotVector,
    v_knots: KnotVector,
    controls: Vec<Vec<Pnt3>>,
    weights: Vec<Vec<f64>>,
}

fn invalid() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "NURBS interpolation grid requires finite samples and nonsingular spline bases")
}

fn reserve<T>(target: &mut Vec<T>, count: usize) -> Result<usize, ValueError> {
    target.try_reserve_exact(count).map_err(|_| ValueError::literal(ValueRefusalKind::AllocationFailed, "NURBS grid storage allocation failed"))?;
    Ok(target.capacity() * size_of::<T>())
}

impl GridSurfaceBuild {
    /// 🏷️ Returns the owning publication's reserved label.
    pub fn label(&self) -> PersistentLabel { self.label }

    pub fn new(rows: usize, columns: usize, degree_u: usize, degree_v: usize, label: PersistentLabel) -> Result<Self, ValueError> {
        let extent = [rows, columns];
        if extent.iter().any(|&n| n < 2) { return Err(invalid()); }
        if extent.iter().any(|&n| n > MAXIMUM_EXTENT) {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original NURBS grid currently accepts two to eight samples per direction"));
        }
        let degree = [degree_u.clamp(1, rows - 1), degree_v.clamp(1, columns - 1)];
        Ok(Self {
            label,
            fit: Fit {
                extent, degree, samples: [[ZERO; MAXIMUM_EXTENT]; MAXIMUM_EXTENT],
                temporary: [[ZERO; MAXIMUM_EXTENT]; MAXIMUM_EXTENT],
                controls: [[ZERO; MAXIMUM_EXTENT]; MAXIMUM_EXTENT],
                parameters: [[0.; MAXIMUM_EXTENT]; 2],
                matrices: [[[0.; MAXIMUM_EXTENT]; MAXIMUM_EXTENT]; 2],
                permutation: [std::array::from_fn(|index| index); 2],
                increments: [0.; MAXIMUM_EXTENT], total: 0., right: [[0.; 3]; MAXIMUM_EXTENT],
                stage: Stage::Reading(0),
                ready_controls: false,
            },
            u_knots: KnotVector { knots: Vec::new(), degree: degree[0] },
            v_knots: KnotVector { knots: Vec::new(), degree: degree[1] },
            controls: Vec::new(), weights: Vec::new(),
        })
    }

    fn from_clamped_controls(extent: [usize; 2], controls: [Line; MAXIMUM_EXTENT], label: PersistentLabel) -> Result<Self, ValueError> {
        let [rows, columns] = extent;
        let mut grid = Self::new(rows, columns, rows - 1, columns - 1, label)?;
        grid.fit.controls = controls;
        grid.fit.parameters[0][rows - 1] = 1.;
        grid.fit.parameters[1][columns - 1] = 1.;
        grid.fit.ready_controls = true;
        grid.fit.stage = Stage::ReserveKnots(0);
        Ok(grid)
    }

    pub fn next_point_index(&self) -> Option<usize> {
        if let Stage::Reading(index) = self.fit.stage { Some(index) } else { None }
    }

    pub fn ready(&self) -> bool { matches!(self.fit.stage, Stage::Ready) }

    pub fn surface(self) -> Surface {
        Surface::Nurbs { u_knots: self.u_knots, v_knots: self.v_knots, controls: self.controls, weights: self.weights }
    }

    fn knot_count(&self, axis: usize) -> usize { self.fit.extent[axis] + self.fit.degree[axis] + 1 }

    fn knot(&self, axis: usize, index: usize) -> f64 {
        let count = self.fit.extent[axis];
        averaged_knot(&self.fit.parameters[axis][..count], self.fit.degree[axis], index)
    }

    fn phase_after_knots(&self, axis: usize, index: usize) -> Stage {
        if index + 1 < self.knot_count(axis) { Stage::Knots { axis, index: index + 1 } }
        else if axis == 0 { Stage::ReserveKnots(1) }
        else if self.fit.ready_controls { Stage::ReserveOuter(0) }
        else { Stage::Matrix { axis: 0, index: 0 } }
    }

    pub fn demand(&self) -> RetirementDemand {
        let [rows, columns] = self.fit.extent;
        let (copy_bytes, capacity_bytes) = match self.fit.stage {
            Stage::Reading(_) => (size_of::<Pnt3>(), 0),
            Stage::Parameterizing { .. } => (2 * size_of::<Pnt3>(), 0),
            Stage::Normalize { axis, .. } => (self.fit.extent[axis] * size_of::<f64>(), 0),
            Stage::Pivot { axis, .. } => (2 * self.fit.extent[axis] * size_of::<f64>(), 0),
            Stage::Eliminate { axis, .. } => (self.fit.extent[axis] * size_of::<f64>(), 0),
            Stage::Forward { .. } | Stage::Backward { .. } => (size_of::<Pnt3>(), 0),
            Stage::ReserveKnots(axis) => (0, self.knot_count(axis) * size_of::<f64>()),
            Stage::Knots { .. } => (size_of::<f64>(), 0),
            Stage::Matrix { axis, .. } => (self.fit.extent[axis] * size_of::<f64>(), 0),
            Stage::ReserveOuter(0) => (0, rows * size_of::<Vec<Pnt3>>()),
            Stage::ReserveOuter(_) => (0, rows * size_of::<Vec<f64>>()),
            Stage::ReserveRow { output: 0, .. } => (0, columns * size_of::<Pnt3>()),
            Stage::ReserveRow { .. } => (0, columns * size_of::<f64>()),
            Stage::Populate(_) => (size_of::<Pnt3>() + size_of::<f64>(), 0),
            Stage::Ready => (0, 0),
        };
        RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() }
    }

    pub fn step(&mut self, sample: Option<EVec3>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let grant = effect.grant();
        let demand = self.demand();
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? { return Ok(Default::default()); }
        let mut progress = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        let [rows, columns] = self.fit.extent;
        self.fit.stage = match self.fit.stage {
            Stage::Reading(index) => {
                let sample = sample.filter(|point| point.iter().all(|value| value.is_finite())).ok_or_else(invalid)?;
                self.fit.samples[index / columns][index % columns] = Pnt3::from_array(sample);
                if index + 1 < rows * columns { Stage::Reading(index + 1) }
                else { Stage::Parameterizing { axis: 0, line: 0, index: 1 } }
            }
            Stage::Parameterizing { axis, line, index } => {
                self.fit.accumulate_step(axis, line, index)?;
                if index + 1 < self.fit.extent[axis] { Stage::Parameterizing { axis, line, index: index + 1 } }
                else { Stage::Normalize { axis, line } }
            }
            Stage::Normalize { axis, line } => {
                self.fit.normalize_line(axis, line);
                let lines = self.fit.extent[1 - axis];
                if line + 1 < lines { Stage::Parameterizing { axis, line: line + 1, index: 1 } }
                else if axis == 0 { Stage::Parameterizing { axis: 1, line: 0, index: 1 } }
                else { Stage::ReserveKnots(0) }
            }
            Stage::ReserveKnots(axis) => {
                let count = self.knot_count(axis);
                let knots = if axis == 0 { &mut self.u_knots } else { &mut self.v_knots };
                progress.retained_capacity_bytes = reserve(&mut knots.knots, count)?;
                Stage::Knots { axis, index: 0 }
            }
            Stage::Knots { axis, index } => {
                let value = self.knot(axis, index);
                let knots = if axis == 0 { &mut self.u_knots } else { &mut self.v_knots };
                knots.knots.push(value);
                self.phase_after_knots(axis, index)
            }
            Stage::Matrix { axis, index } => {
                let knots = if axis == 0 { &self.u_knots } else { &self.v_knots };
                let count = self.fit.extent[axis];
                let t = self.fit.parameters[axis][index];
                self.fit.matrices[axis][index] = collocation_row(knots, count, t)?;
                if index + 1 < count { Stage::Matrix { axis, index: index + 1 } }
                else if axis == 0 { Stage::Matrix { axis: 1, index: 0 } }
                else { Stage::Pivot { axis: 0, column: 0 } }
            }
            Stage::Pivot { axis, column } => {
                self.fit.pivot(axis, column)?;
                if column + 1 < self.fit.extent[axis] {
                    Stage::Eliminate { axis, column, row: column + 1 }
                } else if axis == 0 {
                    Stage::Pivot { axis: 1, column: 0 }
                } else {
                    Stage::Forward { axis: 1, line: 0, row: 0 }
                }
            }
            Stage::Eliminate { axis, column, row } => {
                self.fit.eliminate(axis, column, row);
                if row + 1 < self.fit.extent[axis] { Stage::Eliminate { axis, column, row: row + 1 } }
                else { Stage::Pivot { axis, column: column + 1 } }
            }
            Stage::Forward { axis, line, row } => {
                self.fit.forward(axis, line, row)?;
                if row + 1 < self.fit.extent[axis] { Stage::Forward { axis, line, row: row + 1 } }
                else { Stage::Backward { axis, line, row } }
            }
            Stage::Backward { axis, line, row } => {
                self.fit.backward(axis, line, row)?;
                if row > 0 { Stage::Backward { axis, line, row: row - 1 } }
                else if line + 1 < self.fit.extent[1 - axis] { Stage::Forward { axis, line: line + 1, row: 0 } }
                else if axis == 1 { Stage::Forward { axis: 0, line: 0, row: 0 } }
                else { Stage::ReserveOuter(0) }
            }
            Stage::ReserveOuter(output) => {
                progress.retained_capacity_bytes = if output == 0 { reserve(&mut self.controls, rows)? } else { reserve(&mut self.weights, rows)? };
                if output == 0 { Stage::ReserveOuter(1) } else { Stage::ReserveRow { output: 0, row: 0 } }
            }
            Stage::ReserveRow { output, row } => {
                if output == 0 {
                    let mut values = Vec::new();
                    progress.retained_capacity_bytes = reserve(&mut values, columns)?;
                    self.controls.push(values);
                    Stage::ReserveRow { output: 1, row }
                } else {
                    let mut values = Vec::new();
                    progress.retained_capacity_bytes = reserve(&mut values, columns)?;
                    self.weights.push(values);
                    if row + 1 < rows { Stage::ReserveRow { output: 0, row: row + 1 } } else { Stage::Populate(0) }
                }
            }
            Stage::Populate(index) => {
                let (row, column) = (index / columns, index % columns);
                self.controls[row].push(self.fit.controls[row][column]);
                self.weights[row].push(1.);
                if index + 1 < rows * columns { Stage::Populate(index + 1) } else { Stage::Ready }
            }
            Stage::Ready => return Err(ValueError::invariant("NURBS grid construction is already complete")),
        };
        if !progress.fits(grant) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "NURBS grid allocation exceeded its grant").with_retained_progress(progress));
        }
        Ok(progress)
    }
}

impl Fit {
    fn accumulate_step(&mut self, axis: usize, line: usize, index: usize) -> Result<(), ValueError> {
        let sample = |index: usize| if axis == 0 { self.samples[index][line] } else { self.samples[line][index] };
        let increment = sample(index).distance(sample(index - 1)).sqrt();
        self.total += increment;
        if !self.total.is_finite() { return Err(invalid()); }
        self.increments[index] = increment;
        Ok(())
    }

    fn normalize_line(&mut self, axis: usize, line: usize) {
        let size = self.extent[axis];
        let count = self.extent[1 - axis];
        let mut distance = 0.;
        for (index, value) in self.parameters[axis].iter_mut().enumerate().take(size) {
            distance += self.increments[index];
            *value += if self.total <= 0. { index as f64 / (size - 1) as f64 } else { distance / self.total };
            if line + 1 == count { *value /= count as f64; }
        }
        self.total = 0.;
    }

    fn pivot(&mut self, axis: usize, column: usize) -> Result<(), ValueError> {
        pivot_table(&mut self.matrices[axis], &mut self.permutation[axis], self.extent[axis], column)
    }

    fn eliminate(&mut self, axis: usize, column: usize, row: usize) {
        eliminate_table(&mut self.matrices[axis], self.extent[axis], column, row)
    }

    fn forward(&mut self, axis: usize, line: usize, row: usize) -> Result<(), ValueError> {
        let source = self.permutation[axis][row];
        let point = if axis == 1 { self.samples[line][source] } else { self.temporary[source][line] };
        self.right[row] = forward_row(&self.matrices[axis][row], &self.right, point, row)?;
        Ok(())
    }

    fn backward(&mut self, axis: usize, line: usize, row: usize) -> Result<(), ValueError> {
        let solution = backward_row(&self.matrices[axis][row], &self.right, self.extent[axis], row)?;
        self.right[row] = solution;
        if axis == 1 { self.temporary[line][row] = Pnt3::from_array(solution); }
        else { self.controls[row][line] = Pnt3::from_array(solution); }
        Ok(())
    }
}

fn averaged_knot(params: &[f64], degree: usize, index: usize) -> f64 {
    if index <= degree { 0. }
    else if index >= params.len() { params[params.len() - 1] }
    else { params[index - degree..index].iter().sum::<f64>() / degree as f64 }
}

fn collocation_row(knots: &KnotVector, count: usize, parameter: f64) -> Result<[f64; MAXIMUM_EXTENT], ValueError> {
    let mut row = [0.; MAXIMUM_EXTENT];
    let span = knots.find_span(parameter);
    for (index, value) in row.iter_mut().enumerate().take(count) {
        let jet = homogeneous_jet_at_span_original(knots, count, parameter, 0, span,
            |control| Some([f64::from(u8::from(control == index)), 0., 0., 1.])).ok_or_else(invalid)?;
        *value = jet[0][0];
    }
    Ok(row)
}

fn pivot_table(matrix: &mut Matrix, permutation: &mut [usize; MAXIMUM_EXTENT], n: usize, column: usize) -> Result<(), ValueError> {
    let pivot = (column..n).max_by(|&a, &b| matrix[a][column].abs().total_cmp(&matrix[b][column].abs())).ok_or_else(invalid)?;
    let divisor = matrix[pivot][column];
    if !divisor.is_finite() || divisor == 0. { return Err(invalid()); }
    matrix.swap(column, pivot);
    permutation.swap(column, pivot);
    Ok(())
}

fn eliminate_table(matrix: &mut Matrix, n: usize, column: usize, row: usize) {
    let (start, end) = matrix.split_at_mut(row);
    let pivot = &start[column];
    let target = &mut end[0];
    let factor = target[column] / pivot[column];
    target[column] = factor;
    for index in column + 1..n { target[index] -= factor * pivot[index]; }
}

fn forward_row(lower: &[f64; MAXIMUM_EXTENT], right: &[[f64; 3]; MAXIMUM_EXTENT], point: Pnt3, row: usize) -> Result<[f64; 3], ValueError> {
    let mut result = point.to_array();
    for (coordinate, value) in result.iter_mut().enumerate() {
        for prior in 0..row { *value -= lower[prior] * right[prior][coordinate]; }
    }
    if result.iter().any(|value| !value.is_finite()) { Err(invalid()) } else { Ok(result) }
}

fn backward_row(upper: &[f64; MAXIMUM_EXTENT], right: &[[f64; 3]; MAXIMUM_EXTENT], n: usize, row: usize) -> Result<[f64; 3], ValueError> {
    let mut solution = right[row];
    for (coordinate, value) in solution.iter_mut().enumerate() {
        let sum: f64 = (row + 1..n).map(|index| upper[index] * right[index][coordinate]).sum();
        *value = (*value - sum) / upper[row];
    }
    if solution.iter().any(|value| !value.is_finite()) { Err(invalid()) } else { Ok(solution) }
}
