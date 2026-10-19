//! ⚛️ Facade-owned exported macros cut out of the WP4 trees: `__semio_actor_exports!`, `__semio_owned_core_exports!`, `plugin_exports!`, `__semio_plugin_actor_exports!`, `extension_exports!` (`⚛️reactor/🏃️runtime`).
//!
//! 🧭️ WP4 pre-created this empty module (PLAN-PLUGIN-CRATES.md, section 12.5.6): when the runtime file is cut into its crates, WP4 moves the five `#[macro_export]` definitions here verbatim, so `$crate::` keeps naming the facade.
/// 🧩️ One actor-world export implementation shared by plugin and extension component owners.
#[doc(hidden)]
#[macro_export]
macro_rules! __semio_actor_exports {
    ($guest:ident, $runtime:ident, $ensure:ident, $describe:ident) => {
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        struct $guest;
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        impl $crate::component::wasip2::exports::semio::framework::reactor::Guest for $guest {
            async fn stage_command_page(cursor: $crate::component::wasip2::exports::semio::framework::reactor::CommandPageCursor, bytes: Vec<u8>) -> Result<(), $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $crate::__async::poll::resolve_ready($crate::reactor::stage_command_page(cursor, bytes)).map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn stage_cold_pair_page(page: $crate::component::wasip2::exports::semio::framework::reactor::ColdDocumentPairPage) -> Result<(), $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $crate::__async::poll::resolve_ready($crate::reactor::stage_cold_pair_page(page)).map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn admit_actor_context(input: $crate::component::wasip2::exports::semio::framework::reactor::RetainedTurnInput,retained_policy:$crate::component::wasip2::exports::semio::framework::reactor::RetainedGrant) -> $crate::component::wasip2::exports::semio::framework::reactor::ActorAdmissionCall {
                $ensure();
                $crate::reactor::original_actor_context::exports::admit_wit(input,retained_policy)
            }
            async fn close_actor_context(input: $crate::component::wasip2::exports::semio::framework::reactor::RetainedTurnInput,retained_policy:$crate::component::wasip2::exports::semio::framework::reactor::RetainedGrant) -> $crate::component::wasip2::exports::semio::framework::reactor::ActorContextCloseCall {
                $crate::reactor::original_actor_context::exports::close_wit(input,retained_policy)
            }
            async fn reserve_poll_input(extent:u64,budget:$crate::component::wasip2::exports::semio::framework::reactor::Budget)->Result<$crate::component::wasip2::exports::semio::framework::reactor::ActorPollInputCall,$crate::component::wasip2::exports::semio::framework::reactor::ActorCallRefusal>{
                $ensure();
                $crate::reactor::original_actor_context::exports::reserve_poll_input_wit(extent,budget)
            }
            async fn write_poll_input(page:$crate::component::wasip2::exports::semio::framework::reactor::PollInputPage,budget:$crate::component::wasip2::exports::semio::framework::reactor::Budget)->Result<$crate::component::wasip2::exports::semio::framework::reactor::ActorPollInputCall,$crate::component::wasip2::exports::semio::framework::reactor::ActorCallRefusal>{
                $crate::reactor::original_actor_context::exports::write_poll_input_wit(page,budget)
            }
            async fn close_poll_input(budget:$crate::component::wasip2::exports::semio::framework::reactor::Budget)->Result<$crate::component::wasip2::exports::semio::framework::reactor::ActorPollInputCall,$crate::component::wasip2::exports::semio::framework::reactor::ActorCallRefusal>{
                $crate::reactor::original_actor_context::exports::close_poll_input_wit(budget)
            }
            async fn poll(
                events: Vec<$crate::component::wasip2::exports::semio::framework::reactor::Event>,
                budget: $crate::component::wasip2::exports::semio::framework::reactor::Budget,
            ) -> Result<$crate::component::wasip2::exports::semio::framework::reactor::TurnResult, $crate::component::wasip2::exports::semio::framework::reactor::ActorCallRefusal> {
                $ensure();
                $runtime.with(|runtime| $crate::reactor::original_actor_context::exports::poll_wit(runtime, events, budget))
            }
            async fn channel_version() -> u32 {
                $crate::component::wasip2::channel_version()
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        impl $crate::component::wasip2::exports::semio::framework::jobs::Guest for $guest {
            async fn start_job(source: Option<$crate::component::wasip2::exports::semio::framework::jobs::JobSource>, budget: $crate::component::wasip2::exports::semio::framework::jobs::JobBudget) -> $crate::component::wasip2::exports::semio::framework::jobs::JobStartCall {
                $ensure();
                $crate::reactor::original_actor_context::exports::start_job_wit(source, budget)
            }
            async fn step_job(job: u64, budget: $crate::component::wasip2::exports::semio::framework::jobs::JobBudget) -> $crate::component::wasip2::exports::semio::framework::jobs::JobStepCall {
                $ensure();
                $crate::reactor::original_actor_context::exports::step_job_wit(job, budget)
            }
            async fn read_job_output(job: u64, budget: $crate::component::wasip2::exports::semio::framework::jobs::JobBudget) -> $crate::component::wasip2::exports::semio::framework::jobs::JobOutputCall {
                $ensure();
                $crate::reactor::original_actor_context::exports::read_job_output_wit(job, budget)
            }
            async fn acknowledge_job_output(job: u64, budget: $crate::component::wasip2::exports::semio::framework::jobs::JobBudget) -> $crate::component::wasip2::exports::semio::framework::jobs::JobOutputAckCall {
                $ensure();
                $crate::reactor::original_actor_context::exports::acknowledge_job_output_wit(job, budget)
            }
            async fn cancel_job(job: u64, budget: $crate::component::wasip2::exports::semio::framework::jobs::JobBudget) -> $crate::component::wasip2::exports::semio::framework::jobs::JobCancelCall {
                $crate::reactor::original_actor_context::exports::cancel_job_wit(job, budget)
            }
            async fn close_jobs(budget: $crate::component::wasip2::exports::semio::framework::jobs::JobBudget) -> $crate::component::wasip2::exports::semio::framework::jobs::JobCancelCall {
                $ensure();
                $crate::reactor::original_actor_context::exports::close_jobs_wit(budget)
            }

            async fn take_segmented_download_chunk(instance_id: u32, operation_id: u64) -> Result<Option<Vec<u8>>, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime.with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_take_segmented_download_chunk(runtime, instance_id, operation_id))).map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        impl $crate::component::wasip2::exports::semio::framework::checkpoint::Guest for $guest {
            async fn checkpoint() -> Result<Vec<u8>, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime.with(|runtime| $crate::__async::poll::resolve_ready($crate::reactor::checkpoint_now(runtime))).map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn restore(state: Vec<u8>) -> Result<(), $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime.with(|runtime| $crate::__async::poll::resolve_ready($crate::reactor::restore_now(runtime, &state))).map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        impl $crate::component::wasip2::exports::semio::framework::describe::Guest for $guest {
            async fn describe() -> Vec<u8> {
                $ensure();
                $describe()
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        impl $crate::component::wasip2::exports::semio::framework::codec::Guest for $guest {
            async fn sqlite_schema(dialect: String) -> Result<String, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $crate::plugin_runtime::plugin_snapshot_sqlite_schema(&dialect).map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn sqlite_export(
                dialect: String,
                encoding: $crate::component::wasip2::exports::semio::framework::codec::SnapshotEncoding,
                snapshot: Vec<u8>,
                limits: $crate::component::wasip2::exports::semio::framework::codec::SnapshotLimits,
                native: $crate::component::wasip2::semio::framework::types::RetainedCloneGrant,
            ) -> Result<$crate::component::wasip2::exports::semio::framework::codec::SnapshotFileResult, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                let encoding = match encoding {
                    $crate::component::wasip2::exports::semio::framework::codec::SnapshotEncoding::Binary => "binary",
                    $crate::component::wasip2::exports::semio::framework::codec::SnapshotEncoding::Text => "text",
                }
                .to_string();
                let mut input = Some($crate::sqlite_wire::SnapshotInput { dialect, encoding, payload: snapshot, limits: $crate::component::wasip2::sqlite_limits(limits), native: $crate::component::wasip2::sqlite_grant(native) });
                $crate::operation_authority::with_snapshot_authority(&mut input,$crate::io::io_mechanism::IoNativeDirection::Decode,true,|input,native,snapshot|$crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_snapshot_sqlite_export(input,native,snapshot)))
                    .map_err(|error| $crate::component::wasip2::semio::framework::types::PluginError::OperationRefusal($crate::operation_authority::refusal_code(error.kind)))?
                    .map(|output| match output {
                        $crate::sqlite_wire::SnapshotFileResult::Done(output) => {
                            $crate::component::wasip2::exports::semio::framework::codec::SnapshotFileResult::Done($crate::component::wasip2::exports::semio::framework::codec::SnapshotFile { bytes: output.bytes, diagnostics: output.diagnostics })
                        }
                        $crate::sqlite_wire::SnapshotFileResult::Rejected(output) => {
                            $crate::component::wasip2::exports::semio::framework::codec::SnapshotFileResult::Rejected($crate::component::wasip2::exports::semio::framework::codec::SnapshotRejection {
                                kind: $crate::component::wasip2::sqlite_refusal_kind(output.kind),
                                message: output.message,
                                diagnostics: output.diagnostics,
                            })
                        }
                        $crate::sqlite_wire::SnapshotFileResult::Pending(owner) => $crate::component::wasip2::exports::semio::framework::codec::SnapshotFileResult::Pending($crate::component::wasip2::sqlite_retirement(owner)),
                    })
                    .map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn sqlite_import(
                dialect: String,
                file: Vec<u8>,
                limits: $crate::component::wasip2::exports::semio::framework::codec::SnapshotLimits,
                native: $crate::component::wasip2::semio::framework::types::RetainedCloneGrant,
            ) -> Result<$crate::component::wasip2::exports::semio::framework::codec::SnapshotPayloadResult, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                let mut input = Some($crate::sqlite_wire::SnapshotInput { dialect, encoding: String::new(), payload: file, limits: $crate::component::wasip2::sqlite_limits(limits), native: $crate::component::wasip2::sqlite_grant(native) });
                $crate::operation_authority::with_snapshot_authority(&mut input,$crate::io::io_mechanism::IoNativeDirection::Encode,true,|input,native,snapshot|$crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_snapshot_sqlite_import(input,native,snapshot)))
                    .map_err(|error| $crate::component::wasip2::semio::framework::types::PluginError::OperationRefusal($crate::operation_authority::refusal_code(error.kind)))?
                    .map(|output| match output {
                        $crate::sqlite_wire::SnapshotPayloadResult::Done(output) => {
                            let encoding = match output.encoding.as_str() {
                                "binary" => $crate::component::wasip2::exports::semio::framework::codec::SnapshotEncoding::Binary,
                                _ => $crate::component::wasip2::exports::semio::framework::codec::SnapshotEncoding::Text,
                            };
                            $crate::component::wasip2::exports::semio::framework::codec::SnapshotPayloadResult::Done($crate::component::wasip2::exports::semio::framework::codec::SnapshotPayload {
                                encoding,
                                bytes: output.bytes,
                                diagnostics: output.diagnostics,
                            })
                        }
                        $crate::sqlite_wire::SnapshotPayloadResult::Rejected(output) => {
                            $crate::component::wasip2::exports::semio::framework::codec::SnapshotPayloadResult::Rejected($crate::component::wasip2::exports::semio::framework::codec::SnapshotRejection {
                                kind: $crate::component::wasip2::sqlite_refusal_kind(output.kind),
                                message: output.message,
                                diagnostics: output.diagnostics,
                            })
                        }
                        $crate::sqlite_wire::SnapshotPayloadResult::Pending(owner) => $crate::component::wasip2::exports::semio::framework::codec::SnapshotPayloadResult::Pending($crate::component::wasip2::sqlite_retirement(owner)),
                    })
                    .map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn sqlite_take_file(ticket:u64)->Result<$crate::component::wasip2::exports::semio::framework::codec::SnapshotFileResult,$crate::component::wasip2::semio::framework::types::PluginError>{
                $crate::operation_authority::snapshot_take::<$crate::sqlite_wire::SnapshotFileResult>(ticket,true).map_err(|error|$crate::component::wasip2::semio::framework::types::PluginError::OperationRefusal($crate::operation_authority::refusal_code(error.kind)))?.map($crate::component::wasip2::sqlite_file).map_err(|fault|$crate::component::wasip2::plugin_error(&fault))
            }
            async fn sqlite_take_payload(ticket:u64)->Result<$crate::component::wasip2::exports::semio::framework::codec::SnapshotPayloadResult,$crate::component::wasip2::semio::framework::types::PluginError>{
                $crate::operation_authority::snapshot_take::<$crate::sqlite_wire::SnapshotPayloadResult>(ticket,true).map_err(|error|$crate::component::wasip2::semio::framework::types::PluginError::OperationRefusal($crate::operation_authority::refusal_code(error.kind)))?.map($crate::component::wasip2::sqlite_payload).map_err(|fault|$crate::component::wasip2::plugin_error(&fault))
            }
            async fn sqlite_retirement(copy_bytes:u64) -> Result<$crate::component::wasip2::exports::semio::framework::codec::SnapshotRetirement,u32> {
                $crate::operation_authority::snapshot_retirement(usize::try_from(copy_bytes).map_err(|_|2u32)?).map($crate::component::wasip2::sqlite_retirement).map_err(|error| $crate::operation_authority::refusal_code(error.kind))
            }
            async fn sqlite_close(ticket:u64,native:$crate::component::wasip2::semio::framework::types::RetainedCloneGrant) -> Result<$crate::component::wasip2::exports::semio::framework::codec::SnapshotCloseReceipt,u32> {
                let grant=$crate::component::wasip2::sqlite_grant(native).native().map_err(|error| $crate::operation_authority::refusal_code(error.kind))?;
                $crate::operation_authority::snapshot_close(ticket,grant).map(|(retirement,progress,refusal)|$crate::component::wasip2::exports::semio::framework::codec::SnapshotCloseReceipt {
                    retirement:$crate::component::wasip2::sqlite_retirement(retirement),
                    progress:$crate::component::wasip2::semio::framework::types::RetainedCloneProgress{copied_items:progress.copied_items as u64,copied_bytes:progress.copied_bytes as u64,retained_capacity_bytes:progress.retained_capacity_bytes as u64,released_bytes:progress.released_bytes as u64},refusal,
                }).map_err(|error| $crate::operation_authority::refusal_code(error.kind))
            }
            async fn pack_schema_hash(artifact_schema: String) -> Result<Vec<u8>, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_pack_schema_hash(runtime, &artifact_schema)))
                    .map(|hash| hash.to_vec())
                    .map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn genesis(artifact_schema: String, document_id: String) -> Result<$crate::component::wasip2::exports::semio::framework::codec::DocumentPair, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_genesis(runtime, &artifact_schema, &document_id)))
                    .map(|files| $crate::component::wasip2::exports::semio::framework::codec::DocumentPair { pack: files.pack, spr: files.spr })
                    .map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn print_mirror(artifact_schema: String, pair: $crate::component::wasip2::exports::semio::framework::codec::DocumentPair) -> Result<(String, String), $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_print_mirror(runtime, &artifact_schema, &pair.pack, &pair.spr)))
                    .map(|files| (files.dsl, files.ops))
                    .map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
            async fn apply_ops(
                artifact_schema: String,
                pair: $crate::component::wasip2::exports::semio::framework::codec::DocumentPair,
                ops: Vec<u8>,
            ) -> Result<$crate::component::wasip2::exports::semio::framework::codec::DocumentPair, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime
                    .with(|runtime| $crate::operation_authority::with_wit_operation_authority(|identity| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_apply_ops(runtime, &artifact_schema, &pair.pack, &pair.spr, &ops , identity)).map(|files| $crate::component::wasip2::exports::semio::framework::codec::DocumentPair { pack: files.pack, spr: files.spr }),$crate::operation_authority::return_walk::walk_document_pair))
            }
            async fn replay_envelopes(
                artifact_schema: String,
                pair: $crate::component::wasip2::exports::semio::framework::codec::DocumentPair,
                envelopes: Vec<u8>,
            ) -> Result<$crate::component::wasip2::exports::semio::framework::codec::DocumentPair, $crate::component::wasip2::semio::framework::types::PluginError> {
                $ensure();
                $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_replay_envelopes(runtime, &artifact_schema, &pair.pack, &pair.spr, &envelopes)))
                    .map(|files| $crate::component::wasip2::exports::semio::framework::codec::DocumentPair { pack: files.pack, spr: files.spr })
                    .map_err(|fault| $crate::component::wasip2::plugin_error(&fault))
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        $crate::__semio_export_component_guest!($guest);
    };
}
/// 🧬️ Defines the single owned core ABI used by plugin and extension component owners.
/// Every export must match the interpreter's authored name and core type.
/// Each owner supplies its persistent runtime slot, install hook and descriptor encoder.
#[doc(hidden)]
#[macro_export]
macro_rules! __semio_owned_core_exports {
    ($runtime:ident, $ensure:ident, $describe_bytes:ident) => {
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_alloc_v1(length: u32) -> u32 {
            $crate::owned_abi::allocate(length)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_dealloc_v1(pointer: u32, length: u32) {
            unsafe { $crate::owned_abi::deallocate(pointer, length) };
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_admit_actor_context_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_actor_admission_input(pointer, length) };
            $ensure();
            match input {
                Ok((input,policy)) => $crate::reactor::original_actor_context::exports::admit_owned(input,policy),
                Err(kind) => $crate::operation_authority::owned_result::encode_refusal(kind),
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_close_actor_context_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_actor_admission_input(pointer, length) };
            match input {
                Ok((input,policy)) => $crate::reactor::original_actor_context::exports::close_owned(input,policy),
                Err(kind) => $crate::operation_authority::owned_result::encode_refusal(kind),
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_poll_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_actor_poll_input(pointer, length) };
            $ensure();
            match input {
                Ok(input) => $runtime.with(|runtime| $crate::reactor::original_actor_context::exports::poll_owned(runtime, input)),
                Err(kind) => $crate::operation_authority::owned_result::encode_refusal(kind),
            }
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_start_job_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::owned_abi::StartJobInput>(pointer, length) };
            $ensure();
            let result: Result<(), Vec<u8>> = match input {
                Ok(input) => $runtime.with(|runtime| $crate::reactor::jobs::abi::start(runtime, input.job, input.kind, input.input)).map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_step_job_v1(job:u64,operation:u64,generation:u64,epoch:u64,clock_binding:u64,absolute_deadline_us:u64,total_fuel:u64,items:u64,copy:u64,capacity:u64,release:u64,depth:u64,policy_items:u64,policy_copy:u64,policy_capacity:u64,policy_release:u64,policy_depth:u64,fuel:u64,deadline_us:u64,cancelled:u64) -> u64 {
            match $crate::owned_abi::original_job_budget([operation,generation,epoch,clock_binding,absolute_deadline_us,total_fuel,items,copy,capacity,release,depth,policy_items,policy_copy,policy_capacity,policy_release,policy_depth,fuel,deadline_us,cancelled]){Ok(budget)=>$crate::reactor::original_actor_context::exports::step_job_owned(job,budget),Err(kind)=>$crate::operation_authority::owned_result::encode_refusal(kind)}
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_read_job_output_v1(job:u64,operation:u64,generation:u64,epoch:u64,clock_binding:u64,absolute_deadline_us:u64,total_fuel:u64,items:u64,copy:u64,capacity:u64,release:u64,depth:u64,policy_items:u64,policy_copy:u64,policy_capacity:u64,policy_release:u64,policy_depth:u64,fuel:u64,deadline_us:u64,cancelled:u64) -> u64 {
            match $crate::owned_abi::original_job_budget([operation,generation,epoch,clock_binding,absolute_deadline_us,total_fuel,items,copy,capacity,release,depth,policy_items,policy_copy,policy_capacity,policy_release,policy_depth,fuel,deadline_us,cancelled]){Ok(budget)=>$crate::reactor::original_actor_context::exports::read_job_output_owned(job,budget),Err(kind)=>$crate::operation_authority::owned_result::encode_refusal(kind)}
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_acknowledge_job_output_v1(job:u64,operation:u64,generation:u64,epoch:u64,clock_binding:u64,absolute_deadline_us:u64,total_fuel:u64,items:u64,copy:u64,capacity:u64,release:u64,depth:u64,policy_items:u64,policy_copy:u64,policy_capacity:u64,policy_release:u64,policy_depth:u64,fuel:u64,deadline_us:u64,cancelled:u64) -> u64 {
            match $crate::owned_abi::original_job_budget([operation,generation,epoch,clock_binding,absolute_deadline_us,total_fuel,items,copy,capacity,release,depth,policy_items,policy_copy,policy_capacity,policy_release,policy_depth,fuel,deadline_us,cancelled]){Ok(budget)=>$crate::reactor::original_actor_context::exports::acknowledge_job_output_owned(job,budget),Err(kind)=>$crate::operation_authority::owned_result::encode_refusal(kind)}
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_cancel_job_v1(job:u64,operation:u64,generation:u64,epoch:u64,clock_binding:u64,absolute_deadline_us:u64,total_fuel:u64,items:u64,copy:u64,capacity:u64,release:u64,depth:u64,policy_items:u64,policy_copy:u64,policy_capacity:u64,policy_release:u64,policy_depth:u64,fuel:u64,deadline_us:u64,cancelled:u64) -> u64 {
            match $crate::owned_abi::original_job_budget([operation,generation,epoch,clock_binding,absolute_deadline_us,total_fuel,items,copy,capacity,release,depth,policy_items,policy_copy,policy_capacity,policy_release,policy_depth,fuel,deadline_us,cancelled]){Ok(budget)=>$crate::reactor::original_actor_context::exports::cancel_job_owned(job,budget),Err(kind)=>$crate::operation_authority::owned_result::encode_refusal(kind)}
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_close_jobs_v1(operation:u64,generation:u64,epoch:u64,clock_binding:u64,absolute_deadline_us:u64,total_fuel:u64,items:u64,copy:u64,capacity:u64,release:u64,depth:u64,policy_items:u64,policy_copy:u64,policy_capacity:u64,policy_release:u64,policy_depth:u64,fuel:u64,deadline_us:u64,cancelled:u64) -> u64 {
            match $crate::owned_abi::original_job_budget([operation,generation,epoch,clock_binding,absolute_deadline_us,total_fuel,items,copy,capacity,release,depth,policy_items,policy_copy,policy_capacity,policy_release,policy_depth,fuel,deadline_us,cancelled]){Ok(budget)=>$crate::reactor::original_actor_context::exports::close_jobs_owned(budget),Err(kind)=>$crate::operation_authority::owned_result::encode_refusal(kind)}
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_borrow_job_output_v1(job:u64,operation:u64,generation:u64,offset:u64,length:u64) -> u64 {$crate::reactor::original_actor_context::exports::borrow_job_output_owned(job,operation,generation,offset,length)}
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_acknowledge_job_output_chunk_v1(job:u64,offset:u64,length:u64,operation:u64,generation:u64,epoch:u64,clock_binding:u64,absolute_deadline_us:u64,total_fuel:u64,items:u64,copy:u64,capacity:u64,release:u64,depth:u64,policy_items:u64,policy_copy:u64,policy_capacity:u64,policy_release:u64,policy_depth:u64,fuel:u64,deadline_us:u64,cancelled:u64) -> u64 {match $crate::owned_abi::original_job_budget([operation,generation,epoch,clock_binding,absolute_deadline_us,total_fuel,items,copy,capacity,release,depth,policy_items,policy_copy,policy_capacity,policy_release,policy_depth,fuel,deadline_us,cancelled]){Ok(budget)=>$crate::reactor::original_actor_context::exports::acknowledge_job_output_chunk_owned(job,offset,length,budget),Err(kind)=>$crate::operation_authority::owned_result::encode_refusal(kind)}}
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_checkpoint_v1() -> u64 {
            $ensure();
            let result = $runtime.with(|runtime| $crate::__async::poll::resolve_ready($crate::reactor::checkpoint_now(runtime))).map_err(|fault| $crate::encode_fault_bytes(&fault));
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_restore_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::owned_abi::RestoreInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => $runtime.with(|runtime| $crate::__async::poll::resolve_ready($crate::reactor::restore_now(runtime, &input.state))).map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_describe_v1() -> u64 {
            $ensure();
            $crate::owned_abi::return_bytes($describe_bytes())
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub extern "C" fn semio_owned_channel_version_v1() -> u64 {
            $crate::owned_abi::return_json(&$crate::component::wasip2::channel_version())
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_sqlite_schema_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::sqlite_wire::SnapshotInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => $crate::plugin_runtime::plugin_snapshot_sqlite_schema(&input.dialect).map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_sqlite_export_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::sqlite_wire::SnapshotInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => {let mut input=Some(input);match $crate::operation_authority::with_snapshot_authority(&mut input,$crate::io::io_mechanism::IoNativeDirection::Decode,false,|input,native,snapshot|$crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_snapshot_sqlite_export(input,native,snapshot))){Ok(result)=>result.map_err(|fault| $crate::encode_fault_bytes(&fault)),Err(error)=>return $crate::operation_authority::owned_result::encode_refusal(error.kind)}},
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_sqlite_import_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::sqlite_wire::SnapshotInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => {let mut input=Some(input);match $crate::operation_authority::with_snapshot_authority(&mut input,$crate::io::io_mechanism::IoNativeDirection::Encode,false,|input,native,snapshot|$crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_snapshot_sqlite_import(input,native,snapshot))){Ok(result)=>result.map_err(|fault| $crate::encode_fault_bytes(&fault)),Err(error)=>return $crate::operation_authority::owned_result::encode_refusal(error.kind)}},
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch="wasm32",target_env="p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_sqlite_retirement_v1(pointer:u32,length:u32)->u64 {
            let body=match unsafe{$crate::owned_abi::take_json::<usize>(pointer,length)}{Ok(body)=>body,Err(error)=>return $crate::owned_abi::return_json(&Err::<$crate::sqlite_wire::SnapshotRetirement,_>(error))};
            match $crate::operation_authority::snapshot_retirement(body){Ok(value)=>$crate::owned_abi::return_json(&Ok::<_,Vec<u8>>(value)),Err(error)=>$crate::operation_authority::owned_result::encode_refusal(error.kind)}
        }
        #[cfg(all(target_arch="wasm32",target_env="p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_sqlite_close_v1(pointer:u32,length:u32)->u64 {
            let input=unsafe{$crate::owned_abi::take_json::<$crate::sqlite_wire::SnapshotCloseInput>(pointer,length)};
            let input=match input{Ok(input)=>input,Err(error)=>return $crate::owned_abi::return_json(&Err::<$crate::sqlite_wire::SnapshotCloseReceipt,_>(error))};
            let grant=match input.native.native(){Ok(grant)=>grant,Err(error)=>return $crate::operation_authority::owned_result::encode_refusal(error.kind)};
            match $crate::operation_authority::snapshot_close(input.ticket,grant){Ok((retirement,progress,refusal))=>$crate::owned_abi::return_json(&Ok::<_,Vec<u8>>($crate::sqlite_wire::SnapshotCloseReceipt{retirement,progress:progress.into(),refusal})),Err(error)=>$crate::operation_authority::owned_result::encode_refusal(error.kind)}
        }
        #[cfg(all(target_arch="wasm32",target_env="p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_sqlite_take_file_v1(pointer:u32,length:u32)->u64 {
            let ticket=match unsafe{$crate::owned_abi::take_json::<u64>(pointer,length)}{Ok(ticket)=>ticket,Err(error)=>return $crate::owned_abi::return_json(&Err::<$crate::sqlite_wire::SnapshotFileResult,_>(error))};
            match $crate::operation_authority::snapshot_take::<$crate::sqlite_wire::SnapshotFileResult>(ticket,false){Ok(result)=>$crate::owned_abi::return_json(&result.map_err(|fault|$crate::encode_fault_bytes(&fault))),Err(error)=>$crate::operation_authority::owned_result::encode_refusal(error.kind)}
        }
        #[cfg(all(target_arch="wasm32",target_env="p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_sqlite_take_payload_v1(pointer:u32,length:u32)->u64 {
            let ticket=match unsafe{$crate::owned_abi::take_json::<u64>(pointer,length)}{Ok(ticket)=>ticket,Err(error)=>return $crate::owned_abi::return_json(&Err::<$crate::sqlite_wire::SnapshotPayloadResult,_>(error))};
            match $crate::operation_authority::snapshot_take::<$crate::sqlite_wire::SnapshotPayloadResult>(ticket,false){Ok(result)=>$crate::owned_abi::return_json(&result.map_err(|fault|$crate::encode_fault_bytes(&fault))),Err(error)=>$crate::operation_authority::owned_result::encode_refusal(error.kind)}
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_pack_schema_hash_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::owned_abi::CodecInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_pack_schema_hash(runtime, &input.artifact_schema)))
                    .map(|hash| hash.to_vec())
                    .map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_genesis_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::owned_abi::CodecInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_genesis(runtime, &input.artifact_schema, &input.document_id)))
                    .map(|files| $crate::owned_abi::CodecPair { pack: files.pack, spr: files.spr })
                    .map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_print_mirror_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::owned_abi::CodecInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_print_mirror(runtime, &input.artifact_schema, &input.pack, &input.spr)))
                    .map(|files| $crate::owned_abi::CodecMirror { dsl: files.dsl, ops: files.ops })
                    .map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_apply_ops_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::owned_abi::CodecInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => match $runtime
                    .with(|runtime| $crate::operation_authority::with_owned_operation_authority(|identity| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_apply_ops(runtime, &input.artifact_schema, &input.pack, &input.spr, &input.ops , identity))))
                    {Ok(result)=>result,Err(error)=>return $crate::operation_authority::owned_result::encode_refusal(error.kind)}
                    .map(|files| $crate::owned_abi::CodecPair { pack: files.pack, spr: files.spr })
                    .map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[unsafe(no_mangle)]
        pub unsafe extern "C" fn semio_owned_replay_envelopes_v1(pointer: u32, length: u32) -> u64 {
            let input = unsafe { $crate::owned_abi::take_json::<$crate::owned_abi::CodecInput>(pointer, length) };
            $ensure();
            let result = match input {
                Ok(input) => $runtime
                    .with(|runtime| $crate::__async::poll::resolve_ready($crate::plugin_runtime::plugin_artifact_replay_envelopes(runtime, &input.artifact_schema, &input.pack, &input.spr, &input.ops)))
                    .map(|files| $crate::owned_abi::CodecPair { pack: files.pack, spr: files.spr })
                    .map_err(|fault| $crate::encode_fault_bytes(&fault)),
                Err(error) => Err(error),
            };
            $crate::owned_abi::return_json(&result)
        }
    };
}
/// 🔌️ Exports one typed actor runtime for a plugin bundle.
#[macro_export]
macro_rules! plugin_exports {
    ($mounted_policy:expr, $bundle_fn:expr, $app:ty) => {
        $crate::__semio_plugin_actor_exports!($mounted_policy, $bundle_fn, $app, $crate::describe::describe_plugin, || {});
    };
}
#[macro_export]
macro_rules! __semio_plugin_actor_exports {
    ($mounted_policy:expr, $bundle_fn:expr, $app:ty, $describe:path, $refresh:expr) => {
        $crate::component_persistent_local! {
            static __SEMIO_PLUGIN_RUNTIME: $crate::plugin_runtime::PluginRuntime<$app> = $crate::plugin_runtime::PluginRuntime::new($mounted_policy).expect("explicit embedding mounted policy");
        }
        fn __semio_ensure_plugin_runtime() {
            static ONCE: std::sync::Once = std::sync::Once::new();
            ONCE.call_once(|| {
                $crate::plugin_runtime::install_guest_panic_report();
                __semio_install_plugin_bundle();
            });
            ($refresh)();
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        fn __semio_describe_component() -> Vec<u8> {
            __SEMIO_PLUGIN_RUNTIME.with(|runtime| $crate::__async::poll::resolve_ready($describe(runtime)))
        }
        $crate::__semio_owned_core_exports!(__SEMIO_PLUGIN_RUNTIME, __semio_ensure_plugin_runtime, __semio_describe_component);
        $crate::__semio_actor_exports!(__SemioComponentGuest, __SEMIO_PLUGIN_RUNTIME, __semio_ensure_plugin_runtime, __semio_describe_component);
        fn __semio_install_plugin_bundle() {
            __SEMIO_PLUGIN_RUNTIME.with(|runtime| {
                $crate::plugin_runtime::install_plugin_bundle_result(runtime, ($bundle_fn)());
            });
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[used]
        static _SEMIO_PLUGIN_COMPONENT_LINK: fn() = $crate::component_export_anchor;
        $crate::__semio_plugin_descriptor_fresh_test!($describe);
    };
}
/// 🧩️ Installs an extension crate's bundle builder into TLS for WIT guest exports.
#[macro_export]
macro_rules! extension_exports {
    ($mounted_policy:expr, $bundle_fn:expr, $plugin_fn:expr, $app:ty) => {
        $crate::component_persistent_local! {
            static __SEMIO_PENDING_EXTENSION: std::cell::RefCell<Option<$crate::ExtensionBundle>> = std::cell::RefCell::new(None);
        }
        $crate::__semio_plugin_actor_exports!(
            $mounted_policy,
            || {
                __SEMIO_PENDING_EXTENSION
                    .with(|pending| {
                        let mut pending = pending.borrow_mut();
                        if pending.is_none() {
                            *pending = Some(($bundle_fn)());
                        }
                        $crate::__async::poll::resolve_ready($crate::plugin_runtime::install_extension_bundle(&mut pending))
                    })
                    .map_err(|fault| $crate::PluginAssemblyError::new("extension.install", format!("{fault:?}")))?;
                $crate::__async::poll::resolve_ready($crate::plugin_runtime::extension_activate()).map_err(|fault| $crate::PluginAssemblyError::new("extension.activate", format!("{fault:?}")))?;
                ($plugin_fn)()
            },
            $app,
            $crate::describe::describe_extension_with_apps,
            || __SEMIO_PENDING_EXTENSION.with(|pending| {
                let mut pending = pending.borrow_mut();
                if pending.is_some() && $crate::__async::poll::resolve_ready($crate::plugin_runtime::install_extension_bundle(&mut pending)).expect("retained extension admission") {
                    $crate::__async::poll::resolve_ready($crate::plugin_runtime::extension_activate()).expect("admitted extension activation");
                }
            })
        );
    };
    ($mounted_policy:expr, $bundle_fn:expr) => {
        $crate::component_persistent_local! {
            static __SEMIO_EXTENSION_RUNTIME: $crate::plugin_runtime::PluginRuntime<$crate::app::NoPluginApp> = $crate::plugin_runtime::PluginRuntime::new($mounted_policy).expect("explicit embedding mounted policy");
            static __SEMIO_PENDING_EXTENSION: std::cell::RefCell<Option<$crate::ExtensionBundle>> = std::cell::RefCell::new(None);
        }
        fn __semio_ensure_extension_runtime() {
            static ONCE: std::sync::Once = std::sync::Once::new();
            ONCE.call_once(|| {
                $crate::plugin_runtime::install_guest_panic_report();
                __semio_install_extension_bundle();
                $crate::__async::poll::resolve_ready($crate::plugin_runtime::extension_activate()).expect("installed extension activation");
            });
            __SEMIO_PENDING_EXTENSION.with(|pending| {
                let mut pending = pending.borrow_mut();
                if pending.is_some() && $crate::__async::poll::resolve_ready($crate::plugin_runtime::install_extension_bundle(&mut pending)).expect("retained extension admission") {
                    $crate::__async::poll::resolve_ready($crate::plugin_runtime::extension_activate()).expect("admitted extension activation");
                }
            });
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        fn __semio_describe_component() -> Vec<u8> {
            $crate::__async::poll::resolve_ready($crate::describe::describe_extension())
        }
        $crate::__semio_owned_core_exports!(__SEMIO_EXTENSION_RUNTIME, __semio_ensure_extension_runtime, __semio_describe_component);
        $crate::__semio_actor_exports!(__SemioExtensionGuest, __SEMIO_EXTENSION_RUNTIME, __semio_ensure_extension_runtime, __semio_describe_component);
        fn __semio_install_extension_bundle() {
            __SEMIO_PENDING_EXTENSION.with(|pending| {
                let mut pending = pending.borrow_mut();
                if pending.is_none() {
                    *pending = Some(($bundle_fn)());
                }
                $crate::__async::poll::resolve_ready($crate::plugin_runtime::install_extension_bundle(&mut pending)).expect("retained extension admission");
            });
        }
        #[cfg(all(target_arch = "wasm32", target_env = "p2"))]
        #[used]
        static _SEMIO_EXTENSION_COMPONENT_LINK: fn() = $crate::component_export_anchor;
        $crate::__semio_extension_descriptor_fresh_test!();
    };
}
