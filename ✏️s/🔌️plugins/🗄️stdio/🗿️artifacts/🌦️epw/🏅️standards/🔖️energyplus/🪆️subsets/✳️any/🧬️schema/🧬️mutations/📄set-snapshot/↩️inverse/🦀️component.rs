use crate::artifacts::epw::standards::energyplus::subsets::any::schema::mutations::EpwMutation;
use crate::artifacts::epw::standards::energyplus::subsets::any::schema::snapshot::EpwSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &EpwSnapshot, mutation: &EpwMutation) -> Vec<EpwMutation> {
    <EpwMutation as Mutation<EpwSnapshot>>::inverse(mutation, base)
}
