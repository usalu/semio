use super::resolve_ready;
use std::cell::Cell;
use std::future::poll_fn;
use std::task::Poll;

#[test]
fn single_poll_matches_all_language_neutral_rows_and_preserves_poll_count() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let polls = Cell::new(0);
        let future = poll_fn(|_| {
            polls.set(polls.get() + 1);
            if row["state"] == "Ready" { Poll::Ready(row["value"].clone()) } else { Poll::Pending }
        });
        let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| resolve_ready(future)));
        if row["expected"]["state"] == "Ready" { assert_eq!(result.unwrap(), row["expected"]["value"]); } else { assert!(result.is_err()); }
        assert_eq!(polls.get(), row["expected"]["polls"].as_u64().unwrap());
    }
}
