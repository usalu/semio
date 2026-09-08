
use super::*;

#[semio_framework_async_macros::async_test]
async fn sha256_hex_matches_known_vector() {
    // "" -> e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855 (well-known empty-input SHA-256)
    assert_eq!(semio_framework_hash::sha256_hex(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
}

#[semio_framework_async_macros::async_test]
async fn raw_and_core_hashes_are_independent() {
    let (raw, core) = artifact_hashes(semio_framework_hash::sha256_hex(b"component"), semio_framework_hash::sha256_hex(b"core")).expect("distinct artifact hashes");
    assert_ne!(raw, core);
    let same = semio_framework_hash::sha256_hex(b"same");
    assert!(artifact_hashes(same.clone(), same).is_err());
}

#[semio_framework_async_macros::async_test]
async fn full_catalog_describe_retains_finite_wall_and_fuel_bounds() {
    assert_eq!(DESCRIBE_DEADLINE_MS, 1_800_000);
    assert_eq!(DESCRIBE_FUEL_BUDGET, 2_000_000_000);
}

#[semio_framework_async_macros::async_test]
async fn run_with_no_args_returns_usage_exit_code() {
    assert_eq!(run(Vec::new()).await, 2);
}

#[semio_framework_async_macros::async_test]
async fn run_with_unknown_command_returns_usage_exit_code() {
    assert_eq!(run(vec!["not-describe".to_string()]).await, 2);
}

#[semio_framework_async_macros::async_test]
async fn run_describe_without_core_flag_returns_usage_exit_code() {
    assert_eq!(run(vec!["describe".to_string(), "component.wasm".to_string(), "--out".to_string(), "/tmp/does-not-matter".to_string()]).await, 2);
}

#[semio_framework_async_macros::async_test]
async fn run_describe_on_missing_file_returns_failure_exit_code() {
    let code = run(vec!["describe".to_string(), "/nonexistent/component.wasm".to_string(), "--core".to_string(), "/nonexistent/core.wasm".to_string(), "--out".to_string(), "/tmp/does-not-matter".to_string()]).await;
    assert_eq!(code, 1);
}

mod long {
    use super::*;

    #[semio_framework_async_macros::async_test]
    async fn configured_real_component_suite_owned_descriptors_match_wasmtime() {
        let Some(paths) = std::env::var_os("SEMIO_OWNED_DIFFERENTIAL_FIXTURES") else { return };
        let paths = std::env::split_paths(&paths).collect::<Vec<_>>();
        assert!(!paths.is_empty(), "differential component suite is empty");
        for path in paths {
            let bytes = fs::read(&path).unwrap_or_else(|error| panic!("read differential component {}: {error}", path.display()));
            let oracle = execute_describe_wasmtime(&bytes, &path).await.unwrap_or_else(|error| panic!("execute Wasmtime descriptor oracle for {}: {error}", path.display()));
            let artifact = interpreter::SemioActorArtifact::parse(&bytes).unwrap_or_else(|error| panic!("parse owned Semio actor {}: {error}", path.display()));
            let mut session = interpreter::SemioDescribeSession::start(&artifact, 64 * 1024 * 1024).unwrap_or_else(|error| panic!("start owned describe session for {}: {error}", path.display()));
            let mut total_fuel = 0;
            let mut restored = false;
            let owned = loop {
                match session.step(50_000, interpreter::StepControl::default()) {
                    interpreter::SemioDescribeStepOutcome::Yield { fuel_used } => {
                        total_fuel += fuel_used;
                        assert!(total_fuel <= DESCRIBE_FUEL_BUDGET, "owned describe exceeded oracle fuel cap for {}", path.display());
                        if !restored {
                            let checkpoint = session.checkpoint();
                            session = interpreter::SemioDescribeSession::restore(&artifact, &checkpoint).unwrap_or_else(|error| panic!("restore owned describe checkpoint for {}: {error}", path.display()));
                            assert_eq!(session.checkpoint(), checkpoint, "checkpoint changed after restoring {}", path.display());
                            restored = true;
                        }
                    }
                    interpreter::SemioDescribeStepOutcome::Complete { fuel_used, descriptor } => {
                        total_fuel += fuel_used;
                        break descriptor;
                    }
                    interpreter::SemioDescribeStepOutcome::Cancelled { .. } => panic!("owned describe cancelled for {}", path.display()),
                    interpreter::SemioDescribeStepOutcome::Fault { error, .. } => panic!("owned describe fault for {}: {error}", path.display()),
                }
            };
            assert_eq!(owned, oracle, "owned descriptor differs from Wasmtime for {}", path.display());
            assert!(total_fuel > 0, "owned describe consumed no fuel for {}", path.display());
            let mut cancelled = interpreter::SemioDescribeSession::start(&artifact, 64 * 1024 * 1024).unwrap_or_else(|error| panic!("start cancellation probe for {}: {error}", path.display()));
            assert!(matches!(cancelled.step(1, interpreter::StepControl { cancelled: true }), interpreter::SemioDescribeStepOutcome::Cancelled { fuel_used: 0 }), "owned cancellation did not stop {} before its first instruction", path.display());
        }
    }
}
