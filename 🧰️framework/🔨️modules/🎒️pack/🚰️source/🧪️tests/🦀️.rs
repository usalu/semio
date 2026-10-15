//! 🧪️ Portable random access and append laws against the system Cursor oracle.
use semio_framework_pack_source::{PackSource,PackSink};
use semio_framework_pack_error::PackRefusal;
use std::io::{Cursor,Read,Seek,SeekFrom};
fn bytes(value:&serde_json::Value)->Vec<u8>{let value=value.as_str().unwrap();(0..value.len()).step_by(2).map(|index|u8::from_str_radix(&value[index..index+2],16).unwrap()).collect()}
fn offset(value:&serde_json::Value)->u64{value.as_str().unwrap().parse().unwrap()}
#[semio_framework_async_macros::async_test]
async fn portable_pack_byte_source_and_sink_match_original_cursor(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();
 for row in fixture["reads"].as_array().unwrap(){let input=bytes(&row["input"]);let offset=offset(&row["offset"]);let mut output=vec![0;row["length"].as_u64().unwrap()as usize];let result=input.read_at(offset,&mut output).await;match row["truncatedAt"].as_str(){Some(expected)=>assert_eq!(result,Err(PackRefusal::Truncated(expected.parse().unwrap()))),None=>{let count=*result.as_ref().unwrap();output.truncate(count);assert_eq!(output,bytes(&row["output"]));let mut cursor=Cursor::new(&input);cursor.seek(SeekFrom::Start(offset)).unwrap();let mut oracle=vec![0;row["length"].as_u64().unwrap()as usize];let count=cursor.read(&mut oracle).unwrap();oracle.truncate(count);assert_eq!(output,oracle);}}assert_eq!(PackSource::len(&input).await,input.len()as u64);let mut slice_output=vec![0;row["length"].as_u64().unwrap()as usize];assert_eq!(input.as_slice().read_at(offset,&mut slice_output).await,result);}
 for row in fixture["exact"].as_array().unwrap(){let input=bytes(&row["input"]);let mut output=vec![0;row["length"].as_u64().unwrap()as usize];let result=input.read_exact_at(offset(&row["offset"]),&mut output).await;let expected=bytes(&row["output"]);assert_eq!(&output[..expected.len()],expected);match row["truncatedAt"].as_str(){Some(expected)=>assert_eq!(result,Err(PackRefusal::Truncated(expected.parse().unwrap()))),None=>assert!(result.is_ok())}}
 for row in fixture["writes"].as_array().unwrap(){let mut output=Vec::new();for chunk in row["chunks"].as_array().unwrap(){PackSink::write_all(&mut output,&bytes(chunk)).await.unwrap();}PackSink::flush(&mut output).await.unwrap();assert_eq!(PackSink::position(&output).await,output.len()as u64);assert_eq!(output,bytes(&row["output"]));}
 println!("[DEBUG] Actual pack byte owner portable reads/exact truncation/append match Cursor and closed corpus debug_assertions={}",cfg!(debug_assertions));
}
