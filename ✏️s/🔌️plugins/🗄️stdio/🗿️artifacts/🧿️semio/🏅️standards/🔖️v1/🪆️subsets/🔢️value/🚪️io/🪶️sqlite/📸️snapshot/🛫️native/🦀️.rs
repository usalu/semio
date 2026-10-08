//! 🌳️ Writes explicit lexeme-preserving Semio values with ordered entries and bounded recursion.
use semio_framework_value::{ValueError,ValueRefusalKind};
use crate::standards::v1::subsets::value::schema::snapshot::{SemioValue,SemioValueEntry,SemioValueNode,SemioValueSnapshot};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_encoding::{self,Writer};
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl};
pub(crate) fn encode(snapshot:&SemioValueSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>)->Result<store::io::IoPayload,ValueError>{native_encoding::encode_admitted(encoding,"semio stdio.semio.value.dsl v1\n","stdio.semio.value.pack v1",control,|control|crate::standards::v1::subsets::value::io::sqlite::snapshot::admit_values(snapshot,store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative,control),|writer,_|fields(snapshot,writer))}
pub(crate) fn value(value:&SemioValue,writer:&mut Writer<'_,'_,'_>,encoding:SnapshotEncoding,depth:usize)->Result<(),ValueError>{
 if depth>=64{return Err(ValueError::new(ValueRefusalKind::DepthLimit,"Semio native value output exceeds depth 64"))}writer.entities(1)?;
 let (binary,text)=match value{SemioValue::Null=>(0,b'Z'),SemioValue::Bool{..}=>(1,b'B'),SemioValue::Int{..}=>(2,b'I'),SemioValue::Float{..}=>(3,b'F'),SemioValue::Str{..}=>(4,b'S'),SemioValue::Bytes{..}=>(5,b'Y'),SemioValue::List{..}=>(6,b'L'),SemioValue::Map{..}=>(7,b'M'),SemioValue::Ref{..}=>(8,b'R')};
 writer.byte(if encoding==SnapshotEncoding::Binary{binary}else{text})?;
 if matches!(value,SemioValue::Null){return Ok(())}
 match value{
 SemioValue::Null=>Ok(()),
 SemioValue::Bool{value}=>{writer.delimiter(b"[",encoding)?;writer.byte(if encoding==SnapshotEncoding::Binary{u8::from(*value)}else{if *value{b'1'}else{b'0'}})?;writer.delimiter(b"]",encoding)},
 SemioValue::Int{lexeme}|SemioValue::Float{lexeme}=>{writer.delimiter(b"[",encoding)?;writer.string(lexeme,encoding)?;writer.delimiter(b"]",encoding)},
 SemioValue::Str{value}=>{writer.delimiter(b"[",encoding)?;writer.string(value,encoding)?;writer.delimiter(b"]",encoding)},
 SemioValue::Bytes{value}=>{writer.delimiter(b"[",encoding)?;writer.octets(value,encoding)?;writer.delimiter(b"]",encoding)},
 SemioValue::Ref{id}=>{writer.delimiter(b"[",encoding)?;writer.string(&id.value,encoding)?;writer.delimiter(b"]",encoding)},
 SemioValue::List{items}=>writer.list(items,encoding,|writer,item|self::value(item,writer,encoding,depth+1)),
 SemioValue::Map{entries}=>writer.list(entries,encoding,|writer,item|entry(item,writer,encoding,depth+1))
 }
}
pub(crate) fn entry(entry:&SemioValueEntry,writer:&mut Writer<'_,'_,'_>,encoding:SnapshotEncoding,depth:usize)->Result<(),ValueError>{writer.string(&entry.key,encoding)?;writer.delimiter(b":",encoding)?;value(&entry.value,writer,encoding,depth)}
fn node(node:&SemioValueNode,writer:&mut Writer<'_,'_,'_>)->Result<(),ValueError>{writer.string(&node.id.value,SnapshotEncoding::Text)?;writer.bytes(b":")?;value(&node.value,writer,SnapshotEncoding::Text,0)}
pub(crate) fn fields(snapshot:&SemioValueSnapshot,writer:&mut Writer<'_,'_,'_>)->Result<(),ValueError>{writer.entities(1)?;writer.bytes(b"[")?;writer.string(&snapshot.schema,SnapshotEncoding::Text)?;writer.bytes(b",")?;value(&snapshot.root,writer,SnapshotEncoding::Text,0)?;writer.bytes(b",")?;writer.list(&snapshot.nodes,SnapshotEncoding::Text,|writer,item|node(item,writer))?;writer.bytes(b"]")}
