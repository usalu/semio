//! 📬️ Reply metadata borrows at most twelve bytes and never allocates or validates the frame body.
#[global_allocator]
static ALLOCATOR:semio_framework_trace::HeapWitness=semio_framework_trace::HeapWitness;
use semio_os_channel::{app_frame_reply_prefix,AppFrame,encode_app_frame};
#[test]
fn real_reply_prefix_matches_schema_and_actual_binary_encoder_without_heap(){
 let law:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();for row in law["cases"].as_array().unwrap(){let bytes:Vec<_>=row["bytes"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect();let(got,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app_frame_reply_prefix(&bytes));assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!(got.is_some(),row["valid"].as_bool().unwrap());if let Some(got)=got{assert_eq!(got.in_reply_to.map(|v|v.to_string()),row["replyTo"].as_str().map(str::to_owned));assert_eq!(got.done,row["done"].as_bool().unwrap());assert_eq!(got.transaction_terminal,row["transaction"].as_bool().unwrap());}}
 let frames=[AppFrame::Done{in_reply_to:u64::MAX},AppFrame::Error{in_reply_to:Some(42),fault:vec![7;4096],report:vec![8;8192]},AppFrame::TransactionRolledBack{txn_id:"transaction".into()},AppFrame::MergeReport{in_reply_to:None,report:vec![1;4096]}];for(frame,expected)in frames.into_iter().zip([(Some(u64::MAX),true,false),(Some(42),false,false),(None,false,true),(None,false,false)]){let bytes=semio_framework_async::block_on(encode_app_frame(&frame));let(got,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||app_frame_reply_prefix(&bytes).unwrap());assert_eq!((heap.requested_bytes,heap.released_bytes),(0,0));assert_eq!((got.in_reply_to,got.done,got.transaction_terminal),expected);}
}
