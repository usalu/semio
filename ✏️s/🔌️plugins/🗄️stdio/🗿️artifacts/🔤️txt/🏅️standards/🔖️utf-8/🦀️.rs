//! 🏅️ Standard root for `s.stdio.txt` standard `utf-8` (ticket
//! 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P pilot). Exports
//! `standard() -> StandardDeclaration`, mounting subset `any` (the only subset this standard
//! has). Media values are the REAL `FormatDescriptor` registration for `stdio.txt`, taken
//! verbatim from `📜️artifact-definition.json`'s `representations[0]`
//! (`mimes: ["text/plain"], extensions: [".txt"]`) — not invented.

use crate::standards::v_utf_8::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

/// 🌳️ `standard "utf-8"`'s complete declaration — one subset, `any`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn standard() -> StandardDeclaration<crate::TxtApps> {
    StandardDeclaration { id: StandardId("utf-8"), media: MediaDeclaration { mimes: &["text/plain"], extensions: &["txt"] }, subsets: vec![subsets::any::subset()] }
}

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
/// 📜 Schema-owned package definition.
pub const ARTIFACT_DEFINITION_SCHEMA: &str = include_str!("../../📜️artifact-definition.json");
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems4
pub mod artifact_items_4 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};
use semio_framework_plugin::io::FormatDescriptor;
use semio_framework_plugin::ArtifactDefinitionError;

pub fn formats() -> Result<Vec<FormatDescriptor>, ArtifactDefinitionError> {
    semio_s_artifact_stdio_contract::format_descriptors(ARTIFACT_DEFINITION_SCHEMA)
}
}
//#endregion 🧩️ArtifactItems4

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{STDIO_TXT_DOCUMENT_SCHEMA};
pub use crate::schema::mutations::TxtMutation;
pub use crate::schema::snapshot::TxtSnapshot;

pub fn native_codec() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::artifact_codec::<TxtSnapshot, TxtMutation>(STDIO_TXT_DOCUMENT_SCHEMA);
    codec.extension = "txt";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio"));
    codec
}
}
//#endregion 🧩️ArtifactItems5

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
    use semio_framework_plugin::declarations;
use crate::{formats};
use crate::{STDIO_TXT_DOCUMENT_SCHEMA};
use crate::{native_codec};
use crate::TxtApps;
use crate::{ARTIFACT_DEFINITION_SCHEMA};
use semio_framework_plugin::{ArtifactDeclaration, ArtifactDefinition, ArtifactDefinitionError, PluginAssemblyError};
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
pub use crate::schema::mutations::TxtMutation;
pub use crate::schema::snapshot::TxtSnapshot;

pub fn definition() -> Result<ArtifactDefinition, PluginAssemblyError> {
    let factories = native_codecs();
    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.txt.v1", artifact: "txt", kind: artifact_kind, codec: native_codec }]
}

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn assembly() -> Result<semio_s_artifact_stdio_contract::ArtifactAssembly, PluginAssemblyError> {
    semio_s_artifact_stdio_contract::runtime_assembly("txt", definition()?, declaration)
}

/// 🧩️ The executable facets the hub-native document codec needs: the artifact schema, the declared representations
/// and the `stdio.txt` document codec the linked `stdio.native.txt.v1` receipt instantiates.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn declaration(definition: ArtifactDefinition) -> Result<ArtifactDeclaration, ArtifactDefinitionError> {
    let formats = formats()?;
    ArtifactDeclaration::builder(definition).schema(crate::standards::v_utf_8::subsets::any::schema::txt_artifact_schema_descriptor()).formats(formats).document_codec_bare::<TxtSnapshot, TxtMutation>(STDIO_TXT_DOCUMENT_SCHEMA, Dialect { artifact_kind: "s.stdio.txt", standard: StandardId("utf-8"), subset: SubsetId("*") }).try_build()
}

//#region 🔖️ArtifactDeclaration
/// 🌳️ New tree (ticket 26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM, W2-P pilot): the whole
/// `s.stdio.txt` artifact through the declaration tree — one standard, `utf-8`, one subset,
/// `any`. See `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🦀️.rs`'s `artifact()` doc
/// comment for the `localization: &[]` deferral rationale (identical here).
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact() -> declarations::ArtifactDeclaration<TxtApps> {
    use semio_framework_plugin::app::declarations::ArtifactDeclaration;
    use {semio_framework_artifact_reference::ArtifactKindId};
    ArtifactDeclaration { kind: ArtifactKindId::parse("s.stdio.txt").expect("canonical stdio.txt kind"), localization: &[], standards: vec![crate::standards::v_utf_8::standard()] }
}
//#endregion 🔖️ArtifactDeclaration

// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: "s.stdio.txt".into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Txt", "Txt"),
        source_format: STDIO_TXT_DOCUMENT_SCHEMA.into(),
        component_kind: "stdio".into(),
        dimension: "data".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Text, form: MediaForm::Document },
        schema: STDIO_TXT_DOCUMENT_SCHEMA.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems6

//#region 🧩️ArtifactItems7
pub mod artifact_items_7 {
use crate::{ARTIFACT_DEFINITION_SCHEMA};
use crate::{assembly, definition, native_codecs};
use crate::{formats};

pub fn contribution() -> semio_s_artifact_stdio_contract::ArtifactContribution {
    semio_s_artifact_stdio_contract::ArtifactContribution { definition_constraint: None, identity: "txt", schema: ARTIFACT_DEFINITION_SCHEMA, definition, assembly, formats, native_codecs }
}
}
//#endregion 🧩️ArtifactItems7
