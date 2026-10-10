//! 🪪️ Original source ordinals support bounded authored-name uniqueness and relational identity lookup.
use store::sqlite_snapshot::{SqliteSnapshotControl,SqliteSnapshotPhase,artifact::RowWriter,transfer::{reserve,heap_sort,compare_text}};
use semio_framework_value::{ValueError,ValueRefusalKind};
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}

/// 🗂️ Owns scalar addresses while every comparison borrows bytes from the original source.
#[derive(semio_framework_value::RetireOwned)]
pub(crate)struct NameIndex{indices:Vec<usize>,ready:bool}
impl NameIndex{
 pub(crate)fn empty()->Self{Self{indices:Vec::new(),ready:false}}
 pub(crate)fn initialize<'a>(&mut self,count:usize,name:impl Fn(usize)->&'a str,phase:SqliteSnapshotPhase,control:&mut SqliteSnapshotControl<'_>,duplicate:&'static str)->Result<(),ValueError>{
  if self.ready||self.indices.capacity()!=0{return Err(invalid("original name index is already live"))}
  control.check_rows(count)?;self.indices=reserve(count,control)?;for index in 0..count{self.indices.push(index);control.checkpoint(phase,index+1,count)?;}
  heap_sort(&mut self.indices,phase,control,|a,b,c|compare_text(name(*a),name(*b),phase,c))?;
  for pair in self.indices.windows(2){if compare_text(name(pair[0]),name(pair[1]),phase,control)?.is_eq(){return Err(invalid(duplicate))}}
  self.ready=true;Ok(())
 }
 pub(crate)fn ready(&self)->bool{self.ready}
 pub(crate)fn lookup<'a>(&self,id:&str,name:impl Fn(usize)->&'a str,out:&mut RowWriter<'_,'_>,missing:&'static str)->Result<i64,ValueError>{
  if !self.ready{return Err(invalid("original name index is not ready"))}
  let(mut low,mut high)=(0,self.indices.len());while low<high{let middle=low+(high-low)/2;let index=self.indices[middle];match out.compare_text(name(index),id)?{std::cmp::Ordering::Less=>low=middle+1,std::cmp::Ordering::Greater=>high=middle,std::cmp::Ordering::Equal=>return index.checked_add(1).and_then(|value|i64::try_from(value).ok()).ok_or_else(||invalid("original name identity overflow"))}}Err(invalid(missing))
 }
}
