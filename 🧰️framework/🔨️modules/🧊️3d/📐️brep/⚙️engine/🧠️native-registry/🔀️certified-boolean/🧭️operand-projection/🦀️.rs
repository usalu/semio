//! 🧭️ Original acquisition of native Boolean operand face rosters.

use super::*;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress, RetirementDemand};
use semio_framework_value::retained_clone::retained_clone_grant_admits;

const MAXIMUM_OPERAND_FACES: usize = 128;

/// 🧾️ Inline, source-bound acquisition of the general Boolean's first face roster.
pub struct BooleanPreparation {
    owner: usize,
    revision: u64,
    sources: [SolidId; 2],
    faces: [[Option<FaceId>; MAXIMUM_OPERAND_FACES]; 2],
    counts: [usize; 2],
    operand: usize,
    shell: usize,
    face: usize,
}

impl BooleanPreparation {
    /// 📋️ Borrows a completed face roster for the next original Boolean phase.
    pub fn faces(&self, operand: usize) -> Option<impl Iterator<Item = FaceId> + '_> {
        if self.operand <= operand || operand >= self.sources.len() { return None; }
        Some(self.faces[operand][..self.counts[operand]].iter().flatten().copied())
    }
    fn selected(&self, body: &Body) -> Result<(Option<ShellId>, Option<FaceId>), ValueError> {
        let solid = body.solids.get(self.sources[self.operand]).ok_or_else(|| ValueError::invariant("original Boolean operand arena slot disappeared"))?;
        let shell = if self.shell == 0 { Some(solid.outer) } else { solid.inners.get(self.shell - 1).copied() };
        let face = shell.map(|id| body.shells.get(id).ok_or_else(|| ValueError::invariant("original Boolean shell disappeared")))
            .transpose()?.and_then(|shell| shell.faces.get(self.face).copied());
        Ok((shell, face))
    }
}

impl BrepRegistry {
    fn boolean_sources(&self, a: &str, b: &str) -> Result<[SolidId; 2], ValueError> {
        let resolve = |key| match self.live.get(key) {
            Some(Entity::Solid(id, _)) => Ok(*id),
            _ => Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Boolean operands must be live solids")),
        };
        let sources = [resolve(a)?, resolve(b)?];
        if sources[0] == sources[1] { return Err(ValueError::literal(ValueRefusalKind::InvalidValue, "Boolean operands must be distinct solids")); }
        Ok(sources)
    }

    /// 🎟️ Begins read-only acquisition without entering cold topology cloning.
    pub fn plan_boolean_preparation(&self, a: &str, b: &str) -> Result<BooleanPreparation, ValueError> {
        let sources = self.boolean_sources(a, b)?;
        Ok(BooleanPreparation {
            owner: std::ptr::from_ref(self) as usize, revision: self.shape_mutation_revision, sources,
            faces: [[None; MAXIMUM_OPERAND_FACES]; 2], counts: [0;2], operand: 0, shell: 0, face: 0,
        })
    }

    /// 🪙️ Quotes one source face or one scalar projection boundary.
    pub fn boolean_preparation_demand(&self, a: &str, b: &str, preparation: &BooleanPreparation) -> Result<RetirementDemand, ValueError> {
        if preparation.owner != std::ptr::from_ref(self) as usize || preparation.revision != self.shape_mutation_revision
            || preparation.sources != self.boolean_sources(a,b)? {
            return Err(ValueError::invariant("original Boolean preparation source changed"));
        }
        let mut depth = self.live.next_contains_depth_demand(a)?.max(self.live.next_contains_depth_demand(b)?).max(1);
        if preparation.operand == 2 { return Ok(RetirementDemand { depth, ..Default::default() }); }
        depth = depth.max(self.body.solids.next_remove_depth_demand(preparation.sources[preparation.operand])?);
        let (shell, face) = preparation.selected(&self.body)?;
        if let Some(id) = shell { depth = depth.max(self.body.shells.next_remove_depth_demand(id)?); }
        if let Some(id) = face { depth = depth.max(self.body.faces.next_remove_depth_demand(id)?); }
        Ok(RetirementDemand { copy_bytes: if face.is_some() { size_of::<FaceId>() } else { 0 }, depth, ..Default::default() })
    }

    /// ⏱️ Captures one original face under its physical copy and work grant.
    pub fn step_boolean_preparation(&self, a: &str, b: &str, preparation: &mut BooleanPreparation, effect: &mut RetainedAllocationEffect<'_>)
        -> Result<(bool, RetainedCloneProgress), ValueError> {
        let demand = self.boolean_preparation_demand(a,b,preparation)?;
        if !retained_clone_grant_admits(effect.grant(), demand) || !effect.performed(1)? {
            return Ok((false, RetainedCloneProgress::default()));
        }
        let receipt = RetainedCloneProgress { copied_items:1, copied_bytes:demand.copy_bytes, ..Default::default() };
        if preparation.operand == 2 { return Ok((true, receipt)); }
        let (shell, face) = preparation.selected(&self.body)?;
        if let Some(face) = face {
            let count = &mut preparation.counts[preparation.operand];
            let slot = preparation.faces[preparation.operand].get_mut(*count)
                .ok_or_else(|| ValueError::literal(ValueRefusalKind::OwnershipLimit, "original Boolean preparation exceeds 128 faces per operand").with_retained_progress(receipt))?;
            *slot = Some(face);
            *count += 1;
            preparation.face += 1;
        } else if shell.is_some() {
            preparation.shell += 1;
            preparation.face = 0;
        } else {
            preparation.operand += 1;
            preparation.shell = 0;
        }
        Ok((false, receipt))
    }
}
