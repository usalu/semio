//! 🐚️ Original sharp shell construction from certified box planes.

use super::*;

fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}

/// 🧱️ Captures the exact source bounds and one bit for each selected opening.
pub struct BoxShellDraft {
    owner: usize,
    revision: u64,
    source: SolidId,
    outer: ShellId,
    stock: Bounds,
    thickness: f64,
    open_sides: u8,
}

impl BoxShellDraft {
    fn check(&self, registry: &BrepRegistry) -> Result<(), ValueError> {
        if self.owner == std::ptr::from_ref(registry) as usize && self.revision == registry.shape_mutation_revision {
            Ok(())
        } else {
            Err(ValueError::invariant("box shell source changed during the original operation"))
        }
    }
}

impl BrepRegistry {
    /// 📦️ Admits a certified rectangular source and a positive wall thickness.
    pub fn plan_box_shell(&self, handle: &str, thickness: f64) -> Result<BoxShellDraft, ValueError> {
        if !thickness.is_finite() || thickness <= Tol::DEFAULT.value() {
            return Err(invalid("shell thickness must be positive at kernel tolerance"));
        }
        let unsupported = || ValueError::literal(ValueRefusalKind::UnsupportedOwner, "original Shell requires a certified box");
        let Entity::Solid(id, Some(SolidProvenance::Primitive(source))) = self.original_query_entity(handle)? else { return Err(unsupported()); };
        let SolidPrimitive::Box { origin, width, depth, height } = source.primitive else { return Err(unsupported()); };
        let solid = self.body.solids.get(*id).ok_or_else(|| ValueError::invariant("original box solid is missing"))?;
        Ok(BoxShellDraft {
            owner: std::ptr::from_ref(self) as usize, revision: self.shape_mutation_revision, source: *id, outer: solid.outer,
            stock: Bounds::box_primitive(origin, width, depth, height), thickness, open_sides: 0,
        })
    }

    /// 🎟️ Quotes the original face, support and source shell paths for one selected opening.
    pub fn box_shell_face_depth(&self, draft: &BoxShellDraft, handle: &str) -> Result<usize, ValueError> {
        draft.check(self)?;
        let Entity::Face(id) = self.original_query_entity(handle)? else { return Err(invalid("shell opening must be an original source face")); };
        let face = self.body.faces.get(*id).ok_or_else(|| invalid("shell opening face is unavailable"))?;
        Ok(self.tessellation_source_depth_demand(handle)?.max(self.body.shells.next_remove_depth_demand(draft.outer)?)
            .max(self.body.faces.next_remove_depth_demand(*id)?).max(self.body.surfaces.next_remove_depth_demand(face.surface)?))
    }

    /// 🪟️ Captures one open side from a face belonging to the original certified box.
    pub fn add_box_shell_face(&self, draft: &mut BoxShellDraft, handle: &str) -> Result<(), ValueError> {
        draft.check(self)?;
        let invalid = || invalid("shell opening must be a face on the source box");
        let Entity::Face(id) = self.original_query_entity(handle)? else { return Err(invalid()); };
        let outer = self.body.shells.get(draft.outer).ok_or_else(invalid)?;
        if !outer.faces.contains(id) { return Err(invalid()); }
        let face = self.body.faces.get(*id).ok_or_else(invalid)?;
        let Some(Surface::Plane { frame }) = self.body.surfaces.get(face.surface) else { return Err(invalid()); };
        let normal = if face.flipped { -frame.z } else { frame.z };
        let side = [-NativeVec3::X, NativeVec3::X, -NativeVec3::Y, NativeVec3::Y, -NativeVec3::Z, NativeVec3::Z]
            .into_iter().position(|axis| axis == normal).ok_or_else(invalid)?;
        draft.open_sides |= 1u8 << side;
        Ok(())
    }

    /// 🪚️ Builds the exact stock-minus-cavity boundary with the selected sides open.
    pub fn finish_box_shell(&mut self, draft: &BoxShellDraft, handle: &str) -> Result<SolidPublication, ValueError> {
        draft.check(self)?;
        if !matches!(self.original_query_entity(handle)?, Entity::Solid(id, _) if *id == draft.source) {
            return Err(ValueError::invariant("original box shell source no longer matches"));
        }
        if draft.open_sides == 0b11_1111 { return Err(invalid("removing all box faces leaves no shell")); }
        let inset = |side| if draft.open_sides & (1u8 << side) == 0 { draft.thickness } else { 0.0 };
        let stock = draft.stock;
        let cavity = Bounds {
            lo: std::array::from_fn(|axis| stock.lo[axis] + inset(2 * axis)),
            hi: std::array::from_fn(|axis| stock.hi[axis] - inset(2 * axis + 1)),
        };
        if (0..3).any(|axis| {
            cavity.hi[axis] - cavity.lo[axis] <= Tol::DEFAULT.value()
                || (inset(2 * axis) != 0.0 && cavity.lo[axis] <= stock.lo[axis])
                || (inset(2 * axis + 1) != 0.0 && cavity.hi[axis] >= stock.hi[axis])
        }) { return Err(invalid("shell thickness collapsed an opening, cavity or wall")); }
        self.plan_orthogonal_solid(OrthogonalBoundary::new([stock.lo, stock.hi], [cavity.lo, cavity.hi], BooleanOp::Cut)?)
    }
}
