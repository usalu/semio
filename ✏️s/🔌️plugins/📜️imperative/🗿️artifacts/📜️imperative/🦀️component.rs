//! 📜️ Imperative artifact — the document entity this plugin's app edits: a `Path` of control-flow
//! `Step`s (`state.set`/`log.print`/`control.if`/`control.while`/`math.add`/…), each addressable by a
//! [`PathRef`] for nested `control.*` bodies (drag-and-drop into blocks).
//!
//! `Path`/`Step` are NOT owned here — they live in the shared kernel crate `imperative_engine`
//! (`✏️s/🔨️modules/📜️imperative`, package `semio-s-kernel-imperative`; **do not confuse this kernel crate
//! with this plugin** — same "imperative" name, different crate, different location, a legitimate
//! dependency this plugin has always had). `Dictionary`/`Registry` come from the framework's
//! `neural_engine` kernel. This component re-exports the app-facing surface so every sibling taxonomy
//! node (`🔺️diff`, `🔧️op`, `🗣️dsl`, `🎒️pack`, `📡️spr`, `⚙️engine`) names one artifact-owned symbol
//! instead of reaching into either kernel path directly.

use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
use serde::{Deserialize, Serialize};

//#region 🔖️Types
pub use imperative_engine::{Path, Step};
pub use neural_engine::{Dictionary, Registry};

/// 🗂️ The `store::DocumentStore` schema key — deliberately distinct from `ImperativeDocument.schema`
/// (`"imperative.document"`, the field inside the document itself): this one keys the store envelope.
pub const IMPERATIVE_DOCUMENT_SCHEMA: &str = "imperative.document/v1";

/// 📍️ Address of a nested step list inside a control step body.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathRef {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub slot: Option<String>,
}

/// 📄️ Imperative path document envelope.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImperativeDocument {
    pub schema: String,
    pub path: Path,
    #[serde(default)]
    pub seed: Dictionary,
}

impl Default for ImperativeDocument {
    fn default() -> Self {
        Self { schema: "imperative.document".into(), path: Path::new(), seed: Dictionary::new() }
    }
}
//#endregion 🔖️Types

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::apps::imperative::create_imperative_app`'s `🔖️Manifest` region.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "computation.imperative".into(),
        name: "Imperative".into(),
        source_format: "imperative.document".into(),
        component_kind: "imperative".into(),
        dimension: "graph".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Computation, form: MediaForm::Imperative },
        schema: "imperative.document".into(),
        export_formats: vec![],
        import_formats: vec![],
    }
}
//#endregion 🔖️ArtifactKind

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    /// 🗂️ The manifest-facing `ArtifactKindSpec.schema` ("imperative.document") is deliberately NOT
    /// `IMPERATIVE_DOCUMENT_SCHEMA` ("imperative.document/v1") — the former names the artifact kind in
    /// the OS media catalogue, the latter keys the store envelope. Pinned so a future edit can't silently
    /// merge them.
    #[test]
    fn artifact_kind_keeps_the_media_schema_distinct_from_the_store_schema() {
        assert_eq!(artifact_kind().schema, "imperative.document");
        assert_eq!(IMPERATIVE_DOCUMENT_SCHEMA, "imperative.document/v1");
    }

    #[test]
    fn default_document_is_empty_with_the_bare_schema() {
        let document = ImperativeDocument::default();
        assert_eq!(document.schema, "imperative.document");
        assert!(document.path.steps.is_empty());
        assert!(document.seed.keys().next().is_none());
    }
}
//#endregion 🧪️Tests
