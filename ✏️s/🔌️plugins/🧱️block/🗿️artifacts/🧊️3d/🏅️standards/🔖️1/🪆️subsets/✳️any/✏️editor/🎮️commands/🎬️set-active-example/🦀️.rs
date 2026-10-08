//! 🎬️ Block 3D play app command — `set-active-example`.

//#region 🔖️ExampleIds
pub const BLOCK3D_EXAMPLE_CAPSULE: &str = "nakagin-capsule";
pub const BLOCK3D_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";
//#endregion 🔖️ExampleIds

//#region 🔖️LoadDocument
/// 🗃️ The whole-document load an example switch or a JSON edit answers with. A `LoadDocument` effect is NOT an edit:
/// it carries no mutation rows, no diff and no history row, so re-selecting the open example leaves `canUndo` false.
pub fn load_document_effect(document: &Block3dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <Block3dSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr("block3d", crate::BLOCK_3D_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️LoadDocument

use crate::standards::v1::subsets::any::schema::mutations::Block3dMutation;
use crate::Block3dSnapshot;
use crate::editor::block3d::config::{Block3dConfig, Block3dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "setActiveExample")]
pub struct SetActiveExample {
    pub id: String,
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Block3dSnapshot>, _cfg: &ConfigView<'_, Block3dConfig>) -> Result<Emit<Block3dMutation, Block3dConfigMutation>, Fault> {
    let example = match payload.id.as_str() {
        BLOCK3D_EXAMPLE_CAPSULE => crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(crate::standards::v1::subsets::any::io::text::snapshot::BLOCK3D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).ok(),
        BLOCK3D_EXAMPLE_FOREST_LEFT => crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(crate::standards::v1::subsets::any::io::text::snapshot::BLOCK3D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
        _ => None,
    };
    match example {
        Some(document) if &document != doc.snapshot => Ok(Emit::effect(load_document_effect(&document))),
        Some(_) => Ok(Emit::default()),
        None => Ok(Emit::default()),
    }
}
