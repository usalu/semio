use crate::artifacts::semio::standards::v1::subsets::image::schema::snapshot::SemioImageSnapshot;
use crate::artifacts::semio::standards::v1::subsets::image::schema::mutations::SemioImageMutation;
use protocol::Mutation;

/// ↩️ Inverse of set-icc.
pub fn inverse(base: &SemioImageSnapshot, icc: Option<Vec<u8>>) -> Vec<SemioImageMutation> {
    <SemioImageMutation as Mutation<SemioImageSnapshot>>::inverse(&SemioImageMutation::SetIcc { icc }, base)
}
