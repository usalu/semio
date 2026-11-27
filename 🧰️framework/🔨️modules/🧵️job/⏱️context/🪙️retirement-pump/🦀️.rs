//! 🪙️ The actor host's retirement pump issues owner-less close turns that charge host-funded work.
use super::*;
use semio_framework_job_receiving::OriginalJobWorkRecipient;

/// 🏛️ The actor host's retirement pump is the only issuer of owner-less retirement turns.
#[derive(Clone,Copy,Debug,Default)]
pub struct OriginalRetirementPump;
impl OriginalRetirementPump{
    /// 🆕️ Names the host pump that funds owner-less retirement turns.
    pub fn new()->Self{Self}
    /// 🎬️ Issues one host-funded turn carrying its own Source dispatch token.
    pub fn turn(&self,fuel:u64,deadline_us:u64,now_us:fn()->Option<u64>,grant:RetainedCloneGrant)->Result<OriginalRetirementTurn,ValueError>{OriginalRetirementTurn::new(fuel,deadline_us,now_us,grant)}
}
/// 🪙️ One host-funded owner-less close turn: its work is charged to the host budget, and as physical close it stays admitted after the semantic deadline (see `StepContext::retirement_should_yield`).
pub struct OriginalRetirementTurn{fuel:u64,deadline_us:u64,now_us:fn()->Option<u64>,grant:RetainedCloneGrant,turn:OriginalSourceTurn}
impl OriginalRetirementTurn{
    fn new(fuel:u64,deadline_us:u64,now_us:fn()->Option<u64>,grant:RetainedCloneGrant)->Result<Self,ValueError>{let turn=OriginalSourceTurnIssuer::new().issue().map_err(|_|ValueError::invariant("original retirement turn identity exhausted"))?;Ok(Self{fuel,deadline_us,now_us,grant,turn})}
    /// 🧾️ Reads the Source turn this retirement turn charges.
    pub fn source_turn(&self)->OriginalSourceTurnIdentity{self.turn.identity()}
    /// 🪙️ Runs one closure inside one original effect settled against this turn's budget; `None` means the Work quote was not admitted and nothing ran.
    pub fn with_effect<R>(&mut self,run:impl FnOnce(&mut semio_framework_value_control_contract::RetainedAllocationEffect<'_>)->R)->Result<Option<(R,u128)>,ValueError>{
        use semio_framework_value_control_contract::{OriginalWork,OriginalWorkAuthority,OriginalWorkSink,RetainedAllocationEffect};
        let maximum=self.fuel;let fuel=self.fuel;let deadline=self.deadline_us;let now=self.now_us;let grant=self.grant;let source=Some(self.turn.identity());let retirement=true;
    let mut ready=move||retirement||now().is_some_and(|value|value<deadline);
    let mut authority=OriginalWorkAuthority::new(semio_framework_job_receiving::OriginalJobWorkDriver);
    let result=authority.with_loan(&mut semio_framework_job_receiving::OriginalJobWorkRecipient::new(&fuel,&mut ready).with_source(source),|loan|{
        let mut loan=loan.reborrow();let mut work=OriginalWork::empty();work.recipient=Some(&mut loan);let mut action=None;let mut proof=None;
        if work.prepare_for_action(maximum,&mut action)?!=Some(true){if action.is_some(){work.cancel_for_action(&mut action)?}return Ok(None)}
        let mut exchange=||->Result<Option<(R,u128)>,ValueError>{
        let absent=||ValueError::invariant("original effect receipt absent");
        let mut receipt=None;work.issue_performed_for_action(action,&mut receipt)?;let mut actual=None;
        let output={let sink=receipt.as_mut().ok_or_else(absent)?;let mut effect=RetainedAllocationEffect::new(grant,maximum,&mut actual,sink);let mut run=Some(run);let mut output=None;{let mut call=|effect:&mut RetainedAllocationEffect<'_>|{if let Some(run)=run.take(){output=Some(run(effect))}};semio_framework_value_control_contract::with_original_invocation(source,&mut effect,&mut call)?;}output.ok_or_else(||ValueError::invariant("original effect call did not run"))?};
        let performed=OriginalWorkSink::observed_work(receipt.as_ref().ok_or_else(absent)?);work.claim_performed_for_action(action,&mut receipt)?;
        if !work.performed_seen_for_action(action)?{work.cancel_for_action(&mut action)?}else{
            let mut partial=None;if !work.settle_partial_for_action_into(action,&mut partial)?{return Err(ValueError::invariant("original effect partial receipt is pending"))}
            let partial=partial.take().ok_or_else(||ValueError::invariant("original effect partial outcome absent"))?;if partial.total()!=performed||partial.source_refusal().is_some(){return Err(ValueError::invariant("original effect partial outcome differs from its performed work"))}
            let accepted=work.commit_for_action(performed,action,&mut proof)?;work.finish_for_action(&mut proof,&mut action)?;if !accepted{return Err(ValueError::literal(ValueRefusalKind::WorkLimit,"original effect performed more work than its ceiling"))}
        }
            Ok(Some((output,performed)))
        };
        match exchange(){Ok(done)=>Ok(done),Err(error)=>{if action.is_some(){let _=work.cancel_for_action(&mut action);}Err(error)}}
    })??;
    if authority.has_live_actions(){return Err(ValueError::invariant("original effect left a live work action"))}
    Ok(result)
    }
    /// 🛬️ Lends this turn's remaining fuel as the original work loan of a native decoder and charges the performed work back to the turn.
    pub fn with_native_decoder_work<T,E:From<ValueError>>(&mut self,native:&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>,operation:impl FnOnce(&mut semio_framework_value_native_owned::native_decoding::NativeDecodeControl<'_>)->Result<T,E>)->Result<T,E>{
        let fuel=self.fuel;let deadline=self.deadline_us;let now=self.now_us;let mut ready=move||now().is_some_and(|value|value<deadline);let before=native.actual_work();
        let mut authority=semio_framework_value_control_contract::OriginalWorkAuthority::new(semio_framework_job_receiving::OriginalJobWorkDriver);
        let(mut spent,mut receiving)=(RetainedCloneProgress::default(),RetainedCloneProgress::default());let mut physical=semio_framework_job_receiving::OriginalJobConstructionRecipient::new(self.grant,Default::default(),&mut spent,&mut receiving);
        let mut construction=semio_framework_value_control_contract::OriginalConstructionAuthority::new(semio_framework_job_receiving::OriginalJobConstructionDriver);
        let result=authority.with_loan(&mut OriginalJobWorkRecipient::new(&fuel,&mut ready),|loan|native.with_work_recipient(loan,|native|construction.with_loan(&mut physical,|loan|native.with_construction_recipient(loan,operation)).map_err(E::from).and_then(|result|result))).map_err(E::from).and_then(|result|result)?;
        self.fuel=self.fuel.saturating_sub(u64::try_from(native.actual_work().saturating_sub(before)).unwrap_or(u64::MAX));Ok(result)
    }
    /// 🛫️ Lends this turn's remaining fuel as the original work loan of a native encoder and charges the performed work back to the turn.
    pub fn with_native_encoder_work<T,E:From<ValueError>>(&mut self,native:&mut semio_framework_value::NativeEncodeControl<'_>,operation:impl FnOnce(&mut semio_framework_value::NativeEncodeControl<'_>)->Result<T,E>)->Result<T,E>{
        let fuel=self.fuel;let deadline=self.deadline_us;let now=self.now_us;let mut ready=move||now().is_some_and(|value|value<deadline);let before=native.actual_work();
        let mut authority=semio_framework_value_control_contract::OriginalWorkAuthority::new(semio_framework_job_receiving::OriginalJobWorkDriver);
        let(mut spent,mut receiving)=(RetainedCloneProgress::default(),RetainedCloneProgress::default());let mut physical=semio_framework_job_receiving::OriginalJobConstructionRecipient::new(self.grant,Default::default(),&mut spent,&mut receiving);
        let mut construction=semio_framework_value_control_contract::OriginalConstructionAuthority::new(semio_framework_job_receiving::OriginalJobConstructionDriver);
        let result=authority.with_loan(&mut OriginalJobWorkRecipient::new(&fuel,&mut ready),|loan|native.with_work_recipient(loan,|native|construction.with_loan(&mut physical,|loan|native.with_construction_recipient(loan,operation)).map_err(E::from).and_then(|result|result))).map_err(E::from).and_then(|result|result)?;
        self.fuel=self.fuel.saturating_sub(u64::try_from(native.actual_work().saturating_sub(before)).unwrap_or(u64::MAX));Ok(result)
    }
}
