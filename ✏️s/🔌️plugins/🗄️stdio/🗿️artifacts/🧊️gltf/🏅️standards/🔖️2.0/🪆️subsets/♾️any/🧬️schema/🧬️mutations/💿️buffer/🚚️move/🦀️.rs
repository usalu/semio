//! 🧬️ Direct move-buffer mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.move-buffer.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/buffers"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfMoveBufferPayload {
    pub index: usize,
    pub position: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfMoveBufferPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.index >= base.document.buffers.len() || payload.position >= base.document.buffers.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/buffers", "indices must address items"));
    }
    if payload.index == payload.position {
        return Err(reject("gltf.mutation.no-observable-change", "document/buffers", "destination equals source"));
    }
    if base.document.buffers.len() != base.buffers.len() {
        return Err(reject("gltf.mutation.buffer-alignment", "buffers", "descriptor and bytes arrays must align"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfMoveBufferPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Buffers, &mut after_move(p.index, p.position));
    let slot = diff.buffers.get_or_insert_with(Default::default);
    slot.removed.push(p.index);
    slot.added.push(GltfAdded { index: p.position, item: base.document.buffers[p.index].clone() });
    diff.buffer_bytes.get_or_insert_with(Default::default).removed.push(p.index);
    diff.buffer_bytes.get_or_insert_with(Default::default).added.push(GltfAdded { index: p.position, item: base.buffers[p.index].clone() });
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfMoveBufferPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::move_buffer::mutation(super::move_buffer::GltfMoveBufferPayload { index: p.position, position: p.index })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum MoveBufferMutation {
    Apply(GltfMoveBufferPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfMoveBufferPayload) -> super::GltfMutation {
    super::GltfMutation::MoveBuffer(MoveBufferMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for MoveBufferMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "move", entity: "buffer", kind: "move-buffer", record: "MovedBuffer" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Move Buffer", "Puffer verschieben")
    }

    fn target(&self) -> Vec<String> {
        vec!["move-buffer".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🚚️swaps-the-two-a9f8e2/🦀️.rs"]
mod case_swaps_the_two_a9f8e2;
//#endregion 🧪️Tests
