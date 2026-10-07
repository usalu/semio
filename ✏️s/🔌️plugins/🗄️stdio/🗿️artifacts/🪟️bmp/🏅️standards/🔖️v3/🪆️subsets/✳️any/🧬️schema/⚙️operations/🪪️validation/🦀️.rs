//! 🪪️ Fuelled native sample validation and exact owned BMP revision accumulation.
use super::*;
use semio_framework_value::retained_clone::{RetainedCloneGrant,RetainedCloneProgress};
#[derive(Default)]
pub struct BmpOwnedValidationWork {stage:u8,cursor:usize,hash:u64}
impl BmpOwnedValidationWork {
    pub fn revision(&self)->Option<String> {if self.stage>=5 {Some(format!("{:016x}",self.hash))}else{None}}
    pub fn advance(&mut self,source:&BmpSnapshot,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),String> {
        let mut used=RetainedCloneProgress::default();let image=&source.image;
        while used.copied_items<grant.maximum_items {match self.stage {
            0=>{if source.schema!=crate::STDIO_BMP_DOCUMENT_SCHEMA {return Err("bmp: undeclared semantic schema".into());}image.validate_header()?;self.hash=0xcbf29ce484222325;feed_counted(&mut self.hash,source.schema.as_bytes());feed_counted(&mut self.hash,image.profile.id().as_bytes());feed(&mut self.hash,&[u8::from(image.row_order==crate::schema::snapshot::BmpRowOrder::TopDown)]);for value in [image.width,image.height,image.colors_used,image.colors_important,image.reserved_1.into(),image.reserved_2.into()] {feed(&mut self.hash,&value.to_le_bytes());}feed(&mut self.hash,&image.x_pixels_per_meter.to_le_bytes());feed(&mut self.hash,&image.y_pixels_per_meter.to_le_bytes());for mask in image.masks {feed(&mut self.hash,&mask.to_le_bytes());}feed(&mut self.hash,&(image.palette.len()as u64).to_le_bytes());self.stage=1;},
            1=>{if self.cursor==image.palette.len(){let (kind,count)=match &image.pixels {BmpPixels::Indexed{indices}=>(0,indices.len()),BmpPixels::Direct{samples}=>(1,samples.len())};feed(&mut self.hash,&[kind]);feed(&mut self.hash,&(count as u64).to_le_bytes());self.stage=2;self.cursor=0;continue;}let entry=image.palette[self.cursor];feed(&mut self.hash,&[entry.r,entry.g,entry.b,entry.reserved]);self.cursor+=1;},
            2=>{let count=match &image.pixels {BmpPixels::Indexed{indices}=>indices.len(),BmpPixels::Direct{samples}=>samples.len()};if self.cursor==count {feed(&mut self.hash,&(image.opaque_gap.len()as u64).to_le_bytes());self.stage=3;self.cursor=0;continue;}image.validate_sample(self.cursor)?;match &image.pixels {BmpPixels::Indexed{indices}=>feed(&mut self.hash,&indices[self.cursor..self.cursor+1]),BmpPixels::Direct{samples}=>{let sample=samples[self.cursor];for value in [sample.red,sample.green,sample.blue,sample.alpha,sample.reserved] {feed(&mut self.hash,&value.to_le_bytes());}}}self.cursor+=1;},
            3|4=>{let bytes=if self.stage==3 {&image.opaque_gap}else{&image.opaque_trailer};if self.cursor==bytes.len(){self.stage+=1;self.cursor=0;if self.stage==4 {feed(&mut self.hash,&(image.opaque_trailer.len()as u64).to_le_bytes());}continue;}feed(&mut self.hash,&bytes[self.cursor..self.cursor+1]);self.cursor+=1;},
            _=>{used.copied_items+=1;return Ok((true,used));}
        }used.copied_items+=1;}Ok((false,used))
    }
}
