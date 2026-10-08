//! 🧬️ Direct reorder-buffers mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.reorder-buffers.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/buffers"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfReorderBuffersPayload {
    pub order: Vec<usize>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfReorderBuffersPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.order.len() != base.document.buffers.len() || payload.order.iter().collect::<std::collections::BTreeSet<_>>().len() != payload.order.len() || payload.order.iter().any(|index| *index >= base.document.buffers.len()) {
        return Err(reject("gltf.mutation.invalid-permutation", "document/buffers", "order must contain every index once"));
    }
    if payload.order.iter().enumerate().all(|(index, value)| index == *value) {
        return Err(reject("gltf.mutation.no-observable-change", "document/buffers", "order already matches"));
    }
    if base.document.buffers.len() != base.buffers.len() {
        return Err(reject("gltf.mutation.buffer-alignment", "buffers", "descriptor and bytes arrays must align"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfReorderBuffersPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Buffers, &mut after_reorder(&p.order));
    let slot = diff.buffers.get_or_insert_with(Default::default);
    let bytes = diff.buffer_bytes.get_or_insert_with(Default::default);
    for (new, old) in p.order.iter().enumerate() {
        if new != *old {
            slot.removed.push(*old);
            slot.added.push(GltfAdded { index: new, item: base.document.buffers[*old].clone() });
            bytes.removed.push(*old);
            bytes.added.push(GltfAdded { index: new, item: base.buffers[*old].clone() });
        }
    }
    slot.removed.sort_unstable();
    bytes.removed.sort_unstable();
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfReorderBuffersPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::reorder_buffers::mutation(super::reorder_buffers::GltfReorderBuffersPayload { order: inverse_order(&p.order) })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ReorderBuffersMutation {
    Apply(GltfReorderBuffersPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfReorderBuffersPayload) -> super::GltfMutation {
    super::GltfMutation::ReorderBuffers(ReorderBuffersMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ReorderBuffersMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "reorder", entity: "buffers", kind: "reorder-buffers", record: "ReorderedBuffers" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Reorder Buffers", "Puffer umordnen")
    }

    fn target(&self) -> Vec<String> {
        vec!["reorder-buffers".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔀️flips-the-two-1bb32a/🦀️.rs"]
mod case_flips_the_two_1bb32a;
//#endregion 🧪️Tests
