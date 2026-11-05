//! 🏃️ `semio-plugin-interpreter-runtime`: the owned interpreter as a guest runtime ([`OwnedRuntime`]). It compiles a
//! `world actor` component for the repository-owned interpreter (`semio-plugin-interpreter`), runs its actor turns,
//! jobs, checkpoints and pure `codec` exports under instruction fuel, and answers every host import a guest can observe, in
//! the vocabulary of `semio-plugin-guest-contract`. A tool that only describes or verifies components links this and
//! no JIT.
//! @see https://webassembly.github.io/spec/core/exec/index.html
use semio_plugin_guest_contract::{decode_guest_fault_bytes, identity_turn_fault, operation_authority, original_actor_context, sqlite_wire, validate_cold_page, GuestCallCancellation, GuestDocumentMirror, GuestDocumentPair, JobBudget, JobStep, PluginHostError, TurnFault};
use semio_plugin_interpreter::{CoreError, CoreStepOutcome, HostCall, OwnedSemioArtifact, OwnedSemioInstance, StepControl, Value};
pub use semio_plugin_interpreter::OwnedSemioExport;
use semio_framework::kernel::{Budget, Event, TurnResult};
pub use semio_os_kernel::os_vcs::io::binary::entity_identity::control::OriginalOperationReceiving;
use semio_framework_value_derive::{FromValue, ToValue};
use std::sync::{Arc, Mutex};

/// 🪪️ The authored source of the owned runtime: the root and every child module, one text that the engine identity digests.
const OWNED_RUNTIME_SOURCE: &str = concat!(
    include_str!("🦀️.rs"),
    include_str!("🧮️owned-constants-and-codec-origin/🦀️.rs"),
    include_str!("🛰️owned-operation-and-guest-instance/🦀️.rs"),
    include_str!("📥️snapshot-operation-and-input-records/🦀️.rs"),
    include_str!("🔄️owned-runtime/🦀️.rs"),
    include_str!("⏯️operation-begin-resume-and-cancel/🦀️.rs"),
    include_str!("📣️host-replies-wasi-and-channel-faults/🦀️.rs"),
    include_str!("🎛️operation/🦀️.rs"),
    include_str!("🎛️operation/📥️input/🦀️.rs"),
    include_str!("🎛️operation/📤️checkpoint/🦀️.rs"),
    include_str!("💼️jobs/🦀️.rs"),
    include_str!("🎟️call/🦀️.rs"),
    include_str!("🎟️call/🔄️poll/🦀️.rs"),
    include_str!("🎟️call/🧬️codec/🦀️.rs"),
    include_str!("🎟️call/🧮️unit/🦀️.rs"),
    include_str!("🏗️startup/🦀️.rs"),
);

/// 🛂️ The neutral test caller admission the plugin host's runtime laws share; this crate's laws drive part of it.
#[cfg(test)]
pub(crate) use semio_plugin_host_operation::test_native_authority;

#[cfg(any(test, feature = "testkit"))]
pub mod channel_fixture{
//! 🌱️ The synthetic current Owned ABI exposes only the real channel result and traps unrelated operations.
use super::OwnedSemioExport;

pub fn owned_channel_guest(channel: u32) -> Vec<u8> {
    owned_guest(channel,false,0,None)
}

/// 🪞️ The input export echoes its exact borrowed wire for independent transport laws.
pub fn owned_input_echo_guest(channel:u32)->Vec<u8>{owned_guest(channel,true,0,None)}

/// 🕰️ Startup crosses the actual original clock import before returning the current channel.
pub fn owned_clock_guest(channel:u32,binding:u64)->Vec<u8>{owned_guest(channel,false,0,Some(binding))}

/// 📥️ The synthetic input receiver returns a real borrowed-input handback frame from its actual authority suffix.
pub fn owned_input_receipt_guest(channel:u32)->Vec<u8>{owned_guest(channel,false,3,None)}

/// ⏳️ The synthetic pending receiver retains its borrowed input through every actual invocation.
pub fn owned_input_pending_guest(channel:u32)->Vec<u8>{owned_guest(channel,false,4,None)}

fn owned_guest(channel:u32,input_echo:bool,input_receipt:u8,clock:Option<u64>)->Vec<u8>{
    fn uleb(mut value: u64, output: &mut Vec<u8>) {
        loop {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            output.push(if value == 0 { byte } else { byte | 0x80 });
            if value == 0 {
                return;
            }
        }
    }
    fn sleb(mut value: i64, output: &mut Vec<u8>) {
        loop {
            let byte = (value & 0x7f) as u8;
            value >>= 7;
            let done = (value == 0 && byte & 0x40 == 0) || (value == -1 && byte & 0x40 != 0);
            output.push(if done { byte } else { byte | 0x80 });
            if done {
                return;
            }
        }
    }
    fn section(id: u8, payload: Vec<u8>, output: &mut Vec<u8>) {
        output.push(id);
        uleb(payload.len() as u64, output);
        output.extend(payload);
    }
    
    let (types, bodies): (Vec<u8>, Vec<Vec<u8>>) = OwnedSemioExport::ALL
        .iter()
        .map(|export| match export {
            OwnedSemioExport::Allocate if input_echo||input_receipt!=0 => (0u8, vec![0x00,0x41,0xc0,0x00,0x0b]),
            OwnedSemioExport::Allocate => (0u8, vec![0x00, 0x41, 0x00, 0x0b]),
            OwnedSemioExport::Deallocate => (1, vec![0x00, 0x0b]),
            OwnedSemioExport::ChannelVersion => {
                let mut body = vec![0x00];
                if let Some(binding)=clock{body.push(0x42);sleb(binding as i64,&mut body);body.push(0x41);sleb(64,&mut body);body.extend([0x10,0]);}
                body.push(0x41);
                sleb(channel as i64, &mut body);
                body.push(0x0b);
                (2, body)
            }
            OwnedSemioExport::AdmitActorInput if input_receipt!=0=>{let mut body=vec![0];for index in 0..11{body.push(0x41);sleb(2048,&mut body);body.extend([0x20,index+2,0x37,3,8+index*8]);}body.push(0x42);sleb((136i64<<32)|2048,&mut body);body.push(0x0b);(9,body)},
            OwnedSemioExport::AdmitActorInput if input_echo => (9,vec![0x00,0x20,0x01,0xad,0x42,0x20,0x86,0x20,0x00,0xad,0x84,0x0b]),
            OwnedSemioExport::AdmitActorInput=>(9,vec![0x00,0x42,0,0x0b]),
            
            OwnedSemioExport::StartJob|OwnedSemioExport::PublishActorFailure|OwnedSemioExport::ReadActorFailure|OwnedSemioExport::AcknowledgeActorFailure|OwnedSemioExport::StepJob|OwnedSemioExport::ReadJobOutput|OwnedSemioExport::AcknowledgeJobOutput|OwnedSemioExport::CancelJob=>(4,vec![0x00,0x00,0x0b]),
            OwnedSemioExport::CloseJobs=>(5,vec![0x00,0x00,0x0b]),
            OwnedSemioExport::BorrowJobOutput|OwnedSemioExport::BorrowActorFailure=>(6,vec![0x00,0x00,0x0b]),
            OwnedSemioExport::AcknowledgeJobOutputChunk|OwnedSemioExport::AcknowledgeActorFailureChunk=>(7,vec![0x00,0x00,0x0b]),
            _ => (5, vec![0x00, 0x00, 0x0b]),
        })
        .unzip();
    let mut core = b"\0asm\x01\0\0\0".to_vec();
    let mut signatures=vec![10,0x60,1,0x7f,1,0x7f,0x60,2,0x7f,0x7f,0,0x60,0,1,0x7f,0x60,2,0x7f,0x7f,1,0x7e];
    for count in [20,19,5,22]{signatures.extend([0x60,count]);signatures.extend(std::iter::repeat_n(0x7e,count as usize));signatures.extend([1,0x7e]);}
    signatures.extend([0x60,2,0x7e,0x7f,0,0x60,21,0x7f,0x7f]);signatures.extend(std::iter::repeat_n(0x7e,19));signatures.extend([1,0x7e]);
    section(1,signatures,&mut core);
    if clock.is_some(){let module="semio:framework/pure@1.0.0";let name="operation-clock-us";let mut imports=vec![1];uleb(module.len()as u64,&mut imports);imports.extend(module.as_bytes());uleb(name.len()as u64,&mut imports);imports.extend(name.as_bytes());imports.extend([0,8]);section(2,imports,&mut core);}
    let mut functions = Vec::new();
    uleb(types.len() as u64, &mut functions);
    functions.extend(&types);
    section(3, functions, &mut core);
    section(5, vec![1, 0x00, 1], &mut core);
    let mut exports = Vec::new();
    uleb(OwnedSemioExport::ALL.len() as u64 + 1, &mut exports);
    exports.push(6);
    exports.extend(b"memory");
    exports.extend([2, 0]);
    for (index, export) in OwnedSemioExport::ALL.iter().enumerate() {
        uleb(export.core_name().len() as u64, &mut exports);
        exports.extend(export.core_name().as_bytes());
        exports.push(0);
        uleb(index as u64+u64::from(clock.is_some()), &mut exports);
    }
    section(7, exports, &mut core);
    let mut code = Vec::new();
    uleb(bodies.len() as u64, &mut code);
    for body in &bodies {
        uleb(body.len() as u64, &mut code);
        code.extend(body);
    }
    section(10, code, &mut core);
    if input_receipt!=0{let mut data=vec![1,0,0x41];sleb(2048,&mut data);data.push(0x0b);uleb(136,&mut data);data.extend(b"SAC3");data.extend([4,input_receipt&3,0,0]);data.resize(data.len()+128,0);section(11,data,&mut core);}
    let mut component = b"\0asm\x0d\0\x01\0".to_vec();
    section(1, core, &mut component);
    component
}

}

/// 🧮️ The heap witness the crate's allocation laws read through `semio_framework_trace::observe_heap_allocations_on_this_thread`.
#[cfg(test)]
#[global_allocator]
static OWNED_RUNTIME_HEAP_WITNESS: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

#[path = "🎛️operation/🦀️.rs"]
pub mod operation;
#[path="💼️jobs/🦀️.rs"]
mod original_jobs;
pub use original_jobs::OriginalGuestJobCall;

#[path="🎟️call/🔄️poll/🦀️.rs"]
mod original_poll;
#[path="🎟️call/🧬️codec/🦀️.rs"]
mod original_codec;
#[path="🎟️call/🧮️unit/🦀️.rs"]
mod original_unit;
pub use original_unit::OwnedOriginalUnitCall;

#[path="🎟️call/🦀️.rs"]
mod original_call;
pub use original_call::{OwnedOriginalCallStatus,OwnedOriginalCallProgress,OwnedOriginalCallResult};
pub use semio_framework_plugin_interpreter::Value as OriginalCoreValue;

#[path="🏗️startup/🦀️.rs"]
mod original_startup;
pub use original_startup::{OwnedActorStartup,OwnedActorStartupCall,OwnedActorStartupStatus};

//#region 🧠️OwnedRuntime

#[path = "🧮️owned-constants-and-codec-origin/🦀️.rs"]
mod owned_constants_and_codec_origin;
pub use owned_constants_and_codec_origin::{OWNED_STEP_FUEL, OwnedCompiledGuest};
use owned_constants_and_codec_origin::{OWNED_CHECKPOINT_MAGIC, OWNED_CODEC_ORIGIN_SCHEMA, OWNED_CODEC_ORIGIN_WAIT_POLL, OWNED_RESULT_LIMIT, OWNED_SLICE_DEADLINE_MS, OwnedCodecOrigin, OwnedCodecOriginAssembly, OwnedCodecOriginCell, OwnedCodecOriginState, OwnedCodecOriginUse};

#[path = "🛰️owned-operation-and-guest-instance/🦀️.rs"]
mod owned_operation_and_guest_instance;
pub use owned_operation_and_guest_instance::OwnedGuestInstance;
use owned_operation_and_guest_instance::{OwnedCheckpointMetadata, OwnedCheckpointView, OwnedOperation, OwnedPending, OwnedStage};

#[path = "📥️snapshot-operation-and-input-records/🦀️.rs"]
mod snapshot_operation_and_input_records;
pub use snapshot_operation_and_input_records::{OwnedCodecInput,OwnedInvocation, OwnedRuntime, OwnedSnapshotInput, OwnedSnapshotOperation, owned_engine_identity};
use snapshot_operation_and_input_records::{OwnedPollInput, OwnedRestoreInput, OwnedStartJobInput};

#[path = "🔄️owned-runtime/🦀️.rs"]
mod owned_runtime;

#[path = "⏯️operation-begin-resume-and-cancel/🦀️.rs"]
mod operation_begin_resume_and_cancel;
use operation_begin_resume_and_cancel::{OwnedDeadline, begin_owned_operation, begin_owned_scalar_operation, cancel_owned_operation, original_deadline_at, resume_owned_operation, resume_owned_operation_authored, resume_owned_operation_bound, resume_owned_operation_observed};

#[path = "📣️host-replies-wasi-and-channel-faults/🦀️.rs"]
mod host_replies_wasi_and_channel_faults;
pub use host_replies_wasi_and_channel_faults::decode_owned_result;
use host_replies_wasi_and_channel_faults::{admit_owned_channel, core_fault, reply_owned_host, turn_fault_host, write_owned_memory};
#[cfg(test)]
use host_replies_wasi_and_channel_faults::{owned_wasi_monotonic_clock, owned_wasi_nanoseconds};

#[cfg(test)]
#[path = "🧪️tests/🔬️owned-runtime/🦀️.rs"]
mod owned_runtime_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️microsecond-clock/🦀️.rs"]
mod microsecond_clock_tests;

#[cfg(test)]
#[path = "🧪️tests/🔬️owned-instance-open/🦀️.rs"]
mod owned_instance_open_tests;

//#endregion 🧠️OwnedRuntime
