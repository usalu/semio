use super::*;
use semio_framework_actor::typed_operation_page::{TypedOperationResultLane,TYPED_OPERATION_RESULT_PAGE_BYTES};
use semio_framework_job::{Generation,OperationId,OriginalJobFixtureCaller,OriginalRetirementPump,logical_now_us};
use semio_framework_trace::observe_heap_allocations_on_this_thread as observe;

fn fixture()->serde_json::Value{serde_json::from_str(include_str!("../🧪️.json")).unwrap()}
fn original()->RetainedCloneGrant{
    let fixture=fixture();
    let g=&fixture["receiverGrant"];
    RetainedCloneGrant{
        maximum_items:g["maximumItems"].as_u64().unwrap() as usize,
        maximum_copy_bytes:g["maximumCopyBytes"].as_u64().unwrap() as usize,
        maximum_capacity_bytes:g["maximumCapacityBytes"].as_u64().unwrap() as usize,
        maximum_release_bytes:g["maximumReleaseBytes"].as_u64().unwrap() as usize,
        maximum_depth:g["maximumDepth"].as_u64().unwrap() as usize,
    }
}
fn receipt(p:RetainedCloneProgress,heap:semio_framework_trace::HeapAllocationObservation,g:RetainedCloneGrant){
    assert!(p.fits(g));
    assert_eq!((heap.requested_bytes,heap.released_bytes),(p.retained_capacity_bytes,p.released_bytes));
}
fn effect<R>(grant:RetainedCloneGrant,f:impl FnOnce(&mut semio_framework_value::RetainedAllocationEffect<'_>)->R)->R{
    OriginalRetirementPump::new()
        .turn(1<<16,u64::MAX,logical_now_us,grant)
        .and_then(|mut turn|turn.with_effect(f))
        .unwrap().unwrap().0
}
fn caller(g:RetainedCloneGrant)->OriginalJobFixtureCaller{
    let stock=RetainedCloneGrant{
        maximum_items:1000*g.maximum_items,
        maximum_copy_bytes:1000*g.maximum_copy_bytes,
        maximum_capacity_bytes:1000*g.maximum_capacity_bytes,
        maximum_release_bytes:1000*g.maximum_release_bytes,
        ..g
    };
    let mut caller=OriginalJobFixtureCaller::admit(OperationId(7),Generation(1),stock,logical_now_us,u64::MAX,1<<16);
    caller.retain_dispatch().unwrap();
    caller
}
fn producer_grant(demand:RetirementDemand)->RetainedCloneGrant{
    RetainedCloneGrant{
        maximum_items:1,
        maximum_copy_bytes:demand.copy_bytes,
        maximum_capacity_bytes:demand.capacity_bytes,
        maximum_release_bytes:demand.release_bytes,
        maximum_depth:demand.depth.max(1),
    }
}
fn prepared(length:usize)->(NativeResultPages,Vec<TypedOperationResultPage>){
    let mut pages=NativeResultPages::new();
    let mut producers=Vec::new();
    for (index,slot) in pages.slots.iter_mut().enumerate(){
        let token=TypedOperationResultToken{receiver:7,operation:u64::MAX-index as u64,generation:u64::MAX,sequence:0,attempt:1};
        let mut producer=TypedOperationResultPage::from_original(token,TypedOperationResultLane::Diagnostic,vec![81;TYPED_OPERATION_RESULT_PAGE_BYTES],length);
        for _ in 0..65536{
            let grant=producer_grant(producer.presentation_demand().unwrap());
            let ((page,progress),heap)=observe(||effect(grant,|effect|producer.present(effect)).unwrap());
            receipt(progress,heap,grant);
            if page.is_some(){slot.page=page;break}
            assert_ne!(progress,RetainedCloneProgress::default(),"original producer grant {grant:?}");
        }
        assert_eq!(slot.page.as_ref().expect("original producer presentation exhausted").bytes().as_ptr(),producer.bytes().as_ptr());
        producers.push(producer);
    }
    assert_eq!(pages.slots.iter().filter(|slot|!slot.empty()).count(),fixture()["maximumPages"].as_u64().unwrap() as usize);
    (pages,producers)
}
fn close_producers(producers:&mut[TypedOperationResultPage]){
    let body=original().maximum_copy_bytes;
    for producer in producers{
        for _ in 0..65536{
            if producer.terminal_is_empty(){break}
            let grant=producer_grant(producer.next_close_demand(body).unwrap());
            let (step,heap)=observe(||effect(grant,|effect|producer.close_step(effect)).unwrap());
            receipt(step.progress(),heap,grant);
        }
        assert!(producer.terminal_is_empty());
    }
}
fn close_pages(pages:&mut NativeResultPages,g:RetainedCloneGrant){
    let mut caller=caller(g);
    caller.cancel().cancel_now();
    caller.turn(g,|cx|{
        let denied=RetainedCloneGrant{maximum_items:0,..g};
        let step=cx.with_retained_loan(denied,|cx|pages.close_original(cx))?;
        assert_eq!(step.progress(),RetainedCloneProgress::default());
        Ok(())
    }).unwrap();
    assert!(!pages.terminal_is_empty());
    for _ in 0..1000{
        if pages.terminal_is_empty(){break}
        let (step,heap)=caller.turn(g,|cx|{
            let (step,heap)=observe(||pages.close_original(cx));
            Ok((step?,heap))
        }).unwrap();
        receipt(step.progress(),heap,g);
        assert_ne!(step.progress(),RetainedCloneProgress::default());
    }
    assert!(pages.terminal_is_empty(),"original Reactor page cancellation stalled");
    caller.retire(g);
}
fn retire_effects(effects:Vec<Effect>,g:RetainedCloneGrant){
    let mut owner=ControlledRetirement::new(effects).unwrap_or_else(|_|panic!("typed effects declare controlled retirement"));
    for _ in 0..1000{
        if owner.terminal_is_empty(){return}
        let (step,heap)=observe(||effect(g,|effect|semio_framework_actor::typed_operation_page::poll_step(owner.step(g.maximum_copy_bytes,effect))).unwrap());
        receipt(step.progress(),heap,g);
    }
    panic!("original renderer destination retirement stalled");
}

/// 🎫️ Eight original loans use the same recipient for bounded receiving and effect publication in each issued turn.
#[test]
fn native_page_batch_preserves_eight_original_loans_through_funded_receiving(){
    let g=original();
    for row in fixture()["pageLengths"].as_array().unwrap(){
        let length=row.as_u64().unwrap() as usize;
        let (mut pages,mut producers)=prepared(length);
        let mut effects=Vec::new();
        let mut caller=caller(g);
        for _ in 0..1000{
            if pages.terminal_is_empty(){break}
            caller.turn(g,|cx|{
                for slot in &mut pages.slots{
                    for _ in 0..100{
                        if slot.empty(){break}
                        let grant=cx.retained_grant();
                        let (result,heap)=observe(||{
                            if slot.ready(){
                                let (published,p)=publish(slot,7,&mut effects,64,cx)?;
                                if published{effects.push(slot.publication.take().unwrap())}
                                Ok(p)
                            }else{
                                let Some((result,work))=cx.with_original_source_effect(grant,|_,effect|slot.advance(effect))?else{return Ok(Default::default())};
                                cx.consume_work(work);
                                result
                            }
                        });
                        let progress=result?;
                        receipt(progress,heap,grant);
                        cx.consume_retained(progress)?;
                        if progress==RetainedCloneProgress::default(){break}
                    }
                }
                Ok(())
            }).unwrap();
        }
        assert!(pages.terminal_is_empty());
        assert_eq!(effects.len(),8);
        for (index,effect) in effects.iter().enumerate(){
            let Effect::SendMessage{target:MessageEndpoint::Shell{instance},payload}=effect else{panic!("actual receiving must produce typed shell delivery")};
            assert_eq!(instance.0,"7");
            let header=TypedOperationResultPage::RENDERER_PAGE_MAGIC.len();
            assert_eq!(&payload[..header],TypedOperationResultPage::RENDERER_PAGE_MAGIC);
            assert_eq!(u64::from_le_bytes(payload[header+4..header+12].try_into().unwrap()),u64::MAX-index as u64);
            assert_eq!(u64::from_le_bytes(payload[header+12..header+20].try_into().unwrap()),u64::MAX);
            assert_eq!(payload.len(),header+30+length);
            assert!(payload[header+30..].iter().all(|byte|*byte==81));
        }
        close_producers(&mut producers);
        retire_effects(effects,g);
        caller.retire(g);
        let (_,heap)=observe(||drop(pages));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
    }
}

/// 🛑️ Every portable prefix keeps all original producer addresses and closes every partial destination before producer retirement.
#[test]
fn native_page_batch_cancellation_preserves_original_source_and_partial_destination_custody(){
    let g=original();
    for row in fixture()["pageLengths"].as_array().unwrap(){
        let length=row.as_u64().unwrap() as usize;
        for cut in fixture()["cancellationPrefixes"].as_array().unwrap(){
            let cut=cut.as_u64().unwrap() as usize;
            let (mut pages,mut producers)=prepared(length);
            let pointers:Vec<_>=producers.iter().map(|page|page.bytes().as_ptr()).collect();
            for _ in 0..cut{
                for slot in &mut pages.slots{
                    if !slot.ready(){
                        let (progress,heap)=observe(||effect(g,|effect|slot.advance(effect)).unwrap());
                        receipt(progress,heap,g);
                    }
                }
            }
            for (producer,pointer) in producers.iter().zip(&pointers){assert_eq!(producer.bytes().as_ptr(),*pointer);}
            close_pages(&mut pages,g);
            close_producers(&mut producers);
            let (_,heap)=observe(||drop(pages));
            assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
        }
    }
}

/// 🧳️ A born destination remains in its original receiving slot until the caller accepts and exposes it.
#[test]
fn native_page_publication_cancel_after_birth_keeps_destination_before_ack(){
    let g=original();
    let (mut pages,mut producers)=prepared(4096);
    for slot in &mut pages.slots{
        for _ in 0..1000{
            if slot.ready(){break}
            let (progress,heap)=observe(||effect(g,|effect|slot.advance(effect)).unwrap());
            receipt(progress,heap,g);
        }
        assert!(slot.ready());
    }
    let mut effects=Vec::new();
    let mut caller=caller(g);
    caller.turn(g,|cx|{
        let (result,heap)=observe(||publish(&mut pages.slots[0],7,&mut effects,64,cx));
        let (born,progress)=result?;
        receipt(progress,heap,g);
        assert!(born);
        cx.consume_retained(progress)?;
        Ok(())
    }).unwrap();
    assert!(effects.is_empty());
    assert!(pages.slots[0].receiver.is_none());
    let Effect::SendMessage{payload,..}=pages.slots[0].publication.as_ref().unwrap()else{panic!("original destination missing")};
    assert_eq!(payload.len(),TypedOperationResultPage::RENDERER_PAGE_MAGIC.len()+30+4096);
    close_pages(&mut pages,g);
    close_producers(&mut producers);
    retire_effects(effects,g);
    caller.retire(g);
    let (_,heap)=observe(||drop(pages));
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));
}
