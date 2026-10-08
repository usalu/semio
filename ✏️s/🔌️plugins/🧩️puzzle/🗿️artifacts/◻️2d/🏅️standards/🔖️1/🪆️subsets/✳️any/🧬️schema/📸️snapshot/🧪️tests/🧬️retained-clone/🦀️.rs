//! 🧬️ Native Puzzle2d cloning consumes the authored corpus under exact grants.

use crate::Puzzle2dSnapshot;
use semio_framework_value::retained_clone::{RetainedClone, RetainedCloneBorrowAuthority, RetainedCloneCursor, RetainedCloneGrant, RetainedCloneProgress, RetainedCloneSource, RetainedCloneStep};
use semio_framework_value::retirement::{RetireOwned, controlled::ControlledRetirement};
use std::sync::Arc;

fn settle_clone<T: RetainedClone>(cursor: &mut T::Cursor, bytes: usize) {
    cursor.begin_close();
    for turn in 0..1_000_000 {
        if cursor.terminal_is_empty() { return; }
        let grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(bytes, 128), 1 => RetainedCloneGrant::one_payload_turn(bytes, 128), _ => RetainedCloneGrant::one_release_turn(bytes, 128) };
        let (result,allocation)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_granted(grant));
        let step=result.unwrap();assert!(!allocation.overflowed);assert!(allocation.requested_bytes<=step.progress().retained_capacity_bytes&&allocation.released_bytes<=step.progress().released_bytes,"native cursor close exceeded actual admitted layout: {allocation:?}, {step:?}");
        assert!(step.progress().fits(grant));assert!(step.progress().copied_bytes+step.progress().retained_capacity_bytes<=4096);
    }
    panic!("Puzzle2d clone retained owners after granted close");
}

fn settle_snapshot<T: RetireOwned>(snapshot: T, bytes: usize) {
    let (retirement,allocation)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ControlledRetirement::new(snapshot));
    assert!(!allocation.overflowed);assert_eq!(allocation.requested_bytes,0);assert_eq!(allocation.released_bytes,0);
    let mut retirement=match retirement { Ok(owner)=>owner,Err((error,_))=>panic!("{error:?}") };
    for turn in 0..1_000_000 {
        if retirement.terminal_is_empty() { return; }
        let grant=match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(bytes,128), 1 => RetainedCloneGrant::one_payload_turn(bytes,128), _ => RetainedCloneGrant::one_release_turn(bytes,128) };
        let (result,allocation)=semio_framework_trace::observe_heap_allocations_on_this_thread(||retirement.step(grant));
        let step=result.unwrap();assert!(!allocation.overflowed);assert!(allocation.requested_bytes<=step.progress().retained_capacity_bytes&&allocation.released_bytes<=step.progress().released_bytes,"native owner close exceeded actual admitted layout: {allocation:?}, {step:?}");
        assert!(step.progress().fits(grant));assert!(step.progress().copied_bytes+step.progress().retained_capacity_bytes<=4096);
    }
    panic!("Puzzle2d cloned snapshot retained owners after granted close");
}

#[test]
fn history_edit_puzzle2d_retained_snapshot_clone_preserves_native_ownership_and_cancellation() {
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧬️retained-clone/🔣️.json")).unwrap();
    let original: serde_json::Value = serde_json::from_str(include_str!("../../../../🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔣️.json")).unwrap();
    let controls = &corpus["control"];
    let grant = RetainedCloneGrant {
        maximum_items: controls["maximumItems"].as_u64().unwrap() as usize,
        maximum_copy_bytes: controls["maximumCopyBytes"].as_u64().unwrap() as usize,
        maximum_capacity_bytes: controls["maximumCapacityBytes"].as_u64().unwrap() as usize,
        maximum_depth: controls["maximumDepth"].as_u64().unwrap() as usize, maximum_release_bytes: controls["maximumReleaseBytes"].as_u64().unwrap() as usize };
    for journey in ["empty", "nested", "largeUtf8", "largeNodes", "nestedHandles", "nestedCatalogs", "cancelDuringCopy"] {
        let mut snapshot = if journey == "empty" { Puzzle2dSnapshot::default() } else { serde_json::from_value::<Puzzle2dSnapshot>(original["snapshot"].clone()).unwrap() };
        if journey == "largeUtf8" { snapshot.nodes[0].text = Some("😀".repeat(controls["largeTextScalars"].as_u64().unwrap() as usize).into()); }
        let length = controls["largeCollectionItems"].as_u64().unwrap() as usize;
        if journey == "largeNodes" { snapshot.nodes = (0..length).map(|index| { let mut node = snapshot.nodes[0].clone(); node.id = format!("node-{index}").into(); node }).collect(); }
        if journey == "nestedHandles" { snapshot.nodes[0].handles = (0..length).map(|index| crate::Puzzle2dHandle { id: format!("handle-{index}").into(), ..Default::default() }).collect(); }
        if journey == "nestedCatalogs" { snapshot.meta.kind_catalogs.as_mut().unwrap().nodes[0].representations[0].tags = (0..length).map(|index| format!("tag-{index}").into()).collect(); }
        let expected = serde_json::to_value(&snapshot).unwrap();
        let owner = Arc::new(snapshot);
        let source = RetainedCloneSource::from_authority(Arc::clone(&owner), journey);
        let (mut cursor,allocation)=semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dSnapshot::retained_clone_cursor);
        assert!(!allocation.overflowed);assert!(allocation.requested_bytes<=grant.maximum_capacity_bytes&&allocation.released_bytes<=grant.maximum_capacity_bytes,"native snapshot cursor birth exceeded fixed grant: {allocation:?}");
        let (result,allocation)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(source.borrow(),RetainedCloneGrant::default()));
        let zero=result.unwrap();assert_eq!(allocation.requested_bytes,0);assert_eq!(allocation.released_bytes,0);
        assert_eq!(zero.progress(), RetainedCloneProgress::default());
        let mut completed = false;
        let mut totals = [0usize; 4];
        for turn in 0usize..1_000_000 {
            let turn_grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(grant.maximum_capacity_bytes, grant.maximum_depth), 1 => RetainedCloneGrant::one_payload_turn(grant.maximum_copy_bytes, grant.maximum_depth), _ => RetainedCloneGrant::one_release_turn(grant.maximum_release_bytes, grant.maximum_depth) };
            let (result,allocation)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(source.borrow(),turn_grant));
            let step=result.unwrap();
            let progress = step.progress();
            totals[0] += progress.copied_items; totals[1] += progress.copied_bytes; totals[2] += progress.retained_capacity_bytes; totals[3] += progress.released_bytes;
            if journey == "largeNodes" && ((turn >= 1023 && (turn + 1).is_power_of_two()) || turn == 999999) { eprintln!("[DEBUG] Puzzle2d native snapshot clone journey={journey} turn={} step={step:?} cumulativeItemsCopyCapacityRelease={totals:?}", turn + 1); }
            assert!(!allocation.overflowed);assert!(allocation.requested_bytes<=step.progress().retained_capacity_bytes,"{journey}: actual allocation requests {} exceed admitted retained capacity {}",allocation.requested_bytes,step.progress().retained_capacity_bytes);
            assert!(allocation.released_bytes<=step.progress().released_bytes,"native clone released unadmitted layout bytes: {allocation:?}");
            assert!(step.progress().fits(turn_grant));
            assert!(step.progress().copied_bytes + step.progress().retained_capacity_bytes + step.progress().released_bytes <= controls["maximumTurnBytes"].as_u64().unwrap() as usize);
            if journey == "cancelDuringCopy" && turn + 1 == controls["cancelAfterTurns"].as_u64().unwrap() as usize { break; }
            if matches!(step, RetainedCloneStep::Complete(_)) { completed = true; break; }
        }
        assert_eq!(serde_json::to_value(owner.as_ref()).unwrap(), expected);
        if journey == "cancelDuringCopy" {
            assert!(!completed);
            settle_clone::<Puzzle2dSnapshot>(&mut cursor, grant.maximum_capacity_bytes);
            assert!(cursor.take().is_none());
        } else {
            assert!(completed, "{journey}: clone did not complete within its exact grants");
            let cloned = cursor.take().unwrap();
            assert_eq!(serde_json::to_value(&cloned).unwrap(), expected);
            assert!(cursor.take().is_none());
            settle_clone::<Puzzle2dSnapshot>(&mut cursor, grant.maximum_capacity_bytes);
            settle_snapshot(cloned, grant.maximum_capacity_bytes);
        }
        assert!(cursor.terminal_is_empty());
        eprintln!("[DEBUG] Puzzle2d paged snapshot clone {journey} preserved native JSON and reached exact closure");
    }
}


#[test]
fn history_edit_puzzle2d_borrowed_mutations_clone_every_authored_leaf_with_exact_retirement() {
    use crate::standards::v1::subsets::any::schema::mutations::Puzzle2dMutation;
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations");
    let corpus: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🧬️retained-clone/🔣️.json")).unwrap();
    let controls = &corpus["control"];
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: controls["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: controls["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_depth: 64, maximum_release_bytes: controls["maximumReleaseBytes"].as_u64().unwrap() as usize };
    let mut kinds = std::collections::BTreeSet::new();
    let mut count = 0;
    for leaf in std::fs::read_dir(root).unwrap() {
        let leaf = leaf.unwrap().path();
        if !leaf.is_dir() { continue; }
        for case in std::fs::read_dir(leaf).unwrap() {
            let input = case.unwrap().path().join("🦠️mutation/🔣️.json");
            if !input.is_file() { continue; }
            let text = std::fs::read_to_string(input).unwrap();
            let Ok(original) = serde_json::from_str::<Puzzle2dMutation>(&text) else { continue; };
            let expected = serde_json::to_value(&original).unwrap();
            kinds.insert(expected["mutation"].as_str().unwrap().to_owned());
            let authority = RetainedCloneBorrowAuthority::new(count);
            let (mut cursor,allocation)=semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dMutation::retained_clone_cursor);
            assert!(!allocation.overflowed);assert!(allocation.requested_bytes<=grant.maximum_capacity_bytes&&allocation.released_bytes<=grant.maximum_capacity_bytes,"native mutation cursor birth exceeded fixed grant: {allocation:?}");
            let (result,allocation)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(authority.borrow(&original),RetainedCloneGrant::default()));
            assert_eq!(result.unwrap().progress(),RetainedCloneProgress::default());assert_eq!(allocation.requested_bytes,0);assert_eq!(allocation.released_bytes,0);
            let mut complete = false;
            for turn in 0..1_000_000 {
                let turn_grant = match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(grant.maximum_capacity_bytes, grant.maximum_depth), 1 => RetainedCloneGrant::one_payload_turn(grant.maximum_copy_bytes, grant.maximum_depth), _ => RetainedCloneGrant::one_release_turn(grant.maximum_release_bytes, grant.maximum_depth) };
                let (result,allocation)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(authority.borrow(&original),turn_grant));
                let step=result.unwrap();
                assert!(!allocation.overflowed);assert!(allocation.requested_bytes<=step.progress().retained_capacity_bytes,"native borrowed mutation allocation requests {} exceed admitted capacity {}",allocation.requested_bytes,step.progress().retained_capacity_bytes);
                assert!(allocation.released_bytes<=step.progress().released_bytes,"native clone released unadmitted layout bytes: {allocation:?}");
            assert!(step.progress().fits(turn_grant));
                assert!(step.progress().copied_bytes + step.progress().retained_capacity_bytes + step.progress().released_bytes <= 4096);
                if matches!(step, RetainedCloneStep::Complete(_)) { complete = true; break; }
            }
            assert!(complete, "authored borrowed mutation did not complete");
            let owned = cursor.take().unwrap();
            assert_eq!(serde_json::to_value(&owned).unwrap(), expected);
            assert_eq!(serde_json::to_value(&original).unwrap(), expected);
            assert!(cursor.take().is_none());
            settle_clone::<Puzzle2dMutation>(&mut cursor, grant.maximum_capacity_bytes);
            settle_snapshot(owned, grant.maximum_capacity_bytes);
            count += 1;
        }
    }
    assert_eq!(kinds.len(), corpus["mutationOwners"].as_array().unwrap().len() - 1);
    for cancelled in [false, true] {
        let original = Puzzle2dMutation::EditNodeText(crate::standards::v1::subsets::any::schema::mutations::EditNodeText { id: "owned".into(), new_text: Some("😀".repeat(4096).into()) });
        let swapped = original.clone();
        let authority = RetainedCloneBorrowAuthority::new(cancelled);
        let (mut cursor,allocation)=semio_framework_trace::observe_heap_allocations_on_this_thread(Puzzle2dMutation::retained_clone_cursor);
            assert!(!allocation.overflowed);assert!(allocation.requested_bytes<=grant.maximum_capacity_bytes&&allocation.released_bytes<=grant.maximum_capacity_bytes,"native mutation cursor birth exceeded fixed grant: {allocation:?}");
        let grant = RetainedCloneGrant { maximum_copy_bytes: 4, ..grant };
        for turn in 0..controls["cancelAfterTurns"].as_u64().unwrap() {
            let turn_grant=match turn % 3 { 0 => RetainedCloneGrant::one_capacity_turn(grant.maximum_capacity_bytes,grant.maximum_depth), 1 => RetainedCloneGrant::one_payload_turn(grant.maximum_copy_bytes,grant.maximum_depth), _ => RetainedCloneGrant::one_release_turn(grant.maximum_release_bytes,grant.maximum_depth) };
            let (result,allocation)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance(authority.borrow(&original),turn_grant));
            let step=result.unwrap();assert!(!allocation.overflowed);assert!(allocation.requested_bytes<=step.progress().retained_capacity_bytes);
            assert!(!matches!(step, RetainedCloneStep::Complete(_)));
            assert!(allocation.released_bytes<=step.progress().released_bytes,"native clone released unadmitted layout bytes: {allocation:?}");
            assert!(step.progress().fits(turn_grant));
        }
        if !cancelled { assert!(cursor.advance(authority.borrow(&swapped), grant).is_err()); }
        settle_clone::<Puzzle2dMutation>(&mut cursor, grant.maximum_capacity_bytes);
        assert!(cursor.terminal_is_empty());
        assert!(cursor.take().is_none());
    }
    eprintln!("[DEBUG] Puzzle2d borrowed clone preserved {count} authored mutations across {} leaves and exact cancellation closure", kinds.len());
}
