//! 🔬️ Owned runtime laws on real and synthetic owned-ABI components: describe, fuel-yielded turns, jobs, cancellation,
//! checkpoint/restore and channel admission.
use super::*;

fn budget() -> Budget {
    Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 500_000_000, deadline_ms: 60_000, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 64 }
}

fn cancel_to_completion(runtime: &OwnedRuntime, instance: &mut OwnedGuestInstance, job: u64) {
    let started = std::time::Instant::now();
    loop {
        match runtime.cancel_actor_job(instance, job) {
            Ok(()) => return,
            Err(TurnFault::DeadlineExceeded | TurnFault::FuelExhausted) if started.elapsed() < std::time::Duration::from_secs(60) => {}
            Err(error) => panic!("cancel owned job {job}: {error}"),
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn configured_component_executes_owned_describe_reactor_jobs_cancel_and_checkpoint_restore() {
    let mut original_observer=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;
    let mut original_recipient=semio_framework_value::native_encoding::NativeEncodeRetirementRecipient::new();
    let mut identity=crate::test_native_authority::original(&mut original_observer,&mut original_recipient);

    let Some(path) = std::env::var_os("SEMIO_OWNED_COMPONENT_FIXTURE") else { return };
    let bytes = std::fs::read(path).expect("read owned component fixture");
    let runtime = OwnedRuntime::new();
    let compiled = runtime.compile_component(&bytes).expect("compile owned fixture");
    let mut progress = Vec::new();
    let descriptor = runtime.describe_observed(&compiled, budget(), |fuel, elapsed| progress.push((fuel, elapsed))).await.expect("execute owned describe");
    assert!(!descriptor.is_empty(), "owned describe returned no bytes");
    assert!(progress.last().is_some_and(|(fuel, _)| *fuel > 0), "owned describe emitted no terminal fuel observation");
    let mut deadline_progress = Vec::new();
    let zero_deadline = Budget { deadline_ms: 0, ..budget() };
    assert!(matches!(runtime.describe_observed(&compiled, zero_deadline, |fuel, elapsed| deadline_progress.push((fuel, elapsed))).await, Err(TurnFault::DeadlineExceeded)));
    assert_eq!(deadline_progress.last().map(|(fuel, _)| *fuel), Some(0), "owned describe deadline emitted no exact terminal fuel observation");

    let mut resumed = runtime.instantiate_actor(&compiled).expect("instantiate resumable owned fixture");
    let one_instruction = Budget { retained: semio_framework::kernel::RetainedTurnInput { operation: 1, generation: 1, epoch: 1, grant: semio_framework_value::RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 32_768, maximum_capacity_bytes: 262_144, maximum_release_bytes: 1_048_576, maximum_depth: 4_096 } }, fuel: 1, ..budget() };
    assert!(matches!(runtime.execute_actor_turn(&mut resumed, &[], one_instruction, &mut identity), Err(TurnFault::FuelExhausted)));
    let mid_call = runtime.checkpoint_actor(&mut resumed, &mut identity).expect("checkpoint fuel-yielded owned turn");
    runtime.restore_actor(&mut resumed, &mid_call).expect("restore fuel-yielded owned turn");
    assert!(runtime.execute_actor_turn(&mut resumed, &[], budget(), &mut identity).expect("resume fuel-yielded owned turn").fuel_used > 1);

    let mut cancelled = runtime.instantiate_actor(&compiled).expect("instantiate cancellable owned fixture");
    assert!(matches!(runtime.execute_actor_turn(&mut cancelled, &[], one_instruction, &mut identity), Err(TurnFault::FuelExhausted)));
    cancel_to_completion(&runtime, &mut cancelled, 69);

    let mut instance = runtime.instantiate_actor(&compiled).expect("instantiate owned fixture");
    let turn = runtime.execute_actor_turn(&mut instance, &[], budget(), &mut identity).expect("execute owned empty turn");
    assert!(turn.fuel_used > 0, "owned turn did not report interpreter fuel");

    runtime.start_actor_job(&mut instance, 70, "semio.test-owned-checkpoint", Vec::new()).expect("start checkpointed owned job");
    let checkpoint = runtime.checkpoint_actor(&mut instance, &mut identity).expect("checkpoint owned instance");
    cancel_to_completion(&runtime, &mut instance, 70);
    runtime.restore_actor(&mut instance, &checkpoint).expect("restore owned checkpoint");
    assert!(matches!(runtime.step_actor_job(&mut instance, 70, JobBudget { fuel: 10_000_000, deadline_ms: 10_000 }).expect("step restored owned job"), JobStep::Failed { .. }));

    runtime.start_actor_job(&mut instance, 71, "semio.test-owned-cancel", Vec::new()).expect("start cancellable owned job");
    cancel_to_completion(&runtime, &mut instance, 71);
    assert!(matches!(runtime.step_actor_job(&mut instance, 71, JobBudget { fuel: 10_000_000, deadline_ms: 10_000 }).expect("step cancelled owned job"), JobStep::Failed { .. }));
    crate::test_native_authority::close(&mut identity);
}

/// 🧩️ A real owned-ABI guest of channel `channel`, built byte by byte in the interpreter's own component framing: its
/// `semio_owned_channel_version_v1` answers `channel` as JSON, allocate/deallocate are inert, and every other export traps
/// (`unreachable`), so any frame a host served before admitting the guest would surface as a trap, not a refusal.
fn owned_channel_guest(channel: u32) -> Vec<u8> {
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
    let answer = channel.to_string().into_bytes();
    let (types, bodies): (Vec<u8>, Vec<Vec<u8>>) = OwnedSemioExport::ALL
        .iter()
        .map(|export| match export {
            OwnedSemioExport::Allocate => (0u8, vec![0x00, 0x41, 0x00, 0x0b]),
            OwnedSemioExport::Deallocate => (1, vec![0x00, 0x0b]),
            OwnedSemioExport::ChannelVersion => {
                let mut body = vec![0x00, 0x42];
                sleb(((answer.len() as i64) << 32) | 1024, &mut body);
                body.push(0x0b);
                (2, body)
            }
            OwnedSemioExport::Checkpoint | OwnedSemioExport::Describe => (2, vec![0x00, 0x00, 0x0b]),
            _ => (3, vec![0x00, 0x00, 0x0b]),
        })
        .unzip();
    let mut core = b"\0asm\x01\0\0\0".to_vec();
    section(1, vec![4, 0x60, 1, 0x7f, 1, 0x7f, 0x60, 2, 0x7f, 0x7f, 0, 0x60, 0, 1, 0x7e, 0x60, 2, 0x7f, 0x7f, 1, 0x7e], &mut core);
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
        uleb(index as u64, &mut exports);
    }
    section(7, exports, &mut core);
    let mut code = Vec::new();
    uleb(bodies.len() as u64, &mut code);
    for body in &bodies {
        uleb(body.len() as u64, &mut code);
        code.extend(body);
    }
    section(10, code, &mut core);
    let mut data = vec![1, 0x00, 0x41];
    sleb(1024, &mut data);
    data.push(0x0b);
    uleb(answer.len() as u64, &mut data);
    data.extend(&answer);
    section(11, data, &mut core);
    let mut component = b"\0asm\x0d\0\x01\0".to_vec();
    section(1, core, &mut component);
    component
}

/// 🤝️ LAW (audit F2/F7): both owned host paths admit a guest's app channel before serving it anything. An actor
/// instantiation and a codec call (the origin a hub Check In fold, the SQLite export/import and the wgpu document codec run
/// on) refuse an older or newer guest with `plugin.channel-mismatch` naming both versions — before any other export ran,
/// since every other export of the guest traps — while a guest of the host's own channel is admitted and reaches its
/// first frame.
#[semio_framework_async_macros::async_test]
async fn owned_hosts_admit_a_guest_channel_before_any_frame() {
    let runtime = OwnedRuntime::new();
    let input = OwnedCodecInput { artifact_schema: "s.fixture.document", document_id: "", pack: &[], spr: &[], ops: &[] };
    let refusal = |guest: u32, fault: &semio_framework::Fault| {
        assert_eq!(fault.code.0, semio_framework_os_kernel::CHANNEL_MISMATCH_CODE, "guest {guest}");
        assert_eq!(fault.param("guest"), Some(guest.to_string().as_str()));
        assert_eq!(fault.param("host"), Some(semio_framework_os_kernel::CHANNEL_VERSION.to_string().as_str()));
    };
    for guest in [semio_framework_os_kernel::CHANNEL_VERSION, semio_framework_os_kernel::CHANNEL_VERSION - 1, semio_framework_os_kernel::CHANNEL_VERSION + 1] {
        let compiled = runtime.compile_component(&owned_channel_guest(guest)).expect("compile the synthetic owned guest");
        let actor = runtime.instantiate_actor(&compiled);
        let codec = runtime.codec_call::<Vec<u8>, _>(&compiled, OwnedOperation::PackSchemaHash, &input, budget(), |_, _| {}, None);
        if guest == semio_framework_os_kernel::CHANNEL_VERSION {
            assert!(actor.is_ok(), "a guest of the host's own channel is instantiated");
            assert!(matches!(codec, Err(TurnFault::Trapped(_))), "an admitted guest reaches its first codec frame");
            continue;
        }
        match actor {
            Err(PluginHostError::Refused(fault)) => refusal(guest, &fault),
            other => panic!("guest {guest}: actor instantiation was not refused at admission: {:?}", other.err()),
        }
        match codec {
            Err(TurnFault::Host(PluginHostError::Refused(fault))) => refusal(guest, &fault),
            other => panic!("guest {guest}: codec call was not refused at admission: {:?}", other.err()),
        }
    }
}
