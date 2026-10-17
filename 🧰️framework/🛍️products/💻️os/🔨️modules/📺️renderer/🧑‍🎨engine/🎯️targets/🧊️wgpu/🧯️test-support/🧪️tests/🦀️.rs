//! 🧪️ Closed-fixture laws of the module tree reader; the TypeScript `🟦️.ts` beside the fixture proves the same expectations against the independent `readModuleTreeSource`.

use super::{inspect_module_children, read_module_tree_files, read_module_tree_source, slice_module_tree_source, ModuleChild, ModuleVia};
use crate::repo_reader::{engine_root, renderer_source, repo_bytes, repo_root, repo_text, repo_texts, shell_source, ENGINE_CANVAS_SOURCE_FILES, INTERPRETER_SOURCE_FILES, RENDERER_SOURCE_FILES, SCENES_SOURCE_FILES, SHELL_SOURCE_ROOTS};
use semio_framework_test_fixtures::fixture_path;
use serde_json::Value;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};

static SCRATCH: AtomicU64 = AtomicU64::new(0);

fn fixture() -> Value {
    serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("module tree fixture parses")
}

fn scratch(label: &str) -> PathBuf {
    let base = std::env::temp_dir().join(format!("semio-module-tree-{}-{}-{label}", std::process::id(), SCRATCH.fetch_add(1, Ordering::Relaxed)));
    let _ = std::fs::remove_dir_all(&base);
    std::fs::create_dir_all(&base).expect("scratch directory");
    base
}

fn materialize(label: &str, files: &Value) -> PathBuf {
    let base = scratch(label);
    for (path, text) in files.as_object().expect("files object") {
        let target = base.join(path);
        std::fs::create_dir_all(target.parent().expect("parent")).expect("directory");
        std::fs::write(target, text.as_str().expect("text")).expect("file");
    }
    base
}

fn child(row: &Value) -> ModuleChild {
    ModuleChild {
        path: row["path"].as_str().expect("path").to_owned(),
        modules: row["modules"].as_array().expect("modules").iter().map(|module| module.as_str().expect("module").to_owned()).collect(),
        via: if row["via"] == "include" { ModuleVia::Include } else { ModuleVia::Path },
    }
}

#[test]
fn fixture_identities_are_unique() {
    let fixture = fixture();
    let mut ids: Vec<&str> = ["children", "refusals", "trees", "treeRefusals"].iter().flat_map(|key| fixture[*key].as_array().expect("rows")).map(|row| row["id"].as_str().expect("id")).collect();
    let total = ids.len();
    ids.sort_unstable();
    ids.dedup();
    assert_eq!(ids.len(), total);
}

#[test]
fn children_match_the_closed_fixture() {
    for row in fixture()["children"].as_array().expect("children") {
        let expected: Vec<ModuleChild> = row["expected"].as_array().expect("expected").iter().map(child).collect();
        assert_eq!(inspect_module_children(row["source"].as_str().expect("source")).expect("inspects"), expected, "{}", row["id"]);
    }
}

#[test]
fn ambiguous_path_attributes_are_refused() {
    for row in fixture()["refusals"].as_array().expect("refusals") {
        let error = inspect_module_children(row["source"].as_str().expect("source")).expect_err("refused");
        assert_eq!(error, row["error"].as_str().expect("error"), "{}", row["id"]);
    }
}

#[test]
fn trees_list_files_parent_first_in_declaration_order() {
    for row in fixture()["trees"].as_array().expect("trees") {
        let base = materialize(row["id"].as_str().expect("id"), &row["files"]);
        let listed = read_module_tree_files(&base.join(row["root"].as_str().expect("root"))).expect("lists");
        let expected: Vec<PathBuf> = row["expected"].as_array().expect("expected").iter().map(|path| base.join(path.as_str().expect("path"))).collect();
        assert_eq!(listed, expected, "{}", row["id"]);
        let _ = std::fs::remove_dir_all(base);
    }
}

#[test]
fn source_is_the_newline_terminated_concatenation_of_the_listed_files() {
    for row in fixture()["trees"].as_array().expect("trees") {
        let base = materialize(row["id"].as_str().expect("id"), &row["files"]);
        let root = base.join(row["root"].as_str().expect("root"));
        let joined: String = read_module_tree_files(&root)
            .expect("lists")
            .iter()
            .map(|path| std::fs::read_to_string(path).expect("reads"))
            .map(|text| if text.ends_with('\n') { text } else { text + "\n" })
            .collect();
        assert_eq!(read_module_tree_source(&root).expect("reads"), joined, "{}", row["id"]);
        let _ = std::fs::remove_dir_all(base);
    }
}

#[test]
fn missing_mounts_are_refused() {
    for row in fixture()["treeRefusals"].as_array().expect("refusals") {
        let base = materialize(row["id"].as_str().expect("id"), &row["files"]);
        let error = read_module_tree_files(&base.join(row["root"].as_str().expect("root"))).expect_err("refused");
        assert!(error.contains(row["error"].as_str().expect("error")), "{}: {error}", row["id"]);
        let _ = std::fs::remove_dir_all(base);
    }
}

#[test]
fn slices_run_from_the_marker_to_the_next_marker_inside_one_file() {
    let fixture = fixture();
    let row = &fixture["trees"][0];
    let base = materialize("slice", &row["files"]);
    let root = base.join(row["root"].as_str().expect("root"));
    assert_eq!(slice_module_tree_source(&root, "pub fn alpha", Some("{}")).expect("slices"), "pub fn alpha() ");
    assert_eq!(slice_module_tree_source(&root, "pub fn deep", None).expect("slices"), "pub fn deep() {}");
    assert!(slice_module_tree_source(&root, "pub fn absent", None).is_err());
    let _ = std::fs::remove_dir_all(base);
}

#[test]
fn the_renderer_hubs_resolve_below_their_roots() {
    let root = fixture_path!("../../../🧊️renderer/🦀️.rs");
    if !root.exists() {
        return;
    }
    match read_module_tree_files(&root) {
        Ok(files) => {
            assert!(files.len() > 5);
            assert!(files.iter().all(|file| !file.to_string_lossy().contains("🧪️tests")));
        }
        Err(error) => assert!(error.contains("mount missing"), "{error}"),
    }
}

#[test]
fn repo_texts_read_what_the_standard_reader_reads() {
    let manifest = fixture_path!("Cargo.toml");
    let relative = manifest.strip_prefix(repo_root()).expect("the manifest sits below the repository root").to_string_lossy().into_owned();
    assert_eq!(repo_text(&relative), std::fs::read_to_string(&manifest).expect("reads"));
    assert_eq!(repo_texts(&[&relative, &relative]).trim_end(), format!("{0}\n{0}", repo_text(&relative).trim_end()));
    assert_eq!(repo_bytes(&relative), std::fs::read(&manifest).expect("reads").as_slice());
}

#[test]
fn every_source_set_file_exists_below_the_engine_folder() {
    let engine = engine_root();
    for file in RENDERER_SOURCE_FILES.iter().chain(SCENES_SOURCE_FILES).chain(ENGINE_CANVAS_SOURCE_FILES).chain(INTERPRETER_SOURCE_FILES).chain(SHELL_SOURCE_ROOTS) {
        assert!(engine.join(file).is_file(), "{file}");
    }
    assert!(renderer_source().contains("fn "));
    assert!(shell_source().contains("ShellState"));
}
