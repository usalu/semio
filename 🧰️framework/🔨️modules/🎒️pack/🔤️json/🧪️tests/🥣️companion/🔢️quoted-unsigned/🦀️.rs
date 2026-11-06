//! 🔢️ Borrowed quoted unsigned values preserve exact decimal text and caller output custody.
use super::*;

struct Quoted(u128);
impl JsonWriteSource for Quoted{
 fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{assert!(path.is_empty());Ok(JsonWriteNode::QuotedUnsigned(self.0))}
 fn object_key_at_path(&self,_:&[usize],_:usize)->Result<&str,ValueError>{unreachable!()}
}

#[test]
fn quoted_unsigned_source_sink_matches_independent_json(){
 let fixture:serde_json::Value=serde_json::from_str(crate::text("🧪️tests/🧫️fixtures/🔢️quoted-unsigned.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let text=row.as_str().unwrap();let source=Quoted(text.parse().unwrap());let expected=serde_json::to_string(text).unwrap();let mut actual=Vec::new();let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new_retained(&mut allow);
  let written=write_json_source_into(&source,1024,1,1,&mut |bytes:&[u8],_:&mut semio_framework_value::NativeEncodeControl<'_>|{actual.extend_from_slice(bytes);Ok::<_,ValueError>(())},&mut control).unwrap();
  assert_eq!(written as usize,actual.len());assert_eq!(actual,expected.as_bytes());assert_eq!(serde_json::from_slice::<String>(&actual).unwrap(),text);
 }
}

struct Float32(f32);
impl JsonWriteSource for Float32{
 fn node_at_path(&self,path:&[usize])->Result<JsonWriteNode<'_>,ValueError>{assert!(path.is_empty());Ok(JsonWriteNode::Float32(self.0))}
 fn object_key_at_path(&self,_:&[usize],_:usize)->Result<&str,ValueError>{unreachable!()}
}

#[test]
fn float32_source_sink_matches_independent_serde(){
 let fixture:serde_json::Value=serde_json::from_str(crate::text("🧪️tests/🧫️fixtures/🔢️float32-sink.json")).unwrap();
 for row in fixture["bits"].as_array().unwrap(){let source=Float32(f32::from_bits(u32::from_str_radix(row.as_str().unwrap(),16).unwrap()));let expected=serde_json::to_string(&source.0).unwrap();let mut actual=Vec::new();let mut allow=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new_retained(&mut allow);let written=write_json_source_into(&source,1024,1,1,&mut |bytes:&[u8],_:&mut semio_framework_value::NativeEncodeControl<'_>|{actual.extend_from_slice(bytes);Ok::<_,ValueError>(())},&mut control).unwrap();assert_eq!(written as usize,actual.len());assert_eq!(actual,expected.as_bytes());}
}
