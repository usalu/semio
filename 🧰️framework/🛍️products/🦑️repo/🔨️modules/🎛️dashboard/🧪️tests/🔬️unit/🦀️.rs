use crate::args::parse;
use crate::catalog::{playgrounds_json_text, PlaygroundEntry, Ports};
use crate::env_contract::{build_dev_env, resolve_port, DevOptions};
use crate::options::Lock;

/// 🥒️ Fails unless the scenarios of a feature are exactly the titles of `proofs`, in order, and every test a title names is a test of one of `sources`.
pub(crate) fn assert_proved(feature: &str, sources: &[&str], proofs: &[(&str, &[&str])]) {
    let titles: Vec<&str> = feature.lines().filter_map(|line| line.trim().strip_prefix("Scenario: ")).collect();
    assert_eq!(titles, proofs.iter().map(|(title, _)| *title).collect::<Vec<_>>());
    let is_test = |name: &str| sources.iter().any(|source| source.find(&format!("fn {name}(")).is_some_and(|at| source[..at].trim_end().ends_with("#[test]")));
    for (title, names) in proofs {
        assert!(!names.is_empty(), "{title}: no test proves it");
        for name in *names { assert!(is_test(name), "{title}: {name} is no test"); }
    }
}

/// 🔎️ The verbs a text names: for each line `prefix <verbs>`, the first word split at `|` (and at an escaped one).
fn verbs_named(text: &str, prefix: &str) -> std::collections::BTreeSet<String> {
    text.lines().filter_map(|line| line.trim_start_matches(' ').strip_prefix(prefix.trim_start_matches(' '))).filter(|rest| !rest.starts_with(' ') && !rest.starts_with('<')).flat_map(|rest| {
        let word = rest.split([' ', '`']).next().unwrap_or_default();
        word.replace("\\|", "|").split('|').map(str::to_string).collect::<Vec<_>>()
    }).filter(|verb| verb.chars().next().is_some_and(|first| first.is_ascii_lowercase()) && verb.chars().all(|character| character.is_ascii_lowercase() || character == '-')).collect()
}

/// 🚩️ The flags a text names after each `--`, up to the next character that cannot be part of a flag name.
fn flags_named(text: &str) -> std::collections::BTreeSet<String> {
    text.split("--").skip(1).map(|rest| rest.chars().take_while(|character| character.is_ascii_lowercase() || *character == '-').collect::<String>()).map(|name| name.trim_end_matches('-').to_string()).filter(|name| !name.is_empty()).collect()
}

/// 🪓 The string literals of a source between a marker and the next `terminator`.
fn literals_after(source: &str, marker: &str, terminator: char) -> Vec<String> {
    source.split(marker).skip(1).flat_map(|rest| rest.split(terminator).next().unwrap_or_default().split('"').skip(1).step_by(2).map(str::to_string).collect::<Vec<_>>()).collect()
}

#[test]
fn the_dispatch_table_the_usage_text_and_the_readme_verb_table_name_the_same_verbs() {
    let dispatch = include_str!("../../📦️packages/🦀️rust/🦀️.rs");
    let table = dispatch.split("match parsed.verb.as_str() {").nth(1).and_then(|rest| rest.split("_ => {").next()).unwrap();
    let arms: std::collections::BTreeSet<String> = table.lines().filter_map(|line| line.trim().split(" =>").next().filter(|_| line.contains("=>"))).flat_map(|patterns| patterns.split(" if ").next().unwrap_or_default().split('|').map(|pattern| pattern.trim().trim_matches('"').to_string()).collect::<Vec<_>>()).collect();
    let native: std::collections::BTreeSet<String> = crate::NATIVE_VERBS.iter().map(ToString::to_string).collect();
    assert_eq!(arms, native, "NATIVE_VERBS and the dispatch arms differ");
    let documented: std::collections::BTreeSet<String> = native.iter().filter(|verb| !crate::INTERNAL_VERBS.contains(&verb.as_str())).cloned().collect();
    assert_eq!(verbs_named(crate::usage::USAGE, "  semio "), documented, "the usage text and the dispatch table differ");
    let readme = include_str!("../../README.md");
    let in_table: std::collections::BTreeSet<String> = readme.lines().filter(|line| line.starts_with("| `semio ")).filter_map(|line| line.strip_prefix("| ")).flat_map(|row| verbs_named(row.trim_start_matches('`'), "semio ")).collect();
    assert_eq!(in_table, documented, "the README verb table and the dispatch table differ");
}

#[test]
fn every_flag_a_verb_reads_is_in_the_usage_text_the_readme_verb_row_and_the_argument_readers() {
    let usage = crate::usage::USAGE;
    let readme = include_str!("../../README.md");
    let block = |verb: &str| -> String {
        let lines: Vec<&str> = usage.lines().collect();
        let start = lines.iter().position(|line| line.trim_start().starts_with(&format!("semio {verb}")) || (["stop", "restart", "kill"].contains(&verb) && line.contains("semio stop|restart|kill"))).unwrap_or_else(|| panic!("{verb} has no usage line"));
        lines[start..].iter().enumerate().take_while(|(offset, line)| *offset == 0 || !line.starts_with("  semio ")).map(|(_, line)| *line).collect::<Vec<_>>().join(" ")
    };
    let row = |verb: &str| -> String {
        let marker = if ["stop", "restart", "kill"].contains(&verb) { "| `semio stop\\|restart\\|kill".to_string() } else { format!("| `semio {verb}") };
        readme.lines().find(|line| line.starts_with(&marker)).unwrap_or_else(|| panic!("{verb} has no README row")).to_string()
    };
    for (verb, flags) in crate::usage::FLAGS {
        let (in_usage, in_row) = (flags_named(&block(verb)), flags_named(&row(verb)));
        for flag in *flags {
            assert!(in_usage.contains(*flag), "--{flag} of {verb} is missing in the usage text");
            assert!(in_row.contains(*flag), "--{flag} of {verb} is missing in the README row");
        }
        for flag in &in_usage { assert!(flags.contains(&flag.as_str()) || flag == "help", "usage names --{flag} for {verb}, which FLAGS does not list"); }
    }
    let cli = include_str!("../../🧭️cli/🦀️.rs");
    let mut read: std::collections::BTreeSet<String> = ["has(\"", "value(\"", "pairs(\"", "all(\""].iter().flat_map(|marker| cli.split(marker).skip(1).map(|rest| rest.split('"').next().unwrap_or_default().to_string()).collect::<Vec<_>>()).collect();
    read.extend(literals_after(cli, "Arguments::parse(argv, &[", ']'));
    read.remove("");
    let listed: std::collections::BTreeSet<String> = crate::usage::FLAGS.iter().filter(|(verb, _)| ["commands", "run", "tasks", "logs", "stop", "restart", "kill", "open"].contains(verb)).flat_map(|(_, flags)| flags.iter().map(ToString::to_string)).collect();
    assert_eq!(read, listed, "the flags the command line reads and the flags FLAGS lists differ");
    let help = crate::usage::USAGE;
    assert!(help.contains("semio --help"), "{help}");
}

#[test]
fn args_split_verb_segments_and_flags() {
    let argv: Vec<String> = ["dev", "puzzle", "2d", "--renderer", "wgpu-wasm", "--skip-plugin-build"].iter().map(|s| s.to_string()).collect();
    let parsed = parse(&argv);
    assert_eq!(parsed.verb, "dev");
    assert_eq!(parsed.segments, vec!["puzzle", "2d"]);
    assert_eq!(parsed.flag("renderer"), Some("wgpu-wasm"));
    assert!(parsed.has_flag("skip-plugin-build"));
    assert_eq!(parsed.flag("skip-plugin-build"), None);
}

#[test]
fn dispatch_reads_bare_and_flag_first_invocations_as_the_dashboard() {
    let invocation = |argv: &[&str]| crate::invocation(&argv.iter().map(|argument| argument.to_string()).collect::<Vec<_>>());
    assert_eq!(invocation(&[]), parse(&["dashboard".to_string()]));
    let flagged = invocation(&["--language", "de", "--help"]);
    assert_eq!((flagged.verb.as_str(), flagged.flag("language"), flagged.has_flag("help")), ("dashboard", Some("de"), true));
    assert!(flagged.segments.is_empty());
    let daemon = invocation(&["daemon", "status", "--root", "workspace"]);
    assert_eq!((daemon.verb.as_str(), daemon.segments.as_slice(), daemon.flag("root")), ("daemon", ["status".to_string()].as_slice(), Some("workspace")));
    let registry = invocation(&["plugin", "registry", "check"]);
    assert_eq!((registry.verb.as_str(), registry.segments.as_slice()), ("plugin", ["registry".to_string(), "check".to_string()].as_slice()));
}

#[test]
fn env_contract_sets_locks_only_for_individual() {
    let row = PlaygroundEntry { variant: "puzzle2d".into(), plugin_id: "puzzle2d".into(), ports: Ports { react: 6012, wgpu: 6112 }, ..Default::default() };
    let opts = DevOptions {
        renderer: "react".into(),
        example: Lock::Individual("concrete-forest".into()),
        language: Lock::All,
        terminology: Lock::Individual("reuse".into()),
        theme: Lock::All,
        appearance: Lock::Individual("dark".into()),
        ..Default::default()
    };
    let env = build_dev_env("puzzle2d", Some(&row), &opts);
    let get = |k: &str| env.iter().find(|(key, _)| key == k).map(|(_, v)| v.clone());
    assert_eq!(get("S_OS_PORT"), Some("6012".to_string()));
    assert_eq!(get("PLAYGROUND_LOCKED_EXAMPLE_ID"), Some("concrete-forest".to_string()));
    assert_eq!(get("SEMIO_LOCKED_TERMINOLOGY"), Some("reuse".to_string()));
    assert_eq!(get("SEMIO_LOCKED_APPEARANCE"), Some("dark".to_string()));
    assert_eq!(get("SEMIO_LOCKED_LOCALE"), None);
    assert_eq!(get("SEMIO_LOCKED_THEME"), None);
}

#[test]
fn resolve_port_prefers_explicit_then_catalog_then_fallback() {
    let row = PlaygroundEntry { ports: Ports { react: 6012, wgpu: 6112 }, ..Default::default() };
    assert_eq!(resolve_port(Some(&row), "react", Some(9999)), 9999);
    assert_eq!(resolve_port(Some(&row), "wgpu", None), 6112);
    assert_eq!(resolve_port(None, "react", None), 6066);
}

//#region 🔖️CatalogConsumer
/// 🧪️ Unique scratch dir under the configured artifact root, cleaned up by the caller.
fn temp_root(name: &str) -> std::path::PathBuf {
    let nanos = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos();
    let dir = std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").map_or_else(std::env::temp_dir, std::path::PathBuf::from).join(format!("semio-repo-dashboard-test-{name}-{nanos}"));
    std::fs::create_dir_all(&dir).expect("create temp root");
    dir
}

fn generated_dir_under(root: &std::path::Path) -> std::path::PathBuf {
    root.join("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated")
}

#[test]
fn playgrounds_json_text_falls_back_to_empty_array_when_missing() {
    let root = temp_root("json-text-missing");
    assert_eq!(playgrounds_json_text(&root), "[]\n");
    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn playgrounds_json_text_passes_generated_content_through_verbatim() {
    let root = temp_root("json-text-present");
    let out = generated_dir_under(&root);
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("🚀️playgrounds.json"), "[{\"variant\":\"x\"}]\n").unwrap();
    assert_eq!(playgrounds_json_text(&root), "[{\"variant\":\"x\"}]\n");
    std::fs::remove_dir_all(&root).ok();
}
//#endregion 🔖️CatalogConsumer
