//! 📬️ Granted config publication validates coverage by pages and exchanges owned fields into the cloned post.
use super::*;
use semio_framework_value::{RetirementDemand,RetainedCloneProgress,ValueError,ValueRefusalKind};
use store::snapshot_clone_preparation::config_apply::ConfigApplyMutation;
use std::mem::{size_of,swap};

const HEADER:usize=2048;
fn refusal()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"invalid retained raster configuration mutation")}
fn receipt()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,copied_bytes:HEADER,..Default::default()}}

impl ConfigApplyMutation<RasterConfig> for RasterConfigMutation {
    fn maximum_inverse_rows()->usize{2}
    fn publication_work_items(&self)->usize{<Self as protocol::Mutation<RasterConfig>>::inverse_rows(self)+1}
    fn payload_bytes(&self)->usize{match self{Self::SetPixelSelection(edit)=>edit.selection.as_ref().map_or(0,|selection|selection.layer_id.len()+selection.target.len()+size_of::<crate::selection::RasterSelectionSpans>()),Self::SetBrushColor(edit)=>edit.value.len(),Self::SetPaintTarget(edit)=>edit.value.len(),Self::SetFilterOptions(edit)=>edit.filter.len(),_=>size_of::<Self>()}}
    fn validation_demands(&self,_post:&RasterConfig,_cursor:usize,_body:usize)->Result<RetirementDemand,ValueError>{
        Ok(RetirementDemand{copy_bytes:HEADER,depth:1,..Default::default()})
    }
    fn validation_work_demand(&self,_post:&RasterConfig,_cursor:usize,_body:usize)->Result<u64,ValueError>{
        let Self::SetPixelSelection(edit)=self else{return Ok(1);};let Some(selection)=edit.selection.as_ref()else{return Ok(1);};
        Ok(1+selection.layer_id.len().min(257)as u64)
    }
    fn validate_step(&self,post:&RasterConfig,cursor:&mut usize,_body:usize)->semio_framework_value::RetainedWorkOutcome<(bool,RetainedCloneProgress)>{
        let mut work=1;let result=(||{
        let valid=match self{
            Self::SetBrushSize(edit)=>edit.value.is_finite()&&(1.0..=2048.0).contains(&edit.value),
            Self::SetBrushOpacity(edit)=>edit.value.is_finite()&&(0.0..=1.0).contains(&edit.value),
            Self::SetBrushHardness(edit)=>edit.value.is_finite()&&(0.0..=1.0).contains(&edit.value),
            Self::SetBrushColor(edit)=>valid_brush_color(&edit.value),
            Self::SetPaintTarget(edit)=>matches!(edit.value.as_str(),"pixels"|"mask"),
            Self::SetMaskValue(edit)=>edit.value<=255,
            Self::SetFillTolerance(edit)=>edit.value<=255,
            Self::SetFilterOptions(edit)=>edit.amount.is_finite()&&filter_range(&edit.filter).is_some_and(|range|range.is_none_or(|(min,max)|(min..=max).contains(&edit.amount))),
            Self::SetImageGrid(grid)=>grid.valid(),
            _=>true,
        };
        if !valid{return Err(refusal().with_retained_progress(receipt()));}
        let Self::SetPixelSelection(edit)=self else{return Ok((true,receipt()));};
        let Some(selection)=edit.selection.as_ref() else{return Ok((true,receipt()));};
        let count=semio_framework_pixels::editing::validate_extent(selection.width,selection.height).map_err(|_|refusal().with_retained_progress(receipt()))?;
        let characters=selection.layer_id.chars().take(257).count();work+=characters as u64;
        if selection.target!=post.paint_target||selection.layer_id.is_empty()||characters>256||selection.spans.maximum_end()>count as u64{return Err(refusal().with_retained_progress(receipt()));}
        *cursor=selection.spans.len();Ok((true,receipt()))
        })();semio_framework_value::RetainedWorkOutcome{result,work}
    }
    fn exchange(mut self,post:&mut RasterConfig)->Result<Self,(ValueError,Self)>{
        match &mut self{
            Self::SetBrushSize(edit)=>swap(&mut edit.value,&mut post.brush_size),
            Self::SetBrushOpacity(edit)=>swap(&mut edit.value,&mut post.brush_opacity),
            Self::SetBrushColor(edit)=>swap(&mut edit.value,&mut post.brush_color),
            Self::SetBrushHardness(edit)=>swap(&mut edit.value,&mut post.brush_hardness),
            Self::SetPaintTarget(edit)=>swap(&mut edit.value,&mut post.paint_target),
            Self::SetMaskValue(edit)=>swap(&mut edit.value,&mut post.mask_value),
            Self::SetFillTolerance(edit)=>swap(&mut edit.value,&mut post.fill_tolerance),
            Self::SetFilterOptions(edit)=>{swap(&mut edit.filter,&mut post.filter_kind);swap(&mut edit.amount,&mut post.filter_amount);},
            Self::SetImageGrid(grid)=>swap(grid,&mut post.image_grid),
            Self::SetPixelSelection(edit)=>swap(&mut edit.selection,&mut post.pixel_selection),
            Self::SetCompositeViewport(edit)=>swap(&mut edit.viewport,&mut post.composite_viewport),
            Self::SetCamera(edit)=>swap(&mut edit.camera,&mut post.camera),
        }
        Ok(self)
    }
    fn take_followup_inverse(&self,post:&mut RasterConfig)->Option<Self>{
        matches!(self,Self::SetPaintTarget(edit)if edit.value!=post.paint_target).then(||Self::SetPixelSelection(SetPixelSelectionEdit{selection:post.pixel_selection.take()}))
    }
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
