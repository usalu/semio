//! 🧬️ Direct change-document-extension-data mutation owner: payload, validation, typed diff, inverse, and outcomes.
use crate::schema::diff::*;
use crate::schema::modules::mutation_support::top_level_collections::*;
use crate::schema::modules::mutation_support::top_level::rejection_outcome;
use crate::schema::modules::mutation_support::top_level::{reject, GltfTopLevelMutationRejection};
use crate::schema::snapshot::GltfJson;
use crate::GltfSnapshot;
pub const ID: &str = "s.stdio.gltf.mutation.change-document-extension-data.v1";
pub const TOUCHED_PATHS: &[&str] = &["document/extensions"];
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue)]
#[value(rename_all = "camelCase")]
pub struct GltfChangeDocumentExtensionDataPayload {
    pub data: Option<GltfJson>,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn validate(payload: &GltfChangeDocumentExtensionDataPayload, base: &GltfSnapshot) -> Result<(), GltfTopLevelMutationRejection> {
    if payload.data == base.document.extensions {
        return Err(reject("gltf.mutation.no-observable-change", "document/extensions", "value already has this value"));
    }
    Ok(())
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn plan(p: &GltfChangeDocumentExtensionDataPayload, base: &GltfSnapshot) -> Result<GltfDiff, GltfTopLevelMutationRejection> {
    validate(p, base)?;
    let current = &base.document.extensions;
    Ok(GltfDiff { extensions: (current != &p.data).then(|| p.data.clone()), ..Default::default() })
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(p: &GltfChangeDocumentExtensionDataPayload, base: &GltfSnapshot) -> Vec<super::GltfMutation> {
    if validate(p, base).is_err() {
        return Vec::new();
    }
    let current = &base.document.extensions;
    if current == &p.data {
        return Vec::new();
    }
    vec![super::change_document_extension_data::mutation(super::change_document_extension_data::GltfChangeDocumentExtensionDataPayload { data: current.clone() })]
}

//#region 🧬️DirectMutation
#[derive(Clone, Debug, PartialEq, value_derive::ToValue, value_derive::FromValue, dsl::MutationLeaf)]
#[mutation_leaf(contract = ::protocol, payload = Apply)]
#[value(tag = "phase", content = "value", rename_all = "camelCase")]
pub enum ChangeDocumentExtensionDataMutation {
    Apply(GltfChangeDocumentExtensionDataPayload),
}

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mutation(payload: GltfChangeDocumentExtensionDataPayload) -> super::GltfMutation {
    super::GltfMutation::ChangeDocumentExtensionData(ChangeDocumentExtensionDataMutation::Apply(payload))
}

impl protocol::MutationKind<GltfSnapshot, super::GltfMutation> for ChangeDocumentExtensionDataMutation {
    const SEMANTICS: protocol::SemanticDescriptor = protocol::SemanticDescriptor { verb: "change", entity: "document-extension-data", kind: "change-document-extension-data", record: "ChangedDocumentExtensionData" };

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
        semio_framework_ui_locale::LocalizedLabel::native("Change Document Extension Data", "Erweiterungsdaten des Dokuments ändern")
    }

    fn target(&self) -> Vec<String> {
        vec!["change-document-extension-data".to_string()]
    }
}
//#endregion 🧬️DirectMutation

//#region 🧪️Tests
#[cfg(test)]
#[path = "🧪️tests/🔬️t053/🦀️.rs"]
mod case_t053;
//#endregion 🧪️Tests
