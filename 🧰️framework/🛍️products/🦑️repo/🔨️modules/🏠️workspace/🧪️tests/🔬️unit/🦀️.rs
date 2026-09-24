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
