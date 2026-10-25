//! 🔬️ Cargo tests of the test-runner crate. The frozen planning, detection and cancellation vectors are
//! replayed; the seven report dialects and the scope selectors are judged against
//! `🧫️fixtures/🔬️unit-vectors.json` (an independent Python model); `libtest` itself, reached through the
//! running test binary, is the real oracle of the cargo dialect.

use super::*;
use semio_framework_test_fixtures::fixture_path;
use std::path::PathBuf;

//#region 📥️Fixtures
fn fixture(name: &str) -> Value {
    let path = fixture_path!("..").join("..").join("🧪️tests").join("🧫️fixtures").join(name);
    let bytes = std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_slice(&bytes).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn rows<'a>(value: &'a Value, key: &str) -> &'a Vec<Value> {
    value[key].as_array().unwrap_or_else(|| panic!("{key} is not an array"))
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_else(|| panic!("member {key} is not a string in {value}"))
}

fn typed<T: serde::de::DeserializeOwned>(value: &Value) -> T {
    serde_json::from_value(value.clone()).unwrap_or_else(|error| panic!("{error}: {value}"))
}

fn as_value<T: Serialize>(value: &T) -> Value {
    serde_json::to_value(value).unwrap()
}

fn strings(items: &[&str]) -> Vec<String> {
    items.iter().map(|item| item.to_string()).collect()
}
//#endregion 📥️Fixtures

//#region 🗺️Planning
#[test]
fn every_frozen_planning_vector_plans_exactly_its_expected_argv() {
    let vectors = fixture("🗺️planning-vectors.json");
    let snapshot: FilesystemSnapshot = typed(&vectors["snapshot"]);
    assert!(rows(&vectors, "vectors").len() >= 17);
    for vector in rows(&vectors, "vectors") {
        let scope: TestScope = typed(&vector["scope"]);
        let expected: InvocationPlan = typed(&vector["expected"]);
        assert_eq!(plan_scope(&snapshot, &scope), expected, "{}", text(vector, "id"));
    }
}

#[test]
fn planning_a_list_of_scopes_concatenates_the_plans_in_order() {
    let vectors = fixture("🗺️planning-vectors.json");
    let snapshot: FilesystemSnapshot = typed(&vectors["snapshot"]);
    let scopes: Vec<TestScope> = rows(&vectors, "vectors").iter().take(6).map(|vector| typed(&vector["scope"])).collect();
    let together = plan_scopes(&snapshot, &scopes);
    let mut expected = InvocationPlan::default();
    for scope in &scopes {
        expected.extend(plan_scope(&snapshot, scope));
    }
    assert_eq!(together, expected);
    assert!(plan_scopes(&snapshot, &[]).invocations.is_empty());
}

#[test]
fn narrowing_a_scope_never_widens_the_plan() {
    let vectors = fixture("🗺️planning-vectors.json");
    let snapshot: FilesystemSnapshot = typed(&vectors["snapshot"]);
    let width = |id: &str| {
        let vector = rows(&vectors, "vectors").iter().find(|vector| text(vector, "id") == id).unwrap();
        plan_scope(&snapshot, &typed(&vector["scope"])).invocations.len()
    };
    let widths: Vec<usize> = ["whole-repository", "technology-semio", "bundle-go", "file-go-narrows-to-its-directory"].iter().map(|id| width(id)).collect();
    assert!(widths.windows(2).all(|pair| pair[0] >= pair[1]), "{widths:?}");
    assert_eq!(widths[widths.len() - 1], 1);
}

fn world(entries: &[(&str, bool, &str)], bundles: &[(&str, &str)], uv: bool) -> FilesystemSnapshot {
    FilesystemSnapshot {
        root: "/w".into(),
        entries: entries.iter().map(|(path, directory, contents)| SnapshotEntry { path: path.to_string(), kind: if *directory { EntryKind::Directory } else { EntryKind::File }, contents: if contents.is_empty() { None } else { Some(contents.to_string()) } }).collect(),
        bundles: bundles.iter().map(|(name, root)| SnapshotBundle { name: name.to_string(), root: root.to_string() }).collect(),
        uv_available: uv,
    }
}

#[test]
fn a_python_bundle_runs_through_uv_when_it_exists_and_through_pytest_when_it_does_not() {
    let entries = [("/w/py", true, ""), ("/w/py/pyproject.toml", false, "[project]")];
    for (uv, runner, argv) in [(true, Runner::Uv, vec!["uv", "run", "pytest", "-k", "alpha"]), (false, Runner::Pytest, vec!["pytest", "-k", "alpha"])] {
        let snapshot = world(&entries, &[("t/py", "py")], uv);
        let plan = plan_bundle(&snapshot, "py", "", "", "alpha");
        assert_eq!((plan.invocations[0].runner, plan.invocations[0].argv.clone(), plan.invocations[0].cwd.as_str()), (runner, strings(&argv), "/w/py"));
        assert_eq!(plan.invocations[0].filter.as_deref(), Some("alpha"));
    }
}

#[test]
fn every_language_plans_its_documented_runner_with_a_filter() {
    let snapshot = world(
        &[("/w/g", true, ""), ("/w/g/go.mod", false, "module g"), ("/w/r", true, ""), ("/w/r/Cargo.toml", false, "[package]"), ("/w/c", true, ""), ("/w/c/App.csproj", false, "<Project/>"), ("/w/j", true, ""), ("/w/j/package.json", false, "{\"scripts\":{\"test\":\"x\"}}")],
        &[("t/g", "g"), ("t/r", "r"), ("t/c", "c"), ("t/j", "j")],
        false,
    );
    let argv = |root: &str, filter: &str| plan_bundle(&snapshot, root, "", "", filter).invocations[0].argv.clone();
    assert_eq!(argv("g", "TestX"), strings(&["go", "test", "-run", "TestX", "./..."]));
    assert_eq!(argv("r", "case"), strings(&["cargo", "test", "case"]));
    assert_eq!(argv("c", "Case"), strings(&["dotnet", "test", "--filter", "Case"]));
    assert_eq!(argv("j", ""), strings(&["npm", "test"]));
    assert_eq!(plan_bundle(&snapshot, "g", "", "sub", "").invocations[0].argv, strings(&["go", "test", "./sub/..."]));
    let refused = plan_bundle(&snapshot, "missing", "", "", "");
    assert_eq!((refused.invocations.len(), refused.problems), (0, vec!["unknown language \"\" for bundle missing".to_string()]));
}

#[test]
fn the_javascript_runner_follows_the_manifest_text_and_a_filter_only_appends() {
    let snapshot = world(
        &[("/w/v/package.json", false, "{\"devDependencies\":{\"vitest\":\"3\"}}"), ("/w/j/package.json", false, "{\"devDependencies\":{\"jest\":\"29\"}}"), ("/w/n/package.json", false, "{\"scripts\":{\"test\":\"node --test\"}}"), ("/w/e/package.json", false, "{}")],
        &[],
        false,
    );
    for (root, program, args) in [("/w/v", "npx", vec!["vitest", "run"]), ("/w/j", "npx", vec!["jest"]), ("/w/n", "npm", vec!["test"]), ("/w/e", "npx", vec!["vitest", "run"]), ("/w/none", "npx", vec!["vitest", "run"])] {
        let (runner, found) = detect_js_test_runner(&snapshot, root, "");
        assert_eq!((runner.program(), found.clone()), (program, strings(&args)), "{root}");
        let (_, filtered) = detect_js_test_runner(&snapshot, root, "renders");
        assert_eq!(&filtered[..found.len()], &found[..]);
    }
}

#[test]
fn a_section_of_a_go_file_selects_the_tests_between_its_region_markers() {
    let content = "package p\n// #region Alpha\nfunc TestOne(t *testing.T) {}\nfunc helper() {}\nfunc TestTwo(t *testing.T) {}\n// #endregion Alpha\n// #region 🚀️Beta\nfunc TestThree(t *testing.T) {}\n// #endregion Beta\n";
    assert_eq!(collect_go_tests_in_section(content, "Alpha"), "^(TestOne|TestTwo)$");
    assert_eq!(collect_go_tests_in_section(content, "beta"), "^(TestThree)$");
    assert_eq!(collect_go_tests_in_section(content, "Gamma"), "");
    assert_eq!(collect_go_tests_in_section("", "Alpha"), "");
    for (value, stripped) in [("🚀Gamma", "Gamma"), ("🚀️Gamma", "Gamma"), ("Gamma", "Gamma"), ("👩‍💻 Gamma", "Gamma"), ("", ""), ("🚀", ""), ("1🚀Gamma", "Gamma")] {
        assert_eq!(strip_leading_grapheme(value), stripped, "{value:?}");
    }
}

#[test]
fn a_flat_test_name_resolves_to_the_declared_name_or_is_re_cased_from_its_prefix() {
    let content = "func TestAlphaOne(t *testing.T) {}\nfunc BenchmarkAlpha(b *testing.B) {}\nfunc FuzzParse(f *testing.F) {}\nfunc notATest() {}\nfunc TestNoParen\n";
    assert_eq!(resolve_test_function_name(content, "testalphaone"), "TestAlphaOne");
    assert_eq!(resolve_test_function_name(content, "benchmarkalpha"), "BenchmarkAlpha");
    assert_eq!(resolve_test_function_name(content, "fuzzparse"), "FuzzParse");
    assert_eq!(resolve_test_function_name(content, "testnoparen"), "");
    for (flat_name, cased) in [("testalphaone", "TestAlphaone"), ("benchmarkx", "BenchmarkX"), ("fuzzy", "FuzzY"), ("testbenchmarkz", "TestbenchmarkZ"), ("other", "Other"), ("test", "Test"), ("", "")] {
        assert_eq!(unflatten_test_name(flat_name), cased, "{flat_name:?}");
    }
}

#[test]
fn a_file_scope_narrows_by_language_and_a_definition_scope_by_the_flat_name() {
    let snapshot = world(
        &[
            ("/w/py", true, ""), ("/w/py/pyproject.toml", false, "x"), ("/w/py/tests/test_a.py", false, "x"), ("/w/c", true, ""), ("/w/c/A.csproj", false, "x"), ("/w/c/T.cs", false, "x"), ("/w/r", true, ""), ("/w/r/Cargo.toml", false, "x"), ("/w/r/lib.rs", false, "x"),
        ],
        &[("t/py", "py"), ("t/c", "c"), ("t/r", "r")],
        true,
    );
    assert_eq!(plan_file(&snapshot, "py/tests/test_a.py", "", "").invocations[0].argv, strings(&["uv", "run", "pytest", "tests/test_a.py"]));
    assert_eq!(plan_file(&snapshot, "py/tests/test_a.py", "", "alpha").invocations[0].argv, strings(&["uv", "run", "pytest", "tests/test_a.py", "-k", "alpha"]));
    assert_eq!(plan_file(&snapshot, "c/T.cs", "", "Case").invocations[0].argv, strings(&["dotnet", "test", "--filter", "Case"]));
    assert_eq!(plan_file(&snapshot, "r/lib.rs", "", "").invocations[0].argv, strings(&["cargo", "test"]));
    assert_eq!(plan_file(&snapshot, "r/notes.txt", "", "").problems, vec!["unsupported language \"\" for file test".to_string()]);
    assert_eq!(plan_definition(&snapshot, "py/tests/test_a.py", "", "", "test_one").invocations[0].argv, strings(&["uv", "run", "pytest", "-k", "test_one"]));
    assert_eq!(plan_definition(&snapshot, "c/T.cs", "", "", "testcase").invocations[0].argv, strings(&["dotnet", "test", "--filter", "FullyQualifiedName~TestCase"]));
    assert_eq!(plan_section(&snapshot, "c/T.cs", "", "Any").invocations[0].argv, strings(&["dotnet", "test"]), "only Go narrows to a section");
    assert_eq!(runner_programs(), vec!["go", "cargo", "cargo", "dotnet", "npx", "npm", "uv", "pytest", "rspec"]);
}

fn runner_programs() -> Vec<&'static str> {
    [Runner::Go, Runner::Cargo, Runner::CargoNextest, Runner::Dotnet, Runner::Npx, Runner::Npm, Runner::Uv, Runner::Pytest, Runner::Rspec].iter().map(|runner| runner.program()).collect()
}
//#endregion 🗺️Planning

//#region 🧭️Detection
#[test]
fn every_frozen_detection_vector_names_its_language_and_javascript_runner() {
    let vectors = fixture("🧭️detection-vectors.json");
    let snapshot: FilesystemSnapshot = typed(&vectors["snapshot"]);
    for vector in rows(&vectors, "vectors") {
        let bundle_root = vector["bundleRoot"].as_str().unwrap_or_default();
        assert_eq!(detect_bundle_language(&snapshot, bundle_root), text(vector, "expectedLanguage"), "{}", text(vector, "id"));
        if let Some(expected) = vector["expectedArgv"].as_array() {
            let (_, argv) = detect_js_test_runner(&snapshot, &snapshot.absolute(bundle_root), vector["testFilter"].as_str().unwrap_or_default());
            assert_eq!(argv, expected.iter().map(|item| item.as_str().unwrap().to_string()).collect::<Vec<_>>(), "{}", text(vector, "id"));
        }
    }
}

#[test]
fn the_manifest_order_decides_when_a_bundle_carries_several() {
    let snapshot = world(&[("/w/b/go.mod", false, "x"), ("/w/b/Cargo.toml", false, "x"), ("/w/b/package.json", false, "x")], &[], false);
    assert_eq!(detect_bundle_language(&snapshot, "b"), "go");
    let without_go = world(&[("/w/b/Cargo.toml", false, "x"), ("/w/b/package.json", false, "x")], &[], false);
    assert_eq!(detect_bundle_language(&without_go, "b"), "rust");
    let dotnet = world(&[("/w/b/App.sln", false, "x"), ("/w/b/package.json", false, "x")], &[], false);
    assert_eq!(detect_bundle_language(&dotnet, "b"), "csharp");
    let python = world(&[("/w/b/requirements.txt", false, "x")], &[], false);
    assert_eq!(detect_bundle_language(&python, "b"), "python");
    assert_eq!(detect_bundle_language(&world(&[], &[], false), "b"), "");
    assert_eq!(language_of_extension("x/A.TSX"), Some("typescript"));
    assert_eq!(language_of_extension("x/noext"), None);
}
//#endregion 🧭️Detection

//#region 🛑️Execution
#[test]
fn cancelling_after_the_recorded_count_keeps_the_completed_outcomes_and_skips_the_rest() {
    let vectors = fixture("🛑️cancellation-vectors.json");
    let plan: InvocationPlan = typed(&vectors["plan"]);
    let runner = RecordedProcessRunner::new(typed(&vectors["transcripts"]));
    let cancel_after = vectors["cancelAfter"].as_u64().unwrap() as usize;
    let token = CancellationToken::new();
    let mut events: Vec<ProgressEvent> = Vec::new();
    let report = execute_plan(&plan, &runner, &token, &mut |event| {
        if matches!(event, ProgressEvent::InvocationFinished { .. }) && events.iter().filter(|seen| matches!(seen, ProgressEvent::InvocationFinished { .. })).count() + 1 >= cancel_after {
            token.cancel();
        }
        events.push(event);
    });
    assert!(report.cancelled && token.is_cancelled());
    assert_eq!((report.completed, report.outcomes.len(), report.total), (cancel_after, cancel_after, plan.invocations.len()));
    assert!(matches!(events.first(), Some(ProgressEvent::PlanStarted { total }) if *total == plan.invocations.len()));
    assert!(matches!(events.last(), Some(ProgressEvent::Cancelled { completed }) if *completed == cancel_after));
    assert!(!events.iter().any(|event| matches!(event, ProgressEvent::PlanFinished { .. })));
}

#[test]
fn an_uncancelled_run_reports_before_and_after_every_invocation() {
    let vectors = fixture("🛑️cancellation-vectors.json");
    let plan: InvocationPlan = typed(&vectors["plan"]);
    let runner = RecordedProcessRunner::new(typed(&vectors["transcripts"]));
    let mut events: Vec<ProgressEvent> = Vec::new();
    let report = execute_plan(&plan, &runner, &CancellationToken::new(), &mut |event| events.push(event));
    assert_eq!(events.len(), 2 + 2 * plan.invocations.len());
    assert!(!report.cancelled && report.completed == plan.invocations.len() && report.problems.is_empty());
    let kinds: Vec<&str> = events.iter().map(|event| match event {
        ProgressEvent::PlanStarted { .. } => "plan-started",
        ProgressEvent::InvocationStarted { .. } => "started",
        ProgressEvent::InvocationFinished { .. } => "finished",
        ProgressEvent::Cancelled { .. } => "cancelled",
        ProgressEvent::PlanFinished { .. } => "plan-finished",
    }).collect();
    assert_eq!(kinds.first(), Some(&"plan-started"));
    assert_eq!(kinds.last(), Some(&"plan-finished"));
    assert!(kinds[1..kinds.len() - 1].chunks(2).all(|pair| pair == ["started", "finished"]));
    let encoded = progress_to_json_text(&events);
    assert!(encoded.contains("\"event\":\"plan-started\"") && encoded.contains("\"event\":\"invocation-finished\""), "{encoded}");
}

#[test]
fn a_missing_transcript_is_a_problem_and_the_invocation_is_not_run() {
    let vectors = fixture("🛑️cancellation-vectors.json");
    let plan: InvocationPlan = typed(&vectors["plan"]);
    let report = execute_plan(&plan, &RecordedProcessRunner::default(), &CancellationToken::new(), &mut |_| {});
    assert_eq!(report.problems.len(), plan.invocations.len());
    assert!(report.problems.iter().all(|problem| problem.starts_with("no recorded transcript for ")));
    assert!(report.outcomes.iter().all(|outcome| outcome.status == RunStatus::NotRun && outcome.tests.is_empty()));
    assert_eq!(report.completed, plan.invocations.len());
    let carried = execute_plan(&InvocationPlan { invocations: Vec::new(), problems: vec!["refused".to_string()] }, &RecordedProcessRunner::default(), &CancellationToken::new(), &mut |_| {});
    assert_eq!((carried.problems, carried.completed, carried.total), (vec!["refused".to_string()], 0, 0));
}

#[test]
fn a_recorded_runner_matches_the_argv_and_the_directory_in_order() {
    let transcript = |argv: &[&str], cwd: &str, status: i32| RecordedTranscript { argv: strings(argv), cwd: cwd.to_string(), output: ProcessOutput { status, stdout: String::new(), stderr: String::new() } };
    let runner = RecordedProcessRunner::new(vec![transcript(&["go", "test"], "/a", 1), transcript(&["go", "test"], "", 2), transcript(&["cargo", "test"], "/b/./c", 3)]);
    let request = |program: &str, args: &[&str], cwd: &str| ProcessRequest { program: program.into(), args: strings(args), cwd: cwd.into(), env: BTreeMap::new() };
    assert_eq!(runner.run(&request("go", &["test"], "/a")).unwrap().status, 1);
    assert_eq!(runner.run(&request("go", &["test"], "/elsewhere")).unwrap().status, 2, "an empty cwd answers any directory");
    assert_eq!(runner.run(&request("cargo", &["test"], "/b/c")).unwrap().status, 3, "directories compare cleaned");
    assert_eq!(runner.run(&request("cargo", &["test", "x"], "/b/c")).unwrap_err(), "no recorded transcript for cargo test x");
}

#[test]
fn a_cancellation_token_is_shared_between_its_clones() {
    let token = CancellationToken::new();
    let clone = token.clone();
    assert!(!token.is_cancelled());
    clone.cancel();
    assert!(token.is_cancelled() && clone.is_cancelled());
}
//#endregion 🛑️Execution

//#region 📊️Parsing
fn parsed_answers() -> Value {
    fixture("🔬️unit-vectors.json")
}

fn vector_output(vector: &Value) -> ProcessOutput {
    typed(&vector["output"])
}

#[test]
fn every_recorded_and_extra_transcript_parses_like_the_independent_model() {
    let answers = parsed_answers();
    let mut vectors: Vec<Value> = rows(&fixture("📜️runner-transcripts.json"), "vectors").clone();
    vectors.extend(rows(&answers, "extra").iter().cloned());
    assert!(vectors.len() >= 20);
    for vector in &vectors {
        let id = text(vector, "id");
        let expected = rows(&answers, "parsed").iter().find(|row| text(row, "id") == id).unwrap_or_else(|| panic!("no answer for {id}"));
        let runner: Runner = typed(&vector["runner"]);
        let outcome = parse_outcome(runner, &vector_output(vector));
        assert_eq!(as_value(&outcome), expected["outcome"], "{id}");
        assert_eq!(serde_json::from_str::<Value>(&outcome_to_json_text(&outcome)).unwrap(), expected["outcome"], "{id}");
    }
}

#[test]
fn totals_are_recounted_from_the_cases_and_the_run_fails_on_a_failing_case_or_a_bad_exit() {
    let answers = parsed_answers();
    for row in rows(&answers, "parsed") {
        let outcome = &row["outcome"];
        let tests = outcome["tests"].as_array().unwrap();
        let count = |status: &str| tests.iter().filter(|test| test["status"] == status).count() as u64;
        assert_eq!((outcome["totals"]["passed"].as_u64().unwrap(), outcome["totals"]["failed"].as_u64().unwrap(), outcome["totals"]["skipped"].as_u64().unwrap(), outcome["totals"]["total"].as_u64().unwrap()), (count("passed"), count("failed"), count("skipped"), tests.len() as u64), "{}", text(row, "id"));
        assert_eq!(outcome["status"] == "failed", count("failed") > 0 || outcome["exit_status"] != 0, "{}", text(row, "id"));
    }
}

#[test]
fn an_npm_or_uv_invocation_reports_itself_as_the_runner_not_the_dialect_it_was_parsed_with() {
    let output = ProcessOutput { status: 0, stdout: "{\"testResults\":[]}".into(), stderr: String::new() };
    assert_eq!(parse_outcome(Runner::Npm, &output).runner, Runner::Npm);
    assert_eq!(parse_outcome(Runner::Npx, &output).runner, Runner::Npx);
    assert_eq!(parse_outcome(Runner::Uv, &output).runner, Runner::Uv);
    assert_eq!(parse_outcome(Runner::Pytest, &output).runner, Runner::Pytest);
}

#[test]
fn a_cargo_failure_body_stops_before_the_failures_summary_and_the_next_body() {
    let output = ProcessOutput { status: 101, stdout: "test a ... FAILED\ntest b ... FAILED\n\nfailures:\n\n---- a stdout ----\nfirst body\nline two\n\n---- b stdout ----\nsecond body\n\n\nfailures:\n    a\n    b\n\ntest result: FAILED. 0 passed; 2 failed\n".into(), stderr: String::new() };
    let outcome = parse_cargo_test(&output);
    assert_eq!(outcome.tests[0].message, "first body\nline two");
    assert_eq!(outcome.tests[1].message, "second body");
}

#[test]
fn the_dialect_parsers_are_reachable_by_name_and_agree_with_the_dispatcher() {
    let output = ProcessOutput { status: 0, stdout: "{\"Action\":\"pass\",\"Package\":\"p\",\"Test\":\"T\",\"Elapsed\":0.5}\n".into(), stderr: String::new() };
    assert_eq!(parse_go_test_json(&output), parse_outcome(Runner::Go, &output));
    assert_eq!(parse_outcome(Runner::Go, &output).tests[0].duration_ms, Some(500.0));
    let empty = ProcessOutput::default();
    for outcome in [parse_vitest_json(&empty), parse_pytest(&empty), parse_cargo_test(&empty), parse_cargo_nextest(&empty), parse_dotnet_test(&empty), parse_rspec_json(&empty)] {
        assert_eq!((outcome.tests.len(), outcome.status), (0, RunStatus::Passed));
    }
}
//#endregion 📊️Parsing

//#region 🦀️RealLibtest
#[test]
fn libtest_probe_passes() {}

#[test]
fn libtest_probe_fails() {
    if std::env::var_os("SEMIO_LIBTEST_PROBE").is_some() {
        panic!("probe failure message");
    }
}

#[test]
#[ignore]
fn libtest_probe_ignored() {}

fn spawn_self(arguments: &[&str]) -> ProcessOutput {
    let executable = std::env::current_exe().expect("test binary").to_string_lossy().to_string();
    let env = BTreeMap::from([("SEMIO_LIBTEST_PROBE".to_string(), "1".to_string())]);
    SystemProcessRunner.run(&ProcessRequest { program: executable, args: strings(arguments), cwd: String::new(), env }).expect("the test binary starts")
}

#[test]
fn the_cargo_parser_reads_what_the_real_libtest_prints() {
    let passed = spawn_self(&["--exact", "tests::libtest_probe_passes", "--test-threads=1"]);
    let outcome = parse_cargo_test(&passed);
    assert_eq!((outcome.status, outcome.totals), (RunStatus::Passed, TestTotals { passed: 1, failed: 0, skipped: 0, total: 1 }));
    assert_eq!(outcome.tests[0].name, "tests::libtest_probe_passes");

    let failed = spawn_self(&["--exact", "tests::libtest_probe_fails", "--test-threads=1"]);
    let outcome = parse_cargo_test(&failed);
    assert_eq!((outcome.status, outcome.exit_status), (RunStatus::Failed, Some(101)));
    assert_eq!(outcome.tests[0].status, TestStatus::Failed);
    assert!(outcome.tests[0].message.contains("probe failure message"), "{}", outcome.tests[0].message);
    assert!(!outcome.tests[0].message.contains("failures:") && !outcome.tests[0].message.contains("test result"), "the failure body swallowed the summary: {}", outcome.tests[0].message);

    let skipped = spawn_self(&["--exact", "tests::libtest_probe_ignored", "--test-threads=1"]);
    let outcome = parse_cargo_test(&skipped);
    assert_eq!(outcome.totals, TestTotals { passed: 0, failed: 0, skipped: 1, total: 1 });
}
//#endregion 🦀️RealLibtest

//#region 🧮️Paths
#[test]
fn path_arithmetic_follows_go_filepath() {
    for (input, cleaned) in [("", "."), (".", "."), ("a/./b//c/", "a/b/c"), ("a/../b", "b"), ("../a", "../a"), ("/a/../..", "/"), ("/", "/"), ("C:\\x\\y\\..\\z", "C:/x/z"), ("c:/", "c:/"), ("a\\b", "a/b"), ("/a/b/..", "/a")] {
        assert_eq!(clean_path(input), cleaned, "{input:?}");
    }
    assert_eq!(join_path("/a/", "b/../c"), "/a/c");
    assert_eq!((join_path("", "x/./y"), join_path("/a", "")), ("x/y".to_string(), "/a".to_string()));
    for (input, base, dir) in [("/a/b/c.go", "c.go", "/a/b"), ("c.go", "c.go", "."), ("/a/b/", "b", "/a"), ("/", "/", "/"), ("a/b", "b", "a"), ("/x", "x", "/")] {
        assert_eq!((base_name(input), dir_name(input)), (base.to_string(), dir.to_string()), "{input:?}");
    }
    assert_eq!((relative_path("/a", "/a/b/c"), relative_path("/a", "/a"), relative_path("/a", "/x/y"), relative_path("/a/", "/a/b")), ("b/c".to_string(), ".".to_string(), "/x/y".to_string(), "b".to_string()));
    assert!(is_absolute_path("/a") && is_absolute_path("C:\\a") && is_absolute_path("d:/x") && !is_absolute_path("a/b") && !is_absolute_path("C:") && !is_absolute_path("1:/x"));
    assert_eq!(normalize_separators("a\\b\\c"), "a/b/c");
}

#[test]
fn every_selector_flattens_and_decodes_like_the_independent_model() {
    let answers = parsed_answers();
    assert!(rows(&answers, "selectors").len() >= 25);
    for row in rows(&answers, "selectors") {
        let selector = text(row, "selector");
        assert_eq!(flat(selector), text(row, "flat"), "flat {selector:?}");
        assert_eq!(path_from_uri_path(selector), text(row, "path"), "path {selector:?}");
        assert_eq!(PendingIdentity.flat(selector), flat(selector));
        assert_eq!(PendingIdentity.path_from_uri_path(selector), path_from_uri_path(selector));
    }
}
//#endregion 🧮️Paths

//#region 🌲️Snapshot
fn sample_snapshot() -> FilesystemSnapshot {
    world(
        &[
            ("/w/pkg", true, ""), ("/w/pkg/a_test.go", false, "package pkg"), ("/w/pkg/sub", true, ""), ("/w/pkg/sub/b_test.go", false, "package sub"), ("/w/pkg/c.go", false, "package pkg"), ("/w/pkg/m.csproj", false, "x"), ("/w/pkg/n.csproj", false, "x"), ("/w/pkg/App.sln", false, "x"),
            ("/w/src", true, ""), ("/w/src/lib.rs", false, "#[cfg(test)]\nmod t {}"), ("/w/src/other.rs", false, "fn x() {}"), ("/w/py", true, ""), ("/w/py/test_x.py", false, "x"), ("/w/py/y_test.py", false, "x"), ("/w/py/conftest.py", false, "x"), ("/w/py/z.py", false, "x"),
            ("/w/js", true, ""), ("/w/js/a.test.ts", false, "x"), ("/w/js/b.spec.js", false, "x"), ("/w/js/c.ts", false, "x"), ("/w/rb", true, ""), ("/w/rb/x_spec.rb", false, "x"), ("/w/cs", true, ""), ("/w/cs/T.cs", false, "[Fact]\npublic void A() {}"), ("/w/cs/U.cs", false, "class U {}"),
        ],
        &[("tech/pkg", "pkg"), ("tech/deep", "pkg/sub"), ("@scope/App", "app")],
        false,
    )
}

#[test]
fn a_snapshot_answers_existence_listing_and_walking_questions_like_the_filesystem_it_stands_for() {
    let snapshot = sample_snapshot();
    assert!(snapshot.file_exists("/w/pkg/a_test.go") && !snapshot.file_exists("/w/pkg") && snapshot.directory_exists("/w/pkg") && !snapshot.directory_exists("/w/pkg/a_test.go"));
    assert!(snapshot.file_exists("/w/pkg/./sub/../a_test.go"), "paths compare cleaned");
    assert_eq!(snapshot.read_text("/w/src/lib.rs"), Some("#[cfg(test)]\nmod t {}"));
    assert_eq!(snapshot.read_text("/w/pkg/m.csproj"), Some("x"));
    assert_eq!(snapshot.read_text("/w/missing"), None);
    assert_eq!(snapshot.glob_children("/w/pkg", "*.csproj"), vec!["/w/pkg/m.csproj", "/w/pkg/n.csproj"]);
    assert!(snapshot.glob_children("/w/pkg", "*.sln").len() == 1 && snapshot.glob_children("/w/pkg", "*.fsproj").is_empty());
    assert_eq!(snapshot.walk_files("/w/pkg", true).len(), 6);
    assert_eq!(snapshot.walk_files("/w/pkg", false), vec!["/w/pkg/App.sln", "/w/pkg/a_test.go", "/w/pkg/c.go", "/w/pkg/m.csproj", "/w/pkg/n.csproj"]);
    assert_eq!(snapshot.absolute("pkg/x"), "/w/pkg/x");
    assert_eq!(snapshot.absolute("/elsewhere/./x"), "/elsewhere/x");
}

#[test]
fn bundles_resolve_by_exact_or_flattened_name_and_by_the_longest_root() {
    let snapshot = sample_snapshot();
    assert_eq!(snapshot.bundle_by_name("tech/pkg").map(|bundle| bundle.root.as_str()), Some("pkg"));
    assert_eq!(snapshot.bundle_by_name("TECH/Pkg").map(|bundle| bundle.root.as_str()), Some("pkg"), "flattened equality");
    assert!(snapshot.bundle_by_name("tech/none").is_none());
    assert_eq!(snapshot.bundle_by_path("pkg/sub/b_test.go").map(|bundle| bundle.name.as_str()), Some("tech/deep"));
    assert_eq!(snapshot.bundle_by_path("pkg/c.go").map(|bundle| bundle.name.as_str()), Some("tech/pkg"));
    assert_eq!(snapshot.bundle_by_path("pkg").map(|bundle| bundle.name.as_str()), Some("tech/pkg"));
    assert!(snapshot.bundle_by_path("pkgs/x").is_none());
}

#[test]
fn scope_selectors_resolve_to_their_granularity() {
    let snapshot = sample_snapshot();
    let resolve = |raw: &str| resolve_test_scope(raw, &snapshot, &PendingIdentity, &PendingIdentity);
    assert_eq!(resolve("repo://p/🧰/tech"), TestScope { kind: ScopeKind::Technology, bundle_root: "tech".into(), ..TestScope::default() });
    let bundle = resolve("repo://p/🧰/tech/b/📚/pkg");
    assert_eq!((bundle.kind, bundle.bundle_root.as_str(), bundle.language.as_str()), (ScopeKind::Bundle, "pkg", "csharp"));
    let scoped = resolve("repo://p/🧰/@scope/b/📚/App");
    assert_eq!((scoped.kind, scoped.bundle_root.as_str(), scoped.language.as_str()), (ScopeKind::Bundle, "app", ""), "a leading @ is dropped and the flattened name finds the bundle");
    let file = resolve("repo://f/x.go");
    assert_eq!((file.kind, file.file_path.as_str(), file.language.as_str()), (ScopeKind::File, "x.go", "go"));
    let section = resolve("repo://f/py%20x.py/s/Alpha");
    assert_eq!((section.kind, section.file_path.as_str(), section.section.as_str(), section.language.as_str()), (ScopeKind::Section, "py x.py", "Alpha", "python"));
    let definition = resolve("repo://f/x.go/d/🧪/testone");
    assert_eq!((definition.kind, definition.test_name.as_str(), definition.language.as_str()), (ScopeKind::Definition, "testone", "go"));
    let stripped = resolve("repo\u{FE0F}://f/x.go");
    assert_eq!(stripped.file_path, "x.go", "variation selectors are removed first");
    for raw in ["", "notavalidid", "repo://unknown/x"] {
        assert_eq!(resolve(raw), TestScope::all(), "{raw:?}");
    }
    assert_eq!(resolve_test_scopes(&[], &snapshot, &PendingIdentity, &PendingIdentity), vec![TestScope::all()]);
    assert_eq!(resolve_test_scopes(&strings(&["repo://f/x.rs", ""]), &snapshot, &PendingIdentity, &PendingIdentity).len(), 2);
    assert_eq!(PendingIdentity.id_to_uri("anything"), "");
}

#[test]
fn a_bundle_sub_path_narrows_in_order_and_ignores_unknown_markers() {
    let snapshot = sample_snapshot();
    let scope = resolve_test_scope_from_bundle_sub_path("pkg", "f/sub%20x/a_test.go/s/Beta/d/🧪/testone", &snapshot, &PendingIdentity, &PendingIdentity);
    assert_eq!(scope.kind, ScopeKind::Definition);
    assert_eq!(scope.test_name, "testone");
    assert_eq!(scope.section, "Beta");
    assert_eq!(scope.file_path, "pkg/sub x", "the file route takes one segment and joins it onto the bundle root");
    let bare = resolve_test_scope_from_bundle_sub_path("pkg", "zzz/fd/x/y", &snapshot, &PendingIdentity, &PendingIdentity);
    assert_eq!(bare.kind, ScopeKind::Bundle);
    let file_scope = resolve_test_scope_from_file_sub_path("a.go", "go", "s/One/d/🧪/testtwo", &PendingIdentity);
    assert_eq!((file_scope.kind, file_scope.section.as_str(), file_scope.test_name.as_str(), file_scope.language.as_str()), (ScopeKind::Definition, "One", "testtwo", "go"));
    assert_eq!(resolve_test_scope_from_file_sub_path("a.go", "go", "unknown", &PendingIdentity).kind, ScopeKind::File);
}
//#endregion 🌲️Snapshot

//#region 🔎️Resolution
#[test]
fn a_command_is_classified_as_test_search_edit_or_terminal() {
    for (command, kind) in [
        ("", CommandKind::Terminal), ("go test ./...", CommandKind::Test), ("cargo test -p x", CommandKind::Test), ("cargo nextest run", CommandKind::Test), ("npm test", CommandKind::Test), ("pytest -k x", CommandKind::Test),
        ("uv run pytest", CommandKind::Test), ("make test", CommandKind::Test), ("jest", CommandKind::Test), ("dotnet test", CommandKind::Test), ("ls -la", CommandKind::CodeSearch), ("grep foo bar", CommandKind::CodeSearch),
        ("echo hi", CommandKind::CodeSearch), ("test -f x", CommandKind::CodeSearch), ("[ -f x ]", CommandKind::CodeSearch), ("rm -rf x", CommandKind::CodeEdit), ("mkdir -p x", CommandKind::CodeEdit), ("sed -i s/a/b/ f", CommandKind::CodeEdit),
        ("sed -n 1p f", CommandKind::CodeSearch), ("git status", CommandKind::Terminal), ("make build", CommandKind::Terminal), ("dotnet build", CommandKind::Terminal), ("cd x && go test ./...", CommandKind::Test), ("cat a | grep b", CommandKind::CodeSearch),
        ("ls && rm x", CommandKind::CodeEdit), ("cd x && ls", CommandKind::CodeSearch), ("cd x && git pull", CommandKind::Terminal), ("/usr/bin/pytest", CommandKind::Test),
    ] {
        assert_eq!(classify_command_kind(command), kind, "{command:?}");
    }
}

#[test]
fn a_command_splits_on_unquoted_separators_only() {
    assert_eq!(split_command_segments("a; b && c | d || e"), vec!["a", "b", "c", "d", "e"]);
    assert_eq!(split_command_segments("echo \"a;b\" && ls 'x|y'"), vec!["echo \"a;b\"", "ls 'x|y'"]);
    assert_eq!(split_command_segments("a &b"), vec!["a &b"]);
    assert!(split_command_segments("   ").is_empty() && split_command_segments(";;").is_empty());
    assert_eq!(trim_pipeline_tail("go test ./... | tee out"), "go test ./...");
    assert_eq!(trim_pipeline_tail("echo \"a|b\" | cat"), "echo \"a|b\"");
    assert_eq!(trim_pipeline_tail("  "), "");
}

#[test]
fn the_test_segment_and_its_directory_are_extracted_from_a_composite_command() {
    for (command, segment, cwd) in [
        ("cd sub && go test ./... | tee out", "go test ./...", "sub"), ("export A=1; cargo test", "cargo test", ""), ("npx vitest run", "npx vitest run", ""), ("ls", "", ""), ("cd x && ls", "", "x"), ("", "", ""),
        ("set -e && cd a && cd b && pytest", "pytest", "b"), ("uv run pytest -x", "uv run pytest -x", ""), ("yarn test", "yarn test", ""),
    ] {
        assert_eq!(extract_test_segment_from_command(command), (segment.to_string(), cwd.to_string()), "{command:?}");
    }
    assert!(is_test_command_segment("npx jest", "") && is_test_command_segment("uv run pytest", "") && !is_test_command_segment("npx tsc", "") && !is_test_command_segment("", "") && !is_test_command_segment("uv", ""));
}

#[test]
fn a_test_command_resolves_to_the_files_it_would_run() {
    let snapshot = sample_snapshot();
    let files = |command: &str, cwd: &str| resolve_test_files_from_command(&snapshot, command, cwd);
    assert_eq!(files("go test ./pkg/...", "/w"), vec!["/w/pkg/a_test.go", "/w/pkg/sub/b_test.go"]);
    assert_eq!(files("go test ./pkg", "/w"), vec!["/w/pkg/a_test.go"]);
    assert_eq!(files("go test", "/w"), vec!["/w/pkg/a_test.go", "/w/pkg/sub/b_test.go"]);
    assert_eq!(files("go test ./pkg/a_test.go", "/w"), vec!["/w/pkg/a_test.go"]);
    assert_eq!(files("go test -v ./pkg/sub", "/w"), vec!["/w/pkg/sub/b_test.go"]);
    assert_eq!(files("cargo test", "/w"), vec!["/w/src/lib.rs"]);
    assert_eq!(files("dotnet test", "/w"), vec!["/w/cs/T.cs"]);
    assert_eq!(files("pytest", "/w"), vec!["/w/py/conftest.py", "/w/py/test_x.py", "/w/py/y_test.py"]);
    assert_eq!(files("python3 -m pytest", "/w"), vec!["/w/py/test_x.py", "/w/py/y_test.py"]);
    assert_eq!(files("uv run pytest", "/w"), files("pytest", "/w"));
    assert_eq!(files("cd py && pytest", "/w"), files("pytest", "/w"));
    assert_eq!(files("npm test", "/w"), vec!["/w/js/a.test.ts", "/w/js/b.spec.js"]);
    assert_eq!(files("npx vitest", "/w"), files("npm test", "/w"));
    assert!(files("npx tsc", "/w").is_empty() && files("make", "/w").is_empty() && files("", "/w").is_empty());
    assert_eq!(files("bundle exec rspec", "/w"), vec!["/w/rb/x_spec.rb"]);
    assert_eq!(files("rspec", "/w"), files("bundle exec rspec", "/w"));
    assert_eq!(resolve_pytest_files(&snapshot, &strings(&["py/test_x.py", "-k", "alpha"]), "/w"), vec!["/w/py/test_x.py"]);
    assert_eq!(resolve_pytest_files(&snapshot, &strings(&["py"]), "/w").len(), 3);
    assert_eq!(resolve_pytest_files(&snapshot, &strings(&["py/z.py"]), "/w"), Vec::<String>::new());
}
//#endregion 🔎️Resolution

//#region 💽️RealWorld
struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Scratch {
        let path = std::env::temp_dir().join(format!("semio-repo-test-runner-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        Scratch(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

fn write(root: &Path, relative: &str, contents: &str) {
    let path = root.join(relative);
    std::fs::create_dir_all(path.parent().unwrap()).unwrap();
    std::fs::write(path, contents).unwrap();
}

#[test]
fn walking_a_real_repository_records_every_kept_path_and_only_the_manifests_contents() {
    let scratch = Scratch::new("walk");
    write(&scratch.0, "go/go.mod", "module g\n");
    write(&scratch.0, "go/a_test.go", "package g\n");
    write(&scratch.0, "web/package.json", "{\"devDependencies\":{\"vitest\":\"1\"}}");
    write(&scratch.0, "web/src/app.ts", "export {};\n");
    write(&scratch.0, "ignored/x.ts", "x");
    write(&scratch.0, ".git/config", "[core]");
    let ignore = semio_framework_repo_workspace::GitIgnore::compile_lines(["ignored/"]);
    let snapshot = FilesystemSnapshot::from_root(&scratch.0, Some(&ignore));
    let relative: Vec<String> = snapshot.entries.iter().map(|entry| relative_path(&snapshot.root, &entry.path)).collect();
    assert!(!relative.contains(&"ignored/x.ts".to_string()) && !relative.iter().any(|path| path.starts_with(".git")), "{relative:?}");
    for kept in ["go", "go/a_test.go", "go/go.mod", "web", "web/package.json", "web/src", "web/src/app.ts"] {
        assert!(relative.contains(&kept.to_string()), "{kept} is missing from {relative:?}");
    }
    let mut sorted = relative.clone();
    sorted.sort();
    assert_eq!(relative, sorted);
    assert_eq!(snapshot.read_text(&snapshot.absolute("web/package.json")), Some("{\"devDependencies\":{\"vitest\":\"1\"}}"));
    assert_eq!(snapshot.read_text(&snapshot.absolute("go/a_test.go")), None, "only manifests are read by the walk");
    assert_eq!(detect_bundle_language(&snapshot, "go"), "go");
    assert_eq!(detect_bundle_language(&snapshot, "web"), "typescript");
    let mut hydrated = snapshot.clone();
    hydrated.hydrate("go/a_test.go");
    assert_eq!(hydrated.read_text(&hydrated.absolute("go/a_test.go")), Some("package g\n"));
    hydrated.hydrate("go/new_file_test.go");
    assert_eq!(hydrated.entries.len(), snapshot.entries.len(), "a path that is not on disk is not invented");
    assert!(FilesystemSnapshot::from_root(&scratch.0.join("missing"), None).entries.is_empty());
}

#[test]
fn a_bundle_snapshot_lists_only_the_bundle_roots_and_their_direct_children() {
    let scratch = Scratch::new("bundles");
    write(&scratch.0, "go/go.mod", "module g\n");
    write(&scratch.0, "go/deep/inner/x.go", "package x\n");
    write(&scratch.0, "web/package.json", "{}");
    let bundles = vec![SnapshotBundle { name: "t/go".into(), root: "go".into() }, SnapshotBundle { name: "t/web".into(), root: "web".into() }, SnapshotBundle { name: "t/gone".into(), root: "gone".into() }];
    let snapshot = FilesystemSnapshot::for_bundles(&scratch.0, bundles.clone());
    let relative: Vec<String> = snapshot.entries.iter().map(|entry| relative_path(&snapshot.root, &entry.path)).collect();
    assert_eq!(relative, vec!["go", "go/deep", "go/go.mod", "gone", "web", "web/package.json"]);
    assert_eq!(snapshot.read_text(&snapshot.absolute("web/package.json")), Some("{}"));
    assert_eq!(snapshot.bundles, bundles);
    assert_eq!(plan_all(&snapshot).invocations.len(), 2);
    let rebuilt = FilesystemSnapshot::default().with_bundles(bundles.clone());
    assert_eq!(rebuilt.bundles, bundles);
    let from_model: SnapshotBundle = (&semio_framework_repo_model::Bundle { name: "t/x".into(), root: "x".into(), source_root: String::new(), technology_name: "t".into(), tags: Vec::new(), kind: semio_framework_repo_model::BundleKind::Library, emoji: String::new(), packages: Vec::new() }).into();
    assert_eq!(from_model, SnapshotBundle { name: "t/x".into(), root: "x".into() });
}

#[test]
fn the_system_runner_captures_both_streams_and_the_exit_code_of_a_real_process() {
    let output = spawn_self(&["--exact", "tests::libtest_probe_passes", "--test-threads=1"]);
    assert_eq!(output.status, 0);
    assert!(output.stdout.contains("test tests::libtest_probe_passes ... ok"), "{}", output.stdout);
    let failing = spawn_self(&["--exact", "tests::libtest_probe_fails", "--test-threads=1"]);
    assert_eq!(failing.status, 101);
    let missing = SystemProcessRunner.run(&ProcessRequest { program: "semio-no-such-program-anywhere".into(), args: Vec::new(), cwd: String::new(), env: BTreeMap::new() });
    assert!(missing.unwrap_err().starts_with("semio-no-such-program-anywhere: "));
    assert!(!program_on_path("semio-no-such-program-anywhere"));
}

#[test]
fn an_invocation_spawned_with_the_parents_streams_returns_the_exit_code_and_prints_its_running_line() {
    let executable = std::env::current_exe().unwrap().to_string_lossy().replace('\\', "/");
    let invocation = RunnerInvocation { runner: Runner::Cargo, argv: vec![executable.clone(), "--exact".into(), "tests::libtest_probe_passes".into(), "--test-threads=1".into(), "--quiet".into()], cwd: String::new(), env: BTreeMap::new(), filter: None };
    assert_eq!(run_invocation_inherited(&invocation).unwrap(), 0);
    let line = running_line(&RunnerInvocation { cwd: "/w/x".into(), ..invocation });
    assert!(line.starts_with("Running: ") && line.ends_with(&format!(" (in {})\n", "/w/x".replace('/', std::path::MAIN_SEPARATOR_STR))), "{line}");
    assert!(run_invocation_inherited(&RunnerInvocation { runner: Runner::Go, argv: vec!["semio-no-such-program-anywhere".into()], cwd: String::new(), env: BTreeMap::new(), filter: None }).is_err());
}
//#endregion 💽️RealWorld

//#region 🪞️JsonText
#[test]
fn plans_scopes_and_reports_render_as_json_text_that_parses_back() {
    let plan = InvocationPlan { invocations: vec![RunnerInvocation { runner: Runner::CargoNextest, argv: strings(&["cargo", "nextest", "run"]), cwd: "/w".into(), env: BTreeMap::from([("A".to_string(), "1".to_string())]), filter: Some("f".into()) }], problems: strings(&["p"]) };
    assert_eq!(serde_json::from_str::<InvocationPlan>(&plan_to_json_text(&plan)).unwrap(), plan);
    assert!(plan_to_json_text(&plan).contains("\"runner\":\"cargo-nextest\""));
    let scope = TestScope { kind: ScopeKind::Definition, test_name: "t".into(), ..TestScope::default() };
    assert_eq!(serde_json::from_str::<TestScope>(&scope_to_json_text(&scope)).unwrap(), scope);
    assert!(scope_to_json_text(&scope).contains("\"kind\":\"definition\""));
    let report = ExecutionReport { cancelled: true, completed: 1, total: 2, ..ExecutionReport::default() };
    assert_eq!(serde_json::from_str::<ExecutionReport>(&report_to_json_text(&report)).unwrap(), report);
}
//#endregion 🪞️JsonText
