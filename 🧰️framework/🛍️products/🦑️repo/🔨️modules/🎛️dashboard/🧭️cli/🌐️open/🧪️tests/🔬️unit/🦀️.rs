use super::*;

fn vectors() -> serde_json::Value { serde_json::from_str(include_str!("../../../../🧫️fixtures/🌐️open/🔣️.json")).unwrap() }

#[test]
fn opener_argument_vectors_and_ready_tasks_match_the_language_neutral_contract() {
    let fixture = vectors();
    for row in fixture["openers"].as_array().unwrap() {
        let configured = serde_json::to_string(&row["value"]).unwrap();
        assert_eq!(plan("http://127.0.0.1:8791/health", Some(&configured), Platform::Windows).is_ok(), row["accepted"].as_bool().unwrap());
    }
    for row in fixture["tasks"].as_array().unwrap() {
        let status: dashboard_wire::ipc::SessionStatus = serde_json::from_value(row["status"].clone()).unwrap();
        assert_eq!(ready_url(status, row["url"].as_str()).is_some(), row["accepted"].as_bool().unwrap());
    }
    for (name, platform) in [("windows", Platform::Windows), ("macos", Platform::Macos), ("linux", Platform::Linux)] {
        let url = "http://localhost:6061/path?q=a%20b&next=x";
        let mut expected: Vec<String> = serde_json::from_value(fixture["platforms"][name].clone()).unwrap();
        expected.push(url.into());
        assert_eq!(plan(url, None, platform).unwrap(), expected);
    }
}

#[test]
fn a_cancelled_opener_never_starts_a_helper() {
    let cancellation = AtomicBool::new(true);
    assert_eq!(execute(&["nonexistent-opener".into()], &cancellation).unwrap_err().kind(), io::ErrorKind::Interrupted);
}
