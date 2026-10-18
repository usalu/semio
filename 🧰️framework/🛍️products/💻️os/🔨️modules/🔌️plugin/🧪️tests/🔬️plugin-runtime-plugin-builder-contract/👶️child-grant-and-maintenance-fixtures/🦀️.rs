/// 👶️ Child, grant and maintenance fixtures: the child emit and fixture child preparation, the close grants, the job close and maintenance drivers, the test mutation applier and the surface id.

    /// 🌐️ One locale cell of a history row's label carrier. Every assertion below names the locale
    /// it reads, so a row that carries only English cannot pass by accident; the carrier's own
    /// locale axis is asserted by `a_shell_noted_row_carries_the_same_text_in_every_locale`.
    #[doc(hidden)]
pub fn test_child_emit(slot:impl Into<String>,child_id:impl Into<String>,operations:&[TestMutation])->ChildEmit{
        let mut emit=ChildEmit::open(slot,child_id,operations.len());
        for operation in operations{emit.push::<TestSnapshot,_>(operation).expect("actual fixture mutation must encode completely");}
        emit
    }

    /// 📨️ Funds the actual original child preparation quote within the fixture's declared per-axis ceiling.
    #[doc(hidden)]
pub fn prepare_original_fixture_child(emit:&mut Emit<TestMutation,TestConfigMutation>,maximum_items:usize,maximum_bytes:usize)->Result<ChildEmitPreparationStep,Fault>{
        let demand=emit.child_preparation_demands(maximum_bytes).expect("original child preparation quote");
        assert!(demand.copy_bytes<=maximum_bytes&&demand.capacity_bytes<=maximum_bytes&&demand.release_bytes<=maximum_bytes&&demand.depth<=128);
        let grant=semio_framework_value::retained_clone::RetainedCloneGrant{maximum_items,maximum_copy_bytes:maximum_bytes,maximum_capacity_bytes:maximum_bytes,maximum_release_bytes:maximum_bytes,maximum_depth:128};
        emit.prepare_child_one(grant)
    }
    #[doc(hidden)]
pub fn label_in(label: &LocalizedLabel, locale: Locale) -> &str {
        label.resolve(Terminology::Native, locale)
    }

    /// 🌉️ Test-only convenience: builds a `serde_json::json!` literal, then bridges it to the
    /// `DslValue` `handle_action`/`dispatch_action`/`command_from_action` speak at the trait
    /// boundary — the fixtures below stay readable as JSON literals without keeping a
    /// `serde_json::Value` alive past this one conversion.
    #[doc(hidden)]
pub fn dv(value: Value) -> DslValue {
        semio_framework_value::DslValue::from(&value)
    }

    /// 🎟️ Validates a physical demand within the caller-authored fixture close ceiling.
    #[doc(hidden)]
pub fn fixture_close_grant(demand:semio_framework_value::RetirementDemand)->semio_framework_value::retained_clone::RetainedCloneGrant{
        let grant=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close;
        assert!(grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth,"demand exceeds the original fixture close policy");
        grant
    }

    /// 🎟️ Checks the next physical demand against the fixture's original close policy.
    #[doc(hidden)]
pub fn job_close_grant<J: semio_framework_job::InteractiveJob + ?Sized>(job: &J) -> semio_framework_value::retained_clone::RetainedCloneGrant {
        let grant=semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().close;
        let demand=semio_framework_value::RetirementDemand{copy_bytes:job.next_close_copy_byte_demand().expect("quoted copy demand"),capacity_bytes:job.next_close_capacity_byte_demand(grant.maximum_copy_bytes).expect("quoted capacity demand"),release_bytes:job.next_close_release_byte_demand().expect("quoted release demand"),depth:job.next_close_depth_demand().expect("quoted depth demand")};
        assert!(grant.maximum_items>0&&grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth,"job demand exceeds its original fixture close policy");
        grant
    }

    /// 🧹️ Drives one interactive job to its terminal-empty witness, granting exactly each quoted turn and checking every receipt fits.
    #[doc(hidden)]
pub fn drive_job_close<J:semio_framework_job::InteractiveJob+?Sized>(job:&mut J,caller:&mut semio_framework_job::OriginalJobFixtureCaller){
        for _ in 0..100_000{
            if job.terminal_is_empty(){return}
            let grant=job_close_grant(job);let step=caller.turn(grant,|cx|job.close_step(cx)).expect("original fixture close preserves its full failure");
            assert_eq!(step.progress(),caller.receipt());assert!(step.progress().fits(grant),"job close receipt {step:?} exceeds its original fixture grant {grant:?}");
        }
        panic!("job close did not reach its terminal-empty witness");
    }

    /// 🎟️ Spends exactly the quoted maintenance turn of a hosted app.
    #[doc(hidden)]
pub fn app_maintenance_turn<P: PluginApp>(app: &mut P,cx:&mut semio_framework_job::StepContext<'_>) -> Result<semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleStep, semio_framework_plugin_ownership::bounded_ownership::PluginLifecycleFailure> {
        let demand = app.maintenance_retirement_demands(store::os_store::owned_schema_decode::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES)?;
        let grant = semio_framework_plugin_testing_laws::artifact_app_laws::fixture_mounted_policy().maintenance;
        assert!(grant.maximum_copy_bytes>=demand.copy_bytes&&grant.maximum_capacity_bytes>=demand.capacity_bytes&&grant.maximum_release_bytes>=demand.release_bytes&&grant.maximum_depth>=demand.depth,"app demand exceeds its original fixture maintenance policy");
        let step = semio_framework_plugin_ownership::bounded_ownership::original_lifecycle_step(cx,grant,demand,|same,grant|Ok((app.maintenance_step(same,grant)?,app.maintenance_terminal_is_empty())))?;
        assert!(step.progress().is_none_or(|progress| progress.fits(grant)), "maintenance receipt {step:?} exceeds its exact grant {grant:?}");
        Ok(step)
    }

    /// ✍️ Authors one `Apply` of document mutations under the plugin's caller-owned identity authority.
    #[doc(hidden)]
pub async fn apply_test_mutations(store: &mut store::ArtifactStore<semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::TestSnapshot, semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation>, mutations: Vec<semio_framework_plugin_testing_fixtures::test_app_mutation_fixture::document::mutations::TestMutation>) -> Result<store::CommandReceipt, store::VcsError> {
        semio_framework_plugin_testing_laws::with_authoring_identity!(|identity| Box::pin(store.dispatch(store::os_store::store_schema::ArtifactCommand::Apply { mutations, transaction: None }, &mut identity)).await)
    }

    /// 🪪️ `TestApp`'s one dialect coordinate (contract §1) — every canonical id this module needs
    /// (`TestApp::APP_ID`, the `App::builder` ids below, every `CommandAddress`/`ActionAddress`/
    /// `ClipboardFragment.source_app` literal) derives from this ONE fixture instead of the
    /// pre-migration `"synthetic-play"` string, so `test_app_surface_id` and `TestApp::APP_ID` can
    /// never independently drift (guarded by `test_app_id_matches_its_own_dialect` below).
    #[doc(hidden)]
pub const TEST_APP_DIALECT: Dialect = Dialect { artifact_kind: "s.test.synthetic", standard: StandardId("1"), subset: SubsetId::ANY };

    /// 🪟️ The one window body key `interaction_registry`'s `window_kind("main", …)` declares — the lane
    /// `TestApp::interaction_scope` names, so the fixture and the law can never disagree about it.
    #[doc(hidden)]
pub const TEST_APP_WINDOW_BODY_KEY: &str = "synthetic.main";

    std::thread_local! {
        pub static RENDER_CONTEXT_PROBE: std::cell::RefCell<Option<(String, ViewModel)>> = const { std::cell::RefCell::new(None) };
    }

    /// 🪪️ Live `surface_app_id` call over `TEST_APP_DIALECT` — the id every `App::builder(...)` call
    /// in this module passes, per contract §1 (`AppBuilder::build_definition` rejects a hand-written id).
    #[doc(hidden)]
pub async fn test_app_surface_id() -> String {
        surface_app_id(&TEST_APP_DIALECT.into(), AppRole::Editor)
    }
