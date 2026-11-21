//! 🐚️ Captures original Shell inputs and explicitly selected openings.

use super::*;
use semio_framework_3d_brep_solid_transaction::ShellStock;

fn invalid(message: &'static str) -> ValueError {
    ValueError::literal(ValueRefusalKind::InvalidValue, message)
}

/// 🧱️ Captures the exact stock recipe and one bit for each selected opening.
pub struct ShellDraft {
    owner: usize,
    revision: u64,
    source: SolidId,
    outer: ShellId,
    stock: ShellStock,
    open_sides: u8,
}

impl ShellDraft {
    fn check(&self, registry: &BrepRegistry) -> Result<(), ValueError> {
        if self.owner == std::ptr::from_ref(registry) as usize && self.revision == registry.shape_mutation_revision {
            Ok(())
        } else {
            Err(ValueError::invariant("shell source changed during the original operation"))
        }
    }
}

impl BrepRegistry {
    /// 📦️ Admits a certified source and a positive wall thickness.
    pub fn plan_shell(&self, handle: &str, thickness: f64) -> Result<ShellDraft, ValueError> {
        let unsupported=||ValueError::literal(ValueRefusalKind::UnsupportedOwner,"original Shell requires certified stock");
        let Entity::Solid(id,Some(source))=self.original_query_entity(handle)? else{return Err(unsupported());};
        let stock=match source{
            SolidProvenance::Primitive(source)=>ShellStock::new(source.primitive,thickness)?,
            SolidProvenance::PlanarBoundary(Some(source))=>ShellStock::oriented_box(source.rectangular().ok_or_else(unsupported)?,thickness)?,
            _=>return Err(unsupported()),
        };
        let solid = self.body.solids.get(*id).ok_or_else(|| ValueError::invariant("original shell stock is missing"))?;
        Ok(ShellDraft {
            owner: std::ptr::from_ref(self) as usize, revision: self.shape_mutation_revision, source: *id, outer: solid.outer,
            stock, open_sides:0,
        })
    }

    /// 🎟️ Quotes the original face, support and source shell paths for one selected opening.
    pub fn shell_face_depth(&self, draft: &ShellDraft, handle: &str) -> Result<usize, ValueError> {
        draft.check(self)?;
        let Entity::Face(id) = self.original_query_entity(handle)? else { return Err(invalid("shell opening must be an original source face")); };
        let face = self.body.faces.get(*id).ok_or_else(|| invalid("shell opening face is unavailable"))?;
        Ok(self.tessellation_source_depth_demand(handle)?.max(self.body.shells.next_remove_depth_demand(draft.outer)?)
            .max(self.body.faces.next_remove_depth_demand(*id)?).max(self.body.surfaces.next_remove_depth_demand(face.surface)?))
    }

    /// 🪟️ Captures one open side from a planar face belonging to the original stock.
    pub fn add_shell_face(&self, draft: &mut ShellDraft, handle: &str) -> Result<(), ValueError> {
        draft.check(self)?;
        if matches!(draft.stock,ShellStock::Sphere(_)){return Err(invalid("closed spherical Shell requires no open faces"));}
        let invalid = || invalid("shell opening must be a planar face on the source solid");
        let Entity::Face(id) = self.original_query_entity(handle)? else { return Err(invalid()); };
        let outer = self.body.shells.get(draft.outer).ok_or_else(invalid)?;
        if !outer.faces.contains(id) { return Err(invalid()); }
        let face = self.body.faces.get(*id).ok_or_else(invalid)?;
        let Some(Surface::Plane { frame }) = self.body.surfaces.get(face.surface) else { return Err(invalid()); };
        let normal = if face.flipped { -frame.z } else { frame.z };
        let side = draft.stock.opening(normal)?;
        draft.open_sides |= 1u8 << side;
        Ok(())
    }

    /// 🪚️ Builds the exact stock-minus-cavity boundary with the selected sides open.
    pub fn finish_shell(&mut self, draft: &ShellDraft, handle: &str) -> Result<SolidPublication, ValueError> {
        draft.check(self)?;
        if !matches!(self.original_query_entity(handle)?, Entity::Solid(id, _) if *id == draft.source) {
            return Err(ValueError::invariant("original shell source no longer matches"));
        }
        self.plan_solid_boundary(draft.stock.finish(draft.open_sides)?)
    }
}
