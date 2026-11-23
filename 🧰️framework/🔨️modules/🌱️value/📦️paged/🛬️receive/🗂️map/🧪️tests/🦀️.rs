//! 🗂️ The neutral map matrix exercises real original key indexing, typed row receiving and cancellation custody.
use semio_framework_value::paged::PagedMap;
use semio_framework_value_native_owned::native_decoding::{NativeReceive,NativeReceiver,owned::tree::NativeValueSource};
use semio_framework_value_testing_allocator as _;
use semio_framework_value::DslValue;
use super::{bank,driver,default_receiver::DefaultReceiver};
#[test]fn original_map_keys_receive_in_authored_order_and_reject_duplicates(){let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();for row in fixture["cases"].as_array().unwrap(){let names:std::collections::BTreeSet<_>=row["entries"].as_array().unwrap().iter().map(|pair|pair[0].as_str().unwrap()).collect();let accepted=row["accepted"].as_bool().unwrap();assert_eq!(names.len()==row["entries"].as_array().unwrap().len(),accepted);for cut in [Some(0),Some(1),Some(5),Some(30),None]{let entries=row["entries"].as_array().unwrap().iter().map(|pair|(pair[0].as_str().unwrap().to_string(),DslValue::from(pair[1].clone()))).collect();let mut bank=bank();let source=driver::admit(&mut bank,DslValue::Object(entries));let receiver=<PagedMap<u32,4096> as NativeReceive<DslValue>>::receiver();
 let result=driver::lifecycle(receiver,source,&mut bank,4096,cut,|output:&PagedMap<u32,4096>|{assert_eq!(output.len(),row["entries"].as_array().unwrap().len());for(index,(key,value))in output.iter().enumerate(){assert_eq!(key.to_string_owner(),row["entries"][index][0].as_str().unwrap());assert_eq!(*value,row["entries"][index][1].as_u64().unwrap()as u32)}});
 if accepted{assert_eq!(result.unwrap(),cut.is_none())}else if cut.is_none(){assert!(result.is_err())}}}}

#[test]
fn shared_original_paged_map_supports_erased_receiving_and_funded_default(){
 use semio_framework_value_native_owned::native_decoding::owned::NativeDefault;
 fn supported<T:Sync+NativeReceive<dyn NativeValueSource>+NativeDefault>(){}
 supported::<PagedMap<u32,4096>>();
 let mut bank=bank();let source=driver::admit(&mut bank,DslValue::Null);let mut cursor=DefaultReceiver(<PagedMap<u32,4096> as NativeDefault>::default_cursor());
 let map:PagedMap<u32,4096>=driver::receive_cut(&mut cursor,source.borrow(),&mut bank,4096,None).unwrap().expect("uncut receive publishes");assert_eq!(map.len(),0);driver::retire(map,&mut bank);
 driver::close::<DslValue,PagedMap<u32,4096>,_>(&mut cursor,&mut bank,4096);assert!(NativeReceiver::<DslValue,PagedMap<u32,4096>>::terminal_is_empty(&cursor));drop(cursor);driver::close_source(source,&mut bank);
}
