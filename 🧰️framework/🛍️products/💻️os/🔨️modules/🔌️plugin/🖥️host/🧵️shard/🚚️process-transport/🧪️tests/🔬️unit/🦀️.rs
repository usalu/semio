use super::*;

//#region 🧪️FramingTests
#[semio_framework_async_macros::async_test]
async fn a_data_frame_round_trips_arbitrary_bytes_including_zero_and_newline() {
    let payload = vec![0u8, 1, 2, b'\n', 255, 0u8];
    let mut buffer = Vec::new();
    framing::write_frame(&mut buffer, framing::TAG_DATA, &payload).expect("write");
    let mut cursor = io::Cursor::new(buffer);
    match framing::read_frame(&mut cursor).expect("read").expect("some frame") {
        framing::Frame::Data(bytes) => assert_eq!(bytes, payload),
        framing::Frame::Heartbeat => panic!("expected Data"),
    }
}

#[semio_framework_async_macros::async_test]
async fn a_heartbeat_frame_carries_no_payload_and_does_not_desync_the_next_frame() {
    let mut buffer = Vec::new();
    framing::write_frame(&mut buffer, framing::TAG_HEARTBEAT, &[]).expect("write heartbeat");
    framing::write_frame(&mut buffer, framing::TAG_DATA, b"after").expect("write data");
    let mut cursor = io::Cursor::new(buffer);
    assert!(matches!(framing::read_frame(&mut cursor).expect("read").expect("some"), framing::Frame::Heartbeat));
    match framing::read_frame(&mut cursor).expect("read").expect("some") {
        framing::Frame::Data(bytes) => assert_eq!(bytes, b"after"),
        framing::Frame::Heartbeat => panic!("expected Data"),
    }
}

#[semio_framework_async_macros::async_test]
async fn read_frame_reports_clean_eof_as_none_not_an_error() {
    let mut cursor = io::Cursor::new(Vec::<u8>::new());
    assert!(framing::read_frame(&mut cursor).expect("EOF is Ok(None), not Err").is_none());
}

#[test]
fn nonblocking_decoder_preserves_fragmented_and_concatenated_frames() {
    let mut encoded = Vec::new();
    framing::write_frame(&mut encoded, framing::TAG_DATA, b"first").expect("first");
    framing::write_frame(&mut encoded, framing::TAG_HEARTBEAT, &[]).expect("heartbeat");
    framing::write_frame(&mut encoded, framing::TAG_DATA, b"second").expect("second");
    let split_a = 3;
    let split_b = encoded.len() - 4;
    let mut decoder = framing::Decoder::default();
    for fragment in [&encoded[..split_a], &encoded[split_a..split_b], &encoded[split_b..]] {
        let mut cursor = io::Cursor::new(fragment);
        assert!(matches!(decoder.poll(&mut cursor, fragment.len()).expect("decode fragment"), framing::PipeState::Open));
    }
    assert_eq!(decoder.pop(), Some(framing::Frame::Data(b"first".to_vec())));
    assert_eq!(decoder.pop(), Some(framing::Frame::Heartbeat));
    assert_eq!(decoder.pop(), Some(framing::Frame::Data(b"second".to_vec())));
    assert_eq!(decoder.pop(), None);
}

#[test]
fn nonblocking_decoder_rejects_oversized_prefix_before_payload_allocation() {
    let mut prefix = vec![framing::TAG_DATA];
    prefix.extend_from_slice(&((MAX_FRAME_BYTES as u32) + 1).to_le_bytes());
    let mut cursor = io::Cursor::new(prefix.clone());
    let mut decoder = framing::Decoder::default();
    let error = decoder.poll(&mut cursor, prefix.len()).expect_err("oversized prefix must fail");
    assert_eq!(error.kind(), io::ErrorKind::InvalidData);
}
//#endregion 🧪️FramingTests

//#region 🧪️SelectionTests
#[semio_framework_async_macros::async_test]
async fn shard_runtime_kind_defaults_to_thread_and_opts_into_process_explicitly() {
    // 🧯️ Env-var mutation makes this test order-sensitive vs. any OTHER test reading the same
    // var in the same process; none exists in this crate today (grepped before writing), and
    // `cargo test`'s default multi-threaded runner still serializes same-process env access
    // adequately for a single var this test both sets and restores.
    let previous = std::env::var("SEMIO_SHARD_KIND").ok();
    std::env::remove_var("SEMIO_SHARD_KIND");
    assert_eq!(ShardRuntimeKind::from_env().await, ShardRuntimeKind::Thread, "unset must default to Thread — never Process by default in P1");
    std::env::set_var("SEMIO_SHARD_KIND", "process");
    assert_eq!(ShardRuntimeKind::from_env().await, ShardRuntimeKind::Process);
    match previous {
        Some(value) => std::env::set_var("SEMIO_SHARD_KIND", value),
        None => std::env::remove_var("SEMIO_SHARD_KIND"),
    }
}
//#endregion 🧪️SelectionTests

//#region 🧪️WatchdogTests
#[semio_framework_async_macros::async_test]
async fn watchdog_does_not_fire_while_heartbeats_keep_advancing() {
    let mut watchdog = ProcessShardWatchdog::new(1000, 0).await;
    assert!(!watchdog.poll(100, 100).await);
    assert!(!watchdog.poll(1200, 1300).await);
    assert!(!watchdog.poll(2500, 2600).await);
}

#[semio_framework_async_macros::async_test]
async fn watchdog_fires_after_three_consecutive_stale_windows() {
    let mut watchdog = ProcessShardWatchdog::new(1000, 0).await;
    // heartbeat frozen at 0 forever — three separate `timeout_ms`-apart polls must each count
    // exactly one miss, matching `ShardClient`'s `lastMissCountedAtMs` gate (a flurry of polls
    // inside the SAME window must not multi-count).
    assert!(!watchdog.poll(0, 1500).await);
    assert!(!watchdog.poll(0, 1600).await, "same window as the previous miss — must not double-count");
    assert!(!watchdog.poll(0, 2600).await);
    assert!(watchdog.poll(0, 3700).await, "third consecutive stale window — must fire");
}

#[semio_framework_async_macros::async_test]
async fn watchdog_resets_the_miss_count_on_a_fresh_heartbeat() {
    let mut watchdog = ProcessShardWatchdog::new(1000, 0).await;
    assert!(!watchdog.poll(0, 1500).await);
    assert!(!watchdog.poll(0, 2600).await);
    assert!(!watchdog.poll(5000, 5000).await, "a fresh heartbeat must reset the miss streak");
    assert!(!watchdog.poll(5000, 6600).await);
    assert!(!watchdog.poll(5000, 7700).await);
    assert!(watchdog.poll(5000, 8800).await, "three fresh consecutive misses after the reset");
}

#[semio_framework_async_macros::async_test]
async fn poll_with_liveness_fires_immediately_on_a_dead_child_without_waiting_out_the_timeout() {
    let mut watchdog = ProcessShardWatchdog::new(1000, 0).await;
    assert!(watchdog.poll_with_liveness(0, false, 5).await, "EOF is definitive — no need to wait for three stale windows");
}
//#endregion 🧪️WatchdogTests

//#region 🧪️ProcessTransportTests
/// 🎯️ The one test in this module that spawns a REAL child process (not the `semio-shard`
/// binary — a plain `cat`, which echoes stdin to stdout byte-for-byte) — proves `ProcessTransport
/// ::spawn`/`send`/`recv`/`kill` against an actual OS process without needing a wasmtime
/// component built first. The `semio-shard`-hosted, real-wasmtime-actor version of this same
/// proof (kill -9, detect, rebuild, sibling unaffected) is `👶️child/🦀️.rs`'s own
/// `#[ignore]`d integration test — see the P1 report's `## kill-rebuild-evidence`.
#[semio_framework_async_macros::async_test]
async fn process_transport_round_trips_bytes_through_a_real_child_process() {
    // 👶️ host-dedyn: `#[test] fn` is a sanctioned `block_on` entry point (R4 clause 5) —
    // `ShardTransport`'s methods are `async fn` now (O1); every impl here resolves on its
    // first poll (pure `Mutex`/`AtomicBool`/pipe I/O, no real suspension), so `block_on` never
    // actually parks.
    let transport = ProcessTransport::spawn(Path::new("cat"), &[]).await.expect("spawn cat");
    semio_framework_async::block_on(transport.send(b"hello-process-shard"));
    let mut received = None;
    for _ in 0..200 {
        if let Some(bytes) = semio_framework_async::block_on(transport.recv()) {
            received = Some(bytes);
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    assert_eq!(received, Some(b"hello-process-shard".to_vec()));
    semio_framework_async::block_on(transport.kill());
}

#[semio_framework_async_macros::async_test]
async fn kill_terminates_the_child_and_is_observed_as_eof_on_recv_side() {
    let transport = ProcessTransport::spawn(Path::new("cat"), &[]).await.expect("spawn cat");
    assert!(transport.is_child_alive().await);
    semio_framework_async::block_on(transport.kill());
    let mut dead = false;
    for _ in 0..200 {
        if !transport.is_child_alive().await {
            dead = true;
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    assert!(dead, "reader thread must observe EOF after kill()");
}

#[semio_framework_async_macros::async_test]
async fn an_externally_killed_child_is_detected_as_dead_without_this_type_calling_kill() {
    let transport = ProcessTransport::spawn(Path::new("sleep"), &["30".to_string()]).await.expect("spawn sleep 30");
    let pid = transport.child_id().await.expect("pid");
    // 🔪️ An INVOLUNTARY death — `kill -9` from OUTSIDE this type, mirroring the packet's
    // required proof ("kill -9 a shard child -> the parent detects it") rather than merely
    // exercising this type's OWN `kill()` method (the test above already covers that).
    let status = Command::new("kill").args(["-9", &pid.to_string()]).status().expect("run kill -9");
    assert!(status.success());
    let mut dead = false;
    for _ in 0..300 {
        if !transport.is_child_alive().await {
            dead = true;
            break;
        }
        thread::sleep(Duration::from_millis(10));
    }
    assert!(dead, "an externally SIGKILLed child must be observed as dead via EOF");
}
//#endregion 🧪️ProcessTransportTests

//#region 🧪️KillRebuildEvidence
/// 🎯️ P1's headline acceptance proof, against a REAL `semio-shard` child hosting a REAL
/// `wasm32-wasip2 world actor` component (the F1 scale fixture) through a REAL `WasmtimeRuntime`
/// — not `MockGuestRuntime`, not an in-process loopback. `#[ignore]`d (not part of the default
/// 74-test baseline this packet must not regress) because it needs a pre-built component:
///
/// ```text
/// CARGO_TARGET_DIR=<ticket>/🎯️target-p1 cargo component build -p semio-framework-os-scale-fixture \
///   --target wasm32-wasip2 --features component-guest
/// SEMIO_SCALE_FIXTURE_WASM=<ticket>/🎯️target-p1/wasm32-wasip2/wasm-dev/semio_framework_os_scale_fixture.wasm \
///   cargo test -p semio-framework-plugin-host --lib -- --ignored process_shard_kill_is_detected
/// ```
///
/// Sequence: spawn TWO process shards (`a`, `b`), activate one actor on each (`InstanceOpen`
/// with the fixture's `idle` profile), confirm both reply with a real `ShardOutcome::Turn`.
/// `kill -9` shard `a`'s child from OUTSIDE this process's own `ProcessTransport::kill()` (the
/// packet's exact requirement: "kill -9 a shard child -> the parent detects it"). Poll
/// `ProcessShardWatchdog` until it reports the shard lost. Spawn a FRESH child at the same
/// routing slot ("rebuild"), activate a fresh actor on it, confirm it replies — proving the
/// shard is usable again. Throughout, shard `b` — untouched — must still answer a SECOND turn,
/// proving the failure was isolated to `a`.
#[semio_framework_async_macros::async_test]
#[ignore = "needs a pre-built wasm32-wasip2 component at SEMIO_SCALE_FIXTURE_WASM; see this test's own doc comment"]
async fn process_shard_kill_is_detected_and_the_shard_rebuilds_while_a_sibling_shard_stays_healthy() {
    let Ok(wasm_path) = std::env::var("SEMIO_SCALE_FIXTURE_WASM") else {
        eprintln!("[skip] SEMIO_SCALE_FIXTURE_WASM not set — see this test's doc comment for the build command");
        return;
    };
    let shard_bin = std::env::var("CARGO_BIN_EXE_semio-shard").expect("cargo test sets CARGO_BIN_EXE_semio-shard for this package's own [[bin]] target");

    let shard_a = ProcessTransport::spawn(Path::new(&shard_bin), &[wasm_path.clone(), "scale-fixture-a".to_string(), "1".to_string()]).await.expect("spawn shard a");
    let shard_b = ProcessTransport::spawn(Path::new(&shard_bin), &[wasm_path.clone(), "scale-fixture-b".to_string(), "2".to_string()]).await.expect("spawn shard b");

    semio_framework_async::block_on(shard_a.send(&instance_open_envelope(1, 1, "idle").await));
    semio_framework_async::block_on(shard_b.send(&instance_open_envelope(2, 1, "idle").await));

    let outcome_a = recv_outcome(&shard_a, 400).await.expect("shard a must reply to InstanceOpen with a real ShardOutcome");
    let outcome_b = recv_outcome(&shard_b, 400).await.expect("shard b must reply to InstanceOpen with a real ShardOutcome");
    assert!(matches!(outcome_a, crate::shard::ShardOutcome::Turn { actor: 1, .. }), "shard a: expected ShardOutcome::Turn, got {outcome_a:?}");
    assert!(matches!(outcome_b, crate::shard::ShardOutcome::Turn { actor: 2, .. }), "shard b: expected ShardOutcome::Turn, got {outcome_b:?}");

    let pid_a = shard_a.child_id().await.expect("shard a pid");
    // 🔪️ Involuntary death, exactly the packet's required proof — `kill -9` from OUTSIDE this
    // process's own `ProcessTransport::kill()`.
    let status = Command::new("kill").args(["-9", &pid_a.to_string()]).status().expect("run kill -9 on shard a");
    assert!(status.success(), "kill -9 shard a must succeed");

    let mut watchdog = ProcessShardWatchdog::new(500, now_ms()).await;
    let mut lost = false;
    for _ in 0..100 {
        thread::sleep(Duration::from_millis(50));
        if watchdog.poll_with_liveness(semio_framework_async::block_on(shard_a.heartbeat()), shard_a.is_child_alive().await, now_ms()).await {
            lost = true;
            break;
        }
    }
    assert!(lost, "the watchdog must detect shard a as lost after the external kill -9");

    // ▶️ Rebuild: a fresh child at a fresh actor id (a real restart would restore-from-checkpoint
    // here — out of this test's scope, `GuestRuntime::checkpoint`/`restore` are proven separately
    // by `🧵️shard/🦀️.rs`'s K1 tests; this proves the PROCESS half of rebuild).
    let shard_a2 = ProcessTransport::spawn(Path::new(&shard_bin), &[wasm_path, "scale-fixture-a".to_string(), "3".to_string()]).await.expect("rebuild shard a");
    semio_framework_async::block_on(shard_a2.send(&instance_open_envelope(3, 1, "idle").await));
    let outcome_a2 = recv_outcome(&shard_a2, 400).await.expect("rebuilt shard a must reply");
    assert!(matches!(outcome_a2, crate::shard::ShardOutcome::Turn { actor: 3, .. }), "rebuilt shard a: expected ShardOutcome::Turn, got {outcome_a2:?}");

    // 🎯️ Sibling isolation: shard b, never touched, is still alive and answers a SECOND turn.
    assert!(shard_b.is_child_alive().await, "shard b must be unaffected by shard a's death");
    semio_framework_async::block_on(shard_b.send(&wake_envelope(2, 2).await));
    let outcome_b2 = recv_outcome(&shard_b, 400).await.expect("shard b must still respond after shard a's kill+rebuild");
    assert!(matches!(outcome_b2, crate::shard::ShardOutcome::Turn { actor: 2, .. }), "shard b second turn: expected ShardOutcome::Turn, got {outcome_b2:?}");

    semio_framework_async::block_on(shard_a2.kill());
    semio_framework_async::block_on(shard_b.kill());
}

/// terra-shard-grants: wraps in `crate::shard::ShardFrame::Envelope` — the wire now carries
/// `ShardFrame`, not raw `Envelope` bytes (`ShardLoop::pump`'s own change), so this fixture's
/// hand-rolled encoder must wrap here too, even though its one caller is `#[ignore]`d.
async fn encode_envelope(actor: u64, seq: u64, payload: semio_framework_actor::Payload) -> Vec<u8> {
    let envelope =
        semio_framework_actor::Envelope { to: semio_framework_actor::ActorId(actor), from: semio_framework_actor::Origin::Kernel, lane: semio_framework_actor::Lane::Interactive, seq, deadline_ms: None, coalesce: None, cancel_of: None, payload };
    let mut bytes = Vec::new();
    crate::shard::ShardFrame::Envelope(envelope).pack_encode(&mut bytes).await;
    bytes
}

async fn instance_open_envelope(actor: u64, seq: u64, profile: &str) -> Vec<u8> {
    let config = format!("{{\"profile\":\"{profile}\"}}").into_bytes();
    let event = semio_framework::kernel::Event::InstanceOpen {
        request: semio_framework::kernel::ActorInstanceOpenRequest { activation_generation: 1, instance_id: u32::try_from(actor).expect("fixture actor fits instance id"), request_sequence: seq },
        app_id: semio_framework::kernel::AppInstanceId("scale-fixture".to_string()),
        actor: actor.to_string(),
        config,
        assets: Vec::new(),
        capabilities: Vec::new(),
        quotas: semio_framework::kernel::QuotaSchema::default(),
    };
    let event_bytes = serde_json::to_vec(&event).expect("encode Event::InstanceOpen");
    encode_envelope(actor, seq, semio_framework_actor::Payload::Event { bytes: event_bytes }).await
}

async fn wake_envelope(actor: u64, seq: u64) -> Vec<u8> {
    let event_bytes = serde_json::to_vec(&semio_framework::kernel::Event::Wake).expect("encode Event::Wake");
    encode_envelope(actor, seq, semio_framework_actor::Payload::Event { bytes: event_bytes }).await
}

async fn recv_outcome(transport: &ProcessTransport, attempts: u32) -> Option<crate::shard::ShardOutcome> {
    for _ in 0..attempts {
        if let Some(bytes) = semio_framework_async::block_on(transport.recv()) {
            let mut pos = 0usize;
            return semio_framework_async::block_on(crate::shard::ShardOutcome::pack_decode(&bytes, &mut pos)).ok();
        }
        thread::sleep(Duration::from_millis(50));
    }
    None
}
//#endregion 🧪️KillRebuildEvidence
