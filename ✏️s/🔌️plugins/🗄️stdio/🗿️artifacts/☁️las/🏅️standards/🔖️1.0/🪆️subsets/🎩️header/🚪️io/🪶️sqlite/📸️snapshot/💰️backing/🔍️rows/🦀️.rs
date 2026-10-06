//! 🔍️ LAS admitted ordinal, identity, VLR byte and optional component indexes.
use semio_framework_os_kernel::sqlite_snapshot::{SqliteRow,SqliteTable,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind,artifact::ordered_row_refs,transfer::reserve};
type Result<T>=std::result::Result<T,ValueError>;
const PHASE:SqliteSnapshotPhase=SqliteSnapshotPhase::ReconstructSnapshot;
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn tick(work:&mut usize,control:&mut SqliteSnapshotControl<'_>)->Result<()>{*work=work.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"LAS index work overflow"))?;if *work%256==0{control.checkpoint(PHASE,*work,0)?;}Ok(())}
fn heap_sort<T>(values:&mut[T],phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>,mut compare:impl FnMut(&T,&T,&mut SqliteSnapshotControl<'_>)->Result<std::cmp::Ordering>)->Result<()>{
 fn sift<T>(values:&mut[T],mut root:usize,end:usize,work:&mut usize,control:&mut SqliteSnapshotControl<'_>,compare:&mut impl FnMut(&T,&T,&mut SqliteSnapshotControl<'_>)->Result<std::cmp::Ordering>)->Result<()>{loop{let Some(left)=root.checked_mul(2).and_then(|value|value.checked_add(1)).filter(|value|*value<end)else{return Ok(())};let right=left+1;let child=if right<end&&compare(&values[left],&values[right],control)?.is_lt(){right}else{left};tick(work,control)?;if !compare(&values[root],&values[child],control)?.is_lt(){return Ok(())}values.swap(root,child);root=child;}}
 control.checkpoint(phase,0,values.len())?;let mut work=0;for root in(0..values.len()/2).rev(){sift(values,root,values.len(),&mut work,control,&mut compare)?;}for end in(1..values.len()).rev(){values.swap(0,end);sift(values,0,end,&mut work,control,&mut compare)?;}control.checkpoint(phase,values.len(),values.len())
}
fn identity(row:&SqliteRow,width:usize)->Result<()>{if row.rowid<=0||row.values.len()!=width||row.integer(0)?!=row.rowid{return Err(invalid("LAS row identity or physical width differs"))}Ok(())}
fn unique(ids:&mut[i64],control:&mut SqliteSnapshotControl<'_>)->Result<()>{heap_sort(ids,PHASE,control,|a,b,_|Ok(a.cmp(b)))?;for(index,pair)in ids.windows(2).enumerate(){if pair[0]==pair[1]{return Err(invalid("LAS duplicate row identity"))}if index%256==0{control.checkpoint(PHASE,index,ids.len())?;}}Ok(())}
/// 🗂️ Admitted semantic sequence and separate concrete parent identity lookup.
pub(super) struct Entities<'a>{pub(super) rows:Vec<&'a SqliteRow>,ids:Vec<i64>}
impl<'a> Entities<'a>{
 pub(super) fn new(table:&'a SqliteTable,width:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Self>{
  let rows=ordered_row_refs(table,2,control)?;let mut ids=reserve(rows.len(),control)?;let mut work=0;
  for row in &rows{tick(&mut work,control)?;identity(row,width)?;if row.integer(1)?!=1{return Err(invalid("LAS entity has an unknown document"))}ids.push(row.rowid);}
  unique(&mut ids,control)?;Ok(Self{rows,ids})
 }
 fn owner(&self,id:i64)->Result<()>{self.ids.binary_search(&id).map(|_|()).map_err(|_|invalid("LAS component has an unknown owner"))}
}
/// 🧩️ Paid identity-sorted optional records preserve absence independently of numeric NULL.
pub(super) struct Components<'a>{rows:Vec<&'a SqliteRow>}
impl<'a> Components<'a>{
 pub(super) fn new(table:&'a SqliteTable,width:usize,parents:&Entities<'a>,control:&mut SqliteSnapshotControl<'_>)->Result<Self>{
  let mut rows=reserve(table.rows.len(),control)?;let mut work=0;
  for row in &table.rows{tick(&mut work,control)?;identity(row,width)?;parents.owner(row.rowid)?;rows.push(row);}
  heap_sort(&mut rows,PHASE,control,|a,b,_|Ok(a.rowid.cmp(&b.rowid)))?;
  for pair in rows.windows(2){tick(&mut work,control)?;if pair[0].rowid==pair[1].rowid{return Err(invalid("LAS optional component is duplicated"))}}
  Ok(Self{rows})
 }
 pub(super) fn get(&self,id:i64)->Option<&'a SqliteRow>{self.rows.binary_search_by_key(&id,|row|row.rowid).ok().map(|index|self.rows[index])}
}
/// 📦️ VLR byte relationships use admitted borrowed rows and contiguous child ordinals.
pub(super) struct Octets<'a>{rows:Vec<&'a SqliteRow>}
impl<'a> Octets<'a>{
 pub(super) fn new(table:&'a SqliteTable,parents:&Entities<'a>,control:&mut SqliteSnapshotControl<'_>)->Result<Self>{
  let mut rows=reserve(table.rows.len(),control)?;let mut ids=reserve(table.rows.len(),control)?;let mut work=0;
  for row in &table.rows{tick(&mut work,control)?;identity(row,4)?;parents.owner(row.integer(1)?)?;if row.integer(2)?<0{return Err(invalid("LAS VLR byte ordinal must be nonnegative"))}ids.push(row.rowid);rows.push(row);}
  unique(&mut ids,control)?;heap_sort(&mut rows,PHASE,control,|a,b,_|Ok(a.integer(1)?.cmp(&b.integer(1)?).then(a.integer(2)?.cmp(&b.integer(2)?))))?;
  Ok(Self{rows})
 }
 pub(super) fn children(&self,id:i64,control:&mut SqliteSnapshotControl<'_>)->Result<&[&'a SqliteRow]>{
  let mut work=0;let mut first=0;let mut end=self.rows.len();while first<end{tick(&mut work,control)?;let middle=first+(end-first)/2;if self.rows[middle].integer(1)?<id{first=middle+1}else{end=middle}}
  let start=first;end=self.rows.len();while first<end{tick(&mut work,control)?;let middle=first+(end-first)/2;if self.rows[middle].integer(1)?<=id{first=middle+1}else{end=middle}}
  let rows=&self.rows[start..first];for(index,row)in rows.iter().enumerate(){tick(&mut work,control)?;if row.integer(2)?!=i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"LAS VLR ordinal overflow"))?{return Err(invalid("LAS VLR byte ordinals must be contiguous and unique"))}}Ok(rows)
 }
}
