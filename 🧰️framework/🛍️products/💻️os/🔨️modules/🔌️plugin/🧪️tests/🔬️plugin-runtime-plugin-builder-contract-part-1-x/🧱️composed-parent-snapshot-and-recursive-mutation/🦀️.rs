/// 🧱️ Composed parent snapshot and recursive mutation: the composition grants, the receiving profile law, the composed parent snapshot with its pack and sqlite codecs and the recursive fixture mutation.

/// 🔬️ Items of Part 1 of 7 of the plugin builder contract laws that came from included fixture files.
//#region 🧬️ComposedParentFixture
#[doc(hidden)]
pub const COMPOSITION_MAINTENANCE_GRANT:store::RetainedCloneGrant=store::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:262144,maximum_release_bytes:262144,maximum_depth:656};
const COMPOSITION_PAGE_GRANT:store::RetainedCloneGrant=store::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:4096,maximum_capacity_bytes:4096,maximum_release_bytes:4096,maximum_depth:656};
const COMPOSITION_CLEANUP_GRANT:store::RetainedCloneGrant=store::RetainedCloneGrant{maximum_items:1,maximum_copy_bytes:262144,maximum_capacity_bytes:262144,maximum_release_bytes:262144,maximum_depth:656};

#[test]
fn composition_receiving_profiles_match_authored_independent_schema_values(){let fixture:Value=serde_json::from_str(include_str!("../../🧩️composition/🎟️receiving/🧪️tests/🧫️fixtures/🔣️.json")).unwrap();for(name,grant)in[("maintenance",COMPOSITION_MAINTENANCE_GRANT),("page",COMPOSITION_PAGE_GRANT),("cleanup",COMPOSITION_CLEANUP_GRANT)]{assert_eq!(serde_json::to_value(grant).unwrap(),fixture[name])}}
type ParentFixtureChild = store::os_store::artifact_child::ArtifactChild<TestSnapshot>;

thread_local! { static PARENT_SNAPSHOT_CLONES: std::cell::Cell<usize> = const { std::cell::Cell::new(0) }; }

#[derive(Debug, Default, PartialEq, semio_framework_value::RetireOwned, semio_framework_value::RetainedClone, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue, semio_framework_schema::ArtifactSchema)]
#[artifact_schema(id = "s.test.composed")]
struct ComposedParentSnapshot {
    #[state(artifact)]
    #[child(kind = "s.test.child")]
    pub slot: Option<ParentFixtureChild>,
    #[state(artifact)]
    pub revision: i32,
}


impl store::os_store::serialization::ArtifactSqliteSnapshotHost for ComposedParentSnapshot {



}
impl pack::snapshot::ArtifactSqliteSnapshot for ComposedParentSnapshot {
const SQLITE_SCHEMA: &'static str = include_str!("../../🧩️composition/🪴️parent.sql");
fn to_sqlite_database(&self, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<store::sqlite_snapshot::SqliteDatabase, semio_framework_value::ValueError> {
        use store::sqlite_snapshot::{SqliteDatabase, SqliteRow, SqliteValue, SqliteSnapshotPhase};
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 0, 1)?;
        let mut database = SqliteDatabase::from_schema(Self::SQLITE_SCHEMA)?;
        let mut values = vec![SqliteValue::Integer(1), SqliteValue::Integer(i64::from(self.revision))];
        values.extend(match &self.slot { Some(child) => vec![SqliteValue::Text(child.target.artifact_id.clone()), SqliteValue::Text(child.target.dialect.artifact_kind.clone()), SqliteValue::Text(child.target.dialect.standard.clone()), SqliteValue::Text(child.target.dialect.subset.clone())], None => vec![SqliteValue::Null; 4] });
        database.table_mut("parent_state")?.rows.push(SqliteRow { rowid: 1, values });
        control.checkpoint(SqliteSnapshotPhase::ProjectSnapshot, 1, 1)?;
        Ok(database)
    }
fn from_sqlite_database(database: &store::sqlite_snapshot::SqliteDatabase, control: &mut store::sqlite_snapshot::SqliteSnapshotControl<'_>) -> Result<Self, semio_framework_value::ValueError> {
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_artifact_reference::io::text::artifact_reference::{ArtifactReferenceText as _};

        use store::sqlite_snapshot::{SqliteValue, SqliteSnapshotPhase};
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 0, 1)?;
        let rows = &database.table("parent_state")?.rows;
        if rows.len() != 1 || rows[0].rowid != 1 || rows[0].integer(0)? != 1 { return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "parent_state requires one state row")); }
        let row = &rows[0];
        let child = match row.values.get(2..).ok_or_else(|| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "parent state row is truncated"))? {
            [SqliteValue::Null, SqliteValue::Null, SqliteValue::Null, SqliteValue::Null] => None,
            [SqliteValue::Text(id), SqliteValue::Text(kind), SqliteValue::Text(standard), SqliteValue::Text(subset)] => {
                let target = semio_framework_artifact_reference::ArtifactRef { artifact_id: id.clone(), dialect: semio_framework_artifact_reference::ArtifactDialect { artifact_kind: kind.clone(), standard: standard.clone(), subset: subset.clone() } };
                let target = semio_framework_artifact_reference::ArtifactRef::parse_uri(&target.to_uri()).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error))?;
                Some(store::os_store::artifact_child::ArtifactChild::new(target.artifact_id.clone(), target))
            }
            _ => return Err(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "optional child identity must have all four fields or none")),
        };
        let snapshot = Self { revision: i32::try_from(row.integer(1)?).map_err(|error| semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string()))?, slot: child };
        control.checkpoint(SqliteSnapshotPhase::ReconstructSnapshot, 1, 1)?;
        Ok(snapshot)
    }
}

impl Clone for ComposedParentSnapshot {
    fn clone(&self) -> Self {
        PARENT_SNAPSHOT_CLONES.with(|count| count.set(count.get() + 1));
        Self { slot: self.slot.clone(), revision: self.revision }
    }
}

#[doc(hidden)]
#[derive(Clone, Debug, PartialEq, semio_framework_value::RetireOwned, semio_framework_value::CanonicalJsonTree, serde::Serialize, serde::Deserialize, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
#[canonical_json(owner = semio_framework_pack_json)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[value(rename_all = "camelCase", deny_unknown_fields)]
pub struct RecursiveFixtureMutation {
    pub value: i32,
}

impl protocol::OpText for RecursiveFixtureMutation {
    fn parse_op(line: &str) -> Result<Self, TextError> {
        let value = line
            .strip_prefix("set-recursive-value ")
            .ok_or_else(|| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "expected set-recursive-value", TextSpan::at(1, 1)))?
            .parse()
            .map_err(|_| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, "recursive value must be i32", TextSpan::at(1, 1)))?;
        Ok(Self { value })
    }

    fn print_op(&self) -> String {
        format!("set-recursive-value {}", self.value)
    }
}

impl protocol::OpBinary for RecursiveFixtureMutation {
    fn encode_op(&self) -> Result<Vec<u8>, protocol::ProtocolError> {
        let mut bytes = vec![0x72];
        bytes.extend_from_slice(&self.value.to_be_bytes());
        Ok(bytes)
    }

    fn decode_op(bytes: &[u8]) -> Result<Self, protocol::ProtocolError> {
        if bytes.len() != 5 || bytes.first() != Some(&0x72) {
            return Err(protocol::ProtocolError::Malformed { what: "recursive-fixture-mutation", offset: 0, detail: "expected tag 0x72 and four i32 bytes".into() });
        }
        Ok(Self { value: i32::from_be_bytes(bytes[1..].try_into().expect("exact recursive i32 width")) })
    }
}

const RECURSIVE_FIXTURE_MUTATION_DESCRIPTOR: protocol::MutationLeafDescriptor = protocol::MutationLeafDescriptor {
    schema_version: 1,
    owner: "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🧩️composition",
    semantic_kind: "set-recursive-value",
    display_name: "Set Recursive Value",
    emoji: "🧬️",
    aggregate_variant: "SetRecursiveValue",
    payload_schema: "RecursiveFixtureMutation",
    text_opcode: Some("set-recursive-value"),
    binary_tag: Some(0x72),
    invertibility: protocol::MutationInvertibility::ExplicitMutation,
    diff_participation: protocol::MutationDiffParticipation::ApplyOnly,
    outcome_classes: &[protocol::MutationOutcomeClass::Applied],
    composition: protocol::MutationComposition::Atomic,
    required_language_surfaces: &[protocol::MutationLanguageSurface::Rust, protocol::MutationLanguageSurface::Text, protocol::MutationLanguageSurface::Binary],
};

#[derive(Clone, Debug, Default, PartialEq, semio_framework_value_derive::ToValue, semio_framework_value_derive::FromValue)]
struct ComposedParentDiff {
    pub revision: Option<i32>,
}

impl protocol::DiffAlgebra<ComposedParentSnapshot> for ComposedParentDiff {

    fn is_empty(&self) -> bool {
        self.revision.is_none()
    }
}

impl protocol::ColdDiffAlgebra<ComposedParentSnapshot> for ComposedParentDiff {
    fn inverse(&self, base: &ComposedParentSnapshot) -> Self {
        Self { revision: self.revision.map(|_| base.revision) }
    }
}

impl protocol::MutationDiff<ComposedParentSnapshot> for ComposedParentDiff {



}

impl protocol::ColdMutationDiff<ComposedParentSnapshot> for ComposedParentDiff {
    fn absorb(&mut self, other: Self) {
        if other.revision.is_some() {
            self.revision = other.revision;
        }
    }

    fn apply(&self, base: &mut ComposedParentSnapshot, _capability: protocol::ApplyCapability) -> protocol::MutationApplyResult<()> {
                if let Some(value) = self.revision { base.revision = value; }
        Ok(())
    }
}

impl Mutation<ComposedParentSnapshot> for RecursiveFixtureMutation {
    type Diff = ComposedParentDiff;
    const DESCRIPTORS: &'static [protocol::MutationLeafDescriptor] = &[RECURSIVE_FIXTURE_MUTATION_DESCRIPTOR];
    fn descriptor(&self) -> &'static protocol::MutationLeafDescriptor { &RECURSIVE_FIXTURE_MUTATION_DESCRIPTOR }


}

impl semio_framework_os_kernel::ColdMutation<ComposedParentSnapshot> for RecursiveFixtureMutation {
    type ReferenceDiff = ComposedParentDiff;
    fn diff(&self, _base: &ComposedParentSnapshot) -> protocol::MutationOutcome<Self::ReferenceDiff> { protocol::MutationOutcome::new(ComposedParentDiff { revision: Some(self.value) }) }
    fn inverse(&self, base: &ComposedParentSnapshot) -> Result<Vec<Self>, semio_framework_value::ValueError> {
    Ok((|| { vec![Self { value: base.revision }]
    })())
}
}

impl pack::snapshot::ArtifactDsl for ComposedParentSnapshot {
    const EXTENSION: &'static str = "composed-parent-test";
    fn parse_dsl(text: &str) -> Result<Self, TextError> {
        let value = serde_json::from_str::<Value>(text).map_err(|error| TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), TextSpan::at(1, 1)))?;
        <Self as semio_framework_value::FromValue>::from_value(value.into()).map_err(|error| TextError::from_value_error(error, TextSpan::at(1, 1)))
    }
    fn print_dsl(&self) -> String {
        semio_framework_pack_json::to_json_string(self)
    }
}

impl ArtifactPack for ComposedParentSnapshot {
    /// 🪶️ Publishes this owner's actual relational snapshot capability.


    fn encode_pack_with(&self, _options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        Ok(semio_framework_pack_json::to_json_string(self).into_bytes())
    }
    fn decode_pack_with(bytes: &[u8], _options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        let value = serde_json::from_slice::<Value>(bytes).map_err(|error| match (u32::try_from(error.line()), u32::try_from(error.column())) { (Ok(line), Ok(column)) => store::PackError::from(semio_framework_diagnostic::TextError::new(semio_framework_value::ValueRefusalKind::InvalidValue, error.to_string(), semio_framework_diagnostic::TextSpan::at(line, column))), _ => store::PackError::from(semio_framework_value::ValueError::new(semio_framework_value::ValueRefusalKind::OwnershipLimit, error.to_string())) })?;
        <Self as semio_framework_value::FromValue>::from_value(value.into()).map_err(|error| store::PackError::from(error))
    }
}
impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for ComposedParentSnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}
//#endregion 🧬️ComposedParentFixture
