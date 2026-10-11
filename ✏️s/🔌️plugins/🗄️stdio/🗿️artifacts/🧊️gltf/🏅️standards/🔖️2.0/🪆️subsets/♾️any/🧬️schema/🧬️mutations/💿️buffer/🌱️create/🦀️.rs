//! 🧬️ Direct create-buffer mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::snapshot::*;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.create-buffer.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/buffers"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfCreateBufferPayload {
    pub position: usize,
    pub bytes: Vec<u8>,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub buffer: Option<Box<GltfBuffer>>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfCreateBufferPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.position > base.document.buffers.len() {
        return Err(reject("gltf.mutation.insert-out-of-range", "document/buffers", "position must be within the collection"));
    }
    if base.document.buffers.len() != base.buffers.len() {
        return Err(reject("gltf.mutation.buffer-alignment", "buffers", "descriptor and bytes arrays must align"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfCreateBufferPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Buffers, &mut after_insert(p.position));
    diff.buffers.get_or_insert_with(Default::default).added.push(GltfAdded { index: p.position, item: p.buffer.as_deref().cloned().unwrap_or_else(|| GltfBuffer { byte_length: p.bytes.len(), uri: None, name: None, extensions: None, extras: None }) });
    diff.buffer_bytes.get_or_insert_with(Default::default).added.push(GltfAdded { index: p.position, item: p.bytes.clone() });
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfCreateBufferPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::delete_buffer::mutation(super::delete_buffer::GltfDeleteBufferPayload { index: p.position })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum CreateBufferMutation {
    Apply(GltfCreateBufferPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfCreateBufferPayload) -> super::GltfMutation {
    super::GltfMutation::CreateBuffer(CreateBufferMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for CreateBufferMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "buffer", kind: "create-buffer", record: "CreatedBuffer" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Create Buffer", "Puffer erstellen")
    }

    fn target(&self) -> Vec<String> {
        vec!["create-buffer".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/💿️inserts-a-two-ab4132/🦀️.rs"]
mod case_inserts_a_two_ab4132;
//#endregion 🧪️Tests
