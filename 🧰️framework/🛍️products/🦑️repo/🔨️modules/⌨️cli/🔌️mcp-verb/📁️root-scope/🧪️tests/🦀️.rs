use super::RepoRepository;
use semio_framework_repo_mcp::{repository_server, serve, Limits, Profile};
use serde_json::{json, Value};
use std::collections::BTreeMap;
use std::io::{self, Write};
use std::path::{Component, PathBuf};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
struct Wire(Arc<Mutex<Vec<u8>>>);

impl Write for Wire {
    fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
        let mut output = self.0.lock().expect("wire lock");
        if output.len() + bytes.len() > 65536 { return Err(io::Error::other("bounded MCP wire required")); }
        output.extend_from_slice(bytes);
        Ok(bytes.len())
    }
    fn flush(&mut self) -> io::Result<()> { Ok(()) }
}

struct Fixture(PathBuf);

impl Drop for Fixture {
    fn drop(&mut self) { let _ = std::fs::remove_dir_all(&self.0); }
}

fn fixture_root() -> Fixture {
    let parent = PathBuf::from(std::env::var_os("SEMIO_TEST_ARTIFACT_DIR").expect("owned test artifacts required"));
    assert!(parent.to_string_lossy().encode_utf16().count() <= 256);
    std::fs::create_dir_all(&parent).expect("artifact root");
    for serial in 0..128 {
        let root = parent.join(format!("mr-{}-{serial}", std::process::id()));
        assert!(root.to_string_lossy().encode_utf16().count() <= 256);
        match std::fs::create_dir(&root) {
            Ok(()) => return Fixture(root),
            Err(error) if error.kind() == io::ErrorKind::AlreadyExists => {}
            Err(error) => panic!("owned fixture: {error}"),
        }
    }
    panic!("finite fixture allocation exhausted");
}

fn texts(value: &Value) -> Vec<String> {
    value.as_array().expect("string array").iter().map(|item| item.as_str().expect("string").to_owned()).collect()
}

#[test]
fn provided_root_bounds_real_repository_mcp_files_and_folders() {
    let fixture: Value = serde_json::from_str(include_str!("🧫️fixtures/🔣️.json")).expect("portable fixture");
    let schema: Value = serde_json::from_str(include_str!("../🧬️schema/🔣️.json")).expect("root scope schema");
    let resource_schema: Value = serde_json::from_str(include_str!("../📄️resources/🧬️schema/🔣️.json")).expect("closed resource schema");
    let policy = &schema["$defs"]["Policy"]["const"];
    assert_eq!(fixture["version"], schema["properties"]["version"]["const"]);
    assert_eq!(fixture["profiles"], schema["properties"]["profiles"]["const"]);
    assert_eq!(fixture["reads"], schema["properties"]["reads"]["const"]);
    let original_cwd = std::env::current_dir().expect("cwd");
    let owned = fixture_root();
    let cases = fixture["cases"].as_array().expect("cases");
    assert_eq!(cases.len() as u64, policy["maximumRoots"].as_u64().expect("root bound"));
    for case in cases {
        let root = owned.0.join(case["name"].as_str().expect("case name"));
        assert!(root.to_string_lossy().encode_utf16().count() <= 256);
        std::fs::create_dir(&root).expect("private root");
        let files = case["files"].as_array().expect("files");
        assert!(files.len() <= 4);
        for file in files {
            let relative = file["path"].as_str().expect("relative path");
            assert!(relative.len() <= 64 && !relative.starts_with('.') && PathBuf::from(relative).components().all(|part| matches!(part, Component::Normal(_))));
            let path = root.join(relative);
            assert!(path.to_string_lossy().encode_utf16().count() <= 256);
            std::fs::create_dir_all(path.parent().expect("parent")).expect("fixture parents");
            let text = file["text"].as_str().expect("fixture text");
            assert!(text.len() <= 256);
            std::fs::write(path, text).expect("handcrafted fixture");
        }
    }
    for profile in texts(&fixture["profiles"]) {
        let profile = Profile::parse(&profile).expect("explicit profile");
        for case in cases {
            let root = owned.0.join(case["name"].as_str().expect("case name"));
            let server = Arc::new(repository_server(RepoRepository::open(&root, profile), profile, Limits::default()).expect("actual server"));
            let mut requests = vec![json!({"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":fixture["protocolVersion"],"capabilities":{},"clientInfo":{"name":"root-scope-test","version":"1"}}}), json!({"jsonrpc":"2.0","method":"notifications/initialized"})];
            for read in fixture["reads"].as_array().expect("reads") { requests.push(json!({"jsonrpc":"2.0","id":read["id"],"method":"resources/read","params":{"uri":read["uri"]}})); }
            let input = requests.iter().map(Value::to_string).collect::<Vec<_>>().join("\n") + "\n";
            let output = Wire::default();
            serve(&server, "root-scope", input.as_bytes(), output.clone()).expect("actual serve and EOF settlement");
            let bytes = output.0.lock().expect("wire result").clone();
            let wire = std::str::from_utf8(&bytes).expect("UTF-8 wire");
            let mut responses = BTreeMap::new();
            for line in wire.lines() {
                let response: Value = serde_json::from_str(line).expect("independent wire JSON oracle");
                assert_eq!(response["jsonrpc"], "2.0");
                assert!(response.get("error").is_none(), "{response}");
                assert!(responses.insert(response["id"].as_i64().expect("response id"), response).is_none());
            }
            assert_eq!(responses.keys().copied().collect::<Vec<_>>(), vec![1, 11, 12]);
            assert_eq!(responses[&1]["result"]["serverInfo"]["name"], "repo");
            for read in fixture["reads"].as_array().expect("reads") {
                let id = read["id"].as_i64().expect("read id");
                let contents = responses[&id]["result"]["contents"].as_array().expect("resource contents");
                assert_eq!(contents.len(), 1);
                assert_eq!(contents[0]["uri"], read["uri"]);
                assert_eq!(contents[0]["mimeType"], fixture["mimeType"]);
                let document: Value = serde_json::from_str(contents[0]["text"].as_str().expect("actual JSON resource")).expect("independent JSON resource oracle");
                let field = read["field"].as_str().expect("field");
                let required = texts(&resource_schema["$defs"][if field == "files" { "File" } else { "Folder" }]["required"]);
                for row in document["repo"][field].as_array().expect("resource rows") { let object = row.as_object().expect("resource object"); let mut keys = object.keys().cloned().collect::<Vec<_>>(); keys.sort(); let mut expected = required.clone(); expected.sort(); assert_eq!(keys, expected); assert!(object.values().all(Value::is_string)); }
                let mut actual = document["repo"][field].as_array().expect("rooted resource rows").iter().map(|row| row["path"].as_str().expect("relative returned path").to_owned()).collect::<Vec<_>>();
                actual.sort();
                let mut expected = if field == "files" { case["files"].as_array().expect("files").iter().map(|file| file["path"].as_str().expect("path").to_owned()).collect() } else { texts(&case["folders"]) };
                expected.sort();
                assert_eq!(actual, expected, "root={} resource={field}", root.display());
            }
            assert_eq!(std::env::current_dir().expect("cwd after serve"), original_cwd);
            for file in case["files"].as_array().expect("files") { assert_eq!(std::fs::read_to_string(root.join(file["path"].as_str().expect("path"))).expect("unchanged fixture"), file["text"].as_str().expect("text")); }
        }
    }
}

#[test]
fn ticket_tool_results_preserve_canonical_document_and_path() {
    let fixture: Value = serde_json::from_str(include_str!("../🎫️tickets/🧪️tests/🧫️fixtures/🔣️.json")).expect("shared ticket outcomes");
    for row in fixture["cases"].as_array().expect("cases") {
        let mut ticket: semio_framework_repo_model::Ticket = serde_json::from_value(row["ticket"].clone()).expect("canonical ticket");
        ticket.year = row["year"].as_i64().expect("year");
        ticket.month = row["month"].as_i64().expect("month");
        ticket.day = row["day"].as_i64().expect("day");
        ticket.slug = row["slug"].as_str().expect("slug").into();
        let result = super::ticket_result(&ticket).expect("ticket result");
        assert!(!result.is_error && result.structured.is_none());
        let actual: Value = serde_json::from_str(&result.text).expect("third-party JSON oracle");
        assert_eq!(actual, row["expected"]);
    }
}
