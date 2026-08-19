use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-bit-depth.
pub async fn inverse(base: &SemioImageSnapshot, bit_depth: u8) -> Vec<SemioImageMutation> {
    <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&SemioImageMutation::SetBitDepth { bit_depth }, base)
}
