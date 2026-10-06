//! 🔍️ Paid DAG identity, relationship and ownership frontiers borrow literal rows.
use super::reconstruction::{WIDTHS,SQL};
use store::sqlite_snapshot::{transfer::reserve,SqliteDatabase,SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind};
type Result<T>=std::result::Result<T,ValueError>;
fn invalid(message:&str)->ValueError{ValueError::new(ValueRefusalKind::InvalidValue,message)}
pub(super) fn tick(work:&mut usize,control:&mut SqliteSnapshotControl<'_>)->Result<()>{*work=work.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"DAG index work overflow"))?;if *work%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,*work,0)?;}Ok(())}
pub(super) fn sort<T>(values:&mut[T],work:&mut usize,control:&mut SqliteSnapshotControl<'_>,mut compare:impl FnMut(&T,&T)->Result<std::cmp::Ordering>)->Result<()>{
 fn sift<T>(values:&mut[T],mut root:usize,end:usize,work:&mut usize,control:&mut SqliteSnapshotControl<'_>,compare:&mut impl FnMut(&T,&T)->Result<std::cmp::Ordering>)->Result<()>{loop{let Some(left)=root.checked_mul(2).and_then(|n|n.checked_add(1)).filter(|n|*n<end)else{return Ok(())};let right=left+1;let child=if right<end&&compare(&values[left],&values[right])?.is_lt(){right}else{left};tick(work,control)?;if !compare(&values[root],&values[child])?.is_lt(){return Ok(())}values.swap(root,child);root=child;}}
 for root in(0..values.len()/2).rev(){sift(values,root,values.len(),work,control,&mut compare)?;}for end in(1..values.len()).rev(){values.swap(0,end);sift(values,0,end,work,control,&mut compare)?;}Ok(())
}
struct Group<'a>{table:&'static str,column:usize,rows:Vec<&'a SqliteRow>}
pub(super) struct Read<'a,'c,'p>{
 pub(super) database:&'a SqliteDatabase,
 pub(super) control:&'c mut SqliteSnapshotControl<'p>,
 keys:Vec<(&'static str,i64,&'a SqliteRow)>,
 used:Vec<u8>,
 groups:Vec<Group<'a>>,
 pub(super) work:usize,
}
impl<'a,'c,'p> Read<'a,'c,'p>{
 pub(super) fn new(database:&'a SqliteDatabase,control:&'c mut SqliteSnapshotControl<'p>)->Result<Self>{
  store::sqlite_snapshot::validate_sqlite_database_schema_controlled(database,SQL,SqliteSnapshotPhase::ReconstructSnapshot,control)?;
  control.check_database(database,SqliteSnapshotPhase::ReconstructSnapshot)?;
  let total=WIDTHS.iter().try_fold(0usize,|total,(name,_)|total.checked_add(database.table(name)?.rows.len()).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"DAG total rows overflow")))?;
  let mut keys=reserve(total,control)?;let mut work=0;
  for &(name,width)in WIDTHS{for row in &database.table(name)?.rows{if row.rowid<=0||row.values.len()!=width||row.integer(0)?!=row.rowid{return Err(invalid("DAG entity identity or width differs"))}keys.push((name,row.rowid,row));tick(&mut work,control)?;}}
  sort(&mut keys,&mut work,control,|a,b|Ok((a.0,a.1).cmp(&(b.0,b.1))))?;
  for pair in keys.windows(2){tick(&mut work,control)?;if (pair[0].0,pair[0].1)==(pair[1].0,pair[1].1){return Err(invalid("DAG duplicate entity identity"))}}
  let mut used=reserve(total,control)?;for _ in 0..total{used.push(0);tick(&mut work,control)?;}
  let groups=reserve(WIDTHS.len(),control)?;
  Ok(Self{database,control,keys,used,groups,work})
 }
 pub(super) fn step(&mut self)->Result<()>{tick(&mut self.work,self.control)}
 pub(super) fn take(&mut self,table:&'static str,id:i64)->Result<&'a SqliteRow>{
  let mut start=0;let mut end=self.keys.len();while start<end{self.step()?;let middle=start+(end-start)/2;let row=self.keys[middle];match(row.0,row.1).cmp(&(table,id)){std::cmp::Ordering::Less=>start=middle+1,std::cmp::Ordering::Greater=>end=middle,std::cmp::Ordering::Equal=>{if self.used[middle]!=0{return Err(invalid("DAG entity has repeated ownership or a cycle"))}self.used[middle]=1;return Ok(row.2)}}}Err(invalid("DAG entity missing"))
 }
 pub(super) fn relations(&mut self,table:&'static str,column:usize,owner:i64)->Result<Vec<&'a SqliteRow>>{
  let group=match self.groups.iter().position(|group|group.table==table&&group.column==column){Some(index)=>index,None=>{
   let source=&self.database.table(table)?.rows;let mut rows=reserve(source.len(),self.control)?;for row in source{if row.integer(column)?<=0{return Err(invalid("DAG relationship owner must be positive"))}rows.push(row);self.step()?;}
   sort(&mut rows,&mut self.work,self.control,|a,b|Ok(a.integer(column)?.cmp(&b.integer(column)?).then_with(||a.rowid.cmp(&b.rowid))))?;
   if self.groups.len()==self.groups.capacity(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"DAG relationship index exceeds authored table count"))}
   self.groups.push(Group{table,column,rows});self.groups.len()-1
  }};
  let mut start=0;let mut end=self.groups[group].rows.len();while start<end{self.step()?;let middle=start+(end-start)/2;if self.groups[group].rows[middle].integer(column)?<owner{start=middle+1}else{end=middle}}let first=start;end=self.groups[group].rows.len();while start<end{self.step()?;let middle=start+(end-start)/2;if self.groups[group].rows[middle].integer(column)?<=owner{start=middle+1}else{end=middle}}
  let mut rows=reserve(start-first,self.control)?;for index in first..start{let row=self.groups[group].rows[index];rows.push(self.take(table,row.rowid)?);}Ok(rows)
 }
 pub(super) fn child(&mut self,table:&'static str,owner:i64)->Result<&'a SqliteRow>{let rows=self.relations(table,1,owner)?;if rows.len()!=1{return Err(invalid("DAG required variant entity differs"))}Ok(rows[0])}
 pub(super) fn optional_child(&mut self,table:&'static str,owner:i64)->Result<Option<&'a SqliteRow>>{let rows=self.relations(table,1,owner)?;if rows.len()>1{return Err(invalid("DAG optional entity has repeated owners"))}Ok(rows.first().copied())}
 pub(super) fn ordered(&mut self,table:&'static str,column:usize,owner:i64,ordinal:usize)->Result<Vec<&'a SqliteRow>>{let mut rows=self.relations(table,column,owner)?;sort(&mut rows,&mut self.work,self.control,|a,b|Ok(a.integer(ordinal)?.cmp(&b.integer(ordinal)?)))?;for(index,row)in rows.iter().enumerate(){self.step()?;if row.integer(ordinal)?!=i64::try_from(index).map_err(|_|ValueError::new(ValueRefusalKind::WorkLimit,"DAG ordinal overflow"))?{return Err(invalid("DAG ordinals must be unique and contiguous"))}}Ok(rows)}
 pub(super) fn finish(&mut self)->Result<()>{for index in 0..self.used.len(){self.step()?;if self.used[index]==0{return Err(invalid("DAG unowned entity remains"))}}self.control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,self.work,self.work)}
}
