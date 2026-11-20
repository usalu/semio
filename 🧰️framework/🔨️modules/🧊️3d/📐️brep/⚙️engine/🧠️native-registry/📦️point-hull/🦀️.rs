//! 🎟️ Projects pure point hull drafts into the native publication error contract.

use semio_framework_3d_brep_construction::point_hull::{HullDraft as GeometryHullDraft, HullRefusal};
use semio_framework_value::{ValueError, ValueRefusalKind};
use super::EVec3;

pub(super) use semio_framework_3d_brep_construction::point_hull::HullBoundary;

fn value_refusal(refusal: HullRefusal) -> ValueError {
    match refusal {
        HullRefusal::InvalidGeometry(message) => ValueError::literal(ValueRefusalKind::InvalidValue, message),
        HullRefusal::UnexpectedStep(message) => ValueError::invariant(message),
    }
}

/// 📦️ Keeps the bounded convex-hull draft with its native publication lifecycle.
pub struct HullDraft(GeometryHullDraft);

impl HullDraft {
    pub fn new(count: usize) -> Result<Self, ValueError> {
        GeometryHullDraft::new(count).map(Self).map_err(value_refusal)
    }

    pub fn next_point(&self) -> Option<usize> { self.0.next_point() }
    pub fn has_candidate(&self) -> bool { self.0.has_candidate() }

    pub fn add_point(&mut self, point: EVec3) -> Result<(), ValueError> {
        self.0.add_point(point).map_err(value_refusal)
    }

    pub fn scan_step(&mut self) -> Result<(), ValueError> {
        self.0.scan_step().map_err(value_refusal)
    }

    pub(super) fn finish(&mut self) -> Result<HullBoundary, ValueError> {
        self.0.finish().map_err(value_refusal)
    }
}
