//#region 📤️SegmentedDownloadContract
/** 📤️ THE segmented-download chunk contract — one record every hop of the lane reads, so a bound can
 * never drift between the four places that enforce it:
 *
 * 1. the guest PRODUCER (`🔌️plugin/🦀️.rs`'s `ARTIFACT_OUTPUT_CHUNK_BYTES` /
 *    `ARTIFACT_SEGMENTED_DOWNLOAD_TOTAL_BYTES`, which slice and cap `ArtifactOutputChunks`),
 * 2. the generated shard WORKER (`🔌️plugin/🌐️browser-bundle/🏗️materialization/🟦️.ts`'s `shardWorkerSource`, which
 *    interpolates {@link SEGMENTED_DOWNLOAD_CONTRACT}`.chunkBytes` straight out of the declared mirror
 *    `SEGMENTED_DOWNLOAD_CHUNK_BYTES` there),
 * 3. the host TRANSPORT (`📮️shard-client/🟦️.ts`'s `takeSegmentedDownloadChunk`), and
 * 4. the host DRAIN (`📤️SegmentedDownload/🟦️.ts`'s `drainSegmentedMediaExport`).
 *
 * Language-agnostic owner: `🧬️schema/🔣️.json`
 * (`https://semio.tech/schema/framework/actor/shard-client/segmented-download/schema.json#/$defs/SegmentedDownloadContract`)
 * + `🧫️fixtures/🔣️.json`. This mirror is asserted field-for-field equal to that fixture — and the Rust
 * constants are asserted equal to the same fixture by the plugin crate's own law — so a literal edited
 * in one hop alone fails closed instead of turning a correct producer into a runtime fault.
 *
 * 🧾️ A zero-import leaf on purpose: the drain element and `ShardClient` both read it, and `ShardClient`
 * cannot reach anything that imports it back (`🎠️kernel/🟦️.ts` imports `ShardClient`). */
export const SEGMENTED_DOWNLOAD_CONTRACT = Object.freeze({
  chunkBytes: 4096,
  maximumOutstandingChunks: 8192,
  maximumTotalBytes: 33_554_432,
  maximumOperationId: 18_446_744_073_709_551_615n,
});

/** 📤️ The refusal every hop raises for one violated bound. Distinct codes on purpose: a chunk that is
 * the wrong TYPE is a wire-adaptation defect (the guest's `option<list<u8>>` reaching the worker as
 * jco's tagged `{ tag, val }` object rather than a `Uint8Array`), while an oversized chunk is a producer
 * that sliced by the wrong constant. Reporting both as one "limit" made the 2026-09-12 battery read as
 * an exceeded byte cap when nothing was over any cap at all. */
export const SEGMENTED_DOWNLOAD_REFUSAL = Object.freeze({
  chunkType: "segmented-download-chunk-type",
  chunkEmpty: "segmented-download-chunk-empty",
  chunkOverCap: "segmented-download-chunk-over-cap",
  outstandingOverCap: "segmented-download-outstanding-over-cap",
  totalOverCap: "segmented-download-total-over-cap",
  authorityInvalid: "segmented-download-authority-invalid",
});

/** 📤️ Admits exactly one drained item against the contract, or names the bound it violated.
 *
 * 🧯 `undefined`/`null` is the producer's TERMINAL sentinel and the only non-chunk this accepts — jco's
 * `{ tag: "none" }` is NOT one, which is why the bridge must unwrap the guest option before any hop
 * reaches here (`pluginComponentBridgeSource`'s `unwrapOption`). */
export function admitSegmentedDownloadChunk(value: unknown): Uint8Array | undefined {
  if (value === undefined || value === null) return undefined;
  if (Object.prototype.toString.call(value) !== "[object Uint8Array]") throw new Error(SEGMENTED_DOWNLOAD_REFUSAL.chunkType);
  const chunk = value as Uint8Array;
  if (chunk.byteLength === 0) throw new Error(SEGMENTED_DOWNLOAD_REFUSAL.chunkEmpty);
  if (chunk.byteLength > SEGMENTED_DOWNLOAD_CONTRACT.chunkBytes) throw new Error(SEGMENTED_DOWNLOAD_REFUSAL.chunkOverCap);
  return chunk;
}

/** 🔢 Admits the runtime's canonical positive decimal operation authority without lossy u64 coercion. */
export function admitSegmentedDownloadOperationId(value: bigint | undefined, instanceId?: number): bigint {
  const instanceValid = instanceId === undefined || (Number.isSafeInteger(instanceId) && instanceId >= 0);
  if (!instanceValid || typeof value !== "bigint" || value <= 0n || value > SEGMENTED_DOWNLOAD_CONTRACT.maximumOperationId) throw new Error(SEGMENTED_DOWNLOAD_REFUSAL.authorityInvalid);
  return value;
}
//#endregion 📤️SegmentedDownloadContract
