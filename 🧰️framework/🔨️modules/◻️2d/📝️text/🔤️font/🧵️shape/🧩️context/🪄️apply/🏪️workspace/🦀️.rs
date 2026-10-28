//! 🏪️ An exclusive original glyph allocation moves its uninitialized gap one real scalar record per turn.
use super::super::super::face::{FontClusterMapping,FontMappedScalar};
use std::mem::{MaybeUninit,ManuallyDrop};
#[repr(transparent)]
#[derive(Clone,Copy)]
struct GlyphSlot(MaybeUninit<FontMappedScalar>);
semio_framework_value::artifact_retire_leaf!(GlyphSlot);
const _:()=assert!(size_of::<GlyphSlot>()==size_of::<FontMappedScalar>()&&align_of::<GlyphSlot>()==align_of::<FontMappedScalar>());
#[derive(Default,semio_framework_value::RetireOwned)]
pub(crate) struct FontWorkingMapping{pub face:usize,slots:Vec<GlyphSlot>,gap_start:usize,gap_count:usize}
impl FontWorkingMapping{
 pub fn new(face:usize)->Self{Self{face,slots:Vec::new(),gap_start:0,gap_count:0}}
 /// 🪣️ MaybeUninit slots admit the actual full backing without constructing or reading uninitialized glyph values.
 pub fn admit(&mut self,maximum:usize,work:&mut semio_framework_2d_text_font::work::FontWorkLedger)->Result<bool,&'static str>{if !self.slots.is_empty(){return Err("Glyph gap workspace already admitted");}if !work.contiguous(&mut self.slots,maximum)?{return Ok(false);}unsafe{self.slots.set_len(maximum);}self.gap_count=maximum;Ok(true)}
 pub fn len(&self)->usize{self.slots.len()-self.gap_count}
 pub fn maximum(&self)->usize{self.slots.len()}
 pub fn pointer(&self)->*const FontMappedScalar{self.slots.as_ptr().cast()}
 /// 🔎️ Every logical position lies outside the uninitialized gap and contains an original copied or authored replacement record.
 pub fn get(&self,index:usize)->FontMappedScalar{assert!(index<self.len());let physical=index+if index>=self.gap_start{self.gap_count}else{0};unsafe{self.slots[physical].0.assume_init()}}
 pub fn insert(&mut self,value:FontMappedScalar,work:&mut semio_framework_2d_text_font::work::FontWorkLedger)->Result<bool,&'static str>{if self.gap_count==0{return Err("Glyph gap capacity exceeded".into());}if !work.copy_bytes(size_of::<FontMappedScalar>()){return Ok(false);}self.slots[self.gap_start].0.write(value);self.gap_start+=1;self.gap_count-=1;Ok(true)}
 pub fn delete(&mut self,count:usize)->Result<(),&'static str>{if count>self.len()-self.gap_start{return Err("Glyph gap deletion exceeds original authority".into());}self.gap_count+=count;Ok(())}
 pub fn move_step(&mut self,to:usize,work:&mut semio_framework_2d_text_font::work::FontWorkLedger)->Result<bool,&'static str>{if to>self.len(){return Err("Glyph gap position exceeds authority".into());}if self.gap_start!=to&&!work.copy_bytes(size_of::<FontMappedScalar>()){return Ok(false);}if self.gap_start<to{let value=self.get(self.gap_start);self.slots[self.gap_start].0.write(value);self.gap_start+=1;}else if self.gap_start>to{let value=self.get(self.gap_start-1);self.gap_start-=1;self.slots[self.gap_start+self.gap_count].0.write(value);}Ok(self.gap_start==to)}
 /// 📦️ A compacted initialized prefix transfers the same allocation with identical record layout and exact capacity.
 pub fn into_mapping(self)->Result<FontClusterMapping,Self>{let length=self.len();if self.gap_start!=length{return Err(self);}let mut slots=ManuallyDrop::new(self.slots);let pointer=slots.as_mut_ptr().cast::<FontMappedScalar>();let capacity=slots.capacity();let glyphs=unsafe{Vec::from_raw_parts(pointer,length,capacity)};Ok(FontClusterMapping{face:self.face,glyphs})}
}
#[cfg(test)]
mod tests{
 use super::*;
 fn fixture_work()->semio_framework_2d_text_font::work::FontWorkLedger{semio_framework_2d_text_font::work::FontWorkLedger::new(semio_framework_value::RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:16777216,maximum_capacity_bytes:67108864,maximum_release_bytes:67108864,maximum_depth:128})}
 use semio_framework_value::{retirement::controlled::ControlledRetirement,retained_clone::{RetainedCloneGrant,RetainedCloneStep}};
 fn close<T:semio_framework_value::retirement::RetireOwned>(value:T){let mut owner=ControlledRetirement::new(value).unwrap_or_else(|(error,_)|panic!("{error}"));while !owner.terminal_is_empty(){let copy=owner.next_copy_byte_demand().unwrap();let release=owner.next_release_byte_demand().unwrap();let grant=RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:copy,maximum_capacity_bytes:owner.next_capacity_byte_demand(if copy>0{copy}else{release}).unwrap(),maximum_release_bytes:release,maximum_depth:owner.next_depth_demand().unwrap()};let(step,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.step(grant).unwrap());let(RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress))=step;assert!(progress.fits(grant));assert!(!physical.overflowed);assert_eq!(physical.requested_bytes,progress.retained_capacity_bytes);assert_eq!(physical.released_bytes,progress.released_bytes);}let(_,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(owner));assert_eq!(physical.released_bytes,0);}
 #[test]
 fn font_shape_gap_original_allocation_handoff(){let rows:serde_json::Value=serde_json::from_str(include_str!("../🧪️tests/🧫️fixtures/🔣️.json")).unwrap();let property=semio_framework_2d_text_font_shape_unicode::UnicodeProperty{combining_class:0,category:28,bidi:0,joining:0,grapheme:12,flags:0,script:0};for row in rows.as_array().unwrap(){let repeat=row["repeat"].as_u64().unwrap()as usize;let mut owner=FontWorkingMapping::new(0);while !owner.admit(row["maxGlyphs"].as_u64().unwrap()as usize,&mut fixture_work()).unwrap(){}let pointer=owner.pointer();let(_,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{for at in 0..repeat*2{owner.insert(FontMappedScalar{glyph:4,scalar:0x1f1e6,source_at:at/2*8,property},&mut fixture_work()).unwrap();}for at in 0..repeat{while !owner.move_step(at,&mut fixture_work()).unwrap(){}owner.delete(2).unwrap();owner.insert(FontMappedScalar{glyph:337,scalar:0x1f1e6,source_at:at*8,property},&mut fixture_work()).unwrap();}while !owner.move_step(owner.len(),&mut fixture_work()).unwrap(){}});assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(owner.pointer(),pointer);let(output,physical)=semio_framework_trace::observe_heap_allocations_on_this_thread(||owner.into_mapping().ok().unwrap());assert_eq!((physical.requested_bytes,physical.released_bytes),(0,0));assert_eq!(output.glyphs.as_ptr(),pointer);assert_eq!(output.glyphs.len(),repeat);assert_eq!(output.glyphs.capacity(),4096);for(at,glyph)in output.glyphs.iter().enumerate(){assert_eq!((glyph.glyph,glyph.source_at),(337,at*8));}close(output);}}
}
