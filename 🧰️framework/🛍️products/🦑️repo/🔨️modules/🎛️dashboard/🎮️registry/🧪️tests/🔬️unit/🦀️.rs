use super::*;

const FIXTURE: &str = include_str!("../../../🧫️fixtures/🎮️registry/🏗️workspace.json");

fn materialise(name: &str) -> PathBuf { materialise_text(name, FIXTURE) }

fn materialise_text(name: &str, fixture: &str) -> PathBuf {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let root = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(std::env::temp_dir, PathBuf::from).join(format!("semio-registry-{name}-{nanos}"));
    let document: serde_json::Value = serde_json::from_str(fixture).unwrap();
    for file in document["files"].as_array().unwrap() {
        let target = file["path"].as_str().unwrap().split('/').fold(root.clone(), |path, part| path.join(part));
        std::fs::create_dir_all(target.parent().unwrap()).unwrap();
        let text = file.get("json").map_or_else(|| file["text"].as_str().unwrap().to_string(), |json| serde_json::to_string_pretty(json).unwrap());
        std::fs::write(target, text).unwrap();
    }
    root
}

fn fixture(name: &str) -> (PathBuf, Registry) {
    let root = materialise(name);
    let registry = crate::inventory::discover(&root, &std::sync::atomic::AtomicBool::new(false));
    (root, registry)
}

fn request(parameters: &[(&str, &str)], args: &[&str], env: &[(&str, &str)]) -> Request {
    let pairs = |pairs: &[(&str, &str)]| pairs.iter().map(|(key, value)| (key.to_string(), value.to_string())).collect();
    Request { parameters: pairs(parameters), args: args.iter().map(ToString::to_string).collect(), env: pairs(env) }
}

fn resolve(registry: &Registry, id: &str, request: &Request) -> Launch { registry.resolve_with(id, request, RunPolicy::default()).unwrap_or_else(|error| panic!("{id}: {error}")) }

fn env_of<'a>(process: &'a LaunchProcess, key: &str) -> Option<&'a str> { process.env.iter().find(|(name, _)| name == key).map(|(_, value)| value.as_str()) }

#[test]
fn identities_round_trip_and_name_what_is_wrong() {
    for id in ["a:build", "a/b:build:production", "playground:s", "tool:workspace/bun-test", "compound:w/c", "group:w/g", "ticket:26/09/23/SLUG/probe", "repo:goals.list", "repo:analyze:rust", "repo:ticket.show:26/09/23/X"] {
        assert_eq!(CommandId::parse(id).unwrap().to_string(), id);
    }
    assert!(CommandId::parse("script:build").is_ok_and(|id| matches!(id, CommandId::Target { .. })), "`script` is an ordinary project name now");
    assert!(CommandId::parse("tool:no-slash").unwrap_err().contains("tool:<project>/<id>"));
    assert!(CommandId::parse("ticket:26/9/23/X/p").is_err());
    assert!(CommandId::parse("two words").is_err());
    assert!(CommandId::parse("build").is_err());
}

#[test]
fn verbs_come_from_the_leading_word_with_a_fallback() {
    assert_eq!(verb_of(None, "test-quick"), "test");
    assert_eq!(verb_of(None, "dev"), "dev");
    assert_eq!(verb_of(None, "materialize-dev"), FALLBACK_VERB);
    assert_eq!(verb_of(Some("clean"), "erase"), "clean");
    assert!(verb_rank("dev") < verb_rank("test") && verb_rank("test") < verb_rank(FALLBACK_VERB) && verb_rank(FALLBACK_VERB) < verb_rank("goals") && verb_rank("goals") < verb_rank("unheard-of"));
}

#[test]
fn the_fixture_workspace_lists_every_source_once_and_reports_no_problem() {
    let (_, registry) = fixture("ids");
    assert_eq!(registry.problems(), &[] as &[Problem]);
    let ids: Vec<&str> = registry.entries().iter().map(|entry| entry.id.as_str()).collect();
    for expected in ["workspace:dev", "workspace:build", "hub:dev", "hub:build", "@fixture/quiz:dev", "@fixture/quiz:test-e2e", "@fixture/artifact-wav:check", "playground:shell", "playground:cad", "tool:workspace/inspector", "tool:workspace/bun-test", "compound:workspace/dev-shell-with-hub", "group:workspace/artifact-tests", "ticket:26/09/23/OPEN-PROBE/probe", "ticket:26/09/23/OPEN-PROBE/probe-with-hub", "repo:goals.list", "repo:analyze:rust", "repo:ticket.show:26/09/23/OPEN-PROBE"] {
        assert!(ids.contains(&expected), "missing {expected}");
    }
    assert!(!ids.iter().any(|id| id.starts_with("script:")), "root scripts are not a command source");
    assert!(!ids.iter().any(|id| id.starts_with("@fixture/vanished")), "a graph project whose folder is gone is not offered");
    assert!(!ids.contains(&"ticket:26/08/01/CLOSED-PROBE/stale"), "a closed ticket contributes no tools");
    let unique: HashSet<&str> = ids.iter().copied().collect();
    assert_eq!(unique.len(), ids.len());
    assert_eq!(registry.search(&["stale"], false), Vec::<usize>::new());
}

#[test]
fn a_target_resolves_axes_then_parameters_then_extra_arguments_after_the_double_dash() {
    let (root, registry) = fixture("target");
    let launch = resolve(&registry, "workspace:test", &request(&[("test-level", "quick"), ("cache", "skip-local"), ("dependencies", "true"), ("cargo-jobs", "2")], &["--filter", "x"], &[("EXTRA", "1")]));
    let process = &launch.processes[0];
    assert_eq!((process.cmd.as_str(), process.cwd.as_path()), ("bun", root.as_path()));
    assert_eq!(process.args, ["nx", "run", "workspace:test", "--skip-nx-cache", "--excludeTaskDependencies", "--", "--filter", "x"]);
    assert_eq!((env_of(process, "SEMIO_TEST_LEVEL"), env_of(process, "CARGO_BUILD_JOBS"), env_of(process, "EXTRA")), (Some("quick"), Some("2"), Some("1")));
    assert_eq!((env_of(process, "NX_TUI"), env_of(process, "NX_NATIVE_COMMAND_RUNNER")), (Some("false"), Some("false")));
    assert_eq!(launch.label, TaskLabel { verb: "test".into(), owner: vec!["workspace".into()], subject: "workspace".into(), qualifier: String::new(), parameters: vec![("cache".into(), "skip-local".into()), ("test-level".into(), "quick".into()), ("dependencies".into(), "true".into()), ("cargo-jobs".into(), "2".into())], members: 0 });
}

#[test]
fn a_required_parameter_must_be_chosen_and_a_choice_must_be_one_of_its_values() {
    let (_, registry) = fixture("required");
    assert!(registry.resolve_with("workspace:verify", &Request::default(), RunPolicy::default()).unwrap_err().contains("\"rule\" is required"));
    assert!(registry.resolve_with("workspace:verify", &request(&[("rule", "nope")], &[], &[]), RunPolicy::default()).unwrap_err().contains("choose dependencies, literal-external"));
    assert_eq!(resolve(&registry, "workspace:verify", &request(&[("rule", "literal-external")], &[], &[])).processes[0].args, ["nx", "run", "workspace:verify", "--", "dependencies", "literal-external"]);
    assert!(registry.resolve_with("workspace:verify", &request(&[("rule", "dependencies"), ("ghost", "1")], &[], &[]), RunPolicy::default()).unwrap_err().contains("unknown parameter \"ghost\""));
}

#[test]
fn a_ready_port_comes_from_the_declaration_or_the_resolved_environment_and_is_exported() {
    let (_, registry) = fixture("ready");
    let hub = resolve(&registry, "hub:dev", &Request::default());
    assert_eq!(hub.processes[0].ready, Some(Ready { port: 8787, path: "/admin".into(), printed: false }));
    assert_eq!(env_of(&hub.processes[0], "OS_HUB_PORT"), Some("8787"));
    let quiz = resolve(&registry, "@fixture/quiz:dev", &request(&[("listen", "6070")], &[], &[]));
    assert_eq!(quiz.processes[0].ready.as_ref().map(|ready| ready.port), Some(6070));
    assert_eq!(env_of(&quiz.processes[0], "PROCTOR_URL"), Some("http://127.0.0.1:6070/proctor"));
    assert!(resolve(&registry, "hub:dev", &request(&[("backend", "postgres")], &[], &[])).processes[0].args.contains(&"--migrate".to_string()));
    assert_eq!(env_of(&resolve(&registry, "hub:dev", &request(&[("backend", "postgres")], &[], &[])).processes[0], "OS_HUB_PORT"), Some("8788"));
}

#[test]
fn a_printed_ready_takes_the_whole_printed_address() {
    let (_, registry) = fixture("printed");
    let launch = resolve(&registry, "tool:workspace/printer", &Request::default());
    assert_eq!(launch.processes[0].ready, Some(Ready { port: 6280, path: String::new(), printed: true }));
    assert_eq!(registry.launch_plan_json(&launch)["processes"][0]["ready"], serde_json::json!({ "port": 6280, "path": "", "printed": true }));
    let facts = Facts { projects: vec![ProjectFacts { name: "p".into(), root: ".".into(), manifest: "📋️project.json".into(), dashboard: Some(serde_json::json!({ "tools": [{ "id": "t", "command": ["x"], "ready": { "port": 1, "printed": true, "path": "/x" } }] })), targets: Vec::new() }], ..Default::default() };
    assert!(Registry::build(Path::new("/w"), &facts).problems().iter().any(|problem| problem.message.contains("`printed` takes the whole printed address")));
}

#[test]
fn requires_pin_parameters_and_environment_in_object_form() {
    let (root, registry) = fixture("requires");
    let launch = resolve(&registry, "tool:workspace/printer", &Request::default());
    assert_eq!(launch.requires.len(), 1);
    let hub = &launch.requires[0].processes[0];
    assert_eq!((hub.command_id.as_str(), env_of(hub, "OS_HUB_BACKEND"), env_of(hub, "HUB_NOTE")), ("hub:dev", Some("postgres"), Some(format!("{}/note", root.display()).as_str())));
    let quiz = resolve(&registry, "@fixture/quiz:dev", &Request::default());
    assert_eq!(quiz.requires.iter().map(|need| need.command_id.as_str()).collect::<Vec<_>>(), ["hub:dev", "tool:workspace/inspector"]);
    assert_eq!(quiz.requires[1].requires.iter().map(|need| need.command_id.as_str()).collect::<Vec<_>>(), ["hub:dev"], "requirements nest");
}

#[test]
fn a_compound_starts_its_members_in_order_with_pins_and_member_environment() {
    let (root, registry) = fixture("compound");
    let launch = resolve(&registry, "compound:workspace/dev-shell-with-hub", &Request::default());
    assert_eq!((launch.stop, launch.group.as_deref()), (Stop::Together, Some("compound:workspace/dev-shell-with-hub")));
    assert_eq!(launch.processes.iter().map(|process| process.command_id.as_str()).collect::<Vec<_>>(), ["hub:dev", "playground:shell"]);
    let shell = &launch.processes[1];
    assert_eq!(env_of(shell, "S_HUB_URL"), Some("http://127.0.0.1:8787"));
    assert_eq!(env_of(shell, "S_NOTE"), Some(format!("{}/shell-note", root.display()).as_str()));
    assert!(registry.resolve_with("compound:workspace/dev-shell-with-hub", &request(&[], &["x"], &[]), RunPolicy::default()).unwrap_err().contains("takes no extra arguments"));
    let users = resolve(&registry, "compound:workspace/users", &Request::default());
    assert_eq!(env_of(&users.processes[0], "OS_HUB_BACKEND"), Some("postgres"));
    assert_eq!(users.processes[1].ready.as_ref().map(|ready| ready.port), Some(6072));
    assert_eq!(users.processes[2].ready.as_ref().map(|ready| ready.port), Some(6073));
    assert_eq!(env_of(&users.processes[1], "S_HUB_URL"), Some("http://127.0.0.1:8787"));
    assert_eq!(env_of(&users.processes[2], "S_HUB_URL"), None);
}

#[test]
fn a_group_runs_several_targets_over_nx_project_patterns_unchanged() {
    let (_, registry) = fixture("group");
    let tests = resolve(&registry, "group:workspace/artifact-tests", &request(&[("test-level", "long")], &[], &[]));
    assert_eq!(tests.processes[0].args, ["nx", "run-many", "-t", "test", "-p", "@fixture/artifact-*,!@fixture/artifact-contract"]);
    assert_eq!(env_of(&tests.processes[0], "SEMIO_TEST_LEVEL"), Some("long"));
    let both = resolve(&registry, "group:workspace/all-builds", &Request::default());
    assert_eq!(both.processes[0].args, ["nx", "run-many", "-t", "build,test", "-p", "hub,@fixture/quiz"]);
    assert_eq!(registry.find("group:workspace/all-builds").map(|entry| (entry.verb.as_str(), entry.qualifier.as_str())), Some(("build", "build+test")));
}

#[test]
fn global_axes_select_tools_by_verb_and_nx_flags_do_not_reach_them() {
    let (_, registry) = fixture("tool-axes");
    let tool = registry.find("tool:workspace/bun-test").unwrap();
    let ids: Vec<String> = registry.parameters(tool).iter().map(|parameter| parameter.id.clone()).collect();
    assert_eq!(ids, ["test-level", "build-mode", "cargo-jobs", "project", "file", "bail"]);
    let launch = resolve(&registry, "tool:workspace/bun-test", &request(&[("file", "a.test.ts"), ("build-mode", "ship"), ("test-level", "quick"), ("bail", "true")], &[], &[]));
    let process = &launch.processes[0];
    assert_eq!(process.args, ["nx", "exec", "--projects=workspace", "--", "bun", "test", "a.test.ts", "--bail"]);
    assert_eq!((env_of(process, "SEMIO_BUILD_MODE"), env_of(process, "SEMIO_TEST_LEVEL"), env_of(process, "BUN_TEST_BAIL")), (Some("ship"), Some("quick"), Some("workspace")));
    assert!(!process.args.iter().any(|word| word.starts_with("--skip")), "nxFlags of an axis are ignored on a tool");
    let viewer = registry.find("tool:workspace/viewer").unwrap();
    assert!(registry.parameters(viewer).is_empty());
}

#[test]
fn free_extra_environment_reaches_every_process_and_never_the_runner_variables() {
    let (_, registry) = fixture("env");
    let launch = resolve(&registry, "compound:workspace/dev-shell-with-hub", &request(&[], &[], &[("TRACE_ID", "7")]));
    assert!(launch.processes.iter().all(|process| env_of(process, "TRACE_ID") == Some("7")));
    assert!(launch.requires.iter().all(|need| need.processes.iter().all(|process| env_of(process, "TRACE_ID").is_none())));
    for (key, value) in [("NX_DAEMON", "true"), ("1BAD", "x"), ("A", "x\0y")] {
        assert!(registry.resolve_with("workspace:build", &request(&[], &[], &[(key, value)]), RunPolicy::default()).is_err(), "{key}");
    }
}

#[test]
fn extra_environment_values_expand_the_registry_tokens() {
    let (root, registry) = fixture("env-tokens");
    let launch = resolve(&registry, "hub:dev", &request(&[("backend", "postgres")], &[], &[("NOTE", "{workspace}/n-{backend}-{port}")]));
    assert_eq!(env_of(&launch.processes[0], "NOTE"), Some(format!("{}/n-postgres-8788", root.display()).as_str()));
    assert_eq!(env_of(&resolve(&registry, "hub:dev", &request(&[], &[], &[("JSON", "{\"a\":1}")])).processes[0], "JSON"), Some("{\"a\":1}"), "braces that are no token stay");
    assert!(registry.resolve_with("hub:dev", &request(&[], &[], &[("X", "{nothing}")]), RunPolicy::default()).unwrap_err().contains("`{nothing}` names no chosen parameter"));
    assert!(registry.resolve_with("workspace:build", &request(&[], &[], &[("X", "{port}")]), RunPolicy::default()).unwrap_err().contains("needs a command with a ready port"));
}

#[test]
fn the_ready_port_reads_the_final_environment_including_extra_environment_and_effects() {
    let (_, registry) = fixture("final-env");
    let port = |id: &str, request: &Request| resolve(&registry, id, request).processes[0].ready.as_ref().map(|ready| ready.port);
    assert_eq!(port("hub:dev", &Request::default()), Some(8787));
    assert_eq!(port("hub:dev", &request(&[], &[], &[("OS_HUB_PORT", "9001")])), Some(9001), "an extra environment value moves the ready port");
    assert_eq!(env_of(&resolve(&registry, "hub:dev", &request(&[], &[], &[("OS_HUB_PORT", "9001")])).processes[0], "OS_HUB_PORT"), Some("9001"));
    assert_eq!(port("hub:dev", &request(&[("backend", "postgres")], &[], &[])), Some(8788), "so does a parameter effect");
    assert_eq!(port("hub:dev", &request(&[("backend", "postgres")], &[], &[("OS_HUB_PORT", "9002")])), Some(9002), "and the extra value wins over the effect");
    assert_eq!(port("@fixture/quiz:dev", &request(&[], &[], &[("QUIZ_PORT", "6099")])), Some(6099), "a parameter's variable too");
    assert_eq!(port("playground:cad", &request(&[], &[], &[("S_OS_PORT", "6555")])), Some(6555), "a playground binds S_OS_PORT");
    assert!(registry.resolve_with("hub:dev", &request(&[], &[], &[("OS_HUB_PORT", "no")]), RunPolicy::default()).unwrap_err().contains("is not a port"));
}

#[test]
fn a_playground_pins_the_app_of_its_catalog_row_for_every_door() {
    let (_, registry) = fixture("app");
    for renderer in ["react", "wgpu-wasm", "wgpu-native"] {
        let process = &resolve(&registry, "playground:shell", &request(&[("renderer", renderer)], &[], &[])).processes[0];
        assert_eq!((env_of(process, "SEMIO_APP_ID"), env_of(process, "VITE_SEMIO_APP_ID")), (Some("s.space.shell@1/*#editor"), Some("s.space.shell@1/*#editor")), "{renderer}");
    }
    assert_eq!(env_of(&resolve(&registry, "playground:cad", &Request::default()).processes[0], "SEMIO_APP_ID"), None, "a row without an app pins none");
}

#[test]
fn the_check_reports_ports_that_commands_of_different_owners_claim_together_and_not_alternatives() {
    let tool = |id: &str, port: u32| serde_json::json!({ "id": id, "command": ["x"], "continuous": true, "ready": { "port": port } });
    let project = |name: &str, root: &str, dashboard: serde_json::Value| ProjectFacts { name: name.into(), root: root.into(), manifest: format!("{root}/📋️project.json"), dashboard: Some(dashboard), targets: vec![TargetFacts { name: "dev".into(), continuous: Some(true), dashboard: Some(serde_json::json!({ "ready": { "port": 7001 } })), ..Default::default() }, TargetFacts { name: "dev-b".into(), continuous: Some(true), dashboard: Some(serde_json::json!({ "ready": { "port": 7001 } })), ..Default::default() }] };
    let facts = Facts { projects: vec![
        project("a", "a", serde_json::json!({ "tools": [tool("one", 7002), tool("two", 7002)] })),
        project("b", "b", serde_json::json!({ "tools": [tool("three", 7002), tool("clash", 7001)] })),
    ], playgrounds: vec![PlaygroundFacts { variant: "p".into(), plugin: "x".into(), react: 7100, wgpu: 7101, user_react: vec![7002], ..Default::default() }], ..Default::default() };
    let messages: Vec<String> = Registry::build(Path::new("/w"), &facts).check().iter().map(|problem| problem.message.clone()).collect();
    let claims: Vec<&String> = messages.iter().filter(|message| message.starts_with("port ")).collect();
    assert_eq!(claims.len(), 2, "{messages:#?}");
    assert!(claims.iter().any(|message| message.starts_with("port 7001 ") && message.contains("a:dev") && message.contains("tool:b/clash")), "{claims:?}");
    assert!(claims.iter().any(|message| message.starts_with("port 7002 ") && message.contains("tool:a/one") && message.contains("tool:b/three") && message.contains("playground:p")), "{claims:?}");
    assert!(claims.iter().all(|message| !message.contains("a:dev,") || message.contains("tool:b/clash") || message.contains("b:dev")), "targets of one project are alternatives and are not reported against each other");
}

#[test]
fn the_check_reports_a_playground_renderer_whose_nx_target_the_graph_lacks() {
    let owner = |targets: &[&str]| ProjectFacts { name: "@semio-tech/framework-os-dev".into(), root: "dev".into(), targets: targets.iter().map(|name| TargetFacts { name: (*name).into(), continuous: Some(true), ..Default::default() }).collect(), ..Default::default() };
    let playgrounds = vec![PlaygroundFacts { variant: "p".into(), plugin: "x".into(), react: 7100, wgpu: 7101, ..Default::default() }];
    let complete = Registry::build(Path::new("/w"), &Facts { projects: vec![owner(&["dev-p-react-dev", "dev-p-wgpu-dev", "run-p-native-dev"])], playgrounds: playgrounds.clone(), ..Default::default() });
    assert!(complete.check().iter().all(|problem| !problem.message.contains("project graph does not have")));
    let missing = Registry::build(Path::new("/w"), &Facts { projects: vec![owner(&["dev-p-react-dev"])], playgrounds: playgrounds.clone(), ..Default::default() }).check();
    let lacking: Vec<&str> = missing.iter().filter(|problem| problem.message.contains("project graph does not have")).map(|problem| problem.message.as_str()).collect();
    assert_eq!(lacking.len(), 2, "{lacking:?}");
    assert!(lacking[0].contains("renderer wgpu-wasm resolves to the Nx target @semio-tech/framework-os-dev:dev-p-wgpu-dev") && lacking[1].contains("run-p-native-dev"));
    let nothing = Registry::build(Path::new("/w"), &Facts { playgrounds, ..Default::default() }).check();
    assert_eq!(nothing.iter().filter(|problem| problem.message.contains("project graph does not have")).count(), 3, "no owner project at all: every renderer is reported");
}

#[test]
fn the_fixture_workspace_passes_its_own_check() {
    let (_, registry) = fixture("check");
    assert_eq!(registry.check(), &[] as &[Problem]);
}

#[test]
fn playground_facts_of_the_catalog_become_parameters_with_their_effects() {
    let (root, registry) = fixture("playground");
    let shell = registry.find("playground:shell").unwrap();
    let ids: Vec<String> = registry.parameters(shell).iter().map(|parameter| parameter.id.clone()).collect();
    for expected in ["renderer", "example", "user-slot", "app-role", "hub", "data", "local-only", "language"] { assert!(ids.contains(&expected.to_string()), "{expected} in {ids:?}"); }
    assert!(!registry.parameters(registry.find("playground:cad").unwrap()).iter().any(|parameter| ["hub", "data", "local-only", "user-slot"].contains(&parameter.id.as_str())));
    let base = resolve(&registry, "playground:shell", &Request::default());
    assert_eq!((env_of(&base.processes[0], "S_DATA_DIR"), env_of(&base.processes[0], "S_HUB_URL"), env_of(&base.processes[0], "S_LOCAL_ONLY")), (Some(format!("{}/.data/shell-base", root.display()).as_str()), None, None));
    let slot = resolve(&registry, "playground:shell", &request(&[("user-slot", "2"), ("hub", "true"), ("local-only", "true"), ("renderer", "wgpu-wasm")], &[], &[]));
    assert_eq!((env_of(&slot.processes[0], "S_DATA_DIR"), env_of(&slot.processes[0], "S_HUB_URL"), env_of(&slot.processes[0], "S_LOCAL_ONLY")), (Some(format!("{}/.data/shell-user2", root.display()).as_str()), Some("http://127.0.0.1:8787"), Some("1")));
    assert_eq!(slot.processes[0].ready.as_ref().map(|ready| ready.port), Some(6068));
    let unplugged = resolve(&registry, "playground:shell", &request(&[("data", "false")], &[], &[]));
    assert_eq!(env_of(&unplugged.processes[0], "S_DATA_DIR"), None);
    let viewer = resolve(&registry, "playground:cad", &request(&[("app-role", "viewer")], &[], &[]));
    assert_eq!((env_of(&viewer.processes[0], "SEMIO_APP_ROLE"), viewer.processes[0].ready.clone()), (Some("viewer"), Some(Ready { port: 6020, path: "/?plugin=cad&role=viewer".into(), printed: false })));
    assert_eq!(resolve(&registry, "playground:cad", &Request::default()).processes[0].ready.clone().map(|ready| ready.path), Some(String::new()));
}

#[test]
fn a_playground_example_locks_the_shell_to_it_and_native_renderers_pass_it_as_an_argument() {
    let (_, registry) = fixture("example");
    let react = resolve(&registry, "playground:shell", &request(&[("example", "tower")], &[], &[]));
    assert_eq!((env_of(&react.processes[0], "PLAYGROUND_LOCKED_EXAMPLE_ID"), react.processes[0].args.last().map(String::as_str)), (Some("tower"), Some("@semio-tech/framework-os-dev:dev-shell-react-dev")));
    let native = resolve(&registry, "playground:shell", &request(&[("example", "tower"), ("renderer", "wgpu-native")], &[], &[]));
    assert_eq!(native.processes[0].args, ["nx", "run", "@semio-tech/framework-os-dev:run-shell-native-dev", "--", "--example", "tower"]);
    assert!(native.processes[0].ready.is_none());
}

#[test]
fn a_catalog_example_is_chosen_by_its_slug_or_its_bare_id() {
    let root = materialise("bare-example");
    std::fs::create_dir_all(crate::catalog::generated_dir(&root)).unwrap();
    std::fs::write(crate::catalog::generated_dir(&root).join("🚀️playgrounds.json"), r#"[{"variant":"p","pluginId":"x","cratePath":"x","aliases":[],"ports":{"react":3100,"wgpu":3200},"examples":["🎬️first","🎬️second"]}]"#).unwrap();
    let registry = crate::inventory::discover(&root, &std::sync::atomic::AtomicBool::new(false));
    for chosen in ["second", "🎬️second"] { assert_eq!(env_of(&resolve(&registry, "playground:p", &request(&[("example", chosen)], &[], &[])).processes[0], "PLAYGROUND_LOCKED_EXAMPLE_ID"), Some("🎬️second")); }
    assert!(registry.resolve_with("playground:p", &request(&[("example", "third")], &[], &[]), RunPolicy::default()).unwrap_err().contains("choose 🎬️first, 🎬️second"));
}

#[test]
fn a_target_with_configurations_resolves_the_configuration_into_its_identity() {
    let (_, registry) = fixture("configuration");
    let launch = resolve(&registry, "hub:build:production", &Request::default());
    assert_eq!((launch.command_id.as_str(), launch.processes[0].args.as_slice(), launch.label.qualifier.as_str()), ("hub:build:production", ["nx", "run", "hub:build:production"].map(String::from).as_slice(), "production"));
    assert!(registry.resolve_with("hub:build:nope", &Request::default(), RunPolicy::default()).is_err());
}

#[test]
fn ticket_tools_resolve_relative_to_the_workspace_and_pin_member_parameters() {
    let (root, registry) = fixture("ticket");
    let deep = resolve(&registry, "ticket:26/09/23/OPEN-PROBE/probe", &request(&[("mode", "deep")], &[], &[]));
    assert!(deep.processes[0].args.last().is_some_and(|word| word == "--mode=deep"));
    assert_eq!(env_of(&deep.processes[0], "PROBE_DEPTH"), Some("9"));
    let compound = resolve(&registry, "ticket:26/09/23/OPEN-PROBE/probe-with-hub", &Request::default());
    assert_eq!(compound.processes.len(), 2);
    assert_eq!(env_of(&compound.processes[1], "PROBE_DEPTH"), Some("9"));
    assert!(compound.processes[1].cwd.starts_with(&root));
}

#[test]
fn repo_commands_follow_the_policy_that_picks_the_implementation() {
    let (root, registry) = fixture("repo");
    let rust = registry.resolve_with("repo:goals.list", &Request::default(), RunPolicy::default()).unwrap();
    assert_eq!(rust.processes[0].args[0], "repo-view");
    let go = registry.resolve_with("repo:goals.list", &Request::default(), RunPolicy { repo: RepoImplementation::Go, ..RunPolicy::default() }).unwrap();
    assert_eq!((go.processes[0].cmd.as_str(), go.processes[0].args.as_slice()), (crate::repo_domain::go_binary_path(&root).display().to_string().as_str(), ["goal", "list"].map(String::from).as_slice()));
    assert!(registry.resolve_with("repo:goals.list", &request(&[], &["x"], &[]), RunPolicy::default()).is_err());
    assert_eq!(registry.find("repo:ticket.close:26/09/23/OPEN-PROBE").map(|entry| entry.mutating), Some(true));
}

#[test]
fn references_that_name_nothing_are_problems_not_panics() {
    let facts = Facts { projects: vec![ProjectFacts { name: "p".into(), root: ".".into(), manifest: "📋️project.json".into(), dashboard: Some(serde_json::json!({
        "compounds": [{ "id": "c", "members": ["p:ghost", { "run": "p:build", "parameters": { "ghost": "1" }, "env": { "A": "{nope}" } }] }],
        "groups": [{ "id": "g", "targets": [], "projects": ["p"] }],
        "tools": [{ "id": "t", "command": ["x"], "requires": ["p:ghost", { "run": "p:build", "env": { "NX_DAEMON": "1" } }] }]
    })), targets: vec![TargetFacts { name: "build".into(), ..Default::default() }] }], ..Default::default() };
    let registry = Registry::build(Path::new("/w"), &facts);
    let text: Vec<String> = registry.problems().iter().map(ToString::to_string).collect();
    for expected in ["member \"p:ghost\" names no command", "has no parameter \"ghost\"", "environment token `{nope}`", "a group needs `targets`", "requires \"p:ghost\" names no command", "NX_DAEMON belongs to the dashboard's run policy"] {
        assert!(text.iter().any(|line| line.contains(expected)), "{expected} in {text:#?}");
    }
    let old = Facts { projects: vec![ProjectFacts { name: "p".into(), root: ".".into(), manifest: "📋️project.json".into(), dashboard: Some(serde_json::json!({ "groups": [{ "id": "h", "target": "build", "projects": ["p"] }] })), targets: Vec::new() }], ..Default::default() };
    assert!(Registry::build(Path::new("/w"), &old).problems().iter().any(|problem| problem.message.contains("unknown field `target`")), "the single-target group key no longer exists");
}

#[test]
fn a_compound_that_requires_itself_is_refused() {
    let facts = Facts { projects: vec![ProjectFacts { name: "p".into(), root: ".".into(), manifest: "📋️project.json".into(), targets: vec![TargetFacts { name: "a".into(), dashboard: Some(serde_json::json!({ "requires": ["p:b"] })), ..Default::default() }, TargetFacts { name: "b".into(), dashboard: Some(serde_json::json!({ "requires": ["p:a"] })), ..Default::default() }], dashboard: None }], ..Default::default() };
    let registry = Registry::build(Path::new("/w"), &facts);
    assert!(registry.resolve_with("p:a", &Request::default(), RunPolicy::default()).unwrap_err().contains("requires itself"));
}

#[test]
fn entries_are_searched_by_label_and_identity_and_closed_tickets_stay_unlisted() {
    let (_, registry) = fixture("search");
    let ids = |words: &[&str], unlisted: bool| registry.search(words, unlisted).into_iter().map(|position| registry.entries()[position].id.clone()).collect::<Vec<_>>();
    assert!(ids(&["dev", "shell"], false).contains(&"playground:shell".to_string()));
    assert!(ids(&["design", "3d"], false).contains(&"playground:cad".to_string()), "a playground is found by its aliases");
    assert!(ids(&["CLOSED-PROBE"], false).is_empty());
    assert!(ids(&["closed-probe"], true).iter().any(|id| id == "repo:ticket.reopen:26/08/01/CLOSED-PROBE"));
}

#[test]
fn tab_text_keeps_the_verb_and_elides_the_subject_in_the_middle() {
    let label = TaskLabel { verb: "dev".into(), owner: vec![], subject: "a-very-long-subject-name-indeed".into(), qualifier: "react".into(), parameters: vec![], members: 0 };
    let text = tab_text(&label, Locale::En, TAB_CELLS);
    assert!(text.starts_with("dev ") && text.contains('…') && cells(&text) <= TAB_CELLS, "{text}");
    assert_eq!(tab_text(&TaskLabel { subject: "hub".into(), ..label.clone() }, Locale::En, TAB_CELLS), "dev hub react");
    assert_eq!(tab_texts(&[&label, &label], Locale::En, TAB_CELLS).iter().filter(|text| text.ends_with(" ·2")).count(), 1);
    assert_eq!(window_title(&TaskLabel { subject: "hub".into(), owner: vec!["hub".into()], members: 2, parameters: vec![("backend".into(), "pg".into())], ..label }, Locale::De), "entwickeln · hub ×2 · react · backend=pg");
}

#[test]
fn the_xxh3_hash_matches_the_reference_vectors() {
    assert_eq!(xxh3_64(b""), 0x2d06_8005_38d3_94c2);
    assert_eq!(xxh3_64(b"a"), 0xe6c6_32b6_1e96_4e1f);
    assert_eq!(xxh3_64(b"abc"), 0x78af_5f94_892f_3950);
}

fn publish_graph_basis(root: &Path, registry: &Registry) {
    let rows = |files: Vec<&String>| files.into_iter().map(|file| serde_json::json!({ "file": file, "hash": xxh3_64(&std::fs::read(root.join(file)).unwrap()).to_string() })).collect::<Vec<_>>();
    let (owners, projects): (Vec<&String>, Vec<&String>) = registry.definitions().iter().partition(|file| GRAPH_OWNERS.contains(&file.as_str()));
    let document = serde_json::json!({ "fileMap": { "projectFileMap": { "all": rows(projects) }, "nonProjectFiles": rows(owners) } });
    std::fs::write(root.join(".nx/workspace-data/file-map.json"), document.to_string()).unwrap();
}

#[test]
fn a_build_starts_from_the_published_graph_only_while_the_manifests_still_hash_to_what_nx_recorded() {
    let (root, registry) = fixture("graph");
    let reuses = |registry: &Registry, id: &str| env_of(&registry.resolve(id, &Request::default()).unwrap().processes[0], GRAPH_REUSE.0).is_some();
    assert!(!registry.graph_is_current(), "no file map has been published yet");
    assert!(!reuses(&registry, "workspace:build"));
    publish_graph_basis(&root, &registry);
    assert!(registry.graph_is_current());
    assert!(reuses(&registry, "workspace:build") && reuses(&registry, "@fixture/quiz:test"), "finite Nx tasks reuse the graph");
    assert!(!reuses(&registry, "workspace:dev") && !reuses(&registry, "hub:dev"), "a continuous task builds its own graph");
    assert!(!reuses(&registry, "tool:workspace/format-notes"), "a command that does not run Nx has no graph to reuse");
    let manifest = root.join("🎓️teaching/❓️quiz/📦️packages/🟦️typescript/📋️project.json");
    let mut text = std::fs::read_to_string(&manifest).unwrap();
    text.push('\n');
    std::fs::write(&manifest, text).unwrap();
    assert!(!registry.graph_is_current(), "a changed manifest no longer hashes to the recorded value");
    assert!(!reuses(&registry, "workspace:build"), "so the build rebuilds and republishes the graph");
}

#[test]
fn every_scenario_of_the_registry_feature_is_proved_by_a_test_of_this_file() {
    let feature = include_str!("../../../🧫️fixtures/🎮️registry/🥒️.feature");
    let source = include_str!("🦀️.rs");
    let proofs: &[(&str, &[&str])] = &[
        ("A developer starts a declared command from the dashboard", &["a_ready_port_comes_from_the_declaration_or_the_resolved_environment_and_is_exported", "global_axes_select_tools_by_verb_and_nx_flags_do_not_reach_them", "a_target_resolves_axes_then_parameters_then_extra_arguments_after_the_double_dash"]),
        ("A compound starts its members in order", &["a_compound_starts_its_members_in_order_with_pins_and_member_environment"]),
        ("A command with a required parameter is not started without it", &["a_required_parameter_must_be_chosen_and_a_choice_must_be_one_of_its_values"]),
        ("A declaration that breaks the schema is reported", &["references_that_name_nothing_are_problems_not_panics"]),
        ("Build, test and publish commands are found by their verb and owner", &["verbs_come_from_the_leading_word_with_a_fallback", "entries_are_searched_by_label_and_identity_and_closed_tickets_stay_unlisted"]),
        ("A build starts from the published project graph", &["a_build_starts_from_the_published_graph_only_while_the_manifests_still_hash_to_what_nx_recorded"]),
        ("A changed project manifest rebuilds the project graph", &["a_build_starts_from_the_published_graph_only_while_the_manifests_still_hash_to_what_nx_recorded"]),
    ];
    let titles: Vec<&str> = feature.lines().filter_map(|line| line.trim().strip_prefix("Scenario: ")).collect();
    assert_eq!(titles, proofs.iter().map(|(title, _)| *title).collect::<Vec<_>>());
    for (title, tests) in proofs { for name in *tests { assert!(source.contains(&format!("fn {name}(")), "{title}: no test {name}"); } }
}

#[test]
fn a_tool_that_is_an_mcp_server_lists_the_clients_that_start_it_and_a_wrong_exposure_is_a_problem() {
    let (_, registry) = fixture("mcp");
    let entry = registry.find("tool:workspace/bun-test").unwrap();
    let mcp = entry.mcp.as_ref().expect("the exposure is read into a typed value");
    assert_eq!(mcp.server, "bun-test");
    assert_eq!(mcp.clients.keys().map(String::as_str).collect::<Vec<_>>(), ["claude-code", "codex"]);
    let listed = registry.entry_json(entry);
    assert_eq!(listed["mcp"], serde_json::json!({ "server": "bun-test", "clients": { "claude-code": {}, "codex": { "parameters": { "project": "workspace" } } } }));
    assert!(registry.find("tool:workspace/printer").unwrap().mcp.is_none() && registry.entry_json(registry.find("tool:workspace/printer").unwrap()).get("mcp").is_none());
    for (from, to, message) in [(r#""claude-code": {}"#, r#""emacs": {}"#, "mcp client \"emacs\""), (r#""project": "workspace" } } }"#, r#""nothing": "x" } } }"#, "states \"nothing\""), (r#""server": "bun-test""#, r#""server": "Bun Test""#, "mcp server")] {
        let (_, broken) = {
            let root = materialise_text("mcp-broken", &FIXTURE.replace(from, to));
            (root.clone(), crate::inventory::discover(&root, &std::sync::atomic::AtomicBool::new(false)))
        };
        assert!(broken.problems().iter().any(|problem| problem.message.contains(message)), "{message}: {:?}", broken.problems());
    }
}
