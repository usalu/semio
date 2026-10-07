//! ⚙️ Shared application and inversion of BmpMutation.
use crate::schema::{diff::BmpDiff, mutations::BmpMutation};
use crate::BmpSnapshot;

//#region Operations
pub fn apply_bmp_mutation(snapshot: &mut BmpSnapshot, mutation: &BmpMutation) -> protocol::MutationOutcome<BmpDiff> {
    let outcome = <BmpMutation as protocol::Mutation<BmpSnapshot>>::diff(mutation, snapshot);
    match protocol::MutationDiff::apply(outcome.diff(), snapshot) {
        Ok(next) => {
            *snapshot = next;
            outcome
        }
        Err(error) => protocol::MutationOutcome::fatal(error.code, error.message, error.target).absorb_messages(outcome.messages().to_vec()),
    }
}

//#endregion Operations


use crate::schema::snapshot::{mask_maximum, BmpColor, BmpImage, BmpPixels, BmpRegion};

fn feed(hash: &mut u64, bytes: &[u8]) { for byte in bytes { *hash ^= u64::from(*byte); *hash = hash.wrapping_mul(0x100000001b3); } }
fn feed_counted(hash: &mut u64, bytes: &[u8]) { feed(hash, &(bytes.len() as u64).to_le_bytes()); feed(hash, bytes); }

pub fn bmp_revision(snapshot: &BmpSnapshot) -> String {
    let mut hash = 0xcbf29ce484222325;
    feed_counted(&mut hash, snapshot.schema.as_bytes());
    let image = &snapshot.image;
    feed_counted(&mut hash, image.profile.id().as_bytes());
    feed(&mut hash, &[u8::from(image.row_order == crate::schema::snapshot::BmpRowOrder::TopDown)]);
    for scalar in [image.width, image.height, image.colors_used, image.colors_important, image.reserved_1.into(), image.reserved_2.into()] { feed(&mut hash, &scalar.to_le_bytes()); }
    feed(&mut hash, &image.x_pixels_per_meter.to_le_bytes()); feed(&mut hash, &image.y_pixels_per_meter.to_le_bytes());
    for mask in image.masks { feed(&mut hash, &mask.to_le_bytes()); }
    feed(&mut hash, &(image.palette.len() as u64).to_le_bytes());
    for entry in &image.palette { feed(&mut hash, &[entry.r, entry.g, entry.b, entry.reserved]); }
    match &image.pixels {
        BmpPixels::Indexed { indices } => { feed(&mut hash, &[0]); feed_counted(&mut hash, indices); }
        BmpPixels::Direct { samples } => { feed(&mut hash, &[1]); feed(&mut hash, &(samples.len() as u64).to_le_bytes()); for sample in samples { for scalar in [sample.red, sample.green, sample.blue, sample.alpha, sample.reserved] { feed(&mut hash, &scalar.to_le_bytes()); } } }
    }
    feed_counted(&mut hash, &image.opaque_gap); feed_counted(&mut hash, &image.opaque_trailer);
    format!("{hash:016x}")
}

pub fn checked_region(image: &BmpImage, region: BmpRegion) -> Result<(), String> {
    if region.x.checked_add(region.width).is_none_or(|right| right > image.width) || region.y.checked_add(region.height).is_none_or(|bottom| bottom > image.height) { return Err("bmp: paint region exceeds owned dimensions".into()); }
    Ok(())
}

fn require_revision(snapshot: &BmpSnapshot, revision: &str) -> Result<(), String> { let expected = bmp_revision(snapshot); if revision != expected { return Err(format!("bmp: stale owned revision {revision}; expected {expected}")); } Ok(()) }
fn native_color(value: u8, mask: u32) -> u32 { ((u64::from(value) * u64::from(mask_maximum(mask)) + 127) / 255) as u32 }
fn display_color(value: u32, mask: u32) -> u8 { let maximum = mask_maximum(mask); if maximum == 0 { 0 } else { ((u64::from(value) * 255 + u64::from(maximum) / 2) / u64::from(maximum)) as u8 } }

pub fn bmp_palette(snapshot: &BmpSnapshot) -> Result<Vec<crate::schema::snapshot::BmpPaletteEntry>, String> { snapshot.validate()?; Ok(snapshot.image.palette.clone()) }

pub fn bmp_rgba8_preview(snapshot: &BmpSnapshot) -> Result<Vec<u8>, String> {
    snapshot.validate()?;
    let image = &snapshot.image;
    let count = (image.width as usize).checked_mul(image.height as usize).and_then(|count| count.checked_mul(4)).ok_or("bmp: preview extent overflow")?;
    if count > 64 * 1024 * 1024 { return Err("bmp: preview exceeds display ownership limit".into()); }
    let mut pixels = Vec::with_capacity(count);
    match &image.pixels {
        BmpPixels::Indexed { indices } => { for index in indices { let entry = image.palette[*index as usize]; pixels.extend_from_slice(&[entry.r, entry.g, entry.b, 255]); } }
        BmpPixels::Direct { samples } => { for sample in samples { pixels.extend_from_slice(&[display_color(sample.red, image.masks[0]), display_color(sample.green, image.masks[1]), display_color(sample.blue, image.masks[2]), if image.masks[3] == 0 { 255 } else { display_color(sample.alpha, image.masks[3]) }]); } }
    }
    Ok(pixels)
}

pub fn paint_indexed_region_controlled(snapshot: &BmpSnapshot, revision: &str, region: BmpRegion, palette_index: u8, progress: &mut dyn FnMut(usize, usize) -> bool) -> Result<BmpSnapshot, String> {
    snapshot.validate()?; require_revision(snapshot, revision)?; checked_region(&snapshot.image, region)?;
    if !snapshot.image.profile.is_indexed() || palette_index as usize >= snapshot.image.palette.len() { return Err("bmp: indexed paint needs an owned palette index".into()); }
    let total = region.height as usize;
    if !progress(0, total) { return Err("bmp: paint cancelled".into()); }
    let mut next = snapshot.clone();
    let BmpPixels::Indexed { indices } = &mut next.image.pixels else { unreachable!() };
    for row in 0..total { let offset = (region.y as usize + row) * next.image.width as usize + region.x as usize; indices[offset..offset + region.width as usize].fill(palette_index); if !progress(row + 1, total) { return Err("bmp: paint cancelled".into()); } }
    Ok(next)
}

pub fn paint_direct_region_controlled(snapshot: &BmpSnapshot, revision: &str, region: BmpRegion, color: BmpColor, progress: &mut dyn FnMut(usize, usize) -> bool) -> Result<BmpSnapshot, String> {
    snapshot.validate()?; require_revision(snapshot, revision)?; checked_region(&snapshot.image, region)?;
    if !snapshot.image.profile.is_direct() { return Err("bmp: direct paint needs direct owned samples".into()); }
    let total = region.height as usize;
    if !progress(0, total) { return Err("bmp: paint cancelled".into()); }
    let mut next = snapshot.clone();
    let masks = next.image.masks;
    let components = [color.red, color.green, color.blue, color.alpha].into_iter().zip(masks).map(|(value, mask)| native_color(value, mask)).collect::<Vec<_>>();
    let BmpPixels::Direct { samples } = &mut next.image.pixels else { unreachable!() };
    for row in 0..total { let offset = (region.y as usize + row) * next.image.width as usize + region.x as usize; for sample in &mut samples[offset..offset + region.width as usize] { sample.red = components[0]; sample.green = components[1]; sample.blue = components[2]; sample.alpha = components[3]; } if !progress(row + 1, total) { return Err("bmp: paint cancelled".into()); } }
    Ok(next)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BmpRetainedPaint { Indexed(u8), Direct(BmpColor) }
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BmpPaintWorkStep { Yield { painting: bool, completed: usize, total: usize, owned_bytes: usize }, Complete, Cancelled }
enum BmpPaintSource<'a> { Borrowed(&'a BmpSnapshot), Retained(std::sync::Arc<BmpSnapshot>) }
impl BmpPaintSource<'_> { fn snapshot(&self)->&BmpSnapshot {match self {Self::Borrowed(source)=>source,Self::Retained(source)=>source}} }
pub struct BmpPaintWorkOperation<'a> { source: Option<BmpPaintSource<'a>>, result: Option<BmpSnapshot>, region: BmpRegion, paint: BmpRetainedPaint, stage: u8, cursor: usize, owned_bytes: usize, closing: bool }

impl<'a> BmpPaintWorkOperation<'a> {
    pub fn try_new(source:&'a BmpSnapshot,region:BmpRegion,paint:BmpRetainedPaint,maximum_owned_bytes:usize)->Result<Self,String> {Self::prepare(BmpPaintSource::Borrowed(source),region,paint,maximum_owned_bytes)}
    pub fn try_new_retained(source:std::sync::Arc<BmpSnapshot>,region:BmpRegion,paint:BmpRetainedPaint,maximum_owned_bytes:usize)->Result<Self,String> {Self::prepare(BmpPaintSource::Retained(source),region,paint,maximum_owned_bytes)}
    fn prepare(reader:BmpPaintSource<'a>,region:BmpRegion,paint:BmpRetainedPaint,maximum_owned_bytes:usize)->Result<Self,String> {
        let source=reader.snapshot();if source.schema!=crate::STDIO_BMP_DOCUMENT_SCHEMA {return Err("bmp: undeclared semantic schema".into());}source.image.validate_header()?;checked_region(&source.image,region)?;
        if region.width==0||region.height==0 {return Err("bmp: retained paint region is empty".into());}
        match paint {BmpRetainedPaint::Indexed(index) if source.image.profile.is_indexed()&&(index as usize)<source.image.palette.len()=>{},BmpRetainedPaint::Direct(_) if source.image.profile.is_direct()=>{},_=>return Err("bmp: retained paint profile differs from the owned image".into())}
        let image=&source.image;
        let pixels=match &image.pixels {BmpPixels::Indexed {indices}=>indices.len(),BmpPixels::Direct {samples}=>samples.len().checked_mul(std::mem::size_of::<crate::schema::snapshot::BmpNativeSample>()).ok_or("bmp: retained pixel extent overflow")?};
        let owned_bytes=pixels.checked_add(image.palette.len()*std::mem::size_of::<crate::schema::snapshot::BmpPaletteEntry>()).and_then(|n|n.checked_add(image.opaque_gap.len())).and_then(|n|n.checked_add(image.opaque_trailer.len())).and_then(|n|n.checked_add(source.schema.len()+std::mem::size_of::<BmpSnapshot>())).ok_or("bmp: retained ownership extent overflow")?;
        if owned_bytes>maximum_owned_bytes {return Err("bmp: retained paint exceeds caller ownership limit".into());}
        fn reserve<T>(length:usize)->Result<Vec<T>,String> {let mut values=Vec::new();values.try_reserve_exact(length).map_err(|_|"bmp: retained allocation failed")?;Ok(values)}
        let pixels=match &image.pixels {BmpPixels::Indexed {indices}=>BmpPixels::Indexed {indices:reserve(indices.len())?},BmpPixels::Direct {samples}=>BmpPixels::Direct {samples:reserve(samples.len())?}};
        let result=BmpSnapshot {schema:source.schema.clone(),image:BmpImage {width:image.width,height:image.height,row_order:image.row_order,profile:image.profile,masks:image.masks,palette:reserve(image.palette.len())?,pixels,x_pixels_per_meter:image.x_pixels_per_meter,y_pixels_per_meter:image.y_pixels_per_meter,colors_used:image.colors_used,colors_important:image.colors_important,reserved_1:image.reserved_1,reserved_2:image.reserved_2,opaque_gap:reserve(image.opaque_gap.len())?,opaque_trailer:reserve(image.opaque_trailer.len())?}};
        Ok(Self {source:Some(reader),result:Some(result),region,paint,stage:0,cursor:0,owned_bytes,closing:false})
    }
    pub fn retained_reader_matches(&self,reader:&std::sync::Arc<BmpSnapshot>)->bool {matches!(&self.source,Some(BmpPaintSource::Retained(source)) if std::sync::Arc::ptr_eq(source,reader))}
    pub fn advance(&mut self,context:&mut semio_framework_job::StepContext<'_>)->Result<BmpPaintWorkStep,String> {
        if self.closing {return Err("bmp: retained paint is closing".into());}
        let source=&self.source.as_ref().ok_or("bmp: immutable paint reader was released")?.snapshot().image;let target=&mut self.result.as_mut().ok_or("bmp: retained result was transferred")?.image;
        loop {
            if context.is_cancelled() {return Ok(BmpPaintWorkStep::Cancelled);}
            let total=match self.stage {0=>source.palette.len(),1=>match &source.pixels {BmpPixels::Indexed {indices}=>indices.len(),BmpPixels::Direct {samples}=>samples.len()},2=>source.opaque_gap.len(),3=>source.opaque_trailer.len(),4=>self.region.width as usize*self.region.height as usize,_=>return Ok(BmpPaintWorkStep::Complete)};
            if self.cursor==total {self.cursor=0;self.stage+=1;continue;}
            if context.should_yield() {return Ok(BmpPaintWorkStep::Yield {painting:self.stage==4,completed:self.cursor,total,owned_bytes:self.owned_bytes});}
            match self.stage {
                0=>target.palette.push(source.palette[self.cursor]),
                1=>{source.validate_sample(self.cursor)?;match (&source.pixels,&mut target.pixels) {(BmpPixels::Indexed {indices:source},BmpPixels::Indexed {indices:target})=>target.push(source[self.cursor]),(BmpPixels::Direct {samples:source},BmpPixels::Direct {samples:target})=>target.push(source[self.cursor]),_=>return Err("bmp: immutable pixel storage changed".into())}},
                2=>target.opaque_gap.push(source.opaque_gap[self.cursor]),
                3=>target.opaque_trailer.push(source.opaque_trailer[self.cursor]),
                4=>{let x=self.region.x as usize+self.cursor%self.region.width as usize;let y=self.region.y as usize+self.cursor/self.region.width as usize;let at=y*source.width as usize+x;match (&mut target.pixels,self.paint) {(BmpPixels::Indexed {indices},BmpRetainedPaint::Indexed(index))=>indices[at]=index,(BmpPixels::Direct {samples},BmpRetainedPaint::Direct(color))=>{let sample=&mut samples[at];sample.red=native_color(color.red,source.masks[0]);sample.green=native_color(color.green,source.masks[1]);sample.blue=native_color(color.blue,source.masks[2]);sample.alpha=native_color(color.alpha,source.masks[3]);},_=>return Err("bmp: retained paint storage differs".into())}},
                _=>unreachable!(),
            }
            self.cursor+=1;context.consume_fuel(1);
        }
    }
    pub fn take_result(&mut self)->Result<BmpSnapshot,String> {if self.stage!=5||self.closing {return Err("bmp: retained paint is incomplete".into());}self.result.take().ok_or_else(||"bmp: retained result was transferred".into())}
    pub fn begin_close(&mut self) {self.closing=true;self.source=None;}
    pub fn close_step(&mut self,maximum_items:usize,maximum_bytes:usize)->semio_framework_job::InteractiveJobCloseStep {
        use semio_framework_job::InteractiveJobCloseStep as Step;
        if !self.closing {return Step::Blocked;}
        fn retire<T>(values:&mut Vec<T>,maximum_items:usize,maximum_bytes:usize)->Option<(usize,usize)> {let size=std::mem::size_of::<T>();if !values.is_empty() {let count=values.len().min(maximum_items).min(if size==0 {maximum_items}else{maximum_bytes/size});values.truncate(values.len()-count);return Some((count,count*size));}if values.capacity()>0 {let bytes=values.capacity()*size;if maximum_items==0||maximum_bytes<bytes {return Some((0,0));}*values=Vec::new();return Some((1,bytes));}None}
        if let Some(result)=&mut self.result {
            let pending=|values:(usize,usize)|Step::Pending {released_items:values.0,released_bytes:values.1};
            if let Some(step)=retire(&mut result.image.palette,maximum_items,maximum_bytes) {return pending(step);}
            let pixels=match &mut result.image.pixels {BmpPixels::Indexed {indices}=>retire(indices,maximum_items,maximum_bytes),BmpPixels::Direct {samples}=>retire(samples,maximum_items,maximum_bytes)};if let Some(step)=pixels {return pending(step);}
            if let Some(step)=retire(&mut result.image.opaque_gap,maximum_items,maximum_bytes) {return pending(step);}
            if let Some(step)=retire(&mut result.image.opaque_trailer,maximum_items,maximum_bytes) {return pending(step);}
            if !result.schema.is_empty() {let count=result.schema.len().min(maximum_items).min(maximum_bytes);result.schema.truncate(result.schema.len()-count);return pending((count,count));}
            if result.schema.capacity()>0 {let bytes=result.schema.capacity();if maximum_items==0||maximum_bytes<bytes {return pending((0,0));}result.schema=String::new();return pending((1,bytes));}
            if maximum_items==0||maximum_bytes<std::mem::size_of::<BmpSnapshot>() {return pending((0,0));}self.result=None;return pending((1,std::mem::size_of::<BmpSnapshot>()));
        }
        Step::Complete
    }
    pub fn terminal_is_empty(&self)->bool {self.closing&&self.result.is_none()&&self.source.is_none()}
}

#[path="🪪️validation/🦀️.rs"]
pub mod owned_validation;
