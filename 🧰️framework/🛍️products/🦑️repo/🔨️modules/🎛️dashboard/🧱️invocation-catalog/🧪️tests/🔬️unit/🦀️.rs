use super::xxh3_64;
use crate::args::parse;
use crate::catalog::{PlaygroundEntry, Ports};
use crate::env_contract::{build_dev_env, resolve_port, DevOptions};
use crate::options::Lock;

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

#[test]
fn the_xxh3_hash_matches_the_reference_vectors() {
    assert_eq!(xxh3_64(b""), 0x2d06_8005_38d3_94c2);
    assert_eq!(xxh3_64(b"a"), 0xe6c6_32b6_1e96_4e1f);
    assert_eq!(xxh3_64(b"abc"), 0x78af_5f94_892f_3950);
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
    root.join("catalog")
}

#[test]
fn playgrounds_json_text_falls_back_to_empty_array_when_missing() {
    let root = temp_root("json-text-missing");
    assert_eq!(crate::catalog::playgrounds_json_text_from_catalog(None).unwrap(), "[]\n");
    assert!(crate::catalog::playgrounds_json_text_from_catalog(Some(&root)).is_err());
    std::fs::remove_dir_all(&root).ok();
}

#[test]
fn playgrounds_json_text_passes_generated_content_through_verbatim() {
    let root = temp_root("json-text-present");
    let out = generated_dir_under(&root);
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("🚀️playgrounds.json"), "[{\"variant\":\"x\"}]\n").unwrap();
    assert_eq!(crate::catalog::playgrounds_json_text_from_catalog(Some(&out)).unwrap(), "[{\"variant\":\"x\"}]\n");
    std::fs::remove_dir_all(&root).ok();
}
//#endregion 🔖️CatalogConsumer
