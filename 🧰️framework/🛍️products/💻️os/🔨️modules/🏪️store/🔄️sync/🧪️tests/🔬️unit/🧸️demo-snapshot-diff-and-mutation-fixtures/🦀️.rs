// 🧸️ Demo snapshot, diff and mutation fixtures: the demo snapshot with its pack and text hosts, the demo diff, the demo mutation with its text, binary and mutation laws and the bootstrap fixtures.

// 🎯️ `id` must be dotted `plugin.artifact` — `os_semio`'s preamble validator rejects a bare
// extension (this crate never compiled with `--features sync` before this packet, so the
// mismatch was never exercised at runtime). `extension_suffix` is the id's LAST segment, so
// `"demo.demo"` keeps `__DSL_EXTENSION` == "demo", unchanged from the old bare-extension form.
#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_os_dsl::DslArtifact, semio_framework_value::RetireOwned)]
#[artifact(id = "demo.demo")]
pub(super) struct DemoSnapshot {
    n: i32,
}


impl semio_framework_os_store_text_io::ArtifactSqliteSnapshotHost for DemoSnapshot {
    
    
    
}
impl pack::snapshot::ArtifactSqliteSnapshot for DemoSnapshot {
const SQLITE_SCHEMA: &'static str = include_str!("../🗄️.sql");
fn to_sqlite_database(&self, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<semio_framework_io_sqlite_snapshot::SqliteDatabase, semio_framework_value::ValueError> {
        use semio_framework_io_sqlite_snapshot::{SqliteDatabase, SqliteRow, SqliteValue, SqliteSnapshotPhase};
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        database.table_mut("demo_state")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Integer(i64::from(self.n))] });
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1, 1)?;
        Ok(database)
    }
fn from_sqlite_database(database: &semio_framework_io_sqlite_snapshot::SqliteDatabase, control: &mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        use semio_framework_io_sqlite_snapshot::{SqliteValue, SqliteSnapshotPhase};
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?;
        let rows = &database.table("demo_state")?.rows;
        if rows.len() != 1 || rows[0].rowid != 1 || rows[0].integer(0)? != 1 { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"demo SQLite snapshot requires one state row")); }
        let snapshot = Self { n: i32::try_from(rows[0].integer(1)?).map_err(|_|semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue,"demo state n exceeds i32"))? };
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
        Ok(snapshot)
    }
}

impl ArtifactDsl for DemoSnapshot {
    const EXTENSION: &'static str = Self::__DSL_EXTENSION;
    fn envelope_id() -> &'static str {
        Self::__DSL_ENVELOPE_ID
    }
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let body = match semio_framework_os_semio::split_text_preamble(text) {
            Ok((_, rest)) => rest,
            Err(_) => text,
        };
        let record = semio_framework_dsl_record::parse(body, &Self::__dsl_spec(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Document })?;
        Self::__dsl_from_record(&record)
    }
    fn print_dsl(&self) -> String {
        let body = semio_framework_dsl_record::print(&self.__dsl_to_record(), &Self::__dsl_spec(), semio_framework_dsl_record::JoinMode::Document);
        let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Dsl, 1).expect("valid envelope_id");
        semio_framework_os_semio::wrap_text(&envelope, &body)
    }
}

impl ArtifactPack for DemoSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.
    

    fn encode_pack_with(&self, options: &PackEncodeOptions) -> Result<Vec<u8>, PackError> {
        let inner = pack_rt::encode_document(&Self::__dsl_spec(), &self.__dsl_to_record(), options)?;
        let envelope = semio_framework_os_semio::SemioEnvelope::from_envelope_id(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1).map_err(|error| PackError::from(error.into_value_error()))?;
        Ok(semio_framework_os_semio::wrap_binary(&envelope, &inner))
    }
    fn decode_pack_with(bytes: &[u8], options: &PackDecodeOptions) -> Result<Self, PackError> {
        let (envelope, inner) = semio_framework_os_semio::unwrap_binary(bytes).map_err(|error| PackError::from(error.into_value_error()))?;
        if !envelope.matches_identity(<Self as ArtifactDsl>::envelope_id(), semio_framework_os_semio::Component::Pack, 1) {
            return Err(PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::UnsupportedOwner, format!("pack envelope mismatch: expected {}.pack v1, got {}", <Self as ArtifactDsl>::envelope_id(), envelope.binary_token()))));
        }
        let (record, _report) = pack_rt::decode_document(&inner, &Self::__dsl_spec(), options)?;
        Self::__dsl_from_record(&record).map_err(semio_framework_os_store_text_io::text_error_to_pack_error)
    }
    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        Some(Self::__dsl_spec())
    }
}
impl semio_framework_os_store_text_io::ArtifactStoreSnapshot for DemoSnapshot {
fn sqlite_snapshot_codec() -> Option<semio_framework_os_store_text_io::ArtifactSqliteSnapshotCodec> {
        Some(<Self as semio_framework_os_io::sqlite_snapshot_operations::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub(crate) struct DemoDiff {
    n: Option<i32>,
}

impl semio_framework_os_spr::DiffAlgebra<DemoSnapshot> for DemoDiff {
    
    fn is_empty(&self) -> bool {
        self.n.is_none()
    }
}

impl crate::os_spr::ColdDiffAlgebra<DemoSnapshot> for DemoDiff {
    fn inverse(&self, base: &DemoSnapshot) -> Self {
        Self { n: self.n.map(|_| base.n) }
    }
}

impl MutationDiff<DemoSnapshot> for DemoDiff {
    

    fn absorb(&mut self, other: Self) {
        if other.n.is_some() {
            self.n = other.n;
        }
    }

    fn apply(&self, snapshot: &mut DemoSnapshot, _capability: crate::os_spr::ApplyCapability) -> crate::os_spr::MutationApplyResult<()> {
                if let Some(value) = self.n { snapshot.n = value; }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(tag = "operation")]
#[value(tag = "operation")]
pub(super) enum DemoMutation {
    #[dsl(key = "set-n")]
    SetN { n: i32 },
}

impl OpText for DemoMutation {
    fn parse_op(line: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{} ", keyword);
            if line == keyword.as_str() || line.starts_with(&probe) {
                let record = semio_framework_dsl_record::parse(line, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline)
    }
}

impl OpBinary for DemoMutation {
    fn encode_op(&self) -> Result<Vec<u8>, semio_framework_os_spr::ProtocolError> {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (idx, (_, spec_fn)) = variants.iter().enumerate().find(|(_, (k, _))| k == &keyword).expect("variant spec must exist");
        let body = semio_framework_os_pack::record::encode_record_body(&(spec_fn.ordinary)(), &record, &PackEncodeOptions::default()).map_err(|e| semio_framework_os_spr::ProtocolError::Malformed { what: "op pack", offset: 0, detail: e.to_string() })?;
        let mut out = Vec::with_capacity(2 + body.len());
        out.push(pack_rt::OP_BINARY_FORMAT);
        out.push(idx as u8);
        out.extend_from_slice(&body);
        Ok(out)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, semio_framework_os_spr::ProtocolError> {
        let mut reader = semio_framework_os_pack::ByteReader::new(bytes);
        let format = reader.read_u8().map_err(|e| semio_framework_os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: e.to_string() })?;
        if format != pack_rt::OP_BINARY_FORMAT {
            return Err(semio_framework_os_spr::ProtocolError::Malformed { what: "op format", offset: 0, detail: format!("unsupported op binary format: {format}") });
        }
        let ordinal = reader.read_u8().map_err(|e| semio_framework_os_spr::ProtocolError::Malformed { what: "op ordinal", offset: 1, detail: e.to_string() })?;
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let (keyword, spec_fn) = variants.get(ordinal as usize).ok_or_else(|| semio_framework_os_spr::ProtocolError::Malformed { what: "op ordinal", offset: 1, detail: format!("op ordinal {ordinal} out of range for {}", variants.len()) })?;
        let spec = (spec_fn.ordinary)();
        let body = &bytes[reader.position()..];
        let (record, _report) = semio_framework_os_pack::record::decode_record_body(body, &spec, &PackDecodeOptions::default()).map_err(semio_framework_os_spr::ProtocolError::from)?;
        let offset = reader.position() as u64;
        <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record).map_err(|error| semio_framework_os_spr::ProtocolError::Malformed { what: "op record", offset, detail: error.to_string() })
    }
}

impl Mutation<DemoSnapshot> for DemoMutation {
    type Diff = DemoDiff;
    const DESCRIPTORS: &'static [MutationLeafDescriptor] = &[MutationLeafDescriptor {
        schema_version: 1,
        owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔄️sync",
        semantic_kind: "set-n",
        display_name: "Set N",
        emoji: "🔢️",
        aggregate_variant: "SetN",
        payload_schema: "DemoMutation::SetN",
        text_opcode: Some("set-n"),
        binary_tag: Some(0),
        invertibility: MutationInvertibility::ExplicitMutation,
        diff_participation: MutationDiffParticipation::ApplyOnly,
        outcome_classes: &[MutationOutcomeClass::Applied],
        composition: MutationComposition::Atomic,
        required_language_surfaces: &[MutationLanguageSurface::Rust, MutationLanguageSurface::Text, MutationLanguageSurface::Binary],
    }];

    fn descriptor(&self) -> &'static MutationLeafDescriptor {
        match self {
            Self::SetN { .. } => &Self::DESCRIPTORS[0],
        }
    }

    



    fn may_emit_foreign_steps(&self) -> bool {
        false
    }
}

impl semio_framework_os_spr::ColdMutation<DemoSnapshot> for DemoMutation {
    type ReferenceDiff = DemoDiff;
fn diff(&self, _snapshot: &DemoSnapshot) -> semio_framework_os_spr::MutationOutcome<DemoDiff> {
        semio_framework_os_spr::MutationOutcome::new(match self {
            DemoMutation::SetN { n } => DemoDiff { n: Some(*n) },
        })
    }

    fn inverse(&self, snapshot: &DemoSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| {
        vec![DemoMutation::SetN { n: snapshot.n }]
    
    })())
}

    
}

/// 🎯️ Idempotently registers the `demo/v1` codec (process-global `OnceLock` registry,
/// shared across every test in this binary) — needed by any test exercising `FolderEndpoint`
/// end-to-end (both `Sqlite` and `Pack` now go through `document_codec` per the pack+spr flip),
/// mirroring a real app's program-init-time `register_document_codec_for_app` call. A concurrent
/// caller returns only once the registration has landed (a swapped flag let parallel laws run
/// codec-less: ticket 26/09/23 `📓️wp-h6.md`).
pub(super) async fn ensure_demo_codec_registered() {
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| register_document_codec(artifact_codec::<DemoSnapshot, DemoMutation>("demo/v1")).expect("register demo codec"));
}

fn bootstrap_frontier(document_id: &str, ordinal: u64, edit_id: &str, commit: u64, chain: u8) -> RuntimeFrontierSummary {
    RuntimeFrontierSummary { document_id: ArtifactId(document_id.into()), head_edit_ordinal: ordinal, head_edit_id: edit_id.into(), last_commit_seq: commit, chain_hash: [chain; 32] }
}

async fn demo_artifact_bootstrap(inline: bool) -> (ArtifactBootstrap, ArtifactBootstrapPair) {
    ensure_demo_codec_registered().await;
    let envelope = create_document_envelope::<DemoSnapshot, DemoMutation>("demo/v1", "demo", DemoSnapshot { n: 7 }, None, &semio_framework_os_vcs::io::binary::genesis::ArtifactPackGenesisProducer);
    let files = print_document_pack(&envelope).await.expect("print bootstrap pair");
    drop(envelope.into_owners());
    let pair = ArtifactBootstrapPair { pack: files.pack, spr: files.spr };
    let pack_schema_hash = semio_framework_os_store_codec_registry::document_codec("demo/v1").await.expect("codec lookup").expect("demo codec").pack_schema_hash;
    let bootstrap = ArtifactBootstrap {
        format_version: semio_framework_os_spr::ARTIFACT_BOOTSTRAP_FORMAT_VERSION,
        descriptor_hash: [0x11; 32],
        artifact_schema: "demo/v1".into(),
        artifact_kind: "demo".into(),
        pack_schema_hash,
        baseline_frontier: bootstrap_frontier("demo", 7, "edit-7", 3, 0x33),
        pack_hash: semio_framework_hash::Sha256::digest(&pair.pack),
        spr_hash: semio_framework_hash::Sha256::digest(&pair.spr),
        pack_length: pair.pack.len() as u64,
        spr_length: pair.spr.len() as u64,
        chunk_count: if inline { 0 } else { 3 },
        aggregate_hash: semio_framework_os_spr::artifact_bootstrap_aggregate_hash(&pair.pack, &pair.spr),
        required_tail_frontier: bootstrap_frontier("demo", 9, "edit-9", 4, 0x44),
        inline: inline.then(|| pair.clone()),
    };
    (bootstrap, pair)
}
