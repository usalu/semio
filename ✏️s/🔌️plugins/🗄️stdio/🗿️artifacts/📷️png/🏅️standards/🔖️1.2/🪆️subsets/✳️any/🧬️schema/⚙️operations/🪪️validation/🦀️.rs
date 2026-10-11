//! 🪪️ Fuelled validation of logical image owners and exact native sample paint proofs.
use super::*;
use crate::schema::snapshot::{PngAncillaryChunk,PngTextKind};
use semio_framework_value::retained_clone::{bulk_run_elements,RetainedCloneGrant,RetainedCloneProgress};

#[derive(Default)]
pub struct PngOwnedValidationWork {stage:u8,cursor:usize,field:usize,byte:usize,keyword:usize,after_raster:bool,hash:Option<PngRevisionHash>}

impl PngOwnedValidationWork {
    pub fn revision(&self)->Option<String> { if self.stage>=4 { self.hash.as_ref().map(|hash|format!("{:016x}",hash.0)) } else {None} }
    pub fn advance(&mut self,source:&PngSnapshot,target:Option<(&PngSnapshot,PngRegion,PngNativePaint,&str)>,grant:RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),String> {
        let mut used=RetainedCloneProgress::default();
        if grant.maximum_items==0 {return Ok((false,used));}
        let image=&source.image;
        while used.copied_items<grant.maximum_items {
            let mut page=0usize;
            let budget=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(used.copied_bytes),..grant};
            match self.stage {
                0=>{
                    if source.schema!=crate::STDIO_PNG_DOCUMENT_SCHEMA {return Err("png: undeclared semantic schema".into());}
                    image.validate_header()?;
                    if let Some((next,region,paint,revision))=target {
                        validate_native_paint_target(source,region,paint)?;
                        next.image.validate_header()?;
                        if revision.len()!=16||!revision.bytes().all(|byte|byte.is_ascii_hexdigit()) {return Err("png: malformed source revision".into());}
                        let other=&next.image;
                        if next.schema!=source.schema||image.width!=other.width||image.height!=other.height||image.bit_depth!=other.bit_depth||image.color_type!=other.color_type||image.interlace!=other.interlace||image.palette!=other.palette||image.transparency!=other.transparency||image.gamma!=other.gamma||image.chromaticities!=other.chromaticities||image.srgb!=other.srgb||image.physical_dims!=other.physical_dims||image.timestamp!=other.timestamp||image.background!=other.background||image.text_chunks.len()!=other.text_chunks.len()||image.ancillary_chunks.len()!=other.ancillary_chunks.len() {return Err("png: completed paint changes owned metadata".into());}
                    }
                    let mut hash=PngRevisionHash(0xcbf29ce484222325);hash.text(&source.schema);
                    for n in [u64::from(image.width),u64::from(image.height),u64::from(image.bit_depth),u64::from(image.color_type.to_u8()),u64::from(image.interlace)] {hash.number(n);}
                    hash.number(image.samples.len()as u64);self.hash=Some(hash);self.stage=1;
                },
                1=>{
                    if self.cursor==image.samples.len() {hash_image_metadata(self.hash.as_mut().ok_or("png: missing revision accumulator")?,image);self.hash.as_mut().ok_or("png: missing revision accumulator")?.number(image.text_chunks.len()as u64);self.stage=2;self.cursor=0;continue;}
                    let run=(image.samples.len()-self.cursor).min(bulk_run_elements(budget,std::mem::size_of::<u16>()));
                    if run==0 {return Ok((false,used));}
                    let maximum=if image.bit_depth==16 {u16::MAX}else{(1u16<<image.bit_depth)-1};
                    let spp=image.color_type.samples_per_pixel();
                    let hash=self.hash.as_mut().ok_or("png: missing revision accumulator")?;
                    for cursor in self.cursor..self.cursor+run {
                        let sample=image.samples[cursor];
                        if sample>maximum||image.color_type==PngColorType::Palette&&usize::from(sample)>=image.palette.as_ref().map_or(0,Vec::len) {return Err("png: sample exceeds owned profile".into());}
                        if let Some((next,region,paint,_))=target {
                            let pixel=cursor/spp;let x=pixel%image.width as usize;let y=pixel/image.width as usize;
                            let painted=x>=region.x as usize&&x<(region.x+region.width)as usize&&y>=region.y as usize&&y<(region.y+region.height)as usize;
                            let expected=if painted {paint.samples()[cursor%spp]}else{sample};
                            if next.image.samples[cursor]!=expected {return Err("png: completed paint differs from addressed native samples".into());}
                        }
                        hash.number(u64::from(sample));
                    }
                    self.cursor+=run;page=run*std::mem::size_of::<u16>();
                },
                2=>{
                    if self.cursor==image.text_chunks.len() {self.hash.as_mut().ok_or("png: missing revision accumulator")?.number(image.ancillary_chunks.len()as u64);self.stage=3;self.cursor=0;continue;}
                    let text=&image.text_chunks[self.cursor];
                    if self.field==0&&self.byte==0 {
                        if text.keyword.is_empty()||text.kind==PngTextKind::Text&&text.compressed||text.kind==PngTextKind::ZText&&!text.compressed||text.kind!=PngTextKind::IText&&(!text.language_tag.is_empty()||!text.translated_keyword.is_empty()) {return Err("png: invalid owned text profile".into());}
                        if let Some((next,_,_,_))=target {let other=&next.image.text_chunks[self.cursor];if text.kind!=other.kind||text.compressed!=other.compressed {return Err("png: completed paint changes text profile".into());}}
                    }
                    let input=match self.field {0=>&text.keyword,1=>&text.value,2=>&text.language_tag,_=>&text.translated_keyword};
                    if self.byte<input.len()&&bulk_run_elements(budget,4)==0 {return Ok((false,used));}
                    if self.byte==0 {self.hash.as_mut().ok_or("png: missing revision accumulator")?.number(input.len()as u64);if let Some((next,_,_,_))=target {let other=&next.image.text_chunks[self.cursor];let other=match self.field {0=>&other.keyword,1=>&other.value,2=>&other.language_tag,_=>&other.translated_keyword};if input.len()!=other.len() {return Err("png: completed paint changes text extent".into());}}}
                    if self.byte==input.len() {
                        if self.field==1 {let hash=self.hash.as_mut().ok_or("png: missing revision accumulator")?;hash.number(u64::from(text.compressed));hash.number(match text.kind {PngTextKind::Text=>0,PngTextKind::ZText=>1,PngTextKind::IText=>2});}
                        self.byte=0;self.field+=1;if self.field==4 {self.field=0;self.cursor+=1;self.keyword=0;}
                    }else{
                        let maximum=bulk_run_elements(budget,1);let mut consumed=0usize;
                        while self.byte<input.len() {
                            let character=input[self.byte..].chars().next().ok_or("png: invalid owned text cursor")?;
                            let width=character.len_utf8();
                            if consumed+width>maximum {break;}
                            if self.field==0 {self.keyword+=1;if self.keyword>79||character=='\0'||character as u32>255 {return Err("png: invalid owned keyword".into());}}
                            if self.field==1&&text.kind!=PngTextKind::IText&&character as u32>255||self.field==2&&!character.is_ascii() {return Err("png: invalid owned text character profile".into());}
                            let end=self.byte+width;let bytes=&input.as_bytes()[self.byte..end];
                            if let Some((next,_,_,_))=target {let other=&next.image.text_chunks[self.cursor];let other=match self.field {0=>&other.keyword,1=>&other.value,2=>&other.language_tag,_=>&other.translated_keyword};if bytes!=&other.as_bytes()[self.byte..end] {return Err("png: completed paint changes owned text".into());}}
                            self.hash.as_mut().ok_or("png: missing revision accumulator")?.bytes(bytes);self.byte=end;consumed+=width;
                        }
                        page=consumed;
                    }
                },
                3=>{
                    if self.cursor==image.ancillary_chunks.len() {self.stage=4;continue;}
                    let chunk=&image.ancillary_chunks[self.cursor];
                    let run=(chunk.data.len()-self.byte).min(bulk_run_elements(budget,1));
                    if self.byte<chunk.data.len()&&run==0 {return Ok((false,used));}
                    if self.byte==0 {
                        check_ancillary(chunk,self.after_raster)?;self.after_raster|=chunk.after_raster;
                        if let Some((next,_,_,_))=target {let other=&next.image.ancillary_chunks[self.cursor];if chunk.kind!=other.kind||chunk.after_raster!=other.after_raster||chunk.data.len()!=other.data.len() {return Err("png: completed paint changes ancillary metadata".into());}}
                        let hash=self.hash.as_mut().ok_or("png: missing revision accumulator")?;hash.bytes(&chunk.kind);hash.number(chunk.data.len()as u64);
                    }
                    if self.byte==chunk.data.len() {self.hash.as_mut().ok_or("png: missing revision accumulator")?.number(u64::from(chunk.after_raster));self.byte=0;self.cursor+=1;}else{
                        let end=self.byte+run;
                        if let Some((next,_,_,_))=target {if chunk.data[self.byte..end]!=next.image.ancillary_chunks[self.cursor].data[self.byte..end] {return Err("png: completed paint changes ancillary octets".into());}}
                        self.hash.as_mut().ok_or("png: missing revision accumulator")?.bytes(&chunk.data[self.byte..end]);self.byte=end;page=run;
                    }
                },
                _=>{
                    if let Some((_,_,_,revision))=target {if format!("{:016x}",self.hash.as_ref().ok_or("png: missing revision accumulator")?.0)!=revision {return Err("png: source revision changed".into());}}
                    used.copied_items+=1;return Ok((true,used));
                }
            }
            used.copied_items+=1;used.copied_bytes+=page;
        }
        Ok((false,used))
    }
}

fn check_ancillary(chunk:&PngAncillaryChunk,after_raster:bool)->Result<(),String> {
    if !chunk.kind.iter().all(u8::is_ascii_alphabetic)||!chunk.kind[0].is_ascii_lowercase()||!chunk.kind[2].is_ascii_uppercase()||[*b"tRNS",*b"gAMA",*b"cHRM",*b"sRGB",*b"pHYs",*b"tIME",*b"bKGD",*b"tEXt",*b"zTXt",*b"iTXt"].contains(&chunk.kind)||after_raster&&!chunk.after_raster {return Err("png: invalid owned ancillary identity or placement".into());}Ok(())
}
