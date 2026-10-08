//! 🧬️ Direct reorder-primitives mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::structure_geometry::checked_index;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-primitives.v1";
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfReorderPrimitivesPayload {
    pub mesh: usize,
    pub order: Vec<usize>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderPrimitivesPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    checked_index(payload.mesh, base.document.meshes.len(), "document/meshes")?;
    let length = base.document.meshes[payload.mesh].primitives.len();
    if payload.order.len() != length || payload.order.iter().any(|index| *index >= length) || {
        let mut order = payload.order.clone();
        order.sort_unstable();
        order.dedup();
        order.len() != length
    } {
        return Err(reject("gltf.mutation.invalid-permutation", "document/meshes/primitives", "order must contain each primitive once"));
    }
    if payload.order.iter().enumerate().all(|(index, value)| *value == index) {
        return Err(reject("gltf.mutation.no-observable-change", "document/meshes/primitives", "reorder must change order"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfReorderPrimitivesPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let moved: Vec<(usize, usize)> = p.order.iter().copied().enumerate().filter(|(new, old)| new != old).collect();
    let mut removed: Vec<usize> = moved.iter().map(|(_, old)| *old).collect();
    removed.sort_unstable();
    let added = moved.iter().map(|(new, old)| GltfAdded { index: *new, item: base.document.meshes[p.mesh].primitives[*old].clone() }).collect();
    Ok(GltfDiff { meshes: primitives_rows(p.mesh, GltfPrimitivesDiff { removed, added, ..Default::default() }), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfReorderPrimitivesPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::reorder_primitives::mutation(super::reorder_primitives::GltfReorderPrimitivesPayload { mesh: p.mesh, order: inverse_order(&p.order) })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ReorderPrimitivesMutation {
    Apply(GltfReorderPrimitivesPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfReorderPrimitivesPayload) -> super::GltfMutation {
    super::GltfMutation::ReorderPrimitives(ReorderPrimitivesMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ReorderPrimitivesMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "primitives", kind: "reorder-primitives", record: "ReorderedPrimitives" };

    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<crate::schema::diff::GltfDiff> {
        match self {
            Self::Apply(payload) => match plan(payload, base) {
                Ok(diff) => protocol::MutationOutcome::new(diff),
                Err(error) => rejection_outcome(&error.code, &error.path, error.detail),
            },
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Result<Vec<super::GltfMutation>, semio_framework_value::ValueError> {
        match self {
            Self::Apply(payload) => Ok(inverse(payload, base)),
        }
    }

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Reorder Primitives", "Primitive umordnen")
    }

    fn target(&self) -> Vec<String> {
        vec!["reorder-primitives".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t062/🦀️.rs"]
mod case_t062;
//#endregion 🧪️Tests
