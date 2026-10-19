use semio_framework_job::{CancelToken,Generation,OperationId,OriginalJobDecodeOwner,OriginalJobFixtureCaller,RetainedCloneGrant,StepContext};

/// 🎟️ A fixture keeps one finite original source, decoder and cumulative physical recipient across every turn.
pub struct FixtureTurnV1 {
    grant:RetainedCloneGrant,
    caller:Option<OriginalJobFixtureCaller>,
    decoder:OriginalJobDecodeOwner,
    epoch:u64,
    surfaces:Vec<String>,
}

impl FixtureTurnV1 {
    pub fn new(grant:RetainedCloneGrant)->Self{Self::with_identity(grant,OperationId(41),Generation(7))}
    pub fn with_identity(grant:RetainedCloneGrant,operation:OperationId,generation:Generation)->Self{
        let policy:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).expect("original renderer fixture declaration");
        let stock:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();
        Self{grant,caller:Some(OriginalJobFixtureCaller::admit(operation,generation,stock,||Some(0),u64::MAX,policy["fuel"].as_u64().unwrap())),decoder:Default::default(),epoch:1,surfaces:Vec::new()}
    }
    pub fn receipt(&self)->semio_framework_job::RetainedCloneProgress{self.caller.as_ref().unwrap().receipt()}
    pub fn alias(&mut self)->CancelToken{self.caller.as_mut().unwrap().admit_alias(self.grant)}
    pub fn own_surface(&mut self,surface:&str){if !self.surfaces.iter().any(|owned|owned==surface){self.surfaces.push(surface.to_owned());}}
    pub fn retire_world(&mut self,world:&mut infinite_world::world::World3dBuildContext){
        for _ in 0..65_536{if world.terminal_is_empty(){break}self.turn(|cx|world.close_step(cx)).expect("original fixture world retirement");}
        assert!(world.terminal_is_empty());
    }
    pub fn turn<R>(&mut self,advance:impl FnOnce(&mut StepContext<'_>)->R)->R{self.limited(self.grant,advance)}
    pub fn limited<R>(&mut self,grant:RetainedCloneGrant,advance:impl FnOnce(&mut StepContext<'_>)->R)->R{
        let mut result=None;let epoch=self.epoch;self.epoch=self.epoch.checked_add(1).expect("original renderer fixture epoch exhausted");
        self.caller.as_mut().expect("original renderer fixture is live").turn_with_decoder(grant,&mut self.decoder,|cx|{
            cx.bind_original_source_epoch(epoch)?;
            cx.with_original_work_loan(1,cx.deadline_us(),|same|{result=Some(advance(same));Ok(())})?;Ok(())
        }).expect("original renderer fixture accepts the exact cumulative receipt");
        result.expect("original fixture returned its held value")
    }
    fn close(&mut self){
        if self.caller.is_none(){return}
        for surface in &self.surfaces{semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::request_ui_document_close(surface);}
        for _ in 0..65_536{if !self.surfaces.iter().any(|surface|semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending_for(surface)){break}self.turn(semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::close_ui_document_one).expect("original fixture surface retirement");}
        assert!(!self.surfaces.iter().any(|surface|semio_framework_os_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending_for(surface)));
        let caller=self.caller.as_mut().unwrap();let stock=caller.stock();
        for _ in 0..65_536{if self.decoder.terminal_is_empty(){break}caller.turn(stock,|cx|{let demand=self.decoder.retirement_demands(cx.retained_grant().maximum_copy_bytes)?;semio_framework_job::close_original_retained_step(cx,demand,|grant|self.decoder.close_step(grant))?;Ok(())}).expect("same original renderer decoder close");}
        assert!(self.decoder.terminal_is_empty());self.caller.take().unwrap().retire(stock);
    }
    pub fn finish(mut self){self.close();}
}
impl Drop for FixtureTurnV1{fn drop(&mut self){if !std::thread::panicking(){self.close();}}}
