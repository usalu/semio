use crate::artifacts::curate::CurateSnapshot;
use crate::artifacts::curate::diff::CurateDiff;
use crate::artifacts::curate::mutations::SourcingMutation;

pub fn diff_for(mutation: &SourcingMutation, base: &CurateSnapshot) -> CurateDiff {
    <SourcingMutation as protocol::Mutation<CurateSnapshot>>::diff(mutation, base)
}
