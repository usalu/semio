//! 🧳️ Drawing sparse field preflight receives payloads and closes live post readers before central commit.
use super::{DrawingDiff,DrawingLayerPatch};
use semio_s_artifact_draw_drawing_model::{DrawingLayerNode,FillStyle,StrokeStyle,PathSegment,layer_base_mut,snapshot::{DrawingSnapshot,lookup::original::{DrawingOriginalLookup,DrawingOriginalLayerAddress}}};
use protocol::{RetainedDiff,RetainedDiffApply,RetainedDiffPreflightAdvance,ApplyCapability,MutationMessage,RetainedFieldPreparation, RetainedDone as Done, RetainedReply as Reply, RetainedFlight as Flight, retained_conclude as conclude, retained_native_child as native_child, retained_routed, retained_polled as polled};
use semio_framework_value::{OriginalRetirementCloseLoan, OriginalRetirementFailure, OriginalRetirementOutcome, OriginalRetirementPoll, RetainedAllocationEffect, RetainedWorkPoll, RetirementPollExit, ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose,retained_clone_grant_admits}};
use semio_framework_value_native_owned::{native_decoding::{NativeReceive,NativeReceiver,NativeDecodeControl},owned::NativeAdvanceSelection};
use std::mem::{ManuallyDrop,size_of};
type Text=semio_framework_value::paged::PagedUtf8<{usize::MAX}>;
type Segments=semio_framework_value_persistent::list::PagedList<PathSegment,{usize::MAX}>;
type TextReceiver=<Text as NativeReceive<String>>::Receiver;
fn primary(e:semio_framework_value_native_owned::native_decoding::OriginalReceivingPrimary)->ValueError{let p=e.progress;e.primary.or(e.quote_refusal).or(e.allocation_refusal).unwrap_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"original native construction refused")).with_retained_progress(p)}
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Drawing retained field apply custody changed")}
fn bad()->ValueError{ValueError::literal(ValueRefusalKind::InvalidValue,"Drawing retained field patch failed read-only admission")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn metadata()->usize{2*size_of::<DrawingLayerPatch>()+2*size_of::<Option<Displaced>>()+256}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(metadata()).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-metadata(),maximum_depth:g.maximum_depth-1,..g}}
fn paid(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_items=p.copied_items.max(1);p.copied_bytes+=metadata();p}
fn artboard_only(d:&DrawingDiff)->bool{d.artboard.is_some()&&d.schema.is_none()&&d.id.is_none()&&d.title.is_none()&&d.layers.is_none()&&d.assets.is_none()}
fn empty(d:&DrawingDiff)->bool{d.schema.is_none()&&d.id.is_none()&&d.title.is_none()&&d.layers.is_none()&&d.assets.is_none()&&d.artboard.is_none()}
fn patch<'a>(d:RetainedCloneRef<'a,DrawingDiff>)->Result<RetainedCloneRef<'a,DrawingLayerPatch>,ValueError>{let v=d.get();if v.schema.is_some()||v.id.is_some()||v.title.is_some()||v.assets.is_some()||v.artboard.is_some(){return Err(bad())}let rows=v.layers.as_ref().ok_or_else(bad)?;if !rows.removed.is_empty()||!rows.inserted.is_empty()||!rows.moved.is_empty()||rows.modified.len()!=1{return Err(bad())}Ok(d.project(3,|d|d.layers.as_ref().unwrap()).project(3,|l|&l.modified).project(0,|l|&l[0]).project(1,|r|&r.patch))}
fn id<'a>(d:RetainedCloneRef<'a,DrawingDiff>)->RetainedCloneRef<'a,String>{d.project(3,|d|d.layers.as_ref().unwrap()).project(3,|l|&l.modified).project(0,|l|&l[0]).project(0,|r|&r.id)}
fn text<'a>(p:RetainedCloneRef<'a,DrawingLayerPatch>)->Option<RetainedCloneRef<'a,String>>{let v=p.get();if v.name.is_some(){Some(p.project(2,|p|p.name.as_ref().unwrap()))}else if v.blend_mode.is_some(){Some(p.project(4,|p|p.blend_mode.as_ref().unwrap()))}else if v.boolean_operation.is_some(){Some(p.project(10,|p|p.boolean_operation.as_ref().unwrap()))}else if v.text_content.is_some(){Some(p.project(14,|p|p.text_content.as_ref().unwrap()))}else if v.image_key.is_some(){Some(p.project(17,|p|p.image_key.as_ref().unwrap()))}else{None}}
fn fill<'a>(p:RetainedCloneRef<'a,DrawingLayerPatch>)->RetainedCloneRef<'a,Option<FillStyle>>{p.project(8,|p|&p.fill.as_ref().unwrap().value)}
fn stroke<'a>(p:RetainedCloneRef<'a,DrawingLayerPatch>)->RetainedCloneRef<'a,Option<StrokeStyle>>{p.project(9,|p|&p.stroke.as_ref().unwrap().value)}
fn segments<'a>(p:RetainedCloneRef<'a,DrawingLayerPatch>)->RetainedCloneRef<'a,Segments>{p.project(13,|p|p.path_segments.as_ref().unwrap())}
fn validate(p:&DrawingLayerPatch,n:&DrawingLayerNode)->Result<(),ValueError>{
 let payloads=[p.name.is_some(),p.blend_mode.is_some(),p.boolean_operation.is_some(),p.text_content.is_some(),p.image_key.is_some(),p.fill.is_some(),p.stroke.is_some(),p.path_segments.is_some()].into_iter().filter(|v|*v).count();if payloads>1||p.layer.is_some()||p.shape_coordinates.len()>1{return Err(bad())}
 if p.opacity.is_some_and(|v|!v.is_finite())||p.text_size.is_some_and(|v|!v.is_finite()||v<=0.0)||p.image_width.is_some_and(|v|!v.is_finite()||v<=0.0)||p.image_height.is_some_and(|v|!v.is_finite()||v<=0.0){return Err(bad())}
 if let Some(t)=&p.transform{if ![t.x,t.y,t.scale_x,t.scale_y,t.shear,t.rotation].into_iter().all(f64::is_finite){return Err(bad())}}
 if let Some(t)=&p.trace_params{if !t.threshold.is_finite()||!t.simplify_epsilon.is_finite(){return Err(bad())}}
 if p.boolean_operation.is_some()&&!matches!(n,DrawingLayerNode::Boolean(_))||p.trace_params.is_some()&&!matches!(n,DrawingLayerNode::Trace(_))||p.path_segments.is_some()&&!matches!(n,DrawingLayerNode::Path(_))||(p.text_content.is_some()||p.text_size.is_some()||p.font_family.is_some())&&!matches!(n,DrawingLayerNode::Text(_))||(p.image_key.is_some()||p.image_width.is_some()||p.image_height.is_some())&&!matches!(n,DrawingLayerNode::Image(_))||p.isolation.is_some()&&!matches!(n,DrawingLayerNode::Group(_)){return Err(bad())}
 if let Some(c)=p.shape_coordinates.first(){let DrawingLayerNode::Shape(s)=n else{return Err(bad())};semio_s_artifact_draw_drawing_model::shape_geometry::shape_coordinate(s,&c.field,c.index).map_err(|_|bad())?;if !c.value.is_finite()||(c.field.nonnegative()&&c.value<0.0){return Err(bad())}}
 Ok(())
}
#[derive(semio_framework_value::RetireOwned)]enum Displaced{Text(Text),Fill(Option<FillStyle>),Stroke(Option<StrokeStyle>),Segments(Segments)}
/// 🪡️ Original diff fields and resolved native paths stay retained until commit or cancellation closure.
pub struct DrawingRetainedFieldApply{flight:Flight,multi:transforms::DrawingRetainedTransformsApply,artboard:bool,empty:bool,original:DrawingOriginalLookup,post:DrawingOriginalLookup,text_receiver:Option<TextReceiver>,fill_receiver:RetainedFieldPreparation<Option<FillStyle>>,stroke_receiver:RetainedFieldPreparation<Option<StrokeStyle>>,segments_receiver:RetainedFieldPreparation<Segments>,ready_text:ManuallyDrop<Option<Text>>,ready_fill:ManuallyDrop<Option<Option<FillStyle>>>,ready_stroke:ManuallyDrop<Option<Option<StrokeStyle>>>,ready_segments:ManuallyDrop<Option<Segments>>,trash:ManuallyDrop<Option<Displaced>>,retirement:RetainedCloneClose,at:Option<DrawingOriginalLayerAddress>,phase:u8,closing:bool}
impl RetainedDiff<DrawingSnapshot> for DrawingDiff{type Apply=DrawingRetainedFieldApply;}
impl RetainedDiffApply<DrawingSnapshot,DrawingDiff> for DrawingRetainedFieldApply{
 fn new()->Self{Self{flight:Default::default(),multi:Default::default(),artboard:false,empty:false,original:Default::default(),post:Default::default(),text_receiver:None,fill_receiver:Default::default(),stroke_receiver:Default::default(),segments_receiver:Default::default(),ready_text:ManuallyDrop::new(None),ready_fill:ManuallyDrop::new(None),ready_stroke:ManuallyDrop::new(None),ready_segments:ManuallyDrop::new(None),trash:ManuallyDrop::new(None),retirement:Default::default(),at:None,phase:0,closing:false}}
 fn preflight_demand(&self,d:RetainedCloneRef<'_,DrawingDiff>,b:RetainedCloneRef<'_,DrawingSnapshot>,p:RetainedCloneRef<'_,DrawingSnapshot>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{if d.get().layers.as_ref().is_some_and(|l|l.modified.len()>1){return self.multi.demand(d,b,p,body)}if self.closing||self.phase>=7{return Err(invalid())}if (empty(d.get())||artboard_only(d.get()))&&self.phase==0{return Ok(fixed(metadata()))}let patch=patch(d)?;let inner=body.saturating_sub(metadata());Ok(match self.phase{
  0=>nested(self.original.demand(b,id(d),inner)?)?,1=>nested(self.post.demand(p,id(d),inner)?)?,
  2=>if let Some(t)=text(patch){if let Some(r)=self.text_receiver.as_ref(){if r.output_ready(){fixed(metadata())}else{nested(r.demands(t,inner,maximum.saturating_sub(1))?)?}}else{nested(<Text as NativeReceive<String>>::receiver().demands(t,inner,maximum.saturating_sub(1))?)?}}else if patch.get().fill.is_some(){nested(self.fill_receiver.demand(fill(patch),inner,maximum.saturating_sub(1))?)?}else if patch.get().stroke.is_some(){nested(self.stroke_receiver.demand(stroke(patch),inner,maximum.saturating_sub(1))?)?}else if patch.get().path_segments.is_some(){nested(self.segments_receiver.demand(segments(patch),inner,maximum.saturating_sub(1))?)?}else{fixed(metadata())},
  3=>if text(patch).is_some(){nested(self.text_receiver.as_ref().ok_or_else(invalid)?.take_demand()?)?}else if patch.get().fill.is_some(){nested(self.fill_receiver.exchange_demand()?)?}else if patch.get().stroke.is_some(){nested(self.stroke_receiver.exchange_demand()?)?}else if patch.get().path_segments.is_some(){nested(self.segments_receiver.exchange_demand()?)?}else{fixed(metadata())},
  4=>nested(self.post.close_demand(inner)?)?,5=>nested(self.original.close_demand(inner)?)?,6=>fixed(metadata()),_=>return Err(invalid())
 })}
 fn preflight_work_demand(&self,d:RetainedCloneRef<'_,DrawingDiff>,b:RetainedCloneRef<'_,DrawingSnapshot>,p:RetainedCloneRef<'_,DrawingSnapshot>,body:usize,maximum:u64)->Result<u64,ValueError>{
    self.preflight_demand(d,b,p,body,maximum)?;
    if d.get().layers.as_ref().is_some_and(|rows|rows.modified.len()>1){return self.multi.work_demand(d,b,p,body)}
    if (empty(d.get())||artboard_only(d.get()))&&self.phase==0{return Ok(1)}
    let patch=patch(d)?;
    let inner=body.saturating_sub(metadata());
    let limit=maximum.saturating_sub(1);
    let child_work=match self.phase{
        0=>self.original.advance_work_demand(b,id(d))?,
        1=>self.post.advance_work_demand(p,id(d))?,
        2=>{
            if let Some(source)=text(patch){
                match self.text_receiver.as_ref(){
                    Some(receiver) if !receiver.output_ready()=>receiver.advance_work_demand(source,inner)?,
                    _=>0,
                }
            }else if patch.get().fill.is_some(){self.fill_receiver.advance_work_demand(fill(patch),inner,limit)?}
            else if patch.get().stroke.is_some(){self.stroke_receiver.advance_work_demand(stroke(patch),inner,limit)?}
            else if patch.get().path_segments.is_some(){self.segments_receiver.advance_work_demand(segments(patch),inner,limit)?}
            else{0}
        },
        3=>{
            if text(patch).is_some(){self.text_receiver.as_ref().ok_or_else(invalid)?.take_work_demand()?}
            else{1}
        },
        4=>self.post.close_work_demand(inner)?,
        5=>self.original.close_work_demand(inner)?,
        _=>0,
    };
    child_work.checked_add(1).ok_or_else(invalid)
 }
 fn preflight_reply<'r>(&mut self,d:RetainedCloneRef<'_,DrawingDiff>,b:RetainedCloneRef<'_,DrawingSnapshot>,p:RetainedCloneRef<'_,DrawingSnapshot>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,control:&mut NativeDecodeControl<'_>,reply:Reply<'r,RetainedDiffPreflightAdvance>,recover:bool)->Result<Done<'r,RetainedDiffPreflightAdvance>,ValueError>{
    let base=effect.performed_work();
    let grant=effect.grant();
    let mut work=0;
    let result=(|| -> Result<RetainedDiffPreflightAdvance,RetirementPollExit>{
        if d.get().layers.as_ref().is_some_and(|rows|rows.modified.len()>1){
            if recover{return Err(invalid().into())}
            return Ok(self.multi.preflight(d,b,p,body,effect)?)
        }
        let inner=body.saturating_sub(metadata());
        let mut progress=paid(Default::default());
        if !recover{
            let demand=self.preflight_demand(d,b,p,body,maximum)?;
            let selected=self.preflight_work_demand(d,b,p,body,maximum)?;
            if selected>maximum||selected>effect.remaining_work_units()?||!retained_clone_grant_admits(grant,demand){
                return Ok(RetainedDiffPreflightAdvance::Progress(Default::default()))
            }
            if !effect.performed(1)?{return Err(invalid().into())}
            work=1;
            if artboard_only(d.get())&&self.phase==0{
                let artboard=d.get().artboard.as_ref().unwrap();
                if !artboard.as_ref().is_none_or(|a|a.width.is_finite()&&a.width>0.0&&a.height.is_finite()&&a.height>0.0){
                    return Err(bad().with_retained_progress(progress).into())
                }
                self.artboard=true;
                self.phase=7;
                return Ok(RetainedDiffPreflightAdvance::Ready(progress,true))
            }
            if empty(d.get())&&self.phase==0{
                self.empty=true;
                self.phase=7;
                return Ok(RetainedDiffPreflightAdvance::Ready(progress,true))
            }
        }
        let patch=patch(d)?;
        let child_grant=child(grant);
        let limit=maximum.saturating_sub(1);
        macro_rules! prepare_field {
            ($field:ident,$source:expr,$route:expr)=>{
                retained_routed!(self.flight,effect,recover,$route,inner,limit,None,work,
                    |r| self.$field.advance_into($source,inner,limit,&mut effect.nested(child_grant),r),
                    self.$field.recover_advance_into($source,inner,limit,&mut effect.nested(child_grant),r))
            }
        }
        match self.phase{
            0=>{
                progress=paid(self.original.advance(b,id(d),inner,&mut effect.nested(child_grant))?.progress());
                if self.original.ready(){
                    let at=self.original.address()?.ok_or_else(bad)?;
                    validate(patch.get(),at.node(b.get()).ok_or_else(bad)?)?;
                    self.at=Some(at);
                    self.phase=1;
                }
            },
            1=>{
                progress=paid(self.post.advance(p,id(d),inner,&mut effect.nested(child_grant))?.progress());
                if self.post.ready(){
                    let at=self.post.address()?.ok_or_else(bad)?;
                    validate(patch.get(),at.node(p.get()).ok_or_else(bad)?)?;
                    self.at=Some(at);
                    self.phase=2;
                }
            },
            2=>{
                if let Some(source)=text(patch){
                    if let Some(receiver)=self.text_receiver.as_mut(){
                        if !receiver.output_ready()||recover{
                            let selection=if recover{
                                match self.flight.route(){
                                    Some(1)=>NativeAdvanceSelection::Local{work:0},
                                    Some(2)=>NativeAdvanceSelection::Nested,
                                    _=>return Err(invalid().into()),
                                }
                            }else{receiver.select_advance(source,inner,limit)?};
                            let step=match selection{
                                NativeAdvanceSelection::Local{..}=>retained_routed!(self.flight,effect,recover,1,inner,limit,None,work,
                                    |r| receiver.advance_local(source,inner,&mut effect.nested(child_grant),r),
                                    receiver.recover_advance_local(source,inner,&mut effect.nested(child_grant),r)).map_err(primary)?,
                                NativeAdvanceSelection::Nested=>{
                                    let witness=control.source_witness()?;
                                    polled(&mut self.flight,effect,recover,2,inner,&mut work,
                                        |_| receiver.advance_nested(source,control,witness,inner,limit,child_grant))?
                                        .map_err(|error|ValueError::literal(ValueRefusalKind::InvalidValue,"Drawing retained text receiving failed").with_retained_progress(error.retained_progress()))?
                                },
                            };
                            progress=paid(step.progress());
                        }
                        if receiver.output_ready(){self.phase=3;}
                    }else{self.text_receiver=Some(<Text as NativeReceive<String>>::receiver());}
                }else if patch.get().fill.is_some(){
                    progress=paid(prepare_field!(fill_receiver,fill(patch),3)?.progress());
                    if self.fill_receiver.value().is_some(){self.phase=3;}
                }else if patch.get().stroke.is_some(){
                    progress=paid(prepare_field!(stroke_receiver,stroke(patch),4)?.progress());
                    if self.stroke_receiver.value().is_some(){self.phase=3;}
                }else if patch.get().path_segments.is_some(){
                    progress=paid(prepare_field!(segments_receiver,segments(patch),5)?.progress());
                    if self.segments_receiver.value().is_some(){self.phase=3;}
                }else{
                    self.post.begin_close();
                    self.original.begin_close();
                    self.phase=4;
                }
            },
            3=>{
                if text(patch).is_some(){
                    let receiver=self.text_receiver.as_mut().ok_or_else(invalid)?;
                    progress=paid(retained_routed!(self.flight,effect,recover,6,inner,limit,None,work,
                        |r| receiver.take(&mut self.ready_text,&mut effect.nested(child_grant),r),
                        receiver.recover_take(&mut self.ready_text,&mut effect.nested(child_grant),r)).map_err(primary)?.progress());
                }else if patch.get().fill.is_some(){
                    progress=paid(native_child(effect,&mut work,|e,r|self.fill_receiver.take_into(&mut self.ready_fill,&mut e.nested(child_grant),r))??.progress());
                }else if patch.get().stroke.is_some(){
                    progress=paid(native_child(effect,&mut work,|e,r|self.stroke_receiver.take_into(&mut self.ready_stroke,&mut e.nested(child_grant),r))??.progress());
                }else if patch.get().path_segments.is_some(){
                    progress=paid(native_child(effect,&mut work,|e,r|self.segments_receiver.take_into(&mut self.ready_segments,&mut e.nested(child_grant),r))??.progress());
                }
                self.post.begin_close();
                self.original.begin_close();
                self.phase=4;
            },
            4=>{
                progress=paid(self.post.close_step(inner,&mut effect.nested(child_grant))?.progress());
                if self.post.terminal_is_empty(){self.phase=5;}
            },
            5=>{
                progress=paid(self.original.close_step(inner,&mut effect.nested(child_grant))?.progress());
                if self.original.terminal_is_empty(){self.phase=6;}
            },
            6=>self.phase=7,
            _=>return Err(invalid().into()),
        }
        Ok(if self.phase==7{RetainedDiffPreflightAdvance::Ready(progress,true)}else{RetainedDiffPreflightAdvance::Progress(progress)})
    })();
    let work=work.max(effect.performed_work().saturating_sub(base));
    conclude(effect,base,result,work,RetainedDiffPreflightAdvance::progress,reply)
 }
 fn take_messages(&mut self)->Vec<MutationMessage>{Vec::new()}
 fn commit_demand(&self,d:RetainedCloneRef<'_,DrawingDiff>,b:RetainedCloneRef<'_,DrawingSnapshot>,post:&DrawingSnapshot,_:usize)->Result<RetirementDemand,ValueError>{if d.get().layers.as_ref().is_some_and(|l|l.modified.len()>1){return self.multi.commit_demand(d,b,post)}if self.closing||self.phase!=7{return Err(invalid())}if self.artboard{if !artboard_only(d.get()){return Err(invalid())}return Ok(fixed(metadata()))}if self.empty{if !empty(d.get()){return Err(invalid())}return Ok(fixed(metadata()))}let p=patch(d)?;let at=self.at.ok_or_else(invalid)?;validate(p.get(),at.node(post).ok_or_else(invalid)?)?;Ok(fixed(metadata()+2*size_of::<DrawingLayerNode>()))}
 fn commit_work_demand(&self,d:RetainedCloneRef<'_,DrawingDiff>,b:RetainedCloneRef<'_,DrawingSnapshot>,post:&DrawingSnapshot,body:usize)->Result<u64,ValueError>{
    self.commit_demand(d,b,post,body)?;
    Ok(1)
 }
 fn commit(&mut self,d:RetainedCloneRef<'_,DrawingDiff>,b:RetainedCloneRef<'_,DrawingSnapshot>,post:&mut DrawingSnapshot,effect:&mut RetainedAllocationEffect<'_>,capability:ApplyCapability)->OriginalRetirementOutcome{
    let base=effect.performed_work();
    let grant=effect.grant();
    let result=(||{
        if !retained_clone_grant_admits(grant,self.commit_demand(d,b,post,grant.maximum_copy_bytes)?)||effect.remaining_work_units()?==0{
            return Ok(RetainedCloneStep::Progress(Default::default()))
        }
        if !effect.performed(1)?{return Err(invalid())}
        self.commit_granted(d,b,post,grant,capability)
    })().map_err(OriginalRetirementFailure::from);
    let work=effect.performed_work().saturating_sub(base);
    protocol::retained_settle(effect,base,result,work)
 }
 fn begin_close(&mut self){
    self.closing=true;
    self.multi.begin_close();
    self.post.begin_close();
    self.original.begin_close();
 }
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{
    if !self.multi.terminal_is_empty(){return self.multi.close_demand(body)}
    if !self.post.terminal_is_empty(){return self.post.close_demand(body)}
    if !self.original.terminal_is_empty(){return self.original.close_demand(body)}
    if let Some(receiver)=self.text_receiver.as_ref(){
        return if receiver.terminal_is_empty(){Ok(fixed(2*size_of::<Option<TextReceiver>>()))}
            else if !receiver.is_closing(){receiver.begin_close_demand()}
            else{receiver.close_demands(body)}
    }
    macro_rules! field_demand {
        ($field:ident) => {
            if !self.$field.terminal_is_empty(){
                return if self.$field.close_started(){self.$field.close_demand(body)}else{Ok(self.$field.begin_close_demand())}
            }
        }
    }
    field_demand!(fill_receiver);
    field_demand!(stroke_receiver);
    field_demand!(segments_receiver);
    if !self.retirement.is_empty(){return self.retirement.next_owner_demands::<Displaced>(false,&None,body)}
    if self.trash.is_some(){return self.retirement.next_owner_demands::<Displaced>(true,&None,body)}
    if self.ready_text.is_some(){return self.retirement.next_owner_demands::<Text>(true,&None,body)}
    if self.ready_fill.is_some(){return self.retirement.next_owner_demands::<Option<FillStyle>>(true,&None,body)}
    if self.ready_stroke.is_some(){return self.retirement.next_owner_demands::<Option<StrokeStyle>>(true,&None,body)}
    if self.ready_segments.is_some(){return self.retirement.next_owner_demands::<Segments>(true,&None,body)}
    Ok(Default::default())
 }
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
    if !self.multi.terminal_is_empty(){return self.multi.close_work_demand(body)}
    if !self.post.terminal_is_empty(){return self.post.close_work_demand(body)}
    if !self.original.terminal_is_empty(){return self.original.close_work_demand(body)}
    if let Some(receiver)=self.text_receiver.as_ref(){
        return if receiver.terminal_is_empty(){Ok(1)}
            else if !receiver.is_closing(){receiver.begin_close_work_demand()}
            else{receiver.close_work_demand(body)}
    }
    macro_rules! field_work {
        ($field:ident) => {
            if !self.$field.terminal_is_empty(){
                return if self.$field.close_started(){self.$field.close_work_demand(body)}else{Ok(self.$field.begin_close_work_demand())}
            }
        }
    }
    field_work!(fill_receiver);
    field_work!(stroke_receiver);
    field_work!(segments_receiver);
    if !self.retirement.is_empty(){return self.retirement.next_owner_work::<Displaced>(false,&None,body)}
    Ok(u64::from(self.trash.is_some()||self.ready_text.is_some()||self.ready_fill.is_some()||self.ready_stroke.is_some()||self.ready_segments.is_some()))
 }
 fn requires_original_birth(&self)->bool{self.selected_original(true)}
 fn requires_original_control(&self)->bool{self.selected_original(false)}
 fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
    if !self.multi.terminal_is_empty(){return self.multi.close_original_birth_step(original,grant)}
    if let Some(receiver)=self.text_receiver.as_mut(){return receiver.close_original_birth_step(original,grant)}
    if !self.fill_receiver.terminal_is_empty(){return self.fill_receiver.close_original_birth_step(original,grant)}
    if !self.stroke_receiver.terminal_is_empty(){return self.stroke_receiver.close_original_birth_step(original,grant)}
    if !self.segments_receiver.terminal_is_empty(){return self.segments_receiver.close_original_birth_step(original,grant)}
    self.retirement.step_original_birth(original,grant)
 }
 fn close_reply<'r>(&mut self,mut original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Result<Done<'r>,ValueError>{
    let grant=effect.grant();
    let base=effect.performed_work();
    let mut work=0;
    let result=(|| -> Result<RetainedCloneStep,RetirementPollExit>{
        if !self.closing{return Err(invalid().into())}
        if !recover{
            if !retained_clone_grant_admits(grant,self.close_demand(body)?)
                ||self.close_work_demand(body)?>effect.remaining_work_units()?{
                return Ok(RetainedCloneStep::Progress(Default::default()))
            }
            if !self.multi.terminal_is_empty()&&!self.multi.paths_close_selected(){return Ok(RetainedCloneStep::Progress(self.multi.close_step(body,effect)?.progress()))}
            if !self.post.terminal_is_empty(){return Ok(RetainedCloneStep::Progress(self.post.close_step(body,effect)?.progress()))}
            if !self.original.terminal_is_empty(){return Ok(RetainedCloneStep::Progress(self.original.close_step(body,effect)?.progress()))}
        }
        let loaned=if recover{self.flight.loaned().unwrap_or(false)}else{self.requires_original_control()};
        if loaned&&original.is_none(){return Err(invalid().into())}
        if if recover{self.flight.route()==Some(7)}else{self.multi.paths_close_selected()}{
            let loan=if loaned{original.as_deref_mut()}else{None};
            let step=retained_routed!(self.flight,effect,recover,7,body,0,loan,work,
                |r| self.multi.close_paths_into(if loaned{original.as_deref_mut()}else{None},body,effect,r,false),
                self.multi.close_paths_into(if loaned{original.as_deref_mut()}else{None},body,effect,r,true));
            return Ok(RetainedCloneStep::Progress(step?.progress()))
        }
        let missing=invalid;
        if let Some(receiver)=self.text_receiver.as_mut(){
            if !recover&&receiver.terminal_is_empty(){
                if !effect.performed(1)?{return Err(invalid().into())}
                self.text_receiver=None;
                return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:2*size_of::<Option<TextReceiver>>(),..Default::default()}))
            }
            let step=if !receiver.is_closing()||self.flight.route()==Some(1){
                retained_routed!(self.flight,effect,recover,1,body,0,None,work,
                    |r| receiver.begin_close_into(effect,r),receiver.recover_begin_close_into(effect,r))
            }else{
                let loan=if loaned{original.as_deref_mut()}else{None};
                retained_routed!(self.flight,effect,recover,2,body,0,loan,work,
                    |r| if loaned{receiver.close_original_step(original.as_deref_mut().ok_or_else(missing)?,body,effect,r)}else{receiver.close_step(body,effect,r)},
                    if loaned{receiver.recover_close_original_step(original.as_deref_mut().ok_or_else(missing)?,body,effect,r)}else{receiver.recover_close_step(body,effect,r)})
            };
            return Ok(RetainedCloneStep::Progress(step?.progress()))
        }
        macro_rules! close_field {
            ($field:ident,$route:expr) => {
                if !self.$field.terminal_is_empty()||self.flight.route()==Some($route){
                    if !self.$field.close_started(){
                        let turn=self.$field.begin_close_paid(effect);
                        return Ok(RetainedCloneStep::Progress(turn.result?))
                    }
                    let loan=if loaned{original.as_deref_mut()}else{None};
                    let step=retained_routed!(self.flight,effect,recover,$route,body,0,loan,work,
                        |r| if loaned{self.$field.close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r)}else{self.$field.close_into(body,effect,r)},
                        if loaned{self.$field.recover_close_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r)}else{self.$field.recover_close_into(body,effect,r)});
                    return Ok(RetainedCloneStep::Progress(step?.progress()))
                }
            }
        }
        close_field!(fill_receiver,3);
        close_field!(stroke_receiver,4);
        close_field!(segments_receiver,5);
        if !self.retirement.is_empty()||self.flight.route()==Some(6){
            let loan=if loaned{original.as_deref_mut()}else{None};
            let step=retained_routed!(self.flight,effect,recover,6,body,0,loan,work,
                |r| if loaned{self.retirement.step_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r)}else{self.retirement.step_into(body,effect,r)},
                if loaned{self.retirement.recover_step_original_into(original.as_deref_mut().ok_or_else(missing)?,body,effect,r)}else{self.retirement.recover_step_into(body,effect,r)});
            return Ok(RetainedCloneStep::Progress(step?.progress()))
        }
        if recover{return Err(invalid().into())}
        if let Some(step)=self.retirement.begin_granted(&mut self.trash,effect).result?{return Ok(step)}
        if let Some(step)=self.retirement.begin_granted(&mut self.ready_text,effect).result?{return Ok(step)}
        if let Some(step)=self.retirement.begin_granted(&mut self.ready_fill,effect).result?{return Ok(step)}
        if let Some(step)=self.retirement.begin_granted(&mut self.ready_stroke,effect).result?{return Ok(step)}
        if let Some(step)=self.retirement.begin_granted(&mut self.ready_segments,effect).result?{return Ok(step)}
        Ok(RetainedCloneStep::Complete(Default::default()))
    })();
    let work=work.max(effect.performed_work().saturating_sub(base));
    conclude(effect,base,result,work,|step|step.progress(),reply)
 }
 fn terminal_is_empty(&self)->bool{self.closing&&self.flight.idle()&&self.multi.terminal_is_empty()&&self.post.terminal_is_empty()&&self.original.terminal_is_empty()&&self.text_receiver.is_none()&&self.fill_receiver.terminal_is_empty()&&self.stroke_receiver.terminal_is_empty()&&self.segments_receiver.terminal_is_empty()&&self.ready_text.is_none()&&self.ready_fill.is_none()&&self.ready_stroke.is_none()&&self.ready_segments.is_none()&&self.trash.is_none()&&self.retirement.is_empty()}
}
impl DrawingRetainedFieldApply{
 fn selected_original(&self,birth:bool)->bool{
    if !self.multi.terminal_is_empty(){return if birth{self.multi.requires_original_birth()}else{self.multi.requires_original_control()}}
    if !self.post.terminal_is_empty()||!self.original.terminal_is_empty(){return false}
    if let Some(receiver)=self.text_receiver.as_ref(){
        return receiver.is_closing()&&if birth{receiver.requires_original_birth()}else{receiver.requires_original_control()}
    }
    macro_rules! selected_field {
        ($field:ident) => {
            if !self.$field.terminal_is_empty(){return self.$field.close_started()&&if birth{self.$field.requires_original_birth()}else{self.$field.requires_original_control()}}
        }
    }
    selected_field!(fill_receiver);
    selected_field!(stroke_receiver);
    selected_field!(segments_receiver);
    if birth{self.retirement.requires_original_birth()}else{self.retirement.requires_original_control()}
 }
 fn commit_granted(&mut self,d:RetainedCloneRef<'_,DrawingDiff>,b:RetainedCloneRef<'_,DrawingSnapshot>,post:&mut DrawingSnapshot,g:RetainedCloneGrant,_capability:ApplyCapability)->Result<RetainedCloneStep,ValueError>{if d.get().layers.as_ref().is_some_and(|l|l.modified.len()>1){return self.multi.commit(d,b,post,g,_capability)}let demand=self.commit_demand(d,b,post,g.maximum_copy_bytes)?;if !retained_clone_grant_admits(g,demand){return Ok(RetainedCloneStep::Progress(Default::default()))}if self.artboard{post.artboard=d.get().artboard.as_ref().ok_or_else(invalid)?.as_ref().map(|a|semio_s_artifact_draw_drawing_model::DrawingArtboard{width:a.width,height:a.height});self.phase=8;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}if self.empty{self.phase=8;return Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}let p=patch(d)?.get();let n=self.at.ok_or_else(invalid)?.node_mut(post).ok_or_else(invalid)?;let base=layer_base_mut(n);if let Some(v)=p.visible{base.visible=v}if let Some(v)=p.locked{base.locked=v}if let Some(v)=p.opacity{base.opacity=v}if let Some(v)=p.fill_rule{base.attributes.fill_rule=v}if let Some(t)=&p.transform{base.transform=semio_s_artifact_draw_drawing_model::DrawingTransform{x:t.x,y:t.y,scale_x:t.scale_x,scale_y:t.scale_y,shear:t.shear,rotation:t.rotation}}if p.name.is_some(){*self.trash=Some(Displaced::Text(std::mem::replace(&mut base.name,self.ready_text.take().ok_or_else(invalid)?)))}else if p.blend_mode.is_some(){*self.trash=Some(Displaced::Text(std::mem::replace(&mut base.blend_mode,self.ready_text.take().ok_or_else(invalid)?)))}else if p.fill.is_some(){*self.trash=Some(Displaced::Fill(std::mem::replace(&mut base.attributes.fill,self.ready_fill.take().ok_or_else(invalid)?)))}else if p.stroke.is_some(){*self.trash=Some(Displaced::Stroke(std::mem::replace(&mut base.attributes.stroke,self.ready_stroke.take().ok_or_else(invalid)?)))}
 match n{DrawingLayerNode::Boolean(n)=>if p.boolean_operation.is_some(){*self.trash=Some(Displaced::Text(std::mem::replace(&mut n.operation,self.ready_text.take().ok_or_else(invalid)?)))},DrawingLayerNode::Trace(n)=>if let Some(v)=&p.trace_params{n.params=semio_s_artifact_draw_drawing_model::DrawingTraceParams{threshold:v.threshold,simplify_epsilon:v.simplify_epsilon}},DrawingLayerNode::Path(n)=>if p.path_segments.is_some(){*self.trash=Some(Displaced::Segments(std::mem::replace(&mut n.segments,self.ready_segments.take().ok_or_else(invalid)?)))},DrawingLayerNode::Text(n)=>{if p.text_content.is_some(){*self.trash=Some(Displaced::Text(std::mem::replace(&mut n.content,self.ready_text.take().ok_or_else(invalid)?)))}if let Some(v)=p.text_size{n.size=v}if let Some(v)=p.font_family{n.font_family=v}},DrawingLayerNode::Image(n)=>{if p.image_key.is_some(){*self.trash=Some(Displaced::Text(std::mem::replace(&mut n.image_key,self.ready_text.take().ok_or_else(invalid)?)))}if let Some(v)=p.image_width{n.width=v}if let Some(v)=p.image_height{n.height=v}},DrawingLayerNode::Group(n)=>if let Some(v)=p.isolation{n.isolation=v},DrawingLayerNode::Shape(n)=>if let Some(v)=p.shape_coordinates.first(){semio_s_artifact_draw_drawing_model::shape_geometry::set_shape_coordinate(n,v.field,v.index,v.value).map_err(|_|invalid())?},_=>{}}
 self.phase=8;Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))}
}
impl Drop for DrawingRetainedFieldApply{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Drawing field plan retains native prepared or displaced values");}}

#[path="✋️transforms/🦀️.rs"]mod transforms;
