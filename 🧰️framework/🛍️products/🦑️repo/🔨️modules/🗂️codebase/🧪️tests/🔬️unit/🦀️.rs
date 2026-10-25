//! 🔬️ Cargo tests of the codebase crate over the synthetic repository of `🧫️fixtures/📡️repo-tree.json`.
//! The ignore verdicts and the scope resolution are judged against `🧫️fixtures/🔬️unit-vectors.json`
//! (real `git check-ignore` plus Python models of the structural rules and the lexical walk).

use super::*;
use semio_framework_test_fixtures::fixture_path;
use serde_json::Value;

//#region 📥️Fixtures
fn fixtures_dir() -> PathBuf {
    fixture_path!("..").join("..").join("🧪️tests").join("🧫️fixtures")
}

fn fixture(name: &str) -> Value {
    let path = fixtures_dir().join(name);
    let bytes = std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_slice(&bytes).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn rows<'a>(value: &'a Value, key: &str) -> &'a Vec<Value> {
    value[key].as_array().unwrap_or_else(|| panic!("{key} is not an array"))
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_else(|| panic!("member {key} is not a string in {value}"))
}

fn flag(value: &Value, key: &str) -> bool {
    value[key].as_bool().unwrap_or_else(|| panic!("member {key} is not a boolean in {value}"))
}

fn strings(value: &Value, key: &str) -> Vec<String> {
    rows(value, key).iter().map(|item| item.as_str().unwrap().to_string()).collect()
}

struct Repository(PathBuf);

impl Repository {
    fn new(tag: &str) -> Repository {
        let root = std::env::temp_dir().join(format!("semio-repo-codebase-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&root);
        for entry in rows(&fixture("📡️repo-tree.json"), "entries") {
            let path = root.join(text(entry, "path"));
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, text(entry, "content")).unwrap();
        }
        Repository(root)
    }

    fn codebase(&self) -> Codebase {
        Codebase::new(&self.0)
    }
}

impl Drop for Repository {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
//#endregion 📥️Fixtures

//#region 🎨️Emoji
fn entity(name: &str) -> String {
    let table = fixture("../../../🪪️identity/🧬️schema/🔣️entity-emojis.json");
    let raw = text(&table["entities"], name).to_string();
    let base: String = raw.chars().filter(|value| *value != '\u{FE0E}' && *value != '\u{FE0F}').collect();
    for text_default in rows(&table, "textDefaultEmojis") {
        let text_default = text_default.as_str().unwrap();
        if base.contains(text_default) {
            return base.replace(text_default, &format!("{text_default}\u{FE0F}"));
        }
    }
    base
}
//#endregion 🎨️Emoji

//#region 🙈️Ignore
#[test]
fn every_path_gets_the_independent_structural_gitignore_and_generated_verdicts() {
    let repository = Repository::new("ignore");
    let codebase = repository.codebase();
    let answers = fixture("🔬️unit-vectors.json");
    assert!(rows(&answers, "ignore").len() >= 60);
    for row in rows(&answers, "ignore") {
        let path = text(row, "path");
        assert_eq!(codebase.is_repo_excluded_path(path), flag(row, "structural"), "structural {path:?}");
        assert_eq!(codebase.is_gitignored(path), flag(row, "gitignored"), "gitignore {path:?}");
        assert_eq!(is_generated_folder(path), flag(row, "generated"), "generated {path:?}");
    }
}

#[test]
fn the_owned_ignore_matcher_agrees_with_real_git_except_for_the_recorded_divergence() {
    let repository = Repository::new("divergence");
    let codebase = repository.codebase();
    let answers = fixture("🔬️unit-vectors.json");
    let mut diverging: Vec<&str> = Vec::new();
    for row in rows(&answers, "ignore") {
        let path = text(row, "path");
        if codebase.is_gitignored(path) != flag(row, "git") {
            diverging.push(path);
        }
    }
    assert_eq!(diverging, vec!["compose/🖥️app/ignored/💀️dead.ts"], "only a separator rule below the root may differ from git (decision repo-workspace-ignore-divergence)");
}

#[test]
fn the_gitignore_patterns_are_the_non_comment_lines_in_file_order() {
    let repository = Repository::new("patterns");
    let codebase = repository.codebase();
    assert_eq!(codebase.gitignore_patterns(), strings(&fixture("🔬️unit-vectors.json"), "gitignorePatterns"));
    std::fs::write(repository.0.join(".gitignore"), "# comment\n\n  spaced.txt  \n!kept\n").unwrap();
    assert_eq!(codebase.gitignore_patterns(), vec!["spaced.txt", "!kept"]);
    let bare = std::env::temp_dir().join(format!("semio-repo-codebase-bare-{}", std::process::id()));
    std::fs::create_dir_all(&bare).unwrap();
    assert!(Codebase::new(&bare).gitignore_patterns().is_empty());
    assert!(!Codebase::new(&bare).is_gitignored("a/b.ts"));
    let _ = std::fs::remove_dir_all(&bare);
}

#[test]
fn a_path_normalises_to_a_repository_relative_slash_path() {
    let codebase = Codebase::new("/work/repo");
    for (input, expected) in [("/work/repo/a/b.ts", "a/b.ts"), ("./a/b.ts", "a/b.ts"), ("a\\b.ts", "a/b.ts"), ("a/b.ts", "a/b.ts"), ("/elsewhere/a.ts", "/elsewhere/a.ts"), ("", "")] {
        assert_eq!(codebase.normalize_repo_path(input), expected, "{input:?}");
    }
    assert_eq!(codebase.root_dir(), Path::new("/work/repo"));
    assert_eq!(codebase.root_uri(), "file:///work/repo");
}

#[test]
fn considered_files_drop_excluded_and_ignored_paths_and_keep_the_order() {
    let repository = Repository::new("considered");
    let codebase = repository.codebase();
    let files: Vec<String> = ["b.ts", "node_modules/x.ts", "a/scratch.tmp", "a/dist/x.js", "LICENSE.md", "a.ts"].iter().map(|file| file.to_string()).collect();
    assert_eq!(codebase.filter_considered_files(&files), vec!["b.ts", "a.ts"]);
}

#[test]
fn generated_folders_and_bundle_labels_follow_their_vocabulary() {
    for (path, generated) in [("src/generated/x.ts", true), ("a/Generated/x.ts", false), ("js/vscode/generated", true), ("js/vscode/generatedx", false), ("a\\dist\\x", true), ("", false)] {
        assert_eq!(is_generated_folder(path), generated, "{path:?}");
    }
    for (name, label) in [("", ""), ("@scope/pkg", "@scope/pkg"), ("vscode", "repo/vscode"), ("repo", "repo/go"), ("app", "compose/app"), ("compose/app", "compose/compose/app")] {
        assert_eq!(normalize_bundle_label(name), label, "{name:?}");
    }
}

#[test]
fn an_ignore_pattern_matches_by_glob_and_a_trailing_double_star_also_names_the_folder() {
    for (path, is_dir, pattern, expected) in [
        ("a/node_modules", true, "**/node_modules/**", true),
        ("a/node_modules/x/y.ts", false, "**/node_modules/**", true),
        ("a/b.ts", false, "a/b.ts", true),
        ("a/b.ts", false, "a/c.ts", false),
        ("a/b", true, "a/b/**", true),
        ("a/b", false, "a/b/**", true),
        ("a/b.ts", false, "", false),
        ("a/b.ts", false, "   ", false),
        ("", false, "a", false),
        ("./a/b.ts", false, "a/b.ts", true),
        ("a\\b.ts", false, "a/b.ts", true),
    ] {
        assert_eq!(matches_ignore_pattern(path, is_dir, pattern), expected, "{path:?} {pattern:?}");
    }
}
//#endregion 🙈️Ignore

//#region 🚶️Walk
fn scope_row<'a>(scopes: &'a Value, group: &str, key: &str, value: &str) -> &'a Value {
    rows(scopes, group).iter().find(|row| text(row, key) == value).unwrap_or_else(|| panic!("no {group} row {value}"))
}

#[test]
fn the_repository_scope_resolves_to_the_independent_lexical_walk() {
    let repository = Repository::new("scope-repo");
    let answers = fixture("🔬️unit-vectors.json");
    assert_eq!(repository.codebase().scope_to_files(&Scope::Repo), strings(&answers["scopes"], "repo"));
}

#[test]
fn every_bundle_scope_resolves_to_the_files_below_its_root() {
    let repository = Repository::new("scope-bundles");
    let codebase = repository.codebase();
    let answers = fixture("🔬️unit-vectors.json");
    let bundles = rows(&answers["scopes"], "technologies");
    assert_eq!(bundles.len(), 4);
    for row in bundles {
        assert_eq!(codebase.scope_to_files(&Scope::Technology(text(row, "name").to_string())), strings(row, "files"), "{}", text(row, "name"));
    }
    assert!(codebase.scope_to_files(&Scope::Technology("no/such".to_string())).is_empty());
}

#[test]
fn folder_and_file_scopes_resolve_like_the_independent_model() {
    let repository = Repository::new("scope-folders");
    let codebase = repository.codebase();
    let answers = fixture("🔬️unit-vectors.json");
    for row in rows(&answers["scopes"], "folders") {
        assert_eq!(codebase.scope_to_files(&Scope::Folder(text(row, "path").to_string())), strings(row, "files"), "folder {}", text(row, "path"));
    }
    for row in rows(&answers["scopes"], "files") {
        assert_eq!(codebase.scope_to_files(&Scope::File(text(row, "path").to_string())), strings(row, "files"), "file {}", text(row, "path"));
    }
    assert!(codebase.scope_to_files(&Scope::Folder(String::new())).is_empty());
    assert!(codebase.scope_to_files(&Scope::File(String::new())).is_empty());
    let _ = scope_row(&answers["scopes"], "files", "path", "README.md");
}

#[test]
fn the_walk_skips_dot_folders_compares_extensions_case_insensitively_and_honours_extra_patterns() {
    let repository = Repository::new("walk");
    let root = &repository.0;
    std::fs::write(root.join("UPPER.TS"), "x").unwrap();
    std::fs::write(root.join("skip.me.ts"), "x").unwrap();
    std::fs::create_dir_all(root.join(".dot")).unwrap();
    std::fs::write(root.join(".dot/inside.ts"), "x").unwrap();
    let codebase = repository.codebase();
    let found = codebase.glob_by_extension("**/*", &["TS"], &[], false);
    assert!(found.contains(&"UPPER.TS".to_string()) && found.contains(&"skip.me.ts".to_string()));
    assert!(!found.iter().any(|path| path.starts_with(".dot")), "dot folders are not walked");
    assert!(found.iter().any(|path| path.ends_with("dead.ts")), "without gitignore the ignored folder is walked");
    let narrowed = codebase.glob_by_extension("**/*", &["ts"], &["skip.*".to_string(), "**/node_modules/**".to_string()], false);
    assert!(!narrowed.contains(&"skip.me.ts".to_string()) && narrowed.contains(&"UPPER.TS".to_string()));
    assert!(codebase.glob_by_extension("missing/**/*", &["ts"], &[], false).is_empty());
    let mut sorted = found.clone();
    sorted.sort();
    assert_eq!(found, sorted, "the walk is lexical");
}
//#endregion 🚶️Walk

//#region 🏗️Technologies
#[test]
fn technologies_bundles_kinds_manifests_and_packages_come_from_the_repository_files() {
    let repository = Repository::new("technologies");
    let codebase = repository.codebase();
    let technologies = codebase.technologies();
    let names: Vec<(&str, &str, TechnologyKind, &str)> = technologies.iter().map(|technology| (technology.name.as_str(), technology.root.as_str(), technology.kind, technology.emoji.as_str())).collect();
    assert_eq!(names, vec![("compose", "compose", TechnologyKind::User, "🎼"), ("repo", "repo", TechnologyKind::Infrastructure, "")]);
    let bundles = codebase.bundles();
    let summary: Vec<(String, String, BundleKind)> = bundles.iter().map(|bundle| (bundle.name.clone(), bundle.root.clone(), bundle.kind)).collect();
    let web = "compose/\u{1F310}\u{FE0F}web";
    assert_eq!(
        summary,
        vec![
            (web.to_string(), format!("compose/sites/{}", &web[8..]), BundleKind::Site),
            ("compose/\u{1F4DA}\u{FE0F}core".to_string(), "compose/\u{1F4DA}\u{FE0F}core".to_string(), BundleKind::Library),
            ("compose/\u{1F5A5}\u{FE0F}app".to_string(), "compose/\u{1F5A5}\u{FE0F}app".to_string(), BundleKind::Ui),
            ("repo/\u{1F439}\u{FE0F}tool".to_string(), "repo/\u{1F439}\u{FE0F}tool".to_string(), BundleKind::Library),
        ]
    );
    let app = &bundles[2];
    assert_eq!((app.source_root.as_str(), app.tags.clone(), app.emoji.as_str()), ("compose/\u{1F5A5}\u{FE0F}app/src", vec!["ui".to_string(), "desktop".to_string()], "🖥"));
    let core = &bundles[1];
    assert_eq!(core.packages, vec![Package { name: "@compose/core".into(), version: "1.2.3".into(), path: core.root.clone(), kind: "npm".into() }]);
    assert_eq!(bundles[0].source_root, "compose/sites/\u{1F310}\u{FE0F}web/app");
    assert!(bundles[3].packages.is_empty() && bundles[3].tags.is_empty());
}

#[test]
fn the_deepest_bundle_root_owns_a_path_and_the_cache_is_invalidated_on_request() {
    let repository = Repository::new("owners");
    let codebase = repository.codebase();
    let owner = |path: &str| codebase.bundle_by_path(path).map(|bundle| bundle.name);
    assert_eq!(owner("compose/\u{1F5A5}\u{FE0F}app/ignored/x.ts").as_deref(), Some("compose/\u{1F5A5}\u{FE0F}app"));
    assert_eq!(owner("compose/\u{1F5A5}\u{FE0F}app").as_deref(), Some("compose/\u{1F5A5}\u{FE0F}app"));
    assert_eq!(owner("compose/sites/\u{1F310}\u{FE0F}web/p.tsx").as_deref(), Some("compose/\u{1F310}\u{FE0F}web"));
    assert_eq!(owner("docs/guide.md"), None);
    assert_eq!(owner("compose/\u{1F5A5}\u{FE0F}apple/x.ts"), None, "a sibling that merely shares a prefix is not owned");
    assert_eq!(codebase.technologies().len(), 2);
    std::fs::create_dir_all(repository.0.join("extra")).unwrap();
    std::fs::write(repository.0.join("extra/README.md"), "---\nname: extra\nkind: research\n---\n").unwrap();
    assert_eq!(codebase.technologies().len(), 2, "cached");
    codebase.invalidate_technology_cache();
    let technologies = codebase.technologies();
    assert_eq!(technologies.len(), 3);
    assert_eq!(technologies[0].name, "compose");
    assert_eq!(technologies[1].kind, TechnologyKind::Research);
}

#[test]
fn a_bundle_kind_is_read_from_the_manifest_case_insensitively_and_defaults_to_library() {
    let repository = Repository::new("bundle-kind");
    let root = &repository.0;
    std::fs::create_dir_all(root.join("b1")).unwrap();
    std::fs::write(root.join("b1/package.json"), "{\"bundleKind\": \"BINARY\"}").unwrap();
    std::fs::create_dir_all(root.join("b2")).unwrap();
    std::fs::write(root.join("b2/project.json"), "{\"bundleKind\": \"nonsense\"}").unwrap();
    std::fs::create_dir_all(root.join("b3")).unwrap();
    std::fs::write(root.join("b3/package.json"), "not json").unwrap();
    let codebase = repository.codebase();
    assert_eq!(codebase.derive_bundle_kind("b1"), BundleKind::Binary);
    assert_eq!(codebase.derive_bundle_kind("b2"), BundleKind::Library);
    assert_eq!(codebase.derive_bundle_kind("b3"), BundleKind::Library);
    assert_eq!(codebase.derive_bundle_kind("missing"), BundleKind::Library);
    assert_eq!(codebase.derive_bundle_kind("compose/\u{1F5A5}\u{FE0F}app"), BundleKind::Ui);
}
//#endregion 🏗️Technologies

//#region 🪪️ArtifactIds
#[test]
fn folder_and_file_ids_compose_technology_bundle_folder_and_file_emojis() {
    let repository = Repository::new("ids");
    let codebase = repository.codebase();
    let app = format!("🎼compose🖥{}", "\u{1F5A5}\u{FE0F}app");
    assert_eq!(codebase.build_folder_id("compose/\u{1F5A5}\u{FE0F}app"), app);
    assert_eq!(codebase.build_folder_id("compose/\u{1F5A5}\u{FE0F}app/ignored"), format!("{app}{}ignored", entity("folder-organization")));
    assert_eq!(codebase.build_folder_id("docs"), format!("{}docs", entity("folder-organization")));
    assert_eq!(codebase.build_folder_id("."), "");
    assert_eq!(codebase.build_file_id("compose/\u{1F5A5}\u{FE0F}app/\u{1F9E9}\u{FE0F}widget.ts"), format!("{app}{}\u{1F9E9}\u{FE0F}widget", entity("file-code")));
    assert_eq!(codebase.build_file_id("LICENSE.md"), format!("{}license", entity("file-license")));
    assert_eq!(codebase.build_file_id("README.md"), format!("{}readme", entity("file-docs")));
    let core = codebase.build_file_id("compose/\u{1F4DA}\u{FE0F}core/\u{1F9EA}\u{FE0F}util_test.rs");
    assert!(core.ends_with(&format!("{}\u{1F9EA}\u{FE0F}utiltest", entity("file-lab"))), "{core}");
    assert!(core.starts_with("🎼compose"), "{core}");
}

#[test]
fn an_id_extends_the_id_of_its_container_and_a_uri_is_the_id_under_the_file_scheme() {
    let repository = Repository::new("id-nesting");
    let codebase = repository.codebase();
    let answers = fixture("📡️artifact-id-vectors.json");
    for vector in rows(&answers, "vectors") {
        let path = text(vector, "path");
        match text(vector, "kind") {
            "file" => {
                let id = codebase.build_file_id(path);
                assert!(!id.is_empty(), "{path}");
                assert_eq!(codebase.file_uri(path), format!("repo://file/{id}"));
                let directory = dir_of(path);
                if directory != "." {
                    let parent = codebase.build_folder_id(&directory);
                    assert!(id.starts_with(&parent), "{id} does not extend {parent}");
                }
                assert_eq!(codebase.build_file_id(&format!("./{path}")), id, "a leading ./ changes nothing");
            }
            "folder" => {
                let id = codebase.build_folder_id(path);
                assert_eq!(codebase.folder_uri(path), codebase.file_uri(path), "a folder is addressed through the file builder, as in the Go twin");
                assert!(id.is_empty() || codebase.build_folder_id(&format!("./{path}")) == id, "{path}");
            }
            "section" => {
                let section: Vec<String> = rows(vector, "sectionPath").iter().map(|segment| segment.as_str().unwrap().to_string()).collect();
                let file = codebase.build_file_id(path);
                let id = build_section_id(&file, &section);
                assert!(id.starts_with(&file), "{id}");
                assert_eq!(id == file, section.iter().all(String::is_empty), "{}", text(vector, "name"));
            }
            _ => {}
        }
    }
}

#[test]
fn a_folder_is_required_by_a_toolchain_marker_or_a_leading_dot_and_the_verdict_is_cached() {
    let repository = Repository::new("folder-kind");
    let root = &repository.0;
    for (folder, marker) in [("rust", "Cargo.toml"), ("node", "package.json"), ("py", "pyproject.toml"), ("go", "go.mod"), ("dotnet", "App.csproj"), ("sln", "All.sln"), ("plain", "notes.txt")] {
        std::fs::create_dir_all(root.join(folder)).unwrap();
        std::fs::write(root.join(folder).join(marker), "").unwrap();
    }
    let codebase = repository.codebase();
    for folder in ["rust", "node", "py", "go", "dotnet", "sln"] {
        assert_eq!(codebase.derive_folder_kind(folder), FolderKind::Required, "{folder}");
    }
    assert_eq!(codebase.derive_folder_kind("plain"), FolderKind::Organization);
    assert_eq!(codebase.derive_folder_kind("a/.hidden"), FolderKind::Required);
    std::fs::write(root.join("plain/Cargo.toml"), "").unwrap();
    assert_eq!(codebase.derive_folder_kind("plain"), FolderKind::Organization, "cached");
    codebase.invalidate_technology_cache();
    assert_eq!(codebase.derive_folder_kind("plain"), FolderKind::Required);
}
//#endregion 🪪️ArtifactIds

//#region 🔧️Helpers
fn leaf_section(name: &str, start_line: i64, end_line: i64, children: Vec<Section>) -> Section {
    let mut section: Section = serde_json::from_value(serde_json::json!({"name": name, "emoji": "", "startLine": start_line, "endLine": end_line, "startIndex": 0, "endIndex": 0})).unwrap();
    section.children = children;
    section
}

#[test]
fn sections_are_counted_with_every_nested_child() {
    let section = |name: &str, children: Vec<Section>| leaf_section(name, 0, 0, children);
    let tree = vec![section("a", vec![section("b", vec![section("c", Vec::new())]), section("d", Vec::new())]), section("e", Vec::new())];
    assert_eq!(count_sections(&tree), 5);
    assert_eq!(count_sections(&[]), 0);
}

#[test]
fn a_breach_scope_loses_its_section_and_definition_suffixes_and_a_definition_finds_its_section() {
    assert_eq!(extract_file_path("a/b.ts#Header#2"), "a/b.ts");
    assert_eq!(extract_file_path("a/b.ts\u{00A7}main"), "a/b.ts");
    assert_eq!(extract_file_path("a/b.ts"), "a/b.ts");
    let section = leaf_section;
    let sections = vec![section("Outer", 1, 20, vec![section("Inner", 5, 10, Vec::new())]), section("Other", 21, 30, Vec::new())];
    assert_eq!(find_section_for_definition(&sections, 6, 8, ""), "Outer#Inner");
    assert_eq!(find_section_for_definition(&sections, 2, 3, ""), "Outer");
    assert_eq!(find_section_for_definition(&sections, 22, 25, ""), "Other");
    assert_eq!(find_section_for_definition(&sections, 31, 32, ""), "");
    assert_eq!(find_section_for_definition(&sections, 4, 12, ""), "Outer", "a definition that straddles a child belongs to its parent");
}

#[test]
fn a_contributor_header_line_yields_the_name_and_the_email() {
    for (line, expected) in [
        ("2026 Ueli Saluz <ueli@semio-tech.com>", Some(("Ueli Saluz", "ueli@semio-tech.com"))),
        ("// 2026   Ada   <ada@example.org>  ", Some(("Ada", "ada@example.org"))),
        ("Copyright 1999 B <b@c> and 2026 D <d@e>", Some(("B", "b@c"))),
        ("2026 <ada@example.org>", None),
        ("2026 Ada <>", None),
        ("20267 Ada <ada@example.org>", None),
        ("2026Ada <ada@example.org>", None),
        ("no year here <a@b>", None),
        ("", None),
    ] {
        assert_eq!(parse_contributor_identity(line), expected.map(|(name, email)| (name.to_string(), email.to_string())), "{line:?}");
    }
}

#[test]
fn definitions_carry_their_section_their_emoji_their_file_and_a_stable_id() {
    let content = "//#region \u{1F516}\u{FE0F}Widget\nexport const label = \"widget\";\nexport function render(): string {\n  return label;\n}\n//#endregion \u{1F516}\u{FE0F}Widget\n";
    let definitions = file_definitions(content, "app/widget.ts");
    assert_eq!(definitions.iter().map(|definition| definition.name.as_str()).collect::<Vec<_>>(), vec!["label", "render"]);
    for definition in &definitions {
        assert_eq!(definition.file_path, "app/widget.ts");
        assert_eq!(definition.section_path, "Widget");
        assert_eq!(definition.emoji, definition_kind_emoji(definition.kind));
    }
    let id = definition_id("FILE", &definitions[1]);
    assert!(id.starts_with("FILE") && id.ends_with("render"), "{id}");
    assert!(file_definitions("", "app/widget.ts").is_empty());
    assert!(file_definitions("anything", "notes.unknown-extension").is_empty());
}
//#endregion 🔧️Helpers

//#region 🧱️Aggregates
fn context(codebase: &Codebase) -> CodebaseContext<'_> {
    let mut context = CodebaseContext::new(codebase);
    context.load_bundles();
    context.load_files();
    context
}

#[test]
fn the_context_loads_the_bundles_and_the_considered_files() {
    let repository = Repository::new("context");
    let codebase = repository.codebase();
    let loaded = context(&codebase);
    assert_eq!(loaded.bundles.len(), 4);
    assert_eq!(loaded.files, strings(&fixture("🔬️unit-vectors.json")["scopes"], "repo"));
    assert_eq!(loaded.bundle_for_file("compose/\u{1F5A5}\u{FE0F}app/\u{1F9E9}\u{FE0F}widget.ts"), "compose/compose/\u{1F5A5}\u{FE0F}app");
    assert_eq!(loaded.bundle_for_file("docs/guide.md"), "repo/repo");
    let info = loaded.bundle_info("compose/\u{1F5A5}\u{FE0F}app/x.ts").unwrap();
    assert_eq!(info.1, "compose/\u{1F5A5}\u{FE0F}app");
    assert!(loaded.bundle_info("elsewhere/x.ts").is_none());
}

#[test]
fn the_bundle_projection_counts_folders_files_lines_sections_and_definitions() {
    let repository = Repository::new("bundles");
    let codebase = repository.codebase();
    let bundles = context(&codebase).build_bundles();
    let metrics: Vec<(String, i64, i64, i64, i64, i64)> = bundles
        .iter()
        .map(|bundle| {
            let metrics = bundle.metrics.as_ref().unwrap();
            (bundle.folder.clone(), metrics.folders, metrics.files, metrics.sections, metrics.definitions, metrics.lines)
        })
        .collect();
    assert_eq!(
        metrics,
        vec![
            ("compose/sites/\u{1F310}\u{FE0F}web".to_string(), 1, 1, 0, 1, 4),
            ("compose/\u{1F4DA}\u{FE0F}core".to_string(), 2, 3, 0, 3, 11),
            ("compose/\u{1F5A5}\u{FE0F}app".to_string(), 2, 2, 1, 3, 9),
            ("repo/\u{1F439}\u{FE0F}tool".to_string(), 1, 1, 0, 1, 4),
            (String::new(), 0, 0, 0, 0, 0),
        ]
    );
    assert_eq!(bundles.last().unwrap().id, "repo/repo");
    assert!(bundles.iter().all(|bundle| bundle.metrics.as_ref().unwrap().breachs == 0));
}

#[test]
fn the_folder_projection_lists_every_folder_with_files_in_path_order() {
    let repository = Repository::new("folders");
    let codebase = repository.codebase();
    let folders = context(&codebase).build_folders();
    let paths: Vec<&str> = folders.iter().map(|folder| folder.path.as_str()).collect();
    let mut sorted = paths.clone();
    sorted.sort();
    assert_eq!(paths, sorted);
    assert_eq!(folders.len(), 6);
    let generated = folders.iter().find(|folder| folder.name == "generated").unwrap();
    assert_eq!(generated.metrics.as_ref().map(|metrics| (metrics.files, metrics.lines)), Some((1, 2)));
    assert_eq!(generated.parent_id.as_deref(), Some(codebase.build_folder_id("compose/\u{1F4DA}\u{FE0F}core").as_str()));
    assert_eq!(generated.id, codebase.build_folder_id("compose/\u{1F4DA}\u{FE0F}core/generated"));
    assert_eq!(generated.bundle_id.as_deref(), Some(codebase.bundle_id(&codebase.bundle_by_path(&generated.path).unwrap()).as_str()));
}

#[test]
fn the_file_projection_carries_ids_uris_metrics_and_breach_references_in_path_order() {
    let repository = Repository::new("files");
    let codebase = repository.codebase();
    let breach = Breach { id: "b".into(), summary: "s".into(), kind: "code/section/empty".into(), scope: "compose/\u{1F5A5}\u{FE0F}app/\u{1F9E9}\u{FE0F}widget.ts#Widget".into(), line: 2, column: 0, excerpt: String::new(), lint_priority: Some(BreachPriority::High), lint_autofixable: Some(true), reason: String::new(), solution: "fix it".into() };
    let mut loaded = context(&codebase).with_breachs(vec![breach]);
    loaded.load_files();
    let files = loaded.build_files();
    assert_eq!(files.len(), 7);
    let paths: Vec<&str> = files.iter().map(|file| file.path.as_str()).collect();
    let mut sorted = paths.clone();
    sorted.sort();
    assert_eq!(paths, sorted);
    let widget = files.iter().find(|file| file.path.ends_with("widget.ts")).unwrap();
    assert_eq!(widget.id, codebase.build_file_id(&widget.path));
    assert_eq!(widget.uri, format!("repo://file/{}", widget.id));
    assert_eq!(widget.metrics.as_ref().map(|metrics| (metrics.sections, metrics.definitions)), Some((1, 2)));
    assert_eq!(widget.breachs.len(), 1);
    assert_eq!((widget.breachs[0].priority, widget.breachs[0].autofixable, widget.breachs[0].solution.as_str()), (BreachPriority::High, true, "fix it"));
    assert!(files.iter().filter(|file| !file.path.ends_with("widget.ts")).all(|file| file.breachs.is_empty()));
}

#[test]
fn sections_and_definitions_are_projected_depth_first_with_their_scoped_ids() {
    let repository = Repository::new("sections");
    let codebase = repository.codebase();
    let loaded = context(&codebase);
    let sections = loaded.build_sections();
    assert_eq!(sections.len(), 1);
    assert!(sections[0].path.ends_with("widget.ts#Widget") && sections[0].id.ends_with("#Widget"));
    assert_eq!(sections[0].metrics.as_ref().map(|metrics| (metrics.definitions, metrics.lines)), Some((2, 6)));
    let definitions = loaded.build_definitions();
    assert_eq!(definitions.len(), 8);
    let paths: Vec<&str> = definitions.iter().map(|definition| definition.path.as_str()).collect();
    let mut sorted = paths.clone();
    sorted.sort();
    assert_eq!(paths, sorted);
    let render = definitions.iter().find(|definition| definition.path.ends_with("#Widget\u{00A7}render")).unwrap();
    assert_eq!(render.metrics.as_ref().map(|metrics| metrics.lines), Some(3));
    assert!(render.uri.ends_with("widget\u{00A7}render") && render.uri.starts_with("repo://file/"));
}

#[test]
fn a_breach_projection_names_the_folder_and_the_file_it_points_at() {
    let repository = Repository::new("breach");
    let codebase = repository.codebase();
    let breach = |scope: &str, line: i64| Breach { id: "b".into(), summary: "s".into(), kind: "code/file/wrong-license".into(), scope: scope.into(), line, column: 4, excerpt: String::new(), lint_priority: None, lint_autofixable: None, reason: "r".into(), solution: "s".into() };
    let loaded = context(&codebase).with_breachs(vec![breach("compose/\u{1F4DA}\u{FE0F}core/\u{1F527}\u{FE0F}util.rs#Header", 3), breach("README.md", 1)]);
    let projected = loaded.build_breachs();
    assert_eq!(projected.len(), 2);
    assert_eq!(projected[0].id, "code/file/wrong-license#|compose/compose/\u{1F4DA}\u{FE0F}core|compose/\u{1F4DA}\u{FE0F}core/\u{1F527}\u{FE0F}util.rs#0");
    assert_eq!(projected[0].folders.len(), 1);
    assert_eq!(projected[0].files[0].range.as_ref().map(|range| (range.start.line, range.start.column)), Some((3, 4)));
    assert_eq!((projected[0].priority, projected[0].autofixable, projected[0].reason.as_str()), (BreachPriority::Low, false, "r"));
    assert!(projected[1].folders.is_empty(), "a root file has no folder");
    assert_eq!(projected[1].id, "code/file/wrong-license#|repo/repo|README.md#1");
    let aggregated = loaded.build_bundles();
    assert_eq!(aggregated[1].metrics.as_ref().unwrap().breachs, 1);
    assert_eq!(aggregated.last().unwrap().metrics.as_ref().unwrap().breachs, 1);
}

#[test]
fn the_containment_tree_nests_bundles_folders_and_files_and_the_snapshot_assembles_everything() {
    let repository = Repository::new("tree");
    let codebase = repository.codebase();
    let loaded = context(&codebase);
    let snapshot = loaded.build();
    let tree = snapshot.tree.as_ref().unwrap();
    let root = &tree["compose"];
    assert_eq!(root.kind, CbTreeNodeKind::Repo);
    assert_eq!(root.children.len(), 5);
    let bundles = snapshot.bundles.as_ref().unwrap();
    for bundle in bundles {
        assert_eq!(root.children[&bundle.id].kind, CbTreeNodeKind::Bundle);
    }
    let app = &root.children[&bundles[2].id];
    let folder = &app.children["compose"].children["\u{1F5A5}\u{FE0F}app"];
    assert_eq!(folder.kind, CbTreeNodeKind::Folder);
    assert!(folder.children.values().any(|node| node.kind == CbTreeNodeKind::File));
    assert_eq!(folder.children["ignored"].kind, CbTreeNodeKind::Folder);
    assert_eq!(snapshot.files.as_ref().unwrap().len(), 7);
    assert_eq!(snapshot.folders.as_ref().unwrap().len(), 6);
    assert_eq!(snapshot.definitions.as_ref().unwrap().len(), 8);
    assert!(snapshot.contributors.is_none() && snapshot.tickets.is_none() && snapshot.policies.is_none());
    assert!(snapshot.breachs.as_ref().unwrap().is_empty());
}

#[test]
fn an_empty_context_projects_nothing_but_the_repository_fallback_bundle() {
    let empty = std::env::temp_dir().join(format!("semio-repo-codebase-empty-{}", std::process::id()));
    std::fs::create_dir_all(&empty).unwrap();
    let codebase = Codebase::new(&empty);
    let mut loaded = CodebaseContext::new(&codebase);
    loaded.load_bundles();
    loaded.load_files();
    let snapshot = loaded.build();
    assert_eq!(snapshot.bundles.as_ref().unwrap().len(), 1);
    assert!(snapshot.files.as_ref().unwrap().is_empty() && snapshot.folders.as_ref().unwrap().is_empty() && snapshot.sections.as_ref().unwrap().is_empty());
    let _ = std::fs::remove_dir_all(&empty);
}
//#endregion 🧱️Aggregates
