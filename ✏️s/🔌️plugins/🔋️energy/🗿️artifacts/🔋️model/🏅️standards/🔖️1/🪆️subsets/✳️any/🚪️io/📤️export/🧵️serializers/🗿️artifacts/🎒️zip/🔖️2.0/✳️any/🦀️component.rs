//! model -> zip
use crate::artifacts::model::EnergyModelSnapshot;

pub fn register() {}

pub fn serialize_bytes(snapshot: &EnergyModelSnapshot) -> Result<Vec<u8>, store::TextError> {
    Ok(<EnergyModelSnapshot as store::DocumentDsl>::print_dsl(snapshot).into_bytes())
}
