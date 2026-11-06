//! 📝️ The render owner admits and retains original staged requests beside its source work.
use crate::render_bundle::RenderCtx;
use semio_plugin_decl_app::ArtifactApp;
use semio_plugin_instance_jobs::jobs_instance_root::OriginalRenderTreeIdentity;
use semio_os_kernel::{ReferenceSpaceMember,MemberFactory};
use semio_framework::ViewModel;

impl<'ctx,A:ArtifactApp,M:ReferenceSpaceMember+MemberFactory+Send+'static,X> RenderCtx<'ctx,A,M,X>{
    /// 🧷️ Only the initiating caller may retire its retained render request.
    pub fn render_request_owned(&mut self,cx:&semio_framework_job::StepContext<'_>)->bool{self.pending.pending_render_tree_identity.as_mut().is_some_and(|identity|identity.validate(cx).is_ok())}
    /// 📨️ Bind a real context to the original request before source querying begins.
    pub fn receive_render_request(&mut self,body_key:&str,request:&mut Option<semio_framework::action_staging::StagedFormRequest>,view:&ViewModel,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<()>,semio_framework_job::OriginalTurnFailure>{
        let invalid=||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"staging.render-request-context");
        let value=request.as_ref().ok_or_else(invalid)?;
        if !A::uses_render_work(body_key)||!value.context_matches(view.window_id.as_deref(),view.locale){return Err(invalid().into());}
        if self.pending.receive_render_failure(cx)?||cx.should_yield(){return Ok(None);}
        if self.pending.pending_render_tree.is_some()||self.pending.pending_render_tree_close.is_some()||self.pending.pending_render_source.is_some(){return Err(invalid().into());}
        if self.pending.pending_render_work.is_none()&&self.pending.pending_render_tree_ready{self.pending.pending_render_tree_identity=None;self.pending.pending_render_tree_ready=false;}
        if let Some(identity)=self.pending.pending_render_tree_identity.as_mut(){identity.validate(cx)?;}else{self.pending.pending_render_tree_identity=Some(OriginalRenderTreeIdentity::capture(cx)?);}
        if self.pending.pending_render_work.is_none(){
            if !<A::RenderWork as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported()||!<A::Snapshot as semio_framework_value::retirement::RetireOwned>::controlled_retirement_supported(){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"staged render requires controlled original source retirement").into());}
            let Some(work)=A::begin_render_work(body_key,cx)?else{return Ok(None)};
            *self.pending.pending_render_work=Some(semio_plugin_instance_jobs::jobs_render::work::OriginalRenderWork::retain(work));
            return Ok(None);
        }
        let identity=self.pending.pending_render_tree_identity.as_mut().ok_or_else(invalid)?;
        let work=self.pending.pending_render_work.as_mut().ok_or_else(invalid)?.original(identity,cx)?.ok_or_else(invalid)?;
        let result=A::receive_render_work_request(work,request,view,cx);
        match result{Ok(Some(()))if request.is_some()=>Err(invalid().into()),Ok(None)if request.is_none()=>Err(invalid().into()),other=>other}
    }
}
