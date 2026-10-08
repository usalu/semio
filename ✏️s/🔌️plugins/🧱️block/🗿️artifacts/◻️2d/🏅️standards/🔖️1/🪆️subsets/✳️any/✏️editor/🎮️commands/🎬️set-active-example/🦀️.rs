//! 🎬️ Block 2D play app command — `set-active-example`.

//#region 🔖️ExampleIds
pub const BLOCK2D_EXAMPLE_LEFT: &str = "hexagonal-cut-concrete-forest-left";
pub const BLOCK2D_EXAMPLE_RIGHT: &str = "hexagonal-cut-concrete-forest-right";
//#endregion 🔖️ExampleIds

//#region 🔖️LoadDocument
/// 🗃️ The whole-document load an example switch or a JSON edit answers with. A `LoadDocument` effect is NOT an edit:
/// it carries no mutation rows, no diff and no history row, so re-selecting the open example leaves `canUndo` false.
pub fn load_document_effect(document: &Block2dSnapshot) -> semio_framework::kernel::Effect {
    let pack = <Block2dSnapshot as store::ArtifactPack>::encode_pack(document);
    let spr = ::semio_framework_async::poll::resolve_ready(store::empty_document_spr("block2d", crate::BLOCK_2D_SCHEMA));
    semio_framework::kernel::Effect::LoadDocument { pack, spr }
}
//#endregion 🔖️LoadDocument

use crate::standards::v1::subsets::any::schema::mutations::Block2dMutation;
use crate::Block2dSnapshot;
use crate::editor::block2d::config::{Block2dConfig, Block2dConfigMutation};
use semio_framework_plugin::{ArtifactView, ConfigView, Emit, Fault};
use semio_framework_value_derive::{FromValue, ToValue};

#[derive(Clone, Debug, PartialEq, ToValue, FromValue, semio_framework_dsl_record_derive::DslRecord)]
#[dsl(keyword = "setActiveExample")]
pub struct SetActiveExample {
    pub id: String,
}

pub fn handle(payload: &SetActiveExample, doc: &ArtifactView<'_, Block2dSnapshot>, _cfg: &ConfigView<'_, Block2dConfig>) -> Result<Emit<Block2dMutation, Block2dConfigMutation>, Fault> {
    let example = match payload.id.as_str() {
        BLOCK2D_EXAMPLE_LEFT => crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(crate::standards::v1::subsets::any::io::text::snapshot::BLOCK2D_CONCRETE_FOREST_LEFT_EXAMPLE_TEXT).ok(),
        BLOCK2D_EXAMPLE_RIGHT => crate::standards::v1::subsets::any::io::text::snapshot::parse_dsl(crate::standards::v1::subsets::any::io::text::snapshot::BLOCK2D_CONCRETE_FOREST_RIGHT_EXAMPLE_TEXT).ok(),
        _ => None,
    };
    match example {
        Some(document) if &document != doc.snapshot => Ok(Emit::effect(load_document_effect(&document))),
        Some(_) => Ok(Emit::default()),
        None => Ok(Emit::default()),
    }
}
