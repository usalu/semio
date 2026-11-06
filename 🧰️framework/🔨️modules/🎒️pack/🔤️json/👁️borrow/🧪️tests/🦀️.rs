use semio_framework_pack_json::{JsonNativeSource,JsonNativeBorrow,JsonBorrowNoContext,JsonBorrowContext,JsonBorrowNode,JsonBorrowKey,JsonWriteNode,JsonBorrowError,json_borrow_absent,JsonBorrowedSinkCursor};
use semio_framework_value::RetainedCloneGrant;
#[derive(serde::Serialize,semio_framework_value::NativeJsonBorrow)]
#[serde(rename_all="camelCase")]
#[value(rename_all="camelCase")]
struct Record{name:String,items:Vec<u8>,active:bool,#[serde(skip_serializing_if="Option::is_none")]#[value(skip_serializing_if="Option::is_none")]optional:Option<String>}
#[derive(serde::Serialize,semio_framework_value::NativeJsonBorrow)]
#[serde(rename_all="camelCase",rename_all_fields="camelCase")]
#[value(rename_all="camelCase",rename_all_fields="camelCase")]
enum External{Row{id:u64},Idle}
#[derive(serde::Serialize,semio_framework_value::NativeJsonBorrow)]
#[serde(rename_all="camelCase",tag="kind")]
#[value(rename_all="camelCase",tag="kind")]
enum Internal{Row{id:u64},Idle}
#[derive(serde::Serialize,semio_framework_value::NativeJsonBorrow)]
#[serde(transparent)]
#[value(transparent)]
struct Transparent(Vec<u8>);

fn write(source:&impl semio_framework_pack_json::JsonWriteSource)->String{
 let mut cursor=JsonBorrowedSinkCursor::empty(32).unwrap();let mut observer=|_|true;let mut control=semio_framework_value::NativeEncodeControl::new(65536,&mut observer);let mut output=Vec::new();
 for _ in 0..65536{if cursor.is_complete(){break}let mut bytes=[0;7];let d=cursor.next_demands(bytes.len(),4096).unwrap();let g=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth};let(step,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.step(source,&mut bytes,g,1,&mut control));let step=step.result.unwrap();assert!(step.progress.fits(g));assert_eq!((heap.requested_bytes,heap.released_bytes),(step.progress.retained_capacity_bytes,step.progress.released_bytes));output.extend_from_slice(&bytes[..step.written_bytes]);}
 assert!(cursor.is_complete());super::close(&mut cursor);String::from_utf8(output).unwrap()
}
fn case(value:&(impl JsonNativeBorrow+serde::Serialize),expected:&str){let context=JsonBorrowNoContext;let source=JsonNativeSource{root:value,context:&context};assert_eq!(write(&source),expected);assert_eq!(serde_json::to_string(value).unwrap(),expected);}
struct Text;
impl semio_framework_value::paged::Utf8Text for Text{fn text_bytes(&self)->usize{5}fn text_chunk_count(&self)->usize{3}fn text_chunk(&self,index:usize)->Option<&str>{["a","雪","\n"].get(index).copied()}}
struct Backing(Text);
impl JsonBorrowContext for Backing{
 fn node(&self,key:JsonBorrowKey)->Result<JsonWriteNode<'_>,JsonBorrowError>{if key!=JsonBorrowKey([7,1,1,0]){return Err(json_borrow_absent())}Ok(JsonWriteNode::NativeString(&self.0))}
 fn child(&self,_:JsonBorrowKey,_:usize)->Result<JsonBorrowNode<'_>,JsonBorrowError>{Err(json_borrow_absent())}
 fn key(&self,_:JsonBorrowKey,_:usize)->Result<&str,JsonBorrowError>{Err(json_borrow_absent())}
}
struct Handle(JsonBorrowKey);
impl JsonNativeBorrow for Handle{
 fn json_node<'a>(&'a self,mode:u8,context:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{if mode!=0{return Err(json_borrow_absent())}context.node(self.0)}
 fn json_child<'a>(&'a self,mode:u8,index:usize,context:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{if mode!=0{return Err(json_borrow_absent())}context.child(self.0,index)}
 fn json_key<'a>(&'a self,mode:u8,index:usize,context:&'a dyn JsonBorrowContext)->Result<&'a str,JsonBorrowError>{if mode!=0{return Err(json_borrow_absent())}context.key(self.0,index)}
 fn json_edge(&self,mode:u8,index:usize,cursor:usize,context:&dyn JsonBorrowContext)->Result<semio_framework_pack_json::JsonWriteEdge,JsonBorrowError>{if mode!=0{return Err(json_borrow_absent())}context.edge(self.0,index,cursor)}
}
#[test]
fn borrowed_native_derive_matches_shared_shapes_and_exact_physical_sink_receipts(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let cases=fixture["cases"].as_array().unwrap();
 case(&Record{name:"雪".into(),items:vec![0,255],active:true,optional:None},cases[0]["expected"].as_str().unwrap());case(&External::Row{id:7},cases[1]["expected"].as_str().unwrap());case(&External::Idle,cases[2]["expected"].as_str().unwrap());case(&Internal::Row{id:7},cases[3]["expected"].as_str().unwrap());case(&Transparent(vec![1,2,3]),cases[4]["expected"].as_str().unwrap());case(&Internal::Idle,"{\"kind\":\"idle\"}");
 let backing=Backing(Text);let root=Handle(JsonBorrowKey([7,1,1,0]));assert_eq!(write(&JsonNativeSource{root:&root,context:&backing}),cases[5]["expected"].as_str().unwrap());let foreign=Handle(JsonBorrowKey([7,1,2,0]));assert!(semio_framework_pack_json::JsonWriteSource::node_at_path(&JsonNativeSource{root:&foreign,context:&backing},&[]).is_err());
}

trait Extension:std::any::Any+Sized{
 const JSON:semio_framework_pack_json::JsonBorrowProjection=semio_framework_pack_json::JsonBorrowProjection{node:Self::project_node,child:Self::project_child,key:Self::project_key,edge:Self::project_edge};
 fn json_node<'a>(&'a self,m:u8,c:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>;
 fn json_child<'a>(&'a self,_:u8,_:usize,_:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{Err(json_borrow_absent())}
 fn json_key<'a>(&'a self,_:u8,_:usize,_:&'a dyn JsonBorrowContext)->Result<&'a str,JsonBorrowError>{Err(json_borrow_absent())}
 fn json_edge(&self,_:u8,index:usize,_cursor:usize,_:&dyn JsonBorrowContext)->Result<semio_framework_pack_json::JsonWriteEdge,JsonBorrowError>{Ok(semio_framework_pack_json::JsonWriteEdge{child:index,cursor:0})}
 fn project_edge(value:&dyn std::any::Any,m:u8,i:usize,cursor:usize,c:&dyn JsonBorrowContext)->Result<semio_framework_pack_json::JsonWriteEdge,JsonBorrowError>{value.downcast_ref::<Self>().ok_or_else(json_borrow_absent)?.json_edge(m,i,cursor,c)}
 fn project_node<'a>(value:&'a dyn std::any::Any,m:u8,c:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{value.downcast_ref::<Self>().ok_or_else(json_borrow_absent)?.json_node(m,c)}
 fn project_child<'a>(value:&'a dyn std::any::Any,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{value.downcast_ref::<Self>().ok_or_else(json_borrow_absent)?.json_child(m,i,c)}
 fn project_key<'a>(value:&'a dyn std::any::Any,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<&'a str,JsonBorrowError>{value.downcast_ref::<Self>().ok_or_else(json_borrow_absent)?.json_key(m,i,c)}
}
impl Extension for usize{fn json_node<'a>(&'a self,m:u8,c:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{JsonNativeBorrow::json_node(self,m,c)}}
semio_framework_value::native_json_codec!{
 #[borrowed_json(protocol="Extension")]
 #[value(rename_all="camelCase")]
 struct RetainedCloneGrant{maximum_items:usize,maximum_copy_bytes:usize,maximum_capacity_bytes:usize,maximum_release_bytes:usize,maximum_depth:usize}
}
struct ExternalRoot<'a>(&'a dyn std::any::Any,&'static semio_framework_pack_json::JsonBorrowProjection);
impl JsonNativeBorrow for ExternalRoot<'_>{
 fn json_node<'a>(&'a self,m:u8,c:&'a dyn JsonBorrowContext)->Result<JsonWriteNode<'a>,JsonBorrowError>{(self.1.node)(self.0,m,c)}
 fn json_child<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<JsonBorrowNode<'a>,JsonBorrowError>{(self.1.child)(self.0,m,i,c)}
 fn json_key<'a>(&'a self,m:u8,i:usize,c:&'a dyn JsonBorrowContext)->Result<&'a str,JsonBorrowError>{(self.1.key)(self.0,m,i,c)}
}
#[test]
fn external_codec_checks_real_native_shape_and_rejects_a_foreign_table_owner(){
 let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:2,maximum_capacity_bytes:3,maximum_release_bytes:4,maximum_depth:5};let root=ExternalRoot(&grant,&<RetainedCloneGrant as Extension>::JSON);let context=JsonBorrowNoContext;assert_eq!(write(&JsonNativeSource{root:&root,context:&context}),serde_json::to_string(&grant).unwrap());let foreign=String::from("foreign");let source=ExternalRoot(&foreign,&<RetainedCloneGrant as Extension>::JSON);assert!(JsonNativeBorrow::json_node(&source,0,&context).is_err());
}

struct Linked{edges:std::cell::Cell<usize>}
impl Linked{fn row(index:usize)->Result<(usize,u64,&'static str),JsonBorrowError>{match index{4=>Ok((1,9,"first")),1=>Ok((7,8,"second")),7=>Ok((usize::MAX,7,"third")),_=>Err(json_borrow_absent())}}}
impl JsonBorrowContext for Linked{
 fn node(&self,key:JsonBorrowKey)->Result<JsonWriteNode<'_>,JsonBorrowError>{match key.0{[8,1,1,0]=>Ok(JsonWriteNode::Array(3)),[8,1,2,0]=>Ok(JsonWriteNode::Object(3)),_=>Err(json_borrow_absent())}}
 fn child(&self,_:JsonBorrowKey,index:usize)->Result<JsonBorrowNode<'_>,JsonBorrowError>{Ok(JsonBorrowNode::Scalar(JsonWriteNode::Number(semio_framework_value::Number::UInt(Self::row(index)?.1))))}
 fn key(&self,_:JsonBorrowKey,index:usize)->Result<&str,JsonBorrowError>{Ok(Self::row(index)?.2)}
 fn edge(&self,_:JsonBorrowKey,index:usize,cursor:usize)->Result<semio_framework_pack_json::JsonWriteEdge,JsonBorrowError>{let child=if index==0{4}else{cursor};let next=Self::row(child)?.0;self.edges.set(self.edges.get()+1);Ok(semio_framework_pack_json::JsonWriteEdge{child,cursor:next})}
}
#[test]
fn linked_native_arrays_and_objects_advance_one_original_edge_per_member(){
 let backing=Linked{edges:std::cell::Cell::new(0)};let root=Handle(JsonBorrowKey([8,1,1,0]));assert_eq!(write(&JsonNativeSource{root:&root,context:&backing}),"[9,8,7]");assert_eq!(backing.edges.get(),3);backing.edges.set(0);let root=Handle(JsonBorrowKey([8,1,2,0]));assert_eq!(write(&JsonNativeSource{root:&root,context:&backing}),"{\"first\":9,\"second\":8,\"third\":7}");assert_eq!(backing.edges.get(),3);
}

#[derive(serde::Serialize,semio_framework_value::NativeJsonBorrow)]
struct RecursiveTree{id:u64,children:Vec<RecursiveTree>}
#[derive(serde::Serialize,semio_framework_value::NativeJsonBorrow)]
#[serde(tag="kind",rename_all="camelCase")]
#[value(tag="kind",rename_all="camelCase")]
enum InternalNewtype{Tree(RecursiveTree),Linked(Handle),Scalar(u64)}
impl serde::Serialize for Handle{
 fn serialize<S:serde::Serializer>(&self,serializer:S)->Result<S::Ok,S::Error>{use serde::ser::SerializeMap;let mut map=serializer.serialize_map(Some(3))?;map.serialize_entry("first",&9u64)?;map.serialize_entry("second",&8u64)?;map.serialize_entry("third",&7u64)?;map.end()}
}
#[test]
fn recursive_records_and_internally_tagged_newtypes_borrow_the_actual_graph(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let expected=|kind:&str|fixture["cases"].as_array().unwrap().iter().find(|v|v["kind"]==kind).unwrap()["expected"].as_str().unwrap();
 let tree=||RecursiveTree{id:1,children:vec![RecursiveTree{id:2,children:vec![]}]};case(&tree(),expected("recursive"));case(&InternalNewtype::Tree(tree()),expected("internalNewtype"));
 let linked=Linked{edges:std::cell::Cell::new(0)};let root=InternalNewtype::Linked(Handle(JsonBorrowKey([8,1,2,0])));assert_eq!(write(&JsonNativeSource{root:&root,context:&linked}),expected("internalLinked"));assert_eq!(linked.edges.get(),3);assert_eq!(serde_json::to_string(&root).unwrap(),expected("internalLinked"));
 assert!(JsonNativeBorrow::json_node(&InternalNewtype::Scalar(7),0,&JsonBorrowNoContext).is_err());
}

#[test]
fn borrowed_binary32_preserves_its_actual_shortest_precision(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();let expected=fixture["cases"].as_array().unwrap().iter().find(|v|v["kind"]=="float32").unwrap()["expected"].as_str().unwrap();case(&0.1f32,expected);
 for value in[-0.0f32,0.0,1.0,-1.25,1e-7,1e13,f32::MIN_POSITIVE,f32::MAX,f32::INFINITY,f32::NAN]{case(&value,&serde_json::to_string(&value).unwrap())}
}

#[test]
fn neutral_grant_and_progress_borrow_the_exact_canonical_fields(){
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).unwrap();for row in fixture["cases"].as_array().unwrap(){let expected=row["expected"].as_str().unwrap();match row["kind"].as_str().unwrap(){"grant"=>case(&serde_json::from_value::<RetainedCloneGrant>(row["input"].clone()).unwrap(),expected),"progress"=>case(&serde_json::from_value::<semio_framework_value::RetainedCloneProgress>(row["input"].clone()).unwrap(),expected),_=>{}}}
}
