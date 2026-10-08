//! 🧬️ Fuelled publication and exact native image inverse intent.
use crate::{PngSnapshot,schema::{mutations::{PngMutation,ReplaceImage,ReplaceSamples,SetGamma},snapshot::PngImage}};
use crate::schema::operations::{owned_validation::PngOwnedValidationWork,validate_native_paint_target};
use crate::schema::snapshot::{PngColorType,PngRegion};
use semio_framework_value::retained_clone::{RetainedClone,RetainedCloneCursor,RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose};
use semio_framework_value::ValueError;
use store::snapshot_clone_preparation::{RetainedCloneEdit,RetainedCloneEditCursor,RetainedCloneEditStep,RetainedClonePreparationFactory};
use std::sync::Arc;

pub fn factory()->Arc<dyn store::ArtifactStoreOneItemPreparationFactory<PngSnapshot,PngMutation>>{Arc::new(RetainedClonePreparationFactory::new(Arc::new(PngPublication),Arc::new(semio_framework_value::retirement::OwnedValueRetirementFactory::default()),Arc::new(semio_framework_value::retirement::SharedValueRetirementFactory::default()),64).expect("png retained publication depth"))}
#[derive(semio_framework_value::FactoryPayloadRetirement)]
pub struct PngPublication;
impl RetainedCloneEdit<PngSnapshot,PngMutation> for PngPublication{
    type Cursor=PngPublicationCursor;
    fn preflight(&self,mutation:&PngMutation,_lane:store::HistoryLane)->Result<store::ArtifactStoreOneItemFootprint,String>{Ok(store::ArtifactStoreOneItemFootprint::for_leaf::<PngSnapshot,_>(mutation,store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES))}
    fn begin_demand(&self)->semio_framework_value::retained_clone::RetainedCloneBirthDemand{semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:0,depth:0}}
    fn snapshot_cursor_birth_demand(&self)->semio_framework_value::retained_clone::RetainedCloneBirthDemand{semio_framework_value::retained_clone::RetainedCloneBirthDemand{capacity_bytes:0,depth:0}}
    fn begin(&self,grant:RetainedCloneGrant)->Result<(Self::Cursor,RetainedCloneProgress),ValueError>{let progress=self.begin_demand().admit(grant)?;Ok((PngPublicationCursor::new(),progress))}
}
pub struct PngPublicationCursor{
    phase:u8,copy:Option<<PngImage as RetainedClone>::Cursor>,inverse_image:Option<PngImage>,displaced_image:Option<PngImage>,inverse:Option<Vec<PngMutation>>,old_samples:Vec<u16>,changed:bool,retirement:RetainedCloneClose,validation:PngOwnedValidationWork,pixel:usize,closing:bool,cancelled:bool,
}
impl PngPublicationCursor{
    pub fn new()->Self{Self{phase:0,copy:Some(PngImage::retained_clone_cursor()),inverse_image:None,displaced_image:None,inverse:None,old_samples:Vec::new(),changed:false,retirement:RetainedCloneClose::default(),validation:PngOwnedValidationWork::default(),pixel:0,closing:false,cancelled:false}}
    fn close_copy(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneProgress,String>{
        let cursor=self.copy.as_mut().ok_or("png: missing image clone cursor")?;
        let step=cursor.close_step(grant).map_err(ValueError::into_message)?;
        if matches!(step,RetainedCloneStep::Complete(_)){if !cursor.terminal_is_empty(){return Err("png: image clone close retained ownership".into());}self.copy=None;self.phase=if self.phase==1{8}else{4};}
        Ok(step.progress())
    }
}
fn unit()->RetainedCloneProgress{RetainedCloneProgress{copied_items:1,..Default::default()}}
impl RetainedCloneEditCursor<PngSnapshot,PngMutation> for PngPublicationCursor{
    fn advance(&mut self,base:RetainedCloneRef<'_,PngSnapshot>,post:&mut PngSnapshot,mutation:RetainedCloneRef<'_,PngMutation>,grant:RetainedCloneGrant)->Result<RetainedCloneEditStep,String>{
        if self.closing||self.cancelled{return Err("png: publication cursor is closing".into());}if grant.maximum_items==0{return Ok(RetainedCloneEditStep::Progress(Default::default()));}
        if self.phase==0&&!matches!(mutation.get(),PngMutation::ReplaceImage(_)){self.copy=None;self.phase=5;}
        let progress=match self.phase{
            0=>{let cursor=self.copy.as_mut().ok_or("png: missing inverse image clone")?;let step=cursor.advance(base.project(1,|value|&value.image),grant).map_err(ValueError::into_message)?;if matches!(step,RetainedCloneStep::Complete(_)){self.inverse_image=Some(cursor.take().ok_or("png: inverse image clone lost its owner")?);cursor.begin_close();self.phase=1;}step.progress()},
            1|3=>self.close_copy(grant)?,
            8=>{let bytes=std::mem::size_of::<PngMutation>();if grant.maximum_capacity_bytes<bytes{return Ok(RetainedCloneEditStep::Progress(Default::default()));}let mut inverse=Vec::with_capacity(1);inverse.push(PngMutation::ReplaceImage(ReplaceImage{image:self.inverse_image.take().ok_or("png: inverse image owner is absent")?}));self.inverse=Some(inverse);self.phase=2;RetainedCloneProgress{copied_items:1,retained_capacity_bytes:bytes,..Default::default()}},
            2=>{if let PngMutation::ReplaceImage(_)=mutation.get(){let source=mutation.project(1,|value|match value{PngMutation::ReplaceImage(value)=>&value.image,_=>unreachable!()});let cursor=self.copy.get_or_insert_with(PngImage::retained_clone_cursor);let step=cursor.advance(source,grant).map_err(ValueError::into_message)?;if matches!(step,RetainedCloneStep::Complete(_)){let next=cursor.take().ok_or("png: incoming image clone lost its owner")?;self.displaced_image=Some(std::mem::replace(&mut post.image,next));cursor.begin_close();self.phase=3;}step.progress()}else{self.phase=5;unit()}},
            4=>{
                if let Some(step)=self.retirement.begin_granted(&mut self.displaced_image,grant).map_err(ValueError::into_message)?{step.progress()}
                else{let step=self.retirement.step_granted(grant).map_err(ValueError::into_message)?;if matches!(step,RetainedCloneStep::Complete(_)){self.phase=5;}step.progress()}
            },
            5=>match mutation.get(){
                PngMutation::SetGamma(value)=>{if value.gama==Some(0){return Err("png: gamma must be positive".into());}self.phase=6;unit()},
                PngMutation::ReplaceSamples(value)=>{post.image.validate_region_samples(value.region,&value.samples)?;self.phase=6;unit()},
                _=>{let(done,progress)=self.validation.advance(if matches!(mutation.get(),PngMutation::ReplaceImage(_)){post}else{base.get()},None,grant)?;if done{match mutation.get(){
                    PngMutation::ChangeGamma(value)=>{if self.validation.revision().as_deref()!=Some(&value.revision){return Err("png: source revision changed".into());}if value.gama==Some(0){return Err("png: gamma must be positive".into());}self.phase=6;},
                    PngMutation::PaintNativeSamples(value)=>{if self.validation.revision().as_deref()!=Some(&value.revision){return Err("png: source revision changed".into());}validate_native_paint_target(post,value.region,value.paint)?;self.phase=6;},
                    PngMutation::PatchPixels(value)=>{if self.validation.revision().as_deref()!=Some(&value.revision){return Err("png: source revision changed".into());}if post.image.color_type!=PngColorType::Rgba||post.image.bit_depth!=8{return Err("png: pixel patches need RGBA8".into());}validate_native_paint_target(post,PngRegion{x:value.x,y:value.y,width:value.width,height:value.height},crate::schema::snapshot::PngNativePaint::rgba(value.red.into(),value.green.into(),value.blue.into(),value.alpha.into()))?;self.phase=6;},
                    _=>self.phase=7,
                }}progress},
            },
            6=>{
                let gamma=match mutation.get(){PngMutation::ChangeGamma(value)=>Some(value.gama),PngMutation::SetGamma(value)=>Some(value.gama),_=>None};
                if let Some(gama)=gamma{
                    let bytes=std::mem::size_of::<PngMutation>();
                    if grant.maximum_copy_bytes<std::mem::size_of::<Option<u32>>()||grant.maximum_capacity_bytes<bytes{return Ok(RetainedCloneEditStep::Progress(Default::default()));}
                    let before=base.get().image.gamma;post.image.gamma=gama;
                    self.inverse=Some(if before!=gama{vec![PngMutation::SetGamma(SetGamma{gama:before})]}else{Vec::new()});self.phase=7;
                    return Ok(RetainedCloneEditStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:std::mem::size_of::<Option<u32>>(),retained_capacity_bytes:bytes,..Default::default()}));
                }
                let(region,paint)=match mutation.get(){PngMutation::PatchPixels(value)=>(PngRegion{x:value.x,y:value.y,width:value.width,height:value.height},Some(crate::schema::snapshot::PngNativePaint::rgba(value.red.into(),value.green.into(),value.blue.into(),value.alpha.into()))),PngMutation::PaintNativeSamples(value)=>(value.region,Some(value.paint)),PngMutation::ReplaceSamples(value)=>(value.region,None),_=>return Err("png: paint lost its typed intent".into())};
                let spp=post.image.color_type.samples_per_pixel();let bytes=spp*std::mem::size_of::<u16>();
                if grant.maximum_copy_bytes<bytes||grant.maximum_capacity_bytes<bytes{return Ok(RetainedCloneEditStep::Progress(Default::default()));}
                let pixel=self.pixel;
                let start=((region.y as usize+pixel/region.width as usize)*post.image.width as usize+region.x as usize+pixel%region.width as usize)*spp;
                let paint_samples=paint.map(|paint|paint.samples());
                let target:&[u16]=match (&paint_samples,mutation.get()){(Some(samples),_)=>&samples[..spp],(None,PngMutation::ReplaceSamples(value))=>&value.samples[pixel*spp..pixel*spp+spp],_=>return Err("png: paint lost its typed intent".into())};
                if post.image.samples[start..start+spp]!=*target{self.changed=true;}
                self.old_samples.extend_from_slice(&post.image.samples[start..start+spp]);
                post.image.samples[start..start+spp].copy_from_slice(target);self.pixel+=1;
                if self.pixel==region.width as usize*region.height as usize{
                    let old=std::mem::take(&mut self.old_samples);
                    self.inverse=Some(if self.changed{vec![PngMutation::ReplaceSamples(ReplaceSamples{region,samples:old})]}else{self.old_samples=old;Vec::new()});
                    self.phase=7;
                }
                RetainedCloneProgress{copied_items:1,copied_bytes:bytes,retained_capacity_bytes:bytes,..Default::default()}
            },
            _=>return Ok(RetainedCloneEditStep::Complete(unit())),
        };Ok(RetainedCloneEditStep::Progress(progress))
    }
    fn take_inverse(&mut self)->Option<Vec<PngMutation>>{if self.phase==7{self.inverse.take()}else{None}}
    fn cancel(&mut self){self.cancelled=true;self.begin_close();}
    fn begin_close(&mut self)->bool{if self.closing{return false;}self.closing=true;if let Some(copy)=self.copy.as_mut(){copy.begin_close();}true}
    fn close_step(&mut self,grant:RetainedCloneGrant)->Result<RetainedCloneStep,ValueError>{
        if grant.maximum_items==0{return Ok(RetainedCloneStep::Progress(Default::default()));}
        if let Some(copy)=self.copy.as_mut(){let step=copy.close_step(grant)?;if matches!(step,RetainedCloneStep::Complete(_)){if !copy.terminal_is_empty(){return Err(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"png: closed clone retained owner"));}self.copy=None;}return Ok(RetainedCloneStep::Progress(step.progress()));}
        if !self.retirement.is_empty(){return self.retirement.step_granted(grant).map(|step|RetainedCloneStep::Progress(step.progress()));}
        if self.old_samples.capacity()!=0{if let Some(step)=self.retirement.begin_default_granted(&mut self.old_samples,grant)?{return Ok(step);}return self.retirement.step_granted(grant);}
        if let Some(step)=self.retirement.begin_granted(&mut self.displaced_image,grant)?{return Ok(step);}
        if let Some(step)=self.retirement.begin_granted(&mut self.inverse_image,grant)?{return Ok(step);}
        if let Some(step)=self.retirement.begin_granted(&mut self.inverse,grant)?{return Ok(step);}
        Ok(RetainedCloneStep::Complete(Default::default()))
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{if let Some(copy)=self.copy.as_ref(){return copy.next_close_copy_byte_demand();}self.retirement.next_copy_byte_demand()}
    fn next_close_capacity_byte_demand(&self,copy_bytes:usize)->Result<usize,ValueError>{
        if let Some(copy)=self.copy.as_ref(){return copy.next_close_capacity_byte_demand(copy_bytes);}
        if !self.retirement.is_empty(){return self.retirement.next_capacity_byte_demand(copy_bytes);}
        if self.old_samples.capacity()!=0{return self.retirement.next_owner_capacity_byte_demand::<Vec<u16>>(true,copy_bytes);}
        if self.displaced_image.is_some()||self.inverse_image.is_some(){return self.retirement.next_owner_capacity_byte_demand::<PngImage>(true,copy_bytes);}
        self.retirement.next_owner_capacity_byte_demand::<Vec<PngMutation>>(self.inverse.is_some(),copy_bytes)
    }
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{if let Some(copy)=self.copy.as_ref(){return copy.next_close_release_byte_demand();}self.retirement.next_release_byte_demand()}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{if let Some(copy)=self.copy.as_ref(){return copy.next_close_depth_demand();}if !self.retirement.is_empty(){return self.retirement.next_depth_demand();}Ok(usize::from(self.displaced_image.is_some()||self.inverse_image.is_some()||self.inverse.is_some()||self.old_samples.capacity()!=0))}

    fn terminal_is_empty(&self)->bool{self.closing&&self.copy.is_none()&&self.inverse_image.is_none()&&self.displaced_image.is_none()&&self.inverse.is_none()&&self.old_samples.capacity()==0&&self.retirement.is_empty()}
}
#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
