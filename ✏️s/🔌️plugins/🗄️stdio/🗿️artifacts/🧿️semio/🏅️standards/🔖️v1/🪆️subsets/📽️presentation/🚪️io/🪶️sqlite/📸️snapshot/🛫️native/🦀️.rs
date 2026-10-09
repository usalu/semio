//! 📽️ Emits every Presentation owner and its actual typed Document block vocabulary under one control.
use crate::standards::v1::subsets::document::schema::snapshot::DocBlock;
use crate::standards::v1::subsets::presentation::schema::snapshot::{SemioPresentationSnapshot,SlideShape,SlideFrame,PlaceholderKind};
use crate::standards::v1::subsets::base::io::sqlite::snapshot::native_encoding::{self,Writer};
use crate::standards::v1::subsets::document::io::sqlite::snapshot::native_encoding as document;
use store::sqlite_snapshot::{SnapshotEncoding,SqliteSnapshotControl,ValueError};
pub(crate) fn encode(value:&SemioPresentationSnapshot,encoding:SnapshotEncoding,control:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io::IoPayload,ValueError>{native_encoding::encode_admitted(encoding,"semio s.stdio.semio.presentation.dsl v1\n","s.stdio.semio.presentation.pack v1",control,|control|crate::standards::v1::subsets::presentation::io::sqlite::snapshot::admit_values(value,store::sqlite_snapshot::SqliteSnapshotPhase::EncodeNative,control),|writer,encoding|fields(value,writer,encoding),native_owner)}
fn comma(w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),semio_framework_value::ValueError>{w.delimiter(b",",e)}
fn option(value:Option<&String>,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),semio_framework_value::ValueError>{match value{None=>if e==SnapshotEncoding::Binary{w.byte(0)}else{w.bytes(b"[0]")},Some(value)=>{if e==SnapshotEncoding::Binary{w.byte(1)?}else{w.bytes(b"[1,")?}w.string(value,e)?;w.delimiter(b"]",e)}}}
fn number(value:f64,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),semio_framework_value::ValueError>{if e==SnapshotEncoding::Binary{w.number(value,e)}else{w.integer(value.to_bits(),e)}}
fn frame(value:&SlideFrame,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),semio_framework_value::ValueError>{w.delimiter(b"[[",e)?;number(value.origin.x,w,e)?;comma(w,e)?;number(value.origin.y,w,e)?;w.delimiter(b"]",e)?;comma(w,e)?;number(value.width,w,e)?;comma(w,e)?;number(value.height,w,e)?;w.delimiter(b"]",e)}
fn blocks(values:&[DocBlock],w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),semio_framework_value::ValueError>{w.list(values,e,|w,value|if e==SnapshotEncoding::Binary{w.text_record(|w|document::block(value,w,SnapshotEncoding::Text,0))}else{document::block(value,w,e,0)})}
fn placeholder(value:&PlaceholderKind,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),semio_framework_value::ValueError>{let(binary,text)=match value{PlaceholderKind::Title=>(0,b'T'),PlaceholderKind::Subtitle=>(1,b'S'),PlaceholderKind::Body=>(2,b'B'),PlaceholderKind::Footer=>(3,b'F'),PlaceholderKind::SlideNumber=>(4,b'N'),PlaceholderKind::DateTime=>(5,b'D'),PlaceholderKind::Other{..}=>(6,b'O')};w.byte(if e==SnapshotEncoding::Binary{binary}else{text})?;if let PlaceholderKind::Other{value}=value{w.delimiter(b"[",e)?;w.string(value,e)?;w.delimiter(b"]",e)?}Ok(())}
fn shape(value:&SlideShape,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),semio_framework_value::ValueError>{w.entities(1)?;let(binary,text,rectangle)=match value{SlideShape::TextBox{frame,..}=>(0,b'X',frame),SlideShape::Picture{frame,..}=>(1,b'P',frame),SlideShape::Table{frame,..}=>(2,b'T',frame),SlideShape::Placeholder{frame,..}=>(3,b'H',frame)};w.byte(if e==SnapshotEncoding::Binary{binary}else{text})?;w.delimiter(b"[",e)?;frame(rectangle,w,e)?;comma(w,e)?;match value{
 SlideShape::TextBox{blocks:values,..}=>blocks(values,w,e)?,
 SlideShape::Picture{image,..}=>{w.delimiter(b"[",e)?;w.string(&image.asset_id,e)?;comma(w,e)?;w.string(&image.mime,e)?;comma(w,e)?;w.octets(&image.bytes,e)?;w.delimiter(b"]",e)?},
 SlideShape::Table{rows,..}=>w.list(rows,e,|w,row|w.list(&row.cells,e,|w,cell|blocks(&cell.blocks,w,e)))?,
 SlideShape::Placeholder{kind,..}=>placeholder(kind,w,e)?
 }w.delimiter(b"]",e)}
fn shapes(values:&[SlideShape],w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),semio_framework_value::ValueError>{w.list(values,e,|w,value|shape(value,w,e))}
pub(crate) fn fields(value:&SemioPresentationSnapshot,w:&mut Writer<'_,'_,'_>,e:SnapshotEncoding)->Result<(),semio_framework_value::ValueError>{w.entities(1)?;if e==SnapshotEncoding::Binary{w.byte(1)?}else{w.bytes(b"schema=")?}w.string(&value.schema,e)?;w.delimiter(b"\nmasters=",e)?;
 w.list(&value.masters,e,|w,value|{w.delimiter(b"[",e)?;w.string(&value.id,e)?;comma(w,e)?;shapes(&value.shapes,w,e)?;w.delimiter(b"]",e)})?;w.delimiter(b"\nlayouts=",e)?;
 w.list(&value.layouts,e,|w,value|{w.delimiter(b"[",e)?;w.string(&value.id,e)?;comma(w,e)?;w.string(&value.master_id,e)?;comma(w,e)?;shapes(&value.shapes,w,e)?;w.delimiter(b"]",e)})?;w.delimiter(b"\nslides=",e)?;
 w.list(&value.slides,e,|w,value|{w.delimiter(b"[",e)?;w.string(&value.id,e)?;comma(w,e)?;option(value.layout_id.as_ref(),w,e)?;comma(w,e)?;shapes(&value.shapes,w,e)?;comma(w,e)?;blocks(&value.notes,w,e)?;w.delimiter(b"]",e)})
}
