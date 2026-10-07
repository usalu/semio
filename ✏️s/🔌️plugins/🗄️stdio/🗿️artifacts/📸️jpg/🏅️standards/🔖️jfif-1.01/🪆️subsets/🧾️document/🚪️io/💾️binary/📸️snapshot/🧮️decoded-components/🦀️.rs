//! 🧮️ Controlled native JPEG component projection before color conversion.

use super::super::super::{JpgByteSource,JpgStepDecoder,parse_jpg_header};
use semio_framework_value::{ValueError,ValueRefusalKind};
use std::cell::{Cell,RefCell};

/// 🧊️ Exact native component order with full-resolution rounded and clamped samples.
#[derive(Debug,PartialEq,Eq)]
pub struct JpgDecodedComponents {
    pub width:u32,
    pub height:u32,
    pub component_ids:Vec<u8>,
    pub samples:Vec<u8>,
}

struct CheckedSource<'a> {source:&'a dyn JpgByteSource,cancel:RefCell<&'a mut dyn FnMut()->bool>,stopped:Cell<bool>}
impl JpgByteSource for CheckedSource<'_> {
    fn len(&self)->usize {self.source.len()}
    fn byte(&self,index:usize)->Option<u8> {
        if self.stopped.get() || (self.cancel.borrow_mut())() {self.stopped.set(true);None} else {self.source.byte(index)}
    }
}

fn refusal(kind:ValueRefusalKind,message:&str)->ValueError {ValueError::new(kind,message)}

/// 🕰️ Retains entropy planes and advances bounded initialization, MCU, and pixel units.
pub struct JpgComponentDecoder {
    decoder:Option<JpgStepDecoder>,
    samples:Vec<u8>,
    component_ids:Vec<u8>,
    width:usize,
    height:usize,
    plane:usize,
    initialized:usize,
    initialization_units:usize,
    pixel:usize,
    total:usize,
    working_bytes:usize,
    completed:bool,
}

impl JpgComponentDecoder {
    /// 🛡️ Admits a bounded physical input and reserves planes without initializing the whole frame.
    pub fn new(data:&dyn JpgByteSource,maximum_working_bytes:usize,should_cancel:&mut dyn FnMut()->bool)->Result<Self,ValueError> {
        if should_cancel() {return Err(refusal(ValueRefusalKind::Canceled,"JPEG component admission cancelled"));}
        if data.len()>maximum_working_bytes/8 {return Err(refusal(ValueRefusalKind::OwnershipLimit,"JPEG input exceeds component working budget"));}
        let source=CheckedSource{source:data,cancel:RefCell::new(should_cancel),stopped:Cell::new(false)};
        let parsed=parse_jpg_header(&source);
        if source.stopped.get() {return Err(refusal(ValueRefusalKind::Canceled,"JPEG header admission cancelled"));}
        let header=parsed.map_err(ValueError::from)?;
        let frame=&header.frame;
        if frame.precision!=8 || frame.width==0 || frame.height==0 || !(1..=4).contains(&frame.components.len()) {return Err(refusal(ValueRefusalKind::UnsupportedOwner,"JPEG component projection requires an 8-bit nonempty baseline frame"));}
        let hmax=frame.components.iter().map(|component|component.h_sampling).max().unwrap() as usize;
        let vmax=frame.components.iter().map(|component|component.v_sampling).max().unwrap() as usize;
        if frame.components.iter().any(|component|!(1..=4).contains(&component.h_sampling) || !(1..=4).contains(&component.v_sampling)) {return Err(refusal(ValueRefusalKind::InvalidValue,"JPEG component sampling is outside baseline bounds"));}
        let (width,height)=(frame.width as usize,frame.height as usize);
        let (mcus_x,mcus_y)=(width.div_ceil(8*hmax),height.div_ceil(8*vmax));
        let plane_samples=frame.components.iter().try_fold(0usize,|sum,component|mcus_x.checked_mul(component.h_sampling as usize)?.checked_mul(8)?.checked_mul(mcus_y)?.checked_mul(component.v_sampling as usize)?.checked_mul(8)?.checked_add(sum)).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"JPEG plane footprint overflow"))?;
        let pixels=width.checked_mul(height).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"JPEG pixel footprint overflow"))?;
        let output=pixels.checked_mul(frame.components.len()).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"JPEG component output footprint overflow"))?;
        let working=plane_samples.checked_mul(8).and_then(|bytes|bytes.checked_add(output)).and_then(|bytes|bytes.checked_add(data.len()*8)).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"JPEG component working footprint overflow"))?;
        if working>maximum_working_bytes {return Err(refusal(ValueRefusalKind::OwnershipLimit,"JPEG planes exceed component working budget"));}
        let component_ids=frame.components.iter().map(|component|component.id).collect();
        let initialization_units=plane_samples.div_ceil(64);
        let total=initialization_units.checked_add(mcus_x*mcus_y).and_then(|units|units.checked_add(pixels)).ok_or_else(||refusal(ValueRefusalKind::OwnershipLimit,"JPEG component progress footprint overflow"))?;
        let decoder=JpgStepDecoder::from_header(header,false).map_err(ValueError::from)?;
        let mut samples=Vec::new();samples.try_reserve_exact(output).map_err(|error|ValueError::new(ValueRefusalKind::AllocationFailed,error.to_string()))?;
        Ok(Self{decoder:Some(decoder),samples,component_ids,width,height,plane:0,initialized:0,initialization_units,pixel:0,total,working_bytes:working,completed:false})
    }

    /// 📏️ Physical input, entropy-plane and output footprint charged at admission.
    pub fn working_bytes(&self)->usize {self.working_bytes}

    /// 📊️ Reports completed and total bounded work units across every retained phase.
    pub fn progress(&self)->(usize,usize) {if self.completed {(self.total,self.total)}else{(self.initialized+self.decoder.as_ref().map_or(0,|decoder|decoder.mcu)+self.pixel,self.total)}}

    /// 🛑️ Releases this physical decode's allocations after cancellation.
    pub fn cancel(&mut self) {self.decoder=None;self.samples=Vec::new();self.component_ids=Vec::new();}

    /// 🎛️ Initializes at most 64 plane samples, decodes one MCU, or projects one pixel per unit.
    pub fn step(&mut self,data:&dyn JpgByteSource,unit_budget:usize,should_cancel:&mut dyn FnMut()->bool)->Result<Option<JpgDecodedComponents>,ValueError> {
        if should_cancel() {self.cancel();return Err(refusal(ValueRefusalKind::Canceled,"JPEG component decode cancelled"));}
        if self.decoder.is_none() {return Err(refusal(ValueRefusalKind::InvariantViolated,"JPEG component decoder is terminal"));}
        if unit_budget==0 {return Ok(None);}
        let mut remaining=unit_budget;
        while self.initialized<self.initialization_units && remaining>0 {
            if should_cancel() {self.cancel();return Err(refusal(ValueRefusalKind::Canceled,"JPEG plane initialization cancelled"));}
            let decoder=self.decoder.as_mut().unwrap();
            while decoder.planes[self.plane].len()==decoder.plane_dims[self.plane].0*decoder.plane_dims[self.plane].1 {self.plane+=1;}
            let plane=&mut decoder.planes[self.plane];let length=(plane.len()+64).min(decoder.plane_dims[self.plane].0*decoder.plane_dims[self.plane].1);plane.resize(length,0.0);
            self.initialized+=1;remaining-=1;
        }
        if remaining==0 {return Ok(None);}
        if !self.decoder.as_ref().unwrap().entropy_done {
            let source=CheckedSource{source:data,cancel:RefCell::new(should_cancel),stopped:Cell::new(false)};
            let result=self.decoder.as_mut().unwrap().step_entropy(&source,remaining);
            if source.stopped.get() {self.cancel();return Err(refusal(ValueRefusalKind::Canceled,"JPEG entropy decode cancelled"));}
            result.map_err(ValueError::from)?;return Ok(None);
        }
        while remaining>0 && self.pixel<self.width*self.height {
            if should_cancel() {self.cancel();return Err(refusal(ValueRefusalKind::Canceled,"JPEG component projection cancelled"));}
            let decoder=self.decoder.as_ref().unwrap();let frame=&decoder.header.as_ref().unwrap().frame;let (x,y)=(self.pixel%self.width,self.pixel/self.width);
            for (index,component) in frame.components.iter().enumerate() {let (stride,_)=decoder.plane_dims[index];let sx=x*component.h_sampling as usize/decoder.hmax;let sy=y*component.v_sampling as usize/decoder.vmax;self.samples.push(decoder.planes[index][sy*stride+sx].round().clamp(0.0,255.0) as u8);}
            self.pixel+=1;remaining-=1;
        }
        if self.pixel<self.width*self.height {return Ok(None);}
        self.decoder=None;self.completed=true;
        Ok(Some(JpgDecodedComponents{width:self.width as u32,height:self.height as u32,component_ids:std::mem::take(&mut self.component_ids),samples:std::mem::take(&mut self.samples)}))
    }
}
