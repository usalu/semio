//! 🔍️ Run relationship indexes own only concretely admitted borrowed row references.
use semio_framework_value::{ValueError,ValueRefusalKind};
use store::sqlite_snapshot::{SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,transfer::{reserve,heap_sort}};
const PHASE:SqliteSnapshotPhase=SqliteSnapshotPhase::ReconstructSnapshot;
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn identity(row:&SqliteRow,columns:usize)->Result<(),ValueError>{if row.rowid<=0||row.values.len()!=columns||row.integer(0)?!=row.rowid{return Err(invalid("Run entities require complete columns and positive aliased identities"))}Ok(())}
fn refs<'a>(rows:&'a[SqliteRow],columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
 let mut result=reserve(rows.len(),control)?;
 for(index,row)in rows.iter().enumerate(){identity(row,columns)?;result.push(row);if(index+1)%256==0{control.checkpoint(PHASE,index+1,rows.len())?;}}
 heap_sort(&mut result,PHASE,control,|a,b,_|Ok(a.rowid.cmp(&b.rowid)))?;
 for(index,pair)in result.windows(2).enumerate(){if pair[0].rowid==pair[1].rowid{return Err(invalid("Run collection identities must be unique"))}if(index+1)%256==0{control.checkpoint(PHASE,index+1,result.len())?;}}
 Ok(result)
}
fn document<'a>(rows:&'a[SqliteRow],columns:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
 let mut rows=refs(rows,columns,control)?;
 heap_sort(&mut rows,PHASE,control,|a,b,_|Ok(a.integer(2)?.cmp(&b.integer(2)?)))?;
 for(index,row)in rows.iter().enumerate(){if row.integer(1)?!=1||row.integer(2)?!=i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"Run ordinal exceeds INTEGER width"))?{return Err(invalid("Run document ownership or dense ordinal differs"))}if(index+1)%256==0{control.checkpoint(PHASE,index+1,rows.len())?;}}
 Ok(rows)
}
fn direction(row:&SqliteRow)->Result<u8,ValueError>{match row.text(2)?{"input"=>Ok(0),"output"=>Ok(1),_=>Err(invalid("Run fingerprint direction is undeclared"))}}
fn partition(rows:&[&SqliteRow],node:i64,direction_id:Option<u8>)->Result<std::ops::Range<usize>,ValueError>{
 let key=|row:&SqliteRow|->Result<(i64,u8),ValueError>{Ok((row.integer(1)?,if direction_id.is_some(){direction(row)?}else{0}))};
 let target=(node,direction_id.unwrap_or(0));let mut start=0;let mut end=rows.len();
 while start<end{let middle=start+(end-start)/2;if key(rows[middle])?<target{start=middle+1}else{end=middle}}
 let begin=start;end=rows.len();while start<end{let middle=start+(end-start)/2;if key(rows[middle])?<=target{start=middle+1}else{end=middle}}
 Ok(begin..start)
}
pub(super) struct Rows<'a>{pub nodes:Vec<&'a SqliteRow>,pub parameters:Vec<&'a SqliteRow>,pub logs:Vec<&'a SqliteRow>,fingerprints:Vec<&'a SqliteRow>,outputs:Vec<&'a SqliteRow>}
impl<'a>Rows<'a>{
 pub(super) fn new(database:&'a SqliteDatabase,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{
  let nodes=document(&database.table("run_node")?.rows,10,control)?;
  let mut ids=reserve(nodes.len(),control)?;ids.extend_from_slice(&nodes);heap_sort(&mut ids,PHASE,control,|a,b,_|Ok(a.rowid.cmp(&b.rowid)))?;
  let parameters=document(&database.table("run_parameter_value")?.rows,5,control)?;let logs=document(&database.table("run_log")?.rows,7,control)?;
  let mut fingerprints=refs(&database.table("run_port_fingerprint")?.rows,6,control)?;let mut outputs=refs(&database.table("run_output_artifact")?.rows,6,control)?;
  heap_sort(&mut fingerprints,PHASE,control,|a,b,_|Ok((a.integer(1)?,direction(a)?,a.integer(3)?).cmp(&(b.integer(1)?,direction(b)?,b.integer(3)?))))?;
  heap_sort(&mut outputs,PHASE,control,|a,b,_|Ok((a.integer(1)?,a.integer(2)?).cmp(&(b.integer(1)?,b.integer(2)?))))?;
  for(rows,fingerprint)in[(&fingerprints,true),(&outputs,false)]{let mut previous=None;let mut ordinal=0i64;for(index,row)in rows.iter().enumerate(){let node=row.integer(1)?;if ids.binary_search_by_key(&node,|row|row.rowid).is_err(){return Err(invalid("Run relationship has an unknown node"))}let key=(node,if fingerprint{direction(row)?}else{0});if previous!=Some(key){ordinal=0;previous=Some(key);}if row.integer(if fingerprint{3}else{2})?!=ordinal{return Err(invalid("Run relationship ordinals must be dense and unique"))}ordinal=ordinal.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"Run relationship ordinal overflow"))?;if(index+1)%256==0{control.checkpoint(PHASE,index+1,rows.len())?;}}}
  Ok(Self{nodes,parameters,logs,fingerprints,outputs})
 }
 pub(super) fn fingerprints(&self,node:i64,input:bool)->Result<&[&'a SqliteRow],ValueError>{Ok(&self.fingerprints[partition(&self.fingerprints,node,Some(u8::from(!input)))?])}
 pub(super) fn outputs(&self,node:i64)->Result<&[&'a SqliteRow],ValueError>{Ok(&self.outputs[partition(&self.outputs,node,None)?])}
}
