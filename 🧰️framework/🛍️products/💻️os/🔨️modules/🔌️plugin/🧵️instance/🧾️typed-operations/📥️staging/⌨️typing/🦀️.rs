//! 🏉️ Staged `typing` members of the `instance-typed-ops` use cases, as methods of the borrowed bundle (WP7 of the instance family).
//!
//! ⌨️ Tool dispatch and typing commits: continuous-control arguments, typing runs and their publication.
use semio_framework_plugin_decl_app::ArtifactApp;
use semio_framework_plugin_decl_emission::ActionMeta;
use semio_framework_value::DslValue;
use semio_framework::{ActionKind,Fault,FaultCode,FaultOrigin};
use semio_framework_plugin_tool_run::tool_jobs_tool_machine::{ScrubTag,ToolTag};
use semio_framework_plugin_instance_extension::InstanceExtension;
use semio_framework_tool_machine::{ScrubPhase,ToolAbortReason,SCRUB_ABORT_ARG,SCRUB_GESTURE_ARG,TYPING_BUFFER_ARG,TYPING_COMMIT_ARG};
use store::{MemberFactory,ReferenceSpaceMember};
use crate::typed_bundle::TypedCtx;

//#region 🔖️Driver
impl<'ctx, A: ArtifactApp, M: ReferenceSpaceMember + MemberFactory + 'static, X: InstanceExtension> TypedCtx<'ctx, A, M, X> {
    /// 🚦️ Actual tag capture and bounded roster retirement settle before the declared command producer consumes its arguments.
    pub async fn prepare_original_tool_dispatch(&mut self,action:&str,args:Option<&DslValue>,metadata:&mut Option<ActionMeta>,typing:&mut crate::original_typing::OriginalTypingDispatch<A>,typing_ready:&mut bool,tag:&mut Option<ToolTag>,ready:&mut bool,decision:&mut Option<bool>,cx:&mut semio_framework_job::StepContext<'_>)->Result<Option<bool>,semio_framework_job::OriginalTurnFailure>{
        if !*typing_ready{
            if !typing.complete(){if !self.advance_original_typing_dispatch(action,args,metadata,typing,cx).await?{return Ok(None)}}
            typing.finish_into(typing_ready,decision,metadata,cx)?;return Ok(None)
        }
        let meta=metadata.as_ref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original dispatch metadata was not restored"))?;
        type Host<A>=semio_framework_plugin_scrub_host::OriginalScrubDispatch<semio_framework_plugin_tool_run::tool_jobs_tool_machine::PressLeaf<<A as ArtifactApp>::Mutation,<A as ArtifactApp>::ConfigMutation>>;
        if let Some(owner)=self.tools.original_scrub_dispatch.as_ref(){
            if owner.terminal_is_empty(){
                if owner.frozen(){
                    const CODE:&str="timeTravel.frozen";const MESSAGE:&str="a history edit freezes this original document press";
                    let mut d=Host::<A>::shell_close_demands();d.copy_bytes+=CODE.len()+MESSAGE.len()+2*std::mem::size_of::<Fault>()+std::mem::size_of::<u64>()+std::mem::size_of::<bool>();d.capacity_bytes=CODE.len()+MESSAGE.len()+std::mem::size_of::<semio_framework::FaultScope>();
                    let mut fault=None;
                    semio_framework_job::admit_original_construction(cx,&mut fault,semio_framework_job::OriginalConstructionDemand{items:1,copy_bytes:d.copy_bytes,capacity_bytes:d.capacity_bytes,release_bytes:d.release_bytes,depth:d.depth},|_|{self.tools.original_scrub_dispatch.take();self.as_render().follow_tool_machines(true);Ok((Fault::new(FaultOrigin::Framework,FaultCode::new(CODE),MESSAGE),semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:d.copy_bytes,retained_capacity_bytes:d.capacity_bytes,released_bytes:d.release_bytes}))})?;
                    return if let Some(fault)=fault{Err(fault.into())}else{Ok(None)}
                }
                let mut demand=Host::<A>::shell_close_demands();demand.copy_bytes+=std::mem::size_of::<u64>()+std::mem::size_of::<bool>();let mut received=false;
                semio_framework_job::close_original_retained_step(cx,demand,|_|{self.tools.original_scrub_dispatch.take();self.as_render().follow_tool_machines(true);received=true;Ok(semio_framework_value::RetainedCloneStep::Complete(semio_framework_value::RetainedCloneProgress{copied_items:1,copied_bytes:demand.copy_bytes,released_bytes:demand.release_bytes,..Default::default()}))})?;
                return Ok(if received{*decision}else{None})
            }
        }else if decision.is_some(){return Ok(*decision)}
        if !self.capture_original_tool_tag(action,args,meta,cx,tag,ready).await?{return Ok(None)}
        if self.tools.original_scrub_dispatch.is_none(){
            let frozen=X::freezes_document(&*self.time_travel,&*self.tool_runs)&&matches!(tag,Some(ToolTag::Scrub(ScrubTag{phase:ScrubPhase::Tick{..}|ScrubPhase::Commit{..},..})));
            Host::<A>::admit(&self.tool_machines.presses,meta.view_state.as_ref(),frozen,&mut self.tools.original_scrub_dispatch,cx)?;return Ok(None)
        }
        self.tools.original_scrub_dispatch.as_mut().unwrap().advance(&mut self.tool_machines.presses,meta.view_state.as_ref(),tag,decision,cx)?;
        Ok(None)
    }

    /// 🧺️ The actual original source settles each typing transaction before the addressed verb resumes.
    pub async fn advance_original_typing_before(&mut self,action:&str,args:Option<&DslValue>,metadata:&mut Option<ActionMeta>,typing:&mut crate::original_typing::OriginalTypingDispatch<A>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{self.advance_original_typing_pass(action,args,metadata,typing,false,cx).await}
    pub async fn advance_original_typing_dispatch(&mut self,action:&str,args:Option<&DslValue>,metadata:&mut Option<ActionMeta>,typing:&mut crate::original_typing::OriginalTypingDispatch<A>,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{self.advance_original_typing_pass(action,args,metadata,typing,true,cx).await}
    async fn advance_original_typing_pass(&mut self,action:&str,args:Option<&DslValue>,metadata:&mut Option<ActionMeta>,typing:&mut crate::original_typing::OriginalTypingDispatch<A>,control:bool,cx:&mut semio_framework_job::StepContext<'_>)->Result<bool,semio_framework_job::OriginalTurnFailure>{
        use crate::original_typing::OriginalTypingCommit;
        use semio_framework_tool_machine::OriginalToolLedgerSource;
        use semio_framework_job::{admit_original_construction,OriginalConstructionDemand};
        use semio_framework_value::{ValueError,ValueRefusalKind,RetainedCloneProgress};
        let invalid=||ValueError::literal(ValueRefusalKind::InvariantViolated,"original typing dispatch lost its retained source");
        if !typing.pending(){
            let buffer=args.and_then(|args|args.get(TYPING_BUFFER_ARG)).and_then(DslValue::as_str).filter(|s|!s.is_empty());
            let commit=args.and_then(|args|args.get(TYPING_COMMIT_ARG)).and_then(DslValue::as_str);
            let view=self.tools.registry.get(action).is_some_and(|definition|definition.kind==ActionKind::View);
            let(suppress,all)=crate::original_typing::classify_typing_dispatch(buffer.is_some(),commit,view);
            let typing_control=buffer.is_some()&&(commit.is_none()||suppress);
            let gesture=args.and_then(|args|args.get(SCRUB_GESTURE_ARG)).and_then(DslValue::as_str).is_some_and(|value|!value.is_empty());
            let scrub_control=gesture&&args.and_then(|args|args.get(SCRUB_ABORT_ARG)).and_then(DslValue::as_str).is_none_or(|reason|ToolAbortReason::parse(reason).is_some());
            typing.admit_policy(&self.tool_machines.typing,control&&suppress,!control&&all,control,control&&(typing_control||scrub_control),metadata,cx)?;return Ok(false)
        }
        typing.validate_ledger(&self.tool_machines.typing,cx)?;if typing.control!=control{return Err(invalid().into())}
        if metadata.is_some(){return Err(invalid().into())}
        if cx.is_cancelled(){return Err(ValueError::literal(ValueRefusalKind::Canceled,"original typing dispatch was cancelled").into())}
        match typing.phase{
            11=>{if typing.control||self.tool_machines.typing.is_empty(){typing.skip_unused_clock(&self.tool_machines.typing,cx)?;}else{typing.capture_clock(||self.tool_machines.clock().physical_ms,cx)?;}},
            10=>{typing.admit_parent(cx)?;},
            1=>{
                if !typing.retire_previous(cx)?{return Ok(false)}
                if self.tool_machines.typing.is_empty(){typing.complete_empty_ledger(&self.tool_machines.typing,cx)?;return Ok(typing.complete())}
                if let Some((window,key))=self.tool_machines.typing.next_retained_window(typing.after.as_deref()){typing.capture_window(window,key,cx)?;}else{typing.transition(9,cx)?;}
            },
            2=>{
                let meta=typing.original_metadata.as_ref().ok_or_else(invalid)?;let window=typing.after.as_ref().ok_or_else(invalid)?.as_str();let key=typing.key.ok_or_else(invalid)?;
                if !self.tool_machines.typing.original_live(window,key)?{typing.transition(1,cx)?;return Ok(false)}
                let state=self.tool_machines.typing.open(window).ok_or_else(invalid)?;
                let explicit=typing.suppress&&meta.view_state.as_ref().and_then(|view|view.window_id.as_deref()).unwrap_or("")==window;
                if crate::original_typing::original_typing_commit_required(typing.all,explicit,state.deadline_ms,typing.now,!typing.control,None)==Some(true){typing.transition(3,cx)?;return Ok(false)}
                if typing.retire_windows&&!window.is_empty()&&meta.view_state.as_ref().is_some_and(|view|!view.window_instances.is_empty()){
                    match typing.roster_step(cx)?{Some(false)=>{typing.transition(3,cx)?;},Some(true)=>{},None=>{}}
                }else{typing.transition(1,cx)?;}
            },
            3=>{
                if typing.transaction.is_some(){typing.transition(4,cx)?;return Ok(false)}
                if let Some(row)=typing.row.as_mut(){if row.ready(){if row.take_into(&self.tool_machines.typing,&mut typing.transaction,&mut typing.values,cx)?{typing.transition(4,cx)?;}}else{row.advance(&mut self.tool_machines.typing,cx)?;}}
                else{OriginalTypingCommit::admit(&self.tool_machines.typing,typing.after.as_ref().ok_or_else(invalid)?,typing.key.ok_or_else(invalid)?,cx,&mut typing.row)?;}
                if typing.transaction.is_some(){typing.transition(4,cx)?;}
            },
            4=>{
                if !typing.close_row(cx)?{return Ok(false)}
                if typing.metadata.is_some(){if typing.close_copy(cx)?{typing.transition(5,cx)?;}return Ok(false)}
                if typing.copy.is_none(){semio_framework_plugin_decl_emission::OriginalActionMetaCopy::admit(cx,&mut typing.copy)?;return Ok(false)}
                let copy=typing.copy.as_mut().unwrap();if copy.advance(&mut typing.original_metadata,cx)?{copy.take_into(&mut typing.metadata,cx)?;}
            },
            5=>{if typing.prepare_verb(cx)?{typing.make_input(cx)?;}},
            6=>{
                if typing.receipt.is_some(){typing.transition(7,cx)?;return Ok(false)}
                self.as_dispatch().dispatch_original_emission(&mut typing.input,&mut typing.receipt,cx).await?;
            },
            7=>{
                let operation=typing.receipt.as_ref().ok_or_else(invalid)?.output.get("operationId").and_then(DslValue::as_str).and_then(|value|value.parse::<u64>().ok()).ok_or_else(invalid)?;
                if let Some(mounted)=self.tools.tool_operations.get(operation){if mounted.terminal_fault.is_some(){return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original typing transaction publication refused").into())}if mounted.published_artifact&&!typing.published{typing.observe_publication(cx)?;return Ok(false)}self.advance_typed_operation_publication_one(cx).await.map_err(semio_framework_job::OriginalTurnFailure::from)?;return Ok(false)}
                if typing.expect_artifact&&!typing.published{return Err(ValueError::literal(ValueRefusalKind::InvariantViolated,"original typing transaction retired without observed Store publication").into())}typing.transition(8,cx)?;
            },
            8=>{
                if !typing.close_receipt(cx)?{return Ok(false)}
                let bytes=std::mem::size_of::<u8>()+std::mem::size_of::<u64>()+std::mem::size_of::<bool>();let mut done=None;
                admit_original_construction(cx,&mut done,OriginalConstructionDemand{items:1,copy_bytes:bytes,capacity_bytes:0,release_bytes:0,depth:1},|_|{self.as_render().follow_tool_machines(true);typing.phase=1;Ok(((),RetainedCloneProgress{copied_items:1,copied_bytes:bytes,..Default::default()}))})?;
            },
            9=>return Ok(true),
            _=>return Err(invalid().into())
        }Ok(typing.complete())
    }
}
