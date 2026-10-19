//! 📣️ Host replies, wasi and channel faults: the wasi clock and entropy answers, the owned host reply, the guest memory helpers, the channel admission and result decoding and the turn and core faults. See [`super`].

use super::*;
use semio_framework_plugin_guest_contract::GUEST_DIAGNOSTICS_CAPACITY_BYTES;

//#region 🧠️OwnedRuntime
pub(super) fn owned_wasi_nanoseconds(duration: std::time::Duration) -> Result<u64, PluginHostError> {
    u64::try_from(duration.as_nanos()).map_err(|_| PluginHostError::Plugin("owned WASI monotonic clock exhausted its unsigned nanosecond range".to_string()))
}

pub(super) fn owned_wasi_monotonic_clock() -> Result<Value, PluginHostError> {
    static ORIGIN: std::sync::OnceLock<std::time::Instant> = std::sync::OnceLock::new();
    let nanoseconds = owned_wasi_nanoseconds(ORIGIN.get_or_init(std::time::Instant::now).elapsed())?;
    Ok(Value::I64(i64::from_ne_bytes(nanoseconds.to_ne_bytes())))
}

/// 🕰️ `wasi:clocks/wall-clock.now`'s `datetime` record (`seconds: u64` at 0, `nanoseconds: u32` at 8,
/// padded to 16), read from the host's real wall clock — a guest Store stamps history with it.
fn owned_wasi_wall_clock() -> Result<[u8; 16], PluginHostError> {
    let since_epoch = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_err(|_| PluginHostError::Plugin("owned WASI wall clock reads before the Unix epoch".to_string()))?;
    let mut datetime = [0u8; 16];
    datetime[..8].copy_from_slice(&since_epoch.as_secs().to_le_bytes());
    datetime[8..12].copy_from_slice(&since_epoch.subsec_nanos().to_le_bytes());
    Ok(datetime)
}

/// 🎲️ One platform-entropy word for `wasi:random` — a guest Store's replica identity comes from it.
fn owned_wasi_entropy() -> Result<u64, PluginHostError> {
    semio_framework_os_kernel::os_identity::entropy_u64().map_err(|error| PluginHostError::Plugin(format!("owned WASI random: {error}")))
}

pub(super) fn reply_owned_host<'loan,'source>(state: &mut OwnedGuestInstance, call: &HostCall, receiver: Option<&'loan mut (dyn OriginalOperationReceiving+'source)>) -> Result<Vec<Value>, PluginHostError> {
    let values = match (call.module.as_str(), call.name.as_str()) {
        ("semio:framework/pure@1.0.0", "operation-begin") => {
            let outcome = receiver.ok_or_else(|| operation_authority::refusal(6)).and_then(|receiver| receiver.begin());
            let mut result = [0u8;16];
            match outcome { Ok(maximum) => result[8..].copy_from_slice(&(maximum as u64).to_le_bytes()), Err(error) => { result[..4].copy_from_slice(&1u32.to_le_bytes()); result[8..12].copy_from_slice(&operation_authority::refusal_code(error.kind).to_le_bytes()); } }
            write_owned_memory(state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?, owned_argument_i32(call, 0)?, &result).map_err(turn_fault_host)?;
            Vec::new()
        }
        ("semio:framework/pure@1.0.0", "operation-progress" | "operation-allocation" | "operation-finish") => {
            let outcome = receiver.ok_or_else(|| operation_authority::refusal(6)).and_then(|receiver| {
                let number = |index:usize| match call.arguments.get(index) { Some(Value::I64(value)) => usize::try_from(u64::from_ne_bytes(value.to_ne_bytes())).map_err(|_|operation_authority::refusal(2)), _ => Err(operation_authority::refusal(4)) };
                match call.name.as_str() {
                    "operation-progress" => receiver.progress(semio_framework_value::native_encoding::NativeEncodeProgress{completed:number(0)?,total:number(1)?,owned_bytes:number(2)?}),
                    "operation-allocation" => receiver.allocation(semio_framework_value::native_encoding::NativeEncodeAllocation{bytes:number(0)?,owned_bytes:number(1)?,next_owned_bytes:number(2)?,maximum_bytes:number(3)?}),
                    "operation-finish" => receiver.finish(number(0)?),
                    _ => Err(operation_authority::refusal(4)),
                }
            });
            vec![Value::I32(outcome.map_or_else(|error|operation_authority::refusal_code(error.kind) as i32,|()|0))]
        }
        ("semio:framework/pure@1.0.0", "operation-clock-us") => {
            let binding=match call.arguments.first(){Some(Value::I64(value))=>u64::from_ne_bytes(value.to_ne_bytes()),_=>return Err(PluginHostError::NativeIo(operation_authority::refusal(4)))};let mut result=[0u8;16];if let Some(now)=state.original_job_clock.filter(|(original,_)|*original==binding).and_then(|(_,clock)|clock()){result[..4].copy_from_slice(&1u32.to_le_bytes());result[8..].copy_from_slice(&now.to_le_bytes());}
            write_owned_memory(state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?,owned_argument_i32(call,1)?,&result).map_err(turn_fault_host)?;Vec::new()
        }
        ("semio:framework/pure@1.0.0", "now-ms") => vec![Value::I64(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |duration| duration.as_millis() as i64))],
        ("semio:framework/pure@1.0.0", "log") | ("semio:framework/pure@1.0.0", "trace-span") => Vec::new(),
        ("$root", "[context-get-0]") => vec![Value::I32(state.context)],
        ("$root", "[context-set-0]") => {
            let [Value::I32(context)] = call.arguments.as_slice() else { return Err(PluginHostError::Plugin("invalid owned context-set argument".to_string())) };
            state.context = *context;
            Vec::new()
        }
        ("$root", "[waitable-set-new]")
        | ("wasi:io/streams@0.2.0", "[method]output-stream.subscribe")
        | ("wasi:clocks/monotonic-clock@0.2.0", "subscribe-duration")
        | ("wasi:cli/stdin@0.2.0", "get-stdin")
        | ("wasi:cli/stdout@0.2.0", "get-stdout")
        | ("wasi:cli/stderr@0.2.0", "get-stderr") => {
            let resource = state.next_resource;
            state.next_resource = state.next_resource.wrapping_add(1).max(1);
            vec![Value::I32(resource)]
        }
        ("$root", "[waitable-set-poll]") => vec![Value::I32(0)],
        ("$root", "[waitable-join]")
        | ("$root", "[waitable-set-drop]")
        | ("[export]$root", "[task-cancel]")
        | ("wasi:io/poll@0.2.0", "[method]pollable.block")
        | ("wasi:io/error@0.2.0", "[resource-drop]error")
        | ("wasi:io/poll@0.2.0", "[resource-drop]pollable")
        | ("wasi:io/streams@0.2.0", "[resource-drop]input-stream")
        | ("wasi:io/streams@0.2.0", "[resource-drop]output-stream") => Vec::new(),
        ("wasi:clocks/monotonic-clock@0.2.0", "now") => vec![owned_wasi_monotonic_clock()?],
        ("wasi:random/insecure-seed@0.2.9", "insecure-seed") => {
            let seed = [owned_wasi_entropy()?.to_le_bytes(), owned_wasi_entropy()?.to_le_bytes()].concat();
            write_owned_memory(state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?, owned_argument_i32(call, 0)?, &seed).map_err(turn_fault_host)?;
            Vec::new()
        }
        ("wasi:random/random@0.2.9", "get-random-u64") => vec![Value::I64(i64::from_le_bytes(owned_wasi_entropy()?.to_le_bytes()))],
        ("wasi:cli/environment@0.2.0", "get-environment") => {
            write_owned_zeroes(state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?, owned_argument_i32(call, 0)?, 8)?;
            Vec::new()
        }
        ("wasi:clocks/wall-clock@0.2.0", "now") => {
            write_owned_memory(state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?, owned_argument_i32(call, 0)?, &owned_wasi_wall_clock()?).map_err(turn_fault_host)?;
            Vec::new()
        }
        ("wasi:cli/terminal-stdin@0.2.0", "get-terminal-stdin") | ("wasi:cli/terminal-stdout@0.2.0", "get-terminal-stdout") | ("wasi:cli/terminal-stderr@0.2.0", "get-terminal-stderr") => {
            write_owned_zeroes(state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?, owned_argument_i32(call, 0)?, 8)?;
            Vec::new()
        }
        ("wasi:io/poll@0.2.0", "poll") => {
            write_owned_zeroes(state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?, owned_argument_i32(call, 2)?, 8)?;
            Vec::new()
        }
        ("wasi:io/streams@0.2.0", "[method]output-stream.check-write") => {
            let pointer = owned_argument_i32(call, 1)?;
            write_owned_zeroes(state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?, pointer, 16)?;
            write_owned_memory(state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?, pointer.wrapping_add(8), &65_536u64.to_le_bytes()).map_err(turn_fault_host)?;
            Vec::new()
        }
        ("wasi:io/streams@0.2.0", "[method]output-stream.write") => {
            let pointer = owned_argument_i32(call, 1)?;
            let length = owned_argument_i32(call, 2)?;
            retain_owned_guest_diagnostics(state, pointer, length)?;
            write_owned_zeroes(state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?, owned_argument_i32(call, 3)?, 16)?;
            Vec::new()
        }
        ("wasi:io/streams@0.2.0", "[method]output-stream.blocking-flush") => {
            write_owned_zeroes(state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?, owned_argument_i32(call, 1)?, 16)?;
            Vec::new()
        }
        _ => return Err(PluginHostError::Plugin(format!("owned host import {}::{} is unavailable", call.module, call.name))),
    };
    if values.len() != call.results.len() || values.iter().zip(&call.results).any(|(value, expected)| value.value_type() != *expected) {
        return Err(PluginHostError::Plugin(format!("owned host import {}::{} returned the wrong shape", call.module, call.name)));
    }
    Ok(values)
}

fn owned_argument_i32(call: &HostCall, index: usize) -> Result<i32, PluginHostError> {
    match call.arguments.get(index) {
        Some(Value::I32(value)) => Ok(*value),
        _ => Err(PluginHostError::Plugin(format!("owned host import {}::{} argument {index} is not i32", call.module, call.name))),
    }
}

fn write_owned_zeroes(actor: &mut OwnedSemioInstance, pointer: i32, length: usize) -> Result<(), PluginHostError> {
    write_owned_memory(actor, pointer, &vec![0; length]).map_err(turn_fault_host)
}

/// 🩺️ Retains what the owned guest writes to `wasi:cli/stdout`/`stderr`, bounded by
/// [`GUEST_DIAGNOSTICS_CAPACITY_BYTES`]. The owned shim used to answer `output-stream.write` with a
/// zeroed result and DISCARD the bytes, so a guest panic — which `panic = "abort"` prints to stderr
/// immediately before its `unreachable` — reached the host as the bare text `wasm trap: unreachable
/// executed` and nothing else. That is what made ticket 26/09/18 slice TC3c's `codec.genesis` trap
/// undiagnosable from the outside: the guest had already said exactly what was wrong. The wasmtime
/// path has kept this since [`guest_wasi_ctx`]; this is the owned interpreter's half of it.
fn retain_owned_guest_diagnostics(state: &mut OwnedGuestInstance, pointer: i32, length: i32) -> Result<(), PluginHostError> {
    let start = pointer as u32 as usize;
    let length = length as u32 as usize;
    let memory = state.actor.as_mut().ok_or_else(||PluginHostError::NativeIo(operation_authority::refusal(4)))?.memory_mut().ok_or_else(|| PluginHostError::Plugin("owned actor memory is unavailable".to_string()))?;
    let end = start.checked_add(length).filter(|end| *end <= memory.len()).ok_or_else(|| PluginHostError::Plugin("owned guest diagnostics are outside guest memory".to_string()))?;
    let budget = GUEST_DIAGNOSTICS_CAPACITY_BYTES.saturating_sub(state.diagnostics.len());
    let bytes = &memory[start..end.min(start.saturating_add(budget))];
    state.diagnostics.extend_from_slice(bytes);
    Ok(())
}

pub(super) fn write_owned_memory(actor: &mut OwnedSemioInstance, pointer: i32, bytes: &[u8]) -> Result<(), TurnFault> {
    let start = pointer as u32 as usize;
    let memory = actor.memory_mut().ok_or_else(|| TurnFault::Trapped("owned actor memory is unavailable".to_string()))?;
    let end = start.checked_add(bytes.len()).filter(|end| *end <= memory.len()).ok_or_else(|| TurnFault::Trapped("owned input is outside guest memory".to_string()))?;
    memory[start..end].copy_from_slice(bytes);
    Ok(())
}

/// 🤝️ Fuel and wall bound of the owned `semio_owned_channel_version_v1` read at instantiation: one JSON integer, never a turn.
const OWNED_CHANNEL_VERSION_FUEL: u64 = 1_000_000;

const OWNED_CHANNEL_VERSION_DEADLINE_MS: u32 = 5_000;

/// 🔖️ Reads a fresh owned instance's `semio_owned_channel_version_v1` once and admits it through the one kernel admission
/// ([`semio_framework_os_kernel::admit_guest_channel_version`]) before the instance serves anything — an actor turn or a codec call —
/// so a guest of another channel is refused as `PluginHostError::Refused` (`plugin.channel-mismatch`).
pub(super) fn admit_owned_channel(state: &mut OwnedGuestInstance) -> Result<(), TurnFault> {
    begin_owned_operation(state, OwnedOperation::ChannelVersion, None)?;
    let invocation = resume_owned_operation(state, OwnedOperation::ChannelVersion, OWNED_CHANNEL_VERSION_FUEL, u64::from(OWNED_CHANNEL_VERSION_DEADLINE_MS)*1000)?;
    let guest: u32 = serde_json::from_slice(&invocation.output).map_err(|error| PluginHostError::Json(error.to_string()))?;
    semio_framework_os_kernel::admit_guest_channel_version(guest, semio_framework_os_kernel::CHANNEL_VERSION).map_err(|fault| PluginHostError::Refused(Box::new(fault)))?;
    Ok(())
}

/// 📥️ Preserves current fixed actor receipts before decoding an owned JSON result and its typed guest fault.
pub fn decode_owned_result<T: serde::de::DeserializeOwned>(bytes: &[u8]) -> Result<T, TurnFault> {
    if bytes.starts_with(b"SAC2") {
        let original = original_actor_context::OriginalActorFixedReceipt::read(bytes).map_err(identity_turn_fault)?;
        return Err(TurnFault::ActorCall(original));
    }
    let result: Result<T, Vec<u8>> = serde_json::from_slice(bytes).map_err(|error| PluginHostError::Json(error.to_string()))?;
    result.map_err(|error| decode_guest_fault_bytes(&error))
}

pub(super) fn turn_fault_host(fault: TurnFault) -> PluginHostError {
    match fault {
        TurnFault::Host(error) => error,
        other => PluginHostError::Plugin(other.to_string()),
    }
}

/// 🧯️ A core interpreter error, as the trap a guest call reports.
fn core_fault(error: CoreError) -> TurnFault {
    TurnFault::Trapped(error.to_string())
}
//#endregion 🧠️OwnedRuntime
