//! 🔺️ S Home launcher artifact — operation diff laws (constitutional: diff).

use crate::artifacts::home::op::SHomeOperation;
use crate::artifacts::home::SHomeDocument;

//#region 🔖️OperationDiff
impl protocol::OperationDiff<SHomeDocument> for SHomeOperation {
    fn apply(&self, projection: &SHomeDocument) -> SHomeDocument {
        match self {
            SHomeOperation::NoOperation => projection.clone(),
            SHomeOperation::SetCatalogGeneration { value } => SHomeDocument { catalog_generation: *value, ..projection.clone() },
        }
    }

    fn absorb(&mut self, other: Self) {
        if !matches!(other, SHomeOperation::NoOperation) {
            *self = other;
        }
    }
}
//#endregion 🔖️OperationDiff
