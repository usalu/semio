//! 🧪️ One schema-issued caller proves original native pages survive paid aliases and terminal closure.

use semio_framework_job::{Generation, OperationId, OriginalJobFixtureCaller, RetainedCloneGrant, RetainedCloneProgress};
use semio_framework_ui_runtime::{UiNativeMeshResource, UiNativeResourceLoans, UiNativeResources};
use ui_scene::math::*;

#[global_allocator]
static NATIVE_RESOURCE_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

const TURN: RetainedCloneGrant = RetainedCloneGrant { maximum_items: 1, maximum_copy_bytes: 64, maximum_capacity_bytes: 1_048_576, maximum_release_bytes: 1_048_576, maximum_depth: 256 };

fn clock() -> Option<u64> { Some(1000) }

fn grant(value: &serde_json::Value) -> RetainedCloneGrant {
    RetainedCloneGrant { maximum_items: value["maximumItems"].as_u64().unwrap() as usize, maximum_copy_bytes: value["maximumCopyBytes"].as_u64().unwrap() as usize, maximum_capacity_bytes: value["maximumCapacityBytes"].as_u64().unwrap() as usize, maximum_release_bytes: value["maximumReleaseBytes"].as_u64().unwrap() as usize, maximum_depth: value["maximumDepth"].as_u64().unwrap() as usize }
}

fn paid<R>(caller: &mut OriginalJobFixtureCaller, run: impl FnOnce(RetainedCloneGrant) -> (R, RetainedCloneProgress)) -> R {
    caller.turn(TURN, |cx| {
        assert_eq!(cx.operation(), OperationId(941));
        assert_eq!(cx.generation(), Generation(17));
        assert!(!cx.should_yield());
        let ((result, receipt), heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| run(cx.retained_grant()));
        assert_eq!((receipt.retained_capacity_bytes, receipt.released_bytes), (heap.requested_bytes, heap.released_bytes));
        cx.consume_retained(receipt)?;
        if receipt != RetainedCloneProgress::default() { cx.consume_fuel(1); }
        Ok(result)
    }).unwrap()
}

fn paid_context<R>(caller: &mut OriginalJobFixtureCaller, run: impl FnOnce(&mut semio_framework_job::StepContext<'_>) -> Result<R, semio_framework_job::OriginalTurnFailure>) -> R {
    caller.turn(TURN, |cx| {
        let before = cx.retained_progress();
        let (result, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| run(cx));
        let after = cx.retained_progress();
        assert_eq!((after.retained_capacity_bytes - before.retained_capacity_bytes, after.released_bytes - before.released_bytes), (heap.requested_bytes, heap.released_bytes));
        result
    }).unwrap()
}

#[test]
fn native_resource_owner_and_loan_bundle_preserve_original_pages_and_full_close() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../../../🎬️scene/📐️math/📦️resource/🧫️fixtures/🔣️.json")).unwrap();
    let policy = &fixture["caller"];
    assert_eq!(TURN, grant(&policy["turn"]));
    assert_eq!(clock(), policy["clock"].as_u64());
    let stock = grant(&policy["stock"]);
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(policy["operation"].as_u64().unwrap()), Generation(policy["generation"].as_u64().unwrap()), stock, clock, policy["deadline"].as_u64().unwrap(), policy["fuel"].as_u64().unwrap());
    let token = paid(&mut caller, |grant| mesh3d_begin(17, 23, Mesh3dSchema::triangle_mesh(3, 3), grant).unwrap());
    loop { if paid(&mut caller, |grant| { let step = mesh3d_allocate_step(token, grant).unwrap(); (step.complete, step.progress) }) { break; } }
    let positions = [[0.0, 0.0, 0.0], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]];
    for position in positions { paid(&mut caller, |grant| { assert!(grant.maximum_copy_bytes >= 12); mesh3d_write_vec3(token, Mesh3dField::Positions, position).unwrap(); ((), RetainedCloneProgress { copied_items: 1, copied_bytes: 12, ..Default::default() }) }); }
    for _ in 0..3 { paid(&mut caller, |grant| { assert!(grant.maximum_copy_bytes >= 12); mesh3d_write_vec3(token, Mesh3dField::Normals, [0.0, 0.0, 1.0]).unwrap(); ((), RetainedCloneProgress { copied_items: 1, copied_bytes: 12, ..Default::default() }) }); }
    for index in 0..3 { paid(&mut caller, |grant| { assert!(grant.maximum_copy_bytes >= 4); mesh3d_write_u32(token, Mesh3dField::Indices, index).unwrap(); ((), RetainedCloneProgress { copied_items: 1, copied_bytes: 4, ..Default::default() }) }); }
    let lease = paid(&mut caller, |_| (mesh3d_seal(token).unwrap(), RetainedCloneProgress { copied_items: 1, ..Default::default() }));
    let original_page = lease.with_original_page(0, |page| page.as_ptr() as usize).unwrap();
    assert_eq!(lease.original_page_count().unwrap(), 1);
    let identity = Mesh3dResourceIdentity { operation: 941, generation: 17, revision: 23, cache_epoch: 1, source_slot: 0 };
    let mut source = Some(lease);
    let owner = paid(&mut caller, |grant| { let (owner, receipt) = Mesh3dResourceOwner::admit(&mut source, identity, grant).unwrap().unwrap(); (owner, receipt) });
    assert!(source.is_none());
    assert_eq!(mesh3d_begin_close(lease), Err(Mesh3dFault::Order));
    let mut surface = ui_contract::UiText::default();
    let mut mesh_key = ui_contract::UiText::default();
    for (text, target) in [("original-preview", &mut surface), ("original-native-source-with-an-incremental-key-longer-than-sixty-four-original-bytes", &mut mesh_key)] {
        for chunk in text.as_bytes().chunks(TURN.maximum_copy_bytes) {
            let text = std::str::from_utf8(chunk).unwrap();
            paid(&mut caller, |grant| { assert!(grant.maximum_copy_bytes >= text.len()); std::fmt::Write::write_str(target, text).unwrap(); ((), RetainedCloneProgress { copied_items: 1, copied_bytes: text.len(), ..Default::default() }) });
        }
    }
    let mut row = Some(paid(&mut caller, |_| (UiNativeMeshResource { surface, mesh_key, owner }, RetainedCloneProgress { copied_items: 1, ..Default::default() })));
    let mut resources = UiNativeResources::default();
    while row.is_some() { paid(&mut caller, |grant| { let step = resources.admit_row(&mut row, grant).unwrap(); (step.complete, step.progress) }); }
    let mut first = UiNativeResourceLoans::default();
    while !first.capture_complete() { paid_context(&mut caller, |cx| { first.capture_step_on_original(&resources, cx)?; Ok(()) }); }
    let mut prepared = UiNativeResourceLoans::default();
    while !prepared.capture_complete() { paid_context(&mut caller, |cx| { prepared.capture_step_on_original(&first, cx)?; Ok(()) }); }
    let descriptor = paid_context(&mut caller, |cx| first.describe_mesh_on_original(0, cx)).unwrap();
    assert_eq!(descriptor.geometry_bytes, 84);
    assert_eq!(descriptor.component_counts, [0; 3]);
    let transport: serde_json::Value = serde_json::from_str(include_str!("../📤️transport/🧫️fixtures/🔣️.json")).unwrap();
    let expected: Vec<u8> = transport["geometry"].as_array().unwrap().iter().map(|value| value.as_u64().unwrap() as u8).collect();
    let mut geometry = [0u8; 84];
    let mut offset = 0usize;
    while offset < geometry.len() {
        let chunk = paid_context(&mut caller, |cx| first.read_mesh_chunk_on_original(0, descriptor, semio_framework_ui_runtime::UiNativeMeshChannel::Geometry, offset, &mut geometry[offset..], cx)).unwrap();
        assert!(chunk.bytes > 0 && chunk.bytes <= 64);
        offset += chunk.bytes;
    }
    assert_eq!(geometry.as_slice(), expected.as_slice());
    let original_geometry = geometry;
    paid_context(&mut caller, |cx| cx.with_original_work_loan(0, 1500, |same| {
        assert!(first.read_mesh_chunk_on_original(0, descriptor, semio_framework_ui_runtime::UiNativeMeshChannel::Geometry, 0, &mut geometry, same)?.is_none());
        Ok(())
    }));
    assert_eq!(geometry, original_geometry);
    assert_eq!(first.row(0).unwrap().mesh_key.as_str(), resources.row(0).unwrap().mesh_key.as_str());
    assert_eq!(prepared.row(0).unwrap().loan.lease(), Some(lease));
    assert_eq!(prepared.row(0).unwrap().loan.lease().unwrap().with_original_page(0, |page| page.as_ptr() as usize).unwrap(), original_page);
    let before = caller.cumulative_receipt();
    caller.turn(TURN, |cx| cx.with_original_work_loan(0, 1500, |same| { let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| resources.close_step_on_original(same)); assert_eq!(heap.requested_bytes, 0); assert_eq!(heap.released_bytes, 0); assert_eq!(step.unwrap(), None); Ok(()) })).unwrap();
    assert_eq!(caller.cumulative_receipt(), before);
    paid_context(&mut caller, |cx| { assert_eq!(resources.close_step_on_original(cx)?, Some(false)); Ok(()) });
    let before = caller.cumulative_receipt();
    caller.turn(TURN, |cx| { let (step, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| resources.close_step_on_original(cx)); assert_eq!(heap.requested_bytes, 0); assert_eq!(heap.released_bytes, 0); assert_eq!(step.unwrap(), Some(false)); Ok(()) }).unwrap();
    assert_eq!(caller.cumulative_receipt(), before);
    assert_eq!(prepared.row(0).unwrap().loan.loan(TURN).unwrap_err(), Mesh3dFault::Closing);
    caller.cancel().cancel();
    for _ in 0..4096 { if first.terminal_is_empty() { break; } paid_context(&mut caller, |cx| { first.close_step_on_original(cx)?; Ok(()) }); }
    assert!(first.terminal_is_empty());
    assert_eq!(lease.with_original_page(0, |page| page.as_ptr() as usize).unwrap(), original_page);
    for _ in 0..4096 { if prepared.terminal_is_empty() { break; } paid_context(&mut caller, |cx| { prepared.close_step_on_original(cx)?; Ok(()) }); }
    assert!(prepared.terminal_is_empty());
    for _ in 0..4096 { if resources.terminal_is_empty() { break; } paid_context(&mut caller, |cx| { resources.close_step_on_original(cx)?; Ok(()) }); }
    assert!(resources.terminal_is_empty());
    assert!(mesh3d_terminal_is_empty(lease));
    let spent = caller.cumulative_receipt();
    let roots = caller.born_bytes();
    assert_eq!(spent.retained_capacity_bytes - roots, spent.released_bytes);
    let (_, heap) = semio_framework_trace::observe_heap_allocations_on_this_thread(|| drop((first, prepared, resources, row, source)));
    assert_eq!((heap.requested_bytes, heap.released_bytes), (0, 0));
    caller.retire(TURN);
    println!("[DEBUG] native same-arena original pages/paid aliases/canceled complete row-page close/one finite caller/Drop0");
}

#[test]
fn native_mesh_receiver_same_canonical_pages_all_labels_and_paid_destination_close(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🎬️scene/📐️math/📦️resource/📥️receiving/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let root:serde_json::Value=serde_json::from_str(include_str!("../../../../🎬️scene/📐️math/📦️resource/🧫️fixtures/🔣️.json")).unwrap();let policy=&root["caller"];let mut caller=OriginalJobFixtureCaller::admit(OperationId(941),Generation(17),grant(&policy["stock"]),clock,policy["deadline"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap());let raw:Vec<u8>=fixture["geometry"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();let pointer=raw.as_ptr();let identity=Mesh3dResourceIdentity{operation:941,generation:17,revision:23,cache_epoch:2,source_slot:1};let schema=Mesh3dSchema{vertices:3,indices:3,face_ids:1,vertex_ids:3,edges:1,edge_ids:1,uvs:0,colors:0,surface_uvs:[0;4],tangents:0};
 let(zero,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||Mesh3dResourceReceiving::admit(identity,schema,[2;3],raw.len(),RetainedCloneGrant{maximum_items:0,..TURN}));assert!(zero.unwrap().is_none());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let mut receiver=paid(&mut caller,|grant|Mesh3dResourceReceiving::admit(identity,schema,[2;3],raw.len(),grant).unwrap().unwrap());
 for turn in 0..10000{if receiver.is_ready(){break}let progress=paid(&mut caller,|grant|{let progress=receiver.advance(grant).unwrap();(progress,progress)});if progress==RetainedCloneProgress::default(){if let Some((domain,group,offset))=receiver.label_frontier(){let label=fixture["labels"][domain][group].as_str().unwrap().as_bytes();paid(&mut caller,|grant|{let step=receiver.receive_label(domain,group,offset,label.len(),&label[offset..],grant).unwrap();((),step.progress)});}else{let offset=receiver.geometry_offset();paid(&mut caller,|grant|{let step=receiver.receive_geometry(offset,&raw[offset..],grant).unwrap();((),step.progress)});}}assert_eq!(raw.as_ptr(),pointer);assert!(turn+1<10000)}assert!(receiver.is_ready());let mut owner=paid(&mut caller,|grant|receiver.take_ready(grant).unwrap());assert!(receiver.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(receiver));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));let lease=owner.lease().unwrap();let received=lease.with_original_page(0,|page|page.to_vec()).unwrap();assert_eq!(received,raw);assert_eq!(lease.original_component_counts().unwrap(),[2;3]);for domain in 0..3{for group in 0..2{lease.with_original_component_label(domain,group,|label|assert_eq!(label,fixture["labels"][domain][group].as_str().unwrap())).unwrap()}}assert_eq!(lease.component_label(Mesh3dField::FaceIds,0).unwrap(),Some(9001));assert_eq!(lease.component_label(Mesh3dField::EdgeIds,0).unwrap(),Some(8001));assert_eq!(lease.component_label(Mesh3dField::VertexIds,0).unwrap(),Some(7001));for turn in 0..10000{if owner.terminal_is_empty(){break}paid(&mut caller,|grant|{let step=owner.close_step(grant).unwrap();((),step.progress)});assert!(turn+1<10000)}assert!(owner.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));caller.retire(TURN);eprintln!("[DEBUG] nativeMeshReceiver sameCanonicalSource=true finiteCallerAndSystemConservation=true allVefAndNonemptyUtf8Labels=true destinationPaidClose=true terminalDrop0=true");
}

#[test]
fn native_mesh_receiver_cancelled_ready_destination_cannot_escape_its_paid_close(){
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../🎬️scene/📐️math/📦️resource/📥️receiving/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let root:serde_json::Value=serde_json::from_str(include_str!("../../../../🎬️scene/📐️math/📦️resource/🧫️fixtures/🔣️.json")).unwrap();let policy=&root["caller"];let mut caller=OriginalJobFixtureCaller::admit(OperationId(941),Generation(17),grant(&policy["stock"]),clock,policy["deadline"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap());let raw:Vec<u8>=law["geometry"].as_array().unwrap()[..84].iter().map(|byte|byte.as_u64().unwrap()as u8).collect();let identity=Mesh3dResourceIdentity{operation:941,generation:17,revision:24,cache_epoch:3,source_slot:2};let mut receiver=paid(&mut caller,|grant|Mesh3dResourceReceiving::admit(identity,Mesh3dSchema::triangle_mesh(3,3),[0;3],raw.len(),grant).unwrap().unwrap());for turn in 0..10000{if receiver.is_ready(){break}let progress=paid(&mut caller,|grant|{let progress=receiver.advance(grant).unwrap();(progress,progress)});if progress==RetainedCloneProgress::default(){let offset=receiver.geometry_offset();paid(&mut caller,|grant|{let step=receiver.receive_geometry(offset,&raw[offset..],grant).unwrap();((),step.progress)});}assert!(turn+1<10000)}assert!(receiver.is_ready());let(zero,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||receiver.close_step(RetainedCloneGrant{maximum_items:0,..TURN}));assert_eq!(zero.unwrap().progress,RetainedCloneProgress::default());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(receiver.is_ready());paid(&mut caller,|grant|{let step=receiver.close_step(grant).unwrap();((),step.progress)});assert!(!receiver.is_ready());assert!(receiver.take_ready(TURN).is_none());for turn in 0..10000{if receiver.terminal_is_empty(){break}paid(&mut caller,|grant|{let step=receiver.close_step(grant).unwrap();((),step.progress)});assert!(turn+1<10000)}assert!(receiver.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(receiver));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));caller.retire(TURN);eprintln!("[DEBUG] nativeMeshReceiverCancellation deniedCloseSystem0=true readyOwnerRetained=true noClosingOwnerEscape=true paidDestinationAndSourceClose=true terminalDrop0=true");
}

#[test]
fn native_mesh_message_fixed_windows_bind_patch_resource_and_original_donor_before_lifting(){
 use semio_framework_ui_runtime::{UiNativeMeshPatchIdentity,UiNativeMeshMessageBinding,UiNativeMeshMessage,UiNativeMeshMessageContent,UiNativeMeshDescriptor,UiNativeMeshMessageWriting,UiNativeMeshMessageReceiving,UI_NATIVE_MESH_MESSAGE_BYTES};
 let corpus:serde_json::Value=serde_json::from_str(include_str!("../📤️transport/📨️message/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let root:serde_json::Value=serde_json::from_str(include_str!("../../../../🎬️scene/📐️math/📦️resource/🧫️fixtures/🔣️.json")).unwrap();let policy=&root["caller"];let binding=&corpus["binding"];let field=|index:usize|binding[index].as_u64().unwrap();let id=Mesh3dResourceIdentity{operation:field(6),generation:field(7),revision:field(8),cache_epoch:field(9),source_slot:field(10)as usize};let data=&corpus["descriptor"];let count=|index:usize|data[index].as_u64().unwrap()as u32;let descriptor=UiNativeMeshDescriptor{identity:id,geometry_bytes:count(0)as usize,component_counts:[count(1)as usize,count(2)as usize,count(3)as usize],schema:Mesh3dSchema{vertices:count(4),indices:count(5),face_ids:count(6),vertex_ids:count(7),edges:count(8),edge_ids:count(9),uvs:count(10),colors:count(11),surface_uvs:[count(12),count(13),count(14),count(15)],tangents:count(16)}};let source=UiNativeMeshMessage{binding:UiNativeMeshMessageBinding{patch:UiNativeMeshPatchIdentity{activation_generation:field(0),instance_id:field(1)as u32,guest_lifetime:field(2),patch_sequence:field(3),revision:field(4)},resource_ordinal:field(5)as usize,source:id},sequence:field(11),content:UiNativeMeshMessageContent::Descriptor{descriptor,surface_bytes:count(17)as usize,mesh_key_bytes:count(18)as usize}};
 let mut caller=OriginalJobFixtureCaller::admit(OperationId(941),Generation(17),grant(&policy["stock"]),clock,policy["deadline"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap());let mut writing=paid(&mut caller,|grant|UiNativeMeshMessageWriting::admit(source,grant).unwrap().unwrap());let mut wire=[0u8;UI_NATIVE_MESH_MESSAGE_BYTES];let pointer=wire.as_ptr();let mut page=[0u8;64];for cut in 0..6{let offset=writing.offset();let before=caller.cumulative_receipt();paid(&mut caller,|grant|{let result=writing.copy_window_step(&mut page,RetainedCloneGrant{maximum_items:0,..grant});assert_eq!(result.0,0);assert_eq!(writing.offset(),offset);((),result.1)});assert_eq!(caller.cumulative_receipt(),before);let length=paid(&mut caller,|grant|writing.copy_window_step(&mut page,grant));wire[offset..offset+length].copy_from_slice(&page[..length]);assert_eq!(length,if cut==5{48}else{64});}assert!(writing.complete());assert!(UiNativeMeshMessage::is_mesh_message(&wire));let mut receiving=UiNativeMeshMessageReceiving::default();let before=caller.cumulative_receipt();paid(&mut caller,|grant|{let result=receiving.receive_step(&wire,RetainedCloneGrant{maximum_copy_bytes:63,..grant}).unwrap();assert!(result.0.is_none());((),result.1)});assert_eq!(caller.cumulative_receipt(),before);assert_eq!(receiving.offset(),0);let received=loop{if let Some(message)=paid(&mut caller,|grant|receiving.receive_step(&wire,grant).unwrap()){break message}assert_eq!(wire.as_ptr(),pointer)};assert_eq!(received,source);assert_ne!(received.binding.resource_ordinal,received.binding.source.source_slot);let mut malformed=source.canonical_words().unwrap();malformed[14]=1;assert_eq!(UiNativeMeshMessage::from_canonical_words(malformed),Err(Mesh3dFault::Schema));assert_eq!(semio_framework_trace::observe_heap_allocations_on_this_thread(||drop((writing,receiving))).1.released_bytes,0);caller.retire(TURN);eprintln!("[DEBUG] actual Native Mesh Message canonical46words descriptor source donated only in64byte funded windows; original patch/resource identity stable before lift; ordinal2 distinct slot7; zero-copy/zero-work refused original donor retained; no destination or Ack fabricated");
}

#[test]
fn native_mesh_message_receiver_preserves_whole_donor_and_returns_source_with_live_destination(){
 use semio_framework_ui_runtime::{UiNativeMeshPatchIdentity,UiNativeMeshMessageBinding,UiNativeMeshMessage,UiNativeMeshMessageContent,UiNativeMeshDescriptor,UiNativeMeshChannel,UiNativeMeshReceiving};
 let law:serde_json::Value=serde_json::from_str(include_str!("../../../../🎬️scene/📐️math/📦️resource/📥️receiving/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let root:serde_json::Value=serde_json::from_str(include_str!("../../../../🎬️scene/📐️math/📦️resource/🧫️fixtures/🔣️.json")).unwrap();let policy=&root["caller"];let mut caller=OriginalJobFixtureCaller::admit(OperationId(941),Generation(17),grant(&policy["stock"]),clock,policy["deadline"].as_u64().unwrap(),policy["fuel"].as_u64().unwrap());let raw:Vec<u8>=law["geometry"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect();let identity=Mesh3dResourceIdentity{operation:941,generation:17,revision:23,cache_epoch:8,source_slot:7};let schema=Mesh3dSchema{vertices:3,indices:3,face_ids:1,vertex_ids:3,edges:1,edge_ids:1,uvs:0,colors:0,surface_uvs:[0;4],tangents:0};let binding=UiNativeMeshMessageBinding{patch:UiNativeMeshPatchIdentity{activation_generation:17,instance_id:3,guest_lifetime:11,patch_sequence:19,revision:29},resource_ordinal:2,source:identity};let surface="native:surface";let key="solid:α";let descriptor=UiNativeMeshDescriptor{identity,schema,geometry_bytes:raw.len(),component_counts:[2;3]};let first=UiNativeMeshMessage{binding,sequence:1,content:UiNativeMeshMessageContent::Descriptor{descriptor,surface_bytes:surface.len(),mesh_key_bytes:key.len()}};let mut receiving=None;paid_context(&mut caller,|cx|{receiving=UiNativeMeshReceiving::admit(first,cx)?;Ok(())});let mut receiving=receiving.unwrap();let mut sequence=1u64;
 let mut feed=|content:UiNativeMeshMessageContent,receiving:&mut UiNativeMeshReceiving,caller:&mut OriginalJobFixtureCaller|{let original=UiNativeMeshMessage{binding,sequence,content};let mut donor=Some(original);let before=caller.cumulative_receipt();caller.turn(TURN,|cx|cx.with_retained_loan(RetainedCloneGrant{maximum_items:0,..TURN},|same|{assert!(!receiving.receive_step(&mut donor,same)?);Ok(())})).unwrap();assert_eq!(caller.cumulative_receipt(),before);assert_eq!(donor,Some(original));for turn in 0..10000{if donor.is_none(){break}paid_context(caller,|cx|{receiving.receive_step(&mut donor,cx)?;Ok(())});assert!(donor.is_none()||donor==Some(original));assert!(turn+1<10000)}assert!(donor.is_none());sequence+=1;};
 feed(first.content,&mut receiving,&mut caller);for(channel,text)in[(UiNativeMeshChannel::Surface,surface.as_bytes()),(UiNativeMeshChannel::MeshKey,key.as_bytes()),(UiNativeMeshChannel::Geometry,raw.as_slice())]{for(offset,window)in text.chunks(64).enumerate(){let mut bytes=[0;64];bytes[..window.len()].copy_from_slice(window);feed(UiNativeMeshMessageContent::Window{channel,offset:offset*64,total_bytes:text.len(),bytes,length:window.len()},&mut receiving,&mut caller)}}for domain in 0..3{for group in 0..2{let text=law["labels"][domain][group].as_str().unwrap().as_bytes();for(offset,window)in text.chunks(64).enumerate(){let mut bytes=[0;64];bytes[..window.len()].copy_from_slice(window);feed(UiNativeMeshMessageContent::Window{channel:UiNativeMeshChannel::ComponentLabel{domain,group},offset:offset*64,total_bytes:text.len(),bytes,length:window.len()},&mut receiving,&mut caller)}}}feed(UiNativeMeshMessageContent::ResourceComplete,&mut receiving,&mut caller);assert!(receiving.is_complete());assert_eq!(receiving.destination().unwrap().surface.as_str(),surface);assert_eq!(receiving.destination().unwrap().mesh_key.as_str(),key);let mut displayed=paid(&mut caller,|grant|receiving.destination().unwrap().owner.loan(grant).unwrap().unwrap());let lease=displayed.lease().unwrap();assert_eq!(lease.with_original_page(0,|page|page.to_vec()).unwrap(),raw);for domain in 0..3{for group in 0..2{lease.with_original_component_label(domain,group,|text|assert_eq!(text,law["labels"][domain][group].as_str().unwrap())).unwrap()}}for _ in 0..16{if receiving.source_is_returned(){break}paid_context(&mut caller,|cx|{receiving.return_source_step(cx)?;Ok(())});}assert!(receiving.source_is_returned());assert!(receiving.destination().is_some());assert_eq!(displayed.lease().unwrap(),lease);assert_eq!(lease.with_original_page(0,|page|page.len()).unwrap(),raw.len());paid(&mut caller,|grant|{let step=displayed.return_step(grant).unwrap();((),step.progress)});assert!(displayed.terminal_is_empty());for turn in 0..10000{if receiving.terminal_is_empty(){break}paid_context(&mut caller,|cx|{receiving.close_step(cx)?;Ok(())});assert!(turn+1<10000)}assert!(receiving.terminal_is_empty());let(_,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop((displayed,receiving)));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));caller.retire(TURN);eprintln!("[DEBUG] actual native fixed Message receiver whole donor retained on denied loan; independent canonical geometry/all nonempty labels and exact names; genuine source alias returns while destination/display loan stays alive; paid destination close Drop0");
}
