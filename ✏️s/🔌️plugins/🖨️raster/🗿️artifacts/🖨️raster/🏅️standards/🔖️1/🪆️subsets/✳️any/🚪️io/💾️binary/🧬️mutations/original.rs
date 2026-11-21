//! 📥️ The tagged Raster wire moves its decoded fields through original operation turns.

use super::COMPONENT_PROTOCOL_SEMIO;
use crate::{RasterMutation, text::{RasterMutationDsl, RasterMutationImageDsl, RasterMutationOctets, raster_mutation_from_dsl}};
use dsl::original_receiving::converted::OriginalConvertedOperation;
use semio_framework_value::{
    NativeDecodeControl, RetainedAllocationEffect, RetainedCloneProgress, RetireOwned,
    RetirementDemand, ValueError, ValueRefusalKind,
};
use semio_s_artifact_raster_raster_model::RasterImagePayload;
use semio_s_artifact_raster_raster_mutations_pixels::{
    add_layer_asset::AddLayerAsset as AddAsset, apply_filter::ApplyFilter as Filter, fill_region::{FillRegion as Fill, RasterSeed},
    fill_selection::FillSelection as SelectFill, paint_stroke::{PaintStroke as Stroke, RasterBrush, RasterStrokePoint},
    write_pixel_region::WritePixelRegion as PixelWrite,
};
use semio_s_artifact_stdio_semio_image::schema::snapshot::{
    SemioImageFrame, SemioImageMetadataEntry, SemioImageSnapshot,
};
use std::mem::{size_of, take};

#[path="selection.rs"]
mod selection;
use selection::Selection;

dsl::original_tagged_operation_receiving!(RasterMutationDsl, COMPONENT_PROTOCOL_SEMIO);
dsl::original_converted_operation_receiving!(RasterMutation);

#[derive(Default, RetireOwned)]
#[value(send_only)]
pub struct Conversion {
    frames: Vec<SemioImageFrame>,
    frame_pixels: Vec<u8>,
    image_icc: Vec<u8>,
    metadata: Vec<SemioImageMetadataEntry>,
    image: Option<SemioImageSnapshot>,
    points: Vec<RasterStrokePoint>,
    samples: Vec<u8>,
    sample_offset: usize,
    selection: Selection,
}

fn failed() -> ValueError { ValueError::invariant("Raster original conversion selected an absent field") }

fn fixed(copy_bytes: usize) -> RetirementDemand { RetirementDemand { copy_bytes, depth: 1, ..Default::default() } }

fn output_demand() -> RetirementDemand { fixed(size_of::<Option<RasterMutationDsl>>() + size_of::<Option<RasterMutation>>()) }

fn vec_demand<T>(rows: &Vec<T>, count: usize) -> Result<RetirementDemand,ValueError> {
    if rows.capacity() >= count {
        return Ok(fixed(size_of::<T>()));
    }
    let capacity_bytes = count.checked_mul(size_of::<T>()).ok_or_else(failed)?;
    Ok(RetirementDemand { capacity_bytes, ..fixed(size_of::<Vec<T>>()) })
}

fn reserve<T>(rows: &mut Vec<T>, count: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<usize,ValueError> {
    if effect.actual().is_some() || !rows.is_empty() || rows.capacity()!=0 { return Err(failed()); }
    *effect.actual()=Some(0);
    rows.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"Raster operation row allocation failed"))?;
    let bytes = rows.capacity().checked_mul(size_of::<T>()).ok_or_else(failed)?;
    *effect.actual()=Some(bytes);
    Ok(bytes)
}

fn copy_octets<'a>(output: &Vec<u8>, source: &'a RasterMutationOctets, body: usize) -> Result<&'a [u8], ValueError> {
    let page = source.page(output.len()).ok_or_else(failed)?;
    Ok(&page[..page.len().min(body.clamp(1, 4096))])
}

fn octets_demand(output: &Vec<u8>, source: &RasterMutationOctets, body: usize) -> Result<RetirementDemand, ValueError> {
    if output.capacity() < source.len() {
        return vec_demand(output, source.len());
    }
    Ok(fixed(copy_octets(output, source, body)?.len()))
}

fn append_octets(output: &mut Vec<u8>, source: &RasterMutationOctets, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> Result<usize, ValueError> {
    if output.capacity() < source.len() {
        return reserve(output, source.len(), effect);
    }
    output.extend_from_slice(copy_octets(output, source, body)?);
    Ok(0)
}

impl Conversion {
    fn pending_octets<'a>(&'a self, asset: &'a RasterMutationImageDsl) -> Option<(&'a Vec<u8>, &'a RasterMutationOctets)> {
        if let Some(frame) = asset.frames.get(self.frames.len()) {
            return (self.frames.capacity() >= asset.frames.len() && self.frame_pixels.len() < frame.rgba8.len())
                .then_some((&self.frame_pixels, &frame.rgba8));
        }
        if self.metadata.len() < asset.metadata.len() {
            return None;
        }
        asset.icc.as_ref().filter(|icc| self.image_icc.len() < icc.len()).map(|icc| (&self.image_icc, icc))
    }
}

fn stroke_count(xs: &[f64], ys: &[f64]) -> Result<usize,ValueError> {
    if xs.len()==ys.len() { Ok(xs.len()) }
    else { Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Raster stroke needs one x and y per point")) }
}

fn sample_count(samples: &str) -> Result<usize,ValueError> {
    if samples.len().is_multiple_of(2) { Ok(samples.len()/2) }
    else { Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Raster pixel sample hex has an incomplete byte")) }
}

fn selection_text(input: &RasterMutationDsl) -> Option<&str> {
    match input {
        RasterMutationDsl::PaintStroke { selection, .. }
        | RasterMutationDsl::FillRegion { selection, .. }
        | RasterMutationDsl::ApplyFilter { selection, .. }
        | RasterMutationDsl::FillSelection { selection, .. } => selection.as_deref(),
        _=>None,
    }
}

impl OriginalConvertedOperation for RasterMutation {
    type Mirror = RasterMutationDsl;
    type Conversion = Conversion;

    fn conversion_demand(state:&Conversion,input:&Option<RasterMutationDsl>,body:usize)->Result<RetirementDemand,ValueError>{
        use RasterMutationDsl::*;
        let input = input.as_ref().ok_or_else(failed)?;
        if let Some(source) = selection_text(input).filter(|_|!state.selection.ready()) {
            return state.selection.demand(source,body);
        }
        match input {
            AddLayerAsset { .. } if state.image.is_some() => {
                let mut demand = RasterImagePayload::owned_birth_demands();
                demand.copy_bytes = demand.copy_bytes.checked_add(output_demand().copy_bytes).ok_or_else(failed)?;
                Ok(demand)
            }
            AddLayerAsset { asset, .. } => {
                if let Some((output, source)) = state.pending_octets(asset) {
                    return octets_demand(output, source, body);
                }
                if state.frames.len() < asset.frames.len() {
                    return vec_demand(&state.frames, asset.frames.len());
                }
                if state.metadata.len() < asset.metadata.len() {
                    return vec_demand(&state.metadata, asset.metadata.len());
                }
                Ok(fixed(size_of::<Option<SemioImageSnapshot>>() + size_of::<Vec<SemioImageFrame>>() + size_of::<Vec<SemioImageMetadataEntry>>() + size_of::<Vec<u8>>()))
            }
            PaintStroke { xs, ys, .. } => {
                let count=stroke_count(xs,ys)?;
                if state.points.len()<count { vec_demand(&state.points,count) } else { Ok(output_demand()) }
            }
            WritePixelRegion { samples, .. } => {
                let count=sample_count(samples)?;
                if state.sample_offset<samples.len(){ vec_demand(&state.samples,count) } else { Ok(output_demand()) }
            }
            _=>Ok(output_demand()),
        }
    }

    fn conversion_work_demand(state:&Conversion,input:&Option<RasterMutationDsl>,_:usize)->Result<u64,ValueError>{
        let input=input.as_ref().ok_or_else(failed)?;
        if selection_text(input).is_some()&&!state.selection.ready(){
            return state.selection.work();
        }
        if let RasterMutationDsl::AddLayerAsset { asset, .. } = input {
            if state.image.is_some() {
                return Ok(RasterImagePayload::owned_birth_work());
            }
            if let Some((output, source)) = state.pending_octets(asset) {
                if output.capacity() >= source.len() {
                    return source.lookup_work(output.len())?.checked_add(1).ok_or_else(failed);
                }
            }
        }
        Ok(1)
    }

    fn convert_step(
        state:&mut Conversion,input:&mut Option<RasterMutationDsl>,output:&mut Option<Self>,body:usize,
        native:&mut NativeDecodeControl<'_>,effect:&mut RetainedAllocationEffect<'_>,
    )->Result<RetainedCloneProgress,ValueError>{
        use RasterMutationDsl::*;
        let demand=Self::conversion_demand(state,input,body)?;
        let work=Self::conversion_work_demand(state,input,body)?;
        let mutation=input.as_mut().ok_or_else(failed)?;
        if let Some(source)=selection_text(mutation).filter(|_|!state.selection.ready()){
            return state.selection.step(source,body,native,effect);
        }
        if let AddLayerAsset { asset_id, .. } = mutation {
            if state.image.is_some() {
                let Some((asset,mut progress))=RasterImagePayload::from_owned_in(&mut state.image,effect)? else {return Ok(Default::default());};
                *output=Some(Self::AddLayerAsset(AddAsset { asset_id:take(asset_id),asset }));
                progress.copied_bytes=demand.copy_bytes;
                return Ok(progress);
            }
        }
        if !effect.performed(work)? { return Ok(Default::default()); }
        let mut progress=RetainedCloneProgress {copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()};
        match mutation {
            AddLayerAsset { asset, .. } => {
                if let Some(frame) = asset.frames.get(state.frames.len()) {
                    if state.frames.capacity() < asset.frames.len() {
                        progress.retained_capacity_bytes = reserve(&mut state.frames, asset.frames.len(), effect)?;
                    } else if state.frame_pixels.len() < frame.rgba8.len() {
                        progress.retained_capacity_bytes = append_octets(&mut state.frame_pixels, &frame.rgba8, body, effect)?;
                    } else {
                        state.frames.push(SemioImageFrame { delay_ms: frame.delay_ms, rgba8: take(&mut state.frame_pixels) });
                    }
                } else if state.metadata.len() < asset.metadata.len() {
                    if state.metadata.capacity() < asset.metadata.len() {
                        progress.retained_capacity_bytes = reserve(&mut state.metadata, asset.metadata.len(), effect)?;
                    } else {
                        let entry = &mut asset.metadata[state.metadata.len()];
                        state.metadata.push(SemioImageMetadataEntry { key: take(&mut entry.key), value: take(&mut entry.value) });
                    }
                } else if let Some(icc) = asset.icc.as_ref().filter(|icc| state.image_icc.len() < icc.len()) {
                    progress.retained_capacity_bytes = append_octets(&mut state.image_icc, icc, body, effect)?;
                } else {
                    let icc = asset.icc.as_ref().map(|_| take(&mut state.image_icc));
                    state.image = Some(asset.take_snapshot(take(&mut state.frames), take(&mut state.metadata), icc));
                }
            }
            PaintStroke { layer_id,target,tool,size,hardness,opacity,color,xs,ys,.. } => {
                let count=stroke_count(xs,ys)?;
                let index=state.points.len();
                if demand.capacity_bytes!=0 {
                    progress.retained_capacity_bytes=reserve(&mut state.points,count,effect)?;
                } else if index<count {
                    state.points.push(RasterStrokePoint { x:xs[index],y:ys[index] });
                } else {
                    let brush=RasterBrush { size:*size,hardness:*hardness,opacity:*opacity,color:take(color) };
                    *output=Some(Self::PaintStroke(Stroke {
                        layer_id:take(layer_id),target:take(target),tool:take(tool),brush,
                        points:take(&mut state.points),selection:state.selection.take(),
                    }));
                }
            }
            FillRegion { layer_id,target,x,y,tolerance,color,.. } => {
                *output=Some(Self::FillRegion(Fill {
                    layer_id:take(layer_id),target:take(target),seed:RasterSeed {x:*x,y:*y},
                    tolerance:*tolerance,color:take(color),selection:state.selection.take(),
                }));
            }
            ApplyFilter { layer_id,filter,amount,.. } => {
                *output=Some(Self::ApplyFilter(Filter {
                    layer_id:take(layer_id),filter:take(filter),amount:*amount,selection:state.selection.take(),
                }));
            }
            FillSelection { layer_id,target,color,.. } => {
                *output=Some(Self::FillSelection(SelectFill {
                    layer_id:take(layer_id),target:take(target),color:take(color),selection:state.selection.take(),
                }));
            }
            WritePixelRegion { layer_id,target,x,y,width,height,samples } => {
                if demand.capacity_bytes!=0 {
                    progress.retained_capacity_bytes=reserve(&mut state.samples,sample_count(samples)?,effect)?;
                } else if state.sample_offset<samples.len() {
                    let at=state.sample_offset;
                    let byte=samples.get(at..at+2).and_then(|pair|u8::from_str_radix(pair,16).ok())
                        .ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"Raster pixel samples require hexadecimal bytes"))?;
                    state.samples.push(byte);
                    state.sample_offset+=2;
                } else {
                    *output=Some(Self::WritePixelRegion(PixelWrite {
                        layer_id:take(layer_id),target:take(target),x:*x,y:*y,width:*width,height:*height,
                        samples:take(&mut state.samples),
                    }));
                }
            }
            _=>{
                *output=Some(raster_mutation_from_dsl(input.take().ok_or_else(failed)?)
                    .map_err(|_|ValueError::literal(ValueRefusalKind::InvalidValue,"Raster operation conversion refused"))?);
            }
        }
        Ok(progress)
    }
}
