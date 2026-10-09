use super::*;
#[test]
fn original_owned_actor_layout_retains_borrowed_vm_and_visits_without_allocation(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../🧫️fixtures/🔣️.json")).unwrap();
    let core=CoreInstance::instantiate(std::sync::Arc::new(CoreModule::parse(&fixture["coreModule"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect::<Vec<_>>()).unwrap())).unwrap();
    let actor=OwnedSemioInstance{component_fingerprint:fixture["componentFingerprint"].as_u64().unwrap(),core};let expected=actor.checkpoint();
    let mut observe=|_:semio_framework_value::native_encoding::NativeEncodeProgress|true;let mut control=semio_framework_value::NativeEncodeControl::new(fixture["originalMaximumBytes"].as_u64().unwrap()as usize,&mut observe);
    let(layout,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||actor.checkpoint_layout(&mut control));let layout=layout.unwrap();assert!(!physical.overflowed);assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(control.owned_bytes(),0);
    let mut count=0;let(result,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||layout.visit(&mut |bytes|{assert_eq!(&expected[count..count+bytes.len()],bytes);count+=bytes.len();control.advance(bytes.len())}));result.unwrap();assert!(!physical.overflowed);assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(count,expected.len());assert_eq!(control.owned_bytes(),0);
}
