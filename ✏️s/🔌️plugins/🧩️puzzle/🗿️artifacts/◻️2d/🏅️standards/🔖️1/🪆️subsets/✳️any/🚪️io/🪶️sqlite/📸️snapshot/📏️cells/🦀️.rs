//! 🧩️ Borrowed Puzzle2d native roles admit the complete authored board and catalog before ownership.
use super::{invalid,specificity,Puzzle2dSnapshot};
use crate::{Puzzle2dNodeAnchor,Puzzle2dCompatSpecificity};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::SqliteDatabaseLimits};
use semio_framework_dsl_record::{DslField,RecordValue as R,FieldValue as F};
use semio_framework_value::{ValueError,ValueRefusalKind,NativeDecodeControl as N};
type Result<T>=std::result::Result<T,ValueError>;
#[derive(Clone,Copy)]enum Role{Text,Float,Bool,Int,OptionalText,OptionalFloat,OptionalBool,OptionalInt,Anchor,Specificity}
use Role::*;
const WIDTHS:&[(&str,usize)]=&[("puzzle2d_attribute",7),("puzzle2d_author",8),("puzzle2d_base_kind",4),("puzzle2d_camera",11),("puzzle2d_catalog_edge_kind",9),("puzzle2d_catalog_handle_kind",11),("puzzle2d_catalog_node_kind",11),("puzzle2d_catalog_wire_kind",10),("puzzle2d_compatible_kind",4),("puzzle2d_document",2),("puzzle2d_edge",35),("puzzle2d_handle",18),("puzzle2d_handle_template",19),("puzzle2d_kind_catalogs",2),("puzzle2d_kind_compatibility",8),("puzzle2d_meta",3),("puzzle2d_node",30),("puzzle2d_representation",9),("puzzle2d_representation_tag",4),("puzzle2d_target_region",19)];
pub(super)fn extent(limits:SqliteDatabaseLimits)->Result<()>{if Puzzle2dSnapshot::SQLITE_SCHEMA.len()>limits.max_schema_bytes||limits.max_tables<WIDTHS.len()||limits.max_columns<35||limits.max_rows<3{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Puzzle2d authored schema extent exceeds caller limits"))}Ok(())}
fn add(a:usize,b:usize)->Result<usize>{a.checked_add(b).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Puzzle2d semantic extent overflow"))}
struct Census{limits:SqliteDatabaseLimits,rows:usize,bytes:usize}
impl Census{
 fn row(&mut self,table:&str,bytes:usize,n:&mut N<'_>)->Result<()>{n.step()?;if !WIDTHS.iter().any(|(name,_)|*name==table){return Err(invalid("Puzzle2d census table is not authored"))}let rows=add(self.rows,1)?;let bytes=add(self.bytes,bytes)?;if rows>self.limits.max_rows{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Puzzle2d semantic row limit exceeded"))}if bytes>self.limits.max_value_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Puzzle2d semantic value limit exceeded"))}self.rows=rows;self.bytes=bytes;Ok(())}
}
fn shape(record:&R,count:u16)->Result<()>{if record.fields.keys().copied().eq(0..count){Ok(())}else{Err(invalid("Puzzle2d native record field map differs"))}}
fn field(record:&R,id:u16)->Result<&F>{record.fields.get(&id).ok_or_else(||invalid("Puzzle2d required native role is absent"))}
fn record(value:&F)->Result<&R>{match value{F::Record(value)=>Ok(value),_=>Err(invalid("Puzzle2d native role requires record"))}}
fn block(value:&F)->Result<&R>{match value{F::Block(value)=>record(value),_=>Err(invalid("Puzzle2d native role requires declared block"))}}
fn items(value:&F)->Result<&[F]>{match value{F::List(value)=>Ok(value),_=>Err(invalid("Puzzle2d native role requires ordered list"))}}
fn scalar<T:DslField>(value:&F,n:&mut N<'_>)->Result<T>{n.scoped_stage(|n|{n.begin_stage(0)?;T::from_value_controlled(value,n)})}
fn cost(value:&F,role:Role,n:&mut N<'_>)->Result<usize>{match role{
 Text=>{n.step()?;match value{F::Text(value)=>Ok(value.len()),_=>Err(invalid("Puzzle2d native role requires text"))}},
 Float=>{let value=scalar::<f64>(value,n)?;Ok(if value.is_nan(){11}else if value.is_infinite(){32}else{22})},
 Bool=>{let _=scalar::<bool>(value,n)?;Ok(8)},Int=>{let _=scalar::<i32>(value,n)?;Ok(8)},
 OptionalText|OptionalFloat|OptionalBool|OptionalInt=>if matches!(value,F::Absent){n.step()?;Ok(0)}else{cost(value,match role{OptionalText=>Text,OptionalFloat=>Float,OptionalBool=>Bool,OptionalInt=>Int,_=>unreachable!()},n)},
 Anchor=>Ok(match scalar::<Puzzle2dNodeAnchor>(value,n)?{Puzzle2dNodeAnchor::Fixed=>"fixed",Puzzle2dNodeAnchor::Derived=>"derived"}.len()),
 Specificity=>Ok(specificity(scalar::<Puzzle2dCompatSpecificity>(value,n)?).len())
}}
fn fields(record:&R,roles:&[(u16,Role)],n:&mut N<'_>)->Result<usize>{let mut bytes=0;for(id,role)in roles{bytes=add(bytes,cost(field(record,*id)?,*role,n)?)?;}Ok(bytes)}
fn row(value:&F,count:u16,table:&str,base:usize,roles:&[(u16,Role)],c:&mut Census,n:&mut N<'_>)->Result<()>{let value=record(value)?;shape(value,count)?;c.row(table,add(base,fields(value,roles,n)?)?,n)}
fn collection(value:&F,count:u16,table:&str,roles:&[(u16,Role)],c:&mut Census,n:&mut N<'_>)->Result<()>{for value in items(value)?{row(value,count,table,24,roles,c,n)?;}Ok(())}
fn texts(value:&F,table:&str,c:&mut Census,n:&mut N<'_>)->Result<()>{for value in items(value)?{c.row(table,add(24,cost(value,Text,n)?)?,n)?;}Ok(())}
pub(super)fn admit_record(source:&R,limits:SqliteDatabaseLimits,n:&mut N<'_>)->Result<()>{
 extent(limits)?;n.scoped_stage(|n|{
  n.begin_stage(0)?;shape(source,6)?;let mut c=Census{limits,rows:0,bytes:0};
  c.row("puzzle2d_document",add(8,fields(source,&[(0,Text)],n)?)?,n)?;
  let camera=block(field(source,1)?)?;shape(camera,3)?;
  c.row("puzzle2d_camera",add(16,fields(camera,&[(0,Float),(1,Float),(2,Float)],n)?)?,n)?;
  for value in items(field(source,2)?)?{
   let value=record(value)?;shape(value,16)?;
   c.row("puzzle2d_node",add(24,fields(value,&[(0,Text),(1,OptionalText),(2,OptionalText),(3,Float),(4,Float),(5,OptionalFloat),(6,OptionalFloat),(7,OptionalFloat),(8,OptionalText),(9,OptionalText),(10,OptionalBool),(11,OptionalFloat),(12,OptionalBool),(13,OptionalBool),(14,Anchor)],n)?)?,n)?;
   collection(field(value,15)?,9,"puzzle2d_handle",&[(0,Text),(1,OptionalText),(2,Float),(3,OptionalFloat),(4,OptionalText),(5,OptionalText),(6,OptionalFloat),(7,OptionalBool),(8,OptionalBool)],&mut c,n)?;
  }
  collection(field(source,3)?,16,"puzzle2d_edge",&[(0,Text),(1,Text),(2,Text),(3,OptionalText),(4,Float),(5,Float),(6,Float),(7,Float),(8,Float),(9,Float),(10,Float),(11,Float),(12,OptionalText),(13,OptionalText),(14,OptionalBool),(15,OptionalBool)],&mut c,n)?;
  collection(field(source,4)?,8,"puzzle2d_target_region",&[(0,Text),(1,Float),(2,Float),(3,Float),(4,Float),(5,OptionalText),(6,Bool),(7,Bool)],&mut c,n)?;
  let meta=block(field(source,5)?)?;shape(meta,3)?;
  c.row("puzzle2d_meta",add(16,fields(meta,&[(0,OptionalText)],n)?)?,n)?;
  collection(field(meta,1)?,5,"puzzle2d_kind_compatibility",&[(0,Text),(1,Text),(2,Bool),(3,Bool),(4,Specificity)],&mut c,n)?;
  match field(meta,2)?{
   F::Absent=>n.step()?,
   value=>{
    let catalogs=record(value)?;shape(catalogs,4)?;c.row("puzzle2d_kind_catalogs",16,n)?;
    for value in items(field(catalogs,0)?)?{
     let value=record(value)?;shape(value,13)?;
     c.row("puzzle2d_catalog_node_kind",add(24,fields(value,&[(0,Text),(1,Text),(2,Text),(3,Text),(4,Text),(5,Text),(6,Text),(7,Bool)],n)?)?,n)?;
     texts(field(value,8)?,"puzzle2d_base_kind",&mut c,n)?;
     for representation in items(field(value,9)?)?{
      let representation=record(representation)?;shape(representation,7)?;
      c.row("puzzle2d_representation",add(24,fields(representation,&[(0,Text),(1,Text),(2,Text),(3,Text),(5,OptionalText),(6,Text)],n)?)?,n)?;
      texts(field(representation,4)?,"puzzle2d_representation_tag",&mut c,n)?;
     }
     collection(field(value,10)?,10,"puzzle2d_handle_template",&[(0,Text),(1,Text),(2,Text),(3,Text),(4,Text),(5,OptionalText),(6,Float),(7,OptionalFloat),(8,OptionalBool),(9,OptionalFloat)],&mut c,n)?;
     collection(field(value,11)?,4,"puzzle2d_attribute",&[(0,Text),(1,Text),(2,Text),(3,OptionalText)],&mut c,n)?;
     collection(field(value,12)?,5,"puzzle2d_author",&[(0,Text),(1,Text),(2,Text),(3,OptionalText),(4,OptionalInt)],&mut c,n)?;
    }
    for value in items(field(catalogs,1)?)?{
     let value=record(value)?;shape(value,9)?;
     c.row("puzzle2d_catalog_handle_kind",add(24,fields(value,&[(0,Text),(1,OptionalText),(2,OptionalText),(3,OptionalInt),(5,Text),(6,Text),(7,Text),(8,Text)],n)?)?,n)?;
     texts(field(value,4)?,"puzzle2d_compatible_kind",&mut c,n)?;
    }
    collection(field(catalogs,2)?,6,"puzzle2d_catalog_edge_kind",&[(0,Text),(1,Text),(2,Text),(3,Text),(4,Text),(5,Text)],&mut c,n)?;
    collection(field(catalogs,3)?,7,"puzzle2d_catalog_wire_kind",&[(0,Text),(1,Text),(2,Text),(3,Text),(4,Text),(5,Text),(6,Text)],&mut c,n)?;
   }
  }
  n.checkpoint()
 })
}
