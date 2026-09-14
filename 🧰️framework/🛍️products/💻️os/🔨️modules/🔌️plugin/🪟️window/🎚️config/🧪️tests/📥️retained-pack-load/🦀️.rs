//! 📥️ Current exact registry load/reopen baseline for the retained load lifecycle cutover.

use super::*;

fn block_on_retained_window_load<F: Future>(mut future: Pin<Box<F>>) -> F::Output {
    let waker = std::task::Waker::noop();
    let mut context = std::task::Context::from_waker(waker);
    loop {
        match future.as_mut().poll(&mut context) {
            std::task::Poll::Ready(output) => return output,
            std::task::Poll::Pending => std::thread::yield_now(),
        }
    }
}

fn run_retained_window_load_lane(name: &str, body: impl FnOnce() + Send + 'static) {
    std::thread::Builder::new().name(name.into()).stack_size(2 * 1024 * 1024).spawn(body).expect("spawn retained-load lane").join().expect("retained-load lane");
}

fn retained_load_fixture() -> serde_json::Value {
    serde_json::from_str(include_str!("../../🧫️fixtures/📥️retained-pack-load/🔣️.json")).expect("retained-load fixture")
}

/// 📷️ A real schema-first window camera: enveloped Pack, record spec, and a whole-record snapshot mutation.
#[derive(Clone, Debug, Default, PartialEq, dsl::DslArtifact, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
#[dsl(id = "test.retainedloadcameraconfig", layout = "lines")]
pub(super) struct RetainedLoadCameraConfig {
    #[dsl(block)]
    pub viewport: semio_framework_os_kernel::Viewport2d,
}

impl store::ArtifactDsl for RetainedLoadCameraConfig {
    const EXTENSION: &'static str = "retainedloadcameracfg";
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
        let body = store::semio_format::split_text_preamble(text).map_or(text, |(_, body)| body);
        let record = dsl::parse(body, &Self::__dsl_spec(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = dsl::print(&self.__dsl_to_record(), &Self::__dsl_spec(), dsl::JoinMode::Document);
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Dsl, 1).expect("valid retained-load camera envelope");
        store::semio_format::wrap_text(&envelope, &body)
    }
}

impl store::ArtifactPack for RetainedLoadCameraConfig {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        let body = store::pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = store::semio_format::SemioEnvelope::from_envelope_id(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1).map_err(|error| store::PackError::Schema(error.to_string()))?;
        Ok(store::semio_format::wrap_binary(&envelope, &body))
    }
    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let (envelope, body) = store::semio_format::unwrap_binary(bytes).map_err(|error| store::PackError::Schema(error.to_string()))?;
        if !envelope.matches_identity(<Self as store::ArtifactDsl>::envelope_id(), store::semio_format::Component::Pack, 1) {
            return Err(store::PackError::Schema("retained-load camera pack envelope mismatch".into()));
        }
        let (record, _) = store::pack_rt::decode_document(&body, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(store::text_error_to_pack_error)
    }
    fn record_spec() -> Option<dsl::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}

store::impl_whole_record_config!(RetainedLoadCameraConfig);

#[derive(Clone, Debug, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, dsl::DslOps)]
#[value(tag = "kind", rename_all = "kebab-case", rename_all_fields = "camelCase", deny_unknown_fields)]
pub(super) enum RetainedLoadCameraConfigMutation {
    #[dsl(key = "snapshot")]
    Snapshot {
        #[dsl(block)]
        config: Box<RetainedLoadCameraConfig>,
    },
}

impl protocol::OpText for RetainedLoadCameraConfigMutation {
    fn parse_op(line: &str) -> Result<Self, store::TextError> {
        for (keyword, spec_fn) in &<Self as dsl::DslVariants>::variants() {
            if line == keyword || line.starts_with(&format!("{keyword} ")) {
                let record = dsl::parse(line, &spec_fn(), &dsl::ParseOptions { limits: dsl::Limits::default(), mode: dsl::SourceMode::Inline })?;
                return <Self as dsl::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(dsl::__rt::field_error(format!("unknown retained-load camera mutation '{line}'")))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as dsl::DslVariants>::to_named_record(self);
        let spec = <Self as dsl::DslVariants>::variants().iter().find(|(key, _)| key == &keyword).map(|(_, spec)| spec()).expect("retained-load camera mutation variant");
        dsl::print(&record, &spec, dsl::JoinMode::Inline)
    }
}

impl protocol::OpBinary for RetainedLoadCameraConfigMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        dsl::variants_binary::decode_op(bytes)
    }
}

impl protocol::Mutation<RetainedLoadCameraConfig> for RetainedLoadCameraConfigMutation {
    type Diff = RetainedLoadCameraConfig;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[protocol::MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🧪️tests/📥️retained-pack-load",
        semantic_kind: "set-window-config",
        display_name: "Set Retained Load Camera Window Configuration",
        emoji: "📷️",
        aggregate_variant: "Snapshot",
        payload_schema: "test.retainedloadcameraconfig",
        text_opcode: None,
        binary_tag: None,
        invertibility: protocol::MutationInvertibility::ExplicitMutation,
        diff_participation: protocol::MutationDiffParticipation::Detect,
        outcome_classes: &[protocol::MutationOutcomeClass::Applied],
        composition: protocol::MutationComposition::Atomic,
        required_language_surfaces: &[protocol::MutationLanguageSurface::Rust],
    }];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor {
        &Self::DESCRIPTORS[0]
    }
    fn diff(&self, _base: &RetainedLoadCameraConfig) -> protocol::MutationOutcome<Self::Diff> {
        match self {
            Self::Snapshot { config } => protocol::MutationOutcome::new(config.as_ref().clone()),
        }
    }
    fn inverse(&self, base: &RetainedLoadCameraConfig) -> Vec<Self> {
        vec![Self::Snapshot { config: Box::new(base.clone()) }]
    }
}

pub(super) struct RetainedLoadOwnerA;
struct RetainedLoadOwnerB;

macro_rules! retained_load_owner {
    ($owner:ty, $kind:literal, $schema:literal) => {
        impl WindowConfigOwner for $owner {
            const WINDOW_KIND_ID: &'static str = $kind;
            const SCHEMA: &'static str = $schema;
            const MAXIMUM_PUBLICATION_BYTES: usize = 4_096;
            type State = RetainedLoadCameraConfig;
            type Mutation = RetainedLoadCameraConfigMutation;

            fn build_store_owners() -> store::DocumentStoreOwners<Self::State, Self::Mutation> {
                bounded_window_config_store_owners::<Self>()
            }

            fn build_one_item_preparation_factory() -> Arc<dyn store::ArtifactStoreOneItemPreparationFactory<Self::State, Self::Mutation>> {
                bounded_window_config_preparation_factory::<Self>()
            }

            fn build_store_disposer() -> Box<dyn ArtifactOwnedDisposer<store::ConfigStore<Self::State, Self::Mutation>>> {
                bounded_window_config_store_disposer::<Self>()
            }
        }
    };
}

retained_load_owner!(RetainedLoadOwnerA, "retained-load-a", "test.window.retained-load.a");
retained_load_owner!(RetainedLoadOwnerB, "retained-load-b", "test.window.retained-load.b");

fn register_retained_load_owners(registry: &mut WindowConfigOwnerRegistry) {
    registry.register::<RetainedLoadOwnerA>().expect("register retained-load A");
    registry.register::<RetainedLoadOwnerB>().expect("register retained-load B");
}

pub(super) fn close_retained_load_registry(registry: &mut WindowConfigOwnerRegistry) {
    for _ in 0..65_536 {
        match registry.close_step(1, store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES).expect("retained-load registry close") {
            PluginCloseStep::Complete if registry.terminal_is_empty() => return,
            PluginCloseStep::Pending { released_items, released_bytes } => {
                assert!(released_items <= 1);
                assert!(released_bytes <= store::ARTIFACT_ENVELOPE_DECODE_PAGE_BYTES);
            }
            PluginCloseStep::AwaitingInput { .. } | PluginCloseStep::Blocked { .. } | PluginCloseStep::Complete => {}
        }
    }
    panic!("retained-load registry did not reach terminal emptiness");
}

fn keyed_packs(packs: Vec<WindowConfigPack>) -> BTreeMap<(String, String), store::ArtifactPackFiles> {
    packs.into_iter().map(|pack| ((pack.window_kind_id, pack.window_id), pack.files)).collect()
}

fn saved_camera(fixture: &serde_json::Value) -> RetainedLoadCameraConfig {
    let viewport = &fixture["savedCamera"]["viewport"];
    RetainedLoadCameraConfig { viewport: semio_framework_os_kernel::Viewport2d { x: viewport["x"].as_f64().unwrap(), y: viewport["y"].as_f64().unwrap(), zoom: viewport["zoom"].as_f64().unwrap() } }
}

#[test]
fn window_config_retained_pack_load_current_registry_identity_and_reopen_baseline() {
    run_retained_window_load_lane("window-config-retained-load-baseline", || {
        let fixture = retained_load_fixture();
        assert_eq!(fixture["budgets"]["typedMutationBytes"], 4_096);
        assert_eq!(fixture["requiredScenarios"].as_array().expect("required scenarios").len(), 12);
        block_on_retained_window_load(Box::pin(async {
            let mut source = WindowConfigOwnerRegistry::default();
            register_retained_load_owners(&mut source);
            for (kind, id) in [("retained-load-a", "left"), ("retained-load-a", "right"), ("retained-load-b", "other")] {
                drop(source.owners.get_mut(kind).expect("registered source owner").capture(id).await.expect("materialize baseline source"));
            }
            let expected = source.packs().await.expect("source packs");
            assert_eq!(expected.len(), 3);
            let expected_bytes = keyed_packs(expected.iter().map(|pack| WindowConfigPack { window_id: pack.window_id.clone(), window_kind_id: pack.window_kind_id.clone(), files: pack.files.clone() }).collect());
            close_retained_load_registry(&mut source);

            let mut reopened = WindowConfigOwnerRegistry::default();
            register_retained_load_owners(&mut reopened);
            for pack in expected {
                reopened.load(pack).await.expect("current exact registry load");
            }
            let restored = reopened.packs().await.expect("reopened packs");
            let first = WindowConfigPack { window_id: "malformed-absent".into(), window_kind_id: restored[0].window_kind_id.clone(), files: store::ArtifactPackFiles { pack: vec![1], ..restored[0].files.clone() } };
            let restored_bytes = keyed_packs(restored);
            assert_eq!(restored_bytes, expected_bytes, "two same-kind and one cross-kind Pack+SPR identities survive a new registry lifetime");

            assert!(reopened.load(first).await.is_err(), "malformed state Pack is rejected");
            assert_eq!(keyed_packs(reopened.packs().await.expect("packs after malformed load")), restored_bytes, "malformed load leaves existing owners unchanged and the absent target unmaterialized");
            close_retained_load_registry(&mut reopened);
        }));
        eprintln!("[DEBUG] Window retained-load baseline: owners=2 packs=3 sameKind=2 crossKind=1 malformedAtomic=1 stackBytes=2097152");
    });
}

#[test]
fn window_config_retained_pack_load_round_trips_a_saved_camera_with_history() {
    run_retained_window_load_lane("window-config-retained-load-saved-camera", || {
        let fixture = retained_load_fixture();
        let (kind, window_id) = (fixture["savedCamera"]["kind"].as_str().unwrap().to_owned(), fixture["savedCamera"]["windowId"].as_str().unwrap().to_owned());
        assert_eq!(kind, RetainedLoadOwnerA::WINDOW_KIND_ID);
        let camera = saved_camera(&fixture);
        assert_ne!(camera, RetainedLoadCameraConfig::default());
        block_on_retained_window_load(Box::pin(async move {
            let mut source = WindowConfigOwnerRegistry::default();
            register_retained_load_owners(&mut source);
            let owner = source.owners.get_mut(kind.as_str()).expect("registered camera owner");
            drop(owner.capture(&window_id).await.expect("materialize camera partition"));
            owner.dispatch("camera-actor", WindowConfigMutation::of::<RetainedLoadOwnerA>(window_id.clone(), RetainedLoadCameraConfigMutation::Snapshot { config: Box::new(camera.clone()) }), None, None).await.expect("save camera");
            assert_eq!(source.snapshot(&kind, &window_id).expect("saved camera snapshot").get::<RetainedLoadOwnerA>(), Some(&camera));
            let saved = source.packs().await.expect("saved camera packs");
            assert!(!saved[0].files.spr.is_empty(), "the saved camera carries its history");
            let saved_bytes = keyed_packs(saved.iter().map(|pack| WindowConfigPack { window_id: pack.window_id.clone(), window_kind_id: pack.window_kind_id.clone(), files: pack.files.clone() }).collect());
            close_retained_load_registry(&mut source);

            let mut reopened = WindowConfigOwnerRegistry::default();
            register_retained_load_owners(&mut reopened);
            for pack in saved {
                reopened.load(pack).await.expect("saved camera load");
            }
            assert_eq!(reopened.snapshot(&kind, &window_id).expect("reopened camera snapshot").get::<RetainedLoadOwnerA>(), Some(&camera), "the reopened window reads its saved camera, not Default");
            assert_eq!(keyed_packs(reopened.packs().await.expect("reopened camera packs")), saved_bytes, "the saved camera Pack+SPR bytes survive reopen");
            close_retained_load_registry(&mut reopened);
        }));
        eprintln!("[DEBUG] Window retained-load saved camera: restored=true history=true");
    });
}

#[test]
fn window_config_retained_pack_load_refuses_exhausted_turn_bounds_and_over_bound_packs_with_typed_faults() {
    run_retained_window_load_lane("window-config-retained-load-bound", || {
        let fixture = retained_load_fixture();
        let camera = saved_camera(&fixture);
        block_on_retained_window_load(Box::pin(async move {
            let mut source = WindowConfigOwnerRegistry::default();
            register_retained_load_owners(&mut source);
            let owner = source.owners.get_mut(RetainedLoadOwnerA::WINDOW_KIND_ID).expect("registered camera owner");
            drop(owner.capture("left").await.expect("materialize camera partition"));
            owner.dispatch("camera-actor", WindowConfigMutation::of::<RetainedLoadOwnerA>("left", RetainedLoadCameraConfigMutation::Snapshot { config: Box::new(camera) }), None, None).await.expect("save camera");
            let saved = source.packs().await.expect("saved camera packs").remove(0);
            close_retained_load_registry(&mut source);

            let mut reopened = WindowConfigOwnerRegistry::default();
            register_retained_load_owners(&mut reopened);
            let copy = |pack: &WindowConfigPack| WindowConfigPack { window_id: pack.window_id.clone(), window_kind_id: pack.window_kind_id.clone(), files: pack.files.clone() };
            let exhausted = reopened.load_within(copy(&saved), 1).await.expect_err("a load that exhausts its turn bound must fault, never answer Ok");
            assert_eq!(exhausted.code.0, fixture["grant"]["exhaustionFault"].as_str().unwrap());
            assert!(reopened.snapshot(RetainedLoadOwnerA::WINDOW_KIND_ID, "left").is_none(), "an exhausted load installs nothing");
            assert_eq!(reopened.retiring.len(), 1, "an exhausted load that cannot retire within its bound is parked on the registry, never dropped");

            let mut over_bound = copy(&saved);
            over_bound.files.pack.resize(RetainedLoadOwnerA::MAXIMUM_PUBLICATION_BYTES + 1, 0);
            let refused = reopened.load(over_bound).await.expect_err("a Pack beyond the owner's schema bound is refused");
            assert_eq!(refused.code.0, fixture["grant"]["capacityFault"].as_str().unwrap());
            assert!(reopened.snapshot(RetainedLoadOwnerA::WINDOW_KIND_ID, "left").is_none(), "an over-bound load installs nothing");

            reopened.load(saved).await.expect("the same Pack loads under its full derived bound");
            assert!(reopened.snapshot(RetainedLoadOwnerA::WINDOW_KIND_ID, "left").is_some());
            close_retained_load_registry(&mut reopened);
        }));
        eprintln!("[DEBUG] Window retained-load bound: exhausted=typed overBound=typed retired=true");
    });
}
