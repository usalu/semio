pub(crate) mod fixture {
    // 🎯️ Deliberately NOT `use super::super::*` (the `app` module glob `declarations` itself
    // uses) — that would re-import `app`'s OWN `ArtifactDeclaration` (old, debt D1) bare,
    // colliding with `declarations::ArtifactDeclaration` (new, this region) also brought in by
    // `use super::*` below: `error[E0659]: ArtifactDeclaration is ambiguous`. Explicit named
    // imports instead, for exactly what this fixture needs from `app`.
    use super::super::super::declaration_fixture_mutations::{std1_any as std1_any_mutations, std1_strict as std1_strict_mutations, std2_any as std2_any_mutations};
    use semio_framework_2d::compute::EngineHandles;
    use super::super::{
        AppDefinition, ArtifactDialect, ArtifactEditor, ArtifactKindId, ArtifactPack, ArtifactView, ArtifactViewer, ComponentTree, ConfigView, Dialect, DraftView, Editor, Emit, Fault, IconName, InteractionView, LocalizedLabel,
        artifact_app_laws, Mutation, MutationDiff, NoConfig, NoConfigMutation, NoDraft, NoDraftMutation, NoPresence, NoPresenceMutation, Plugin, StandardId, SubsetId, SurfaceKind, UiAssemblyResult, ViewEmit, Viewer, ViewModel,
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
        ($snapshot:ident, $diff:ident, $mutation:ident, $leaf_owner:ident, $command:ident, $dialect:expr, $schema:literal) => {
            #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ::semio_framework_value_derive::ToValue, ::semio_framework_value_derive::FromValue)]
            pub(crate) struct $snapshot {
                pub value: i32,
            }
            impl semio_framework_schema_composition::ArtifactCompositionFields for $snapshot {
                fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
                    Ok(())
                }
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
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    fn sqlite_snapshot_codec() -> Option<store::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::ArtifactSqliteSnapshot>::sqlite_codec())
    }

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

            impl store::ArtifactSqliteSnapshot for $snapshot {
                const SQLITE_SCHEMA: &'static str = include_str!("../../../../../../🔨️modules/🚪️io/🧫️fixtures/🪶️sqlite-snapshot-registration/🗄️.sql");
                fn preflight_sqlite_snapshot_encoding(&self, _: store::sqlite_snapshot::SnapshotEncoding, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<(), String> {
                    let mut bound = store::sqlite_snapshot::artifact::NativeEncodingBound::new(control)?;
                    bound.add(64)?;
                    bound.finish()
                }

                fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework::io_schema::ArtifactDialect,_database:&store::sqlite_snapshot::SqliteDatabase,control:&mut store::sqlite_snapshot::SqliteSnapshotControl<'_>)->semio_framework::io_schema::IoResult<()>{
                    control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot,0,1)?;
                    if dialect.subset=="*"{return Ok(semio_framework::io_schema::IoOutcome::clean(()));}
                    if dialect!=&semio_framework::io_schema::ArtifactDialect::from($dialect)||dialect.subset!="strict"{return Err("fixture subset has no semantic validator".to_string().into());}
                    let diagnostics=if self.value<0{vec![dsl::Diagnostic{code:dsl::FaultCode::new(if self.value==i32::MIN{"fixture.strict.fatal-value"}else{"fixture.strict.negative-value"}),severity:if self.value==i32::MIN{dsl::Severity::Fatal}else{dsl::Severity::Error},span:dsl::TextSpan::at(1,1),message:"strict fixture requires a non-negative value".into(),expected:None,scope:dsl::FaultScope::default()}]}else if self.value==0{vec![dsl::Diagnostic{code:dsl::FaultCode::new("fixture.strict.zero-value"),severity:dsl::Severity::Warning,span:dsl::TextSpan::at(1,1),message:"strict fixture has no positive value".into(),expected:None,scope:dsl::FaultScope::default()}]}else{Vec::new()};
                    Ok(semio_framework::io_schema::IoOutcome{value:(),diagnostics})
                }

                fn to_sqlite_database(&self, control: &mut semio_framework::io::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<store::sqlite_snapshot::SqliteDatabase, String> {
                    control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?;
                    let mut database = store::sqlite_snapshot::SqliteDatabase::from_schema(Self::SQLITE_SCHEMA).map_err(|error| error.to_string())?;
                    database.table_mut("fixture_value")?.rows.push(store::sqlite_snapshot::SqliteRow { rowid: 1, values: vec![store::sqlite_snapshot::SqliteValue::Integer(1), store::sqlite_snapshot::SqliteValue::Integer(i64::from(self.value))] });
                    control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ProjectSnapshot, 1, 1)?;
                    Ok(database)
                }

                fn from_sqlite_database(database: &store::sqlite_snapshot::SqliteDatabase, control: &mut semio_framework::io::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, String> {
                    control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?;
                    let table = database.table("fixture_value")?;
                    if table.rows.len() != 1 || table.rows[0].rowid != 1 {
                        return Err("fixture SQLite snapshot requires one value row".to_string());
                    }
                    let [store::sqlite_snapshot::SqliteValue::Integer(1), store::sqlite_snapshot::SqliteValue::Integer(value)] = table.rows[0].values.as_slice() else {
                        return Err("fixture SQLite snapshot value columns are invalid".to_string());
                    };
                    let snapshot = Self { value: i32::try_from(*value).map_err(|error| error.to_string())? };
                    control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
                    Ok(snapshot)
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

        };
    }

    fixture_channel!(Std1AnySnapshot, Std1AnyDiff, Std1AnyMutation, std1_any_mutations, Std1AnyCommand, STD1_ANY_DIALECT, "semio.testkit.w1c-fixture.std1-any/v1");
    fixture_channel!(Std1StrictSnapshot, Std1StrictDiff, Std1StrictMutation, std1_strict_mutations, Std1StrictCommand, STD1_STRICT_DIALECT, "semio.testkit.w1c-fixture.std1-strict/v1");
    fixture_channel!(Std2AnySnapshot, Std2AnyDiff, Std2AnyMutation, std2_any_mutations, Std2AnyCommand, STD2_ANY_DIALECT, "semio.testkit.w1c-fixture.std2-any/v1");

    #[derive(Default)]
    pub(crate) struct Std1AnyEditor;
    impl ArtifactEditor for Std1AnyEditor {
        const DIALECT: Dialect = STD1_ANY_DIALECT;
        const DOCUMENT_SCHEMA: &'static str = "semio.testkit.w1c-fixture.std1-any/v1";
        type Snapshot = Std1AnySnapshot;
        type Mutation = Std1AnyMutation;
        type Config = NoConfig;
        type ConfigMutation = NoConfigMutation;
        type Draft = NoDraft;
        type DraftMutation = NoDraftMutation;
        type Presence = NoPresence;
        type PresenceMutation = NoPresenceMutation;
        type Transient = crate::app::NoTransient;
        type TransientMutation = crate::app::NoTransientMutation;
        type Command = Std1AnyCommand;
        fn initial_snapshot() -> Std1AnySnapshot { Std1AnySnapshot::default() }
        fn handle(_command: &Std1AnyCommand, doc: &ArtifactView<'_, Std1AnySnapshot>, _cfg: &ConfigView<'_, NoConfig>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _draft: &DraftView<'_, NoDraft>, _engines: &EngineHandles) -> ArtifactMutationOutcome<Std1AnyMutation> {
            Ok(Emit { artifact_mutations: vec![Std1AnyMutation::SetValue(std1_any_mutations::SetValue { value: doc.snapshot.value })], ..Default::default() })
        }
        fn render(_body_key: &str, doc: &ArtifactView<'_, Std1AnySnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
            built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("value={}", doc.snapshot.value)))
        }
    }

    #[derive(Default)]
    pub(crate) struct Std1AnyViewer;
    impl ArtifactViewer for Std1AnyViewer {
        const DIALECT: Dialect = STD1_ANY_DIALECT;
        const DOCUMENT_SCHEMA: &'static str = "semio.testkit.w1c-fixture.std1-any/v1";
        type Snapshot = Std1AnySnapshot;
        type Mutation = Std1AnyMutation;
        type Config = NoConfig;
        type ConfigMutation = NoConfigMutation;
        type Presence = NoPresence;
        type PresenceMutation = NoPresenceMutation;
        type Transient = crate::app::NoTransient;
        type TransientMutation = crate::app::NoTransientMutation;
        type Command = Std1AnyCommand;
        fn initial_snapshot() -> Std1AnySnapshot { Std1AnySnapshot::default() }
        fn handle(_command: &Std1AnyCommand, _doc: &ArtifactView<'_, Std1AnySnapshot>, _cfg: &ConfigView<'_, NoConfig>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _engines: &EngineHandles) -> Result<ViewEmit<NoConfigMutation>, Fault> {
            Ok(ViewEmit::default())
        }
        fn render(_body_key: &str, doc: &ArtifactView<'_, Std1AnySnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
            built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("value={}", doc.snapshot.value)))
        }
    }

    #[derive(Default)]
    pub(crate) struct Std1StrictEditor;
    impl ArtifactEditor for Std1StrictEditor {
        const DIALECT: Dialect = STD1_STRICT_DIALECT;
        const DOCUMENT_SCHEMA: &'static str = "semio.testkit.w1c-fixture.std1-strict/v1";
        type Snapshot = Std1StrictSnapshot;
        type Mutation = Std1StrictMutation;
        type Config = NoConfig;
        type ConfigMutation = NoConfigMutation;
        type Draft = NoDraft;
        type DraftMutation = NoDraftMutation;
        type Presence = NoPresence;
        type PresenceMutation = NoPresenceMutation;
        type Transient = crate::app::NoTransient;
        type TransientMutation = crate::app::NoTransientMutation;
        type Command = Std1StrictCommand;
        fn initial_snapshot() -> Std1StrictSnapshot { Std1StrictSnapshot::default() }
        fn handle(_command: &Std1StrictCommand, doc: &ArtifactView<'_, Std1StrictSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _draft: &DraftView<'_, NoDraft>, _engines: &EngineHandles) -> ArtifactMutationOutcome<Std1StrictMutation> {
            Ok(Emit { artifact_mutations: vec![Std1StrictMutation::SetValue(std1_strict_mutations::SetValue { value: doc.snapshot.value })], ..Default::default() })
        }
        fn render(_body_key: &str, doc: &ArtifactView<'_, Std1StrictSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
            built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("value={}", doc.snapshot.value)))
        }
    }

    #[derive(Default)]
    pub(crate) struct Std1StrictViewer;
    impl ArtifactViewer for Std1StrictViewer {
        const DIALECT: Dialect = STD1_STRICT_DIALECT;
        const DOCUMENT_SCHEMA: &'static str = "semio.testkit.w1c-fixture.std1-strict/v1";
        type Snapshot = Std1StrictSnapshot;
        type Mutation = Std1StrictMutation;
        type Config = NoConfig;
        type ConfigMutation = NoConfigMutation;
        type Presence = NoPresence;
        type PresenceMutation = NoPresenceMutation;
        type Transient = crate::app::NoTransient;
        type TransientMutation = crate::app::NoTransientMutation;
        type Command = Std1StrictCommand;
        fn initial_snapshot() -> Std1StrictSnapshot { Std1StrictSnapshot::default() }
        fn handle(_command: &Std1StrictCommand, _doc: &ArtifactView<'_, Std1StrictSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _engines: &EngineHandles) -> Result<ViewEmit<NoConfigMutation>, Fault> {
            Ok(ViewEmit::default())
        }
        fn render(_body_key: &str, doc: &ArtifactView<'_, Std1StrictSnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
            built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("value={}", doc.snapshot.value)))
        }
    }

    #[derive(Default)]
    pub(crate) struct Std2AnyEditor;
    impl ArtifactEditor for Std2AnyEditor {
        const DIALECT: Dialect = STD2_ANY_DIALECT;
        const DOCUMENT_SCHEMA: &'static str = "semio.testkit.w1c-fixture.std2-any/v1";
        type Snapshot = Std2AnySnapshot;
        type Mutation = Std2AnyMutation;
        type Config = NoConfig;
        type ConfigMutation = NoConfigMutation;
        type Draft = NoDraft;
        type DraftMutation = NoDraftMutation;
        type Presence = NoPresence;
        type PresenceMutation = NoPresenceMutation;
        type Transient = crate::app::NoTransient;
        type TransientMutation = crate::app::NoTransientMutation;
        type Command = Std2AnyCommand;
        fn initial_snapshot() -> Std2AnySnapshot { Std2AnySnapshot::default() }
        fn handle(_command: &Std2AnyCommand, doc: &ArtifactView<'_, Std2AnySnapshot>, _cfg: &ConfigView<'_, NoConfig>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _draft: &DraftView<'_, NoDraft>, _engines: &EngineHandles) -> ArtifactMutationOutcome<Std2AnyMutation> {
            Ok(Emit { artifact_mutations: vec![Std2AnyMutation::SetValue(std2_any_mutations::SetValue { value: doc.snapshot.value })], ..Default::default() })
        }
        fn render(_body_key: &str, doc: &ArtifactView<'_, Std2AnySnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
            built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("value={}", doc.snapshot.value)))
        }
    }

    #[derive(Default)]
    pub(crate) struct Std2AnyViewer;
    impl ArtifactViewer for Std2AnyViewer {
        const DIALECT: Dialect = STD2_ANY_DIALECT;
        const DOCUMENT_SCHEMA: &'static str = "semio.testkit.w1c-fixture.std2-any/v1";
        type Snapshot = Std2AnySnapshot;
        type Mutation = Std2AnyMutation;
        type Config = NoConfig;
        type ConfigMutation = NoConfigMutation;
        type Presence = NoPresence;
        type PresenceMutation = NoPresenceMutation;
        type Transient = crate::app::NoTransient;
        type TransientMutation = crate::app::NoTransientMutation;
        type Command = Std2AnyCommand;
        fn initial_snapshot() -> Std2AnySnapshot { Std2AnySnapshot::default() }
        fn handle(_command: &Std2AnyCommand, _doc: &ArtifactView<'_, Std2AnySnapshot>, _cfg: &ConfigView<'_, NoConfig>, _interaction: &InteractionView<'_>, _view_state: Option<&ViewModel>, _engines: &EngineHandles) -> Result<ViewEmit<NoConfigMutation>, Fault> {
            Ok(ViewEmit::default())
        }
        fn render(_body_key: &str, doc: &ArtifactView<'_, Std2AnySnapshot>, _cfg: &ConfigView<'_, NoConfig>, _view_state: &ViewModel) -> UiAssemblyResult<ComponentTree> {
            built_text_to_component_tree(semio_framework_ui_locale::Label::data(format!("value={}", doc.snapshot.value)))
        }
    }
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
    fn schema_descriptor(id: &'static str) -> ::semio_framework_schema_registry::ArtifactSchemaDescriptor {
        let leaves = ::semio_framework_schema_registry::FacetLeaves { rust: "", typescript: "", graphql: "", json_schema: "{}", proto: "" };
        ::semio_framework_schema_registry::ArtifactSchemaDescriptor { id, artifact: leaves.clone(), snapshot: leaves.clone(), diff: leaves.clone(), mutations: leaves }
    }

    fn native_codecs<S, M>(schema: &str) -> NativeCodecs
    where
        S: Clone + PartialEq + protocol::ToValue + protocol::FromValue + Send + Sync + store::ArtifactDsl + ArtifactPack + store::ArtifactSqliteSnapshot + 'static,
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
        artifact_app_laws::assert_subset_declaration_ids_are_derived(&build_declaration()).await;
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
        artifact_app_laws::assert_declaration_tree_registers_all("testkit", build_declaration()).await;
    }

    #[test]
    fn every_declared_surface_names_the_schema_it_opens() {
        let projected = project_artifact_declarations(&[build_declaration()]);
        let opened: Vec<(AppRole, &str)> = projected.app_defs.iter().map(|(app, _)| (app.definition.role, app.definition.io.artifact_schema.as_str())).collect();
        let expected: Vec<(AppRole, &str)> = ["semio.testkit.w1c-fixture.std1-any/v1", "semio.testkit.w1c-fixture.std1-strict/v1", "semio.testkit.w1c-fixture.std2-any/v1"].into_iter().flat_map(|schema| [(AppRole::Editor, schema), (AppRole::Viewer, schema)]).collect();
        assert_eq!(opened, expected);
    }

    #[test]
    fn subset_examples_reach_the_editor_catalogue_without_an_editor_method() {
        let mut declaration = build_declaration();
        let leaked: &'static [ExampleSource] = Box::leak(vec![ExampleSource::new("demo", LocalizedLabel::data("Demo"), "semio fixture.dsl v1", "file")].into_boxed_slice());
        declaration.standards[0].subsets[0].examples = leaked;
        let projected = project_artifact_declarations(std::slice::from_ref(&declaration));
        let (app, _) = projected.app_defs.iter().find(|(app, _)| app.definition.role == AppRole::Editor && app.examples.iter().any(|example| example.id() == "demo")).expect("editor row");
        assert_eq!(app.examples[0].document(), "semio fixture.dsl v1");
        let body = EditorApp::<Std1AnyEditor>::catalogue_example_document("demo").expect("catalogue").expect("body");
        assert_eq!(body, "semio fixture.dsl v1");
        assert!(EditorApp::<Std1AnyEditor>::catalogue_example_document("missing").expect("catalogue").is_err());
    }

    /// 🪪️ A codec call constructs no app: every registered app records the document schema and the codec answers of
    /// its own type, each schema's owners are exactly its own editor and viewer (two of the fixture's six apps), the
    /// one answering is the editor, and a bundle whose every app factory refuses to run answers every codec call of
    /// every owned schema; a schema nobody owns is refused.
    #[semio_framework_async_macros::async_test]
    async fn codec_calls_construct_no_app() {
        fn refuse_construction(_definition: &AppDefinition) -> FixtureApps {
            panic!("a codec call constructed an app")
        }
        let projected = project_artifact_declarations(&[build_declaration()]);
        let mut plugin = crate::app::Plugin::<FixtureApps>::new("testkit", "Testkit", "1.0.0");
        for (app, mut factory) in projected.app_defs {
            assert_eq!(factory.document_schema, app.definition.io.artifact_schema, "{}", app.definition.id);
            factory.create = refuse_construction;
            plugin = plugin.register_app_factory(app, factory);
        }
        let owners: Vec<(String, &'static str)> = plugin.manifest.apps.iter().map(|definition| (definition.id.clone(), plugin.app_document_schema(&definition.id).expect("recorded schema"))).collect();
        assert_eq!(owners.len(), 6);
        let schemas = ["semio.testkit.w1c-fixture.std1-any/v1", "semio.testkit.w1c-fixture.std1-strict/v1", "semio.testkit.w1c-fixture.std2-any/v1"];
        for schema in schemas {
            let expected: Vec<&str> = owners.iter().filter(|(_, owned)| *owned == schema).map(|(id, _)| id.as_str()).collect();
            assert_eq!(expected.len(), 2, "{schema}: its editor and its viewer");
            let candidates: Vec<&str> = crate::plugin_runtime::artifact_codec_candidates(&plugin, schema).map(|definition| definition.id.as_str()).collect();
            assert_eq!(candidates, expected, "{schema}");
            let owner = crate::plugin_runtime::artifact_codec_owner(&plugin, schema).expect("one owner");
            assert_eq!(owner.role, AppRole::Editor, "{schema}: the editor is the creation authority");
        }
        assert!(crate::plugin_runtime::artifact_codec_owner(&plugin, "semio.testkit.nobody/v1").is_err(), "no owner is refused");
        let runtime = crate::plugin_runtime::PluginRuntime::new();
        crate::plugin_runtime::install_plugin_bundle(&runtime, plugin);
        for schema in schemas {
            let hash = crate::plugin_runtime::plugin_artifact_pack_schema_hash(&runtime, schema).await;
            assert!(hash.as_ref().is_ok_and(|hash| *hash != [0; 32]) || format!("{hash:?}").contains("no structural record specification"), "{schema}: {hash:?}");
            let document_id = format!("artifact-{}", "1".repeat(32));
            let pair = crate::plugin_runtime::plugin_artifact_genesis(&runtime, schema, &document_id).await.expect("genesis without an app");
            let mirror = crate::plugin_runtime::plugin_artifact_print_mirror(&runtime, schema, &pair.pack, &pair.spr).await.expect("print mirror without an app");
            assert!(!pair.pack.is_empty() && !mirror.dsl.is_empty(), "{schema}");
        }
        assert!(crate::plugin_runtime::plugin_artifact_pack_schema_hash(&runtime, "semio.testkit.nobody/v1").await.is_err(), "no owner is refused");
    }

    /// 🎯️ Force a preflight failure by construction: standard "2"'s subset gets standard
    /// "1"/"any"'s schema id, but with DIFFERENT facet content — `schema_descriptor` alone
    /// would build byte-identical (harmlessly idempotent) descriptors for the same id, so
    /// the `rust` leaf is perturbed to make the two genuinely conflict. Every OTHER field
    /// (dialect, io, surfaces) stays standard "2"'s own — `preflight_artifact_schema_
    /// descriptors`'s internal batch dedup rejects two DIFFERENT descriptors sharing one id
    /// before anything commits.
    #[semio_framework_async_macros::async_test]
    async fn a_conflicting_declaration_leaves_zero_rows_behind() {
        let mut invalid = build_declaration();
        let mut conflicting = schema_descriptor("s.testkit.w1c-fixture@1/*");
        conflicting.artifact.rust = "// a different, conflicting facet body";
        invalid.standards[1].subsets[0].schema.descriptor = conflicting;
        artifact_app_laws::assert_declaration_registration_is_atomic("testkit", invalid).await;
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

    fn sqlite_fixture_strict_validate(payload: &semio_framework::io_schema::IoPayload) -> Vec<dsl::Diagnostic> {
        let decoded = match payload { semio_framework::io_schema::IoPayload::Binary(bytes) => Std1StrictSnapshot::decode_pack(bytes).map_err(|error| error.to_string()), semio_framework::io_schema::IoPayload::Text(text) => <Std1StrictSnapshot as store::ArtifactDsl>::parse_dsl(text).map_err(|error| error.to_string()) };
        if decoded.as_ref().is_ok_and(|snapshot| snapshot.value >= 0) { return Vec::new(); }
        let fatal=decoded.as_ref().is_ok_and(|snapshot|snapshot.value==i32::MIN);
        vec![dsl::Diagnostic { code: dsl::FaultCode::new(if fatal{"fixture.strict.fatal-value"}else{"fixture.strict.negative-value"}), severity: if fatal{dsl::Severity::Fatal}else{dsl::Severity::Error}, span: dsl::TextSpan::at(1, 1), message: "strict fixture requires a non-negative value".into(), expected: None, scope: dsl::FaultScope::default() }]
    }

    static SQLITE_FIXTURE_STRICT_VALIDATOR: semio_framework::io::SubsetValidatorEntry = semio_framework::io::SubsetValidatorEntry { dialect: STD1_STRICT_DIALECT, validate: sqlite_fixture_strict_validate };

    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_covers_every_declared_subset() {
        use semio_framework::io::io_mechanism::{NativeSnapshotRegistration, io_entries, io_identify, io_route, io_run, io_run_with_snapshot_control, preflight_native_snapshots};
        use semio_framework::io::sqlite_snapshot::{SqliteDatabaseLimits, SqliteSnapshotPhase, export_sqlite_database, import_sqlite_database};
        use semio_framework::io_schema::{Confidence, IoFidelity, IoPayload, IoRoute, SQLITE_SNAPSHOT};
        semio_framework::io::register_subset_validator(&SQLITE_FIXTURE_STRICT_VALIDATOR).expect("strict fixture conformance registration");
        let _plugin = Plugin::<FixtureApps>::builder("testkit").label("sqlite-fixture").version("0.0.1").package_id("semio:testkit").declare_artifact(build_declaration()).try_build().expect("fixture assembly");
        let sqlite = ArtifactDialect::from(SQLITE_SNAPSHOT);
        let format = semio_framework::io::format_descriptor("sqlite").expect("format registry").expect("framework SQLite file endpoint");
        assert_eq!(format.kind_id, sqlite.artifact_kind);
        assert_eq!(semio_framework::io::format_accept_filter(&["sqlite"]).expect("SQLite file selector"), ".sqlite");
        let corpus: serde_json::Value = serde_json::from_str(include_str!("../../../../../../🔨️modules/🚪️io/🧫️fixtures/🪶️sqlite-snapshot-registration/🔣️.json")).expect("language-neutral registration corpus");
        assert_eq!(sqlite.to_coordinate(), corpus["sqliteDialect"].as_str().expect("SQLite dialect"));
        let rows = [
            (STD1_ANY_DIALECT, Std1AnySnapshot { value: 7 }.encode_pack(), "{\"value\":7}"),
            (STD1_STRICT_DIALECT, Std1StrictSnapshot { value: 8 }.encode_pack(), "{\"value\":8}"),
            (STD2_ANY_DIALECT, Std2AnySnapshot { value: 9 }.encode_pack(), "{\"value\":9}"),
        ];
        let entries = io_entries();
        for ((dialect, binary, text), fixture) in rows.into_iter().zip(corpus["nativeSnapshots"].as_array().expect("native snapshot fixtures")) {
            let dialect = ArtifactDialect::from(dialect);
            assert_eq!(dialect.to_coordinate(), fixture["dialect"].as_str().expect("native dialect"));
            assert_eq!(text, fixture["snapshot"].as_str().expect("native snapshot"));
            assert!(entries.iter().any(|entry| entry.from == dialect && entry.into == sqlite && entry.fidelity == IoFidelity::Exact));
            assert!(entries.iter().any(|entry| entry.from == sqlite && entry.into == dialect && entry.fidelity == IoFidelity::Exact));
            let export = io_route(&dialect, &sqlite, 1).await.expect("SQLite export route").value;
            let import = io_route(&sqlite, &dialect, 1).await.expect("SQLite import route").value;
            for payload in [IoPayload::Binary(binary), IoPayload::Text(text.to_string())] {
                let exported = io_run(&export, payload.clone()).await.expect("native export").value;
                let IoPayload::Binary(bytes) = &exported else { panic!("SQLite file is binary") };
                let database = import_sqlite_database(bytes, SqliteDatabaseLimits::default(), &mut |_| true).expect("semantic SQLite database");
                assert_eq!(semio_framework::io::io_mechanism::sqlite_snapshot_metadata(&database).expect("snapshot metadata").0, dialect);
                assert_eq!(database.table("fixture_value").expect("semantic value table").rows[0].values[1], semio_framework::io::sqlite_snapshot::SqliteValue::Integer(serde_json::from_str::<serde_json::Value>(text).expect("independent snapshot oracle")["value"].as_i64().expect("value")));
                assert_eq!(io_identify(&exported).await, vec![(sqlite.clone(), Confidence::High)]);
                let mut cancellation_phases = Vec::new();
                assert!(io_run_with_snapshot_control(&export, payload.clone(), SqliteDatabaseLimits::default(), &mut |progress| { cancellation_phases.push(progress.phase); progress.phase != SqliteSnapshotPhase::ProjectSnapshot }).await.is_err());
                assert!(!cancellation_phases.contains(&SqliteSnapshotPhase::WritePages));
                cancellation_phases.clear();
                assert!(io_run_with_snapshot_control(&import, exported.clone(), SqliteDatabaseLimits::default(), &mut |progress| { cancellation_phases.push(progress.phase); progress.phase != SqliteSnapshotPhase::ReconstructSnapshot }).await.is_err());
                assert!(!cancellation_phases.contains(&SqliteSnapshotPhase::EncodeNative));
                if dialect == ArtifactDialect::from(STD1_STRICT_DIALECT) {
                    let mut nonconforming = database.clone();
                    nonconforming.table_mut("fixture_value").expect("value table").rows[0].values[1] = semio_framework::sqlite_snapshot::SqliteValue::Integer(-1);
                    let nonconforming = export_sqlite_database(&nonconforming, SqliteDatabaseLimits::default(), &mut |_| true).expect("valid relational scalar");
                    assert!(io_run(&import, IoPayload::Binary(nonconforming)).await.is_err());
                }
                let mut invalid = database.clone();
                invalid.table_mut("fixture_value").expect("value table").sql = "CREATE TABLE fixture_value (id INTEGER PRIMARY KEY, opaque BLOB)".to_string();
                invalid.table_mut("fixture_value").expect("value table").rows.clear();
                let invalid = export_sqlite_database(&invalid, SqliteDatabaseLimits::default(), &mut |_| true).expect("physical SQLite may carry another declared schema");
                assert!(io_run(&import, IoPayload::Binary(invalid)).await.is_err());
                assert_eq!(io_run(&import, exported.clone()).await.expect("native import").value, payload);
                let wrong = if dialect == ArtifactDialect::from(STD1_ANY_DIALECT) { STD2_ANY_DIALECT } else { STD1_ANY_DIALECT };
                let wrong_import = io_route(&sqlite, &ArtifactDialect::from(wrong), 1).await.expect("other import route").value;
                assert!(io_run(&wrong_import, exported).await.is_err());
                assert!(io_run_with_snapshot_control(&export, payload, SqliteDatabaseLimits::default(), &mut |_| false).await.is_err());
            }
            assert!(io_run(&export, IoPayload::Binary(vec![0, 1, 2])).await.is_err());
            assert!(io_route(&dialect, &sqlite, 0).await.is_err());
            let bridge = IoRoute { hops: vec![export.hops[0].clone(), io_route(&sqlite, &ArtifactDialect::from(STD2_ANY_DIALECT), 1).await.expect("import").value.hops[0].clone()], fidelity: IoFidelity::Exact };
            assert!(io_run(&bridge, IoPayload::Text(text.to_string())).await.is_err());
            let disconnected = IoRoute { hops: vec![export.hops[0].clone(), export.hops[0].clone()], fidelity: IoFidelity::Exact };
            assert!(io_run(&disconnected, IoPayload::Text(text.to_string())).await.is_err());
        }
        let strict_export = io_route(&ArtifactDialect::from(STD1_STRICT_DIALECT), &sqlite, 1).await.expect("strict export route").value;
        assert!(io_run(&strict_export, IoPayload::Text("{\"value\":-1}".into())).await.is_err());
        let fatal=io_run(&strict_export,IoPayload::Binary(Std1StrictSnapshot{value:i32::MIN}.encode_pack())).await.unwrap_err();assert_eq!(fatal.diagnostics[0].severity,dsl::Severity::Fatal);
        let strict_import = io_route(&sqlite, &ArtifactDialect::from(STD1_STRICT_DIALECT), 1).await.unwrap().value;
        for payload in [IoPayload::Binary(Std1StrictSnapshot { value: 0 }.encode_pack()), IoPayload::Text("{\"value\":0}".into())] {
            let exported = io_run(&strict_export, payload.clone()).await.unwrap();
            assert_eq!(exported.diagnostics.len(), 1);
            assert_eq!(exported.diagnostics[0].severity, dsl::Severity::Warning);
            let restored = io_run(&strict_import, exported.value).await.unwrap();
            assert_eq!(restored.value, payload);
            assert_eq!(restored.diagnostics, exported.diagnostics);
        }
        let snapshot = Std1StrictSnapshot { value: 1 };
        let provider = <Std1StrictSnapshot as store::ArtifactSqliteSnapshot>::sqlite_codec();
        let mut progress = |_| true;
        let mut control = semio_framework::sqlite_snapshot::SqliteSnapshotControl::new(&mut progress, SqliteDatabaseLimits::default());
        let database = <Std1StrictSnapshot as store::ArtifactSqliteSnapshot>::to_sqlite_database(&snapshot, &mut control).unwrap();
        let foreign = ArtifactDialect::from(STD2_ANY_DIALECT);
        let mut foreign_strict = foreign.clone();
        foreign_strict.subset = "strict".into();
        assert!((provider.export)("fixture.foreign/v1", &foreign_strict, &IoPayload::Binary(snapshot.encode_pack()), &mut control).is_err());
        assert!((provider.import)("fixture.foreign/v1", &foreign_strict, database, semio_framework::sqlite_snapshot::SnapshotEncoding::Binary, &mut control).is_err());

        let mut missing_provider = store::ArtifactCodec::of::<Std1AnySnapshot, Std1AnyMutation>("semio.testkit.w1c-fixture.std1-any/v1");
        missing_provider.snapshot_sqlite = None;
        assert!(preflight_native_snapshots(&[NativeSnapshotRegistration { dialect: STD1_ANY_DIALECT.into(), codec: missing_provider }]).is_err());
        let conflicting = store::ArtifactCodec::of::<Std2AnySnapshot, Std2AnyMutation>("conflicting.snapshot.owner");
        assert!(preflight_native_snapshots(&[NativeSnapshotRegistration { dialect: STD1_ANY_DIALECT.into(), codec: conflicting }]).is_err());
        let alias = store::ArtifactCodec::of::<Std1AnySnapshot, Std1AnyMutation>("foreign.schema.alias");
        assert!(preflight_native_snapshots(&[NativeSnapshotRegistration { dialect: STD1_ANY_DIALECT.into(), codec: alias }]).is_ok());
    }

    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_owned_typed_io_bypasses_native_and_checks_type_identity() {
        use semio_framework::io::io_mechanism::{io_export_sqlite_snapshot, io_import_sqlite_snapshot, NativeSnapshotRegistration};
        use semio_framework::sqlite_snapshot::{SnapshotEncoding, SqliteDatabaseLimits, SqliteSnapshotPhase};
        const TYPED: Dialect = Dialect { artifact_kind: "s.testkit.typed-fixture", standard: StandardId("1"), subset: SubsetId("*") };
        let codec = store::ArtifactCodec::of::<Std1AnySnapshot, Std1AnyMutation>("semio.testkit.typed-fixture/v1");
        let assembly = store::begin_artifact_assembly().unwrap();
        semio_framework::io::commit_artifact_assembly_registry_plan(&assembly, semio_framework::io::ArtifactAssemblyRegistryPlan { document_codecs: vec![codec.clone()], native_snapshots: vec![NativeSnapshotRegistration { dialect: TYPED.into(), codec }], ..Default::default() }).unwrap();
        drop(assembly);
        let snapshot = Std1AnySnapshot { value: 42 };
        let limits = SqliteDatabaseLimits::default();
        let mut phases = Vec::new();
        let file = io_export_sqlite_snapshot(&TYPED.into(), &snapshot, SnapshotEncoding::Binary, limits, &mut |event| { phases.push(event.phase); true }).await.unwrap().value;
        let restored = io_import_sqlite_snapshot::<Std1AnySnapshot>(&TYPED.into(), &file, limits, &mut |event| { phases.push(event.phase); true }).await.unwrap().value;
        assert_eq!(restored, snapshot);
        assert!(!phases.contains(&SqliteSnapshotPhase::DecodeNative));
        assert!(!phases.contains(&SqliteSnapshotPhase::EncodeNative));
        assert!(io_import_sqlite_snapshot::<Std2AnySnapshot>(&TYPED.into(), &file, limits, &mut |_| true).await.is_err());
        assert!(io_export_sqlite_snapshot(&TYPED.into(), &Std2AnySnapshot { value: 42 }, SnapshotEncoding::Binary, limits, &mut |_| true).await.is_err());
        assert!(io_import_sqlite_snapshot::<Std1AnySnapshot>(&STD1_ANY_DIALECT.into(), &file, limits, &mut |_| true).await.is_err());
        assert!(io_export_sqlite_snapshot(&TYPED.into(), &snapshot, SnapshotEncoding::Binary, limits, &mut |_| false).await.is_err());
        let _plugin=Plugin::<FixtureApps>::builder("testkit").label("typed strict fixture").version("0.0.1").package_id("semio:testkit").declare_artifact(build_declaration()).try_build().unwrap();
        let cases:serde_json::Value=serde_json::from_str(include_str!("../../../../../../🔨️modules/🚪️io/🧫️fixtures/🪶️sqlite-snapshot-registration/🔣️.json")).unwrap();let cases=&cases["ownedSubsetValidation"];
        let mut strict_phases=Vec::new();let outcome=io_export_sqlite_snapshot(&STD1_STRICT_DIALECT.into(),&Std1StrictSnapshot{value:i32::try_from(cases["warning"]["value"].as_i64().unwrap()).unwrap()},SnapshotEncoding::Binary,limits,&mut |event|{strict_phases.push(event.phase);true}).await.unwrap();assert_eq!(outcome.diagnostics.len(),1);assert_eq!(outcome.diagnostics[0].severity,dsl::Severity::Warning);
        let restored=io_import_sqlite_snapshot::<Std1StrictSnapshot>(&STD1_STRICT_DIALECT.into(),&outcome.value,limits,&mut |event|{strict_phases.push(event.phase);true}).await.unwrap();assert_eq!(restored.value.value,0);assert_eq!(restored.diagnostics,outcome.diagnostics);assert!(!strict_phases.iter().any(|phase|matches!(phase,SqliteSnapshotPhase::EncodeNative|SqliteSnapshotPhase::DecodeNative)));
        let rejected=io_export_sqlite_snapshot(&STD1_STRICT_DIALECT.into(),&Std1StrictSnapshot{value:i32::try_from(cases["rejected"]["value"].as_i64().unwrap()).unwrap()},SnapshotEncoding::Binary,limits,&mut |_|true).await.unwrap_err();assert_eq!(rejected.diagnostics[0].code.0.as_str(),cases["rejected"]["code"].as_str().unwrap());
    }

    #[semio_framework_async_macros::async_test]
    #[ignore = "activate mandatory document capability enforcement after the semantic provider roster is complete"]
    async fn sqlite_snapshot_missing_document_owner_capability_is_rejected_before_publication(){
        let schema="semio.testkit.missing-sqlite-owner/v1";let mut codec=store::ArtifactCodec::of::<Std1AnySnapshot,Std1AnyMutation>(schema);codec.snapshot_sqlite=None;let mut plan=semio_framework::io::ArtifactAssemblyRegistryPlan::new().await;plan.document_codecs.push(codec);let assembly=store::begin_artifact_assembly().unwrap();assert!(semio_framework::io::commit_artifact_assembly_registry_plan(&assembly,plan).is_err());drop(assembly);assert!(store::document_codec(schema).await.unwrap().is_none());
    }

    #[semio_framework_async_macros::async_test]
    async fn sqlite_snapshot_covers_headless_atomic_assembly() {
        use semio_framework::io::{ArtifactAssemblyRegistryPlan, commit_artifact_assembly_registry_plan};
        use semio_framework::io::io_mechanism::{NativeSnapshotRegistration, io_entries, io_route, io_run};
        use semio_framework::io_schema::{IoPayload, SQLITE_SNAPSHOT};
        const HEADLESS: Dialect = Dialect { artifact_kind: "s.testkit.headless-fixture", standard: StandardId("1"), subset: SubsetId("*") };
        const REJECTED: Dialect = Dialect { artifact_kind: "s.testkit.rejected-headless", standard: StandardId("1"), subset: SubsetId("*") };
        let codec = store::ArtifactCodec::of::<Std1AnySnapshot, Std1AnyMutation>("semio.testkit.headless-fixture/v1");
        let mut plan = ArtifactAssemblyRegistryPlan::new().await;
        plan.document_codecs.push(codec.clone());
        plan.native_snapshots.push(NativeSnapshotRegistration { dialect: HEADLESS.into(), codec });
        let assembly = store::begin_artifact_assembly().expect("headless assembly barrier");
        commit_artifact_assembly_registry_plan(&assembly, plan).expect("headless atomic commit");
        drop(assembly);
        let sqlite = ArtifactDialect::from(SQLITE_SNAPSHOT);
        let export = io_route(&HEADLESS.into(), &sqlite, 1).await.expect("headless export").value;
        let import = io_route(&sqlite, &HEADLESS.into(), 1).await.expect("headless import").value;
        let payload = IoPayload::Binary(Std1AnySnapshot { value: 12 }.encode_pack());
        let database = io_run(&export, payload.clone()).await.expect("headless SQLite file").value;
        assert_eq!(io_run(&import, database).await.expect("headless restore").value, payload);
        let mut rejected_codec = store::ArtifactCodec::of::<Std1AnySnapshot, Std1AnyMutation>("semio.testkit.rejected-headless/v1");
        rejected_codec.snapshot_sqlite = None;
        let mut rejected = ArtifactAssemblyRegistryPlan::new().await;
        rejected.document_codecs.push(rejected_codec.clone());
        rejected.native_snapshots.push(NativeSnapshotRegistration { dialect: REJECTED.into(), codec: rejected_codec });
        let assembly = store::begin_artifact_assembly().expect("rejected assembly barrier");
        assert!(commit_artifact_assembly_registry_plan(&assembly, rejected).is_err());
        drop(assembly);
        assert!(store::document_codec("semio.testkit.rejected-headless/v1").await.expect("codec registry").is_none());
        assert!(!io_entries().iter().any(|entry| entry.from == REJECTED.into() || entry.into == REJECTED.into()));
    }

    /// 🧹️ THE fail-closed-disposer law. `Std1AnyEditor`/`Std1AnyViewer` declare nothing at all about
    /// store ownership — they are the shape 229 of the 298 editor/viewer impls under
    /// `✏️s/🔌️plugins` had on 2026-09-22 — and `VcsArtifactApp`'s close ladder drives five owned
    /// lanes through `drive_artifact_owned_disposer`, whose missing-disposer branch is the
    /// fail-closed `interactive-job.close-owned-disposer-missing`. An app that cannot be CLOSED
    /// cannot be published: the `codec` resolver constructs every app of a bundle to read its schema
    /// and closes each one it rejects, which is how one undeclared disposer in `🗒️note` killed the
    /// first three-package trusted-catalog bootstrap (ticket 26/09/18 slices TC3d/TC3e).
    ///
    /// 🎯️ Every lane is asserted through `ArtifactApp`, the trait the ladder actually reads, and for
    /// BOTH adapters, because `EditorApp<E>`/`ViewerApp<V>` forward `E::`/`V::`'s answer and an
    /// authoring-trait default of `None` would silently take the framework default back out.
    #[semio_framework_async_macros::async_test]
    async fn the_framework_owns_every_bounded_close_lane_an_app_declares_nothing_for() {
        type Edit = crate::app::EditorApp<Std1AnyEditor>;
        type View = crate::app::ViewerApp<Std1AnyViewer>;
        let lanes = [
            ("editor document-store", <Edit as crate::app::ArtifactApp>::build_document_store_disposer().is_some()),
            ("editor config-store", <Edit as crate::app::ArtifactApp>::build_config_store_disposer().is_some()),
            ("editor draft-store", <Edit as crate::app::ArtifactApp>::build_draft_store_disposer().is_some()),
            ("editor presence-store", <Edit as crate::app::ArtifactApp>::build_presence_store_disposer().is_some()),
            ("editor transient-store", <Edit as crate::app::ArtifactApp>::build_transient_store_disposer().is_some()),
            ("viewer document-store", <View as crate::app::ArtifactApp>::build_document_store_disposer().is_some()),
            ("viewer config-store", <View as crate::app::ArtifactApp>::build_config_store_disposer().is_some()),
            ("viewer draft-store", <View as crate::app::ArtifactApp>::build_draft_store_disposer().is_some()),
            ("viewer presence-store", <View as crate::app::ArtifactApp>::build_presence_store_disposer().is_some()),
            ("viewer transient-store", <View as crate::app::ArtifactApp>::build_transient_store_disposer().is_some()),
        ];
        let missing: Vec<&str> = lanes.iter().filter(|(_, present)| !present).map(|(lane, _)| *lane).collect();
        assert!(missing.is_empty(), "an app that declares no store ownership must still close: {} of {} lanes are fail-closed: {missing:?}", missing.len(), lanes.len());
        assert!(
            <Edit as crate::app::ArtifactApp>::build_presence_peer_retirement_factory().is_some() && <View as crate::app::ArtifactApp>::build_presence_peer_retirement_factory().is_some(),
            "the peer half of the presence lane is framework-owned too"
        );
    }
    //#endregion 🔖️Tests
}
