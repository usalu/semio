//! 🧳️ Source-pinned original render queries shared by IFC editor surfaces.
use super::*;
use semio_framework_job::{OriginalTurnFailure, StepContext};
use semio_framework_value::{RetainedCloneProgress, ValueError, ValueRefusalKind};
#[path="guided-work.rs"]
mod guided;
use guided::GuidedWork;

#[derive(semio_framework_value::RetireOwned)]
pub struct Ifc4x3RenderWork {
    binding: Option<RenderBinding>,
    details: crate::details::IfcDetailsRenderWork,
    guided:Option<Box<GuidedWork>>,
    staged:bool,
}
#[derive(Clone, Copy, PartialEq, Eq, semio_framework_value::RetireOwned)]
struct RenderBinding {
    source: usize,
    revision: [u8;32],
}

impl Ifc4x3RenderWork {
    pub fn begin(body: &str,cx: &mut StepContext<'_>) -> Result<Option<Self>,OriginalTurnFailure> {
        if body!=editing::SNAPSHOT_DETAILS_BODY_KEY&&body!=semio_framework_plugin::action_staging::STAGED_FORM_BODY_KEY { return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"IFC render work body is undeclared").into()); }
        if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"IFC original render work cancelled").into());}
        let bytes=std::mem::size_of::<Self>();
        let grant=cx.retained_grant();
        if cx.should_yield()||grant.maximum_items==0||grant.maximum_copy_bytes<bytes||grant.maximum_depth==0{return Ok(None);}
        let work=Self{binding:None,details:Default::default(),guided:None,staged:body==semio_framework_plugin::action_staging::STAGED_FORM_BODY_KEY};
        cx.consume_retained(RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()})?;
        cx.consume_fuel(1);
        Ok(Some(work))
    }

    pub fn receive_request(&mut self,donor:&mut Option<semio_framework_plugin::action_staging::StagedFormRequest>,view:&semio_framework_plugin::ViewModel,cx:&mut StepContext<'_>)->Result<Option<()>,OriginalTurnFailure>{
        let invalid=||ValueError::literal(ValueRefusalKind::InvalidValue,"staging.native-controller");let request=donor.as_ref().ok_or_else(invalid)?;
        if !self.staged||!request.context_matches(view.window_id.as_deref(),view.locale)||!GuidedWork::supports(&request.token.action)||self.binding.is_some(){return Err(invalid().into());}
        GuidedWork::receive(&mut self.guided,donor,cx)
    }
    pub fn demand(&self,maximum:usize)->Result<semio_framework_value::RetirementDemand,ValueError>{
        if self.staged{if let Some(work)=&self.guided{return work.demand();}return Ok(semio_framework_value::RetirementDemand{copy_bytes:std::mem::size_of::<GuidedWork>()+std::mem::size_of::<Box<GuidedWork>>(),capacity_bytes:std::mem::size_of::<GuidedWork>(),depth:1,..Default::default()});}self.details.demands().map(|demand|{let _=maximum;demand})
    }

    pub fn render(&mut self,body: &str,source:&std::sync::Arc<IfcSnapshot>,doc: &ArtifactView<'_,IfcSnapshot>,view: &semio_framework_plugin::ViewModel,cx: &mut StepContext<'_>) -> semio_framework_plugin::OriginalRenderResult<semio_framework_plugin::ComponentTree> {
        if !std::ptr::eq(source.as_ref(),doc.snapshot){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"IFC original render source loan changed").into());}
        if body!=if self.staged{semio_framework_plugin::action_staging::STAGED_FORM_BODY_KEY}else{editing::SNAPSHOT_DETAILS_BODY_KEY}{return Err(ValueError::literal(ValueRefusalKind::InvalidValue,"IFC retained render body changed").into());}
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
        if !self.details.advance::<crate::details::Ifc4x3DetailsProfile>(doc.snapshot,revision.0,cx)?{return Ok(None);}
        let provider=crate::details::Ifc4x3DetailsProvider::new(doc.snapshot).with_contextual_enumerations(&self.details,revision.0)?;
        editing::render_snapshot_details_provider_revisioned(&provider,revision,view.locale,IFC4X3_CONTROLLER,&semio_framework_plugin::TreeWindows::for_body(view,editing::SNAPSHOT_DETAILS_BODY_KEY)).map(semio_framework_plugin::built_to_component_tree).map(Some).map_err(Into::into)
    }
}
