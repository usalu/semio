//! ↩️ Drawing field inverses retain the original command and only the affected original payload field.
use semio_s_artifact_draw_drawing_model::{DrawingLayerNode,FillStyle,StrokeStyle,layer_base,snapshot::DrawingSnapshot};
use crate::DrawingMutation;
use semio_s_artifact_draw_drawing_model::snapshot::lookup::original::{DrawingOriginalLookup,DrawingOriginalLayerAddress as DrawingLayerAddress};
type DrawingNativeText=semio_framework_value::paged::PagedUtf8<{usize::MAX}>;
type DrawingNativeSegments=semio_framework_value_persistent::list::PagedList<semio_s_artifact_draw_drawing_model::PathSegment,{usize::MAX}>;
use protocol::{RetainedFieldPreparation,RetainedMutationCloseContext,RetainedFlight as Flight,retained_native_child as native_child};
use semio_framework_value::{OriginalRetirementCloseLoan,OriginalRetirementPoll,RetainedAllocationEffect,RetirementPollExit};
use crate::retained::{field_close,measured,owner_close,poll_close};
use semio_framework_value::factory::vector::FactoryVectorRetirement;
use semio_framework_value::{ValueError,ValueRefusalKind,RetirementDemand,retained_clone::{RetainedCloneRef,RetainedCloneGrant,RetainedCloneProgress,RetainedCloneStep,RetainedCloneClose,retained_clone_grant_admits}};
use std::mem::{ManuallyDrop,size_of};
fn invalid()->ValueError{ValueError::literal(ValueRefusalKind::InvariantViolated,"Drawing original field inverse custody changed")}
fn fixed(copy_bytes:usize)->RetirementDemand{RetirementDemand{copy_bytes,depth:1,..Default::default()}}
fn metadata()->usize{2*size_of::<DrawingMutation>()+256}
fn nested(mut d:RetirementDemand)->Result<RetirementDemand,ValueError>{d.copy_bytes=d.copy_bytes.checked_add(metadata()).ok_or_else(invalid)?;d.depth=d.depth.checked_add(1).ok_or_else(invalid)?;Ok(d)}
fn child(g:RetainedCloneGrant)->RetainedCloneGrant{RetainedCloneGrant{maximum_copy_bytes:g.maximum_copy_bytes-metadata(),maximum_depth:g.maximum_depth-1,..g}}
fn receipt(mut p:RetainedCloneProgress)->RetainedCloneProgress{p.copied_items=p.copied_items.max(1);p.copied_bytes=p.copied_bytes.saturating_add(metadata());p}
fn refused(error:ValueError)->ValueError{let p=receipt(error.retained_progress());error.with_retained_progress(p)}
pub(crate) fn target<'a>(m:RetainedCloneRef<'a,DrawingMutation>)->Result<RetainedCloneRef<'a,DrawingNativeText>,ValueError>{Ok(m.project(0,|m|match m{DrawingMutation::SetLayerVisible(p)=>&p.layer_id,DrawingMutation::SetLayerLocked(p)=>&p.layer_id,DrawingMutation::SetLayerOpacity(p)=>&p.layer_id,DrawingMutation::SetLayerBlendMode(p)=>&p.layer_id,DrawingMutation::RenameLayer(p)=>&p.layer_id,DrawingMutation::UpdateLayerTransform(p)=>&p.layer_id,DrawingMutation::ReplaceLayerFill(p)=>&p.layer_id,DrawingMutation::ReplaceLayerStroke(p)=>&p.layer_id,DrawingMutation::SetLayerBooleanOperation(p)=>&p.layer_id,DrawingMutation::UpdateLayerTraceParams(p)=>&p.layer_id,DrawingMutation::UpdatePathGeometry(p)=>&p.layer_id,DrawingMutation::UpdateText(p)=>&p.layer_id,DrawingMutation::SetLayerFillRule(p)=>&p.layer_id,DrawingMutation::SetGroupIsolation(p)=>&p.layer_id,DrawingMutation::UpdateImage(p)=>&p.layer_id,DrawingMutation::SetShapeCoordinate(p)=>&p.layer_id,_=>unreachable!("admitted original field target")}))}
pub(crate) fn original<'a>(mut b:RetainedCloneRef<'a,DrawingSnapshot>,at:DrawingLayerAddress)->RetainedCloneRef<'a,DrawingLayerNode>{let root=at.indices[0];let mut node=b.project(3,|b|&b.layers).project(root,move|rows|rows.get(root).unwrap());for i in 1..at.length{let index=at.indices[i];node=node.project(3,|node|match node{DrawingLayerNode::Group(group)=>&group.children,_=>unreachable!("admitted original layer address")}).project(index,move|rows|rows.get(index).unwrap())}node}
fn text<'a>(m:&DrawingMutation,node:RetainedCloneRef<'a,DrawingLayerNode>)->RetainedCloneRef<'a,DrawingNativeText>{match m{DrawingMutation::SetLayerBlendMode(_)=>node.project(1,|n|&layer_base(n).blend_mode),DrawingMutation::RenameLayer(_)=>node.project(2,|n|&layer_base(n).name),DrawingMutation::SetLayerBooleanOperation(_)=>node.project(3,|n|match n{DrawingLayerNode::Boolean(n)=>&n.operation,_=>unreachable!("admitted boolean inverse")}),DrawingMutation::UpdateText(_)=>node.project(4,|n|match n{DrawingLayerNode::Text(n)=>&n.content,_=>unreachable!("admitted text inverse")}),DrawingMutation::UpdateImage(_)=>node.project(5,|n|match n{DrawingLayerNode::Image(n)=>&n.image_key,_=>unreachable!("admitted image inverse")}),_=>unreachable!("admitted original text inverse")}}
fn has_text(m:&DrawingMutation)->bool{matches!(m,DrawingMutation::SetLayerBlendMode(_)|DrawingMutation::RenameLayer(_)|DrawingMutation::SetLayerBooleanOperation(_)|DrawingMutation::UpdateText(_)|DrawingMutation::UpdateImage(_))}
#[derive(semio_framework_value::RetireOwned)]
enum Displaced{Text(DrawingNativeText),Fill(Option<FillStyle>),Stroke(Option<StrokeStyle>),Segments(DrawingNativeSegments)}
/// 🧳️ Old affected fields and displaced authored values retain their exact native retirement capabilities.
pub struct DrawingOriginalFieldInverse{flight:Flight,lookup:DrawingOriginalLookup,authored:RetainedFieldPreparation<DrawingMutation>,text:RetainedFieldPreparation<DrawingNativeText>,fill:RetainedFieldPreparation<Option<FillStyle>>,stroke:RetainedFieldPreparation<Option<StrokeStyle>>,segments:RetainedFieldPreparation<DrawingNativeSegments>,row:ManuallyDrop<Option<DrawingMutation>>,rows:ManuallyDrop<Option<Vec<DrawingMutation>>>,trash:ManuallyDrop<Option<Displaced>>,retirement:RetainedCloneClose,mutation_retirement:Option<FactoryVectorRetirement<DrawingMutation>>,at:Option<DrawingLayerAddress>,phase:u8,closing:bool}
impl Default for DrawingOriginalFieldInverse{fn default()->Self{Self{flight:Default::default(),lookup:Default::default(),authored:Default::default(),text:Default::default(),fill:Default::default(),stroke:Default::default(),segments:Default::default(),row:ManuallyDrop::new(None),rows:ManuallyDrop::new(None),trash:ManuallyDrop::new(None),retirement:Default::default(),mutation_retirement:None,at:None,phase:0,closing:false}}}
impl DrawingOriginalFieldInverse{
 pub fn recognizes(m:&DrawingMutation)->bool{matches!(m,DrawingMutation::SetLayerVisible(_)|DrawingMutation::SetLayerLocked(_)|DrawingMutation::SetLayerOpacity(_)|DrawingMutation::SetLayerBlendMode(_)|DrawingMutation::RenameLayer(_)|DrawingMutation::UpdateLayerTransform(_)|DrawingMutation::ReplaceLayerFill(_)|DrawingMutation::ReplaceLayerStroke(_)|DrawingMutation::SetLayerBooleanOperation(_)|DrawingMutation::UpdateLayerTraceParams(_)|DrawingMutation::UpdatePathGeometry(_)|DrawingMutation::UpdateText(_)|DrawingMutation::SetLayerFillRule(_)|DrawingMutation::SetGroupIsolation(_)|DrawingMutation::UpdateImage(_)|DrawingMutation::SetShapeCoordinate(_))}
 pub(crate) fn validate_target(m:&DrawingMutation,n:&DrawingLayerNode)->Result<(),ValueError>{let valid=match m{DrawingMutation::SetLayerBooleanOperation(_)=>matches!(n,DrawingLayerNode::Boolean(_)),DrawingMutation::UpdateLayerTraceParams(_)=>matches!(n,DrawingLayerNode::Trace(_)),DrawingMutation::UpdatePathGeometry(_)=>matches!(n,DrawingLayerNode::Path(_)),DrawingMutation::UpdateText(_)=>matches!(n,DrawingLayerNode::Text(_)),DrawingMutation::SetGroupIsolation(_)=>matches!(n,DrawingLayerNode::Group(_)),DrawingMutation::UpdateImage(_)=>matches!(n,DrawingLayerNode::Image(_)),DrawingMutation::SetShapeCoordinate(_)=>matches!(n,DrawingLayerNode::Shape(_)),_=>true};if valid{Ok(())}else{Err(ValueError::literal(ValueRefusalKind::InvalidValue,"Drawing original inverse target kind mismatch"))}}
 pub fn demand(&self,b:RetainedCloneRef<'_,DrawingSnapshot>,m:RetainedCloneRef<'_,DrawingMutation>,body:usize,max:u64)->Result<RetirementDemand,ValueError>{
  if self.closing||!Self::recognizes(m.get()){return Err(invalid())}let inner=body.saturating_sub(metadata());Ok(match self.phase{
   0=>nested(self.lookup.demand(b,target(m)?,inner)?)?,1=>RetirementDemand{capacity_bytes:size_of::<DrawingMutation>(),..fixed(metadata())},2=>if self.authored.value().is_some(){fixed(metadata())}else{nested(self.authored.demand(m,inner,max.saturating_sub(1))?)?},3=>nested(self.authored.exchange_demand()?)?,
   4=>{let at=self.at.ok_or_else(invalid)?;let n=original(b,at);Self::validate_target(m.get(),n.get())?;if has_text(m.get()){nested(self.text.demand(text(m.get(),n),inner,max.saturating_sub(1))?)?}else{match m.get(){DrawingMutation::ReplaceLayerFill(_)=>nested(self.fill.demand(n.project(6,|n|&layer_base(n).attributes.fill),inner,max.saturating_sub(1))?)?,DrawingMutation::ReplaceLayerStroke(_)=>nested(self.stroke.demand(n.project(7,|n|&layer_base(n).attributes.stroke),inner,max.saturating_sub(1))?)?,DrawingMutation::UpdatePathGeometry(_)=>nested(self.segments.demand(n.project(8,|n|match n{DrawingLayerNode::Path(n)=>&n.segments,_=>unreachable!("admitted path inverse")}),inner,max.saturating_sub(1))?)?,_=>fixed(metadata())}}},
   5=>if has_text(m.get()){nested(self.text.exchange_demand()?)?}else{match m.get(){DrawingMutation::ReplaceLayerFill(_)=>nested(self.fill.exchange_demand()?)?,DrawingMutation::ReplaceLayerStroke(_)=>nested(self.stroke.exchange_demand()?)?,DrawingMutation::UpdatePathGeometry(_)=>nested(self.segments.exchange_demand()?)?,_=>fixed(metadata())}},6=>fixed(metadata()),7=>fixed(2*size_of::<Option<Vec<DrawingMutation>>>()),_=>return Err(invalid()),
  })
 }
}

impl DrawingOriginalFieldInverse{
 pub fn work_demand(&self,b:RetainedCloneRef<'_,DrawingSnapshot>,m:RetainedCloneRef<'_,DrawingMutation>,body:usize,max:u64)->Result<u64,ValueError>{
  self.demand(b,m,body,max)?;
  let inner=body.saturating_sub(metadata());
  let child_max=max.saturating_sub(1);
  let child=match self.phase{
   0=>Some(self.lookup.advance_work_demand(b,target(m)?)?),
   2 if self.authored.value().is_none()=>Some(self.authored.advance_work_demand(m,inner,child_max)?),
   3=>Some(1),
   4=>{
    let n=original(b,self.at.ok_or_else(invalid)?);
    if has_text(m.get()){Some(self.text.advance_work_demand(text(m.get(),n),inner,child_max)?)}else{match m.get(){
     DrawingMutation::ReplaceLayerFill(_)=>Some(self.fill.advance_work_demand(n.project(6,|n|&layer_base(n).attributes.fill),inner,child_max)?),
     DrawingMutation::ReplaceLayerStroke(_)=>Some(self.stroke.advance_work_demand(n.project(7,|n|&layer_base(n).attributes.stroke),inner,child_max)?),
     DrawingMutation::UpdatePathGeometry(_)=>Some(self.segments.advance_work_demand(n.project(8,|n|match n{DrawingLayerNode::Path(n)=>&n.segments,_=>unreachable!("admitted path inverse")}),inner,child_max)?),
     _=>None,
    }}
   },
   5 if has_text(m.get())||matches!(m.get(),DrawingMutation::ReplaceLayerFill(_)|DrawingMutation::ReplaceLayerStroke(_)|DrawingMutation::UpdatePathGeometry(_))=>Some(1),
   _=>None,
  };
  if child==Some(0){return Ok(0)}
  let work=child.unwrap_or(0).checked_add(1).ok_or_else(invalid)?;
  Ok(if work<=max{work}else{0})
 }
 pub fn advance(&mut self,b:RetainedCloneRef<'_,DrawingSnapshot>,m:RetainedCloneRef<'_,DrawingMutation>,body:usize,max:u64,effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
  let g=effect.grant();
  let d=if recover{Default::default()}else{let d=self.demand(b,m,body,max)?;if !retained_clone_grant_admits(g,d)||self.work_demand(b,m,body,max)?==0{return Ok(RetainedCloneStep::Progress(Default::default()))}d};
  let inner=body.saturating_sub(metadata());
  let child_max=max.saturating_sub(1);
  let mut p=RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()};
  let result=(||->Result<RetainedCloneStep,RetirementPollExit>{match self.phase{
   0=>{p=receipt(measured(effect,work,|e|self.lookup.advance(b,target(m)?,inner,&mut e.nested(child(g)))).map_err(refused)?.progress());if self.lookup.ready(){self.at=Some(self.lookup.address()?.ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"Drawing original inverse target is missing"))?);self.phase=1}},
   1=>{*self.rows=Some(Vec::new());self.phase=254;let rows=self.rows.as_mut().unwrap();let result=rows.try_reserve_exact(1);p.retained_capacity_bytes=rows.capacity()*size_of::<DrawingMutation>();if result.is_err()||!p.fits(g){return Err(invalid().with_retained_progress(p).into())}self.phase=2},
   2=>if self.authored.value().is_some(){self.phase=3}else{p=receipt(protocol::retained_routed!(self.flight,effect,recover,1,inner,child_max,None,*work,|reply|
    self.authored.advance_into(m,inner,child_max,&mut effect.nested(child(g)),reply),
    self.authored.recover_advance_into(m,inner,child_max,&mut effect.nested(child(g)),reply)
   ).map_err(RetirementPollExit::from)?.progress())},
   3=>{let mut value=None;let paid=native_child(effect,work,|e,r|self.authored.take_into(&mut value,&mut e.nested(child(g)),r))??.progress();*self.row=Some(value.ok_or_else(invalid)?);p=receipt(paid);self.phase=4},
   4=>{let n=original(b,self.at.ok_or_else(invalid)?);Self::validate_target(m.get(),n.get())?;if has_text(m.get()){p=receipt(protocol::retained_routed!(self.flight,effect,recover,2,inner,child_max,None,*work,|reply|
    self.text.advance_into(text(m.get(),n),inner,child_max,&mut effect.nested(child(g)),reply),
    self.text.recover_advance_into(text(m.get(),n),inner,child_max,&mut effect.nested(child(g)),reply)
   ).map_err(RetirementPollExit::from)?.progress());if self.text.value().is_some(){self.phase=5}}else{match m.get(){DrawingMutation::ReplaceLayerFill(_)=>{p=receipt(protocol::retained_routed!(self.flight,effect,recover,3,inner,child_max,None,*work,|reply|
    self.fill.advance_into(n.project(6,|n|&layer_base(n).attributes.fill),inner,child_max,&mut effect.nested(child(g)),reply),
    self.fill.recover_advance_into(n.project(6,|n|&layer_base(n).attributes.fill),inner,child_max,&mut effect.nested(child(g)),reply)
   ).map_err(RetirementPollExit::from)?.progress());if self.fill.value().is_some(){self.phase=5}},DrawingMutation::ReplaceLayerStroke(_)=>{p=receipt(protocol::retained_routed!(self.flight,effect,recover,4,inner,child_max,None,*work,|reply|
    self.stroke.advance_into(n.project(7,|n|&layer_base(n).attributes.stroke),inner,child_max,&mut effect.nested(child(g)),reply),
    self.stroke.recover_advance_into(n.project(7,|n|&layer_base(n).attributes.stroke),inner,child_max,&mut effect.nested(child(g)),reply)
   ).map_err(RetirementPollExit::from)?.progress());if self.stroke.value().is_some(){self.phase=5}},DrawingMutation::UpdatePathGeometry(_)=>{p=receipt(protocol::retained_routed!(self.flight,effect,recover,5,inner,child_max,None,*work,|reply|
    self.segments.advance_into(n.project(8,|n|match n{DrawingLayerNode::Path(n)=>&n.segments,_=>unreachable!("admitted original path")}),inner,child_max,&mut effect.nested(child(g)),reply),
    self.segments.recover_advance_into(n.project(8,|n|match n{DrawingLayerNode::Path(n)=>&n.segments,_=>unreachable!("admitted original path")}),inner,child_max,&mut effect.nested(child(g)),reply)
   ).map_err(RetirementPollExit::from)?.progress());if self.segments.value().is_some(){self.phase=5}},_=>self.phase=6}}},
   5=>{let row=self.row.as_mut().ok_or_else(invalid)?;let displaced=if has_text(m.get()){let mut value=None;let paid=native_child(effect,work,|e,r|self.text.take_into(&mut value,&mut e.nested(child(g)),r))??.progress();p=receipt(paid);let target=match row{DrawingMutation::SetLayerBlendMode(p)=>&mut p.blend_mode,DrawingMutation::RenameLayer(p)=>&mut p.new_name,DrawingMutation::SetLayerBooleanOperation(p)=>&mut p.boolean_operation,DrawingMutation::UpdateText(p)=>&mut p.content,DrawingMutation::UpdateImage(p)=>&mut p.image_key,_=>return Err(invalid().into())};Displaced::Text(std::mem::replace(target,value.ok_or_else(invalid)?))}else{match row{DrawingMutation::ReplaceLayerFill(row)=>{let mut value=None;let paid=native_child(effect,work,|e,r|self.fill.take_into(&mut value,&mut e.nested(child(g)),r))??.progress();p=receipt(paid);Displaced::Fill(std::mem::replace(&mut row.fill,value.ok_or_else(invalid)?))},DrawingMutation::ReplaceLayerStroke(row)=>{let mut value=None;let paid=native_child(effect,work,|e,r|self.stroke.take_into(&mut value,&mut e.nested(child(g)),r))??.progress();p=receipt(paid);Displaced::Stroke(std::mem::replace(&mut row.stroke,value.ok_or_else(invalid)?))},DrawingMutation::UpdatePathGeometry(row)=>{let mut value=None;let paid=native_child(effect,work,|e,r|self.segments.take_into(&mut value,&mut e.nested(child(g)),r))??.progress();p=receipt(paid);Displaced::Segments(std::mem::replace(&mut row.segments,value.ok_or_else(invalid)?))},_=>return Err(invalid().into())}};*self.trash=Some(displaced);self.phase=6},
   6=>{let n=original(b,self.at.ok_or_else(invalid)?);let node=n.get();let base=layer_base(node);match self.row.as_mut().ok_or_else(invalid)?{
    DrawingMutation::SetLayerVisible(p)=>p.visible=base.visible,DrawingMutation::SetLayerLocked(p)=>p.locked=base.locked,DrawingMutation::SetLayerOpacity(p)=>p.opacity=base.opacity,
    DrawingMutation::UpdateLayerTransform(p)=>{let v=&base.transform;p.transform=semio_s_artifact_draw_drawing_model::DrawingTransform{x:v.x,y:v.y,scale_x:v.scale_x,scale_y:v.scale_y,shear:v.shear,rotation:v.rotation}},
    DrawingMutation::UpdateLayerTraceParams(p)=>{let DrawingLayerNode::Trace(n)=node else{return Err(invalid().into())};p.params=semio_s_artifact_draw_drawing_model::DrawingTraceParams{threshold:n.params.threshold,simplify_epsilon:n.params.simplify_epsilon}},
    DrawingMutation::UpdateText(p)=>{let DrawingLayerNode::Text(n)=node else{return Err(invalid().into())};p.size=n.size;p.font_family=n.font_family},DrawingMutation::UpdateImage(p)=>{let DrawingLayerNode::Image(n)=node else{return Err(invalid().into())};p.width=n.width;p.height=n.height},
    DrawingMutation::SetLayerFillRule(p)=>p.fill_rule=base.attributes.fill_rule,DrawingMutation::SetGroupIsolation(p)=>{let DrawingLayerNode::Group(n)=node else{return Err(invalid().into())};p.isolation=n.isolation},
    DrawingMutation::SetShapeCoordinate(p)=>{let DrawingLayerNode::Shape(n)=node else{return Err(invalid().into())};p.value=semio_s_artifact_draw_drawing_model::shape_geometry::shape_coordinate(n,&p.field,p.index).map_err(|_|invalid())?},_=>{}
   }let rows=self.rows.as_mut().ok_or_else(invalid)?;if rows.len()==rows.capacity(){return Err(invalid().into())}rows.push(self.row.take().ok_or_else(invalid)?);self.phase=7;return Ok(RetainedCloneStep::Complete(p))},
   7=>return Ok(RetainedCloneStep::Complete(Default::default())),_=>return Err(invalid().into()),
  }Ok(RetainedCloneStep::Progress(p))})();
  if !matches!(result,Err(RetirementPollExit::Pending(_))){*work+=1;}
  result
 }
 pub fn ready(&self)->bool{self.phase==7&&!self.closing&&self.rows.is_some()}
 pub fn take_demand(&self)->Result<RetirementDemand,ValueError>{if !self.ready(){return Err(invalid())}Ok(fixed(2*size_of::<Option<Vec<DrawingMutation>>>()))}
 pub fn take(&mut self,g:RetainedCloneGrant)->Result<(Option<Vec<DrawingMutation>>,RetainedCloneProgress),ValueError>{let d=self.take_demand()?;if !retained_clone_grant_admits(g,d){return Ok((None,Default::default()))}self.phase=8;Ok((self.rows.take(),RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,..Default::default()}))}
 pub fn begin_close(&mut self){self.closing=true;self.lookup.begin_close();}
 pub fn close_demand(&self,context:RetainedMutationCloseContext<'_,DrawingMutation>,body:usize)->Result<RetirementDemand,ValueError>{
  if !self.lookup.terminal_is_empty(){return self.lookup.close_demand(body)}
  if !self.authored.terminal_is_empty(){return if self.authored.close_started(){self.authored.close_demand(body)}else{Ok(self.authored.begin_close_demand())}}
  if !self.text.terminal_is_empty(){return if self.text.close_started(){self.text.close_demand(body)}else{Ok(self.text.begin_close_demand())}}
  if !self.fill.terminal_is_empty(){return if self.fill.close_started(){self.fill.close_demand(body)}else{Ok(self.fill.begin_close_demand())}}
  if !self.stroke.terminal_is_empty(){return if self.stroke.close_started(){self.stroke.close_demand(body)}else{Ok(self.stroke.begin_close_demand())}}
  if !self.segments.terminal_is_empty(){return if self.segments.close_started(){self.segments.close_demand(body)}else{Ok(self.segments.begin_close_demand())}}
  if !self.retirement.is_empty()||self.trash.is_some(){return self.retirement.next_owner_demands::<Displaced>(self.trash.is_some(),&None,body)}
  if self.row.is_some(){return Ok(fixed(metadata()))}
  if let Some(owner)=self.mutation_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(fixed(size_of::<Option<FactoryVectorRetirement<DrawingMutation>>>()))}else{owner.demands(context.mutations,body)}}
  if self.rows.is_some(){return Ok(FactoryVectorRetirement::<DrawingMutation>::constructor_demand())}
  Ok(Default::default())
 }
 pub fn close_work_demand(&self,context:RetainedMutationCloseContext<'_,DrawingMutation>,body:usize)->Result<u64,ValueError>{
  if !self.lookup.terminal_is_empty(){return self.lookup.close_work_demand(body)}
  if !self.authored.terminal_is_empty(){return if self.authored.close_started(){self.authored.close_work_demand(body)}else{Ok(self.authored.begin_close_work_demand())}}
  if !self.text.terminal_is_empty(){return if self.text.close_started(){self.text.close_work_demand(body)}else{Ok(self.text.begin_close_work_demand())}}
  if !self.fill.terminal_is_empty(){return if self.fill.close_started(){self.fill.close_work_demand(body)}else{Ok(self.fill.begin_close_work_demand())}}
  if !self.stroke.terminal_is_empty(){return if self.stroke.close_started(){self.stroke.close_work_demand(body)}else{Ok(self.stroke.begin_close_work_demand())}}
  if !self.segments.terminal_is_empty(){return if self.segments.close_started(){self.segments.close_work_demand(body)}else{Ok(self.segments.begin_close_work_demand())}}
  if !self.retirement.is_empty()||self.trash.is_some(){return self.retirement.next_owner_work::<Displaced>(self.trash.is_some(),&None,body)}
  if self.row.is_some(){return Ok(1)}
  if let Some(owner)=self.mutation_retirement.as_ref(){return if owner.terminal_is_empty(){Ok(1)}else{owner.close_work_demand(context.mutations,body)}}
  Ok(u64::from(self.rows.is_some()))
 }
 pub fn requires_original_birth(&self)->bool{
  if !self.flight.idle()||!self.lookup.terminal_is_empty(){return false}
  if !self.authored.terminal_is_empty(){return self.authored.close_started()&&self.authored.requires_original_birth()}
  if !self.text.terminal_is_empty(){return self.text.close_started()&&self.text.requires_original_birth()}
  if !self.fill.terminal_is_empty(){return self.fill.close_started()&&self.fill.requires_original_birth()}
  if !self.stroke.terminal_is_empty(){return self.stroke.close_started()&&self.stroke.requires_original_birth()}
  if !self.segments.terminal_is_empty(){return self.segments.close_started()&&self.segments.requires_original_birth()}
  if !self.retirement.is_empty(){return self.retirement.requires_original_birth()}
  self.trash.is_none()&&self.row.is_none()&&self.mutation_retirement.as_ref().is_some_and(FactoryVectorRetirement::requires_original_birth)
 }
 pub fn requires_original_control(&self)->bool{
  if let Some(loaned)=self.flight.loaned(){return loaned}
  if !self.lookup.terminal_is_empty(){return false}
  if !self.authored.terminal_is_empty(){return self.authored.close_started()&&self.authored.requires_original_control()}
  if !self.text.terminal_is_empty(){return self.text.close_started()&&self.text.requires_original_control()}
  if !self.fill.terminal_is_empty(){return self.fill.close_started()&&self.fill.requires_original_control()}
  if !self.stroke.terminal_is_empty(){return self.stroke.close_started()&&self.stroke.requires_original_control()}
  if !self.segments.terminal_is_empty(){return self.segments.close_started()&&self.segments.requires_original_control()}
  if !self.retirement.is_empty(){return self.retirement.requires_original_control()}
  self.trash.is_none()&&self.row.is_none()&&self.mutation_retirement.as_ref().is_some_and(FactoryVectorRetirement::requires_original_control)
 }
 pub fn close_original_birth_step(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,grant:RetainedCloneGrant)->OriginalRetirementPoll{
  if !self.authored.terminal_is_empty(){return self.authored.close_original_birth_step(original,grant)}
  if !self.text.terminal_is_empty(){return self.text.close_original_birth_step(original,grant)}
  if !self.fill.terminal_is_empty(){return self.fill.close_original_birth_step(original,grant)}
  if !self.stroke.terminal_is_empty(){return self.stroke.close_original_birth_step(original,grant)}
  if !self.segments.terminal_is_empty(){return self.segments.close_original_birth_step(original,grant)}
  if !self.retirement.is_empty(){return self.retirement.step_original_birth(original,grant)}
  self.mutation_retirement.as_mut().expect("selected original mutation vector").close_original_birth_step(original,grant)
 }
 pub fn close_step(&mut self,context:RetainedMutationCloseContext<'_,DrawingMutation>,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,recover:bool,work:&mut u128)->Result<RetainedCloneStep,RetirementPollExit>{
  if !self.closing{return Err(invalid().into())}
  if !self.lookup.terminal_is_empty(){return Ok(measured(effect,work,|e|self.lookup.close_step(body,e))?)}
  if !self.authored.terminal_is_empty(){return field_close(&mut self.authored,&mut self.flight,6,original,body,effect,recover,work)}
  if !self.text.terminal_is_empty(){return field_close(&mut self.text,&mut self.flight,7,original,body,effect,recover,work)}
  if !self.fill.terminal_is_empty(){return field_close(&mut self.fill,&mut self.flight,8,original,body,effect,recover,work)}
  if !self.stroke.terminal_is_empty(){return field_close(&mut self.stroke,&mut self.flight,9,original,body,effect,recover,work)}
  if !self.segments.terminal_is_empty(){return field_close(&mut self.segments,&mut self.flight,10,original,body,effect,recover,work)}
  if !self.retirement.is_empty()||self.trash.is_some(){return owner_close(&mut self.retirement,&mut self.trash,&mut self.flight,11,original,body,effect,recover,work)}
  if let Some(row)=self.row.take(){
   let Some(rows)=self.rows.as_mut()else{*self.row=Some(row);return Err(invalid().into())};
   if rows.len()==rows.capacity(){*self.row=Some(row);return Err(invalid().into())}
   rows.push(row);*work+=1;
   return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:metadata(),..Default::default()}))
  }
  if let Some(owner)=self.mutation_retirement.as_mut(){
   if owner.terminal_is_empty(){self.mutation_retirement=None;*work+=1;return Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:size_of::<Option<FactoryVectorRetirement<DrawingMutation>>>(),..Default::default()}))}
   return poll_close(&mut self.flight,12,original,body,effect,recover,work,|original,effect|if let Some(original)=original{owner.close_original_step(original,body,effect)}else{owner.close_step(context.mutations,body,effect)})
  }
  if self.rows.is_some(){return poll_close(&mut self.flight,13,None,body,effect,recover,work,|_,effect|FactoryVectorRetirement::admit(&mut self.rows,&mut self.mutation_retirement,effect).map_err(Into::into))}
  Ok(RetainedCloneStep::Complete(Default::default()))
 }
 pub fn terminal_is_empty(&self)->bool{self.closing&&self.lookup.terminal_is_empty()&&self.authored.terminal_is_empty()&&self.text.terminal_is_empty()&&self.fill.terminal_is_empty()&&self.stroke.terminal_is_empty()&&self.segments.terminal_is_empty()&&self.row.is_none()&&self.rows.is_none()&&self.trash.is_none()&&self.mutation_retirement.is_none()&&self.retirement.is_empty()&&self.flight.idle()}
}
impl Drop for DrawingOriginalFieldInverse{fn drop(&mut self){assert!(std::thread::panicking()||self.terminal_is_empty(),"Drawing inverse retains its original fields or displaced authored values");}}

#[cfg(test)]#[path="🧪️tests/🦀️.rs"]mod tests;
