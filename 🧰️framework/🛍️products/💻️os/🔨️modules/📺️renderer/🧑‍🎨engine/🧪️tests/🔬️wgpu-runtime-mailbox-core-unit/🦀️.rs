
use super::*;

fn completion(key: Option<&'static str>, revision: u64) -> Completion<()> {
    Completion { key, revision, requires_interaction: false, apply: () }
}

#[test]
fn lossless_and_coalesced_work_share_a_fixed_bound_with_an_interaction_reserve() {
    let mut queue = BoundedCompletionQueue::<(), 4>::new();
    assert!(queue.enqueue(completion(None, 1)));
    assert!(queue.reserve(None));
    assert!(queue.reserve(None));
    assert!(!queue.enqueue(completion(None, 2)));
    assert!(queue.reserve_interaction());
    assert!(!queue.reserve_interaction());

    let mut queue = BoundedCompletionQueue::<(), 4>::new();
    assert!(queue.enqueue(completion(Some("preview"), 1)));
    assert!(queue.enqueue(completion(Some("preview"), 2)));
    assert!(queue.enqueue(completion(Some("preview"), 3)));
    assert!(queue.enqueue(completion(Some("preview"), 4)));
    assert_eq!(queue.len(), 3);
    assert_eq!(queue.ready.back().expect("latest preview").revision, 4);
}

#[test]
fn rejected_interaction_submission_releases_only_its_reserved_slot() {
    let mut queue = BoundedCompletionQueue::<(), 4>::new();
    assert!(queue.enqueue(completion(None, 1)));
    assert!(queue.reserve_interaction());
    assert_eq!(queue.len(), 2);
    assert!(queue.cancel_interaction_reservation());
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.ready.front().map(|completion| completion.revision), Some(1));
    assert!(!queue.cancel_interaction_reservation());
}
