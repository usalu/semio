//! 🫳️ Immutable authored schema views for retained Record encoding without owned metadata copies.
use crate::{RecordLayout,ValueError,ValueRefusalKind};
use crate::native_encoding::FieldProjectionSource;
use semio_framework_value::NativeEncodeControl;

/// 🧩️ Borrows the same declarative shape vocabulary used by ordinary Record producers.
#[derive(Clone,Copy)]
pub enum BorrowedShape{
 Bool,Int,UInt,Float,Text,Bytes64,Enum(&'static[(&'static str,u32)]),
 Tuple(fn()->BorrowedShape,Option<usize>),List(fn()->BorrowedShape),Record(fn()->BorrowedRecordSpec),
 Block(fn()->BorrowedShape),Statements(&'static[(&'static str,fn()->BorrowedRecordSpec)]),
 Map(fn()->BorrowedShape),Value,Table(fn()->BorrowedRecordSpec),Wire,
 Quantity(&'static semio_framework_dsl::UnitSpec),Angle(&'static semio_framework_dsl::UnitSpec),Ref(&'static str),
 Coord(u8),Dir,Dim(u8),Range,Count,Expr,Embed(&'static str),EmbedFrom(&'static str),
}

/// 🏷️ One static authored field retains its identity, rank and shape without allocating a key.
#[derive(Clone,Copy)]
pub struct BorrowedFieldSpec{pub id:u16,pub key:&'static str,pub position:Option<u8>,pub shape:BorrowedShape,pub optional:bool,pub flatten:bool,pub defines:Option<&'static str>,pub is_call_name:bool}
impl BorrowedFieldSpec{
 /// 🏭️ Declares one keyed required field with the canonical ordinary metadata defaults.
 pub const fn new(id:u16,key:&'static str,shape:BorrowedShape)->Self{Self{id,key,position:None,shape,optional:false,flatten:false,defines:None,is_call_name:false}}
}

/// 📑️ Lazily borrows complete record metadata, including recursive record edges.
#[derive(Clone,Copy)]
pub struct BorrowedRecordSpec{pub keyword:Option<&'static str>,pub layout:RecordLayout,pub fields:&'static[BorrowedFieldSpec]}

semio_framework_value::artifact_retire_leaf!(BorrowedShape);
semio_framework_value::artifact_retire_leaf!(BorrowedRecordSpec);

/// 📏️ Exact canonical Document Text demand from a retained source and borrowed schema.
pub fn measure_print_borrowed<T:FieldProjectionSource>(source:&T,spec:&BorrowedRecordSpec,maximum:usize,control:&mut NativeEncodeControl<'_>)->Result<usize,ValueError>{crate::controlled_encoding::measure(source,spec,maximum,control)}
