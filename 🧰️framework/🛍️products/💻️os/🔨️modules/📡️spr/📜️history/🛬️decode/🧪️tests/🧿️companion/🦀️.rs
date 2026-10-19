//! 🧿️ Companion tests of os-history-record-decode: the retained History record decoders.
//!
//! 🧭️ The tests are mounted unchanged from the production tree and reach the production crate through its public API only.
#![cfg(test)]

#[global_allocator]
static TEST_ALLOCATION_OBSERVER: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

pub use semio_framework_os_history_record_decode::*;
use semio_framework_os_history_testing_retirement as test_retirement;

#[path = "../../📋️records/🧪️tests"]
pub mod retained_history_records {
    pub use semio_framework_os_history_record_decode::testkit::retained_history_records::*;
    use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand, RetainedCloneGrant, RetainedCloneProgress};

    #[path = "🦀️.rs"]
    mod tests;
}

#[path = "../../📋️records/📦️paged/🧪️tests"]
pub mod retained_history_paged_records {
    pub use semio_framework_os_history_record_decode::testkit::retained_history_paged_records::*;
    use semio_framework_os_history_record_decode::RetainedHistoryRecordsStep;
    use semio_framework_value_persistent::{list::PagedList, list::PagedListError};
    use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand, RetainedCloneGrant, RetainedCloneProgress};

    #[path = "🦀️.rs"]
    mod tests;
}

#[path = "../../🧾️operation/🧪️tests"]
pub mod retained_history_operation {
    pub use semio_framework_os_history_record_decode::testkit::retained_history_operation::*;
    use semio_framework_os_history_record_decode::OpPayload;
    use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand, RetainedCloneGrant, RetainedCloneProgress};
    use protocol::codec::{RetainedWireFieldDecode, RetainedWireByteSource, OriginalWireSourceIdentity, HistoryReadLimits};

    #[path = "🦀️.rs"]
    mod tests;
}

#[path = "../../🪪️identity/🧪️tests"]
pub mod retained_history_id {
    pub use semio_framework_os_history_record_decode::testkit::retained_history_id::*;
    use semio_framework_os_history_record_decode::{OriginalHistoryEdits, HistoryEdit};
    use protocol::codec::{RetainedWireFieldDecode, RetainedWireByteSource, OriginalWireSourceIdentity};
    use protocol::dictionary::RetainedSourceDictionary;
    use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand, RetainedCloneGrant, RetainedCloneProgress};

    #[path = "🦀️.rs"]
    mod tests;
}

#[path = "../../📨️message/🧪️tests"]
pub mod retained_history_message {
    pub use semio_framework_os_history_record_decode::testkit::retained_history_message::*;
    use semio_framework_os_history_record_decode::{OriginalHistoryEdits, RetainedHistoryRecords};
    use protocol::codec::{RetainedWireFieldDecode, RetainedWireByteSource, OriginalWireSourceIdentity, HistoryReadLimits};
    use protocol::dictionary::RetainedSourceDictionary;
    use semio_framework_value::{ValueError, ValueRefusalKind, RetirementDemand, RetainedCloneGrant, RetainedCloneProgress};

    #[path = "🦀️.rs"]
    mod tests;
}

#[path = "."]
pub mod retained_history_origin {
    pub use semio_framework_os_history_record_decode::testkit::retained_history_origin::*;
    use semio_framework_value::{DslValue, Number, ValueError, ValueRefusalKind, RetirementDemand, RetainedCloneGrant, RetainedCloneProgress, ErasedSnapshotRetirement, admit_owned_retirement, owned_retirement_birth_bytes};
    use std::mem::size_of;
    use protocol::{MutationOrigin, ForeignTarget, SchemaId, PayloadHash};

    #[path = "../../🔀️origin/🧪️tests/🦀️.rs"]
    mod tests;

    #[path = "../../🔀️origin/🔣️json/🧪️tests"]
    pub mod json {
        pub use semio_framework_os_history_record_decode::testkit::retained_history_origin::json::*;
        use semio_framework_pack_json::{JsonGrammarCursor, JsonSourceCursor, JsonReadLimits, JsonMemberPolicy, JsonError, JsonReadSource};
        use semio_framework_value::{DslValue, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ValueError, ValueRefusalKind};
        use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
        use protocol::MutationOrigin;
        use protocol::codec::{OriginalWireSourceIdentity, RetainedWireByteSource};

        #[path = "🦀️.rs"]
        mod tests;
    }
}

#[path = "../../🧾️metadata/🧪️tests"]
pub mod retained_history_metadata {
    pub use semio_framework_os_history_record_decode::testkit::retained_history_metadata::*;
    use semio_framework_os_history_record_decode::{OriginalHistoryEdits, HistoryEdit};
    use protocol::codec::{RetainedWireByteSource, OriginalWireSourceIdentity, HistoryReadLimits};
    use protocol::dictionary::RetainedSourceDictionary;
    use protocol::{MutationOrigin, TransactionRef};
    use semio_framework_pack_json::{JsonReadLimits, JsonGrammarCursor, JsonError};
    use semio_framework_value::{DslValue, SharedUtf8, ValueError, ValueRefusalKind, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ErasedSnapshotRetirement, admit_owned_retirement, owned_retirement_birth_bytes};
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;
    use std::mem::size_of_val;

    #[path = "🦀️.rs"]
    mod tests;
}

#[path = "../../🔀️transition/🧪️tests"]
pub mod retained_history_transition {
    pub use semio_framework_os_history_record_decode::testkit::retained_history_transition::*;
    use protocol::codec::{RetainedWireByteSource, OriginalWireSourceIdentity, HistoryReadLimits, RetainedWireFieldDecode};
    use protocol::dictionary::RetainedSourceDictionary;
    use semio_framework_value::{SharedUtf8, ValueError, ValueRefusalKind, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand};
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

    #[path = "🦀️.rs"]
    mod tests;
}

#[path = "../../🧵️edit/🧪️tests"]
pub mod retained_history_edit {
    pub use semio_framework_os_history_record_decode::testkit::retained_history_edit::*;
    use semio_framework_os_history_record_decode::{OriginalHistoryEdits, HistoryEdit};
    use protocol::codec::{RetainedWireByteSource, OriginalWireSourceIdentity, HistoryReadLimits, RetainedWireFieldDecode};
    use protocol::dictionary::RetainedSourceDictionary;
    use semio_framework_value::{SharedUtf8, ValueError, ValueRefusalKind, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ErasedSnapshotRetirement, admit_owned_retirement, owned_retirement_birth_bytes};
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

    #[path = "🦀️.rs"]
    mod tests;
}

#[path = "../../📇️identity-records/🧪️tests"]
pub mod retained_history_identity_records {
    pub use semio_framework_os_history_record_decode::testkit::retained_history_identity_records::*;
    use protocol::codec::{RetainedWireByteSource, OriginalWireSourceIdentity, HistoryReadLimits};
    use protocol::dictionary::RetainedSourceDictionary;
    use semio_framework_value::{ValueError, ValueRefusalKind, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand};
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

    #[path = "🦀️.rs"]
    mod tests;
}

#[path = "../../⚔️conflict/🧪️tests"]
pub mod retained_history_conflict {
    pub use semio_framework_os_history_record_decode::testkit::retained_history_conflict::*;
    use semio_framework_os_history_record_decode::{OriginalHistoryEdits, HistoryEdit};
    use protocol::codec::{RetainedWireByteSource, OriginalWireSourceIdentity, HistoryReadLimits, RetainedWireFieldDecode};
    use protocol::dictionary::RetainedSourceDictionary;
    use semio_framework_value::{SharedUtf8, ValueError, ValueRefusalKind, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand, ErasedSnapshotRetirement, admit_owned_retirement, owned_retirement_birth_bytes};
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

    #[path = "🦀️.rs"]
    mod tests;
}

#[path = "../../🔀️transition/🛂️payload/🧪️tests"]
pub mod retained_history_transition_validation {
    pub use semio_framework_os_history_record_decode::testkit::retained_history_transition_validation::*;
    use protocol::codec::HistoryReadLimits;
    use pack::format::RetainedUtf8Cursor;
    use semio_framework_value::{ValueError, ValueRefusalKind, RetainedCloneGrant, RetainedCloneProgress, RetirementDemand};
    use semio_framework_value_native_owned::native_decoding::NativeDecodeControl;

    #[path = "🦀️.rs"]
    mod tests;
}
