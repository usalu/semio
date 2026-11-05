//! 🔑️ Canonical decoded-image identity advances through explicit byte grants without encoding a carrier.
use super::*;

pub const RASTER_IMAGE_KEY_BYTES: usize = 22;
pub const RASTER_IMAGE_CHILD_ID_BYTES: usize = 29;

fn format_key(prefix:&str,digest:[u8;8])->String{
    let mut key=String::with_capacity(prefix.len()+17);key.push_str(prefix);key.push('-');
    for byte in digest{key.push(char::from(b"0123456789abcdef"[(byte>>4)as usize]));key.push(char::from(b"0123456789abcdef"[(byte&15)as usize]));}
    key
}

/// 🗝️ Canonical shared asset identity, independent of its layer and every image carrier.
pub fn raster_image_key(digest: [u8;8]) -> String {
    format_key("image",digest)
}

/// 🪆️ The canonical local image-child identity uses the same decoded-content digest.
pub fn raster_image_child_id(digest:[u8;8])->String{format_key("raster-asset",digest)}

enum ContentField<'a> { Bytes(&'a [u8]), Inline([u8;8],usize) }

impl ContentField<'_> {
    fn bytes(&self) -> &[u8] { match self { Self::Bytes(bytes) => bytes, Self::Inline(bytes,length) => &bytes[..*length] } }
}

fn word(value: u64, length: usize) -> ContentField<'static> { ContentField::Inline(value.to_le_bytes(),length) }

fn field(image: &SemioImageSnapshot, index: usize) -> Option<ContentField<'_>> {
    match index {
        0 => return Some(ContentField::Bytes(image.schema.as_bytes())),
        1 => return Some(word(u64::from(image.width),4)),
        2 => return Some(word(u64::from(image.height),4)),
        3 => return Some(ContentField::Inline([image.colorspace as u8,image.bit_depth,0,0,0,0,0,0],2)),
        4 => return Some(word(image.frames.len() as u64,8)),
        _ => {}
    }
    let index = index - 5;
    if index / 2 < image.frames.len() {
        let frame = &image.frames[index / 2];
        return Some(if index % 2 == 0 { word(u64::from(frame.delay_ms),4) } else { ContentField::Bytes(&frame.rgba8) });
    }
    let mut index = index - image.frames.len() * 2;
    if index == 0 { return Some(word(u64::from(image.icc.is_some()),1)); }
    index -= 1;
    if let Some(icc) = &image.icc {
        if index == 0 { return Some(ContentField::Bytes(icc)); }
        index -= 1;
    }
    if index == 0 { return Some(word(image.metadata.len() as u64,8)); }
    index -= 1;
    image.metadata.get(index / 2).map(|entry| ContentField::Bytes(if index % 2 == 0 { entry.key.as_bytes() } else { entry.value.as_bytes() }))
}

/// 📊️ Bytes actually processed in this grant and whether every canonical field was hashed.
pub struct RasterImageHashProgress { pub processed_bytes: usize, pub done: bool }

/// 🟰️ Exact equality is established only after every canonical field has been compared.
pub struct RasterImageEqualityProgress {pub processed_bytes:usize,pub done:bool,pub equal:bool}

/// 🔍️ Allocation-free canonical comparison borrows the caller's captured immutable image roots.
pub struct RasterImageContentEquality {field:usize,offset:usize,binding:Option<(usize,usize)>,done:bool,equal:bool}

impl Default for RasterImageContentEquality {fn default()->Self{Self::new()}}

impl RasterImageContentEquality {
    pub fn new()->Self{Self {field:0,offset:0,binding:None,done:false,equal:true}}

    pub fn advance(&mut self,left:&SemioImageSnapshot,right:&SemioImageSnapshot,budget:usize)->Result<RasterImageEqualityProgress,&'static str>{
        if !(2..=32768).contains(&budget){return Err("invalid image equality byte grant");}
        let binding=(left as *const SemioImageSnapshot as usize,right as *const SemioImageSnapshot as usize);
        if self.binding.is_some_and(|original|original!=binding){return Err("image equality source changed");}self.binding=Some(binding);
        let mut processed=0;
        while !self.done&&budget-processed>=2{
            let (left,right)=match(field(left,self.field),field(right,self.field)){
                (Some(left),Some(right))=>(left,right),
                (None,None)=>{self.done=true;break;},
                _=>{self.done=true;self.equal=false;break;},
            };
            let left=left.bytes();let right=right.bytes();
            if self.offset<8{
                let count=(8-self.offset).min((budget-processed)/2);let left_prefix=(left.len() as u64).to_le_bytes();let right_prefix=(right.len() as u64).to_le_bytes();
                let equal=left_prefix[self.offset..self.offset+count]==right_prefix[self.offset..self.offset+count];self.offset+=count;processed+=count*2;
                if !equal{self.equal=false;self.done=true;break;}
            }
            if self.offset>=8{
                let start=self.offset-8;let count=(left.len()-start).min((budget-processed)/2);
                let equal=left[start..start+count]==right[start..start+count];self.offset+=count;processed+=count*2;
                if !equal{self.equal=false;self.done=true;break;}
                if self.offset-8==left.len(){self.field+=1;self.offset=0;}
            }
        }
        if !self.done{match(field(left,self.field),field(right,self.field)){(None,None)=>self.done=true,(None,Some(_))|(Some(_),None)=>{self.done=true;self.equal=false;},_=>{}}}
        Ok(RasterImageEqualityProgress {processed_bytes:processed,done:self.done,equal:self.done&&self.equal})
    }
}

/// 🧵️ Incremental SHA-256 of the same decoded fields used by Raster asset identities.
pub struct RasterImageContentHash {
    hash: framework_hash::Sha256,
    field: usize,
    offset: usize,
    binding: Option<usize>,
    done: bool,
}

impl Default for RasterImageContentHash { fn default() -> Self { Self::new() } }

impl RasterImageContentHash {
    pub fn new() -> Self { Self { hash: framework_hash::Sha256::new(), field: 0, offset: 0, binding: None, done: false } }

    pub fn advance(&mut self, image: &SemioImageSnapshot, budget: usize) -> Result<RasterImageHashProgress,&'static str> {
        if budget == 0 || budget > 32_768 { return Err("invalid image hash byte grant"); }
        let binding = image as *const SemioImageSnapshot as usize;
        if self.binding.is_some_and(|original| original != binding) { return Err("image hash source changed"); }
        self.binding = Some(binding);
        if self.done { return Ok(RasterImageHashProgress { processed_bytes: 0, done: true }); }
        let mut processed = 0;
        while processed < budget {
            let Some(current) = field(image,self.field) else { self.done = true; break; };
            let bytes = current.bytes();
            if self.offset < 8 {
                let prefix = (bytes.len() as u64).to_le_bytes();
                let count = (8 - self.offset).min(budget - processed);
                self.hash.update(&prefix[self.offset..self.offset + count]);
                self.offset += count;
                processed += count;
            }
            if self.offset >= 8 {
                let start = self.offset - 8;
                let count = (bytes.len() - start).min(budget - processed);
                self.hash.update(&bytes[start..start + count]);
                self.offset += count;
                processed += count;
                if self.offset - 8 == bytes.len() { self.field += 1; self.offset = 0; }
            }
        }
        if field(image,self.field).is_none() { self.done = true; }
        Ok(RasterImageHashProgress { processed_bytes: processed, done: self.done })
    }

    pub fn finish(self) -> Option<[u8;8]> { self.done.then(|| self.hash.finalize()[..8].try_into().expect("SHA-256 prefix has eight bytes")) }
}

#[cfg(test)]
#[path = "🧪️tests/🦀️.rs"]
mod tests;
