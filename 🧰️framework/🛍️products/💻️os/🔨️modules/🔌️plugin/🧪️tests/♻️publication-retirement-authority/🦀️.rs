//! ♻️ Every typed-operation publication lane retires a REJECTED authority without faulting per turn.
//!
//! Regression cover for ticket 26/09/09/PROCEDURAL-3D-END-TO-END
//! (`📓️invoke-extension-rejected-authority-2026-09-11.md`): all seven lanes used to answer every
//! incomplete `close_step` of a faulted `Closing` publication with
//! `<lane> publication is retiring a rejected authority`, so the host saw
//! `typed-operation failed: …` on every turn of the drain and `invokeExtension` never completed.

use super::{ArtifactApp, ArtifactMutationOutcome, ArtifactView, ConfigView, DraftView, EngineHandles, InteractionView, PendingArtifactStorePublication, PendingArtifactStorePublicationRetirement, UiAssemblyResult, WindowConfigMutation, WindowConfigOwner, WindowConfigOwnerRegistry, WindowTransientMutation, WindowTransientOwner, WindowTransientOwnerBundle, WindowTransientOwnerRegistry};
use crate::app::{built_text_to_component_tree, bounded_config_store_one_item_preparation_factory, bounded_config_store_owners, bounded_config_store_disposer, bounded_document_store_disposer, bounded_document_store_owners, testkit, testkit::close_registered_fixture_app};
use crate::publication_fixture::{ChangePublicationPresence, ChangePublicationTransient, PublicationPresence, PublicationPresenceMutation, PublicationTransient, PublicationTransientMutation};
use crate::store;
use crate::test_app_mutation_fixture::{ChangeTestConfigSelection, SetCount, TestConfig, TestConfigMutation, TestMutation, TestSnapshot};
use semio_framework::{Fault, ViewModel, ViewWindowInstance};
use std::sync::Arc;

const RETIREMENT_AUTHORITY_FIXTURE_JSON: &str = include_str!("../../🧫️fixtures/♻️publication-retirement-authority/🔣️.json");

/// 🪟️ The one window kind both window lanes of this fixture partition under.
const RETIREMENT_WINDOW_KIND: &str = "publication-retirement-window";

fn fixture() -> serde_json::Value {
    let fixture: serde_json::Value = serde_json::from_str(RETIREMENT_AUTHORITY_FIXTURE_JSON).expect("publication retirement authority fixture parses");
    assert_eq!(fixture["schema"], "framework.plugin.publication-retirement-authority.v1");
    fixture
}

fn grant(fixture: &serde_json::Value) -> store::ArtifactStoreOneItemGrant {
    store::ArtifactStoreOneItemGrant { maximum_items: fixture["grant"]["maximumItems"].as_u64().expect("fixture grant items") as usize, maximum_bytes: fixture["grant"]["maximumBytes"].as_u64().expect("fixture grant bytes") as usize }
}

//#region ♻️RetirementFixtureLeaves
impl store::retirement::RetireOwned for PublicationPresence {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        store::retirement::leaf(self.revision)
    }
}

impl store::retirement::RetireOwned for PublicationPresenceMutation {
    fn retirement(self) -> Box<dyn store::retirement::RetirementCursor> {
        let Self::ChangePublicationPresence(value) = self;
        store::retirement::leaf(value.revision)
    }
}

fn presence_footprint(_: &PublicationPresenceMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: size_of::<PublicationPresenceMutation>() })
}

fn presence_transfer(mutation: PublicationPresenceMutation) -> PublicationPresence {
    let PublicationPresenceMutation::ChangePublicationPresence(value) = mutation;
    PublicationPresence { revision: value.revision }
}

fn presence_preparation_factory() -> Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<PublicationPresence, PublicationPresenceMutation>> {
    Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(
        presence_footprint,
        presence_transfer,
        Arc::new(store::retirement::OwnedValueRetirementFactory::<PublicationPresence>::default()),
        Arc::new(store::retirement::OwnedValueRetirementFactory::<PublicationPresenceMutation>::default()),
    ))
}

fn transient_footprint(_: &PublicationTransientMutation) -> Result<store::ArtifactStoreOneItemFootprint, String> {
    Ok(store::ArtifactStoreOneItemFootprint { work_items: 1, retained_bytes: size_of::<PublicationTransientMutation>() })
}

fn transient_transfer(mutation: PublicationTransientMutation) -> PublicationTransient {
    let PublicationTransientMutation::ChangePublicationTransient(value) = mutation;
    PublicationTransient { revision: value.revision }
}

fn transient_preparation_factory() -> Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<PublicationTransient, PublicationTransientMutation>> {
    Arc::new(store::ArtifactEphemeralTransferPreparationFactory::new(
        transient_footprint,
        transient_transfer,
        Arc::new(store::retirement::OwnedValueRetirementFactory::<PublicationTransient>::default()),
        Arc::new(store::retirement::OwnedValueRetirementFactory::<PublicationTransientMutation>::default()),
    ))
}

/// ♻️ Releases a displaced ephemeral root the moment its owner hands it over. The shared-value
/// factory cannot serve here: while a REJECTED sibling publication is still draining it co-owns the
/// very root the superseding write displaced, and a unique-ownership retirement would block forever.
struct FixtureRootRetirement<T>(Option<Arc<T>>);

impl<T: Send + Sync + 'static> store::ErasedSnapshotRetirement for FixtureRootRetirement<T> {
    fn close_step(&mut self, maximum_items: usize, _maximum_bytes: usize) -> Result<store::SnapshotRetirementStep, String> {
        if maximum_items == 0 {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if self.0.take().is_some() {
            return Ok(store::SnapshotRetirementStep::Pending { released_items: 1, released_bytes: 0 });
        }
        Ok(store::SnapshotRetirementStep::Complete)
    }

    fn terminal_is_empty(&self) -> bool {
        self.0.is_none()
    }
}

struct FixtureRootRetirementFactory<T>(std::marker::PhantomData<fn() -> T>);

impl<T> Default for FixtureRootRetirementFactory<T> {
    fn default() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<T: Send + Sync + 'static> store::SnapshotRetirementFactory<T> for FixtureRootRetirementFactory<T> {
    fn retire(&self, snapshot: Arc<T>) -> Box<dyn store::ErasedSnapshotRetirement> {
        Box::new(FixtureRootRetirement(Some(snapshot)))
    }
}

/// 🧹️ Bounded presence-store disposer: the fixture app must hand the runtime one, or bounded close
/// fails with `interactive-job.close-owned-disposer-missing`.
struct FixturePresenceStoreDisposer(Option<store::PresenceStoreRetirement<PublicationPresence>>);

impl crate::app::ArtifactOwnedDisposer<store::PresenceStore<PublicationPresence, PublicationPresenceMutation>> for FixturePresenceStoreDisposer {
    fn close_step(&mut self, owner: &mut store::PresenceStore<PublicationPresence, PublicationPresenceMutation>, maximum_items: usize, maximum_bytes: usize) -> Result<crate::app::PluginCloseStep, Fault> {
        if maximum_items == 0 {
            return Ok(crate::app::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        if let Some(active) = self.0.as_mut() {
            return active.close_step(1, maximum_bytes).map_err(Fault::from).map(|step| match step {
                store::SnapshotRetirementStep::Pending { released_items, released_bytes } => crate::app::PluginCloseStep::Pending { released_items, released_bytes },
                store::SnapshotRetirementStep::Blocked => crate::app::PluginCloseStep::Blocked { reason: "presence fixture retains captured readers" },
                store::SnapshotRetirementStep::Complete => crate::app::PluginCloseStep::Complete,
            });
        }
        self.0 = Some(owner.begin_retirement(Arc::new(PublicationPresence::default()), |_| true).map_err(|(reason, _)| Fault::from(reason))?);
        Ok(crate::app::PluginCloseStep::Pending { released_items: 1, released_bytes: 0 })
    }

    fn terminal_is_empty(&self, owner: &store::PresenceStore<PublicationPresence, PublicationPresenceMutation>) -> bool {
        owner.retirement_started() && self.0.as_ref().is_some_and(store::PresenceStoreRetirement::terminal_is_empty) && owner.peers_root().is_empty()
    }
}

/// 🧹️ Bounded transient-store disposer paired with [`FixturePresenceStoreDisposer`].
struct FixtureTransientStoreDisposer;

impl crate::app::ArtifactOwnedDisposer<store::TransientStore<PublicationTransient, PublicationTransientMutation>> for FixtureTransientStoreDisposer {
    fn close_step(&mut self, _owner: &mut store::TransientStore<PublicationTransient, PublicationTransientMutation>, maximum_items: usize, _maximum_bytes: usize) -> Result<crate::app::PluginCloseStep, Fault> {
        if maximum_items == 0 {
            return Ok(crate::app::PluginCloseStep::Pending { released_items: 0, released_bytes: 0 });
        }
        Ok(crate::app::PluginCloseStep::Complete)
    }

    fn terminal_is_empty(&self, _owner: &store::TransientStore<PublicationTransient, PublicationTransientMutation>) -> bool {
        true
    }
}

struct RetirementWindowConfigOwner;

impl WindowConfigOwner for RetirementWindowConfigOwner {
    const WINDOW_KIND_ID: &'static str = RETIREMENT_WINDOW_KIND;
    const SCHEMA: &'static str = "plugin.testkit.publication-retirement-window-config";
    const MAXIMUM_PUBLICATION_BYTES: usize = 1_024;
    type State = TestConfig;
    type Mutation = TestConfigMutation;

    fn build_store_owners() -> store::MemberStoreOwners<Self::State, Self::Mutation> {
        crate::app::bounded_window_config_store_owners::<Self>()
    }

    fn build_one_item_preparation_factory() -> Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
        crate::app::bounded_window_config_preparation_factory::<Self>()
    }

    fn build_store_disposer() -> Box<dyn crate::app::ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
        crate::app::bounded_window_config_store_disposer::<Self>()
    }
}

struct RetirementWindowTransientOwner;

impl WindowTransientOwner for RetirementWindowTransientOwner {
    const WINDOW_KIND_ID: &'static str = RETIREMENT_WINDOW_KIND;
    type State = PublicationTransient;
    type Mutation = PublicationTransientMutation;

    fn build_owners() -> WindowTransientOwnerBundle<Self::State, Self::Mutation> {
        crate::window_transient_testkit::owners()
    }
}
//#endregion ♻️RetirementFixtureLeaves

//#region ♻️RetirementFixtureApp
/// 🧪️ The one fixture app that owns a REAL store on every publication lane at once, so the shared
/// retirement law can be driven against seven concrete publication types rather than a stand-in.
#[derive(Default)]
struct RetirementApp;

impl ArtifactApp for RetirementApp {
    const DIALECT: crate::Dialect = crate::Dialect { artifact_kind: "s.test.publication-retirement", standard: crate::StandardId("1"), subset: crate::SubsetId::ANY };
    const APP_ID: &'static str = "testkit-publication-retirement";
    const DOCUMENT_SCHEMA: &'static str = "semio.testkit-publication-retirement/v1";
    type Snapshot = TestSnapshot;
    type Mutation = TestMutation;
    type Config = TestConfig;
    type ConfigMutation = TestConfigMutation;
    type Draft = TestConfig;
    type DraftMutation = TestConfigMutation;
    type Presence = PublicationPresence;
    type PresenceMutation = PublicationPresenceMutation;
    type Transient = PublicationTransient;
    type TransientMutation = PublicationTransientMutation;
    type Command = PublicationTransientMutation;

    fn register_window_config_owners(registry: &mut WindowConfigOwnerRegistry) -> Result<(), Fault> {
        registry.register::<RetirementWindowConfigOwner>()
    }

    fn register_window_transient_owners(registry: &mut WindowTransientOwnerRegistry) -> Result<(), Fault> {
        registry.register::<RetirementWindowTransientOwner>()
    }

    fn build_document_store_owners() -> Option<store::MemberStoreOwners<Self::Snapshot, Self::Mutation>> {
        Some(bounded_document_store_owners::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_owners() -> Option<store::MemberStoreOwners<Self::Config, Self::ConfigMutation>> {
        Some(bounded_config_store_owners::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_owners() -> Option<store::MemberStoreOwners<Self::Draft, Self::DraftMutation>> {
        Some(bounded_config_store_owners::<Self::Draft, Self::DraftMutation>())
    }

    fn build_document_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::ArtifactStore<Self::Snapshot, Self::Mutation>>>> {
        Some(bounded_document_store_disposer::<Self::Snapshot, Self::Mutation>())
    }

    fn build_config_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::ConfigStore<Self::Config, Self::ConfigMutation>>>> {
        Some(bounded_config_store_disposer::<Self::Config, Self::ConfigMutation>())
    }

    fn build_draft_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::DraftStore<Self::Draft, Self::DraftMutation>>>> {
        Some(bounded_config_store_disposer::<Self::Draft, Self::DraftMutation>())
    }

    fn build_artifact_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Snapshot, Self::Mutation>>> {
        Some(bounded_config_store_one_item_preparation_factory::<Self::Snapshot, Self::Mutation>("retirement-doc", 1_024))
    }

    fn build_config_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Config, Self::ConfigMutation>>> {
        Some(bounded_config_store_one_item_preparation_factory::<Self::Config, Self::ConfigMutation>("retirement-cfg", 1_024))
    }

    fn build_draft_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::Draft, Self::DraftMutation>>> {
        Some(bounded_config_store_one_item_preparation_factory::<Self::Draft, Self::DraftMutation>("retirement-draft", 1_024))
    }

    fn build_presence_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Presence, Self::PresenceMutation>>> {
        Some(presence_preparation_factory())
    }

    fn build_transient_store_one_item_preparation_factory() -> Option<Arc<dyn store::ArtifactEphemeralOneItemPreparationFactory<Self::Transient, Self::TransientMutation>>> {
        Some(transient_preparation_factory())
    }

    fn build_presence_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::PresenceStore<Self::Presence, Self::PresenceMutation>>>> {
        Some(Box::new(FixturePresenceStoreDisposer(None)))
    }

    fn build_transient_store_disposer() -> Option<Box<dyn crate::app::ArtifactOwnedDisposer<store::TransientStore<Self::Transient, Self::TransientMutation>>>> {
        Some(Box::new(FixtureTransientStoreDisposer))
    }

    fn build_presence_local_root_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(Arc::new(FixtureRootRetirementFactory::<PublicationPresence>::default()))
    }

    fn build_presence_peer_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Presence>>> {
        Some(Arc::new(FixtureRootRetirementFactory::<PublicationPresence>::default()))
    }

    fn build_transient_local_root_retirement_factory() -> Option<Arc<dyn store::SnapshotRetirementFactory<Self::Transient>>> {
        Some(Arc::new(FixtureRootRetirementFactory::<PublicationTransient>::default()))
    }

    async fn initial_snapshot() -> TestSnapshot {
        TestSnapshot::default()
    }

    async fn handle(
        _command: &PublicationTransientMutation,
        _doc: &ArtifactView<'_, TestSnapshot>,
        _cfg: &ConfigView<'_, TestConfig>,
        _interaction: &InteractionView<'_>,
        _view_state: Option<&ViewModel>,
        _draft: &DraftView<'_, TestConfig>,
        _engines: &EngineHandles,
    ) -> ArtifactMutationOutcome<TestMutation, TestConfigMutation, TestConfigMutation> {
        Ok(Default::default())
    }

    async fn render(_body_key: &str, doc: &ArtifactView<'_, TestSnapshot>, _cfg: &ConfigView<'_, TestConfig>, _view_state: &ViewModel) -> UiAssemblyResult<semio_framework_ui_runtime::ComponentTree> {
        built_text_to_component_tree(ui_wgpu::wgpu::Label::data(format!("count={}", doc.snapshot.count)))
    }
}
//#endregion ♻️RetirementFixtureApp

//#region ♻️RetirementDrivers
/// 🔭️ Drives ONE rejected publication through the shared retirement law and reports how many
/// incomplete turns it answered with `Ok` before its terminal turn.
fn drive_rejected_retirement(pending: &mut PendingArtifactStorePublication<RetirementApp>, grant: store::ArtifactStoreOneItemGrant, row: &serde_json::Value) -> usize {
    let (lane, label) = pending.lane();
    assert_eq!(format!("{lane:?}"), row["lane"].as_str().expect("fixture lane"), "lane identity");
    assert_eq!(label, row["storeLabel"].as_str().expect("fixture store label"), "store label for {lane:?}");
    assert_eq!(format!("{label} publication closed without terminal emptiness"), row["falseTerminalFault"].as_str().expect("fixture false terminal fault"));
    assert!(pending.is_closing(), "{lane:?} store moved its rejected publication into retirement");
    assert_eq!(pending.fault(), Some(row["supersededFault"].as_str().expect("fixture superseded fault")), "{lane:?} retains its store rejection reason");
    let mut retiring_turns = 0;
    for _ in 0..4_096 {
        match pending.retirement_turn(grant.maximum_items, grant.maximum_bytes).unwrap_or_else(|fault| panic!("{lane:?} answered an incomplete retirement turn with a fault: {fault:?}")) {
            PendingArtifactStorePublicationRetirement::Retiring => retiring_turns += 1,
            PendingArtifactStorePublicationRetirement::Retired => {
                assert_eq!(row["terminalOutcome"], "retired", "{lane:?} terminal outcome");
                assert!(row["rejectedFault"].is_null(), "{lane:?} declares no terminal fault");
                return retiring_turns;
            }
            PendingArtifactStorePublicationRetirement::Rejected(fault) => {
                assert_eq!(row["terminalOutcome"], "rejected", "{lane:?} terminal outcome");
                assert_eq!(fault.message, row["rejectedFault"].as_str().expect("fixture rejected fault"), "{lane:?} terminal fault text");
                return retiring_turns;
            }
        }
    }
    panic!("{lane:?} rejected publication never reached its terminal retirement turn");
}

/// ✅️ Publishes one ephemeral mutation to completion, which is what moves the store past the
/// generation an already-begun sibling publication captured. It is NOT retired here: its displaced
/// root stays co-owned by the rejected sibling's own base read until that sibling has retired, so
/// the superseding owner is retired last.
fn publish_ephemeral<P: Send + Sync + 'static, M>(
    publication: &mut store::ArtifactEphemeralOneItemPublication<P, M>,
    grant: store::ArtifactStoreOneItemGrant,
    mut advance: impl FnMut(&mut store::ArtifactEphemeralOneItemPublication<P, M>, store::ArtifactStoreOneItemGrant) -> Result<store::ArtifactStoreOneItemAdvance, String>,
) {
    for _ in 0..4_096 {
        if matches!(advance(publication, grant).expect("superseding ephemeral publication advances"), store::ArtifactStoreOneItemAdvance::Published(_)) {
            assert!(publication.acknowledge());
            return;
        }
    }
    panic!("superseding ephemeral publication never published");
}

/// ✅️ Retires a publication the store accepted — the same shared law, whose terminal turn owes the
/// host nothing because nothing rejected it.
fn retire_accepted(pending: &mut PendingArtifactStorePublication<RetirementApp>, grant: store::ArtifactStoreOneItemGrant) {
    let (lane, _) = pending.lane();
    for _ in 0..8_192 {
        match pending.retirement_turn(grant.maximum_items, grant.maximum_bytes).unwrap_or_else(|fault| panic!("{lane:?} accepted publication retires: {fault:?}")) {
            PendingArtifactStorePublicationRetirement::Retiring => {}
            PendingArtifactStorePublicationRetirement::Retired => {
                assert!(pending.terminal_is_empty());
                return;
            }
            PendingArtifactStorePublicationRetirement::Rejected(fault) => panic!("{lane:?} accepted publication reported a rejection: {fault:?}"),
        }
    }
    panic!("{lane:?} accepted publication never retired");
}

fn retirement_view() -> ViewModel {
    ViewModel {
        window_id: Some("publication-retirement-window-left".into()),
        window_instances: vec![ViewWindowInstance { id: "publication-retirement-window-left".into(), window_kind_id: RETIREMENT_WINDOW_KIND.into() }],
        ..Default::default()
    }
}

fn document_mutation(value: i32) -> TestMutation {
    SetCount { value }.into()
}

fn config_mutation(value: &str) -> TestConfigMutation {
    ChangeTestConfigSelection { selected: Some(value.to_string()) }.into()
}
//#endregion ♻️RetirementDrivers

#[semio_framework_async_macros::async_test]
async fn every_publication_lane_retires_a_rejected_authority_without_faulting_each_turn() {
    let fixture = fixture();
    let grant = grant(&fixture);
    let rows = fixture["lanes"].as_array().expect("fixture lanes").clone();
    let minimum_turns = fixture["law"]["minimumIncompleteTurnsWhileRejected"].as_u64().expect("fixture minimum turns") as usize;
    assert_eq!(rows.len(), 7, "the law covers every publication lane");
    assert_eq!(fixture["law"]["incompleteRetirementTurnIsOk"], true);
    let operation = semio_framework_job::OperationId(1);
    let mut app = testkit::new_app::<RetirementApp>().await;
    let view = retirement_view();
    let window_config_authority = app.window_config_store.capture(Some(&view)).await.expect("window config capture").expect("registered window config owner");
    let window_transient_authority = app.window_transient_store.capture(Some(&view)).expect("window transient capture").expect("registered window transient owner");

    for row in &rows {
        // 🪢️ The superseding owner is retired LAST: until the rejected sibling has drained, both
        // still co-own the root the superseding write displaced.
        let mut superseding_owner: Option<PendingArtifactStorePublication<RetirementApp>> = None;
        let mut pending = match row["id"].as_str().expect("fixture lane id") {
            "artifact" => {
                let publication = app
                    .store
                    .begin_apply_batch(operation, app.store.generation_now(), app.store.content_revision_now(), "fixture".into(), vec![document_mutation(1)], None, store::HistoryLane::Document, app.artifact_one_item_factory.as_ref())
                    .unwrap_or_else(|rejected| panic!("artifact publication admitted: {}", rejected.into_owners().0));
                app.store.dispatch(store::ArtifactCommand::Apply { mutations: vec![document_mutation(9)], description: None }).await.expect("superseding document write");
                let mut publication = publication;
                assert!(app.store.advance_apply_batch(&mut publication, grant).is_err(), "the superseded document publication is rejected");
                PendingArtifactStorePublication::Artifact(publication)
            }
            "config" => {
                let publication = app
                    .config_store
                    .begin_apply_batch(operation, app.config_store.generation_now(), app.config_store.content_revision_now(), "fixture".into(), vec![config_mutation("first")], None, store::HistoryLane::Document, app.config_one_item_factory.as_ref())
                    .unwrap_or_else(|rejected| panic!("config publication admitted: {}", rejected.into_owners().0));
                app.config_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![config_mutation("superseding")], description: None }).await.expect("superseding config write");
                let mut publication = publication;
                assert!(app.config_store.advance_apply_batch(&mut publication, grant).is_err(), "the superseded config publication is rejected");
                PendingArtifactStorePublication::Config(publication)
            }
            "draft" => {
                let publication = app
                    .draft_store
                    .begin_apply_batch(operation, app.draft_store.generation_now(), app.draft_store.content_revision_now(), "fixture".into(), vec![config_mutation("first")], None, store::HistoryLane::Document, app.draft_one_item_factory.as_ref())
                    .unwrap_or_else(|rejected| panic!("draft publication admitted: {}", rejected.into_owners().0));
                app.draft_store.dispatch(store::ArtifactCommand::Apply { mutations: vec![config_mutation("superseding")], description: None }).await.expect("superseding draft write");
                let mut publication = publication;
                assert!(app.draft_store.advance_apply_batch(&mut publication, grant).is_err(), "the superseded draft publication is rejected");
                PendingArtifactStorePublication::Draft(publication)
            }
            "presence" => {
                let generation = app.presence_store.generation_now();
                let factory = app.presence_one_item_factory.clone().expect("presence preparation factory");
                let retirement = app.presence_local_root_retirement_factory.clone();
                let mut publication = app
                    .presence_store
                    .begin_publish_one(operation, generation, ChangePublicationPresence { revision: 1 }.into(), Some(factory.as_ref()), retirement.clone())
                    .unwrap_or_else(|rejected| panic!("presence publication admitted: {}", rejected.into_owners().0));
                let mut superseding = app.presence_store.begin_publish_one(operation, generation, ChangePublicationPresence { revision: 2 }.into(), Some(factory.as_ref()), retirement).expect("superseding presence publication admitted");
                publish_ephemeral(&mut superseding, grant, |publication, grant| app.presence_store.advance_publish_one(publication, grant));
                superseding_owner = Some(PendingArtifactStorePublication::Presence(superseding));
                assert!(app.presence_store.advance_publish_one(&mut publication, grant).is_err(), "the superseded presence publication is rejected");
                PendingArtifactStorePublication::Presence(publication)
            }
            "transient" => {
                let generation = app.transient_store.generation_now();
                let factory = app.transient_one_item_factory.clone().expect("transient preparation factory");
                let retirement = app.transient_local_root_retirement_factory.clone();
                let mut publication = app
                    .transient_store
                    .begin_publish_one(operation, generation, ChangePublicationTransient { revision: 1 }.into(), Some(factory.as_ref()), retirement.clone())
                    .unwrap_or_else(|rejected| panic!("transient publication admitted: {}", rejected.into_owners().0));
                let mut superseding = app.transient_store.begin_publish_one(operation, generation, ChangePublicationTransient { revision: 2 }.into(), Some(factory.as_ref()), retirement).expect("superseding transient publication admitted");
                publish_ephemeral(&mut superseding, grant, |publication, grant| app.transient_store.advance_publish_one(publication, grant));
                superseding_owner = Some(PendingArtifactStorePublication::Transient(superseding));
                assert!(app.transient_store.advance_publish_one(&mut publication, grant).is_err(), "the superseded transient publication is rejected");
                PendingArtifactStorePublication::Transient(publication)
            }
            "windowConfig" => {
                let mutation = |value: &str| WindowConfigMutation::of::<RetirementWindowConfigOwner>("publication-retirement-window-left", config_mutation(value));
                let mut publication = app.window_config_store.begin(operation, "fixture".into(), &window_config_authority, mutation("first")).expect("window config publication admitted");
                let mut superseding = app.window_config_store.begin(operation, "fixture".into(), &window_config_authority, mutation("superseding")).expect("superseding window config publication admitted");
                for _ in 0..4_096 {
                    if matches!(app.window_config_store.advance(superseding.as_mut(), grant).expect("superseding window config advances"), store::ArtifactStoreOneItemAdvance::Published(_)) {
                        assert!(superseding.acknowledge());
                        break;
                    }
                }
                superseding_owner = Some(PendingArtifactStorePublication::WindowConfig(superseding));
                assert!(app.window_config_store.advance(publication.as_mut(), grant).is_err(), "the superseded window config publication is rejected");
                PendingArtifactStorePublication::WindowConfig(publication)
            }
            "windowTransient" => {
                let mutation = |revision| WindowTransientMutation::of::<RetirementWindowTransientOwner>("publication-retirement-window-left", ChangePublicationTransient { revision }.into());
                let mut publication = app.window_transient_store.begin(operation, &window_transient_authority, mutation(11)).expect("window transient publication admitted");
                let mut superseding = app.window_transient_store.begin(operation, &window_transient_authority, mutation(12)).expect("superseding window transient publication admitted");
                for _ in 0..4_096 {
                    if matches!(app.window_transient_store.advance(superseding.as_mut(), grant).expect("superseding window transient advances"), store::ArtifactStoreOneItemAdvance::Published(_)) {
                        assert!(superseding.acknowledge());
                        break;
                    }
                }
                superseding_owner = Some(PendingArtifactStorePublication::WindowTransient(superseding));
                assert!(app.window_transient_store.advance(publication.as_mut(), grant).is_err(), "the superseded window transient publication is rejected");
                PendingArtifactStorePublication::WindowTransient(publication)
            }
            other => panic!("fixture declares an unknown publication lane {other}"),
        };
        let retiring_turns = drive_rejected_retirement(&mut pending, grant, row);
        assert!(retiring_turns >= minimum_turns, "{} drained its rejection over {retiring_turns} incomplete turns, every one of which used to be a fault", row["id"]);
        assert!(pending.terminal_is_empty(), "{} retired terminal-empty", row["id"]);
        if let Some(mut owner) = superseding_owner {
            retire_accepted(&mut owner, grant);
        }
        eprintln!("[DEBUG] {} retired a rejected authority over {retiring_turns} Ok turns then {}", row["id"], row["terminalOutcome"]);
    }
    drop((window_config_authority, window_transient_authority));
    close_registered_fixture_app(&mut app);
}

#[semio_framework_async_macros::async_test]
async fn window_transient_re_begin_needs_the_refreshed_live_generation() {
    let fixture = fixture();
    let grant = grant(&fixture);
    let expected = fixture["windowTransientReBegin"].clone();
    assert_eq!(expected["windowKindId"], RETIREMENT_WINDOW_KIND);
    let window_id = expected["windowId"].as_str().expect("fixture window id");
    let operation = semio_framework_job::OperationId(2);
    let mut app = testkit::new_app::<RetirementApp>().await;
    let view = retirement_view();
    let mut authority = app.window_transient_store.capture(Some(&view)).expect("window transient capture").expect("registered window transient owner");
    let captured_generation = authority.generation;
    let mutation = |revision| WindowTransientMutation::of::<RetirementWindowTransientOwner>(window_id, ChangePublicationTransient { revision }.into());

    let mut rejected = app.window_transient_store.begin(operation, &authority, mutation(expected["capturedRevision"].as_u64().expect("captured revision"))).expect("captured authority admits its first publication");
    let mut superseding = app.window_transient_store.begin(operation, &authority, mutation(expected["supersedingRevision"].as_u64().expect("superseding revision"))).expect("superseding publication admitted");
    for _ in 0..4_096 {
        if matches!(app.window_transient_store.advance(superseding.as_mut(), grant).expect("superseding window transient advances"), store::ArtifactStoreOneItemAdvance::Published(_)) {
            assert!(superseding.acknowledge());
            break;
        }
    }
    superseding.begin_close();
    for _ in 0..4_096 {
        if superseding.close_step(grant).expect("superseding window transient retires") == store::SnapshotRetirementStep::Complete {
            break;
        }
    }
    assert!(superseding.terminal_is_empty());
    assert!(app.window_transient_store.advance(rejected.as_mut(), grant).is_err());

    let mut pending = PendingArtifactStorePublication::<RetirementApp>::WindowTransient(rejected);
    let row = fixture["lanes"].as_array().expect("fixture lanes").iter().find(|row| row["id"] == "windowTransient").expect("window transient row").clone();
    let turns = drive_rejected_retirement(&mut pending, grant, &row);
    assert!(pending.terminal_is_empty());

    // 🔁️ The authority captured at admission is now stale by construction, which is exactly why the
    // window-transient emission branch refreshes it before it begins.
    let stale = app.window_transient_store.begin(operation, &authority, mutation(expected["rejectedRevision"].as_u64().expect("rejected revision")));
    assert_eq!(stale.is_ok(), expected["staleAuthorityBeginAccepted"].as_bool().expect("stale begin expectation"));
    drop(stale);
    app.window_transient_store.refresh(&mut authority).expect("window transient authority refreshes onto the live generation");
    assert_eq!(authority.generation > captured_generation, expected["refreshedGenerationExceedsCaptured"].as_bool().expect("refreshed generation expectation"));
    let mut re_begun = app.window_transient_store.begin(operation, &authority, mutation(expected["reBeginRevision"].as_u64().expect("re-begin revision"))).expect("refreshed authority admits a new publication");
    assert!(expected["refreshedAuthorityBeginAccepted"].as_bool().expect("refreshed begin expectation"));
    for _ in 0..4_096 {
        if matches!(app.window_transient_store.advance(re_begun.as_mut(), grant).expect("re-begun window transient advances"), store::ArtifactStoreOneItemAdvance::Published(_)) {
            assert!(re_begun.acknowledge());
            break;
        }
    }
    re_begun.begin_close();
    for _ in 0..4_096 {
        if re_begun.close_step(grant).expect("re-begun window transient retires") == store::SnapshotRetirementStep::Complete {
            break;
        }
    }
    assert!(re_begun.terminal_is_empty());
    drop(authority);
    eprintln!("[DEBUG] window transient retired a rejected authority over {turns} Ok turns, refused a stale re-begin, and admitted the refreshed one at generation {}", app.window_transient_store.capture(Some(&view)).unwrap().unwrap().generation);
    close_registered_fixture_app(&mut app);
}
