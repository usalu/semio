//! 🧰️ Derived relational snapshot operations of the codec layer: they compose the registry-layer export and import with the typed file mechanism, so they sit above the core host trait and below nothing that the host trait needs.
//!
//! @see 🪶️ `ArtifactSqliteSnapshotHost` the core trait whose authored hooks the operations consume.
use pack::snapshot::{ArtifactDsl,ArtifactPack};
use semio_framework_os_store_text_io::ArtifactSqliteSnapshotCodec;
use semio_framework_os_store_space_history::lifted_serialization::ArtifactSqliteSnapshotHost;
use semio_framework_io_control::{NativeSnapshotDecodeOwner, NativeSnapshotEncodeOwner};

/// 🏭️ Builds the erased relational codec of every core host; the blanket implementation is the only one, so no owner can fork the derived behaviour.
pub trait ArtifactSqliteSnapshotOperations: ArtifactSqliteSnapshotHost {
    /// 🪶️ Builds the relational codec only when the snapshot actually implements it.
    fn sqlite_codec() -> ArtifactSqliteSnapshotCodec
    where
        Self: Sized + ArtifactDsl + ArtifactPack + semio_framework_value::retirement::RetireOwned + Sync;
}

fn export_snapshot_impl<P: ArtifactDsl + ArtifactPack + ArtifactSqliteSnapshotHost + semio_framework_value::retirement::RetireOwned>(schema:&str,dialect:&semio_framework_artifact_reference::ArtifactDialect,payload:&semio_framework_io_schema::IoPayload,control:&mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>,native:&mut NativeSnapshotDecodeOwner<'_,'_>)->semio_framework_io_schema::IoResult<semio_framework_io_sqlite_snapshot::SqliteDatabase>{semio_framework_os_store_codec_registry::sqlite_snapshot_retirement::export::<P>(schema,dialect,payload,control,native)}
fn import_snapshot_impl<P: ArtifactDsl + ArtifactPack + ArtifactSqliteSnapshotHost + semio_framework_value::retirement::RetireOwned>(schema:&str,dialect:&semio_framework_artifact_reference::ArtifactDialect,input:&mut Option<semio_framework_io_sqlite_snapshot::SqliteDatabase>,encoding:semio_framework_io_sqlite_snapshot::SnapshotEncoding,control:&mut semio_framework_io_sqlite_snapshot::SqliteSnapshotControl<'_>,native:&mut NativeSnapshotEncodeOwner<'_,'_>)->semio_framework_io_schema::IoResult<semio_framework_io_schema::IoPayload>{semio_framework_os_store_codec_registry::sqlite_snapshot_retirement::import::<P>(schema,dialect,input,encoding,control,native)}

impl<T: ArtifactSqliteSnapshotHost + ?Sized> ArtifactSqliteSnapshotOperations for T {
    fn sqlite_codec() -> ArtifactSqliteSnapshotCodec
    where
        Self: Sized + ArtifactDsl + ArtifactPack + semio_framework_value::retirement::RetireOwned + Sync,
    {
        ArtifactSqliteSnapshotCodec { schema: std::borrow::Cow::Borrowed(Self::sqlite_schema()), snapshot_type: Some(std::any::TypeId::of::<Self>()), export: export_snapshot_impl::<Self>, import: import_snapshot_impl::<Self>,export_file:Some(crate::io_mechanism::export_snapshot_file_read::<Self>),import_file:Some(crate::io_mechanism::import_snapshot_file_into::<Self>) }
    }
}
