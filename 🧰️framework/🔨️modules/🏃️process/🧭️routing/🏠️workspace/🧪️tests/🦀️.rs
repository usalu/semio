//! 🧪️ The Rust workspace identity resolves every vector of `../🧬️schema/🔣️.json` cases (`🧫️fixtures/🔣️.json`) like the TypeScript, Go, C#, Python, shell and PowerShell hosts.

use semio_framework_process_routing_workspace::{declares_workspaces, find_workspace_root};
use serde_json::Value;
use std::{fs, path::PathBuf};

fn corpus() -> Value {
    serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("workspace corpus parses")
}

#[test]
fn every_corpus_vector_resolves_to_its_expected_root() {
    let base = PathBuf::from(env!("CARGO_TARGET_TMPDIR")).join(format!("workspace-identity-{}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    let corpus = corpus();
    for (index, row) in corpus["cases"].as_array().expect("cases").iter().enumerate() {
        let expected = row["expected"].as_str();
        let mut directory = base.join(index.to_string());
        if expected.is_none() {
            directory.extend(std::iter::repeat("a").take(32));
        }
        let start = directory.join(row["start"].as_str().expect("start"));
        fs::create_dir_all(&start).expect("start directory");
        for (name, content) in row["files"].as_object().expect("files") {
            let path = directory.join(name);
            fs::create_dir_all(path.parent().expect("parent")).expect("file directory");
            fs::write(&path, content.as_str().expect("content")).expect("file");
        }
        let found = find_workspace_root(&start).ok().flatten();
        let id = row["id"].as_str().expect("id");
        assert_eq!(found, expected.map(|relative| directory.join(relative).components().collect::<PathBuf>()), "{id}");
    }
    let _ = fs::remove_dir_all(&base);
}

#[test]
fn package_declarations_match_a_json_parser() {
    for source in [
        r#"{"workspaces":[]}"#, r#"{"workspaces":{"packages":[]}}"#, r#"{"workspaces":null}"#, r#"{"workspaces":"a"}"#, r#"{"workspaces":[],"workspaces":1}"#,
        r#"{"name":"x","nested":{"workspaces":[]}}"#, r#"{"a":[1,-2.5e+3,true,false,null,"é\n"],"workspaces":{}}"#, r#"[]"#, r#"{}"#, " {\"workspaces\" : [ ] } \n",
    ] {
        let parsed: Value = serde_json::from_str(source).expect("valid");
        let expected = matches!(parsed.get("workspaces"), Some(Value::Array(_) | Value::Object(_)));
        assert_eq!(declares_workspaces(source), Ok(expected), "{source}");
    }
    for source in ["", "{", "}", r#"{"workspaces":[}"#, r#"{"workspaces":[],}"#, r#"{"a" 1}"#, "{} {}", r#"{"a":01}"#, r#"{"a":"\q"}"#, "{\"a\":\"\n\"}", r#"{"a":tru}"#, r#"{"a":-}"#, r#"{"a":1.}"#] {
        assert!(serde_json::from_str::<Value>(source).is_err(), "{source}");
        assert!(declares_workspaces(source).is_err(), "{source}");
    }
}
