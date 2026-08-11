use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// ↩️ Inverse of move-frame — swaps `from`/`to` (structural, base-content-independent).
pub fn inverse(base: &SemioImageSnapshot, from: usize, to: usize) -> Vec<SemioImageMutation> {
    <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&SemioImageMutation::MoveFrame { from, to }, base)
}
