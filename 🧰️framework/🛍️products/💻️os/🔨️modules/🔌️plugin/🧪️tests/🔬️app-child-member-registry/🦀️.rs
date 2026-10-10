mod child_member_registry_tests {
    use super::*;
    /// 🧪️ Uses the actual funded payload producer and bounded capture to witness original identity and every native effect.
    #[test]
    fn original_mounted_fault_capture_preserves_native_payload_and_independent_copy_turns(){
        use semio_framework_job::{JobPayloadAuthority,RetainedPayloadBuilder,JobPayloadStream,OperationId,Generation,StepContext,StepBudget};
        use semio_framework_async::{CancelToken,CancelTokenRetirement};
        use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;
        let law:serde_json::Value=serde_json::from_str(include_str!("../🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🪟️outcome-borrow/🔣️.json")).unwrap();
        let values=law["faultCapture"]["originalGrant"].as_array().unwrap();let grant=RetainedCloneGrant{maximum_items:values[0].as_u64().unwrap()as usize,maximum_copy_bytes:values[1].as_u64().unwrap()as usize,maximum_capacity_bytes:values[2].as_u64().unwrap()as usize,maximum_release_bytes:values[3].as_u64().unwrap()as usize,maximum_depth:values[4].as_u64().unwrap()as usize};
        let source:[u8;321]=std::array::from_fn(|index|(index%251)as u8);let source_pointer=source.as_ptr();let operation=OperationId(99271);let generation=Generation(7);let mut born=0;let mut released=0;
        let((cancel,birth),heap)=observe(||CancelToken::admit_root(grant).unwrap().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(birth.retained_capacity_bytes,0));assert_eq!(birth.copied_bytes,0);assert!(birth.fits(grant));born+=heap.requested_bytes;
        let((mut authority,birth),heap)=observe(||JobPayloadAuthority::admit(operation,generation,grant).unwrap().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(birth.retained_capacity_bytes,0));assert_eq!(birth.copied_bytes,0);assert!(birth.fits(grant));born+=heap.requested_bytes;
        let mut sequence=0;let mut first=RetainedPayloadBuilder::new(JobPayloadStream::Fault);let mut other=RetainedPayloadBuilder::new(JobPayloadStream::Fault);
        for owner in [&mut first,&mut other]{
            let mut cursor=0;let mut sealed=false;let mut turns=0;let mut copied=0;
            while !sealed{
                let mut receipt=RetainedCloneProgress::default();let(_,heap)=observe(||{
                    let mut context=StepContext::with_payload_authority(operation,generation,StepBudget::new(1,u64::MAX,grant),&cancel,||Some(0),&mut sequence,&mut receipt,&authority).unwrap();
                    if !owner.is_initialized(){owner.advance_initialization(&mut context).unwrap();}else if cursor<source.len(){owner.append_original(&mut context,&source,&mut cursor).unwrap();}else{sealed=owner.seal(&mut context).unwrap();}
                });
                assert!(receipt.fits(grant));assert_eq!(receipt.copied_items,1);assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;copied+=receipt.copied_bytes;turns+=1;assert!(turns<4096);assert_eq!(source.as_ptr(),source_pointer);
            }
            assert_eq!(copied,source.len());assert_eq!(owner.published().unwrap().len(),source.len());
        }
        let original=first.published().unwrap();let pointer=original as*const _;let page_pointer=original.page(0).unwrap().as_ptr();assert_eq!(original.page_count(),law["faultCapture"]["pageBytes"].as_array().unwrap().len());assert_ne!(pointer,other.published().unwrap()as*const _);
        let mut capture=None;let mut recipient=None;let mut copied=0;let mut turns=0;
        while recipient.is_none(){
            let(demand,heap)=observe(||original_fault_capture_demands(&capture,original).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!((demand.capacity_bytes,demand.release_bytes,demand.depth),(0,0,1));
            let prior=capture.as_ref().map(|owner:&MountedWorkerFaultCapture|(owner.original,owner.expected,owner.page,owner.offset,owner.bounded.len));
            for mask in 0..32{
                if mask&(1|16)==0&&(demand.copy_bytes==0||mask&2==0){continue;}
                let denied=RetainedCloneGrant{maximum_items:if mask&1!=0{0}else{grant.maximum_items},maximum_copy_bytes:if mask&2!=0{0}else{grant.maximum_copy_bytes},maximum_capacity_bytes:if mask&4!=0{0}else{grant.maximum_capacity_bytes},maximum_release_bytes:if mask&8!=0{0}else{grant.maximum_release_bytes},maximum_depth:if mask&16!=0{0}else{grant.maximum_depth}};
                let(step,heap)=observe(||advance_original_fault_capture(&mut capture,original,&mut recipient,denied));assert_eq!(step.progress().unwrap_or_default(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(capture.as_ref().map(|owner|(owner.original,owner.expected,owner.page,owner.offset,owner.bounded.len)),prior);assert!(recipient.is_none());
            }
            if capture.is_some(){
                let(step,heap)=observe(||advance_original_fault_capture(&mut capture,other.published().unwrap(),&mut recipient,grant));assert!(matches!(step,PluginLifecycleStep::Blocked{..}));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(capture.as_ref().map(|owner|(owner.original,owner.expected,owner.page,owner.offset,owner.bounded.len)),prior);assert!(recipient.is_none());
            }
            let incoming=RetainedCloneGrant{maximum_capacity_bytes:0,maximum_release_bytes:0,..grant};let(step,heap)=observe(||advance_original_fault_capture(&mut capture,original,&mut recipient,incoming));let progress=step.progress().unwrap();assert!(progress.fits(incoming));assert_eq!(progress.copied_items,1);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));copied+=progress.copied_bytes;turns+=1;assert!(turns<64);if let Some(owner)=capture.as_ref(){assert_eq!(owner.bounded.as_bytes(),&source[..owner.bounded.len]);}assert_eq!(first.published().unwrap()as*const _,pointer);assert_eq!(original.page(0).unwrap().as_ptr(),page_pointer);
        }
        assert!(capture.is_none());assert_eq!(recipient.as_ref().unwrap().as_bytes(),source);assert_eq!(copied,source.len());assert_eq!(turns,law["faultCapture"]["copyReceipts"].as_array().unwrap().len()+2);
        let(step,heap)=observe(||advance_original_fault_capture(&mut capture,original,&mut recipient,grant));assert!(matches!(step,PluginLifecycleStep::Blocked{..}));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(recipient.as_ref().unwrap().as_bytes(),source);
        for owner in [&mut first,&mut other]{let mut turns=0;while !owner.terminal_is_empty(){let(step,heap)=observe(||owner.close_step_granted(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));born+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<4096);}}
        let(step,heap)=observe(||authority.close_step(grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress().retained_capacity_bytes,step.progress().released_bytes));released+=heap.released_bytes;assert!(authority.terminal_is_empty());
        let mut cancellation=CancelTokenRetirement::from_token(cancel);let mut cancel_turns=0;while !cancellation.terminal_is_empty(){let(step,heap)=observe(||cancellation.close_step(grant).unwrap());let progress=step.progress();assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));released+=heap.released_bytes;cancel_turns+=1;assert!(cancel_turns<8);}
        assert_eq!(released,born);let(_,heap)=observe(||drop((first,other,authority,cancellation,capture,recipient)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        eprintln!("[DEBUG] actual original mounted fault source321 captureTurns={turns} copied={copied} originalPayload={pointer:p} originalPage={page_pointer:p} birth={born} released={released} sameContentOtherOwnerRefused=true pureQuotes0heap independentDenied0heap capture0heap terminalDrop0");
    }
    #[test]
    fn original_scheduled_archive_close_preserves_actual_native_progress_and_registry_backing(){
        use crate::test_app_mutation_fixture::{TestSnapshot,TestMutation};
        use semio_framework_trace::observe_heap_allocations_on_this_thread;
        let law:serde_json::Value=serde_json::from_str(include_str!("../🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🗃️archive-turn/🔣️.json")).unwrap();
        let values=law["admitted"].as_array().unwrap();let grant=RetainedCloneGrant{maximum_items:values[0].as_u64().unwrap()as usize,maximum_copy_bytes:values[1].as_u64().unwrap()as usize,maximum_capacity_bytes:values[2].as_u64().unwrap()as usize,maximum_release_bytes:values[3].as_u64().unwrap()as usize,maximum_depth:values[4].as_u64().unwrap()as usize};
        for count in [1,2]{
            let(mut registry,heap)=observe_heap_allocations_on_this_thread(||{
                let mut registry=ArtifactFixedRegistry::new();
                for operation in 19..19+count{let bytes=||{let mut value=Vec::with_capacity(8192);value.extend_from_slice(b"original/alpha\0utf8");value};let archive=protocol::DocumentArchivePack{parent_pack:bytes(),parent_spr:bytes(),members:Vec::with_capacity(3)};let decoder=protocol::RetainedHistoryDecode::new_persisted_document(archive.parent_spr.len(),protocol::RetainedSprLimits{file_bytes:protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES as u64,frame_body_bytes:65536,records:8192}).unwrap();registry.insert_admitted(operation,ActiveDocumentArchiveLoad::<TestSnapshot,TestMutation>::new(operation,archive,decoder));}registry
            });
            let held=heap.requested_bytes-heap.released_bytes;let original=registry.get(19).unwrap().archive.as_ref().unwrap().parent_spr.as_ptr();let mut births=0;let mut released=0;let mut turns=0;let mut terminal_marks=0;let mut removals=0;
            while original_document_archive_close_demands(&registry,grant.maximum_copy_bytes,true).unwrap().is_some(){
                let(demand,heap)=observe_heap_allocations_on_this_thread(||original_document_archive_close_demands(&registry,grant.maximum_copy_bytes,true).unwrap().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(demand.copy_bytes<=grant.maximum_copy_bytes&&demand.capacity_bytes<=grant.maximum_capacity_bytes&&demand.release_bytes<=grant.maximum_release_bytes&&demand.depth<=grant.maximum_depth);
                let pending=registry.len();let prior=registry.get(19).map(|active|(active.state,active.phase,active.terminal_target,active.archive.as_ref().map(|archive|archive.parent_spr.as_ptr())));
                for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),Some(RetainedCloneGrant{maximum_depth:demand.depth.saturating_sub(1),..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant})].into_iter().flatten(){let(step,heap)=observe_heap_allocations_on_this_thread(||advance_original_document_archive_close(&mut registry,denied,true).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(registry.len(),pending);assert_eq!(registry.get(19).map(|active|(active.state,active.phase,active.terminal_target,active.archive.as_ref().map(|archive|archive.parent_spr.as_ptr()))),prior);if turns==0{assert_eq!(registry.get(19).unwrap().archive.as_ref().unwrap().parent_spr.as_ptr(),original);}}
                let before_terminal=registry.get(19).is_some_and(ActiveDocumentArchiveLoad::terminal);let(step,heap)=observe_heap_allocations_on_this_thread(||advance_original_document_archive_close(&mut registry,grant,true).unwrap());assert!(matches!(step,RetainedCloneStep::Progress(_)));let progress=step.progress();assert!(progress.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(progress.retained_capacity_bytes,progress.released_bytes));births+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<100000);if registry.len()<pending{removals+=1;assert_eq!((progress.copied_items,progress.copied_bytes,progress.retained_capacity_bytes,progress.released_bytes),(1,0,0,0));}if !before_terminal&&registry.get(19).is_some_and(ActiveDocumentArchiveLoad::terminal){terminal_marks+=1;assert!(registry.get(19).unwrap().terminal_is_empty());assert_eq!((progress.copied_items,progress.copied_bytes,progress.retained_capacity_bytes,progress.released_bytes),(1,0,0,0));}
            }
            assert_eq!(removals,count);assert_eq!(terminal_marks,1);assert!(registry.is_empty());assert_eq!(registry.empty_backing_byte_demand(),Some(0));assert_eq!(released,held+births);let(_,heap)=observe_heap_allocations_on_this_thread(||drop(registry));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original scheduled Archive count={count} turns={turns} original={held} births={births} released={released} queryHeap0 allDeniedHeap0 statusAfterEmpty=true removalSeparate=true backingSeparate=true terminalDrop0");
        }
    }
    #[test]
    fn original_exchange_intersects_all_native_authority_axes_without_heap_effects(){
        let law:serde_json::Value=serde_json::from_str(include_str!("../🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🤝️exchange-context/🔣️.json")).unwrap();
        let grant=|name:&str|{let v=law[name].as_array().unwrap();RetainedCloneGrant{maximum_items:v[0].as_u64().unwrap()as usize,maximum_copy_bytes:v[1].as_u64().unwrap()as usize,maximum_capacity_bytes:v[2].as_u64().unwrap()as usize,maximum_release_bytes:v[3].as_u64().unwrap()as usize,maximum_depth:v[4].as_u64().unwrap()as usize}};
        let policy=grant("phasePolicy");let incoming=grant("incomingRemaining");let expected=grant("admitted");
        let(actual,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::plugin_runtime::original_plugin_turn_grant(policy,incoming));assert_eq!(actual,expected);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        for mask in 0..32{let denied=RetainedCloneGrant{maximum_items:if mask&1!=0{0}else{incoming.maximum_items},maximum_copy_bytes:if mask&2!=0{0}else{incoming.maximum_copy_bytes},maximum_capacity_bytes:if mask&4!=0{0}else{incoming.maximum_capacity_bytes},maximum_release_bytes:if mask&8!=0{0}else{incoming.maximum_release_bytes},maximum_depth:if mask&16!=0{0}else{incoming.maximum_depth}};let(actual,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||crate::plugin_runtime::original_plugin_turn_grant(policy,denied));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(actual,RetainedCloneGrant{maximum_items:if mask&1!=0{0}else{expected.maximum_items},maximum_copy_bytes:if mask&2!=0{0}else{expected.maximum_copy_bytes},maximum_capacity_bytes:if mask&4!=0{0}else{expected.maximum_capacity_bytes},maximum_release_bytes:if mask&8!=0{0}else{expected.maximum_release_bytes},maximum_depth:if mask&16!=0{0}else{expected.maximum_depth}});}
        eprintln!("[DEBUG] original exchange native authority axes=5 denialShapes=32 heapBirth=0 heapRelease=0 originalPolicyUnchanged=true");
    }
    #[test]
    fn original_archive_history_and_physical_backing_retire_under_full_native_grants(){
        use crate::test_app_mutation_fixture::{TestSnapshot,TestMutation};
        use semio_framework_trace::observe_heap_allocations_on_this_thread;
        let law:serde_json::Value=serde_json::from_str(include_str!("../🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🗃️archive-close/🔣️.json")).unwrap();
        let values=law["originalGrant"].as_array().unwrap();let grant=RetainedCloneGrant{maximum_items:values[0].as_u64().unwrap()as usize,maximum_copy_bytes:values[1].as_u64().unwrap()as usize,maximum_capacity_bytes:values[2].as_u64().unwrap()as usize,maximum_release_bytes:values[3].as_u64().unwrap()as usize,maximum_depth:values[4].as_u64().unwrap()as usize};
        for capacity in law["originalCapacities"].as_array().unwrap(){for count in law["originalMemberCounts"].as_array().unwrap(){
            let capacity=capacity.as_u64().unwrap()as usize;let count=count.as_u64().unwrap()as usize;
            let(mut active,heap)=observe_heap_allocations_on_this_thread(||{
                let text=||{let mut value=String::with_capacity(capacity);value.push_str("original/α😀");value};
                let bytes=||{let mut value=Vec::with_capacity(capacity+31);value.extend_from_slice(b"original persisted payload");value};
                let reference=||protocol::DocumentArchiveArtifactRef{artifact_id:text(),artifact_kind:text(),standard:text(),subset:text()};
                let mut members=Vec::with_capacity(count+7);for ordinal in 0..count{members.push(protocol::OwnedDocumentMemberPackEntry{ordinal:ordinal as u32,reference:reference(),owner:protocol::DocumentArchiveOwnerRef{parent:reference(),slot:text(),child_id:text()},envelope_pack:bytes()});}
                let archive=protocol::DocumentArchivePack{parent_pack:bytes(),parent_spr:bytes(),members};
                let history=protocol::RetainedHistoryDecode::new_persisted_document(archive.parent_spr.len(),protocol::RetainedSprLimits{file_bytes:protocol::DOCUMENT_ARCHIVE_MAXIMUM_BYTES as u64,frame_body_bytes:65536,records:8192}).unwrap();
                let mut active=ActiveDocumentArchiveLoad::<TestSnapshot,TestMutation>::new(19,archive,history);
                active.decoded_history=Some(protocol::HistoryLog{doc_id:text(),schema:text(),edits:vec![protocol::HistoryEdit{id:text(),actor:None,started_at:text(),finished_at:Some(text()),verb:Some(text()),line:Some(text()),ops:vec![protocol::OpPayload{text:Some(text()),binary:Some(bytes())}],inverse:vec![protocol::OpPayload{text:None,binary:Some(bytes())}],meta:Some(vec![protocol::HistoryOpMeta{op_id:Some(text()),dependencies:vec![text()],group_id:Some(text()),..Default::default()}]),lane:Some(text())}],transitions:vec![protocol::HistoryTransitionRecord{id:text(),actor:Default::default(),hlt:(1,2,3),dependencies:vec![text()],observed:Some(text()),payload:bytes()}],composition:Some(protocol::HistoryComposition{owner:Some((text(),text(),text())),dialect:Some((text(),text(),text()))}),conflicts:vec![protocol::HistoryConflict{id:text(),kind:0,status:0,actors:vec![Default::default()],hlt:(1,2,3),edit_ids:vec![text()],envelopes:vec![bytes()],messages:vec![protocol::HistoryMessage{level:1,code:text(),message:text(),target:vec![text()],op_index:Some(0)}]}],viewer_line:Some(text()),viewer_checkpoint:Some(text())});active
            });
            let held=heap.requested_bytes-heap.released_bytes;let original=active.archive.as_ref().unwrap().parent_pack.as_ptr();let original_history=active.decoded_history.as_ref().unwrap().doc_id.as_ptr();let mut births=0;let mut released=0;let mut turns=0;
            while !active.terminal_is_empty(){
                let(demand,heap)=observe_heap_allocations_on_this_thread(||active.retirement_demands(grant.maximum_copy_bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(demand.copy_bytes<=grant.maximum_copy_bytes&&demand.capacity_bytes<=grant.maximum_capacity_bytes&&demand.release_bytes<=grant.maximum_release_bytes&&demand.depth<=grant.maximum_depth);
                for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),Some(RetainedCloneGrant{maximum_depth:demand.depth-1,..grant}),(demand.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes.saturating_sub(1),..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes.saturating_sub(1),..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes.saturating_sub(1),..grant})].into_iter().flatten(){
                    let(step,heap)=observe_heap_allocations_on_this_thread(||active.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if turns==0{assert_eq!(active.archive.as_ref().unwrap().parent_pack.as_ptr(),original);assert_eq!(active.decoded_history.as_ref().unwrap().doc_id.as_ptr(),original_history);}
                }
                let(step,heap)=observe_heap_allocations_on_this_thread(||active.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=heap.requested_bytes;released+=heap.released_bytes;turns+=1;assert!(turns<100000);if matches!(step,RetainedCloneStep::Complete(_)){assert!(active.terminal_is_empty());}
            }
            assert_eq!(released,held+births);let(_,heap)=observe_heap_allocations_on_this_thread(||drop(active));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original Archive/History native memberCount={count} capacity={capacity} turns={turns} held={held} births={births} released={released} allDenied0heap originalPointers=true terminalDrop0");
        }}
    }
#[test]
    fn original_composition_pin_retirement_preserves_native_fields_and_vector_backing(){
        use semio_framework_trace::observe_heap_allocations_on_this_thread;
        let law:serde_json::Value=serde_json::from_str(include_str!("../🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🔄️replacement-retirement/🔣️.json")).unwrap();let values=law["originalGrant"].as_array().unwrap();let grant=RetainedCloneGrant{maximum_items:values[0].as_u64().unwrap()as usize,maximum_copy_bytes:values[1].as_u64().unwrap()as usize,maximum_capacity_bytes:values[2].as_u64().unwrap()as usize,maximum_release_bytes:values[3].as_u64().unwrap()as usize,maximum_depth:values[4].as_u64().unwrap()as usize};
        for count in [0,1,3]{for index in 0..3{
            let capacity=law["pins"]["capacities"][index].as_u64().unwrap()as usize;let text=law["pins"]["texts"][index].as_str().unwrap();
            let(pins,heap)=observe_heap_allocations_on_this_thread(||{let mut pins=Vec::with_capacity(count+7);for _ in 0..count{let text=||{let mut value=String::with_capacity(capacity);value.push_str(text);value};pins.push(vcs::CompositionPin{child_ref:ArtifactRef{artifact_id:text(),dialect:ArtifactDialect{artifact_kind:text(),standard:text(),subset:text()}},checkpoint_id:text()});}pins});let retained=heap.requested_bytes-heap.released_bytes;let original=pins.as_ptr();let original_text=pins.last().map(|pin|pin.child_ref.artifact_id.as_ptr());let backing=pins.capacity()*size_of::<vcs::CompositionPin>();assert_eq!(retained,backing+count*capacity*5);
            if let Some(pin)=pins.last(){let encoded=semio_framework_pack_json::to_json_string(pin).unwrap();assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded).unwrap()["checkpointId"],law["pins"]["texts"][index]);}
            let mut owner=CompositionPinsRetirement::new(pins);let mut release=0;let mut turns=0;let mut final_release=0;
            while !owner.terminal_is_empty(){
                let(demand,heap)=observe_heap_allocations_on_this_thread(||owner.retirement_demands().unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(demand.copy_bytes>0);let field=owner.field;let len=owner.pins.as_ref().map(Vec::len);
                for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),Some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant}),Some(RetainedCloneGrant{maximum_depth:demand.depth-1,..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant})].into_iter().flatten(){
                    let(step,heap)=observe_heap_allocations_on_this_thread(||owner.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(owner.field,field);assert_eq!(owner.pins.as_ref().map(Vec::len),len);if turns==0{assert_eq!(owner.pins.as_ref().unwrap().as_ptr(),original);assert_eq!(owner.pins.as_ref().unwrap().last().map(|pin|pin.child_ref.artifact_id.as_ptr()),original_text);}
                }
                let(step,heap)=observe_heap_allocations_on_this_thread(||owner.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(receipt.copied_bytes,demand.copy_bytes);assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));release+=receipt.released_bytes;final_release=receipt.released_bytes;turns+=1;assert!(turns<=count*7+1);
            }
            assert_eq!(release,retained);assert_eq!(final_release,backing);let(_,heap)=observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original composition pin count={count} capacity={capacity} turns={turns} originalPointer={original:p} physical={release} finalBacking={final_release} independentSerde=true allDenied0heap terminalDrop0 parentReplacementExcluded=true");
        }}
    }

    #[test]
    fn original_snapshot_return_pump_conserves_box_custody_and_each_full_receipt(){
        use semio_framework_trace::observe_heap_allocations_on_this_thread;
        let law:serde_json::Value=serde_json::from_str(include_str!("../../⏪️time-travel/♻️retirement/🧫️fixtures/🔣️.json")).unwrap();let policy=&law["snapshotReturnGrant"];let grant=RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize};
        for capacity in [0,8192,65536]{
            let(mut pending,heap)=observe_heap_allocations_on_this_thread(||{let mut value=String::with_capacity(capacity);value.push_str("snapshot/Original/α\0😀");Some(value)});let held=heap.requested_bytes-heap.released_bytes;let pointer=pending.as_ref().unwrap().as_ptr();let mut pump=SnapshotReadReturnPump::new();let mut births=0;let mut releases=0;let mut turns=0;
            while pending.is_some()||!pump.terminal_is_empty(){
                let(quote,heap)=observe_heap_allocations_on_this_thread(||pump.retirement_demands(grant.maximum_copy_bytes,||store::artifact_retirement_owned_birth_demands(&pending)).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(quote.copy_bytes<=grant.maximum_copy_bytes&&quote.capacity_bytes<=grant.maximum_capacity_bytes&&quote.release_bytes<=grant.maximum_release_bytes&&quote.depth<=grant.maximum_depth);
                let next_demand=store::artifact_retirement_owned_birth_demands(&pending).unwrap();for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),Some(RetainedCloneGrant{maximum_depth:quote.depth-1,..grant}),(quote.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:quote.copy_bytes-1,..grant}),(quote.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:quote.capacity_bytes-1,..grant}),(quote.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:quote.release_bytes-1,..grant})].into_iter().flatten(){let(step,heap)=observe_heap_allocations_on_this_thread(||pump.drive(next_demand,|original|{let mut next=None;let receipt=store::artifact_retirement_admit_owned(&mut pending,&mut next,original).map_err(|error|plugin_sdk_fault(error.into_message()))?.progress();Ok((next,receipt))},denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if turns==0{assert_eq!(pending.as_ref().unwrap().as_ptr(),pointer)}}
                let(step,heap)=observe_heap_allocations_on_this_thread(||pump.drive(next_demand,|original|{let mut next=None;let receipt=store::artifact_retirement_admit_owned(&mut pending,&mut next,original).map_err(|error|plugin_sdk_fault(error.into_message()))?.progress();Ok((next,receipt))},grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=heap.requested_bytes;releases+=heap.released_bytes;turns+=1;assert!(turns<100000);
            }
            assert!(pending.is_none());assert!(pump.active.is_none());assert_eq!(releases,held+births);let(_,heap)=observe_heap_allocations_on_this_thread(||drop(pump));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original snapshot pump concrete typed Box capacity={capacity} turns={turns} original={held} births={births} releases={releases} originalPointer={pointer:p} allDenied0heap terminalDrop0");
        }
    }

    #[test]
    fn original_child_member_reference_metadata_retires_each_native_field_under_full_grants(){
        use semio_framework_trace::observe_heap_allocations_on_this_thread;
        let law:serde_json::Value=serde_json::from_str(include_str!("../../⏪️time-travel/♻️retirement/🧫️fixtures/🔣️.json")).unwrap();let policy=&law["memberCloseGrant"];let grant=RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize};
        for capacity in [0,8192,65536]{
            let(original,heap)=observe_heap_allocations_on_this_thread(||{let text=|label:&str|{let mut value=String::with_capacity(capacity);value.push_str(label);value};let reference=|label:&str|ArtifactRef{artifact_id:text(label),dialect:ArtifactDialect{artifact_kind:text("kind/Art"),standard:text("standard/Norm"),subset:text("subset/Teil")}};ChildMemberMetadata{reference:reference("child/α\0😀"),owner:store::OwnerRef{parent:reference("parent/Pfad"),slot:text("slot/Kind"),child_id:text("child/Original")}}});let held=heap.requested_bytes-heap.released_bytes;let pointer=original.reference.artifact_id.as_ptr();let(mut owner,heap)=observe_heap_allocations_on_this_thread(||ChildMemberRetirement::<store::NoMembers>{entry:std::mem::ManuallyDrop::new(None),metadata:std::mem::ManuallyDrop::new(Some(semio_framework_value::retirement::controlled::ControlledRetirement::new(original).unwrap_or_else(|_|panic!("actual original child reference metadata"))))});assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut births=0;let mut releases=0;let mut turns=0;
            while !owner.terminal_is_empty(){
                let(quote,heap)=observe_heap_allocations_on_this_thread(||owner.retirement_demands(grant.maximum_copy_bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(quote.copy_bytes<=grant.maximum_copy_bytes&&quote.capacity_bytes<=grant.maximum_capacity_bytes&&quote.release_bytes<=grant.maximum_release_bytes&&quote.depth<=grant.maximum_depth);
                for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),Some(RetainedCloneGrant{maximum_depth:0,..grant}),(quote.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:0,..grant}),(quote.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:0,..grant}),(quote.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:0,..grant})].into_iter().flatten(){let(step,heap)=observe_heap_allocations_on_this_thread(||owner.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if turns==0{assert_eq!(owner.metadata.as_ref().unwrap().original().unwrap().reference.artifact_id.as_ptr(),pointer)}}
                let(step,heap)=observe_heap_allocations_on_this_thread(||owner.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=heap.requested_bytes;releases+=heap.released_bytes;turns+=1;assert!(turns<100000);
            }
            assert!(owner.metadata.is_none());assert!(owner.entry.is_none());assert_eq!(releases,held+births);let(_,heap)=observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original child member ArtifactRef/OwnerRef metadata capacity={capacity} turns={turns} original={held} births={births} releases={releases} originalPointer={pointer:p} allDenied0heap terminalDrop0");
        }
    }

    #[test]
    fn original_child_snapshot_metadata_retires_each_native_field_under_full_grants(){
        use semio_framework_trace::observe_heap_allocations_on_this_thread;
        let law:serde_json::Value=serde_json::from_str(include_str!("../../⏪️time-travel/♻️retirement/🧫️fixtures/🔣️.json")).unwrap();let policy=&law["memberCloseGrant"];let grant=RetainedCloneGrant{maximum_items:policy["items"].as_u64().unwrap()as usize,maximum_copy_bytes:policy["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:policy["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:policy["release"].as_u64().unwrap()as usize,maximum_depth:policy["depth"].as_u64().unwrap()as usize};
        for capacity in [0,8192,65536]{
            let mut children=ChildMemberRegistry::<store::NoMembers>::new();let retainers=ChildContentOwners::none();
            let(original,heap)=observe_heap_allocations_on_this_thread(||{let text=|label:&str|{let mut value=String::with_capacity(capacity);value.push_str(label);value};ChildContentMemberMetadata{key:MemberKey{owner:text("owner/α\0😀"),slot:text("slot/Kind"),child_id:text("child/Pfad")},artifact_id:text("artifact/Original"),dialect:ArtifactDialect{artifact_kind:text("kind/Art"),standard:text("standard/Norm"),subset:text("subset/Teil")}}});let held=heap.requested_bytes-heap.released_bytes;let pointer=original.key.owner.as_ptr();let mut owner=ChildContentRetirement::new(ChildContentView::EMPTY,false);*owner.active_member=Some(original);let mut births=0;let mut releases=0;let mut turns=0;
            while !owner.terminal_is_empty(){
                let(quote,heap)=observe_heap_allocations_on_this_thread(||owner.retirement_demands(&children,None,grant.maximum_copy_bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(quote.copy_bytes<=grant.maximum_copy_bytes&&quote.capacity_bytes<=grant.maximum_capacity_bytes&&quote.release_bytes<=grant.maximum_release_bytes&&quote.depth<=grant.maximum_depth);
                for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),Some(RetainedCloneGrant{maximum_depth:0,..grant}),(quote.copy_bytes>0).then_some(RetainedCloneGrant{maximum_copy_bytes:0,..grant}),(quote.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:0,..grant}),(quote.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:0,..grant})].into_iter().flatten(){let(step,heap)=observe_heap_allocations_on_this_thread(||owner.close_step(&mut children,None,&ChildContentView::EMPTY,&retainers,denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if turns==0{assert_eq!(owner.active_member.as_ref().unwrap().key.owner.as_ptr(),pointer)}}
                let(step,heap)=observe_heap_allocations_on_this_thread(||owner.close_step(&mut children,None,&ChildContentView::EMPTY,&retainers,grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));births+=heap.requested_bytes;releases+=heap.released_bytes;turns+=1;assert!(turns<100000);
            }
            assert!(owner.metadata_close.is_none());assert!(owner.active_member.is_none());assert_eq!(releases,held+births);let(_,heap)=observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original child snapshot metadata capacity={capacity} turns={turns} original={held} births={births} releases={releases} originalPointer={pointer:p} allDenied0heap terminalDrop0");
        }
    }

    #[test]
    fn original_member_ingress_headers_and_empty_registry_backing_keep_full_physical_receipts(){
        use semio_framework_trace::observe_heap_allocations_on_this_thread;
        let law:serde_json::Value=serde_json::from_str(include_str!("../🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/📮️member-ingress/🔣️.json")).unwrap();let values=law["originalGrant"].as_array().unwrap();let grant=RetainedCloneGrant{maximum_items:values[0].as_u64().unwrap()as usize,maximum_copy_bytes:values[1].as_u64().unwrap()as usize,maximum_capacity_bytes:values[2].as_u64().unwrap()as usize,maximum_release_bytes:values[3].as_u64().unwrap()as usize,maximum_depth:values[4].as_u64().unwrap()as usize};
        for index in 0..3{
            let text=law["texts"][index].as_str().unwrap();let capacity=law["capacities"][index].as_u64().unwrap()as usize;
            let(mut owner,birth)=observe_heap_allocations_on_this_thread(||{let text=||{let mut value=String::with_capacity(capacity);value.push_str(text);value};let reference=||ArtifactRef{artifact_id:text(),dialect:ArtifactDialect{artifact_kind:text(),standard:text(),subset:text()}};OwnedDocumentMemberIngress{identity:std::mem::ManuallyDrop::new(Some((17,reference(),store::OwnerRef{parent:reference(),slot:text(),child_id:text()}))),request:std::mem::ManuallyDrop::new(None),identity_field:0}});let original=owner.identity.as_ref().unwrap().1.artifact_id.as_ptr();let retained=birth.requested_bytes-birth.released_bytes;assert_eq!(retained,capacity*law["fields"].as_u64().unwrap()as usize);
            let encoded=semio_framework_pack_json::to_json_string(&owner.identity.as_ref().unwrap().1).unwrap();assert_eq!(serde_json::from_str::<serde_json::Value>(&encoded).unwrap()["artifactId"],law["texts"][index]);let mut release=0;let mut turns=0;
            while !owner.terminal_is_empty(){
                let(demand,heap)=observe_heap_allocations_on_this_thread(||owner.retirement_demands(grant.maximum_copy_bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(demand.copy_bytes>0);
                for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),Some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant}),Some(RetainedCloneGrant{maximum_depth:demand.depth-1,..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant}),(demand.capacity_bytes>0).then_some(RetainedCloneGrant{maximum_capacity_bytes:demand.capacity_bytes-1,..grant})].into_iter().flatten(){let(step,heap)=observe_heap_allocations_on_this_thread(||owner.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));if turns==0{assert_eq!(owner.identity.as_ref().unwrap().1.artifact_id.as_ptr(),original);}}
                let(step,heap)=observe_heap_allocations_on_this_thread(||owner.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(receipt.copied_bytes,demand.copy_bytes);assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));release+=receipt.released_bytes;turns+=1;assert!(turns<=law["fields"].as_u64().unwrap()as usize+1);
            }
            assert_eq!(release,retained);let(_,heap)=observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original ingress metadata capacity={capacity} turns={turns} originalPointer={original:p} physical={release} nativeHeadersPaid=true deniedHeap0 terminalDrop0 independentSerde=true requestProducerExcluded=true");
        }
        let(mut registry,birth)=observe_heap_allocations_on_this_thread(||OwnedDocumentMemberIngressRegistry::try_new(1).unwrap());let original=registry.pages.as_ptr();let retained=birth.requested_bytes-birth.released_bytes;let mut release=0;let mut turns=0;
        while !registry.terminal_is_empty(){let(demand,heap)=observe_heap_allocations_on_this_thread(||registry.retirement_demands(grant.maximum_copy_bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(demand.copy_bytes>0);let sealed=registry.sealed;
            for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),Some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant}),Some(RetainedCloneGrant{maximum_depth:demand.depth-1,..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant})].into_iter().flatten(){let(step,heap)=observe_heap_allocations_on_this_thread(||registry.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(registry.sealed,sealed);if turns==0{assert_eq!(registry.pages.as_ptr(),original);}}
            let(step,heap)=observe_heap_allocations_on_this_thread(||registry.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(receipt.copied_bytes,demand.copy_bytes);assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));release+=receipt.released_bytes;turns+=1;assert!(turns<=3);
        }
        assert_eq!(retained,release);let(_,heap)=observe_heap_allocations_on_this_thread(||drop(registry));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original empty ingress registry turns={turns} originalPointer={original:p} physical={release} paidSeal=true separatePageAndBacking=true deniedHeap0 terminalDrop0 constructorAdmissionExcluded=true occupiedRegistryExcluded=true");
    }

    #[test]
    fn original_child_root_registry_custody_and_full_physical_parent_receipt() {
        use semio_framework_trace::observe_heap_allocations_on_this_thread;
        let law: serde_json::Value = serde_json::from_str(include_str!("../🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/🌳️child-root/🔣️.json")).unwrap();
        let declared = law["originalGrant"].as_array().unwrap();
        let grant = RetainedCloneGrant { maximum_items: declared[0].as_u64().unwrap() as usize, maximum_copy_bytes: declared[1].as_u64().unwrap() as usize, maximum_capacity_bytes: declared[2].as_u64().unwrap() as usize, maximum_release_bytes: declared[3].as_u64().unwrap() as usize, maximum_depth: declared[4].as_u64().unwrap() as usize };
        let mut results = Vec::new();
        for retained in 0..3 {
            let mut children = ChildMemberRegistry::<store::NoMembers>::new();
            let mut retiring = ArtifactFixedRegistry::<ChildMemberRetirement<store::NoMembers>>::new();
            let ((mut registry, original), birth) = observe_heap_allocations_on_this_thread(|| (ArtifactFixedRegistry::<ChildContentRetirement>::new(), ChildContentView { root: Some(std::sync::Arc::new(ChildContentRoot { pages: std::array::from_fn(|_| None), len: 0 })) }));
            assert_eq!(birth.released_bytes, 0);
            let original_pointer = std::sync::Arc::as_ptr(original.root.as_ref().unwrap());
            let original_root_bytes = child_content_arc_bytes::<ChildContentRoot>();
            let original_registry_bytes = registry.empty_backing_byte_demand().unwrap();
            assert_eq!(birth.requested_bytes, original_root_bytes + original_registry_bytes);
            let retained_view = if retained == 0 { None } else {
                let ((review, receipt), heap) = observe_heap_allocations_on_this_thread(|| time_travel::TimeTravelReviewChildren::capture(&original, semio_framework_time_travel::TimeTravelBase { content_revision: [9; 32] }, 7, grant).unwrap());
                assert!(receipt.fits(grant));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                Some(review.view)
            };
            registry.insert_admitted(7, ChildContentRetirement::new(original, false));
            let member = (retained == 1).then_some(retained_view.as_ref()).flatten();
            let review = (retained == 2).then_some(retained_view.as_ref()).flatten();
            let mut cursor = 0;
            let (demand, heap) = observe_heap_allocations_on_this_thread(|| original_child_root_retirement_demands(&registry, &children, Some(&retiring), cursor, grant.maximum_copy_bytes, false).unwrap().unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(demand.copy_bytes, ChildContentBorrowedOwners::copy_reservation() + std::mem::size_of::<Option<std::sync::Arc<ChildContentRoot>>>());
            assert_eq!(demand.release_bytes, if retained == 0 { original_root_bytes } else { 0 });
            let mut denials = vec![RetainedCloneGrant { maximum_items: 0, ..grant }, RetainedCloneGrant { maximum_copy_bytes: demand.copy_bytes - 1, ..grant }, RetainedCloneGrant { maximum_depth: demand.depth - 1, ..grant }];
            if demand.release_bytes != 0 { denials.push(RetainedCloneGrant { maximum_release_bytes: demand.release_bytes - 1, ..grant }); }
            for denied in denials {
                let (step, heap) = observe_heap_allocations_on_this_thread(|| advance_original_child_root_retirement(&mut registry, &mut children, Some(&mut retiring), &ChildContentView::EMPTY, member, review, &mut cursor, denied, false).unwrap());
                assert_eq!(step.progress(), RetainedCloneProgress::default());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
                assert_eq!(cursor, 0);
                assert_eq!(std::sync::Arc::as_ptr(registry.get(7).unwrap().view.root.as_ref().unwrap()), original_pointer);
            }
            let (step, heap) = observe_heap_allocations_on_this_thread(|| advance_original_child_root_retirement(&mut registry, &mut children, Some(&mut retiring), &ChildContentView::EMPTY, member, review, &mut cursor, grant, false).unwrap());
            let first = step.progress();
            assert!(first.fits(grant));
            assert!(matches!(step, RetainedCloneStep::Progress(_)));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (first.retained_capacity_bytes, first.released_bytes));
            assert_eq!(first.copied_bytes, ChildContentBorrowedOwners::copy_reservation() + std::mem::size_of::<Option<std::sync::Arc<ChildContentRoot>>>());
            assert_eq!(first.released_bytes, demand.release_bytes);
            assert!(registry.get(7).unwrap().terminal_is_empty());
            let (remove_demand, heap) = observe_heap_allocations_on_this_thread(|| original_child_root_retirement_demands(&registry, &children, Some(&retiring), cursor, grant.maximum_copy_bytes, false).unwrap().unwrap());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let (denied, heap) = observe_heap_allocations_on_this_thread(|| advance_original_child_root_retirement(&mut registry, &mut children, Some(&mut retiring), &ChildContentView::EMPTY, member, review, &mut cursor, RetainedCloneGrant { maximum_copy_bytes: remove_demand.copy_bytes - 1, ..grant }, false).unwrap());
            assert_eq!(denied.progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(registry.get(7).is_some());
            let (removed, heap) = observe_heap_allocations_on_this_thread(|| advance_original_child_root_retirement(&mut registry, &mut children, Some(&mut retiring), &ChildContentView::EMPTY, member, review, &mut cursor, grant, false).unwrap());
            assert!(removed.progress().fits(grant));
            assert_eq!(removed.progress().copied_bytes, remove_demand.copy_bytes);
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert!(registry.is_empty());
            let backing_pointer = registry.slots.as_ptr();
            let (denied, heap) = observe_heap_allocations_on_this_thread(|| advance_original_child_root_retirement(&mut registry, &mut children, Some(&mut retiring), &ChildContentView::EMPTY, member, review, &mut cursor, RetainedCloneGrant { maximum_release_bytes: original_registry_bytes - 1, ..grant }, true).unwrap());
            assert_eq!(denied.progress(), RetainedCloneProgress::default());
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            assert_eq!(registry.slots.as_ptr(), backing_pointer);
            let (backing, heap) = observe_heap_allocations_on_this_thread(|| advance_original_child_root_retirement(&mut registry, &mut children, Some(&mut retiring), &ChildContentView::EMPTY, member, review, &mut cursor, grant, true).unwrap());
            assert!(backing.progress().fits(grant));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, original_registry_bytes));
            assert_eq!(backing.progress().released_bytes, original_registry_bytes);
            assert_eq!(registry.empty_backing_byte_demand(), Some(0));
            let (complete, heap) = observe_heap_allocations_on_this_thread(|| advance_original_child_root_retirement(&mut registry, &mut children, Some(&mut retiring), &ChildContentView::EMPTY, member, review, &mut cursor, grant, true).unwrap());
            assert_eq!(complete, RetainedCloneStep::Complete(Default::default()));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            let mut release = first.released_bytes + backing.progress().released_bytes;
            if let Some(view) = retained_view {
                assert_eq!(std::sync::Arc::as_ptr(view.root.as_ref().unwrap()), original_pointer);
                let mut owner = ChildContentRetirement::new(view, false);
                let (step, heap) = observe_heap_allocations_on_this_thread(|| owner.close_step(&mut children, Some(&mut retiring), &ChildContentView::EMPTY, &ChildContentOwners::none(), grant).unwrap());
                assert!(step.progress().fits(grant));
                assert!(owner.terminal_is_empty());
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, original_root_bytes));
                release += step.progress().released_bytes;
                let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(owner));
                assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            }
            assert_eq!(birth.requested_bytes, release);
            let (_, heap) = observe_heap_allocations_on_this_thread(|| drop(registry));
            assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
            eprintln!("[DEBUG] original child root retained={retained} originalPointer={original_pointer:p} birth={} release={release} borrowedCopy={} removalCopy={} deniedHeap0 terminalDrop0", birth.requested_bytes, first.copied_bytes, removed.progress().copied_bytes);
            results.push(serde_json::json!({"retained":retained,"birth":birth.requested_bytes,"release":release,"rootBytes":original_root_bytes,"registryBytes":original_registry_bytes,"firstRelease":first.released_bytes,"backingRelease":backing.progress().released_bytes,"borrowedCopy":ChildContentBorrowedOwners::copy_reservation(),"rootHeaderCopy":std::mem::size_of::<Option<std::sync::Arc<ChildContentRoot>>>(),"firstCopy":first.copied_bytes,"removalCopy":removed.progress().copied_bytes,"originalPointer":original_pointer as usize,"deniedBirth":0,"deniedRelease":0,"terminalDropBirth":heap.requested_bytes,"terminalDropRelease":heap.released_bytes}));
        }
        if let Some(path) = std::env::var_os("SEMIO_CHILD_ROOT_RESULTS") {
            let path = std::path::Path::new(&path);
            std::fs::create_dir_all(path.parent().expect("original child output has an explicit ticket directory")).unwrap();
            std::fs::write(path, serde_json::to_vec(&results).unwrap()).unwrap();
        }
    }

    #[test]
    fn original_review_view_capture_and_root_alias_retire_with_exact_receipts() {
        use semio_framework_trace::observe_heap_allocations_on_this_thread;
        use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
        let law:serde_json::Value=serde_json::from_str(include_str!("../../⏪️time-travel/♻️retirement/🧫️fixtures/🔣️.json")).unwrap();let declared=&law["reviewViewGrant"];
        let grant=RetainedCloneGrant{maximum_items:declared["items"].as_u64().unwrap()as usize,maximum_copy_bytes:declared["copy"].as_u64().unwrap()as usize,maximum_capacity_bytes:declared["capacity"].as_u64().unwrap()as usize,maximum_release_bytes:declared["release"].as_u64().unwrap()as usize,maximum_depth:declared["depth"].as_u64().unwrap()as usize};
        let (original,heap)=observe_heap_allocations_on_this_thread(||ChildContentView{root:Some(std::sync::Arc::new(ChildContentRoot{pages:std::array::from_fn(|_|None),len:0}))});let held=heap.requested_bytes;assert_eq!(heap.released_bytes,0);let pointer=std::sync::Arc::as_ptr(original.root.as_ref().unwrap());let base=semio_framework_time_travel::TimeTravelBase{content_revision:[7;32]};
        for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let (captured,heap)=observe_heap_allocations_on_this_thread(||time_travel::TimeTravelReviewChildren::capture(&original,base,3,denied));assert!(captured.is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(std::sync::Arc::as_ptr(original.root.as_ref().unwrap()),pointer);assert_eq!(std::sync::Arc::strong_count(original.root.as_ref().unwrap()),1)}
        let ((review,receipt),heap)=observe_heap_allocations_on_this_thread(||time_travel::TimeTravelReviewChildren::capture(&original,base,3,grant).unwrap());assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(review.view.same_owner(&original));assert_eq!(review.base,base);assert_eq!(review.generation,3);
        let mut children=ChildMemberRegistry::<store::NoMembers>::new();let owners=ChildContentOwners::none();let mut alias=ChildContentRetirement::new(original,false);assert!(!alias.terminal_is_empty());
        for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:0,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{let (step,heap)=observe_heap_allocations_on_this_thread(||alias.close_step(&mut children,None,&review.view,&owners,denied).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(!alias.terminal_is_empty());assert_eq!(step.progress(),RetainedCloneProgress::default())}
        let (step,heap)=observe_heap_allocations_on_this_thread(||alias.close_step(&mut children,None,&review.view,&owners,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(alias.terminal_is_empty());assert_eq!(std::sync::Arc::as_ptr(review.view.root.as_ref().unwrap()),pointer);
        let mut terminal=ChildContentRetirement::new(review.view,false);assert!(!terminal.terminal_is_empty());let (step,heap)=observe_heap_allocations_on_this_thread(||terminal.close_step(&mut children,None,&ChildContentView::EMPTY,&owners,grant).unwrap());assert!(step.progress().fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,held));assert_eq!(step.progress().released_bytes,held);assert!(terminal.terminal_is_empty());let (_,heap)=observe_heap_allocations_on_this_thread(||drop((alias,terminal)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] actual original review root lease original={held} captureBirth0 denied0heap aliasRelease0 finalRelease={held} originalPointer={pointer:p} terminalDrop0");
    }

    fn dialect() -> ArtifactDialect {
        ArtifactDialect { artifact_kind: "test-child".into(), standard: "native".into(), subset: "*".into() }
    }

    fn insert<M>(registry: &mut ChildMemberRegistry<M>, key: (String, String), member: M) {
        let admission = registry.admit(&key).expect("exact child admission");
        let reference = ArtifactRef { artifact_id: key.1.clone(), dialect: dialect() };
        let owner = store::OwnerRef {
            parent: ArtifactRef { artifact_id: "parent".into(), dialect: dialect() },
            slot: key.0,
            child_id: key.1,
        };
        registry.insert_admitted(admission, reference, owner, member);
    }

    fn member_ingress(ordinal: usize, operation: u64, generation: u64, slot: &str, child_id: &str) -> OwnedDocumentMemberIngress {
        let reference = ArtifactRef { artifact_id: child_id.into(), dialect: dialect() };
        let owner = store::OwnerRef {
            parent: ArtifactRef { artifact_id: "parent".into(), dialect: dialect() },
            slot: slot.into(),
            child_id: child_id.into(),
        };
        let mut pages = store::OwnedSchemaDecodePages::try_with_credits(store::OwnedSchemaDecodeCredits { maximum_pages: 1, maximum_bytes: 3 }).expect("one retained member page credit");
        pages.admit_page(store::OwnedSchemaDecodePage::try_from_slice(&[1, 2, 3]).expect("bounded member page")).unwrap_or_else(|_| panic!("pre-admitted member page"));
        pages.seal().expect("complete member page set");
        let request = store::MemberOpenRequest::new(
            semio_framework_job::OperationId(operation),
            semio_framework_job::Generation(generation),
            999,
            reference.clone(),
            Some(owner.clone()),
            pages,
            protocol::ActorId("actor:child-registry-fixture".into()),
        )
        .admit(1)
        .unwrap_or_else(|_| panic!("valid retained member request"));
        OwnedDocumentMemberIngress::try_new(ordinal, reference, owner, request).unwrap_or_else(|_| panic!("exact member ingress"))
    }


    fn original_member_fixture_grant() -> RetainedCloneGrant {
        let law:serde_json::Value=serde_json::from_str(include_str!("../🔬️plugin-runtime-runtime-close-budget/🧫️fixtures/📮️member-ingress/🔣️.json")).unwrap();let values=law["originalGrant"].as_array().unwrap();
        RetainedCloneGrant{maximum_items:values[0].as_u64().unwrap()as usize,maximum_copy_bytes:values[1].as_u64().unwrap()as usize,maximum_capacity_bytes:values[2].as_u64().unwrap()as usize,maximum_release_bytes:values[3].as_u64().unwrap()as usize,maximum_depth:values[4].as_u64().unwrap()as usize}
    }

    /// 🧳️ Preserves the original prepared policy without deriving funding from a demand.
    fn original_prepared_fixture_grant() -> store::ArtifactStoreOneItemGrant {
        let original = original_member_fixture_grant();
        store::ArtifactStoreOneItemGrant { maximum_items: original.maximum_items, maximum_copy_bytes: original.maximum_copy_bytes, maximum_capacity_bytes: original.maximum_capacity_bytes, maximum_release_bytes: original.maximum_release_bytes, maximum_depth: original.maximum_depth }
    }

    fn close_ingress(mut ingress: OwnedDocumentMemberIngress, grant: RetainedCloneGrant) {
        for _ in 0..4096 {
            let step=ingress.close_step(grant).expect("original full ingress close");assert!(step.progress().fits(grant));
            if matches!(step,RetainedCloneStep::Complete(_)){assert!(ingress.terminal_is_empty());drop(ingress);return;}
        }
        panic!("member ingress did not retire within its original bounded owner count");
    }

    #[test]
    fn fixed_child_member_registry_admits_exact_capacity_rejects_plus_one_and_cursor_detaches_every_owner() {
        let mut registry = ChildMemberRegistry::new();
        for index in 0..CHILD_CONTENT_SLOTS {
            let key = (format!("slot-{index}"), format!("child-{index}"));
            insert(&mut registry, key, index);
        }
        let rejected_key = ("slot-plus-one".to_string(), "child-plus-one".to_string());
        assert!(registry.admit(&rejected_key).is_err(), "capacity plus one is rejected before an owner is transferred");
        for index in 0..CHILD_CONTENT_SLOTS {
            assert!(registry.take_at(index).is_some(), "one exact child owner detaches per cursor step");
        }
        assert!(registry.is_empty());
    }

    #[test]
    fn fixed_child_member_registry_resolves_hash_collisions_without_replacement() {
        let mut first_by_hash: [Option<(String, String)>; CHILD_CONTENT_SLOTS] = std::array::from_fn(|_| None);
        let mut collision = None;
        for index in 0..=CHILD_CONTENT_SLOTS {
            let key = ("slot".to_string(), format!("collision-{index}"));
            let hash = ChildMemberRegistry::<usize>::hash(MemberKeyRef::root(&key.0, &key.1)).expect("bounded key hash");
            if let Some(first) = first_by_hash[hash].take() {
                collision = Some((first, key));
                break;
            }
            first_by_hash[hash] = Some(key);
        }
        let (first, second) = collision.expect("pigeonhole collision across capacity plus one keys");
        let mut registry = ChildMemberRegistry::new();
        insert(&mut registry, first.clone(), 1);
        insert(&mut registry, second.clone(), 2);
        assert_eq!(registry.get(&first).map(|entry| entry.member), Some(1));
        assert_eq!(registry.get(&second).map(|entry| entry.member), Some(2));
        for index in 0..CHILD_CONTENT_SLOTS {
            drop(registry.take_at(index));
        }
    }

    #[test]
    fn stale_child_member_admission_cannot_cancel_a_reused_slot_generation() {
        let key = ("slot".to_string(), "child".to_string());
        let mut registry = ChildMemberRegistry::new();
        let stale = registry.admit(&key).expect("first admission");
        assert!(registry.cancel_admission(&stale));
        let current = registry.admit(&key).expect("same direct slot is reused with a fresh generation");
        assert_ne!(stale.generation, current.generation);
        assert!(!registry.cancel_admission(&stale), "stale generation cannot cancel the reused reservation");
        let reference = ArtifactRef { artifact_id: key.1.clone(), dialect: dialect() };
        let owner = store::OwnerRef { parent: ArtifactRef { artifact_id: "parent".into(), dialect: dialect() }, slot: key.0, child_id: key.1 };
        registry.insert_admitted(current, reference, owner, 7);
        for index in 0..CHILD_CONTENT_SLOTS {
            drop(registry.take_at(index));
        }
    }

    #[test]
    fn incomplete_child_member_registry_drop_faults_in_release_instead_of_destroying_nested_owners() {
        let result = std::panic::catch_unwind(|| {
            let key = ("slot".to_string(), "retained".to_string());
            let mut registry = ChildMemberRegistry::new();
            insert(&mut registry, key, vec![0u8; 64 * 1024]);
        });
        assert!(result.is_err(), "ordinary incomplete Drop is observably fail-closed and MaybeUninit keeps the nested owner from implicit destruction");
    }


    #[test]
    fn owned_document_ingress_is_heap_backed_and_retires_duplicate_and_never_opened_requests_incrementally() {
        let grant=original_member_fixture_grant();assert!(size_of::<OwnedDocumentMemberIngressRegistry>()<1024);
        let mut registry=OwnedDocumentMemberIngressRegistry::try_new(2).unwrap();
        registry.admit(member_ingress(0,7,11,"a","child-a")).unwrap_or_else(|_|panic!("original first ordinal"));
        let(_,duplicate)=registry.admit(member_ingress(0,7,11,"a","child-a-duplicate")).expect_err("duplicate retains original");close_ingress(duplicate,grant);
        assert!(registry.seal().is_err());let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||registry.close_step(RetainedCloneGrant{maximum_items:0,..grant}).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        for _ in 0..8192{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||registry.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));if matches!(step,RetainedCloneStep::Complete(_)){assert!(registry.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(registry));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original duplicate/never-opened ingress all full receipts exact; no force drain; terminalDrop0");return;}}
        panic!("original never-opened ingress did not reach terminal custody");
    }

    #[test]
    fn owned_document_ingress_refuses_extra_ordinal_and_preserves_the_returned_request() {
        let ingress = member_ingress(2, 7, 11, "extra", "child-extra");
        let mut registry = OwnedDocumentMemberIngressRegistry::try_new(2).expect("two exact ingress slots");
        let (_, ingress) = registry.admit(ingress).expect_err("extra ordinal is rejected unchanged");
        close_ingress(ingress, original_member_fixture_grant());
        assert!(registry.seal().is_err());
        for _ in 0..32 { registry.close_step(original_member_fixture_grant()).expect("empty registry close"); if registry.terminal_is_empty() { break; } }
        assert!(registry.terminal_is_empty());
        drop(registry);
    }

    #[test]
    fn displaced_composition_pins_retire_under_exact_byte_and_item_grants() {
        let grant = original_member_fixture_grant();
        let mut retirement = CompositionPinsRetirement::new(vec![vcs::CompositionPin {
            child_ref: ArtifactRef { artifact_id: "child-δ".into(), dialect: dialect() },
            checkpoint_id: "checkpoint-α".into(),
        }]);
        assert_eq!(retirement.close_step(RetainedCloneGrant { maximum_items: 0, ..grant }).unwrap(), RetainedCloneStep::Progress(Default::default()));
        assert_eq!(retirement.close_step(RetainedCloneGrant { maximum_release_bytes: 1, ..grant }).unwrap(), RetainedCloneStep::Progress(Default::default()));
        for _ in 0..512 {
            let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| retirement.close_step(grant).unwrap());
            assert!(step.progress().fits(grant)); assert_eq!((heap.requested_bytes, heap.released_bytes), (step.progress().retained_capacity_bytes, step.progress().released_bytes));
            if matches!(step, RetainedCloneStep::Complete(_)) {
                assert!(retirement.terminal_is_empty());
                drop(retirement);
                return;
            }
        }
        panic!("composition pin retirement did not reach exact terminal emptiness");
    }

    #[test]
    fn prepared_child_content_known_root_alias_releases_zero_and_external_reader_still_blocks() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("🧫️fixtures/🔗️retained-alias/🔣️.json")).unwrap();
        assert_eq!(fixture["cases"].as_array().unwrap().len(), 6);
        let retained = ChildContentView { root: Some(std::sync::Arc::new(ChildContentRoot::default())) };
        let mut alias = retained.clone();
        let pointer = std::sync::Arc::as_ptr(alias.root.as_ref().unwrap());
        let full = original_prepared_fixture_grant();
        let zero = store::ArtifactStoreOneItemGrant { maximum_items: 0, ..full };
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| alias.close_prepared_structure_step(&retained, zero).unwrap());
        assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(std::sync::Arc::as_ptr(alias.root.as_ref().unwrap()), pointer);
        let one = full;
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| alias.close_prepared_structure_step(&retained, one).unwrap());
        assert_eq!(step, RetainedCloneStep::Progress(RetainedCloneProgress { copied_items: 1, ..Default::default() }));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert!(alias.root.is_none());
        assert_eq!(std::sync::Arc::strong_count(retained.root.as_ref().unwrap()), 1);
        let mut exclusive = retained;
        let external = exclusive.clone();
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| exclusive.close_prepared_structure_step(&ChildContentView::EMPTY, one).unwrap());
        assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(external)).1.released_bytes, 0);
        let bytes = child_content_arc_bytes::<ChildContentRoot>();
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| exclusive.close_prepared_structure_step(&ChildContentView::EMPTY, store::ArtifactStoreOneItemGrant { maximum_release_bytes: bytes - 1, ..full }).unwrap());
        assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
        let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| exclusive.close_prepared_structure_step(&ChildContentView::EMPTY, full).unwrap());
        assert_eq!(step, RetainedCloneStep::Complete(RetainedCloneProgress { copied_items: 1, released_bytes: bytes, ..Default::default() }));
        assert_eq!((heap.requested_bytes, heap.released_bytes), (0, bytes));
        println!("[DEBUG] prepared child exact retained root alias releases0; unrelated reader blocks until return; original root frame{bytes} denies one-below and frees exactly once");
    }

    #[test]
    fn prepared_child_content_metadata_retains_denied_backing_and_reports_each_physical_release() {
        let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🧵️job/🧪️tests/🧫️fixtures/📏️close-demand/🔣️.json")).unwrap();
        let visibility = vcs::ArtifactGroupVisibilityOwner::new();
        let original_visibility = visibility.view();
        let unrelated_visibility = vcs::ArtifactGroupVisibilityOwner::new().view();
        let original = original_member_fixture_grant();
        let admission = fixture["admissionBytes"].as_u64().unwrap() as usize;
        for row in fixture["cases"].as_array().unwrap() {
            let physical = row["physicalBytes"].as_u64().unwrap() as usize;
            let caller = row["callerBytes"].as_u64().unwrap() as usize;
            let mut text = String::with_capacity(physical);
            text.push('x');
            assert_eq!(text.capacity(), physical);
            let pointer = text.as_ptr();
            let mut owner = PreparedChildContentEntry { entry: std::mem::ManuallyDrop::new(None), metadata: std::mem::ManuallyDrop::new(Some([text, String::new(), String::new(), String::new(), String::new(), String::new(), String::new()])), metadata_cursor: 0, visibility: std::mem::ManuallyDrop::new(Some(original_visibility.clone())), generation: 7 };
            assert_eq!(owner.next_close_byte_demand(), physical);
            let full = store::ArtifactStoreOneItemGrant { maximum_items: original.maximum_items, maximum_copy_bytes: original.maximum_copy_bytes, maximum_capacity_bytes: original.maximum_capacity_bytes, maximum_release_bytes: original.maximum_release_bytes, maximum_depth: original.maximum_depth };
            let demand = owner.retirement_demands();
            assert_eq!(demand.copy_bytes, 0);
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_metadata_step(&unrelated_visibility, full).unwrap());
            assert_eq!(step, RetainedCloneStep::Progress(Default::default())); assert_eq!((events.requested_bytes, events.released_bytes), (0, 0)); assert_eq!(owner.metadata.as_ref().unwrap()[0].as_ptr(), pointer);
            for grant in [store::ArtifactStoreOneItemGrant { maximum_items: 0, ..full }, store::ArtifactStoreOneItemGrant { maximum_release_bytes: physical - 1, ..full }, store::ArtifactStoreOneItemGrant { maximum_depth: 0, ..full }] {
                let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_metadata_step(&original_visibility, grant).unwrap());
                assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
                assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
                assert_eq!(owner.metadata.as_ref().unwrap()[0].as_ptr(), pointer);
                assert_eq!(owner.next_close_byte_demand(), physical);
            }
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_metadata_step(&original_visibility, store::ArtifactStoreOneItemGrant { maximum_release_bytes: caller, ..full }).unwrap());
            let expected = row["releasedBytes"].as_u64().unwrap() as usize;
            assert_eq!(step.progress(), RetainedCloneProgress { copied_items: usize::from(expected != 0), copied_bytes: if expected != 0 { demand.copy_bytes } else { 0 }, released_bytes: expected, ..Default::default() });
            assert_eq!((events.requested_bytes, events.released_bytes), (0, expected));
            let mut paid = expected;
            for _ in 0..10 {
                if owner.terminal_is_empty() { break; }
                let demand = owner.retirement_demands();
                assert!(demand.release_bytes <= admission);
                assert_eq!(demand.copy_bytes, 0, "metadata movement preserves the defining semantic copy contract");
                let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| owner.close_metadata_step(&original_visibility, full).unwrap());
                assert_eq!(events.requested_bytes, 0);
                let progress = step.progress(); assert!(progress.fits(original));
                assert_eq!(progress.copied_items, 1); assert_eq!(progress.copied_bytes, demand.copy_bytes);
                assert_eq!(progress.released_bytes, demand.release_bytes); assert_eq!(events.released_bytes, progress.released_bytes); paid += progress.released_bytes;
            }
            assert!(owner.terminal_is_empty());
            assert_eq!(paid, physical);
            let (_, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop(owner));
            assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
            println!("[DEBUG] prepared child metadata physical={physical} denied-grant={} actual-paid-release={paid}", physical - 1);
        }
    }

    #[test]
    fn prepared_child_content_empty_scaffolds_release_whole_arc_frames_in_distinct_turns() {
        let full = original_prepared_fixture_grant();
        let ((mut view, page_bytes, root_bytes), birth) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| {
            let page = std::sync::Arc::new(ChildContentPage::default());
            let mut root = ChildContentRoot::default();
            root.pages[0] = Some(page);
            (ChildContentView { root: Some(std::sync::Arc::new(root)) }, child_content_arc_bytes::<ChildContentPage>(), child_content_arc_bytes::<ChildContentRoot>())
        });
        assert_eq!(birth.requested_bytes, page_bytes + root_bytes);
        assert_eq!(birth.released_bytes, 0);
        for (turn, physical) in [page_bytes, root_bytes].into_iter().enumerate() {
            assert_eq!(view.prepared_structure_close_byte_demand(), physical);
            let root_pointer = std::sync::Arc::as_ptr(view.root.as_ref().unwrap());
            for grant in [store::ArtifactStoreOneItemGrant { maximum_items: 0, ..full }, store::ArtifactStoreOneItemGrant { maximum_release_bytes: physical - 1, ..full }, store::ArtifactStoreOneItemGrant { maximum_depth: view.prepared_structure_close_depth_demand() - 1, ..full }] {
                let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| view.close_prepared_structure_step(&ChildContentView::EMPTY, grant).unwrap());
                assert_eq!(step, RetainedCloneStep::Progress(Default::default()));
                assert_eq!((events.requested_bytes, events.released_bytes), (0, 0));
                assert_eq!(std::sync::Arc::as_ptr(view.root.as_ref().unwrap()), root_pointer);
                assert_eq!(view.prepared_structure_close_byte_demand(), physical);
            }
            let (step, events) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| view.close_prepared_structure_step(&ChildContentView::EMPTY, full).unwrap());
            assert_eq!(step.progress(), RetainedCloneProgress { copied_items: 1, released_bytes: physical, ..Default::default() });
            assert_eq!((events.requested_bytes, events.released_bytes), (0, physical));
        }
        assert_eq!(view.prepared_structure_close_byte_demand(), 0);
        assert_eq!(view.close_prepared_structure_step(&ChildContentView::EMPTY, store::ArtifactStoreOneItemGrant { maximum_items: 0, ..full }).unwrap(), RetainedCloneStep::Complete(Default::default()));
        println!("[DEBUG] prepared child scaffold exact Arc page={page_bytes} root={root_bytes} allocator release matches each whole grant");
    }

    #[test]
    fn owned_document_member_ingress_identity_reports_actual_whole_physical_release() {
        let grant=original_member_fixture_grant();let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🏪️store/🧩️composition/🚪️open/🏭️operation/📏️birth/🧫️fixtures/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap(){
            let mut ingress=member_ingress(0,18,19,row["dialect"]["subset"].as_str().unwrap(),row["artifactId"].as_str().unwrap());let mut request=ingress.take_request().unwrap();
            for _ in 0..4096{let step=request.close_step(grant).unwrap();assert!(step.progress().fits(grant));if matches!(step,RetainedCloneStep::Complete(_)){break;}}assert!(request.terminal_is_empty());drop(request);
            let mut paid=0;let mut actual=0;for _ in 0..4096{let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ingress.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));paid+=receipt.released_bytes;actual+=heap.released_bytes;if ingress.terminal_is_empty(){break;}}
            assert!(ingress.terminal_is_empty());assert_eq!(paid,actual);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(ingress));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original ingress identity actualPhysical={actual} paid={paid} fullCallerGrant=true terminalDrop0");
        }
    }

    #[test]
    fn owned_document_member_ingress_identity_preserves_denied_original_capacity_even_when_empty() {
        let grant=original_member_fixture_grant();let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../🏪️store/🧩️composition/🚪️open/📏️retirement/🧫️fixtures/🔣️.json")).unwrap();
        for row in fixture["cases"].as_array().unwrap(){
            let mut ingress=member_ingress(0,18,19,"slot","child");let mut request=ingress.take_request().unwrap();for _ in 0..4096{let step=request.close_step(grant).unwrap();assert!(step.progress().fits(grant));if matches!(step,RetainedCloneStep::Complete(_)){break;}}assert!(request.terminal_is_empty());drop(request);
            let bytes=row["capacityBytes"].as_u64().unwrap()as usize;let mut original=String::with_capacity(bytes);original.push_str(row["text"].as_str().unwrap());drop(std::mem::replace(ingress.identity_string_mut().unwrap(),original));let pointer=ingress.identity_string_mut().unwrap().as_ptr();let demand=ingress.retirement_demands(grant.maximum_copy_bytes).unwrap();assert_eq!(demand.release_bytes,bytes);
            for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant},RetainedCloneGrant{maximum_release_bytes:0,..grant},RetainedCloneGrant{maximum_release_bytes:bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
                let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ingress.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(ingress.identity_string_mut().unwrap().as_ptr(),pointer);assert_eq!(ingress.retirement_demands(grant.maximum_copy_bytes).unwrap(),demand);
            }
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||ingress.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(receipt.released_bytes,bytes);assert_eq!(receipt.copied_bytes,demand.copy_bytes);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,bytes));close_ingress(ingress,grant);eprintln!("[DEBUG] original ingress field capacity={bytes} deniedAxesPreservePointer=true exactPhysicalReleaseOnce=true");
        }
    }

    #[test]
    fn owned_document_ingress_registry_backing_remains_owned_until_exact_release() {
        let grant=original_member_fixture_grant();let(mut registry,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||OwnedDocumentMemberIngressRegistry::try_new(17).unwrap());let retained=birth.requested_bytes-birth.released_bytes;let mut released=0;let mut turns=0;
        while !registry.terminal_is_empty(){let demand=registry.retirement_demands(grant.maximum_copy_bytes).unwrap();let pointer=registry.pages.as_ptr();
            for denied in [Some(RetainedCloneGrant{maximum_items:0,..grant}),Some(RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant}),Some(RetainedCloneGrant{maximum_depth:0,..grant}),(demand.release_bytes>0).then_some(RetainedCloneGrant{maximum_release_bytes:demand.release_bytes-1,..grant})].into_iter().flatten(){
                let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||registry.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(registry.pages.as_ptr(),pointer);assert_eq!(registry.retirement_demands(grant.maximum_copy_bytes).unwrap(),demand);
            }
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||registry.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(receipt.copied_bytes,demand.copy_bytes);assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,receipt.released_bytes));released+=receipt.released_bytes;turns+=1;assert!(turns<=5);
        }
        assert_eq!(retained,released);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(registry));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original empty ingress17 turns={turns} physical={released} paidSealAndBacking=true allDeniedHeap0 terminalDrop0");
    }

    #[test]
    fn owned_document_ingress_paging_matches_neutral_frames_and_exact_physical_retirement() {
        let grant=original_member_fixture_grant();let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/📦️ingress-paging/🔣️.json")).unwrap();assert_eq!(OWNED_DOCUMENT_INGRESS_PAGE_SLOTS,fixture["pageSlots"].as_u64().unwrap()as usize);assert!(size_of::<OwnedDocumentMemberIngressRegistry>()<1024);
        for row in fixture["cases"].as_array().unwrap(){
            let entries=row["entries"].as_u64().unwrap()as usize;let(mut registry,birth)=semio_framework_trace::observe_heap_allocations_on_this_thread(||OwnedDocumentMemberIngressRegistry::try_new(entries).unwrap());let frames:Vec<_>=registry.pages.iter().map(|page|page.as_ref().unwrap().len()).collect();let expected:Vec<_>=row["frames"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize).collect();assert_eq!(frames,expected);assert_eq!(registry.pages.capacity(),row["pointerSlots"].as_u64().unwrap()as usize);
            let frame_bytes:Vec<_>=registry.pages.iter().map(|page|size_of_val(page.as_ref().unwrap().as_ref())).collect();assert!(frame_bytes.iter().all(|bytes|*bytes<=fixture["maximumAdmissionBytes"].as_u64().unwrap()as usize));let pointer_bytes=registry.pages.capacity()*size_of::<Option<Box<[std::mem::MaybeUninit<OwnedDocumentMemberIngress>]>>>();let expected_birth=frame_bytes.iter().sum::<usize>()+pointer_bytes;assert_eq!((birth.requested_bytes,birth.released_bytes),(expected_birth,0));
            let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||registry.close_step(grant).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(step.progress(),RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<bool>(),..Default::default()});
            let mut released=0;for bytes in frame_bytes.into_iter().chain((pointer_bytes!=0).then_some(pointer_bytes)){
                let pointer=registry.pages.as_ptr();let page_pointer=registry.pages.iter().find_map(|page|page.as_ref().map(|page|page.as_ptr()));let demand=registry.retirement_demands(grant.maximum_copy_bytes).unwrap();assert_eq!(demand.release_bytes,bytes);
                for denied in [RetainedCloneGrant{maximum_items:0,..grant},RetainedCloneGrant{maximum_copy_bytes:demand.copy_bytes-1,..grant},RetainedCloneGrant{maximum_release_bytes:bytes-1,..grant},RetainedCloneGrant{maximum_depth:0,..grant}]{
                    let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||registry.close_step(denied).unwrap());assert_eq!(step.progress(),Default::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(registry.pages.as_ptr(),pointer);assert_eq!(registry.pages.iter().find_map(|page|page.as_ref().map(|page|page.as_ptr())),page_pointer);assert_eq!(registry.retirement_demands(grant.maximum_copy_bytes).unwrap(),demand);
                }
                let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||registry.close_step(grant).unwrap());let receipt=step.progress();assert!(receipt.fits(grant));assert_eq!(receipt.copied_bytes,demand.copy_bytes);assert_eq!((heap.requested_bytes,heap.released_bytes),(receipt.retained_capacity_bytes,bytes));released+=bytes;
            }
            assert!(registry.terminal_is_empty());assert_eq!(registry.close_step(grant).unwrap(),RetainedCloneStep::Complete(Default::default()));assert_eq!(released,expected_birth);let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(registry));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));eprintln!("[DEBUG] original ingress entries={entries} birth={expected_birth} release={released} samePolicyAllTurns=true paidSeal=true exactPointersRetained=true terminalDrop0");
        }
    }

}
