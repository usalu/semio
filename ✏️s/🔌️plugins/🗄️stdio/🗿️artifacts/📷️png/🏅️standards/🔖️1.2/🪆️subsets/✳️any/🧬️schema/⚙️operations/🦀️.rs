//! ⚙️ Shared application and inversion of PngMutation.
use crate::schema::{diff::PngDiff, mutations::PngMutation};
use crate::PngSnapshot;



//#endregion Operations

use crate::schema::snapshot::{PngImage,PngColorType,PngTransparency,PngNativeProfile,PngNativePaint,PngRegion};

pub const MAXIMUM_NATIVE_PAINT_OWNED_BYTES: usize = 512 * 1024 * 1024;

    struct PngRevisionHash(u64);
    impl PngRevisionHash { fn bytes(&mut self,bytes:&[u8]) { for byte in bytes { self.0 ^= u64::from(*byte); self.0=self.0.wrapping_mul(0x100000001b3); } } fn number(&mut self,n:u64) { self.bytes(&n.to_le_bytes()); } fn text(&mut self,s:&str) { self.number(s.len() as u64);self.bytes(s.as_bytes()); } }

pub fn png_revision(snapshot: &PngSnapshot) -> String {
    let mut hash=PngRevisionHash(0xcbf29ce484222325);let image=&snapshot.image;
    hash.text(&snapshot.schema);for n in [u64::from(image.width),u64::from(image.height),u64::from(image.bit_depth),u64::from(image.color_type.to_u8()),u64::from(image.interlace)] { hash.number(n); }
    hash.number(image.samples.len() as u64);for sample in &image.samples { hash.number(u64::from(*sample)); }
    hash_image_metadata(&mut hash,image);
    hash.number(image.text_chunks.len() as u64);for text in &image.text_chunks {hash.text(&text.keyword);hash.text(&text.value);hash.number(u64::from(text.compressed));hash.number(match text.kind {crate::schema::snapshot::PngTextKind::Text=>0,crate::schema::snapshot::PngTextKind::ZText=>1,crate::schema::snapshot::PngTextKind::IText=>2});hash.text(&text.language_tag);hash.text(&text.translated_keyword);}
    hash.number(image.ancillary_chunks.len() as u64);for chunk in &image.ancillary_chunks {hash.bytes(&chunk.kind);hash.number(chunk.data.len() as u64);hash.bytes(&chunk.data);hash.number(u64::from(chunk.after_raster));}
    format!("{:016x}",hash.0)
}

fn hash_image_metadata(hash:&mut PngRevisionHash,image:&PngImage) {
    hash.number(u64::from(image.palette.is_some()));if let Some(palette)=&image.palette { hash.number(palette.len() as u64); for entry in palette { hash.bytes(&[entry.r,entry.g,entry.b]); } }
    hash.number(u64::from(image.transparency.is_some()));if let Some(transparency)=&image.transparency { match transparency { PngTransparency::Indexed { alpha } => { hash.number(3);hash.number(alpha.len() as u64);hash.bytes(alpha); },PngTransparency::Grayscale { gray }=>{hash.number(0);hash.number(u64::from(*gray));},PngTransparency::Rgb { r,g,b }=>{hash.number(2);for channel in [r,g,b] { hash.number(u64::from(*channel)); }} } }
    hash.number(u64::from(image.gamma.is_some()));if let Some(gamma)=image.gamma { hash.number(u64::from(gamma)); }
    hash.number(u64::from(image.chromaticities.is_some()));if let Some(c)=image.chromaticities { for n in [c.white_x,c.white_y,c.red_x,c.red_y,c.green_x,c.green_y,c.blue_x,c.blue_y] {hash.number(u64::from(n));} }
    hash.number(u64::from(image.srgb.is_some()));if let Some(intent)=image.srgb {hash.number(u64::from(intent.to_u8()));}
    hash.number(u64::from(image.physical_dims.is_some()));if let Some(p)=image.physical_dims { for n in [u64::from(p.ppu_x),u64::from(p.ppu_y),u64::from(p.unit_is_meter)] {hash.number(n);} }
    hash.number(u64::from(image.timestamp.is_some()));if let Some(t)=image.timestamp { for n in [u64::from(t.year),u64::from(t.month),u64::from(t.day),u64::from(t.hour),u64::from(t.minute),u64::from(t.second)] {hash.number(n);} }
    hash.number(u64::from(image.background.is_some()));if let Some(background)=&image.background { use crate::schema::snapshot::PngBackground;match background { PngBackground::Indexed {index}=>{hash.number(3);hash.number(u64::from(*index));},PngBackground::Grayscale {gray}=>{hash.number(0);hash.number(u64::from(*gray));},PngBackground::Rgb {r,g,b}=>{hash.number(2);for n in [r,g,b] {hash.number(u64::from(*n));}} } }
}

pub fn require_revision(snapshot: &PngSnapshot, revision: &str) -> Result<(), String> { if png_revision(snapshot) != revision { return Err("png: stale source revision".into()); } Ok(()) }

pub fn png_native_profile(image: &PngImage) -> PngNativeProfile { match image.color_type { PngColorType::Palette=>PngNativeProfile::Indexed,PngColorType::Grayscale=>PngNativeProfile::Grayscale,PngColorType::GrayscaleAlpha=>PngNativeProfile::GrayscaleAlpha,PngColorType::Rgb=>PngNativeProfile::Rgb,PngColorType::Rgba=>PngNativeProfile::Rgba } }

pub fn validate_native_paint(snapshot: &PngSnapshot, region: PngRegion, paint: PngNativePaint) -> Result<usize, String> {
    snapshot.validate()?;validate_native_paint_target(snapshot,region,paint)
}

pub fn validate_native_paint_target(snapshot:&PngSnapshot,region:PngRegion,paint:PngNativePaint)->Result<usize,String> {
    if snapshot.schema!=crate::STDIO_PNG_DOCUMENT_SCHEMA {return Err("png: undeclared semantic schema".into());}snapshot.image.validate_header()?;let image=&snapshot.image;
    if region.width==0 || region.height==0 || region.x.checked_add(region.width).is_none_or(|end|end>image.width) || region.y.checked_add(region.height).is_none_or(|end|end>image.height) { return Err("png: region exceeds the owned image or is empty".into()); }
    if paint.profile != png_native_profile(image) {return Err("png: native paint profile differs from the image".into());}
    let spp=image.color_type.samples_per_pixel();let maximum=if image.bit_depth==16 {u16::MAX} else {(1u16<<image.bit_depth)-1};let samples=paint.samples();
    if image.color_type==PngColorType::Palette && usize::from(paint.first)>=image.palette.as_ref().map_or(0,Vec::len) {return Err("png: native paint palette index is absent".into());}
    if samples[..spp].iter().any(|sample|*sample>maximum) || samples[spp..].iter().any(|sample|*sample!=0) {return Err("png: native paint exceeds sample precision".into());}
    Ok(region.height as usize)
}

pub fn png_native_pixel(snapshot: &PngSnapshot,x:u32,y:u32) -> Result<Vec<u16>,String> { snapshot.validate()?;let image=&snapshot.image;if x>=image.width || y>=image.height {return Err("png: pixel exceeds owned image".into());}let spp=image.color_type.samples_per_pixel();let start=(y as usize*image.width as usize+x as usize)*spp;Ok(image.samples[start..start+spp].to_vec()) }

pub fn paint_native_region_controlled(snapshot:&PngSnapshot,revision:&str,region:PngRegion,paint:PngNativePaint,mut progress:impl FnMut(usize,usize)->bool)->Result<PngSnapshot,String> {
    require_revision(snapshot,revision)?;let rows=validate_native_paint(snapshot,region,paint)?;
    if !progress(0,rows) {return Err("png: native paint cancelled".into());}
    let mut next=snapshot.clone();let spp=next.image.color_type.samples_per_pixel();let samples=paint.samples();
    for row in 0..region.height { for x in region.x..region.x+region.width {let start=((region.y+row)as usize*next.image.width as usize+x as usize)*spp;next.image.samples[start..start+spp].copy_from_slice(&samples[..spp]);}if !progress(row as usize+1,rows) {return Err("png: native paint cancelled".into());} }
    Ok(next)
}

pub fn paint_rgba8_region_controlled(snapshot:&PngSnapshot,revision:&str,region:PngRegion,color:[u8;4],progress:impl FnMut(usize,usize)->bool)->Result<PngSnapshot,String> {
    if snapshot.image.color_type!=PngColorType::Rgba || snapshot.image.bit_depth!=8 {return Err("png: pixel patches need an 8-bit RGBA image".into());}
    paint_native_region_controlled(snapshot,revision,region,PngNativePaint::rgba(u16::from(color[0]),u16::from(color[1]),u16::from(color[2]),u16::from(color[3])),progress)
}

/// 🎨 The sparse rectangle a native paint writes — `None` when the region already holds the paint — computed from the payload and the base samples in that region only.
pub fn paint_native_rect(snapshot: &PngSnapshot, revision: &str, region: PngRegion, paint: PngNativePaint) -> Result<Option<crate::schema::snapshot::PngSampleRect>, String> {
    require_revision(snapshot, revision)?;
    validate_native_paint(snapshot, region, paint)?;
    Ok(native_rect(snapshot, region, paint))
}

/// 🎨 The rectangle holding `paint` over `region`, `None` when the base already holds exactly those samples (the region must have passed [`validate_native_paint`]).
pub fn native_rect(snapshot: &PngSnapshot, region: PngRegion, paint: PngNativePaint) -> Option<crate::schema::snapshot::PngSampleRect> {
    let spp = snapshot.image.color_type.samples_per_pixel();
    let samples: Vec<u16> = paint.samples()[..spp].iter().copied().cycle().take(region.width as usize * region.height as usize * spp).collect();
    (snapshot.image.region_samples(region).as_deref() != Some(samples.as_slice())).then_some(crate::schema::snapshot::PngSampleRect { region, samples })
}

/// 🩹 [`paint_native_rect`] for the RGBA8 profile patch.
pub fn paint_rgba8_rect(snapshot: &PngSnapshot, revision: &str, region: PngRegion, color: [u8; 4]) -> Result<Option<crate::schema::snapshot::PngSampleRect>, String> {
    if snapshot.image.color_type != PngColorType::Rgba || snapshot.image.bit_depth != 8 {
        return Err("png: pixel patches need an 8-bit RGBA image".into());
    }
    paint_native_rect(snapshot, revision, region, PngNativePaint::rgba(u16::from(color[0]), u16::from(color[1]), u16::from(color[2]), u16::from(color[3])))
}

/// 🌗 Checks a guarded gamma change against the base — stale revision, zero gamma — and answers the gamma value the diff sets, `None` when the chunk already holds it.
pub fn gamma_change(snapshot: &PngSnapshot, revision: &str, gama: Option<u32>) -> Result<Option<crate::schema::snapshot::PngGammaValue>, String> {
    snapshot.validate()?;
    require_revision(snapshot, revision)?;
    if gama == Some(0) {
        return Err("png: gamma must be positive".into());
    }
    Ok((snapshot.image.gamma != gama).then_some(crate::schema::snapshot::PngGammaValue { gama }))
}

pub fn set_gamma_chunk_controlled(snapshot:&PngSnapshot,revision:&str,gamma:Option<u32>,mut progress:impl FnMut(usize,usize)->bool)->Result<PngSnapshot,String> {snapshot.validate()?;require_revision(snapshot,revision)?;if gamma==Some(0) {return Err("png: gamma must be positive".into());}if !progress(0,1) {return Err("png: gamma change cancelled".into());}let mut next=snapshot.clone();next.image.gamma=gamma;if !progress(1,1) {return Err("png: gamma change cancelled".into());}Ok(next)}

pub fn validate_completed_native_paint(base:&PngSnapshot,result:&PngSnapshot,region:PngRegion,paint:PngNativePaint)->Result<(),String> {
    validate_native_paint(base,region,paint)?;result.validate()?;
    if !base.image.same_metadata(&result.image) || base.schema!=result.schema {return Err("png: completed paint changed unaddressed metadata".into());}
    let spp=base.image.color_type.samples_per_pixel();let paint=paint.samples();
    for y in 0..base.image.height { for x in 0..base.image.width {let start=(y as usize*base.image.width as usize+x as usize)*spp;let expected=if x>=region.x&&x<region.x+region.width&&y>=region.y&&y<region.y+region.height {&paint[..spp]} else {&base.image.samples[start..start+spp]};if &result.image.samples[start..start+spp]!=expected {return Err("png: completed paint differs from the addressed sample operation".into());}}}Ok(())
}

pub fn png_rgba8_preview(image:&PngImage)->Result<Vec<u8>,String> {
    image.validate()?;let length=(image.width as usize).checked_mul(image.height as usize).and_then(|n|n.checked_mul(4)).ok_or("png: preview extent overflow")?;if length>64*1024*1024 {return Err("png: preview exceeds 64 MiB".into());}
    let maximum=if image.bit_depth==16 {65535} else {(1u32<<image.bit_depth)-1};let scale=|sample:u16|((u32::from(sample)*255+maximum/2)/maximum)as u8;let spp=image.color_type.samples_per_pixel();let mut output=Vec::with_capacity(length);
    for samples in image.samples.chunks_exact(spp) { let pixel=match image.color_type {
        PngColorType::Palette=>{let entry=&image.palette.as_ref().unwrap()[usize::from(samples[0])];let alpha=match &image.transparency {Some(PngTransparency::Indexed {alpha})=>alpha.get(usize::from(samples[0])).copied().unwrap_or(255),_=>255};[entry.r,entry.g,entry.b,alpha]},
        PngColorType::Grayscale=>{let alpha=if matches!(&image.transparency,Some(PngTransparency::Grayscale {gray})if *gray==samples[0]) {0}else{255};[scale(samples[0]),scale(samples[0]),scale(samples[0]),alpha]},
        PngColorType::Rgb=>{let alpha=if matches!(&image.transparency,Some(PngTransparency::Rgb {r,g,b})if [*r,*g,*b]==samples) {0}else{255};[scale(samples[0]),scale(samples[1]),scale(samples[2]),alpha]},
        PngColorType::GrayscaleAlpha=>[scale(samples[0]),scale(samples[0]),scale(samples[0]),scale(samples[1])],
        PngColorType::Rgba=>[scale(samples[0]),scale(samples[1]),scale(samples[2]),scale(samples[3])],
    };output.extend_from_slice(&pixel); }Ok(output)
}

fn retire_png_vec_step<T>(values: &mut Vec<T>, maximum_items: usize, maximum_bytes: usize) -> Option<(usize, usize)> {
    let item_bytes = std::mem::size_of::<T>();
    if !values.is_empty() {
        let byte_items = if item_bytes == 0 { maximum_items } else { maximum_bytes / item_bytes };
        let released_items = values.len().min(maximum_items).min(byte_items);
        if released_items == 0 { return Some((0, 0)); }
        values.truncate(values.len() - released_items);
        return Some((released_items, released_items * item_bytes));
    }
    if values.capacity() == 0 { return None; }
    let backing = values.capacity().checked_mul(item_bytes).unwrap_or(usize::MAX);
    if maximum_items == 0 || maximum_bytes < backing { return Some((0, 0)); }
    drop(std::mem::take(values));
    Some((1, backing))
}

fn retire_png_string_step(value: &mut String, maximum_items: usize, maximum_bytes: usize) -> Option<(usize, usize)> {
    if !value.is_empty() {
        let bytes = value.chars().next_back().map_or(0, char::len_utf8);
        if maximum_items == 0 || maximum_bytes < bytes { return Some((0, 0)); }
        value.pop();
        return Some((1, bytes));
    }
    if value.capacity() == 0 { return None; }
    let backing = value.capacity();
    if maximum_items == 0 || maximum_bytes < backing { return Some((0, 0)); }
    drop(std::mem::take(value));
    Some((1, backing))
}


#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PngNativePaintPhase { Copy, Paint }
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub struct PngNativePaintProgress {pub phase:PngNativePaintPhase,pub completed:usize,pub total:usize,pub owned_bytes:usize}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]
pub enum PngNativePaintWorkStep {Yield(PngNativePaintProgress),Complete,Cancelled}

enum PngPaintSource<'a> { Borrowed(&'a PngSnapshot), Retained(std::sync::Arc<PngSnapshot>) }
impl PngPaintSource<'_> { fn snapshot(&self)->&PngSnapshot {match self {Self::Borrowed(source)=>source,Self::Retained(source)=>source}} }

pub struct PngNativePaintWorkOperation<'a> {
    source:Option<PngPaintSource<'a>>,revision:String,region:PngRegion,paint:PngNativePaint,result:Option<PngSnapshot>,stage:u8,cursor:usize,field:usize,byte:usize,paint_cursor:usize,owned_bytes:usize,maximum_owned_bytes:usize,hash:PngRevisionHash,expected_revision:bool,keyword_characters:usize,after_raster:bool,closing:bool,
}

impl<'a> PngNativePaintWorkOperation<'a> {
    pub fn try_new(snapshot:&'a PngSnapshot,revision:&str,region:PngRegion,paint:PngNativePaint,maximum_owned_bytes:usize)->Result<Self,String> { Self::prepare(PngPaintSource::Borrowed(snapshot),revision,region,paint,maximum_owned_bytes) }
    pub fn try_new_retained(snapshot:std::sync::Arc<PngSnapshot>,region:PngRegion,paint:PngNativePaint,maximum_owned_bytes:usize)->Result<Self,String> { Self::prepare(PngPaintSource::Retained(snapshot),"",region,paint,maximum_owned_bytes) }
    fn prepare(reader:PngPaintSource<'a>,revision:&str,region:PngRegion,paint:PngNativePaint,maximum_owned_bytes:usize)->Result<Self,String> {
        let snapshot=reader.snapshot();if !revision.is_empty()&&(revision.len()!=16||!revision.bytes().all(|byte|byte.is_ascii_hexdigit())) {return Err("png: stale source revision has an invalid exact identity".into());}validate_native_paint_target(snapshot,region,paint)?;let source=&snapshot.image;
        let total=source.samples.len().checked_mul(2).and_then(|n|n.checked_add(source.text_chunks.len()*std::mem::size_of::<crate::schema::snapshot::PngTextChunk>())).and_then(|n|n.checked_add(source.ancillary_chunks.len()*std::mem::size_of::<crate::schema::snapshot::PngAncillaryChunk>())).and_then(|n|n.checked_add(4096)).ok_or("png: retained sample ownership overflow")?;
        if total>maximum_owned_bytes {return Err("png: retained sample paint exceeds caller ownership limit".into());}
        let mut samples=Vec::new();samples.try_reserve_exact(source.samples.len()).map_err(|_|"png: retained sample allocation failed")?;
        let mut text_chunks=Vec::new();text_chunks.try_reserve_exact(source.text_chunks.len()).map_err(|_|"png: retained text row allocation failed")?;let mut ancillary_chunks=Vec::new();ancillary_chunks.try_reserve_exact(source.ancillary_chunks.len()).map_err(|_|"png: retained ancillary row allocation failed")?;
        let image=PngImage {width:source.width,height:source.height,bit_depth:source.bit_depth,color_type:source.color_type,interlace:source.interlace,samples,palette:source.palette.clone(),transparency:source.transparency.clone(),gamma:source.gamma,chromaticities:source.chromaticities,srgb:source.srgb,physical_dims:source.physical_dims,timestamp:source.timestamp,background:source.background.clone(),text_chunks,ancillary_chunks};
        let schema=snapshot.schema.clone();let revision=revision.to_owned();let owned_bytes=image.samples.capacity()*2+image.text_chunks.capacity()*std::mem::size_of::<crate::schema::snapshot::PngTextChunk>()+image.ancillary_chunks.capacity()*std::mem::size_of::<crate::schema::snapshot::PngAncillaryChunk>()+schema.capacity()+revision.capacity()+image.palette.as_ref().map_or(0,|palette|palette.capacity()*std::mem::size_of::<crate::schema::snapshot::PngRgb>())+match &image.transparency {Some(PngTransparency::Indexed {alpha})=>alpha.capacity(),_=>0};
        let expected_revision=!revision.is_empty();let mut hash=PngRevisionHash(0xcbf29ce484222325);hash.text(&schema);for n in [u64::from(image.width),u64::from(image.height),u64::from(image.bit_depth),u64::from(image.color_type.to_u8()),u64::from(image.interlace)] {hash.number(n);}hash.number(source.samples.len()as u64);
        Ok(Self {source:Some(reader),revision,region,paint,result:Some(PngSnapshot {schema,image}),stage:0,cursor:0,field:0,byte:0,paint_cursor:0,owned_bytes,maximum_owned_bytes,hash,expected_revision,keyword_characters:0,after_raster:false,closing:false})
    }
    pub fn advance(&mut self,context:&mut semio_framework_job::StepContext<'_>)->Result<PngNativePaintWorkStep,String> {
        if self.closing {return Err("png: sample paint work is closing".into());}
        let source=&self.source.as_ref().ok_or("png: immutable source reader was released")?.snapshot().image;let result=self.result.as_mut().ok_or("png: sample result was transferred")?;let target=&mut result.image;
        loop {
            if context.is_cancelled() {return Ok(PngNativePaintWorkStep::Cancelled);}
            if context.should_yield() {return Ok(PngNativePaintWorkStep::Yield(PngNativePaintProgress {phase:if self.stage<3 {PngNativePaintPhase::Copy}else{PngNativePaintPhase::Paint},completed:if self.stage<3 {target.samples.len()}else{self.paint_cursor},total:if self.stage<3 {source.samples.len()}else{self.region.width as usize*self.region.height as usize},owned_bytes:self.owned_bytes}));}
            match self.stage {
                0=>{if self.cursor==source.samples.len() {hash_image_metadata(&mut self.hash,source);self.hash.number(source.text_chunks.len()as u64);self.stage=1;self.cursor=0;continue;}let sample=source.samples[self.cursor];let maximum=if source.bit_depth==16 {u16::MAX}else{(1u16<<source.bit_depth)-1};if sample>maximum||source.color_type==PngColorType::Palette&&usize::from(sample)>=source.palette.as_ref().map_or(0,Vec::len) {return Err("png: retained sample differs from its owned native profile".into());}target.samples.push(sample);self.hash.number(u64::from(sample));self.cursor+=1;},
                1=>{
                    if self.cursor==source.text_chunks.len() {self.hash.number(source.ancillary_chunks.len()as u64);self.stage=2;self.cursor=0;self.byte=0;continue;}
                    let text=&source.text_chunks[self.cursor];
                    if target.text_chunks.len()==self.cursor {if text.keyword.is_empty()||text.kind==crate::schema::snapshot::PngTextKind::Text&&text.compressed||text.kind==crate::schema::snapshot::PngTextKind::ZText&&!text.compressed||text.kind!=crate::schema::snapshot::PngTextKind::IText&&(!text.language_tag.is_empty()||!text.translated_keyword.is_empty()) {return Err("png: retained text profile is invalid".into());}self.keyword_characters=0;target.text_chunks.push(crate::schema::snapshot::PngTextChunk {compressed:text.compressed,kind:text.kind,..Default::default()});}
                    let output=&mut target.text_chunks[self.cursor];let (input,output)=match self.field {0=>(&text.keyword,&mut output.keyword),1=>(&text.value,&mut output.value),2=>(&text.language_tag,&mut output.language_tag),_=>(&text.translated_keyword,&mut output.translated_keyword)};
                    if self.byte==0 {self.hash.number(input.len()as u64);}
                    if self.byte==input.len() {if self.field==1 {self.hash.number(u64::from(text.compressed));self.hash.number(match text.kind {crate::schema::snapshot::PngTextKind::Text=>0,crate::schema::snapshot::PngTextKind::ZText=>1,crate::schema::snapshot::PngTextKind::IText=>2});}self.byte=0;self.field+=1;if self.field==4 {self.field=0;self.cursor+=1;}context.consume_fuel(1);continue;}
                    let character=input[self.byte..].chars().next().ok_or("png: retained text cursor invalid")?;if self.field==0 {self.keyword_characters+=1;if self.keyword_characters>79||character=='\0'||character as u32>255 {return Err("png: retained text keyword is invalid".into());}}if self.field==1&&text.kind!=crate::schema::snapshot::PngTextKind::IText&&character as u32>255||self.field==2&&!character.is_ascii() {return Err("png: retained text differs from its native character profile".into());}if output.capacity()==0 {if self.owned_bytes.checked_add(input.len()).is_none_or(|n|n>self.maximum_owned_bytes) {return Err("png: retained text exceeds caller ownership limit".into());}output.try_reserve_exact(input.len()).map_err(|_|"png: retained text allocation failed")?;self.owned_bytes=self.owned_bytes.checked_add(output.capacity()).filter(|n|*n<=self.maximum_owned_bytes).ok_or("png: retained text exceeds caller ownership limit")?;}output.push(character);self.hash.bytes(&input.as_bytes()[self.byte..self.byte+character.len_utf8()]);self.byte+=character.len_utf8();
                },
                2=>{
                    if self.cursor==source.ancillary_chunks.len() {let observed=format!("{:016x}",self.hash.0);if self.expected_revision&&observed!=self.revision {return Err("png: stale source revision".into());}self.owned_bytes=self.owned_bytes.saturating_sub(self.revision.capacity()).checked_add(observed.capacity()).filter(|n|*n<=self.maximum_owned_bytes).ok_or("png: retained revision exceeds caller ownership limit")?;self.revision=observed;self.stage=3;self.cursor=0;continue;}
                    let chunk=&source.ancillary_chunks[self.cursor];if target.ancillary_chunks.len()==self.cursor {if !chunk.kind.iter().all(u8::is_ascii_alphabetic)||!chunk.kind[0].is_ascii_lowercase()||!chunk.kind[2].is_ascii_uppercase()||[*b"tRNS",*b"gAMA",*b"cHRM",*b"sRGB",*b"pHYs",*b"tIME",*b"bKGD",*b"tEXt",*b"zTXt",*b"iTXt"].contains(&chunk.kind)||self.after_raster&&!chunk.after_raster {return Err("png: retained ancillary placement or identity is invalid".into());}self.after_raster|=chunk.after_raster;self.hash.bytes(&chunk.kind);self.hash.number(chunk.data.len()as u64);if self.owned_bytes.checked_add(chunk.data.len()).is_none_or(|n|n>self.maximum_owned_bytes) {return Err("png: retained ancillary data exceeds caller ownership limit".into());}let mut data=Vec::new();data.try_reserve_exact(chunk.data.len()).map_err(|_|"png: retained ancillary data allocation failed")?;self.owned_bytes=self.owned_bytes.checked_add(data.capacity()).filter(|n|*n<=self.maximum_owned_bytes).ok_or("png: retained ancillary data exceeds caller ownership limit")?;target.ancillary_chunks.push(crate::schema::snapshot::PngAncillaryChunk {kind:chunk.kind,data,after_raster:chunk.after_raster});}
                    if self.byte==chunk.data.len() {self.hash.number(u64::from(chunk.after_raster));self.byte=0;self.cursor+=1;context.consume_fuel(1);continue;}self.hash.bytes(&chunk.data[self.byte..self.byte+1]);target.ancillary_chunks[self.cursor].data.push(chunk.data[self.byte]);self.byte+=1;
                },
                3=>{let total=self.region.width as usize*self.region.height as usize;if self.paint_cursor==total {self.stage=4;continue;}let x=self.region.x+self.paint_cursor as u32%self.region.width;let y=self.region.y+(self.paint_cursor/self.region.width as usize)as u32;let spp=source.color_type.samples_per_pixel();let start=(y as usize*source.width as usize+x as usize)*spp;target.samples[start..start+spp].copy_from_slice(&self.paint.samples()[..spp]);self.paint_cursor+=1;},
                _=>return Ok(PngNativePaintWorkStep::Complete),
            }
            context.consume_fuel(1);
        }
    }
    pub fn retained_reader_matches(&self,reader:&std::sync::Arc<PngSnapshot>)->bool {matches!(&self.source,Some(PngPaintSource::Retained(source)) if std::sync::Arc::ptr_eq(source,reader))}
    pub fn source_revision(&self)->&str {&self.revision}
    pub fn take_result(&mut self)->Result<PngSnapshot,String> {if self.stage!=4 || self.closing {return Err("png: retained paint is incomplete".into());}self.result.take().ok_or_else(||"png: paint result was already transferred".into())}
    pub fn begin_close(&mut self) {self.closing=true;self.source=None;}
    pub fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep as Step;
        if !self.closing {return Step::Blocked;}
        let pending=|step:(usize,usize)|Step::Pending {released_items:step.0,released_bytes:step.1};
        if let Some(result)=&mut self.result {
            let image=&mut result.image;
            if let Some(step)=retire_png_vec_step(&mut image.samples,maximum_items,maximum_bytes) {return pending(step);}
            if let Some(text)=image.text_chunks.last_mut() {for field in [&mut text.keyword,&mut text.value,&mut text.language_tag,&mut text.translated_keyword] {if let Some(step)=retire_png_string_step(field,maximum_items,maximum_bytes) {return pending(step);}}if maximum_items==0||maximum_bytes<std::mem::size_of::<crate::schema::snapshot::PngTextChunk>() {return pending((0,0));}image.text_chunks.pop();return pending((1,std::mem::size_of::<crate::schema::snapshot::PngTextChunk>()));}
            if let Some(step)=retire_png_vec_step(&mut image.text_chunks,maximum_items,maximum_bytes) {return pending(step);}
            if let Some(chunk)=image.ancillary_chunks.last_mut() {if let Some(step)=retire_png_vec_step(&mut chunk.data,maximum_items,maximum_bytes) {return pending(step);}if maximum_items==0||maximum_bytes<std::mem::size_of::<crate::schema::snapshot::PngAncillaryChunk>() {return pending((0,0));}image.ancillary_chunks.pop();return pending((1,std::mem::size_of::<crate::schema::snapshot::PngAncillaryChunk>()));}
            if let Some(step)=retire_png_vec_step(&mut image.ancillary_chunks,maximum_items,maximum_bytes) {return pending(step);}
            if let Some(palette)=&mut image.palette {if let Some(step)=retire_png_vec_step(palette,maximum_items,maximum_bytes) {return pending(step);}image.palette=None;}
            if let Some(PngTransparency::Indexed {alpha})=&mut image.transparency {if let Some(step)=retire_png_vec_step(alpha,maximum_items,maximum_bytes) {return pending(step);}}image.transparency=None;
            if let Some(step)=retire_png_string_step(&mut result.schema,maximum_items,maximum_bytes) {return pending(step);}
            if maximum_items==0||maximum_bytes<std::mem::size_of::<PngSnapshot>() {return pending((0,0));}self.result=None;return pending((1,std::mem::size_of::<PngSnapshot>()));
        }
        if let Some(step)=retire_png_string_step(&mut self.revision,maximum_items,maximum_bytes) {return pending(step);}Step::Complete
    }
    pub fn terminal_is_empty(&self)->bool {self.closing&&self.result.is_none()&&self.revision.capacity()==0}
}

pub fn paint_native_region_owned_controlled(snapshot:&PngSnapshot,revision:&str,region:PngRegion,paint:PngNativePaint,maximum_owned_bytes:usize,progress:&mut dyn FnMut(PngNativePaintProgress)->bool)->Result<PngSnapshot,String> {
    let mut operation=PngNativePaintWorkOperation::try_new(snapshot,revision,region,paint,maximum_owned_bytes)?;
    let mut sequence=0;let id=semio_framework_job::allocate_operation_id();let cancel=semio_framework_job::root_cancel_token();
    let result=if !progress(PngNativePaintProgress {phase:PngNativePaintPhase::Copy,completed:0,total:snapshot.image.samples.len(),owned_bytes:0}) {Err("png: native paint cancelled".into())} else {loop {
        let mut context=semio_framework_job::StepContext::new(id,semio_framework_job::Generation(1),semio_framework_job::StepBudget::new(512,u64::MAX),cancel.clone(),||Some(0),&mut sequence);
        match operation.advance(&mut context) {
            Ok(PngNativePaintWorkStep::Yield(checkpoint))=>{if !progress(checkpoint) {break Err("png: native paint cancelled".into());}},
            Ok(PngNativePaintWorkStep::Cancelled)=>break Err("png: native paint cancelled".into()),
            Ok(PngNativePaintWorkStep::Complete)=>{if !progress(PngNativePaintProgress {phase:PngNativePaintPhase::Paint,completed:region.width as usize*region.height as usize,total:region.width as usize*region.height as usize,owned_bytes:0}) {break Err("png: native paint cancelled".into());}break operation.take_result();}
            Err(error)=>break Err(error),
        }
    }};
    operation.begin_close();while !operation.terminal_is_empty() {operation.close_step(512,maximum_owned_bytes);}
    result
}

#[path="🪪️validation/🦀️.rs"]
pub mod owned_validation;
