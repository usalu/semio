use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::{SemioColorspace, SemioImageSnapshot};
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-colorspace.
pub async fn inverse(base: &SemioImageSnapshot, colorspace: SemioColorspace) -> Vec<SemioImageMutation> {
    <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&SemioImageMutation::SetColorspace { colorspace }, base)
}
