//! 🪆️ Genuine Drawing-owned child registrations retain the editor dialect and canonical admission producer.
use semio_framework_plugin::app::child_editor::{registration::ChildEditorRegistration,source::ChildEditorCapability};
use semio_s_artifact_draw_drawing_model::{snapshot::DrawingSnapshot,validation::DRAWING_ADMITTER,vector_source::DRAWING_VECTOR_SOURCE,DRAWING_DIALECT};
use std::sync::LazyLock;
static EDITOR_ID:LazyLock<String>=LazyLock::new(||semio_framework::surface_app_id(&DRAWING_DIALECT.into(),semio_framework::AppRole::Editor));
fn registration(command_id:&'static str)->ChildEditorRegistration<DrawingSnapshot>{ChildEditorRegistration::new(ChildEditorCapability{artifact_kind:DRAWING_DIALECT.artifact_kind,standard:DRAWING_DIALECT.standard.0,subset:DRAWING_DIALECT.subset.0,editor_id:EDITOR_ID.as_str(),command_id},&DRAWING_ADMITTER,&DRAWING_VECTOR_SOURCE)}
pub static DRAWING_CHILD_EDIT_PATH:LazyLock<ChildEditorRegistration<DrawingSnapshot>>=LazyLock::new(||registration("editPath"));
pub static DRAWING_CHILD_EDIT_FILL:LazyLock<ChildEditorRegistration<DrawingSnapshot>>=LazyLock::new(||registration("editFill"));
pub static DRAWING_CHILD_PATCH_LAYER:LazyLock<ChildEditorRegistration<DrawingSnapshot>>=LazyLock::new(||registration("patchLayer"));
#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;

/// 📖️ Genuine vector reads use an explicit read capability and never fabricate an edit command.
pub static DRAWING_CHILD_VECTOR_READ:semio_framework_plugin::app::vector_read::ChildVectorReadRegistration<DrawingSnapshot>=semio_framework_plugin::app::vector_read::ChildVectorReadRegistration::new(semio_framework_plugin::app::vector_read::ChildVectorReadCapability{artifact_kind:DRAWING_DIALECT.artifact_kind,standard:DRAWING_DIALECT.standard.0,subset:DRAWING_DIALECT.subset.0},&DRAWING_ADMITTER,&DRAWING_VECTOR_SOURCE);

pub static DRAWING_CHILD_SET_ARTBOARD:LazyLock<ChildEditorRegistration<DrawingSnapshot>>=LazyLock::new(||registration("setArtboard"));
