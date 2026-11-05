#[cfg(test)]
#[global_allocator]
static PUBLICATION_WIRE_HEAP:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;

use semio_framework_plugin_result_page::result_page_wire::*;
use semio_framework::kernel::UiDirtyScope;
use semio_framework_value::NativeEncodeControl;

fn encode(source:&PublicationWire<'_>,expected:&str){
    let mut output=[0u8;4096];
    let mut observer=|_|true;
    let mut control=NativeEncodeControl::new(4096,&mut observer);
    let (written,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.write_into(&mut output,8,4096,&mut control));
    written.result.unwrap();
    assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0),"borrowed writer creates no heap owner");
    assert_eq!(&output[..written.written],expected.as_bytes());
}

#[test]
fn borrowed_publication_wire_matches_neutral_corpus_and_two_existing_json_oracles(){
    let fixture:serde_json::Value=serde_json::from_str(include_str!("../../../../🧪️tests/📃️page-admission/🧫️fixtures/🔣️.json")).unwrap();
    for row in fixture["cases"].as_array().unwrap(){
        if row["expected"]["state"]["frame"].is_null(){continue}
        let value=&row["payload"]["value"];
        let expected=row["expected"]["state"]["frame"].as_str().unwrap();
        let kind=row["payload"]["kind"].as_str().unwrap();
        if row["cancelPayload"].is_object(){continue}
        match kind{
            "laneReceipt"=>{
                let before=value["generationBefore"].as_u64().unwrap();let after=value["generationAfter"].as_u64().unwrap();
                encode(&PublicationWire::LaneReceipt{generation_before:before,generation_after:after},expected);
                let receipt=semio_framework_os_store_one_item::LaneItemReceipt{generation_before:before,generation_after:after};
                assert_eq!(semio_framework_pack_json::to_json_string(&receipt),expected);
                assert_eq!(serde_json::to_string(&receipt).unwrap(),expected);
            },
            "accepted"=>{
                let count=value[1].as_u64().unwrap();encode(&PublicationWire::Accepted(count),expected);
                assert_eq!(semio_framework_pack_json::to_json_string(&("accepted",count)),expected);
                assert_eq!(serde_json::to_string(&("accepted",count)).unwrap(),expected);
            },
            "ui"=>{
                let scope:UiDirtyScope=serde_json::from_value(value.clone()).unwrap();encode(&PublicationWire::Ui(&scope),expected);
                assert_eq!(semio_framework_pack_json::to_json_string(&scope),expected);
                assert_eq!(serde_json::to_string(&scope).unwrap(),expected);
            },
            "download"=>{
                let name=value[0].as_str().unwrap();let mime=value[1].as_str().unwrap();let encoding=value[2].as_str();let bytes=value[3].as_u64().unwrap();
                encode(&PublicationWire::Download{filename:name,mime,encoding,bytes},expected);
                assert_eq!(semio_framework_pack_json::to_json_string(&semio_framework_value::DslValue::from(value)),expected);
                assert_eq!(serde_json::to_string(&(name,mime,encoding,bytes)).unwrap(),expected);
            },
            "fault"|"terminal"=>{
                let mut output=[0;4096];let mut observer=|_|true;let mut control=NativeEncodeControl::new(4096,&mut observer);
                let (written,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||write_publication_raw_into(value.as_str().unwrap().as_bytes(),&mut output,&mut control));
                written.result.unwrap();assert_eq!(written.work,value.as_str().unwrap().len().div_ceil(64) as u64);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(&output[..written.written],expected.as_bytes());
            },
            _=>panic!("unknown finite publication payload"),
        }
    }
}

#[test]
fn borrowed_publication_wire_refuses_exact_output_depth_items_and_cancel_without_owned_fallback(){
    let scope=UiDirtyScope::Partial{window_bodies:vec!["雪".repeat(1<<20)],panel_bodies:vec!["details".into()],utilities:true,tools:false,engagements:false,measures:false,labels:true};
    let source=PublicationWire::Ui(&scope);
    for (length,depth,items)in[(32,8,4096),(4096,0,4096),(4096,8,0)]{
        let mut output=[0xa5;4096];let mut observer=|_|true;let mut control=NativeEncodeControl::new(4096,&mut observer);
        let (written,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.write_into(&mut output[..length],depth,items,&mut control));
        assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0),"refused writer creates no heap owner");
        assert!(written.result.is_err());assert!(written.written<=length);assert!(output[length..].iter().all(|byte|*byte==0xa5));
    }
    let mut output=[0;4096];let mut cancelled=|_|false;let mut control=NativeEncodeControl::new(4096,&mut cancelled);
    let (written,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||source.write_into(&mut output,8,4096,&mut control));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(written.result.is_err());assert_eq!(written.written,0);
}

#[test]
fn borrowed_publication_raw_refusal_preserves_accepted_prefix(){
    let source=[7;129];let mut output=[0;65];let mut observer=|_|true;let mut control=NativeEncodeControl::new(4096,&mut observer);
    let (written,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||write_publication_raw_into(&source,&mut output,&mut control));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert!(written.result.is_err());assert_eq!(written.work,1);assert_eq!(written.written,64);assert_eq!(&output[..64],&source[..64]);assert_eq!(output[64],0);
    let mut cancelled=|_|false;let mut control=NativeEncodeControl::new(4096,&mut cancelled);
    let written=write_publication_raw_into(&source,&mut output,&mut control);assert!(written.result.is_err());assert_eq!((written.written,written.work),(0,0));
}
