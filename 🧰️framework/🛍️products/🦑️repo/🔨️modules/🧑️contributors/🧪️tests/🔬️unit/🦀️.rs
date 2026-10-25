//! 🔬️ Cargo tests of the contributors crate. Identity lines, author lines, alias resolution, the
//! checkpoint log and the sessions are judged against `🧫️fixtures/🔬️unit-vectors.json`, which an
//! independent Python model computed over the real contributor documents and the real `git log`;
//! the checkpoint source and the git identity against a real `git` repository with fixed dates.

use super::*;
use semio_framework_repo_events::Emitter;
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

fn answers() -> Value {
    fixture("🔬️unit-vectors.json")
}

fn rows<'a>(value: &'a Value, key: &str) -> &'a Vec<Value> {
    value[key].as_array().unwrap_or_else(|| panic!("{key} is not an array"))
}

fn text<'a>(value: &'a Value, key: &str) -> &'a str {
    value[key].as_str().unwrap_or_else(|| panic!("member {key} is not a string in {value}"))
}

fn store() -> MemoryContributorStore {
    let documents = fixture("🧑️‍💻️contributor-documents.json");
    MemoryContributorStore::seeded(rows(&documents, "documents").iter().map(|entry| (text(entry, "directory").to_string(), text(entry, "json").to_string())))
}

struct Scratch(PathBuf);

impl Scratch {
    fn new(tag: &str) -> Scratch {
        let path = std::env::temp_dir().join(format!("semio-repo-contributors-{tag}-{}", std::process::id()));
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

fn contributor(alias: &str, name: &str, email: &str) -> Contributor {
    serde_json::from_value(serde_json::json!({"alias": alias, "name": name, "email": email})).unwrap()
}
//#endregion 📥️Fixtures

//#region ✍️Authors
#[test]
fn every_identity_line_parses_like_the_independent_regular_expression() {
    let answers = answers();
    assert!(rows(&answers, "identities").len() >= 25);
    for row in rows(&answers, "identities") {
        let expected = row["parsed"].as_array().map(|pair| (pair[0].as_str().unwrap().to_string(), pair[1].as_str().unwrap().to_string()));
        assert_eq!(parse_contributor_identity(text(row, "line")), expected, "{:?}", row["line"]);
    }
}

#[test]
fn every_author_line_splits_into_name_and_email_and_renders_back() {
    let answers = answers();
    for row in rows(&answers, "authors") {
        let line = text(row, "line");
        let parsed = parse_git_author(line);
        assert_eq!((parsed.name.as_str(), parsed.email.as_str()), (text(row, "name"), text(row, "email")), "{line:?}");
        let rendered = parsed.to_string();
        assert_eq!(rendered, if parsed.email.is_empty() { parsed.name.clone() } else { format!("{} <{}>", parsed.name, parsed.email) });
        assert!(parsed.github.is_empty());
    }
}
//#endregion ✍️Authors

//#region 🧑️Contributors
#[test]
fn the_registry_resolves_every_author_line_like_the_independent_model() {
    let answers = answers();
    let store = store();
    for row in rows(&answers, "authors") {
        assert_eq!(find_contributor(&store, text(row, "line")), text(row, "alias"), "{:?}", row["line"]);
    }
}

#[test]
fn listing_reads_the_real_documents_in_directory_order_with_directory_defaults() {
    let answers = answers();
    let listed = list_contributors(&store());
    let expected = rows(&answers, "contributors");
    assert_eq!(listed.len(), expected.len());
    for (contributor, row) in listed.iter().zip(expected) {
        assert_eq!((contributor.alias.as_str(), contributor.github.as_str(), contributor.name.as_str(), contributor.email.as_str()), (text(row, "alias"), text(row, "github"), text(row, "name"), text(row, "email")));
        assert_eq!(contributor.emails, rows(row, "emails").iter().map(|email| email.as_str().unwrap().to_string()).collect::<Vec<_>>());
    }
}

#[test]
fn an_empty_registry_has_the_single_unknown_contributor_and_a_broken_document_is_skipped() {
    let unknown = list_contributors(&MemoryContributorStore::default());
    assert_eq!(unknown.len(), 1);
    assert_eq!((unknown[0].alias.as_str(), unknown[0].github.as_str(), unknown[0].name.as_str(), unknown[0].email.as_str()), ("unknown", "unknown", "Unknown", ""));
    let seeded = MemoryContributorStore::seeded([("b-dir".to_string(), "{\"name\":\"Bee\"}".to_string()), ("a-broken".to_string(), "not json".to_string()), ("c-dir".to_string(), "{\"alias\":\"cee\",\"github\":\"gh\"}".to_string())]);
    let listed = list_contributors(&seeded);
    assert_eq!(listed.iter().map(|c| (c.alias.as_str(), c.github.as_str())).collect::<Vec<_>>(), vec![("b-dir", "b-dir"), ("cee", "gh")]);
}

#[test]
fn search_matches_alias_name_and_handle_case_insensitively() {
    let store = store();
    let found = |term: &str| search_contributors(&store, term).into_iter().map(|c| c.alias).collect::<Vec<_>>();
    assert_eq!(found("UEL"), vec!["ueli"]);
    assert_eq!(found("saluz"), vec!["ueli"]);
    assert_eq!(found("kinansarak"), vec!["kinan"]);
    assert_eq!(found(""), list_contributors(&store).into_iter().map(|c| c.alias).collect::<Vec<_>>());
    assert!(found("no such person at all").is_empty());
}

#[test]
fn contributors_are_tried_in_directory_order_each_by_email_first_and_blank_parts_never_match() {
    let store = MemoryContributorStore::seeded([
        ("first".to_string(), "{\"alias\":\"first\",\"name\":\"Same Name\",\"email\":\"one@x.org\"}".to_string()),
        ("second".to_string(), "{\"alias\":\"second\",\"name\":\"Other\",\"email\":\"two@x.org\",\"emails\":[\"alt@x.org\"],\"names\":[\"Nick\"]}".to_string()),
    ]);
    assert_eq!(resolve_author_to_alias(&store, "Same Name", "alt@x.org"), "first", "the earlier contributor claims the name before the later one is asked for the email");
    assert_eq!(resolve_author_to_alias(&store, "Other", "ONE@X.ORG"), "first");
    assert_eq!(resolve_author_to_alias(&store, "Stranger", "alt@x.org"), "second");
    assert_eq!(resolve_author_to_alias(&store, "same name", ""), "first");
    assert_eq!(resolve_author_to_alias(&store, "NICK", ""), "second");
    assert_eq!(resolve_author_to_alias(&store, "", ""), "");
    assert_eq!(resolve_author_to_alias(&store, "Stranger", ""), "Stranger");
    assert_eq!(resolve_author_to_alias(&store, "", "who@x.org"), "who@x.org");
    assert_eq!(resolve_author_to_alias(&store, "Stranger", "who@x.org"), "Stranger <who@x.org>");
    assert_eq!(find_contributor(&store, ""), "unknown");
    assert_eq!(find_contributor(&store, "Other <TWO@X.ORG>"), "second");
}

#[test]
fn the_filesystem_store_reads_directories_with_a_document_in_directory_order() {
    let scratch = Scratch::new("fs-store");
    let documents = rows(&fixture("🧑️‍💻️contributor-documents.json"), "documents").clone();
    for entry in &documents {
        let directory = contributor_dir(&scratch.0, text(entry, "directory"));
        std::fs::create_dir_all(&directory).unwrap();
        std::fs::write(directory.join("🧑️‍💻️contributor.json"), text(entry, "json")).unwrap();
    }
    std::fs::create_dir_all(contributor_dir(&scratch.0, "no-document")).unwrap();
    std::fs::write(contributor_dir(&scratch.0, "").join("stray-file.txt"), "x").unwrap();
    let fs_store = FsContributorStore::new(&scratch.0);
    let names: Vec<String> = fs_store.documents().into_iter().map(|(name, _)| name).collect();
    let mut expected: Vec<String> = documents.iter().map(|entry| text(entry, "directory").to_string()).collect();
    expected.sort();
    assert_eq!(names, expected);
    assert_eq!(list_contributors(&fs_store).iter().map(|c| c.alias.clone()).collect::<Vec<_>>(), list_contributors(&store()).iter().map(|c| c.alias.clone()).collect::<Vec<_>>());
    assert!(FsContributorStore::new(&scratch.0.join("missing")).documents().is_empty());
}

#[test]
fn a_contributor_is_created_saved_loaded_and_removed_with_the_go_wording() {
    let scratch = Scratch::new("writes");
    let root = &scratch.0;
    assert_eq!(load_contributor(root, "ada").unwrap_err(), "contributor not found: ada");
    let created = create_contributor(root, "ada").unwrap();
    assert_eq!(created.alias, "ada");
    assert_eq!(create_contributor(root, "ada").unwrap_err(), "contributor already exists: ada");
    let document = std::fs::read_to_string(contributor_document(root, "ada")).unwrap();
    assert!(document.starts_with("{\n  \"alias\": \"ada\""), "two-space indentation: {document}");
    let mut saved = contributor("", "Ada Lovelace", "ada@example.org");
    saved.github = "ada-gh".to_string();
    save_contributor(root, &saved).unwrap();
    let loaded = load_contributor(root, "ada-gh").unwrap();
    assert_eq!((loaded.alias.as_str(), loaded.name.as_str(), loaded.email.as_str()), ("ada-gh", "Ada Lovelace", "ada@example.org"));
    assert_eq!(serde_json::from_str::<Value>(&std::fs::read_to_string(contributor_document(root, "ada-gh")).unwrap()).unwrap()["email"], "ada@example.org");
    remove_contributor(root, "ada").unwrap();
    assert_eq!(remove_contributor(root, "ada").unwrap_err(), "contributor not found: ada");
    assert!(!contributor_dir(root, "ada").exists() && contributor_dir(root, "ada-gh").exists());
    std::fs::write(contributor_document(root, "ada-gh"), "{\"name\":\"No Alias\"}").unwrap();
    assert_eq!(load_contributor(root, "ada-gh").unwrap().alias, "ada-gh", "a blank alias takes the directory name");
}
//#endregion 🧑️Contributors

//#region 🏁️Checkpoints
fn expected_checkpoints() -> Vec<Value> {
    rows(&answers(), "checkpoints").clone()
}

#[test]
fn the_real_log_parses_into_the_independent_checkpoint_rows() {
    let answers = answers();
    let parsed = parse_checkpoint_log(text(&answers, "checkpointLog"));
    let expected = expected_checkpoints();
    assert!(parsed.len() > 10, "the real log carries more than ten checkpoints");
    assert_eq!(parsed.len(), expected.len());
    for (checkpoint, row) in parsed.iter().zip(&expected) {
        assert_eq!((checkpoint.id.as_str(), checkpoint.sha.as_str(), checkpoint.title.as_str(), checkpoint.date.as_str()), (text(row, "id"), text(row, "sha"), text(row, "title"), text(row, "date")));
        assert_eq!(checkpoint.author_id.as_deref(), row["authorId"].as_str());
        assert_eq!(checkpoint_id(checkpoint), text(row, "identifier"));
    }
}

#[test]
fn the_malformed_lines_of_the_real_log_fixture_are_rejected_not_guessed() {
    let log = fixture("🏁️checkpoint-log.json");
    let malformed = rows(&log, "malformed").iter().map(|line| line.as_str().unwrap()).collect::<Vec<_>>().join("\n");
    assert!(parse_checkpoint_log(&malformed).iter().all(|checkpoint| checkpoint.sha.is_empty() || checkpoint.sha == "|"), "{malformed:?}");
    assert!(parse_checkpoint_log("").is_empty());
    assert!(parse_checkpoint_log("only|three|parts").is_empty());
    let kept = parse_checkpoint_log("a|b|c|d|e|f");
    assert_eq!((kept[0].title.as_str(), kept[0].author_id.as_deref()), ("d|e|f", Some("b")));
}

#[test]
fn the_memory_source_serves_the_frozen_log_and_honours_the_limit() {
    let answers = answers();
    let source = MemoryCheckpointSource::new(text(&answers, "checkpointLog"));
    let all = list_checkpoints(&source, None);
    assert_eq!(all.len(), expected_checkpoints().len());
    let limited = list_checkpoints(&source, Some(3));
    assert_eq!(limited.iter().map(|c| c.sha.clone()).collect::<Vec<_>>(), all.iter().take(3).map(|c| c.sha.clone()).collect::<Vec<_>>());
    assert!(list_checkpoints(&source, Some(0)).is_empty());
    assert_eq!(list_checkpoints(&MemoryCheckpointSource::default(), None).len(), 0);
}

#[test]
fn checkpoint_search_matches_the_sha_or_the_title_case_insensitively() {
    let source = MemoryCheckpointSource::new("AbC123|Ann|2026-01-01T00:00:00+00:00|Fix THE thing\ndef456|Bob|2026-01-02T00:00:00+00:00|Add feature");
    let shas = |term: &str| search_checkpoints(&source, None, term).into_iter().map(|c| c.sha).collect::<Vec<_>>();
    assert_eq!(shas("abc"), vec!["AbC123"]);
    assert_eq!(shas("THE THING"), vec!["AbC123"]);
    assert_eq!(shas("FEATURE"), vec!["def456"]);
    assert_eq!(shas("").len(), 2);
    assert!(shas("zzz").is_empty());
    assert_eq!(search_checkpoints(&source, Some(1), "").len(), 1);
}

fn git(directory: &Path, arguments: &[&str], date: Option<&str>) -> String {
    let mut command = std::process::Command::new("git");
    command.args(["-c", "commit.gpgsign=false", "-c", "core.autocrlf=false"]).args(arguments).current_dir(directory).env("GIT_CONFIG_NOSYSTEM", "1");
    if let Some(date) = date {
        command.env("GIT_AUTHOR_DATE", date).env("GIT_COMMITTER_DATE", date);
    }
    let outcome = command.output().unwrap_or_else(|error| panic!("git is required to run this suite: {error}"));
    assert!(outcome.status.success(), "git {arguments:?} failed: {}", String::from_utf8_lossy(&outcome.stderr));
    String::from_utf8_lossy(&outcome.stdout).trim().to_string()
}

fn repository_with_commits(scratch: &Scratch) -> Vec<(&'static str, &'static str, &'static str)> {
    git(&scratch.0, &["init", "-q"], None);
    git(&scratch.0, &["config", "user.name", "Ada Lovelace"], None);
    git(&scratch.0, &["config", "user.email", "ada@example.org"], None);
    let commits = vec![("Ada Lovelace", "2026-01-02T03:04:05+02:00", "first | piped title"), ("Grace Hopper", "2026-01-03T04:05:06+02:00", "second"), ("Ada Lovelace", "2026-01-04T05:06:07+02:00", "third")];
    for (author, date, title) in &commits {
        git(&scratch.0, &["-c", &format!("user.name={author}"), "-c", "user.email=x@example.org", "commit", "--allow-empty", "-m", title], Some(date));
    }
    commits
}

#[test]
fn the_git_source_reads_the_checkpoints_a_real_repository_holds_newest_first() {
    let scratch = Scratch::new("git-log");
    let commits = repository_with_commits(&scratch);
    let source = GitCheckpoints::new(&scratch.0);
    let listed = list_checkpoints(&source, None);
    let shas: Vec<String> = git(&scratch.0, &["rev-list", "HEAD"], None).lines().map(str::to_string).collect();
    assert_eq!(listed.iter().map(|c| c.sha.clone()).collect::<Vec<_>>(), shas);
    for (checkpoint, (author, date, title)) in listed.iter().zip(commits.iter().rev()) {
        assert_eq!((checkpoint.author_id.as_deref(), checkpoint.date.as_str(), checkpoint.title.as_str()), (Some(*author), *date, *title));
    }
    assert_eq!(list_checkpoints(&source, Some(2)).len(), 2);
    assert_eq!(search_checkpoints(&source, None, "piped").len(), 1);
    assert_eq!(list_checkpoints(&GitCheckpoints::new(scratch.0.join("missing")), None).len(), 0);
}

#[test]
fn the_git_identity_resolves_through_the_registry_or_falls_back_to_the_author_line() {
    let scratch = Scratch::new("git-identity");
    git(&scratch.0, &["init", "-q"], None);
    git(&scratch.0, &["config", "user.name", "Ada Lovelace"], None);
    git(&scratch.0, &["config", "user.email", "ada@example.org"], None);
    assert_eq!(git_author_alias(&scratch.0), "Ada Lovelace <ada@example.org>");
    save_contributor(&scratch.0, &contributor("ada", "Ada Lovelace", "ada@example.org")).unwrap();
    assert_eq!(git_author_alias(&scratch.0), "ada");
    git(&scratch.0, &["config", "user.email", "other@example.org"], None);
    assert_eq!(git_author_alias(&scratch.0), "ada", "the name still claims the alias");
    git(&scratch.0, &["config", "user.name", "Somebody Else"], None);
    assert_eq!(git_author_alias(&scratch.0), "Somebody Else <other@example.org>");
}
//#endregion 🏁️Checkpoints

//#region 🪅️Sessions
fn session_source() -> MemorySessionSource {
    let vectors = fixture("📡️session-vectors.json");
    MemorySessionSource::seeded(rows(&vectors, "sessions").iter().map(|entry| {
        let key = SessionKey { year: entry["year"].as_i64().unwrap(), month: entry["month"].as_i64().unwrap(), day: entry["day"].as_i64().unwrap(), uuid: text(entry, "uuid").to_string() };
        let files = entry["files"].as_object().unwrap().iter().map(|(name, content)| (name.clone(), content.as_str().unwrap().to_string())).collect();
        (key, MemorySession { files, age_seconds: entry["ageSeconds"].as_i64() })
    }))
}

fn session_answer<'a>(answers: &'a Value, uuid: &str) -> &'a Value {
    rows(answers, "sessions").iter().find(|row| text(row, "uuid") == uuid).unwrap_or_else(|| panic!("no answer for {uuid}"))
}

#[test]
fn every_session_vector_derives_the_independent_kind_client_second_and_checkpoint() {
    let source = session_source();
    let answers = answers();
    let keys = source.sessions();
    assert_eq!(keys.len(), 7);
    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(keys, sorted, "sessions are served in year, month, day, uuid order");
    for (year, month, day, uuid) in keys {
        let row = session_answer(&answers, &uuid);
        let key = SessionKey { year, month, day, uuid };
        assert_eq!(derive_session_kind(&source, &key).as_str(), text(row, "kind"), "{}", key.uuid);
        assert_eq!(extract_session_client(&source, &key), text(row, "client"), "{}", key.uuid);
        assert_eq!(extract_session_second(&source, &key), text(row, "second"), "{}", key.uuid);
        assert_eq!(extract_session_checkpoint(&source, &key), text(row, "checkpoint"), "{}", key.uuid);
    }
}

#[test]
fn the_listed_sessions_carry_the_derived_members_and_the_independent_identifiers() {
    let source = session_source();
    let answers = answers();
    let listed = list_sessions(&source);
    assert_eq!(listed.len(), 7);
    for session in &listed {
        let row = session_answer(&answers, &session.uuid);
        assert_eq!((session.kind.as_str(), session.client.as_str(), session.started_at.as_str(), session.checkpoint.as_str()), (text(row, "kind"), text(row, "client"), text(row, "second"), text(row, "checkpoint")));
        assert_eq!(session.id(), text(row, "id"), "{}", session.uuid);
        assert_eq!(session.uri(), text(row, "uri"));
    }
}

#[test]
fn session_search_matches_the_identifier_the_kind_and_the_client() {
    let source = session_source();
    let uuids = |term: &str| search_sessions(&source, term).into_iter().map(|session| session.uuid).collect::<Vec<_>>();
    assert_eq!(uuids("CODEX"), vec!["1111-running-recent-write"]);
    assert_eq!(uuids("completed"), vec!["4444-completed-by-file-name", "0000-completed-by-event"]);
    assert_eq!(uuids("").len(), 7);
    assert!(uuids("no-such-session").is_empty());
}

#[test]
fn a_session_is_running_until_it_has_been_silent_for_thirty_minutes() {
    let key = SessionKey { year: 26, month: 9, day: 6, uuid: "edge".to_string() };
    let source = |age: Option<i64>| MemorySessionSource::seeded([(key.clone(), MemorySession { files: BTreeMap::from([("0001-event.json".to_string(), "{\"event\":{\"kind\":\"tool.used\"}}".to_string())]), age_seconds: age })]);
    assert_eq!(RUNNING_WINDOW_SECONDS, 1800);
    assert_eq!(derive_session_kind(&source(Some(1799)), &key), SessionKind::Running);
    assert_eq!(derive_session_kind(&source(Some(1800)), &key), SessionKind::Interrupted);
    assert_eq!(derive_session_kind(&source(None), &key), SessionKind::Interrupted);
    assert_eq!(derive_session_kind(&MemorySessionSource::default(), &key), SessionKind::Interrupted);
}

#[test]
fn a_session_kind_has_its_wire_string_and_its_shared_emoji() {
    for (kind, wire, entity) in [(SessionKind::Running, "running", "session-running"), (SessionKind::Completed, "completed", "session-completed"), (SessionKind::Interrupted, "interrupted", "session-interrupted")] {
        assert_eq!((kind.as_str(), kind.to_string().as_str()), (wire, wire));
        assert_eq!(serde_json::to_value(kind).unwrap(), Value::String(wire.to_string()));
        assert_eq!(session_kind_emoji(Some(kind)), identity::entity(entity));
    }
    assert_eq!(session_kind_emoji(None), identity::entity("session"));
    assert_eq!(AGENT_ENDED_KIND, "agent.ended");
}
//#endregion 🪅️Sessions

//#region 🎁️Interactions
fn interaction(author: &str, checkpoint: &str) -> Interaction {
    serde_json::from_value(serde_json::json!({"kind": "started", "date": "d", "author": author, "system": "s", "client": "c", "checkpoint": checkpoint})).unwrap()
}

#[test]
fn the_first_interaction_that_names_an_author_or_a_checkpoint_decides() {
    let interactions = vec![interaction("", ""), interaction("Ann <a@x>", ""), interaction("Bob <b@x>", "cp-1"), interaction("", "cp-2")];
    assert_eq!(interactions_author(&interactions), "Ann <a@x>");
    assert_eq!(interactions_checkpoint(&interactions), "cp-1");
    assert_eq!(interactions_author(&[]), "");
    assert_eq!(interactions_checkpoint(&[interaction("", "")]), "");
}

#[test]
fn interactions_are_attributed_to_the_alias_their_author_resolves_to() {
    let origin = InteractionOrigin { source_kind: "ticket".into(), source_id: "26/09/06/X".into(), goal_id: "G".into(), ticket_id: "T".into() };
    let attributed = attribute_interactions(&store(), &origin, &[interaction("Ueli Saluz <ueli@semio-tech.com>", ""), interaction("Stranger <s@x.org>", ""), interaction("", "")]);
    assert_eq!(attributed.iter().map(|(_, alias)| alias.as_str()).collect::<Vec<_>>(), vec!["ueli", "Stranger <s@x.org>", ""]);
    assert!(attributed.iter().all(|(carried, _)| *carried == origin));
    let encoded = serde_json::to_value(&origin).unwrap();
    assert_eq!(encoded["sourceKind"], "ticket");
    assert_eq!(encoded["goalId"], "G");
    assert!(serde_json::to_value(InteractionOrigin::default()).unwrap().get("goalId").is_none(), "empty members are omitted");
}
//#endregion 🎁️Interactions

//#region 🕰️Ports
#[test]
fn the_clocks_tell_the_frozen_and_the_real_time() {
    assert_eq!(FixedClock { instant: 1_700_000_000 }.now_seconds(), 1_700_000_000);
    let first = SystemClock.now_seconds();
    assert!(first > 1_700_000_000, "the system clock is before 2023");
    assert!(SystemClock.now_seconds() >= first);
}

#[test]
fn the_memory_emitter_keeps_tab_separated_envelopes_in_order() {
    let emitter = MemoryEmitter::new();
    emitter.emit("contributor.add", "repo-cli", &serde_json::json!({"alias": "ada"}));
    emitter.emit("contributor.remove", "repo-cli", &serde_json::json!({}));
    assert_eq!(emitter.envelopes(), vec!["contributor.add\trepo-cli\t{\"alias\":\"ada\"}", "contributor.remove\trepo-cli\t{}"]);
}
//#endregion 🕰️Ports
