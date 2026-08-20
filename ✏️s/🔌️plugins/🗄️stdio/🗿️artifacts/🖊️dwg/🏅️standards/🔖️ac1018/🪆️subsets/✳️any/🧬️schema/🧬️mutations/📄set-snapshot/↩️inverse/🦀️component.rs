use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::mutations::DwgMutation;
use crate::artifacts::dwg::standards::v_ac1018::subsets::any::schema::snapshot::DwgSnapshot;
use protocol::Mutation;

/// ↩️ Inverse of set-snapshot.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &DwgSnapshot, mutation: &DwgMutation) -> Vec<DwgMutation> {
    <DwgMutation as Mutation<DwgSnapshot>>::inverse(mutation, base)
}
