//! 🔍️ Paid record identity and field relationship indexes over borrowed authored cells.
use semio_framework_os_kernel::sqlite_snapshot::{SqliteRow,SqliteTable,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind,artifact::ordered_row_refs,transfer::reserve};
use std::cmp::Ordering;
type Result<T>=std::result::Result<T,ValueError>;
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
fn tick(work:&mut usize,control:&mut SqliteSnapshotControl<'_>)->Result<()>{*work=work.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"record index work overflow"))?;if *work%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,*work,0)?;}Ok(())}
fn sort<T>(values:&mut[T],control:&mut SqliteSnapshotControl<'_>,mut compare:impl FnMut(&T,&T)->Result<Ordering>)->Result<()>{
 fn sift<T>(values:&mut[T],mut root:usize,end:usize,work:&mut usize,control:&mut SqliteSnapshotControl<'_>,compare:&mut impl FnMut(&T,&T)->Result<Ordering>)->Result<()>{loop{let Some(left)=root.checked_mul(2).and_then(|n|n.checked_add(1)).filter(|n|*n<end)else{return Ok(())};let right=left+1;let next=if right<end&&compare(&values[left],&values[right])?.is_lt(){right}else{left};tick(work,control)?;if !compare(&values[root],&values[next])?.is_lt(){return Ok(())}values.swap(root,next);root=next;}}
 let mut work=0;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,values.len())?;for root in(0..values.len()/2).rev(){sift(values,root,values.len(),&mut work,control,&mut compare)?;}for end in(1..values.len()).rev(){values.swap(0,end);sift(values,0,end,&mut work,control,&mut compare)?;}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,values.len(),values.len())
}
/// 🗂️ Holds only paid borrowed row references and concrete identity indexes.
pub(super) struct Rows<'a>{pub(super) records:Vec<&'a SqliteRow>,fields:Vec<&'a SqliteRow>}
impl<'a> Rows<'a>{
 pub(super) fn new(records:&'a SqliteTable,fields:&'a SqliteTable,width:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Self>{
  let records=ordered_row_refs(records,2,control)?;let mut identities=reserve(records.len(),control)?;let mut work=0;
  for row in &records{tick(&mut work,control)?;if row.rowid<=0||row.values.len()!=3||row.integer(0)?!=row.rowid||row.integer(1)?!=1{return Err(invalid("record identity or document ownership"))}identities.push(row.rowid);}
  sort(&mut identities,control,|a,b|Ok(a.cmp(b)))?;for pair in identities.windows(2){tick(&mut work,control)?;if pair[0]==pair[1]{return Err(invalid("duplicate record identity"))}}
  let mut field_ids=reserve(fields.rows.len(),control)?;let mut ordered=reserve(fields.rows.len(),control)?;
  for row in &fields.rows{tick(&mut work,control)?;if row.rowid<=0||row.values.len()!=width||row.integer(0)?!=row.rowid{return Err(invalid("field identity or width"))}if identities.binary_search(&row.integer(1)?).is_err(){return Err(invalid("field has an unknown record"))}if row.integer(2)?<0{return Err(invalid("field ordinals must be contiguous"))}field_ids.push(row.rowid);ordered.push(row);}
  sort(&mut field_ids,control,|a,b|Ok(a.cmp(b)))?;for pair in field_ids.windows(2){tick(&mut work,control)?;if pair[0]==pair[1]{return Err(invalid("duplicate field identity"))}}
  sort(&mut ordered,control,|a,b|Ok(a.integer(1)?.cmp(&b.integer(1)?).then(a.integer(2)?.cmp(&b.integer(2)?))))?;Ok(Self{records,fields:ordered})
 }
 pub(super) fn fields(&self,parent:i64,control:&mut SqliteSnapshotControl<'_>)->Result<&[&'a SqliteRow]>{
  let mut work=0;let mut first=0;let mut end=self.fields.len();while first<end{tick(&mut work,control)?;let middle=first+(end-first)/2;if self.fields[middle].integer(1)?<parent{first=middle+1}else{end=middle}}let start=first;end=self.fields.len();while first<end{tick(&mut work,control)?;let middle=first+(end-first)/2;if self.fields[middle].integer(1)?<=parent{first=middle+1}else{end=middle}}let fields=&self.fields[start..first];for(index,row)in fields.iter().enumerate(){tick(&mut work,control)?;if row.integer(2)?!=i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"field ordinal overflow"))?{return Err(invalid("field ordinals must be contiguous"))}}Ok(fields)
 }
}
