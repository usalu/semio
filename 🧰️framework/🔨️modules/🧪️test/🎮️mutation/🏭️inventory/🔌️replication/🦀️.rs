use super::*;
use protocol::{MutationLeafDescriptor, MutationOutcomeClass};
/// 🧬️ Projects real static replication descriptors into an immutable compile-time inventory.
pub const fn replication_inventory_leaves<const N: usize>(descriptors: &[MutationLeafDescriptor]) -> [MutationInventoryLeaf<'static>; N] {
    assert!(descriptors.len() == N);
    let mut output = [MutationInventoryLeaf { owner: "", id: "", variant: "", outcomes: [MutationInventoryOutcome::Applied; 5], outcome_count: 0 }; N];
    let mut index = 0;
    while index < N {
        let row = descriptors[index];
        assert!(!row.outcome_classes.is_empty() && row.outcome_classes.len() <= 5);
        let mut outcomes = [MutationInventoryOutcome::Applied; 5];
        let mut outcome = 0;
        while outcome < row.outcome_classes.len() {
            outcomes[outcome] = match row.outcome_classes[outcome] {
                MutationOutcomeClass::Applied => MutationInventoryOutcome::Applied,
                MutationOutcomeClass::NoOp => MutationInventoryOutcome::NoOp,
                MutationOutcomeClass::Empty => MutationInventoryOutcome::Empty,
                MutationOutcomeClass::Disjoint => MutationInventoryOutcome::Disjoint,
                MutationOutcomeClass::Rejected => MutationInventoryOutcome::Rejected,
            };
            outcome += 1;
        }
        output[index] = MutationInventoryLeaf { owner: row.owner, id: row.semantic_kind, variant: row.aggregate_variant, outcomes, outcome_count: row.outcome_classes.len() };
        index += 1;
    }
    output
}
