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
    let mut weak = Some(Arc::downgrade(&owner));
    let source = RetainedCloneSource::fixture_from_authority(owner, ());
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
                "zero" => child.close_step(Default::default()).unwrap(),
                "parent" => parent.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:parent.next_close_release_byte_demand().unwrap(),maximum_depth:parent.next_close_depth_demand().unwrap(),..Default::default()}).unwrap(),
                "child"|"complete" => child.close_step(RetainedCloneGrant{maximum_items:1,maximum_release_bytes:child.next_close_release_byte_demand().unwrap(),maximum_depth:child.next_close_depth_demand().unwrap(),..Default::default()}).unwrap(),
                _ => panic!("unknown native projection closure intent"),
            });
            assert_eq!(allocation, (0, 0));
            match action {
                "zero" => assert_eq!(step, RetainedCloneStep::Progress(Default::default())),
                "complete" => assert_eq!(step, RetainedCloneStep::Complete(Default::default())),
                _ => assert_eq!(step, RetainedCloneStep::Progress(crate::retained_clone::RetainedCloneProgress{copied_items:1,..Default::default()})),
            }
            assert!(weak.as_ref().unwrap().upgrade().is_some());
        }
        assert!(parent.terminal_is_empty() && child.terminal_is_empty());
        assert!(parent.borrow().is_err() && child.borrow().is_err());
        let (_, allocation) = observe_retirement_allocations(|| { drop(parent); drop(child); });
        assert_eq!(allocation, (0, 0));
        assert_eq!(serde_json::to_value(source.borrow().get()).unwrap(), before);
    }
    assert!(weak.as_ref().unwrap().upgrade().is_some());
    let (_,allocation)=observe_retirement_allocations(||drop(weak.take()));
    assert_eq!(allocation,(0,0));
    let owner = source.into_owner();
    let (retirement, allocation) = observe_retirement_allocations(|| crate::retirement::shared::SharedControlledRetirement::new(owner));
    assert_eq!(allocation, (0, 0));
    let mut retirement = retirement;
    for turn in 0..100000 {
        let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(4096, usize::MAX), 1 => RetainedCloneGrant::one_payload_turn(4096, usize::MAX), _ => RetainedCloneGrant::one_release_turn(4096, usize::MAX) };
        let (step, allocation) = observe_retirement_allocations(|| retirement.step(grant).unwrap());
        let progress = step.progress();
        assert!(progress.fits(grant));
        assert!(allocation.0 <= progress.retained_capacity_bytes && allocation.1 <= progress.released_bytes);
        if matches!(step, RetainedCloneStep::Complete(_)) { break; }
    }
    assert!(retirement.terminal_is_empty());
    assert!(weak.as_ref().and_then(|weak|weak.upgrade()).is_none());
    let (_, allocation) = observe_retirement_allocations(|| drop(retirement));
    assert_eq!(allocation, (0, 0));
    eprintln!("[DEBUG] owned native projections matched RFC6902 value paths, retained immutable root, born/read/closed aliases with0heap, transferred root into actual granted retirement");
}
