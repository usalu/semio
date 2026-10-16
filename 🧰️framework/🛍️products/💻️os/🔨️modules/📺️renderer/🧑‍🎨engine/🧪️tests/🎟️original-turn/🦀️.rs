use semio_framework_job::{CancelToken,Generation,JobPayloadAuthority,OperationId,RetainedCloneGrant,RetainedCloneProgress,StepBudget,StepContext};

/// 🎟️ A fixture caller retains one admitted ledger, cancellation, clock and physical receipt recipient.
pub struct FixtureTurnV1 {
    grant:RetainedCloneGrant,
    parent:semio_framework_job::OriginalCallerSourceParent,
    sequence:u64,
    epoch:u64,
    receipt:RetainedCloneProgress,
    surfaces:Vec<String>,
}

impl FixtureTurnV1 {
    pub fn new(grant:RetainedCloneGrant)->Self{
        let (cancel,birth)=CancelToken::admit_root(grant).expect("fixture cancellation admission").expect("fixture cancellation funded");
        assert!(birth.fits(grant));
        let (authority,birth)=JobPayloadAuthority::admit(OperationId(41),Generation(7),grant).expect("fixture ledger admission").expect("fixture ledger funded");
        assert!(birth.fits(grant));
        let (parent,birth)=semio_framework_job::OriginalCallerSourceParent::admit(authority,cancel,grant).unwrap_or_else(|_|panic!("fixture original source parent funded"));assert!(birth.fits(grant));
        Self{grant,parent,epoch:1,sequence:0,receipt:Default::default(),surfaces:Vec::new()}
    }

    pub fn own_surface(&mut self,surface:&str){if !self.surfaces.iter().any(|owned|owned==surface){self.surfaces.push(surface.to_owned());}}

    pub fn retire_world(&mut self,world:&mut infinite_world::world::World3dBuildContext){
        for _ in 0..65_536{if world.terminal_is_empty(){break}self.turn(|cx|world.close_step(cx)).expect("original fixture world retirement");}
        assert!(world.terminal_is_empty());
    }

    pub fn turn<R>(&mut self,advance:impl FnOnce(&mut StepContext<'_>)->R)->R{
        assert!(self.receipt.fits(self.grant),"fixture explicitly accepts only a funded previous receipt");
        self.receipt=Default::default();
        let epoch=self.epoch;self.epoch=self.epoch.checked_add(1).expect("fixture original issued epoch is finite");
        let mut cx=self.parent.borrow_context(StepBudget::new(1,u64::MAX,self.grant),||Some(0),&mut self.sequence,&mut self.receipt).expect("same original fixture caller");
        cx.bind_original_source_epoch(epoch).expect("fixture supplier stamps its own admitted turn epoch");
        advance(&mut cx)
    }

    pub fn finish(mut self){
        for surface in &self.surfaces{semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::request_ui_document_close(surface);}
        for _ in 0..65_536{if !self.surfaces.iter().any(|surface|semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending_for(surface)){break}self.turn(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::close_ui_document_one).expect("original fixture surface retirement");}
        assert!(!self.surfaces.iter().any(|surface|semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending_for(surface)));
        assert!(self.receipt.fits(self.grant));
        for _ in 0..128{if self.parent.terminal_is_empty(){break}self.parent.close_step(self.grant).expect("original fixture parent physical retirement");}
        assert!(self.parent.terminal_is_empty(),"fixture must return real original source loans before retiring parent roots");
    }
}
