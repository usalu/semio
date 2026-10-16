//! 📣️ Explicit owner registration of the space-history SQLite snapshot: it publishes the history factory and the semantic capability atomically, so it sits above the codec registry, the IO mechanism and the codec factory.
use semio_framework_os_store_space_history::io::sqlite::snapshot::SQLITE_SNAPSHOT_DIALECT;
use semio_framework_os_store_space_schema::space_history_document::{S_SPACE_HISTORY_SCHEMA, SpaceHistorySnapshot};
use semio_framework_os_store_codec_factory::lifted_store::artifact_codec;
use semio_framework_os_store_space_schema::space_history_mutations::SpaceHistoryMutation;

/// 📣️ Explicit owner registration publishes the history factory and semantic capability atomically.
/// Call before registry I/O; generic Store construction and retained hydration do not publish codecs.
pub fn register_sqlite_snapshot() -> Result<(), semio_framework_os_io::ArtifactAssemblyRegistryError> {
    semio_framework_os_io::register_native_snapshot_codec(SQLITE_SNAPSHOT_DIALECT, artifact_codec::<SpaceHistorySnapshot, SpaceHistoryMutation>(S_SPACE_HISTORY_SCHEMA))
}
