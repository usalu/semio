//! 🎯️ One retained intrinsic selection intent; only completed coverage enters config.
use crate::{RasterCommand,RasterSnapshot,RasterMutation,RasterConfig,RasterConfigMutation};
use crate::commands::work_app::RasterWorkApp;
use crate::config::SetPixelSelectionEdit;
use crate::selection::RasterPixelSelection;
use semio_framework_plugin::retained_command::{ArtifactCommandInputs,ArtifactCommandWork,ArtifactCommandWorkStep};
use semio_framework_plugin::{ArtifactView,ConfigView,Emit,Fault};
use semio_framework_pixels::editing::{PixelSelectionJob,PixelFloodSelectionJob,SelectionShape,SelectionMerge,SelectionCombineJob};
use semio_s_artifact_raster_raster_mutations_pixels::paint_stroke::RasterSelectionSpan;
use semio_s_artifact_raster_raster_host::RasterPixelSourceWork;
use semio_framework_value::{RetirementDemand,RetainedCloneProgress,RetainedCloneStep,ValueError,ValueRefusalKind};
use semio_framework_job::{StepContext,OriginalTurnFailure};
use std::sync::Arc;
use std::mem::size_of;

#[path="📦️coverage/🦀️.rs"]
mod coverage;
use coverage::SelectionSpanWriter;

#[derive(semio_framework_value::RetireOwned)]
struct SelectionGarbage {request:Option<SelectPixels>,publication:Vec<RasterConfigMutation>,points:Vec<[f64;2]>,shape_points:Vec<[f64;2]>,coverage:Vec<u8>,mask:Vec<u8>,flood_mask:Vec<u8>,intersections:Vec<f64>,seen:Vec<bool>,queue:Vec<u32>,source:Option<Arc<semio_framework_pixels::RasterImage>>}

#[derive(Clone,Debug,Default,PartialEq,semio_framework_value::ToValue,semio_framework_dsl_record_derive::DslRecord,semio_framework_value::RetireOwned)]
#[value(rename_all="camelCase")]
#[dsl(keyword="select-pixels")]
pub struct SelectPixels {
    pub layer_id:String,
    pub target:String,
    pub expected_image_key:Option<String>,
    pub width:u32,
    pub height:u32,
    pub method:String,
    pub mode:String,
    pub path:String,
}

impl SelectPixels{
    /// 📥️ Checks bounded metadata while the actual ingress retains every original argument owner on refusal.
    pub fn validate_original_value(value:&semio_framework_value::DslValue)->Result<(),ValueError>{
        Self::validate_fields(value,false)
    }
    fn validate_fields(value:&semio_framework_value::DslValue,scoped:bool)->Result<(),ValueError>{
        use semio_framework_value::{DslValue,FromValue};
        let DslValue::Object(fields)=value else{return Err(value_refused("selection intent requires an object"));};
        if fields.len()>8+usize::from(scoped){return Err(value_refused("selection intent has unknown fields"));}
        let mut seen=0u16;
        for(key,value)in fields{
            let bit=match key.as_str(){"layerId"=>1,"target"=>2,"expectedImageKey"=>4,"width"=>8,"height"=>16,"method"=>32,"mode"=>64,"path"=>128,"windowId" if scoped=>256,_=>return Err(value_refused("selection intent has unknown fields"))};
            if seen&bit!=0{return Err(value_refused("selection intent has duplicate fields"));}seen|=bit;
            let valid=match key.as_str(){
                "path"=>matches!(value,DslValue::String(text)if text.len()<=1_048_576),
                "windowId"=>matches!(value,DslValue::String(text)if !text.is_empty()&&text.len()<=4096),
                "layerId"=>matches!(value,DslValue::String(text)if text.chars().take(257).count()<=256),
                "expectedImageKey"=>matches!(value,DslValue::Null)||matches!(value,DslValue::String(text)if text.chars().take(257).count()<=256),
                "target"=>value.as_str().is_some_and(|text|matches!(text,"pixels"|"mask")),
                "method"=>value.as_str().is_some_and(|text|matches!(text,"rectangle"|"ellipse"|"lasso"|"wand"|"all"|"invert"|"none")),
                "mode"=>value.as_str().is_some_and(|text|matches!(text,"replace"|"add"|"subtract"|"intersect")),
                _=>matches!(value,DslValue::Number(_))&&u32::from_value(value.clone()).is_ok_and(|value|value<=16_384),
            };
            if !valid{return Err(value_refused("selection intent field is invalid"));}
        }
        if seen&251!=251{return Err(value_refused("selection intent is missing fields"));}Ok(())
    }
    /// 🪟️ Payload ownership moves out of its scoped action envelope; unused metadata stays with the ingress retirement owner.
    pub fn from_original_action_value(source:&mut Option<semio_framework_value::DslValue>)->Result<Self,ValueError>{
        let value=source.as_ref().ok_or_else(||value_refused("selection intent is missing"))?;Self::validate_fields(value,true)?;
        let Some(semio_framework_value::DslValue::Object(fields))=source.as_mut()else{unreachable!()};Self::take_fields(fields)
    }
    fn take_fields(fields:&mut[(String,semio_framework_value::DslValue)])->Result<Self,ValueError>{
        use semio_framework_value::{DslValue,FromValue};let mut result=Self::default();
        for(key,source)in fields{if key=="windowId"{continue;}let value=std::mem::replace(source,DslValue::Null);match(key.as_str(),value){
            ("layerId",DslValue::String(text))=>result.layer_id=text,
            ("target",DslValue::String(text))=>result.target=text,
            ("expectedImageKey",DslValue::String(text))=>result.expected_image_key=Some(text),
            ("width",value)=>result.width=u32::from_value(value)?,
            ("height",value)=>result.height=u32::from_value(value)?,
            ("method",DslValue::String(text))=>result.method=text,
            ("mode",DslValue::String(text))=>result.mode=text,
            ("path",DslValue::String(text))=>result.path=text,
            _=>{},
        }}Ok(result)
    }
}

impl semio_framework_value::FromValue for SelectPixels{
    fn from_value(value:semio_framework_value::DslValue)->Result<Self,ValueError>{
        use semio_framework_value::DslValue;
        Self::validate_original_value(&value)?;
        let DslValue::Object(mut fields)=value else{unreachable!()};Self::take_fields(&mut fields)
    }
}

fn refused(message:&'static str)->OriginalTurnFailure{Fault::from(message).into()}
fn value_refused(message:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,message)}

pub fn handle(_payload:&SelectPixels,_doc:&ArtifactView<'_,RasterSnapshot>,_cfg:&ConfigView<'_,RasterConfig>)->Result<Emit<RasterMutation,RasterConfigMutation>,Fault>{Err(Fault::from("raster-select-pixels-requires-retained-work"))}

#[derive(Default)]
pub struct SelectPixelsWork {
    request:Option<SelectPixels>,
    publication:Vec<RasterConfigMutation>,
    cursor:usize,
    path_state:u8,
    points:Vec<[f64;2]>,
    source:Option<RasterPixelSourceWork>,
    shape:Option<PixelSelectionJob>,
    flood:Option<PixelFloodSelectionJob>,
    raster_done:bool,
    coverage:Vec<u8>,
    previous_span:usize,
    previous_active:Option<(usize,usize,u8)>,
    writer:Option<SelectionSpanWriter>,
    combined:usize,
    run_count:usize,
    stage:u8,
    closing:bool,
    snapshot_binding:usize,
    config_binding:usize,
    spent:bool,
    temporary_bytes:usize,
    pixel_writes:Option<usize>,
    cpu_units:usize,
    source_arc_bytes:usize,
    vertices:usize,
    garbage:std::mem::ManuallyDrop<Option<SelectionGarbage>>,
    retirement:std::mem::ManuallyDrop<Option<Box<dyn store::ErasedSnapshotRetirement>>>,
}

impl SelectPixelsWork {
    fn count(&self)->usize{self.request.as_ref().map_or(0,|request|request.width as usize*request.height as usize)}
    fn point_capacity(&self)->usize{self.request.as_ref().map_or(0,|request|match request.method.as_str(){"lasso"=>4096,"rectangle"|"ellipse"=>2,"wand"=>1,_=>0})}
    fn capacity(&self)->usize{self.source_arc_bytes+self.points.capacity()*size_of::<[f64;2]>()+self.coverage.capacity()+self.shape.as_ref().map_or(0,PixelSelectionJob::storage_capacity_bytes)+self.flood.as_ref().map_or(0,PixelFloodSelectionJob::storage_capacity_bytes)}
    fn initialized(&self)->usize{self.points.len()*size_of::<[f64;2]>()+self.coverage.len()+self.shape.as_ref().map_or(0,PixelSelectionJob::initialized_bytes)+self.flood.as_ref().map_or(0,PixelFloodSelectionJob::initialized_bytes)}
    fn path_step(&mut self,maximum_work:usize,continue_work:&mut dyn FnMut()->bool)->Result<bool,OriginalTurnFailure>{
        let capacity=self.point_capacity();let request=self.request.as_ref().ok_or_else(||refused("raster-selection-request-missing"))?;let path=request.path.as_bytes();
        self.cpu_units=0;
        for _ in 0..64{
            while self.cpu_units<maximum_work&&continue_work()&&path.get(self.cursor).is_some_and(|byte|matches!(*byte,b' '|b'\n'|b'\r'|b'\t')){self.cursor+=1;self.cpu_units+=1;}
            if self.cursor==path.len(){return if self.path_state==4{Ok(true)}else{Err(refused("raster-selection-path-invalid"))};}
            if self.cpu_units==maximum_work||!continue_work(){return Ok(false);}self.cpu_units+=1;
            if self.path_state==4{return Err(refused("raster-selection-path-invalid"));}
            if self.path_state==0{if path[self.cursor]!=b'['{return Err(refused("raster-selection-path-invalid"));}self.cursor+=1;self.path_state=1;continue;}
            if matches!(self.path_state,1|2)&&path[self.cursor]==b']'{self.cursor+=1;self.path_state=4;continue;}
            if self.path_state==2{if path[self.cursor]!=b','{return Err(refused("raster-selection-path-invalid"));}self.cursor+=1;self.path_state=3;continue;}
            if path.get(self.cursor)!=Some(&b'[')||self.points.len()==capacity{return Err(refused("raster-selection-path-invalid"));}
            let start=self.cursor;let end=path[start..].iter().take(128).position(|byte|*byte==b']').map(|offset|start+offset+1).ok_or_else(||refused("raster-selection-point-invalid"))?;
            let text=std::str::from_utf8(&path[start..end]).map_err(|_|refused("raster-selection-point-invalid"))?;
            let point:[f64;2]=semio_framework_pack_json::from_json_str(text,semio_framework_pack_json::JsonMemberPolicy::Reject).map_err(|_|refused("raster-selection-point-invalid"))?;
            if !point.iter().all(|value|value.is_finite()){return Err(refused("raster-selection-point-invalid"));}if self.points.capacity()==0{self.points.try_reserve_exact(capacity).map_err(|_|refused("raster-selection-reservation-failed"))?;}self.points.push(point);self.cursor=end;self.path_state=2;
        }Ok(false)
    }
    fn initialize(&mut self,input:&RasterSnapshot,config:&RasterConfig)->Result<(),OriginalTurnFailure>{
        let request=self.request.as_ref().ok_or_else(||refused("raster-selection-request-missing"))?;
        if request.path.len()>1024*1024||request.target!=config.paint_target||!matches!(request.mode.as_str(),"replace"|"add"|"subtract"|"intersect")||!matches!(request.method.as_str(),"rectangle"|"ellipse"|"lasso"|"wand"|"all"|"invert"|"none"){return Err(refused("raster-selection-intent-invalid"));}
        self.snapshot_binding=input as *const RasterSnapshot as usize;self.config_binding=config as *const RasterConfig as usize;
        if request.method=="none"{self.stage=8;return Ok(());}
        if request.layer_id.is_empty()||request.layer_id.chars().take(257).count()>256{return Err(refused("raster-selection-target-invalid"));}
        semio_framework_pixels::editing::validate_extent(request.width,request.height).map_err(|_|refused("raster-selection-extent-invalid"))?;
        self.stage=11;Ok(())
    }
    fn initialize_raster(&mut self,tolerance:u8)->Result<(),OriginalTurnFailure>{
        self.vertices=self.points.len();
        let request=self.request.as_ref().ok_or_else(||refused("raster-selection-request-missing"))?;
        if matches!(request.method.as_str(),"all"|"invert"){
            if !self.points.is_empty(){return Err(refused("raster-selection-path-invalid"));}
            self.shape=Some(PixelSelectionJob::begin(request.width,request.height,SelectionShape::Box {ellipse:false,x:0.0,y:0.0,width:f64::from(request.width),height:f64::from(request.height)}).map_err(|_|refused("raster-selection-shape-invalid"))?);self.stage=3;return Ok(());
        }
        let first=self.points.first().copied().ok_or_else(||refused("raster-selection-path-empty"))?;
        let shape=match request.method.as_str(){
            "lasso" if self.points.len()>=3=>Some(SelectionShape::Polygon(std::mem::take(&mut self.points))),
            "rectangle"|"ellipse" if self.points.len()==2=>{let last=self.points[1];Some(SelectionShape::Box {ellipse:request.method=="ellipse",x:first[0].min(last[0]),y:first[1].min(last[1]),width:(last[0]-first[0]).abs(),height:(last[1]-first[1]).abs()})},
            "wand" if self.points.len()==1=>None,
            _=>return Err(refused("raster-selection-method-invalid")),
        };
        let request=self.request.as_ref().ok_or_else(||refused("raster-selection-request-missing"))?;
        if let Some(shape)=shape{self.shape=Some(PixelSelectionJob::begin(request.width,request.height,shape).map_err(|_|refused("raster-selection-shape-invalid"))?);self.stage=3;return Ok(());}
        if first[0]<0.0||first[1]<0.0||first[0]>=f64::from(request.width)||first[1]>=f64::from(request.height){return Err(refused("raster-selection-seed-invalid"));}
        if request.expected_image_key.is_none(){self.flood=Some(PixelFloodSelectionJob::uniform(request.width,request.height,if request.target=="mask"{[255;4]}else{[0;4]},first[0].floor() as u32,first[1].floor() as u32,tolerance).map_err(|_|refused("raster-selection-flood-invalid"))?);self.flood.as_mut().unwrap().reserve_queue().map_err(|_|refused("raster-selection-reservation-failed"))?;self.stage=3;}else{self.stage=2;}
        Ok(())
    }
    fn advance(&mut self,snapshot:&RasterSnapshot,config:&RasterConfig,grant:semio_framework_value::RetainedCloneGrant)->Result<(bool,RetainedCloneProgress),OriginalTurnFailure>{
        self.advance_controlled(snapshot,config,grant,1024,&mut||true)
    }
    fn advance_controlled(&mut self,snapshot:&RasterSnapshot,config:&RasterConfig,grant:semio_framework_value::RetainedCloneGrant,maximum_work:usize,continue_work:&mut dyn FnMut()->bool)->Result<(bool,RetainedCloneProgress),OriginalTurnFailure>{
        if self.closing||self.spent{return Err(refused("raster-selection-work-closed"));}
        self.temporary_bytes=0;
        self.pixel_writes=None;
        self.cpu_units=1;
        if self.snapshot_binding!=0&&(self.snapshot_binding!=snapshot as *const RasterSnapshot as usize||self.config_binding!=config as *const RasterConfig as usize){return Err(refused("raster-selection-source-changed"));}
        let mut receipt=RetainedCloneProgress::default();
        match self.stage {
            0=>self.initialize(snapshot,config)?,
            1=>if self.path_step(maximum_work,continue_work)?{self.stage=10;},
            11=>{
                if self.source.is_none(){let request=self.request.as_ref().ok_or_else(||refused("raster-selection-request-missing"))?;if let Some((source,progress))=RasterPixelSourceWork::begin(snapshot,&request.layer_id,&request.target,grant)?{receipt=progress;self.source=Some(source);}return Ok((false,receipt));}
                let source=self.source.as_mut().ok_or_else(||refused("raster-selection-source-missing"))?;
                let step=source.advance_geometry(snapshot,grant)?;let done=matches!(step,RetainedCloneStep::Complete(_));receipt=step.progress();
                if done{let descriptor=source.descriptor()?;let request=self.request.as_ref().ok_or_else(||refused("raster-selection-request-missing"))?;if !descriptor.visible||descriptor.width!=request.width||descriptor.height!=request.height||descriptor.image_key!=request.expected_image_key{return Err(refused("raster-selection-source-changed"));}self.stage=1;}
            }
            2=>{
                if self.source.is_none(){let request=self.request.as_ref().ok_or_else(||refused("raster-selection-request-missing"))?;if let Some((source,progress))=RasterPixelSourceWork::begin(snapshot,&request.layer_id,&request.target,grant)?{receipt=progress;self.source=Some(source);}return Ok((false,receipt));}
                let source=self.source.as_mut().ok_or_else(||refused("raster-selection-source-missing"))?;
                let step=source.advance(snapshot,grant)?;let done=matches!(step,RetainedCloneStep::Complete(_));receipt=match step{RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress};
                if done{self.stage=9;}
            }
            3=>{
                let progress=if let Some(shape)=self.shape.as_mut(){let progress=shape.advance_budget_with_control(maximum_work,grant.maximum_copy_bytes.min(32768),continue_work);self.pixel_writes=Some(shape.last_write_bytes());self.cpu_units=shape.last_work_units();progress}else{let flood=self.flood.as_mut().ok_or_else(||refused("raster-selection-job-missing"))?;let progress=flood.advance((grant.maximum_copy_bytes.min(32768)/16).max(1).min(maximum_work.max(1)));self.pixel_writes=Some(flood.last_write_bytes());self.cpu_units=flood.last_work_units();progress}.map_err(|_|refused("raster-selection-raster-failed"))?;
                if progress.done{self.raster_done=true;self.stage=4;}
            }
            4=>{
                let request=self.request.as_ref().ok_or_else(||refused("raster-selection-request-missing"))?;let count=self.count();let budget=grant.maximum_copy_bytes.min(32768);let mut written=0;self.cpu_units=0;
                if self.coverage.capacity()==0{self.coverage.try_reserve_exact(count).map_err(|_|refused("raster-selection-reservation-failed"))?;}
                let previous=config.pixel_selection.as_ref().filter(|selection|selection.layer_id==request.layer_id&&selection.target==request.target&&selection.width==request.width&&selection.height==request.height);
                while self.coverage.len()<count&&self.cpu_units<maximum_work&&continue_work(){
                    let index=self.coverage.len();
                    if self.previous_active.is_some_and(|(_,end,_)|end<=index){self.previous_active=None;self.previous_span+=1;}
                    if self.previous_active.is_none(){if let Some(previous)=previous.filter(|previous|self.previous_span<previous.spans.len()){
                        if budget.saturating_sub(written)<size_of::<RasterSelectionSpan>(){break;}
                        let demand=previous.spans.rank_observation_demand(self.previous_span)?;if grant.maximum_depth<demand.depth{break;}
                        let bound=2+16*demand.depth.saturating_sub(1);if maximum_work.saturating_sub(self.cpu_units)<bound{break;}
                        let(span,work)=previous.spans.get_with_work(self.previous_span);self.cpu_units+=work as usize;let span=span.ok_or_else(||refused("raster-selection-previous-span-missing"))?;self.previous_active=Some((span.start as usize,span.start as usize+span.length as usize,span.coverage.min(255)as u8));written+=size_of::<RasterSelectionSpan>();continue;
                    }}
                    if written==budget{break;}self.coverage.push(self.previous_active.filter(|(start,_,_)|*start<=index).map_or(0,|(_,_,alpha)|alpha));written+=1;self.cpu_units+=1;
                }
                self.pixel_writes=Some(written);if self.coverage.len()==count{self.stage=5;}
            }
            5=>{
                let request=self.request.as_ref().ok_or_else(||refused("raster-selection-request-missing"))?;let count=self.count();let factor=if request.method=="invert"{3}else{2};let chunk=(grant.maximum_copy_bytes.min(32768)/factor).min(maximum_work/factor);if chunk==0{self.cpu_units=0;self.pixel_writes=Some(0);return Ok((false,receipt));}let end=count.min(self.combined+chunk);
                let next=if let Some(shape)=&self.shape{shape.result()}else{self.flood.as_ref().ok_or_else(||refused("raster-selection-job-missing"))?.result()}.map_err(|_|refused("raster-selection-result-missing"))?;
                let mode=if matches!(request.method.as_str(),"all"|"invert"){SelectionMerge::Replace}else{match request.mode.as_str(){"add"=>SelectionMerge::Add,"subtract"=>SelectionMerge::Subtract,"intersect"=>SelectionMerge::Intersect,_=>SelectionMerge::Replace}};
                let inverted:Option<Vec<u8>>=(request.method=="invert").then(||self.coverage[self.combined..end].iter().map(|coverage|255-*coverage).collect());let next=inverted.as_deref().unwrap_or(&next[self.combined..end]);
                let mut job=SelectionCombineJob::new(Some(&self.coverage[self.combined..end]),next,mode).map_err(|_|refused("raster-selection-combine-invalid"))?;job.advance(32768).map_err(|_|refused("raster-selection-combine-failed"))?;
                self.temporary_bytes=job.output.capacity()+inverted.as_ref().map_or(0,Vec::capacity);
                let output=job.into_result().map_err(|_|refused("raster-selection-combine-failed"))?;self.cpu_units=(end-self.combined)*factor;self.pixel_writes=Some(self.cpu_units);
                for (offset,coverage) in output.iter().copied().enumerate(){
                    let index=self.combined+offset;
                    if coverage>0&&(index==0||self.coverage[index-1]!=coverage){self.run_count+=1;}self.coverage[index]=coverage;
                }self.combined=end;if end==count{self.stage=6;}
            }
            6=>{self.writer=Some(SelectionSpanWriter::new(self.run_count,self.count())?);self.pixel_writes=Some(size_of::<SelectionSpanWriter>());self.stage=7;}
            7=>{
                let writer=self.writer.as_mut().ok_or_else(||refused("selection page writer missing"))?;let(done,progress)=writer.advance(&self.coverage,grant,maximum_work,continue_work)?;self.cpu_units=writer.work;receipt=progress;if done{self.stage=8;}
            }
            9=>{
                let source=self.source.as_mut().and_then(RasterPixelSourceWork::take_pixels).ok_or_else(||refused("raster-selection-source-missing"))?;let first=self.points[0];
                self.source_arc_bytes=size_of::<semio_framework_pixels::RasterImage>()+2*size_of::<usize>();
                let mut job=PixelFloodSelectionJob::new(Arc::new(source),first[0].floor() as u32,first[1].floor() as u32,config.fill_tolerance.min(255) as u8).map_err(|_|refused("raster-selection-flood-invalid"))?;
                job.reserve_queue().map_err(|_|refused("raster-selection-reservation-failed"))?;self.flood=Some(job);self.stage=3;
            }
            10=>self.initialize_raster(config.fill_tolerance.min(255) as u8)?,
            _=>return Ok((true,receipt)),
        }Ok((self.stage==8,receipt))
    }
    fn take_result(&mut self)->Result<Option<RasterPixelSelection>,OriginalTurnFailure>{
        if self.stage!=8||self.closing||self.spent{return Err(refused("raster-selection-result-incomplete"));}self.spent=true;
        let request=self.request.as_mut().ok_or_else(||refused("raster-selection-request-missing"))?;
        if request.method=="none"{return Ok(None);}
        let spans=self.writer.as_mut().and_then(SelectionSpanWriter::take).ok_or_else(||refused("selection page result missing"))?;
        Ok(Some(RasterPixelSelection {layer_id:std::mem::take(&mut request.layer_id),target:std::mem::take(&mut request.target),width:request.width,height:request.height,spans}))
    }
    fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
        if let Some(source)=&self.source{return if source.terminal_is_empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<RasterPixelSourceWork>>(),depth:1,..Default::default()})}else{source.close_demands(body)};}
        if let Some(writer)=&self.writer{return if writer.empty(){Ok(RetirementDemand{copy_bytes:size_of::<Option<SelectionSpanWriter>>(),depth:1,..Default::default()})}else{writer.close_demands(body)};}
        if let Some(active)=self.retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(active,body);}
        if self.garbage.is_some(){return store::os_store::snapshot_read::artifact_retirement_owned_birth_demands(&self.garbage);}Ok(Default::default())
    }
    fn begin_closing(&mut self){
        if self.closing{return;}self.closing=true;
        let (mask,intersections,extra_points)=self.shape.take().map(|job|{let storage=job.into_storage();(storage.mask,storage.intersections,storage.points)}).unwrap_or_default();
        let (flood_mask,seen,queue,source)=self.flood.take().map(|job|{let storage=job.into_storage();(storage.mask,storage.seen,storage.queue,storage.source)}).unwrap_or_default();
        self.garbage.replace(SelectionGarbage {request:self.request.take(),publication:std::mem::take(&mut self.publication),points:std::mem::take(&mut self.points),shape_points:extra_points,coverage:std::mem::take(&mut self.coverage),mask,flood_mask,intersections,seen,queue,source});
    }
    fn close_one(&mut self,grant:semio_framework_value::RetainedCloneGrant)->semio_framework_value::RetainedWorkOutcome{
        self.begin_closing();
        if let Some(owner)=self.source.as_mut(){if !owner.terminal_is_empty(){return owner.close_step(grant)}let bytes=size_of::<Option<RasterPixelSourceWork>>();if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.source=None;return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})),work:1};}
        if let Some(owner)=self.writer.as_mut(){if !owner.empty(){return owner.close_step(grant)}let bytes=size_of::<Option<SelectionSpanWriter>>();if grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(Default::default())),work:0}}self.writer=None;return semio_framework_value::RetainedWorkOutcome{result:Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})),work:1};}
        if self.retirement.is_some(){store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant)}else{store::os_store::snapshot_read::artifact_retirement_admit_owned(&mut self.garbage,&mut self.retirement,grant)}
    }
    fn empty(&self)->bool{self.closing&&self.request.is_none()&&self.source.is_none()&&self.writer.is_none()&&self.shape.is_none()&&self.flood.is_none()&&self.garbage.is_none()&&self.retirement.is_none()}
}

impl<A:RasterWorkApp> ArtifactCommandWork<A> for SelectPixelsWork {
    fn tool_id(&self)->&'static str{"selectPixels"}
    fn receive_original_command(&mut self,command:&mut RasterCommand,_cx:&mut StepContext<'_>)->Result<bool,OriginalTurnFailure>{if self.request.is_none(){let RasterCommand::SelectPixels(request)=command else{return Err(refused("raster-selection-route-mismatch"));};self.request=Some(std::mem::take(request));}Ok(true)}
    fn extent(&self,command:&RasterCommand,_snapshot:&RasterSnapshot,_interaction:&protocol::InteractionState,_context:Option<&semio_framework_plugin::app::ArtifactOwnedToolJobContext<A>>)->Option<usize>{matches!(command,RasterCommand::SelectPixels(_)).then_some(1)}
    fn work_demands(&self,input:&ArtifactCommandInputs<'_,A>,maximum_copy_bytes:usize)->Result<RetirementDemand,ValueError>{
        if self.stage==8{return Ok(RetirementDemand{copy_bytes:size_of::<Vec<RasterConfigMutation>>()+size_of::<RasterConfigMutation>(),capacity_bytes:if self.publication.capacity()==0{size_of::<RasterConfigMutation>()}else{0},depth:4,..Default::default()});}
        if self.stage==7{return self.writer.as_ref().ok_or_else(||value_refused("selection writer missing"))?.demands(maximum_copy_bytes);}
        if matches!(self.stage,2|11){if let Some(source)=&self.source{return if self.stage==11{source.geometry_demands(maximum_copy_bytes)}else{source.demands(maximum_copy_bytes)};}let request=self.request.as_ref().ok_or_else(||value_refused("selection request missing"))?;return RasterPixelSourceWork::begin_demands(input.snapshot,&request.layer_id,&request.target);}
        let capacity=match self.stage{1 if self.points.capacity()==0=>self.point_capacity()*size_of::<[f64;2]>(),4 if self.coverage.capacity()==0=>self.count(),5=>32768,9=>self.count()*6+size_of::<semio_framework_pixels::RasterImage>()+2*size_of::<usize>(),10=>self.count()*6+self.points.len()*size_of::<f64>(),_=>0};
        let copy=32768;
        let depth=if self.stage==4{input.config.pixel_selection.as_ref().filter(|selection|self.previous_span<selection.spans.len()).map_or(Ok(4),|selection|selection.spans.rank_observation_demand(self.previous_span).map(|demand|demand.depth.max(4)))?}else{4};
        Ok(RetirementDemand {copy_bytes:copy,capacity_bytes:capacity,release_bytes:if self.stage==5{32768}else{0},depth})
    }
    fn step(&mut self,input:&ArtifactCommandInputs<'_,A>,cx:&mut StepContext<'_>)->Result<ArtifactCommandWorkStep<A>,OriginalTurnFailure>{
        if cx.is_cancelled(){return Err(refused("raster-selection-cancelled"));}
        if cx.should_yield(){return Ok(ArtifactCommandWorkStep::Pending {stage:"selection-grant",preview:b""});}
        if self.request.is_none(){return Err(refused("raster-selection-original-command-required"));}
        if self.stage==8{
            let demand=RetirementDemand{copy_bytes:size_of::<Vec<RasterConfigMutation>>()+size_of::<RasterConfigMutation>(),capacity_bytes:if self.publication.capacity()==0{size_of::<RasterConfigMutation>()}else{0},depth:4,..Default::default()};
            if !semio_framework_value::retained_clone::retained_clone_grant_admits(cx.retained_grant(),demand){return Ok(ArtifactCommandWorkStep::Pending{stage:"selection-publication",preview:b""});}
            if self.publication.capacity()==0{
                self.publication.try_reserve_exact(1).map_err(|_|refused("raster-selection-publication-reservation-failed"))?;
                let progress=RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Vec<RasterConfigMutation>>(),retained_capacity_bytes:self.publication.capacity()*size_of::<RasterConfigMutation>(),..Default::default()};
                cx.consume_fuel(1);cx.consume_retained(progress)?;
                return Ok(ArtifactCommandWorkStep::Progress{stage:"selection-publication",preview:b""});
            }
            let selection=self.take_result()?;
            self.publication.push(RasterConfigMutation::SetPixelSelection(SetPixelSelectionEdit{selection}));
            cx.consume_fuel(1);cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<RasterConfigMutation>()+size_of::<Vec<RasterConfigMutation>>(),..Default::default()})?;
            return Ok(ArtifactCommandWorkStep::Complete(Emit::config(std::mem::take(&mut self.publication))));
        }
        let before=self.capacity();let initialized=self.initialized();let source_step=matches!(self.stage,2|7|11);
        let maximum_work=cx.fuel_remaining().min(1024)as usize;
        let advanced=self.advance_controlled(input.snapshot,input.config,cx.retained_grant(),maximum_work,&mut||!cx.should_yield()&&!cx.is_cancelled());cx.consume_fuel(self.cpu_units as u64);
        let (done,receipt)=advanced?;cx.consume_retained(receipt)?;
        if !source_step{cx.consume_retained(RetainedCloneProgress {copied_items:1,copied_bytes:self.pixel_writes.unwrap_or_else(||self.initialized().saturating_sub(initialized)+self.temporary_bytes),retained_capacity_bytes:self.capacity().saturating_sub(before)+self.temporary_bytes,released_bytes:self.temporary_bytes})?;}
        if !done{return Ok(ArtifactCommandWorkStep::Progress {stage:["selection-target","selection-path","selection-source","selection-raster","selection-restore","selection-combine","selection-reserve","selection-spans"][self.stage.min(7) as usize],preview:b""});}
        Ok(ArtifactCommandWorkStep::Progress{stage:"selection-publication",preview:b""})
    }
    fn begin_close(&mut self){self.begin_closing();}
    fn close_step(&mut self,cx:&mut StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,OriginalTurnFailure>{
        let turn=self.close_one(cx.retained_grant());cx.consume_fuel(turn.work);let result=turn.result?;
        let progress=match result{RetainedCloneStep::Progress(progress)|RetainedCloneStep::Complete(progress)=>progress};
        Ok(if self.empty(){semio_framework_job::InteractiveJobCloseStep::Complete {progress}}else{semio_framework_job::InteractiveJobCloseStep::Pending {progress}})
    }
    fn next_close_copy_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(32768)?.copy_bytes)}
    fn next_close_capacity_byte_demand(&self,body:usize)->Result<usize,ValueError>{Ok(self.close_demands(body)?.capacity_bytes)}
    fn next_close_release_byte_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(32768)?.release_bytes)}
    fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(self.close_demands(32768)?.depth)}
    fn terminal_is_empty(&self)->bool{self.empty()}
}

#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]
mod tests;
