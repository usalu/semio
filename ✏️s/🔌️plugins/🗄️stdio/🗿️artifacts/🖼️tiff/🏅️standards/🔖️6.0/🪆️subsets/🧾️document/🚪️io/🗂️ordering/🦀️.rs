//! 🗂️ Stable TIFF tag ordering with admitted index storage and bounded merge work.
use semio_framework_value::{ValueError,native_decoding::NativeDecodeControl,native_encoding::NativeEncodeControl};
pub trait TagOrderControl{fn indices(&mut self,count:usize)->Result<Vec<usize>,ValueError>;fn begin(&mut self,total:usize)->Result<(),ValueError>;fn step(&mut self)->Result<(),ValueError>;}
impl TagOrderControl for NativeDecodeControl<'_>{fn indices(&mut self,count:usize)->Result<Vec<usize>,ValueError>{self.allocate_vec(count)}fn begin(&mut self,total:usize)->Result<(),ValueError>{self.begin_stage(total)}fn step(&mut self)->Result<(),ValueError>{self.step()}}
impl TagOrderControl for NativeEncodeControl<'_>{fn indices(&mut self,count:usize)->Result<Vec<usize>,ValueError>{self.allocate_vec(count)}fn begin(&mut self,total:usize)->Result<(),ValueError>{self.begin_stage(total)}fn step(&mut self)->Result<(),ValueError>{self.step()}}
/// 🔢️ Sorts literal tags stably without copying owned tag values.
pub fn order<T>(entries:&mut[T],tag:impl Fn(&T)->u16,control:&mut impl TagOrderControl)->Result<(),ValueError>{
    let count=entries.len();if count<2{return Ok(())}let mut indices=control.indices(count)?;let mut scratch=control.indices(count)?;control.begin(count)?;for index in 0..count{indices.push(index);scratch.push(0);control.step()?;}control.begin(0)?;let mut width=1usize;
    while width<count{let mut start=0;while start<count{let middle=start.saturating_add(width).min(count);let end=middle.saturating_add(width).min(count);let(mut left,mut right)=(start,middle);for target in start..end{let from_left=left<middle&&(right==end||tag(&entries[indices[left]])<=tag(&entries[indices[right]]));scratch[target]=if from_left{let value=indices[left];left+=1;value}else{let value=indices[right];right+=1;value};control.step()?;}start=end;}std::mem::swap(&mut indices,&mut scratch);if width>count/2{break}width*=2;}
    for(new,&old)in indices.iter().enumerate(){scratch[old]=new;control.step()?;}for index in 0..count{while scratch[index]!=index{let target=scratch[index];entries.swap(index,target);scratch.swap(index,target);control.step()?;}control.step()?;}Ok(())
}
