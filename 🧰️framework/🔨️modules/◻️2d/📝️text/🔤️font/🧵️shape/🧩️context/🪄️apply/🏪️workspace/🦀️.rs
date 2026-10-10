//! 🏪️ An exclusive original glyph allocation moves its uninitialized gap one real scalar record per turn.
use super::super::super::face::{FontClusterMapping,FontMappedScalar};
use std::mem::{MaybeUninit,ManuallyDrop};
#[repr(transparent)]
#[derive(Clone,Copy)]
struct GlyphSlot(MaybeUninit<FontMappedScalar>);
semio_framework_value::artifact_retire_leaf!(GlyphSlot);
const _:()=assert!(std::mem::size_of::<GlyphSlot>()==std::mem::size_of::<FontMappedScalar>()&&std::mem::align_of::<GlyphSlot>()==std::mem::align_of::<FontMappedScalar>());
#[derive(Default,semio_framework_value::RetireOwned)]
pub(crate) struct FontWorkingMapping{pub face:usize,slots:Vec<GlyphSlot>,gap_start:usize,gap_count:usize}
impl FontWorkingMapping{
 pub fn new(face:usize)->Self{Self{face,slots:Vec::new(),gap_start:0,gap_count:0}}
 /// 🪣️ MaybeUninit slots admit the actual full backing without constructing or reading uninitialized glyph values.
 pub fn admit(&mut self,maximum:usize)->Result<(),String>{if !self.slots.is_empty()||self.slots.capacity()!=0{return Err("Glyph gap workspace already admitted".into());}self.slots.try_reserve_exact(maximum).map_err(|_|"Original glyph gap admission refused")?;unsafe{self.slots.set_len(maximum);}self.gap_count=maximum;Ok(())}
 pub fn len(&self)->usize{self.slots.len()-self.gap_count}
 pub fn maximum(&self)->usize{self.slots.len()}
 pub fn pointer(&self)->*const FontMappedScalar{self.slots.as_ptr().cast()}
 /// 🔎️ Every logical position lies outside the uninitialized gap and contains an original copied or authored replacement record.
 pub fn get(&self,index:usize)->FontMappedScalar{assert!(index<self.len());let physical=index+if index>=self.gap_start{self.gap_count}else{0};unsafe{self.slots[physical].0.assume_init()}}
 pub fn insert(&mut self,value:FontMappedScalar)->Result<(),String>{if self.gap_count==0{return Err("Glyph gap capacity exceeded".into());}self.slots[self.gap_start].0.write(value);self.gap_start+=1;self.gap_count-=1;Ok(())}
 pub fn delete(&mut self,count:usize)->Result<(),String>{if count>self.len()-self.gap_start{return Err("Glyph gap deletion exceeds original authority".into());}self.gap_count+=count;Ok(())}
 pub fn move_step(&mut self,to:usize)->Result<bool,String>{if to>self.len(){return Err("Glyph gap position exceeds authority".into());}if self.gap_start<to{let value=self.get(self.gap_start);self.slots[self.gap_start].0.write(value);self.gap_start+=1;}else if self.gap_start>to{let value=self.get(self.gap_start-1);self.gap_start-=1;self.slots[self.gap_start+self.gap_count].0.write(value);}Ok(self.gap_start==to)}
 /// 📦️ A compacted initialized prefix transfers the same allocation with identical record layout and exact capacity.
 pub fn into_mapping(self)->Result<FontClusterMapping,Self>{let length=self.len();if self.gap_start!=length{return Err(self);}let mut slots=ManuallyDrop::new(self.slots);let pointer=slots.as_mut_ptr().cast::<FontMappedScalar>();let capacity=slots.capacity();let glyphs=unsafe{Vec::from_raw_parts(pointer,length,capacity)};Ok(FontClusterMapping{face:self.face,glyphs})}
}
#[cfg(test)]
mod tests{
 use super::*;
 use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneStep}};
 fn close<T:semio_framework_value::retirement::RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("{error}"));while !owner.terminal_is_empty(){let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());let(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=step;assert!(progress.fits(grant));assert!(!physical.overflowed);assert_eq!(physical.requested_bytes,progress.retained_capacity_bytes);assert_eq!(physical.released_bytes,progress.released_bytes);}let(_,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!(physical.released_bytes,0);}
 #[test]
 fn font_shape_gap_original_allocation_handoff(){let rows:serde_json::Value=serde_json::from_str(include_str!("../🧫️fixtures/🔣️.json")).unwrap();let property=super::super::super::super::unicode::UnicodeProperty{combining_class:0,category:28,bidi:0,joining:0,grapheme:12,flags:0,script:0};for row in rows.as_array().unwrap(){let repeat=row["repeat"].as_u64().unwrap()as usize;let mut owner=FontWorkingMapping::new(0);owner.admit(row["maxGlyphs"].as_u64().unwrap()as usize).unwrap();let pointer=owner.pointer();let(_,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{for at in 0..repeat*2{owner.insert(FontMappedScalar{glyph:4,scalar:0x1f1e6,source_at:at/2*8,property}).unwrap();}for at in 0..repeat{while !owner.move_step(at).unwrap(){}owner.delete(2).unwrap();owner.insert(FontMappedScalar{glyph:337,scalar:0x1f1e6,source_at:at*8,property}).unwrap();}while !owner.move_step(owner.len()).unwrap(){}});assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(owner.pointer(),pointer);let(output,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.into_mapping().ok().unwrap());assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(output.glyphs.as_ptr(),pointer);assert_eq!(output.glyphs.len(),repeat);assert_eq!(output.glyphs.capacity(),4096);for(at,glyph)in output.glyphs.iter().enumerate(){assert_eq!((glyph.glyph,glyph.source_at),(337,at*8));}close(output);println!("[DEBUG] Original gap {}: same allocation across scalar edits and zero-allocation typed handoff; actual caller-funded backing close",row["name"]);}}
}
