//! 🖌️ Precise owned TIFF region paint and intrinsic revision identity.
use crate::schema::diff::{differing_runs, normalize_runs, TiffSampleRun};
use crate::schema::snapshot::*;

#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct TiffRegion { pub x:u32, pub y:u32, pub width:u32, pub height:u32 }
pub const TIFF_MAXIMUM_INTERACTIVE_PAINT_ROWS:u32=65_536;

pub fn tiff_revision(snapshot:&TiffSnapshot)->String {
 struct Fingerprint(u64);
 impl Fingerprint {fn bytes(&mut self,bytes:&[u8]){for byte in bytes{self.0=(self.0^u64::from(*byte)).wrapping_mul(0x100000001b3);}}fn count(&mut self,count:usize){self.bytes(&(count as u64).to_le_bytes());}}
 let mut hash=Fingerprint(0xcbf29ce484222325);hash.count(snapshot.schema.len());hash.bytes(snapshot.schema.as_bytes());hash.count(snapshot.ifds.len());
 for ifd in &snapshot.ifds {
  hash.count(ifd.entries.len());
  for tag in &ifd.entries {
   hash.bytes(&tag.tag.to_le_bytes());hash.bytes(&[tag.values.kind()as u8]);hash.count(tag.values.count()as usize);
   macro_rules! words {($values:expr)=>{for value in $values{hash.bytes(&value.to_le_bytes());}};}
   match &tag.values {
    TiffValues::Byte(v)|TiffValues::Undefined(v)=>hash.bytes(v),
    TiffValues::Ascii(texts)=>for text in texts{hash.count(text.len());hash.bytes(text.as_bytes());},
    TiffValues::Short(v)=>words!(v),TiffValues::Long(v)=>words!(v),TiffValues::SByte(v)=>for value in v{hash.bytes(&[*value as u8]);},
    TiffValues::SShort(v)=>words!(v),TiffValues::SLong(v)=>words!(v),
    TiffValues::Rational(v)=>for(a,b)in v{hash.bytes(&a.to_le_bytes());hash.bytes(&b.to_le_bytes());},
    TiffValues::SRational(v)=>for(a,b)in v{hash.bytes(&a.to_le_bytes());hash.bytes(&b.to_le_bytes());},
    TiffValues::Float(v)=>for value in v{hash.bytes(&value.bits.to_le_bytes());},
    TiffValues::Double(v)=>for value in v{hash.bytes(&value.lo.to_le_bytes());hash.bytes(&value.hi.to_le_bytes());}
   }
  }
  hash.count(ifd.blocks.len());for block in &ifd.blocks{for value in [block.x,block.y,block.width,block.height,u32::from(block.channels)]{hash.bytes(&value.to_le_bytes());}hash.count(block.samples.len());for sample in &block.samples{hash.bytes(&sample.lo.to_le_bytes());hash.bytes(&sample.hi.to_le_bytes());}}
 }
 format!("{:016x}",hash.0)
}

fn colors(ifd:&TiffIfd,color:[u8;4])->Result<Vec<TiffWord64>,String>{
 let block=ifd.blocks.first().ok_or("tiff: paint requires owned image samples")?;let channels=block.channels as usize;
 let mut depths=ifd.integers(TAG_BITS_PER_SAMPLE);if depths.is_empty(){depths.push(1)}if depths.len()==1{depths.resize(channels,depths[0])}
 let mut formats=ifd.integers(339);if formats.is_empty(){formats.push(1)}if formats.len()==1{formats.resize(channels,formats[0])}
 let photo=ifd.integer(TAG_PHOTOMETRIC).unwrap_or(1);
 let components=match (photo,channels){
  (0|1,1)if color[0]==color[1]&&color[1]==color[2]&&color[3]==255=>vec![if photo==0{255-color[0]}else{color[0]}],
  (2,3)if color[3]==255=>color[..3].to_vec(),(2,4)=>color.to_vec(),
  (3,1)if color[3]==255=>{
   let palette=ifd.integers(320);let count=palette.len()/3;let index=(0..count).find(|index|(0..3).all(|lane|(u64::from(palette[lane*count+index])*255/65535)as u8==color[lane])).ok_or("tiff: indexed paint color is absent from owned palette")?;return Ok(vec![TiffWord64::from_word(index as u64)]);
  }
  _=>return Err("tiff: paint color differs from owned channel interpretation".into())
 };
 Ok(components.into_iter().enumerate().map(|(lane,component)|{
  let depth=depths[lane];let word=match formats[lane]{
   3 if depth==16=>{let bits=(f32::from(component)/255.0).to_bits();if component==0{0}else{let exponent=((bits>>23)&255)as i32-127+15;let fraction=bits&0x7fffff;let retained=fraction>>13;let remainder=fraction&8191;((exponent as u64)<<10)+u64::from(retained)+u64::from(remainder>4096||remainder==4096&&retained&1!=0)}},
   3 if depth==32=>(f32::from(component)/255.0).to_bits()as u64,
   3=>(f64::from(component)/255.0).to_bits(),
   format=>{let maximum=if depth==64{u64::MAX}else{(1u64<<depth)-1};let unsigned=((u128::from(component)*u128::from(maximum)+127)/255)as u64;if format==2{unsigned^(1u64<<(depth-1))}else{unsigned}}
  };TiffWord64::from_word(word)
 }).collect())
}
pub fn validate_tiff_region_paint(snapshot:&TiffSnapshot,ifd_index:usize,region:TiffRegion,color:[u8;4])->Result<(),String>{
 snapshot.validate()?;let ifd=snapshot.ifds.get(ifd_index).ok_or("tiff: paint page index outside owned document")?;let block=ifd.blocks.first().ok_or("tiff: paint page has no owned samples")?;
 if region.width==0||region.height==0||region.height>TIFF_MAXIMUM_INTERACTIVE_PAINT_ROWS||region.x.checked_add(region.width).is_none_or(|end|end>block.width)||region.y.checked_add(region.height).is_none_or(|end|end>block.height){return Err("tiff: paint region exceeds owned page extent or interactive row limit".into())}
 colors(ifd,color)?;Ok(())
}
pub fn paint_tiff_region_runs(snapshot:&TiffSnapshot,revision:&str,ifd_index:usize,region:TiffRegion,color:[u8;4],progress:&mut dyn FnMut(usize,usize)->bool)->Result<Vec<TiffSampleRun>,String>{
 if revision!=tiff_revision(snapshot){return Err("tiff: stale owned revision".into())}validate_tiff_region_paint(snapshot,ifd_index,region,color)?;
 let values=colors(&snapshot.ifds[ifd_index],color)?;let total=region.height as usize;if !progress(0,total){return Err("tiff: paint cancelled".into())}
 let block=&snapshot.ifds[ifd_index].blocks[0];let channels=block.channels as usize;let width=region.width as usize*channels;
 let painted:Vec<TiffWord64>=values.iter().copied().cycle().take(width).collect();let mut runs=Vec::new();
 for row in 0..total{let start=((region.y as usize+row)*block.width as usize+region.x as usize)*channels;runs.extend(differing_runs(0,start,&block.samples[start..start+width],&painted));if !progress(row+1,total){return Err("tiff: paint cancelled".into())}}
 Ok(normalize_runs(runs))
}
pub fn paint_tiff_region_controlled(snapshot:&TiffSnapshot,revision:&str,ifd_index:usize,region:TiffRegion,color:[u8;4],progress:&mut dyn FnMut(usize,usize)->bool)->Result<TiffSnapshot,String>{
 let runs=paint_tiff_region_runs(snapshot,revision,ifd_index,region,color,progress)?;let mut next=snapshot.clone();
 for run in &runs{next.ifds[ifd_index].blocks[run.block].samples[run.offset..run.offset+run.samples.len()].copy_from_slice(&run.samples);}
 Ok(next)
}
