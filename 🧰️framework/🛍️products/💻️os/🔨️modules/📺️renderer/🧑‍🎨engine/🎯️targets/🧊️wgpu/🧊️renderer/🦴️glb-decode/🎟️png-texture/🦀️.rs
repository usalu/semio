//! 🎟️ Embedded PNG textures retain source and pixels on the initiating renderer caller.
use super::*;
use semio_framework_job::{StepContext, OriginalTurnFailure, close_original_effect_step, close_original_retained_effect_step, receive_original_retained_work};
use semio_framework_value::{RetainedCloneGrant, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetainedWorkPoll, RetirementDemand, ValueError, ValueRefusalKind, retained_clone::RetainedCloneClose, retirement::RetireOwned};
use semio_framework_pixels::{RasterImage, png_decoding::{PngDecodeInput, PngDecodeJob, PngDecodeError}};
use std::mem::{size_of, size_of_val};

#[derive(Default)]
pub(super) struct OriginalPngTexture {
    job: Option<PngDecodeJob>,
    image: Option<RasterImage>,
    retirement: RetainedCloneClose,
    binding: Option<semio_framework_job::OriginalSourceBinding>,
}

fn refused(error: PngDecodeError) -> ValueError {
    let (kind, message) = match error {
        PngDecodeError::Invalid(message) => (ValueRefusalKind::InvalidValue, message),
        PngDecodeError::Incomplete => (ValueRefusalKind::InvariantViolated, "PNG decode is incomplete"),
        PngDecodeError::Cancelled => (ValueRefusalKind::Canceled, "PNG decode cancelled"),
        PngDecodeError::WorkLimit => (ValueRefusalKind::WorkLimit, "PNG decode exceeded original Work"),
        PngDecodeError::AllocationFailed => (ValueRefusalKind::AllocationFailed, "PNG decode allocation failed"),
    };
    ValueError::literal(kind, message)
}

fn invalid() -> ValueError {
    ValueError::invariant("original renderer PNG owner or output is absent")
}

fn transfer(cx: &mut StepContext<'_>, bytes: usize, move_value: impl FnOnce(RetainedCloneGrant) -> Result<(), ValueError>) -> Result<bool, OriginalTurnFailure> {
    let demand = RetirementDemand { copy_bytes: bytes, depth: 1, ..Default::default() };
    let step = close_original_retained_effect_step(cx, demand, |effect| -> Result<_, ValueError> {
        if effect.remaining_work_units()? == 0 || !effect.performed(1)? {
            return Ok(RetainedCloneStep::Progress(Default::default()));
        }
        let progress = RetainedCloneProgress { copied_items: 1, copied_bytes: bytes, ..Default::default() };
        move_value(effect.grant()).map_err(|error| error.with_retained_progress(progress))?;
        Ok(RetainedCloneStep::Complete(progress))
    })?;
    Ok(step.progress().copied_items != 0)
}

fn retire_resident<T: RetireOwned>(cx: &mut StepContext<'_>, retirement: &mut RetainedCloneClose, value: &mut Option<T>) -> Result<(), OriginalTurnFailure> {
    let body = cx.retained_grant().maximum_copy_bytes;
    let demand = retirement.next_owner_demands::<T>(value.is_some(), &None, body)?;
    if cx.fuel_remaining() < retirement.next_owner_work::<T>(value.is_some(), &None, body)? {
        return Ok(());
    }
    close_original_effect_step(cx, demand, |effect| {
        if !retirement.is_empty() {
            return retirement.step_granted(body, effect);
        }
        let turn = retirement.begin_granted(value, effect);
        RetainedWorkPoll::Complete(RetainedWorkOutcome {
            work: turn.work,
            result: turn.result.map_err(Into::into).map(|step| step.unwrap_or(RetainedCloneStep::Complete(Default::default()))),
        })
    })?;
    Ok(())
}

impl OriginalPngTexture {
    pub(super) fn source(&self) -> Option<&[u8]> {
        self.job.as_ref().map(PngDecodeJob::source)
    }

    pub(super) fn pending(&self) -> bool {
        self.job.is_some() || self.image.is_some() || !self.retirement.is_empty() || self.binding.is_some()
    }

    fn validate(&self, cx: &StepContext<'_>) -> Result<(), ValueError> {
        if self.binding.is_some_and(|binding| !binding.matches(cx)) {
            return Err(ValueError::invariant("PNG texture belongs to another original caller"));
        }
        Ok(())
    }

    pub(super) fn admit(&mut self, source: &mut Vec<u8>, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if self.job.is_some() {
            return if source.is_empty() { Ok(true) } else { Err(ValueError::invariant("original PNG input receiver is occupied").into()) };
        }
        if self.pending() { return Err(invalid().into()); }
        if cx.is_cancelled() || cx.should_yield() { return Ok(false); }

        let original = semio_framework_job::OriginalSourceBinding::borrow(cx);
        let bytes = 2 * size_of::<Vec<u8>>() + size_of::<Option<PngDecodeJob>>() + size_of_val(&self.binding);
        transfer(cx, bytes, |_| {
            let input = PngDecodeInput {
                data: std::mem::take(source),
                max_pixels: 16_777_216,
                max_bytes: 16_000_000,
                max_chunks: 65_536,
            };
            match PngDecodeJob::new(input) {
                Ok(job) => {
                    self.job = Some(job);
                    self.binding = Some(original);
                    Ok(())
                }
                Err((error, input)) => {
                    *source = input.data;
                    Err(refused(error))
                }
            }
        })
    }

    pub(super) fn advance(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        cx.original_source_parent()?.validate_borrowed_context(cx)?;
        if self.image.is_some() { return Ok(true); }
        let job = self.job.as_mut().ok_or_else(invalid)?;
        if job.progress().done { return Ok(true); }
        if cx.is_cancelled() || cx.should_yield() { return Ok(false); }

        let mut grant = cx.retained_grant();
        grant.maximum_items = grant.maximum_items.min(usize::try_from(cx.fuel_remaining()).unwrap_or(usize::MAX));
        if grant.maximum_items == 0
            || grant.maximum_depth < job.next_depth_demand().map_err(refused)?
            || grant.maximum_copy_bytes < job.next_copy_byte_demand().map_err(refused)?
            || grant.maximum_capacity_bytes < job.next_capacity_byte_demand().map_err(refused)? {
            return Ok(false);
        }
        let Some((result, work)) = cx.with_original_effect(grant, |_, effect| {
            let result = job.advance(effect);
            let progress = job.last_turn_progress();
            result.map(|(state, _)| {
                if state.done { RetainedCloneStep::Complete(progress) } else { RetainedCloneStep::Progress(progress) }
            }).map_err(|error| refused(error).with_retained_progress(progress))
        })? else { return Ok(false); };
        receive_original_retained_work(cx, RetainedWorkOutcome { work, result })?;
        Ok(job.progress().done)
    }

    pub(super) fn take_image(&mut self, output: &mut Option<DecodedReferenceImage>, digest: [u64; 2], cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        if output.is_some() { return Err(ValueError::invariant("original PNG output receiver is occupied").into()); }
        if cx.is_cancelled() || cx.should_yield() { return Ok(false); }
        if self.image.is_none() {
            let job = self.job.as_mut().ok_or_else(invalid)?;
            if !job.progress().done { return Ok(false); }
            let bytes = size_of::<RasterImage>() + size_of::<Option<RasterImage>>();
            transfer(cx, bytes, |grant| {
                let (image, _) = job.take_result(grant).map_err(refused)?.ok_or_else(invalid)?;
                self.image = Some(image);
                Ok(())
            })?;
            return Ok(false);
        }
        if self.job.is_some() || !self.retirement.is_empty() {
            retire_resident(cx, &mut self.retirement, &mut self.job)?;
            return Ok(false);
        }

        let bytes = size_of::<Option<RasterImage>>() + size_of::<DecodedReferenceImage>() + size_of_val(&self.binding);
        transfer(cx, bytes, |_| {
            let image = self.image.take().ok_or_else(invalid)?;
            *output = Some(DecodedReferenceImage { width: image.width, height: image.height, source_digest: digest, pixels: image.pixels });
            self.binding = None;
            Ok(())
        })
    }

    pub(super) fn close(&mut self, cx: &mut StepContext<'_>) -> Result<bool, OriginalTurnFailure> {
        self.validate(cx)?;
        cx.with_original_retirement(|cx| {
            if self.job.is_some() || !self.retirement.is_empty() {
                retire_resident(cx, &mut self.retirement, &mut self.job)?;
            } else if self.image.is_some() {
                retire_resident(cx, &mut self.retirement, &mut self.image)?;
            } else if self.binding.is_some() {
                transfer(cx, size_of_val(&self.binding), |_| {
                    self.binding = None;
                    Ok(())
                })?;
            }
            Ok(!self.pending())
        })
    }

    pub(super) fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if self.job.is_some() || !self.retirement.is_empty() {
            return self.retirement.next_owner_demands::<PngDecodeJob>(self.job.is_some(), &None, body);
        }
        if self.image.is_some() {
            return self.retirement.next_owner_demands::<RasterImage>(true, &None, body);
        }
        Ok(if self.binding.is_some() {
            RetirementDemand { copy_bytes: size_of_val(&self.binding), depth: 1, ..Default::default() }
        } else {
            Default::default()
        })
    }
}

impl Drop for OriginalPngTexture {
    fn drop(&mut self) {
        assert!(std::thread::panicking() || !self.pending(), "original PNG texture requires paid retirement");
    }
}

#[cfg(test)]
mod tests{
 use super::*;
 use semio_framework_job::{OriginalJobFixtureCaller,OriginalJobDecodeOwner,OperationId,Generation};
 fn stock()->RetainedCloneGrant{RetainedCloneGrant{maximum_items:4096,maximum_copy_bytes:1<<20,maximum_capacity_bytes:1<<25,maximum_release_bytes:1<<25,maximum_depth:256}}
 fn physical<T>(cx:&mut StepContext<'_>,step:impl FnOnce(&mut StepContext<'_>)->Result<T,OriginalTurnFailure>)->Result<T,OriginalTurnFailure>{
  let before=cx.retained_progress();let(result,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||step(cx));let after=cx.retained_progress();
  assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(after.retained_capacity_bytes-before.retained_capacity_bytes,after.released_bytes-before.released_bytes));result
 }
 fn close_decoder(caller:&mut OriginalJobFixtureCaller,decoder:&mut OriginalJobDecodeOwner){while !decoder.terminal_is_empty(){caller.turn(stock(),|cx|{decoder.close_on_source(cx)?;Ok(())}).unwrap();}}
 #[test]
 fn original_png_texture_heap_witness_observes_real_allocations(){
  let(value,heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||std::hint::black_box(vec![3u8;173]));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(173,0));
  let((),heap)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(value));assert!(!heap.overflowed);assert_eq!((heap.requested_bytes,heap.released_bytes),(0,173));
 }
 #[test]
 fn original_png_texture_restores_owned_input_on_invalid_signature() {
    let mut bytes = vec![0; 8];
    let original = bytes.as_ptr();
    let mut caller = OriginalJobFixtureCaller::admit(OperationId(8715), Generation(2), stock(), || Some(0), 8000, 1024);
    let mut png = OriginalPngTexture::default();
    assert!(caller.bare_turn(stock(), |cx| physical(cx, |cx| png.admit(&mut bytes, cx))).is_err());
    assert_eq!(bytes.as_ptr(), original);
    assert_eq!(bytes, vec![0; 8]);
    assert!(!png.pending());
    assert_eq!(caller.receipt().copied_items, 1);
    caller.retire(stock());
 }
 #[test]
 fn original_png_texture_denied_handoffs_keep_private_pixels_until_cancelled_close(){
  let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/cases.json")).unwrap();
  for stage in law["custodyStages"].as_array().unwrap(){
   let mut bytes=semio_framework_io_base64::base64_standard_decode(law["data"].as_str().unwrap().as_bytes()).unwrap();let source_pointer=bytes.as_ptr();
   let mut caller=OriginalJobFixtureCaller::admit(OperationId(8714),Generation(2),stock(),||Some(0),8000,1<<28);let mut decoder=OriginalJobDecodeOwner::empty();let mut png=OriginalPngTexture::default();let mut output=None;
   caller.turn_with_decoder(stock(),&mut decoder,|cx|physical(cx,|cx|{assert!(png.admit(&mut bytes,cx)?);Ok(())})).unwrap();
   for turn in 0..10000{if caller.turn_with_decoder(stock(),&mut decoder,|cx|physical(cx,|cx|png.advance(cx))).unwrap(){break}assert!(turn<9999);}
   let pixel_pointer=png.job.as_ref().unwrap().result().unwrap().pixels.as_ptr();
   let name=stage["name"].as_str().unwrap();
   if name!="decoded"{caller.turn_with_decoder(stock(),&mut decoder,|cx|physical(cx,|cx|{assert!(!png.take_image(&mut output,[7,11],cx)?);Ok(())})).unwrap();assert!(output.is_none());assert_eq!(png.image.as_ref().unwrap().pixels.as_ptr(),pixel_pointer);}
   if name=="retiredDecoder"{for turn in 0..10000{if png.job.is_none()&&png.retirement.is_empty(){break}caller.turn_with_decoder(stock(),&mut decoder,|cx|physical(cx,|cx|{assert!(!png.take_image(&mut output,[7,11],cx)?);Ok(())})).unwrap();assert!(turn<9999);}assert!(png.source().is_none());}
   for axis in stage["denials"].as_array().unwrap(){
    let mut grant=stock();match axis.as_str().unwrap(){"items"=>grant.maximum_items=0,"copy"=>grant.maximum_copy_bytes=0,"capacity"=>grant.maximum_capacity_bytes=0,"depth"=>grant.maximum_depth=0,_=>{}}
    let before=caller.cumulative_receipt();let fuel=caller.fuel_remaining();
    caller.turn_with_decoder(grant,&mut decoder,|cx|physical(cx,|cx|{if axis=="work"{let deadline=cx.deadline_us();cx.with_original_work_loan(0,deadline,|cx|{assert!(!png.take_image(&mut output,[7,11],cx)?);Ok(())})?;}else{assert!(!png.take_image(&mut output,[7,11],cx)?);}Ok(())})).unwrap();
    assert_eq!(caller.cumulative_receipt(),before);assert_eq!(caller.fuel_remaining(),fuel);assert!(output.is_none());assert!(png.pending());
    if name=="decoded"{assert_eq!(png.job.as_ref().unwrap().result().unwrap().pixels.as_ptr(),pixel_pointer);}else{assert_eq!(png.image.as_ref().unwrap().pixels.as_ptr(),pixel_pointer);}
    if name!="retiredDecoder"{assert_eq!(png.source().unwrap().as_ptr(),source_pointer);}
   }
   let mut other=OriginalJobFixtureCaller::admit(OperationId(8714),Generation(2),stock(),||Some(0),8000,1<<28);
   assert!(other.turn_with_decoder(stock(),&mut decoder,|cx|physical(cx,|cx|png.take_image(&mut output,[7,11],cx))).is_err());assert_eq!(other.receipt(),Default::default());assert!(output.is_none());assert!(png.pending());other.retire(stock());
   caller.cancel().cancel_now();
   for turn in 0..10000{if !png.pending(){break}caller.turn_with_decoder(stock(),&mut decoder,|cx|physical(cx,|cx|{assert!(cx.is_cancelled());png.close(cx)?;Ok(())})).unwrap();assert!(turn<9999);}
   assert!(!png.pending());assert!(output.is_none());assert!(png.source().is_none());assert!(png.image.is_none());close_decoder(&mut caller,&mut decoder);caller.retire(stock());
  }
 }
 #[test]
 fn original_png_texture_keeps_source_on_denial_and_retires_pixels_after_transfer(){
  let law:serde_json::Value=serde_json::from_str(include_str!("🧪️tests/cases.json")).unwrap();let mut bytes=semio_framework_io_base64::base64_standard_decode(law["data"].as_str().unwrap().as_bytes()).unwrap();let pointer=bytes.as_ptr();
  let mut caller=OriginalJobFixtureCaller::admit(OperationId(8713),Generation(2),stock(),||Some(0),8000,1<<28);let mut decoder=OriginalJobDecodeOwner::empty();let mut png=OriginalPngTexture::default();
  for axis in law["denials"].as_array().unwrap(){let mut grant=stock();match axis.as_str().unwrap(){"items"=>grant.maximum_items=0,"copy"=>grant.maximum_copy_bytes=0,"depth"=>grant.maximum_depth=0,_=>{}}
   caller.turn_with_decoder(grant,&mut decoder,|cx|{if axis=="work"{let deadline=cx.deadline_us();cx.with_original_work_loan(0,deadline,|cx|{assert!(!png.admit(&mut bytes,cx)?);Ok(())})?;}else{assert!(!png.admit(&mut bytes,cx)?);}Ok(())}).unwrap();
   assert_eq!(bytes.as_ptr(),pointer);assert!(!bytes.is_empty());assert!(!png.pending());
  }
  caller.turn_with_decoder(stock(),&mut decoder,|cx|{assert!(png.admit(&mut bytes,cx)?);Ok(())}).unwrap();assert!(bytes.is_empty());assert_eq!(png.job.as_ref().unwrap().source().as_ptr(),pointer);
  for turn in 0..10000{let ready=caller.turn_with_decoder(stock(),&mut decoder,|cx|png.advance(cx)).unwrap();if ready{break}assert!(turn<9999);}
  let pixel_pointer=png.job.as_ref().unwrap().result().unwrap().pixels.as_ptr();
  let mut output=None;for turn in 0..10000{let complete=caller.turn_with_decoder(stock(),&mut decoder,|cx|png.take_image(&mut output,[7,11],cx)).unwrap();if complete{break}assert!(turn<9999);}assert!(!png.pending());assert_eq!(output.as_ref().unwrap().pixels.as_ptr(),pixel_pointer);
  let image=output.as_ref().unwrap();assert_eq!((image.width,image.height),(law["width"].as_u64().unwrap()as u32,law["height"].as_u64().unwrap()as u32));assert_eq!(image.pixels,law["rgba"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as u8).collect::<Vec<_>>());
  while png.pending(){caller.turn(stock(),|cx|{png.close(cx)?;Ok(())}).unwrap();}
  let mut pixels = None;
  caller.bare_turn(stock(), |cx| {
    let bytes = size_of::<DecodedReferenceImage>() + size_of::<Option<Vec<u8>>>();
    assert!(transfer(cx, bytes, |_| {
        pixels = Some(output.take().unwrap().pixels);
        Ok(())
    })?);
    Ok(())
  }).unwrap();
  let mut retirement=RetainedCloneClose::default();
  while pixels.is_some()||!retirement.is_empty(){
   caller.bare_turn(stock(),|cx|retire_resident(cx,&mut retirement,&mut pixels)).unwrap();
  }
  close_decoder(&mut caller,&mut decoder);caller.retire(stock());
 }
}
