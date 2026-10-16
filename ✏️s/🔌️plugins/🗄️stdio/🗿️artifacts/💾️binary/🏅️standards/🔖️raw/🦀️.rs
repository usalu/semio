//! 🏅️ Standard root for `s.stdio.binary` standard `raw` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P pilot). Exports
//! `standard() -> StandardDeclaration`, mounting subset `any` (the only subset this standard
//! has). Media values are the REAL `FormatDescriptor` registration for `stdio.binary`, taken
//! verbatim from `📜️artifact-definition.json`'s `representations[0]`
//! (`mimes: ["application/octet-stream"], extensions: [".bin"]`) — not invented.

use crate::standards::v_raw::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

/// 🌳️ `standard "raw"`'s complete declaration — one subset, `any`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn standard() -> StandardDeclaration<crate::BinaryApps> {
    StandardDeclaration { id: StandardId("raw"), media: MediaDeclaration { mimes: &["application/octet-stream"], extensions: &["bin"] }, subsets: vec![subsets::any::subset()] }
}

//#region 🧩️ArtifactItems7
pub mod artifact_items_7 {
use crate::{definition};
use crate::{formats};
use crate::{STDIO_BINARY_DOCUMENT_SCHEMA};
use semio_framework_plugin::{ArtifactDeclaration, ArtifactDefinition, ArtifactDefinitionError, PluginAssemblyError};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
pub use crate::schema::mutations::BinaryMutation;
pub use crate::schema::snapshot::BinarySnapshot;

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("binary", definition()?, declaration)
}

/// 🧾️ The runtime `binary` declares: its schema, format, inference descriptor, composers and document codec.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: ArtifactDefinition) -> Result<ArtifactDeclaration, ArtifactDefinitionError> {
    let formats = formats()?;
    ArtifactDeclaration::builder(definition)
        .schema(crate::standards::v_raw::subsets::any::schema::binary_artifact_schema_descriptor())
        .formats(formats)
        .inferences([crate::standards::v_raw::subsets::any::schema::inferences::binary_artifact_inference_descriptor()])
        .composers(crate::standards::v_raw::subsets::any::io::io_registry::entries())
        .document_codec_bare::<BinarySnapshot, BinaryMutation>(STDIO_BINARY_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.binary", standard: StandardId("raw"), subset: SubsetId("*") })
        .try_build()
}

//#region 🔖️ArtifactDeclaration
/// 🌳️ New tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P pilot): the whole
/// `s.stdio.binary` artifact through the declaration tree — one standard, `raw`, one subset,
/// `any`. `localization: &[]` — the real en/de localized descriptors already live in
/// `📜️artifact-definition.json` (the OLD `ArtifactDefinition` channel this artifact stays
/// on for `📇️registry`'s catalog, kept per `assembly()` below); wiring them into this NEW field
/// too is a follow-up, not required for the carrier law or for this tree to register cleanly
/// (see `📓️w2-p-report.md` `## openQuestions`).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact() -> declarations::ArtifactDeclaration<BinaryApps> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.stdio.binary").expect("canonical stdio.binary kind"), localization: &[], standards: vec![crate::standards::v_raw::standard()] }
}
//#endregion 🔖️ArtifactDeclaration

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.binary".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Binary", "Binärdatei"),
        source_format: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Data, form: MediaForm::Value },
        schema: STDIO_BINARY_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems7

//#region 🧩️ArtifactItems8
pub mod artifact_items_8 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};
use crate::{definition};
use crate::{assembly};
use crate::{formats};
use crate::{native_codecs};

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "binary", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}
}
//#endregion 🧩️ArtifactItems8
