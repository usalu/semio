//! 🧩️ WASI P2 bindings for the actor world shared by plugins and extensions.
//! The package role identifies each guest; reactor, jobs, checkpoint and describe use the same world.
#![allow(unsafe_op_in_unsafe_fn)]

use wit_bindgen::generate;

generate!({
    world: "actor",
    path: "../../../🧬️schema",
    async: [
        "-import:semio:framework/pure@1.0.0#operation-begin",
        "-import:semio:framework/pure@1.0.0#operation-progress",
        "-import:semio:framework/pure@1.0.0#operation-allocation",
        "-import:semio:framework/pure@1.0.0#operation-reserve-return",
        "-import:semio:framework/pure@1.0.0#operation-finish",
    ],
    pub_export_macro: true,
    default_bindings_module: "$crate::component::wasip2",
    disable_custom_section_link_helpers: true,
});

/// 🪶️ Converts the schema-declared ABI bounds without platform truncation.
pub fn sqlite_limits(value: exports::semio::framework::codec::SnapshotLimits) -> crate::sqlite_wire::SnapshotLimits {
    crate::sqlite_wire::SnapshotLimits {
        max_file_bytes: value.max_file_bytes,
        max_value_bytes: value.max_value_bytes,
        max_allocation_bytes: value.max_allocation_bytes,
        max_schema_bytes: value.max_schema_bytes,
        max_rows: value.max_rows,
        max_columns: value.max_columns,
        max_tables: value.max_tables,
        max_pages: value.max_pages,
    }
}

/// 🎟️ Preserves all five original native ownership axes in the semantic codec request.
pub fn sqlite_grant(value: semio::framework::types::RetainedCloneGrant) -> crate::sqlite_wire::SnapshotGrant {
    crate::sqlite_wire::SnapshotGrant { maximum_items:value.maximum_items, maximum_copy_bytes:value.maximum_copy_bytes, maximum_capacity_bytes:value.maximum_capacity_bytes, maximum_release_bytes:value.maximum_release_bytes, maximum_depth:value.maximum_depth }
}

/// 👓️ Projects only the actual same-instance retirement ticket and scalar demands.
pub fn sqlite_retirement(value:crate::sqlite_wire::SnapshotRetirement)->exports::semio::framework::codec::SnapshotRetirement {
    exports::semio::framework::codec::SnapshotRetirement { ticket:value.ticket,owned_bytes:value.owned_bytes,refusal:value.refusal,native_pending:value.native_pending,output_pending:value.output_pending,terminal:value.terminal,demands:exports::semio::framework::codec::SnapshotDemands{items:value.demands.items,copy_bytes:value.demands.copy_bytes,capacity_bytes:value.demands.capacity_bytes,release_bytes:value.demands.release_bytes,depth:value.demands.depth} }
}

/// 📤️ Moves the exact already reserved snapshot file owner into its generated component DTO.
pub fn sqlite_file(value:crate::sqlite_wire::SnapshotFileResult)->exports::semio::framework::codec::SnapshotFileResult {
    use exports::semio::framework::codec as Wire;match value {crate::sqlite_wire::SnapshotFileResult::Done(value)=>Wire::SnapshotFileResult::Done(Wire::SnapshotFile{bytes:value.bytes,diagnostics:value.diagnostics}),crate::sqlite_wire::SnapshotFileResult::Rejected(value)=>Wire::SnapshotFileResult::Rejected(Wire::SnapshotRejection{kind:sqlite_refusal_kind(value.kind),message:value.message,diagnostics:value.diagnostics}),crate::sqlite_wire::SnapshotFileResult::Pending(value)=>Wire::SnapshotFileResult::Pending(sqlite_retirement(value))}
}
/// 📥️ Moves the exact already reserved native payload without allocating encoding text.
pub fn sqlite_payload(value:crate::sqlite_wire::SnapshotPayloadResult)->exports::semio::framework::codec::SnapshotPayloadResult {
    use exports::semio::framework::codec as Wire;match value {crate::sqlite_wire::SnapshotPayloadResult::Done(value)=>Wire::SnapshotPayloadResult::Done(Wire::SnapshotPayload{encoding:match value.encoding{crate::sqlite_wire::SnapshotEncoding::Binary=>Wire::SnapshotEncoding::Binary,crate::sqlite_wire::SnapshotEncoding::Text=>Wire::SnapshotEncoding::Text},bytes:value.bytes,diagnostics:value.diagnostics}),crate::sqlite_wire::SnapshotPayloadResult::Rejected(value)=>Wire::SnapshotPayloadResult::Rejected(Wire::SnapshotRejection{kind:sqlite_refusal_kind(value.kind),message:value.message,diagnostics:value.diagnostics}),crate::sqlite_wire::SnapshotPayloadResult::Pending(value)=>Wire::SnapshotPayloadResult::Pending(sqlite_retirement(value))}
}

/// 🏷️ Projects every canonical cause into the declared codec ABI.
pub fn sqlite_refusal_kind(value: semio_framework_value::ValueRefusalKind) -> exports::semio::framework::codec::ValueRefusalKind {
    use semio_framework_value::ValueRefusalKind as Native;
    use exports::semio::framework::codec::ValueRefusalKind as Wire;
    match value { Native::InvalidValue => Wire::InvalidValue, Native::Canceled => Wire::Canceled, Native::OwnershipLimit => Wire::OwnershipLimit, Native::AllocationFailed => Wire::AllocationFailed, Native::WorkLimit => Wire::WorkLimit, Native::DepthLimit => Wire::DepthLimit, Native::UnsupportedOwner => Wire::UnsupportedOwner, Native::InvariantViolated => Wire::InvariantViolated }
}

/// 🧯️ Converts a framework fault into the generated actor-world error type.
pub fn plugin_error(fault: &semio_framework::Fault) -> semio::framework::types::PluginError {
    semio::framework::types::PluginError::Fault(semio_framework_diagnostic::encode_fault_bytes(fault))
}

/// 🤝️ The app-channel version this guest was compiled for — what `reactor.channel-version` and its owned twin answer, and
/// what every host admits before any frame (`protocol::admit_guest_channel_version`).
pub fn channel_version() -> u32 {
    semio_framework_os_kernel::CHANNEL_VERSION
}

#[doc(hidden)]
#[macro_export]
macro_rules! __semio_export_component_guest {
    ($guest:ident) => {
        $crate::component::wasip2::export!($guest);
    };
}

pub fn component_export_anchor() {}

pub async fn log(level: &str, message: &str) {
    semio::framework::pure::log(level, message);
}

pub async fn now_ms() -> i64 {
    semio::framework::pure::now_ms()
}

pub async fn trace_span(name: &str) {
    semio::framework::pure::trace_span(name);
}
