use semio_framework_job::{CancelToken,Generation,OperationId,OriginalJobDecodeOwner,OriginalJobFixtureCaller,RetainedCloneGrant,StepContext};

/// 🎟️ The fuel one document-close turn is quoted: a paid close settles one unit per node and layout record it retires, which the one-unit default turn cannot hold.
const CLOSE_TURN_WORK:u64=1<<20;

/// 🎟️ The factor by which a wide fixture stretches its one finite Source stock: it stands for a whole scenario of native frame events, each of which is a fresh caller with its own stock, so its cumulative copies must not run out within a scenario that mounts hundreds of windows.
const WIDE_STOCK_SCALE:usize=1<<10;

/// 🎟️ A fixture keeps one finite original source, decoder and cumulative physical recipient across every turn.
pub struct FixtureTurnV1 {
    grant:RetainedCloneGrant,
    caller:Option<OriginalJobFixtureCaller>,
    decoder:OriginalJobDecodeOwner,
    epoch:u64,
    surfaces:Vec<String>,
    _arena:semio_ui_contract_values::testkit::UiArenaFixture,
}

impl FixtureTurnV1 {
    pub fn new(grant:RetainedCloneGrant)->Self{Self::with_identity(grant,OperationId(41),Generation(7))}
    /// 🎟️ A fixture whose every turn runs at `generation`, for the callers whose own identity a law pins.
    pub fn for_generation(grant:RetainedCloneGrant,generation:u64)->Self{Self::with_identity(grant,OperationId(41),Generation(generation))}
    /// 🎟️ A fixture at `generation` whose `grant` is widened enough for layout, paint and surface retirement to copy their parameter records.
    pub fn wide(mut grant:RetainedCloneGrant,generation:u64)->Self{
        grant.maximum_items=1<<24;grant.maximum_copy_bytes=1<<30;grant.maximum_capacity_bytes=1<<30;grant.maximum_release_bytes=1<<30;grant.maximum_depth=4096;
        Self::with_stock(grant,OperationId(41),Generation(generation),WIDE_STOCK_SCALE)
    }
    pub fn with_identity(grant:RetainedCloneGrant,operation:OperationId,generation:Generation)->Self{Self::with_stock(grant,operation,generation,1)}
    fn with_stock(grant:RetainedCloneGrant,operation:OperationId,generation:Generation,stock_scale:usize)->Self{
        let arena=semio_ui_contract_values::testkit::UiArenaFixture::new();
        let policy:serde_json::Value=serde_json::from_str(include_str!("🔣️.json")).expect("original renderer fixture declaration");
        let mut stock:RetainedCloneGrant=serde_json::from_value(policy["grant"].clone()).unwrap();
        stock.maximum_items*=stock_scale;stock.maximum_copy_bytes*=stock_scale;stock.maximum_capacity_bytes*=stock_scale;stock.maximum_release_bytes*=stock_scale;
        let mut caller = OriginalJobFixtureCaller::admit(operation, generation, stock, ||Some(0), u64::MAX, policy["fuel"].as_u64().unwrap());
        let mut decoder = OriginalJobDecodeOwner::empty();
        let admitted = (0..64).any(|_| caller.turn(stock, |cx| decoder.admit_step(cx)).expect("original renderer fixture decoder admission"));
        assert!(admitted, "original renderer fixture decoder admission must complete");
        Self { grant, caller: Some(caller), decoder, epoch: 1, surfaces: Vec::new(), _arena: arena }
    }
    pub fn receipt(&self)->semio_framework_job::RetainedCloneProgress{self.caller.as_ref().unwrap().receipt()}
    pub fn cumulative_receipt(&self)->semio_framework_job::RetainedCloneProgress{self.caller.as_ref().unwrap().cumulative_receipt()}
    pub fn alias(&mut self)->CancelToken{self.caller.as_mut().unwrap().admit_alias(self.grant)}
    pub fn own_surface(&mut self,surface:&str){if !self.surfaces.iter().any(|owned|owned==surface){self.surfaces.push(surface.to_owned());}}
    pub fn retire_world(&mut self,world:&mut infinite_world::world::World3dBuildContext){
        for _ in 0..65_536{if world.terminal_is_empty(){break}self.turn(|cx|world.close_step(cx)).expect("original fixture world retirement");}
        assert!(world.terminal_is_empty());
    }
    pub fn turn<R>(&mut self,advance:impl FnOnce(&mut StepContext<'_>)->R)->R{self.limited(self.grant,advance)}
    /// 🎟️ One turn that quotes `work` fuel units, for steps (such as source admission) that demand more than the default one.
    pub fn turn_funded<R>(&mut self,work:u64,advance:impl FnOnce(&mut StepContext<'_>)->R)->R{self.limited_funded(self.grant,work,advance)}
    pub fn limited<R>(&mut self,grant:RetainedCloneGrant,advance:impl FnOnce(&mut StepContext<'_>)->R)->R{self.limited_funded(grant,1,advance)}
    fn limited_funded<R>(&mut self,grant:RetainedCloneGrant,work:u64,advance:impl FnOnce(&mut StepContext<'_>)->R)->R{
        let mut result=None;let epoch=self.epoch;self.epoch=self.epoch.checked_add(1).expect("original renderer fixture epoch exhausted");
        self.caller.as_mut().expect("original renderer fixture is live").turn_with_decoder(grant,&mut self.decoder,|cx|{
            cx.bind_original_source_epoch(epoch)?;
            cx.with_original_work_loan(work,cx.deadline_us(),|same|{result=Some(advance(same));Ok(())})?;Ok(())
        }).expect("original renderer fixture accepts the exact cumulative receipt");
        result.expect("original fixture returned its held value")
    }
    fn close(&mut self){self.close_with(&mut||{})}
    /// 🎟️ Closes the owned surfaces, running `host` after every close turn so the caller can serve the component close requests the engine hands its host.
    fn close_with(&mut self,host:&mut dyn FnMut()){
        if self.caller.is_none(){return}
        for surface in &self.surfaces{semio_renderer_wgpu_ui_engine::retained_ui_engine::request_ui_document_close(surface);}
        for _ in 0..65_536{if !self.surfaces.iter().any(|surface|semio_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending_for(surface)){break}self.turn_funded(CLOSE_TURN_WORK,semio_renderer_wgpu_ui_engine::retained_ui_engine::close_ui_document_one).expect("original fixture surface retirement");host();}
        assert!(!self.surfaces.iter().any(|surface|semio_renderer_wgpu_ui_engine::retained_ui_engine::ui_document_close_pending_for(surface)));
        let caller=self.caller.as_mut().unwrap();let stock=caller.stock();
        for _ in 0..65_536{if self.decoder.terminal_is_empty(){break}caller.turn(stock,|cx|{self.decoder.close_on_source(cx)?;Ok(())}).expect("same original renderer decoder close");}
        assert!(self.decoder.terminal_is_empty());self.caller.take().unwrap().retire(stock);
    }
    pub fn finish(mut self){self.close();}
    pub fn finish_with(mut self,host:&mut dyn FnMut()){self.close_with(host);}
}
impl Drop for FixtureTurnV1{fn drop(&mut self){if !std::thread::panicking(){self.close();}}}

#[cfg(test)]
mod arena_lifetime_laws{
    use super::*;
    #[test]
    fn a_fixture_turn_registers_the_ui_arena_lifetime_for_its_whole_scope_and_composes_with_an_explicit_scope(){
        let fixture=FixtureTurnV1::new(ui_contract::UI_WORKER_RETIREMENT_POLICY);
        assert!(semio_ui_contract_values::ui_value_arena_registry().borrow().expect("registered").is_some(),"shell fixtures own an accepting UI arena lifetime");
        drop(semio_ui_contract_values::testkit::UiArenaFixture::new());assert!(semio_ui_contract_values::ui_value_arena_registry().borrow().expect("registered").is_some(),"an explicit scope on the same thread nests instead of deadlocking");
        fixture.finish();
        assert!(semio_ui_contract_values::ui_value_arena_registry().borrow().is_err(),"closing the fixture closes the lifetime");
    }
}
