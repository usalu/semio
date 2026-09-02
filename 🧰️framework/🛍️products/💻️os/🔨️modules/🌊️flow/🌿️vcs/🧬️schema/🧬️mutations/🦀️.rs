//! 🧬️ Transparent Flow direct-leaf dispatch and generic codec surfaces.
use super::{FlowFixture, FlowDiff};
#[cfg(test)]
use serde::{Deserialize, Serialize};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🧩️Leaves
#[path = "➕️add-widget/🦀️.rs"] mod add_widget;
pub use add_widget::AddWidget;
#[path = "🗑️remove-widget/🦀️.rs"] mod remove_widget;
pub use remove_widget::RemoveWidget;
#[path = "↔️move-widget/🦀️.rs"] mod move_widget;
pub use move_widget::MoveWidget;
#[path = "🩹change-widget/🦀️.rs"] mod change_widget;
pub use change_widget::ChangeWidget;
#[path = "🔗️add-synapse/🦀️.rs"] mod add_synapse;
pub use add_synapse::AddSynapse;
#[path = "✂️remove-synapse/🦀️.rs"] mod remove_synapse;
pub use remove_synapse::RemoveSynapse;
#[path = "🔀️move-synapse/🦀️.rs"] mod move_synapse;
pub use move_synapse::MoveSynapse;
#[path = "🔄change-synapse/🦀️.rs"] mod change_synapse;
pub use change_synapse::ChangeSynapse;
#[path = "📐️change-layout/🦀️.rs"] mod change_layout;
pub use change_layout::ChangeLayout;
#[path = "♻️replace-flow-fixture/🦀️.rs"] mod replace_flow_fixture;
pub use replace_flow_fixture::ReplaceFlowFixture;
//#endregion 🧩️Leaves

//#region 🧬️Aggregate
/// 🔮️ `serde` is TEST-ONLY (RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS, 26/09/01,
/// tenth-seam pass): `AddWidget`/`ChangeWidget`/`ReplaceFlowFixture` variants lost their own
/// unconditional `Serialize`/`Deserialize` this pass — see `📓️orderedmap-tenth-seam.md`.
/// `dsl::Mutations`/`ToValue`/`FromValue` are unaffected (already the real wire encoding, seam 1).
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, crate::os_dsl::DslOps)]
#[cfg_attr(test, derive(Serialize, Deserialize))]
#[cfg_attr(test, serde(tag = "operation", rename_all = "camelCase", deny_unknown_fields))]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = FlowFixture, diff = FlowDiff, schema = "flow.fixture")]
pub enum FlowMutation {
    AddWidget(AddWidget),
    RemoveWidget(RemoveWidget),
    MoveWidget(MoveWidget),
    ChangeWidget(ChangeWidget),
    AddSynapse(AddSynapse),
    RemoveSynapse(RemoveSynapse),
    MoveSynapse(MoveSynapse),
    ChangeSynapse(ChangeSynapse),
    ChangeLayout(ChangeLayout),
    ReplaceFlowFixture(ReplaceFlowFixture),
}
//#endregion 🧬️Aggregate

//#region 🔤️Codecs
impl crate::os_spr::OpText for FlowMutation {
    fn parse_op(line: &str) -> Result<Self, crate::os_store::TextError> {
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = crate::os_dsl::parse(line, &spec_fn(), &crate::os_dsl::ParseOptions { limits: crate::os_dsl::Limits::default(), mode: crate::os_dsl::SourceMode::Inline })?;
                return <Self as crate::os_dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(crate::os_dsl::__rt::field_error(format!("unknown operation line '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as crate::os_dsl::DslVariants>::to_named_record(self);
        let variants = <Self as crate::os_dsl::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        crate::os_dsl::print(&record, &spec_fn(), crate::os_dsl::JoinMode::Inline)
    }
}

impl crate::os_spr::OpBinary for FlowMutation {
    fn encode_op(&self) -> Result<Vec<u8>, crate::os_spr::ProtocolError> {
        crate::os_dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, crate::os_spr::ProtocolError> {
        crate::os_dsl::variants_binary::decode_op(bytes)
    }
}
//#endregion 🔤️Codecs
