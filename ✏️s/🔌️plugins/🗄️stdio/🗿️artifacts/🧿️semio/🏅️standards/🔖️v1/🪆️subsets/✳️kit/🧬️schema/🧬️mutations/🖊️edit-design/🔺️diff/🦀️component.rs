//! 🔺️ `edit-design` — sparse diff construction, built directly from `(payload, base)`.

use super::mutation::EditDesign;
use crate::artifacts::semio::standards::v1::subsets::kit::schema::diff::{SemioKitDesignList, SemioKitDiff};
use crate::artifacts::semio::standards::v1::subsets::kit::schema::snapshot::SemioKitSnapshot;

//#region 🔖️Diff
pub async fn diff(payload: &EditDesign, base: &SemioKitSnapshot) -> protocol::MutationOutcome<SemioKitDiff> {
    let Some(existing) = base.designs.iter().find(|d| d.id == payload.id) else {
        return protocol::MutationOutcome::error(
            "mutation.target-missing",
            format!("Design \"{}\" does not exist.", payload.id),
            [payload.id.clone()],
        ).await;
    };
    if existing.pieces == payload.pieces && existing.connections == payload.connections {
        return protocol::MutationOutcome::empty().await.warn("mutation.no-op", format!("Design \"{}\" already has that content.", payload.id)).await;
    }
    let mut designs = base.designs.clone();
    if let Some(d) = designs.iter_mut().find(|d| d.id == payload.id) {
        d.pieces = payload.pieces.clone();
        d.connections = payload.connections.clone();
    }
    protocol::MutationOutcome::new(SemioKitDiff { designs: Some(SemioKitDesignList { values: designs }), ..Default::default() }).await
}
//#endregion 🔖️Diff
