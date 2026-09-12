//! 🧬️ Curation snapshot schema — artifact-lane fields only.

use crate::{CuratedItem, ObjectKindExtra};
use framework_schema::ArtifactSchema;
use semio_s_artifact_stdio_semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Snapshot
/// 📸️ Persisted Kit catalog child, sourcing geometry and availability, and ordered selection.
#[derive(Clone, Debug, PartialEq, dsl::ToValue, dsl::DslRecord, ArtifactSchema)]
#[value(rename_all = "camelCase")]
#[dsl(id = "curation.curation", layout = "lines")]
#[artifact_schema(id = "s.sourcing.curation")]
pub struct CurationSnapshot {
    #[state(artifact)]
    #[child(kind = "s.stdio.semio")]
    pub catalog: store::ArtifactChild<SemioKitSnapshot>,
    #[state(artifact)]
    #[value(default)]
    pub stock_extra: Vec<ObjectKindExtra>,
    #[state(artifact)]
    #[value(default)]
    #[dsl(table)]
    pub curated: Vec<CuratedItem>,
}

impl Default for CurationSnapshot {
    /// 🌱 Builds the empty catalog and selection through the document's child constructor.
    fn default() -> Self {
        Self { catalog: crate::catalog_child_handle(&[]), stock_extra: Vec::new(), curated: Vec::new() }
    }
}

impl dsl::FromValue for CurationSnapshot {
    fn from_value(value: dsl::DslValue) -> Result<Self, dsl::ValueError> {
        let mut catalog = None;
        let mut stock_extra = None;
        let mut curated = None;
        for (key, value) in dsl::DslValue::into_object(value)? {
            match key.as_str() {
                "catalog" if catalog.is_none() => catalog = Some(dsl::FromValue::from_value(value)?),
                "stockExtra" if stock_extra.is_none() => stock_extra = Some(dsl::FromValue::from_value(value)?),
                "curated" if curated.is_none() => curated = Some(dsl::FromValue::from_value(value)?),
                _ => return Err(dsl::ValueError::new(format!("unknown or duplicate Curation field {key}"))),
            }
        }
        let result = Self { catalog: catalog.ok_or_else(|| dsl::ValueError::new("missing catalog"))?, stock_extra: stock_extra.ok_or_else(|| dsl::ValueError::new("missing stockExtra"))?, curated: curated.ok_or_else(|| dsl::ValueError::new("missing curated"))? };
        result.validate().map_err(dsl::ValueError::new)?;
        Ok(result)
    }
}
impl CurationSnapshot {
    /// 🪆 Requires a Kit child with equal persisted slot and artifact identity.
    pub fn validate(&self) -> Result<(), String> {
        semio_s_artifact_stdio_semio::standards::v1::subsets::base::schema::child::validate_semio_child_identity(&self.catalog.child_id, &self.catalog.target, "kit")
    }
}
/// 🧺 Formats a persisted ordered selection for scenario diagnostics.
pub fn curation_selection_summary(snapshot: &CurationSnapshot) -> String {
    snapshot.curated.iter().map(|item| format!("{}x{}", item.object_id, item.count)).collect::<Vec<_>>().join(" ")
}
#[cfg(test)]
#[path = "../🧪️tests/🪪️document-contract/🦀️.rs"]
mod document_contract_tests;
