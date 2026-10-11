//! 🧬️ Procedural module document mutations and their operation codecs.

use super::*;

#[path = "📦️set-payload/🦀️.rs"]
mod set_payload;
pub use set_payload::SetPayload;

#[derive(Clone, Debug, PartialEq, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::ToValue, semio_framework_value::FromValue, dsl::Mutations, semio_framework_value::CanonicalJsonTree, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[canonical_json(owner = semio_framework_pack_json)]
#[mutations(snapshot = ModuleRenderPayload, diff = ModulePayloadDiff, schema = "playbook.module.procedural.payload")]
#[value(tag = "mutation", rename_all = "camelCase")]
pub enum ModulePayloadMutation {
    #[dsl(key = "set-payload")]
    SetPayload(SetPayload),
}

//#region 🔖️OpCodec




//#endregion 🔖️OpCodec
