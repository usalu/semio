//! 🧬️ Transparent Flow direct-leaf dispatch and generic codec surfaces.
use super::{FlowHostSnapshot, FlowDiff};
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
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations, semio_framework_dsl_record_derive::DslEnum)]
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

//#region 🔤️Codecs



//#endregion 🔤️Codecs
