//! 🧬️ Direct delete-buffer-view mutation owner: payload, validation, typed diff, inverse, and outcomes.
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::standards::v2_0::subsets::any::schema::modules::mutation_support::top_level_collections::*;
use semio_s_artifact_stdio_gltf_model::standards::v2_0::subsets::any::schema::snapshot::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.delete-buffer-view.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/bufferViews"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(rename_all = "camelCase")]
pub struct GltfDeleteBufferViewPayload {
    pub index: usize,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfDeleteBufferViewPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.index >= base.document.buffer_views.len() {
        return Err(reject("gltf.mutation.index-out-of-range", "document/bufferViews", "index must address an item"));
    }
    require_unreferenced(base, GltfTopLevelFamily::BufferViews, payload.index)?;
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfDeleteBufferViewPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::BufferViews, &mut after_delete(p.index));
    let slot = diff.buffer_views.get_or_insert_with(Default::default);
    slot.removed.push(p.index);
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse<Op>(p: &GltfDeleteBufferViewPayload, base: &GltfSnapshot) -> Vec<Op>
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::remove_used_extension::RemoveUsedExtensionMutation>,
{
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let mut rows = vec![crate::standards::v2_0::subsets::any::schema::mutations::create_buffer_view::mutation(crate::standards::v2_0::subsets::any::schema::mutations::create_buffer_view::GltfCreateBufferViewPayload { position: p.index, buffer: base.document.buffer_views[p.index].buffer, byte_offset: base.document.buffer_views[p.index].byte_offset, byte_length: base.document.buffer_views[p.index].byte_length, buffer_view: Some(Box::new(base.document.buffer_views[p.index].clone())) })];

    rows.reverse();
    rows
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum DeleteBufferViewMutation {
    Apply(GltfDeleteBufferViewPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation<Op>(payload: GltfDeleteBufferViewPayload) -> Op
where
    Op: From<crate::standards::v2_0::subsets::any::schema::mutations::delete_buffer_view::DeleteBufferViewMutation>,
{
    Op::from(DeleteBufferViewMutation::Apply(payload))
}

impl<Op> protocol::MutationKind<GltfSnapshot, Op> for DeleteBufferViewMutation
where
    Op: protocol::Mutation<GltfSnapshot, Diff = GltfDiff> + From<DeleteBufferViewMutation>,
{
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "delete", entity: "buffer-view", kind: "delete-buffer-view", record: "DeletedBufferView" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Delete Buffer View", "Pufferansicht löschen")
    }

    fn target(&self) -> Vec<String> {
        vec!["delete-buffer-view".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests


//#endregion 🧪️Tests
