use super::*;

#[test]
fn retained_paged_list_owner_page_ceiling_preserves_order_and_exact_retirement() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/📏️owner-page-ceiling.json")).unwrap();
    let ceiling = fixture["ownerPayloadPageBytes"].as_u64().unwrap() as usize;
    for invalid in fixture["invalidPayloadPageBytes"].as_array().unwrap() {
        assert_eq!(PagedList::<u64, 200>::with_payload_page_bytes(invalid.as_u64().unwrap() as usize).err().unwrap().kind, PagedListRefusalKind::OwnershipLimit);
    }
    for count in fixture["boundaryCounts"].as_array().unwrap() {
        let count = count.as_u64().unwrap() as usize;
        let mut owner = PagedList::<u64, 200>::with_payload_page_bytes(ceiling).unwrap();
        let mut admitted = 0;
        for value in 0..count as u64 {
            while !owner.has_reserved_slot() {
                let bytes = owner.next_allocation_bytes().unwrap();
                assert!(bytes <= ceiling);
                let before = (owner.capacity(), owner.allocated_bytes(), owner.len());
                assert!(!owner.reserve_one(bytes - 1).unwrap().progressed);
                assert_eq!((owner.capacity(), owner.allocated_bytes(), owner.len()), before);
                let step = owner.reserve_one(bytes).unwrap();
                assert!(step.progressed);
                assert_eq!(step.allocated_bytes, bytes);
                admitted += bytes;
            }
            owner.push_reserved(value).unwrap();
        }
        assert_eq!(owner.iter().copied().collect::<Vec<_>>(), (0..count as u64).collect::<Vec<_>>());
        while owner.pop().is_some() {}
        let mut returned = 0;
        while !owner.terminal_is_empty() {
            let bytes = owner.next_release_allocation_bytes().unwrap();
            assert!(!owner.release_empty_page(bytes - 1).unwrap().progressed);
            let step = owner.release_empty_page(bytes).unwrap();
            assert!(step.progressed);
            returned += step.released_allocation_bytes;
        }
        assert_eq!(returned, admitted);
    }
    let mut owner = PagedList::<u64, 200>::with_payload_page_bytes(ceiling).unwrap();
    for value in fixture["input"].as_array().unwrap().iter().chain(fixture["append"].as_array().unwrap()) {
        while !owner.has_reserved_slot() { assert!(owner.reserve_one(ceiling).unwrap().progressed); }
        owner.push_reserved(value.as_u64().unwrap()).unwrap();
    }
    assert_eq!(serde_json::to_value(owner.iter().copied().collect::<Vec<_>>()).unwrap(), fixture["expected"]);
    assert_eq!(owner.iter().sum::<u64>(), fixture["sum"].as_u64().unwrap());
    let mut default = PagedList::<u64, 600>::default();
    assert!(default.reserve_one(ceiling).unwrap().progressed);
    assert_eq!(default.next_allocation_bytes().unwrap(), fixture["defaultPayloadPageBytes"].as_u64().unwrap() as usize);
    println!("[DEBUG] Paged list owner ceiling={} boundary cases={} exact ordered rows={}", ceiling, fixture["boundaryCounts"].as_array().unwrap().len(), owner.len());
}

#[test]
fn retained_paged_list_neutral_order_capacity_and_close() {
    let data: serde_json::Value = serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let grant = data["maximumPageBytes"].as_u64().unwrap() as usize;
    let mut list = PagedList::<u64, 600>::default();
    let mut admitted = 0;
    for index in 0..data["ordered"]["count"].as_u64().unwrap() {
        while !list.has_reserved_slot() {
            let step = list.reserve_one(grant).unwrap();
            assert!(step.progressed);
            assert!(step.allocated_bytes <= grant);
            admitted += step.allocated_bytes;
        }
        assert_eq!(list.push_reserved(index), Ok(()));
        assert_eq!(list.initialized_len(), list.len());
    }
    let expected: Vec<u64> = (0..600).collect();
    assert_eq!(serde_json::to_value(list.iter().copied().collect::<Vec<_>>()).unwrap(), serde_json::to_value(expected).unwrap());
    assert_eq!(list.iter().copied().sum::<u64>(), data["ordered"]["sum"].as_u64().unwrap());
    assert_eq!(list.push_reserved(data["capacity"]["rejected"].as_u64().unwrap()), Err(601));
    let before = list.allocated_bytes();
    assert!(list.release_empty_page(grant).is_err());
    assert_eq!(list.allocated_bytes(), before);
    while list.pop().is_some() { assert_eq!(list.initialized_len(), list.len()); }
    assert!(!list.release_empty_page(0).unwrap().progressed);
    assert_eq!(list.allocated_bytes(), before);
    let mut released = 0;
    while !list.terminal_is_empty() {
        let exact = list.next_release_allocation_bytes().unwrap();
        let step = list.release_empty_page(exact).unwrap();
        assert!(step.progressed);
        assert_eq!(step.released_allocation_bytes, exact);
        released += step.released_allocation_bytes;
    }
    assert_eq!(released, admitted);
    assert_eq!(list.len(), 0);
    assert_eq!(list.capacity(), 0);
    assert_eq!(list.allocated_bytes(), 0);
    assert_eq!(list.root.capacity(), 0);
}

#[test]
fn retained_paged_list_capacity_admission_and_exact_release_grants() {
    let mut list = PagedList::<u64, 600>::default();
    while list.capacity() < 600 {
        let exact = list.next_capacity_allocation_bytes(600).unwrap().expect("target still needs backing");
        assert!(exact > 0, "a target beyond current capacity must request its next real backing even while the current leaf has unused slots");
        let before = (list.capacity(), list.allocated_bytes());
        assert!(!list.reserve_capacity_one(600, exact - 1).unwrap().progressed);
        assert_eq!((list.capacity(), list.allocated_bytes()), before);
        let step = list.reserve_capacity_one(600, exact).unwrap();
        assert!(step.progressed);
        assert_eq!(step.allocated_bytes, exact);
    }
    assert_eq!(list.next_capacity_allocation_bytes(600), Ok(None));
    assert_eq!(list.len(), 0);
    assert_eq!(list.initialized_len(), 0);
    assert!(list.reserve_capacity_one(601, 4096).is_err());
    assert_eq!(list.capacity(), 600);
    let pointer = list.backing_ptr(512).unwrap();
    let exact = list.next_release_allocation_bytes().unwrap();
    assert_eq!(exact, 88 * size_of::<u64>());
    assert!(!list.release_empty_page(exact - 1).unwrap().progressed);
    assert_eq!(list.backing_ptr(512).unwrap(), pointer);
    let released = list.release_empty_page(88 * size_of::<u64>()).unwrap();
    assert_eq!(released.released_allocation_bytes, 88 * size_of::<u64>());
    assert_eq!(list.capacity(), 512);
    while !list.terminal_is_empty() {
        let exact = list.next_release_allocation_bytes().unwrap();
        list.release_empty_page(exact).unwrap();
    }
    assert_eq!((list.root.capacity(), list.len(), list.capacity(), list.allocated_bytes()), (0, 0, 0, 0));
}

#[test]
fn paged_child_source_returns_genuine8194_backing_to_parent_without_physical_credit(){
    use crate::retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🏠️parent-return.json")).unwrap();
    let maximum=fixture["maximumBytes"].as_u64().unwrap()as usize;
    let exact=fixture["wireBytes"].as_u64().unwrap()as usize;
    let mut source=PagedList::<u8,8194>::empty();
    for byte in fixture["wire"].as_str().unwrap().bytes().chain(std::iter::repeat_n(fixture["paddingByte"].as_u64().unwrap()as u8,fixture["paddingBytes"].as_u64().unwrap()as usize)){
        while !source.has_reserved_slot(){let progress=source.reserve_one(maximum).unwrap();assert!(progress.progressed&&progress.allocated_bytes<=maximum);}
        source.push_reserved(byte).unwrap();
    }
    assert_eq!(source.len(),exact);
    let pointer=source.backing_ptr(0).unwrap();
    let initial=source.allocated_bytes();
    let mut parent=ParentAllocationReturn::<1>::try_new(maximum,fixture["parentTotalBytes"].as_u64().unwrap()as usize).unwrap();
    assert!(!source.return_empty_page(&mut parent,0).unwrap().progressed);
    assert_eq!(source.backing_ptr(0),Some(pointer));assert_eq!(source.len(),exact);assert!(parent.terminal_is_empty());
    assert_eq!(source.return_empty_page(&mut parent,1).unwrap_err().kind,crate::ValueRefusalKind::InvariantViolated);
    assert_eq!(source.backing_ptr(0),Some(pointer));assert_eq!(source.len(),exact);assert!(parent.terminal_is_empty());
    while source.pop().is_some(){}
    let expected=source.next_release_allocation_bytes().unwrap();
    let mut blocker=Vec::with_capacity(1);blocker.push(9u8);assert!(parent.return_bytes(&mut blocker,1).unwrap());
    let before=source.allocated_bytes();
    assert!(!source.return_empty_page(&mut parent,1).unwrap().progressed);
    assert_eq!(source.allocated_bytes(),before);assert_eq!(parent.retained_bytes(),1);
    assert_eq!(parent.close_step(1,maximum),AllocationReturnStep::Pending{released_items:1,released_bytes:1});assert!(parent.terminal_is_empty());
    let mut insufficient=ParentAllocationReturn::<1>::try_new(expected-1,expected-1).unwrap();
    assert_eq!(source.return_empty_page(&mut insufficient,1).unwrap_err().kind,crate::ValueRefusalKind::OwnershipLimit);
    assert_eq!(source.allocated_bytes(),before);assert!(insufficient.terminal_is_empty());
    let mut returned=0;let mut released=0;
    for _ in 0..fixture["lawTurns"].as_u64().unwrap(){
        if source.terminal_is_empty()&&parent.terminal_is_empty(){break;}
        if !parent.terminal_is_empty(){
            let exact=parent.next_close_byte_demand();
            assert_eq!(parent.close_step(1,exact-1),AllocationReturnStep::Pending{released_items:0,released_bytes:0});
            assert!(!parent.terminal_is_empty());
            match parent.close_step(1,maximum){AllocationReturnStep::Pending{released_items,released_bytes}=>{assert_eq!(released_items,1);assert_eq!(released_bytes,exact);assert!(released_bytes<=maximum);released+=released_bytes;},AllocationReturnStep::Complete=>panic!("actual parent owner disappeared")}
        }else{
            let before=source.allocated_bytes();let progress=source.return_empty_page(&mut parent,1).unwrap();
            assert!(progress.progressed);assert_eq!(before-source.allocated_bytes(),progress.returned_allocation_bytes);assert_eq!(parent.retained_bytes(),progress.returned_allocation_bytes);assert!(!parent.terminal_is_empty());returned+=progress.returned_allocation_bytes;
        }
    }
    assert!(source.terminal_is_empty()&&parent.terminal_is_empty());assert_eq!(returned,initial);assert_eq!(released,initial);
    eprintln!("[DEBUG] original8194 paged child backing transfers actual payload/metadata allocations under preadmitted one-slot parent; zero/live/full/small refusal unchanged, child zero physical credit, parent actual1/4096 deallocation");
}


#[test]
fn paged_actual_height_preserves_maximum_authority_pointer_and_parent_handoff(){
    use crate::retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🌳️actual-height.json")).unwrap();assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);assert_eq!(fixture["ownerWords"],7);assert_eq!(size_of::<PagedList<u64,{isize::MAX as usize}>>(),7*size_of::<usize>());
    let mut scalar=PagedList::<u64,{isize::MAX as usize}>::empty();let mut allocations=0;while !scalar.has_reserved_slot(){let required=scalar.next_allocation_bytes().unwrap();assert!(required<=4096);let step=scalar.reserve_one(4096).unwrap();assert!(step.progressed);assert_eq!(step.allocated_bytes,required);allocations+=1;}assert!(scalar.push_reserved(fixture["scalar"].as_u64().unwrap()).is_ok());assert!(allocations<=2,"one scalar preallocated maximum-height scaffolding");assert!(scalar.allocated_bytes()<=8192);assert_eq!(scalar[0],7);scalar.pop();while !scalar.terminal_is_empty(){assert!(scalar.release_empty_page(4096).unwrap().progressed)}
    for bytes in fixture["growthPayloadBytes"].as_array().unwrap(){
        let length=bytes.as_u64().unwrap()as usize;let mut owner=PagedList::<u8,{isize::MAX as usize}>::empty();let mut first=None;assert_eq!(owner.next_push_depth_demand().unwrap_err().kind,PagedListRefusalKind::OwnershipLimit);let mut depths=Vec::new();
        for index in 0..length{
            while !owner.has_reserved_slot(){let required=owner.next_allocation_bytes().unwrap();assert!(required<=4096);let retained=owner.allocated_bytes();let pointer=owner.backing_ptr(0);assert!(!owner.reserve_one(required.saturating_sub(1)).unwrap().progressed);assert_eq!(owner.allocated_bytes(),retained);assert_eq!(owner.backing_ptr(0),pointer);let step=owner.reserve_one(4096).unwrap();assert!(step.progressed);assert_eq!(step.allocated_bytes,required);}
            if let Some(row)=fixture["reservedPushDepth"].as_array().unwrap().iter().find(|row|row["index"].as_u64().unwrap()as usize==index){let before=(owner.len(),owner.allocated_bytes(),owner.backing_ptr(0));let depth=owner.next_push_depth_demand().unwrap();assert_eq!(depth,row["depth"].as_u64().unwrap()as usize);assert_eq!((owner.len(),owner.allocated_bytes(),owner.backing_ptr(0)),before);depths.push(serde_json::json!({"index":index,"depth":depth}));}
            assert!(owner.push_reserved((index%251)as u8).is_ok());if index==0{first=owner.backing_ptr(0);}assert_eq!(owner.backing_ptr(0),first);
        }
        assert_eq!(serde_json::to_value(depths).unwrap(),serde_json::Value::Array(fixture["reservedPushDepth"].as_array().unwrap().iter().filter(|row|row["index"].as_u64().unwrap()<length as u64).cloned().collect()));
        assert_eq!(owner.len(),length);for(index,byte)in owner.iter().enumerate(){assert_eq!(*byte,(index%251)as u8)}let allocated=owner.allocated_bytes();while owner.pop().is_some(){}let mut parent=ParentAllocationReturn::<512>::try_new(4096,allocated).unwrap();let mut returned=0;
        while !owner.terminal_is_empty(){let before=owner.allocated_bytes();let step=owner.return_empty_page(&mut parent,1).unwrap();assert!(step.progressed);assert_eq!(before-owner.allocated_bytes(),step.returned_allocation_bytes);returned+=step.returned_allocation_bytes;}assert_eq!(returned,allocated);assert_eq!(owner.allocated_bytes(),0);assert_eq!(parent.retained_bytes(),allocated);let mut physical=0;for _ in 0..512+128{match parent.close_step(1,4096){AllocationReturnStep::Complete=>break,AllocationReturnStep::Pending{released_items,released_bytes}=>{assert!(released_items<=1);assert!(released_bytes<=4096);physical+=released_bytes;}}}assert!(parent.terminal_is_empty());assert_eq!(physical,allocated);
    }
    println!("[DEBUG] actual-height paged owners preserve maximum logical authority and seven-word owner-ceiling layout, singleton only leaf+payload allocations,8194/65537 exact source and original pointer, denied growth unchanged and real parent handoff with zero child physical disposal1/4096");
}

#[test]
fn paged_formatted_exact_extent_preserves_full_source_and_four_locale_cells_at_original_64k(){
    use crate::{NativeEncodeControl,paged_text::{PagedText,InlineTextBuffer,TextReadSource,write_encoding_format},ErasedSnapshotRetirement,retained_clone::RetainedCloneStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🏷️formatted-exact-extent.json")).unwrap();assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);assert_eq!(fixture["payloadBytes"],8194);assert_eq!(fixture["originalWireBytes"],8194);
    struct Wire;impl TextReadSource for Wire{fn byte_len(&self)->usize{8194}fn byte_at(&self,index:usize)->Option<u8>{if index>=8194{None}else{Some(if index==0{b'1'}else if index==1{b'7'}else{b' '})}}}
    let value="x".repeat(8194);let pointer=value.as_ptr();let mut owners:[PagedText<{isize::MAX as usize}>;5]=std::array::from_fn(|_|PagedText::empty());let mut inlines:[InlineTextBuffer;5]=std::array::from_fn(|_|InlineTextBuffer::empty());let mut paged=[false;5];let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);let mut refusal=None;
    if let Err(error)=owners[0].read_from_encoding_source(&Wire,&mut control){refusal=Some(error);}
    for index in 1..5{if refusal.is_some(){break}let language=fixture["matrix"][index-1].as_str().unwrap();let(prefix,suffix)=if language=="En"{(fixture["prefixEn"].as_str().unwrap(),fixture["suffixEn"].as_str().unwrap())}else{(fixture["prefixDe"].as_str().unwrap(),fixture["suffixDe"].as_str().unwrap())};if let Err(error)=owners[index].encode_with_inline(&mut inlines[index],&mut paged[index],16384,&mut control,|output|write_encoding_format(output,format_args!("{prefix}{value}{suffix}"))){refusal=Some(error);}}
    let all_complete=refusal.is_none();let admitted=control.owned_bytes();assert_eq!(value.as_ptr(),pointer);let retained=owners.iter().map(PagedText::allocated_bytes).sum::<usize>();
    if all_complete{assert_eq!(owners[0].borrow().unwrap().byte_len(),8194);for(index,owner)in owners.iter().enumerate().skip(1){let en=fixture["matrix"][index-1]=="En";let expected=fixture[if en{"expectedEnBytes"}else{"expectedDeBytes"}].as_u64().unwrap()as usize;let text=owner.borrow().unwrap();assert!(paged[index]);assert_eq!(text.byte_len(),expected);let(prefix,suffix)=if en{(fixture["prefixEn"].as_str().unwrap(),fixture["suffixEn"].as_str().unwrap())}else{(fixture["prefixDe"].as_str().unwrap(),fixture["suffixDe"].as_str().unwrap())};for(at,byte)in prefix.bytes().chain(value.bytes()).chain(suffix.bytes()).enumerate(){assert_eq!(text.byte_at(at),Some(byte));}}assert_eq!(retained,admitted);assert!(admitted<=65536);}
    let mut disposed=0;for(index,owner)in owners.iter_mut().enumerate(){inlines[index].close_one(1);for _ in 0..8194+256{match owner.close_step(crate::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_release_bytes:4096,maximum_depth:1,..Default::default()}).unwrap(){RetainedCloneStep::Complete(progress)=>{assert_eq!(progress,Default::default());break;},RetainedCloneStep::Progress(crate::retained_clone::RetainedCloneProgress{copied_items:released_items,released_bytes,..})=>{assert!(released_items<=1&&released_bytes<=4096);disposed+=released_bytes;}}}assert!(owner.terminal_is_empty());}
    assert_eq!(disposed,retained);assert!(all_complete,"full original source and all four original locale cells exceed unchanged64k because final payload pages retain unused capacity: {refusal:?}; admitted={admitted}; retained={retained}");eprintln!("[DEBUG] unchanged full8194 literal source and actual four8207/8218 locale outputs fit original cumulative64k through exact final extents; original typed input pointer survives; all retained physical text pages independently dispose1/4096; encoded operation trait/model/formatter scaffolds remain separate");
}

#[test]
fn paged_formatted_exact_extent_retains_every_refused_prefix_and_complete_utf8_borrow(){
    use crate::{NativeEncodeControl,native_encoding::NativeEncodeProgress,paged_text::{PagedText,InlineTextBuffer,write_encoding_format},ErasedSnapshotRetirement,retained_clone::RetainedCloneStep};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/❌️formatted-exact-extent.json")).unwrap();assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);assert_eq!(fixture["initialAllocationBytes"],4096);assert_eq!(fixture["interiorBytes"],256);
    struct Changed<'a>{calls:std::cell::Cell<usize>,first:&'a str,second:&'a str}
    impl std::fmt::Display for Changed<'_>{fn fmt(&self,output:&mut std::fmt::Formatter<'_>)->std::fmt::Result{let call=self.calls.get();self.calls.set(call+1);output.write_str(if call==0{self.first}else{self.second})}}
    struct Refused;impl std::fmt::Display for Refused{fn fmt(&self,output:&mut std::fmt::Formatter<'_>)->std::fmt::Result{output.write_str("short")?;Err(std::fmt::Error)}}
    let payload=fixture["shortText"].as_str().unwrap().repeat(fixture["longRepeat"].as_u64().unwrap()as usize)+fixture["longSuffix"].as_str().unwrap();assert_eq!(payload.len(),8194);let pointer=payload.as_ptr();
    for mode in fixture["modes"].as_array().unwrap(){
        let mode=mode.as_str().unwrap();let mut canceled=false;let mut measured=false;let mut allow=|progress:NativeEncodeProgress|{if progress.total==1{measured=true}let refuse=!canceled&&match mode{"count-cancel"=>progress.total==1&&progress.completed==0,"declaration-cancel"|"ignored-declaration-cancel"=>measured&&progress.total==0,"interior-cancel"=>progress.total==8194&&progress.completed>=256,_=>false};if refuse{canceled=true;false}else{true}};
        let mut control=NativeEncodeControl::new(if mode=="initial4096"{4096}else{65536},&mut allow);let mut owner=PagedText::<{isize::MAX as usize}>::empty();let changed=Changed{calls:std::cell::Cell::new(0),first:if mode=="second-long"{"short"}else{&payload},second:if mode=="second-short"{"short"}else{&payload}};
        let result=owner.encode_with(if mode=="segment-one-short"{8193}else{16384},&mut control,|output|match mode{"second-short"|"second-long"=>write_encoding_format(output,format_args!("{changed}")),"ignored-declaration-cancel"=>{let _=write_encoding_format(output,format_args!("{payload}"));Ok(())},"duplicate-formatter"=>{write_encoding_format(output,format_args!("short"))?;let _=write_encoding_format(output,format_args!("short"));Ok(())},"ignored-formatter-error"=>{let _=write_encoding_format(output,format_args!("{}",Refused));Ok(())},_=>write_encoding_format(output,format_args!("{payload}"))});
        assert!(result.is_err(),"refused original semantic extent gained complete authority: {mode}");assert!(owner.borrow().is_err());assert_eq!(payload.as_ptr(),pointer);assert_eq!(owner.allocated_bytes(),control.owned_bytes());if mode=="interior-cancel"{assert_eq!(owner.byte_len(),256)}if mode=="second-short"||mode=="duplicate-formatter"{assert_eq!(owner.byte_len(),5)}if mode=="count-cancel"||mode=="declaration-cancel"||mode=="ignored-declaration-cancel"||mode=="segment-one-short"||mode=="second-long"||mode=="ignored-formatter-error"{assert_eq!(owner.allocated_bytes(),0)}if mode=="initial4096"{assert_eq!(owner.byte_len(),0);assert!(owner.allocated_bytes()>0&&owner.allocated_bytes()<4096)}
        let retained=owner.allocated_bytes();let mut disposed=0;for _ in 0..8194+256{match owner.close_step(crate::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_release_bytes:4096,maximum_depth:1,..Default::default()}).unwrap(){RetainedCloneStep::Complete(progress)=>{assert_eq!(progress,Default::default());break;},RetainedCloneStep::Progress(crate::retained_clone::RetainedCloneProgress{copied_items:released_items,released_bytes,..})=>{assert!(released_items<=1&&released_bytes<=4096);disposed+=released_bytes}}}assert!(owner.terminal_is_empty());assert_eq!(disposed,retained);
    }
    for text in [fixture["shortText"].as_str().unwrap(),payload.as_str()]{let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);let mut owner=PagedText::<{isize::MAX as usize}>::empty();let mut inline=InlineTextBuffer::empty();let mut paged=false;owner.encode_with_inline(&mut inline,&mut paged,8194,&mut control,|output|write_encoding_format(output,format_args!("{text}"))).unwrap();if text.len()<=128{assert!(!paged);assert_eq!(inline.borrow().unwrap(),text);assert!(owner.borrow().is_err());assert_eq!(control.owned_bytes(),0)}else{assert!(paged);assert!(inline.borrow().is_err());let span=owner.borrow().unwrap();assert_eq!(span.byte_len(),8194);for(index,byte)in text.bytes().enumerate(){assert_eq!(span.byte_at(index),Some(byte))}}let retained=owner.allocated_bytes();inline.close_one(1);let mut disposed=0;for _ in 0..8194+256{match owner.close_step(crate::retained_clone::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_release_bytes:4096,maximum_depth:1,..Default::default()}).unwrap(){RetainedCloneStep::Complete(progress)=>{assert_eq!(progress,Default::default());break;},RetainedCloneStep::Progress(crate::retained_clone::RetainedCloneProgress{copied_items:released_items,released_bytes,..})=>{assert!(released_items<=1&&released_bytes<=4096);disposed+=released_bytes}}}assert!(owner.terminal_is_empty());assert_eq!(disposed,retained);}
    eprintln!("[DEBUG] exact complete UTF8 source8194 and intrinsic6 bytes preserve real composite borrows; all ten extent/count/declaration/interior/initial4096/changed-output/ignored-refusal traces retain their original prefixes and dispose actual physical backing1/4096");
}

#[test]
fn paged_list_exact_final_extent_preserves_original_8194_and_parent_authority(){
    use crate::{NativeEncodeControl,retirement::allocation_return::{ParentAllocationReturn,AllocationReturnStep}};
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/📏️exact-final-extent.json")).unwrap();
    assert_eq!(fixture["ownerWords"],7);assert_eq!(fixture["sourceBytes"],8194);assert_eq!(fixture["sourceCount"],5);assert_eq!(fixture["maximumAllocationBytes"],65536);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);assert_eq!(size_of::<PagedList<u8,{isize::MAX as usize}>>(),7*size_of::<usize>());
    let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);let mut owners:[PagedList<u8,{isize::MAX as usize}>;5]=std::array::from_fn(|_|PagedList::empty());let mut pointers=[None;5];let mut total=0;
    for(ordinal,owner)in owners.iter_mut().enumerate(){
        for index in 0..8194{
            while !owner.has_reserved_slot(){
                let required=owner.next_exact_capacity_allocation_bytes(8194).unwrap().unwrap();assert!(required<=4096);let retained=owner.allocated_bytes();let pointer=owner.backing_ptr(0);assert!(!owner.reserve_exact_capacity_one(8194,required.saturating_sub(1)).unwrap().progressed);assert_eq!(owner.allocated_bytes(),retained);assert_eq!(owner.backing_ptr(0),pointer);control.charge(required).unwrap();let step=owner.reserve_exact_capacity_one(8194,4096).unwrap();assert!(step.progressed);assert_eq!(step.allocated_bytes,required);
            }
            assert!(owner.push_reserved(if index==0{b'1'}else if index==1{b'7'}else{b' '}).is_ok());if index==0{pointers[ordinal]=owner.backing_ptr(0);}assert_eq!(owner.backing_ptr(0),pointers[ordinal]);
        }
        assert_eq!(owner.len(),8194);assert_eq!(owner.capacity(),8194);assert_eq!(owner.leaf(0).unwrap().capacity(),4096);assert_eq!(owner.leaf(4096).unwrap().capacity(),4096);assert_eq!(owner.leaf(8192).unwrap().capacity(),2);for(index,byte)in owner.iter().enumerate(){assert_eq!(*byte,if index==0{b'1'}else if index==1{b'7'}else{b' '});}
        let retained=owner.allocated_bytes();assert!(owner.next_exact_capacity_allocation_bytes(8195).is_err());assert!(owner.reserve_exact_capacity_one(8195,4096).is_err());assert_eq!(owner.backing_ptr(0),pointers[ordinal]);assert_eq!(owner.allocated_bytes(),retained);assert_eq!(owner.len(),8194);total+=retained;
    }
    assert_eq!(control.owned_bytes(),total);assert!(total<=65536);assert!(total>=40970);let mut parent=ParentAllocationReturn::<512>::try_new(4096,total).unwrap();let mut returned=0;
    for owner in &mut owners{
        while owner.pop().is_some(){}let before=owner.allocated_bytes();assert!(!owner.return_empty_page(&mut parent,0).unwrap().progressed);assert_eq!(owner.allocated_bytes(),before);
        while !owner.terminal_is_empty(){let before=owner.allocated_bytes();let step=owner.return_empty_page(&mut parent,1).unwrap();assert!(step.progressed);assert_eq!(before-owner.allocated_bytes(),step.returned_allocation_bytes);returned+=step.returned_allocation_bytes;}
    }
    assert_eq!(returned,total);assert_eq!(parent.retained_bytes(),total);let mut disposed=0;let mut observed_tail=false;
    for _ in 0..512+128{if parent.terminal_is_empty(){break}let required=parent.next_close_byte_demand();if required==2{observed_tail=true;}assert_eq!(parent.close_step(1,required.saturating_sub(1)),AllocationReturnStep::Pending{released_items:0,released_bytes:0});match parent.close_step(1,4096){AllocationReturnStep::Pending{released_items,released_bytes}=>{assert_eq!(released_items,1);assert_eq!(released_bytes,required);assert!(released_bytes<=4096);disposed+=released_bytes;},AllocationReturnStep::Complete=>panic!("genuine parent token disappeared before its physical allocation")}}
    assert!(observed_tail);assert_eq!(disposed,total);assert!(parent.terminal_is_empty());assert!(owners.iter().all(PagedList::terminal_is_empty));
    let mut initial=PagedList::<u8,{isize::MAX as usize}>::empty();let mut allow=|_|true;let mut limited=NativeEncodeControl::new(4096,&mut allow);let metadata=initial.next_exact_capacity_allocation_bytes(8194).unwrap().unwrap();limited.charge(metadata).unwrap();assert!(initial.reserve_exact_capacity_one(8194,4096).unwrap().progressed);let payload=initial.next_exact_capacity_allocation_bytes(8194).unwrap().unwrap();assert_eq!(payload,4096);assert!(limited.charge(payload).is_err());assert_eq!(initial.len(),0);assert_eq!(initial.allocated_bytes(),metadata);while !initial.terminal_is_empty(){assert!(initial.release_empty_page(4096).unwrap().progressed)}
    eprintln!("[DEBUG] five original8194 literal sources retain exact4096/4096/2 payload pages and actual metadata under unchanged cumulative64k; seven-word owner-ceiling wrapper and pointers survive short grants/forbidden extension; initial4096 still refuses actual first payload; real parent alone physically deallocates every full token1/4096");
}
