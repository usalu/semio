use super::*;

#[test]
fn double_star_spans_directories() {
    assert!(glob_match("**/*.go", "a/b/two.go").unwrap());
    assert!(glob_match("**/*.go", "root.go").unwrap());
    assert!(!glob_match("*.go", "a/one.go").unwrap());
}

#[test]
fn ignore_precedence_is_last_rule_wins() {
    let ignore = GitIgnore::compile_lines(["build/", "!build/keep.txt"]);
    assert!(ignore.matches_path("build/out.o"));
    assert!(!ignore.matches_path("build/keep.txt"));
}

#[test]
fn missing_config_yields_defaults() {
    let config = parse_repo_config("");
    assert!(!config.logging.session);
    assert!(config.logging.operations);
    assert_eq!(config.logging.detail, "standard");
}

#[test]
fn logging_section_overrides_defaults() {
    let config = parse_repo_config("[logging]\nsession = on\ndetail = \"full\"\n");
    assert!(config.logging.session);
    assert!(config.logging.include_native());
}

#[test]
fn tool_names_match_registry_patterns() {
    assert!(tool_name_matches("VSCODE_*", "VSCODE_GIT_ASKPASS"));
    assert!(tool_name_matches("LC_*", "LC_ALL"));
    assert!(tool_name_matches("*_X_*", "A_X_B"));
    assert!(tool_name_matches("NODE_OPTIONS", "NODE_OPTIONS"));
    assert!(!tool_name_matches("NODE_OPTIONS", "NODE_OPTIONS2"));
    assert!(!tool_name_matches("npm_config_*", "NPM"));
}

#[test]
fn registry_rows_parse_into_tools_and_denied_names() {
    let registry = parse_tool_registry(r#"{"environment":{"deny":["VSCODE_*"]},"tools":[{"id":"git","commands":["git"],"locate":[{"kind":"environment","name":"SEMIO_TOOL_GIT"},{"kind":"path","directories":["/opt/homebrew/bin"]}],"environment":{"deny":["GIT_DIR"],"set":{"A":"1"}}}]}"#).unwrap();
    assert_eq!(registry.deny, vec!["VSCODE_*".to_string()]);
    let git = &registry.tools[0];
    assert_eq!((git.id.as_str(), git.commands.clone(), git.directories.clone(), git.deny.clone()), ("git", vec!["git".to_string()], vec!["/opt/homebrew/bin".to_string()], vec!["GIT_DIR".to_string()]));
    assert_eq!(git.set, vec![("A".to_string(), "1".to_string())]);
    assert!(parse_tool_registry("not json").is_none());
}

#[test]
fn run_tool_runs_a_registered_tool_and_refuses_an_unknown_id() {
    if !program_exists("git") {
        return;
    }
    let result = run_tool("git", &["--version".to_string()], &ToolRunOptions::default()).unwrap();
    assert_eq!(result.status, Some(0));
    assert!(String::from_utf8_lossy(&result.stdout).starts_with("git version"));
    assert!(run_tool("no-such-tool-for-the-registry", &[], &ToolRunOptions::default()).is_err());
}

#[test]
fn declared_invocation_projection_matches_shared_neutral_corpus() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🏃️process/🧰️tools/🧪️tests/🔎️resolve/📣️invocations/🔣️.json")).unwrap();
    for row in corpus["projections"].as_array().unwrap() {
        let registry = parse_tool_registry(&serde_json::json!({"tools":[{"id":"fixture","commands":row["commands"],"invocations":row["invocations"]}]}).to_string()).unwrap();
        let (selected, prefix) = registered_tool_invocation(&registry.tools[0], row["requested"].as_str().unwrap());
        assert_eq!(selected, row["selected"].as_str().unwrap());
        assert_eq!(prefix, json_strings(row.get("prefix")).as_slice());
    }
    for invalid in corpus["invalid"].as_array().unwrap() {
        assert!(parse_tool_registry(&serde_json::json!({"tools":[{"id":"fixture","commands":["bun","bunx"],"invocations":[invalid]}]}).to_string()).is_none());
    }
    for invalid in corpus["invalidLists"].as_array().unwrap() {
        assert!(parse_tool_registry(&serde_json::json!({"tools":[{"id":"fixture","commands":["bun","bunx"],"invocations":invalid}]}).to_string()).is_none());
    }
}

#[test]
fn mapped_registered_invocation_keeps_prefix_before_caller_arguments() {
    let mut command = spawn_command("bunx");
    command.arg("--version");
    let args = command.get_args().map(|value| value.to_string_lossy().into_owned()).collect::<Vec<_>>();
    assert_eq!(args, vec!["x", "--version"]);
    assert!(command.get_envs().any(|(key,value)| key == "SEMIO_TOOL_RUN" && value.is_some_and(|value| value == "bun")));
}

#[cfg(windows)]
#[test]
fn windows_invocation_recorder() {
    let args = std::env::args().collect::<Vec<_>>();
    if let Some(index) = args.iter().position(|value| value == "--semio-record") {
        println!("SEMIO_ARGV={}", serde_json::to_string(&args[index + 1..]).unwrap());
        std::process::exit(0);
    }
}

#[cfg(windows)]
#[test]
fn windows_batch_invocation_preserves_literal_arguments() {
    let base = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
    let directory = base.join(format!("batch Ω space {}", std::process::id()));
    fs::create_dir_all(&directory).unwrap();
    let script = directory.join("recorder.cmd");
    let executable = std::env::current_exe().unwrap();
    fs::write(&script, format!("@echo off\r\n\"{}\" windows_invocation_recorder --nocapture -- --semio-record %*\r\n", executable.display())).unwrap();
    let expected = vec!["x", "two words", "Grüße Ω", "a&b|c(d)^e!f", "", "C:\\path\\"];
    let output = spawn_command(script.to_str().unwrap()).args(&expected).output().unwrap();
    assert!(output.status.success());
    let stdout = String::from_utf8(output.stdout).unwrap();
    let observed: Vec<String> = serde_json::from_str(stdout.lines().find_map(|line| line.strip_prefix("SEMIO_ARGV=")).unwrap()).unwrap();
    assert_eq!(observed, expected);
    for argument in ["a\"b", "%SEMIO_ARGV_SENTINEL%", "line\nnext"] {
        match spawn_command(script.to_str().unwrap()).arg(argument).env("SEMIO_ARGV_SENTINEL", "expanded").output() {
            Ok(output) => {
                assert!(output.status.success());
                let stdout = String::from_utf8(output.stdout).unwrap();
                let observed: Vec<String> = serde_json::from_str(stdout.lines().find_map(|line| line.strip_prefix("SEMIO_ARGV=")).unwrap()).unwrap();
                assert_eq!(observed, vec![argument]);
            }
            Err(error) => assert_eq!(error.kind(), std::io::ErrorKind::InvalidInput),
        }
    }
    let refused = spawn_command(script.to_str().unwrap()).arg("line\nnext").output().unwrap_err();
    assert_eq!(refused.kind(), std::io::ErrorKind::InvalidInput);
    fs::remove_dir_all(directory).unwrap();
}
