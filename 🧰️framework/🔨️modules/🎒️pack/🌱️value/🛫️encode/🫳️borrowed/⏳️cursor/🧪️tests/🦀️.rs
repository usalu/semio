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

fn fixture() -> serde_json::Value { serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap() }
fn expected(value:&str) -> Vec<u8> { value.as_bytes().chunks_exact(2).map(|pair|u8::from_str_radix(std::str::from_utf8(pair).unwrap(),16).unwrap()).collect() }
fn grant(capacity:usize,release:usize) -> RetainedCloneGrant { RetainedCloneGrant { maximum_items:1,maximum_copy_bytes:64,maximum_capacity_bytes:capacity,maximum_release_bytes:release,maximum_depth:64 } }
fn close(cursor:&mut BorrowedProjectedPackCursor) {
    for _ in 0..20000 {
        let required=cursor.next_close_byte_demand().unwrap();
        if required != 0 {
            let (step,birth,free)=crate::test_allocation::observe_backing(||cursor.close(grant(0,required-1)).unwrap());
            assert_eq!((birth,free),(0,0)); assert_eq!(step.progress,Default::default()); assert_eq!(cursor.next_close_byte_demand().unwrap(),required);
        }
        let (step,birth,free)=crate::test_allocation::observe_backing(||cursor.close(grant(0,required)).unwrap());
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
            let (mut cursor,birth,free)=crate::test_allocation::observe_backing(BorrowedProjectedPackCursor::default); assert_eq!((birth,free),(0,0));
            let mut actual=Vec::new(); let mut complete=false; let mut output=[0;256];
            for _ in 0..20000 {
                let demand=cursor.next_capacity_byte_demand().unwrap();
                let (zero,birth,free)=crate::test_allocation::observe_backing(||cursor.advance(&source,spec,&mut output[..output_bytes],RetainedCloneGrant::default()).unwrap());
                assert_eq!(zero.progress,Default::default());assert_eq!((birth,free),(0,0));
                if demand!=0 {
                    let (denied,birth,free)=crate::test_allocation::observe_backing(||cursor.advance(&source,spec,&mut output[..output_bytes],grant(demand-1,0)).unwrap());
                    assert_eq!(denied.progress,Default::default());assert_eq!((birth,free),(0,0));assert_eq!(cursor.next_capacity_byte_demand().unwrap(),demand);
                }
                let admitted=grant(demand,0);
                let (step,birth,free)=crate::test_allocation::observe_backing(||cursor.advance(&source,spec,&mut output[..output_bytes],admitted).unwrap());
                assert_eq!(birth,step.progress.retained_capacity_bytes);assert_eq!(free,0);assert!(step.progress.fits(admitted));assert!(step.written_bytes<=64 && step.written_bytes<=output_bytes);
                actual.extend_from_slice(&output[..step.written_bytes]);
                if step.complete { complete=true;break; }
            }
            assert!(complete && cursor.is_complete());assert_eq!(actual,bytes);close(&mut cursor);
            let (_,birth,free)=crate::test_allocation::observe_backing(||drop(cursor));assert_eq!((birth,free),(0,0));
        }
        for stop in corpus["cancelStops"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap() as usize) {
            let mut cursor=BorrowedProjectedPackCursor::default();let mut output=[0;64];let mut actual=Vec::new();
            for _ in 0..stop { let demand=cursor.next_capacity_byte_demand().unwrap();let step=cursor.advance(&source,spec,&mut output,grant(demand,0)).unwrap();actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{break} }
            assert!(bytes.starts_with(&actual));close(&mut cursor);
        }
        eprintln!("[DEBUG] original borrowed Pack case{index} exact{}bytes five output grants/cancellation stops; copy<=64 one item, actual births/releases equal queried whole symbol owners",bytes.len());
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
    let corpus:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🌱️intrinsic/🔣️.json")).unwrap();
    for row in corpus["cases"].as_array().unwrap() {
        let source=intrinsic_node(&row["node"]);let field=row["fieldId"].as_u64().unwrap()as u16;let bytes=expected(row["expectedHex"].as_str().unwrap());
        for copy in corpus["copyGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize) {
            for width in corpus["outputGrants"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize) {
                let(mut cursor,birth,free)=crate::test_allocation::observe_backing(BorrowedProjectedPackCursor::default);assert_eq!((birth,free),(0,0));
                let mut output=[0;256];let mut actual=Vec::new();let mut complete=false;
                for _ in 0..20000 {
                    let demand=cursor.next_capacity_byte_demand().unwrap();let minimum=cursor.next_minimum_copy_bytes();let admitted=RetainedCloneGrant{maximum_copy_bytes:copy,..grant(demand,0)};
                    let(zero,birth,free)=crate::test_allocation::observe_backing(||cursor.advance_intrinsic(&source,field,&mut output[..width],RetainedCloneGrant{maximum_items:0,..admitted}).unwrap());assert_eq!(zero.progress,Default::default());assert_eq!((birth,free),(0,0));
                    if minimum!=0 {let(denied,birth,free)=crate::test_allocation::observe_backing(||cursor.advance_intrinsic(&source,field,&mut output[..width],RetainedCloneGrant{maximum_copy_bytes:minimum-1,..admitted}).unwrap());assert_eq!(denied.progress,Default::default());assert_eq!((birth,free),(0,0));}
                    if demand!=0 {let(denied,birth,free)=crate::test_allocation::observe_backing(||cursor.advance_intrinsic(&source,field,&mut output[..width],RetainedCloneGrant{maximum_capacity_bytes:demand-1,..admitted}).unwrap());assert_eq!(denied.progress,Default::default());assert_eq!((birth,free),(0,0));assert_eq!(cursor.next_capacity_byte_demand().unwrap(),demand);}
                    let(step,birth,free)=crate::test_allocation::observe_backing(||cursor.advance_intrinsic(&source,field,&mut output[..width],admitted).unwrap());assert!(step.progress.fits(admitted));assert_eq!(birth,step.progress.retained_capacity_bytes);assert_eq!(free,0);assert!(step.written_bytes<=copy.min(64).min(width));actual.extend_from_slice(&output[..step.written_bytes]);
                    if step.complete{complete=true;break;}
                }
                if complete {let(step,birth,free)=crate::test_allocation::observe_backing(||cursor.advance_intrinsic(&source,field,&mut output,RetainedCloneGrant::default()).unwrap());assert!(step.complete);assert_eq!(step.progress,Default::default());assert_eq!((birth,free),(0,0));}close(&mut cursor);assert!(complete,"intrinsic case {} did not complete",row["name"].as_str().unwrap());assert_eq!(actual,bytes);
                let(_,birth,free)=crate::test_allocation::observe_backing(||drop(cursor));assert_eq!((birth,free),(0,0));
            }
        }
        for stop in corpus["cancelStops"].as_array().unwrap().iter().map(|value|value.as_u64().unwrap()as usize){let mut cursor=BorrowedProjectedPackCursor::default();let mut output=[0;64];let mut actual=Vec::new();for _ in 0..stop{let demand=cursor.next_capacity_byte_demand().unwrap();let step=cursor.advance_intrinsic(&source,field,&mut output,grant(demand,0)).unwrap();actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{break;}}assert!(bytes.starts_with(&actual));close(&mut cursor);}
        eprintln!("[DEBUG] intrinsic Pack {} {} bytes; original ordered source, 15 finite copy/output controls and five explicit cancellation-close positions",row["name"].as_str().unwrap(),bytes.len());
    }
}

#[test]
fn borrowed_pack_failure_preserves_real_comparison_and_original_symbol_backing() {
    let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⚠️failure/🔣️.json")).unwrap();
    let p=&rows["policy"];
    let policy=RetainedCloneGrant { maximum_items:p["maximumItems"].as_u64().unwrap()as usize, maximum_copy_bytes:p["maximumCopyBytes"].as_u64().unwrap()as usize, maximum_capacity_bytes:p["maximumCapacityBytes"].as_u64().unwrap()as usize, maximum_release_bytes:p["maximumReleaseBytes"].as_u64().unwrap()as usize, maximum_depth:p["maximumDepth"].as_u64().unwrap()as usize };
    let source=DslValue::String(rows["examples"][0]["text"].as_str().unwrap().into());
    let original=match &source { DslValue::String(text)=>text.as_ptr(), _=>unreachable!() };
    let mut cursor=BorrowedProjectedPackCursor::default();
    let mut retained=0;
    while !cursor.symbols.has_reserved_slot() {
        let(step,birth,free)=crate::test_allocation::observe_backing(||cursor.symbols.reserve_one(policy.maximum_capacity_bytes).unwrap());
        assert_eq!(birth,step.allocated_bytes);assert_eq!(free,0);assert!(birth<=policy.maximum_capacity_bytes);retained+=birth;
    }
    cursor.symbols.push_reserved(Symbol { path:[0;64], depth:0, occurrences:usize::MAX, selected:false, forced:false }).unwrap();
    cursor.note=Some(Symbol { path:[0;64], depth:0, occurrences:1, selected:false, forced:false });
    cursor.phase=Phase::Find;
    let mut output=[0xa5;64];
    let(failure,birth,free)=crate::test_allocation::observe_backing(||cursor.advance_intrinsic(&source,1,&mut output,policy).unwrap_err());
    let receipt=failure.reason.retained_progress();
    assert_eq!(failure.written_bytes,0);assert_eq!((birth,free),(0,0));assert_eq!(output,[0xa5;64]);
    assert_eq!(receipt.copied_items,1);assert_eq!(receipt.copied_bytes,rows["examples"][0]["failure"]["reason"]["retainedProgress"]["copiedBytes"].as_u64().unwrap()as usize);
    assert!(receipt.fits(policy));assert_eq!(failure.reason.message,rows["examples"][0]["failure"]["reason"]["message"].as_str().unwrap());
    assert_eq!(match &source {DslValue::String(text)=>text.as_ptr(),_=>unreachable!()},original);
    let mut released=0;
    for _ in 0..64 {
        let(step,birth,free)=crate::test_allocation::observe_backing(||cursor.close(policy).unwrap());
        assert_eq!(birth,0);assert_eq!(free,step.progress.released_bytes);assert!(step.progress.fits(policy));released+=free;
        if step.complete {break;}
    }
    assert!(cursor.terminal_is_empty());assert_eq!(released,retained);
    eprintln!("[DEBUG] Pack post-comparison refusal retained original UTF8/source and every System symbol backing; sole Value receipt {} copy bytes",receipt.copied_bytes);
}


#[test]
fn borrowed_pack_flat_intrinsic_uses_only_original_actual_depth_and_receipts(){
    let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/⚠️failure/🔣️.json")).unwrap();
    let row=&rows["depthProbe"];let source=DslValue::Bool(row["source"].as_bool().unwrap());let field=row["fieldId"].as_u64().unwrap()as u16;
    let p=&rows["policy"];let policy=RetainedCloneGrant{maximum_items:p["maximumItems"].as_u64().unwrap()as usize,maximum_copy_bytes:p["maximumCopyBytes"].as_u64().unwrap()as usize,maximum_capacity_bytes:p["maximumCapacityBytes"].as_u64().unwrap()as usize,maximum_release_bytes:p["maximumReleaseBytes"].as_u64().unwrap()as usize,maximum_depth:row["maximumDepth"].as_u64().unwrap()as usize};
    let mut cursor=BorrowedProjectedPackCursor::default();let mut output=[0;64];let mut actual=Vec::new();
    for _ in 0..64{
        let(denied,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance_intrinsic(&source,field,&mut output,RetainedCloneGrant{maximum_depth:0,..policy}).unwrap());
        assert_eq!(denied.progress,Default::default());assert_eq!((event.requested_bytes,event.released_bytes),(0,0));
        assert!(cursor.next_advance_depth_demand().unwrap()<=policy.maximum_depth);
        let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.advance_intrinsic(&source,field,&mut output,policy).unwrap());
        assert!(step.progress.fits(policy));assert_eq!(event.requested_bytes,step.progress.retained_capacity_bytes);assert_eq!(event.released_bytes,step.progress.released_bytes);actual.extend_from_slice(&output[..step.written_bytes]);if step.complete{break;}
    }
    assert!(cursor.is_complete());assert_eq!(actual,expected(row["expectedHex"].as_str().unwrap()));
    let(step,event)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close(policy).unwrap());
    assert!(step.complete);assert!(step.progress.fits(policy));assert_eq!((event.requested_bytes,event.released_bytes),(step.progress.retained_capacity_bytes,step.progress.released_bytes));
    eprintln!("[DEBUG] original flat intrinsic Pack byte oracle and System receipts use depth1, unchanged64 inline bound and fixed currencies");
}
