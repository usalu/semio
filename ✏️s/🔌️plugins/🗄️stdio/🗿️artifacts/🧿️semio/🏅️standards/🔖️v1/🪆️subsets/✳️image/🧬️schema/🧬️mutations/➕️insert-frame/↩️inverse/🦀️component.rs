use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioImageFrame, SemioImageSnapshot};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// ↩️ Inverse of insert-frame — a `RemoveFrame` at the same index.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn inverse(base: &SemioImageSnapshot, index: usize, frame: SemioImageFrame) -> Vec<SemioImageMutation> {
    <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&SemioImageMutation::InsertFrame { index, frame }, base)
}
