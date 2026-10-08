use super::*;
use crate::os_vcs::ArtifactGroupVisibilityOwner;
#[semio_framework_async_macros::async_test]
async fn composition_group_siblings_commit_without_post_flip_allocation_or_stale_tokens() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut graph=CompositionGraph::new().await;
    let rows=fixture["siblings"]["additions"].as_array().unwrap();
    let mut owner=ArtifactGroupVisibilityOwner::new();let visibility=owner.view();
    let mut preparation=graph.begin_owns_group(&visibility,rows.len()).unwrap();
    assert!(!graph.terminal_is_empty(),"bare private admission remains an exact graph authority until cancelled or committed");
    for row in rows { prepare_edge(&graph,&mut preparation,row[0].as_str().unwrap(),row[1].as_str().unwrap(),row[2].as_str().unwrap()).unwrap(); }
    for _ in 0..10000 { if graph.seal_owns_group(&mut preparation,ArtifactStoreOneItemGrant{maximum_items:1,maximum_bytes:65536}).unwrap()==GroupOwnsStep::RootPrepared{break} }
    assert!(graph.owns_group_ready(&preparation));assert!(owner.commit());
    let (result,allocated,released)=crate::test_allocation::observe_backing(||graph.commit_owns_group(&mut preparation));
    assert_eq!(result,Ok(()));assert_eq!((allocated,released),(0,0));assert_eq!(graph.owns_generation,1);
    close_group(&mut graph,&mut preparation);
    while !graph.terminal_is_empty(){graph.close_step(1,graph.next_close_byte_demand());}
    println!("[DEBUG] ownership siblings common-root commit allocated0 released0 generation1");
}
fn close_group(graph: &mut CompositionGraph, preparation: &mut GroupOwnsPreparation) {
    for _ in 0..10000 {
        let demand = preparation.next_close_byte_demand();
        if demand != 0 {
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| graph.close_owns_group(preparation, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: demand - 1 }));
            assert_eq!((allocated, released), (0, 0));
            assert_eq!(step, SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
            assert_eq!(preparation.next_close_byte_demand(), demand);
        }
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| graph.close_owns_group(preparation, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: demand }));
        assert_eq!(allocated, 0);
        match step { SnapshotRetirementStep::Complete => { assert_eq!(released, 0); assert!(preparation.terminal_is_empty()); return; }, SnapshotRetirementStep::Pending { released_bytes, .. } => assert_eq!(released, released_bytes), SnapshotRetirementStep::Blocked => panic!("exact graph physical owner grant") }
    }
    panic!("ownership group reaches its exact terminal owner");
}

fn prepare_edge(graph: &CompositionGraph, preparation: &mut GroupOwnsPreparation, parent: &str, slot: &str, child: &str) -> Result<usize, GroupOwnsError> {
    for turns in 1..10000 {
        let demand = preparation.next_edge_byte_demand(graph, parent, slot, child)?;
        match graph.prepare_owns_group_edge(preparation, parent, slot, child, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: demand })? { GroupOwnsStep::EdgePrepared => return Ok(turns), GroupOwnsStep::Progress => (), other => panic!("funded edge turn: {other:?}") }
    }
    panic!("one-hop cycle admission is finite");
}

#[semio_framework_async_macros::async_test]
async fn composition_group_sorted_root_is_atomic_and_every_owner_retirement_is_physically_funded() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut graph = CompositionGraph::new().await;
    graph.insert_owns("prior", "old", "before").await.unwrap();
    let before_generation = graph.owns_generation;
    let mut owner = ArtifactGroupVisibilityOwner::new();
    let visibility = owner.view();
    let (result, allocated, released) = crate::test_allocation::observe_backing(|| graph.begin_owns_group(&visibility, 2));
    assert_eq!((allocated, released), (0, 0));
    let mut preparation = result.unwrap();
    let rows = fixture["siblings"]["additions"].as_array().unwrap();
    for row in rows.iter().rev() {
        let parent = row[0].as_str().unwrap(); let slot = row[1].as_str().unwrap(); let child = row[2].as_str().unwrap();
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| graph.prepare_owns_group_edge(&mut preparation, parent, slot, child, ArtifactStoreOneItemGrant { maximum_items: 0, maximum_bytes: 65536 }));
        assert_eq!(step.unwrap(), GroupOwnsStep::Blocked); assert_eq!((allocated, released), (0, 0));
        let demand = preparation.next_byte_demand(&graph).max(GroupOwnsPreparation::edge_birth_bytes(parent, slot, child).unwrap());
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| graph.prepare_owns_group_edge(&mut preparation, parent, slot, child, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: demand - 1 }));
        assert_eq!(step.unwrap(), GroupOwnsStep::Blocked); assert_eq!((allocated, released), (0, 0));
        prepare_edge(&graph, &mut preparation, parent, slot, child).unwrap();
        assert_eq!(graph.owns.len(), 1); assert_eq!(graph.owns_generation, before_generation);
    }
    for _ in 0..10000 {
        let demand = preparation.next_byte_demand(&graph);
        let step = graph.seal_owns_group(&mut preparation, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: demand.max(65536) }).unwrap();
        assert_eq!(graph.owns.len(), 1);
        if step == GroupOwnsStep::RootPrepared { break; }
    }
    assert!(graph.owns_group_ready(&preparation));
    let mut foreign = CompositionGraph::new().await;
    assert!(!foreign.owns_group_ready(&preparation));
    assert_eq!(foreign.commit_owns_group(&mut preparation), Err(GroupOwnsError::Foreign));
    graph.owns_generation += 1; assert!(!graph.owns_group_ready(&preparation)); graph.owns_generation -= 1;
    assert!(owner.commit());
    let (result, allocated, released) = crate::test_allocation::observe_backing(|| graph.commit_owns_group(&mut preparation));
    assert_eq!(result, Ok(())); assert_eq!((allocated, released), (0, 0));
    assert_eq!(graph.owns_generation, before_generation + 1);
    for row in rows { assert_eq!(graph.owner_of(row[2].as_str().unwrap()).await, row[0].as_str()); assert_eq!(graph.slot_of(row[2].as_str().unwrap()).await, row[1].as_str()); }
    assert_eq!(graph.owner_of("before").await, Some("prior"));
    close_group(&mut graph, &mut preparation);
    while !graph.terminal_is_empty() { graph.close_step(1, 65536); }
    println!("[DEBUG] sorted sibling root committed allocation0/free0 generation1; every String/row backing undergrant retained and exact free reported");
}

#[semio_framework_async_macros::async_test]
async fn composition_group_cycle_hops_and_cancellation_preserve_original_forest() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut graph = CompositionGraph::new().await;
    for row in fixture["cycle"]["initial"].as_array().unwrap() { graph.insert_owns(row[0].as_str().unwrap(), row[1].as_str().unwrap(), row[2].as_str().unwrap()).await.unwrap(); }
    let mut owner = ArtifactGroupVisibilityOwner::new(); let visibility = owner.view();
    let mut preparation = graph.begin_owns_group(&visibility, 1).unwrap();
    let row = &fixture["cycle"]["addition"];
    assert_eq!(prepare_edge(&graph, &mut preparation, row[0].as_str().unwrap(), row[1].as_str().unwrap(), row[2].as_str().unwrap()), Err(GroupOwnsError::Cycle));
    assert_eq!(graph.owns.len(), 65); assert!(owner.abort()); close_group(&mut graph, &mut preparation);
    while !graph.terminal_is_empty() { graph.close_step(1, 65536); }
    for stop in 0..16 {
        let mut graph = CompositionGraph::new().await; graph.insert_owns("prior", "slot", "old").await.unwrap();
        let mut owner = ArtifactGroupVisibilityOwner::new(); let visibility = owner.view();
        let mut preparation = graph.begin_owns_group(&visibility, 1).unwrap();
        let mut accepted = false;
        for _ in 0..stop {
            if accepted { graph.seal_owns_group(&mut preparation, ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 65536 }).unwrap(); }
            else { accepted = graph.prepare_owns_group_edge(&mut preparation, "root", "slot", "child", ArtifactStoreOneItemGrant { maximum_items: 1, maximum_bytes: 65536 }).unwrap() == GroupOwnsStep::EdgePrepared; }
        }
        assert_eq!(graph.owns.len(), 1); assert!(owner.abort()); close_group(&mut graph, &mut preparation);
        assert_eq!(graph.owner_of("old").await, Some("prior")); assert_eq!(graph.owner_of("child").await, None);
        while !graph.terminal_is_empty() { graph.close_step(1, 65536); }
    }
    let mut graph = CompositionGraph::new().await;
    let mut owner = ArtifactGroupVisibilityOwner::new(); let visibility = owner.view(); let mut preparation = graph.begin_owns_group(&visibility, 2).unwrap();
    let rows = fixture["pendingCycle"]["additions"].as_array().unwrap();
    for (index, row) in rows.iter().enumerate() { let result = prepare_edge(&graph, &mut preparation, row[0].as_str().unwrap(), row[1].as_str().unwrap(), row[2].as_str().unwrap()); if index == 0 { assert!(result.is_ok()); } else { assert_eq!(result, Err(GroupOwnsError::Cycle)); } }
    assert!(owner.abort()); close_group(&mut graph, &mut preparation); assert!(graph.terminal_is_empty());
    println!("[DEBUG] SQLite neutral65hop/pending cycles refused;16 partial metadata/merge/seal cancellations preserve original live root and exact physical ownership");
}

#[semio_framework_async_macros::async_test]
async fn composition_group_live_root_retirement_moves_one_row_without_metadata_birth() {
    let mut graph = CompositionGraph::new().await;
    graph.insert_owns("parent", "slot", "child").await.unwrap();
    let (step, allocated, released) = crate::test_allocation::observe_backing(|| graph.close_step(1, 0));
    while !graph.terminal_is_empty() { graph.close_step(1, 65536); }
    assert_eq!(step, SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
    assert_eq!((allocated, released), (0, 0));
    println!("[DEBUG] actual live graph row transfer allocated0 released0 under structural one-item/zero-byte grant");
}
