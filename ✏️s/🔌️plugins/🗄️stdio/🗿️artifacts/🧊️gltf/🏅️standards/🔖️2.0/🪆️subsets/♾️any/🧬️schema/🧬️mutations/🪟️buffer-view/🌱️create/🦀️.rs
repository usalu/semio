//! 🧬️ Direct create-buffer-view mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.create-buffer-view.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/bufferViews"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfCreateBufferViewPayload {
    pub position: usize,
    pub buffer: usize,
    pub byte_offset: usize,
    pub byte_length: usize,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub buffer_view: Option<Box<GltfBufferView>>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfCreateBufferViewPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.position > base.document.buffer_views.len() {
        return Err(reject("gltf.mutation.insert-out-of-range", "document/bufferViews", "position must be within the collection"));
    }
    let lengths = GltfLengths::after_insert(base, GltfTopLevelFamily::BufferViews);
    if let Some(record) = &payload.buffer_view {
        if !(record.buffer == payload.buffer && record.byte_offset == payload.byte_offset && record.byte_length == payload.byte_length) {
            return Err(reject("gltf.mutation.record-mismatch", "document/bufferViews", "the record must carry the fields the payload names"));
        }
        check_buffer_view(record, &lengths)?;
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfCreateBufferViewPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::BufferViews, &mut after_insert(p.position));
    diff.buffer_views.get_or_insert_with(Default::default).added.push(GltfAdded { index: p.position, item: p.buffer_view.as_deref().cloned().unwrap_or_else(|| GltfBufferView { buffer: p.buffer, byte_offset: p.byte_offset, byte_length: p.byte_length, byte_stride: None, target: None, name: None, extensions: None, extras: None }) });
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfCreateBufferViewPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![crate::standards::v2_0::subsets::any::schema::mutations::delete_buffer_view::mutation(crate::standards::v2_0::subsets::any::schema::mutations::delete_buffer_view::GltfDeleteBufferViewPayload { index: p.position })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum CreateBufferViewMutation {
    Apply(GltfCreateBufferViewPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfCreateBufferViewPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::create_buffer_view::CreateBufferViewMutation>,
{
    Op::from(CreateBufferViewMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for CreateBufferViewMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<CreateBufferViewMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "buffer-view", kind: "create-buffer-view", record: "CreatedBufferView" };

    fn label(&self) -> semio_framework_ui_locale::LocalizedLabel {
        semio_framework_ui_locale::LocalizedLabel::native("Create Buffer View", "Pufferansicht erstellen")
    }

    fn target(&self) -> Vec<String> {
        vec!["create-buffer-view".to_string()]
    }
}

impl<Op> protocol::ColdMutationKind<GltfSnapshot, Op> for CreateBufferViewMutation
where
    Op: protocol::ColdMutation<GltfSnapshot, ReferenceDiff = GltfDiff> + From<CreateBufferViewMutation>,
{
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
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests

//#endregion 🧪️Tests
