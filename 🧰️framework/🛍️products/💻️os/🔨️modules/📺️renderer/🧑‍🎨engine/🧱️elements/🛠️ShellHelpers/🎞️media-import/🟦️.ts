/** 🎞️ Chunked file import and the `requestMediaFrames` WebCodecs / `<video>` tiered decode pipeline.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | ShellHelpers}, which re-exports every public name below so import specifiers stay unchanged.
 */

import { type AppDefinition } from "@semio-tech/framework";
import { hopTrace } from "@semio-tech/framework-trace";
import { type ContinuationCancel, type ContinuationScheduler, hostContinuations } from "@semio-tech/framework/async/continuation";
import { importChunkArguments, importPayloadChunks } from "@semio-tech/framework";
import { requestFileOpen } from "../🔩️plumbing/📥️downloads-and-file-requests/🟦️.ts";
import { type EffectDispatchOne } from "../🔩️plumbing/🚀️effect-dispatch/🟦️.ts";

//#region ShellHelpers
/** 📤️ D3 fan-out: one {@link EffectDispatchOne} call per opened file, and — since wave B59 — one call per
 * `importPayloadChunks` CHUNK of each file, so no single import command asks the guest for a
 * contiguous block above its own per-request ceiling. A file that fits one chunk dispatches exactly one
 * call whose args are the pre-B59 `{payload, name}` plus the chunk envelope naming itself as `0` of `1`.
 *
 * 🧯 The chunks of one file are dispatched in ORDER and awaited one at a time: the guest's staging
 * (`ImportStaging`, `🎠️kernel/🦀️.rs`) refuses a gap rather than resuming into bytes nobody can account for, so a
 * concurrent fan-out would cost the whole file. `progress(completed, total)` counts chunks over every picked file; an
 * aborted `signal` stops before the next chunk and the guest's unfinished run gives its slot to the next pick. */
export async function dispatchOpenedFiles(
  opened: readonly { readonly contents: string; readonly name: string }[],
  importAction: string,
  multiple: boolean,
  dispatchOne: EffectDispatchOne,
  signal?: AbortSignal,
  progress?: (completed: number, total: number) => void,
  retainedArgs?: Readonly<Record<string, unknown>>,
): Promise<void> {
  const total = opened.length;
  const pages = opened.map((file) => importPayloadChunks(file.contents));
  const chunks = pages.reduce((sum, filePages) => sum + filePages.length, 0);
  let completed = 0;
  for (let index = 0; index < opened.length; index += 1) {
    const file = opened[index]!;
    for (const page of pages[index]!) {
      signal?.throwIfAborted();
      await dispatchOne(importAction, { ...retainedArgs, ...importChunkArguments(file.name, page, multiple ? { index, total } : undefined) });
      completed += 1;
      progress?.(completed, chunks);
    }
  }
}

/** ⛔️ The verb an app declares to free an import a host stopped part-way (remodel's `importAbort {reason?}`): a host sends it
 * without a reason — a person's cancel — once a cancelled import has handed the guest at least one chunk, so the open import
 * transaction leaves zero trace instead of refusing every later verb with `toolTransaction.open`. */
export const IMPORT_ABORT_ACTION_ID = "importAbort";

/** 📥️ {@link dispatchOpenedFiles} as one cancellable task: `"done"`, or `"cancelled"` once `signal` stopped it — and then, when
 * a chunk already reached the guest and the app declares {@link IMPORT_ABORT_ACTION_ID}, that abort is dispatched. Any other
 * failure rethrows. */
export async function importOpenedFilesV1(
  app: Pick<AppDefinition, "actions">,
  opened: readonly { readonly contents: string; readonly name: string }[],
  importAction: string,
  multiple: boolean,
  dispatchOne: EffectDispatchOne,
  signal: AbortSignal,
  progress?: (completed: number, total: number) => void,
  retainedArgs?: Readonly<Record<string, unknown>>,
): Promise<"done" | "cancelled"> {
  let delivered = 0;
  try {
    await dispatchOpenedFiles(opened, importAction, multiple, dispatchOne, signal, (completed, total) => {
      delivered = completed;
      progress?.(completed, total);
    }, retainedArgs);
    return "done";
  } catch (error) {
    if (!signal.aborted) throw error;
    if (delivered > 0 && app.actions.some((action) => action.id === IMPORT_ABORT_ACTION_ID)) await dispatchOne(IMPORT_ABORT_ACTION_ID, {});
    return "cancelled";
  }
}

/** 🔁️ D2: schedules `action` onto `dispatchOne` after `delayMs` through the host's ONE
 * {@link ContinuationScheduler} — `delayMs <= 0` is an unthrottled macrotask, never a timer; a
 * positive `delayMs` is a real deadline. Tests pass {@link createContinuationScheduler} over
 * {@link createVirtualContinuationHost}'s ports instead of driving fake timers. Returns the cancel.
 *
 * 🛑️ Why this is not `setTimeout`: every `rearm()` in the flow/generation2d/generation3d command set
 * asks for `delay_ms: 0`, and the host answers each one by re-entering `applyHostEffects` from inside
 * the PREVIOUS dispatch's callback — a textbook nested zero-delay chain. Chrome clamps exactly that
 * shape to ~1 tick/s in a hidden, unfocused or headless renderer, which is where this ticket's
 * measured ~24 s per extension hop came from (two silent ~11-13 s gaps per hop against a 0.58 s
 * native baseline, `📓️audit-extension-hop-latency-2026-09-12.md`). Evaluation must keep converging
 * when nobody is looking at the tab; only paint may stop.
 *
 * 🩺 The scheduled dispatch has no awaiting caller, so its rejection is reported HERE and named — an
 * unnamed unhandled `SemioFaultError` is exactly how the deferred `flowEvalTick` re-arm's
 * `missing field \`locale\`` reached the page with nothing pointing at the dispatch that raised it
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */
export function scheduleDispatchAction(
  action: string,
  args: Record<string, unknown> | undefined,
  delayMs: number,
  dispatchOne: EffectDispatchOne,
  scheduler: ContinuationScheduler = hostContinuations,
): ContinuationCancel {
  const closeArm = hopTrace.open("arm", { action, delayMs });
  return scheduler.schedule(() => {
    closeArm();
    void dispatchOne(action, args).catch((error: unknown) => console.error(`scheduled dispatch of "${action}" failed`, error));
  }, delayMs);
}

//#region RequestMediaFrames
//#region Bmff
/** 🧱️ One parsed ISO-BMFF box: `[type, payloadStart, payloadEnd)` — enough to recurse into containers
 * and slice leaf payloads without copying. */
type BmffBox = { readonly type: string; readonly start: number; readonly end: number };

/** 🧱️ Walks sibling boxes in `[start, end)` — handles 64-bit extended sizes (`size===1`) and to-end
 * boxes (`size===0`); malformed/truncated input just stops early rather than throwing, since MP4
 * probing here is best-effort — the Tier-2 `<video>` fallback covers anything this can't parse. */
function walkBmffBoxes(view: DataView, start: number, end: number): BmffBox[] {
  const boxes: BmffBox[] = [];
  let offset = start;
  while (offset + 8 <= end) {
    const size32 = view.getUint32(offset);
    const type = String.fromCharCode(view.getUint8(offset + 4), view.getUint8(offset + 5), view.getUint8(offset + 6), view.getUint8(offset + 7));
    let headerSize = 8;
    let boxSize = size32;
    if (size32 === 1) {
      if (offset + 16 > end) break;
      boxSize = Number(view.getBigUint64(offset + 8));
      headerSize = 16;
    } else if (size32 === 0) {
      boxSize = end - offset;
    }
    if (boxSize < headerSize || offset + boxSize > end) break;
    boxes.push({ type, start: offset + headerSize, end: offset + boxSize });
    offset += boxSize;
  }
  return boxes;
}

function findBmffBox(boxes: readonly BmffBox[], type: string): BmffBox | undefined {
  return boxes.find((box) => box.type === type);
}
//#endregion Bmff

//#region Tier1
type Mp4Sample = { readonly offset: number; readonly size: number; readonly timestampMs: number; readonly isSync: boolean };
type Mp4Track = {
  readonly width: number;
  readonly height: number;
  readonly codec: "avc1" | "hvc1";
  readonly description: Uint8Array;
  readonly samples: readonly Mp4Sample[];
};

/** 🎞️ Minimal MP4 sample-table extraction — `moov > trak[] > mdia > {mdhd, hdlr, minf > stbl}` for the
 * first video track (`hdlr`'s handler-type `"vide"`), enough to feed `VideoDecoder`: sample byte ranges
 * from `stsc` + `stco`/`co64` + `stsz`, decode timestamps from `stts`, sync flags from `stss` (absent
 * `stss` ⇒ every sample is sync per spec), and the AVC/HEVC decoder config from `stsd`'s `avcC`/`hvcC`.
 * Returns `null` for anything unrecognized (non-AVC/HEVC, missing boxes, malformed tables) so the
 * caller falls back to Tier 2 rather than guessing. */
function probeMp4VideoTrack(bytes: Uint8Array): Mp4Track | null {
  const view = new DataView(bytes.buffer, bytes.byteOffset, bytes.byteLength);
  const moov = findBmffBox(walkBmffBoxes(view, 0, bytes.byteLength), "moov");
  if (!moov) return null;
  for (const trak of walkBmffBoxes(view, moov.start, moov.end).filter((box) => box.type === "trak")) {
    const mdia = findBmffBox(walkBmffBoxes(view, trak.start, trak.end), "mdia");
    if (!mdia) continue;
    const mdiaBoxes = walkBmffBoxes(view, mdia.start, mdia.end);
    const hdlr = findBmffBox(mdiaBoxes, "hdlr");
    if (!hdlr || hdlr.end - hdlr.start < 12) continue;
    const handlerType = String.fromCharCode(view.getUint8(hdlr.start + 8), view.getUint8(hdlr.start + 9), view.getUint8(hdlr.start + 10), view.getUint8(hdlr.start + 11));
    if (handlerType !== "vide") continue;
    const mdhd = findBmffBox(mdiaBoxes, "mdhd");
    const minf = findBmffBox(mdiaBoxes, "minf");
    if (!mdhd || !minf) continue;
    const timescale = view.getUint8(mdhd.start) === 1 ? view.getUint32(mdhd.start + 20) : view.getUint32(mdhd.start + 12);
    if (timescale <= 0) continue;
    const stbl = findBmffBox(walkBmffBoxes(view, minf.start, minf.end), "stbl");
    if (!stbl) continue;
    const track = probeSampleTable(view, walkBmffBoxes(view, stbl.start, stbl.end), timescale);
    if (track) return track;
  }
  return null;
}

function parseStsd(view: DataView, stsd: BmffBox): { width: number; height: number; codec: "avc1" | "hvc1"; description: Uint8Array } | null {
  if (view.getUint32(stsd.start + 4) < 1) return null;
  const entryOffset = stsd.start + 8;
  const entrySize = view.getUint32(entryOffset);
  const format = String.fromCharCode(
    view.getUint8(entryOffset + 4),
    view.getUint8(entryOffset + 5),
    view.getUint8(entryOffset + 6),
    view.getUint8(entryOffset + 7),
  );
  if (format !== "avc1" && format !== "hvc1" && format !== "hev1") return null;
  const codec = format === "avc1" ? "avc1" : "hvc1";
  const visualEntryStart = entryOffset + 8;
  const width = view.getUint16(visualEntryStart + 24);
  const height = view.getUint16(visualEntryStart + 26);
  const inner = walkBmffBoxes(view, visualEntryStart + 78, entryOffset + entrySize);
  const config = findBmffBox(inner, codec === "avc1" ? "avcC" : "hvcC");
  if (!config) return null;
  return { width, height, codec, description: new Uint8Array(view.buffer.slice(config.start, config.end)) };
}

function parseStsz(view: DataView, box: BmffBox): number[] {
  const uniformSize = view.getUint32(box.start + 4);
  const sampleCount = view.getUint32(box.start + 8);
  if (uniformSize !== 0) return new Array(sampleCount).fill(uniformSize) as number[];
  const sizes: number[] = [];
  for (let i = 0; i < sampleCount; i += 1) sizes.push(view.getUint32(box.start + 12 + i * 4));
  return sizes;
}

function parseChunkOffsets(view: DataView, box: BmffBox, is64: boolean): number[] {
  const count = view.getUint32(box.start + 4);
  const offsets: number[] = [];
  for (let i = 0; i < count; i += 1) {
    offsets.push(is64 ? Number(view.getBigUint64(box.start + 8 + i * 8)) : view.getUint32(box.start + 8 + i * 4));
  }
  return offsets;
}

function parseChunkOfSample(view: DataView, box: BmffBox, sampleCount: number, chunkCount: number): number[] | null {
  const entryCount = view.getUint32(box.start + 4);
  const entries: { firstChunk: number; samplesPerChunk: number }[] = [];
  for (let i = 0; i < entryCount; i += 1) {
    entries.push({ firstChunk: view.getUint32(box.start + 8 + i * 12), samplesPerChunk: view.getUint32(box.start + 12 + i * 12) });
  }
  const chunkOfSample: number[] = [];
  for (let entryIndex = 0; entryIndex < entries.length; entryIndex += 1) {
    const entry = entries[entryIndex]!;
    const nextFirstChunk = entries[entryIndex + 1]?.firstChunk ?? chunkCount + 1;
    for (let chunk = entry.firstChunk; chunk < nextFirstChunk; chunk += 1) {
      for (let inChunk = 0; inChunk < entry.samplesPerChunk; inChunk += 1) chunkOfSample.push(chunk);
    }
  }
  return chunkOfSample.length >= sampleCount ? chunkOfSample : null;
}

function computeSampleOffsets(chunkOfSample: readonly number[], chunkOffsets: readonly number[], sizes: readonly number[]): number[] {
  const offsets: number[] = [];
  const cursorByChunk = new Map<number, number>();
  for (let i = 0; i < sizes.length; i += 1) {
    const chunk = chunkOfSample[i]!;
    const base = cursorByChunk.get(chunk) ?? chunkOffsets[chunk - 1] ?? 0;
    offsets.push(base);
    cursorByChunk.set(chunk, base + sizes[i]!);
  }
  return offsets;
}

function accumulateTimestampsMs(view: DataView, stts: BmffBox, sampleCount: number, timescale: number): number[] {
  const entryCount = view.getUint32(stts.start + 4);
  const timestamps: number[] = [];
  let ticks = 0;
  for (let entryIndex = 0; entryIndex < entryCount && timestamps.length < sampleCount; entryIndex += 1) {
    const count = view.getUint32(stts.start + 8 + entryIndex * 8);
    const delta = view.getUint32(stts.start + 12 + entryIndex * 8);
    for (let i = 0; i < count && timestamps.length < sampleCount; i += 1) {
      timestamps.push((ticks / timescale) * 1000);
      ticks += delta;
    }
  }
  return timestamps;
}

function parseSyncSamples(view: DataView, box: BmffBox): Set<number> {
  const count = view.getUint32(box.start + 4);
  const sync = new Set<number>();
  for (let i = 0; i < count; i += 1) sync.add(view.getUint32(box.start + 8 + i * 4));
  return sync;
}

function probeSampleTable(view: DataView, stblBoxes: readonly BmffBox[], timescale: number): Mp4Track | null {
  const stsd = findBmffBox(stblBoxes, "stsd");
  const stts = findBmffBox(stblBoxes, "stts");
  const stsc = findBmffBox(stblBoxes, "stsc");
  const stsz = findBmffBox(stblBoxes, "stsz");
  const stco = findBmffBox(stblBoxes, "stco") ?? findBmffBox(stblBoxes, "co64");
  if (!stsd || !stts || !stsc || !stsz || !stco) return null;
  const entry = parseStsd(view, stsd);
  if (!entry) return null;
  const sizes = parseStsz(view, stsz);
  const offsets = parseChunkOffsets(view, stco, stco.type === "co64");
  const chunkOfSample = parseChunkOfSample(view, stsc, sizes.length, offsets.length);
  if (!chunkOfSample) return null;
  const sampleOffsets = computeSampleOffsets(chunkOfSample, offsets, sizes);
  const timestampsMs = accumulateTimestampsMs(view, stts, sizes.length, timescale);
  const stss = findBmffBox(stblBoxes, "stss");
  const syncSamples = stss ? parseSyncSamples(view, stss) : null;
  const samples: Mp4Sample[] = sizes.map((size, index) => ({
    offset: sampleOffsets[index]!,
    size,
    timestampMs: timestampsMs[index] ?? 0,
    isSync: syncSamples ? syncSamples.has(index + 1) : true,
  }));
  return { width: entry.width, height: entry.height, codec: entry.codec, description: entry.description, samples };
}

/** 🌐️ Feature-detects the WebCodecs `VideoDecoder`/`EncodedVideoChunk` globals (Tier 1's prerequisite;
 * absent in most JS test environments and in browsers that only support WebM/VP9 without an AVC path). */
function webCodecsAvailable(): boolean {
  const scope = window as unknown as { VideoDecoder?: unknown; EncodedVideoChunk?: unknown };
  return typeof scope.VideoDecoder === "function" && typeof scope.EncodedVideoChunk === "function";
}

/** 🔢️ Derives a WebCodecs `avc1.PPCCLL` codec string from an `avcC` box's profile/compat/level bytes
 * (offsets 1/2/3 — version is byte 0). */
function avcCodecString(description: Uint8Array): string {
  const hex = (byte: number | undefined) => (byte ?? 0).toString(16).padStart(2, "0");
  return `avc1.${hex(description[1])}${hex(description[2])}${hex(description[3])}`;
}

type WebCodecsVideoFrame = { readonly codedWidth: number; readonly codedHeight: number; close: () => void };
type WebCodecsVideoDecoderCtor = new (init: { output: (frame: WebCodecsVideoFrame) => void; error: (error: unknown) => void }) => {
  configure: (config: { codec: string; codedWidth: number; codedHeight: number; description: Uint8Array }) => void;
  decode: (chunk: unknown) => void;
  flush: () => Promise<void>;
  close: () => void;
};
type WebCodecsEncodedVideoChunkCtor = new (init: { type: "key" | "delta"; timestamp: number; data: Uint8Array }) => unknown;

function jpegDataUrlFromFrame(frame: WebCodecsVideoFrame): { readonly dataUrl: string; readonly width: number; readonly height: number } {
  const canvas = document.createElement("canvas");
  canvas.width = frame.codedWidth;
  canvas.height = frame.codedHeight;
  canvas.getContext("2d")?.drawImage(frame as unknown as CanvasImageSource, 0, 0);
  return { dataUrl: canvas.toDataURL("image/jpeg", 0.9), width: frame.codedWidth, height: frame.codedHeight };
}

/** 🎞️ Decodes exactly the samples needed for one target frame — from its nearest preceding sync sample
 * through the target — via a fresh `VideoDecoder`, capturing only the last output frame. Simplification:
 * each target frame re-decodes its GOP prefix from scratch instead of streaming continuously across
 * targets and demuxing outputs by timestamp; acceptable because sampled ingestion (`sampleStride`/
 * `maxFrames`) keeps GOP prefixes short between targets, and Tier 2's `<video>` element is always the
 * correctness fallback if Tier 1 fails or the codec isn't baseline-friendly. */
async function decodeOneMp4Frame(track: Mp4Track, bytes: Uint8Array, targetIndex: number): Promise<{ dataUrl: string; width: number; height: number } | null> {
  const scope = window as unknown as { VideoDecoder: WebCodecsVideoDecoderCtor; EncodedVideoChunk: WebCodecsEncodedVideoChunkCtor };
  let syncIndex = targetIndex;
  while (syncIndex > 0 && !track.samples[syncIndex]!.isSync) syncIndex -= 1;
  let captured: { dataUrl: string; width: number; height: number } | null = null;
  await new Promise<void>((resolve, reject) => {
    const decoder = new scope.VideoDecoder({
      output: (frame) => {
        captured = jpegDataUrlFromFrame(frame);
        frame.close();
      },
      error: reject,
    });
    decoder.configure({ codec: avcCodecString(track.description), codedWidth: track.width, codedHeight: track.height, description: track.description });
    for (let i = syncIndex; i <= targetIndex; i += 1) {
      const sample = track.samples[i]!;
      decoder.decode(
        new scope.EncodedVideoChunk({ type: sample.isSync ? "key" : "delta", timestamp: sample.timestampMs * 1000, data: bytes.subarray(sample.offset, sample.offset + sample.size) }),
      );
    }
    decoder.flush().then(() => {
      decoder.close();
      resolve();
    }, reject);
  });
  return captured;
}

/** 🎞️ Tier 1 orchestration: demuxes `bytes` as MP4/AVC, decodes one frame per sampled timestamp, and
 * dispatches `frameAction` per frame + `doneAction` once, reporting each dispatched frame to `run` and stopping with
 * `run.signal`'s reason before the next frame once it is aborted. Returns `false` (no dispatch performed at all) when the
 * demux can't find a usable AVC video track, so the caller falls through to Tier 2. */
async function runTier1VideoFrames(bytes: Uint8Array, effect: RequestMediaFramesArgs, name: string, dispatchOne: EffectDispatchOne, run: MediaFramesRunV1): Promise<boolean> {
  const track = probeMp4VideoTrack(bytes);
  if (!track || track.samples.length === 0) return false;
  const durationMs = track.samples[track.samples.length - 1]!.timestampMs;
  const timestamps = sampleMediaFrameTimestampsMs(durationMs, effect.sampleStride, effect.maxFrames, effect.fpsHint);
  let sampledCount = 0;
  for (let index = 0; index < timestamps.length; index += 1) {
    run.signal.throwIfAborted();
    const targetMs = timestamps[index]!;
    let targetSampleIndex = 0;
    for (let i = 0; i < track.samples.length; i += 1) if (track.samples[i]!.timestampMs <= targetMs) targetSampleIndex = i;
    const frame = await decodeOneMp4Frame(track, bytes, targetSampleIndex);
    if (!frame) continue;
    sampledCount += 1;
    await dispatchOne(effect.frameAction, {
      payload: frame.dataUrl,
      name,
      frameIndex: index,
      timestampMs: targetMs,
      index,
      total: timestamps.length,
      width: frame.width,
      height: frame.height,
      ...effect.args,
    });
    run.frame(index + 1, timestamps.length);
  }
  run.signal.throwIfAborted();
  await dispatchOne(effect.doneAction, {
    name,
    durationMs,
    frameCount: track.samples.length,
    sampledCount,
    width: track.width,
    height: track.height,
    codec: track.codec,
    ...effect.args,
  });
  return true;
}
//#endregion Tier1

//#region Tier2
/** ⏱️ Tier-2 (`<video>` seek-and-capture) target timestamps, ms — one every `sampleStride /
 * (fpsHint || 30)` seconds starting at 0, capped at `maxFrames` (0 ⇒ unlimited, bounded only by
 * `durationMs`). Pure/deterministic so it's unit-testable without any DOM or media APIs. Computes each
 * timestamp as `k * stepMs` rather than an accumulating `ts += stepMs` loop — repeated float addition
 * drifts enough over dozens of steps to occasionally land just under an exact multiple of `durationMs`,
 * sneaking in one extra timestamp; multiplying from the loop index is exact per-step and deterministic. */
export function sampleMediaFrameTimestampsMs(durationMs: number, sampleStride: number, maxFrames: number, fpsHint: number): number[] {
  const stride = sampleStride > 0 ? sampleStride : 1;
  const fps = fpsHint > 0 ? fpsHint : 30;
  const stepMs = (stride / fps) * 1000;
  const timestamps: number[] = [];
  if (durationMs <= 0 || stepMs <= 0) return timestamps;
  for (let k = 0; ; k += 1) {
    if (maxFrames > 0 && timestamps.length >= maxFrames) break;
    const ts = k * stepMs;
    if (ts >= durationMs) break;
    timestamps.push(ts);
  }
  return timestamps;
}

function captureCanvasFrame(video: HTMLVideoElement, maxLongEdgePx: number): { readonly dataUrl: string; readonly width: number; readonly height: number } {
  const sourceWidth = video.videoWidth || 0;
  const sourceHeight = video.videoHeight || 0;
  const scale = maxLongEdgePx > 0 ? Math.min(1, maxLongEdgePx / Math.max(sourceWidth, sourceHeight, 1)) : 1;
  const width = Math.max(1, Math.round(sourceWidth * scale));
  const height = Math.max(1, Math.round(sourceHeight * scale));
  const canvas = document.createElement("canvas");
  canvas.width = width;
  canvas.height = height;
  canvas.getContext("2d")?.drawImage(video, 0, 0, width, height);
  return { dataUrl: canvas.toDataURL("image/jpeg", 0.9), width, height };
}

function waitForVideoEvent(video: HTMLVideoElement, type: string, signal?: AbortSignal): Promise<void> {
  return new Promise((resolve, reject) => {
    if (signal?.aborted) return reject(signal.reason);
    const abort = () => {
      video.removeEventListener(type, handler);
      reject(signal!.reason);
    };
    const handler = () => {
      video.removeEventListener(type, handler);
      signal?.removeEventListener("abort", abort);
      resolve();
    };
    video.addEventListener(type, handler);
    signal?.addEventListener("abort", abort, { once: true });
  });
}

/** 🎞️ Tier 2 orchestration: waits for `loadedmetadata` (if not already available), seeks `video`
 * through {@link sampleMediaFrameTimestampsMs}'s schedule, captures each landed frame to a scaled JPEG
 * data URL, dispatches `frameAction` per frame, then `doneAction` once. Used both as the WebM/no-
 * WebCodecs fallback and directly by tests (which inject a real `<video>` element with overridden
 * `duration`/`videoWidth`/`videoHeight`/`readyState` and manually dispatch `loadedmetadata`/`seeked`,
 * since headless test environments have no real media decoder). */
export async function runTier2VideoFrames(video: HTMLVideoElement, effect: RequestMediaFramesArgs, name: string, dispatchOne: EffectDispatchOne, run: MediaFramesRunV1 = MEDIA_FRAMES_UNCANCELLABLE_RUN_V1): Promise<void> {
  if (video.readyState < 1) await waitForVideoEvent(video, "loadedmetadata", run.signal);
  const durationMs = Number.isFinite(video.duration) ? video.duration * 1000 : 0;
  const width = video.videoWidth || 0;
  const height = video.videoHeight || 0;
  const timestamps = sampleMediaFrameTimestampsMs(durationMs, effect.sampleStride, effect.maxFrames, effect.fpsHint);
  const total = timestamps.length;
  for (let index = 0; index < total; index += 1) {
    run.signal.throwIfAborted();
    const timestampMs = timestamps[index]!;
    video.currentTime = timestampMs / 1000;
    await waitForVideoEvent(video, "seeked", run.signal);
    const frame = captureCanvasFrame(video, effect.maxLongEdgePx);
    await dispatchOne(effect.frameAction, {
      payload: frame.dataUrl,
      name,
      frameIndex: index,
      timestampMs,
      index,
      total,
      width: frame.width,
      height: frame.height,
      ...effect.args,
    });
    run.frame(index + 1, total);
  }
  run.signal.throwIfAborted();
  await dispatchOne(effect.doneAction, { name, durationMs, frameCount: total, sampledCount: total, width, height, codec: "unknown", ...effect.args });
}
//#endregion Tier2

/** ⏹️ What a media-frames decode reports and obeys while it runs: each dispatched frame (`completed` of `total`) and the
 * signal that stops it before the next frame. */
export type MediaFramesRunV1 = { readonly signal: AbortSignal; readonly frame: (completed: number, total: number) => void };

/** ⏹️ A run nothing cancels — a decode driven directly (tests, a tier on its own). */
const MEDIA_FRAMES_UNCANCELLABLE_RUN_V1: MediaFramesRunV1 = { signal: new AbortController().signal, frame: () => {} };

/** 🎞️ D5 `RequestMediaFrames` fields the two decode tiers need, decoupled from the raw `Effect`
 * union member shape so orchestration functions above take a plain, easily-constructed-in-tests object. */
export type RequestMediaFramesArgs = {
  readonly frameAction: string;
  readonly doneAction: string;
  readonly fallbackAction: string;
  readonly sampleStride: number;
  readonly maxFrames: number;
  readonly maxLongEdgePx: number;
  readonly fpsHint: number;
  readonly args?: Record<string, unknown>;
};

function bytesFromDataUrl(dataUrl: string): Uint8Array {
  const binary = atob(dataUrl.slice(dataUrl.indexOf(",") + 1));
  const bytes = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i += 1) bytes[i] = binary.charCodeAt(i);
  return bytes;
}

function bytesToDataUrl(bytes: Uint8Array, mime: string): string {
  let binary = "";
  for (let i = 0; i < bytes.length; i += 1) binary += String.fromCharCode(bytes[i]!);
  return `data:${mime};base64,${btoa(binary)}`;
}

/** 🎞️ D5 source: the video a `RequestMediaFrames` decodes — its `payload` data URL already in hand (a drop zone; named
 * `video`), else the one file the native picker returns; `null` when the person picks nothing. */
export async function requestMediaFramesSourceV1(accept: string, payload: string | undefined): Promise<{ readonly bytes: Uint8Array; readonly name: string } | null> {
  if (payload) return { bytes: bytesFromDataUrl(payload), name: "video" };
  const opened = await requestFileOpen(accept || "video/*", "dataUrl", false);
  return opened.length === 0 ? null : { bytes: bytesFromDataUrl(opened[0]!.contents), name: opened[0]!.name };
}

/** 🎞️ D5 as one cancellable task: tries Tier 1 when WebCodecs is available and the demux finds a usable AVC track, otherwise
 * Tier 2's `<video>` seek-and-capture, reporting each dispatched frame to `progress`; on total failure (can't demux AND Tier 2
 * also throws, e.g. a corrupt file) dispatches `fallbackAction` once with the raw original bytes as a data URL. `signal` stops it
 * before the next frame: `"cancelled"`, no done or fallback dispatch, and — when a frame already reached the guest and the app
 * declares {@link IMPORT_ABORT_ACTION_ID} — that abort, so the streamed import transaction leaves zero trace. */
export async function runMediaFramesV1(
  effect: RequestMediaFramesArgs,
  source: { readonly bytes: Uint8Array; readonly name: string },
  dispatchOne: EffectDispatchOne,
  app: Pick<AppDefinition, "actions">,
  signal: AbortSignal,
  progress?: (completed: number, total: number) => void,
  createVideoElement: () => HTMLVideoElement = () => document.createElement("video"),
): Promise<"done" | "cancelled"> {
  const { bytes, name } = source;
  let delivered = 0;
  const run: MediaFramesRunV1 = {
    signal,
    frame: (completed, total) => {
      delivered = completed;
      progress?.(completed, total);
    },
  };
  const cancelled = async (): Promise<"cancelled"> => {
    if (delivered > 0 && app.actions.some((action) => action.id === IMPORT_ABORT_ACTION_ID)) await dispatchOne(IMPORT_ABORT_ACTION_ID, {});
    return "cancelled";
  };
  try {
    signal.throwIfAborted();
    if (webCodecsAvailable() && (await runTier1VideoFrames(bytes, effect, name, dispatchOne, run))) return "done";
    const url = URL.createObjectURL(new Blob([new Uint8Array(bytes)], { type: "video/mp4" }));
    const video = createVideoElement();
    video.muted = true;
    video.playsInline = true;
    video.src = url;
    try {
      await runTier2VideoFrames(video, effect, name, dispatchOne, run);
    } finally {
      URL.revokeObjectURL(url);
    }
    return "done";
  } catch (error) {
    if (signal.aborted) return cancelled();
    console.error("[os-shell] requestMediaFrames: decode failed, falling back to raw bytes", error);
    await dispatchOne(effect.fallbackAction, { payload: bytesToDataUrl(bytes, "video/mp4"), name, ...effect.args });
    return "done";
  }
}
//#endregion RequestMediaFrames
//#endregion ShellHelpers
