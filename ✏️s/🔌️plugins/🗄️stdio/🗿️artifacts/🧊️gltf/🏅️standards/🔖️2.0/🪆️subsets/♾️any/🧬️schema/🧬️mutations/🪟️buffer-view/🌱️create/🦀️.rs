//! 🧬️ Direct create-buffer-view mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::snapshot::*;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.create-buffer-view.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/bufferViews"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
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
pub fn inverse(p: &GltfCreateBufferViewPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::delete_buffer_view::mutation(super::delete_buffer_view::GltfDeleteBufferViewPayload { index: p.position })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum CreateBufferViewMutation {
    Apply(GltfCreateBufferViewPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfCreateBufferViewPayload) -> super::GltfMutation {
    super::GltfMutation::CreateBufferView(CreateBufferViewMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for CreateBufferViewMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "buffer-view", kind: "create-buffer-view", record: "CreatedBufferView" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Create Buffer View", "Pufferansicht erstellen")
    }

    fn target(&self) -> Vec<String> {
        vec!["create-buffer-view".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t066/🦀️.rs"]
mod case_t066;
//#endregion 🧪️Tests
