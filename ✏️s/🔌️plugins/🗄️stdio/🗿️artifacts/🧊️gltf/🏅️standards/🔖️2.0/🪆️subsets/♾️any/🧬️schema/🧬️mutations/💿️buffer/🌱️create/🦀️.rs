//! 🧬️ Direct create-buffer mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
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
pub fn inverse<Op>(p: &GltfCreateBufferPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![crate::standards::v2_0::subsets::any::schema::mutations::delete_buffer::mutation(crate::standards::v2_0::subsets::any::schema::mutations::delete_buffer::GltfDeleteBufferPayload { index: p.position })]
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
pub fn mutation<Op>(payload: GltfCreateBufferPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::create_buffer::CreateBufferMutation>,
{
    Op::from(CreateBufferMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for CreateBufferMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<CreateBufferMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "buffer", kind: "create-buffer", record: "CreatedBuffer" };

    fn diff(&self, base: &GltfSnapshot) -> protocol::MutationOutcome<GltfDiff> {
        match self {
            Self::Apply(payload) => match plan(payload, base) {
                Ok(diff) => protocol::MutationOutcome::new(diff),
                Err(error) => rejection_outcome(&error.code, &error.path, error.detail),
            },
        }
    }

    fn inverse(&self, base: &GltfSnapshot) -> Result<Vec<Op>, semio_framework_value::ValueError> {
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

//#endregion 🧪️Tests
