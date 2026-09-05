//! 🧬️ Transparent space-history mutation aggregate.
use super::super::{SpaceHistoryDiff, SpaceHistorySnapshot};
use semio_framework_value_derive::{FromValue, ToValue};

//#region 🔖️Leaves
#[path = "📌️commit-space-checkpoint/🦀️.rs"]
pub mod commit_space_checkpoint;
#[path = "🌿️create-space-alternative/🦀️.rs"]
pub mod create_space_alternative;
#[path = "🧹️remove-space-alternative/🦀️.rs"]
pub mod remove_space_alternative;
#[path = "🗑️remove-space-checkpoint/🦀️.rs"]
pub mod remove_space_checkpoint;
#[path = "🎯️restore-active-space-alternative/🦀️.rs"]
pub mod restore_active_space_alternative;
#[path = "🔀️switch-space-alternative/🦀️.rs"]
pub mod switch_space_alternative;

pub use commit_space_checkpoint::CommitSpaceCheckpoint;
pub use create_space_alternative::CreateSpaceAlternative;
pub use remove_space_alternative::RemoveSpaceAlternative;
pub use remove_space_checkpoint::RemoveSpaceCheckpoint;
pub use restore_active_space_alternative::RestoreActiveSpaceAlternative;
pub use switch_space_alternative::SwitchSpaceAlternative;
//#endregion 🔖️Leaves

//#region 🔖️Aggregate
#[derive(Clone, Debug, PartialEq, ToValue, FromValue, dsl::Mutations)]
#[value(tag = "operation", content = "payload", rename_all = "camelCase", deny_unknown_fields)]
#[mutations(snapshot = SpaceHistorySnapshot, diff = SpaceHistoryDiff, schema = "os.space.history")]
pub enum SpaceHistoryMutation {
    CommitSpaceCheckpoint(CommitSpaceCheckpoint),
    CreateSpaceAlternative(CreateSpaceAlternative),
    SwitchSpaceAlternative(SwitchSpaceAlternative),
    RemoveSpaceCheckpoint(RemoveSpaceCheckpoint),
    RemoveSpaceAlternative(RemoveSpaceAlternative),
    RestoreActiveSpaceAlternative(RestoreActiveSpaceAlternative),
}
//#endregion 🔖️Aggregate

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::os_spr::SemanticMutation;

    #[test]
    fn aggregate_roster_is_structural_and_exact() {
        assert_eq!(
            SpaceHistoryMutation::kinds().iter().map(|value| value.kind).collect::<Vec<_>>(),
            ["commit-space-checkpoint", "create-space-alternative", "switch-space-alternative", "remove-space-checkpoint", "remove-space-alternative", "restore-active-space-alternative"]
        );
    }
}
//#endregion 🧪️Tests
