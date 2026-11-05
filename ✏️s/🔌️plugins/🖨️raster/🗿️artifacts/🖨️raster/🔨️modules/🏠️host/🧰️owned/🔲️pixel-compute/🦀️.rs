//! 🔲️ Six semantic pixel routes share source preparation, bounded coverage and pixel jobs, and explicit owner retirement.
use super::*;
use semio_framework_pixels::{RasterImage,editing::{PixelEditJob,PixelFloodSelectionJob,PixelOperation}};
use semio_s_artifact_raster_raster_mutations_pixels::{apply_filter,fill_region,paint_stroke,transform_image};

#[derive(Default,semio_framework_value::RetireOwned)]
struct ComputeOwned {
    before: Option<RasterImage>,
    after: Option<RasterImage>,
    selection: Option<Vec<u8>>,
    retired_selection: Option<Vec<u8>>,
    operation: Option<PixelOperation>,
    edit: Option<PixelEditJob>,
    flood: Option<PixelFloodSelectionJob>,
    flood_source: Option<std::sync::Arc<RasterImage>>,
    seen: Vec<bool>,
    queue: Vec<u32>,
    points: Vec<[f64;2]>,
    segments: Vec<usize>,
}

#[derive(Clone,Copy,PartialEq,Eq)]
enum Phase { Begin,Source,ReserveSelection,Selection,ReservePoints,Points,Operation,ReserveFloodQueue,Flood,EndFlood,AdmitEdit,Scratch,Edit,EndEdit,ReserveWrite,Write,Done }

/// 🧵️ No partially initialized or filtered image is observable before both prepared buffers are complete.
pub(super) struct RasterPixelComputeWork {
    owned: std::mem::ManuallyDrop<Option<ComputeOwned>>,
    retirement: std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
    source: Option<RasterPixelSourceWork>,
    phase: Phase,
    clip_span: Option<paint_stroke::RasterSelectionSpan>,
    selected_span: usize,
    cursor: usize,
    flood_complete: bool,
    work_units:usize,
    closing: bool,
}

fn compute_error(message: &'static str) -> ValueError { ValueError::literal(ValueRefusalKind::InvalidValue,message) }

pub(super) fn pixel_target(operation: &RasterMutation) -> Option<(&str,&str)> {
    match operation {
        RasterMutation::PaintStroke(value) => Some((&value.layer_id,&value.target)),
        RasterMutation::FillRegion(value) => Some((&value.layer_id,&value.target)),
        RasterMutation::ApplyFilter(value) => Some((&value.layer_id,"pixels")),
        RasterMutation::TransformImage(value) => Some((&value.layer_id,"pixels")),
        RasterMutation::FillSelection(value) => Some((&value.layer_id,&value.target)),
        RasterMutation::WritePixelRegion(value) => Some((&value.layer_id,&value.target)),
        _ => None,
    }
}

fn selection(operation: &RasterMutation) -> Option<&semio_s_artifact_raster_raster_model::RasterSelectionSpans> {
    match operation {
        RasterMutation::PaintStroke(value) => value.selection.as_ref(),
        RasterMutation::FillRegion(value) => value.selection.as_ref(),
        RasterMutation::ApplyFilter(value) => value.selection.as_ref(),
        RasterMutation::FillSelection(value) => value.selection.as_ref(),
        _ => None,
    }
}

fn valid_fields(operation: &RasterMutation) -> bool {
    let unit = |value:f64|value.is_finite()&&(0.0..=1.0).contains(&value);
    let color = |value:&[f64]|value.len()==4&&value.iter().copied().all(unit);
    match operation {
        RasterMutation::PaintStroke(value) => paint_stroke::RASTER_PAINT_TARGETS.contains(&value.target.as_str())&&paint_stroke::RASTER_PAINT_TOOLS.contains(&value.tool.as_str())&&!value.points.is_empty()&&value.points.len()<=paint_stroke::RASTER_STROKE_MAXIMUM_POINTS&&value.brush.size.is_finite()&&(0.1..=4096.0).contains(&value.brush.size)&&unit(value.brush.hardness)&&unit(value.brush.opacity)&&color(&value.brush.color),
        RasterMutation::FillRegion(value) => paint_stroke::RASTER_PAINT_TARGETS.contains(&value.target.as_str())&&value.tolerance<=255&&color(&value.color),
        RasterMutation::FillSelection(value) => paint_stroke::RASTER_PAINT_TARGETS.contains(&value.target.as_str())&&color(&value.color),
        RasterMutation::ApplyFilter(value) => apply_filter::RASTER_FILTERS.iter().find(|(kind,_)|*kind==value.filter).is_some_and(|(_,range)|match range {None=>value.amount==0.0,Some((min,max))=>value.amount.is_finite()&&(*min..=*max).contains(&value.amount)&&(!matches!(value.filter.as_str(),"blur"|"posterize")||value.amount.fract()==0.0)})&&(!matches!(value.filter.as_str(),"flipHorizontal"|"flipVertical")||value.selection.is_none()),
        RasterMutation::TransformImage(value) => transform_image::RASTER_IMAGE_TRANSFORMS.contains(&value.operation.as_str())&&match value.operation.as_str(){"resize"=>(1..=16384).contains(&value.width)&&(1..=16384).contains(&value.height)&&value.x==0&&value.y==0,"crop"=>(1..=16384).contains(&value.width)&&(1..=16384).contains(&value.height)&&!value.bilinear,_=>value.x==0&&value.y==0&&value.width==0&&value.height==0&&!value.bilinear},
        RasterMutation::WritePixelRegion(value) => paint_stroke::RASTER_PAINT_TARGETS.contains(&value.target.as_str())&&value.width>0&&value.height>0&&u64::from(value.width).checked_mul(u64::from(value.height)).and_then(|count|count.checked_mul(4)).is_some_and(|bytes|bytes==value.samples.len() as u64),
        _ => false,
    }
}

impl RasterPixelComputeWork {
    pub(super) fn new() -> Self { Self { owned:std::mem::ManuallyDrop::new(Some(ComputeOwned::default())),retirement:std::mem::ManuallyDrop::new(None),source:None,phase:Phase::Begin,clip_span:None,selected_span:0,cursor:0,flood_complete:false,work_units:0,closing:false } }

    pub(super) fn last_work_units(&self)->usize{self.work_units}

    fn data(&self) -> Result<&ComputeOwned,ValueError> { self.owned.as_ref().ok_or_else(||compute_error("pixel compute owner is absent")) }

    pub(super) fn descriptor(&self)->Result<&RasterPixelSourceDescriptor,ValueError>{self.source.as_ref().ok_or_else(||compute_error("pixel source owner missing"))?.descriptor()}

    #[cfg(test)]
    pub(super) fn copied_points(&self)->usize {self.owned.as_ref().map_or(0,|data|data.points.len())}

    fn operation_phase(operation:&RasterMutation)->Phase {if matches!(operation,RasterMutation::PaintStroke(_)){Phase::ReservePoints}else{Phase::Operation}}

    fn count(&self) -> Result<usize,ValueError> { let descriptor=self.source.as_ref().ok_or_else(||compute_error("pixel compute source is absent"))?.descriptor()?;Ok(descriptor.width as usize*descriptor.height as usize) }

    fn edit_budget(&self,operation:&RasterMutation,body:usize)->Result<(usize,usize),ValueError> {
        let data=self.data()?;let edit=data.edit.as_ref().ok_or_else(||compute_error("pixel edit job is absent"))?;
        let total=edit.output.width as usize*edit.output.height as usize;
        let budget=edit.recommended_grant().min(total-self.cursor).min(body.min(32768)/4);
        if budget==0{return Ok((0,4));}
        Ok((budget,if matches!(operation,RasterMutation::PaintStroke(_)){body.min(32768).max(size_of::<usize>())}else{budget*4}))
    }

    pub(super) fn demands(&self,snapshot:&RasterSnapshot,operation:&RasterMutation,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError> {
        if self.closing{return Err(compute_error("pixel computation is closing"))}
        let data=self.data()?;let mut demand=semio_framework_value::RetirementDemand{depth:1,..Default::default()};
        match self.phase {
            Phase::Begin=>{let(layer,target)=pixel_target(operation).ok_or_else(||compute_error("pixel mutation required"))?;return RasterPixelSourceWork::begin_demands(snapshot,layer,target)},
            Phase::Source=>return self.source.as_ref().ok_or_else(||compute_error("pixel source missing"))?.demands(body),
            Phase::ReserveSelection=>demand.capacity_bytes=self.count()?,
            Phase::Selection=>{let spans=selection(operation).ok_or_else(||compute_error("pixel selection disappeared"))?;if self.clip_span.is_none()&&self.selected_span<spans.len(){demand=spans.rank_observation_demand(self.selected_span)?;demand.copy_bytes=size_of::<paint_stroke::RasterSelectionSpan>();}else{let remaining=self.count()?-data.selection.as_ref().map_or(0,Vec::len);demand.copy_bytes=if remaining==0{0}else{remaining.min(body.min(32768)).max(1)};}},
            Phase::ReservePoints=>if let RasterMutation::PaintStroke(value)=operation{demand.capacity_bytes=value.points.len()*size_of::<[f64;2]>();},
            Phase::Points=>if let RasterMutation::PaintStroke(value)=operation{demand.copy_bytes=(value.points.len()-data.points.len()).min(body.min(32768)/size_of::<[f64;2]>()).max(1)*size_of::<[f64;2]>();},
            Phase::Operation=>if matches!(operation,RasterMutation::FillRegion(_))&&!self.flood_complete{demand.capacity_bytes=self.count()?.saturating_mul(2)+size_of::<RasterImage>()+2*size_of::<usize>();},
            Phase::ReserveFloodQueue=>demand.capacity_bytes=self.count()?.saturating_mul(size_of::<u32>()),
            Phase::Flood=>demand.copy_bytes=if body<26{26}else{((body.min(32768)-5)/21).min(1024)*21+5},
            Phase::EndFlood=>{demand.copy_bytes=size_of::<semio_framework_pixels::editing::PixelFloodStorage>()+size_of::<RasterImage>();demand.release_bytes=size_of::<RasterImage>()+2*size_of::<usize>();},
            Phase::AdmitEdit=>{let image=data.before.as_ref().ok_or_else(||compute_error("pixel input missing"))?;let editing=data.operation.as_ref().ok_or_else(||compute_error("pixel operation missing"))?;editing.validate(image,data.selection.is_some()).map_err(|_|compute_error("pixel operation is invalid for the image"))?;let(width,height)=editing.extent(image);demand.capacity_bytes=width as usize*height as usize*4;},
            Phase::Scratch=>demand.capacity_bytes=data.edit.as_ref().ok_or_else(||compute_error("pixel edit missing"))?.scratch_reservation_bytes(),
            Phase::Edit=>demand.copy_bytes=self.edit_budget(operation,body)?.1,
            Phase::EndEdit=>demand.copy_bytes=size_of::<semio_framework_pixels::editing::PixelEditStorage>(),
            Phase::ReserveWrite=>demand.capacity_bytes=self.count()?*4,
            Phase::Write=>{let remaining=self.count()?*4-data.after.as_ref().map_or(0,|image|image.pixels.len());demand.copy_bytes=if remaining==0{0}else{remaining.min(body.min(32768)/4*4).max(4)};},
            Phase::Done=>{},
        }
        Ok(demand)
    }

    pub(super) fn advance(&mut self,snapshot:&RasterSnapshot,operation:&RasterMutation,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError> {
        self.advance_controlled(snapshot,operation,grant,1024,||false)
    }

    pub(super) fn advance_controlled(&mut self,snapshot:&RasterSnapshot,operation:&RasterMutation,grant:RetainedCloneGrant,work_budget:usize,mut should_stop:impl FnMut()->bool)->Result<RetainedCloneStep,ValueError>{
        self.work_units=0;
        let demand=self.demands(snapshot,operation,grant.maximum_copy_bytes)?;
        if work_budget==0||should_stop()||grant.maximum_items==0||grant.maximum_depth<demand.depth||raster_short(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}
        self.work_units=1;
        if self.phase==Phase::Begin{
            if !valid_fields(operation){return Err(compute_error("pixel mutation fields are invalid"))}
            let(layer,target)=pixel_target(operation).ok_or_else(||compute_error("pixel mutation required"))?;
            let Some((source,receipt))=RasterPixelSourceWork::begin(snapshot,layer,target,grant)?else{return Ok(RetainedCloneStep::Progress(Default::default()))};self.source=Some(source);self.phase=Phase::Source;return Ok(RetainedCloneStep::Progress(receipt))
        }
        if self.phase==Phase::Source{
            let source=self.source.as_mut().ok_or_else(||compute_error("pixel source owner missing"))?;if source.geometry_ready()&&source.descriptor()?.protected{return Err(compute_error("pixel layer is protected"))}let step=source.advance(snapshot,grant)?;
            if matches!(step,RetainedCloneStep::Complete(_)){self.owned.as_mut().unwrap().before=source.take_pixels();self.phase=if let Some(spans)=selection(operation){let descriptor=source.descriptor()?;if spans.maximum_end()>u64::from(descriptor.width)*u64::from(descriptor.height){return Err(compute_error("pixel selection lies outside image"))}Phase::ReserveSelection}else{Self::operation_phase(operation)};}
            return Ok(RetainedCloneStep::Progress(step.progress()))
        }
        let count=self.count()?;
        let edit_budget=if self.phase==Phase::Edit{Some(self.edit_budget(operation,grant.maximum_copy_bytes)?.0)}else{None};
        let data=self.owned.as_mut().ok_or_else(||compute_error("pixel compute data missing"))?;
        let mut receipt=RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:demand.capacity_bytes,released_bytes:demand.release_bytes};
        match self.phase {
            Phase::ReserveSelection=>{let mut coverage=Vec::new();coverage.try_reserve_exact(count).map_err(|_|compute_error("pixel selection reservation failed"))?;receipt.retained_capacity_bytes=coverage.capacity();data.selection=Some(coverage);self.phase=Phase::Selection;}
            Phase::Selection=>{
                let spans=selection(operation).ok_or_else(||compute_error("pixel selection disappeared"))?;let coverage=data.selection.as_mut().ok_or_else(||compute_error("pixel selection storage missing"))?;
                if self.clip_span.is_none()&&self.selected_span<spans.len(){let bound=2+16*demand.depth.saturating_sub(1);if work_budget<bound{self.work_units=0;return Ok(RetainedCloneStep::Progress(Default::default()))}let(span,work)=spans.get_with_work(self.selected_span);self.work_units=work as usize;self.clip_span=Some(*span.ok_or_else(||compute_error("pixel selection rank is absent"))?);}
                else{let cursor=coverage.len();let(value,limit)=match self.clip_span{Some(span)if cursor<span.start as usize=>(0,span.start as usize),Some(span)=>(span.coverage as u8,span.start as usize+span.length as usize),None=>(0,count)};let end=(cursor+demand.copy_bytes.min(work_budget)).min(limit);coverage.resize(end,value);self.work_units=(end-cursor).max(1);receipt.copied_bytes=end-cursor;if end==limit&&self.clip_span.is_some()&&end>=self.clip_span.unwrap().start as usize+self.clip_span.unwrap().length as usize{self.clip_span=None;self.selected_span+=1;}}
                if coverage.len()==count{self.phase=Self::operation_phase(operation);}
            }
            Phase::ReservePoints=>{
                let RasterMutation::PaintStroke(value)=operation else{return Err(compute_error("stroke points disappeared"))};
                data.points.try_reserve_exact(value.points.len()).map_err(|_|compute_error("stroke points reservation failed"))?;
                receipt.retained_capacity_bytes=data.points.capacity()*size_of::<[f64;2]>();self.phase=Phase::Points;
            }
            Phase::Points=>{
                let RasterMutation::PaintStroke(value)=operation else{return Err(compute_error("stroke points disappeared"))};
                let end=(data.points.len()+(demand.copy_bytes/size_of::<[f64;2]>()).min(work_budget)).min(value.points.len());
                receipt.copied_bytes=0;
                self.work_units=0;for point in &value.points[data.points.len()..end]{self.work_units+=1;receipt.copied_bytes+=size_of::<[f64;2]>();if !point.x.is_finite()||!point.y.is_finite(){return Err(compute_error("stroke point is not finite").with_retained_progress(receipt));}data.points.push([point.x,point.y]);}
                if data.points.len()==value.points.len(){self.phase=Phase::Operation;}
            }
            Phase::Operation=>{
                if let RasterMutation::WritePixelRegion(value)=operation{let image=data.before.as_ref().ok_or_else(||compute_error("write source missing"))?;if u64::from(value.x)+u64::from(value.width)>u64::from(image.width)||u64::from(value.y)+u64::from(value.height)>u64::from(image.height){return Err(compute_error("written region lies outside image"))}self.phase=Phase::ReserveWrite;}
                else if let RasterMutation::FillRegion(value)=operation{if !self.flood_complete{let image=data.before.as_ref().ok_or_else(||compute_error("flood source missing"))?;if value.seed.x>=image.width||value.seed.y>=image.height{return Err(compute_error("flood seed lies outside image"))}data.flood_source=Some(std::sync::Arc::new(data.before.take().unwrap()));data.flood=Some(PixelFloodSelectionJob::new(data.flood_source.as_ref().unwrap().clone(),value.seed.x,value.seed.y,value.tolerance as u8).map_err(|_|compute_error("flood creation failed"))?);receipt.retained_capacity_bytes=data.flood.as_ref().unwrap().storage_capacity_bytes()+size_of::<RasterImage>()+2*size_of::<usize>();self.phase=Phase::ReserveFloodQueue;}else{data.operation=Some(fill_region::fill_operation(&value.target,&value.color));self.phase=Phase::AdmitEdit;}}
                else{let editing=match operation{RasterMutation::PaintStroke(value)=>paint_stroke::operation_with_points(value,std::mem::take(&mut data.points)),RasterMutation::ApplyFilter(value)=>apply_filter::operation(value),RasterMutation::TransformImage(value)=>{let image=data.before.as_ref().ok_or_else(||compute_error("transform source missing"))?;if value.operation=="crop"&&(u64::from(value.x)+u64::from(value.width)>u64::from(image.width)||u64::from(value.y)+u64::from(value.height)>u64::from(image.height)){return Err(compute_error("crop lies outside image"))}transform_image::operation(value)},RasterMutation::FillSelection(value)=>fill_region::fill_operation(&value.target,&value.color),_=>return Err(compute_error("pixel operation missing"))};data.operation=Some(editing);self.phase=Phase::AdmitEdit;}
            }
            Phase::ReserveFloodQueue=>{receipt.retained_capacity_bytes=data.flood.as_mut().ok_or_else(||compute_error("flood owner missing"))?.reserve_queue().map_err(|_|compute_error("flood queue reservation failed"))?;self.phase=Phase::Flood;}
            Phase::Flood=>{let budget=((grant.maximum_copy_bytes.min(32768)-5)/21).min(1024).min(work_budget);let flood=data.flood.as_mut().ok_or_else(||compute_error("flood owner missing"))?;let progress=flood.advance_selected(budget,data.selection.as_deref()).map_err(|_|compute_error("flood computation failed"))?;self.work_units=flood.last_work_units().max(1);receipt.copied_bytes=flood.last_write_bytes();if progress.done{self.phase=Phase::EndFlood;}}
            Phase::EndFlood=>{let storage=data.flood.take().ok_or_else(||compute_error("flood owner missing"))?.into_storage();data.seen=storage.seen;data.queue=storage.queue;drop(storage.source);let source=data.flood_source.take().ok_or_else(||compute_error("flood source missing"))?;match std::sync::Arc::try_unwrap(source){Ok(source)=>data.before=Some(source),Err(source)=>{data.flood_source=Some(source);return Err(compute_error("flood source retained another alias"))}}data.retired_selection=data.selection.take();data.selection=Some(storage.mask);self.flood_complete=true;self.phase=Phase::Operation;}
            Phase::AdmitEdit=>{let edit=PixelEditJob::admit(&mut data.before,&mut data.operation,&mut data.selection).map_err(|_|compute_error("pixel edit admission failed"))?;receipt.retained_capacity_bytes=edit.output.pixels.capacity();data.edit=Some(edit);self.phase=Phase::Scratch;}
            Phase::Scratch=>{receipt.retained_capacity_bytes=data.edit.as_mut().ok_or_else(||compute_error("pixel edit owner missing"))?.reserve_scratch().map_err(|_|compute_error("pixel scratch reservation failed"))?;self.phase=Phase::Edit;self.cursor=0;}
            Phase::Edit=>{let edit=data.edit.as_mut().ok_or_else(||compute_error("pixel edit owner missing"))?;let progress=edit.advance_controlled(edit_budget.unwrap(),demand.copy_bytes,work_budget.min(65536),&mut should_stop).map_err(|_|compute_error("pixel edit failed"))?;self.work_units=edit.last_work_units();receipt.copied_items=usize::from(self.work_units>0);receipt.copied_bytes=edit.last_write_bytes();self.cursor=progress.completed;if progress.done{self.phase=Phase::EndEdit;}}
            Phase::EndEdit=>{let storage=data.edit.take().ok_or_else(||compute_error("pixel edit owner missing"))?.into_storage();data.before=Some(storage.source);data.after=Some(storage.output);data.selection=storage.selection;data.points=storage.points;data.segments=storage.stroke_segments;self.phase=Phase::Done;}
            Phase::ReserveWrite=>{let image=data.before.as_ref().ok_or_else(||compute_error("write source missing"))?;let mut pixels=Vec::new();pixels.try_reserve_exact(image.pixels.len()).map_err(|_|compute_error("write output reservation failed"))?;receipt.retained_capacity_bytes=pixels.capacity();data.after=Some(RasterImage{width:image.width,height:image.height,pixels});self.phase=Phase::Write;}
            Phase::Write=>{let RasterMutation::WritePixelRegion(value)=operation else{return Err(compute_error("write operation disappeared"))};let before=data.before.as_ref().unwrap();let after=data.after.as_mut().unwrap();let end=after.pixels.len()+demand.copy_bytes.min(work_budget.saturating_mul(4));self.work_units=0;receipt.copied_bytes=0;while after.pixels.len()<end{let index=after.pixels.len()/4;let(x,y)=(index%before.width as usize,index/before.width as usize);let bytes=if x>=value.x as usize&&x<(value.x+value.width)as usize&&y>=value.y as usize&&y<(value.y+value.height)as usize{let local=((y-value.y as usize)*value.width as usize+x-value.x as usize)*4;&value.samples[local..local+4]}else{&before.pixels[index*4..index*4+4]};after.pixels.extend_from_slice(bytes);self.work_units+=1;receipt.copied_bytes+=4;}if after.pixels.len()==count*4{self.phase=Phase::Done;}}
            Phase::Done=>return Ok(RetainedCloneStep::Complete(Default::default())),
            Phase::Begin|Phase::Source=>unreachable!(),
        }
        if !receipt.fits(grant){return Err(compute_error("pixel computation exceeded its granted physical resources").with_retained_progress(receipt))}
        Ok(if self.phase==Phase::Done{RetainedCloneStep::Complete(receipt)}else{RetainedCloneStep::Progress(receipt)})
    }

    pub(super) fn take(&mut self)->Option<(RasterImage,RasterImage)>{if self.closing||self.phase!=Phase::Done{return None}let data=self.owned.as_mut()?;Some((data.before.take()?,data.after.take()?))}

    pub(super) fn close_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{if let Some(source)=self.source.as_ref(){return source.close_demands(body)}if let Some(active)=self.retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(active,body)}if self.owned.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.owned)}Ok(Default::default())}

    pub(super) fn close_step(&mut self,grant:RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome<RetainedCloneStep>{self.closing=true;if grant.maximum_items==0||grant.maximum_depth==0{return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}if let Some(source)=self.source.as_mut(){if source.terminal_is_empty(){self.source.take();return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,..Default::default()})),work:1}}return source.close_step(grant)}if self.retirement.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant)}else{store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.owned,&mut self.retirement,grant)}}
    pub(super) fn terminal_is_empty(&self)->bool{self.closing&&self.source.is_none()&&self.owned.is_none()&&self.retirement.is_none()}
}

impl Drop for RasterPixelComputeWork{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"pixel computation must hand off or retire every owner before Drop");}}
