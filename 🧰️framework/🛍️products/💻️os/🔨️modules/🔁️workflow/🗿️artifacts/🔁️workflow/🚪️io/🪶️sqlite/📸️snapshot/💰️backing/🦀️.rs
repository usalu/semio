//! \u{1f4b0}\uFE0F Paid Workflow indexes retain original rows under one cumulative native allocation ledger.
use super::{SqliteRow,ValueError,entity};
use semio_framework_value::NativeDecodeControl;
use std::cmp::Ordering;
fn invalid(message:&str)->ValueError{crate::workflow_invalid(message)}
fn sort(rows:&mut[&SqliteRow],native:&mut NativeDecodeControl<'_>,mut compare:impl FnMut(&SqliteRow,&SqliteRow)->Result<Ordering,ValueError>)->Result<(),ValueError>{
 fn sift(rows:&mut[&SqliteRow],mut root:usize,end:usize,native:&mut NativeDecodeControl<'_>,compare:&mut impl FnMut(&SqliteRow,&SqliteRow)->Result<Ordering,ValueError>)->Result<(),ValueError>{loop{let Some(mut child)=root.checked_mul(2).and_then(|n|n.checked_add(1)).filter(|n|*n<end)else{return Ok(())};if child+1<end{native.step()?;if compare(rows[child],rows[child+1])?==Ordering::Less{child+=1;}}native.step()?;if compare(rows[root],rows[child])?!=Ordering::Less{return Ok(())}rows.swap(root,child);root=child;}}
 native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(0)?;for root in(0..rows.len()/2).rev(){let end=rows.len();sift(rows,root,end,native,&mut compare)?;}for end in(1..rows.len()).rev(){rows.swap(0,end);sift(rows,0,end,native,&mut compare)?;}native.checkpoint()})
}
fn unique(rows:&mut[&SqliteRow],columns:usize,native:&mut NativeDecodeControl<'_>)->Result<(),ValueError>{
 native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(rows.len())?;for row in rows.iter(){entity(row,columns)?;row.integer(1)?;native.step()?;}Ok(())})?;
 sort(rows,native,|a,b|Ok(a.rowid.cmp(&b.rowid)))?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(0)?;for pair in rows.windows(2){if pair[0].rowid==pair[1].rowid{return Err(invalid("Workflow entity identity is duplicated"))}native.step()?;}Ok(())})
}
fn refs<'a>(rows:&'a[SqliteRow],columns:usize,native:&mut NativeDecodeControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{let mut result=native.allocate_vec(rows.len())?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(rows.len())?;for row in rows{result.push(row);native.step()?;}Ok(())})?;unique(&mut result,columns,native)?;Ok(result)}
pub(super) fn ordered<'a>(rows:&[&'a SqliteRow],columns:usize,slot:usize,native:&mut NativeDecodeControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
 let mut result=native.allocate_vec(rows.len())?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(rows.len())?;for row in rows{result.push(*row);native.step()?;}Ok(())})?;unique(&mut result,columns,native)?;sort(&mut result,native,|a,b|Ok(a.integer(slot)?.cmp(&b.integer(slot)?)))?;
 native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(result.len())?;for(index,row)in result.iter().enumerate(){if row.integer(slot)?!=i64::try_from(index).map_err(|_|invalid("Workflow ordinal exceeds INTEGER width"))?{return Err(invalid("Workflow collection ordinals must be dense and unique"))}native.step()?;}Ok(())})?;Ok(result)
}
pub(super) fn parent_rows<'a>(rows:&'a[SqliteRow],parent:i64,columns:usize,native:&mut NativeDecodeControl<'_>)->Result<Vec<&'a SqliteRow>,ValueError>{
 let rows=refs(rows,columns,native)?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(rows.len())?;for row in&rows{if row.integer(1)?!=parent{return Err(invalid("Workflow child has an unknown parent"))}native.step()?;}Ok(())})?;ordered(&rows,columns,2,native)
}
pub(super) struct Groups<'a>{rows:Vec<&'a SqliteRow>,used:Vec<u8>,remaining:usize}
impl<'a>Groups<'a>{
 fn new(rows:&'a[SqliteRow],columns:usize,single:bool,native:&mut NativeDecodeControl<'_>)->Result<Self,ValueError>{let mut rows=refs(rows,columns,native)?;sort(&mut rows,native,|a,b|Ok(a.integer(1)?.cmp(&b.integer(1)?)))?;if single{native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(0)?;for pair in rows.windows(2){if pair[0].integer(1)?==pair[1].integer(1)?{return Err(invalid("Workflow branch requires one unique entity per parent"))}native.step()?;}Ok(())})?;}let mut used=native.allocate_vec(rows.len())?;native.scoped_stage(|native|->Result<(),ValueError>{native.begin_stage(rows.len())?;for _ in&rows{used.push(0);native.step()?;}Ok(())})?;let remaining=rows.len();Ok(Self{rows,used,remaining})}
 fn range(&self,parent:i64)->std::ops::Range<usize>{let start=self.rows.partition_point(|row|row.integer(1).expect("validated Workflow parent INTEGER")<parent);let end=self.rows.partition_point(|row|row.integer(1).expect("validated Workflow parent INTEGER")<=parent);start..end}
 pub(super) fn remove(&mut self,parent:&i64)->Option<&[&'a SqliteRow]>{let range=self.range(*parent);if range.is_empty()||self.used[range.start]!=0{return None}self.used[range.start]=1;self.remaining-=range.len();Some(&self.rows[range])}
 pub(super) fn is_empty(&self)->bool{self.remaining==0}
}
pub(super) struct Single<'a>(Groups<'a>);
impl<'a>Single<'a>{pub(super) fn remove(&mut self,parent:&i64)->Option<&'a SqliteRow>{self.0.remove(parent).map(|rows|rows[0])}pub(super) fn is_empty(&self)->bool{self.0.is_empty()}}
pub(super) fn single_rows<'a>(rows:&'a[SqliteRow],columns:usize,native:&mut NativeDecodeControl<'_>)->Result<Single<'a>,ValueError>{Groups::new(rows,columns,true,native).map(Single)}
pub(super) fn grouped<'a>(rows:&'a[SqliteRow],columns:usize,native:&mut NativeDecodeControl<'_>)->Result<Groups<'a>,ValueError>{Groups::new(rows,columns,false,native)}
