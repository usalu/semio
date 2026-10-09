//! 🧪️ Actual-grant session byte retirement, final cache ownership, and strict lifecycle guards.

use super::*;
use crate::dag::DagHostRetirement;
// 🧬️ `DagCamera`/`IoPortSpec` are the dag ARTIFACT crate's own records; `crate::dag` glob-imports them
// privately, so they are named at their source rather than through that re-export.
use semio_framework_artifact_infinite_dag::{DagCamera, IoPortSpec};

//#region 🧪️SessionRetirement
#[test]
fn original_preview_retention_preserves_roster_and_closes_one_original_row_per_turn(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️session-source/🔣️.json")).unwrap();let law=&fixture["retention"];
    for copy in [1,3,64]{
        let ((mut session,roster),heap)=observe(||{let mut session=FlowEvalSession::new();for handle in law["handles"].as_array().unwrap(){let handle=handle.as_str().unwrap().to_owned();session.preview_mesh_pack_by_handle.insert(handle.clone(),handle.repeat(257));session.preview_diagnostics_by_handle.insert(handle.clone(),handle.repeat(129));}let roster=law["roster"].as_array().unwrap().iter().map(|value|value.as_str().unwrap().to_owned()).collect::<Vec<_>>();(session,roster)});
        let original=heap.requested_bytes-heap.released_bytes;let roster_pointer=roster.as_ptr();let mesh=session.preview_mesh_pack_by_handle.get("live-高").unwrap().as_ptr();
        let (result,heap)=observe(||session.begin_retain_preview_meshes(roster));assert!(result.is_ok());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(session.state.preview_retention.as_ref().unwrap().roster.as_ref().unwrap().as_ptr(),roster_pointer);
        let zero=RetainedCloneGrant{maximum_items:0,maximum_copy_bytes:copy,..Default::default()};let (denied,heap)=observe(||session.retain_preview_meshes_step(zero).unwrap());assert_eq!(denied.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let mut born=0;let mut freed=0;let mut turns=0;
        while !session.preview_retention_terminal_is_empty(){turns+=1;assert!(turns<100000);let demand=session.next_preview_retention_demands(copy).unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let before=session.preview_mesh_pack_by_handle.len()+session.preview_diagnostics_by_handle.len();let (step,heap)=observe(||session.retain_preview_meshes_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!(step.progress(),session.preview_retention_step_progress());assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;assert!(before-(session.preview_mesh_pack_by_handle.len()+session.preview_diagnostics_by_handle.len())<=1);}
        let actual:Vec<_>=session.preview_mesh_pack_by_handle.keys().map(String::as_str).collect();let expected:Vec<_>=law["expected"].as_array().unwrap().iter().map(|value|value.as_str().unwrap()).collect();assert_eq!(actual,expected);assert_eq!(session.preview_mesh_pack_by_handle.get("live-高").unwrap().as_ptr(),mesh);
        close(session,copy,original+born-freed);
        eprintln!("[DEBUG] Original preview retention copy={copy} turns={turns} allocated={born} released={freed} Drop=0");
    }
}

#[test]
fn session_begin_close_preserves_original_text_fault_and_latch_backing() {
    let fixture: serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️session-source/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap() {
        let text=row["text"].as_str().unwrap().repeat(row["repeat"].as_u64().unwrap() as usize);
        assert_eq!(text.len(),row["utf8Bytes"].as_u64().unwrap() as usize);
        let mut session=FlowEvalSession::new();
        session.painted_eval_json=text.clone();session.converged_eval_json=text.clone();
        session.extension_evaluate_fault=Some(ExtensionEvaluateFault{extension_id:text.clone(),capability:text.clone(),code:text.clone(),message:text.clone()});
        let latches=row["latches"].as_u64().unwrap() as usize;
        for key in 0..latches { session.window_tick_latches.insert(key as u64,FlowEvalWindowTickLatch{armed:true,in_flight:key as u32,owed:true,unfinished:true}); }
        let painted=session.painted_eval_json.as_ptr();let converged=session.converged_eval_json.as_ptr();let fault=session.extension_evaluate_fault.as_ref().unwrap().message.as_ptr();
        let (_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||session.begin_close());
        let original=session.painted_eval_json.as_ptr()==painted&&session.converged_eval_json.as_ptr()==converged&&session.extension_evaluate_fault.as_ref().is_some_and(|value|value.message.as_ptr()==fault)&&session.window_tick_latches.len()==latches;
        let hidden_birth=heap.requested_bytes;let hidden_release=heap.released_bytes;
        session.retire_cold();
        assert_eq!((hidden_birth,hidden_release),(0,0),"begin_close must retain every original backing before grant");
        assert!(original,"begin_close must preserve the original painted/converged/fault/latch sources");
        eprintln!("[DEBUG] Original Flow session begin-close id={} bytes={} latches={latches} hiddenBirth=0 hiddenFree=0",row["id"],text.len());
    }
}

#[test]
fn session_reset_preserves_original_lease_and_restores_every_owner_on_refusal() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    for copy in [1,3,64] {
        let ((mut session,incoming,second),heap)=observe(||{
            let mut session=FlowEvalSession::new();
            session.eval_json=Some(Arc::new("原🌊".repeat(4097)));
            session.painted_eval_json="painted".repeat(257);session.converged_eval_json="converged".repeat(257);
            session.extension_evaluate_fault=Some(ExtensionEvaluateFault{message:"actual-fault".repeat(257),..Default::default()});
            session.preview_mesh_pack_by_handle.insert("original-handle".into(),"original-mesh".repeat(257));
            session.window_tick_latches.insert(7,FlowEvalWindowTickLatch{armed:true,..Default::default()});
            (session,Arc::new("incoming".repeat(257)),Arc::new("second".repeat(257)))
        });
        let original=heap.requested_bytes-heap.released_bytes;
        let lease=session.lease_eval_json().unwrap();let painted=session.painted_eval_json.as_ptr();let mesh=session.preview_mesh_pack_by_handle.get("original-handle").unwrap().as_ptr();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};
        let (refused,denied)=observe(||session.set_eval_json(incoming,RetainedCloneGrant{maximum_depth:0,..grant}));
        let incoming=refused.err().unwrap().1;
        assert_eq!((denied.requested_bytes,denied.released_bytes),(0,0));
        assert!(session.owns_eval_json(&lease));assert_eq!(session.painted_eval_json.as_ptr(),painted);
        assert_eq!(session.preview_mesh_pack_by_handle.get("original-handle").unwrap().as_ptr(),mesh);assert!(session.extension_evaluate_fault.is_some());
        let (progress,admitted)=observe(||session.set_eval_json(incoming,grant).unwrap_or_else(|_|panic!("original reset admission refused")));
        assert_eq!(progress,RetainedCloneProgress{copied_items:1,..Default::default()});
        assert_eq!((admitted.requested_bytes,admitted.released_bytes),(0,0));
        assert!(!session.owns_eval_json(&lease));assert_eq!(lease.as_str(),"原🌊".repeat(4097));
        assert!(session.extension_evaluate_fault.is_none());assert_eq!(session.window_tick_latches.len(),1);
        let (_,alias)=observe(||drop(lease));assert_eq!((alias.requested_bytes,alias.released_bytes),(0,0));
        let reserve=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:session.next_retirement_reserve_capacity_byte_demand().unwrap(),maximum_depth:1,..Default::default()};
        let (receipt,reserved)=observe(||session.reserve_retirement_step(reserve).unwrap());
        assert_eq!((reserved.requested_bytes,reserved.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));assert!(receipt.fits(reserve));
        let demand=session.next_retirement_admission_demands().unwrap();assert!(demand.capacity_bytes>0);
        let funded=RetainedCloneGrant{maximum_items:1,maximum_capacity_bytes:demand.capacity_bytes,maximum_depth:demand.depth,..Default::default()};
        let current=session.lease_eval_json().unwrap();
        let (result,denied)=observe(||session.set_eval_json(second,RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..funded}));
        let second=result.err().unwrap().1;assert_eq!((denied.requested_bytes,denied.released_bytes),(0,0));assert!(session.owns_eval_json(&current));
        let (receipt,admitted)=observe(||session.set_eval_json(second,funded).unwrap_or_else(|_|panic!("funded original queue reset refused")));
        assert_eq!((admitted.requested_bytes,admitted.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));assert!(receipt.fits(funded));
        let (_,alias)=observe(||drop(current));assert_eq!((alias.requested_bytes,alias.released_bytes),(0,0));
        close(session,copy,original+reserved.requested_bytes+admitted.requested_bytes);
    }
}

#[test]
fn original_registry_invalidation_preserves_source_on_refusal_and_admits_all_owners(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️session-source/🔣️.json")).unwrap();
    let generation=fixture["invalidation"]["generation"].as_u64().unwrap();
    for copy in [1,3,64]{
        let (mut session,source)=observe(||{
            let mut session=FlowEvalSession::new();session.flow_extension_generation=0;
            session.painted_eval_json="原🌊".repeat(4097);session.converged_eval_json="converged".repeat(257);
            session.extension_evaluate_fault=Some(ExtensionEvaluateFault{message:"original-fault".repeat(257),..Default::default()});
            session.window_tick_latches.insert(7,FlowEvalWindowTickLatch{armed:true,..Default::default()});session
        });
        let painted=session.painted_eval_json.as_ptr();let fault=session.extension_evaluate_fault.as_ref().unwrap().message.as_ptr();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};
        let (refused,heap)=observe(||session.invalidate_for_flow_extension_registry(generation,RetainedCloneGrant{maximum_depth:0,..grant}));
        assert!(refused.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        assert_eq!(session.flow_extension_generation,0);assert_eq!(session.painted_eval_json.as_ptr(),painted);assert_eq!(session.extension_evaluate_fault.as_ref().unwrap().message.as_ptr(),fault);assert_eq!(session.window_tick_latches.len(),1);
        let ((changed,receipt),heap)=observe(||session.invalidate_for_flow_extension_registry(generation,grant).unwrap());
        assert!(changed);assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        assert_eq!(session.flow_extension_generation,generation);assert!(session.painted_eval_json.is_empty());assert!(session.extension_evaluate_fault.is_none());assert!(session.window_tick_latches.is_empty());
        let ((changed,receipt),heap)=observe(||session.invalidate_for_flow_extension_registry(generation,grant).unwrap());
        assert!(!changed);assert_eq!(receipt,RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        close(session,copy,source.requested_bytes-source.released_bytes);
    }
}

#[test]
fn original_session_pending_tick_host_retains_same_source_through_refusal_and_close(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    for copy in [1,3,64]{
        let ((mut session,host),source)=observe(||{let mut host=FlowHost::default();host.host_catalogue_json="原🌊".repeat(4097);(FlowEvalSession::new(),host)});
        let pointer=host.host_catalogue_json.as_ptr();let grant=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};
        let (refused,heap)=observe(||session.retain_tick_host(host,RetainedCloneGrant{maximum_items:0,..grant}));
        let host=refused.err().unwrap().1;assert_eq!(host.host_catalogue_json.as_ptr(),pointer);assert!(session.tick_host().is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let (receipt,heap)=observe(||session.retain_tick_host(host,grant).unwrap_or_else(|_|panic!("original pending Host source refused")));
        assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(session.tick_host().unwrap().host_catalogue_json.as_ptr(),pointer);
        let (_,heap)=observe(||session.begin_close());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(session.tick_host().unwrap().host_catalogue_json.as_ptr(),pointer);
        close(session,copy,source.requested_bytes-source.released_bytes);
    }
}

#[test]
fn original_live_evaluation_publication_preserves_progress_and_exact_source_lease(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    for copy in [1,3,64]{
        let ((mut session,incoming),source)=observe(||{
            let mut session=FlowEvalSession::new();session.eval_json=Some(Arc::new("原🌊".repeat(4097)));
            session.extension_evaluate_fault=Some(ExtensionEvaluateFault{message:"original-fault".repeat(257),..Default::default()});
            session.preview_mesh_pack_by_handle.insert("actual-handle".into(),"actual-mesh".repeat(257));
            session.eval_progress_by_hash.insert(31,PreviewEvalProgress::default());(session,Arc::new("incoming-source".repeat(257)))
        });
        let original=session.lease_eval_json().unwrap();let mesh=session.preview_mesh_pack_by_handle.get("actual-handle").unwrap().as_ptr();let pointer=incoming.as_ptr();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};
        let (refused,heap)=observe(||session.publish_eval_json(incoming,RetainedCloneGrant{maximum_depth:0,..grant}));
        let incoming=refused.err().unwrap().1;assert!(session.owns_eval_json(&original));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let (receipt,heap)=observe(||session.publish_eval_json(incoming,grant).unwrap_or_else(|_|panic!("original live publication refused")));
        assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(session.eval_json.as_ref().unwrap().as_ptr(),pointer);
        assert!(session.extension_evaluate_fault.is_some());assert_eq!(session.preview_mesh_pack_by_handle.get("actual-handle").unwrap().as_ptr(),mesh);assert!(session.eval_progress_by_hash.contains_key(&31));
        let same=session.lease_eval_json().unwrap();let (receipt,heap)=observe(||session.publish_eval_json(same,grant).unwrap_or_else(|_|panic!("original unchanged lease refused")));assert_eq!(receipt,RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let (_,heap)=observe(||drop(original));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        close(session,copy,source.requested_bytes-source.released_bytes);
    }
}

#[test]
fn original_session_latch_birth_is_admitted_before_original_tick_mutations(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    for copy in [1,3,64]{
        let (mut session,source)=observe(FlowEvalSession::new);
        let original=source.requested_bytes-source.released_bytes;
        let mut born=0;
        let name="original-latch";
        let grant=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};
        let (missing,heap)=observe(||session.arm_window_tick(name,grant));
        assert!(missing.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        for _ in 0..1000{
            let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:session.next_window_tick_latch_capacity_byte_demand(name,copy).unwrap(),maximum_release_bytes:session.next_window_tick_latch_release_byte_demand(name).unwrap(),maximum_depth:session.next_window_tick_latch_depth_demand(name).unwrap()};
            let (refused,heap)=observe(||session.prepare_window_tick_latch(name,RetainedCloneGrant{maximum_items:0,..grant}));
            assert!(refused.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            if grant.maximum_capacity_bytes>0{
                let (refused,heap)=observe(||session.prepare_window_tick_latch(name,RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}));
                assert!(refused.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            }
            let (step,heap)=observe(||session.prepare_window_tick_latch(name,grant).unwrap());
            let progress=step.progress();assert!(progress.fits(grant));assert_eq!((progress.retained_capacity_bytes,progress.released_bytes),(heap.requested_bytes,heap.released_bytes));born+=heap.requested_bytes;
            if matches!(step,RetainedCloneStep::Complete(_)){break;}
        }
        assert!(session.window_tick_latches.contains_key(&flow_eval_window_key(name)));
        let grant=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};
        let ((armed,receipt),heap)=observe(||session.arm_window_tick(name,grant).unwrap());
        assert!(armed);assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        session.begin_window_tick(name,grant).unwrap();session.note_window_extensions_in_flight(name,2,grant).unwrap();
        assert!(!session.arm_window_tick(name,grant).unwrap().0);
        let (first,heap)=observe(||session.settle_window_extension(name,grant).unwrap());
        assert!(!first.0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        let (last,heap)=observe(||session.settle_window_extension(name,grant).unwrap());
        assert!(last.0);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        assert_eq!(session.window_extensions_in_flight(name),0);assert!(session.window_tick_is_armed(name));
        close(session,copy,original+born);
    }
}

#[test]
fn original_preview_cancel_preserves_sources_and_resets_one_actual_latch_per_turn(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    for copy in [1,3,64]{
        let (mut session,source)=observe(||{let mut session=FlowEvalSession::new();session.pending_tessellate_by_hash.insert(7,"actual-handle".repeat(257));session.tessellate_chunks_by_hash.insert(7,"原🌊".repeat(4097));session.eval_progress_by_hash.insert(7,PreviewEvalProgress{units_done:9,..Default::default()});for key in 0..65{session.window_tick_latches.insert(key,FlowEvalWindowTickLatch{armed:true,owed:true,in_flight:1,unfinished:true});}session});
        let pointer=session.tessellate_chunks_by_hash.get(&7).unwrap().as_ptr();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};
        let (refused,heap)=observe(||session.cancel_preview_evaluation("",RetainedCloneGrant{maximum_items:0,..grant}));assert!(refused.is_err());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!session.preview_cancelled);
        let ((count,receipt),heap)=observe(||session.cancel_preview_evaluation("",grant).unwrap());assert_eq!(count,1);assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(session.tessellate_chunks_by_hash.get(&7).unwrap().as_ptr(),pointer);assert_eq!(session.window_tick_latches.values().filter(|latch|latch.armed).count(),65);
        let mut born=0;let mut freed=0;
        while !session.preview_cancellation_terminal_is_empty(){let demand=session.next_preview_cancellation_demands(copy).unwrap();let funded=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};let before=session.window_tick_latches.values().filter(|latch|latch.armed).count();let (step,heap)=observe(||session.preview_cancellation_step(funded).unwrap());assert!(step.progress().fits(funded));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;let after=session.window_tick_latches.values().filter(|latch|latch.armed).count();assert!(before-after<=1);}
        assert!(session.pending_tessellate_by_hash.is_empty());assert!(session.tessellate_chunks_by_hash.is_empty());assert!(session.eval_progress_by_hash.is_empty());assert_eq!(session.window_tick_latches.len(),65);assert!(session.window_tick_latches.values().all(|latch|*latch==FlowEvalWindowTickLatch::default()));close(session,copy,source.requested_bytes-source.released_bytes+born-freed);
    }
}

#[test]
fn session_latch_pruning_borrows_roster_and_preserves_original_arena_until_close(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    let names:Vec<String>=(0..65).map(|key|format!("window-{key}")).collect();
    let live:Vec<&str>=names.iter().step_by(2).map(String::as_str).collect();
    let (mut session,source)=observe(||{
        let mut session=FlowEvalSession::new();
        for name in &names{session.window_tick_latches.insert(flow_eval_window_key(name),FlowEvalWindowTickLatch{armed:true,..Default::default()});}
        session
    });
    let grant=RetainedCloneGrant{maximum_items:1,maximum_depth:1,..Default::default()};
    let mut cursor=0;
    for _ in 0..1000{
        let grant=RetainedCloneGrant{maximum_depth:session.window_tick_latches.next_extract_slot_depth_demand(cursor).unwrap(),..grant};
        let (_,denied)=observe(||session.retain_window_tick_latches(&live,&mut cursor,RetainedCloneGrant{maximum_items:0,..grant}).unwrap());
        assert_eq!((denied.requested_bytes,denied.released_bytes),(0,0));
        let (step,heap)=observe(||session.retain_window_tick_latches(&live,&mut cursor,grant).unwrap());
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(step.progress().fits(grant));
        if matches!(step,RetainedCloneStep::Complete(_)){break;}
    }
    assert_eq!(session.window_tick_latches.len(),live.len());
    for name in live{assert!(session.window_tick_latches.contains_key(&flow_eval_window_key(name)));}
    close(session,1,source.requested_bytes-source.released_bytes);
}

#[test]
fn original_baseline_publication_admits_arc_headers_and_leases_same_payload(){
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    let mut host=FlowHost::default();
    let ((mut session,snapshot,channels),source)=observe(||{
        let session=FlowEvalSession::new();
        let channels=EvalChannels{outputs:HistoryFoldIndex::from([("out".into(),Dictionary::new().insert("payload",NeuralValue::Atom(Atom::String("原🌊".repeat(4097)))))]),inputs:HistoryFoldIndex::new()};
        let tree=Tree{neurons:vec![Neuron{id:"高🌊".repeat(257),kind:"actual-original-kind".into(),params:Dictionary::new(),tree:None}],synapses:Vec::new()};
        let snapshot=TreeSnapshot::capture(&tree,&HashMap::new());tree.retire_cold();
        (session,snapshot,channels)
    });
    let payload=match channels.outputs.get("out").unwrap().get("payload").unwrap(){NeuralValue::Atom(Atom::String(text))=>text.as_ptr(),_=>unreachable!()};
    let (_,begin)=observe(||host.begin_baseline_publication(snapshot,channels,7).unwrap_or_else(|_|panic!("original source publication occupied")));
    assert_eq!((begin.requested_bytes,begin.released_bytes),(0,0));
    let mut born=0;
    for _ in 0..1000{
        if host.baseline_publication_terminal_is_empty(){break;}
        let demand=host.next_baseline_publication_demands(0).unwrap();
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:demand.copy_bytes,maximum_capacity_bytes:demand.capacity_bytes,maximum_release_bytes:demand.release_bytes,maximum_depth:demand.depth};
        if demand.capacity_bytes>0{
            let (_,denied)=observe(||host.baseline_publication_step(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant}).unwrap());
            assert_eq!((denied.requested_bytes,denied.released_bytes),(0,0));
        }
        let (step,heap)=observe(||host.baseline_publication_step(grant).unwrap());
        assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));
        assert_eq!(step.progress().copied_bytes,0);born+=heap.requested_bytes;
    }
    assert!(host.baseline_publication_terminal_is_empty());
    assert!(Arc::ptr_eq(host.current_channels.as_ref().unwrap(),host.previous_channels.as_ref().unwrap()));
    assert_eq!(host.output_entries().count(),1);
    assert_eq!(match host.output_channels("out").unwrap().get("payload").unwrap(){NeuralValue::Atom(Atom::String(text))=>text.as_ptr(),_=>unreachable!()},payload);
    let ((snapshot,channels),lease)=observe(||host.eval_baseline());
    assert_eq!((lease.requested_bytes,lease.released_bytes),(0,0));
    assert!(Arc::ptr_eq(snapshot.as_ref().unwrap(),host.previous_snapshot.as_ref().unwrap()));assert!(Arc::ptr_eq(channels.as_ref().unwrap(),host.previous_channels.as_ref().unwrap()));
    assert_eq!(match channels.as_ref().unwrap().outputs.get("out").unwrap().get("payload").unwrap(){NeuralValue::Atom(Atom::String(text))=>text.as_ptr(),_=>unreachable!()},payload);
    session.previous_snapshot=snapshot;session.previous_channels=channels;
    let (_,alias)=observe(||{drop(host.previous_snapshot.take());drop(host.previous_channels.take());});
    assert_eq!((alias.requested_bytes,alias.released_bytes),(0,0));host.retire_cold();
    close(session,1,source.requested_bytes-source.released_bytes+born);
}

fn close(mut session: FlowEvalSession, copy_budget: usize, original_bytes: usize) -> usize {
    use semio_framework_job::InteractiveJobCloseStep as Step;
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    session.begin_close();
    let mut born=0;let mut released=0;
    for _ in 0..1_000_000 {
        if session.terminal_is_empty(){break;}
        let copy=copy_budget;
        let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:session.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:session.next_close_release_byte_demand().unwrap(),maximum_depth:session.next_close_depth_demand().unwrap()};
        let (step,heap)=observe(||session.close_step(RetainedCloneGrant{maximum_items:0,..grant}));
        assert!(matches!(step,Step::Pending{progress} if progress==RetainedCloneProgress::default()));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        for denied in (grant.maximum_capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes.saturating_sub(1),..grant}).into_iter().chain((grant.maximum_release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:grant.maximum_release_bytes.saturating_sub(1),..grant})){
            let (step,heap)=observe(||session.close_step(denied));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
            assert!(matches!(step,Step::Pending{progress} if progress==RetainedCloneProgress::default())||matches!(step,Step::Refused(_)));
        }
        let (step,heap)=observe(||session.close_step(grant));
        let receipt=match step{Step::Pending{progress}|Step::Complete{progress}=>progress,step=>panic!("funded original session close refused: {step:?}")};
        assert!(receipt.fits(grant));assert!(!heap.overflowed);
        assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));
        born+=heap.requested_bytes;released+=heap.released_bytes;
    }
    assert!(session.terminal_is_empty());
    assert_eq!(released,original_bytes+born);
    let (_,heap)=observe(||drop(session));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    eprintln!("[DEBUG] Original Flow session source={original_bytes} admittedBirth={born} physicalRelease={released} terminalDrop=0 copy={copy_budget}");
    released
}

#[test]
fn session_semantic_bytes_larger_than_production_grant_retire_exactly_across_workers() {
    use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🧹️session-retirement/🔣️.json")).unwrap();
    for copy in [1,64,4096] {
        let text=fixture["text"]["text"].as_str().unwrap().repeat(fixture["text"]["repeat"].as_u64().unwrap() as usize);
        let preview=fixture["preview"]["text"].as_str().unwrap().repeat(fixture["preview"]["repeat"].as_u64().unwrap() as usize);
        let (session,heap)=observe(||{
            let mut session=FlowEvalSession::new();
            let primary=Arc::get_mut(session.eval_json.as_mut().unwrap()).unwrap();
            primary.reserve_exact(fixture["text"]["reservedCapacity"].as_u64().unwrap() as usize);primary.push_str(&text);
            session.painted_eval_json=text.clone();session.converged_eval_json=preview.clone();
            session.extension_evaluate_fault=Some(ExtensionEvaluateFault{extension_id:text.clone(),capability:preview.clone(),code:text.clone(),message:preview.clone()});
            session.window_tick_latches.insert(1,FlowEvalWindowTickLatch{armed:true,..Default::default()});
            session.preview_mesh_pack_by_handle.insert("mesh".into(),preview.clone());
            session.pending_tessellate_by_hash.insert(1,"pending".into());
            session.live_geometry_handles.insert("geometry".into(),());
            session.previous_channels=Some(Arc::new(EvalChannels{outputs:HistoryFoldIndex::from([("output".into(),Dictionary::new().insert("label",NeuralValue::Atom(Atom::String(preview.clone()))))]),inputs:HistoryFoldIndex::new()}));
            session.neural_cache().seed(1,Dictionary::new().insert("label",NeuralValue::Atom(Atom::String(text.clone())))).unwrap_or_else(|_|panic!("original cache admission contended"));
            session
        });
        let original=heap.requested_bytes.checked_sub(heap.released_bytes).unwrap();
        std::thread::spawn(move||close(session,copy,original)).join().unwrap();
    }
}

#[test]
fn empty_reserved_text_preserves_whole_original_capacity_until_release_grant() {
    let (session,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{
        let mut session=FlowEvalSession::new();
        Arc::get_mut(session.eval_json.as_mut().unwrap()).unwrap().reserve_exact(65536);
        session
    });
    close(session,1,heap.requested_bytes-heap.released_bytes);
}

#[test]
fn live_session_drop_is_rejected_without_recursive_payload_destruction() {
    assert!(std::panic::catch_unwind(|| drop(FlowEvalSession::new())).is_err());
}

/// 🏠️ Pure source handoff retains the whole original host until every real close receipt completes.
#[test]
fn original_host_retirement_preserves_whole_source_and_full_close_receipts(){
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 let law:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🎟️host-source/🔣️.json")).unwrap();
 flow_extension_registry_generation();
 for copy in law["copyGrants"].as_array().unwrap(){
  let (host,source_heap)=observe(||{
   let mut host=FlowHost::default();let baseline=host.host_snapshot.clone();assert!(host.history_store_from_baseline(baseline).is_some(),"original Host store authority installed");
   let mut text=String::with_capacity(law["capacityBytes"].as_u64().unwrap()as usize);text.push_str(&law["text"].as_str().unwrap().repeat(law["repeat"].as_u64().unwrap()as usize));assert_eq!(text.len(),law["utf8Bytes"].as_u64().unwrap()as usize);
   host.host_catalogue_json=text;host.edited_note=Some(String::from("retained-original-note"));host
  });
  let source_bytes=source_heap.requested_bytes-source_heap.released_bytes;let pointers=(host.host_catalogue_json.as_ptr(),host.edited_note.as_ref().unwrap().as_ptr());
  let (mut retirement,handoff)=observe(||FlowHostRetirement::new(host));assert_eq!((handoff.requested_bytes,handoff.released_bytes),(0,0));let source=retirement.state.source.as_ref().unwrap();assert_eq!((source.host_catalogue_json.as_ptr(),source.edited_note.as_ref().unwrap().as_ptr()),pointers);
  let(mut born,mut freed,mut turns,mut stalled)=(0,0,0,0);
  while !retirement.terminal_is_empty(){
   turns+=1;assert!(turns<law["maximumTurns"].as_u64().unwrap());let copy=copy.as_u64().unwrap()as usize;
   let (demands,query_heap)=observe(||(retirement.next_close_capacity_byte_demand(copy).unwrap(),retirement.next_close_release_byte_demand().unwrap(),retirement.next_close_depth_demand().unwrap()));assert_eq!((query_heap.requested_bytes,query_heap.released_bytes),(0,0));
   let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:demands.0,maximum_release_bytes:demands.1,maximum_depth:demands.2};
   for _ in 0..law["expected"]["denialRepeats"].as_u64().unwrap(){
    let(step,heap)=observe(||retirement.close_step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    if grant.maximum_capacity_bytes>0{let(step,heap)=observe(||retirement.close_step(RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
    if grant.maximum_release_bytes>0{let(step,heap)=observe(||retirement.close_step(RetainedCloneGrant{maximum_release_bytes:grant.maximum_release_bytes-1,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
    if grant.maximum_depth>0{let(error,heap)=observe(||retirement.close_step(RetainedCloneGrant{maximum_depth:grant.maximum_depth-1,..grant}).unwrap_err());assert_eq!(error.retained_progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   }
   let(step,heap)=observe(||retirement.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;
   stalled=if step.progress()==RetainedCloneProgress::default(){stalled+1}else{0};assert!(stalled<64,"original Host must progress with fixed positive copy grant");
  }
  assert_eq!(freed,source_bytes+born);let(_,terminal)=observe(||drop(retirement));assert_eq!((terminal.requested_bytes,terminal.released_bytes),(0,0));eprintln!("[DEBUG] original whole Host copy={copy} pureSourceHandoff=true originalPointers=true source={source_bytes} admitted={born} physical={freed} turns={turns} terminalDrop0");
 }
}

fn dag_retirement_fixture() -> (DagHostRetirement, usize) {
    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🧹️session-retirement/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let dag_fixture = fixture.get("dag").unwrap();
    let repeat = dag_fixture.get("repeat").and_then(semio_framework_pack_json::Value::as_u64).unwrap() as usize;
    let text = dag_fixture.get("text").and_then(semio_framework_pack_json::Value::as_str).unwrap().repeat(repeat);
    let minimum_bytes = dag_fixture.get("minimumUtf8Bytes").and_then(semio_framework_pack_json::Value::as_u64).unwrap() as usize;
    let node = DagNodeSpec {
        id: dag_fixture.get("nodeId").and_then(semio_framework_pack_json::Value::as_str).unwrap().into(),
        name: dag_fixture.get("nodeName").and_then(semio_framework_pack_json::Value::as_str).unwrap().into(),
        abbreviation: "RN".into(),
        icon: "note".into(),
        x: 0.0,
        y: 0.0,
        width: 320.0,
        height: 180.0,
        operator_kind: None,
        properties: PropertyBag::new(),
        kind: DagNodeKind::Note { text, output: IoPortSpec::simple("out", "note") },
    };
    let host =
        DagHost::from_host_snapshot_without_layout(DagHostSnapshot { schema: dag_fixture.get("schemaText").and_then(semio_framework_pack_json::Value::as_str).unwrap().into(), camera: DagCamera { x: 0.0, y: 0.0, zoom: 1.0 }, nodes: vec![node], edges: Vec::new() });
    let (owner,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||DagHostRetirement::new(host));
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0),"original DAG constructor preserves source without heap work");
    (owner, minimum_bytes)
}

#[test]
fn original_dag_host_retirement_preserves_source_and_independent_physical_receipts(){
 use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
 for copy in [1,3,64]{
  let(mut owner,minimum_bytes)=dag_retirement_fixture();
  let original=owner.original().unwrap().host_snapshot.schema.as_ptr();
  let(step,heap)=observe(||owner.close_step(RetainedCloneGrant::default()).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.original().unwrap().host_snapshot.schema.as_ptr(),original);
  let(mut born,mut freed,mut turns)=(0,0,0);
  while !owner.terminal_is_empty(){turns+=1;assert!(turns<262144);let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_close_capacity_byte_demand(copy).unwrap(),maximum_release_bytes:owner.next_close_release_byte_demand().unwrap(),maximum_depth:owner.next_close_depth_demand().unwrap()};
   if grant.maximum_capacity_bytes>0{let(step,heap)=observe(||owner.close_step(RetainedCloneGrant{maximum_capacity_bytes:grant.maximum_capacity_bytes-1,..grant}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   if grant.maximum_release_bytes>0{let(step,heap)=observe(||owner.close_step(RetainedCloneGrant{maximum_release_bytes:grant.maximum_release_bytes-1,..grant}).unwrap());assert_eq!(step.progress(),RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));}
   let(step,heap)=observe(||owner.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));born+=heap.requested_bytes;freed+=heap.released_bytes;
  }
  assert!(freed>=born+minimum_bytes);let(_,heap)=observe(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original whole DAG copy={copy} sameSource=true admitted={born} physical={freed} turns={turns} terminalDrop0");
 }
}

#[test]
fn session_close_dag_host_nonterminal_drop_refuses_recursive_release() {
    let (retirement, _) = dag_retirement_fixture();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| drop(retirement))).is_err());
}

#[test]
fn session_close_vector_scene_retirement_retains_and_reuses_exact_slot() {
    use semio_framework_canvas::{advance_opaque_scene_retirement, append_svg_document, publish_opaque_scene_retirement, reserve_opaque_scene_retirement, Affine, BezPath, Color, FillRule, OpaqueSceneRetirementStep, Scene, SvgDocument};

    let fixture = semio_framework_pack_json::parse(include_str!("../../🧫️fixtures/🧹️session-retirement/🔣️.json"), semio_framework_pack_json::JsonMemberPolicy::Reject).unwrap();
    let capacity = fixture.get("scene").and_then(|value| value.get("retirementCapacity")).and_then(semio_framework_pack_json::Value::as_u64).unwrap() as usize;
    let retained_commands = fixture.get("scene").and_then(|value| value.get("retainedCommands")).and_then(semio_framework_pack_json::Value::as_u64).unwrap() as usize;
    let retained_path_elements = fixture.get("scene").and_then(|value| value.get("retainedPathElements")).and_then(semio_framework_pack_json::Value::as_u64).unwrap() as usize;
    let retained_vello_rects = fixture.get("scene").and_then(|value| value.get("retainedVelloRects")).and_then(semio_framework_pack_json::Value::as_u64).unwrap() as usize;
    for index in 0..=capacity {
        let token = reserve_opaque_scene_retirement().expect("terminal vector scene retirement slot is reusable");
        let mut scene = Scene::new();
        for _ in 0..if index == 1 { retained_commands } else { 1 } {
            scene.pop_layer();
        }
        if index == 0 {
            let mut svg = String::from("<svg xmlns='http://www.w3.org/2000/svg' width='256' height='256'>");
            for rect in 0..retained_vello_rects {
                svg.push_str(&format!("<rect x='{}' y='{}' width='1' height='1' fill='black'/>", rect % 256, rect / 256));
            }
            svg.push_str("</svg>");
            let document = SvgDocument::parse_icons(&svg).expect("retained Vello fragment SVG remains canonical");
            append_svg_document(&mut scene, &document);
        }
        if index == 1 {
            let mut path = BezPath::new();
            path.move_to((0.0, 0.0));
            for point in 0..retained_path_elements {
                path.line_to((point as f64, point as f64));
            }
            scene.fill(FillRule::NonZero, Affine::IDENTITY, Color::from_rgba8(0, 0, 0, 255), None, &path);
        }
        publish_opaque_scene_retirement(token, scene);
        assert_eq!(advance_opaque_scene_retirement(token, 0, usize::MAX), OpaqueSceneRetirementStep::Blocked);
        let mut credited_total = 0usize;
        let mut released_total = 0usize;
        let mut outstanding_credit = 0usize;
        loop {
            match advance_opaque_scene_retirement(token, 1, 4096) {
                OpaqueSceneRetirementStep::Blocked | OpaqueSceneRetirementStep::Fault => panic!("positive scene credit lost the exact cursor"),
                OpaqueSceneRetirementStep::Pending { released_items, credited_bytes, released_bytes } => {
                    assert!(released_items <= 1 && credited_bytes <= 4096);
                    credited_total += credited_bytes;
                    outstanding_credit += credited_bytes;
                    assert!(released_bytes <= outstanding_credit);
                    outstanding_credit -= released_bytes;
                    released_total += released_bytes;
                }
                OpaqueSceneRetirementStep::Complete { released_items, credited_bytes, released_bytes } => {
                    assert_eq!(released_items, 1);
                    assert!(credited_bytes <= 4096);
                    credited_total += credited_bytes;
                    outstanding_credit += credited_bytes;
                    assert!(released_bytes <= outstanding_credit);
                    outstanding_credit -= released_bytes;
                    released_total += released_bytes;
                    break;
                }
            }
        }
        assert_eq!(outstanding_credit, 0);
        assert_eq!(credited_total, released_total);
        assert!(released_total > 0);
        if index <= 1 {
            assert!(released_total > 4096, "large scene or Vello backing crosses multiple retained credits before actual terminal release");
        }
    }
}
//#endregion 🧪️SessionRetirement

//#region 🖐️GestureHistoryRetirement
/// 🖐️ LAW: a gesture that changed NOTHING — every plain click on the graph — must retire its history
/// baseline instead of dropping it.
///
/// `begin_gesture` clones the whole `FlowHostSnapshot` as the undo baseline, and a `FlowHostSnapshot` owns the
/// fail-closed `OrderedMap<WidgetLayout>` root. `commit_gesture_history` only consumed that clone on
/// the `content_changed` branch, so a no-op gesture let it fall out of scope and the process aborted
/// with `ordered-map root must be explicitly retired before drop`. A bare drop aborts rather than
/// unwinds, so this law cannot be written with `catch_unwind`: it is the RUN that proves it, exactly
/// as it was the run on 6118 that found it — the first click on the node graph killed the pool worker
/// (ticket 26/09/09/PROCEDURAL-3D-END-TO-END, `📓️wgpu-node-graph-surface-retention-2026-09-13.md`).
#[test]
fn a_gesture_that_changed_nothing_retires_its_history_baseline() {
    let json = r#"{
  "schema": "flow.host_snapshot",
  "camera": { "x": 0, "y": 0, "zoom": 1 },
  "widgets": [{ "id": "rect", "kind": "neuron", "neuronKind": "rectangle" }],
  "synapses": [],
  "layout": { "rect": { "x": 40, "y": 40 } }
}
"#;
    let fixture = FlowHost::parse_host_snapshot_json(json).expect("fixture json");
    let mut host = FlowHost::from_host_snapshot(fixture);
    host.set_viewport(1280, 800, 1.0);
    host.rebuild_dag();
    for _ in 0..3 {
        host.pointer_down_screen(4.0, 4.0, 0, false, false, false, false);
        host.pointer_up_screen(4.0, 4.0, false, false, false);
    }
    assert!(!host.widget_drag_active(), "a bare press and release leaves no gesture in flight");
    host.retire_cold();
}

/// 🧭️ LAW: arming an undo baseline while one is ALREADY armed must retire the one it replaces.
///
/// The bounded pointer path arms a baseline on the press that STARTS a gesture — it calls
/// `begin_gesture` when the plan goes from idle to active — and closes it on the release. But the
/// projection a plan is derived from is rebuilt as IDLE by `refresh_interaction_projection`, which
/// every screen-path gesture triggers through `resync_interaction_projection`. So a bounded gesture
/// interleaved with a screen-path one is seen as never having started: its release closes nothing,
/// its baseline stays armed, and the NEXT press's `begin_gesture` simply assigned over it — dropping a
/// `FlowHostSnapshot`, and with it an unretired `OrderedMap<WidgetLayout>` root. The pool worker aborted
/// with `ordered-map root must be explicitly retired before drop`, measured on 6118 as the FOURTH
/// middle-button pan of one session (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
/// `📓️wgpu-node-graph-gestures-2026-09-13.md` §4). A bare drop aborts rather than unwinds, so — like
/// its neighbour above — it is the RUN that proves it.
#[test]
fn arming_a_second_history_baseline_retires_the_first() {
    let json = r#"{
  "schema": "flow.host_snapshot",
  "camera": { "x": 0, "y": 0, "zoom": 1 },
  "widgets": [{ "id": "rect", "kind": "neuron", "neuronKind": "rectangle" }],
  "synapses": [],
  "layout": { "rect": { "x": 40, "y": 40 } }
}
"#;
    let fixture = FlowHost::parse_host_snapshot_json(json).expect("fixture json");
    let mut host = FlowHost::from_host_snapshot(fixture);
    host.set_viewport(1280, 800, 1.0);
    host.rebuild_dag();
    let pan = |x: f64, y: f64| dag::DagPointerIntent { phase: dag::DagPointerPhase::Down, x, y, button: 1, shift: false, ctrl_or_meta: false, alt: false, pan: true };
    for turn in 0..8 {
        let plan = host.plan_pointer(pan(20.0 + turn as f64, 20.0)).expect("the pan press plans");
        assert!(host.commit_pointer(plan), "turn {turn}: the pan press commits");
        host.resync_interaction_projection();
    }
    host.retire_cold();
}
//#endregion 🖐️GestureHistoryRetirement
