//! 🧵️ Pixel results retain private ownership through byte comparison, metadata preparation and canonical identity checks.
use super::*;
use semio_framework_pixels::RasterImage;
use semio_s_artifact_raster_raster_model::{SemioImageSnapshot,RasterImageContentHash,RasterImageContentEquality,RASTER_IMAGE_KEY_BYTES,raster_image_key};

#[derive(Default,semio_framework_value::RetireOwned)]
pub(super) struct RasterPixelPreparedImage {
    pub before:Option<RasterImage>,
    pub image:Option<SemioImageSnapshot>,
    pub key:Option<String>,
    pub digest:Option<[u8;8]>,
    pub changed:bool,
    pub reuse_destination:bool,
}

#[derive(Default,semio_framework_value::RetireOwned)]
struct ResultOwned {prepared:RasterPixelPreparedImage,after:Option<RasterImage>}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase {Compute,Compare,Image,Hash,Key,Destination,Done}

pub(super) struct RasterPixelResultWork {
    owned:std::mem::ManuallyDrop<Option<ResultOwned>>,
    retirement:std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    compute:Option<RasterPixelComputeWork>,
    copy:Option<RasterImageCopyWork>,
    hash:Option<RasterImageContentHash>,
    equality:RasterImageContentEquality,
    phase:Phase,
    cursor:usize,
    work_units:usize,
    closing:bool,
}

fn result_error(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}

impl RasterPixelResultWork {
    pub(super) fn new()->Self{Self{owned:std::mem::ManuallyDrop::new(Some(ResultOwned::default())),retirement:std::mem::ManuallyDrop::new(None),compute:Some(RasterPixelComputeWork::new()),copy:None,hash:None,equality:RasterImageContentEquality::new(),phase:Phase::Compute,cursor:0,work_units:0,closing:false}}

    fn data(&self)->Result<&ResultOwned,ValueError>{self.owned.as_ref().ok_or_else(||result_error("pixel result owner is absent"))}

    fn source(&self,base:&RasterSnapshot)->Result<Option<std::sync::Arc<SemioImageSnapshot>>,ValueError>{
        let descriptor=self.compute.as_ref().ok_or_else(||result_error("pixel computation is absent"))?.descriptor()?;
        descriptor.image_key.as_ref().map(|key|base.assets.get(key).and_then(|child|child.local_owner::<SemioImageSnapshot>()).ok_or_else(||result_error("pixel result source is unmaterialized"))).transpose()
    }

    fn preserve_metadata(&self,operation:&RasterMutation)->bool{!matches!(operation,RasterMutation::TransformImage(_))}

    pub(super) fn step(&mut self,base:&RasterSnapshot,operation:&RasterMutation,cx:&mut dyn RasterWork)->Result<bool,&'static str>{
        if cx.should_yield(){return Ok(false);}
        let grant=cx.retained_grant();let demand=self.demands(base,operation,grant.maximum_copy_bytes).map_err(|_|"raster-store.pixel-result-demand")?;
        if grant.maximum_items==0||grant.maximum_depth<demand.depth||raster_short(grant,demand){return Ok(false);}
        let work_budget=cx.fuel_remaining().min(1024)as usize;
        let turn=self.advance_controlled(base,operation,grant,work_budget,||cx.should_yield()||cx.is_cancelled());
        cx.consume_fuel(self.work_units as u64);
        match turn{
            Ok(step)=>{cx.consume_retained(step.progress()).map_err(|_|"raster-store.pixel-result-receipt")?;Ok(matches!(step,RetainedCloneStep::Complete(_)))},
            Err(error)=>{cx.consume_retained(error.retained_progress()).map_err(|_|"raster-store.pixel-result-error-receipt")?;Err("raster-store.pixel-result-refused")},
        }
    }

    pub(super) fn demands(&self,base:&RasterSnapshot,operation:&RasterMutation,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
        if self.closing{return Err(result_error("pixel result is closing"));}
        let mut demand=semio_framework_value::RetirementDemand{depth:1,..Default::default()};
        match self.phase{
            Phase::Compute=>return self.compute.as_ref().ok_or_else(||result_error("pixel computation missing"))?.demands(base,operation,body),
            Phase::Compare=>{let bytes=self.data()?.after.as_ref().ok_or_else(||result_error("pixel output missing"))?.pixels.len();demand.copy_bytes=(bytes-self.cursor).min(body.min(32768)/2)*2;if self.cursor<bytes&&demand.copy_bytes==0{demand.copy_bytes=2;}},
            Phase::Image=>{
                if self.preserve_metadata(operation){if let Some(source)=self.source(base)?{return self.copy.as_ref().map_or(Ok(semio_framework_value::RetirementDemand{copy_bytes:size_of::<RasterImageCopyWork>(),depth:1,..Default::default()}),|copy|copy.demands(&source,body));}}
                demand.capacity_bytes=b"stdio.semio.image".len()+size_of::<semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageFrame>();demand.copy_bytes=size_of::<SemioImageSnapshot>()+b"stdio.semio.image".len();
            },
            Phase::Hash=>demand.copy_bytes=body.min(32768).max(1),
            Phase::Key=>{demand.copy_bytes=RASTER_IMAGE_KEY_BYTES;demand.capacity_bytes=RASTER_IMAGE_KEY_BYTES;},
            Phase::Destination=>demand.copy_bytes=body.min(32768).max(2),
            Phase::Done=>{},
        }
        Ok(demand)
    }

    pub(super) fn advance(&mut self,base:&RasterSnapshot,operation:&RasterMutation,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        self.advance_controlled(base,operation,grant,1024,||false)
    }

    pub(super) fn advance_controlled(&mut self,base:&RasterSnapshot,operation:&RasterMutation,grant:RetainedCloneGrant,work_budget:usize,mut should_stop:impl FnMut()->bool)->Result<RetainedCloneStep,ValueError>{
        self.work_units=0;
        let demand=self.demands(base,operation,grant.maximum_copy_bytes)?;
        if work_budget==0||should_stop()||grant.maximum_items==0||grant.maximum_depth<demand.depth||raster_short(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}
        self.work_units=1;
        if self.phase==Phase::Compute{
            let compute=self.compute.as_mut().ok_or_else(||result_error("pixel computation missing"))?;let result=compute.advance_controlled(base,operation,grant,work_budget,&mut should_stop);self.work_units=compute.last_work_units();let step=result?;
            if matches!(step,RetainedCloneStep::Complete(_)){let(before,after)=compute.take().ok_or_else(||result_error("pixel computation did not hand off"))?;let data=self.owned.as_mut().unwrap();data.prepared.changed=matches!(operation,RasterMutation::TransformImage(_));data.prepared.before=Some(before);data.after=Some(after);self.phase=if data.prepared.changed{Phase::Image}else{Phase::Compare};}
            return Ok(RetainedCloneStep::Progress(step.progress()));
        }
        let mut receipt=RetainedCloneProgress{copied_items:1,..Default::default()};
        match self.phase{
            Phase::Compare=>{
                let data=self.owned.as_mut().unwrap();let before=data.prepared.before.as_ref().ok_or_else(||result_error("pixel source missing"))?;let after=data.after.as_ref().ok_or_else(||result_error("pixel output missing"))?;
                if before.width!=after.width||before.height!=after.height||before.pixels.len()!=after.pixels.len(){data.prepared.changed=true;self.phase=Phase::Image;}
                else{if work_budget<2&&self.cursor<after.pixels.len(){self.work_units=0;return Ok(RetainedCloneStep::Progress(Default::default()))}let end=(self.cursor+demand.copy_bytes.min(work_budget)/2).min(after.pixels.len());let equal=before.pixels[self.cursor..end]==after.pixels[self.cursor..end];receipt.copied_bytes=(end-self.cursor)*2;self.work_units=receipt.copied_bytes.max(1);self.cursor=end;if !equal{data.prepared.changed=true;self.phase=Phase::Image;}else if end==after.pixels.len(){self.phase=Phase::Done;}}
            },
            Phase::Image=>{
                if self.preserve_metadata(operation){if let Some(source)=self.source(base)?{
                    if self.copy.is_none(){self.copy=Some(RasterImageCopyWork::new());receipt.copied_bytes=size_of::<RasterImageCopyWork>();return Ok(RetainedCloneStep::Progress(receipt));}
                    let copy=self.copy.as_mut().unwrap();let data=self.owned.as_mut().unwrap();let step=copy.advance(&source,&mut data.after,grant)?;
                    if matches!(step,RetainedCloneStep::Complete(_)){data.prepared.image=copy.take();self.hash=Some(RasterImageContentHash::new());self.phase=Phase::Hash;}
                    return Ok(RetainedCloneStep::Progress(step.progress()));
                }}
                let data=self.owned.as_mut().unwrap();let after=data.after.take().ok_or_else(||result_error("pixel output owner missing"))?;
                let image=semio_s_artifact_raster_raster_model::semio_image_from_rgba8(after.width,after.height,after.pixels);
                receipt.copied_bytes=demand.copy_bytes;receipt.retained_capacity_bytes=image.schema.capacity()+image.frames.capacity()*size_of::<semio_s_artifact_stdio_semio_image::schema::snapshot::SemioImageFrame>();data.prepared.image=Some(image);self.hash=Some(RasterImageContentHash::new());self.phase=Phase::Hash;
            },
            Phase::Hash=>{
                let data=self.owned.as_mut().unwrap();let hash=self.hash.as_mut().ok_or_else(||result_error("pixel image hash missing"))?;let progress=hash.advance(data.prepared.image.as_ref().ok_or_else(||result_error("pixel image missing"))?,demand.copy_bytes.min(work_budget)).map_err(result_error)?;receipt.copied_bytes=progress.processed_bytes;self.work_units=progress.processed_bytes.max(1);if progress.done{data.prepared.digest=self.hash.take().unwrap().finish();self.phase=Phase::Key;}
            },
            Phase::Key=>{let data=self.owned.as_mut().unwrap();let key=raster_image_key(data.prepared.digest.ok_or_else(||result_error("pixel image digest missing"))?);receipt.copied_bytes=key.len();receipt.retained_capacity_bytes=key.capacity();data.prepared.key=Some(key);self.phase=Phase::Destination;},
            Phase::Destination=>{
                let data=self.owned.as_mut().unwrap();let key=data.prepared.key.as_ref().ok_or_else(||result_error("pixel image key missing"))?;
                if let Some(child)=base.assets.get(key){if work_budget<2{self.work_units=0;return Ok(RetainedCloneStep::Progress(Default::default()))}let destination=child.local_owner::<SemioImageSnapshot>().ok_or_else(||result_error("canonical pixel destination is unmaterialized"))?;let progress=self.equality.advance(data.prepared.image.as_ref().ok_or_else(||result_error("pixel image missing"))?,&destination,demand.copy_bytes.min(work_budget)).map_err(result_error)?;receipt.copied_bytes=progress.processed_bytes;self.work_units=progress.processed_bytes.max(1);if progress.done{if !progress.equal{return Err(result_error("canonical pixel image key collides with different content").with_retained_progress(receipt));}data.prepared.reuse_destination=true;self.phase=Phase::Done;}}
                else{self.phase=Phase::Done;}
            },
            Phase::Done=>return Ok(RetainedCloneStep::Complete(Default::default())),
            Phase::Compute=>unreachable!(),
        }
        if !receipt.fits(grant){return Err(result_error("pixel result exceeded granted physical resources").with_retained_progress(receipt));}
        Ok(if self.phase==Phase::Done{RetainedCloneStep::Complete(receipt)}else{RetainedCloneStep::Progress(receipt)})
    }

    pub(super) fn take(&mut self)->Option<RasterPixelPreparedImage>{if self.closing||self.phase!=Phase::Done{return None;}Some(std::mem::take(&mut self.owned.as_mut()?.prepared))}

    pub(super) fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{if let Some(compute)=self.compute.as_ref(){return compute.close_demands(body);}if let Some(copy)=self.copy.as_ref(){return copy.close_demands(body);}if let Some(active)=self.retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(active,body);}if self.owned.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.owned);}Ok(Default::default())}

    pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep>{self.closing=true;if grant.maximum_items==0||grant.maximum_depth==0{return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0};}if let Some(compute)=self.compute.as_mut(){if compute.terminal_is_empty(){self.compute.take();return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1};}return compute.close_step(grant);}if let Some(copy)=self.copy.as_mut(){if copy.terminal_is_empty(){self.copy.take();return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1};}return copy.close_step(grant);}if self.retirement.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant)}else{store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.owned,&mut self.retirement,grant)}}

    pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.compute.is_none()&&self.copy.is_none()&&self.owned.is_none()&&self.retirement.is_none()}
}

impl Drop for RasterPixelResultWork{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"pixel result must hand off or retire every owner before Drop");}}
