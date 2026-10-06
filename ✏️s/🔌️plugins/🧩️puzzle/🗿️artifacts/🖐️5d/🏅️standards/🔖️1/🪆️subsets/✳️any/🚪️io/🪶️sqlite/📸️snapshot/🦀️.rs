//! 🧩️ Complete Puzzle5d entities and literal owned relationships.
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;
use crate::*;
use std::collections::{BTreeMap,BTreeSet};
use store::{ArtifactSqliteSnapshot,sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteValue,SqliteSnapshotControl,SqliteSnapshotPhase,SnapshotEncoding,artifact::{Cell,Projection,Reconstruction}}};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:impl Into<String>)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}

const TABLES:[(&str,usize);25]=[("puzzle5_document",5),("puzzle5_catalog_child",7),("puzzle5_catalog_extra",2),("puzzle5_part_kind",11),("puzzle5_part_kind_base",4),("puzzle5_representation",9),("puzzle5_representation_tag",4),("puzzle5_grip_template",34),("puzzle5_attribute",7),("puzzle5_author",8),("puzzle5_grip_kind",11),("puzzle5_grip_kind_compatible",4),("puzzle5_fastener_kind",6),("puzzle5_rope_kind",7),("puzzle5_compatibility",8),("puzzle5_part",6),("puzzle5_part_board",22),("puzzle5_part_world",25),("puzzle5_part_scale",15),("puzzle5_grip",5),("puzzle5_grip_board",9),("puzzle5_grip_world",24),("puzzle5_fastener",31),("puzzle5_target_volume",27),("puzzle5_target_scale",15)];
fn class(value:f64)->&'static str{if value.is_nan(){"nan"}else if value==f64::INFINITY{"positiveInfinity"}else if value==f64::NEG_INFINITY{"negativeInfinity"}else{"finite"}}
fn ordinal(value:usize)->Result<Cell<'static>,ValueError>{Ok(Cell::Integer(i64::try_from(value).map_err(|e|invalid(e.to_string()))?))}
fn optional_text(value:&Option<String>)->Cell<'_>{value.as_deref().map(Cell::Text).unwrap_or(Cell::Null)}
fn optional_bool(value:Option<bool>)->Cell<'static>{value.map(|v|Cell::Integer(i64::from(v))).unwrap_or(Cell::Null)}
fn optional_i32(value:Option<i32>)->Cell<'static>{value.map(|v|Cell::Integer(i64::from(v))).unwrap_or(Cell::Null)}
struct Cells<'a>{values:[Cell<'a>;40],len:usize}
impl<'a> Cells<'a>{
 fn new(values:&[Cell<'a>])->Self{let mut row=Self{values:[Cell::Null;40],len:0};for value in values{row.push(*value);}row}
 fn push(&mut self,value:Cell<'a>){self.values[self.len]=value;self.len+=1;}
 fn f64(&mut self,value:f64){self.push(if value.is_nan(){Cell::Null}else{Cell::Real(value)});self.push(Cell::Integer(value.to_bits() as i64));self.push(Cell::Text(class(value)));}
 fn optional(&mut self,value:Option<f64>){if let Some(value)=value{self.f64(value);}else{for _ in 0..3{self.push(Cell::Null);}}}
 fn xyz(&mut self,value:[f64;3]){for value in value{self.f64(value);}}
 fn direction(&mut self,value:Option<[f64;3]>){for i in 0..3{self.optional(value.map(|v|v[i]));}}
 fn orientation(&mut self,value:Option<[f64;4]>){for i in 0..4{self.optional(value.map(|v|v[i]));}}
 fn scale(&mut self,value:Puzzle5dScale){match value{Puzzle5dScale::Uniform(v)=>{self.push(Cell::Text("uniform"));self.f64(v);self.direction(None);},Puzzle5dScale::Vec3(v)=>{self.push(Cell::Text("vec3"));self.optional(None);self.xyz(v);}}}
 fn insert(&self,out:&mut Projection<'_,'_>,table:usize)->Result<i64,ValueError>{out.insert(TABLES[table].0,&self.values[..self.len])}
}
fn add(total:&mut usize,count:usize,c:&SqliteSnapshotControl<'_>)->Result<(),ValueError>{*total=total.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Puzzle5d row count overflow"))?;c.check_rows(*total)}
fn admit_schema(c:&SqliteSnapshotControl<'_>)->Result<(),ValueError>{if Puzzle5dSnapshot::SQLITE_SCHEMA.len()>c.limits().max_schema_bytes{return Err(ValueError::new(ValueRefusalKind::OwnershipLimit,"Puzzle5d authored SQL exceeds schema byte limit"));}Ok(())}
fn forecast(v:&Puzzle5dSnapshot,c:&mut SqliteSnapshotControl<'_>,phase:SqliteSnapshotPhase)->Result<usize,ValueError>{
 admit_schema(c)?;let mut total=1;c.check_rows(total)?;c.checkpoint(phase,0,0)?;add(&mut total,usize::from(v.kind_catalogs.is_some()),c)?;add(&mut total,v.kind_compatibility.len(),c)?;add(&mut total,v.fasteners.len(),c)?;
 for(n,p)in v.parts.iter().enumerate(){add(&mut total,3+usize::from(p.part_3d.scale.is_some()),c)?;add(&mut total,p.grips.len().checked_mul(3).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Puzzle5d grip count overflow"))?,c)?;if n%256==0{c.checkpoint(phase,n,v.parts.len())?;}}
 for(n,t)in v.target_volumes.iter().enumerate(){add(&mut total,1+usize::from(t.scale.is_some()),c)?;if n%256==0{c.checkpoint(phase,n,v.target_volumes.len())?;}}
 if let Some(e)=&v.kind_catalogs_extra{add(&mut total,1,c)?;add(&mut total,e.fasteners.len(),c)?;add(&mut total,e.ropes.len(),c)?;for(n,p)in e.parts.iter().enumerate(){for count in[1,p.base_kinds.len(),p.grips.len(),p.attributes.len(),p.authors.len()]{add(&mut total,count,c)?;}for(i,r)in p.representations.iter().enumerate(){add(&mut total,1,c)?;add(&mut total,r.tags.len(),c)?;if i%256==0{c.checkpoint(phase,i,p.representations.len())?;}}if n%256==0{c.checkpoint(phase,n,e.parts.len())?;}}for(n,g)in e.grips.iter().enumerate(){add(&mut total,1,c)?;add(&mut total,g.compatible_with.len(),c)?;if n%256==0{c.checkpoint(phase,n,e.grips.len())?;}}}
 c.checkpoint(phase,total,total)?;Ok(total)
}
fn native_record(value:Option<&semio_framework_dsl_record::FieldValue>)->Result<Option<&semio_framework_dsl_record::RecordValue>,ValueError>{match value{None|Some(semio_framework_dsl_record::FieldValue::Absent)=>Ok(None),Some(semio_framework_dsl_record::FieldValue::Record(record))=>Ok(Some(record)),_=>Err(invalid("Puzzle5d native entity requires its literal record"))}}
fn native_list(value:Option<&semio_framework_dsl_record::FieldValue>)->Result<&[semio_framework_dsl_record::FieldValue],ValueError>{match value{None|Some(semio_framework_dsl_record::FieldValue::Absent)=>Ok(&[]),Some(semio_framework_dsl_record::FieldValue::List(values))=>Ok(values),_=>Err(invalid("Puzzle5d native collection requires its literal list"))}}
fn native_add(total:&mut usize,count:usize,maximum:usize)->Result<(),ValueError>{*total=total.checked_add(count).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Puzzle5d native entity count overflow"))?;if *total>maximum{return Err(ValueError::new(ValueRefusalKind::WorkLimit,"Puzzle5d native entity count exceeds row limit"));}Ok(())}
fn admit_native_rows(record:&semio_framework_dsl_record::RecordValue,maximum:usize,native:&mut semio_framework_value::NativeDecodeControl<'_>)->Result<(),semio_framework_value::ValueError>{
 native.scoped_stage(|native|->Result<(),ValueError>{
  let mut count=1;native_add(&mut count,usize::from(native_record(record.get(4))?.is_some()),maximum)?;native_add(&mut count,native_list(record.get(6))?.len(),maximum)?;native_add(&mut count,native_list(record.get(8))?.len(),maximum)?;
  let parts=native_list(record.get(7))?;native.begin_stage(parts.len())?;for part in parts{let p=native_record(Some(part))?.ok_or_else(||invalid("Puzzle5d part missing record"))?;let w=native_record(p.get(4))?;let scale=w.and_then(|w|w.get(3)).is_some_and(|v|!matches!(v,semio_framework_dsl_record::FieldValue::Absent));native_add(&mut count,3+usize::from(scale),maximum)?;native_add(&mut count,native_list(p.get(5))?.len().checked_mul(3).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Puzzle5d native grip count overflow"))?,maximum)?;native.step()?;}
  let targets=native_list(record.get(9))?;native.begin_stage(targets.len())?;for target in targets{let t=native_record(Some(target))?.ok_or_else(||invalid("Puzzle5d target missing record"))?;native_add(&mut count,1+usize::from(t.get(3).is_some_and(|v|!matches!(v,semio_framework_dsl_record::FieldValue::Absent))),maximum)?;native.step()?;}
  if let Some(extra)=native_record(record.get(5))?{native_add(&mut count,1,maximum)?;native_add(&mut count,native_list(extra.get(2))?.len(),maximum)?;native_add(&mut count,native_list(extra.get(3))?.len(),maximum)?;let parts=native_list(extra.get(0))?;native.begin_stage(parts.len())?;for part in parts{let p=native_record(Some(part))?.ok_or_else(||invalid("Puzzle5d catalog part missing record"))?;for number in[1,native_list(p.get(8))?.len(),native_list(p.get(10))?.len(),native_list(p.get(11))?.len(),native_list(p.get(12))?.len()]{native_add(&mut count,number,maximum)?;}let representations=native_list(p.get(9))?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(representations.len())?;for representation in representations{let r=native_record(Some(representation))?.ok_or_else(||invalid("Puzzle5d representation missing record"))?;native_add(&mut count,1,maximum)?;native_add(&mut count,native_list(r.get(4))?.len(),maximum)?;native.step()?;}Ok(())})?;native.step()?;}let grips=native_list(extra.get(1))?;native.begin_stage(grips.len())?;for grip in grips{let g=native_record(Some(grip))?.ok_or_else(||invalid("Puzzle5d catalog grip missing record"))?;native_add(&mut count,1,maximum)?;native_add(&mut count,native_list(g.get(4))?.len(),maximum)?;native.step()?;}}
  native.checkpoint()?;Ok(())
 })
}
impl ArtifactSqliteSnapshot for Puzzle5dSnapshot{
 const SQLITE_SCHEMA:&'static str=include_str!("🗄️.sql");
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{admit_schema(c)?;c.check_rows(1)?;let maximum=c.limits().max_rows;store::decode_sqlite_snapshot_record_native(payload,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|record,native|{admit_native_rows(record,maximum,native)?;Self::__dsl_from_record_controlled(record,native)},c)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{forecast(self,c,SqliteSnapshotPhase::EncodeNative)?;store::encode_sqlite_snapshot_record_native(encoding,<Self as store::ArtifactDsl>::envelope_id(),Self::__dsl_spec_producer(),|native|self.__dsl_to_record_controlled(native),c)}
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{
  let total=forecast(self,c,SqliteSnapshotPhase::ProjectSnapshot)?;let mut out=Projection::new(Self::SQLITE_SCHEMA,c)?;
  let doc=out.insert(TABLES[0].0,&[Cell::Text(&self.schema),Cell::Text(&self.domain),optional_text(&self.label),Cell::Text(&self.meta.description)])?;
  if let Some(child)=&self.kind_catalogs{let d=&child.target.dialect;out.insert(TABLES[1].0,&[Cell::Integer(doc),Cell::Text(&child.child_id),Cell::Text(&child.target.artifact_id),Cell::Text(&d.artifact_kind),Cell::Text(&d.standard),Cell::Text(&d.subset)])?;}
  if let Some(extra)=&self.kind_catalogs_extra{
   let owner=out.insert(TABLES[2].0,&[Cell::Integer(doc)])?;
   for(n,k)in extra.parts.iter().enumerate(){let id=out.insert(TABLES[3].0,&[Cell::Integer(owner),ordinal(n)?,Cell::Text(&k.id),Cell::Text(&k.name),Cell::Text(&k.label),Cell::Text(&k.description),Cell::Text(&k.icon),Cell::Text(&k.image),Cell::Text(&k.unit),Cell::Integer(i64::from(k.is_abstract))])?;
    for(i,value)in k.base_kinds.iter().enumerate(){out.insert(TABLES[4].0,&[Cell::Integer(id),ordinal(i)?,Cell::Text(value)])?;}
    for(i,r)in k.representations.iter().enumerate(){let rid=out.insert(TABLES[5].0,&[Cell::Integer(id),ordinal(i)?,Cell::Text(&r.id),Cell::Text(&r.name),Cell::Text(&r.url),Cell::Text(&r.mime),optional_text(&r.lod),Cell::Text(&r.description)])?;for(j,tag)in r.tags.iter().enumerate(){out.insert(TABLES[6].0,&[Cell::Integer(rid),ordinal(j)?,Cell::Text(tag)])?;}}
    for(i,t)in k.grips.iter().enumerate(){let mut row=Cells::new(&[Cell::Integer(id),ordinal(i)?,Cell::Text(&t.id),Cell::Text(&t.name),Cell::Text(&t.label),Cell::Text(&t.description),Cell::Text(&t.icon),optional_text(&t.grip_kind)]);row.xyz(t.point);row.xyz(t.direction);row.optional(t.t);row.push(optional_bool(t.mandatory));row.optional(t.radius);row.insert(&mut out,7)?;}
    for(i,a)in k.attributes.iter().enumerate(){out.insert(TABLES[8].0,&[Cell::Integer(id),ordinal(i)?,Cell::Text(&a.id),Cell::Text(&a.key),Cell::Text(&a.value),optional_text(&a.definition)])?;}
    for(i,a)in k.authors.iter().enumerate(){out.insert(TABLES[9].0,&[Cell::Integer(id),ordinal(i)?,Cell::Text(&a.id),Cell::Text(&a.name),Cell::Text(&a.email),optional_text(&a.role),optional_i32(a.rank)])?;}
   }
   for(n,k)in extra.grips.iter().enumerate(){let id=out.insert(TABLES[10].0,&[Cell::Integer(owner),ordinal(n)?,Cell::Text(&k.id),optional_text(&k.code),optional_text(&k.label),optional_i32(k.order),Cell::Text(&k.description),Cell::Text(&k.icon),Cell::Text(&k.color),Cell::Text(&k.default_rope_kind)])?;for(i,value)in k.compatible_with.iter().enumerate(){out.insert(TABLES[11].0,&[Cell::Integer(id),ordinal(i)?,Cell::Text(value)])?;}}
   for(n,k)in extra.fasteners.iter().enumerate(){out.insert(TABLES[12].0,&[Cell::Integer(owner),ordinal(n)?,Cell::Text(&k.id),Cell::Text(&k.name),optional_text(&k.label)])?;}
   for(n,k)in extra.ropes.iter().enumerate(){out.insert(TABLES[13].0,&[Cell::Integer(owner),ordinal(n)?,Cell::Text(&k.id),Cell::Text(&k.name),Cell::Text(&k.label),Cell::Text(&k.default_fastener_kind)])?;}
  }
  for(n,k)in self.kind_compatibility.iter().enumerate(){let kind=match k.specificity{Puzzle5dCompatSpecificity::General=>"general",Puzzle5dCompatSpecificity::Part=>"part",Puzzle5dCompatSpecificity::Fastener=>"fastener",Puzzle5dCompatSpecificity::Grip=>"grip",Puzzle5dCompatSpecificity::Rope=>"rope"};out.insert(TABLES[14].0,&[Cell::Integer(doc),ordinal(n)?,Cell::Text(&k.source),Cell::Text(&k.target),Cell::Integer(i64::from(k.bidirectional)),Cell::Integer(i64::from(k.important)),Cell::Text(kind)])?;}
  for(n,p)in self.parts.iter().enumerate(){let id=out.insert(TABLES[15].0,&[Cell::Integer(doc),ordinal(n)?,Cell::Text(&p.id),optional_text(&p.part_kind),Cell::Text(match p.anchor{Puzzle5dPartAnchor::Fixed=>"fixed",Puzzle5dPartAnchor::Derived=>"derived"})])?;let b=&p.part_2d;let w=&p.part_3d;
   let mut board=Cells::new(&[Cell::Integer(id)]);board.f64(b.x);board.f64(b.y);board.push(optional_text(&b.shape));board.optional(b.radius);board.optional(b.width);board.optional(b.height);for cell in[optional_text(&b.text),optional_text(&b.icon_kind),optional_bool(b.hidden),optional_bool(b.locked)]{board.push(cell);}board.insert(&mut out,16)?;
   let mut world=Cells::new(&[Cell::Integer(id)]);world.xyz(w.origin);world.push(optional_text(&w.mesh_url));world.orientation(w.orientation);world.push(optional_text(&w.label));world.insert(&mut out,17)?;
   if let Some(scale)=w.scale{let mut row=Cells::new(&[Cell::Integer(id)]);row.scale(scale);row.insert(&mut out,18)?;}
   for(i,g)in p.grips.iter().enumerate(){let gid=out.insert(TABLES[19].0,&[Cell::Integer(id),ordinal(i)?,Cell::Text(&g.id),optional_text(&g.grip_kind)])?;let b=&g.grip_2d;let w=&g.grip_3d;let mut board=Cells::new(&[Cell::Integer(gid)]);board.f64(b.angle);board.push(optional_text(&b.grip_kind));board.optional(b.radius);board.insert(&mut out,20)?;let mut world=Cells::new(&[Cell::Integer(gid)]);world.xyz(w.position);world.direction(w.direction);world.optional(w.radius);world.push(optional_text(&w.label));world.insert(&mut out,21)?;}
   if n%256==0{out.checkpoint_total(total)?;}
  }
  for(n,f)in self.fasteners.iter().enumerate(){let mut row=Cells::new(&[Cell::Integer(doc),ordinal(n)?,Cell::Text(&f.id),Cell::Text(&f.source),Cell::Text(&f.target),optional_text(&f.fastener_kind)]);for value in[f.gap,f.shift,f.rise,f.rotation,f.turn,f.tilt,f.x,f.y]{row.f64(value);}row.insert(&mut out,22)?;}
  for(n,t)in self.target_volumes.iter().enumerate(){let mut row=Cells::new(&[Cell::Integer(doc),ordinal(n)?,Cell::Text(&t.id)]);row.xyz(t.origin);row.orientation(t.orientation);row.push(Cell::Integer(i64::from(t.hidden)));row.push(Cell::Integer(i64::from(t.locked)));let id=row.insert(&mut out,23)?;if let Some(scale)=t.scale{let mut row=Cells::new(&[Cell::Integer(id)]);row.scale(scale);row.insert(&mut out,24)?;}}
  out.checkpoint_total(total)?;out.finish()
 }
 fn from_sqlite_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{reconstruct(d,c)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,_d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1).map_err(store::io_schema::IoError::from_value_error)?;if dialect.artifact_kind!="s.puzzle.puzzle5d"||dialect.standard!="1"||dialect.subset!="*"{return Err(store::io_schema::IoError::from_value_error(invalid("Puzzle5d owned dialect differs")));}c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1).map_err(store::io_schema::IoError::from_value_error)?;Ok(store::io_schema::IoOutcome::clean(()))}
}
struct Cursor<'a,'c,'p>{row:&'a SqliteRow,index:usize,control:&'c mut SqliteSnapshotControl<'p>}
impl<'a,'c,'p> Cursor<'a,'c,'p>{
 fn new(row:&'a SqliteRow,index:usize,control:&'c mut SqliteSnapshotControl<'p>)->Self{Self{row,index,control}}
 fn next(&mut self)->Result<&'a SqliteValue,ValueError>{let value=self.row.values.get(self.index).ok_or_else(||invalid("Puzzle5d missing cell"))?;self.index+=1;Ok(value)}
 fn borrowed(&mut self)->Result<&'a str,ValueError>{match self.next()?{SqliteValue::Text(v)=>Ok(v),_=>Err(invalid("Puzzle5d TEXT required"))}}
 fn text(&mut self)->Result<String,ValueError>{let value=self.borrowed()?;Reconstruction::new(self.control)?.text(value)}
 fn integer(&mut self)->Result<i64,ValueError>{match self.next()?{SqliteValue::Integer(v)=>Ok(*v),_=>Err(invalid("Puzzle5d INTEGER required"))}}
 fn optional_text(&mut self)->Result<Option<String>,ValueError>{if self.row.values.get(self.index)==Some(&SqliteValue::Null){self.next()?;Ok(None)}else{self.text().map(Some)}}
 fn boolean(&mut self)->Result<bool,ValueError>{let value=match self.integer()?{0=>false,1=>true,_=>return Err(invalid("Puzzle5d boolean differs"))};Reconstruction::new(self.control)?.scalar()?;Ok(value)}
 fn optional_bool(&mut self)->Result<Option<bool>,ValueError>{if self.row.values.get(self.index)==Some(&SqliteValue::Null){self.next()?;Ok(None)}else{self.boolean().map(Some)}}
 fn optional_i32(&mut self)->Result<Option<i32>,ValueError>{if self.row.values.get(self.index)==Some(&SqliteValue::Null){self.next()?;Ok(None)}else{let value=i32::try_from(self.integer()?).map_err(|e|invalid(e.to_string()))?;Reconstruction::new(self.control)?.scalar()?;Ok(Some(value))}}
 fn absent(&mut self)->Result<(),ValueError>{if self.next()?!=&SqliteValue::Null{return Err(invalid("Puzzle5d partial optional presence"));}Ok(())}
 fn f64(&mut self)->Result<f64,ValueError>{let query=self.next()?;let value=f64::from_bits(self.integer()? as u64);if self.borrowed()?!=class(value){return Err(invalid("Puzzle5d IEEE class differs"));}let exact=if value.is_nan(){query==&SqliteValue::Null}else{match query{SqliteValue::Real(v)=>*v==value,SqliteValue::Integer(v)=>value.is_finite()&&value.fract()==0.0&&value>=i64::MIN as f64&&value < -(i64::MIN as f64)&&value as i64==*v,_=>false}};if !exact{return Err(invalid("Puzzle5d IEEE query differs"));}Reconstruction::new(self.control)?.scalar()?;Ok(value)}
 fn optional_float(&mut self)->Result<Option<f64>,ValueError>{if self.row.values.get(self.index+1)==Some(&SqliteValue::Null){self.absent()?;self.absent()?;self.absent()?;Ok(None)}else{self.f64().map(Some)}}
 fn xyz(&mut self)->Result<[f64;3],ValueError>{Ok([self.f64()?,self.f64()?,self.f64()?])}
 fn direction(&mut self)->Result<Option<[f64;3]>,ValueError>{match(self.optional_float()?,self.optional_float()?,self.optional_float()?){(None,None,None)=>Ok(None),(Some(x),Some(y),Some(z))=>Ok(Some([x,y,z])),_=>Err(invalid("Puzzle5d partial vector presence"))}}
 fn orientation(&mut self)->Result<Option<[f64;4]>,ValueError>{match(self.optional_float()?,self.optional_float()?,self.optional_float()?,self.optional_float()?){(None,None,None,None)=>Ok(None),(Some(w),Some(x),Some(y),Some(z))=>Ok(Some([w,x,y,z])),_=>Err(invalid("Puzzle5d partial quaternion presence"))}}
 fn scale(&mut self)->Result<Puzzle5dScale,ValueError>{let kind=self.borrowed()?;let uniform=self.optional_float()?;let axes=self.direction()?;match(kind,uniform,axes){("uniform",Some(v),None)=>Ok(Puzzle5dScale::Uniform(v)),("vec3",None,Some(v))=>Ok(Puzzle5dScale::Vec3(v)),_=>Err(invalid("Puzzle5d scale variant fields differ"))}}
 fn done(self)->Result<(),ValueError>{if self.index!=self.row.values.len(){return Err(invalid("Puzzle5d extra cell"));}Ok(())}
}
struct Rows<'a>{groups:[BTreeMap<i64,BTreeMap<i64,&'a SqliteRow>>;25]}
impl<'a> Rows<'a>{
 fn new(d:&'a SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  store::sqlite_snapshot::validate_sqlite_database_schema(d,Puzzle5dSnapshot::SQLITE_SCHEMA,c.limits())?;c.check_database(d,SqliteSnapshotPhase::ReconstructSnapshot)?;
  let mut ids:[BTreeSet<i64>;25]=std::array::from_fn(|_|BTreeSet::new());let mut groups=std::array::from_fn(|_|BTreeMap::<i64,BTreeMap<i64,&SqliteRow>>::new());
  for(table,(name,width))in TABLES.iter().enumerate(){let rows=&d.table(name)?.rows;for(n,row)in rows.iter().enumerate(){if row.rowid<=0||row.integer(0)?!=row.rowid||row.values.len()!=*width||!ids[table].insert(row.rowid){return Err(invalid("Puzzle5d row alias/width/identity differs"));}if n%256==0{c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,n,rows.len())?;}}}
  let parents:[usize;25]=[0,0,0,2,3,3,5,3,3,3,2,10,2,2,0,0,15,15,15,15,19,19,0,0,23];
  for(table,(name,_))in TABLES.iter().enumerate(){let rows=&d.table(name)?.rows;for(n,row)in rows.iter().enumerate(){let parent=if table==0{0}else{row.integer(1)?};if table!=0&&!ids[parents[table]].contains(&parent){return Err(invalid("Puzzle5d orphan owned relationship"));}let order=if matches!(table,0|1|2|16|17|18|20|21|24){row.rowid}else{let order=row.integer(2)?;if order<0{return Err(invalid("Puzzle5d negative ordinal"));}order};if groups[table].entry(parent).or_default().insert(order,row).is_some(){return Err(invalid("Puzzle5d duplicate ownership ordinal"));}if n%256==0{c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,n,rows.len())?;}}}
  Ok(Self{groups})
 }
 fn take(&mut self,table:usize,parent:i64,c:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{let rows=self.groups[table].remove(&parent).unwrap_or_default();let total=rows.len();c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,total)?;let mut result=Vec::new();result.try_reserve_exact(total).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"Puzzle5d relationship allocation failed"))?;for(n,(order,row))in rows.into_iter().enumerate(){if order!=i64::try_from(n).map_err(|e|invalid(e.to_string()))?{return Err(invalid("Puzzle5d noncontiguous ordinal"));}result.push(row);if(n+1)%256==0{c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,n+1,total)?;}}c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,total,total)?;Ok(result)}
 fn one(&mut self,table:usize,parent:i64,required:bool)->Result<Option<&'a SqliteRow>,ValueError>{let rows=self.groups[table].remove(&parent).unwrap_or_default();if rows.len()>1||required&&rows.len()!=1{return Err(invalid("Puzzle5d owned relationship cardinality differs"));}Ok(rows.into_values().next())}
 fn strings(&mut self,table:usize,parent:i64,c:&mut SqliteSnapshotControl<'_>)->Result<Vec<String>,ValueError>{let mut result=Vec::new();for row in self.take(table,parent,c)?{let mut value=Cursor::new(row,3,c);result.push(value.text()?);value.done()?;}Ok(result)}
 fn scale(&mut self,table:usize,parent:i64,c:&mut SqliteSnapshotControl<'_>)->Result<Option<Puzzle5dScale>,ValueError>{if let Some(row)=self.one(table,parent,false)?{let mut value=Cursor::new(row,2,c);let scale=value.scale()?;value.done()?;Ok(Some(scale))}else{Ok(None)}}
 fn finish(self,c:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{for groups in self.groups{if !groups.is_empty(){return Err(invalid("Puzzle5d unowned entity"));}}c.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,1,1)}
}
fn reconstruct(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Puzzle5dSnapshot,ValueError>{
 let mut rows=Rows::new(d,c)?;let doc=rows.one(0,0,true)?.ok_or_else(||invalid("Puzzle5d document missing"))?;let mut root=Cursor::new(doc,1,c);let schema=root.text()?;let domain=root.text()?;let label=root.optional_text()?;let meta=Puzzle5dMeta{description:root.text()?};root.done()?;
 let kind_catalogs=if let Some(row)=rows.one(1,doc.rowid,false)?{let mut value=Cursor::new(row,2,c);let child_id=value.text()?;let target=store::os_io::ArtifactRef{artifact_id:value.text()?,dialect:store::os_io::ArtifactDialect{artifact_kind:value.text()?,standard:value.text()?,subset:value.text()?}};value.done()?;Some(store::ArtifactChild::new(child_id,target))}else{None};
 let kind_catalogs_extra=if let Some(extra)=rows.one(2,doc.rowid,false)?{
  let(mut parts,mut grips,mut fasteners,mut ropes)=(Vec::new(),Vec::new(),Vec::new(),Vec::new());
  for row in rows.take(3,extra.rowid,c)?{let mut value=Cursor::new(row,3,c);let id=value.text()?;let name=value.text()?;let label=value.text()?;let description=value.text()?;let icon=value.text()?;let image=value.text()?;let unit=value.text()?;let is_abstract=value.boolean()?;value.done()?;
   let base_kinds=rows.strings(4,row.rowid,c)?;let(mut representations,mut templates,mut attributes,mut authors)=(Vec::new(),Vec::new(),Vec::new(),Vec::new());
   for row in rows.take(5,row.rowid,c)?{let mut value=Cursor::new(row,3,c);let id=value.text()?;let name=value.text()?;let url=value.text()?;let mime=value.text()?;let lod=value.optional_text()?;let description=value.text()?;value.done()?;let tags=rows.strings(6,row.rowid,c)?;representations.push(Puzzle5dRepresentation{id,name,url,mime,tags,lod,description});}
   for row in rows.take(7,row.rowid,c)?{let mut value=Cursor::new(row,3,c);templates.push(Puzzle5dGripTemplate{id:value.text()?,name:value.text()?,label:value.text()?,description:value.text()?,icon:value.text()?,grip_kind:value.optional_text()?,point:value.xyz()?,direction:value.xyz()?,t:value.optional_float()?,mandatory:value.optional_bool()?,radius:value.optional_float()?});value.done()?;}
   for row in rows.take(8,row.rowid,c)?{let mut value=Cursor::new(row,3,c);attributes.push(Puzzle5dAttribute{id:value.text()?,key:value.text()?,value:value.text()?,definition:value.optional_text()?});value.done()?;}
   for row in rows.take(9,row.rowid,c)?{let mut value=Cursor::new(row,3,c);authors.push(Puzzle5dAuthor{id:value.text()?,name:value.text()?,email:value.text()?,role:value.optional_text()?,rank:value.optional_i32()?});value.done()?;}
   parts.push(Puzzle5dCatalogPartKindExtra{id,name,label,description,icon,image,unit,is_abstract,base_kinds,representations,grips:templates,attributes,authors});
  }
  for row in rows.take(10,extra.rowid,c)?{let mut value=Cursor::new(row,3,c);let id=value.text()?;let code=value.optional_text()?;let label=value.optional_text()?;let order=value.optional_i32()?;let description=value.text()?;let icon=value.text()?;let color=value.text()?;let default_rope_kind=value.text()?;value.done()?;let compatible_with=rows.strings(11,row.rowid,c)?;grips.push(Puzzle5dCatalogGripKindExtra{id,code,label,order,description,icon,color,default_rope_kind,compatible_with});}
  for row in rows.take(12,extra.rowid,c)?{let mut value=Cursor::new(row,3,c);fasteners.push(Puzzle5dCatalogFastenerKindExtra{id:value.text()?,name:value.text()?,label:value.optional_text()?});value.done()?;}
  for row in rows.take(13,extra.rowid,c)?{let mut value=Cursor::new(row,3,c);ropes.push(Puzzle5dCatalogRopeKindExtra{id:value.text()?,name:value.text()?,label:value.text()?,default_fastener_kind:value.text()?});value.done()?;}
  Some(Puzzle5dKindCatalogsExtra{parts,grips,fasteners,ropes})
 }else{None};
 let(mut kind_compatibility,mut parts,mut fasteners,mut target_volumes)=(Vec::new(),Vec::new(),Vec::new(),Vec::new());
 for row in rows.take(14,doc.rowid,c)?{let mut value=Cursor::new(row,3,c);let source=value.text()?;let target=value.text()?;let bidirectional=value.boolean()?;let important=value.boolean()?;let specificity=match value.borrowed()?{"general"=>Puzzle5dCompatSpecificity::General,"part"=>Puzzle5dCompatSpecificity::Part,"fastener"=>Puzzle5dCompatSpecificity::Fastener,"grip"=>Puzzle5dCompatSpecificity::Grip,"rope"=>Puzzle5dCompatSpecificity::Rope,_=>return Err(invalid("Puzzle5d compatibility variant differs"))};value.done()?;kind_compatibility.push(Puzzle5dKindCompatibility{source,target,bidirectional,important,specificity});}
 for row in rows.take(15,doc.rowid,c)?{
  let mut value=Cursor::new(row,3,c);let id=value.text()?;let part_kind=value.optional_text()?;let anchor=match value.borrowed()?{"fixed"=>Puzzle5dPartAnchor::Fixed,"derived"=>Puzzle5dPartAnchor::Derived,_=>return Err(invalid("Puzzle5d anchor variant differs"))};value.done()?;
  let mut b=Cursor::new(rows.one(16,row.rowid,true)?.ok_or_else(||invalid("Puzzle5d board missing"))?,2,c);let part_2d=Puzzle5dPart2d{x:b.f64()?,y:b.f64()?,shape:b.optional_text()?,radius:b.optional_float()?,width:b.optional_float()?,height:b.optional_float()?,text:b.optional_text()?,icon_kind:b.optional_text()?,hidden:b.optional_bool()?,locked:b.optional_bool()?};b.done()?;
  let mut w=Cursor::new(rows.one(17,row.rowid,true)?.ok_or_else(||invalid("Puzzle5d world missing"))?,2,c);let origin=w.xyz()?;let mesh_url=w.optional_text()?;let orientation=w.orientation()?;let label=w.optional_text()?;w.done()?;let part_3d=Puzzle5dPart3d{origin,mesh_url,orientation,label,scale:rows.scale(18,row.rowid,c)?};let mut grips=Vec::new();
  for row in rows.take(19,row.rowid,c)?{let mut value=Cursor::new(row,3,c);let id=value.text()?;let grip_kind=value.optional_text()?;value.done()?;let mut b=Cursor::new(rows.one(20,row.rowid,true)?.ok_or_else(||invalid("Puzzle5d grip board missing"))?,2,c);let grip_2d=Puzzle5dGrip2d{angle:b.f64()?,grip_kind:b.optional_text()?,radius:b.optional_float()?};b.done()?;let mut w=Cursor::new(rows.one(21,row.rowid,true)?.ok_or_else(||invalid("Puzzle5d grip world missing"))?,2,c);let grip_3d=Puzzle5dGrip3d{position:w.xyz()?,direction:w.direction()?,radius:w.optional_float()?,label:w.optional_text()?};w.done()?;grips.push(Puzzle5dGrip{id,grip_kind,grip_2d,grip_3d});}
  parts.push(Puzzle5dPart{id,part_kind,anchor,part_2d,part_3d,grips});
 }
 for row in rows.take(22,doc.rowid,c)?{let mut value=Cursor::new(row,3,c);fasteners.push(Puzzle5dFastener{id:value.text()?,source:value.text()?,target:value.text()?,fastener_kind:value.optional_text()?,gap:value.f64()?,shift:value.f64()?,rise:value.f64()?,rotation:value.f64()?,turn:value.f64()?,tilt:value.f64()?,x:value.f64()?,y:value.f64()?});value.done()?;}
 for row in rows.take(23,doc.rowid,c)?{let mut value=Cursor::new(row,3,c);let id=value.text()?;let origin=value.xyz()?;let orientation=value.orientation()?;let hidden=value.boolean()?;let locked=value.boolean()?;value.done()?;let scale=rows.scale(24,row.rowid,c)?;target_volumes.push(Puzzle5dTargetVolume{id,origin,orientation,scale,hidden,locked});}
 rows.finish(c)?;Ok(Puzzle5dSnapshot{schema,domain,label,meta,kind_catalogs,kind_catalogs_extra,kind_compatibility,parts,fasteners,target_volumes})
}
impl ArtifactSqliteSnapshot for crate::Puzzle5dPlaySnapshot{
 const SQLITE_SCHEMA:&'static str=Puzzle5dSnapshot::SQLITE_SCHEMA;
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.typed().to_sqlite_database(c)}
 fn from_sqlite_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{Puzzle5dSnapshot::from_sqlite_database(d,c).map(Self::from_typed)}
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{Puzzle5dSnapshot::decode_sqlite_snapshot_native(payload,c).map(Self::from_typed)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,ValueError>{self.typed().encode_sqlite_snapshot_native(encoding,c)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{self.typed().validate_sqlite_snapshot_subset(dialect,d,c)}
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;

