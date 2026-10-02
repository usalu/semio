//! 🔋️ Handcrafted Energy model entities and ordered structural ownership.
use super::EnergyModelSnapshot;
use crate::model::*;
use store::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,validate_sqlite_database_schema,artifact::{Cell,FloatColumn,FloatRow,Projection,insert_ieee754,insert_key_ieee754,reconstruct_text}};
use std::collections::BTreeMap;
#[path="🏘️envelope/🦀️.rs"]mod envelope;
#[path="⚙️systems/🦀️.rs"]mod systems;
#[path="🗓️schedules/🦀️.rs"]mod schedules;
pub const SCHEMA:&str=include_str!("🗄️.sql");
impl store::ArtifactSqliteSnapshot for EnergyModelSnapshot{
 const SQLITE_SCHEMA:&'static str=SCHEMA;
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{super::native::decode(payload,c)}
 fn encode_sqlite_snapshot_native(&self,encoding:store::sqlite_snapshot::SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>)->Result<store::io_schema::IoPayload,String>{super::native::encode(self,encoding,c)}
 fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{project(self,c)}
 fn from_sqlite_database(database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,String>{reconstruct(database,c)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&store::io_schema::ArtifactDialect,_database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,0,1)?;if dialect.artifact_kind!="s.energy.model"||dialect.standard!="1"||dialect.subset!="*"{return Err(String::from("Energy snapshot dialect differs").into())}c.checkpoint(SqliteSnapshotPhase::ProjectSnapshot,1,1)?;Ok(store::io_schema::IoOutcome::clean(()))}
}
const DOCUMENT:&[FloatColumn]=&[FloatColumn::Binary64(4),FloatColumn::Binary64(5),FloatColumn::Binary64(6),FloatColumn::Binary64(7),FloatColumn::Binary64(8),FloatColumn::Binary64(9)];
const MONTH:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4)];
const VERTEX:&[FloatColumn]=&[FloatColumn::Binary64(3),FloatColumn::Binary64(4),FloatColumn::Binary64(5)];
fn i(v:u32)->Cell<'static>{Cell::Integer(i64::from(v))}
fn b(v:bool)->Cell<'static>{Cell::Integer(i64::from(v))}
fn optional_id(v:Option<EntityId>)->Cell<'static>{v.map_or(Cell::Null,|v|i(v.0))}
fn optional_schedule(v:Option<ScheduleId>)->Cell<'static>{v.map_or(Cell::Null,|v|i(v.0))}
fn ordinal(n:usize)->Result<Cell<'static>,String>{Ok(Cell::Integer(i64::try_from(n).map_err(|_|"Energy ordinal exceeds signed64")?))}
fn emit(p:&mut Projection<'_,'_>,table:&str,n:usize,fields:&[Cell<'_>],floats:&[FloatColumn])->Result<i64,String>{let mut cells=Vec::with_capacity(fields.len()+2);cells.push(Cell::Integer(1));cells.push(ordinal(n)?);cells.extend_from_slice(fields);insert_ieee754(p,table,&cells,floats)}
fn edge(p:&mut Projection<'_,'_>,table:&str,parent:i64,n:usize,fields:&[Cell<'_>],floats:&[FloatColumn])->Result<i64,String>{let mut cells=Vec::with_capacity(fields.len()+2);cells.push(Cell::Integer(parent));cells.push(ordinal(n)?);cells.extend_from_slice(fields);insert_ieee754(p,table,&cells,floats)}
fn vertices(p:&mut Projection<'_,'_>,table:&str,parent:i64,vertices:&[[f64;3]])->Result<(),String>{for(n,v)in vertices.iter().enumerate(){edge(p,table,parent,n,&[Cell::Real(v[0]),Cell::Real(v[1]),Cell::Real(v[2])],VERTEX)?;}Ok(())}
fn ids(p:&mut Projection<'_,'_>,table:&str,parent:i64,ids:&[EntityId])->Result<(),String>{for(n,id)in ids.iter().enumerate(){edge(p,table,parent,n,&[i(id.0)],&[])?;}Ok(())}
fn literal_child<S>(p:&mut Projection<'_,'_>,table:&str,child:&store::ArtifactChild<S>)->Result<(),String>{let target=&child.target;p.insert_key(table,1,&[Cell::Text(&child.child_id),Cell::Text(&target.artifact_id),Cell::Text(&target.dialect.artifact_kind),Cell::Text(&target.dialect.standard),Cell::Text(&target.dialect.subset)])}
fn literal_link(p:&mut Projection<'_,'_>,table:&str,link:&store::ArtifactLink)->Result<(),String>{let target=&link.target;let(pin,id,hash,high,low,media)=match &link.pin{store::LinkPin::Head=>("head",Cell::Null,Cell::Null,Cell::Null,Cell::Null,Cell::Null),store::LinkPin::Checkpoint{id}=>("checkpoint",Cell::Text(id),Cell::Null,Cell::Null,Cell::Null,Cell::Null),store::LinkPin::Snapshot{blob}=>("snapshot",Cell::Null,Cell::Text(&blob.hash),i((blob.size>>32)as u32),i(blob.size as u32),Cell::Text(&blob.media_type))};p.insert_key(table,1,&[Cell::Text(&target.artifact_id),Cell::Text(&target.dialect.artifact_kind),Cell::Text(&target.dialect.standard),Cell::Text(&target.dialect.subset),Cell::Text(&link.role),Cell::Text(pin),id,hash,high,low,media])}
pub fn project(s:&EnergyModelSnapshot,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,String>{
 super::native::rows(s,c,SqliteSnapshotPhase::ProjectSnapshot)?;let m=&s.model;let mut p=Projection::new(SCHEMA,c)?;
 insert_key_ieee754(&mut p,"energy_document",1,&[Cell::Text(&s.schema),Cell::Text(&m.name),Cell::Text(&m.version),Cell::Real(m.site.latitude_deg),Cell::Real(m.site.longitude_deg),Cell::Real(m.site.elevation_m),Cell::Real(m.site.time_zone_hours),Cell::Real(m.site.north_axis_deg),Cell::Real(m.ground_temperature.deep_c),i(u32::from(m.run_period.start_month)),i(u32::from(m.run_period.start_day)),i(u32::from(m.run_period.end_month)),i(u32::from(m.run_period.end_day)),i(u32::from(m.run_period.year))],DOCUMENT)?;
 literal_child(&mut p,"energy_structure_child",&s.structure)?;literal_child(&mut p,"energy_zones_child",&s.zones)?;
 if let Some(link)=&s.referenced_model{literal_link(&mut p,"energy_referenced_model_link",link)?;}if let Some(link)=&s.weather_link{literal_link(&mut p,"energy_weather_link",link)?;}
 for n in 0..12{emit(&mut p,"energy_ground_month",n,&[Cell::Real(m.ground_temperature.building_surface_c[n]),Cell::Real(m.ground_temperature.shallow_c[n])],MONTH)?;}
 envelope::project(m,&mut p)?;systems::project(m,&mut p)?;schedules::project(&m.schedules,&mut p)?;p.finish()
}
struct Reader<'d,'c,'p>{database:&'d SqliteDatabase,control:&'c mut SqliteSnapshotControl<'p>,groups:BTreeMap<&'static str,BTreeMap<i64,Vec<FloatRow<'d>>>>,used:usize,visits:usize}
impl<'d,'c,'p> Reader<'d,'c,'p>{
 fn tick(&mut self)->Result<(),String>{self.visits=self.visits.checked_add(1).ok_or("Energy traversal overflow")?;if self.visits%256==0{self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.visits,0)?;}Ok(())}
 fn rows(&mut self,table:&'static str,parent:i64,columns:usize,floats:&'static[FloatColumn])->Result<Vec<FloatRow<'d>>,String>{
  if !self.groups.contains_key(table){let mut grouped=BTreeMap::<i64,BTreeMap<i64,FloatRow<'d>>>::new();let mut identities=std::collections::BTreeSet::new();for row in &self.database.table(table)?.rows{self.tick()?;let r=FloatRow::new(row,floats)?;if r.values.len()!=columns||r.rowid<=0||r.integer(0)?!=r.rowid||!identities.insert(r.rowid){return Err("Energy entity shape or identity differs".into())}let ordinal=r.integer(2)?;if ordinal<0||grouped.entry(r.integer(1)?).or_default().insert(ordinal,r).is_some(){return Err("Energy relationship ordinal differs".into())}}let mut groups=BTreeMap::new();for(parent,rows)in grouped{for(n,index)in rows.keys().enumerate(){self.tick()?;if i64::try_from(n).ok()!=Some(*index){return Err("Energy relationship order is not dense".into())}}groups.insert(parent,rows.into_values().collect());}self.groups.insert(table,groups);}
  let rows=self.groups.get_mut(table).and_then(|groups|groups.remove(&parent)).unwrap_or_default();self.used=self.used.checked_add(rows.len()).ok_or("Energy entity count overflow")?;Ok(rows)
 }
 fn one(&mut self,table:&str,columns:usize,floats:&'static[FloatColumn],required:bool)->Result<Option<FloatRow<'d>>,String>{let rows=&self.database.table(table)?.rows;if rows.is_empty()&&!required{return Ok(None)}if rows.len()!=1{return Err("Energy singleton presence differs".into())}self.tick()?;let r=FloatRow::new(&rows[0],floats)?;if r.rowid!=1||r.integer(0)?!=1||r.values.len()!=columns{return Err("Energy singleton shape differs".into())}self.used+=1;Ok(Some(r))}
 fn optional(&mut self,table:&'static str,parent:i64,columns:usize,floats:&'static[FloatColumn])->Result<Option<FloatRow<'d>>,String>{let row=self.database.table(table)?.rows.iter().find(|row|row.rowid==parent);if let Some(row)=row{self.tick()?;let r=FloatRow::new(row,floats)?;if r.values.len()!=columns||r.integer(0)?!=parent{return Err("Energy optional relationship shape differs".into())}self.used+=1;Ok(Some(r))}else{Ok(None)}}
 fn text(&mut self,r:FloatRow<'_>,index:usize)->Result<String,String>{reconstruct_text(self.control,r.text(index)?)}
 fn finish(self)->Result<(),String>{if self.groups.values().any(|groups|!groups.is_empty())||self.database.tables.iter().map(|table|table.rows.len()).sum::<usize>()!=self.used{return Err("Energy has unowned or multiply owned entities".into())}Ok(())}
}
fn u32_value(r:FloatRow<'_>,n:usize)->Result<u32,String>{u32::try_from(r.integer(n)?).map_err(|_|"Energy unsigned32 value differs".into())}
fn entity(r:FloatRow<'_>,n:usize)->Result<EntityId,String>{u32_value(r,n).map(EntityId)}
fn schedule(r:FloatRow<'_>,n:usize)->Result<ScheduleId,String>{u32_value(r,n).map(ScheduleId)}
fn boolean(r:FloatRow<'_>,n:usize)->Result<bool,String>{match r.integer(n)?{0=>Ok(false),1=>Ok(true),_=>Err("Energy boolean differs".into())}}
fn optional_entity(r:FloatRow<'_>,n:usize)->Result<Option<EntityId>,String>{if matches!(r.values.get(n),Some(store::sqlite_snapshot::SqliteValue::Null)){Ok(None)}else{entity(r,n).map(Some)}}
fn optional_schedule_value(r:FloatRow<'_>,n:usize)->Result<Option<ScheduleId>,String>{optional_entity(r,n).map(|id|id.map(|id|ScheduleId(id.0)))}
fn read_vertices(r:&mut Reader<'_,'_,'_>,table:&'static str,parent:i64)->Result<Vec<[f64;3]>,String>{r.rows(table,parent,6,VERTEX)?.into_iter().map(|v|Ok([v.real(3)?,v.real(4)?,v.real(5)?])).collect()}
fn read_ids(r:&mut Reader<'_,'_,'_>,table:&'static str,parent:i64)->Result<Vec<EntityId>,String>{r.rows(table,parent,4,&[])?.into_iter().map(|v|entity(v,3)).collect()}
fn read_reference(r:&mut Reader<'_,'_,'_>,row:FloatRow<'_>,start:usize)->Result<store::io_schema::ArtifactRef,String>{Ok(store::io_schema::ArtifactRef{artifact_id:r.text(row,start)?,dialect:store::io_schema::ArtifactDialect{artifact_kind:r.text(row,start+1)?,standard:r.text(row,start+2)?,subset:r.text(row,start+3)?}})}
fn read_child<S>(r:&mut Reader<'_,'_,'_>,table:&str)->Result<store::ArtifactChild<S>,String>{let row=r.one(table,6,&[],true)?.ok_or("missing Energy child")?;Ok(store::ArtifactChild::new(r.text(row,1)?,read_reference(r,row,2)?))}
fn read_link(r:&mut Reader<'_,'_,'_>,table:&str)->Result<Option<store::ArtifactLink>,String>{let Some(row)=r.one(table,12,&[],false)?else{return Ok(None)};let target=read_reference(r,row,1)?;let role=r.text(row,5)?;let pin=match row.text(6)?{"head"=>{if row.values[7..12].iter().any(|v|!matches!(v,store::sqlite_snapshot::SqliteValue::Null)){return Err("Energy head pin has extra payload".into())}store::LinkPin::Head},"checkpoint"=>{if row.values[8..12].iter().any(|v|!matches!(v,store::sqlite_snapshot::SqliteValue::Null)){return Err("Energy checkpoint pin has extra payload".into())}store::LinkPin::Checkpoint{id:r.text(row,7)?}},"snapshot"=>{if !matches!(row.values[7],store::sqlite_snapshot::SqliteValue::Null){return Err("Energy snapshot pin has checkpoint payload".into())}store::LinkPin::Snapshot{blob:store::BlobRef{hash:r.text(row,8)?,size:(u64::from(u32_value(row,9)?)<<32)|u64::from(u32_value(row,10)?),media_type:r.text(row,11)?}}},_=>return Err("Energy pin tag differs".into())};Ok(Some(store::ArtifactLink{target,pin,role}))}
pub fn reconstruct(database:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<EnergyModelSnapshot,String>{
 c.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;validate_sqlite_database_schema(database,SCHEMA,c.limits()).map_err(|e|e.to_string())?;
 let mut r=Reader{database,control:c,groups:BTreeMap::new(),used:0,visits:0};let root=r.one("energy_document",15,DOCUMENT,true)?.ok_or("missing Energy document")?;
 let mut m=Model{name:r.text(root,2)?,version:r.text(root,3)?,site:Site{latitude_deg:root.real(4)?,longitude_deg:root.real(5)?,elevation_m:root.real(6)?,time_zone_hours:root.real(7)?,north_axis_deg:root.real(8)?},ground_temperature:GroundTemperatureConfig{building_surface_c:[0.0;12],shallow_c:[0.0;12],deep_c:root.real(9)?},run_period:crate::calendar::RunPeriod{start_month:u8::try_from(u32_value(root,10)?).map_err(|_|"Energy month exceeds unsigned8")?,start_day:u8::try_from(u32_value(root,11)?).map_err(|_|"Energy day exceeds unsigned8")?,end_month:u8::try_from(u32_value(root,12)?).map_err(|_|"Energy month exceeds unsigned8")?,end_day:u8::try_from(u32_value(root,13)?).map_err(|_|"Energy day exceeds unsigned8")?,year:u16::try_from(u32_value(root,14)?).map_err(|_|"Energy year exceeds unsigned16")?},..Model::default()};
 let months=r.rows("energy_ground_month",1,5,MONTH)?;if months.len()!=12{return Err("Energy ground temperature has exactly12months".into())}for(n,row)in months.into_iter().enumerate(){m.ground_temperature.building_surface_c[n]=row.real(3)?;m.ground_temperature.shallow_c[n]=row.real(4)?;}
 envelope::reconstruct(&mut m,&mut r)?;systems::reconstruct(&mut m,&mut r)?;m.schedules=schedules::reconstruct(&mut r)?;
 let snapshot=EnergyModelSnapshot{schema:r.text(root,1)?,model:m,structure:read_child(&mut r,"energy_structure_child")?,zones:read_child(&mut r,"energy_zones_child")?,referenced_model:read_link(&mut r,"energy_referenced_model_link")?,weather_link:read_link(&mut r,"energy_weather_link")?};r.finish()?;Ok(snapshot)
}
