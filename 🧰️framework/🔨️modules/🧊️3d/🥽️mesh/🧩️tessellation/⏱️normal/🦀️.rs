use super::*;
use protocol::value::{ValueError,ValueRefusalKind,retained_clone::{RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep}};

#[derive(Clone,Copy,PartialEq,Eq)]
pub(super) enum TriangleFinish{Last,Clip,Fan}
#[derive(Clone,Copy)]
pub(super) struct TriangleFrame{locals:[usize;3],base:u32,emitted:u8,indices_done:bool,finish:TriangleFinish}
#[derive(Clone,Copy,Default)]
pub(super) struct MeshNormalReceipt(pub RetainedCloneProgress);
protocol::value::artifact_retire_leaf!(TriangleFinish,TriangleFrame,MeshNormalReceipt);

#[derive(Clone,Copy)]
enum Frontier{Buffer,Metadata,Face,Corner(usize),TriangleSetup(TriangleFinish),TriangleIndices,TriangleFinish,Ear,Edge(usize),Attribute,Materials,Textures,Mark,Transfer}
fn refusal(reason:&'static str)->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,reason)}

impl MeshTessellationJob{
    fn normal_frontier(&self)->Result<Frontier,ValueError>{
        if self.output.is_none(){return Err(refusal("original tessellation output already transferred"));}
        if !self.buffer_reservation_complete(){return Ok(Frontier::Buffer);}
        if !self.metadata_capture_complete(){return Ok(Frontier::Metadata);}
        if self.face<self.mesh.face_count(){
            if let Some(frame)=self.triangle_pending{if frame.emitted<3{return Ok(Frontier::Corner(frame.locals[frame.emitted as usize]));}return Ok(if frame.indices_done{Frontier::TriangleFinish}else{Frontier::TriangleIndices});}
            return Ok(match self.phase{0..=6=>Frontier::Face,7=>if self.mesh.faces[self.face].smooth{Frontier::Corner(self.cursor)}else{Frontier::Ear},8=>if self.remaining==3{Frontier::TriangleSetup(TriangleFinish::Last)}else{Frontier::Ear},9=>Frontier::Ear,10=>Frontier::TriangleSetup(TriangleFinish::Fan),11=>Frontier::Edge(self.cursor),12=>Frontier::TriangleSetup(TriangleFinish::Clip),_=>return Err(refusal("original normal tessellation phase is invalid"))});
        }
        if self.metadata_done{return Ok(Frontier::Transfer);}
        if self.selected_faces.is_some(){return Ok(Frontier::Mark);}
        Ok(if !self.attribute_projection_complete(){Frontier::Attribute}else{match self.normal_metadata_phase{0|1=>Frontier::Materials,2=>Frontier::Textures,_=>Frontier::Mark}})
    }
    fn triangle_locals(&self,finish:TriangleFinish)->[usize;3]{match finish{TriangleFinish::Last=>[self.head,self.links[self.head].1,self.links[self.head].0],TriangleFinish::Clip=>[self.links[self.ear].0,self.ear,self.links[self.ear].1],TriangleFinish::Fan=>[self.head,self.cursor,self.links[self.cursor].1]}}
    fn pending_triangle_indices(&self)->Result<[u32;3],ValueError>{
        let frame=self.triangle_pending.ok_or_else(||refusal("original triangle frame is absent"))?;
        if self.mesh.faces[self.face].smooth{let mut result=[0;3];for(index,local)in frame.locals.into_iter().enumerate(){result[index]=frame.base.checked_add(u32::try_from(local).map_err(|_|refusal("original triangle local exceeds index range"))?).ok_or_else(||refusal("original triangle index overflow"))?;}Ok(result)}else{Ok([frame.base,frame.base.checked_add(1).ok_or_else(||refusal("original triangle index overflow"))?,frame.base.checked_add(2).ok_or_else(||refusal("original triangle index overflow"))?])}
    }
    /// 🧪️ Advances one original geometric ear predicate without emitting a triangle or copying source channels.
    pub(super) fn choose_triangle(&mut self)->Option<(TriangleFinish,[usize;3])>{
        match self.phase{
            8=>{if self.remaining==3{return Some((TriangleFinish::Last,self.triangle_locals(TriangleFinish::Last)));}let(prev,next)=self.links[self.ear];let cross=cross2(self.projected[prev],self.projected[self.ear],self.projected[next]);if if self.area>0.0{cross>1e-14}else{cross< -1e-14}{self.probe=self.head;self.phase=9;}else{self.reject_ear();}}
            9=>{let(prev,next)=self.links[self.ear];let k=self.probe;if k!=prev&&k!=self.ear&&k!=next&&point_in_triangle(self.projected[k],self.projected[prev],self.projected[self.ear],self.projected[next]){self.reject_ear();}else{self.probe=self.links[k].1;if self.probe==self.head{return Some((TriangleFinish::Clip,[prev,self.ear,next]));}}}
            10=>return Some((TriangleFinish::Fan,self.triangle_locals(TriangleFinish::Fan))),
            _=>unreachable!("original ear predicate requires its source phase"),
        }None
    }
    /// 🔗️ Applies the same original ear/fan continuation after a triangle has really reached its output buffers.
    pub(super) fn finish_triangle(&mut self,finish:TriangleFinish,locals:[usize;3]){
        match finish{TriangleFinish::Last=>{self.phase=11;self.cursor=0;},TriangleFinish::Clip=>{let[prev,ear,next]=locals;self.links[prev].1=next;self.links[next].0=prev;if ear==self.head{self.head=next;}self.remaining-=1;self.ear=self.head;self.candidates=0;self.phase=8;},TriangleFinish::Fan=>{self.cursor=locals[2];if self.links[self.cursor].1==self.head{self.phase=11;self.cursor=0;}}}
    }
    fn smooth_corners_done(&mut self){let n=self.hes.len();self.phase=if n==3||self.normal.length()<1e-8||self.scale==0.0||self.area.abs()<1e-14{10}else{8};self.ear=self.head;self.cursor=self.links[self.head].1;}
    fn finish_edge(&mut self){self.cursor+=1;if self.cursor==self.hes.len(){self.face+=1;self.phase=0;self.hes.clear();self.points.clear();self.projected.clear();self.links.clear();self.normal_sum=[0.0;3];self.scale=0.0;self.area=0.0;self.head=0;self.candidates=0;}}
    pub(super) fn normal_output_is_atomic(&self)->bool{matches!(self.normal_frontier(),Ok(Frontier::Face|Frontier::Corner(_)|Frontier::TriangleSetup(_)|Frontier::TriangleIndices|Frontier::TriangleFinish))||matches!(self.normal_frontier(),Ok(Frontier::Edge(_)))&&self.edge_pending.is_some()||matches!(self.normal_frontier(),Ok(Frontier::Attribute))&&self.attribute_stage==2}
    pub fn next_normal_copy_byte_demand(&self)->Result<usize,ValueError>{
        if self.cancelled{return Ok(0);}
        match self.normal_frontier()?{Frontier::Buffer=>self.next_buffer_copy_byte_demand(),Frontier::Metadata=>self.next_metadata_copy_byte_demand(),Frontier::Face=>self.next_face_copy_byte_demand(),Frontier::Corner(local)=>self.next_corner_copy_byte_demand(local),Frontier::TriangleSetup(_)=>Ok(3*std::mem::size_of::<usize>()+std::mem::size_of::<u32>()),Frontier::TriangleIndices=>self.next_triangle_copy_byte_demand(self.pending_triangle_indices()?),Frontier::TriangleFinish=>Ok(if self.triangle_pending.unwrap().finish==TriangleFinish::Clip{2*std::mem::size_of::<usize>()}else{0}),Frontier::Edge(local)=>self.next_edge_copy_byte_demand(local),Frontier::Attribute=>self.next_attribute_copy_byte_demand(),_=>Ok(0)}
    }
    pub fn next_normal_capacity_byte_demand(&self,copy:usize)->Result<usize,ValueError>{
        if self.cancelled{return Ok(0);}
        match self.normal_frontier()?{Frontier::Buffer=>self.next_buffer_capacity_byte_demand(copy),Frontier::Metadata=>self.next_metadata_capacity_byte_demand(copy),Frontier::Face=>self.next_face_capacity_byte_demand(copy),Frontier::Corner(local)=>self.next_corner_capacity_byte_demand(local,copy),Frontier::TriangleIndices=>self.next_triangle_capacity_byte_demand(self.pending_triangle_indices()?,copy),Frontier::Edge(local)=>self.next_edge_capacity_byte_demand(local,copy),Frontier::Attribute=>self.next_attribute_capacity_byte_demand(copy),_=>Ok(0)}
    }
    pub fn next_normal_release_byte_demand(&self)->Result<usize,ValueError>{if self.cancelled{return Ok(0);}match self.normal_frontier()?{Frontier::Attribute=>self.next_attribute_release_byte_demand(),_=>Ok(0)}}
    pub fn next_normal_depth_demand(&self)->Result<usize,ValueError>{
        if self.cancelled{return Ok(0);}
        match self.normal_frontier()?{Frontier::Buffer=>self.next_buffer_depth_demand(),Frontier::Metadata=>self.next_metadata_depth_demand(),Frontier::Face=>self.next_face_depth_demand(),Frontier::Corner(local)=>self.next_corner_depth_demand(local),Frontier::TriangleIndices=>self.next_triangle_depth_demand(self.pending_triangle_indices()?),Frontier::Edge(local)=>self.next_edge_depth_demand(local),Frontier::Attribute=>self.next_attribute_depth_demand(),Frontier::TriangleSetup(_)|Frontier::TriangleFinish|Frontier::Ear=>Ok(2),_=>Ok(1)}
    }
    /// 🧾️ Keeps the actual child receipt on success or failure through the original normal result boundary.
    pub fn normal_step_progress(&self)->RetainedCloneProgress{self.normal_receipt.0}
    /// ⏱️ Executes one actual original producer event with the caller's unchanged full wallet and source custody.
    pub fn step(&mut self,grant:RetainedCloneGrant)->Result<(MeshTessellationStep,RetainedCloneProgress),ValueError>{
        self.normal_receipt.0=Default::default();if self.cancelled{return Ok((MeshTessellationStep::Cancelled(self.progress()),self.normal_receipt.0));}
        if grant.maximum_items==0||grant.maximum_capacity_bytes<self.next_normal_capacity_byte_demand(grant.maximum_copy_bytes)?||grant.maximum_release_bytes<self.next_normal_release_byte_demand()?||grant.maximum_depth<self.next_normal_depth_demand()?||(self.normal_output_is_atomic()&&grant.maximum_copy_bytes<self.next_normal_copy_byte_demand()?){return Ok((MeshTessellationStep::Working(self.progress()),self.normal_receipt.0));}
        let frontier=self.normal_frontier()?;let added=match frontier{Frontier::Corner(local)=>self.next_corner_copy_byte_demand(local)?-std::mem::size_of::<u32>(),Frontier::TriangleIndices=>4*std::mem::size_of::<u32>(),Frontier::Edge(_)if self.edge_pending.is_some()=>10*std::mem::size_of::<f32>()+std::mem::size_of::<u32>()+std::mem::size_of::<u8>(),Frontier::Attribute if self.attribute_stage==2=>self.next_attribute_copy_byte_demand()?,_=>0};
        let output_bytes=self.normal_output_bytes.checked_add(added).ok_or_else(||refusal("original mesh preview byte extent overflow"))?;if output_bytes>self.maximum_preview_bytes{return Err(refusal("original mesh preview exceeds admitted output extent"));}
        let before=self.done;let result=self.normal_event(frontier,grant);match result{Ok(step)=>{self.normal_output_bytes=output_bytes;if self.done==before&&self.normal_receipt.0.copied_items!=0{self.done=self.done.checked_add(1).ok_or_else(||refusal("original normal progress overflow").with_retained_progress(self.normal_receipt.0))?;}Ok((step.unwrap_or_else(||MeshTessellationStep::Working(self.progress())),self.normal_receipt.0))},Err(error)=>{self.normal_receipt.0=error.retained_progress();Err(error)}}
    }
    fn normal_event(&mut self,frontier:Frontier,grant:RetainedCloneGrant)->Result<Option<MeshTessellationStep>,ValueError>{
        let one=RetainedCloneProgress {copied_items:1,..Default::default()};
        self.normal_receipt.0=match frontier{
            Frontier::Buffer=>self.reserve_buffer_step(grant)?.progress(),Frontier::Metadata=>self.capture_metadata_step(grant)?.progress(),Frontier::Face=>self.prepare_face_step(grant)?.progress(),
            Frontier::Corner(local)=>{let step=self.emit_corner_step(local,grant)?;if matches!(step,RetainedCloneStep::Complete(_)){if let Some(frame)=&mut self.triangle_pending{frame.emitted+=1;}else{self.cursor+=1;if self.cursor==self.hes.len(){self.smooth_corners_done();}}}step.progress()},
            Frontier::TriangleSetup(finish)=>{let locals=self.triangle_locals(finish);let base=if self.mesh.faces[self.face].smooth{self.base}else{u32::try_from(self.output.as_ref().unwrap().positions.len()/3).map_err(|_|refusal("original mesh triangle output exceeds index range"))?};self.triangle_pending=Some(TriangleFrame {locals,base,emitted:if self.mesh.faces[self.face].smooth{3}else{0},indices_done:false,finish});RetainedCloneProgress {copied_bytes:3*std::mem::size_of::<usize>()+std::mem::size_of::<u32>(),..one}},
            Frontier::TriangleIndices=>{let step=self.emit_triangle_step(self.pending_triangle_indices()?,grant)?;if matches!(step,RetainedCloneStep::Complete(_)){self.triangle_pending.as_mut().unwrap().indices_done=true;}step.progress()},
            Frontier::TriangleFinish=>{let frame=self.triangle_pending.take().unwrap();let copy=if frame.finish==TriangleFinish::Clip{2*std::mem::size_of::<usize>()}else{0};self.finish_triangle(frame.finish,frame.locals);RetainedCloneProgress {copied_bytes:copy,..one}},
            Frontier::Ear=>{if self.phase==7{self.smooth_corners_done();}else if self.choose_triangle().is_some(){self.phase=12;}one},
            Frontier::Edge(local)=>{let step=self.emit_edge_step(local,grant)?;if matches!(step,RetainedCloneStep::Complete(_)){self.finish_edge();}step.progress()},
            Frontier::Attribute=>self.project_attribute_step(grant)?.progress(),
            Frontier::Materials=>{let out=self.output.as_mut().unwrap();if !out.materials.terminal_is_empty(){return Err(refusal("original output material destination is not empty"));}out.materials=std::mem::take(&mut self.mesh.materials);self.normal_metadata_phase=2;one},
            Frontier::Textures=>{let out=self.output.as_mut().unwrap();if !out.textures.terminal_is_empty(){return Err(refusal("original output texture destination is not empty"));}out.textures=std::mem::take(&mut self.mesh.textures);self.normal_metadata_phase=3;one},
            Frontier::Mark=>{self.metadata_done=true;one},
            Frontier::Transfer=>{self.normal_receipt.0=one;return Ok(Some(MeshTessellationStep::Done(self.output.take().unwrap())));},
        };Ok(None)
    }
}
