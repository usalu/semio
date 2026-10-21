use super::*;
use semio_framework_dsl_record::{BorrowedFieldSpec as F, RecordLayout};

static REFERENCE_FIELDS: [F; 4] = [F::new(4,"subset",B::Text),F::new(1,"id",B::Text),F::new(3,"standard",B::Text),F::new(2,"kind",B::Text)];
fn reference_spec() -> BorrowedRecordSpec { BorrowedRecordSpec { keyword: None, layout: RecordLayout::Inline, fields: &REFERENCE_FIELDS } }
fn float_shape() -> B { B::Float }
fn reference_shape() -> B { B::Record(reference_spec) }
static ROOT_FIELDS: [F; 3] = [F::new(2,"target",B::Block(reference_shape)),F::new(1,"local",B::Text),F::new(3,"points",B::List(float_shape))];
static TABLE_ROW_FIELDS: [F;4]=[F::new(3,"visible",B::Bool),F::new(1,"text",B::Text),F::new(2,"optional",B::Float),F::new(4,"tuple",B::Tuple(float_shape,Some(2)))];
fn table_row_spec()->BorrowedRecordSpec{BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:&TABLE_ROW_FIELDS}}
static TABLE_FIELDS:[F;1]=[F::new(1,"rows",B::Table(table_row_spec))];
static LONG_FIELDS: [F; 6] = [F::new(1,"first",B::Text),F::new(2,"again",B::Text),F::new(3,"inline",B::Text),F::new(4,"index",B::UInt),F::new(5,"signed",B::Int),F::new(6,"absent",B::UInt)];
fn text_shape()->B{B::Text}
static MAP_FIELDS:[F;1]=[F::new(1,"map",B::Map(text_shape))];
static STMT_ROW:[F;1]=[F::new(0,"value",B::Text)];
fn stmt_row()->BorrowedRecordSpec{BorrowedRecordSpec{keyword:Some("put"),layout:RecordLayout::Inline,fields:&STMT_ROW}}
static VARIANTS:[(&str,fn()->BorrowedRecordSpec);1]=[("put",stmt_row)];
static STMT_FIELDS:[F;1]=[F::new(1,"statements",B::Statements(&VARIANTS))];
static WIRE_FIELDS:[F;1]=[F::new(1,"wire",B::Wire)];

#[test]
fn borrowed_projected_pack_original_expression_ast_matches_independent_corpus_and_cancellation(){
 use semio_framework_dsl_record::{ExprValue as E,ExprOp};
 fn expression(node:&serde_json::Value)->E{match node[0].as_str().unwrap(){"num"=>E::Num(node[1].as_f64().unwrap()),"var"=>E::Var(node[1].as_str().unwrap().into()),"neg"=>E::Neg(Box::new(expression(&node[1]))),"binary"=>E::Binary(match node[1].as_str().unwrap(){"add"=>ExprOp::Add,"sub"=>ExprOp::Sub,"mul"=>ExprOp::Mul,_=>ExprOp::Div},Box::new(expression(&node[2])),Box::new(expression(&node[3]))),"call"=>E::Call(node[1].as_str().unwrap().into(),node[2].as_array().unwrap().iter().map(expression).collect()),_=>panic!("expression fixture")}}
 struct Source(E);
 impl FieldProjectionSource for Source{fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{match path{[]=>Ok(V::Record(&[1])),[0]=>Ok(V::Expr(&self.0)),_=>Err(invalid("original expression field path"))}}}
 static FIELDS:[F;1]=[F::new(1,"formula",B::Expr)];let spec=BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:&FIELDS};let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🧮️expression/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let source=Source(expression(&row["ast"]));let expected:Vec<u8>=row["bodyHex"].as_str().unwrap().as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect();let mut cursor=BorrowedProjectedPackCursor::default();let mut output=[0;1];let mut actual=Vec::new();for _ in 0..1000000{let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:cursor.next_minimum_copy_bytes(),maximum_capacity_bytes:cursor.next_capacity_byte_demand().unwrap(),maximum_release_bytes:0,maximum_depth:cursor.next_advance_depth_demand().unwrap()};let(step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance(&source,spec,&mut output,grant).unwrap());assert_eq!((birth,free),(step.progress.retained_capacity_bytes,0));assert!(step.progress.fits(grant));actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{break;}}let complete=cursor.is_complete();close(&mut cursor);assert!(complete);assert_eq!(actual,expected,"{}",row["text"]);assert_eq!(semio_framework_dsl_record::print_expr(&source.0),row["text"].as_str().unwrap());for stop in fixture["cancelNodes"].as_array().unwrap(){let mut cursor=BorrowedProjectedPackCursor::default();for _ in 0..stop.as_u64().unwrap(){let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:0,maximum_depth:128};if cursor.advance(&source,spec,&mut output,grant).unwrap().complete{break;}}close(&mut cursor);}eprintln!("[DEBUG] retained original expression{} emits literal independent corpus bytes and cancels without AST/text shadow",row["text"]);}
}

#[test]
fn borrowed_projected_pack_standard_map_statements_wire_match_portable_oracle(){
 struct Source<'a>(&'a serde_json::Value);
 impl FieldProjectionSource for Source<'_>{
  fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{if path.is_empty(){return Ok(V::Record(&[1]));}if path[0]!=0{return Err(invalid("branch ordinal"));}let row=self.0;let tail=&path[1..];Ok(match row["kind"].as_str().unwrap(){
   "map"=>if tail.is_empty(){V::Map(row["pairs"].as_array().unwrap().len())}else{row["pairs"][tail[0]][1].as_str().map_or(V::Absent,V::Text)},
   "statements"=>if tail.is_empty(){V::Statements(row["values"].as_array().unwrap().len())}else if tail.len()==1{V::Record(&[0])}else{V::Text(row["values"][tail[0]].as_str().unwrap())},
   "wire"=>if tail.is_empty(){V::Wire(Some(row["directed"].as_bool().unwrap()))}else if tail[0]==8{V::IntrinsicNull}else{row["slots"][tail[0]].as_str().map_or(V::Absent,V::Text)},_=>return Err(invalid("branch kind"))})}
  fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{if path!=[0]{return Err(invalid("branch key path"));}Ok(if self.0["kind"]=="map"{self.0["pairs"][index][0].as_str().unwrap()}else{self.0["keyword"].as_str().unwrap()})}
 }
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🧩️branches/🔣️.json")).unwrap();
 for row in fixture["cases"].as_array().unwrap(){let source=Source(row);let fields=match row["kind"].as_str().unwrap(){"map"=>&MAP_FIELDS,"statements"=>&STMT_FIELDS,_=>&WIRE_FIELDS};let spec=BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields};let mut cursor=BorrowedProjectedPackCursor::default();let mut output=[0;1];let mut actual=Vec::new();
  for _ in 0..20000{let capacity=cursor.next_capacity_byte_demand().unwrap();let(step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance(&source,spec,&mut output,grant(capacity,0)).unwrap());assert_eq!((birth,free),(step.progress.retained_capacity_bytes,0));actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{break;}}
  let complete=cursor.is_complete();close(&mut cursor);assert!(complete);assert_eq!(actual,expected(row["expectedHex"].as_str().unwrap()));eprintln!("[DEBUG] original borrowed Pack {} exact portable canonical bytes and every backing receipt",row["kind"]);
 }
}

#[test]
fn borrowed_projected_pack_native_paged_key_and_text_cross_unicode_pages(){
 use semio_framework_value::{NativeEncodeControl, ErasedSnapshotRetirement, RetainedCloneStep};
 use semio_framework_value_paged_text::paged_text::{PagedText, TextReadSource};
 let fixture:serde_json::Value=serde_json::from_str(include_str!("🧫️fixtures/🌿️native-text/🔣️.json")).unwrap();let marker=fixture["scalar"].as_str().unwrap().as_bytes();let split=fixture["split"].as_u64().unwrap()as usize;let length=fixture["bytes"].as_u64().unwrap()as usize;
 struct Bytes<'a>{marker:&'a[u8],split:usize,length:usize}impl TextReadSource for Bytes<'_>{fn byte_len(&self)->usize{self.length}fn byte_at(&self,index:usize)->Option<u8>{if index>=self.length{None}else if index>=self.split&&index<self.split+self.marker.len(){Some(self.marker[index-self.split])}else{Some(b'x')}}}
 struct Source{value:PagedText<8194>}impl FieldProjectionSource for Source{fn projection_view(&self,path:&[usize])->Result<V<'_>,ValueError>{Ok(match path{[]=>V::Record(&[1]),[0]=>V::Map(1),[0,0]=>V::TextSource(self.value.read_view()?),_=>return Err(invalid("native text path"))})}fn projection_key_view(&self,path:&[usize],index:usize)->Result<TextReadView<'_>,ValueError>{if path!=[0]||index!=0{return Err(invalid("native key path"));}self.value.read_view()}}
 let mut source=Source{value:PagedText::empty()};let mut allow=|_|true;let mut control=NativeEncodeControl::new(65536,&mut allow);source.value.read_from_encoding_source(&Bytes{marker,split,length},&mut control).unwrap();assert!(source.value.read_view().unwrap().contiguous().is_none());let backing=source.value.allocated_bytes();
 let spec=BorrowedRecordSpec{keyword:None,layout:RecordLayout::Inline,fields:&MAP_FIELDS};let mut cursor=BorrowedProjectedPackCursor::default();let mut output=[0;1];let mut actual=Vec::new();
 for _ in 0..100000{let capacity=cursor.next_capacity_byte_demand().unwrap();let(step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance(&source,spec,&mut output,grant(capacity,0)).unwrap());assert_eq!((birth,free),(step.progress.retained_capacity_bytes,0));actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{break;}}
 let complete=cursor.is_complete();close(&mut cursor);assert!(complete);let mut expected=vec![1,130,64];expected.extend((0..length).map(|index|Bytes{marker,split,length}.byte_at(index).unwrap()));expected.extend(self::expected(fixture["bodyHex"].as_str().unwrap()));assert_eq!(actual,expected);assert_eq!(source.value.allocated_bytes(),backing);
 for _ in 0..20000{match source.value.close_step(RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:0,maximum_release_bytes:4096,maximum_depth:64}).unwrap(){RetainedCloneStep::Progress(_)=>{},RetainedCloneStep::Complete(_)=>break,}}assert!(source.value.terminal_is_empty());eprintln!("[DEBUG] retained original native8194byte map key/text crosses Unicode page boundary with no contiguous key port or flattened text");
}

struct Node { value: Kind, ids: &'static [u16] }
enum Kind { Block(Box<Node>), Record(Vec<Node>), List(Vec<Node>,bool), Text(String), Bool(bool), Float(f64), Int(i64), UInt(u64), Absent }
impl Node {
    fn parse(value: &serde_json::Value) -> Self {
        let mut ids = &[][..];
        let value = match value["kind"].as_str().unwrap() {
            "block" => Kind::Block(Box::new(Self::parse(&value["items"][0]))),
            "record" => { let fields = value["fields"].as_array().unwrap(); let words: Vec<_> = fields.iter().map(|row|row["id"].as_u64().unwrap() as u16).collect(); ids = match words.as_slice() { [2,1,3]=>&[2,1,3], [4,1,3,2]=>&[4,1,3,2], []=>&[], [1,2,3,4,5,6]=>&[1,2,3,4,5,6], [1]=>&[1], [3,1,2,4]=>&[3,1,2,4], _=>panic!("neutral Record IDs lack authored static authority") }; Kind::Record(fields.iter().map(|row|Self::parse(&row["node"])).collect()) },
            "text" => Kind::Text(value["value"].as_str().unwrap().repeat(value["repeat"].as_u64().unwrap_or(1) as usize)),
            "bool" => Kind::Bool(value["value"].as_bool().unwrap()),
            "float" => Kind::Float(f64::from_bits(u64::from_str_radix(value["bits"].as_str().unwrap(),16).unwrap())),
            "int" => Kind::Int(value["value"].as_i64().unwrap()),
            "uint" => Kind::UInt(value["value"].as_u64().unwrap()),
            "absent" => Kind::Absent,
            kind => Kind::List(value["items"].as_array().unwrap().iter().map(Self::parse).collect(),kind=="tuple"),
        };
        Self { value, ids }
    }
    fn at(&self,path:&[usize]) -> Result<&Self,ValueError> {
        let Some((head,tail))=path.split_first() else { return Ok(self) };
        match &self.value { Kind::Block(inner) if *head==0 => inner.at(tail), Kind::Record(items)|Kind::List(items,_) => items.get(*head).ok_or_else(||invalid("neutral original ordinal changed"))?.at(tail), _ => Err(invalid("neutral scalar has no child")) }
    }
}
impl FieldProjectionSource for Node {
    fn projection_view(&self,path:&[usize]) -> Result<V<'_>,ValueError> {
        let node = self.at(path)?;
        Ok(match &node.value { Kind::Block(_) => V::Block, Kind::Record(_) => V::Record(node.ids), Kind::List(items,true) => V::Tuple(items.len()), Kind::List(items,false) => V::List(items.len()), Kind::Text(text) => V::Text(text), Kind::Bool(value) => V::Bool(*value), Kind::Float(value) => V::Float(*value), Kind::Int(value) => V::Int(*value), Kind::UInt(value) => V::UInt(*value), Kind::Absent => V::Absent })
    }
}

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🔣️.json")).unwrap() }
fn expected(value:&str) -> Vec<u8> { value.as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect() }
fn grant(capacity:usize,release:usize) -> RetainedCloneGrant { RetainedCloneGrant { maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:64 } }
fn close(cursor:&mut BorrowedProjectedPackCursor) {
    for _ in 0..20000 {
        let required=cursor.next_close_byte_demand().unwrap();
        if required != 0 {
            let (step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.close(grant(0,required-1)).unwrap());
            assert_eq!((birth,free),(0,0)); assert_eq!(step.progress,Default::default()); assert_eq!(cursor.next_close_byte_demand().unwrap(),required);
        }
        let (step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.close(grant(0,required)).unwrap());
        assert_eq!(birth,0); assert_eq!(free,step.progress.released_bytes); assert!(step.progress.fits(grant(0,required)));
        if step.complete { assert!(cursor.terminal_is_empty()); return; }
    }
    panic!("original Pack symbol owners did not reach bounded terminal close");
}

#[test]
fn borrowed_projected_pack_cursor_matches_neutral_buffer_oracle_and_exact_physical_grants() {
    let corpus=fixture();
    for (index,row) in corpus["cases"].as_array().unwrap().iter().enumerate() {
        let source=Node::parse(row); let bytes=expected(corpus["expectedHex"][index].as_str().unwrap());
        let spec=BorrowedRecordSpec { keyword:None,layout:RecordLayout::Inline,fields:match index { 0=>&ROOT_FIELDS,1=>&[],2=>&LONG_FIELDS,_=>&TABLE_FIELDS } };
        for output_bytes in corpus["grants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as usize) {
            let (mut cursor,birth,free)=semio_framework_trace_memory_testing::observe_backing(BorrowedProjectedPackCursor::default); assert_eq!((birth,free),(0,0));
            let mut actual=Vec::new(); let mut complete=false; let mut output=[0;256];
            for _ in 0..20000 {
                let demand=cursor.next_capacity_byte_demand().unwrap();
                let (zero,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance(&source,spec,&mut output[..output_bytes],RetainedCloneGrant::default()).unwrap());
                assert_eq!(zero.progress,Default::default());assert_eq!((birth,free),(0,0));
                if demand!=0 {
                    let (denied,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance(&source,spec,&mut output[..output_bytes],grant(demand-1,0)).unwrap());
                    assert_eq!(denied.progress,Default::default());assert_eq!((birth,free),(0,0));assert_eq!(cursor.next_capacity_byte_demand().unwrap(),demand);
                }
                let admitted=grant(demand,0);
                let (step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance(&source,spec,&mut output[..output_bytes],admitted).unwrap());
                assert_eq!(birth,step.progress.retained_capacity_bytes);assert_eq!(free,0);assert!(step.progress.fits(admitted));assert!(step.written_bytes<=64 && step.written_bytes<=output_bytes);
                actual.extend_from_slice(&output[..step.written_bytes]);
                if step.complete { complete=true;break; }
            }
            assert!(complete && cursor.is_complete());assert_eq!(actual,bytes);close(&mut cursor);
            let (_,birth,free)=semio_framework_trace_memory_testing::observe_backing(||drop(cursor));assert_eq!((birth,free),(0,0));
        }
        for stop in corpus["cancelStops"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as usize) {
            let mut cursor=BorrowedProjectedPackCursor::default();let mut output=[0;64];let mut actual=Vec::new();
            for _ in 0..stop { let demand=cursor.next_capacity_byte_demand().unwrap();let step=cursor.advance(&source,spec,&mut output,grant(demand,0)).unwrap();actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{break} }
            assert!(bytes.starts_with(&actual));close(&mut cursor);
        }
    }
}

fn intrinsic_node(node:&serde_json::Value)->DslValue {
    match node["kind"].as_str().unwrap() {
        "null"=>DslValue::Null,
        "bool"=>DslValue::Bool(node["value"].as_bool().unwrap()),
        "int"=>DslValue::Number(Number::Int(node["value"].as_str().unwrap().parse().unwrap())),
        "uint"=>DslValue::Number(Number::UInt(node["value"].as_str().unwrap().parse().unwrap())),
        "float"=>DslValue::Number(Number::Float(f64::from_bits(u64::from_str_radix(node["bits"].as_str().unwrap(),16).unwrap()))),
        "text"=>DslValue::String(node["value"].as_str().unwrap().into()),
        "bytes"=>DslValue::Bytes(node["value"].as_array().unwrap().iter().map(|byte|byte.as_u64().unwrap()as u8).collect()),
        "array"=>DslValue::Array(node["items"].as_array().unwrap().iter().map(intrinsic_node).collect()),
        "object"=>DslValue::Object(node["members"].as_array().unwrap().iter().map(|member|(member["key"].as_str().unwrap().into(),intrinsic_node(&member["node"]))).collect()),
        _=>panic!("intrinsic corpus has no authored node authority"),
    }
}

#[test]
fn borrowed_projected_pack_cursor_intrinsic_order_duplicates_and_exact_grants() {
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🌱️intrinsic/🔣️.json")).unwrap();
    for row in corpus["cases"].as_array().unwrap() {
        let source=intrinsic_node(&row["node"]);let field=row["fieldId"].as_u64().unwrap()as u16;let bytes=expected(row["expectedHex"].as_str().unwrap());
        for copy in corpus["copyGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize) {
            for width in corpus["outputGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize) {
                let(mut cursor,birth,free)=semio_framework_trace_memory_testing::observe_backing(BorrowedProjectedPackCursor::default);assert_eq!((birth,free),(0,0));
                let mut output=[0;256];let mut actual=Vec::new();let mut complete=false;
                for _ in 0..20000 {
                    let demand=cursor.next_capacity_byte_demand().unwrap();let minimum=cursor.next_minimum_copy_bytes();let admitted=RetainedCloneGrant{maximum_copy_bytes:copy.max(minimum),..grant(demand,0)};
                    let(zero,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance_intrinsic(&source,field,&mut output[..width],RetainedCloneGrant{maximum_items:0,..admitted}).unwrap());assert_eq!(zero.progress,Default::default());assert_eq!((birth,free),(0,0));
                    if minimum!=0 {let(denied,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance_intrinsic(&source,field,&mut output[..width],RetainedCloneGrant{maximum_copy_bytes:minimum-1,..admitted}).unwrap());assert_eq!(denied.progress,Default::default());assert_eq!((birth,free),(0,0));}
                    if demand!=0 {let(denied,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance_intrinsic(&source,field,&mut output[..width],RetainedCloneGrant{maximum_capacity_bytes:demand-1,..admitted}).unwrap());assert_eq!(denied.progress,Default::default());assert_eq!((birth,free),(0,0));assert_eq!(cursor.next_capacity_byte_demand().unwrap(),demand);}
                    let(step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance_intrinsic(&source,field,&mut output[..width],admitted).unwrap());assert!(step.progress.fits(admitted));assert_eq!(birth,step.progress.retained_capacity_bytes);assert_eq!(free,0);assert!(step.written_bytes<=copy.min(64).min(width));actual.extend_from_slice(&output[..step.written_bytes]);
                    if step.complete{complete=true;break;}
                }
                if complete {let(step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance_intrinsic(&source,field,&mut output,RetainedCloneGrant::default()).unwrap());assert!(step.complete);assert_eq!(step.progress,Default::default());assert_eq!((birth,free),(0,0));}close(&mut cursor);assert!(complete,"intrinsic case {} did not complete",row["name"].as_str().unwrap());assert_eq!(actual,bytes);
                let(_,birth,free)=semio_framework_trace_memory_testing::observe_backing(||drop(cursor));assert_eq!((birth,free),(0,0));
            }
        }
        for stop in corpus["cancelStops"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){let mut cursor=BorrowedProjectedPackCursor::default();let mut output=[0;64];let mut actual=Vec::new();for _ in 0..stop{let demand=cursor.next_capacity_byte_demand().unwrap();let step=cursor.advance_intrinsic(&source,field,&mut output,grant(demand,0)).unwrap();actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{break;}}assert!(bytes.starts_with(&actual));close(&mut cursor);}
    }
}

#[test]
fn borrowed_pack_failure_preserves_real_comparison_and_original_symbol_backing() {
    let rows:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/⚠️failure/🔣️.json")).unwrap();
    let p=&rows["policy"];
    let policy=RetainedCloneGrant { maximum_items:p["maximumItems"].as_u64().unwrap()as usize, maximum_copy_bytes:p["maximumCopyBytes"].as_u64().unwrap()as usize, maximum_capacity_bytes:p["maximumCapacityBytes"].as_u64().unwrap()as usize, maximum_release_bytes:p["maximumReleaseBytes"].as_u64().unwrap()as usize, maximum_depth:p["maximumDepth"].as_u64().unwrap()as usize };
    let source=DslValue::String(rows["examples"][0]["text"].as_str().unwrap().into());
    let original=match &source { DslValue::String(text)=>text.as_ptr(), _=>unreachable!() };
    let mut cursor=BorrowedProjectedPackCursor::default();
    let mut retained=0;
    while !cursor.symbols.has_reserved_slot() {
        let(step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.symbols.reserve_one(policy.maximum_capacity_bytes).unwrap());
        assert_eq!(birth,step.allocated_bytes);assert_eq!(free,0);assert!(birth<=policy.maximum_capacity_bytes);retained+=birth;
    }
    cursor.symbols.push_reserved(Symbol { path:[0;64], depth:0, occurrences:usize::MAX, selected:false, forced:false,key:None }).unwrap();
    cursor.note=Some(Symbol { path:[0;64], depth:0, occurrences:1, selected:false, forced:false,key:None });
    cursor.phase=Phase::Find;
    let mut output=[0xa5;64];
    let(failure,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.advance_intrinsic(&source,1,&mut output,policy).unwrap_err());
    let receipt=failure.reason.retained_progress();
    assert_eq!(failure.written_bytes,0);assert_eq!((birth,free),(0,0));assert_eq!(output,[0xa5;64]);
    assert_eq!(receipt.copied_items,1);assert_eq!(receipt.copied_bytes,rows["examples"][0]["failure"]["reason"]["retainedProgress"]["copiedBytes"].as_u64().unwrap()as usize);
    assert!(receipt.fits(policy));assert_eq!(failure.reason.message,rows["examples"][0]["failure"]["reason"]["message"].as_str().unwrap());
    assert_eq!(match &source {DslValue::String(text)=>text.as_ptr(),_=>unreachable!()},original);
    let mut released=0;
    for _ in 0..64 {
        let(step,birth,free)=semio_framework_trace_memory_testing::observe_backing(||cursor.close(policy).unwrap());
        assert_eq!(birth,0);assert_eq!(free,step.progress.released_bytes);assert!(step.progress.fits(policy));released+=free;
        if step.complete {break;}
    }
    assert!(cursor.terminal_is_empty());assert_eq!(released,retained);
}


#[test]
fn borrowed_pack_flat_intrinsic_uses_only_original_actual_depth_and_receipts(){
    let rows:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/⚠️failure/🔣️.json")).unwrap();
    let row=&rows["depthProbe"];let source=DslValue::Bool(row["source"].as_bool().unwrap());let field=row["fieldId"].as_u64().unwrap()as u16;
    let p=&rows["policy"];let policy=RetainedCloneGrant{maximum_items:p["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:4096,maximum_capacity_bytes:p["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:p["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:row["maximumDepth"].as_u64().unwrap()as usize};
    let mut cursor=BorrowedProjectedPackCursor::default();let mut output=[0;64];let mut actual=Vec::new();
    for _ in 0..256{
        let(denied,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance_intrinsic(&source,field,&mut output,RetainedCloneGrant{maximum_depth:0,..policy}).unwrap());
        assert_eq!(denied.progress,Default::default());assert_eq!((event.requested_bytes,event.released_bytes),(0,0));
        assert!(cursor.next_advance_depth_demand().unwrap()<=policy.maximum_depth);
        let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance_intrinsic(&source,field,&mut output,policy).unwrap());
        assert!(step.progress.fits(policy));assert_eq!(event.requested_bytes,step.progress.retained_capacity_bytes);assert_eq!(event.released_bytes,step.progress.released_bytes);actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{break;}
    }
    assert!(cursor.is_complete());assert_eq!(actual,expected(row["expectedHex"].as_str().unwrap()));
    let mut step=BorrowedProjectedPackProgress::default();for _ in 0..256{let(next,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close(policy).unwrap());assert_eq!((event.requested_bytes,event.released_bytes),(next.progress.retained_capacity_bytes,next.progress.released_bytes));step=next;if step.complete{break;}}
    assert!(step.complete);assert!(step.progress.fits(policy));
    eprintln!("[DEBUG] original flat intrinsic Pack byte oracle and System receipts use source8/backing bound and unchanged64 logical bound and fixed currencies");
}
