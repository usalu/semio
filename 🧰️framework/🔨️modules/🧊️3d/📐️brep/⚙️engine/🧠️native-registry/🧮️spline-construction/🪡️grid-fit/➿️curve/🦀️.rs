//! ➿️ Original centripetal interpolation and least-squares approximation of ordered data.

use super::*;
use semio_framework_3d_brep_curve::curve_ops::fitting_knot;

#[derive(Clone, Copy)]
enum CurveTurn {
    Read(usize),
    Chord(usize),
    Normalize,
    ReserveKnots,
    Knot(usize),
    Matrix(usize),
    ApproximationResidual(usize),
    ApproximationRow { sample: usize, row: usize },
    Pivot(usize),
    Eliminate { column: usize, row: usize },
    Forward(usize),
    Backward(usize),
    ReserveControls,
    ReserveWeights,
    Populate(usize),
    Ready,
}

#[derive(Clone, Copy)]
struct CurveFit {
    count: usize,
    approximation_controls: Option<usize>,
    samples: Line,
    parameters: [f64; MAXIMUM_EXTENT],
    total: f64,
    matrix: Matrix,
    permutation: [usize; MAXIMUM_EXTENT],
    right: [[f64; 3]; MAXIMUM_EXTENT],
    normal_rhs: [Pnt3; MAXIMUM_EXTENT],
    basis: [f64; MAXIMUM_EXTENT],
    residual: [f64; 3],
    turn: CurveTurn,
}

semio_framework_value::artifact_retire_leaf!(CurveFit);

/// 📌️ Holds an original curve fit, its exact knots, and its unit-weight control owner.
#[derive(semio_framework_value::RetireOwned)]
pub struct CurveSplineBuild {
    label: PersistentLabel,
    fit: CurveFit,
    knots: KnotVector,
    controls: Vec<Pnt3>,
    weights: Vec<f64>,
}

impl CurveSplineBuild {
    /// 🏷️ Returns the owning publication's reserved label.
    pub fn label(&self) -> PersistentLabel { self.label }

    pub fn interpolation(count: usize, degree: usize, label: PersistentLabel) -> Result<Self, ValueError> {
        Self::new(count, degree, None, label)
    }

    pub fn approximation(count: usize, degree: usize, controls: usize, label: PersistentLabel) -> Result<Self, ValueError> {
        Self::new(count, degree, Some(controls), label)
    }

    fn new(count: usize, degree: usize, approximation_controls: Option<usize>, label: PersistentLabel) -> Result<Self, ValueError> {
        if count < 2 {
            return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "curve fitting needs at least two points"));
        }
        if count > MAXIMUM_EXTENT {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original curve fitting currently accepts up to eight samples"));
        }
        let approximation_controls = approximation_controls.map(|controls| controls.clamp(2, count));
        let controls = approximation_controls.unwrap_or(count);
        let fit = CurveFit {
            count, approximation_controls, samples: [ZERO; MAXIMUM_EXTENT], parameters: [0.; MAXIMUM_EXTENT], total: 0.,
            matrix: [[0.; MAXIMUM_EXTENT]; MAXIMUM_EXTENT], permutation: std::array::from_fn(|index| index),
            right: [[0.; 3]; MAXIMUM_EXTENT], normal_rhs: [ZERO; MAXIMUM_EXTENT],
            basis: [0.; MAXIMUM_EXTENT], residual: [0.; 3], turn: CurveTurn::Read(0),
        };
        Ok(Self {
            label, fit, knots: KnotVector { knots: Vec::new(), degree: degree.clamp(1, controls - 1) },
            controls: Vec::new(), weights: Vec::new(),
        })
    }

    pub fn next_sample(&self) -> Option<usize> {
        if let CurveTurn::Read(index) = self.fit.turn { Some(index) } else { None }
    }

    pub fn ready(&self) -> bool { matches!(self.fit.turn, CurveTurn::Ready) }

    pub fn curve(self) -> Curve3 {
        Curve3::Nurbs { knots: self.knots, controls: self.controls, weights: self.weights }
    }

    fn control_count(&self) -> usize { self.fit.approximation_controls.unwrap_or(self.fit.count) }

    fn equation_count(&self) -> usize { self.fit.approximation_controls.map_or(self.fit.count, |controls| controls - 2) }

    fn knot_count(&self) -> usize { self.control_count() + self.knots.degree + 1 }

    fn knot(&self, index: usize) -> f64 {
        let parameters = &self.fit.parameters[..self.fit.count];
        self.fit.approximation_controls.map_or_else(
            || averaged_knot(parameters, self.knots.degree, index),
            |controls| fitting_knot(parameters, self.knots.degree, controls, index),
        )
    }

    fn start_equations(&self) -> CurveTurn {
        match self.fit.approximation_controls {
            Some(2) => CurveTurn::ReserveControls,
            Some(_) => CurveTurn::Matrix(1),
            None => CurveTurn::Matrix(0),
        }
    }

    fn prepare_approximation_residual(&mut self, sample: usize, controls: usize) {
        let start = self.fit.samples[0].to_array();
        let end = self.fit.samples[self.fit.count - 1].to_array();
        let point = self.fit.samples[sample].to_array();
        self.fit.residual = std::array::from_fn(|axis| {
            point[axis] - start[axis] * self.fit.basis[0] - end[axis] * self.fit.basis[controls - 1]
        });
    }

    fn accumulate_approximation_row(&mut self, row: usize, equations: usize) {
        let weight = self.fit.basis[row + 1];
        for (column, value) in self.fit.matrix[row].iter_mut().enumerate().take(equations) {
            *value += weight * self.fit.basis[column + 1];
        }
        let rhs = self.fit.normal_rhs[row].to_array();
        self.fit.normal_rhs[row] = Pnt3::from_array(std::array::from_fn(|axis| rhs[axis] + weight * self.fit.residual[axis]));
    }

    fn control(&self, index: usize) -> Pnt3 {
        let Some(controls) = self.fit.approximation_controls else { return Pnt3::from_array(self.fit.right[index]); };
        if index == 0 { self.fit.samples[0] }
        else if index + 1 == controls { self.fit.samples[self.fit.count - 1] }
        else { Pnt3::from_array(self.fit.right[index - 1]) }
    }

    pub fn demand(&self) -> RetirementDemand {
        let n = self.fit.count;
        let controls = self.control_count();
        let equations = self.equation_count();
        let (copy_bytes, capacity_bytes) = match self.fit.turn {
            CurveTurn::Read(_) | CurveTurn::Forward(_) | CurveTurn::Backward(_) => (size_of::<Pnt3>(), 0),
            CurveTurn::Chord(_) => (2 * size_of::<Pnt3>(), 0),
            CurveTurn::Normalize => (n * size_of::<f64>(), 0),
            CurveTurn::Matrix(_) => (controls * size_of::<f64>(), 0),
            CurveTurn::ApproximationResidual(_) => (4 * size_of::<Pnt3>() + 2 * size_of::<f64>(), 0),
            CurveTurn::ApproximationRow { .. } => (2 * size_of::<Pnt3>() + (equations + 1) * size_of::<f64>(), 0),
            CurveTurn::Eliminate { .. } => (equations * size_of::<f64>(), 0),
            CurveTurn::ReserveKnots => (0, self.knot_count() * size_of::<f64>()),
            CurveTurn::Knot(_) => (size_of::<f64>(), 0),
            CurveTurn::Pivot(_) => (2 * equations * size_of::<f64>(), 0),
            CurveTurn::ReserveControls => (0, controls * size_of::<Pnt3>()),
            CurveTurn::ReserveWeights => (0, controls * size_of::<f64>()),
            CurveTurn::Populate(_) => (size_of::<Pnt3>() + size_of::<f64>(), 0),
            CurveTurn::Ready => (0, 0),
        };
        RetirementDemand { copy_bytes, capacity_bytes, depth: 1, ..Default::default() }
    }

    pub fn step(&mut self, sample: Option<EVec3>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let demand = self.demand();
        let grant = effect.grant();
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? {
            return Ok(Progress::default());
        }
        let mut progress = Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        let n = self.fit.count;
        let controls = self.control_count();
        let equations = self.equation_count();
        self.fit.turn = match self.fit.turn {
            CurveTurn::Read(index) => {
                let value = sample.filter(|p| p.iter().all(|value| value.is_finite())).ok_or_else(invalid)?;
                self.fit.samples[index] = Pnt3::from_array(value);
                if index + 1 < n { CurveTurn::Read(index + 1) } else { CurveTurn::Chord(1) }
            }
            CurveTurn::Chord(index) => {
                let step = self.fit.samples[index].distance(self.fit.samples[index - 1]).sqrt();
                self.fit.total += step;
                if !self.fit.total.is_finite() { return Err(invalid()); }
                self.fit.parameters[index] = self.fit.total;
                if index + 1 < n { CurveTurn::Chord(index + 1) } else { CurveTurn::Normalize }
            }
            CurveTurn::Normalize => {
                for (index, value) in self.fit.parameters.iter_mut().enumerate().take(n) {
                    *value = if self.fit.total <= 0. { index as f64 / (n - 1) as f64 } else { *value / self.fit.total };
                }
                CurveTurn::ReserveKnots
            }
            CurveTurn::ReserveKnots => {
                let count = self.knot_count();
                progress.retained_capacity_bytes = reserve(&mut self.knots.knots, count)?;
                CurveTurn::Knot(0)
            }
            CurveTurn::Knot(index) => {
                self.knots.knots.push(self.knot(index));
                if index + 1 < self.knot_count() { CurveTurn::Knot(index + 1) } else { self.start_equations() }
            }
            CurveTurn::Matrix(index) => {
                if self.fit.approximation_controls.is_some() {
                    self.fit.basis = collocation_row(&self.knots, controls, self.fit.parameters[index])?;
                    CurveTurn::ApproximationResidual(index)
                } else {
                    self.fit.matrix[index] = collocation_row(&self.knots, n, self.fit.parameters[index])?;
                    if index + 1 < n { CurveTurn::Matrix(index + 1) } else { CurveTurn::Pivot(0) }
                }
            }
            CurveTurn::ApproximationResidual(sample) => {
                self.prepare_approximation_residual(sample, controls);
                CurveTurn::ApproximationRow { sample, row: 0 }
            }
            CurveTurn::ApproximationRow { sample, row } => {
                self.accumulate_approximation_row(row, equations);
                if row + 1 < equations { CurveTurn::ApproximationRow { sample, row: row + 1 } }
                else if sample + 1 < n - 1 { CurveTurn::Matrix(sample + 1) }
                else { CurveTurn::Pivot(0) }
            }
            CurveTurn::Pivot(column) => {
                pivot_table(&mut self.fit.matrix, &mut self.fit.permutation, equations, column)?;
                if column + 1 < equations { CurveTurn::Eliminate { column, row: column + 1 } } else { CurveTurn::Forward(0) }
            }
            CurveTurn::Eliminate { column, row } => {
                eliminate_table(&mut self.fit.matrix, equations, column, row);
                if row + 1 < equations { CurveTurn::Eliminate { column, row: row + 1 } } else { CurveTurn::Pivot(column + 1) }
            }
            CurveTurn::Forward(row) => {
                let index = self.fit.permutation[row];
                let point = if self.fit.approximation_controls.is_some() { self.fit.normal_rhs[index] } else { self.fit.samples[index] };
                self.fit.right[row] = forward_row(&self.fit.matrix[row], &self.fit.right, point, row)?;
                if row + 1 < equations { CurveTurn::Forward(row + 1) } else { CurveTurn::Backward(row) }
            }
            CurveTurn::Backward(row) => {
                self.fit.right[row] = backward_row(&self.fit.matrix[row], &self.fit.right, equations, row)?;
                if row > 0 { CurveTurn::Backward(row - 1) } else { CurveTurn::ReserveControls }
            }
            CurveTurn::ReserveControls => {
                progress.retained_capacity_bytes = reserve(&mut self.controls, controls)?;
                CurveTurn::ReserveWeights
            }
            CurveTurn::ReserveWeights => {
                progress.retained_capacity_bytes = reserve(&mut self.weights, controls)?;
                CurveTurn::Populate(0)
            }
            CurveTurn::Populate(index) => {
                self.controls.push(self.control(index));
                self.weights.push(1.);
                if index + 1 < controls { CurveTurn::Populate(index + 1) } else { CurveTurn::Ready }
            }
            CurveTurn::Ready => return Err(ValueError::invariant("original curve fitting is already complete")),
        };
        if !progress.fits(grant) {
            return Err(ValueError::literal(ValueRefusalKind::OwnershipLimit, "curve fitting storage exceeded its grant").with_retained_progress(progress));
        }
        Ok(progress)
    }
}
