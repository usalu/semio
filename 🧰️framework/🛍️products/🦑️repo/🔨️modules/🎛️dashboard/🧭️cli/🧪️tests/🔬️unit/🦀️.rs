use super::*;

fn voice() -> Voice { Voice::new(vec![dashboard_labels::labels(Locale::En)], Some(Locale::En)) }

fn words(text: &str) -> Vec<String> { text.split_whitespace().map(str::to_string).collect() }

#[test]
fn flags_valued_and_bare_words_and_the_tail_after_the_double_dash_are_told_apart() {
    let arguments = Arguments::parse(&words("hub:dev --param backend=postgres --param=data=x --env A=1 --detach --wait-ready -- --filter name"), &["param", "env"], &voice()).unwrap();
    assert_eq!(arguments.words, ["hub:dev"]);
    assert_eq!(arguments.pairs("param", &voice()).unwrap(), [("backend".to_string(), "postgres".to_string()), ("data".to_string(), "x".to_string())]);
    assert_eq!(arguments.pairs("env", &voice()).unwrap(), [("A".to_string(), "1".to_string())]);
    assert!(arguments.has("detach") && arguments.has("wait-ready") && !arguments.has("json"));
    assert_eq!(arguments.rest, ["--filter", "name"]);
}

#[test]
fn a_switch_may_stand_before_the_words_without_swallowing_them() {
    let arguments = Arguments::parse(&words("--json --all test quiz"), &["root"], &voice()).unwrap();
    assert_eq!(arguments.words, ["test", "quiz"]);
    assert!(arguments.has("json") && arguments.has("all"));
}

#[test]
fn malformed_flags_are_refused_with_the_flag_named() {
    assert!(Arguments::parse(&words("--param"), &["param"], &voice()).unwrap_err().contains("--param needs a value"));
    assert!(Arguments::parse(&words("--detach=yes"), &["param"], &voice()).unwrap_err().contains("--detach takes no value"));
    assert!(Arguments::parse(&words("--env NOPE"), &["env"], &voice()).unwrap().pairs("env", &voice()).unwrap_err().contains("not `key=value`"));
}

#[test]
fn the_request_carries_parameters_extra_environment_and_arguments() {
    let request = Arguments::parse(&words("x --param a=b --env K=V=W -- t1 t2"), &["param", "env"], &voice()).unwrap().request(&|_| None, &voice()).unwrap();
    assert_eq!(request, Request { parameters: vec![("a".into(), "b".into())], args: vec!["t1".into(), "t2".into()], env: vec![("K".into(), "V=W".into())] });
}

#[test]
fn a_bare_parameter_id_switches_a_flag_on_and_is_refused_for_choices_and_text() {
    let kind = |name: &str| match name { "steady" => Some(ParameterKind::Flag), "backend" => Some(ParameterKind::Choice), "listen" => Some(ParameterKind::Text), _ => None };
    let request = |text: &str| Arguments::parse(&words(text), &["param"], &voice()).unwrap().request(&kind, &voice());
    assert_eq!(request("x --param steady --param backend=pg --param steady2=false").unwrap().parameters, [("steady".to_string(), "true".to_string()), ("backend".into(), "pg".into()), ("steady2".into(), "false".into())]);
    assert_eq!(request("x --param steady=true").unwrap().parameters, [("steady".to_string(), "true".to_string())]);
    assert!(request("x --param backend").unwrap_err().contains("a choice parameter needs a value, write `--param backend=<value>`"));
    assert!(request("x --param listen").unwrap_err().contains("a text parameter needs a value"));
    assert_eq!(request("x --param ghost").unwrap().parameters, [("ghost".to_string(), "true".to_string())], "an unknown id is the registry's to refuse");
    assert!(request("x --param =v").unwrap_err().contains("is not `id` or `id=value`"));
}

#[test]
fn a_launch_becomes_the_group_message_the_daemon_starts() {
    let root = std::env::temp_dir().join("semio-cli-group");
    let process = |id: &str, ready: Option<dashboard_launch::Ready>| dashboard_launch::LaunchProcess { command_id: id.into(), cmd: "bun".into(), args: vec!["nx".into()], cwd: root.clone(), env: vec![], label: Default::default(), ready, long_running: true };
    let hub = dashboard_launch::Launch { command_id: "hub:dev".into(), label: Default::default(), processes: vec![process("hub:dev", Some(dashboard_launch::Ready { port: 8787, path: "/admin".into(), printed: false }))], requires: vec![], group: None, stop: dashboard_launch::Stop::Independent };
    let compound = dashboard_launch::Launch { command_id: "compound:w/c".into(), label: Default::default(), processes: vec![process("a:dev", None), process("b:dev", None)], requires: vec![hub], group: Some("compound:w/c".into()), stop: dashboard_launch::Stop::Together };
    let group = sized(SpawnGroup::from_launch(&compound, &[]));
    assert_eq!(group.requires.iter().map(|member| member.command.command_id.as_str()).collect::<Vec<_>>(), ["hub:dev"]);
    assert_eq!(group.members.iter().map(|member| member.command.command_id.as_str()).collect::<Vec<_>>(), ["a:dev", "b:dev"]);
    assert_eq!(group.stop, ipc::GroupStop::Together);
    assert!(group.members.iter().all(|member| member.command.group.as_deref() == Some(group.group_id.as_str()) && member.command.cols > 0));
    assert_eq!(group.requires[0].command.ready, Some(dashboard_launch::Ready { port: 8787, path: "/admin".into(), printed: false }));
}

#[test]
fn control_sequences_are_dropped_across_chunk_borders_and_text_is_kept() {
    let mut plain = Plain::default();
    let chunks: [&[u8]; 5] = [b"\x1b[0m\x1b[?25l\x1b[2J\x1b[m\x1b[Hhel", b"lo \x1b[3", b"1mred\x1b[0m\r\n\x1b]0;C:\\WIN", b"DOWS\\cmd.exe\x07tab\there\x07", b"\x1b]8;;x\x1b\\link\x1bcend"];
    let text: Vec<u8> = chunks.iter().flat_map(|chunk| plain.feed(chunk)).collect();
    assert_eq!(String::from_utf8(text).unwrap(), "hello red\r\ntab\therelinkend");
    assert_eq!(Plain::default().feed("héllo ✓".as_bytes()), "héllo ✓".as_bytes(), "multi-byte text passes unchanged");
}

#[test]
fn a_running_task_is_the_launch_asked_for_when_command_words_and_chosen_parameters_agree() {
    let command = |args: &[&str], parameters: &[(&str, &str)]| SessionCommand { cmd: "bun".into(), args: args.iter().map(ToString::to_string).collect(), command_id: "hub:dev".into(), label: dashboard_launch::TaskLabel { parameters: parameters.iter().map(|(key, value)| (key.to_string(), value.to_string())).collect(), ..Default::default() }, cwd: "C:/canonical".into(), ..Default::default() };
    assert!(same_launch(&command(&["nx"], &[("backend", "pg")]), &SessionCommand { cwd: "other".into(), env: vec![("TERM".into(), "x".into())], ..command(&["nx"], &[("backend", "pg")]) }, Path::new("/workspace")));
    assert!(!same_launch(&command(&["nx"], &[("backend", "pg")]), &command(&["nx"], &[]), Path::new("/workspace")));
    assert!(!same_launch(&command(&["nx"], &[]), &command(&["nx", "--"], &[]), Path::new("/workspace")));
}

#[test]
fn every_scenario_of_the_cli_feature_is_proved_by_a_test() {
    dashboard_core::testkit::assert_proved(include_str!("../../../🧪️tests/🧭️cli/🥒️.feature"), &[include_str!("🦀️.rs")], &[
        ("Flags, bare words and the tail after the double dash are told apart", &["flags_valued_and_bare_words_and_the_tail_after_the_double_dash_are_told_apart", "a_switch_may_stand_before_the_words_without_swallowing_them", "malformed_flags_are_refused_with_the_flag_named"]),
        ("A request carries parameters, extra environment and arguments", &["the_request_carries_parameters_extra_environment_and_arguments", "a_bare_parameter_id_switches_a_flag_on_and_is_refused_for_choices_and_text"]),
        ("A launch becomes the group message the daemon starts", &["a_launch_becomes_the_group_message_the_daemon_starts"]),
        ("A running task is the launch asked for when command words and parameters agree", &["a_running_task_is_the_launch_asked_for_when_command_words_and_chosen_parameters_agree"]),
        ("Control sequences are dropped from the text of a log", &["control_sequences_are_dropped_across_chunk_borders_and_text_is_kept"]),
        ("The handle printed by run is the handle of tasks, logs, open and stop", &["a_handle_is_a_session_id_a_group_a_command_or_a_unique_part_and_never_a_position"]),
        ("A command id or a group id names the live session", &["a_handle_is_a_session_id_a_group_a_command_or_a_unique_part_and_never_a_position"]),
        ("A position in a listing is no handle", &["a_handle_is_a_session_id_a_group_a_command_or_a_unique_part_and_never_a_position"]),
        ("Running the same command again reuses the live task", &["a_running_task_is_the_launch_asked_for_when_command_words_and_chosen_parameters_agree"]),
    ]);
}

fn session(id: &str, command: &str, status: SessionStatus, started: u64, group: Option<&str>) -> SessionInfo {
    SessionInfo { session_id: id.into(), status, started_ms: started, group: group.map(str::to_string), command: SessionCommand { command_id: command.into(), ..Default::default() }, ..Default::default() }
}

#[test]
fn a_handle_is_a_session_id_a_group_a_command_or_a_unique_part_and_never_a_position() {
    let sessions = vec![
        session("group-a-1.0", "hub:dev", SessionStatus::Exited, 10, None),
        session("group-b-2.0", "hub:dev", SessionStatus::Running, 20, None),
        session("group-c-3.0", "tool:w/pair-a", SessionStatus::Running, 30, Some("compound:w/pair")),
        session("group-c-3.1", "tool:w/pair-b", SessionStatus::Running, 31, Some("compound:w/pair")),
        session("group-d-4.0", "quiz:dev", SessionStatus::Exited, 40, None),
        session("group-e-5.0", "quiz:dev", SessionStatus::Exited, 50, None),
    ];
    let ids = |needle: &str| pick(&sessions, needle, &voice()).map(|picked| picked.sessions.iter().map(|session| session.session_id.clone()).collect::<Vec<_>>());
    assert_eq!(ids("group-a-1.0").unwrap(), ["group-a-1.0"], "an exact id, even of an ended session");
    assert_eq!(ids("hub:dev").unwrap(), ["group-b-2.0"], "a command id names its live session");
    assert_eq!(ids("quiz:dev").unwrap(), ["group-e-5.0"], "without a live session, its latest");
    assert_eq!(pick(&sessions, "group-c-3", &voice()).unwrap().group.as_deref(), Some("group-c-3"));
    assert_eq!(ids("group-c-3").unwrap(), ["group-c-3.0", "group-c-3.1"], "a launch id names all its members");
    assert_eq!(ids("compound:w/pair").unwrap(), ["group-c-3.0", "group-c-3.1"], "so does the compound's id");
    assert_eq!(ids("group-d").unwrap(), ["group-d-4.0"], "a unique part of a session id");
    assert_eq!(ids("pair-b").unwrap(), ["group-c-3.1"], "a unique part of a command id");
    assert!(ids("group-").unwrap_err().contains("matches 6 tasks"), "an ambiguous part lists the candidates");
    assert!(ids("1").unwrap_err().contains("no task matches"), "a bare number is no handle");
    assert!(ids("dev").unwrap_err().contains("matches several commands: hub:dev, quiz:dev"));
    assert!(ids("").is_err());
    let twice = vec![session("a.0", "x:dev", SessionStatus::Running, 1, None), session("b.0", "x:dev", SessionStatus::Running, 2, None)];
    assert!(pick(&twice, "x:dev", &voice()).unwrap_err().contains("has 2 live tasks: a.0 (x:dev), b.0 (x:dev)"));
    assert!(only(&pick(&sessions, "group-c-3", &voice()).unwrap(), "group-c-3", &voice()).unwrap_err().contains("names 2 tasks of one launch"));
}

#[test]
fn a_running_task_reuses_only_the_same_effective_build_tool() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/♻️reuse/🔣️.json")).unwrap();
    let pairs = |value: &serde_json::Value| value.as_array().unwrap().iter().map(|pair| (pair[0].as_str().unwrap().to_string(), pair[1].as_str().unwrap().to_string())).collect::<Vec<_>>();
    for row in fixture["cases"].as_array().unwrap() {
        let root = std::path::absolute(std::env::var("SEMIO_TEST_ARTIFACT_DIR").expect("caller-owned policy artifacts")).unwrap().join(format!("rp-{}", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let running = SessionCommand { cwd: row["cwd"].as_str().unwrap_or("/workspace").replace("/workspace", &root.to_string_lossy()), cmd: row["cmd"].as_str().unwrap_or("bun").into(), args: row["args"].as_array().unwrap().iter().map(|value| value.as_str().unwrap().replace("/workspace", &root.to_string_lossy())).collect(), command_id: "app:dev".into(), env: pairs(&row["running"]), ..Default::default() };
        let mut asked = SessionCommand { env: pairs(&row["asked"]), ..running.clone() };
        let client = pairs(&row["client"]);
        let path = ipc::build_authority::build_policy_path(&root);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, serde_json::json!({"version":1}).to_string()).unwrap();
        ipc::bind_build_tools(&root, ipc::build_tool_environment(&client), std::iter::once(&mut asked)).unwrap();
        std::fs::remove_dir_all(&root).unwrap();
        assert_eq!(asked.selected_build_tool(&root, None), row["tool"].as_str(), "{}", row["name"]);
        assert_eq!(same_launch(&running, &asked, &root), row["same"].as_bool().unwrap(), "{}", row["name"]);
    }
}

#[test]
fn registry_requirements_reach_the_group_without_losing_selected_tools() {
    use dashboard_registry::{Facts, ProjectFacts, TargetFacts};
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/♻️reuse/🔣️requirements.json")).unwrap();
    for case in fixture["cases"].as_array().unwrap() {
        let mut projects = Vec::new();
        let mut compounds = Vec::new();
        for entry in case["entries"].as_array().unwrap() {
            let id = entry["id"].as_str().unwrap();
            if let Some(id) = id.strip_prefix("compound:workspace/") { compounds.push(serde_json::json!({"id":id,"members":entry["members"]})); continue; }
            let (project, target) = id.split_once(':').unwrap();
            projects.push(ProjectFacts { name: project.into(), root: ".".into(), manifest: "📋️project.json".into(), dashboard: None, targets: vec![TargetFacts { name: target.into(), dashboard: Some(serde_json::json!({"requires":entry["requires"]})), ..Default::default() }] });
        }
        projects.push(ProjectFacts { name:"workspace".into(),root:".".into(),manifest:"📋️project.json".into(),targets:vec![],dashboard:Some(serde_json::json!({"compounds":compounds})) });
        let registry = Registry::build(Path::new("/workspace"), &Facts { projects, ..Default::default() });
        assert!(registry.problems().is_empty(), "{}: {:?}", case["name"], registry.problems());
        let launch = registry.resolve_with(case["root"].as_str().unwrap(), &Request::default(), RunPolicy::default()).unwrap();
        let mut group = SpawnGroup::from_launch(&launch, &[]);
        let mut ids = std::collections::HashSet::new();
        let actual: Vec<String> = group.requires.iter_mut().map(|service| { assert!(ids.insert(service.session_id.clone())); ipc::bind_build_tools(Path::new("/unused"),case["inherited"].as_str(),std::iter::once(&mut service.command)).unwrap(); format!("{}={}", service.command.command_id, service.command.selected_build_tool(Path::new("/unused"), None).unwrap_or("")) }).collect();
        let expected: Vec<&str> = case["required"].as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect();
        assert_eq!(actual,expected,"{}",case["name"]);
    }
}

#[test]
fn workspace_policy_binding_matches_portable_refusals_and_pinned_defaults() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../🧪️tests/🧫️fixtures/♻️reuse/🔣️.json")).unwrap();
    let pairs = |value: &serde_json::Value| value.as_array().unwrap().iter().map(|pair| (pair[0].as_str().unwrap().to_string(), pair[1].as_str().unwrap().to_string())).collect::<Vec<_>>();
    for row in fixture["policyCases"].as_array().unwrap() {
        let root = std::path::absolute(std::env::var("SEMIO_TEST_ARTIFACT_DIR").expect("caller-owned policy artifacts")).unwrap().join(format!("rp-{}",std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        let path = ipc::build_authority::build_policy_path(&root);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        if let Some(policy) = row["policy"].as_str() { std::fs::write(&path,policy).unwrap(); }
        let client = pairs(&row["client"]);
        let mut command = SessionCommand { cmd:row["cmd"].as_str().unwrap().into(),args:row["args"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().into()).collect(),env:pairs(&row["declared"]),..Default::default() };
        let before = command.env.clone();
        let mut other = command.clone();
        let mut count = 0;
        let commands = std::iter::once(&mut command).chain(row["duringBatchPolicy"].as_str().map(|_|&mut other)).inspect(|_| { count+=1; if count==2 { std::fs::write(&path,row["duringBatchPolicy"].as_str().unwrap()).unwrap(); } });
        let mut result = ipc::bind_build_tools(&root,ipc::build_tool_environment(&client),commands);
        if result.is_ok() { if let Some(policy) = row["rebindPolicy"].as_str() { std::fs::write(&path,policy).unwrap(); result=ipc::bind_build_tools(&root,ipc::build_tool_environment(&client),std::iter::once(&mut command)); } }
        assert_eq!(result.is_err(),row["error"].as_bool().unwrap(),"{}",row["name"]);
        if result.is_ok() { assert_eq!(command.selected_build_tool(&root, None),row["tool"].as_str(),"{}",row["name"]); }
        if !command.uses_build_tool(&root) { assert_eq!(command.env,before); }
        if row["duringBatchPolicy"].is_string() { assert_eq!(command.selected_build_tool(&root, None),other.selected_build_tool(&root, None)); }
        std::fs::remove_dir_all(&root).unwrap();
    }
}

#[test]
fn a_voice_hands_its_chosen_language_to_the_tasks_and_none_while_no_language_is_chosen() {
    let german = Voice::new(vec![dashboard_labels::labels(Locale::De)], Some(Locale::De));
    let both = Voice::new(vec![dashboard_labels::labels(Locale::En), dashboard_labels::labels(Locale::De)], None);
    assert_eq!(german.task_environment(), [("SEMIO_LANGUAGE".to_string(), "de".to_string())]);
    assert_eq!(voice().task_environment(), [("SEMIO_LANGUAGE".to_string(), "en".to_string())]);
    assert!(both.task_environment().is_empty());
}

#[test]
fn every_refusal_speaks_the_chosen_language_or_both_joined() {
    let german = Voice::new(vec![dashboard_labels::labels(Locale::De)], Some(Locale::De));
    let both = Voice::new(vec![dashboard_labels::labels(Locale::En), dashboard_labels::labels(Locale::De)], None);
    assert_eq!(Arguments::parse(&words("--param"), &["param"], &german).unwrap_err(), "--param braucht einen Wert");
    assert_eq!(Arguments::parse(&words("--param"), &["param"], &both).unwrap_err(), "--param needs a value / --param braucht einen Wert");
    assert!(Arguments::parse(&words("--env NOPE"), &["env"], &german).unwrap().pairs("env", &german).unwrap_err().contains("ist nicht `Schlüssel=Wert`"));
    assert!(pick(&[], "x", &german).unwrap_err().contains("Keine Aufgabe passt zu \"x\""));
    assert!(pick(&[], "", &both).unwrap_err().contains(" / Ein Aufgabenname ist leer"));
}
