//! 🧬️ Transparent Flow direct-leaf dispatch and generic codec surfaces.
use super::{FlowHostSnapshot, FlowDiff};
use semio_framework_value::retirement::{deferred, deferred_birth_bytes_for, sequence, sequence_birth_bytes, RetireOwned, RetirementCursor};
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
//#endregion 🧩️Leaves

//#region 🧬️Aggregate
/// 🔮️ First-party Flow mutation wire aggregate.
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[value(tag = "operation", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = FlowHostSnapshot, diff = FlowDiff, schema = "flow.host_snapshot", retire_cold = retire_flow_mutation)]
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
}

/// 🧊️ Cold-retires one flow host operation — the generated `Mutation::retire_cold`. A widget owns
/// fail-closed roots (`Dictionary`, `OrderedSet`, `OrderedMap`, `Tree`) that refuse a bare drop; every other payload is plain data.
pub fn retire_flow_mutation(mutation: FlowMutation) {
    match mutation {
        FlowMutation::AddWidget(add) => add.widget.retire_cold(),
        FlowMutation::ChangeWidget(change) => change.widget.retire_cold(),
        _ => {}
    }
}
//#endregion 🧬️Aggregate

#[path = "🔏️canonical/🦀️.rs"]
mod canonical;
pub use canonical::prepared_operation_wire_source;

//#region 🔤️Codecs
impl crate::os_spr::OpText for FlowMutation {
    fn parse_op(line: &str) -> Result<Self, crate::os_store::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
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

//#region 🧹️OwnedRetirement
macro_rules! owned_record {
    ($type:ty {$($field:ident),+})=>{impl RetireOwned for $type {
        fn retirement(self)->Box<dyn RetirementCursor> {let Self {$($field,)*..}=self;sequence(vec![$(deferred($field)),+])}
        fn retirement_birth_bytes(&self)->Option<usize> {sequence_birth_bytes(&[$(deferred_birth_bytes_for(&self.$field)),+])}
        fn controlled_retirement_supported()->bool {true}
    }};
}
owned_record!(crate::AddWidget {widget});
owned_record!(crate::RemoveWidget {id});
owned_record!(crate::MoveWidget {id});
owned_record!(crate::ChangeWidget {id,widget});
owned_record!(crate::AddSynapse {synapse});
owned_record!(crate::RemoveSynapse {id});
owned_record!(crate::MoveSynapse {id});
owned_record!(crate::ChangeSynapse {id,synapse});
owned_record!(crate::ChangeLayout {entries});
impl RetireOwned for crate::FlowMutation {
    fn retirement(self)->Box<dyn RetirementCursor> {match self {Self::AddWidget(value)=>value.retirement(),Self::RemoveWidget(value)=>value.retirement(),Self::MoveWidget(value)=>value.retirement(),Self::ChangeWidget(value)=>value.retirement(),Self::AddSynapse(value)=>value.retirement(),Self::RemoveSynapse(value)=>value.retirement(),Self::MoveSynapse(value)=>value.retirement(),Self::ChangeSynapse(value)=>value.retirement(),Self::ChangeLayout(value)=>value.retirement()}}
    fn retirement_birth_bytes(&self)->Option<usize> {match self {Self::AddWidget(value)=>value.retirement_birth_bytes(),Self::RemoveWidget(value)=>value.retirement_birth_bytes(),Self::MoveWidget(value)=>value.retirement_birth_bytes(),Self::ChangeWidget(value)=>value.retirement_birth_bytes(),Self::AddSynapse(value)=>value.retirement_birth_bytes(),Self::RemoveSynapse(value)=>value.retirement_birth_bytes(),Self::MoveSynapse(value)=>value.retirement_birth_bytes(),Self::ChangeSynapse(value)=>value.retirement_birth_bytes(),Self::ChangeLayout(value)=>value.retirement_birth_bytes()}}
    fn controlled_retirement_supported()->bool {true}
}
//#endregion 🧹️OwnedRetirement
