use super::operation_bytes::{OperationByteCloseStep, OwnedOperationBytes};

#[test]
fn owned_operation_byte_pages_preserve_8194_words_and_return_each_actual_backing_under_4096() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let source = &fixture["source"];
    let expected: Vec<u8> = source["prefixUtf8"].as_str().unwrap().bytes().chain(std::iter::repeat_n(source["paddingByte"].as_u64().unwrap() as u8, source["paddingCount"].as_u64().unwrap() as usize)).collect();
    let maximum = fixture["maximumBytes"].as_u64().unwrap() as usize;
    let (constructed, requested, released) = crate::test_allocation::observe_backing(|| OwnedOperationBytes::try_new(fixture["maximumPayloadBytes"].as_u64().unwrap() as usize, fixture["maximumAllocationBytes"].as_u64().unwrap() as usize));
    let mut owner = constructed.expect("real bounded byte owner constructor");
    assert_eq!((requested, released), (0, 0));
    let (zero, requested, released) = crate::test_allocation::observe_backing(|| owner.reserve_one(0));
    assert!(!zero.unwrap().progressed);
    assert_eq!((requested, released), (0, 0));
    for &byte in &expected {
        while !owner.has_reserved_slot() {
            let required = owner.next_allocation_bytes().unwrap();
            assert!(required <= maximum);
            let (narrow, requested, released) = crate::test_allocation::observe_backing(|| owner.reserve_one(required - 1));
            assert!(!narrow.unwrap().progressed);
            assert_eq!((requested, released), (0, 0));
            let (progress, requested, released) = crate::test_allocation::observe_backing(|| owner.reserve_one(maximum));
            let progress = progress.unwrap();
            assert!(progress.progressed);
            assert_eq!(progress.allocated_bytes, requested);
            assert_eq!(released, 0);
            assert!(requested <= maximum);
        }
        let (appended, requested, released) = crate::test_allocation::observe_backing(|| owner.push_reserved(byte));
        appended.expect("one source-owned reserved byte");
        assert_eq!((requested, released), (0, 0));
    }
    assert_eq!(owner.len(), fixture["expected"]["payloadBytes"].as_u64().unwrap() as usize);
    assert!(owner.iter().eq(expected.iter().copied()));
    assert_eq!(serde_json::to_value(&owner).unwrap(), serde_json::to_value(&expected).unwrap());
    let retained = owner.allocated_bytes();
    let rejected = fixture["expected"]["overflowByteReturned"].as_u64().unwrap() as u8;
    let (refusal, requested, released) = crate::test_allocation::observe_backing(|| owner.push_reserved(rejected));
    assert_eq!(refusal.unwrap_err().byte, rejected);
    assert_eq!((requested, released), (0, 0));
    assert!(owner.iter().eq(expected.iter().copied()));
    for (items, bytes) in [(0, maximum), (1, 0)] {
        let (step, requested, released) = crate::test_allocation::observe_backing(|| owner.close_one(items, bytes));
        assert_eq!(step.unwrap(), OperationByteCloseStep::Pending { released_items: 0, released_bytes: 0 });
        assert_eq!((requested, released), (0, 0));
        assert_eq!(owner.allocated_bytes(), retained);
        assert!(owner.iter().eq(expected.iter().copied()));
    }
    let mut total = 0;
    for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap() {
        let (step, requested, released) = crate::test_allocation::observe_backing(|| owner.close_one(1, maximum));
        assert_eq!(requested, 0);
        assert!(released <= maximum);
        match step.unwrap() {
            OperationByteCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert_eq!(released_bytes, released);
                total += released;
            }
            OperationByteCloseStep::Complete => { assert_eq!(released, 0); break; }
        }
    }
    assert!(owner.terminal_is_empty());
    assert_eq!(owner.allocated_bytes(), fixture["expected"]["terminalAllocatedBytes"].as_u64().unwrap() as usize);
    assert_eq!(total, retained);
    assert!(total >= expected.len());
    println!("[DEBUG] All 8194 original operation octets retain exact source identity; each measured physical allocation and return obeys the unchanged 4096 grant, with zero grants preserving the same owner");
}

#[test]
fn owned_operation_byte_pages_borrowed_reader_keeps_the_same_8194_source_without_flattening() {
    use crate::codec::{ByteReader, ByteSpan};
    use super::operation_bytes::OperationByteOutput;
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let expected: Vec<u8> = fixture["source"]["prefixUtf8"].as_str().unwrap().bytes().chain(std::iter::repeat_n(32, 8192)).collect();
    let mut owner = OwnedOperationBytes::try_new(8194, 65536).unwrap();
    let mut allow = |_| true;
    let mut control = crate::value::NativeEncodeControl::new(65536, &mut allow);
    owner.write_bytes(&expected, &mut control).unwrap();
    let ((), allocated, returned) = crate::test_allocation::observe_backing(|| {
        let span = ByteSpan::from_source(&owner);
        assert_eq!(span.len(), expected.len());
        assert!(span.iter().eq(expected.iter().copied()));
        assert!(std::ptr::eq(span.get(4096).unwrap(), owner.byte_ref(4096).unwrap()));
        let mut reader = ByteReader::from_source(&owner);
        assert_eq!(reader.read_u16_le().unwrap(), 0x3731);
        reader.read_span(4092).unwrap();
        assert_eq!(reader.read_u64_le().unwrap(), 0x2020202020202020);
        assert_eq!(reader.position(), 4102);
        let mut fork = reader.fork();
        assert_eq!(fork.read_f64_le().unwrap().to_bits(), 0x2020202020202020);
        assert_eq!(reader.position(), 4102);
        let mut refused = ByteReader::from_source(&owner);
        let error = refused.read_bytes(8194).unwrap_err();
        assert_eq!(error.kind(), crate::value::ValueRefusalKind::UnsupportedOwner);
        assert!(matches!(error, crate::PackRefusal::RetainedMalformed { offset: 0, .. }));
        assert_eq!(refused.position(), 0);
        assert!(refused.read_span(8195).is_err());
        assert_eq!(refused.position(), 0);
        let tail = span.slice(8190, 4).unwrap();
        assert!(tail.iter().eq([32; 4]));
    });
    assert_eq!((allocated, returned), (0, 0));
    for _ in 0..8322 {
        if owner.close_one(1, 4096).unwrap() == OperationByteCloseStep::Complete { break; }
    }
    assert!(owner.terminal_is_empty());
    println!("[DEBUG] Paged operation reader borrows the same full8194 source, crosses physical pages with fixed stack words, and never allocates a flattened operation");
}

#[test]
fn owned_operation_byte_pages_canonical_comparison_retains_exact_source_and_first_difference_without_allocation(){
    use super::operation_bytes::{OperationByteComparison,OperationByteOutput};
    use crate::codec::ByteSpan;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let expected:Vec<u8>=fixture["source"]["prefixUtf8"].as_str().unwrap().bytes().chain(std::iter::repeat_n(32,8192)).collect();
    let mut source=OwnedOperationBytes::try_new(expected.len(),65536).unwrap();
    let mut allow=|_|true;
    let mut control=crate::value::NativeEncodeControl::new(65536,&mut allow);
    source.write_bytes(&expected,&mut control).unwrap();
    let retained=source.allocated_bytes();
    for span in [ByteSpan::from_slice(&expected),ByteSpan::from_source(&source)]{
        let mut allow=|_|true;
        let mut control=crate::value::NativeEncodeControl::new(65536,&mut allow);
        let(result,requested,released)=crate::test_allocation::observe_backing(||{
            let mut comparison=OperationByteComparison::new(span);
            comparison.write_bytes(&expected[..4095],&mut control)?;
            comparison.write_bytes(&expected[4095..],&mut control)?;
            comparison.finish()
        });
        result.unwrap();
        assert_eq!((requested,released),(0,0));
        let mut allow=|_|true;
        let mut control=crate::value::NativeEncodeControl::new(65536,&mut allow);
        let mut comparison=OperationByteComparison::new(span);
        comparison.write_bytes(&expected[..4095],&mut control).unwrap();
        let(error,requested,released)=crate::test_allocation::observe_backing(||comparison.write_bytes(&[255],&mut control));
        let crate::PackRefusal::RetainedMalformed{kind,offset,..}=error.unwrap_err()else{panic!("exact typed mismatch lost")};
        assert_eq!(kind,crate::value::ValueRefusalKind::InvalidValue);
        assert_eq!(offset,4095);
        assert_eq!(comparison.position(),4095);
        assert_eq!((requested,released),(0,0));
        assert!(comparison.write_bytes(&expected[4095..],&mut control).is_err());
        assert!(comparison.finish().is_err());
        let mut comparison=OperationByteComparison::new(span);
        comparison.write_bytes(&expected[..expected.len()-1],&mut control).unwrap();
        assert!(comparison.finish().is_err());
        let mut comparison=OperationByteComparison::new(span);
        comparison.write_bytes(&expected,&mut control).unwrap();
        assert!(comparison.write_bytes(&[0],&mut control).is_err());
        assert!(comparison.finish().is_err());
        let mut canceled=|progress:crate::value::native_encoding::NativeEncodeProgress|progress.completed<256;
        let mut control=crate::value::NativeEncodeControl::new(65536,&mut canceled);
        let mut comparison=OperationByteComparison::new(span);
        let refusal=comparison.write_bytes(&expected,&mut control).unwrap_err();
        assert_eq!(refusal.kind(),crate::value::ValueRefusalKind::Canceled);
        assert_eq!(comparison.position(),256);
    }
    assert_eq!(source.allocated_bytes(),retained);
    assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
    for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){if source.close_one(1,4096).unwrap()==OperationByteCloseStep::Complete{break;}}
    assert!(source.terminal_is_empty());
    println!("[DEBUG] Exact whole 8194 source comparison has zero backing allocation, keeps first mismatch and cancellation position, and retains source until fixed 4096 terminal retirement");
}

#[test]
fn owned_operation_byte_pages_complete_operation_limit_counts_header_and_returns_refused_prefix(){
    use super::operation_bytes::{OperationByteLimitedOutput,OperationByteOutput};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let expected:Vec<u8>=fixture["source"]["prefixUtf8"].as_str().unwrap().bytes().chain(std::iter::repeat_n(32,8192)).collect();
    let close=|source:&mut OwnedOperationBytes|{for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){if source.close_one(1,4096).unwrap()==OperationByteCloseStep::Complete{break;}}assert!(source.terminal_is_empty());assert_eq!(source.allocated_bytes(),0);};
    for maximum in [expected.len(),expected.len()-1]{
        let mut source=OwnedOperationBytes::try_new(expected.len(),65536).unwrap();
        let mut allow=|_|true;
        let mut control=crate::value::NativeEncodeControl::new(65536,&mut allow);
        let completed={
            let mut limited=OperationByteLimitedOutput::new(&mut source,maximum as u64);
            limited.write_bytes(&expected[..2],&mut control).unwrap();
            let result=limited.write_bytes(&expected[2..],&mut control);
            if maximum==expected.len(){result.unwrap();assert_eq!(limited.completed_bytes(),expected.len()as u64);}else{
                let crate::PackRefusal::RetainedMalformed{kind,offset,..}=result.unwrap_err()else{panic!("whole operation typed ceiling lost")};
                assert_eq!(kind,crate::value::ValueRefusalKind::OwnershipLimit);
                assert_eq!(offset,2);
                assert!(limited.write_bytes(&[],&mut control).is_err());
                assert_eq!(limited.completed_bytes(),2);
            }
            limited.completed_bytes()as usize
        };
        assert!(source.iter().eq(expected[..completed].iter().copied()));
        assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected[..completed]).unwrap());
        close(&mut source);
    }
    let mut source=OwnedOperationBytes::try_new(expected.len(),65536).unwrap();
    let mut allow=|_|true;
    let mut control=crate::value::NativeEncodeControl::new(65536,&mut allow);
    {
        let mut limited=OperationByteLimitedOutput::new(&mut source,expected.len()as u64);
        limited.write_bytes(&expected[..2],&mut control).unwrap();
        let mut cancel=|progress:crate::value::native_encoding::NativeEncodeProgress|progress.completed<256;
        let mut control=crate::value::NativeEncodeControl::new(65536,&mut cancel);
        assert_eq!(limited.write_bytes(&expected[2..],&mut control).unwrap_err().kind(),crate::value::ValueRefusalKind::Canceled);
        assert_eq!(limited.completed_bytes(),2);
        assert!(limited.write_bytes(&[],&mut control).is_err());
    }
    assert_eq!(source.len(),258);
    assert!(source.iter().eq(expected[..258].iter().copied()));
    close(&mut source);
    println!("[DEBUG] Complete 8194 operation limit includes its header, one-byte-short refusal preserves the accepted whole fragment prefix, and inner cancellation retains its exact actual source for fixed 4096 terminal release");
}

#[test]
fn owned_operation_byte_pages_fund_complete_backing_across_hops_before_actual_emission(){
    use super::operation_bytes::{OperationBytePreparation,OperationByteFundingStep,OperationByteOutput};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let expected:Vec<u8>=fixture["source"]["prefixUtf8"].as_str().unwrap().bytes().chain(std::iter::repeat_n(32,8192)).collect();
    let maximum=fixture["maximumBytes"].as_u64().unwrap()as usize;
    for canceled in [false,true]{
        let mut prepared=OperationBytePreparation::try_new(expected.len(),65536).unwrap();
        let mut allow=|_|true;let control=crate::value::NativeEncodeControl::new(65536,&mut allow);
        let mut receipt=control.pause().unwrap();
        for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){
            let mut allow=|_|true;let mut control=crate::value::NativeEncodeControl::resume(receipt,&mut allow).unwrap();
            let (zero,requested,released)=crate::test_allocation::observe_backing(||prepared.fund_one(1,0,&mut control));
            assert_eq!(zero.unwrap(),OperationByteFundingStep::Pending{prepared_items:0,allocated_bytes:0});assert_eq!((requested,released),(0,0));
            let required=prepared.next_funding_byte_demand().unwrap();
            let (narrow,requested,released)=crate::test_allocation::observe_backing(||prepared.fund_one(1,required.saturating_sub(1),&mut control));
            assert_eq!(narrow.unwrap(),OperationByteFundingStep::Pending{prepared_items:0,allocated_bytes:0});assert_eq!((requested,released),(0,0));
            let (step,requested,released)=crate::test_allocation::observe_backing(||prepared.fund_one(1,maximum,&mut control));
            assert_eq!(released,0);assert!(requested<=maximum);
            match step.unwrap(){OperationByteFundingStep::Pending{prepared_items,allocated_bytes}=>{assert!(prepared_items<=1);assert_eq!(allocated_bytes,requested);},OperationByteFundingStep::Ready=>{assert_eq!(requested,0);}}
            assert_eq!(prepared.accepted_prefix().unwrap().len(),0);
            assert_eq!(control.owned_bytes(),prepared.allocated_bytes());
            receipt=control.pause().unwrap();
            if prepared.is_funded(){break;}
        }
        assert!(prepared.is_funded());
        let retained=prepared.allocated_bytes();
        let mut callback=|progress:crate::value::native_encoding::NativeEncodeProgress|!canceled||progress.completed<256;
        let mut control=crate::value::NativeEncodeControl::resume(receipt,&mut callback).unwrap();
        if canceled{
            assert_eq!(prepared.write_bytes(&expected,&mut control).unwrap_err().kind(),crate::value::ValueRefusalKind::Canceled);
            let prefix=prepared.accepted_prefix().unwrap();assert_eq!(prefix.len(),256);assert!(prefix.iter().eq(expected[..256].iter().copied()));
            assert!(prepared.take_ready().is_none());
            for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){if prepared.close_one(1,maximum).unwrap()==OperationByteCloseStep::Complete{break;}}
            assert!(prepared.terminal_is_empty());assert_eq!(prepared.allocated_bytes(),0);
        }else{
            let (emitted,requested,released)=crate::test_allocation::observe_backing(||prepared.write_bytes(&expected,&mut control));
            emitted.unwrap();assert_eq!((requested,released),(0,0));assert_eq!(prepared.allocated_bytes(),retained);
            assert!(prepared.accepted_prefix().unwrap().iter().eq(expected.iter().copied()));
            let mut source=prepared.take_ready().unwrap();assert!(prepared.terminal_is_empty());
            assert_eq!(source.allocated_bytes(),retained);assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::to_value(&expected).unwrap());
            for _ in 0..fixture["maximumCloseSteps"].as_u64().unwrap(){if source.close_one(1,maximum).unwrap()==OperationByteCloseStep::Complete{break;}}
            assert!(source.terminal_is_empty());assert_eq!(source.allocated_bytes(),0);
        }
    }
    println!("[DEBUG] Same8194 original octets use step-funded actual backing under4096 and same consuming Native admission receipt; actual emission allocates no backing, cancellation retains only genuine256 prefix and all unpaid-to-release ownership");
}

#[test]
fn owned_operation_byte_pages_empty_prepaid_handback_keeps_exact_zero_payload_authority(){
    use super::operation_bytes::OperationBytePreparation;
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let mut prepared=OperationBytePreparation::try_new(fixture["emptyOperationLength"].as_u64().unwrap()as usize,65536).unwrap();
    assert!(prepared.is_funded());assert_eq!(prepared.accepted_prefix().unwrap().len(),0);
    let mut source=prepared.take_ready().unwrap();assert!(prepared.terminal_is_empty());assert_eq!(source.allocated_bytes(),0);
    assert_eq!(serde_json::to_value(&source).unwrap(),serde_json::json!([]));
    let demand=source.next_allocation_bytes();
    assert_eq!(source.close_one(1,4096).unwrap(),OperationByteCloseStep::Complete);assert!(source.terminal_is_empty());
    assert_eq!(demand.expect_err("exact zero emitted operation cannot acquire a later payload allocation").kind,crate::value::ValueRefusalKind::OwnershipLimit);
    println!("[DEBUG] Empty measured operation handback preserves exact zero payload authority and cannot acquire future backing after publication");
}

#[test]
fn owned_operation_byte_pages_measure_complete_operation_without_retaining_payload_or_resetting_control(){
    use super::operation_bytes::{OperationByteMeasurement,OperationByteOutput};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();
    let expected:Vec<u8>=fixture["source"]["prefixUtf8"].as_str().unwrap().bytes().chain(std::iter::repeat_n(32,8192)).collect();
    for maximum in [expected.len(),expected.len()-1]{
        let mut measured=OperationByteMeasurement::new(maximum as u64);
        let mut allow=|_|true;let mut control=crate::value::NativeEncodeControl::new(65536,&mut allow);
        let (result,requested,released)=crate::test_allocation::observe_backing(||{
            measured.write_bytes(&expected[..2],&mut control)?;
            measured.write_bytes(&expected[2..],&mut control)
        });
        assert_eq!((requested,released),(0,0));assert_eq!(control.owned_bytes(),0);
        if maximum==expected.len(){result.unwrap();assert_eq!(measured.exact_length().unwrap(),expected.len());}else{
            assert_eq!(result.unwrap_err().kind(),crate::value::ValueRefusalKind::OwnershipLimit);
            assert!(measured.exact_length().is_err());assert!(measured.write_bytes(&[],&mut control).is_err());
        }
    }
    let mut measured=OperationByteMeasurement::new(expected.len()as u64);
    let mut allow=|_|true;let mut control=crate::value::NativeEncodeControl::new(65536,&mut allow);
    measured.write_bytes(&expected[..2],&mut control).unwrap();
    let receipt=control.pause().unwrap();
    let mut cancel=|progress:crate::value::native_encoding::NativeEncodeProgress|progress.completed==0;
    let mut control=crate::value::NativeEncodeControl::resume(receipt,&mut cancel).unwrap();
    assert_eq!(measured.write_bytes(&expected[2..],&mut control).unwrap_err().kind(),crate::value::ValueRefusalKind::Canceled);
    assert_eq!(control.owned_bytes(),0);assert!(measured.exact_length().is_err());
    assert_eq!(serde_json::to_value(&expected).unwrap().as_array().unwrap().len(),fixture["expected"]["payloadBytes"].as_u64().unwrap()as usize);
    println!("[DEBUG] Genuine operation measurement preserves complete8194 whole-frame ceiling and same Native control without any payload backing; finite refusal/cancellation latch prevents a false funding length");
}

#[test]
fn operation_byte_comparison_preserves_enclosing_workload_on_acceptance_and_refusal(){
    use super::operation_bytes::{OperationByteComparison,OperationByteOutput};
    let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🧭️caller-stage.json")).unwrap();
    let source:Vec<u8>=serde_json::from_value(f["source"].clone()).unwrap();let foreign:Vec<u8>=serde_json::from_value(f["foreign"].clone()).unwrap();
    for candidate in [&source,&foreign]{
        let observed=std::cell::Cell::new((0,0));
        let mut accept=|progress:crate::value::native_encoding::NativeEncodeProgress|{observed.set((progress.completed,progress.total));true};
        let mut control=crate::value::NativeEncodeControl::new(0,&mut accept);
        let total=f["callerTotal"].as_u64().unwrap()as usize;control.begin_stage(total).unwrap();control.advance(f["callerBefore"].as_u64().unwrap()as usize).unwrap();
        let mut comparison=OperationByteComparison::new(crate::codec::ByteSpan::from_slice(&source));
        let result=comparison.write_bytes(candidate,&mut control);
        if candidate==&source{result.unwrap();comparison.finish().unwrap();}else{assert_eq!(result.unwrap_err().kind(),crate::value::ValueRefusalKind::InvalidValue);assert_eq!(comparison.position(),0);assert!(comparison.finish().is_err());}
        control.step().expect("exact enclosing caller stage survives nested sink workload");assert_eq!(observed.get(),(total,total));assert_eq!(control.owned_bytes(),0);
    }
    println!("[DEBUG] Exact borrowed comparison restores caller2/2 workload after nested acceptance and original typed refusal, with no source/output ownership transfer");
}

#[test]
fn child_paged_operation_carrier_borrows_exact8194_and_returns_actual_pages_without_physical_credit(){
    use super::operation_bytes::{OperationSourceCollection,OperationByteReturnStep,OperationByteOutput};
    use crate::value::list::PagedList;
    use crate::value::retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep};
    let f:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🏠️child-carrier.json")).unwrap();
    let max=f["maximumBytes"].as_u64().unwrap()as usize;
    let mut operation=OwnedOperationBytes::try_new(f["maximumPayloadBytes"].as_u64().unwrap()as usize,f["maximumAllocationBytes"].as_u64().unwrap()as usize).unwrap();
    let mut allow=|_|true;let mut encoding=crate::value::NativeEncodeControl::new(f["maximumAllocationBytes"].as_u64().unwrap()as usize,&mut allow);
    operation.write_bytes(f["wire"].as_str().unwrap().as_bytes(),&mut encoding).unwrap();
    for _ in 0..f["paddingBytes"].as_u64().unwrap(){operation.write_bytes(&[f["paddingByte"].as_u64().unwrap()as u8],&mut encoding).unwrap();}
    let retained=operation.allocated_bytes();assert_eq!(retained,encoding.owned_bytes());assert_eq!(operation.len(),f["wireBytes"].as_u64().unwrap()as usize);
    let mut collection=PagedList::<OwnedOperationBytes,8194>::empty();
    while !collection.has_reserved_slot(){let step=collection.reserve_one(max).unwrap();assert!(step.progressed&&step.allocated_bytes<=max);}
    assert!(collection.push_reserved(operation).is_ok());
    let collection_bytes=collection.allocated_bytes();
    let ((),allocated,released)=crate::test_allocation::observe_backing(||{
        let sources:&dyn OperationSourceCollection=&collection;assert_eq!(sources.len(),1);assert!(!sources.is_empty());assert!(sources.source_at(1).is_none());
        let span=sources.source_at(0).unwrap();assert_eq!(span.len(),8194);assert_eq!(span.get(0),Some(&49));assert_eq!(span.get(1),Some(&55));
        for index in 2..8194{assert_eq!(span.get(index),Some(&32));assert!(std::ptr::eq(span.get(index).unwrap(),collection.get(0).unwrap().byte_ref(index).unwrap()));}
    });assert_eq!((allocated,released),(0,0));
    let mut parent=ParentAllocationReturn::<1>::try_new(max,f["maximumAllocationBytes"].as_u64().unwrap()as usize).unwrap();
    let owner=collection.get_mut(0).unwrap();
    for (items,bytes)in[(0,max),(1,0)]{let(step,a,r)=crate::test_allocation::observe_backing(||owner.return_one(&mut parent,items,bytes));assert_eq!(step.unwrap(),OperationByteReturnStep::Pending{returned_items:0,returned_bytes:0});assert_eq!((a,r),(0,0));assert_eq!(owner.len(),8194);assert_eq!(owner.allocated_bytes(),retained);}
    let mut physical=0;let mut handed=0;
    for _ in 0..f["maximumTurns"].as_u64().unwrap(){
        if !parent.terminal_is_empty(){
            let demand=parent.next_close_byte_demand();
            let(step,a,r)=crate::test_allocation::observe_backing(||parent.close_step(1,demand-1));assert_eq!(step,AllocationReturnStep::Pending{released_items:0,released_bytes:0});assert_eq!((a,r),(0,0));
            let(step,a,r)=crate::test_allocation::observe_backing(||parent.close_step(1,max));assert_eq!(a,0);assert_eq!(step,AllocationReturnStep::Pending{released_items:1,released_bytes:demand});assert_eq!(r,demand);assert!(r<=max);physical+=r;
        }else{
            let(step,a,r)=crate::test_allocation::observe_backing(||owner.return_one(&mut parent,1,max));assert_eq!((a,r),(0,0));
            match step.unwrap(){OperationByteReturnStep::Pending{returned_items,returned_bytes}=>{assert!(returned_items<=1);assert_eq!(parent.retained_bytes(),returned_bytes);handed+=returned_bytes;if returned_bytes!=0&&owner.allocated_bytes()!=0{let kept=owner.allocated_bytes();let(step,a,r)=crate::test_allocation::observe_backing(||owner.return_one(&mut parent,1,max));assert_eq!(step.unwrap(),OperationByteReturnStep::Pending{returned_items:0,returned_bytes:0});assert_eq!((a,r),(0,0));assert_eq!(owner.allocated_bytes(),kept);assert_eq!(parent.retained_bytes(),returned_bytes);}},OperationByteReturnStep::Complete=>break}
        }
    }
    assert!(owner.terminal_is_empty()&&parent.terminal_is_empty());assert_eq!(handed,retained);assert_eq!(physical,retained);assert_eq!(owner.allocated_bytes(),0);
    let((),a,r)=crate::test_allocation::observe_backing(||{assert!(collection.pop().unwrap().terminal_is_empty());});assert_eq!((a,r),(0,0));
    let mut outer=0;
    while !collection.terminal_is_empty(){let(step,a,r)=crate::test_allocation::observe_backing(||collection.return_empty_page(&mut parent,1));let step=step.unwrap();assert!(step.progressed);assert_eq!((a,r),(0,0));assert_eq!(step.returned_allocation_bytes,parent.retained_bytes());let(drain,a,r)=crate::test_allocation::observe_backing(||parent.close_step(1,max));assert_eq!(a,0);assert!(r<=max);assert_eq!(drain,AllocationReturnStep::Pending{released_items:1,released_bytes:r});outer+=r;}
    assert!(parent.terminal_is_empty());assert_eq!(outer,collection_bytes);
    eprintln!("[DEBUG] actual paged Child operation capability borrows exact original8194 refs without allocation; physical allocator proves every source and outer collection handoff releases0 child bytes, parent full4096 deallocation only");
}

