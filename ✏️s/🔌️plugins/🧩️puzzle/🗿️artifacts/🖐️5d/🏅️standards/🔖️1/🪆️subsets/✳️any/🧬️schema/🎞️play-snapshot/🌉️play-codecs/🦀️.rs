//! 🌉️ Puzzle 5d play snapshot codecs — the pack, DSL text, store and SQLite snapshot capabilities of `Puzzle5dPlaySnapshot`, each delegating to the typed `Puzzle5dSnapshot` authority.

use super::Puzzle5dPlaySnapshot;
use crate::standards::v1::subsets::any::schema::snapshot::Puzzle5dSnapshot;
use pack::snapshot::ArtifactSqliteSnapshot as _;
use semio_framework_value::{ValueError};
use store::{os_store::serialization::ArtifactSqliteSnapshotHost,sqlite_snapshot::{SqliteDatabase,SqliteSnapshotControl,SnapshotEncoding}};

/// 📦️ Packs through the typed authority, so the play kind shares `Puzzle5dSnapshot`'s derived record
/// layout and pack-schema identity.
impl pack::snapshot::ArtifactPack for Puzzle5dPlaySnapshot {
    fn encode_pack_with(&self, options: &store::PackEncodeOptions) -> Result<Vec<u8>, store::PackError> {
        self.typed().encode_pack_with(options)
    }

    fn decode_pack_with(bytes: &[u8], options: &store::PackDecodeOptions) -> Result<Self, store::PackError> {
        <Puzzle5dSnapshot as pack::snapshot::ArtifactPack>::decode_pack_with(bytes, options).map(Self::from_typed)
    }

    fn record_spec() -> Option<semio_framework_dsl_record::RecordSpec> {
        <Puzzle5dSnapshot as pack::snapshot::ArtifactPack>::record_spec()
    }
    
}

impl ::semio_framework_os_kernel::os_store::serialization::ArtifactStoreSnapshot for Puzzle5dPlaySnapshot {
    fn reference_snapshot_codec()->Option<::semio_framework_os_kernel::os_store::serialization::ArtifactReferenceSnapshotCodec<Self>>{Some(::semio_framework_os_kernel::os_store::serialization::reference_snapshot_codec::<Self>())}

fn sqlite_snapshot_codec() -> Option<store::os_store::serialization::ArtifactSqliteSnapshotCodec> {
        Some(<Self as store::os_store::ArtifactSqliteSnapshotOperations>::sqlite_codec())
    }
}

impl pack::snapshot::ArtifactDsl for Puzzle5dPlaySnapshot {
    const EXTENSION: &'static str = "puzzle5d-play";

    fn parse_dsl(text: &str) -> Result<Self, semio_framework_diagnostic::TextError> {
        <Puzzle5dSnapshot as pack::snapshot::ArtifactDsl>::parse_dsl(text).map(Self::from_typed)
    }

    fn print_dsl(&self) -> String {
        <Puzzle5dSnapshot as pack::snapshot::ArtifactDsl>::print_dsl(self.typed())
    }
}

impl ArtifactSqliteSnapshotHost for Puzzle5dPlaySnapshot{
 
 
 
 fn decode_sqlite_snapshot_native(payload:&store::io_schema::IoPayload,c:&mut SqliteSnapshotControl<'_>,native_control:&mut semio_framework_os_kernel::NativeSnapshotDecodeOwner<'_, '_>)->Result<Self,ValueError>{Puzzle5dSnapshot::decode_sqlite_snapshot_native(payload,c,native_control).map(Self::from_typed)}
 fn encode_sqlite_snapshot_native(&self,encoding:SnapshotEncoding,c:&mut SqliteSnapshotControl<'_>,native_owner:&mut semio_framework_os_kernel::NativeSnapshotEncodeOwner<'_, '_>)->Result<store::io_schema::IoPayload,ValueError>{self.typed().encode_sqlite_snapshot_native(encoding,c,native_owner)}
 fn validate_sqlite_snapshot_subset(&self,dialect:&semio_framework_artifact_reference::ArtifactDialect,d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->store::io_schema::IoResult<()>{self.typed().validate_sqlite_snapshot_subset(dialect,d,c)}
}

impl pack::snapshot::ArtifactSqliteSnapshot for Puzzle5dPlaySnapshot {
const SQLITE_SCHEMA:&'static str=Puzzle5dSnapshot::SQLITE_SCHEMA;
fn to_sqlite_database(&self,c:&mut SqliteSnapshotControl<'_>)->Result<SqliteDatabase,ValueError>{self.typed().to_sqlite_database(c)}
fn from_sqlite_database(d:&SqliteDatabase,c:&mut SqliteSnapshotControl<'_>)->Result<Self,ValueError>{Puzzle5dSnapshot::from_sqlite_database(d,c).map(Self::from_typed)}
}
