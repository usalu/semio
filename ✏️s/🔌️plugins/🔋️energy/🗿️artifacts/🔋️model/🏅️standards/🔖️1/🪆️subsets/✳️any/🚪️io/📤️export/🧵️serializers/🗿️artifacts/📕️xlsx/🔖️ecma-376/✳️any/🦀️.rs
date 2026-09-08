//! model -> xlsx
use crate::EnergyModelSnapshot;

pub async fn register() {}

pub fn serialize_bytes(snapshot: &EnergyModelSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<EnergyModelSnapshot as store::ArtifactDsl>::print_dsl(snapshot).into_bytes())
}
