//! 🔬️ Cargo tests of the goals crate. The identifier scheme, the forest and its rendering are judged
//! against `🧫️fixtures/🔬️unit-vectors.json`, which an independent Python model computed from the
//! shared inputs; the document codec against real documents written by neither implementation, with
//! `serde_json` as the third-party reader; the lifecycle against in-memory ports.

use super::*;
use semio_framework_test_fixtures::fixture_path;
use serde_json::Value;

//#region 📥️Fixtures
fn fixture(name: &str) -> Value {
    let path = fixture_path!("..").join("..").join("🧪️tests").join("🧫️fixtures").join(name);
    let bytes = std::fs::read(&path).unwrap_or_else(|error| panic!("{}: {error}", path.display()));
    serde_json::from_slice(&bytes).unwrap_or_else(|error| panic!("{}: {error}", path.display()))
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_else(|| panic!("member {key} is not a string in {value}"))
}

fn flag(value: &Value, key: &str) -> bool {
    value[key].as_bool().unwrap_or_else(|| panic!("member {key} is not a boolean in {value}"))
}

fn seeds<T: serde::de::DeserializeOwned>(value: &Value) -> Vec<T> {
    serde_json::from_value(value.clone()).expect("seed rows")
}
//#endregion 📥️Fixtures

//#region 🪪️IdScheme
#[test]
fn every_path_vector_resolves_like_the_independent_model() {
    let answers = fixture("🔬️unit-vectors.json");
    let rows = answers["paths"].as_array().expect("paths");
    assert!(rows.len() >= 10);
    for row in rows {
        let path = text(row, "path");
        assert_eq!(goal_depth(path) as u64, row["depth"].as_u64().unwrap(), "depth of {path:?}");
        assert_eq!(is_root_goal(path), flag(row, "root"), "root {path:?}");
        assert_eq!(is_first_gen_goal(path), flag(row, "firstGen"), "first generation {path:?}");
        assert_eq!(is_deeper_goal(path), flag(row, "deeper"), "deeper {path:?}");
        assert_eq!(goal_id_for_filesystem(path), text(row, "filesystem"), "filesystem {path:?}");
        assert_eq!(root_goal_id(path), text(row, "rootId"), "root id {path:?}");
        assert_eq!(parent_goal_id(path), text(row, "parentId"), "parent id {path:?}");
        assert_eq!(goal_path_to_compose_id(path), text(row, "compose"), "compose {path:?}");
        assert_eq!(compose_id_to_goal_path(text(row, "compose")), text(row, "composeBack"), "compose back {path:?}");
    }
}

#[test]
fn every_title_vector_composes_the_independent_identifier() {
    let answers = fixture("🔬️unit-vectors.json");
    for row in answers["titles"].as_array().expect("titles") {
        assert_eq!(compose_goal_id(text(row, "parent"), text(row, "title")), text(row, "id"), "title {:?}", row["title"]);
    }
}

#[test]
fn every_reference_vector_yields_the_independent_number_or_a_refusal() {
    let answers = fixture("🔬️unit-vectors.json");
    for row in answers["references"].as_array().expect("references") {
        let reference = text(row, "reference");
        for parsed in [parse_milestone_number(reference), parse_issue_number(reference)] {
            match (&row["milestone"], parsed) {
                (Value::Null, Err(error)) => assert_eq!(error.class(), "unparsable", "{reference:?}"),
                (Value::Null, Ok(number)) => panic!("{reference:?} must be refused, got {number}"),
                (expected, Ok(number)) => assert_eq!(expected.as_i64(), Some(number), "{reference:?}"),
                (_, Err(error)) => panic!("{reference:?} must parse, got {error}"),
            }
        }
    }
}

#[test]
fn the_compose_form_survives_the_filesystem_form() {
    for path in ["A", "A/B", "AI-OPTIMIZED-REPO/REPO-CLIENT"] {
        let composed = goal_path_to_compose_id(path);
        assert_eq!(goal_id_for_filesystem(&composed), compose_id_to_goal_path(&composed));
        assert_eq!(goal_id_for_filesystem(path), path);
    }
    assert_eq!(goal_path_to_compose_id(""), "");
    assert_eq!(compose_id_to_goal_path(""), "");
}
//#endregion 🪪️IdScheme

//#region 📄️DocumentCodec
#[test]
fn real_documents_re_encode_to_the_same_bytes() {
    let documents = fixture("🎯️goal-documents.json");
    for entry in documents["documents"].as_array().expect("documents") {
        let id = text(entry, "id");
        let json = text(entry, "json");
        let mut goal = decode_goal(id, json).unwrap_or_else(|error| panic!("{id}: {error}"));
        goal.parent = serde_json::from_str::<Value>(json).unwrap()["parent"].as_str().unwrap_or_default().to_string();
        assert_eq!(encode_goal(&goal).unwrap(), json, "{id}");
        let reread: Value = serde_json::from_str(&encode_goal(&goal).unwrap()).unwrap();
        let original: Value = serde_json::from_str(json).unwrap();
        assert_eq!(reread, original, "{id} differs structurally for serde_json");
    }
}

#[test]
fn decoding_derives_the_identifier_and_the_parent_from_the_path() {
    let documents = fixture("🎯️goal-documents.json");
    let entry = documents["documents"].as_array().unwrap().iter().find(|entry| text(entry, "id") == "AI-OPTIMIZED-REPO/CONSISTENT-REPO-HISTORY").expect("nested document");
    let goal = decode_goal(text(entry, "id"), text(entry, "json")).unwrap();
    assert_eq!(goal.id, "AI-OPTIMIZED-REPO/CONSISTENT-REPO-HISTORY");
    assert_eq!(goal.parent, "AI-OPTIMIZED-REPO");
    assert_eq!(goal.title, "Consistent Repo History");
    assert_eq!(goal.status, GoalStatus::Open);
    assert_eq!(goal.dates.due, "2026-02-15");
    assert_eq!(goal.management.unwrap().issue, "https://github.com/usalu/semio/issues/404");
}

#[test]
fn encoding_writes_the_parent_in_its_compose_form() {
    let documents = fixture("🎯️goal-documents.json");
    let entry = &documents["documents"].as_array().unwrap()[2];
    let goal = decode_goal(text(entry, "id"), text(entry, "json")).unwrap();
    assert_eq!(goal.parent, "AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY");
    let encoded: Value = serde_json::from_str(&encode_goal(&goal).unwrap()).unwrap();
    assert_eq!(encoded["parent"], goal_path_to_compose_id("AI-OPTIMIZED-REPO/REPO-CLIENT/REPO-BINARY"));
}

#[test]
fn a_document_that_is_not_json_is_refused_as_a_document() {
    for broken in ["", "not json", "{\"title\":", "[]", "{\"title\":\"x\"}"] {
        assert_eq!(decode_goal("A", broken).unwrap_err().class(), "document", "{broken:?}");
    }
}

#[test]
fn an_encoded_document_is_two_space_indented_with_one_trailing_newline() {
    let store = MemoryGoalStore::new();
    let management = NullManagement;
    let emitter = MemoryEmitter::new();
    let goals = Goals::new(&store, &management, &emitter, "ueli");
    goals.create(&create_input("Indent Check", "")).unwrap();
    let (_, document) = store.snapshot().into_iter().next().unwrap();
    assert!(document.starts_with("{\n  \"title\": \"Indent Check\""));
    assert!(document.ends_with("}\n") && !document.ends_with("\n\n"));
}
//#endregion 📄️DocumentCodec

//#region 🗄️Stores
fn scratch(name: &str) -> PathBuf {
    let path = std::env::temp_dir().join(format!("semio-repo-goals-{name}-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&path);
    std::fs::create_dir_all(&path).unwrap();
    path
}

fn exercise(store: &dyn GoalStore) {
    assert!(store.ids().is_empty());
    assert_eq!(store.read("A").unwrap_err().class(), "not-found");
    for id in ["B", "A/Z", "A", "A/Y"] {
        store.write(id, &format!("doc {id}")).unwrap();
    }
    assert_eq!(store.ids(), vec!["A", "A/Y", "A/Z", "B"]);
    assert!(store.exists("A/Y") && !store.exists("A/X"));
    assert_eq!(store.read("A/Z").unwrap(), "doc A/Z");
    store.rename("A", "C").unwrap();
    assert_eq!(store.ids(), vec!["B", "C", "C/Y", "C/Z"]);
    assert_eq!(store.read("C/Y").unwrap(), "doc A/Y");
    store.remove("C").unwrap();
    assert_eq!(store.ids(), vec!["B"]);
}

#[test]
fn the_memory_store_moves_and_removes_whole_subtrees() {
    exercise(&MemoryGoalStore::new());
    let store = MemoryGoalStore::seeded([("X".to_string(), "x".to_string())]);
    assert_eq!(store.snapshot(), vec![("X".to_string(), "x".to_string())]);
    assert_eq!(store.rename("missing", "other").unwrap_err().class(), "not-found");
}

#[test]
fn the_filesystem_store_behaves_like_the_memory_store() {
    let root = scratch("fs-store");
    let store = FsGoalStore::new(&root);
    exercise(&store);
    assert!(store.directory("A/Y").ends_with("A/Y") || store.directory("A/Y").ends_with("A\\Y"));
    let _ = std::fs::remove_dir_all(&root);
}
//#endregion 🗄️Stores

//#region 🔓️Lifecycle
fn create_input(title: &str, parent: &str) -> GoalCreateInput {
    GoalCreateInput {
        title: title.to_string(),
        description: format!("{title} description"),
        prompt: format!("{title} prompt"),
        due_date: "2026-12-31".to_string(),
        llm: "opus-4-5".to_string(),
        client: "claude-code".to_string(),
        parent: parent.to_string(),
        ..Default::default()
    }
}

struct World {
    store: MemoryGoalStore,
    management: RecordingManagement,
    emitter: MemoryEmitter,
}

impl World {
    fn new() -> World {
        World { store: MemoryGoalStore::new(), management: RecordingManagement::new("https://example.test/repo", 100), emitter: MemoryEmitter::new() }
    }

    fn goals(&self) -> Goals<'_> {
        Goals::new(&self.store, &self.management, &self.emitter, "ueli")
    }

    fn kinds(&self) -> Vec<String> {
        self.emitter.envelopes().iter().map(|line| line.split('\t').next().unwrap().to_string()).collect()
    }

    fn payload(&self, index: usize) -> Value {
        let line = &self.emitter.envelopes()[index];
        let parts: Vec<&str> = line.splitn(3, '\t').collect();
        assert_eq!(parts[1], "repo-cli");
        serde_json::from_str(parts[2]).unwrap()
    }
}

#[test]
fn a_root_goal_opens_a_milestone_and_stores_its_url() {
    let world = World::new();
    let goal = world.goals().create(&create_input("Rust Repo Implementation", "")).unwrap();
    assert_eq!(goal.id, "RUST-REPO-IMPLEMENTATION");
    assert_eq!(goal.status, GoalStatus::Open);
    assert_eq!(goal.dates.due, "2026-12-31");
    assert_eq!(goal.management.unwrap().milestone, "https://example.test/repo/milestone/100");
    assert_eq!(
        world.management.calls(),
        vec!["create-milestone 100 Rust Repo Implementation Rust Repo Implementation description", "update-milestone 100    2026-12-31"]
    );
    assert_eq!(world.kinds(), vec!["goal.open.ended"]);
    let payload = world.payload(0);
    assert_eq!((text(&payload, "id"), text(&payload, "title"), text(&payload, "llm"), text(&payload, "author")), ("RUST-REPO-IMPLEMENTATION", "Rust Repo Implementation", "opus-4-5", "ueli"));
}

#[test]
fn a_first_generation_goal_opens_an_issue_inside_the_root_milestone() {
    let world = World::new();
    world.goals().create(&create_input("Root Goal", "")).unwrap();
    let child = world.goals().create(&create_input("Child Goal", "ROOT-GOAL")).unwrap();
    assert_eq!(child.id, "ROOT-GOAL/CHILD-GOAL");
    assert_eq!(child.management.unwrap().issue, "https://example.test/repo/issues/101");
    assert_eq!(world.management.calls().last().unwrap(), "create-goal-issue 101 Child Goal Child Goal description 100");
}

#[test]
fn a_deeper_goal_becomes_a_sub_issue_of_its_parent_issue() {
    let world = World::new();
    world.goals().create(&create_input("Root Goal", "")).unwrap();
    world.goals().create(&create_input("Child Goal", "ROOT-GOAL")).unwrap();
    let grandchild = world.goals().create(&create_input("Leaf Goal", "ROOT-GOAL/CHILD-GOAL")).unwrap();
    assert_eq!(grandchild.id, "ROOT-GOAL/CHILD-GOAL/LEAF-GOAL");
    let calls = world.management.calls();
    assert_eq!(calls[calls.len() - 2], "create-goal-issue 102 Leaf Goal Leaf Goal description -");
    assert_eq!(calls[calls.len() - 1], "add-sub-issue https://example.test/repo/issues/101 https://example.test/repo/issues/102");
}

#[test]
fn a_given_milestone_is_adopted_instead_of_created() {
    let world = World::new();
    let mut input = create_input("Adopting Goal", "");
    input.milestone = "https://example.test/repo/milestone/7".to_string();
    let goal = world.goals().create(&input).unwrap();
    assert_eq!(goal.management.unwrap().milestone, "https://example.test/repo/milestone/7");
    assert!(world.management.calls().is_empty());
}

#[test]
fn switching_management_off_touches_no_provider_and_stores_no_link() {
    let world = World::new();
    let mut input = create_input("Quiet Goal", "");
    input.no_management = true;
    let goal = world.goals().create(&input).unwrap();
    assert!(goal.management.is_none());
    assert!(world.management.calls().is_empty());
    assert!(!world.store.read("QUIET-GOAL").unwrap().contains("github"));
}

#[test]
fn every_blank_required_input_is_refused_by_name() {
    let world = World::new();
    let cases: [(&str, fn(&mut GoalCreateInput)); 6] = [
        ("missing title", |input| input.title.clear()),
        ("missing description", |input| input.description.clear()),
        ("missing prompt", |input| input.prompt.clear()),
        ("missing due date", |input| input.due_date.clear()),
        ("missing llm", |input| input.llm.clear()),
        ("missing client", |input| input.client.clear()),
    ];
    for (message, blank) in cases {
        let mut input = create_input("Some Goal", "");
        blank(&mut input);
        let error = world.goals().create(&input).unwrap_err();
        assert_eq!((error.to_string().as_str(), error.class()), (message, "missing"));
    }
    assert!(world.store.ids().is_empty() && world.management.calls().is_empty() && world.emitter.envelopes().is_empty());
}

#[test]
fn an_unknown_model_a_duplicate_and_a_free_form_model_are_judged() {
    let world = World::new();
    let mut unknown = create_input("Unknown Model", "");
    unknown.llm = "not-a-model".to_string();
    assert_eq!(world.goals().create(&unknown).unwrap_err().class(), "not-allowed");
    let mut free = create_input("Free Model", "");
    free.llm = "Claude Opus 4.5 (thinking)".to_string();
    free.effort = "High".to_string();
    let goal = world.goals().create(&free).unwrap();
    assert_eq!((goal.llm.as_str(), goal.effort.as_str()), ("opus-4-5", "high"));
    assert_eq!(world.goals().create(&create_input("Free Model", "")).unwrap_err().to_string(), "goal with id FREE-MODEL already exists");
}

#[test]
fn changing_a_title_moves_the_document_and_tells_the_provider() {
    let world = World::new();
    world.goals().create(&create_input("Root Goal", "")).unwrap();
    world.goals().create(&create_input("Old Name", "ROOT-GOAL")).unwrap();
    let changed = world
        .goals()
        .change(&GoalChangeInput { id: "ROOT-GOAL/OLD-NAME".to_string(), title: Some("New Name".to_string()), description: Some("fresh".to_string()), ..Default::default() })
        .unwrap();
    assert_eq!((changed.id.as_str(), changed.title.as_str(), changed.description.as_str()), ("ROOT-GOAL/NEW-NAME", "New Name", "fresh"));
    assert_eq!(world.store.ids(), vec!["ROOT-GOAL", "ROOT-GOAL/NEW-NAME"]);
    assert_eq!(world.management.calls().last().unwrap(), "update-goal-issue https://example.test/repo/issues/101 New Name fresh");
    assert_eq!(world.kinds().last().unwrap(), "goal.change.ended");
}

#[test]
fn a_slug_is_not_accepted_as_a_title() {
    let world = World::new();
    world.goals().create(&create_input("Root Goal", "")).unwrap();
    for (title, class) in [("ROOT-GOAL", "title"), ("root-goal", "title"), ("   ", "title")] {
        let error = world.goals().change(&GoalChangeInput { id: "ROOT-GOAL".to_string(), title: Some(title.to_string()), ..Default::default() }).unwrap_err();
        assert_eq!(error.class(), class, "{title:?}");
    }
    assert_eq!(world.store.ids(), vec!["ROOT-GOAL"]);
}

#[test]
fn re_parenting_moves_the_whole_subtree_and_refuses_a_collision() {
    let world = World::new();
    let quiet = |title: &str, parent: &str| {
        let mut input = create_input(title, parent);
        input.no_management = true;
        world.goals().create(&input).unwrap();
    };
    quiet("Left", "");
    quiet("Right", "");
    quiet("Branch", "LEFT");
    quiet("Twig", "LEFT/BRANCH");
    quiet("Branch", "RIGHT");
    let moved = world.goals().change(&GoalChangeInput { id: "LEFT/BRANCH".to_string(), parent: Some("".to_string()), no_management: true, ..Default::default() }).unwrap();
    assert_eq!(moved.id, "BRANCH");
    assert_eq!(world.store.ids(), vec!["BRANCH", "BRANCH/TWIG", "LEFT", "RIGHT", "RIGHT/BRANCH"]);
    let collision = world.goals().change(&GoalChangeInput { id: "BRANCH".to_string(), parent: Some("RIGHT".to_string()), no_management: true, ..Default::default() }).unwrap_err();
    assert_eq!(collision.class(), "already-exists");
}

#[test]
fn closing_twice_and_reopening_an_open_goal_are_refused() {
    let world = World::new();
    world.goals().create(&create_input("Root Goal", "")).unwrap();
    world.goals().create(&create_input("Child Goal", "ROOT-GOAL")).unwrap();
    let closed = world.goals().close(&GoalCloseInput { id: "ROOT-GOAL/CHILD-GOAL".to_string(), summary: "done".to_string(), no_management: false }).unwrap();
    assert_eq!((closed.status, closed.summary.as_str()), (GoalStatus::Closed, "done"));
    assert_eq!(world.management.calls().last().unwrap(), "close-issue https://example.test/repo/issues/101");
    let again = world.goals().close(&GoalCloseInput { id: "ROOT-GOAL/CHILD-GOAL".to_string(), summary: "twice".to_string(), no_management: false }).unwrap_err();
    assert_eq!((again.class(), again.to_string().as_str()), ("already-in-state", "goal is already closed"));
    let reopen = GoalReopenInput { id: "ROOT-GOAL/CHILD-GOAL".to_string(), prompt: "again".to_string(), client: "claude-code".to_string(), llm: "opus-4-6".to_string(), effort: "max".to_string(), ..Default::default() };
    let reopened = world.goals().reopen(&reopen).unwrap();
    assert_eq!((reopened.status, reopened.prompt.as_str(), reopened.llm.as_str(), reopened.effort.as_str()), (GoalStatus::Open, "again", "opus-4-6", "max"));
    assert_eq!(world.goals().reopen(&reopen).unwrap_err().to_string(), "goal is already open");
    assert_eq!(world.kinds()[2..], ["goal.close.ended", "goal.reopen.ended"]);
}

#[test]
fn closing_a_root_goal_closes_its_milestone() {
    let world = World::new();
    world.goals().create(&create_input("Root Goal", "")).unwrap();
    world.goals().close(&GoalCloseInput { id: "ROOT-GOAL".to_string(), summary: "shipped".to_string(), no_management: false }).unwrap();
    assert_eq!(world.management.calls().last().unwrap(), "update-milestone 100 Root Goal Root Goal description closed ");
}

#[test]
fn deleting_retires_the_provider_object_and_the_whole_subtree() {
    let world = World::new();
    world.goals().create(&create_input("Root Goal", "")).unwrap();
    world.goals().create(&create_input("Child Goal", "ROOT-GOAL")).unwrap();
    assert!(world.goals().delete(&GoalDeleteInput { id: "ROOT-GOAL".to_string(), no_management: false }).unwrap());
    assert_eq!(world.management.calls().last().unwrap(), "delete-milestone 100");
    assert!(world.store.ids().is_empty());
    assert_eq!(world.goals().read("ROOT-GOAL").unwrap_err().to_string(), "goal file not found: ROOT-GOAL");
    assert_eq!(world.goals().delete(&GoalDeleteInput { id: "ROOT-GOAL".to_string(), no_management: false }).unwrap_err().class(), "not-found");
}

#[test]
fn deleting_a_child_deletes_its_issue_by_number() {
    let world = World::new();
    world.goals().create(&create_input("Root Goal", "")).unwrap();
    world.goals().create(&create_input("Child Goal", "ROOT-GOAL")).unwrap();
    world.goals().delete(&GoalDeleteInput { id: "ROOT-GOAL/CHILD-GOAL".to_string(), no_management: false }).unwrap();
    assert_eq!(world.management.calls().last().unwrap(), "delete-issue 101");
    assert_eq!(world.store.ids(), vec!["ROOT-GOAL"]);
}

#[test]
fn the_aggregate_lists_reads_and_searches() {
    let world = World::new();
    for (title, parent) in [("Alpha Goal", ""), ("Beta Goal", "ALPHA-GOAL"), ("Gamma Goal", "")] {
        let mut input = create_input(title, parent);
        input.no_management = true;
        input.description = format!("about {}", title.to_lowercase());
        world.goals().create(&input).unwrap();
    }
    let ids: Vec<String> = world.goals().list().unwrap().into_iter().map(|goal| goal.id).collect();
    assert_eq!(ids, vec!["ALPHA-GOAL", "ALPHA-GOAL/BETA-GOAL", "GAMMA-GOAL"]);
    let flat_store = MemoryGoalStore::seeded([("alphagoal/betagoal".to_string(), "{\"title\":\"Beta\",\"description\":\"\",\"prompt\":\"\",\"status\":\"open\",\"client\":\"claude-code\",\"llm\":\"opus-4-5\"}".to_string())]);
    let flat_goals = Goals::new(&flat_store, &world.management, &world.emitter, "ueli");
    let beta = flat_goals.read(&goal_path_to_compose_id("alphagoal/betagoal")).unwrap();
    assert_eq!((beta.id.as_str(), beta.parent.as_str(), beta.title.as_str()), ("alphagoal/betagoal", "alphagoal", "Beta"));
    let found: Vec<String> = world.goals().search("BETA").unwrap().into_iter().map(|goal| goal.id).collect();
    assert_eq!(found, vec!["ALPHA-GOAL/BETA-GOAL"]);
    assert_eq!(world.goals().search("").unwrap().len(), 3);
    assert_eq!(world.goals().search("closed").unwrap().len(), 0);
    assert_eq!(world.goals().search("OPEN").unwrap().len(), 3);
}
//#endregion 🔓️Lifecycle

//#region 🌳️Tree
fn answers_forest() -> (Value, Vec<GoalNode>) {
    let answers = fixture("🔬️unit-vectors.json");
    let tree = fixture("🌳️tree-vectors.json");
    let forest = build_goal_tree(&seeds::<GoalSeed>(&tree["goals"]), &seeds::<TicketSeed>(&tree["tickets"]));
    (answers, forest)
}

#[test]
fn the_forest_renders_as_the_independent_model_lines() {
    let (answers, forest) = answers_forest();
    assert_eq!(render_goal_tree(&forest, TreeFormat::Text, &PlainTreeLines), text(&answers["tree"], "text"));
    assert_eq!(render_goal_tree(&forest, TreeFormat::Markdown, &PlainTreeLines), text(&answers["tree"], "markdown"));
}

#[test]
fn the_open_counts_agree_with_the_independent_model_at_every_node() {
    let (answers, forest) = answers_forest();
    fn walk(node: &GoalNode, out: &mut Vec<(String, usize, usize)>) {
        out.push((node.id.clone(), count_open_subgoals(node), count_open_tickets(node)));
        for child in node.children.as_deref().unwrap_or_default() {
            walk(child, out);
        }
    }
    let mut actual = Vec::new();
    for root in &forest {
        walk(root, &mut actual);
    }
    let expected: Vec<(String, usize, usize)> =
        answers["tree"]["counts"].as_array().unwrap().iter().map(|row| (text(row, "id").to_string(), row["openSubgoals"].as_u64().unwrap() as usize, row["openTickets"].as_u64().unwrap() as usize)).collect();
    assert_eq!(actual, expected);
}

#[test]
fn an_empty_forest_renders_nothing_and_a_goal_without_tickets_has_no_ticket_list() {
    assert_eq!(render_goal_tree(&[], TreeFormat::Text, &PlainTreeLines), "");
    let forest = build_goal_tree(&[GoalSeed { id: "A".into(), title: "A".into(), status: "open".into(), ..Default::default() }], &[]);
    assert_eq!(forest.len(), 1);
    assert!(forest[0].children.is_none() && forest[0].tickets.is_none());
    assert_eq!(render_goal_tree(&forest, TreeFormat::Markdown, &PlainTreeLines), "- goal A A [open] \n");
}

#[test]
fn a_ticket_that_names_itself_as_parent_stays_a_root_ticket() {
    let ticket = TicketSeed { id: "t".into(), slug: "SELF".into(), status: "open".into(), title: "Self".into(), goal: "A".into(), parent: "t".into(), ..Default::default() };
    let forest = build_goal_tree(&[GoalSeed { id: "A".into(), ..Default::default() }], &[ticket]);
    let tickets = forest[0].tickets.as_ref().unwrap();
    assert_eq!(tickets.len(), 1);
    assert!(tickets[0].children.is_none());
}
//#endregion 🌳️Tree
