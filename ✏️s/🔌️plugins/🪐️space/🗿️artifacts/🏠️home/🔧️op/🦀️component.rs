//! ⚡️ S Home launcher artifact — operation enum + laws (constitutional: op).

use crate::artifacts::home::SHomeDocument;
use serde::{Deserialize, Serialize};

//#region 🔖️Types
/// @emoji 🔢️ The Home launcher's only document operation: pins the catalog-generation counter that forces a
/// re-materialize of the studio list after a create/import/delete side-effect on the catalog port.
/// It is its own {@link protocol::OperationDiff} (idempotent set), so forward/backward are symmetric.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, dsl::DslOps)]
#[serde(tag = "operation", rename_all = "camelCase")]
pub enum SHomeOperation {
    /// 🫙️ The identity operation — an `OperationDiff` needs `Default`; never emitted by `handle`.
    #[default]
    NoOperation,
    SetCatalogGeneration {
        value: u64,
    },
}

impl protocol::Operation<SHomeDocument> for SHomeOperation {
    type Diff = SHomeOperation;

    fn diff(&self, _projection: &SHomeDocument) -> SHomeOperation {
        self.clone()
    }

    fn backwards(&self, projection: &SHomeDocument) -> Vec<Self> {
        vec![SHomeOperation::SetCatalogGeneration { value: projection.catalog_generation }]
    }
}
//#endregion 🔖️Types

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn home_op_text_round_trips_every_variant() {
        store::test_support::assert_op_line_round_trip(&SHomeOperation::NoOperation);
        store::test_support::assert_op_line_round_trip(&SHomeOperation::SetCatalogGeneration { value: 7 });
    }
}
//#endregion 🧪️Tests
