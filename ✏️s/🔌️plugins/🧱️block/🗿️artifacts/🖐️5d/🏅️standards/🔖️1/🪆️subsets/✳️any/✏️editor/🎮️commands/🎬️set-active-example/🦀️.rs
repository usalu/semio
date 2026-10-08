//! 🎬️ Block 5D play app command — `set-active-example`.

//#region 🔖️ExampleIds
pub const BLOCK5D_EXAMPLE_FOREST_LEFT: &str = "hexagonal-cut-concrete-forest-left";
pub const BLOCK5D_EXAMPLE_CAPSULE: &str = "nakagin-capsule";
//#endregion 🔖️ExampleIds

//#region 🔖️LoadDocument
/// 🗃️ The whole-document load an example switch or a JSON edit answers with. A `LoadDocument` effect is NOT an edit:
/// it carries no mutation rows, no diff and no history row, so re-selecting the open example leaves `canUndo` false.
pub fn load_document_effect(document: &Block5dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <Block5dSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr("block5d", crate::BLOCK_5D_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️LoadDocument

use crate::standards::v1::subsets::any::schema::mutations::Block5dMutation;
use crate::Block5dSnapshot;
use crate::editor::block5d::config::{Block5dConfig, Block5dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "setActiveExample")]
pub struct SetActiveExample {
    pub id: String,
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Block5dSnapshot>, _cfg: &ConfigView<'_, Block5dConfig>) -> Result<Emit<Block5dMutation, Block5dConfigMutation>, Fault> {
    let example = match payload.id.as_str() {
        BLOCK5D_EXAMPLE_FOREST_LEFT => crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(crate::standards::v1::subsets::any::io::text::snapshot::BLOCK5D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
        BLOCK5D_EXAMPLE_CAPSULE => crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(crate::standards::v1::subsets::any::io::text::snapshot::BLOCK5D_NAKAGIN_CAPSULE_EXAMPLE_TEXT).ok(),
        _ => None,
    };
    match example {
        Some(document) if &document != doc.snapshot => Ok(Emit::effect(load_document_effect(&document))),
        Some(_) => Ok(Emit::default()),
        None => Ok(Emit::default()),
    }
}
