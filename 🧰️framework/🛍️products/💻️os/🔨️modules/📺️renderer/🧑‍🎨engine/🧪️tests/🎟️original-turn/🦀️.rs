use semio_framework_job::{CancelToken,Generation,JobPayloadAuthority,OperationId,RetainedCloneGrant,RetainedCloneProgress,StepBudget,StepContext};

/// 🎟️ A fixture caller retains one admitted ledger, cancellation, clock and physical receipt recipient.
pub(crate) struct FixtureTurnV1 {
    grant:RetainedCloneGrant,
    cancel:CancelToken,
    authority:JobPayloadAuthority,
    sequence:u64,
    receipt:RetainedCloneProgress,
}

impl FixtureTurnV1 {
    pub(crate) fn new(grant:RetainedCloneGrant)->Self{
        let (cancel,birth)=CancelToken::admit_root(grant).expect("fixture cancellation admission").expect("fixture cancellation funded");
        assert!(birth.fits(grant));
        let (authority,birth)=JobPayloadAuthority::admit(OperationId(41),Generation(7),grant).expect("fixture ledger admission").expect("fixture ledger funded");
        assert!(birth.fits(grant));
        Self{grant,cancel,authority,sequence:0,receipt:Default::default()}
    }

    pub(crate) fn turn<R>(&mut self,advance:impl FnOnce(&mut StepContext<'_>)->R)->R{
        assert!(self.receipt.fits(self.grant),"fixture explicitly accepts only a funded previous receipt");
        self.receipt=Default::default();
        let mut cx=StepContext::with_payload_authority(OperationId(41),Generation(7),StepBudget::new(1,u64::MAX,self.grant),&self.cancel,||Some(0),&mut self.sequence,&mut self.receipt,&self.authority).expect("same original fixture caller");
        advance(&mut cx)
    }

    pub(crate) fn finish(mut self){
        assert!(self.receipt.fits(self.grant));
        for _ in 0..128{if self.authority.terminal_is_empty(){break}self.authority.close_step(self.grant).expect("original fixture ledger retirement");}
        assert!(self.authority.terminal_is_empty(),"fixture must close its real retained children before retiring caller authority");
        drop(self.authority);
        let mut cancel=semio_framework_async::CancelTokenRetirement::from_token(self.cancel);
        for _ in 0..128{if cancel.terminal_is_empty(){break}cancel.close_step(self.grant).expect("original fixture cancellation retirement");}
        assert!(cancel.terminal_is_empty());
    }
}
