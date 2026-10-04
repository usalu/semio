//! 👁️ Declared child admission resolves exact types without mutating a captured parent.
use super::ChildRefFields;
pub trait ChildReadSource{
 type Error;
 fn step(&mut self)->Result<(),Self::Error>;
 fn child<S:Send+Sync+'static>(&mut self,slot:&'static str,fields:ChildRefFields<'_>)->Result<(),Self::Error>;
}
pub trait ChildFieldReadAdmission<R:ChildReadSource>{fn admit_child_field(&self,slot:&'static str,source:&mut R)->Result<(),R::Error>;}
impl<T,R> ChildFieldReadAdmission<R> for Option<T> where R:ChildReadSource,T:ChildFieldReadAdmission<R>{
 fn admit_child_field(&self,slot:&'static str,source:&mut R)->Result<(),R::Error>{source.step()?;if let Some(value)=self{value.admit_child_field(slot,source)?;}Ok(())}
}
impl<T,R> ChildFieldReadAdmission<R> for Vec<T> where R:ChildReadSource,T:ChildFieldReadAdmission<R>{
 fn admit_child_field(&self,slot:&'static str,source:&mut R)->Result<(),R::Error>{source.step()?;for value in self{value.admit_child_field(slot,source)?;}Ok(())}
}
pub trait ArtifactChildReadAdmission{fn admit_child_reads<R:ChildReadSource>(&self,source:&mut R)->Result<(),R::Error>;}
