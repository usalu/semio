//! 📦️ The native Energy Pack declares one intrinsic document value and borrows its actual canonical field tree.
use super::EnergyModelSnapshot;
use semio_framework_dsl_record::{BorrowedDslRecord,BorrowedRecordSpec,BorrowedFieldSpec,BorrowedShape,RecordLayout,native_encoding::{FieldProjectionSource,FieldProjectionView as View}};
use semio_framework_pack_json::{ArtifactCanonicalJsonTree as Tree,ArtifactCanonicalJsonNode as Node,ArtifactCanonicalJsonText as Text};
use semio_framework_value::{ValueError,ValueRefusalKind,Number};
impl BorrowedDslRecord for EnergyModelSnapshot{const RECORD:BorrowedRecordSpec=BorrowedRecordSpec{keyword:Some("energy-model"),layout:RecordLayout::Inline,fields:&[BorrowedFieldSpec::new(1,"document",BorrowedShape::Value)]};}
fn node<'a>(source:&'a dyn Tree,path:&[usize])->Result<&'a dyn Tree,ValueError>{let mut value=source;for ordinal in path{value=value.canonical_tree_child(*ordinal)?;}Ok(value)}
fn text(value:Text<'_>)->Result<&str,ValueError>{match value{Text::Contiguous(value)=>Ok(value),Text::Native(value)=>if value.text_bytes()==0{Ok("")}else if value.text_chunk_count()==1{value.text_chunk(0).ok_or_else(invalid)}else{Err(invalid())}}}
impl FieldProjectionSource for EnergyModelSnapshot{
 fn projection_view(&self,path:&[usize])->Result<View<'_>,ValueError>{let Some((first,path))=path.split_first()else{return Ok(View::Record(&[1]));};if *first!=0{return Err(invalid());}Ok(match node(self,path)?.canonical_tree_node()?{Node::Null=>View::IntrinsicNull,Node::Bool(value)=>View::IntrinsicBool(value),Node::I64(value)=>View::IntrinsicNumber(Number::Int(value)),Node::U64(value)=>View::IntrinsicNumber(Number::UInt(value)),Node::F64(value)=>View::IntrinsicNumber(Number::Float(value)),Node::F32(value)=>View::IntrinsicNumber(Number::Float(value as f64)),Node::String(value)=>View::IntrinsicText(value),Node::Text(value)=>View::IntrinsicText(text(value)?),Node::Array(length)=>View::IntrinsicArray(length),Node::Object(length)=>View::IntrinsicObject(length),_=>return Err(invalid())})}
 fn projection_key(&self,path:&[usize],index:usize)->Result<&str,ValueError>{let Some((first,path))=path.split_first()else{return Err(invalid());};if *first!=0{return Err(invalid());}text(node(self,path)?.canonical_tree_key(index)?)}
}
pub fn source(state:&EnergyModelSnapshot)->(&dyn FieldProjectionSource,BorrowedRecordSpec){(state,<EnergyModelSnapshot as BorrowedDslRecord>::RECORD)}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"native Energy intrinsic Pack projection differs from its declared source")}

impl semio_framework_dsl_record::receiving::owned::NativeIntrinsicRecord for EnergyModelSnapshot{const INTRINSIC_FIELD_ID:u16=1;}
impl semio_framework_dsl_record::receiving::owned::DslOwnedReceivingField for EnergyModelSnapshot{type Cursor=semio_framework_dsl_record::receiving::owned::NativeIntrinsicReceiving<Self>;}
impl semio_framework_dsl_record::receiving::owned::DslOwnedReceivingRecord for EnergyModelSnapshot{}

/// 🔏️ Digest of the authored one-field intrinsic Record graph, checked against the defining borrowed schema hasher.
pub const SCHEMA_HASH:[u8;32]=[100,45,221,210,163,204,167,21,88,134,193,246,137,148,215,207,164,191,106,235,139,57,218,237,240,87,152,188,92,39,226,14];

