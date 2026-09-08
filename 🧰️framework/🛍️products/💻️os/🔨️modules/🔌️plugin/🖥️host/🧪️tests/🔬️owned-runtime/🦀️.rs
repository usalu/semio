use super::*;

fn budget() -> Budget {
    Budget { fuel: 500_000_000, deadline_ms: 60_000, max_effects: 64, max_patch_bytes: 1 << 20, max_frames: 64 }
}

async fn cancel_to_completion(runtime: &OwnedRuntime, instance: &mut GuestInstance, job: u64) {
    let started = std::time::Instant::now();
    loop {
        match runtime.cancel_job(instance, job).await {
            Ok(()) => return,
            Err(TurnFault::DeadlineExceeded | TurnFault::FuelExhausted) if started.elapsed() < std::time::Duration::from_secs(60) => {}
            Err(error) => panic!("cancel owned job {job}: {error}"),
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn configured_component_executes_owned_describe_reactor_jobs_cancel_and_checkpoint_restore() {
    let Some(path) = std::env::var_os("SEMIO_OWNED_COMPONENT_FIXTURE") else { return };
    let bytes = std::fs::read(path).expect("read owned component fixture");
    let runtime = OwnedRuntime::new();
    let package = PackageRef { package: PackageId("owned-fixture".to_string()), hash: PackageHash([9; 32]) };
    let compiled = runtime.compile(&package, &bytes).await.expect("compile owned fixture");
    let mut progress = Vec::new();
    let descriptor = runtime.describe_observed(&compiled, budget(), |fuel, elapsed| progress.push((fuel, elapsed))).await.expect("execute owned describe");
    assert!(!descriptor.is_empty(), "owned describe returned no bytes");
    assert!(progress.last().is_some_and(|(fuel, _)| *fuel > 0), "owned describe emitted no terminal fuel observation");
    let mut deadline_progress = Vec::new();
    let zero_deadline = Budget { deadline_ms: 0, ..budget() };
    assert!(matches!(runtime.describe_observed(&compiled, zero_deadline, |fuel, elapsed| deadline_progress.push((fuel, elapsed))).await, Err(TurnFault::DeadlineExceeded)));
    assert_eq!(deadline_progress.last().map(|(fuel, _)| *fuel), Some(0), "owned describe deadline emitted no exact terminal fuel observation");

    let mut resumed = runtime.instantiate(&compiled, RuntimeActorId(40), &[], &budget()).await.expect("instantiate resumable owned fixture");
    let one_instruction = Budget { fuel: 1, ..budget() };
    assert!(matches!(runtime.execute_turn(&mut resumed, &[], one_instruction).await, Err(TurnFault::FuelExhausted)));
    let mid_call = runtime.checkpoint(&mut resumed).await.expect("checkpoint fuel-yielded owned turn");
    runtime.restore(&mut resumed, &mid_call).await.expect("restore fuel-yielded owned turn");
    assert!(runtime.execute_turn(&mut resumed, &[], budget()).await.expect("resume fuel-yielded owned turn").fuel_used > 1);

    let mut cancelled = runtime.instantiate(&compiled, RuntimeActorId(42), &[], &budget()).await.expect("instantiate cancellable owned fixture");
    assert!(matches!(runtime.execute_turn(&mut cancelled, &[], one_instruction).await, Err(TurnFault::FuelExhausted)));
    cancel_to_completion(&runtime, &mut cancelled, 69).await;

    let mut instance = runtime.instantiate(&compiled, RuntimeActorId(41), &[], &budget()).await.expect("instantiate owned fixture");
    let turn = runtime.execute_turn(&mut instance, &[], budget()).await.expect("execute owned empty turn");
    assert!(turn.fuel_used > 0, "owned turn did not report interpreter fuel");

    runtime.start_job(&mut instance, 70, "semio.test-owned-checkpoint", Vec::new()).await.expect("start checkpointed owned job");
    let checkpoint = runtime.checkpoint(&mut instance).await.expect("checkpoint owned instance");
    cancel_to_completion(&runtime, &mut instance, 70).await;
    runtime.restore(&mut instance, &checkpoint).await.expect("restore owned checkpoint");
    assert!(matches!(runtime.step_job(&mut instance, 70, JobBudget { fuel: 10_000_000, deadline_ms: 10_000 }).await.expect("step restored owned job"), JobStep::Failed { .. }));

    runtime.start_job(&mut instance, 71, "semio.test-owned-cancel", Vec::new()).await.expect("start cancellable owned job");
    cancel_to_completion(&runtime, &mut instance, 71).await;
    assert!(matches!(runtime.step_job(&mut instance, 71, JobBudget { fuel: 10_000_000, deadline_ms: 10_000 }).await.expect("step cancelled owned job"), JobStep::Failed { .. }));
}
