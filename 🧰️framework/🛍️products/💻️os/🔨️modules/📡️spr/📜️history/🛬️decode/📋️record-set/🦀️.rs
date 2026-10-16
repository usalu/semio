//! 📋️ Original retained History record decoders and receiving owners.
pub use semio_framework_os_history_schema::*;
pub use semio_framework_os_history_codec::*;
pub use semio_framework_os_history_identity as identity;
#[path="../📋️records/🦀️.rs"]
mod retained_history_records;
pub use retained_history_records::{RetainedHistoryRecords,RetainedHistoryRecordsStep};
#[path="../📋️records/📦️paged/🦀️.rs"]
mod retained_history_paged_records;
pub use retained_history_paged_records::RetainedHistoryPagedRecords;
#[path="../🧾️operation/🦀️.rs"]
mod retained_history_operation;
pub use retained_history_operation::{RetainedHistoryOpDecode,RetainedHistoryOpDecodeStep};
#[path="../🪪️identity/🦀️.rs"]
mod retained_history_id;
pub use retained_history_id::{RetainedHistoryIdDecode,RetainedHistoryIdDecodeStep};
#[path="../📨️message/🦀️.rs"]
mod retained_history_message;
pub use retained_history_message::{RetainedHistoryMessageDecode,RetainedHistoryMessageDecodeStep,RetainedHistoryMessagePartial};
#[path="../🔀️origin/🦀️.rs"]
mod retained_history_origin;
pub use retained_history_origin::{RetainedHistoryOriginProjection,RetainedHistoryOriginPartial,RetainedHistoryOriginJsonDecode,RetainedHistoryOriginJsonStep};
#[path="../🧾️metadata/🦀️.rs"]
mod retained_history_metadata;
pub use retained_history_metadata::{RetainedHistoryOpMetaDecode,RetainedHistoryOpMetaStep,RetainedHistoryOpMetaPartial,RetainedHistoryOpMetaFault};
#[path="../🔀️transition/🦀️.rs"]
mod retained_history_transition;
pub use retained_history_transition::{RetainedHistoryTransitionDecode,RetainedHistoryTransitionStep,RetainedHistoryTransitionPartial};
#[path="../🧵️edit/🦀️.rs"]
mod retained_history_edit;
pub use retained_history_edit::{RetainedHistoryEditDecode,RetainedHistoryEditStep,RetainedHistoryEditPartial};
#[path="../📇️identity-records/🦀️.rs"]
mod retained_history_identity_records;
pub use retained_history_identity_records::{RetainedHistoryIdentityRecordDecode,RetainedHistoryIdentityRecordKind,RetainedHistoryIdentityRecord,RetainedHistoryIdentityRecordPartial,RetainedHistoryIdentityRecordStep};
#[path="../⚔️conflict/🦀️.rs"]
mod retained_history_conflict;
pub use retained_history_conflict::{RetainedHistoryConflictDecode,RetainedHistoryConflictPartial,RetainedHistoryConflictStep};
#[path="../🔀️transition/🛂️payload/🦀️.rs"]
mod retained_history_transition_validation;
pub use retained_history_transition_validation::{RetainedHistoryTransitionPayloadValidation,RetainedHistoryTransitionPayloadValidationReceipt,RetainedHistoryTransitionPayloadValidationStep};

/// 🪝️ Test hook of the companion crate `semio-framework-os-history-record-decode-tests`: the private decoder modules, public only behind the `testkit` feature.
#[cfg(feature = "testkit")]
#[doc(hidden)]
pub mod testkit {
    pub mod retained_history_records {
        pub use crate::retained_history_records::*;
    }
    pub mod retained_history_paged_records {
        pub use crate::retained_history_paged_records::*;
    }
    pub mod retained_history_operation {
        pub use crate::retained_history_operation::*;
    }
    pub mod retained_history_id {
        pub use crate::retained_history_id::*;
    }
    pub mod retained_history_message {
        pub use crate::retained_history_message::*;
    }
    pub mod retained_history_origin {
        pub use crate::retained_history_origin::*;
        pub use crate::retained_history_origin::testkit::json;
    }
    pub mod retained_history_metadata {
        pub use crate::retained_history_metadata::*;
    }
    pub mod retained_history_transition {
        pub use crate::retained_history_transition::*;
    }
    pub mod retained_history_edit {
        pub use crate::retained_history_edit::*;
    }
    pub mod retained_history_identity_records {
        pub use crate::retained_history_identity_records::*;
    }
    pub mod retained_history_conflict {
        pub use crate::retained_history_conflict::*;
    }
    pub mod retained_history_transition_validation {
        pub use crate::retained_history_transition_validation::*;
    }
}
