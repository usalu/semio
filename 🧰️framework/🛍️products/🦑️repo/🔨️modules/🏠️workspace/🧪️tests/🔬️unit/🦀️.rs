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
