//! 🧬️ Direct create-accessor mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::standards::v2_0::subsets::any::schema::snapshot::{GltfAccessorType, GltfComponentType};
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::snapshot::*;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.create-accessor.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/accessors"];
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfCreateAccessorPayload {
    pub position: usize,
    pub component_type: GltfComponentType,
    pub count: usize,
    pub kind: GltfAccessorType,
    #[value(default, skip_serializing_if = "Option::is_none")]
    pub accessor: Option<Box<GltfAccessor>>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfCreateAccessorPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.position > base.document.accessors.len() {
        return Err(reject("gltf.mutation.insert-out-of-range", "document/accessors", "position must be within the collection"));
    }
    let lengths = GltfLengths::after_insert(base, GltfTopLevelFamily::Accessors);
    if let Some(record) = &payload.accessor {
        if !(record.component_type == payload.component_type && record.count == payload.count && record.kind == payload.kind) {
            return Err(reject("gltf.mutation.record-mismatch", "document/accessors", "the record must carry the fields the payload names"));
        }
        check_accessor(record, &lengths)?;
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfCreateAccessorPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let mut diff = rewire(base, GltfTopLevelFamily::Accessors, &mut after_insert(p.position));
    diff.accessors.get_or_insert_with(Default::default).added.push(GltfAdded { index: p.position, item: p.accessor.as_deref().cloned().unwrap_or_else(|| GltfAccessor { buffer_view: None, byte_offset: 0, component_type: p.component_type, normalized: false, count: p.count, kind: p.kind, max: None, min: None, sparse: None, name: None, extensions: None, extras: None }) });
    Ok(diff)
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfCreateAccessorPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    vec![super::delete_accessor::mutation(super::delete_accessor::GltfDeleteAccessorPayload { index: p.position })]
}

//#region 🧬️DirectMutation
#[derive(semio_framework_value::RetireOwned, Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum CreateAccessorMutation {
    Apply(GltfCreateAccessorPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfCreateAccessorPayload) -> super::GltfMutation {
    super::GltfMutation::CreateAccessor(CreateAccessorMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for CreateAccessorMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "create", entity: "accessor", kind: "create-accessor", record: "CreatedAccessor" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Create Accessor", "Accessor erstellen")
    }

    fn target(&self) -> Vec<String> {
        vec!["create-accessor".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t054/🦀️.rs"]
mod case_t054;
//#endregion 🧪️Tests
