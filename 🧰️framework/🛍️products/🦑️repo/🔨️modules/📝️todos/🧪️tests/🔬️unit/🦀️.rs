//! 🔬️ Cargo tests of the todos crate. The todo line grammars, the rewrites, the scans and the comment
//! openers are judged against `🧫️fixtures/🔬️unit-vectors.json`, which an independent Python model computed
//! (the JavaScript grammars of the TS oracle restated as regular expressions) from the real line vectors and
//! the committed scan tree; the lifecycle and the drafts against explicit histories over in-memory ports.

use super::*;
use semio_framework_repo_events::Emitter;
use semio_framework_test_fixtures::fixture_path;
use serde_json::Value;

//#region 📥️Fixtures
fn fixture(name: &str) -> Value {
    let path = fixture_path!("..").join("..").join("🧪️tests").join("🧫️fixtures").join(name);
    let bytes = std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_slice(&bytes).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn answers() -> Value {
    fixture("🔬️unit-vectors.json")
}

fn rows<'a>(value: &'a Value, key: &str) -> &'a Vec<Value> {
    value[key].as_array().unwrap_or_else(|| panic!("{key} is not an array"))
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_else(|| panic!("member {key} is not a string in {value}"))
}

fn expect_todos(found: &[Todo], expected: &[Value], what: &str) {
    assert_eq!(found.len(), expected.len(), "{what}: {found:?}");
    for (todo, row) in found.iter().zip(expected) {
        assert_eq!((todo.id.as_str(), todo.name.as_str(), todo.description.as_str(), todo.parent_id.as_str()), (text(row, "id"), text(row, "name"), text(row, "description"), text(row, "parent")), "{what}");
        let location = todo.location.as_ref().unwrap_or_else(|| panic!("{what}: {} has no location", todo.name));
        assert_eq!((location.file_path.as_str(), location.line, location.column), (text(row, "file"), row["line"].as_i64().unwrap(), row["column"].as_i64().unwrap()), "{what}: {}", todo.name);
    }
}

fn scan_files() -> Vec<(String, String)> {
    rows(&fixture("🔍️scan-tree.json"), "files").iter().map(|file| (text(file, "path").to_string(), text(file, "content").to_string())).collect()
}

struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Scratch {
        let path = std::env::temp_dir().join(format!("semio-repo-todos-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).unwrap();
        Scratch(path)
    }
}

impl Drop for Scratch {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}
//#endregion 📥️Fixtures

//#region 🔍️Grammar
#[test]
fn the_markdown_vector_parses_like_the_independent_model() {
    let vectors = fixture("📝️line-vectors.json");
    let parent = text(&vectors, "markdownPath").rsplit_once('/').map(|(parent, _)| parent.to_string()).unwrap();
    expect_todos(&parse_todo_markdown(text(&vectors, "markdown"), &parent), rows(&answers(), "markdownParse"), "markdown");
}

#[test]
fn the_source_vector_parses_like_the_independent_model() {
    let vectors = fixture("📝️line-vectors.json");
    expect_todos(&parse_todo_comments(text(&vectors, "source"), text(&vectors, "sourcePath")), rows(&answers(), "sourceParse"), "source");
}

#[test]
fn every_comment_line_splits_into_prefix_name_and_description_like_the_independent_model() {
    let answers = answers();
    assert!(rows(&answers, "commentLines").len() >= 20);
    for row in rows(&answers, "commentLines") {
        let line = text(row, "line");
        let parts = split_todo_comment_parts(line);
        match row["parts"].as_array() {
            Some(expected) => assert_eq!(parts, Some((expected[0].as_str().unwrap().to_string(), expected[1].as_str().unwrap().to_string(), expected[2].as_str().unwrap().to_string())), "{line:?}"),
            None => assert_eq!(parts, None, "{line:?}"),
        }
    }
}

#[test]
fn a_comment_opener_follows_the_file_extension() {
    let answers = answers();
    for row in rows(&answers, "openers") {
        assert_eq!(todo_comment_opener(text(row, "path")), text(row, "opener"), "{}", text(row, "path"));
    }
    assert_eq!(todo_comment_opener("a\\b\\q.SQL"), "//", "the extension compares case-sensitively");
    assert_eq!(TODO_COMMENT_OPENERS, ["//", "#", "--"]);
}

#[test]
fn an_item_without_a_colon_has_a_blank_description_and_every_id_is_a_slug() {
    let parsed = parse_todo_markdown("- TODO Bare Item\n- TODO  Spaced Name :  spaced description  \n-  TODO Two Spaces: not an item\n  - TODO Indented: ok\n", "dir");
    assert_eq!(parsed.iter().map(|todo| (todo.id.as_str(), todo.name.as_str(), todo.description.as_str())).collect::<Vec<_>>(), vec![("BARE-ITEM", "Bare Item", ""), ("SPACED-NAME", "Spaced Name", "spaced description"), ("INDENTED", "Indented", "ok")]);
    assert!(parsed.iter().all(|todo| todo.parent_id == "dir" && todo.location.as_ref().unwrap().file_path == "dir/.todos.md"));
    assert_eq!(parse_todo_markdown("- TODO X: y", "").pop().unwrap().location.unwrap().file_path, ".todos.md");
    assert_eq!(parse_todo_markdown("- TODO X: y", ".").pop().unwrap().location.unwrap().file_path, ".todos.md");
}
//#endregion 🔍️Grammar

//#region ✏️Rewrites
#[test]
fn every_markdown_rewrite_and_removal_matches_the_independent_model() {
    let vectors = fixture("📝️line-vectors.json");
    let markdown = text(&vectors, "markdown");
    for row in rows(&answers(), "markdownRewrites") {
        let result = replace_in_markdown(markdown, text(row, "oldName"), text(row, "newName"), text(row, "newDescription"));
        match (row["result"]["ok"].as_str(), row["result"]["err"].as_str()) {
            (Some(document), _) => assert_eq!(result.unwrap(), document, "{}", text(row, "oldName")),
            (_, Some(class)) => assert_eq!(result.unwrap_err().class(), class, "{}", text(row, "oldName")),
            _ => panic!("the model answered neither"),
        }
        assert_eq!(remove_from_markdown(markdown, text(row, "oldName")), text(row, "removed"), "{}", text(row, "oldName"));
    }
}

#[test]
fn every_source_rewrite_and_removal_matches_the_independent_model() {
    let vectors = fixture("📝️line-vectors.json");
    let source = text(&vectors, "source");
    for row in rows(&answers(), "sourceRewrites") {
        let line = row["line"].as_i64().unwrap();
        let result = replace_in_file(source, line, text(row, "newName"), text(row, "newDescription"));
        match (row["result"]["ok"].as_str(), row["result"]["err"].as_str()) {
            (Some(document), _) => {
                assert_eq!(result.as_deref().unwrap(), document, "line {line}");
                let reparsed = parse_todo_comments(document, "src/app.ts");
                assert!(reparsed.iter().any(|todo| todo.name == text(row, "newName") && todo.description == text(row, "newDescription")), "a rewrite must read back: line {line}");
            }
            (_, Some(class)) => assert_eq!(result.unwrap_err().class(), class, "line {line}"),
            _ => panic!("the model answered neither"),
        }
        assert_eq!(remove_from_file(source, line), text(row, "removed"), "line {line}");
    }
}

#[test]
fn a_rewritten_comment_keeps_its_indentation_and_opener_and_a_markdown_rewrite_drops_its_indentation() {
    assert_eq!(replace_in_file("a\n    # TODO Old: x\nb", 2, "New", "y").unwrap(), "a\n    # TODO New: y\nb");
    assert_eq!(replace_in_markdown("- TODO Old: x\n  - TODO Nested: z", "Nested", "Moved", "w").unwrap(), "- TODO Old: x\n- TODO Moved: w");
    assert_eq!(remove_from_markdown("- TODO A: 1\n- TODO A: 2\n- TODO B: 3", "A"), "- TODO B: 3");
    assert_eq!(remove_from_file("a\nb\nc", 2), "a\nc");
    assert_eq!(remove_from_file("a\nb", 3), "a\nb");
    assert_eq!(remove_from_file("a\nb", 0), "a\nb");
}
//#endregion ✏️Rewrites

//#region 🌲️Scan
#[test]
fn the_memory_tree_scans_every_file_and_directory_it_holds() {
    let tree = MemoryTodoTree::seeded(scan_files());
    expect_todos(&scan_todos(&tree), rows(&answers()["scan"], "memory"), "memory scan");
}

#[test]
fn the_filesystem_tree_skips_dependency_build_and_dotted_directories() {
    let scratch = Scratch::new("fs-scan");
    let tree = FsTodoTree::new(&scratch.0);
    for (path, content) in scan_files() {
        tree.write(&path, &content).unwrap();
    }
    expect_todos(&scan_todos(&tree), rows(&answers()["scan"], "filesystem"), "filesystem scan");
    let walked: Vec<String> = tree.walk().into_iter().map(|entry| entry.path).collect();
    assert_eq!(walked.first().map(String::as_str), Some(""));
    assert!(!walked.iter().any(|path| path.starts_with("node_modules") || path.starts_with("dist") || path.starts_with(".cache")), "{walked:?}");
    assert!(tree.is_dir("src") && !tree.is_dir("src/app.ts") && tree.exists("src/app.ts") && !tree.exists("missing"));
    assert_eq!(tree.read("docs/.todos.md").as_deref(), Some("- TODO Write Guide: the guide is missing\n"));
    assert_eq!(tree.read("missing"), None);
}

#[test]
fn a_meta_directory_is_walked_but_other_dotted_directories_are_not() {
    let scratch = Scratch::new("meta-scan");
    let tree = FsTodoTree::new(&scratch.0);
    tree.write(&format!("{}/a.md", semio_framework_repo_workspace::SEMIO_DIR_NAME), "x").unwrap();
    tree.write(".other/b.md", "x").unwrap();
    tree.write("build/c.md", "x").unwrap();
    let walked: Vec<String> = tree.walk().into_iter().map(|entry| entry.path).collect();
    assert!(walked.iter().any(|path| path.ends_with("/a.md")) && !walked.iter().any(|path| path.contains("other") || path.starts_with("build")), "{walked:?}");
}

#[test]
fn a_scan_ignores_unsupported_extensions_and_the_markdown_items_of_ordinary_documents() {
    let tree = MemoryTodoTree::seeded([
        ("a.txt".to_string(), "// TODO Txt: unsupported extension\n".to_string()),
        ("docs/readme.md".to_string(), "- TODO Item: only in .todos.md\n// TODO Comment: but comments count in markdown\n".to_string()),
        ("noext".to_string(), "// TODO Bare: no extension\n".to_string()),
    ]);
    let found = scan_todos(&tree);
    assert_eq!(found.iter().map(|todo| todo.name.as_str()).collect::<Vec<_>>(), vec!["Comment"]);
    assert_eq!(TODO_SCAN_EXTENSIONS.len(), 9);
    assert_eq!(TODO_SKIPPED_DIRECTORIES, ["node_modules", "dist", "build"]);
}

#[test]
fn search_matches_the_name_or_the_description_case_insensitively() {
    let tree = MemoryTodoTree::seeded(scan_files());
    let all = scan_todos(&tree);
    for term in rows(&fixture("🔍️scan-tree.json"), "searchTerms").iter().map(|term| term.as_str().unwrap()).chain(["", "CACHE", "memoise", "no such todo anywhere"]) {
        let needle = term.to_lowercase();
        let expected: Vec<String> = all.iter().filter(|todo| needle.is_empty() || todo.name.to_lowercase().contains(&needle) || todo.description.to_lowercase().contains(&needle)).map(|todo| todo.id.clone()).collect();
        assert_eq!(search_todos(&tree, term).into_iter().map(|todo| todo.id).collect::<Vec<_>>(), expected, "{term:?}");
    }
    assert_eq!(search_todos(&tree, "cache").len(), 1);
}

#[test]
fn the_memory_tree_creates_every_ancestor_directory_and_walks_directories_before_their_contents() {
    let tree = MemoryTodoTree::seeded([("a/b/c.ts".to_string(), "x".to_string()), ("a/d.ts".to_string(), "y".to_string()), ("root.ts".to_string(), "z".to_string())]);
    let walked: Vec<(String, bool)> = tree.walk().into_iter().map(|entry| (entry.path, entry.is_dir)).collect();
    assert_eq!(walked, vec![("".into(), true), ("a".into(), true), ("a/b".into(), true), ("a/b/c.ts".into(), false), ("a/d.ts".into(), false), ("root.ts".into(), false)]);
    assert!(tree.is_dir("") && tree.is_dir("a/b") && !tree.is_dir("a/d.ts") && tree.exists("a/d.ts") && !tree.exists("a/x"));
    tree.append("a/d.ts", "+more").unwrap();
    tree.append("new.ts", "fresh").unwrap();
    assert_eq!(tree.read("a/d.ts").as_deref(), Some("y+more"));
    assert_eq!(tree.snapshot().iter().map(|(path, _)| path.as_str()).collect::<Vec<_>>(), vec!["a/b/c.ts", "a/d.ts", "new.ts", "root.ts"]);
}
//#endregion 🌲️Scan

//#region 🔓️Lifecycle
fn fixture_world() -> (MemoryTodoTree, MemoryEmitter, RecordingTicketOpener) {
    let tree = MemoryTodoTree::seeded([
        (".todos.md".to_string(), "- TODO Rename Region: the header region has to be renamed\n".to_string()),
        ("src/app.ts".to_string(), "export const value = 1;\n// TODO Cache Result: memoise the lookup\n".to_string()),
        ("src/script.py".to_string(), "value = 1\n".to_string()),
    ]);
    (tree, MemoryEmitter::new(), RecordingTicketOpener::new())
}

fn names(todos: &[Todo]) -> Vec<String> {
    todos.iter().map(|todo| todo.id.clone()).collect()
}

#[test]
fn the_lifecycle_history_rewrites_the_source_lines_and_emits_one_envelope_per_change() {
    let (tree, emitter, opener) = fixture_world();
    let todos = Todos::new(&tree, &emitter, "ueli");
    assert_eq!(names(&todos.list()), vec!["RENAME-REGION", "CACHE-RESULT"]);

    let directory = todos.create(&TodoCreateInput { parent_id: "src".into(), name: "Directory Todo".into(), description: "appended to the directory markdown".into() }).unwrap();
    assert_eq!((directory.id.as_str(), directory.parent_id.as_str(), directory.location.is_none()), ("DIRECTORY-TODO", "src", true));
    assert_eq!(tree.read("src/.todos.md").as_deref(), Some("- TODO Directory Todo: appended to the directory markdown\n"));

    let file = todos.create(&TodoCreateInput { parent_id: "src/script.py".into(), name: "File Todo".into(), description: "appended in the file comment style".into() }).unwrap();
    assert_eq!(file.location.as_ref().map(|location| location.file_path.as_str()), Some("src/script.py"));
    assert_eq!(tree.read("src/script.py").as_deref(), Some("value = 1\n\n# TODO File Todo: appended in the file comment style\n"));
    assert_eq!(todos.create(&TodoCreateInput { parent_id: "src/missing.ts".into(), name: "Nowhere".into(), description: "x".into() }).unwrap_err().to_string(), "invalid parent id (must be path to folder or file)");

    assert_eq!(names(&todos.list()), vec!["RENAME-REGION", "DIRECTORY-TODO", "CACHE-RESULT", "FILE-TODO"]);
    assert_eq!(todos.find("CACHE-RESULT").unwrap().location.unwrap().line, 2);

    let renamed = todos.change(&TodoChangeInput { id: "CACHE-RESULT".into(), name: Some("Cache Result Twice".into()), description: None }).unwrap();
    assert_eq!((renamed.id.as_str(), renamed.description.as_str()), ("CACHE-RESULT-TWICE", "memoise the lookup"));
    assert_eq!(tree.read("src/app.ts").as_deref(), Some("export const value = 1;\n// TODO Cache Result Twice: memoise the lookup\n"));
    todos.change(&TodoChangeInput { id: "RENAME-REGION".into(), name: None, description: Some("the header region was renamed".into()) }).unwrap();
    assert_eq!(tree.read(".todos.md").as_deref(), Some("- TODO Rename Region: the header region was renamed\n"));
    assert_eq!(todos.change(&TodoChangeInput { id: "NOPE".into(), name: Some("x".into()), description: None }).unwrap_err().class(), "not-found");

    let ticket = todos.to_ticket("RENAME-REGION", &opener, "Rename The Header Region", "rename it everywhere").unwrap();
    assert_eq!(ticket, "ticket-1");
    assert_eq!(opener.opened(), vec!["Rename The Header Region\tthe header region was renamed\n\nrename it everywhere"]);
    assert_eq!(tree.read(".todos.md").as_deref(), Some(""));
    assert_eq!(todos.to_ticket("RENAME-REGION", &opener, "Again", "gone").unwrap_err().class(), "not-found");
    assert_eq!(opener.opened().len(), 1, "no ticket is opened for a todo that is gone");

    assert!(todos.delete("DIRECTORY-TODO").unwrap());
    assert_eq!(todos.delete("DIRECTORY-TODO").unwrap_err().class(), "not-found");
    assert_eq!(names(&todos.list()), vec!["CACHE-RESULT-TWICE", "FILE-TODO"]);
    assert_eq!(todos.list().iter().map(|todo| todo.location.as_ref().unwrap().line).collect::<Vec<_>>(), vec![2, 3]);

    let kinds: Vec<String> = emitter.envelopes().iter().map(|line| line.split('\t').next().unwrap().to_string()).collect();
    assert_eq!(kinds, vec!["todo.create.ended", "todo.create.ended", "todo.change.ended", "todo.change.ended", "todo.delete.ended", "todo.delete.ended"]);
    let payload = |index: usize| serde_json::from_str::<Value>(emitter.envelopes()[index].splitn(3, '\t').nth(2).unwrap()).unwrap();
    assert_eq!(payload(0), serde_json::json!({"id": "DIRECTORY-TODO", "parent_id": "src", "name": "Directory Todo", "author": "ueli"}));
    assert_eq!(payload(2), serde_json::json!({"id": "CACHE-RESULT-TWICE", "parent_id": "src/app.ts", "author": "ueli", "name": "Cache Result Twice"}));
    assert_eq!(payload(3), serde_json::json!({"id": "RENAME-REGION", "author": "ueli", "description": "the header region was renamed"}));
    assert!(emitter.envelopes().iter().all(|line| line.split('\t').nth(1) == Some("repo-cli")));
}

#[test]
fn a_markdown_item_changes_in_its_document_and_the_last_comment_of_a_file_deletes_to_an_empty_file() {
    let (tree, emitter, _) = fixture_world();
    tree.write("src/.todos.md", "- TODO Item: x\n").unwrap();
    let todos = Todos::new(&tree, &emitter, "ueli");
    let changed = todos.change(&TodoChangeInput { id: "ITEM".into(), name: Some("Renamed".into()), description: None }).unwrap();
    assert_eq!((changed.id.as_str(), changed.parent_id.as_str()), ("RENAMED", "src"));
    assert_eq!(tree.read("src/.todos.md").as_deref(), Some("- TODO Renamed: x\n"));
    let bare = MemoryTodoTree::seeded([("a.ts".to_string(), "// TODO One: x\n".to_string())]);
    assert!(Todos::new(&bare, &emitter, "ueli").delete("ONE").unwrap());
    assert_eq!(bare.read("a.ts").as_deref(), Some(""));
    assert_eq!(TodoError::NoLocation.to_string(), "todo has no location");
}

#[test]
fn the_filesystem_tree_backs_the_whole_lifecycle() {
    let scratch = Scratch::new("fs-lifecycle");
    let tree = FsTodoTree::new(&scratch.0);
    tree.write("src/app.ts", "// TODO Cache: memoise\n").unwrap();
    let emitter = MemoryEmitter::new();
    let todos = Todos::new(&tree, &emitter, "ada");
    todos.create(&TodoCreateInput { parent_id: "src".into(), name: "Fs Todo".into(), description: "on disk".into() }).unwrap();
    assert_eq!(std::fs::read_to_string(scratch.0.join("src").join(".todos.md")).unwrap(), "- TODO Fs Todo: on disk\n");
    todos.change(&TodoChangeInput { id: "CACHE".into(), name: None, description: Some("changed".into()) }).unwrap();
    assert_eq!(std::fs::read_to_string(scratch.0.join("src").join("app.ts")).unwrap(), "// TODO Cache: changed\n");
    todos.delete("FS-TODO").unwrap();
    assert_eq!(names(&todos.list()), vec!["CACHE"]);
    assert_eq!(emitter.envelopes().len(), 3);
    assert!(tree.write("src/app.ts/impossible", "x").is_err(), "a file cannot be a directory");
}
//#endregion 🔓️Lifecycle

//#region 🎨️Drafts
fn entity(name: &str) -> String {
    let path = fixture_path!("..").join("..").join("..").join("🪪️identity").join("🧬️schema").join("🔣️entity-emojis.json");
    let table: Value = serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap();
    table["entities"][name].as_str().unwrap().to_string()
}

fn draft_files(paths: &[(&str, &str)]) -> Vec<(String, String)> {
    paths.iter().map(|(path, content)| (path.to_string(), content.to_string())).collect()
}

#[test]
fn the_draft_history_slugs_titles_refuses_duplicates_and_copies_files_by_base_name() {
    let store = MemoryDraftStore::new();
    assert!(list_drafts(&store).is_empty());
    let first = create_draft(&store, "Sketch Of The Tree", &draft_files(&[("notes/tree.md", "# tree\n"), ("notes/graph.svg", "<svg/>\n")])).unwrap();
    assert_eq!(first.id, "SKETCH-OF-THE-TREE");
    assert_eq!(store.files("SKETCH-OF-THE-TREE"), vec![("graph.svg".to_string(), "<svg/>\n".to_string()), ("tree.md".to_string(), "# tree\n".to_string())]);
    let duplicate = create_draft(&store, "sketch of the tree", &draft_files(&[("notes/other.md", "x")])).unwrap_err();
    assert_eq!((duplicate.class(), duplicate.to_string().as_str()), ("draft-exists", "draft already exists: SKETCH-OF-THE-TREE"));
    assert_eq!(store.files("SKETCH-OF-THE-TREE").len(), 2, "a refused draft changes nothing");
    let blank = create_draft(&store, "///", &[]).unwrap_err();
    assert_eq!((blank.class(), blank.to_string().as_str()), ("invalid-title", "invalid draft title"));
    create_draft(&store, "Second Draft", &draft_files(&[("a/b/c/deep.txt", "deep\n"), ("win\\path\\file.txt", "w")])).unwrap();
    assert_eq!(store.files("SECOND-DRAFT").iter().map(|(name, _)| name.clone()).collect::<Vec<_>>(), vec!["deep.txt", "file.txt"]);
    assert_eq!(list_drafts(&store).into_iter().map(|draft| draft.id).collect::<Vec<_>>(), vec!["SECOND-DRAFT", "SKETCH-OF-THE-TREE"]);
    delete_draft(&store, "SKETCH-OF-THE-TREE").unwrap();
    delete_draft(&store, "SKETCH-OF-THE-TREE").unwrap();
    assert_eq!(list_drafts(&store).into_iter().map(|draft| draft.id).collect::<Vec<_>>(), vec!["SECOND-DRAFT"]);
    assert!(store.files("missing").is_empty() && !store.exists("missing"));
}

#[test]
fn every_title_slugs_like_the_independent_model() {
    let answers = answers();
    for row in rows(&answers, "slugs") {
        let store = MemoryDraftStore::new();
        let created = create_draft(&store, text(row, "title"), &[]);
        match text(row, "slug") {
            "" => assert_eq!(created.unwrap_err().class(), "invalid-title", "{:?}", row["title"]),
            slug => assert_eq!(created.unwrap().id, slug, "{:?}", row["title"]),
        }
    }
}

#[test]
fn the_filesystem_draft_store_keeps_each_draft_in_its_own_directory() {
    let scratch = Scratch::new("fs-drafts");
    let store = FsDraftStore::new(&scratch.0);
    assert!(store.ids().is_empty());
    create_draft(&store, "Beta Draft", &draft_files(&[("x/one.md", "1")])).unwrap();
    create_draft(&store, "Alpha Draft", &draft_files(&[("y/two.md", "2"), ("y/three.md", "3")])).unwrap();
    assert_eq!(store.ids(), vec!["ALPHA-DRAFT", "BETA-DRAFT"]);
    assert_eq!(store.files("ALPHA-DRAFT"), vec![("three.md".to_string(), "3".to_string()), ("two.md".to_string(), "2".to_string())]);
    assert!(store.exists("BETA-DRAFT") && !store.exists("GAMMA"));
    assert_eq!(create_draft(&store, "beta draft", &[]).unwrap_err().class(), "draft-exists");
    delete_draft(&store, "BETA-DRAFT").unwrap();
    delete_draft(&store, "BETA-DRAFT").unwrap();
    assert_eq!(list_drafts(&store).into_iter().map(|draft| draft.id).collect::<Vec<_>>(), vec!["ALPHA-DRAFT"]);
    assert!(store.files("BETA-DRAFT").is_empty());
}

#[test]
fn a_draft_is_addressed_by_its_entity_emoji_and_the_flattened_identifier() {
    let draft = Draft { id: "SKETCH-OF-THE-TREE".to_string() };
    let bare = entity("draft");
    assert!(draft_id(&draft).starts_with(&bare) && draft_id(&draft).ends_with("sketchofthetree"), "{}", draft_id(&draft));
    assert_eq!(draft_uri(&draft), format!("repo://draft/{}", draft_id(&draft)));
    assert_eq!(draft_id(&Draft { id: "A B-c".to_string() }), format!("{}abc", draft_id(&Draft { id: String::new() })));
}
//#endregion 🎨️Drafts

//#region 🧾️Ports
#[test]
fn the_recording_opener_numbers_tickets_and_the_memory_emitter_keeps_tab_separated_envelopes() {
    let opener = RecordingTicketOpener::new();
    assert_eq!(opener.open("A", "pa").unwrap(), "ticket-1");
    assert_eq!(opener.open("B", "pb").unwrap(), "ticket-2");
    assert_eq!(opener.opened(), vec!["A\tpa", "B\tpb"]);
    let emitter = MemoryEmitter::new();
    emitter.emit("todo.create.ended", "repo-cli", &serde_json::json!({"id": "X"}));
    assert_eq!(emitter.envelopes(), vec!["todo.create.ended\trepo-cli\t{\"id\":\"X\"}"]);
}

#[test]
fn every_refusal_has_its_stable_class_and_the_go_wording() {
    for (error, class, message) in [
        (TodoError::NotFound, "not-found", "todo not found"),
        (TodoError::NoLocation, "no-location", "todo has no location"),
        (TodoError::InvalidParent, "invalid-parent", "invalid parent id (must be path to folder or file)"),
        (TodoError::DraftExists("X".into()), "draft-exists", "draft already exists: X"),
        (TodoError::InvalidTitle, "invalid-title", "invalid draft title"),
        (TodoError::Port("disk full".into()), "port", "disk full"),
    ] {
        assert_eq!((error.class(), error.to_string().as_str()), (class, message));
    }
    assert_eq!((TODO_MARKDOWN_NAME, TODO_MARKDOWN_PREFIX), (".todos.md", "- TODO "));
}
//#endregion 🧾️Ports
