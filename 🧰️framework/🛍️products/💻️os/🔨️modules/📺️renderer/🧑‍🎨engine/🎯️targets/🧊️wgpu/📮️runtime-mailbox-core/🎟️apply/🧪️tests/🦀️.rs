use super::*;

#[test]
fn checked_out_completion_keeps_capacity_and_return_custody_through_refusal() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🔣️.json")).unwrap();
    let mut queue = BoundedCompletionQueue::<(), 4>::new();
    let mut held = None;
    let mut index = 0;
    let completion = |revision, restores_interaction| Completion { key: None, revision, requires_interaction: false, restores_interaction, apply: () };
    for owner in fixture["ready"].as_array().unwrap() { assert!(queue.enqueue(completion(owner["revision"].as_u64().unwrap(), owner["restoresInteraction"].as_bool().unwrap()))); }
    for step in fixture["steps"].as_array().unwrap() {
        match step["op"].as_str().unwrap() {
            "checkout" => { index = step["index"].as_u64().unwrap() as usize; held = queue.checkout_at(index); }
            "enqueue" => assert_eq!(queue.enqueue(completion(step["revision"].as_u64().unwrap(), false)), step["admitted"].as_bool().unwrap()),
            "pending" | "refuse" => { let (owner, checkout) = held.take().unwrap(); queue.return_checkout(index, owner, checkout); }
            "complete" => { let (_, checkout) = held.take().unwrap(); queue.finish_checkout(checkout); }
            _ => panic!("unknown authored checkout event"),
        }
        assert_eq!(queue.ready.iter().map(|owner| owner.revision).collect::<Vec<_>>(), step["ready"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap()).collect::<Vec<_>>());
        assert_eq!(held.as_ref().map(|(owner, _)| owner.revision), step["held"].as_u64());
        assert_eq!(queue.len(), step["length"].as_u64().unwrap() as usize);
        assert_eq!(queue.interaction_owner_outstanding(), step["ownerOutstanding"].as_bool().unwrap());
    }
}
