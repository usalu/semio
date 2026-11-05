//! 🔲️ Exact-base image preparation with separately granted reservation, copy, mask normalization and retirement.
use super::*;
use semio_s_artifact_raster_raster_model::SemioImageSnapshot;

/// 🖼️ Intrinsic pixel coordinates and attachment captured before any source bytes are prepared.
#[derive(semio_framework_value::RetireOwned)]
pub struct RasterPixelSourceDescriptor {
    pub layer_id: String,
    pub target: String,
    pub image_key: Option<String>,
    pub width: u32,
    pub height: u32,
    pub mask: bool,
    pub protected: bool,
    pub visible: bool,
}

#[derive(semio_framework_value::RetireOwned)]
struct SourceOwned {
    descriptor: RasterPixelSourceDescriptor,
    pixels: Option<Vec<u8>>,
}

/// 🧵️ Unpublished source storage remains owned until the result handoff or supplied-grant closure.
pub struct RasterPixelSourceWork {
    owned: std::mem::ManuallyDrop<Option<SourceOwned>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    base_binding: usize,
    frame_binding: Option<usize>,
    phase: u8,
    path: [usize;RASTER_MAXIMUM_NESTED_DEPTH],
    locks: [bool;RASTER_MAXIMUM_NESTED_DEPTH],
    visibility: [bool;RASTER_MAXIMUM_NESTED_DEPTH],
    depth: usize,
    address: Option<RasterLayerAddress>,
    key_bytes: usize,
    closing: bool,
}

fn source_error(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue, message) }

fn source_geometry<'a>(base: &'a RasterSnapshot, address:RasterLayerAddress, target: &str) -> Result<(Option<&'a str>, u32, u32, bool), ValueError> {
    let layer = RasterLayerLocator::node_at(base,address).ok_or_else(|| source_error("pixel source layer is missing"))?;
    let (width, height) = match layer { RasterLayerNode::Pixel { width, height, .. } => (width.unwrap_or(512), height.unwrap_or(512)), _ => (512, 512) };
    let (key, width, height, mask) = match (target, layer) {
        ("pixels", RasterLayerNode::Pixel { image_key, .. }) => (image_key.as_deref(), width, height, false),
        ("mask", RasterLayerNode::Pixel { mask: Some(mask), .. } | RasterLayerNode::Group { mask: Some(mask), .. }) => (mask.image_key.as_deref(), mask.width.unwrap_or(width), mask.height.unwrap_or(height), true),
        _ => return Err(source_error("pixel source target is unavailable")),
    };
    if key.is_some_and(|key|key.len()>RASTER_OWNED_FIELD_BYTES){return Err(source_error("pixel source attachment exceeds its field budget"));}
    let (width, height) = if let Some(key) = key {
        let image = source_image(base, key)?;
        let count = semio_framework_pixels::editing::validate_extent(image.width, image.height).map_err(|_| source_error("pixel source extent exceeds its budget"))?;
        if image.frames.first().is_none_or(|frame| frame.rgba8.len() != count * 4) { return Err(source_error("pixel source frame disagrees with its extent")); }
        (image.width, image.height)
    } else { (width, height) };
    semio_framework_pixels::editing::validate_extent(width, height).map_err(|_| source_error("pixel source extent exceeds its budget"))?;
    Ok((key, width, height, mask))
}

fn source_image(base: &RasterSnapshot, key: &str) -> Result<std::sync::Arc<SemioImageSnapshot>, ValueError> {
    base.assets.get(key).and_then(|child| child.local_owner::<SemioImageSnapshot>()).ok_or_else(|| source_error("pixel source image is missing or unmaterialized"))
}

impl RasterPixelSourceWork {
    pub fn begin_demands(_base: &RasterSnapshot, layer_id: &str, target: &str) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if !matches!(target,"pixels"|"mask")||layer_id.len().max(target.len())>RASTER_OWNED_FIELD_BYTES{return Err(source_error("pixel source descriptor exceeds its field contract"));}
        let bytes=layer_id.len()+target.len();
        Ok(semio_framework_value::RetirementDemand { copy_bytes: bytes+size_of::<Self>(), capacity_bytes: bytes, depth: 1, ..Default::default() })
    }

    pub fn begin(base: &RasterSnapshot, layer_id: &str, target: &str, grant: RetainedCloneGrant) -> Result<Option<(Self, RetainedCloneProgress)>, ValueError> {
        let demand = Self::begin_demands(base, layer_id, target)?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || raster_short(grant, demand) { return Ok(None); }
        let descriptor = RasterPixelSourceDescriptor { layer_id: raster_exact_string_from_parts(&[layer_id.as_bytes()]).map_err(source_error)?, target: raster_exact_string_from_parts(&[target.as_bytes()]).map_err(source_error)?, image_key: None, width:0, height:0, mask:target=="mask", protected:false,visible:false };
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, retained_capacity_bytes: demand.capacity_bytes, ..Default::default() };
        Ok(Some((Self { owned: std::mem::ManuallyDrop::new(Some(SourceOwned { descriptor, pixels: Some(Vec::new()) })), retirement: std::mem::ManuallyDrop::new(None), base_binding: base as *const RasterSnapshot as usize, frame_binding:None, phase:0,path:[0;RASTER_MAXIMUM_NESTED_DEPTH],locks:[false;RASTER_MAXIMUM_NESTED_DEPTH],visibility:[true;RASTER_MAXIMUM_NESTED_DEPTH],depth:1,address:None,key_bytes:0,closing:false }, progress)))
    }

    pub fn descriptor(&self) -> Result<&RasterPixelSourceDescriptor, ValueError> { self.owned.as_ref().map(|owned| &owned.descriptor).ok_or_else(|| source_error("pixel source descriptor was retired")) }

    pub fn demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if self.closing || self.phase > 3 { return Err(source_error("pixel source cannot advance after handoff or close")); }
        if self.phase==0{return Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<RasterLayerAddress>()+size_of::<(usize,bool)>(),depth:1,..Default::default()})}
        if self.phase==1{return Ok(semio_framework_value::RetirementDemand{copy_bytes:self.key_bytes,capacity_bytes:self.key_bytes,depth:1,..Default::default()})}
        let descriptor = self.descriptor()?;
        let length = descriptor.width as usize * descriptor.height as usize * 4;
        if self.phase == 2 { return Ok(semio_framework_value::RetirementDemand { capacity_bytes: length, depth: 1, ..Default::default() }); }
        let prepared = self.owned.as_ref().and_then(|owned| owned.pixels.as_ref()).ok_or_else(|| source_error("pixel source storage is missing"))?.len();
        Ok(semio_framework_value::RetirementDemand { copy_bytes: (length - prepared).min(body.min(32_768) / 4 * 4), depth: 1, ..Default::default() })
    }

    pub fn geometry_ready(&self)->bool{!self.closing&&self.phase>=2}

    pub fn geometry_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{if self.geometry_ready(){Ok(Default::default())}else{self.demands(body)}}

    pub fn advance_geometry(&mut self,base:&RasterSnapshot,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if self.geometry_ready(){return Ok(RetainedCloneStep::Complete(Default::default()))}
        let demand=self.geometry_demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items==0||grant.maximum_depth<demand.depth||raster_short(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
        if base as *const RasterSnapshot as usize!=self.base_binding{return Err(source_error("pixel source was resumed against another base"))}
        let owned=self.owned.as_mut().ok_or_else(||source_error("pixel source owner disappeared"))?;
        if self.phase==0{
            let address=RasterLayerAddress{length:self.depth,indices:self.path};
            if let Some(node)=RasterLayerLocator::node_at(base,address){
                let inherited=self.depth>1&&self.locks[self.depth-2];
                let protected=inherited||semio_s_artifact_raster_raster_model::layer_locked(node);
                let visible=(self.depth==1||self.visibility[self.depth-2])&&semio_s_artifact_raster_raster_model::layer_visible(node);
                self.locks[self.depth-1]=protected;
                self.visibility[self.depth-1]=visible;
                if RasterLayerLocator::node_id(node)==&owned.descriptor.layer_id{
                    let(key,width,height,mask)=source_geometry(base,address,&owned.descriptor.target)?;
                    self.address=Some(address);self.key_bytes=key.map_or(0,str::len);owned.descriptor.width=width;owned.descriptor.height=height;owned.descriptor.mask=mask;owned.descriptor.protected=protected;owned.descriptor.visible=visible;self.phase=1;
                }else if matches!(node,RasterLayerNode::Group{children,..}if !children.is_empty()){
                    if self.depth==RASTER_MAXIMUM_NESTED_DEPTH{return Err(source_error("pixel source layer depth exceeds its bounded lookup"))}
                    self.path[self.depth]=0;self.depth+=1;
                }else{self.path[self.depth-1]+=1;}
            }else if self.depth>1{self.depth-=1;self.path[self.depth-1]+=1;}else{return Err(source_error("pixel source layer is missing"))}
            return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))
        }
        let(key,width,height,_)=source_geometry(base,self.address.ok_or_else(||source_error("pixel source address disappeared"))?,&owned.descriptor.target)?;
        if key.map_or(0,str::len)!=self.key_bytes||width!=owned.descriptor.width||height!=owned.descriptor.height{return Err(source_error("pixel source geometry changed"))}
        owned.descriptor.image_key=key.map(|key|raster_exact_string_from_parts(&[key.as_bytes()]).map_err(source_error)).transpose()?;
        self.frame_binding=key.map(|key|source_image(base,key).map(|image|image.frames[0].rgba8.as_ptr()as usize)).transpose()?;
        self.phase=2;
        Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:owned.descriptor.image_key.as_ref().map_or(0,String::capacity),..Default::default()}))
    }

    pub fn advance(&mut self, base: &RasterSnapshot, grant: RetainedCloneGrant) -> Result<RetainedCloneStep, ValueError> {
        if self.phase<2{let step=self.advance_geometry(base,grant)?;return Ok(RetainedCloneStep::Progress(step.progress()))}
        let demand = self.demands(grant.maximum_copy_bytes)?;
        if grant.maximum_items == 0 || grant.maximum_depth < demand.depth || raster_short(grant, demand) { return Ok(RetainedCloneStep::Progress(Default::default())); }
        if base as *const RasterSnapshot as usize != self.base_binding { return Err(source_error("pixel source was resumed against another base")); }
        let owned = self.owned.as_mut().ok_or_else(|| source_error("pixel source owner is missing"))?;
        let descriptor = &owned.descriptor;
        let length = descriptor.width as usize * descriptor.height as usize * 4;
        let pixels = owned.pixels.as_mut().ok_or_else(|| source_error("pixel source pixels were handed off"))?;
        if self.phase == 2 {
            pixels.try_reserve_exact(length).map_err(|_| source_error("pixel source reservation failed"))?;
            let receipt = RetainedCloneProgress { copied_items: 1, retained_capacity_bytes: pixels.capacity(), ..Default::default() };
            if !receipt.fits(grant) { return Err(source_error("pixel source allocator exceeded its admitted reservation").with_retained_progress(receipt)); }
            self.phase = 3;
            return Ok(RetainedCloneStep::Progress(receipt));
        }
        if demand.copy_bytes == 0 && pixels.len() != length { return Ok(RetainedCloneStep::Progress(Default::default())); }
        let end = pixels.len() + demand.copy_bytes;
        if let Some(key) = &descriptor.image_key {
            let image = source_image(base, key)?;
            let frame = image.frames.first().ok_or_else(|| source_error("pixel source frame disappeared"))?;
            if Some(frame.rgba8.as_ptr() as usize) != self.frame_binding { return Err(source_error("pixel source image revision changed")); }
            let bytes = frame.rgba8.get(pixels.len()..end).ok_or_else(|| source_error("pixel source frame extent changed"))?;
            if descriptor.mask {
                for pixel in bytes.chunks_exact(4) { pixels.extend_from_slice(&[255,255,255,semio_framework_pixels::compositing::layers::mask_coverage([pixel[0],pixel[1],pixel[2],pixel[3]])]); }
            } else { pixels.extend_from_slice(bytes); }
        } else { pixels.resize(end, if descriptor.mask { 255 } else { 0 }); }
        let receipt = RetainedCloneProgress { copied_items: 1, copied_bytes: demand.copy_bytes, ..Default::default() };
        if pixels.len() == length { self.phase = 4; Ok(RetainedCloneStep::Complete(receipt)) } else { Ok(RetainedCloneStep::Progress(receipt)) }
    }

    pub fn progress(&self) -> (usize, usize) { self.owned.as_ref().map_or((0,0), |owned| (owned.pixels.as_ref().map_or(0, Vec::len), owned.descriptor.width as usize * owned.descriptor.height as usize * 4)) }

    pub fn take_pixels(&mut self) -> Option<semio_framework_pixels::RasterImage> {
        if self.closing || self.phase != 4 { return None; }
        let owned = self.owned.as_mut()?;
        let pixels = owned.pixels.take()?;
        self.phase = 5;
        Some(semio_framework_pixels::RasterImage { width: owned.descriptor.width, height: owned.descriptor.height, pixels })
    }

    pub fn close_demands(&self, body: usize) -> Result<semio_framework_value::RetirementDemand, ValueError> {
        if let Some(active) = self.retirement.as_ref() { return store::os_store::snapshot_read::artifact_retirement_box_demands(active, body); }
        if self.owned.is_some() { return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.owned); }
        Ok(Default::default())
    }

    pub fn close_step(&mut self, grant: RetainedCloneGrant) -> semio_framework_value::RetainedWorkOutcome<RetainedCloneStep> {
        self.closing = true;
        if self.retirement.is_some() { store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement, grant) } else { store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.owned, &mut self.retirement, grant) }
    }

    pub fn terminal_is_empty(&self) -> bool { self.closing && self.owned.is_none() && self.retirement.is_none() }
}

impl Drop for RasterPixelSourceWork {
    fn drop(&mut self) { assert!(std::thread::panicking() || self.terminal_is_empty(), "pixel source must hand off or retire every owner before Drop"); }
}
