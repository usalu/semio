//! 🌈️ Exact Coons control nets from independently interpolated clamped boundaries.

use super::*;

const BOUNDARIES: usize = 4;

#[derive(Clone, Copy)]
enum Turn {
    Sample { boundary: usize, index: usize },
    Fit(usize),
    Validate,
    Blend(usize),
    Ready,
}

#[derive(Clone, Copy)]
pub struct CoonsSurfaceBuild {
    counts: [usize; BOUNDARIES],
    extent: [usize; 2],
    curves: [Line; BOUNDARIES],
    controls: [Line; MAXIMUM_EXTENT],
    turn: Turn,
    label: PersistentLabel,
}

semio_framework_value::artifact_retire_leaf!(CoonsSurfaceBuild);

fn invalid_boundary() -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, "Coons patch requires four finite boundary polylines with matching oriented corners")
}

fn mix(a: Pnt3, b: Pnt3, parameter: f64) -> Pnt3 {
    a + (b - a) * parameter
}

impl CoonsSurfaceBuild {
    pub fn new(counts: [usize; BOUNDARIES], label: PersistentLabel) -> Result<Self, ValueError> {
        if counts.iter().any(|&count| count < 2) { return Err(invalid_boundary()); }
        if counts.iter().any(|&count| count > 4) {
            return Err(ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original Coons patch currently accepts two through four samples on each boundary"));
        }
        let extent = [counts[0].max(counts[2]), counts[1].max(counts[3])];
        Ok(Self {
            counts, extent, curves: [[ZERO; MAXIMUM_EXTENT]; BOUNDARIES], controls: [[ZERO; MAXIMUM_EXTENT]; MAXIMUM_EXTENT],
            turn: Turn::Sample { boundary: 0, index: 0 }, label,
        })
    }

    pub fn next_sample(&self) -> Option<(usize, usize)> {
        if let Turn::Sample { boundary, index } = self.turn { Some((boundary, index)) } else { None }
    }

    pub fn demand(&self) -> RetirementDemand {
        let copy_bytes = match self.turn {
            Turn::Sample { .. } => size_of::<Pnt3>(),
            Turn::Fit(boundary) => self.counts[boundary] * size_of::<Pnt3>(),
            Turn::Validate => 4 * size_of::<Pnt3>(),
            Turn::Blend(_) => 5 * size_of::<Pnt3>(),
            Turn::Ready => 0,
        };
        RetirementDemand { copy_bytes, depth: 1, ..Default::default() }
    }

    pub fn ready(&self) -> bool { matches!(self.turn, Turn::Ready) }

    pub fn into_grid(self) -> Result<GridSurfaceBuild, ValueError> {
        GridSurfaceBuild::from_clamped_controls(self.extent, self.controls, self.label)
    }

    pub fn step(&mut self, point: Option<EVec3>, effect: &mut RetainedAllocationEffect<'_>) -> Result<Progress, ValueError> {
        let grant = effect.grant();
        let demand = self.demand();
        if grant.maximum_items == 0 || !retained_clone_grant_admits(grant, demand) || !effect.performed(1)? {
            return Ok(Progress::default());
        }
        self.turn = match self.turn {
            Turn::Sample { boundary, index } => {
                let point = point.filter(|point| point.iter().all(|value| value.is_finite())).ok_or_else(invalid_boundary)?;
                self.curves[boundary][index] = Pnt3::from_array(point);
                if index + 1 < self.counts[boundary] { Turn::Sample { boundary, index: index + 1 } }
                else if boundary + 1 < BOUNDARIES { Turn::Sample { boundary: boundary + 1, index: 0 } }
                else { Turn::Fit(0) }
            }
            Turn::Fit(boundary) => {
                let size = self.counts[boundary];
                let target = self.extent[boundary % 2];
                let mut controls = fit_curve(self.curves[boundary], size)?;
                for old_size in size..target { controls = elevate(controls, old_size); }
                self.curves[boundary] = controls;
                if boundary + 1 < BOUNDARIES { Turn::Fit(boundary + 1) } else { Turn::Validate }
            }
            Turn::Validate => {
                self.validate()?;
                Turn::Blend(0)
            }
            Turn::Blend(index) => {
                let [nu, nv] = self.extent;
                let (i, j) = (index / nv, index % nv);
                self.controls[i][j] = self.control(i, j)?;
                if index + 1 < nu * nv { Turn::Blend(index + 1) } else { Turn::Ready }
            }
            Turn::Ready => return Err(ValueError::invariant("Coons patch fitting has already completed")),
        };
        Ok(Progress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() })
    }

    fn validate(&self) -> Result<(), ValueError> {
        let [nu, nv] = self.extent;
        let [bottom, right, top, left] = &self.curves;
        for (a, b) in [(bottom[0], left[0]), (bottom[nu - 1], right[0]), (top[0], left[nv - 1]), (top[nu - 1], right[nv - 1])] {
            let separation = a.distance(b);
            if !separation.is_finite() || separation > 1e-6 { return Err(invalid_boundary()); }
        }
        Ok(())
    }

    fn control(&self, i: usize, j: usize) -> Result<Pnt3, ValueError> {
        let [nu, nv] = self.extent;
        let [bottom, right, top, left] = &self.curves;
        let u = i as f64 / (nu - 1) as f64;
        let v = j as f64 / (nv - 1) as f64;
        let across = mix(bottom[i], top[i], v);
        let along = mix(left[j], right[j], u);
        let bilinear = mix(mix(bottom[0], bottom[nu - 1], u), mix(top[0], top[nu - 1], u), v);
        let output = across + (along - bilinear);
        if output.to_array().iter().any(|value| !value.is_finite()) { return Err(invalid_boundary()); }
        Ok(output)
    }
}

fn elevate(points: Line, old_size: usize) -> Line {
    std::array::from_fn(|index| {
        if index == 0 { points[0] }
        else if index == old_size { points[old_size - 1] }
        else if index < old_size { mix(points[index], points[index - 1], index as f64 / old_size as f64) }
        else { ZERO }
    })
}

fn fit_curve(points: Line, n: usize) -> Result<Line, ValueError> {
    let mut increments = [0.0; BOUNDARIES];
    for i in 1..n { increments[i] = points[i].distance(points[i - 1]).sqrt(); }
    let total: f64 = increments[..n].iter().sum();
    if !total.is_finite() || total == 0. { return Err(invalid_boundary()); }
    let mut matrix = [[0.; MAXIMUM_EXTENT]; MAXIMUM_EXTENT];
    let mut distance = 0.;
    for (row, step) in increments.into_iter().enumerate().take(n) {
        distance += step;
        let t = distance / total;
        let s = 1. - t;
        let values = match n {
            2 => [s, t, 0., 0.],
            3 => [s*s, 2.*s*t, t*t, 0.],
            _ => [s*s*s, 3.*s*s*t, 3.*s*t*t, t*t*t],
        };
        matrix[row][..n].copy_from_slice(&values[..n]);
    }
    let mut permutation = std::array::from_fn(|index| index);
    for column in 0..n {
        pivot_table(&mut matrix, &mut permutation, n, column)?;
        for row in column + 1..n { eliminate_table(&mut matrix, n, column, row); }
    }
    let mut right = [[0.; 3]; MAXIMUM_EXTENT];
    for row in 0..n {
        right[row] = forward_row(&matrix[row], &right, points[permutation[row]], row)?;
    }
    for row in (0..n).rev() {
        right[row] = backward_row(&matrix[row], &right, n, row)?;
    }
    Ok(right.map(Pnt3::from_array))
}
