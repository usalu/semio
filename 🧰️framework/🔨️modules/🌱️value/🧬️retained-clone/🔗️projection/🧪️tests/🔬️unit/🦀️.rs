use super::*;
use crate::{list::PagedList, paged::PagedUtf8, retirement::controlled::ControlledRetirement};
use crate::retained_clone::{RetainedCloneGrant, RetainedCloneSource, RetainedCloneStep};
use crate::value::observe_retirement_allocations;

#[derive(Debug, crate::RetireOwned, serde::Deserialize, serde::Serialize)]
struct Row { label: PagedUtf8<{usize::MAX}>, choice: Option<PagedUtf8<{usize::MAX}>> }
#[derive(Debug, crate::RetireOwned, serde::Deserialize, serde::Serialize)]
struct Root { rows: PagedList<Row, {usize::MAX}> }

#[test]
fn paged_native_owned_projection_retains_root_and_closes_one_actual_alias() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../📦️paged/🧫️fixtures/🎮️native-owner/🔣️.json")).unwrap();
    let law = &corpus["ownedProjection"];
    let root: Root = serde_json::from_value(law["input"].clone()).unwrap();
    let owner = Arc::new(root);
    let weak = Arc::downgrade(&owner);
    let source = RetainedCloneSource::from_authority(owner, ());
    let before = serde_json::to_value(source.borrow().get()).unwrap();
    for (index, vector) in law["paths"].as_array().unwrap().iter().enumerate() {
        let (mut parent, allocation) = observe_retirement_allocations(|| source.project_owned(1, |root| &root.rows));
        assert_eq!(allocation, (0, 0));
        let (child, allocation) = observe_retirement_allocations(|| parent.project(index, |rows| if index == 0 { &rows[index].label } else { rows[index].choice.as_ref().unwrap() }));
        assert_eq!(allocation, (0, 0));
        let mut child = child.unwrap();
        fn shared_owner<T: Send + Sync>() {}
        shared_owner::<RetainedOwnedProjection<PagedUtf8<{usize::MAX}>>>();
        let (same, allocation) = observe_retirement_allocations(|| child.borrow().unwrap().get().eq_str(vector["value"].as_str().unwrap()));
        assert!(same);
        assert_eq!(allocation, (0, 0));
        for action in law["closeSequence"].as_array().unwrap().iter().map(|value| value.as_str().unwrap()) {
            let (step, allocation) = observe_retirement_allocations(|| match action {
                "zero" => child.close_step(0).unwrap(),
                "parent" => parent.close_step(1).unwrap(),
                "child" => child.close_step(1).unwrap(),
                "complete" => child.close_step(1).unwrap(),
                _ => panic!("unknown native projection closure intent"),
            });
            assert_eq!(allocation, (0, 0));
            match action {
                "zero" => assert_eq!(step, SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 }),
                "complete" => assert_eq!(step, SnapshotRetirementStep::Complete),
                _ => assert_eq!(step, SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 }),
            }
            assert!(weak.upgrade().is_some());
        }
        assert!(parent.terminal_is_empty() && child.terminal_is_empty());
        assert!(parent.borrow().is_err() && child.borrow().is_err());
        let (_, allocation) = observe_retirement_allocations(|| { drop(parent); drop(child); });
        assert_eq!(allocation, (0, 0));
        assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(), before);
    }
    let owner = source.into_owner();
    let root = Arc::try_unwrap(owner).unwrap();
    let (retirement, allocation) = observe_retirement_allocations(|| ControlledRetirement::new(root));
    assert_eq!(allocation, (0, 0));
    let mut retirement = retirement.unwrap();
    for turn in 0..100000 {
        let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, usize::MAX), 1 => RetainedCloneGrant::one_payload_turn(4096, usize::MAX), _ => RetainedCloneGrant::one_release_turn(4096, usize::MAX) };
        let (step, allocation) = observe_retirement_allocations(|| retirement.step(grant).unwrap());
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert!(allocation.0 <= progress.retained_capacity_bytes && allocation.1 <= progress.released_bytes);
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
    }
    assert!(retirement.terminal_is_empty());
    assert!(weak.upgrade().is_none());
    let (_, allocation) = observe_retirement_allocations(|| drop(retirement));
    assert_eq!(allocation, (0, 0));
    eprintln!("[DEBUG] owned native projections matched RFC6902 value paths, retained immutable root, born/read/closed aliases with0heap, transferred root into actual granted retirement");
}
