// 🏭️ Registered factory and natural decode of the surface fixture: the controller and tool ids, the fixture job and factory with its owned tool job factory, the surface manifest, the editor fixture and the natural decode cursor.

//#region 🧪️SurfaceRegisteredFactory
/// 🪟️ The canonical surface id `EditorApp<SurfaceEditorFixture>` derives at runtime
/// (`surface_app_id(SURFACE_TESTKIT_DIALECT, Editor)`), never `EditorApp::APP_ID`'s `"surface"`
/// placeholder. It is the controller every tool key, manifest and proof row is stated against, and
/// `surface_editor_controller_is_the_derived_id` joins this literal to the derived value.
const SURFACE_CONTROLLER_ID: &str = "testkit.surface@1/*#editor";
const SURFACE_TOOL_ID: &str = "increment";
const SURFACE_PAYLOAD_SCHEMA: &str = "semio.testkit-surface.command.v1";
const SURFACE_TOOL_CONTRACT: ToolExecutionContract = ToolExecutionContract::resumable(4_096, 1, 1, 4_096, 500, 1, 1);

/// 🛠️ The fixture's real owned reducer body. It is a genuine [`semio_framework_job::InteractiveJob`]:
/// it owns its typed command, its retained wire pages and its completion, yields once per admitted
/// page, answers cancellation before anything else, and hands its one exact completion to the
/// publication ladder — the same shape a product editor's job has. A generic bounded proof is not a
/// substitute: it resolves to `interactive-job.missing-owned-reducer` at dispatch, so an editor
/// surface with no owned factory can only ever prove what the runtime FORBIDS.
struct SurfaceFixtureJob {
    owners: crate::app_testing::mutation_fixture::job_close::FixtureJobOwners<EditorApp<SurfaceEditorFixture>>,
    count: i32,
    page: usize,
}

impl semio_framework_job::InteractiveJob for SurfaceFixtureJob {
    fn step<'a>(&'a mut self, cx: &mut semio_framework_job::StepContext<'_>) -> Result<Option<semio_framework_job::JobOutcomeBorrow<'a>>, semio_framework_job::OriginalTurnFailure> {
        if cx.is_cancelled() {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into);
        }
        if cx.should_yield() {
            return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into);
        }
        if self.owners.completion_is_delivered(){return semio_framework_job::JobOutcomeBorrow::admit_complete(cx,None,None).map_err(Into::into)}
        if self.owners.raw.as_ref().is_some_and(|raw| self.page < raw.page_count()) {
            self.page += 1;
            return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into);
        }
        let Some(SurfaceEditorCommand::Increment) = self.owners.command() else {
            return semio_framework_job::JobOutcomeBorrow::admit_cancelled(cx).map_err(Into::into);
        };
        if self.owners.pending_is_empty(){let value=self.count+1;if !self.owners.prepare_emission(cx,std::mem::size_of::<i32>(),0,||Ok(Emit{artifact_mutations:vec![SetSurfaceCount{value}.into()],..Default::default()}))?{return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
        if !self.owners.complete(cx)?{return semio_framework_job::JobOutcomeBorrow::admit_yield(cx).map_err(Into::into)}
        semio_framework_job::JobOutcomeBorrow::admit_complete(cx, None, None).map_err(Into::into)
    }

    fn borrow_outcome<'a>(&'a self, descriptor: &'a semio_framework_job::JobOutcomeDescriptor) -> Result<semio_framework_job::JobOutcomeView<'a>, semio_framework_value::ValueError> {
        crate::app_testing::artifact_app_laws::fixture_job_outcome(descriptor)
    }

    fn begin_close(&mut self) {
        self.owners.begin_close();
    }

    fn close_step(&mut self,cx:&mut semio_framework_job::StepContext<'_>)->Result<semio_framework_job::InteractiveJobCloseStep,semio_framework_job::OriginalTurnFailure>{self.owners.close_step(cx)}

    fn next_close_copy_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.demand(0)?.copy_bytes)
    }

    fn next_close_capacity_byte_demand(&self, body: usize) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.demand(body)?.capacity_bytes)
    }

    fn next_close_release_byte_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.demand(0)?.release_bytes)
    }

    fn next_close_depth_demand(&self) -> Result<usize, semio_framework_value::ValueError> {
        Ok(self.owners.demand(0)?.depth)
    }

    fn terminal_is_empty(&self) -> bool {
        self.owners.terminal_is_empty()
    }
}

struct SurfaceFixtureFactory {
    keys: Vec<ToolFactoryKey>,
}

impl ToolJobFactory for SurfaceFixtureFactory {
    type Payload = SurfaceFixtureJob;
    type Job = SurfaceFixtureJob;
    fn keys(&self) -> &[ToolFactoryKey] {
        &self.keys
    }
    fn payload_schema_id(&self) -> &str {
        SURFACE_PAYLOAD_SCHEMA
    }
    fn classification(&self) -> semio_framework::InteractiveJobClassification {
        semio_framework::InteractiveJobClassification::Migrated
    }
    fn execution_contract(&self) -> ToolExecutionContract {
        SURFACE_TOOL_CONTRACT
    }
    fn admission_demands(&self,_operation:&semio_framework_job::Operation,payload:&Self::Payload,input:Option<&semio_framework_action_bus::RetainedToolWireInput>,checkpoint:Option<&semio_framework_action_bus::RetainedToolWireInput>)->Result<semio_framework::action_bus::ToolJobAdmissionDemand,semio_framework_value::ValueError>{if checkpoint.is_some()||(input.is_some()&&payload.owners.raw.is_some()){return Err(semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::UnsupportedOwner,"original typed job refuses a replacement wire owner or restart checkpoint"));}Ok(semio_framework::action_bus::ToolJobAdmissionDemand::reused_job())}
fn admit_job(&mut self,context:&mut semio_framework_job::StepContext<'_>,operation:&semio_framework_job::Operation,payload:&mut Option<Box<Self::Payload>>,input:&mut Option<semio_framework_action_bus::RetainedToolWireInput>,checkpoint:&mut Option<semio_framework_action_bus::RetainedToolWireInput>,output:&mut Option<Box<Self::Job>>)->Result<bool,semio_framework_job::OriginalTurnFailure>{let demand=self.admission_demands(operation,payload.as_deref().ok_or_else(||semio_framework_value::ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"original typed job payload is absent"))?,input.as_ref(),checkpoint.as_ref())?;semio_framework::action_bus::admit_original_tool_job_controlled(context,operation,payload,input,checkpoint,output,demand,|_,payload,input,_|{if input.is_some(){payload.as_mut().expect("retained original job").owners.raw=input.take();}let job=payload.take().expect("admitted same original job");Ok((job,semio_framework_job::RetainedCloneProgress{copied_items:1,..Default::default()}))})}

}

impl ArtifactOwnedToolJobFactory for SurfaceFixtureFactory {
    type Owner = EditorApp<SurfaceEditorFixture>;
    const TOOL_IDS: &'static [&'static str] = &[SURFACE_TOOL_ID];
    const DOCUMENT_SCHEMA: &'static str = <SurfaceEditorFixture as ArtifactEditor>::DOCUMENT_SCHEMA;
    const PUBLICATION_CONTRACTS: &'static [ArtifactToolPublicationContract] = &[ArtifactToolPublicationContract { tool_id: SURFACE_TOOL_ID, lanes: &[ArtifactToolPublicationLane::Artifact] }];
    fn latest_wins_target(_command: &SurfaceEditorCommand) -> Option<&str> {
        None
    }
    fn build_latest_wins_command_disposer() -> Option<Box<dyn semio_framework_plugin_ownership::store_lifecycle::ArtifactOwnedDisposer<SurfaceEditorCommand>>> {
        None
    }
}

async fn surface_manifest() -> crate::app_builder::App {
    crate::app_builder::App::from_builder(
        crate::app_builder::App::builder(SURFACE_CONTROLLER_ID, LocalizedLabel::data("Surface Fixture"))
            .await
            .document(["state"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .await
            .window_kind("main", LocalizedLabel::data("Main"), "surface.main", semio_framework_ui_contract::SurfaceKind::Canvas2d, IconName::AppWindow)
            .await
            .app_command(SURFACE_TOOL_ID, LocalizedLabel::data("Increment"), "fixture", ActionKind::Mutation)
            .await
            .interactive_jobs(semio_framework::InteractiveJobClassification::Migrated)
            .await,
    )
    .await
}
//#endregion 🧪️SurfaceRegisteredFactory

#[derive(Default)]
struct SurfaceEditorFixture;

struct SurfaceNaturalDecodeCursor {
    bytes: Option<Vec<u8>>,
    retirement: Option<Box<dyn store::ErasedSnapshotRetirement>>,
    pub(crate) offset: usize,
    pub(crate) decoded_items: usize,
    maximum_work_units_per_step:usize,
    word: [u8; 4],
    sum: i64,
    complete: bool,
    pub(crate) closing: bool,
}

impl SurfaceNaturalDecodeCursor {
    fn new(bytes: Vec<u8>,maximum_work_units_per_step:usize) -> Self {
        Self { bytes: Some(bytes), retirement: None, offset: 0, decoded_items: 0,maximum_work_units_per_step, word: [0; 4], sum: 0, complete: false, closing: false }
    }

    fn progress(&self) -> crate::app_contract::NaturalFileDecodeProgress {
        crate::app_contract::NaturalFileDecodeProgress { consumed_input_bytes: self.offset, decoded_items: self.decoded_items, owned_output_bytes: 0, completed_work_units: self.offset as u64 }
    }
}

impl crate::app_contract::NaturalFileDecodeCursor<SurfaceSnapshot> for SurfaceNaturalDecodeCursor {
    fn advance(&mut self, context: &mut semio_framework_job::StepContext<'_>) -> Result<crate::app_contract::NaturalFileDecodeStep<SurfaceSnapshot>, semio_framework::MediaError> {
        if self.closing || self.complete {
            return Err(semio_framework::MediaError::Payload(crate::app_contract::NATURAL_FILE_PORT.into(), "surface decoder is no longer advanceable".into()));
        }
        let bytes = self.bytes.as_ref().expect("surface decoder retains its bytes until close");
        let stop=self.offset.saturating_add(self.maximum_work_units_per_step).min(bytes.len());
        while self.offset < stop && !context.should_yield() && !context.is_cancelled() {
            self.word[self.offset % 4] = bytes[self.offset];
            self.offset += 1;
            context.consume_fuel(1);
            if self.offset % 4 == 0 {
                self.sum = self.sum.checked_add(i64::from(i32::from_be_bytes(self.word))).ok_or_else(|| semio_framework::MediaError::Payload(crate::app_contract::NATURAL_FILE_PORT.into(), "surface natural count overflow".into()))?;
                self.decoded_items += 1;
            }
        }
        if self.offset < bytes.len() {
            return Ok(crate::app_contract::NaturalFileDecodeStep::Progress(self.progress()));
        }
        let count = i32::try_from(self.sum).map_err(|_| semio_framework::MediaError::Payload(crate::app_contract::NATURAL_FILE_PORT.into(), "surface natural count is outside signed 32-bit range".into()))?;
        self.complete = true;
        Ok(crate::app_contract::NaturalFileDecodeStep::Complete { snapshot: SurfaceSnapshot { count }, progress: self.progress() })
    }

    fn begin_close(&mut self) {
        self.closing = true;
    }

    fn close_step(&mut self, grant: semio_framework_value::retained_clone::RetainedCloneGrant) -> Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep, Fault> {
        if self.retirement.is_some() {
            let step=store::os_store::snapshot_read::artifact_retirement_box_close_step(&mut self.retirement,grant).map_err(|error|Fault::from(error.into_message()))?;
            return Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(step.progress()));
        }
        if let Some(original)=self.bytes.take(){
            return match semio_framework_value::retirement::admit_owned_retirement(original,grant){
                Ok((owner,progress))=>{self.retirement=Some(owner);Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Progress(progress))},
                Err((error,original))=>{self.bytes=Some(original);Err(Fault::from(error.into_message()))},
            };
        }
        Ok(semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep::Complete(Default::default()))
    }

    fn retirement_demands(&self,body:usize)->Result<semio_framework_value::RetirementDemand,semio_framework_value::ValueError>{
        if let Some(owner)=self.retirement.as_ref(){return store::os_store::snapshot_read::artifact_retirement_box_demands(owner,body);}
        Ok(semio_framework_value::RetirementDemand{capacity_bytes:if self.bytes.is_some(){semio_framework_value::retirement::owned_retirement_birth_bytes::<Vec<u8>>()}else{0},depth:usize::from(self.bytes.is_some()),..Default::default()})
    }

    fn terminal_is_empty(&self) -> bool {
        self.closing && self.bytes.is_none() && self.retirement.is_none()
    }
}
