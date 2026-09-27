//! 🗂️ Retained preparation of centered raster layers for the shared compositor.
use super::*;

#[derive(Clone,Copy,Debug)]
pub struct RasterStackTransform {pub x:f64,pub y:f64,pub scale_x:f64,pub scale_y:f64,pub rotation:f64}
#[derive(Clone,Debug)]
pub struct RasterStackMask {pub enabled:bool,pub linked:bool,pub invert:bool,pub transform:RasterStackTransform,pub width:Option<u32>,pub height:Option<u32>,pub image_key:Option<String>}
#[derive(Clone,Debug)]
pub enum RasterStackContent {Pixel {width:Option<u32>,height:Option<u32>,image_key:Option<String>},Group(Vec<RasterStackLayer>),BrightnessContrast {brightness:f64,contrast:f64}}
#[derive(Clone,Debug)]
pub struct RasterStackLayer {pub id:String,pub visible:bool,pub opacity:f64,pub blend_mode:CompositeBlend,pub transform:RasterStackTransform,pub mask:Option<RasterStackMask>,pub content:RasterStackContent}
pub struct RasterStackInput {pub layers:Vec<RasterStackLayer>,pub images:BTreeMap<String,Arc<RasterImage>>}
pub struct RasterStackResult {pub image:RasterImage,pub origin:[f64;2],pub empty:bool}
struct Preparation {image:Arc<RasterImage>,coverage:Arc<[u8]>,offset:usize}
struct Planner<'a> {source:&'a BTreeMap<String,Arc<RasterImage>>,images:BTreeMap<String,Arc<RasterImage>>,preparations:Vec<Preparation>,mask_indices:BTreeMap<String,usize>,mask_slots:Vec<usize>,ids:std::collections::BTreeSet<String>,bounds:[f64;4],commands:usize,coverage_total:usize}
fn matrix(value:RasterStackTransform)->Result<CompositeAffine,PixelEditError>{
    if ![value.x,value.y,value.scale_x,value.scale_y,value.rotation].iter().all(|v|v.is_finite()){return Err(PixelEditError::Invalid("Layer transform requires finite coordinates"));}
    let (s,c)=value.rotation.to_radians().sin_cos();let result=[c*value.scale_x,s*value.scale_x,-s*value.scale_y,c*value.scale_y,value.x,value.y];inverse(result)?;Ok(result)
}
fn centered(value:CompositeAffine,width:u32,height:u32,source_width:u32,source_height:u32)->Result<CompositeAffine,PixelEditError>{
    validate_extent(width,height)?;
    Ok(multiply(value,[f64::from(width)/f64::from(source_width),0.0,0.0,f64::from(height)/f64::from(source_height),-f64::from(width)/2.0,-f64::from(height)/2.0]))
}
impl Planner<'_> {
    fn image(&self,key:&str)->Result<Arc<RasterImage>,PixelEditError>{
        if key.is_empty(){return Err(PixelEditError::Invalid("Layer image is missing"));}
        self.source.get(key).cloned().ok_or(PixelEditError::Invalid("Layer image is missing"))
    }
    fn mask(&mut self,mask:&Option<RasterStackMask>,local:CompositeAffine,layer:CompositeAffine,world:CompositeAffine)->Result<Option<CompositeMask>,PixelEditError>{
        let Some(mask)=mask else{return Ok(None)};let placement=matrix(mask.transform)?;
        if let Some(width)=mask.width {validate_extent(width,1)?;}
        if let Some(height)=mask.height {validate_extent(1,height)?;}
        if let (Some(width),Some(height))=(mask.width,mask.height){validate_extent(width,height)?;}
        if !mask.enabled{return Ok(None)}
        let Some(key)=mask.image_key.as_deref() else{return Ok(None)};let image=self.image(key)?;
        let transform=multiply(inverse(local)?,multiply(if mask.linked{layer}else{IDENTITY},centered(placement,mask.width.unwrap_or(image.width),mask.height.unwrap_or(image.height),image.width,image.height)?));
        inverse(transform)?;inverse(multiply(world,transform))?;
        let index=if let Some(index)=self.mask_indices.get(key){*index}else{
            let count=validate_extent(image.width,image.height)?;
            if self.coverage_total+count>67108864{return Err(PixelEditError::Invalid("Mask coverage exceeds 64 MiB preparation budget"));}
            self.coverage_total+=count;let index=self.preparations.len();self.mask_indices.insert(key.to_owned(),index);
            self.preparations.push(Preparation {image:Arc::clone(&image),coverage:vec![0;count].into(),offset:0});index
        };
        self.mask_slots.push(index);
        Ok(Some(CompositeMask {width:image.width,height:image.height,coverage:Arc::from([]),transform,invert:mask.invert}))
    }
    fn layers(&mut self,layers:Vec<RasterStackLayer>,parent:CompositeAffine,depth:usize,enabled:bool,paint_enabled:bool)->Result<Vec<CompositeLayer>,PixelEditError>{
        if depth>32{return Err(PixelEditError::Invalid("Invalid layer nesting"));}
        let mut output=Vec::with_capacity(layers.len());
        for layer in layers {
            if layer.id.is_empty()||!self.ids.insert(layer.id.clone())||self.ids.len()>1024{return Err(PixelEditError::Invalid("Layer identifiers must be unique within 1024 nodes"));}
            if !layer.opacity.is_finite()||!(0.0..=1.0).contains(&layer.opacity){return Err(PixelEditError::Invalid("Invalid layer opacity"));}
            let transform=matrix(layer.transform)?;let visible=enabled&&layer.visible;
            let opacity=if layer.mask.as_ref().is_some_and(|m|m.enabled&&m.image_key.is_none()&&m.invert){0.0}else{layer.opacity};
            let active=paint_enabled&&visible&&opacity>0.0;let mut local=transform;let mut pixel=None;
            if let RasterStackContent::Pixel {width,height,image_key}=&layer.content {
                let source=image_key.as_deref().map(|key|self.image(key)).transpose()?;
                let width=width.unwrap_or_else(||source.as_ref().map_or(512,|image|image.width));let height=height.unwrap_or_else(||source.as_ref().map_or(512,|image|image.height));
                let image=source.unwrap_or_else(||Arc::new(RasterImage::new(1,1)));
                local=centered(transform,width,height,image.width,image.height)?;
                let world=multiply(parent,local);
                if visible {for (x,y) in [(0.0,0.0),(f64::from(image.width),0.0),(0.0,f64::from(image.height)),(f64::from(image.width),f64::from(image.height))]{
                    let px=world[0]*x+world[2]*y+world[4];let py=world[1]*x+world[3]*y+world[5];
                    self.bounds[0]=self.bounds[0].min(px);self.bounds[1]=self.bounds[1].min(py);self.bounds[2]=self.bounds[2].max(px);self.bounds[3]=self.bounds[3].max(py);
                }}
                pixel=Some(image);
            }
            let world=multiply(parent,local);inverse(world)?;
            if matches!(layer.content,RasterStackContent::BrightnessContrast {..})&&layer.mask.is_some(){return Err(PixelEditError::Invalid("Adjustment layers cannot own masks"));}
            let mask=self.mask(&layer.mask,local,transform,world)?;
            let content=match layer.content {
                RasterStackContent::Pixel {..}=>{let key=self.ids.len().to_string();self.images.insert(key.clone(),pixel.unwrap());if active{self.commands+=1;}CompositeContent::Pixels(key)},
                RasterStackContent::Group(children)=>{if active{self.commands+=2;}CompositeContent::Group(self.layers(children,world,depth+1,visible,active)?)},
                RasterStackContent::BrightnessContrast {brightness,contrast}=>{
                    if !brightness.is_finite()||!contrast.is_finite()||!(-1.0..=1.0).contains(&brightness)||!(-1.0..=1.0).contains(&contrast){return Err(PixelEditError::Invalid("Adjustment exceeds allowed range"));}
                    if active{self.commands+=1;}CompositeContent::Adjustment {brightness,contrast}
                }
            };
            output.push(CompositeLayer {opacity,blend:layer.blend_mode,visible:layer.visible,transform:local,mask,content});
        }
        Ok(output)
    }
}
/// 🧵️ Converts unique mask buffers within bounded grants before publishing a composite.
pub struct RasterStackJob {pending:Option<CompositeInput>,composite:Option<CompositeJob>,preparations:Vec<Preparation>,mask_slots:Vec<usize>,preparation:usize,prepared:usize,total:usize,origin:[f64;2],empty:bool,cancelled:bool}
impl RasterStackJob {
    pub fn new(input:RasterStackInput)->Result<Self,PixelEditError>{
        if input.images.len()>1024{return Err(PixelEditError::Invalid("Image count exceeds compositor budget"));}
        for image in input.images.values(){validate_image(image)?;}
        let mut plan=Planner {source:&input.images,images:BTreeMap::new(),preparations:Vec::new(),mask_indices:BTreeMap::new(),mask_slots:Vec::new(),ids:Default::default(),bounds:[f64::INFINITY,f64::INFINITY,f64::NEG_INFINITY,f64::NEG_INFINITY],commands:1,coverage_total:0};
        let layers=plan.layers(input.layers,IDENTITY,0,true,true)?;let empty=plan.bounds[0]==f64::INFINITY;
        let origin=if empty{[0.0;2]}else{[plan.bounds[0],plan.bounds[1]]};
        let extents=if empty{[1.0;2]}else{[(plan.bounds[2]-origin[0]).ceil().max(1.0),(plan.bounds[3]-origin[1]).ceil().max(1.0)]};
        if !origin.iter().all(|v|v.is_finite())||!extents.iter().all(|v|v.is_finite()&&*v<=16384.0){return Err(PixelEditError::Invalid("Raster stack bounds exceed allowed extents"));}
        let width=extents[0] as u32;let height=extents[1] as u32;let count=validate_extent(width,height)?;
        let total=plan.commands.checked_mul(count).and_then(|n|n.checked_add(plan.coverage_total)).ok_or(PixelEditError::Invalid("Compositing work exceeds numeric limits"))?;
        Ok(Self {pending:Some(CompositeInput {width,height,origin,images:plan.images,layers}),composite:None,preparations:plan.preparations,mask_slots:plan.mask_slots,preparation:0,prepared:0,total,origin,empty,cancelled:false})
    }
    pub fn advance(&mut self,mut budget:usize)->Result<PixelProgress,PixelEditError>{
        if self.cancelled{return Err(PixelEditError::Cancelled);}
        if !(1..=1048576).contains(&budget){return Err(PixelEditError::Invalid("Stack grant must be between 1 and 1048576"));}
        while budget>0&&self.preparation<self.preparations.len(){
            let prep=&mut self.preparations[self.preparation];let end=prep.coverage.len().min(prep.offset+budget);budget-=end-prep.offset;self.prepared+=end-prep.offset;
            let coverage=Arc::get_mut(&mut prep.coverage).unwrap();
            for (offset,value) in coverage.iter_mut().enumerate().take(end).skip(prep.offset){let at=offset*4;let bytes=&prep.image.pixels;*value=((0.2126*f64::from(bytes[at])+0.7152*f64::from(bytes[at+1])+0.0722*f64::from(bytes[at+2]))*f64::from(bytes[at+3])/255.0).round() as u8;}
            prep.offset=end;if end==coverage.len(){self.preparation+=1;}
        }
        if self.preparation==self.preparations.len()&&self.composite.is_none(){
            fn attach(layers:&mut [CompositeLayer],slots:&mut impl Iterator<Item=usize>,preparations:&[Preparation]){
                for layer in layers{if let Some(mask)=&mut layer.mask{mask.coverage=Arc::clone(&preparations[slots.next().unwrap()].coverage);}if let CompositeContent::Group(children)=&mut layer.content{attach(children,slots,preparations);}}
            }
            let mut input=self.pending.take().unwrap();attach(&mut input.layers,&mut self.mask_slots.iter().copied(),&self.preparations);
            self.composite=Some(CompositeJob::new(input)?);self.preparations.clear();self.mask_slots.clear();
        }
        let progress=if let Some(job)=&mut self.composite{if budget>0{job.advance(budget)?}else{PixelProgress {completed:job.completed,total:job.total,done:job.completed==job.total}}}else{PixelProgress {completed:0,total:0,done:false}};
        Ok(PixelProgress {completed:self.prepared+progress.completed,total:self.total,done:progress.done})
    }
    pub fn into_result(mut self)->Result<RasterStackResult,PixelEditError>{
        if self.cancelled{return Err(PixelEditError::Cancelled);}
        let image=self.composite.take().ok_or(PixelEditError::Incomplete)?.into_result()?;
        Ok(RasterStackResult {image,origin:self.origin,empty:self.empty})
    }
    pub fn cancel(&mut self){self.cancelled=true;self.pending=None;self.composite=None;self.preparations.clear();self.mask_slots.clear();}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
