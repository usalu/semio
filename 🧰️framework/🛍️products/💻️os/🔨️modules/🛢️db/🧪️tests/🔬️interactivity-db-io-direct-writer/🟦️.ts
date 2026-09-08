import { interactivityDbIoDirectWriterFailures } from "../../../../../../../📜️script.ts";

/** 🧪️ Executes interactivity db io direct writer policy assertions. */
export function interactivityDbIoDirectWriterSelfTests(): void {
  const engine = "async fn encode_catalog_pages DbIoPageWriter::try_reserve catalog_write_json_string writer.seal_retained().await";
  const snapshot = "struct SnapshotPageSink SnapshotPageSink::try_new()? pub async fn build_generation_pages writer.seal_retained().await descriptor.write_retained(&mut descriptor_segment).await? begin_identity_segment(pack::KIND_SNAPSHOT";
  const index = "async fn encode_run_pages DbIoPageWriter::try_reserve Crc32cCursor run_write writer.seal_retained().await";
  const wal = "struct SharedBuf(std::sync::Arc<std::sync::Mutex<db_storage::DbIoPageWriter>>) fn try_new() -> Result<Self, DbError> async fn copy_range async fn read_exact write_fragment(&bytes[cursor..]) record.write_retained(&mut self.writer).await?";
  const spr = "async fn write_frame_retained Crc32cCursor::new() pending_chain_hasher: blake3::Hasher write_frame_retained(&mut self.sink";
  const pack = "pub struct PackIdentitySegment pub async fn begin_identity_segment let chunks = std::mem::take(&mut self.chunks) segment.write_fragment(retained_varint self.begin_identity_segment(crate::KIND_CHUNK";
  if (interactivityDbIoDirectWriterFailures(engine, snapshot, index, wal, spr, pack).length !== 0) throw new Error("[verify interactivity] DB direct-writer self-test rejected retained writers.");
  const mutations = [
    ["catalog-encode-then-copy", `${engine} serde_json::to_vec`, snapshot, index, wal, spr, pack],
    ["catalog-missing-writer", engine.replace("catalog_write_json_string", "serde_json_escape"), snapshot, index, wal, spr, pack],
    ["snapshot-vec-sink", engine, `${snapshot} PackWriter::begin(Vec`, index, wal, spr, pack],
    ["snapshot-missing-page-sink", engine, snapshot.replace("struct SnapshotPageSink", "struct SnapshotBuffer"), index, wal, spr, pack],
    ["index-copy", engine, snapshot, `${index} db_io_copy_pages(&encoded`, wal, spr, pack],
    ["index-missing-crc-cursor", engine, snapshot, index.replace("Crc32cCursor", "crc32c"), wal, spr, pack],
    ["wal-vec", engine, snapshot, index, wal.replace("Mutex<db_storage::DbIoPageWriter>", "Mutex<Vec<u8>>"), spr, pack],
    ["wal-snapshot", engine, snapshot, index, `${wal} .snapshot()`, spr, pack],
    ["wal-frame-scratch", engine, snapshot, index, wal, `${spr} scratch: Vec<u8>`, pack],
    ["wal-digest-list", engine, snapshot, index, wal, `${spr} pending_digests: Vec`, pack],
    ["snapshot-pack-table-copy", engine, snapshot, index, wal, spr, `${pack} let table_bytes = encode_chunk_table`],
    ["snapshot-pack-missing-cursor", engine, snapshot, index, wal, spr, pack.replace("pub struct PackIdentitySegment", "pub struct PackSegmentBytes")],
  ] as const;
  for (const [name, mutatedEngine, mutatedSnapshot, mutatedIndex, mutatedWal, mutatedSpr, mutatedPack] of mutations)
    if (interactivityDbIoDirectWriterFailures(mutatedEngine, mutatedSnapshot, mutatedIndex, mutatedWal, mutatedSpr, mutatedPack).length === 0) throw new Error(`[verify interactivity] DB direct-writer self-test ${name} was falsely accepted.`);
}
