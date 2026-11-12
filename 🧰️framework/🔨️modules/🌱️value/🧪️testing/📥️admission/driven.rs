//! 🧭️ Drives owner actions through genuine original authorities: producer invocation across pending turns, and nested construction and Work recipients.
use super::*;
use super::native_decoding::{NativeDecodeControl,OriginalConstructionAuthority,OriginalConstructionRecipient,OriginalWorkAuthority,OriginalWorkRecipient,OriginalWorkPartialCheck,OriginalWorkPartialCheckComplete,OriginalWorkQuote,OriginalWorkQuoteComplete};
/// 🏦️ Runs one funded turn and takes its Work verdict.
pub fn bank_run<T,E>(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->(RetainedWorkPoll<T,E>,Option<usize>){bank_run_with_source(bank,grant,maximum,|effect,_|operation(effect))}
/// 🏦️ Runs one funded turn with the Source witness and takes its Work verdict.
pub fn bank_run_with_source<T,E>(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>,OriginalSourceTurnWitness<'_>)->RetainedWorkPoll<T,E>)->(RetainedWorkPoll<T,E>,Option<usize>){
    let turn=bank.run_with_source(grant,maximum,operation);
    if let RetainedWorkPoll::Complete(complete)=&turn.0{let verdict=bank.take_admitted();if complete.result.is_ok(){assert_ne!(verdict,Some(false),"Work refused receipt")}}
    turn
}
type Reply2<'r,F>=OriginalEffectReply<'r,RetainedCloneStep,F>;
type Done2<'r,F>=Result<OriginalEffectReplyComplete<'r,RetainedCloneStep,F>,ValueError>;
type CallFn<'t,F>=dyn for<'r,'e,'a> FnMut(&'e mut RetainedAllocationEffect<'a>,Reply2<'r,F>,bool)->Done2<'r,F>+'t;
struct Call<'t,F>(&'t mut CallFn<'t,F>);
struct Action<F>{turn:OriginalSourceTurnIdentity,kinds:std::marker::PhantomData<fn()->F>}
impl<F:Send+'static> OriginalEffectProducer for Action<F>{
    type Output=RetainedCloneStep;type Failure=F;type Context<'turn,'source>=Call<'turn,F> where 'source:'turn;
    fn validate_context<'turn,'source>(&self,_:&Call<'turn,F>)->Result<(),ValueError>where 'source:'turn{Ok(())}
    fn source_turn<'turn,'source>(&self,_:&Call<'turn,F>)->Option<OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}
    fn receive<'r,'turn,'source>(&mut self,c:&mut Call<'turn,F>,e:&mut RetainedAllocationEffect<'_>,r:Reply2<'r,F>)->Done2<'r,F>where 'source:'turn{(c.0)(e,r,false)}
    fn recover<'r,'turn,'source>(&mut self,c:&mut Call<'turn,F>,e:&mut RetainedAllocationEffect<'_>,r:Reply2<'r,F>)->Done2<'r,F>where 'source:'turn{(c.0)(e,r,true)}
}
fn quote(d:RetirementDemand)->RetainedCloneGrant{RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:d.copy_bytes,maximum_capacity_bytes:d.capacity_bytes,maximum_release_bytes:d.release_bytes,maximum_depth:d.depth}}
/// ♻️ Drives one reply-shaped action under a genuine producer authority, across pending turns.
pub fn driven<F:Send+'static>(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,mut call:impl for<'r,'e,'a> FnMut(&'e mut RetainedAllocationEffect<'a>,Reply2<'r,F>,bool)->Done2<'r,F>)->(RetainedWorkPoll<RetainedCloneStep,F>,Option<usize>){
    type Authority<F>=OriginalEffectAuthority<Action<F>>;
    let turn=bank.source_witness().identity();let call:&mut CallFn<'_,F>=&mut call;let mut authority=Some(Authority::<F>::new(Action{turn,kinds:std::marker::PhantomData}));let mut action=None;let mut spent=0u128;
    loop{
        let(poll,_)=bank_run(bank,grant,maximum,|effect|{let before=effect.performed_work();let ready=authority.as_mut().unwrap().drive(&mut Call(&mut *call),effect,&mut action);let work=effect.performed_work()-before;match ready{Ok(true)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<(),ValueError>(()),work}),Ok(false)=>RetainedWorkPoll::Pending{work},Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work})}});
        match poll{RetainedWorkPoll::Pending{work}=>spent+=work,RetainedWorkPoll::Complete(turn)=>{spent+=turn.work;turn.result.expect("driven action");break}}
    }
    let mut output=None;let transfer=quote(Authority::<F>::transfer_reply_demand().expect("reply quote"));
    bank_run(bank,transfer,1,|effect|{let turn=authority.as_mut().unwrap().transfer_reply_into(&Call(&mut *call),&mut action,&mut output,effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(drop),work:turn.work})}).0;
    let mut producer=None;let transfer=quote(Authority::<F>::producer_transfer_demand().expect("producer quote"));
    bank_run(bank,transfer,1,|effect|{let turn=Authority::<F>::transfer_producer(&mut authority,&Call(&mut *call),&mut producer,effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(drop),work:turn.work})});
    (RetainedWorkPoll::Complete(RetainedWorkOutcome{result:output.take().expect("driven reply"),work:spent}),None)
}
struct ConstructionTurn{original:std::sync::Arc<std::sync::Mutex<RetainedCloneGrant>>,accepted:std::sync::Arc<std::sync::atomic::AtomicBool>,submitted:RetainedCloneProgress,turn:OriginalSourceTurnIdentity}
impl OriginalConstructionRecipient for ConstructionTurn{
    type Context<'turn,'source>=() where 'source:'turn;
    fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
    fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}
    fn grant<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalConstructionGrant<'a>)->OriginalConstructionGrantComplete<'a>where 'source:'turn{quote.complete(Ok(*self.original.lock().unwrap()))}
    fn accept<'a,'turn,'source>(&mut self,_:&mut (),receipt:OriginalConstructionAccept<'a>)->OriginalConstructionAcceptComplete<'a>where 'source:'turn{let p=receipt.progress();self.accepted.store(true,std::sync::atomic::Ordering::SeqCst);let result=self.submitted.checked_add(p).map(|sum|self.submitted=sum);receipt.complete(result)}
    fn submitted<'turn,'source>(&self,_:&())->RetainedCloneProgress where 'source:'turn{self.submitted}
}
struct ConstructionWork{turn:OriginalSourceTurnIdentity}
impl OriginalWorkRecipient for ConstructionWork{
    type Context<'turn,'source>=() where 'source:'turn;
    fn validate_context<'turn,'source>(&self,_:&())->Result<(),ValueError>where 'source:'turn{Ok(())}
    fn source_turn<'turn,'source>(&self,_:&())->Option<OriginalSourceTurnIdentity>where 'source:'turn{Some(self.turn)}
    fn settle_partial<'a,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'a>)->OriginalWorkPartialCheckComplete<'a>where 'source:'turn{check.complete(Ok(()))}
    fn recover_partial<'a,'turn,'source>(&mut self,_:&mut (),check:OriginalWorkPartialCheck<'a>)->OriginalWorkPartialCheckComplete<'a>where 'source:'turn{check.complete(Ok(()))}
    fn admits<'a,'turn,'source>(&mut self,_:&mut (),quote:OriginalWorkQuote<'a>,_:u64,_:u64)->OriginalWorkQuoteComplete<'a>where 'source:'turn{quote.complete(Ok(true))}
}
/// 🪪️ Lends construction and Work authorities to nested turns.
pub struct Authorities{turn:OriginalSourceTurn,construction:OriginalConstructionAuthority<ConstructionTurn>,work:OriginalWorkAuthority<ConstructionWork>,grant:std::sync::Arc<std::sync::Mutex<RetainedCloneGrant>>}
impl Authorities{
    /// 🏦️ Runs one funded local effect under the same persistent Work and construction authorities as every nested turn.
    pub fn effect<R>(&mut self,control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant,work:u64,f:impl FnOnce(&mut RetainedAllocationEffect<'_>)->R)->Option<R>{
        *self.grant.lock().unwrap()=g;
        self.construction.with_loan(&mut (),|loan|self.work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|control.with_effect(g,work,|_,effect|f(effect)))))).unwrap().unwrap().unwrap()
    }
    /// 🎟️ Runs one funded effect that also lends the wired native control, so a receiving cursor can step under the same persistent authorities; an unadmitted quote ran nothing and yields `None`.
    pub fn step_effect<R>(&mut self,control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant,work:u64,f:impl FnOnce(&mut NativeDecodeControl<'_>,&mut RetainedAllocationEffect<'_>)->R)->Option<R>{
        *self.grant.lock().unwrap()=g;
        self.construction.with_loan(&mut (),|loan|self.work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|control.with_effect(g,work,|control,effect|f(control,effect)))))).unwrap().unwrap().unwrap()
    }
    /// 🧾️ Reads the physical progress the resident construction recipient has accepted so far.
    pub fn submitted(&self)->RetainedCloneProgress{self.construction.inspect(|turn|turn.submitted)}
    pub fn new()->Self{let issuer=OriginalSourceTurnIssuer::new();let turn=issuer.issue().expect("fixture Source dispatch");let identity=turn.identity();let grant=std::sync::Arc::new(std::sync::Mutex::new(RetainedCloneGrant::default()));let accepted=std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));Self{turn,construction:OriginalConstructionAuthority::new(ConstructionTurn{original:grant.clone(),accepted,submitted:Default::default(),turn:identity}),work:OriginalWorkAuthority::new(ConstructionWork{turn:identity}),grant}}
    pub fn nested<T,E>(&mut self,control:&mut NativeDecodeControl<'_>,g:RetainedCloneGrant,f:impl FnOnce(&mut NativeDecodeControl<'_>,OriginalSourceTurnWitness<'_>)->RetainedWorkPoll<T,E>)->RetainedWorkPoll<T,E>{
        *self.grant.lock().unwrap()=g;let witness=self.turn.witness();let bound=unsafe{&*std::ptr::from_ref(&self.turn)}.witness();let mut poll=None;
        self.construction.with_loan(&mut (),|loan|self.work.with_loan(&mut (),|work_loan|control.with_work_recipient(work_loan,|control|control.with_construction_recipient(loan,|control|{control.with_source_witness(Some(bound),|control|{poll=Some(f(control,witness))});Ok::<(),ValueError>(())}))).unwrap()).unwrap();
        poll.expect("nested turn polled")
    }
}

const PENDING_PASSES:u32=2048;
struct Session{bank:OriginalSourceTurnIdentity,payer:Authorities,control:NativeDecodeControl<'static>}
thread_local!{
    static SESSION:std::cell::RefCell<Option<std::mem::ManuallyDrop<Session>>>=const{std::cell::RefCell::new(None)};
    static ADMIT:std::cell::RefCell<Option<std::sync::Arc<std::sync::atomic::AtomicBool>>>=const{std::cell::RefCell::new(None)};
}
fn open_session(bank:OriginalSourceTurnIdentity)->Session{
    let callback:&'static mut dyn FnMut(super::native_decoding::NativeDecodeProgress)->bool=Box::leak(Box::new(|_|ADMIT.with(|flag|flag.borrow().as_ref().map_or(true,|flag|flag.load(std::sync::atomic::Ordering::SeqCst)))));
    let mut session=Session{bank,payer:Authorities::new(),control:NativeDecodeControl::new(1<<24,callback)};
    let recipient:&'static mut super::native_decoding::NativeDecodeRetirementRecipient=Box::leak(Box::new(super::native_decoding::NativeDecodeRetirementRecipient::new()));session.control.install_retirement_recipient(recipient).expect("session retirement recipient");
    let _=session.payer.nested(&mut session.control,RetainedCloneGrant::default(),|_,_|RetainedWorkPoll::<(),ValueError>::Pending{work:0});
    session
}
fn with_session<R>(bank:&EffectBank,operation:impl FnOnce(&mut Session)->R)->R{
    SESSION.with(|cell|{
        let mut slot=cell.borrow_mut();let id=bank.source_witness().identity();
        if slot.as_ref().is_none_or(|session|session.bank!=id){*slot=Some(std::mem::ManuallyDrop::new(open_session(id)))}
        operation(slot.as_mut().expect("decoder session open"))
    })
}
/// 🪪️ Opens the decoder session of this bank before any heap-observed turn.
pub fn session_open(bank:&EffectBank){with_session(bank,|_|())}
/// ✂️ Sets the cancellation flag the session's checkpoints read.
pub fn session_admit(flag:Option<std::sync::Arc<std::sync::atomic::AtomicBool>>){ADMIT.with(|slot|*slot.borrow_mut()=flag)}
/// 🧵️ Runs one nested turn under the persistent decoder session, so every turn of one receiver shares one Work scope, issuer and Source.
pub fn session_nested<T,E>(bank:&EffectBank,g:RetainedCloneGrant,f:impl FnOnce(&mut NativeDecodeControl<'_>,OriginalSourceTurnWitness<'_>)->RetainedWorkPoll<T,E>)->RetainedWorkPoll<T,E>{with_session(bank,|session|{let Session{payer,control,..}=session;payer.nested(control,g,f)})}
/// 🏦️ Runs one funded local turn under the persistent decoder session and debits its work from the bank wallet.
pub fn session_run<T,E>(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,operation:impl FnOnce(&mut RetainedAllocationEffect<'_>)->RetainedWorkPoll<T,E>)->(RetainedWorkPoll<T,E>,Option<usize>){
    assert!(u128::from(maximum)<=u128::from(bank.wallet().remaining_work()),"session fixture work stock exhausted: the turn needs {maximum} Work, {} remain",bank.wallet().remaining_work());
    if maximum==0{return bank.run(grant,0,operation)}
    let(turn,actual)=with_session(bank,|session|{let Session{payer,control,..}=session;let mut actual=None;let turn=payer.effect(control,grant,maximum,|effect|{let turn=operation(effect);actual=effect.actual().take();turn}).expect("session Work quote refused");(turn,actual)});
    let spent=match &turn{RetainedWorkPoll::Pending{work}=>*work,RetainedWorkPoll::Complete(done)=>done.work};bank.debit_work(spent);
    let actual=matches!(&turn,RetainedWorkPoll::Complete(_)).then_some(actual).flatten();(turn,actual)
}
/// ♻️ Like [`driven`], but every turn runs under the persistent decoder session.
pub fn session_driven<F:Send+'static>(bank:&mut EffectBank,grant:RetainedCloneGrant,maximum:u64,mut call:impl for<'r,'e,'a> FnMut(&'e mut RetainedAllocationEffect<'a>,Reply2<'r,F>,bool)->Done2<'r,F>)->(RetainedWorkPoll<RetainedCloneStep,F>,Option<usize>){
    type Authority<F>=OriginalEffectAuthority<Action<F>>;
    let turn=with_session(bank,|session|session.payer.turn.identity());let call:&mut CallFn<'_,F>=&mut call;let mut authority=Some(Authority::<F>::new(Action{turn,kinds:std::marker::PhantomData}));let mut action=None;let mut spent=0u128;
    for pass in 0u32..{
        assert!(pass<PENDING_PASSES,"driven action stayed pending for {PENDING_PASSES} passes: spent {spent} Work, grant {grant:?}, maximum {maximum}");
        let(poll,_)=session_run(bank,grant,maximum,|effect|{let before=effect.performed_work();let ready=authority.as_mut().unwrap().drive(&mut Call(&mut *call),effect,&mut action);let work=effect.performed_work()-before;match ready{Ok(true)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Ok::<(),ValueError>(()),work}),Ok(false)=>RetainedWorkPoll::Pending{work},Err(error)=>RetainedWorkPoll::Complete(RetainedWorkOutcome{result:Err(error),work})}});
        match poll{RetainedWorkPoll::Pending{work}=>spent+=work,RetainedWorkPoll::Complete(turn)=>{spent+=turn.work;turn.result.expect("driven action");break}}
    }
    let mut output=None;let transfer=quote(Authority::<F>::transfer_reply_demand().expect("reply quote"));
    session_run(bank,transfer,1,|effect|{let turn=authority.as_mut().unwrap().transfer_reply_into(&Call(&mut *call),&mut action,&mut output,effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(drop),work:turn.work})}).0;
    let mut producer=None;let transfer=quote(Authority::<F>::producer_transfer_demand().expect("producer quote"));
    session_run(bank,transfer,1,|effect|{let turn=Authority::<F>::transfer_producer(&mut authority,&Call(&mut *call),&mut producer,effect);RetainedWorkPoll::Complete(RetainedWorkOutcome{result:turn.result.map(drop),work:turn.work})});
    (RetainedWorkPoll::Complete(RetainedWorkOutcome{result:output.take().expect("driven reply"),work:spent}),None)
}
