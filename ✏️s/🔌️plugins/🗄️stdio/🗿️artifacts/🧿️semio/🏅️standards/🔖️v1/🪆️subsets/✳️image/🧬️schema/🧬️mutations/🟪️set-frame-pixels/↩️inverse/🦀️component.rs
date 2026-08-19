use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-frame-pixels.
pub async fn inverse(base: &SemioImageSnapshot, index: usize, rgba8: Vec<u8>) -> Vec<SemioImageMutation> {
    <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&SemioImageMutation::SetFramePixels { index, rgba8 }, base)
}
