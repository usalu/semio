
use super::*;

#[derive(Clone, Debug, PartialEq)]
struct Delta {
    key: u32,
    revision: u32,
}

impl ProjectionDelta for Delta {
    type Key = u32;
    fn key(&self) -> u32 {
        self.key
    }
}

#[test]
fn push_beyond_capacity_returns_overflow_without_dropping_existing_entries() {
    let mut inbox = ProjectionInbox::new(2);
    inbox.push(Delta { key: 1, revision: 1 }).expect("fits");
    inbox.push(Delta { key: 2, revision: 1 }).expect("fits");

    assert_eq!(inbox.push(Delta { key: 3, revision: 1 }), Err(InboxOverflow));
    assert_eq!(inbox.len(), 2, "the refused delta must not have displaced the queued ones");

    let mut out = Vec::new();
    inbox.drain_into(10, &mut out);
    assert_eq!(out, vec![Delta { key: 1, revision: 1 }, Delta { key: 2, revision: 1 }]);
}

#[test]
fn same_key_pushes_coalesce_to_the_newest_value() {
    let mut inbox = ProjectionInbox::new(1);
    inbox.push(Delta { key: 1, revision: 1 }).expect("fits");
    inbox.push(Delta { key: 1, revision: 2 }).expect("coalesces, does not consume a second slot");
    inbox.push(Delta { key: 1, revision: 3 }).expect("coalesces again");

    assert_eq!(inbox.len(), 1);
    let mut out = Vec::new();
    inbox.drain_into(10, &mut out);
    assert_eq!(out, vec![Delta { key: 1, revision: 3 }]);
}

#[test]
fn drain_into_respects_limit_and_leaves_the_remainder_queued() {
    let mut inbox = ProjectionInbox::new(3);
    inbox.push(Delta { key: 1, revision: 1 }).expect("fits");
    inbox.push(Delta { key: 2, revision: 1 }).expect("fits");
    inbox.push(Delta { key: 3, revision: 1 }).expect("fits");

    let mut out = Vec::new();
    inbox.drain_into(2, &mut out);
    assert_eq!(out, vec![Delta { key: 1, revision: 1 }, Delta { key: 2, revision: 1 }]);
    assert_eq!(inbox.len(), 1, "the third delta must remain queued, not dropped");

    inbox.drain_into(10, &mut out);
    assert_eq!(out, vec![Delta { key: 1, revision: 1 }, Delta { key: 2, revision: 1 }, Delta { key: 3, revision: 1 }]);
    assert!(inbox.is_empty());
}

#[test]
fn drain_into_on_an_empty_inbox_is_a_no_op() {
    let mut inbox: ProjectionInbox<Delta> = ProjectionInbox::new(3);
    let mut out = Vec::new();
    inbox.drain_into(5, &mut out);
    assert!(out.is_empty());
}
