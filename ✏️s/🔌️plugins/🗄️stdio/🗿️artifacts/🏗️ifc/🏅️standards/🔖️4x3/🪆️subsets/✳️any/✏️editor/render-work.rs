//! 🧳️ Source-pinned original render queries shared by IFC editor surfaces.
use super::*;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_job::{OriginalConstructionAdmission};
use semio_framework_value::retained_clone::RetainedCloneClose;
use semio_framework_value::{RetainedAllocationEffect, RetainedCloneProgress, RetainedCloneStep, RetainedWorkOutcome, RetirementDemand, ValueError, ValueRefusalKind};
use crate::details::{owned_step, retire_self, settled};
#[path="guided-work.rs"]
mod guided;
use guided::GuidedWork;

pub struct Ifc4x3RenderWork {
    binding: Option<RenderBinding>,
    details: Option<crate::details::IfcDetailsRenderWork>,
    guided:Option<Box<GuidedWork>>,
    construction:Option<OriginalConstructionAdmission>,
    retirement:RetainedCloneClose,
    staged:bool,
    closing:bool,
}
#[derive(Clone, Copy, PartialEq, Eq, semio_framework_value::RetireOwned)]
struct RenderBinding {
    source: usize,
    revision: [u8;32],
}

impl Ifc4x3RenderWork {
    pub fn begin(body: &str,cx: &mut StepContext<'_>) -> Result<Option<Self>,OriginalTurnFailure> {
        if body!=editing::SNAPSHOT_DETAILS_BODY_KEY&&body!=semio_plugin::action_staging::STAGED_FORM_BODY_KEY { return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"IFC render work body is undeclared").into()); }
        if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"IFC original render work cancelled").into());}
        let bytes=std::mem::size_of::<Self>();
        let grant=cx.retained_grant();
        if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(None);}
        let work=Self{binding:None,details:Some(Default::default()),guided:None,construction:None,retirement:Default::default(),staged:body==semio_plugin::action_staging::STAGED_FORM_BODY_KEY,closing:false};
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
        cx.consume_fuel(1);
        Ok(Some(work))
    }

    pub fn receive_request(&mut self,donor:&mut Option<semio_plugin::action_staging::StagedFormRequest>,view:&semio_plugin::ViewModel,cx:&mut StepContext<'_>)->Result<Option<()>,OriginalTurnFailure>{
        let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,"staging.native-controller");let request=donor.as_ref().ok_or_else(invalid)?;
        if !self.staged||!request.context_matches(view.window_id.as_deref(),view.locale)||!GuidedWork::supports(&request.token.action)||self.binding.is_some(){return Err(invalid().into());}
        GuidedWork::receive(&mut self.guided,&mut self.construction,donor,cx)
    }
    pub fn demand(&self,maximum:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
        if self.staged{if let Some(work)=&self.guided{return work.demand();}return Ok(semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<GuidedWork>()+std::mem::size_of::<Box<GuidedWork>>(),capacity_bytes:std::mem::size_of::<GuidedWork>(),depth:1,..Default::default()});}self.details.as_ref().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"IFC render details are closed"))?.demands().map(|demand|{let _=maximum;demand})
    }

    pub fn render(&mut self,body: &str,source:&std::sync::Arc<IfcSnapshot>,doc: &ArtifactView<'_,IfcSnapshot>,view: &semio_plugin::ViewModel,cx: &mut StepContext<'_>) -> semio_plugin::OriginalRenderResult<semio_plugin::ComponentTree> {
        if !std::ptr::eq(source.as_ref(),doc.snapshot){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IFC original render source loan changed").into());}
        if body!=if self.staged{semio_plugin::action_staging::STAGED_FORM_BODY_KEY}else{editing::SNAPSHOT_DETAILS_BODY_KEY}{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"IFC retained render body changed").into());}
        if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"IFC original render query cancelled").into());}
        let operation=doc.render_operation().ok_or_else(||ValueError::literal(ValueRefusalKind::InvariantViolated,"IFC retained render query requires canonical source provenance"))?;
        let binding=RenderBinding{source:doc.snapshot as *const _ as usize,revision:operation.canonical_base_revision};
        if let Some(original)=self.binding{
            if original!=binding{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IFC retained render source revision or request changed").into());}
        }else{
            let bytes=std::mem::size_of::<RenderBinding>();
            let grant=cx.retained_grant();
            if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(None);}
            self.binding=Some(binding);
            cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
            cx.consume_fuel(1);
            return Ok(None);
        }
        if self.staged{return self.guided.as_deref_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"staging.original-donor-required"))?.render(source,doc,binding.revision,view,cx);}
        let revision=semio_s_artifact_stdio_contract::window_kit_artifact_publication_revision(doc)?;
        let details=self.details.as_mut().ok_or_else(||ValueError::literal(ValueRefusalKind::InvalidValue,"IFC render details are closed"))?;
        if !details.advance::<crate::details::Ifc4x3DetailsProfile>(doc.snapshot,revision.0,cx)?{return Ok(None);}
        let provider=crate::details::Ifc4x3DetailsProvider::new(doc.snapshot).with_contextual_enumerations(details,revision.0)?;
        editing::render_snapshot_details_provider_revisioned(&provider,revision,view.locale,IFC4X3_CONTROLLER,&semio_plugin::TreeWindows::for_body(view,editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_plugin::built_to_component_tree).map(Some).map_err(Into::into)
    }
}
impl Ifc4x3RenderWork {
    pub fn begin_close(&mut self) { self.closing = true; }
    pub fn empty(&self) -> bool { self.details.is_none() && self.guided.is_none() && self.construction.is_none() && self.retirement.is_empty() }
    pub fn close_demands(&self, body: usize) -> Result<RetirementDemand, ValueError> {
        if !self.retirement.is_empty() { return self.retirement.next_owner_demands::<crate::details::IfcDetailsRenderWork>(false, &None, body); }
        if self.guided.is_some() { return self.retirement.next_owner_demands::<Box<GuidedWork>>(true, &None, body); }
        if self.details.is_some() { return self.retirement.next_owner_demands::<crate::details::IfcDetailsRenderWork>(true, &None, body); }
        if self.construction.is_some() { return self.retirement.next_owner_demands::<OriginalConstructionAdmission>(true, &None, body); }
        Ok(Default::default())
    }
    pub fn close_owned(&mut self, body: usize, effect: &mut RetainedAllocationEffect<'_>) -> RetainedWorkOutcome {
        if !self.retirement.is_empty() { return settled(self.retirement.step_granted(body, effect)); }
        if self.guided.is_some() { return owned_step(self.retirement.begin_granted(&mut self.guided, effect), "IFC guided render work omitted its owner"); }
        if self.details.is_some() { return owned_step(self.retirement.begin_granted(&mut self.details, effect), "IFC render details omitted their owner"); }
        if self.construction.is_some() { return owned_step(self.retirement.begin_granted(&mut self.construction, effect), "IFC render construction omitted its owner"); }
        RetainedWorkOutcome { result: Ok(RetainedCloneStep::Complete(Default::default())), work: 0 }
    }
}
retire_self!([] Ifc4x3RenderWork);
