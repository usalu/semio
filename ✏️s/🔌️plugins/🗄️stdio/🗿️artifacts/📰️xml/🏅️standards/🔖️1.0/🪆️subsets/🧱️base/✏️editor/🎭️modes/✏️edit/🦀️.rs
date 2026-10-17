//! ✏️ Xml editor — the `edit` mode: a single full-pane Tree window over the whole `XmlDocument`.

use crate::editor::xml_any::modes::edit::windows::main;
use semio_framework_ui_locale::LocalizedLabel;
use semio_framework_plugin::ModeDefinition;
use semio_framework_plugin::WindowLayout;
use semio_framework_plugin::WindowLayoutRoot;
use semio_framework_plugin::WindowLayoutStackNode;
use semio_framework_plugin::WindowLayoutWindowNode;

pub const XML_EDIT_MODE_ID: &str = "edit";

//#region 🔖️Definition
/// 🧱️ Stitched into the editor manifest by `crate::editor::xml_any::create_xml_editor`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn definition() -> ModeDefinition {
    ModeDefinition { id: XML_EDIT_MODE_ID.into(), label: LocalizedLabel::native("Edit", "Bearbeiten"), icon_id: "pencil".into(), tools: Vec::new(), layout_id: None, commands: Vec::new() }
}

/// 🪟️ Single full-pane Tree window — one document tree, no quadrant layout to allocate.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn layout() -> WindowLayout {
    WindowLayout {
        root: WindowLayoutRoot::Stack(WindowLayoutStackNode {
            kind: "stack".into(),
            size: None,
            active_window_kind_id: None,
            children: vec![WindowLayoutWindowNode { kind: "window".into(), window_kind_id: main::WINDOW_KIND_ID.into(), title: Some("Tree".into()), instance_id: None, template_id: None, corner: None }],
        }),
    }
}
//#endregion 🔖️Definition

//#region 🧩️ArtifactItems5
pub mod artifact_items_5 {
use crate::{STDIO_XML_DOCUMENT_SCHEMA};
pub use crate::schema::mutations::XmlMutation;
pub use crate::schema::snapshot::XmlSnapshot;

pub fn native_codec() -> store::os_store::serialization::ArtifactCodec {
    let mut codec = store::artifact_codec::<XmlSnapshot, XmlMutation>(STDIO_XML_DOCUMENT_SCHEMA);
    codec.extension = "xml";
    codec.pack_schema_hash = semio_framework_hash::Sha256::digest(include_bytes!("../../../🚪️io/💾️binary/📸️snapshot/📡️.protocol.semio"));
    codec
}
}
//#endregion 🧩️ArtifactItems5

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
use crate::{artifact_kind};
use crate::{native_codec};

pub fn native_codecs() -> Vec<semio_s_artifact_stdio_contract::NativeCodecFactory> {
    vec![semio_s_artifact_stdio_contract::NativeCodecFactory { id: "stdio.native.xml.v1", artifact: "xml", kind: artifact_kind, codec: native_codec }]
}
}
//#endregion 🧩️ArtifactItems6

//#region 🧩️ArtifactItems7
pub mod artifact_items_7 {
use crate::{native_codecs};
use crate::{ARTIFACT_DEFINITION_SCHEMA};

pub fn definition() -> Result<semio_framework_plugin::ArtifactDefinition, semio_framework_plugin::PluginAssemblyError> {
    let factories = native_codecs();
    let executables = semio_s_artifact_stdio_contract::native_codec_executables(ARTIFACT_DEFINITION_SCHEMA, &factories)?;
    semio_s_artifact_stdio_contract::definition_from_schema_with_executables(ARTIFACT_DEFINITION_SCHEMA, executables)
}
}
//#endregion 🧩️ArtifactItems7
