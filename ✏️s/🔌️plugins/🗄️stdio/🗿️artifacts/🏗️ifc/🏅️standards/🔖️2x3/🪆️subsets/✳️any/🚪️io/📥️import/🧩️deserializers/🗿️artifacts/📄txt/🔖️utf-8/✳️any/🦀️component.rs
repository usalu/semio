//! 📥️ Deserialize `stdio.ifc.2x3` from stdio.txt.

use crate::artifacts::txt::TxtSnapshot;
use crate::artifacts::ifc::standards::v2x3::subsets::any::schema::snapshot::Ifc2x3Snapshot;

pub fn register() {}

pub fn deserialize(from: &TxtSnapshot) -> Result<Ifc2x3Snapshot, store::TextError> {
    crate::artifacts::ifc::standards::v2x3::engine::decode_ifc2x3(from.to_body().trim().as_bytes())
        .map_err(|e| store::TextError::new(format!("ifc2x3 parse: {e}"), dsl::TextSpan::at(1, 1)))
}

pub fn deserialize_text(text: &str) -> Result<Ifc2x3Snapshot, store::TextError> {
    deserialize(&<TxtSnapshot as store::ArtifactDsl>::parse_dsl(text)?)
}
