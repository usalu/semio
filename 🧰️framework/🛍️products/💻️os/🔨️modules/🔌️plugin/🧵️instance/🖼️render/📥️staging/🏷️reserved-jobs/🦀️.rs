//! 🎽️ Staged `reserved-jobs` members of the `instance-render` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! 🏷️ The inputs of the reserved action and media jobs.
use semio_plugin_decl_emission::{ActionMeta, Emit};
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer;
use pack::snapshot::ArtifactPack;
use semio_plugin_decl_app::tool_contract::{ArtifactReservedToolInput, ArtifactReservedToolJob, ArtifactReservedToolJobRequest, ArtifactToolCompletion};
use semio_plugin_decl_views::child_view::ChildContentView;
use semio_framework_value::DslValue;
use semio_framework::Media;
use std::future::Future;
use semio_plugin_instance_extension::InstanceExtension;
use store::{MemberFactory, Mutation, ReferenceSpaceMember};
use protocol::OpText;
use semio_plugin_reactor_app::PluginApp;
use crate::render_bundle::RenderCtx;
use semio_ui_contract::*;

impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> RenderCtx<'ctx, A, M, X> {
    /// 🧷️ Prepares the exact current reserved owner metadata and completion under the initiating turn.
    pub fn prepare_reserved_header(&mut self, action:&str, meta:&ActionMeta, cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if let Some(owner)=self.pending.reserved_preparation.as_ref(){owner.validate(self.stores.store.content_revision(),cx)?;}
        if self.current_reserved_cache().is_none(){return Ok(false)}
        let (_,snapshot,config,history)=self.history.cache.as_ref().unwrap();let roots=(snapshot,config,history);
        let revision=self.stores.store.content_revision();
        if !semio_plugin_instance_jobs::reserved_preparation::ReservedPreparation::<A>::admit(&mut self.pending.reserved_preparation,&mut self.pending.reserved_birth,revision,cx)?{return Ok(false)}
        let proof=self.tools.qualified_tool_proof(&*self.stores,action)?;
        let schema=proof.schema_parts();
        let owner=self.pending.reserved_preparation.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("reserved preparation disappeared"))?;
        owner.validate(revision,cx)?;
        if !owner.bind_meta(&meta.actor,meta.instance_id,meta.view_state.as_deref().map(|view|(view.locale,view.terminology)),cx)?{return Ok(false)}
        if !owner.bind_cache(roots,cx)?{return Ok(false)}
        if !owner.copy_metadata(0,[self.tools.tool_controller_id(),""],cx)?{return Ok(false)}
        if !owner.copy_metadata(1,[action,""],cx)?{return Ok(false)}
        if !owner.copy_metadata(2,[schema.0,schema.1],cx)?{return Ok(false)}
        if !owner.copy_metadata(3,[self.stores.store.envelope().id.as_str(),""],cx)?{return Ok(false)}
        if !owner.copy_metadata(5,[self.tools.tool_controller_id(),""],cx)?{return Ok(false)}
        if !owner.copy_metadata(6,[action,""],cx)?{return Ok(false)}
        if !owner.copy_metadata(7,[schema.0,schema.1],cx)?{return Ok(false)}
        if !owner.copy_metadata(8,[action,""],cx)?{return Ok(false)}
        for field in [0,2,3,5,6,7,8]{if !owner.stage_header(field,cx)?{return Ok(false)}}
        if !self.stores.advance_authoring_seed(&meta.actor,&mut owner.seed,&mut owner.seed_output,cx)?{return Ok(false)}
        owner.completion_birth.advance(&mut owner.completion,cx)
    }
    /// 📜️ Pages the actual action and argument tree into the canonical wire while retaining its original source.
    pub fn prepare_reserved_action_wire(&mut self,action:&str,args:&mut Option<DslValue>,meta:&ActionMeta,maximum:usize,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if !semio_plugin_instance_jobs::reserved_preparation::ReservedPreparation::<A>::admit(&mut self.pending.reserved_preparation,&mut self.pending.reserved_birth,self.stores.store.content_revision(),cx)?{return Ok(false)}
        if !self.pending.reserved_preparation.as_mut().unwrap().bind_meta(&meta.actor,meta.instance_id,meta.view_state.as_deref().map(|view|(view.locale,view.terminology)),cx)?{return Ok(false)}
        if !self.prepare_reserved_interaction_read(cx)?{return Ok(false)}
        if !self.prepare_reserved_header(action,meta,cx)?{return Ok(false)}
        let owner=self.pending.reserved_preparation.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("reserved preparation disappeared"))?;
        if !owner.begin_action(args,cx)?{return Ok(false)}
        owner.advance_action_wire(maximum,cx)
    }
    /// 🖱️ Captures the original interaction read before paging or encoding the action.
    pub fn prepare_reserved_interaction_read(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let owner=self.pending.reserved_preparation.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("reserved interaction requires its preparation"))?;
        owner.validate(self.stores.store.content_revision(),cx)?;if owner.interaction_read.is_some()||owner.input.is_some(){return Ok(true)}
        let demand=semio_plugin_decl_app::media_contract::media_original_read_capture_demand::<protocol::InteractionState>();let grant=cx.retained_grant();
        if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok(false)}
        *owner.interaction_read=Some(self.stores.interaction_store.snapshot_read_original()?);cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }
    /// 🎞️ Retains the import verb alongside the other prepared metadata before media donation.
    pub fn prepare_reserved_media_header(&mut self,port:&str,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if !self.prepare_reserved_header("import-media",meta,cx)?{return Ok(false)}
        let owner=self.pending.reserved_preparation.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("reserved preparation disappeared"))?;
        if !owner.stage_header(1,cx)?{return Ok(false)}
        if !owner.copy_metadata(4,[port,""],cx)?{return Ok(false)}
        owner.stage_header(4,cx)
    }
    /// 🎁️ Moves the supplied media and its paged port into the original input receiver.
    pub fn prepare_reserved_media_input(&mut self,media:&mut Option<Media>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let owner=self.pending.reserved_preparation.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("reserved media requires its preparation"))?;
        owner.validate(self.stores.store.content_revision(),cx)?;if owner.input.is_some(){return if media.is_none(){Ok(true)}else{Err(semio_framework_value::ValueError::invariant("reserved media source was replaced after donation").into())}}
        if !owner.header_ready(4){return Ok(false)}
        if !Self::media_donation_funded(cx){return Ok(false)}
        if media.is_none(){return Err(semio_framework_value::ValueError::invariant("reserved media source is absent").into())}
        if owner.raw_wire.is_some(){return Err(semio_framework_value::ValueError::invariant("reserved media wire receiver is occupied").into())}
        let bytes=Self::MEDIA_DONATION_BYTES;
        let headers=owner.headers.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("reserved media port was already donated"))?;
        let port=std::mem::take(&mut headers[4]);*owner.input=Some(ArtifactReservedToolInput::Media{port,media:media.take().unwrap()});*owner.raw_wire=Some(Vec::new());
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }
    /// 📸️ Acquires the actual store read lease and retains it until request handoff or funded close.
    pub fn prepare_reserved_snapshot_read(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if let Some(owner)=self.pending.reserved_preparation.as_ref(){owner.validate(self.stores.store.content_revision(),cx)?;}
        if self.current_reserved_cache().is_none(){return Ok(false)}
        let (_,snapshot,config,history)=self.history.cache.as_ref().unwrap();let roots=(snapshot,config,history);
        let revision=self.stores.store.content_revision();
        let owner=self.pending.reserved_preparation.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("reserved snapshot requires its preparation"))?;
        owner.validate(revision,cx)?;
        if !owner.bind_cache(roots,cx)?{return Ok(false)}
        if owner.snapshot_read.is_some(){return Ok(true)}
        let demand=semio_plugin_decl_app::media_contract::media_original_read_capture_demand::<A::Snapshot>();let grant=cx.retained_grant();
        if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<demand.copy_bytes||grant.maximum_depth<demand.depth{return Ok(false)}
        *owner.snapshot_read=Some(self.stores.store.snapshot_read_original()?);
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,..Default::default()})?;cx.consume_fuel(1);
        let read=owner.snapshot_read.as_ref().unwrap();let snapshot=owner.snapshot_root().unwrap();
        if !semio_plugin_decl_app::media_contract::media_original_read_matches(read,snapshot,self.stores.store.generation(),revision){return Err(semio_framework_value::ValueError::invariant("reserved read differs from its cached snapshot authority").into())}        Ok(true)
    }
    /// 🖱️ Donates the returned original arguments with the actual interaction read authority.
    pub fn prepare_reserved_action_input(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let revision=self.stores.store.content_revision();
        let owner=self.pending.reserved_preparation.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("reserved action input requires its preparation"))?;
        owner.validate(revision,cx)?;if owner.input.is_some(){return Ok(true)}
        if owner.raw_wire.is_none()||owner.action_returned.is_none()||owner.interaction_read.is_none(){return Ok(false)}
        let bytes=std::mem::size_of::<ArtifactReservedToolInput>();
        let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
        let interaction=owner.interaction_read.take().unwrap();
        let args=owner.action_returned.as_mut().unwrap().1.take();
        *owner.input=Some(ArtifactReservedToolInput::Action{args,interaction});
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }
    /// 🚪️ Admits the paged wire against the actual registered bus without cloning its key or schema.
    pub fn prepare_reserved_wire_admission(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let owner=self.pending.reserved_preparation.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("reserved wire requires its preparation"))?;
        owner.validate(self.stores.store.content_revision(),cx)?;let Some(raw)=owner.raw_wire.as_ref()else{return Ok(false)};let length=raw.len();
        if owner.admission.is_some(){
            if !owner.wire_construction.terminal_is_empty(){owner.wire_construction.reset_step(cx)?;return Ok(false)}
            let Some(cursor)=owner.wire_cursor.as_mut()else{return Ok(true)};
            if !cursor.terminal_is_empty(){cx.with_original_retirement(|cx|cursor.close_step(&*self.tools.tool_jobs,cx))?;return Ok(false)}
            let bytes=std::mem::size_of::<Option<semio_framework::action_bus::OriginalWireAdmission>>();cx.with_original_retirement(|cx|semio_framework_job::close_original_retained_step(cx,semio_framework_value::RetirementDemand{copy_bytes:bytes,depth:1,..Default::default()},|_|{owner.wire_cursor.take();semio_framework_value::RetainedWorkOutcome{result:Ok::<_,semio_framework_job::OriginalTurnPrimary>(semio_framework_value::RetainedCloneStep::Progress(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})),work:1}}))?;return Ok(false)        }
        if owner.wire_cursor.is_some(){let advanced=owner.wire_cursor.as_mut().unwrap().advance(&*self.tools.tool_jobs,cx,&mut owner.wire_construction,&mut owner.admission)?;if advanced&&owner.admission.is_none(){owner.wire_construction.reset_step(cx)?;}return Ok(false)}
        if owner.wire_key.is_none()&&owner.wire_schema.is_none(){
            if ![5,6,7].into_iter().all(|field|owner.header_ready(field)){return Ok(false)}let bytes=std::mem::size_of::<semio_framework::ToolFactoryKey>()+std::mem::size_of::<String>();let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
            let headers=owner.headers.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("reserved wire metadata was donated"))?;*owner.wire_key=Some(semio_framework::ToolFactoryKey::new(std::mem::take(&mut headers[5]),std::mem::take(&mut headers[6])));*owner.wire_schema=Some(std::mem::take(&mut headers[7]));cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);return Ok(false)
        }
        match semio_framework::action_bus::OriginalWireAdmission::admit(&mut owner.wire_key,&mut owner.wire_schema,length,cx,&mut owner.wire_construction,&mut owner.wire_cursor)?{semio_framework_job::OriginalConstructionStatus::Failed=>return Err(semio_framework_value::ValueError::invariant("reserved wire admission construction failed; its causes remain in the preparation").into()),semio_framework_job::OriginalConstructionStatus::Ready=>{owner.wire_construction.reset_step(cx)?;}semio_framework_job::OriginalConstructionStatus::Pending=>{}}Ok(false)
    }
    /// 🎞️ Advances the original media request from its retained source through the concrete producer builder.
    pub fn advance_reserved_media(&mut self,port:&str,media:&mut Option<Media>,meta:&ActionMeta,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if let Some(owner)=self.pending.reserved_preparation.as_mut(){owner.validate(self.stores.store.content_revision(),cx)?;if owner.job.is_some(){return Ok(true)}if owner.request.is_some()||owner.body.is_some(){return owner.advance_builder(cx)}}
        if !self.prepare_reserved_media_header(port,meta,cx)?{return Ok(false)}
        if !self.prepare_reserved_media_input(media,cx)?{return Ok(false)}
        if !self.prepare_reserved_snapshot_read(cx)?{return Ok(false)}
        if !self.prepare_reserved_wire_admission(cx)?{return Ok(false)}
        let mut no_external_admission=None;if !self.assemble_reserved_request("import-media",meta,&mut no_external_admission,1,cx)?{return Ok(false)}
        Ok(false)
    }
    /// 🎟️ Donates the complete staged request only after its exact registered wire admission.
    pub fn assemble_reserved_request(&mut self,action:&str,meta:&ActionMeta,admission:&mut Option<semio_framework::ToolWireAdmission>,decoded_items:usize,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if let Some(owner)=self.pending.reserved_preparation.as_ref(){owner.validate(self.stores.store.content_revision(),cx)?;}
        if self.current_reserved_cache().is_none(){return Ok(false)}
        let (_,snapshot,config,history)=self.history.cache.as_ref().unwrap();let roots=(snapshot,config,history);
        let proof=self.tools.qualified_tool_proof(&*self.stores,action)?;let contract=proof.contract();
        let owner=self.pending.reserved_preparation.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("reserved request requires its preparation"))?;
        owner.validate(self.stores.store.content_revision(),cx)?;if !owner.bind_meta(&meta.actor,meta.instance_id,meta.view_state.as_deref().map(|view|(view.locale,view.terminology)),cx)?{return Ok(false)}if owner.request.is_some(){return Ok(true)}if !owner.bind_cache(roots,cx)?{return Ok(false)}
        let registration=match proof{semio_plugin_job::reserved_jobs::QualifiedToolView::FrameworkOwned(row)|semio_plugin_job::reserved_jobs::QualifiedToolView::AppOwned(row)=>row,_=>return Err(semio_framework_value::ValueError::invariant("reserved request requires an owned factory").into())};
        let registration_id=registration as *const _ as usize;
        if owner.admission.is_none(){
            let Some(source)=admission.as_ref()else{return Ok(false)};
            if registration.owner!=semio_plugin_decl_app::tool_contract::ToolOwnerWitness::of::<A>()||source.factory_type_id!=registration.factory_type_id||source.contract!=registration.contract||decoded_items>contract.max_decoded_items{return Err(semio_framework_value::ValueError::invariant("reserved request requires its exact admitted factory and decoded bound").into())}
            let bytes=std::mem::size_of::<semio_framework::ToolWireAdmission>()+std::mem::size_of::<usize>()*2;let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
            *owner.admission=admission.take();owner.admission_registration=Some(registration_id);owner.admission_decoded=decoded_items;cx.consume_fuel(1);cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;return Ok(false)
        }
        if owner.admission_registration.is_none(){let source=owner.admission.as_ref().unwrap();if registration.owner!=semio_plugin_decl_app::tool_contract::ToolOwnerWitness::of::<A>()||source.factory_type_id!=registration.factory_type_id||source.contract!=registration.contract||decoded_items>contract.max_decoded_items{return Err(semio_framework_value::ValueError::invariant("reserved request requires its exact admitted factory and decoded bound").into())}let bytes=std::mem::size_of::<usize>()*2;let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}owner.admission_registration=Some(registration_id);owner.admission_decoded=decoded_items;cx.consume_fuel(1);cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;return Ok(false)}
        if owner.admission_registration!=Some(registration_id)||owner.admission_decoded!=decoded_items{return Err(semio_framework_value::ValueError::invariant("reserved admission requires its original registration and decoded bound").into())}
        if owner.admission_cursor.0<4{
            let admitted=owner.admission.as_ref().unwrap();let (left,right)=match owner.admission_cursor.0{0=>(admitted.key.controller_id.as_str(),registration.key.controller_id.as_str()),1=>(admitted.key.tool_id.as_str(),registration.key.tool_id.as_str()),2=>(admitted.schema_id.as_str(),registration.schema_id.as_str()),_=>(admitted.factory_type_name,registration.factory_type_name)};
            if left.len()!=right.len(){return Err(semio_framework_value::ValueError::invariant("reserved admission metadata differs from its registered factory").into())}
            let grant=cx.retained_grant();if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_depth==0{return Ok(false)}let offset=owner.admission_cursor.1;let count=(left.len()-offset).min(4096).min(grant.maximum_copy_bytes).min(cx.fuel_remaining() as usize);if count==0&&offset<left.len(){return Ok(false)}
            if left.as_bytes()[offset..offset+count]!=right.as_bytes()[offset..offset+count]{return Err(semio_framework_value::ValueError::invariant("reserved admission metadata differs from its registered factory").into())}
            owner.admission_cursor.1+=count;if owner.admission_cursor.1==left.len(){owner.admission_cursor.0+=1;owner.admission_cursor.1=0;}cx.consume_fuel(count.max(1) as u64);cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:count,..Default::default()})?;return Ok(false)
        }
        let Some(raw)=owner.raw_wire.as_ref()else{return Ok(false)};if raw.len()>contract.max_raw_wire_bytes{return Err(semio_framework_value::ValueError::invariant("reserved request exceeds its admitted wire bound").into())}
        if owner.input.is_none()||owner.snapshot_read.is_none()||owner.cache_roots.is_none()||owner.seed_output.is_none()||owner.completion.is_none()||owner.headers.is_none()||![0,1,2,3].into_iter().all(|field|owner.header_ready(field)){return Ok(false)}
        let media=matches!(owner.input.as_ref(),Some(ArtifactReservedToolInput::Media{..}));if !media&&owner.action_returned.as_ref().is_none_or(|(_,args)|args.is_some()){return Ok(false)}
        let bytes=std::mem::size_of::<ArtifactReservedToolJobRequest<A>>()+std::mem::size_of::<ArtifactToolCompletion<A>>()+std::mem::size_of::<ChildContentView>();let grant=cx.retained_grant();
        if cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(false)}
        let headers=owner.headers.as_mut().unwrap();let controller_id=std::mem::take(&mut headers[0]);let media_tool=std::mem::take(&mut headers[1]);let payload_schema_id=std::mem::take(&mut headers[2]);let parent_document_id=std::mem::take(&mut headers[3]);
        let tool_id=if media{media_tool}else{owner.action_returned.take().unwrap().0};
        let (snapshot,config,history)=owner.take_cache_roots().unwrap();
        *owner.request=Some(ArtifactReservedToolJobRequest{source_binding:owner.source,retained:grant,controller_id,tool_id,payload_schema_id,contract,app_instance_id:meta.instance_id,ui_axes:meta.view_state.as_deref().map(|view|(view.locale,view.terminology)),parent_document_id,canonical_base_revision:owner.revision,authoring_seed:owner.seed_output.take().unwrap(),snapshot,snapshot_read:owner.snapshot_read.take().unwrap(),config,history,children:ChildContentView::clone(&self.child_composition.child_content_root),raw_wire:owner.raw_wire.take().unwrap(),input:owner.input.take(),completion:owner.completion.as_ref().unwrap().clone()});
        cx.consume_retained(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;cx.consume_fuel(1);Ok(true)
    }
    /// 🧾️ Drives the clipboard action through the stepped reserved preparation until its concrete job is built.
    pub fn advance_reserved_action(&mut self,action:&str,args:Option<&DslValue>,decoded_items:usize,meta:&ActionMeta,maximum:usize,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        if let Some(owner)=self.pending.reserved_preparation.as_mut(){owner.validate(self.stores.store.content_revision(),cx)?;if owner.job.is_some(){return Ok(true)}if owner.request.is_some()||owner.body.is_some(){return owner.advance_builder(cx)}}
        let started=self.pending.reserved_preparation.as_ref().is_some_and(|owner|owner.action_wire.is_some()||owner.action_returned.is_some());
        let mut source=if started{None}else{args.cloned()};
        if !self.prepare_reserved_action_wire(action,&mut source,meta,maximum,cx)?{return Ok(false)}
        if !self.prepare_reserved_action_input(cx)?{return Ok(false)}
        if !self.prepare_reserved_snapshot_read(cx)?{return Ok(false)}
        if !self.prepare_reserved_wire_admission(cx)?{return Ok(false)}
        let mut no_external_admission=None;self.assemble_reserved_request(action,meta,&mut no_external_admission,decoded_items,cx)?;Ok(false)
    }
    /// 🤝️ Moves the built producer, its completion and its wire admission out of the preparation in one paid step.
    pub fn handoff_reserved_producer(&mut self,output:&mut Option<(ArtifactReservedToolJob,ArtifactToolCompletion<A>,semio_framework::ToolWireAdmission)>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        let owner=self.pending.reserved_preparation.as_mut().ok_or_else(||semio_framework_value::ValueError::invariant("reserved handoff requires its preparation"))?;
        owner.handoff(output,cx)    }
    const MEDIA_DONATION_BYTES:usize=std::mem::size_of::<ArtifactReservedToolInput>()+std::mem::size_of::<Option<Media>>()+std::mem::size_of::<Vec<u8>>();
    fn media_donation_funded(cx:&semio_framework_job::StepContext<'_>)->bool{let grant=cx.retained_grant();!(cx.should_yield()||cx.is_cancelled()||grant.maximum_items==0||grant.maximum_copy_bytes<Self::MEDIA_DONATION_BYTES||grant.maximum_depth==0)}
    /// 🎞️ Whether the staged reserved media preparation awaits its media source and this step funds the donation, so a decoded source is never left unowned.
    pub fn reserved_media_due(&self,cx:&semio_framework_job::StepContext<'_>)->bool{Self::media_donation_funded(cx)&&self.pending.reserved_preparation.as_ref().is_some_and(|owner|owner.header_ready(4)&&owner.input.is_none()&&owner.raw_wire.is_none())}
}
