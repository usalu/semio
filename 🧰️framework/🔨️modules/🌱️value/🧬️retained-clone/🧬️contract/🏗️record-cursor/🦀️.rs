//! 🏗️ The record clone machine, once for every derived struct and enum: a derive emits only its row table, a variant selector, a constructor and two thin row dispatchers; every phase, route and close decision lives here.
//! Row codes keep the derived layout: `4g` advance, `4g+1` take, `4g+2` begin close, `4g+3` close, `4n` done, `4n+1` output; close codes `100+2g`, `101+2g`, `200` (owner close), `201` (binding close).
use super::*;
use std::mem::size_of;

type Reply<'r>=OriginalEffectReply<'r,RetainedCloneStep,OriginalRetirementFailure>;
type Complete<'r>=Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,OriginalRetirementFailure>,ValueError>;

/// 🧱️ One retained field row: its lazily born field cursor and the field value it publishes.
pub struct RetainedRecordRow<T:RetainedClone>{cursor:RetainedFieldCursor<T>,value:Option<T>}
impl<T:RetainedClone> Default for RetainedRecordRow<T>{fn default()->Self{Self{cursor:Default::default(),value:None}}}
const _:()={fn send<X:Send>(){}#[allow(dead_code)]fn row<T:RetainedClone>(){send::<RetainedFieldCursor<T>>();send::<Option<T>>()}};
/// 🧵️ Sound: the assertion above proves both fields `Send` for every `T: RetainedClone`; stating it here keeps the auto-trait walk of deeply nested records from expanding every row structurally.
unsafe impl<T:RetainedClone> Send for RetainedRecordRow<T>{}
impl<T:RetainedClone> RetainedRecordRow<T>{
 /// 📤️ Moves the published field value into the record constructor.
 pub fn take_value(&mut self)->Result<T,ValueError>{self.value.take().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"retained field owner is absent"))}
}

/// 🔭️ Type-erased queries of one row; the projected source is present only for the planned row of the live variant.
pub trait RetainedRecordQuery{
 fn present(&self)->bool;
 fn value_bytes(&self)->usize;
 fn terminal(&self)->bool;
 fn closing(&self)->bool;
 fn ready(&self)->bool;
 fn released(&self)->bool;
 fn advance_demand(&self,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>;
 fn advance_work(&self,body:usize,maximum:u64)->Result<u64,ValueError>;
 fn advance_selection(&self,body:usize,maximum:u64)->Result<RetainedCloneSelection,ValueError>;
 fn take_demand(&self)->Result<RetirementDemand,ValueError>;
 fn take_work(&self)->Result<u64,ValueError>;
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>;
 fn begin_close_work(&self)->Result<u64,ValueError>;
 fn close_work(&self,body:usize)->Result<u64,ValueError>;
 fn owner_demands(&self,close:&RetainedCloneClose,body:usize)->Result<RetirementDemand,ValueError>;
 fn owner_work(&self,close:&RetainedCloneClose,body:usize)->Result<u64,ValueError>;
}

/// 🕹️ Type-erased producers of one row under the enclosing record reply.
pub trait RetainedRecordDrive{
 fn take<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,child:OriginalEffectReply<'r,RetainedCloneStep,ValueError>,recover:bool)->Result<(),ValueError>;
 fn advance<'r>(&mut self,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,child:Reply<'r>,recover:bool)->Result<(),ValueError>;
 fn begin_close<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,child:Reply<'r>,recover:bool)->Result<(),ValueError>;
 fn close<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,child:Reply<'r>,recover:bool)->Result<(),ValueError>;
 fn begin_value_close(&mut self,close:&mut RetainedCloneClose,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneStep>>;
}

/// 👁️ One row lent to the machine with its optionally projected original source.
pub struct RetainedRecordField<'a,T:RetainedClone>{row:&'a RetainedRecordRow<T>,source:Option<RetainedCloneRef<'a,T>>}
impl<'a,T:RetainedClone> RetainedRecordField<'a,T>{
 pub fn new(row:&'a RetainedRecordRow<T>,source:Option<RetainedCloneRef<'a,T>>)->Self{Self{row,source}}
 fn source(&self)->Result<RetainedCloneRef<'a,T>,ValueError>{self.source.ok_or_else(unprojected)}
}
/// ✍️ One row lent mutably to the machine with its optionally projected original source.
pub struct RetainedRecordFieldMut<'a,T:RetainedClone>{row:&'a mut RetainedRecordRow<T>,source:Option<RetainedCloneRef<'a,T>>}
impl<'a,T:RetainedClone> RetainedRecordFieldMut<'a,T>{
 pub fn new(row:&'a mut RetainedRecordRow<T>,source:Option<RetainedCloneRef<'a,T>>)->Self{Self{row,source}}
}
fn unprojected()->ValueError{ValueError::invariant("record row advance lacks its projected source")}

impl<T:RetainedClone> RetainedRecordQuery for RetainedRecordField<'_,T>{
 fn present(&self)->bool{self.row.value.is_some()}
 fn value_bytes(&self)->usize{size_of::<Option<T>>()}
 fn terminal(&self)->bool{RetainedCloneCursor::terminal_is_empty(&self.row.cursor)}
 fn closing(&self)->bool{RetainedCloneCursor::close_started(&self.row.cursor)}
 fn ready(&self)->bool{RetainedCloneCursor::output_ready(&self.row.cursor)}
 fn released(&self)->bool{RetainedCloneCursor::native_terminal_release(&self.row.cursor).is_some_and(|release|release.validates(&self.row.cursor))}
 fn advance_demand(&self,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{RetainedCloneCursor::advance_demands(&self.row.cursor,self.source()?,body,maximum)}
 fn advance_work(&self,body:usize,maximum:u64)->Result<u64,ValueError>{RetainedCloneCursor::advance_work_demand(&self.row.cursor,self.source()?,body,maximum)}
 fn advance_selection(&self,body:usize,maximum:u64)->Result<RetainedCloneSelection,ValueError>{RetainedCloneCursor::advance_selection(&self.row.cursor,self.source()?,body,maximum)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{RetainedCloneCursor::take_demand(&self.row.cursor)}
 fn take_work(&self)->Result<u64,ValueError>{RetainedCloneCursor::take_work_demand(&self.row.cursor)}
 fn close_demand(&self,body:usize)->Result<RetirementDemand,ValueError>{clone_cursor_close_demand::<T>(&self.row.cursor,body)}
 fn begin_close_work(&self)->Result<u64,ValueError>{RetainedCloneCursor::begin_close_work_demand(&self.row.cursor)}
 fn close_work(&self,body:usize)->Result<u64,ValueError>{RetainedCloneCursor::close_work_demand(&self.row.cursor,body)}
 fn owner_demands(&self,close:&RetainedCloneClose,body:usize)->Result<RetirementDemand,ValueError>{close.next_owner_demands::<T>(true,&None,body)}
 fn owner_work(&self,close:&RetainedCloneClose,body:usize)->Result<u64,ValueError>{close.next_owner_work::<T>(true,&None,body)}
}

impl<T:RetainedClone> RetainedRecordDrive for RetainedRecordFieldMut<'_,T>{
 fn take<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,child:OriginalEffectReply<'r,RetainedCloneStep,ValueError>,recover:bool)->Result<(),ValueError>{let row=&mut*self.row;if recover{RetainedCloneCursor::recover_take(&mut row.cursor,&mut row.value,effect,child)?;}else{RetainedCloneCursor::take(&mut row.cursor,&mut row.value,effect,child)?;}Ok(())}
 fn advance<'r>(&mut self,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,child:Reply<'r>,recover:bool)->Result<(),ValueError>{let source=self.source.ok_or_else(unprojected)?;if recover{RetainedCloneCursor::recover_advance_into(&mut self.row.cursor,source,body,maximum,effect,child)?;}else{RetainedCloneCursor::advance_into(&mut self.row.cursor,source,body,maximum,effect,child)?;}Ok(())}
 fn begin_close<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,child:Reply<'r>,recover:bool)->Result<(),ValueError>{if recover{RetainedCloneCursor::recover_begin_close_into(&mut self.row.cursor,effect,child)?;}else{RetainedCloneCursor::begin_close_into(&mut self.row.cursor,effect,child)?;}Ok(())}
 fn close<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,child:Reply<'r>,recover:bool)->Result<(),ValueError>{
  let cursor=&mut self.row.cursor;
  match original{Some(original)=>if recover{RetainedCloneCursor::recover_close_original_into(cursor,original,body,effect,child)?;}else{RetainedCloneCursor::close_original_into(cursor,original,body,effect,child)?;},None=>if recover{RetainedCloneCursor::recover_close_into(cursor,body,effect,child)?;}else{RetainedCloneCursor::close_into(cursor,body,effect,child)?;}}
  Ok(())
 }
 fn begin_value_close(&mut self,close:&mut RetainedCloneClose,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<Option<RetainedCloneStep>>{close.begin_granted(&mut self.row.value,effect)}
}

/// 📋️ The per-type surface a derive emits: the row count, rows per variant (contiguous, in declaration order), the live variant, two row dispatchers and the constructor.
pub trait RetainedRecordRows<O>:Default+Send{
 const ROWS:usize;
 const VARIANTS:&'static [usize];
 fn variant(source:&O)->usize;
 fn query(&self,row:usize,source:Option<RetainedCloneRef<'_,O>>,visit:&mut dyn FnMut(&dyn RetainedRecordQuery));
 fn drive(&mut self,row:usize,source:Option<RetainedCloneRef<'_,O>>,visit:&mut dyn FnMut(&mut dyn RetainedRecordDrive));
 fn finish(&mut self,variant:usize)->Result<O,ValueError>;
}

fn absent()->ValueError{ValueError::invariant("record clone row is absent")}
fn phase_invalid()->ValueError{ValueError::invariant("record clone state is invalid")}
fn query<O,R:RetainedRecordRows<O>,T>(rows:&R,row:usize,source:Option<RetainedCloneRef<'_,O>>,f:impl FnOnce(&dyn RetainedRecordQuery)->T)->Result<T,ValueError>{let mut f=Some(f);let mut out=None;rows.query(row,source,&mut|field|if let Some(f)=f.take(){out=Some(f(field))});out.ok_or_else(absent)}
fn drive<O,R:RetainedRecordRows<O>,T>(rows:&mut R,row:usize,source:Option<RetainedCloneRef<'_,O>>,f:impl FnOnce(&mut dyn RetainedRecordDrive)->Result<T,ValueError>)->Result<T,ValueError>{let mut f=Some(f);let mut out=None;rows.drive(row,source,&mut|field|if let Some(f)=f.take(){out=Some(f(field))});out.unwrap_or_else(||Err(absent()))}
fn variant_rows<O,R:RetainedRecordRows<O>>(variant:usize)->Result<std::ops::Range<usize>,ValueError>{let count=*R::VARIANTS.get(variant).ok_or_else(phase_invalid)?;let first=R::VARIANTS[..variant].iter().sum::<usize>();Ok(first..first+count)}

/// 🧬️ The reply-shaped native clone cursor of every derived record and enum.
pub struct RetainedRecordCursor<O,R>{
 phase:usize,
 output:Option<O>,
 source:Option<RetainedCloneBinding>,
 spent:bool,
 closing:bool,
 close:RetainedCloneClose,
 route:Option<(OriginalEffectInvocationIdentity,usize)>,
 selected:Option<(RetirementDemand,u64)>,
 child_reply:Option<Result<RetainedCloneStep,ValueError>>,
 rows:R,
}
impl<O,R:Default> Default for RetainedRecordCursor<O,R>{fn default()->Self{Self{phase:0,output:None,source:None,spent:false,closing:false,close:Default::default(),route:None,selected:None,child_reply:None,rows:R::default()}}}

impl<O:RetainedClone,R:RetainedRecordRows<O>> RetainedRecordCursor<O,R>{
 fn key(&self,effect:&RetainedAllocationEffect<'_>,code:usize)->Result<(OriginalEffectInvocationIdentity,usize),ValueError>{let key=(effect.invocation().ok_or_else(||ValueError::invariant("record child requires its original invocation"))?.identity(),code);if self.route.is_some_and(|route|route!=key){return Err(ValueError::invariant("record original child invocation changed"))}Ok(key)}
 fn metadata_demand()->RetirementDemand{RetirementDemand{copy_bytes:size_of::<usize>(),depth:1,..Default::default()}}
 fn output_demand(&self,variant:usize)->Result<RetirementDemand,ValueError>{
  let Ok(rows)=variant_rows::<O,R>(variant)else{return Ok(Default::default())};
  let mut copy_bytes=size_of::<Option<O>>()+size_of::<usize>();for row in rows{copy_bytes+=query(&self.rows,row,None,|field|field.value_bytes())?;}
  Ok(RetirementDemand{copy_bytes,depth:1,..Default::default()})
 }
 fn plan(&self,source:RetainedCloneRef<'_,O>)->Result<usize,ValueError>{
  let n=R::ROWS;let rows=variant_rows::<O,R>(R::variant(source.get()))?;
  if self.phase<rows.len(){let g=rows.start+self.phase;return query(&self.rows,g,None,|field|if field.present(){if field.terminal(){4*n}else if field.closing(){4*g+3}else{4*g+2}}else if field.ready(){4*g+1}else{4*g})}
  if self.phase==rows.len(){return Ok(4*n+1)}
  Err(phase_invalid())
 }
 fn advance_reply<'r>(&mut self,source:RetainedCloneRef<'_,O>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Complete<'r>{
  let n=R::ROWS;
  let code=if let Some((_,code))=self.route{code}else{
   if recover{return Err(ValueError::invariant("record recovery has no original advance"))}
   let demand=RetainedCloneCursor::advance_demands(self,source,body,maximum)?;let work=RetainedCloneCursor::advance_work_demand(self,source,body,maximum)?;
   if work==0&&!RetainedCloneCursor::output_ready(self)||effect.remaining_work_units()?<work||!retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   if RetainedCloneCursor::output_ready(self){return Ok(reply.complete(Ok(RetainedCloneStep::Complete(Default::default()))))}
   if self.source.is_none(){let turn=source.bind(&mut self.source,effect);return Ok(reply.complete(turn.result.map(|progress|RetainedCloneStep::Progress(progress.unwrap_or_default())).map_err(Into::into)))}
   let code=self.plan(source)?;
   if code==4*n+1{
    let variant=R::variant(source.get());
    for row in variant_rows::<O,R>(variant)?{if !query(&self.rows,row,None,|field|field.present())?{return Err(ValueError::invariant("record original output field missing"))}}
    crate::record_retirement_work(effect,1).map_err(|(error,_)|error)?;self.output=Some(self.rows.finish(variant)?);self.phase+=1;
    return Ok(reply.complete(Ok(RetainedCloneStep::Complete(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))
   }
   self.selected=Some((demand,work));
   if code==4*n{crate::record_retirement_work(effect,1).map_err(|(error,_)|error)?;self.phase+=1;return Ok(reply.complete(Ok(RetainedCloneStep::Progress(RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()}))))}
   code
  };
  let key=self.key(effect,code)?;
  let g=code/4;if g>=n{return Err(ValueError::invariant("record advance route invalid"))}
  if code%4==1{
   let held=self.route.is_some();self.route=Some(key);
   if self.child_reply.is_none(){
    let taken=(||->Result<(),ValueError>{let child=OriginalEffectReply::borrow(&mut self.child_reply)?;drive(&mut self.rows,g,None,|field|field.take(effect,child,recover))})();
    if let Err(error)=taken{if !held&&!recover{self.route=None}return Err(error)}
   }
   let Some(result)=self.child_reply.take()else{return Ok(reply.pending())};
   let mut scope=reply.progress().scope(&mut self.route,key)?;OriginalEffectReplyScope::borrow(&mut scope)?.complete(result.map_err(Into::into));return Ok(scope.forwarded())
  }
  let mut scope=reply.progress().scope(&mut self.route,key)?;let child=OriginalEffectReplyScope::borrow(&mut scope)?;
  match code%4{
   0=>drive(&mut self.rows,g,Some(source),|field|field.advance(body,maximum,effect,child,recover))?,
   2=>drive(&mut self.rows,g,None,|field|field.begin_close(effect,child,recover))?,
   3=>drive(&mut self.rows,g,None,|field|field.close(None,body,effect,child,recover))?,
   _=>return Err(ValueError::invariant("record child route invalid")),
  }
  Ok(scope.forwarded())
 }
 fn close_reply<'r>(&mut self,original:Option<&mut OriginalRetirementCloseLoan<'_>>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>,recover:bool)->Complete<'r>{
  if !self.closing{return Err(ValueError::invariant("record close has not started"))}
  let n=R::ROWS;
  let code=if let Some((_,code))=self.route{code}else{
   if recover{return Err(ValueError::invariant("record recovery has no original close"))}
   let demand=RetainedCloneCursor::close_demands(self,body)?;let work=RetainedCloneCursor::close_work_demand(self,body)?;
   if effect.remaining_work_units()?<work||!retained_clone_grant_admits(effect.grant(),demand){return Ok(reply.complete(Ok(RetainedCloneStep::Progress(Default::default()))))}
   self.selected=Some((demand,work));
   let mut child=None;
   for g in 0..n{if let Some(code)=query(&self.rows,g,None,|field|(!field.terminal()).then(||100+2*g+usize::from(field.closing())))?{child=Some(code);break}}
   if let Some(code)=child{code}else if !self.close.is_empty(){200}else{
    for g in 0..n{if query(&self.rows,g,None,|field|field.present())?{let close=&mut self.close;let turn=drive(&mut self.rows,g,None,|field|Ok(field.begin_value_close(close,effect)))?;return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}}
    if self.output.is_some(){let turn=self.close.begin_granted(&mut self.output,effect);return Ok(reply.complete(turn.result.map(|step|step.unwrap_or(RetainedCloneStep::Progress(Default::default()))).map_err(Into::into)))}
    201
   }
  };
  let key=self.key(effect,code)?;let reply=if code==201{reply}else{reply.progress()};let mut scope=reply.scope(&mut self.route,key)?;let child=OriginalEffectReplyScope::borrow(&mut scope)?;
  if (100..100+2*n).contains(&code){
   let g=(code-100)/2;
   if code%2==0{drive(&mut self.rows,g,None,|field|field.begin_close(effect,child,recover))?}else{drive(&mut self.rows,g,None,|field|field.close(original,body,effect,child,recover))?}
  }else if code==200{
   match original{Some(original)=>if recover{self.close.recover_step_original_into(original,body,effect,child)?;}else{self.close.step_original_into(original,body,effect,child)?;},None=>if recover{self.close.recover_step_into(body,effect,child)?;}else{self.close.step_into(body,effect,child)?;}}
  }else if code==201{
   match original{Some(original)=>if recover{RetainedCloneBinding::recover_close_original_into(&mut self.source,original,body,effect,child)?;}else{RetainedCloneBinding::close_original_into(&mut self.source,original,body,effect,child)?;},None=>if recover{RetainedCloneBinding::recover_close_into(&mut self.source,body,effect,child)?;}else{RetainedCloneBinding::close_into(&mut self.source,body,effect,child)?;}}
  }else{return Err(ValueError::invariant("record original close route invalid"))}
  Ok(scope.forwarded())
 }
 fn rows_released(&self)->bool{(0..R::ROWS).all(|g|query(&self.rows,g,None,|field|field.released()).unwrap_or(false))}
 fn rows_empty(&self)->bool{(0..R::ROWS).all(|g|query(&self.rows,g,None,|field|!field.present()&&field.terminal()).unwrap_or(false))}
}

fn native_terminal<O:RetainedClone,R:RetainedRecordRows<O>>(cursor:&RetainedRecordCursor<O,R>)->bool{RetainedCloneCursor::terminal_is_empty(cursor)&&cursor.rows_released()}

impl<O:RetainedClone,R:RetainedRecordRows<O>> RetainedCloneCursor<O> for RetainedRecordCursor<O,R>{
 fn native_terminal_release(&self)->Option<RetainedCursorRelease<Self>>{native_terminal(self).then(||unsafe{RetainedCursorRelease::native(native_terminal::<O,R>)})}
 fn output_ready(&self)->bool{!self.closing&&!self.spent&&self.output.is_some()}
 fn close_started(&self)->bool{self.closing}
 fn advance_demands(&self,source:RetainedCloneRef<'_,O>,body:usize,maximum:u64)->Result<RetirementDemand,ValueError>{
  if self.route.is_some(){return self.selected.map(|selected|selected.0).ok_or_else(||ValueError::invariant("record selected quote missing"))}
  if let Some(demand)=source.binding_demand(&self.source)?{return Ok(demand)}
  if self.closing||self.spent{return Err(ValueError::invariant("record cursor is closing or spent"))}
  if RetainedCloneCursor::output_ready(self){return Ok(Default::default())}
  let n=R::ROWS;let code=self.plan(source)?;
  if code==4*n{return Ok(Self::metadata_demand())}
  if code==4*n+1{return self.output_demand(R::variant(source.get()))}
  let g=code/4;
  match code%4{0=>query(&self.rows,g,Some(source),|field|field.advance_demand(body,maximum))?,1=>query(&self.rows,g,None,|field|field.take_demand())?,_=>query(&self.rows,g,None,|field|field.close_demand(body))?}
 }
 fn advance_work_demand(&self,source:RetainedCloneRef<'_,O>,body:usize,maximum:u64)->Result<u64,ValueError>{
  if self.route.is_some(){return self.selected.map(|selected|selected.1).ok_or_else(||ValueError::invariant("record selected work missing"))}
  source.binding_demand(&self.source)?;
  if RetainedCloneCursor::output_ready(self){return Ok(0)}
  if self.source.is_none(){return source.binding_work_demand(&self.source)}
  let n=R::ROWS;let code=self.plan(source)?;
  let units=if code==4*n||code==4*n+1{1}else{let g=code/4;match code%4{0=>return query(&self.rows,g,Some(source),|field|field.advance_work(body,maximum))?,1=>query(&self.rows,g,None,|field|field.take_work())??,2=>query(&self.rows,g,None,|field|field.begin_close_work())??,_=>query(&self.rows,g,None,|field|field.close_work(body))??}};
  Ok(if units<=maximum{units}else{0})
 }
 fn advance_selection(&self,source:RetainedCloneRef<'_,O>,body:usize,maximum:u64)->Result<RetainedCloneSelection,ValueError>{
  if self.route.is_some(){let selected=self.selected.ok_or_else(||ValueError::invariant("record selected quote missing"))?;return Ok(RetainedCloneSelection::new(selected.0,selected.1))}
  if let Some(demand)=source.binding_demand(&self.source)?{return Ok(RetainedCloneSelection::new(demand,source.binding_work_demand(&self.source)?))}
  if self.closing||self.spent{return Err(ValueError::invariant("record cursor is closing or spent"))}
  if RetainedCloneCursor::output_ready(self){return Ok(RetainedCloneSelection::new(Default::default(),0))}
  let n=R::ROWS;let code=self.plan(source)?;
  let (demand,units)=if code==4*n{(Self::metadata_demand(),1)}else if code==4*n+1{(self.output_demand(R::variant(source.get()))?,1)}else{let g=code/4;match code%4{
   0=>return query(&self.rows,g,Some(source),|field|field.advance_selection(body,maximum))?,
   1=>query(&self.rows,g,None,|field|Ok::<_,ValueError>((field.take_demand()?,field.take_work()?)))??,
   2=>query(&self.rows,g,None,|field|Ok::<_,ValueError>((field.close_demand(body)?,field.begin_close_work()?)))??,
   _=>query(&self.rows,g,None,|field|Ok::<_,ValueError>((field.close_demand(body)?,field.close_work(body)?)))??,
  }};
  Ok(RetainedCloneSelection::new(demand,if units<=maximum{units}else{0}))
 }
 fn captured_advance_selection(&self,source:RetainedCloneRef<'_,O>,body:usize,maximum:u64)->Result<RetainedCloneSelection,ValueError>{RetainedCloneCursor::advance_selection(self,source,body,maximum)}
 fn advance_into<'r>(&mut self,source:RetainedCloneRef<'_,O>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Complete<'r>{self.advance_reply(source,body,maximum,effect,reply,false)}
 fn recover_advance_into<'r>(&mut self,source:RetainedCloneRef<'_,O>,body:usize,maximum:u64,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Complete<'r>{self.advance_reply(source,body,maximum,effect,reply,true)}
 fn take_demand(&self)->Result<RetirementDemand,ValueError>{retained_clone_cursor_take_demand::<O,bool>()}
 fn take_work_demand(&self)->Result<u64,ValueError>{Ok(u64::from(RetainedCloneCursor::output_ready(self)))}
 fn take<'r>(&mut self,output:&mut Option<O>,effect:&mut RetainedAllocationEffect<'_>,reply:OriginalEffectReply<'r,RetainedCloneStep,ValueError>)->Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,ValueError>,ValueError>{
  semio_framework_value_control_contract::take_retained_clone_cursor_output!(RetainedCloneCursor::output_ready(self),self.output,output,self.spent,true,effect,reply,O,bool)
 }
 fn begin_close_demand(&self)->Result<RetirementDemand,ValueError>{Ok(retained_close_start_demand(self.closing))}
 fn begin_close_work_demand(&self)->Result<u64,ValueError>{Ok(retained_close_start_work_demand(self.closing))}
 fn begin_close_paid(&mut self,effect:&mut RetainedAllocationEffect<'_>)->RetainedWorkOutcome<RetainedCloneProgress,ValueError>{if self.route.is_some(){return RetainedWorkOutcome{result:Err(ValueError::invariant("record action must settle before close")),work:0}}start_retained_close(&mut self.closing,effect)}
 fn begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Complete<'r>{if self.route.is_some(){return Err(ValueError::invariant("record action must settle before close"))}let turn=start_retained_close(&mut self.closing,effect);Ok(reply.complete(turn.result.map(RetainedCloneStep::Progress).map_err(Into::into)))}
 fn recover_begin_close_into<'r>(&mut self,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Complete<'r>{self.begin_close_into(effect,reply)}
 fn close_demands(&self,body:usize)->Result<RetirementDemand,ValueError>{
  if self.route.is_some(){return self.selected.map(|selected|selected.0).ok_or_else(||ValueError::invariant("record selected close quote missing"))}
  if !self.closing{return Ok(Default::default())}
  for g in 0..R::ROWS{if let Some(demand)=query(&self.rows,g,None,|field|(!field.terminal()).then(||field.close_demand(body)))?{return demand}}
  if !self.close.is_empty(){return self.close.demands(body)}
  for g in 0..R::ROWS{if let Some(demand)=query(&self.rows,g,None,|field|field.present().then(||field.owner_demands(&self.close,body)))?{return demand}}
  self.close.next_owner_demands::<O>(self.output.is_some(),&self.source,body)
 }
 fn close_work_demand(&self,body:usize)->Result<u64,ValueError>{
  if self.route.is_some(){return self.selected.map(|selected|selected.1).ok_or_else(||ValueError::invariant("record selected close work missing"))}
  if !self.closing{return Ok(0)}
  for g in 0..R::ROWS{if let Some(work)=query(&self.rows,g,None,|field|(!field.terminal()).then(||if field.closing(){field.close_work(body)}else{field.begin_close_work()}))?{return work}}
  if !self.close.is_empty(){return self.close.next_owner_work::<()>(false,&None,body)}
  for g in 0..R::ROWS{if let Some(work)=query(&self.rows,g,None,|field|field.present().then(||field.owner_work(&self.close,body)))?{return work}}
  self.close.next_owner_work::<O>(self.output.is_some(),&self.source,body)
 }
 fn close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Complete<'r>{self.close_reply(None,body,effect,reply,false)}
 fn recover_close_into<'r>(&mut self,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Complete<'r>{self.close_reply(None,body,effect,reply,true)}
 fn close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Complete<'r>{self.close_reply(Some(original),body,effect,reply,false)}
 fn recover_close_original_into<'r>(&mut self,original:&mut OriginalRetirementCloseLoan<'_>,body:usize,effect:&mut RetainedAllocationEffect<'_>,reply:Reply<'r>)->Complete<'r>{self.close_reply(Some(original),body,effect,reply,true)}
 fn next_close_depth_demand(&self)->Result<usize,ValueError>{Ok(RetainedCloneCursor::close_demands(self,0)?.depth)}
 fn terminal_is_empty(&self)->bool{self.closing&&self.route.is_none()&&self.child_reply.is_none()&&self.output.is_none()&&self.source.is_none()&&self.close.is_empty()&&self.rows_empty()}
}
