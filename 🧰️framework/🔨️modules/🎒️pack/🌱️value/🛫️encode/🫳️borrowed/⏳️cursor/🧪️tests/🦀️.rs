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
