//! 📋️ Fixed native rows retain only their initialized prefix, with one caller-owned slot moved per funded action.
use crate::{RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,ValueError,ValueRefusalKind,retained_clone_grant_admits};
use std::mem::MaybeUninit;
pub struct NativeOwnedPrefix<T,const N:usize>{rows:MaybeUninit<[T;N]>,initialized:usize}
fn invalid(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,message)}
fn pending()->RetainedWorkOutcome{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}
fn complete()->RetainedWorkOutcome{RetainedWorkOutcome{result:Ok(RetainedCloneStep::Complete(Default::default())),work:0}}
impl<T,const N:usize>NativeOwnedPrefix<T,N>{
 pub const fn empty()->Self{Self{rows:MaybeUninit::uninit(),initialized:0}}
 pub fn len(&self)->usize{self.initialized}
 pub const fn capacity(&self)->usize{N}
 pub fn is_empty(&self)->bool{self.initialized==0}
 pub fn as_slice(&self)->&[T]{unsafe{std::slice::from_raw_parts(self.rows.as_ptr().cast::<T>(),self.initialized)}}
 pub fn last(&self)->Option<&T>{self.as_slice().last()}
 pub fn get(&self,index:usize)->Option<&T>{self.as_slice().get(index)}
 pub fn push_demand()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 pub fn move_demand()->RetirementDemand{RetirementDemand{depth:1,..Default::default()}}
 pub fn push_from(&mut self,input:&mut Option<T>,g:RetainedCloneGrant)->RetainedWorkOutcome{if input.is_none(){return pending()}if self.initialized==N{return RetainedWorkOutcome{result:Err(invalid("native inline prefix has no uninitialized row")),work:0}}let d=Self::push_demand();if !retained_clone_grant_admits(g,d){return pending()}let value=match input.take(){Some(v)=>v,None=>return RetainedWorkOutcome{result:Err(ValueError::invariant("input absent").into()),work:0}};unsafe{self.rows.as_mut_ptr().cast::<T>().add(self.initialized).write(value)}self.initialized+=1;RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()})),work:1}}
 pub fn pop_into(&mut self,output:&mut Option<T>,g:RetainedCloneGrant)->RetainedWorkOutcome{if self.initialized==0{return complete()}if output.is_some(){return RetainedWorkOutcome{result:Err(invalid("native inline prefix output slot already retains its owner")),work:0}}if !retained_clone_grant_admits(g,Self::move_demand()){return pending()}self.initialized-=1;*output=Some(unsafe{self.rows.as_ptr().cast::<T>().add(self.initialized).read()});RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
 pub fn swap(&mut self,left:usize,right:usize,g:RetainedCloneGrant)->RetainedWorkOutcome{if left>=self.initialized||right>=self.initialized{return RetainedWorkOutcome{result:Err(invalid("native inline prefix swap names an uninitialized row")),work:0}}if left==right{return complete()}if !retained_clone_grant_admits(g,Self::move_demand()){return pending()}unsafe{let rows=self.rows.as_mut_ptr().cast::<T>();std::ptr::swap(rows.add(left),rows.add(right));}RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}
}
impl<T,const N:usize>Drop for NativeOwnedPrefix<T,N>{fn drop(&mut self){assert!(std::thread::panicking()||self.initialized==0,"native inline prefix retains live rows until their paid pop");}}
#[cfg(test)]#[path="🎟️work/🧪️tests/🦀️.rs"]mod tests;
