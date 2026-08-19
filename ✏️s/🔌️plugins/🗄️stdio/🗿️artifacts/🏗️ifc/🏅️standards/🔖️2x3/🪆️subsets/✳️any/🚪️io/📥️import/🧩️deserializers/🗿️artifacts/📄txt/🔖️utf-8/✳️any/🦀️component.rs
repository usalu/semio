//! 📥️ Deserialize `stdio.ifc.2x3` from stdio.txt.

use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;
use crate::artifacts::txt::TxtSnapshot;

pub async fn register() {}

pub async fn deserialize(from: &TxtSnapshot) -> Result<Ifc2x3Snapshot, store::TextError> {
    crate::artifacts::ifc::standards::v2x3::engine::decode_ifc2x3(from.to_body().as_bytes()).map_err(|e| store::TextError::new(format!("ifc2x3 parse: {e}"), dsl::TextSpan::at(1, 1)))
}

pub async fn deserialize_text(text: &str) -> Result<Ifc2x3Snapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
