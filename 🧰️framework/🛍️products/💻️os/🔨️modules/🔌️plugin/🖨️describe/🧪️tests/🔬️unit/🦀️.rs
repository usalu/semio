
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
    assert_eq!(DESCRIBE_FUEL_BUDGET, 8_000_000_000);
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

//#region 🔖️OwnedCoreExportLaw

/// 🧬️ The plugin SDK's own source — `plugin_exports!`, `__semio_plugin_actor_exports!` and
/// `extension_exports!` all live in this one file, so the law below reads their real bodies rather
/// than a restatement of them.
const PLUGIN_SDK_SOURCE: &str = include_str!("../../../🦀️.rs");

/// 🧱 The text of one `macro_rules!` body, brace-matched from its opening `{`.
fn macro_rules_body<'a>(source: &'a str, name: &str) -> &'a str {
    let header = format!("macro_rules! {name} {{");
    let start = source.find(&header).unwrap_or_else(|| panic!("`macro_rules! {name}` is not declared in the plugin SDK source")) + header.len();
    let mut depth = 1_usize;
    for (offset, character) in source[start..].char_indices() {
        match character {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return &source[start..start + offset];
                }
            }
            _ => {}
        }
    }
    panic!("`macro_rules! {name}` is unbalanced in the plugin SDK source")
}

/// 🔍️ One macro arm's text, from its pattern header to the next arm's header.
fn macro_arm<'a>(body: &'a str, pattern: &str) -> &'a str {
    let header = format!("{pattern} => {{");
    let start = body.find(&header).unwrap_or_else(|| panic!("macro arm `{pattern}` is not declared")) + header.len();
    let rest = &body[start..];
    let end = rest.find("        };\n        (").map_or(rest.len(), |offset| offset);
    &rest[..end]
}

/// 🏷️ Every `semio_owned_*_v1` symbol a chunk of source defines as a core export.
fn owned_export_definitions(source: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut rest = source;
    while let Some(offset) = rest.find("extern \"C\" fn semio_owned_") {
        let tail = &rest[offset + "extern \"C\" fn ".len()..];
        let end = tail.find('(').expect("owned core export declaration has an argument list");
        names.push(tail[..end].to_string());
        rest = &tail[end..];
    }
    names.sort();
    names
}

/// 🧩️ The owned core exports an owner entry point reaches, following the one indirection each arm
/// uses — `__semio_owned_core_exports!` directly, or `__semio_plugin_actor_exports!` which invokes it.
fn owned_exports_reached_by(arm: &str) -> Vec<String> {
    let core = macro_rules_body(PLUGIN_SDK_SOURCE, "__semio_owned_core_exports");
    let direct = arm.contains("__semio_owned_core_exports!(");
    let indirect = arm.contains("__semio_plugin_actor_exports!(") && macro_rules_body(PLUGIN_SDK_SOURCE, "__semio_plugin_actor_exports").contains("__semio_owned_core_exports!(");
    assert!(direct || indirect, "owner arm reaches no `__semio_owned_core_exports!` invocation:\n{arm}");
    owned_export_definitions(core)
}

/// 🛡️ Every component owner exports the SAME thirteen `semio_owned_*_v1` core symbols, from ONE
/// shared definition, and that set is exactly the one `OwnedSemioArtifact::from_component` demands.
/// The last four are the owned twin of `world actor`'s `codec` interface (ticket 26/09/18 slice
/// TC3b) — a component that carries the other nine but not those cannot answer a headless hub's
/// creation or validation call, so the ABI admits it as a whole or not at all.
///
/// 🐛️ Regression guard for `📓️a3-descriptor-regeneration.md` §3b: `extension_exports!`'s
/// single-argument arm used to expand `__semio_actor_exports!` directly and emit none of them,
/// so every extension component failed owned-ABI validation and no extension could be described.
/// Copying them into the extension arm would have satisfied a symbol-set assertion while
/// leaving two bodies free to drift, so the law also refuses any second definition of them.
#[semio_framework_async_macros::async_test]
async fn owned_core_exports_are_defined_once_and_invoked_by_both_owners() {
    let mut required: Vec<String> = interpreter::OwnedSemioExport::ALL.iter().map(|export| export.core_name().to_string()).collect();
    required.sort();
    assert_eq!(required.len(), 13, "the owned Semio actor ABI is thirteen core exports");

    let defined_in_shared_macro = owned_export_definitions(macro_rules_body(PLUGIN_SDK_SOURCE, "__semio_owned_core_exports"));
    assert_eq!(defined_in_shared_macro, required, "the shared owned-core-export macro does not define exactly `OwnedSemioExport::ALL`");
    assert_eq!(owned_export_definitions(PLUGIN_SDK_SOURCE), required, "a `semio_owned_*_v1` core export is defined outside `__semio_owned_core_exports!` — the arms must share one body, never a copy");

    let extension = macro_rules_body(PLUGIN_SDK_SOURCE, "extension_exports");
    let plugin_arm = owned_exports_reached_by(macro_rules_body(PLUGIN_SDK_SOURCE, "plugin_exports"));
    let extension_with_apps_arm = owned_exports_reached_by(macro_arm(extension, "($bundle_fn:expr, $plugin_fn:expr, $app:ty)"));
    let extension_only_arm = owned_exports_reached_by(macro_arm(extension, "($bundle_fn:expr)"));

    assert_eq!(plugin_arm, required, "`plugin_exports!` does not reach every owned core export");
    assert_eq!(extension_with_apps_arm, plugin_arm, "`extension_exports!`'s three-argument arm exports a different core symbol set than `plugin_exports!`");
    assert_eq!(extension_only_arm, plugin_arm, "`extension_exports!`'s single-argument arm exports a different core symbol set than `plugin_exports!`");
}
//#endregion 🔖️OwnedCoreExportLaw

/// 🪪️ The emitter classifies a declared-but-unowned kind by the plugin crate's own fault text; the law keeps the two in step.
#[semio_framework_async_macros::async_test]
async fn unowned_codec_schema_fault_is_the_plugin_crates_own_text() {
    assert!(PLUGIN_SDK_SOURCE.contains(&format!("plugin_internal_fault(\"{UNOWNED_ARTIFACT_CODEC_SCHEMA}\")")), "the plugin crate no longer faults with UNOWNED_ARTIFACT_CODEC_SCHEMA");
}
