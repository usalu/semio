//! 🗂️ Block plugin kit catalog declaration, independent of each removable dimensional artifact.
use semio_framework::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
/// 🪪️ The catalog identity shared by every Block producer.
pub const ARTIFACT_ID:&str="kit.catalog";
/// 🗂️ The plugin-owned catalog descriptor; producer dimensions do not change its mesh authority.
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: ARTIFACT_ID.into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Kit Catalog", "Bausatzkatalog"),
        source_format: ARTIFACT_ID.into(),
        component_kind: "kit-catalog".into(),
        dimension: "3d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::Kit, form: MediaForm::Type },
        schema: ARTIFACT_ID.into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: vec![],
        import_stdio_kinds: vec![],
    }
}
