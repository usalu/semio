use crate::schema::mutations::{apply_md_mutation, MdMutation};
use crate::MdSnapshot;

/// ▶️ Applies a set-snapshot mutation.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn apply(projection: &mut MdSnapshot, mutation: &MdMutation) {
    apply_md_mutation(projection, mutation);
}
