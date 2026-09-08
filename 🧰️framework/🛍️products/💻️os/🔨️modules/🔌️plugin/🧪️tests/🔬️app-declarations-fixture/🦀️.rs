pub(crate) mod fixture {
    // 🎯️ Deliberately NOT `use super::super::*` (the `app` module glob `declarations` itself
    // uses) — that would re-import `app`'s OWN `ArtifactDeclaration` (old, debt D1) bare,
    // colliding with `declarations::ArtifactDeclaration` (new, this region) also brought in by
    // `use super::*` below: `error[E0659]: ArtifactDeclaration is ambiguous`. Explicit named
    // imports instead, for exactly what this fixture needs from `app`.
    use super::super::super::declaration_fixture_mutations::{std1_any as std1_any_mutations, std1_strict as std1_strict_mutations, std2_any as std2_any_mutations};
    use super::super::{
        AppDefinition, ArtifactDialect, ArtifactEditor, ArtifactKindId, ArtifactPack, ArtifactView, ArtifactViewer, ComponentTree, ConfigView, Dialect, DraftView, Editor, Emit, EngineHandles, Fault, IconName, InteractionView, LocalizedLabel,
        Mutation, MutationDiff, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, Plugin, StandardId, SubsetId, SurfaceKind, UiAssemblyResult, ViewEmit, Viewer, ViewModel, testkit,
    };
    use super::*;
    use serde::{Deserialize, Serialize};
    use std1_any_mutations::Std1AnyMutation;
    use std1_strict_mutations::Std1StrictMutation;
    use std2_any_mutations::Std2AnyMutation;

    //#region 🔖️Dialects
    pub(crate) const STD1_ANY_DIALECT: Dialect = Dialect { artifact_kind: "s.testkit.w1c-fixture", standard: StandardId("1"), subset: SubsetId::ANY };
    pub(crate) const STD1_STRICT_DIALECT: Dialect = Dialect { artifact_kind: "s.testkit.w1c-fixture", standard: StandardId("1"), subset: SubsetId("strict") };
    pub(crate) const STD2_ANY_DIALECT: Dialect = Dialect { artifact_kind: "s.testkit.w1c-fixture", standard: StandardId("2"), subset: SubsetId::ANY };
    //#endregion 🔖️Dialects

    //#region 🔖️FixtureChannel
    /// 🏗️ One self-contained subset's snapshot/diff/mutation/command/editor/viewer set — every
    /// subset owns its own types (design.md rule 2), even though the three fixture subsets are
    /// structurally identical (a real subset would differ in shape; only the WIRING this
    /// fixture proves is what matters here). Plain `serde_json` text/binary codecs (no hand-
    /// authored `.grammar.semio`/`.protocol.semio`) — `NativeCodecs.snapshot/diff/mutations`
    /// stay `LanguagePair { text: None, binary: None }`, which the type itself documents as legal.
    macro_rules! fixture_channel {
        ($snapshot:ident, $diff:ident, $mutation:ident, $leaf_owner:ident, $command:ident, $editor:ident, $viewer:ident, $dialect:expr, $schema:literal) => {
            #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
            pub(crate) struct $snapshot {
                pub value: i32,
            }
            impl store::ArtifactDsl for $snapshot {
                const EXTENSION: &'static str = $schema;
                fn parse_dsl(text: &str) -> Result<Self, store::TextError> {
                    if text.trim().is_empty() {
                        return Ok(Self::default());
                    }
                    serde_json::from_str(text).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
                }
                fn print_dsl(&self) -> String {
                    serde_json::to_string(self).unwrap_or_default()
                }
            }
            impl ArtifactPack for $snapshot {
                fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
                    serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()))
                }
                fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
                    if bytes.is_empty() {
                        return Ok(Self::default());
                    }
                    serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()))
                }
            }

            #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
            pub(crate) struct $diff {
                pub value: Option<i32>,
            }
            impl MutationDiff<$snapshot> for $diff {
                fn apply(&self, base: &$snapshot) -> protocol::MutationApplyResult<$snapshot> {
                    Ok($snapshot { value: self.value.unwrap_or(base.value) })
                }
                fn absorb(&mut self, other: Self) {
                    if other.value.is_some() {
                        self.value = other.value;
                    }
                }
            }

            impl protocol::OpText for $mutation {
                fn parse_op(line: &str) -> Result<Self, store::TextError> {
                    serde_json::from_str(line).map_err(|error| store::TextError::new(error.to_string(), store::TextSpan::at(1, 1)))
                }
                fn print_op(&self) -> String {
                    serde_json::to_string(self).unwrap_or_default()
                }
            }
            impl protocol::OpBinary for $mutation {
                fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
                    serde_json::to_vec(self).map_err(|error| store::PackError::Schema(error.to_string()).into())
                }
                fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
                    serde_json::from_slice(bytes).map_err(|error| store::PackError::Schema(error.to_string()).into())
                }
            }

            #[derive(Clone, Copy, Debug, Default, PartialEq)]
            pub(crate) enum $command {
                #[default]
                Noop,
            }
            impl protocol::OpBinary for $command {
                fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
                    Ok(Vec::new())
                }
                fn decode_op(_bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
                    Ok($command::Noop)
                }
            }

            #[derive(Default)]
            pub(crate) struct $editor;
            impl ArtifactEditor for $editor {
                const DIALECT: Dialect = $dialect;
                const DOCUMENT_SCHEMA: &'static str = $schema;
                type Snapshot = $snapshot;
                type Mutation = $mutation;
                type Config = NoConfig;
                type ConfigMutation = NoConfigMutation;
                type Draft = NoDraft;
                type DraftMutation = NoDraftMutation;
                type Presence = NoPresence;
                type PresenceMutation = NoPresenceMutation;
                type Transient = crate::app::NoTransient;
                type TransientMutation = crate::app::NoTransientMutation;
                type Command = $command;
                fn initial_snapshot() -> $snapshot {
                    $snapshot::default()
                }
                fn handle(_command: &$command, doc: &ArtifactView<'_, $snapshot>, _cfg: &ConfigView<'_, NoConfig>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _draft: &DraftView<'_, NoDraft>, _engines: &EngineHandles) -> ArtifactMutationOutcome<$mutation> {
                    Ok(Emit { artifact_mutations: vec![$mutation::SetValue($leaf_owner::SetValue { value: doc.snapshot.value })], ..Default::default() })
                }
                fn render(_body_key: &str, doc: &ArtifactView<'_, $snapshot>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
                    built_text_to_component_tree(ui_wgpu::wgpu::Label::data(format!("value={}", doc.snapshot.value)))
                }
            }

            #[derive(Default)]
            pub(crate) struct $viewer;
            impl ArtifactViewer for $viewer {
                const DIALECT: Dialect = $dialect;
                const DOCUMENT_SCHEMA: &'static str = $schema;
                type Snapshot = $snapshot;
                type Mutation = $mutation;
                type Config = NoConfig;
                type ConfigMutation = NoConfigMutation;
                type Presence = NoPresence;
                type PresenceMutation = NoPresenceMutation;
                type Transient = crate::app::NoTransient;
                type TransientMutation = crate::app::NoTransientMutation;
                type Command = $command;
                fn initial_snapshot() -> $snapshot {
                    $snapshot::default()
                }
                fn handle(_command: &$command, _doc: &ArtifactView<'_, $snapshot>, _cfg: &ConfigView<'_, NoConfig>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _engines: &EngineHandles) -> Result<ViewEmit<NoConfigMutation>, Fault> {
                    Ok(ViewEmit::default())
                }
                fn render(_body_key: &str, doc: &ArtifactView<'_, $snapshot>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
                    built_text_to_component_tree(ui_wgpu::wgpu::Label::data(format!("value={}", doc.snapshot.value)))
                }
            }
        };
    }

    fixture_channel!(Std1AnySnapshot, Std1AnyDiff, Std1AnyMutation, std1_any_mutations, Std1AnyCommand, Std1AnyEditor, Std1AnyViewer, STD1_ANY_DIALECT, "semio.testkit.w1c-fixture.std1-any/v1");
    fixture_channel!(Std1StrictSnapshot, Std1StrictDiff, Std1StrictMutation, std1_strict_mutations, Std1StrictCommand, Std1StrictEditor, Std1StrictViewer, STD1_STRICT_DIALECT, "semio.testkit.w1c-fixture.std1-strict/v1");
    fixture_channel!(Std2AnySnapshot, Std2AnyDiff, Std2AnyMutation, std2_any_mutations, Std2AnyCommand, Std2AnyEditor, Std2AnyViewer, STD2_ANY_DIALECT, "semio.testkit.w1c-fixture.std2-any/v1");
    //#endregion 🔖️FixtureChannel

    //#region 🔖️ProfileHop
    /// 🚪️ `standard 1 / subset strict` is a CONFORMANCE PROFILE of `standard 1 / subset any` —
    /// the one relationship design.md's io system models as an ordinary `IoEntry`, not a
    /// separate mechanism. `Deserializer::CONFORMANCE` rejects a negative `value` (this
    /// fixture's stand-in "profile" rule), proving §3's payload-law hop AND the W1-C
    /// conformance decision (`Deserializer::CONFORMANCE`, not `IoDeclaration.conformance`) in
    /// one exercise.
    pub(crate) struct StrictFromAny;
    impl semio_framework::io::io_mechanism::Deserializer<Std1StrictSnapshot> for StrictFromAny {
        const FROM: Dialect = STD1_ANY_DIALECT;
        const FIDELITY: semio_framework::io_schema::IoFidelity = semio_framework::io_schema::IoFidelity::Semantic;
        const CONFORMANCE: Option<fn(&Std1StrictSnapshot) -> Vec<dsl::Diagnostic>> = Some(check_non_negative);
        async fn deserialize(payload: &semio_framework::io_schema::IoPayload) -> semio_framework::io_schema::IoResult<Std1StrictSnapshot> {
            let semio_framework::io_schema::IoPayload::Binary(bytes) = payload else {
                return Err(semio_framework::io_schema::IoError { message: "StrictFromAny: expected a binary payload".to_string(), diagnostics: Vec::new() });
            };
            let base = Std1AnySnapshot::decode_pack(bytes).map_err(|error| semio_framework::io_schema::IoError { message: format!("StrictFromAny: base decode failed: {error}"), diagnostics: Vec::new() })?;
            Ok(semio_framework::io_schema::IoOutcome { value: Std1StrictSnapshot { value: base.value }, diagnostics: Vec::new() })
        }
    }

    // 🚫️async: E4 fn-pointer slot — `Deserializer::CONFORMANCE: Option<fn(&T) -> Vec<Diagnostic>>`.
    fn check_non_negative(snapshot: &Std1StrictSnapshot) -> Vec<dsl::Diagnostic> {
        if snapshot.value < 0 { vec![dsl::Diagnostic::error("s.testkit.w1c-fixture.negative-value", dsl::TextSpan::at(0, 0), "conformance profile requires a non-negative value")] } else { Vec::new() }
    }

    pub(crate) struct StrictIntoAny;
    impl semio_framework::io::io_mechanism::Serializer<Std1StrictSnapshot> for StrictIntoAny {
        const INTO: Dialect = STD1_ANY_DIALECT;
        const FIDELITY: semio_framework::io_schema::IoFidelity = semio_framework::io_schema::IoFidelity::Exact;
        async fn serialize(from: &Std1StrictSnapshot) -> semio_framework::io_schema::IoResult<semio_framework::io_schema::IoPayload> {
            Ok(semio_framework::io_schema::IoOutcome { value: semio_framework::io_schema::IoPayload::Binary(Std1AnySnapshot { value: from.value }.encode_pack()), diagnostics: Vec::new() })
        }
    }

    // 🚫️async: E1 pure fixture builder — `OnceLock::get_or_init`'s closure is a fixed sync
    // `FnOnce() -> T` (std API), so the entries inside are resolved via `resolve_ready`; both
    // never truly suspend (io-async-signatures).
    fn std1_strict_entries() -> &'static [semio_framework::io::io_mechanism::IoEntry] {
        static ENTRIES: std::sync::OnceLock<Vec<semio_framework::io::io_mechanism::IoEntry>> = std::sync::OnceLock::new();
        ENTRIES.get_or_init(|| {
            vec![semio_framework::io::io_mechanism::deserializer_entry::<Std1StrictSnapshot, StrictFromAny>(STD1_STRICT_DIALECT), semio_framework::io::io_mechanism::serializer_entry::<Std1StrictSnapshot, StrictIntoAny>(STD1_STRICT_DIALECT)]
        })
    }
    //#endregion 🔖️ProfileHop

    //#region 🔖️Builders
    fn schema_descriptor(id: &'static str) -> ::semio_framework_schema::ArtifactSchemaDescriptor {
        let leaves = ::semio_framework_schema::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "{}", proto: "" };
        ::semio_framework_schema::ArtifactSchemaDescriptor { id, artifact: leaves.clone(), snapshot: leaves.clone(), diff: leaves.clone(), mutations: leaves }
    }

    fn native_codecs<S, M>(schema: &str) -> NativeCodecs
    where
        S: Clone + PartialEq + protocol::ToValue + protocol::FromValue + Send + Sync + store::ArtifactDsl + ArtifactPack + 'static,
        M: Mutation<S> + PartialEq + protocol::ToValue + protocol::FromValue + Send + Sync + OpText + OpBinary + 'static,
    {
        NativeCodecs {
            snapshot: LanguagePair { text: None, binary: None },
            diff: LanguagePair { text: None, binary: None },
            mutations: LanguagePair { text: None, binary: None },
            inferences: None,
            codec: store::ArtifactCodec::of::<S, M>(schema.to_string()),
        }
    }

    /// 🏗️ Minimal valid `AppDefinition` — `.document(...)`/`.mode(...)`/`.window_kind(...)` are
    /// `AppBuilder::build_definition`'s hard asserts (non-empty document segments, ≥1 mode, ≥1
    /// window kind); every fixture surface shares this shape since only the WIRING matters here.
    fn editor_definition(dialect: Dialect) -> AppDefinition {
        Editor::builder(dialect)
            .document(["semio", "testkit", "w1c-fixture"])
            .mode("edit", LocalizedLabel::data("Edit"), "pencil")
            .window_kind("main", LocalizedLabel::data("Main"), "w1c.main", SurfaceKind::Canvas2d, IconName::AppWindow)
            .build_definition()
    }

    fn viewer_definition(dialect: Dialect) -> AppDefinition {
        Viewer::builder(dialect)
            .document(["semio", "testkit", "w1c-fixture"])
            .mode("view", LocalizedLabel::data("View"), "eye")
            .window_kind("main", LocalizedLabel::data("Main"), "w1c.main", SurfaceKind::Canvas2d, IconName::AppWindow)
            .build_definition()
    }

    semio_framework_dispatch_macros::dyn_enum_close! {
        /// 🗃️ Closed fixture app set for every declared editor and viewer surface.
        pub(crate) enum FixtureApps: PluginApp {
            Std1AnyEditorApp(VcsArtifactApp<EditorApp<Std1AnyEditor>>),
            Std1AnyViewerApp(VcsArtifactApp<ViewerApp<Std1AnyViewer>>),
            Std1StrictEditorApp(VcsArtifactApp<EditorApp<Std1StrictEditor>>),
            Std1StrictViewerApp(VcsArtifactApp<ViewerApp<Std1StrictViewer>>),
            Std2AnyEditorApp(VcsArtifactApp<EditorApp<Std2AnyEditor>>),
            Std2AnyViewerApp(VcsArtifactApp<ViewerApp<Std2AnyViewer>>),
        }
    }

    /// 🌳️ The fixture's whole tree: TWO standards, THREE subsets total — `standard "1"` owns
    /// `any` (base) and `strict` (conformance profile of `any`, wired via `std1_strict_entries`);
    /// `standard "2"` owns one independent `any` subset, proving the walk covers >1 standard.
    pub(crate) fn build_declaration() -> ArtifactDeclaration<FixtureApps> {
        ArtifactDeclaration {
            kind: ArtifactKindId::parse("s.testkit.w1c-fixture").expect("canonical fixture kind"),
            localization: &[],
            standards: vec![
                StandardDeclaration {
                    id: StandardId("1"),
                    media: MediaDeclaration { mimes: &["application/vnd.semio.w1c-fixture+json"], extensions: &["w1cfixture"] },
                    subsets: vec![
                        SubsetDeclaration {
                            dialect: STD1_ANY_DIALECT,
                            schema: SchemaDeclaration { descriptor: schema_descriptor("s.testkit.w1c-fixture@1/*"), inferences: &[], inference_services: Vec::new() },
                            io: IoDeclaration { native: native_codecs::<Std1AnySnapshot, Std1AnyMutation>("semio.testkit.w1c-fixture.std1-any/v1"), entries: &[] },
                            viewer: viewer_surface::<Std1AnyViewer, FixtureApps>(viewer_definition(STD1_ANY_DIALECT)),
                            editor: editor_surface::<Std1AnyEditor, FixtureApps>(editor_definition(STD1_ANY_DIALECT)),
                            examples: &[],
                        },
                        SubsetDeclaration {
                            dialect: STD1_STRICT_DIALECT,
                            schema: SchemaDeclaration { descriptor: schema_descriptor("s.testkit.w1c-fixture@1/strict"), inferences: &[], inference_services: Vec::new() },
                            io: IoDeclaration { native: native_codecs::<Std1StrictSnapshot, Std1StrictMutation>("semio.testkit.w1c-fixture.std1-strict/v1"), entries: std1_strict_entries() },
                            viewer: viewer_surface::<Std1StrictViewer, FixtureApps>(viewer_definition(STD1_STRICT_DIALECT)),
                            editor: editor_surface::<Std1StrictEditor, FixtureApps>(editor_definition(STD1_STRICT_DIALECT)),
                            examples: &[],
                        },
                    ],
                },
                StandardDeclaration {
                    id: StandardId("2"),
                    media: MediaDeclaration { mimes: &["application/vnd.semio.w1c-fixture-2+json"], extensions: &["w1cfixture2"] },
                    subsets: vec![SubsetDeclaration {
                        dialect: STD2_ANY_DIALECT,
                        schema: SchemaDeclaration { descriptor: schema_descriptor("s.testkit.w1c-fixture@2/*"), inferences: &[], inference_services: Vec::new() },
                        io: IoDeclaration { native: native_codecs::<Std2AnySnapshot, Std2AnyMutation>("semio.testkit.w1c-fixture.std2-any/v1"), entries: &[] },
                        viewer: viewer_surface::<Std2AnyViewer, FixtureApps>(viewer_definition(STD2_ANY_DIALECT)),
                        editor: editor_surface::<Std2AnyEditor, FixtureApps>(editor_definition(STD2_ANY_DIALECT)),
                        examples: &[],
                    }],
                },
            ],
        }
    }
    //#endregion 🔖️Builders

    //#region 🔖️Tests
    #[semio_framework_async_macros::async_test]
    async fn ids_are_derived_from_the_dialect() {
        testkit::assert_subset_declaration_ids_are_derived(&build_declaration()).await;
    }

    #[test]
    fn format_descriptor_identity_is_scoped_to_artifact_and_standard() {
        let artifact = ArtifactDeclaration::<FixtureApps> { kind: ArtifactKindId::parse("s.testkit.other").expect("canonical artifact kind"), localization: &[], standards: Vec::new() };
        let standard = StandardDeclaration::<FixtureApps> { id: StandardId("1"), media: MediaDeclaration { mimes: &["application/vnd.semio.testkit-other+json"], extensions: &["testkit-other"] }, subsets: Vec::new() };
        let descriptor = format_descriptor_of(&artifact, &standard);
        assert_eq!(descriptor.kind_id, "s.testkit.other@1");
        assert_eq!(descriptor.short_id, "s.testkit.other@1");
    }

    #[semio_framework_async_macros::async_test]
    async fn declaring_registers_schema_io_and_surfaces() {
        testkit::assert_declaration_tree_registers_all("testkit", build_declaration()).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn a_conflicting_declaration_leaves_zero_rows_behind() {
        let mut invalid = build_declaration();
        // 🎯️ Force a preflight failure by construction: standard "2"'s subset gets standard
        // "1"/"any"'s schema id, but with DIFFERENT facet content — `schema_descriptor` alone
        // would build byte-identical (harmlessly idempotent) descriptors for the same id, so
        // the `rust` leaf is perturbed to make the two genuinely conflict. Every OTHER field
        // (dialect, io, surfaces) stays standard "2"'s own — `preflight_artifact_schema_
        // descriptors`'s internal batch dedup rejects two DIFFERENT descriptors sharing one id
        // before anything commits.
        let mut conflicting = schema_descriptor("s.testkit.w1c-fixture@1/*");
        conflicting.artifact.rust = "// a different, conflicting facet body";
        invalid.standards[1].subsets[0].schema.descriptor = conflicting;
        testkit::assert_declaration_registration_is_atomic("testkit", invalid).await;
    }

    #[semio_framework_async_macros::async_test]
    async fn open_mutate_save_round_trips_through_the_generic_snapshot_builder() {
        type Construction = SnapshotBuilder<Std1AnySnapshot, Std1AnyMutation>;
        let bytes = Std1AnySnapshot { value: 1 }.encode_pack();
        let opened = Construction::from_binary(&bytes).expect("open");
        let (mutated, outcome) = opened.mutate(Std1AnyMutation::SetValue(std1_any_mutations::SetValue { value: 42 }));
        assert!(outcome.messages().is_empty(), "a fresh mutate must not fail");
        let saved = mutated.build().expect("save");
        assert_eq!(saved.value, 42);
    }

    #[semio_framework_async_macros::async_test]
    async fn io_route_finds_the_conformance_profile_hop() {
        let _plugin = Plugin::<FixtureApps>::builder("testkit").label("w1c-fixture-route").version("0.0.1").package_id("semio:testkit").declare_artifact(build_declaration()).try_build().expect("fixture declares cleanly");
        let route = semio_framework::io::io_mechanism::io_route(&ArtifactDialect::from(STD1_ANY_DIALECT), &ArtifactDialect::from(STD1_STRICT_DIALECT), 3).await.expect("a route from the base subset into its conformance profile must exist");
        assert_eq!(route.value.hops.len(), 1, "the profile hop is direct");

        let payload = semio_framework::io_schema::IoPayload::Binary(Std1AnySnapshot { value: 7 }.encode_pack());
        let result = semio_framework::io::io_mechanism::io_run(&route.value, payload).await.expect("running the route decodes into the profile");
        let semio_framework::io_schema::IoPayload::Binary(bytes) = result.value else {
            panic!("profile hop must produce a binary payload");
        };
        let profiled = Std1StrictSnapshot::decode_pack(&bytes).expect("profile snapshot decodes");
        assert_eq!(profiled.value, 7);
    }
    //#endregion 🔖️Tests
}
