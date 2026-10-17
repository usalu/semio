//! 🏅️ Standard root — `standard() -> StandardDeclaration` (design.md §2), mounts subset `any`.
//! `mimes`/`extensions`: no real MIME/extension registration exists for `s.note.note` outside the
//! old `ArtifactDefinition`'s capability rows (`definition()`, kept — debt D1) — that channel only
//! ever claimed a codec id (`note.document:note`) and an extension (`note`), never a mime type.
//! `mimes: ["application/vnd.semio.note+json"]` is a documented synthesis (see the fan-out report's
//! `## openQuestions`, matching the sequence pilot's identical documented deviation);
//! `extensions: ["note"]` is the real, carried-over value.

use crate::standards::v1::subsets;
use semio_framework_plugin::app::declarations::{MediaDeclaration, StandardDeclaration};
use {semio_framework_artifact_reference::StandardId};

pub fn standard<PA>() -> StandardDeclaration<PA>
where
    PA: semio_framework_plugin::PluginApp
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::EditorApp<crate::editor::note::NotePlayApp>>>
        + From<semio_framework_plugin::VcsArtifactApp<semio_framework_plugin::ViewerApp<crate::viewer::note::NoteViewer>>>,
{
    StandardDeclaration { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.note+json"], extensions: &["note"] }, subsets: vec![subsets::any::subset()] }
}

//#region 🧩️ArtifactItems2
pub mod artifact_items_2 {
use semio_framework_artifact_reference::{Dialect, StandardId, SubsetId};

//#region 🔖️Dialect
/// 👁️✏️ C1's canonical surface id grammar (`<artifact_kind>@<standard>/<subset>#<role>`) — lives at
/// the ARTIFACT level, not under `editor`/`viewer`, so a viewer file can read it without ever
/// importing through the sibling `editor` module. `artifact_kind` matches this file's own
/// `definition()` `"s.note.schema.artifact"` capability row's descriptor (`"s.note.note"`, the same
/// schema id `NoteSnapshot`'s `#[artifact_schema(id = "s.note.note")]` already keys off);
/// `standard`/`subset` match this file's own `🏅️standards/🔖️1/🪆️subsets/✳️any` location — i.e. the
/// canonical surface ids are `s.note.note@1/*#editor` / `s.note.note@1/*#viewer`.
pub const NOTE_DIALECT: Dialect = Dialect { artifact_kind: "s.note.note", standard: StandardId("1"), subset: SubsetId::ANY };
//#endregion 🔖️Dialect
}
//#endregion 🧩️ArtifactItems2

//#region 🧩️ArtifactItems3
pub mod artifact_items_3 {
use crate::{NOTE_DIALECT};
use semio_framework_plugin::{ArtifactKindSpec, MediaClass, MediaForm, MediaType, OsMediaCapability};
pub use crate::standards::v1::subsets::any::schema;

//#region 🔖️ArtifactKind
/// 🗂️ This artifact's `ArtifactKindSpec` — stitched into the app manifest by
/// `crate::editor::note::create_note_app`'s `🔖️Manifest` region. `id` IS `NOTE_DIALECT
/// .artifact_kind`: the spec, the dialect every surface id is built from and the activation event
/// name ONE identity, exactly as `semio_s_artifact_gis_gismap::artifact_kind()` already does. The
/// former `"2d.note"` spelling was the only place this artifact answered to a second name, and a
/// hub refuses to open a document kind whose declared spec id differs from its app's dialect
/// (`validate_descriptor_open_target`, 🌎️hub/🗿️artifact-authority/🔏️trusted-catalog/🦀️.rs).
pub fn artifact_kind() -> ArtifactKindSpec {
    ArtifactKindSpec {
        id: NOTE_DIALECT.artifact_kind.into(),
        label: semio_framework_ui_locale::LocalizedLabel::native("Note", "Notiz"),
        source_format: "note.document".into(),
        component_kind: "note".into(),
        dimension: "2d".into(),
        media_capability: OsMediaCapability::MeshOnly,
        media_type: MediaType { class: MediaClass::TwoD, form: MediaForm::Document },
        schema: "note.document".into(),
        export_formats: vec![],
        import_formats: vec![],
        export_stdio_kinds: crate::standards::v1::subsets::any::io::export_stdio_kinds().iter().map(|kind| (*kind).to_owned()).collect(),
        import_stdio_kinds: crate::standards::v1::subsets::any::io::import_stdio_kinds().iter().map(|kind| (*kind).to_owned()).collect(),
    }
}
//#endregion 🔖️ArtifactKind
}
//#endregion 🧩️ArtifactItems3

//#region 🧩️ArtifactItems6
pub mod artifact_items_6 {
use crate::{NoteSnapshot};

//#region 🧬️ChildRestoreProjection
/// 🧬️ The loaded-parent child projection, read off the snapshot's own derived composition fields — the ONE
/// definition the editor and the viewer both declare. The `ArtifactEditor`/`ArtifactViewer` trait default
/// refuses it (`… did not declare a loaded-parent child projection`), and since PX1 the live envelope load
/// asks for it before the decoded document may replace the store, so an undeclared app fails every live load.
pub fn note_child_restore_projection(snapshot: &NoteSnapshot) -> Result<semio_framework_composition_admission::ChildRestoreProjection<'_>, semio_framework_plugin::Fault> {
    semio_framework_composition_admission::ChildRestoreProjection::from_snapshot(snapshot).map_err(|error| semio_framework_plugin::Fault::new(semio_framework_plugin::FaultOrigin::App, semio_framework_plugin::FaultCode::new("note.child-projection"), error.to_string()))
}
//#endregion 🧬️ChildRestoreProjection
}
//#endregion 🧩️ArtifactItems6
