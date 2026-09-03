//! 🧬️ Playbook diff schema — sparse field delta over the artifact.
//!
//! Ticket `26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` (`playbook→C:document,flow`): the identified-
//! collection `steps: Option<PlaybookStepsDelta>` (and its nested `PlaybookBlocksDelta`/
//! `PlaybookStepPatch`/`PlaybookBlockPatch`) is replaced by single-Option whole-handle-replace
//! `document`/`flow` fields — the slots are never absent, only ever replaced, matching writer's
//! `document`/flow's `content` fields exactly (not `Option<Option<…>>` — that shape is for a slot
//! whose PRESENCE itself can change, e.g. lowpoly's `mesh`, which does not apply here).

use crate::artifacts::playbook::{PlaybookDocumentChild, PlaybookFlowChild};
use schema::ArtifactSchema;
// 🔬️ `Serialize`/`Deserialize` survive ONLY as a `#[cfg(test)]` differential oracle — committed
// `🧪️tests/<fixture>/🦀️.rs` fixture vectors decode/re-encode through them — never a production
// dependency of this crate.
#[cfg(test)]
use serde::{Deserialize, Serialize};

//#region 🔖️Diff
/// 🔺️ Sparse field delta for the playbook artifact; persistent entries apply via [`MutationDiff`](protocol::MutationDiff).
#[derive(Clone, Debug, Default, PartialEq, ArtifactSchema)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[artifact_schema(id = "s.playbook.playbook")]
pub struct PlaybookDiff {
    #[state(artifact)]
    pub artifact: Option<Box<crate::artifacts::playbook::schema::PlaybookArtifact>>,
    #[state(artifact)]
    pub schema: Option<String>,
    #[state(artifact)]
    pub id: Option<String>,
    #[state(artifact)]
    pub version: Option<String>,
    #[state(artifact)]
    pub title: Option<Option<String>>,
    #[state(artifact)]
    pub document: Option<PlaybookDocumentChild>,
    #[state(artifact)]
    pub flow: Option<PlaybookFlowChild>,
    #[state(presence)]
    pub selected_ids: Option<PlaybookStringList>,
    #[state(config)]
    pub locale: Option<String>,
    #[state(config)]
    pub contributions_json: Option<String>,
}
//#endregion 🔖️Diff

//#region 🔖️DeltaHelpers
/// 📋 String-list wrapper so optional list diffs stay scalar across formats.
#[derive(Clone, Debug, Default, PartialEq, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(rename_all = "camelCase", default))]
#[value(rename_all = "camelCase", default)]
pub struct PlaybookStringList {
    pub values: Vec<String>,
}
//#endregion 🔖️DeltaHelpers

//#region 🔖️ValueCodec
/// 🔀️ Hand-written, not derived: `document`/`flow` are `store::ArtifactChild<S>` composed-artifact
/// handles bridged through `to_dsl_value`/`from_dsl_value` (see the sibling `🧬️schema/🦀️component.rs`
/// impl for [`crate::artifacts::playbook::schema::PlaybookArtifact`] — same trap, same fix). This is
/// [`crate::mutation::MutationDiff::Diff`]'s own wire shape, so it must implement `ToValue`/
/// `FromValue`, not just the domain types it composes.
impl ::semio_framework_os_kernel::ToValue for PlaybookDiff {
    fn to_value(&self) -> ::semio_framework_os_kernel::DslValue {
        ::semio_framework_os_kernel::DslValue::object([
            ("artifact".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.artifact)),
            ("schema".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.schema)),
            ("id".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.id)),
            ("version".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.version)),
            ("title".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.title)),
            ("document".to_string(), self.document.as_ref().map_or(::semio_framework_os_kernel::DslValue::Null, |document| ::semio_framework_os_kernel::to_dsl_value(document).expect("ArtifactChild serializes"))),
            ("flow".to_string(), self.flow.as_ref().map_or(::semio_framework_os_kernel::DslValue::Null, |flow| ::semio_framework_os_kernel::to_dsl_value(flow).expect("ArtifactChild serializes"))),
            ("selectedIds".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.selected_ids)),
            ("locale".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.locale)),
            ("contributionsJson".to_string(), ::semio_framework_os_kernel::ToValue::to_value(&self.contributions_json)),
        ])
    }
}
impl ::semio_framework_os_kernel::FromValue for PlaybookDiff {
    fn from_value(value: ::semio_framework_os_kernel::DslValue) -> Result<Self, ::semio_framework_os_kernel::ValueError> {
        let entries = value.into_object()?;
        let get = |key: &str| entries.iter().find(|(k, _)| k == key).map(|(_, v)| v.clone());
        let child = |key: &str| -> Result<Option<PlaybookDocumentChild>, ::semio_framework_os_kernel::ValueError> {
            match get(key) {
                None | Some(::semio_framework_os_kernel::DslValue::Null) => Ok(None),
                Some(value) => ::semio_framework_os_kernel::from_dsl_value(value).map(Some).map_err(::semio_framework_os_kernel::ValueError::new),
            }
        };
        let flow_child = |key: &str| -> Result<Option<PlaybookFlowChild>, ::semio_framework_os_kernel::ValueError> {
            match get(key) {
                None | Some(::semio_framework_os_kernel::DslValue::Null) => Ok(None),
                Some(value) => ::semio_framework_os_kernel::from_dsl_value(value).map(Some).map_err(::semio_framework_os_kernel::ValueError::new),
            }
        };
        Ok(Self {
            artifact: get("artifact").map_or(Ok(None), ::semio_framework_os_kernel::FromValue::from_value)?,
            schema: get("schema").map_or(Ok(None), ::semio_framework_os_kernel::FromValue::from_value)?,
            id: get("id").map_or(Ok(None), ::semio_framework_os_kernel::FromValue::from_value)?,
            version: get("version").map_or(Ok(None), ::semio_framework_os_kernel::FromValue::from_value)?,
            title: get("title").map_or(Ok(None), ::semio_framework_os_kernel::FromValue::from_value)?,
            document: child("document")?,
            flow: flow_child("flow")?,
            selected_ids: get("selectedIds").map_or(Ok(None), ::semio_framework_os_kernel::FromValue::from_value)?,
            locale: get("locale").map_or(Ok(None), ::semio_framework_os_kernel::FromValue::from_value)?,
            contributions_json: get("contributionsJson").map_or(Ok(None), ::semio_framework_os_kernel::FromValue::from_value)?,
        })
    }
}
//#endregion 🔖️ValueCodec
