//! 📤️ Original export custody covers raw composition, stored PNG, bounded base64 pages and funded close.
use semio_s_artifact_raster_raster_model::snapshot::RasterSnapshot;
use semio_s_artifact_raster_raster_io::RasterCompositeWork;
use semio_framework_pixels::{RasterImage,compositing::layers::RasterStackResult,png_encoding::PngEncodeWork};
use semio_framework_value::{OriginalRetirementFailure, OriginalRetirementOutcome, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkPoll, ValueError,ValueRefusalKind,RetirementDemand,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedWorkOutcome,retained_clone::{RetainedCloneClose,retained_clone_grant_admits}, RetirementPollExit};
use std::mem::{ManuallyDrop,size_of};
#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
struct ExportData{raw:Option<RasterStackResult>,encoded:Vec<u8>,page:Option<Vec<u8>>}
pub struct ImageExportWork{composite:ManuallyDrop<Option<RasterCompositeWork>>,png:ManuallyDrop<Option<PngEncodeWork<RasterImage>>>,data:ManuallyDrop<Option<ExportData>>,retirement:RetainedCloneClose,phase:u8,cursor:usize,document:usize,pub done:bool,pub closing:bool,failed:bool}
impl Default for ImageExportWork{fn default()->Self{Self{composite:ManuallyDrop::new(None),png:ManuallyDrop::new(None),data:ManuallyDrop::new(Some(ExportData{raw:None,encoded:Vec::new(),page:None})),retirement:Default::default(),phase:0,cursor:0,document:0,done:false,closing:false,failed:false}}}
fn fault(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}
impl ImageExportWork{
 pub fn stage(&self)->&'static str{match self.phase{0=>"prepare",1..=3=>"composite",4..=7=>"encode",_=>"output"}}
 pub fn page_ready(&self)->bool{!self.closing&&!self.failed&&self.phase==10}
 fn page_input(&self)->usize{(self.data.as_ref().unwrap().encoded.len()-self.cursor).min(super::BASE64_INPUT_BYTES)}
 pub fn demands(&self) -> Result<RetirementDemand, ValueError> {
    if self.closing || self.failed || self.done { return Err(fault("raster.export-inactive")); }
    let item = RetirementDemand { depth: 1, ..Default::default() };
    match self.phase {
        1 if !self.composite.as_ref().unwrap().output_ready() => self.composite.as_ref().unwrap().demands(512),
        2 => Ok(RetirementDemand { copy_bytes: size_of::<RasterStackResult>(), ..item }),
        3 => self.composite.as_ref().unwrap().close_demands(512),
        5 if !self.png.as_ref().unwrap().output_ready() => self.png.as_ref().unwrap().demands(512),
        6 | 10 => Ok(RetirementDemand { copy_bytes: size_of::<Vec<u8>>(), ..item }),
        7 => self.png.as_ref().unwrap().close_demands(512),
        8 => Ok(RetirementDemand { capacity_bytes: self.page_input().div_ceil(3) * 4, ..item }),
        9 => Ok(RetirementDemand { copy_bytes: self.page_input() + self.page_input().div_ceil(3) * 4, ..item }),
        _ => Ok(item),
    }
 }

 pub fn advance(&mut self, document: &RasterSnapshot, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementOutcome {
    let start = effect.performed_work();
    let result = self.advance_step(document, effect);
    if result.is_err() { self.failed = true; }
    OriginalRetirementOutcome { result, work: effect.performed_work() - start }
 }

 fn advance_step(&mut self, document: &RasterSnapshot, effect: &mut RetainedAllocationEffect<'_>) -> Result<RetainedCloneStep, OriginalRetirementFailure> {
    let demand = self.demands()?;
    let grant = effect.grant();
    let idle = RetainedCloneStep::Progress(Default::default());
    let remaining = effect.remaining_work_units()?;
    if remaining == 0 || !retained_clone_grant_admits(grant, demand) { return Ok(idle); }
    if self.document != 0 && self.document != document as *const RasterSnapshot as usize {
        return Err(fault("raster.export-source-revision-changed").into());
    }
    let mut progress = RetainedCloneProgress { copied_items: 1, ..Default::default() };
    match self.phase {
        0 => {
            super::pay_export_work(effect, 1)?;
            let composite = RasterCompositeWork::new(document)?;
            self.document = document as *const RasterSnapshot as usize;
            *self.composite = Some(composite);
            self.phase = 1;
        }
        1 => {
            let composite = self.composite.as_mut().unwrap();
            if !composite.output_ready() {
                return composite.advance(document, 512, effect).result.map_err(Into::into);
            }
            super::pay_export_work(effect, 1)?;
            self.phase = 2;
        }
        2 => {
            let composite = self.composite.as_mut().unwrap();
            let turn = composite.take_result(grant);
            super::pay_export_work(effect, turn.work)?;
            let Some((raw, receipt)) = turn.result? else { return Ok(idle); };
            self.data.as_mut().unwrap().raw = Some(raw);
            progress = RetainedCloneProgress { copied_bytes: demand.copy_bytes, ..receipt };
            composite.cancel();
            self.phase = 3;
        }
        3 => {
            let composite = self.composite.as_mut().unwrap();
            let poll = composite.close_step(512, effect);
            let step = match poll {
                RetainedWorkPoll::Pending { .. } => return Ok(idle),
                RetainedWorkPoll::Complete(turn) => turn.result?,
            };
            progress = step.progress();
            if composite.terminal_is_empty() {
                self.composite.take();
                self.phase = 4;
            }
        }
        4 => {
            super::pay_export_work(effect, 1)?;
            let raw = self.data.as_mut().unwrap().raw.take().unwrap();
            if raw.empty {
                self.data.as_mut().unwrap().raw = Some(raw);
                return Err(fault("raster.export-no-visible-pixels").into());
            }
            match PngEncodeWork::new(raw.image, 67_108_864) {
                Ok(png) => { *self.png = Some(png); self.phase = 5; }
                Err((error, image)) => {
                    self.data.as_mut().unwrap().raw = Some(RasterStackResult { image, origin: raw.origin, empty: raw.empty });
                    return Err(error.into());
                }
            }
        }
        5 => {
            let png = self.png.as_mut().unwrap();
            if !png.output_ready() {
                let body = 512.min(usize::try_from(remaining).unwrap_or(usize::MAX));
                let quote = png.demands(body)?;
                if u128::from(remaining) < quote.copy_bytes.max(1) as u128 { return Ok(idle); }
                let turn = png.advance(body, grant);
                super::pay_export_work(effect, turn.work)?;
                return turn.result.map_err(Into::into);
            }
            super::pay_export_work(effect, 1)?;
            self.phase = 6;
        }
        6 => {
            let png = self.png.as_mut().unwrap();
            let turn = png.take_output(grant);
            super::pay_export_work(effect, turn.work)?;
            let Some((bytes, receipt)) = turn.result? else { return Ok(idle); };
            self.data.as_mut().unwrap().encoded = bytes;
            progress = receipt;
            png.cancel();
            self.phase = 7;
        }
        7 => {
            let png = self.png.as_mut().unwrap();
            let poll = png.close_step(512, effect);
            let step = match poll {
                RetainedWorkPoll::Pending { .. } => return Ok(idle),
                RetainedWorkPoll::Complete(turn) => turn.result?,
            };
            progress = step.progress();
            if png.terminal_is_empty() {
                self.png.take();
                self.phase = 8;
            }
        }
        8 => {
            super::pay_export_work(effect, 1)?;
            if effect.actual().is_some() { return Err(fault("raster.export-allocation-marker-occupied").into()); }
            let mut page = Vec::new();
            *effect.actual() = Some(0);
            page.try_reserve_exact(demand.capacity_bytes).map_err(|_| fault("raster.export-page-allocation-refused"))?;
            *effect.actual() = Some(page.capacity());
            progress.retained_capacity_bytes = page.capacity();
            self.data.as_mut().unwrap().page = Some(page);
            self.phase = 9;
        }
        9 => {
            const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
            let count = self.page_input();
            let work = count.div_ceil(3) as u128;
            if u128::from(remaining) < work { return Ok(idle); }
            super::pay_export_work(effect, work)?;
            let data = self.data.as_mut().unwrap();
            let page = data.page.as_mut().unwrap();
            for chunk in data.encoded[self.cursor..self.cursor + count].chunks(3) {
                let a = chunk[0];
                let b = chunk.get(1).copied().unwrap_or(0);
                let c = chunk.get(2).copied().unwrap_or(0);
                page.extend_from_slice(&[
                    ALPHABET[(a >> 2) as usize],
                    ALPHABET[(((a & 3) << 4) | (b >> 4)) as usize],
                    if chunk.len() > 1 { ALPHABET[(((b & 15) << 2) | (c >> 6)) as usize] } else { b'=' },
                    if chunk.len() > 2 { ALPHABET[(c & 63) as usize] } else { b'=' },
                ]);
            }
            progress.copied_bytes = count + page.len();
            self.cursor += count;
            self.phase = 10;
        }
        10 => return Ok(idle),
        _ => return Err(fault("raster.export-phase").into()),
    }
    Ok(RetainedCloneStep::Progress(progress))
 }
 pub fn take_page(&mut self,grant:RetainedCloneGrant)->RetainedWorkOutcome<Option<(Vec<u8>,RetainedCloneProgress)>>{if !self.page_ready(){return RetainedWorkOutcome{result:Err(fault("raster.export-page-unavailable")),work:0}}let bytes=size_of::<Vec<u8>>();if grant.maximum_items==0||grant.maximum_depth==0||grant.maximum_copy_bytes<bytes{return RetainedWorkOutcome{result:Ok(None),work:0}}let data=self.data.as_mut().unwrap();let page=data.page.take().unwrap();self.done=self.cursor==data.encoded.len();self.phase=if self.done{11}else{8};RetainedWorkOutcome{result:Ok(Some((page,RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}))),work:1}}
 pub fn cancel(&mut self){self.closing=true;if let Some(composite)=self.composite.as_mut(){composite.cancel()}if let Some(png)=self.png.as_mut(){png.cancel()}}
 pub fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{if let Some(composite)=self.composite.as_ref(){return composite.close_demands(body)}if let Some(png)=self.png.as_ref(){return png.close_demands(body)}self.retirement.next_owner_demands::<ExportData>(self.data.is_some(),&None,body)}
 pub fn close_step(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> OriginalRetirementPoll {
    let result = (|| -> Result<RetainedCloneStep, RetirementPollExit> {
        if !self.closing { return Err(fault("raster.export-close-intent-required").into()); }
        if let Some(composite) = self.composite.as_mut() {
            let step = RetirementPollExit::resume(composite.close_step(body, effect))?;
            if composite.terminal_is_empty() { self.composite.take(); }
            return Ok(step);
        }
        if let Some(png) = self.png.as_mut() {
            let step = RetirementPollExit::resume(png.close_step(body, effect))?;
            if png.terminal_is_empty() { self.png.take(); }
            return Ok(step);
        }
        if !self.retirement.is_empty() { return RetirementPollExit::resume(self.retirement.step_granted(body, effect)); }
        let turn = self.retirement.begin_granted(&mut self.data, effect);
        Ok(turn.result?.unwrap_or(RetainedCloneStep::Complete(Default::default())))
    })();
    RetirementPollExit::settle(effect, result)
 }
 pub fn terminal_is_empty(&self)->bool{self.composite.is_none()&&self.png.is_none()&&self.data.is_none()&&self.retirement.is_empty()}
}
impl Drop for ImageExportWork{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"export private roots require funded close");if self.terminal_is_empty(){unsafe{ManuallyDrop::drop(&mut self.composite);ManuallyDrop::drop(&mut self.png);ManuallyDrop::drop(&mut self.data)}}}}
