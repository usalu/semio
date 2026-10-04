//! 🧮️ Explicit TIFF row indexes and typed collection backing share the transfer ledger.
use semio_framework_os_kernel::sqlite_snapshot::{SqliteRow,SqliteSnapshotControl,SqliteSnapshotPhase,ValueError,ValueRefusalKind};

pub(super) fn owned_vec<T>(count:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Vec<T>,ValueError>{
    control.admit_allocation_bytes(count.checked_mul(std::mem::size_of::<T>()).ok_or_else(||ValueError::new(ValueRefusalKind::OwnershipLimit,"TIFF construction backing overflow"))?)?;
    let mut output=Vec::new();output.try_reserve_exact(count).map_err(|_|ValueError::new(ValueRefusalKind::AllocationFailed,"TIFF construction allocation failed"))?;Ok(output)
}
pub(super) fn copy_blob(control:&mut SqliteSnapshotControl<'_>,value:&[u8])->Result<Vec<u8>,ValueError>{control.admit_allocation_bytes(value.len())?;super::reconstruct_blob(control,value)}
pub(super) fn copy_text(control:&mut SqliteSnapshotControl<'_>,value:&str)->Result<String,ValueError>{control.admit_allocation_bytes(value.len())?;super::reconstruct_text(control,value)}
pub(super) fn collect<'a,T>(rows:impl ExactSizeIterator<Item=&'a SqliteRow>,control:&mut SqliteSnapshotControl<'_>,read:impl Fn(&SqliteRow)->Result<T,ValueError>)->Result<Vec<T>,ValueError>{
    let count=rows.len();let mut output=owned_vec(count,control)?;control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,0,count)?;
    for(index,row)in rows.enumerate(){output.push(read(row)?);if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index+1,count)?;}}
    control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,count,count)?;Ok(output)
}
pub(super) struct Index<'a,K>{rows:Vec<(K,&'a SqliteRow,bool)>}
impl<'a,K:Ord+Copy> Index<'a,K>{
    pub(super) fn new(count:usize,control:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{Ok(Self{rows:owned_vec(count,control)?})}
    pub(super) fn push(&mut self,key:K,row:&'a SqliteRow)->Result<(),ValueError>{if self.rows.len()==self.rows.capacity(){return Err(ValueError::new(ValueRefusalKind::InvariantViolated,"TIFF row index exceeds its admitted frontier"))}self.rows.push((key,row,false));Ok(())}
    pub(super) fn finish(&mut self,control:&mut SqliteSnapshotControl<'_>)->Result<(),ValueError>{
        let mut work=0usize;let count=self.rows.len();for root in(0..count/2).rev(){sift(&mut self.rows,root,count,control,&mut work)?;}for end in(1..count).rev(){step(control,&mut work)?;self.rows.swap(0,end);sift(&mut self.rows,0,end,control,&mut work)?;}for index in 1..count{step(control,&mut work)?;if self.rows[index-1].0==self.rows[index].0{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"duplicate TIFF identity or relationship ordinal"))}}control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,work,work)
    }
    pub(super) fn get(&self,key:K)->Option<&'a SqliteRow>{self.rows.binary_search_by_key(&key,|r|r.0).ok().map(|i|self.rows[i].1)}
    pub(super) fn take(&mut self,key:K)->Option<&'a SqliteRow>{let i=self.rows.binary_search_by_key(&key,|r|r.0).ok()?;if self.rows[i].2{return None}self.rows[i].2=true;Some(self.rows[i].1)}
    pub(super) fn iter(&self)->impl ExactSizeIterator<Item=&'a SqliteRow>+'_ {self.rows.iter().map(|r|r.1)}
    pub(super) fn exhausted(&self,control:&mut SqliteSnapshotControl<'_>)->Result<bool,ValueError>{for(index,row)in self.rows.iter().enumerate(){if !row.2{return Ok(false)}if(index+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,index+1,self.rows.len())?}}Ok(true)}
}
impl<'a> Index<'a,(i64,usize)>{
    pub(super) fn group(&mut self,owner:i64,control:&mut SqliteSnapshotControl<'_>)->Result<impl ExactSizeIterator<Item=&'a SqliteRow>+'_,ValueError>{let start=self.rows.partition_point(|r|r.0.0<owner);let end=self.rows.partition_point(|r|r.0.0<=owner);for(expected,row)in self.rows[start..end].iter_mut().enumerate(){if row.0.1!=expected||row.2{return Err(ValueError::new(ValueRefusalKind::InvalidValue,"TIFF relationship ordinals must be contiguous and uniquely owned"))}row.2=true;if(expected+1)%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,expected+1,end-start)?;}}Ok(self.rows[start..end].iter().map(|r|r.1))}
}
fn step(control:&mut SqliteSnapshotControl<'_>,work:&mut usize)->Result<(),ValueError>{*work=work.checked_add(1).ok_or_else(||ValueError::new(ValueRefusalKind::WorkLimit,"TIFF ordering work overflow"))?;if *work%256==0{control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot,*work,0)?;}Ok(())}
fn sift<K:Ord>(rows:&mut[(K,&SqliteRow,bool)],mut root:usize,end:usize,control:&mut SqliteSnapshotControl<'_>,work:&mut usize)->Result<(),ValueError>{while root<end/2{step(control,work)?;let mut child=root*2+1;if child+1<end&&rows[child].0<rows[child+1].0{child+=1;}if rows[root].0>=rows[child].0{break}rows.swap(root,child);root=child;}Ok(())}
