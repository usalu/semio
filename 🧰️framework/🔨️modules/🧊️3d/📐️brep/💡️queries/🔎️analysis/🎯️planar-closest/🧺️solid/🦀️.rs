//! 🧺️ Finds the nearest point over the original faces of a solid or bounded compound.

use super::{Body, PlanarClosest, Pnt3, RetirementDemand, ValueError, invalid, size_of};
use crate::PlanarMeasureSource;
use semio_framework_3d_brep_representation::arena::{FaceId, ShellId, SolidId};

/// 🔬️ Keeps one face projection in progress and retains the best completed boundary point.
pub struct SolidClosest {
    source: PlanarMeasureSource,
    target: Pnt3,
    member: usize,
    shell: usize,
    face: usize,
    projecting: Option<PlanarClosest>,
    best: Option<(Pnt3, f64)>,
}

impl SolidClosest {
    pub fn new(source: PlanarMeasureSource, target: Pnt3) -> Self {
        Self { source, target, member: 0, shell: 0, face: 0, projecting: None, best: None }
    }

    pub fn source(&self) -> PlanarMeasureSource { self.source }

    fn solid(&self) -> Option<SolidId> {
        match self.source {
            PlanarMeasureSource::Solid(id) => (self.member == 0).then_some(id),
            PlanarMeasureSource::Compound(ids) => ids.get(self.member).copied().flatten(),
            PlanarMeasureSource::Face(_) => None,
        }
    }

    fn shell(&self, body: &Body, id: SolidId) -> Result<Option<ShellId>, ValueError> {
        let solid = body.solids.get(id).ok_or_else(invalid)?;
        Ok(if self.shell == 0 { Some(solid.outer) } else { solid.inners.get(self.shell - 1).copied() })
    }

    pub fn next_demand(&self, body: &Body) -> Result<RetirementDemand, ValueError> {
        if let Some(projecting) = &self.projecting { return projecting.next_demand(body); }
        let Some(solid) = self.solid() else {
            return Ok(RetirementDemand { depth: 1, copy_bytes: size_of::<Pnt3>(), ..Default::default() });
        };
        let mut depth = body.solids.next_remove_depth_demand(solid)?;
        if let Some(shell) = self.shell(body, solid)? { depth = depth.max(body.shells.next_remove_depth_demand(shell)?); }
        Ok(RetirementDemand { depth, copy_bytes: size_of::<FaceId>(), ..Default::default() })
    }

    pub fn step(&mut self, body: &Body) -> Result<Option<Pnt3>, ValueError> {
        if let Some(projecting) = &mut self.projecting {
            if let Some(point) = projecting.step(body)? {
                let delta = point - self.target;
                let distance = delta.x.hypot(delta.y).hypot(delta.z);
                if !distance.is_finite() { return Err(invalid()); }
                if self.best.is_none_or(|(_, best)| distance < best) { self.best = Some((point, distance)); }
                self.projecting = None;
                self.face += 1;
            }
            return Ok(None);
        }
        let Some(solid) = self.solid() else {
            return self.best.map(|(point, _)| Some(point)).ok_or_else(invalid);
        };
        let Some(shell) = self.shell(body, solid)? else {
            self.member += 1;
            self.shell = 0;
            return Ok(None);
        };
        let faces = &body.shells.get(shell).ok_or_else(invalid)?.faces;
        if let Some(&face) = faces.get(self.face) {
            self.projecting = Some(PlanarClosest::new(face, self.target));
        } else {
            self.shell += 1;
            self.face = 0;
        }
        Ok(None)
    }
}
