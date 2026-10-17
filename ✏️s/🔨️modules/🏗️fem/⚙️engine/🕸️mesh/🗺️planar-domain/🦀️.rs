//! 🗺️ Planar domain, mounted domain owners, mesh options and errors, and point-in-polygon.

use super::*;

pub(super) fn close_vec_owner_step<T>(owner: &mut Vec<T>, maximum_bytes: usize) -> Result<Option<(usize, usize)>, ()> {
    if owner.pop().is_some() {
        return Ok(Some((1, 0)));
    }
    let bytes = owner.capacity().checked_mul(size_of::<T>()).ok_or(())?;
    if bytes == 0 {
        return Ok(None);
    }
    if bytes > maximum_bytes {
        return Err(());
    }
    *owner = Vec::new();
    Ok(Some((1, bytes)))
}

// #region 🔖️PlanarDomain
/// 📐️ A planar region to mesh: an outer boundary loop and zero or more hole loops, each a closed
/// polygon (points NOT repeating the first point at the end), in consistent (either) winding order.
#[derive(Clone, Debug, PartialEq)]
pub struct PlanarDomain {
    pub outer: Vec<[f64; 2]>,
    pub holes: Vec<Vec<[f64; 2]>>,
}

pub const MOUNTED_DOMAIN_POINT_SLOTS: usize = 128;
pub const MOUNTED_DOMAIN_HOLE_SLOTS: usize = 32;

/// 🚧️ A mounted polygon admission cannot fit or address the requested slot.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MountedDomainFault {
    PointCapacity { requested: usize, maximum: usize },
    HoleCapacity { requested: usize, maximum: usize },
    HoleNotAdmitted,
    InvalidHole { index: usize },
}

/// 🧩 Fixed mounted polygon whose admission and copy advance one slot at a time.
pub struct MountedPlanarPolygon {
    pub(super) points: [[f64; 2]; MOUNTED_DOMAIN_POINT_SLOTS],
    pub(super) admitted: usize,
    pub(super) len: usize,
}

impl MountedPlanarPolygon {
    pub(super) const fn new() -> Self {
        Self { points: [[0.0; 2]; MOUNTED_DOMAIN_POINT_SLOTS], admitted: 0, len: 0 }
    }

    pub fn admit_one(&mut self, target: usize) -> Result<bool, MountedDomainFault> {
        if target > MOUNTED_DOMAIN_POINT_SLOTS {
            return Err(MountedDomainFault::PointCapacity { requested: target, maximum: MOUNTED_DOMAIN_POINT_SLOTS });
        }
        if self.admitted < target {
            self.admitted += 1;
            return Ok(false);
        }
        Ok(true)
    }

    pub fn push(&mut self, point: [f64; 2]) -> Result<(), [f64; 2]> {
        if self.len == self.admitted {
            return Err(point);
        }
        self.points[self.len] = point;
        self.len += 1;
        Ok(())
    }

    pub(super) fn as_slice(&self) -> &[[f64; 2]] {
        &self.points[..self.len]
    }

    pub fn close_step(&mut self) -> bool {
        if self.len != 0 {
            self.len -= 1;
            return false;
        }
        if self.admitted != 0 {
            self.admitted -= 1;
            return false;
        }
        true
    }
}

/// 🗺️ Fixed mounted planar-domain owner with one admitted polygon or point per opportunity.
pub struct MountedPlanarDomain {
    pub(super) outer: MountedPlanarPolygon,
    pub(super) holes: [MountedPlanarPolygon; MOUNTED_DOMAIN_HOLE_SLOTS],
    pub(super) admitted_holes: usize,
    pub(super) hole_count: usize,
    pub(super) close_hole: usize,
}

impl MountedPlanarDomain {
    pub fn new() -> Self {
        Self { outer: MountedPlanarPolygon::new(), holes: [const { MountedPlanarPolygon::new() }; MOUNTED_DOMAIN_HOLE_SLOTS], admitted_holes: 0, hole_count: 0, close_hole: 0 }
    }

    pub fn admit_outer_one(&mut self, target: usize) -> Result<bool, MountedDomainFault> {
        self.outer.admit_one(target)
    }

    pub fn push_outer(&mut self, point: [f64; 2]) -> Result<(), [f64; 2]> {
        self.outer.push(point)
    }

    pub fn admit_hole_one(&mut self, target: usize) -> Result<bool, MountedDomainFault> {
        if target > MOUNTED_DOMAIN_HOLE_SLOTS {
            return Err(MountedDomainFault::HoleCapacity { requested: target, maximum: MOUNTED_DOMAIN_HOLE_SLOTS });
        }
        if self.admitted_holes < target {
            self.admitted_holes += 1;
            return Ok(false);
        }
        Ok(true)
    }

    pub fn begin_hole(&mut self) -> Result<usize, MountedDomainFault> {
        if self.hole_count == self.admitted_holes {
            return Err(MountedDomainFault::HoleNotAdmitted);
        }
        let index = self.hole_count;
        self.hole_count += 1;
        Ok(index)
    }

    pub fn admit_hole_point_one(&mut self, hole: usize, target: usize) -> Result<bool, MountedDomainFault> {
        self.holes.get_mut(hole).ok_or(MountedDomainFault::InvalidHole { index: hole })?.admit_one(target)
    }

    pub fn push_hole_point(&mut self, hole: usize, point: [f64; 2]) -> Result<(), [f64; 2]> {
        self.holes.get_mut(hole).ok_or(point)?.push(point)
    }

    pub(super) fn outer(&self) -> &[[f64; 2]] {
        self.outer.as_slice()
    }

    pub(super) fn holes_len(&self) -> usize {
        self.hole_count
    }

    pub(super) fn hole(&self, index: usize) -> Option<&[[f64; 2]]> {
        (index < self.hole_count).then(|| self.holes[index].as_slice())
    }

    pub fn close_step(&mut self) -> bool {
        if self.close_hole < self.hole_count {
            if !self.holes[self.close_hole].close_step() {
                return false;
            }
            self.close_hole += 1;
            return false;
        }
        if self.hole_count != 0 {
            self.hole_count -= 1;
            return false;
        }
        if self.admitted_holes != 0 {
            self.admitted_holes -= 1;
            return false;
        }
        self.outer.close_step()
    }
}

impl Default for MountedPlanarDomain {
    fn default() -> Self {
        Self::new()
    }
}

#[expect(clippy::large_enum_variant, reason = "Mounted domains retain their admitted fixed polygon slots inline so adopting and retiring the domain adds no allocation.")]
pub(super) enum MeshDomainOwner {
    Dynamic(PlanarDomain),
    Mounted(MountedPlanarDomain),
}

impl MeshDomainOwner {
    pub(super) fn outer(&self) -> &[[f64; 2]] {
        match self {
            Self::Dynamic(domain) => &domain.outer,
            Self::Mounted(domain) => domain.outer(),
        }
    }

    pub(super) fn holes_len(&self) -> usize {
        match self {
            Self::Dynamic(domain) => domain.holes.len(),
            Self::Mounted(domain) => domain.holes_len(),
        }
    }

    pub(super) fn hole(&self, index: usize) -> Option<&[[f64; 2]]> {
        match self {
            Self::Dynamic(domain) => domain.holes.get(index).map(Vec::as_slice),
            Self::Mounted(domain) => domain.hole(index),
        }
    }
}

/// 🕸️ A triangulated mesh: shared node positions plus triangles as index triples into `points`.
#[derive(Clone, Debug, PartialEq)]
pub struct TriMesh2 {
    pub points: Vec<[f64; 2]>,
    pub tris: Vec<[u32; 3]>,
}

/// ⚙️ Refinement targets for [`triangulate`] — either left at `0.0` to disable that constraint.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MeshOpts {
    pub max_edge: f64,
    pub min_angle_deg: f64,
}

/// ⚠️ Everything that can go wrong building a mesh.
#[derive(Debug)]
pub enum MeshError {
    DegenerateDomain,
    TriangulationFailed(String),
}

impl std::fmt::Display for MeshError {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DegenerateDomain => formatter.write_str("domain has a degenerate outer boundary (fewer than 3 points)"),
            Self::TriangulationFailed(detail) => write!(formatter, "triangulation failed: {detail}"),
        }
    }
}

impl std::error::Error for MeshError {}

/// 🎯️ Ray-casting point-in-polygon test (standard even-odd rule; polygon need not be convex).
pub(super) fn point_in_polygon(point: [f64; 2], polygon: &[[f64; 2]]) -> bool {
    if polygon.len() < 3 {
        return false;
    }
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let (xi, yi) = (polygon[i][0], polygon[i][1]);
        let (xj, yj) = (polygon[j][0], polygon[j][1]);
        let crosses = (yi > point[1]) != (yj > point[1]);
        if crosses && point[0] < (xj - xi) * (point[1] - yi) / (yj - yi) + xi {
            inside = !inside;
        }
        j = i;
    }
    inside
}
// #endregion 🔖️PlanarDomain
