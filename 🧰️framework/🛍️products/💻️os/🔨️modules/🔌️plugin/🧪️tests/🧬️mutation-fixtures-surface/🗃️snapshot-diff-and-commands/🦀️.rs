/// 🗃️ Snapshot, diff and commands of the surface fixture: the testkit dialect, the surface snapshot with its sqlite, composition, dsl and pack implementations, the surface diff and the editor and viewer commands with their op text and binary codecs.

const SURFACE_TESTKIT_DIALECT: Dialect = Dialect { artifact_kind: "testkit.surface", standard: StandardId("1"), subset: SubsetId::ANY };

#[derive(semio_framework_dsl_record_derive::DslRecord, Clone, Debug, Default, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_os_kernel::DslArtifact, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone)]
#[artifact(extension = "testkit-surface")]
pub struct SurfaceSnapshot {
    pub(crate) count: i32,
}



impl store::os_store::serialization::ArtifactSqliteSnapshotHost for SurfaceSnapshot {



}
impl pack::snapshot::ArtifactSqliteSnapshot for SurfaceSnapshot {
const SQLITE_SCHEMA: &'static str = include_str!("../🗄️.sql");
fn to_sqlite_database(&self, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<store::sqlite_snapshot::SqliteDatabase, semio_framework_value::ValueError> {
        use store::sqlite_snapshot::{SqliteDatabase, SqliteRow, SqliteSnapshotPhase, SqliteValue};
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        database.table_mut("surface_state")?.rows.push(SqliteRow { rowid: 1, values: vec![SqliteValue::Integer(1), SqliteValue::Integer(i64::from(self.count))] });
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1, 1)?;
        Ok(database)
    }
fn from_sqlite_database(database: &store::sqlite_snapshot::SqliteDatabase, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
        control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?;
        let rows = &database.table("surface_state")?.rows;
        if rows.len() != 1 || rows[0].rowid != 1 || rows[0].integer(0)? != 1 {
            return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "surface_state requires one state row"));
        }
        let snapshot = Self { count: i32::try_from(rows[0].integer(1)?).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))? };
        control.checkpoint(store::sqlite_snapshot::SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
        Ok(snapshot)
    }
}

impl semio_framework_schema_composition::ArtifactCompositionFields for SurfaceSnapshot {
    fn visit_child_refs<'a, V: semio_framework_schema_composition::ChildRefVisitor<'a>>(&'a self, _visitor: &mut V) -> Result<(), V::Error> {
        Ok(())
    }
}

impl pack::snapshot::ArtifactDsl for SurfaceSnapshot {
    const EXTENSION: &'static str = "testkit-surface";
    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        if text.trim().is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_str(text).map_err(|error| semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }
}

impl pack::snapshot::ArtifactPack for SurfaceSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.


    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        serde_json::to_vec(self).map_err(|error| store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string())))
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        if bytes.is_empty() {
            return Ok(Self::default());
        }
        serde_json::from_slice(bytes).map_err(|error| match (u32::try_from(error.line()), u32::try_from(error.column())) { (Ok(line), Ok(column)) => store::PackError::from(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(line, column))), _ => store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, error.to_string())) })
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for SurfaceSnapshot {
fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize, ToValue, FromValue)]
pub struct SurfaceDiff {
    pub(crate) count: Option<i32>,
}

impl protocol::DiffAlgebra<SurfaceSnapshot> for SurfaceDiff {

    fn is_empty(&self) -> bool {
        self.count.is_none()
    }
}

impl protocol::ColdDiffAlgebra<SurfaceSnapshot> for SurfaceDiff {
    fn inverse(&self, base: &SurfaceSnapshot) -> Self {
        Self { count: self.count.map(|_| base.count) }
    }
}

impl MutationDiff<SurfaceSnapshot> for SurfaceDiff {


}

impl ColdMutationDiff<SurfaceSnapshot> for SurfaceDiff {
    fn absorb(&mut self, other: Self) {
        if other.count.is_some() {
            self.count = other.count;
        }
    }

    fn apply(&self, snapshot: &mut SurfaceSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
                if let Some(value) = self.count { snapshot.count = value; }
        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, ToValue, Deserialize, FromValue, semio_framework_dsl_record_derive::DslEnum, semio_framework_value::RetireOwned)]
enum SurfaceEditorCommand {
    #[dsl(key = "increment")]
    Increment,
}

impl ::protocol::OpText for SurfaceEditorCommand {
    fn parse_op(line: &str) -> Result<Self, ::semio_framework_diagnostic::TextError> {
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        for (keyword, spec_fn) in &variants {
            let probe = format!("{keyword} ");
            if line == keyword.as_str() || line.starts_with(&probe) {
                let body = if line.len() > keyword.len() { line[keyword.len()..].trim_start() } else { "" };
                let record = semio_framework_dsl_record::parse(body, &(spec_fn.ordinary)(), &semio_framework_dsl_record::ParseOptions { limits: ::semio_framework_diagnostic::Limits::default(), mode: semio_framework_dsl_record::SourceMode::Inline })?;
                return <Self as semio_framework_dsl_record::DslVariants>::from_named_record(keyword, &record);
            }
        }
        Err(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue,(format!("unknown operation line '{line}'")).to_string(),semio_framework_diagnostic::TextSpan::at(1,1)))
    }
    fn print_op(&self) -> String {
        let (keyword, record) = <Self as semio_framework_dsl_record::DslVariants>::to_named_record(self);
        let variants = <Self as semio_framework_dsl_record::DslVariants>::variants();
        let spec_fn = variants.iter().find(|(k, _)| k == &keyword).map(|(_, s)| *s).expect("variant spec must exist for its own keyword");
        let body = semio_framework_dsl_record::print(&record, &(spec_fn.ordinary)(), semio_framework_dsl_record::JoinMode::Inline);
        if body.is_empty() {
            keyword
        } else {
            format!("{keyword} {body}")
        }
    }
}

impl ::protocol::OpBinary for SurfaceEditorCommand {
    const TOOL_JOB_IDS: &'static [&'static str] = &[SURFACE_TOOL_ID];

    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        ::dsl::variants_binary::encode_op(self)
    }
    fn decode_op(bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        ::dsl::variants_binary::decode_op(bytes)
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, semio_framework_value::RetireOwned)]
enum SurfaceViewerCommand {
    #[default]
    Noop,
}

impl ::protocol::OpBinary for SurfaceViewerCommand {
    fn encode_op(&self) -> Result<Vec<u8>, ::protocol::ProtocolError> {
        Ok(Vec::new())
    }
    fn decode_op(_bytes: &[u8]) -> Result<Self, ::protocol::ProtocolError> {
        Ok(SurfaceViewerCommand::Noop)
    }
}
