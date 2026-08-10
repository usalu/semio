//! 🧬️ Puzzle5d mutation apply delegate — bridges play Value document onto typed snapshot apply.
use crate::artifacts::puzzle5d::Puzzle5dPlaySnapshot;
use crate::artifacts::puzzle5d::Puzzle5dSnapshot;
use crate::artifacts::puzzle5d::mutations::Puzzle5dMutation;

pub fn apply(projection: &mut Puzzle5dPlaySnapshot, mutation: &Puzzle5dMutation) {
    let mut snapshot: Puzzle5dSnapshot = serde_json::from_value(projection.0.clone()).unwrap_or_default();
    crate::artifacts::puzzle5d::mutations::apply_puzzle5d_mutation(&mut snapshot, mutation);
    projection.0 = serde_json::to_value(&snapshot).unwrap_or_else(|_| projection.0.clone());
}
