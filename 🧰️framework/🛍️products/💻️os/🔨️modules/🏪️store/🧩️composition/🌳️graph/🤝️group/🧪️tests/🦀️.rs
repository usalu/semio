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
    for _ in 0..10000 { if seal_root(&graph,&mut preparation).unwrap()==GroupOwnsStep::RootPrepared{break} }
    assert!(graph.owns_group_ready(&preparation));assert!(owner.commit());
    let (result,allocated,released)=crate::test_allocation::observe_backing(||graph.commit_owns_group(&mut preparation));
    assert_eq!(result,Ok(()));assert_eq!((allocated,released),(0,0));assert_eq!(graph.owns_generation,1);
    close_group(&mut graph,&mut preparation);
    close_graph(&mut graph);
    println!("[DEBUG] ownership siblings common-root commit allocated0 released0 generation1");
}
fn seal_root(graph: &CompositionGraph, preparation: &mut GroupOwnsPreparation) -> Result<GroupOwnsStep, GroupOwnsError> {
    let demand=preparation.next_preparation_demands(graph);let grant=preparation_grant(demand);
    for refused in [ArtifactStoreOneItemGrant{maximum_items:0,..grant},ArtifactStoreOneItemGrant{maximum_depth:0,..grant},ArtifactStoreOneItemGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(1),..grant},ArtifactStoreOneItemGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(1),..grant}] {
        if refused.maximum_items==grant.maximum_items&&refused.maximum_depth==grant.maximum_depth&&refused.maximum_copy_bytes==grant.maximum_copy_bytes&&refused.maximum_capacity_bytes==grant.maximum_capacity_bytes{continue;}
        let(answer,allocated,released)=crate::test_allocation::observe_backing(||graph.seal_owns_group(preparation,refused));assert_eq!(answer,Ok(GroupOwnsStep::Blocked));assert_eq!((allocated,released),(0,0));assert_eq!(preparation.next_preparation_demands(graph),demand);
    }
    let(answer,allocated,released)=crate::test_allocation::observe_backing(||graph.seal_owns_group(preparation,grant));assert_eq!(allocated,demand.capacity_bytes);assert_eq!(released,0);assert!(demand.copy_bytes<=4096);answer
}
fn preparation_grant(demand: semio_framework_value::RetirementDemand) -> ArtifactStoreOneItemGrant { ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: demand.copy_bytes, maximum_capacity_bytes: demand.capacity_bytes, maximum_release_bytes: demand.release_bytes, maximum_depth: demand.depth } }
fn group_grant(preparation: &GroupOwnsPreparation) -> RetainedCloneGrant { RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: preparation.next_close_copy_byte_demand(), maximum_capacity_bytes: 0, maximum_release_bytes: preparation.next_close_release_byte_demand(), maximum_depth: preparation.next_close_depth_demand() } }
fn close_graph(graph: &mut CompositionGraph) {
    for _ in 0..10000 {
        let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: graph.next_close_copy_byte_demand().unwrap(), maximum_capacity_bytes: graph.next_close_capacity_byte_demand(0).unwrap(), maximum_release_bytes: graph.next_close_release_byte_demand().unwrap(), maximum_depth: graph.next_close_depth_demand().unwrap() };
        let (result, allocated, released) = crate::test_allocation::observe_backing(|| graph.close_step(grant));
        let step = result.unwrap(); assert_eq!(allocated, 0); assert_eq!(released, step.progress().released_bytes); assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) { assert!(graph.terminal_is_empty()); return; }
    }
    panic!("graph close finite");
}
fn close_group(graph: &mut CompositionGraph, preparation: &mut GroupOwnsPreparation) {
    for _ in 0..10000 {
        let grant = group_grant(preparation);
        for refused in [RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_depth: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: grant.maximum_copy_bytes.saturating_sub(1), ..grant }, RetainedCloneGrant { maximum_release_bytes: grant.maximum_release_bytes.saturating_sub(1), ..grant }] {
            if refused == grant { continue; }
            let (step, allocated, released) = crate::test_allocation::observe_backing(|| graph.close_owns_group(preparation, refused));
            assert_eq!((allocated, released), (0, 0)); assert_eq!(step, RetainedCloneStep::Progress(Default::default())); assert_eq!(group_grant(preparation), grant);
        }
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| graph.close_owns_group(preparation, grant));
        assert_eq!(allocated, 0); assert_eq!(released, step.progress().released_bytes); assert!(step.progress().fits(grant));
        if matches!(step, RetainedCloneStep::Complete(_)) { assert_eq!(released, 0); assert!(preparation.terminal_is_empty()); return; }
    }
    panic!("ownership group reaches its exact terminal owner");
}

fn prepare_edge(graph: &CompositionGraph, preparation: &mut GroupOwnsPreparation, parent: &str, slot: &str, child: &str) -> Result<usize, GroupOwnsError> {
    for turns in 1..10000 {
        let demand = preparation.next_edge_demands(graph, parent, slot, child)?;
        let grant=preparation_grant(demand);
        for refused in [ArtifactStoreOneItemGrant {maximum_items:0,..grant},ArtifactStoreOneItemGrant {maximum_depth:0,..grant},ArtifactStoreOneItemGrant {maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(1),..grant},ArtifactStoreOneItemGrant {maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(1),..grant}] {
            if refused.maximum_items==grant.maximum_items&&refused.maximum_depth==grant.maximum_depth&&refused.maximum_copy_bytes==grant.maximum_copy_bytes&&refused.maximum_capacity_bytes==grant.maximum_capacity_bytes{continue;}
            let (answer,allocated,released)=crate::test_allocation::observe_backing(||graph.prepare_owns_group_edge(preparation,parent,slot,child,refused));
            assert_eq!(answer,Ok(GroupOwnsStep::Blocked));assert_eq!((allocated,released),(0,0));assert_eq!(preparation.next_edge_demands(graph,parent,slot,child)?,demand);
        }
        let (answer,allocated,released)=crate::test_allocation::observe_backing(||graph.prepare_owns_group_edge(preparation,parent,slot,child,grant));
        assert_eq!(allocated,demand.capacity_bytes);assert_eq!(released,0);assert!(demand.copy_bytes<=4096);
        match answer? { GroupOwnsStep::EdgePrepared=>return Ok(turns),GroupOwnsStep::Progress=>(),other=>panic!("funded edge turn: {other:?}") }
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
        let demand = preparation.next_edge_demands(&graph, parent, slot, child).unwrap();
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| graph.prepare_owns_group_edge(&mut preparation, parent, slot, child, ArtifactStoreOneItemGrant { maximum_items: 0, ..preparation_grant(demand) }));
        assert_eq!(step.unwrap(), GroupOwnsStep::Blocked); assert_eq!((allocated, released), (0, 0));
        let demand = preparation.next_edge_demands(&graph, parent, slot, child).unwrap();
        let (step, allocated, released) = crate::test_allocation::observe_backing(|| graph.prepare_owns_group_edge(&mut preparation, parent, slot, child, ArtifactStoreOneItemGrant { maximum_capacity_bytes: demand.capacity_bytes - 1, ..preparation_grant(demand) }));
        assert_eq!(step.unwrap(), GroupOwnsStep::Blocked); assert_eq!((allocated, released), (0, 0));
        prepare_edge(&graph, &mut preparation, parent, slot, child).unwrap();
        assert_eq!(graph.owns.len(), 1); assert_eq!(graph.owns_generation, before_generation);
    }
    for _ in 0..10000 {
        let step = seal_root(&graph, &mut preparation).unwrap();
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
    close_graph(&mut graph);
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
    close_graph(&mut graph);
    for stop in 0..16 {
        let mut graph = CompositionGraph::new().await; graph.insert_owns("prior", "slot", "old").await.unwrap();
        let mut owner = ArtifactGroupVisibilityOwner::new(); let visibility = owner.view();
        let mut preparation = graph.begin_owns_group(&visibility, 1).unwrap();
        let mut accepted = false;
        for _ in 0..stop {
            if accepted { seal_root(&graph,&mut preparation).unwrap(); }
            else { let grant=preparation_grant(preparation.next_edge_demands(&graph,"root","slot","child").unwrap()); accepted = graph.prepare_owns_group_edge(&mut preparation, "root", "slot", "child", grant).unwrap() == GroupOwnsStep::EdgePrepared; }
        }
        assert_eq!(graph.owns.len(), 1); assert!(owner.abort()); close_group(&mut graph, &mut preparation);
        assert_eq!(graph.owner_of("old").await, Some("prior")); assert_eq!(graph.owner_of("child").await, None);
        close_graph(&mut graph);
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
    let bytes = graph.next_close_copy_byte_demand().unwrap();
    let grant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: bytes, maximum_capacity_bytes: 0, maximum_release_bytes: 0, maximum_depth: 1 };
    let (blocked, allocated, released) = crate::test_allocation::observe_backing(|| graph.close_step(RetainedCloneGrant { maximum_copy_bytes: bytes - 1, ..grant }));
    assert_eq!(blocked.unwrap(), RetainedCloneStep::Progress(Default::default())); assert_eq!((allocated, released), (0, 0));
    let (step, allocated, released) = crate::test_allocation::observe_backing(|| graph.close_step(grant));
    assert_eq!(step.unwrap(), RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() }));
    assert_eq!((allocated, released), (0, 0)); close_graph(&mut graph);
    println!("[DEBUG] actual live graph row transfer copied={bytes} allocated0 released0, exact separate backing close");
}

#[semio_framework_async_macros::async_test]
async fn composition_group_link_retirement_retains_underfunded_original_backing() {
    let mut graph = CompositionGraph::new().await;
    graph.links.insert("source".into(), ["target".into()].into_iter().collect());
    let capacity = graph.links.capacity();
    let grant = RetainedCloneGrant { maximum_items: 0, maximum_copy_bytes: 4096, maximum_capacity_bytes: 0, maximum_release_bytes: 4096, maximum_depth: 1 };
    let (result, allocated, released) = crate::test_allocation::observe_backing(|| graph.close_step(grant));
    assert_eq!(result.unwrap(), RetainedCloneStep::Progress(Default::default())); assert_eq!((allocated, released), (0, 0));
    assert_eq!(graph.links.capacity(), capacity); assert!(graph.links.get("source").unwrap().iter().any(|target| target == "target"));
    close_graph(&mut graph);
    let (_, allocated, released) = crate::test_allocation::observe_backing(|| drop(graph)); assert_eq!((allocated, released), (0, 0));
    println!("[DEBUG] original linked graph underfunded allocation0 free0; exact string/row/backing release closes terminalDrop0");
}

#[semio_framework_async_macros::async_test]
async fn composition_group_weak_authority_remains_external_without_false_frame_release() {
    let mut graph = CompositionGraph::new().await;
    let weak = Arc::downgrade(graph.owns_authority.as_ref().unwrap());
    assert_eq!(graph.next_close_release_byte_demand().unwrap(), 0);
    close_graph(&mut graph); assert!(weak.upgrade().is_none());
    println!("[DEBUG] graph final strong lease closes allocation0 free0 with external weak backing retained");
    drop(weak);
}

#[semio_framework_async_macros::async_test]
async fn composition_group_empty_additions_preserve_original_root_with_exact_backing_grant() {
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let row=&fixture["emptyAdditionRoot"]["original"];
    let mut graph=CompositionGraph::new().await;graph.insert_owns(row[0].as_str().unwrap(),row[1].as_str().unwrap(),row[2].as_str().unwrap()).await.unwrap();
    let mut owner=ArtifactGroupVisibilityOwner::new();let visibility=owner.view();let mut preparation=graph.begin_owns_group(&visibility,0).unwrap();
    for _ in 0..10000{if seal_root(&graph,&mut preparation).unwrap()==GroupOwnsStep::RootPrepared{break;}}
    assert!(graph.owns_group_ready(&preparation));assert_eq!(graph.owner_of(row[2].as_str().unwrap()).await,row[0].as_str());assert!(owner.commit());
    let(answer,allocated,released)=crate::test_allocation::observe_backing(||graph.commit_owns_group(&mut preparation));assert_eq!(answer,Ok(()));assert_eq!((allocated,released),(0,0));
    assert_eq!(graph.owner_of(row[2].as_str().unwrap()).await,row[0].as_str());assert_eq!(graph.slot_of(row[2].as_str().unwrap()).await,row[1].as_str());close_group(&mut graph,&mut preparation);close_graph(&mut graph);
    println!("[DEBUG] zero additions keep original owner/slot through exact work/backing grants and allocation0 committed flip");
}
