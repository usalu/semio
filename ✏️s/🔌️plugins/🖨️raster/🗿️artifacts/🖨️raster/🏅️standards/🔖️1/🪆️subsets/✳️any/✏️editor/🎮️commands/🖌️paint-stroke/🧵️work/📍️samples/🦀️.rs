//! 📍️ Incoming stroke columns become one owned typed point buffer through bounded validation and copies.
use super::*;
use semio_framework_value::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,RetirementDemand,ValueError,ValueRefusalKind,retained_clone::retained_clone_grant_admits};

#[derive(Default)]
pub(super) struct StrokeColumns{pub(super) points:Vec<RasterStrokePoint>,pub(super) cursor:usize,failed:bool}
fn fault()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"raster stroke requires matching finite point columns within its declared point limit")}
impl StrokeColumns{
    pub(super) fn reserve_demands(count:usize)->Result<RetirementDemand,ValueError>{if count>RASTER_STROKE_MAXIMUM_POINTS{return Err(fault());}Ok(RetirementDemand{copy_bytes:size_of::<Vec<RasterStrokePoint>>(),capacity_bytes:count*size_of::<RasterStrokePoint>(),depth:1,..Default::default()})}
    pub(super) fn reserve(&mut self,count:usize,grant:RetainedCloneGrant)->RetainedWorkOutcome{
        let mut work=0;let result=(||{let demand=Self::reserve_demands(count)?;if !retained_clone_grant_admits(grant,demand){return Ok(RetainedCloneStep::Progress(Default::default()));}if self.points.capacity()!=0||self.cursor!=0{return Err(fault());}work=1;self.points.try_reserve_exact(count).map_err(|_|ValueError::literal(ValueRefusalKind::AllocationFailed,"raster stroke point reservation failed"))?;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,retained_capacity_bytes:self.points.capacity()*size_of::<RasterStrokePoint>(),..Default::default()}))})();RetainedWorkOutcome{result,work}
    }
    pub(super) fn advance(&mut self,xs:&[f64],ys:&[f64],maximum_work:usize,grant:RetainedCloneGrant,continue_work:&mut dyn FnMut()->bool)->RetainedWorkOutcome<(bool,RetainedCloneProgress)>{
        let mut progress=RetainedCloneProgress::default();let mut work=0;let result=(||{if self.failed||xs.len()!=ys.len()||xs.len()>RASTER_STROKE_MAXIMUM_POINTS||self.points.capacity()<xs.len(){return Err(fault());}let limit=maximum_work.min(grant.maximum_copy_bytes/size_of::<RasterStrokePoint>());if grant.maximum_items==0||grant.maximum_depth==0{return Ok((false,progress));}while self.cursor<xs.len()&&work<limit as u64&&continue_work(){let point=RasterStrokePoint{x:xs[self.cursor],y:ys[self.cursor]};self.cursor+=1;work+=1;progress.copied_items=1;progress.copied_bytes+=size_of::<RasterStrokePoint>();if !point.x.is_finite()||!point.y.is_finite(){self.failed=true;return Err(fault().with_retained_progress(progress));}self.points.push(point);}Ok((self.cursor==xs.len(),progress))})();RetainedWorkOutcome{result,work}
    }
    pub(super) fn take_points(&mut self)->Vec<RasterStrokePoint>{std::mem::take(&mut self.points)}
}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
