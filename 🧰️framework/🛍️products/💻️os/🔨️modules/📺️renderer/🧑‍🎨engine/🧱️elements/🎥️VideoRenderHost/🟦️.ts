// #region 🧲️Header
// 🎨️ framework/products/os/modules/renderer/engine/elements/VideoRenderHost/module.ts
/** @emoji 🎥️ `🎥️VideoRenderHost` — the browser host of the `media.video-render` capability (`Effect::VideoRenderExport`,
 * `🎠️kernel`): a guest has no canvas and no encoder, so a plugin that requested {@link MEDIA_VIDEO_RENDER_CAPABILITY} hands
 * the host a `VideoRenderProgram`; the host paints every distinct scene once on a 2D canvas, encodes the timeline as H.264
 * and muxes an MP4 with the raster video tier's first-party writer (`🖌️raster/🎥️video/🟦️.ts`).
 *
 * Two encoder tiers behind ports: the platform's own H.264 encoder (WebCodecs `VideoEncoder`, hardware where the OS has
 * one) whenever the page supports the stream's configuration, otherwise the tier's first-party `AvcPcmEncoder` (exact
 * `I_PCM` pictures, `P_Skip` repeats). Every export is an event-sourced job: the host appends
 * `VideoRenderJobEvent`s to one log and the Task Manager's rows are the kernel ledger's fold of it; a cancel appends
 * `cancelRequested`, which the render loop reads back from the ledger before its next frame. React-free, so its laws run
 * without a browser (`🧪️tests/🔬️unit/🟦️.ts`).
 */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { MEDIA_VIDEO_RENDER_CAPABILITY, type VideoRenderEncoderTier, type VideoRenderJobEvent, VideoRenderJobLedger, type VideoRenderJobOutcome, type VideoRenderJobRow, type VideoRenderPath, type VideoRenderProgram, videoRenderFrameCount, videoRenderProgramProblem } from "../../../../../../../🔨️modules/🎠️kernel/🟦️.ts";
import { AvcPcmEncoder, avcConfigurationFromRecord, avcLevelIdc, type AvcDecoderConfiguration, type EncodedVideoSample, type VideoStreamParameters, writeAvcMp4 } from "../../../../../../../🔨️modules/🖌️raster/🎥️video/🟦️.ts";
// #endregion 🔌️Adapters

//#region 🌐️Labels
/** 🌐️ What the host tells the user about an export, in every shell language (English first, German second). */
export const VIDEO_RENDER_EXPORT_TEXT_V1 = {
  started: { en: "Rendering video {filename} — progress and cancel in Task Manager.", de: "Video {filename} wird gerendert — Fortschritt und Abbruch im Task-Manager." },
  done: { en: "Video {filename} exported ({frames} frames).", de: "Video {filename} exportiert ({frames} Bilder)." },
  cancelled: { en: "Video export {filename} cancelled.", de: "Videoexport {filename} abgebrochen." },
  refused: { en: "Video export {filename} refused: the program is invalid ({code}).", de: "Videoexport {filename} abgelehnt: das Programm ist ungültig ({code})." },
  refusedCapability: { en: "Video export {filename} refused: {owner} did not request the {capability} capability.", de: "Videoexport {filename} abgelehnt: {owner} hat die Fähigkeit {capability} nicht angefordert." },
  failed: { en: "Video export {filename} failed: {reason}", de: "Videoexport {filename} fehlgeschlagen: {reason}" },
  progress: { en: "Frame {completed} of {total}", de: "Bild {completed} von {total}" },
} as const;

/** 🌐️ One {@link VIDEO_RENDER_EXPORT_TEXT_V1} message in `locale` with its `{placeholders}` filled. */
export function videoRenderExportTextV1(kind: keyof typeof VIDEO_RENDER_EXPORT_TEXT_V1, locale: string, values: Readonly<Record<string, string | number>>): string {
  const template = locale === "de" ? VIDEO_RENDER_EXPORT_TEXT_V1[kind].de : VIDEO_RENDER_EXPORT_TEXT_V1[kind].en;
  return template.replace(/\{(\w+)\}/g, (match, key: string) => (key in values ? String(values[key]) : match));
}
//#endregion 🌐️Labels

//#region 🖌️Paint
/** 🖼️ A decoded picture the host draws from (`ImageBitmap`, `HTMLImageElement`, …) with its pixel size. */
export type VideoRenderImageSourceV1 = Readonly<{ source: CanvasImageSource; width: number; height: number }>;

/** 🖌️ The 2D context surface this host paints on (`OffscreenCanvasRenderingContext2D` or a DOM canvas's). */
export type VideoRenderContext2dV1 = Pick<CanvasRenderingContext2D, "setTransform" | "clearRect" | "fillRect" | "fill" | "stroke" | "drawImage" | "getImageData"> & { fillStyle: unknown; strokeStyle: unknown; lineWidth: number; globalAlpha: number };

/** ✏️ A program path as a `Path2D` (`M`, `L`, `Q`, `C`, `Z` over flat points). */
export function videoRenderPath2dV1(path: VideoRenderPath, create: () => Path2D = () => new Path2D()): Path2D {
  const shape = create();
  let cursor = 0;
  const next = (): number => path.points[cursor++] ?? 0;
  for (const verb of path.verbs) {
    if (verb === "M") shape.moveTo(next(), next());
    else if (verb === "L") shape.lineTo(next(), next());
    else if (verb === "Q") shape.quadraticCurveTo(next(), next(), next(), next());
    else if (verb === "C") shape.bezierCurveTo(next(), next(), next(), next(), next(), next());
    else if (verb === "Z") shape.closePath();
  }
  return shape;
}

/** 🎨️ Straight RGBA 0..=1 as a CSS colour. */
export function videoRenderCssColorV1(color: readonly number[]): string {
  const channel = (value: number | undefined): number => Math.round(Math.min(1, Math.max(0, value ?? 0)) * 255);
  return `rgba(${channel(color[0])}, ${channel(color[1])}, ${channel(color[2])}, ${Math.min(1, Math.max(0, color[3] ?? 1))})`;
}

/** 🖼️ Paints scene `sceneIndex` over the program background, in list order, each op under its own device transform; an
 * image op draws its crop of the picture onto the unit square its transform places. */
export function drawVideoRenderSceneV1(context: VideoRenderContext2dV1, program: VideoRenderProgram, sceneIndex: number, paths: readonly Path2D[], images: readonly VideoRenderImageSourceV1[]): void {
  context.setTransform(1, 0, 0, 1, 0, 0);
  context.globalAlpha = 1;
  context.clearRect(0, 0, program.width, program.height);
  context.fillStyle = videoRenderCssColorV1(program.background);
  context.fillRect(0, 0, program.width, program.height);
  for (const op of program.scenes[sceneIndex]?.ops ?? []) {
    context.setTransform(op.transform[0], op.transform[1], op.transform[2], op.transform[3], op.transform[4], op.transform[5]);
    if (op.kind === "image") {
      const image = images[op.image];
      if (!image) continue;
      context.globalAlpha = op.opacity;
      context.drawImage(image.source, op.crop[0] * image.width, op.crop[1] * image.height, op.crop[2] * image.width, op.crop[3] * image.height, 0, 0, 1, 1);
      context.globalAlpha = 1;
      continue;
    }
    const path = paths[op.path];
    if (!path) continue;
    if (op.kind === "stroke") {
      context.strokeStyle = videoRenderCssColorV1(op.color);
      context.lineWidth = op.width;
      context.stroke(path);
    } else {
      context.fillStyle = videoRenderCssColorV1(op.color);
      context.fill(path, "nonzero");
    }
  }
}
//#endregion 🖌️Paint

//#region 🎞️Encode
/** 🌍️ What the host environment offers; injected so the laws run the whole pipeline without a browser. */
export interface VideoRenderEnvironmentV1 {
  readonly createCanvas: (width: number, height: number) => { readonly context: VideoRenderContext2dV1; readonly source: unknown };
  readonly createPath: () => Path2D;
  readonly loadImage: (url: string) => Promise<VideoRenderImageSourceV1>;
  readonly webCodecs: WebCodecsVideoEncoderPortV1 | null;
  readonly yieldToHost: () => Promise<void>;
  readonly now: () => number;
}

/** 🔌️ The slice of WebCodecs this host drives (`VideoEncoder`, `VideoFrame`) — the platform encoder port, injectable for laws. */
export interface WebCodecsVideoEncoderPortV1 {
  isConfigSupported(config: WebCodecsVideoConfigV1): Promise<boolean>;
  open(config: WebCodecsVideoConfigV1, output: (sample: EncodedVideoSample, description: Uint8Array | null) => void, error: (reason: unknown) => void): WebCodecsSessionV1;
}

/** 🎛️ The encoder configuration this host asks the platform for. */
export type WebCodecsVideoConfigV1 = Readonly<{ codec: string; width: number; height: number; framerate: number; avc: { readonly format: "avc" } }>;

/** 🎛️ One open platform encoder. */
export interface WebCodecsSessionV1 {
  encode(source: unknown, timestampMicros: number, durationMicros: number, keyFrame: boolean): void;
  readonly queueSize: number;
  flush(): Promise<void>;
  close(): void;
}

/** 🚨️ Why a render stopped short of a file. */
export class VideoRenderExportErrorV1 extends Error {
  constructor(
    readonly code: "cancelled" | "encoder",
    readonly detail: string,
  ) {
    super(`${code}: ${detail}`);
  }
}

/** 📦️ A finished render. */
export interface VideoRenderResultV1 {
  readonly bytes: Uint8Array;
  readonly tier: VideoRenderEncoderTier;
  readonly frames: number;
}

/** 🔤️ The WebCodecs codec string asked for: Constrained Baseline at the level the stream needs. */
export function videoRenderWebCodecsCodecV1(parameters: VideoStreamParameters): string {
  return `avc1.42E0${avcLevelIdc(parameters).toString(16).toUpperCase().padStart(2, "0")}`;
}

/** ⏱️ The longest stretch the render loop keeps the page's thread before yielding (the interactive step ceiling). */
export const VIDEO_RENDER_YIELD_BUDGET_MS = 8;

/** 🎬️ Renders and encodes an ADMITTED `program` into MP4 bytes: each timeline run painted once, one encode per frame
 * (a repeat per further frame of a run), `progress` after every frame, `cancelled()` read before every frame. */
export async function renderVideoProgramV1(program: VideoRenderProgram, environment: VideoRenderEnvironmentV1, progress: (completed: number, total: number) => void, cancelled: () => boolean): Promise<VideoRenderResultV1> {
  const parameters: VideoStreamParameters = { width: program.width, height: program.height, fps: program.fps };
  const total = videoRenderFrameCount(program);
  const canvas = environment.createCanvas(program.width, program.height);
  const paths = program.paths.map((path) => videoRenderPath2dV1(path, environment.createPath));
  const images = await Promise.all(program.images.map((image) => environment.loadImage(image.url)));
  const config: WebCodecsVideoConfigV1 = { codec: videoRenderWebCodecsCodecV1(parameters), width: program.width, height: program.height, framerate: program.fps, avc: { format: "avc" } };
  const webCodecs = environment.webCodecs !== null && (await environment.webCodecs.isConfigSupported(config).catch(() => false)) ? environment.webCodecs : null;
  const samples: EncodedVideoSample[] = [];
  let description: Uint8Array | null = null;
  let failure: unknown = null;
  const session = webCodecs?.open(
    config,
    (sample, record) => {
      samples.push(sample);
      if (record !== null) description = record;
    },
    (reason) => {
      failure = reason;
    },
  );
  const firstParty = session ? null : new AvcPcmEncoder(parameters);
  const frameMicros = 1_000_000 / program.fps;
  const keyInterval = program.fps * 2;
  let frame = 0;
  let yieldedAt = environment.now();
  try {
    for (const run of program.timeline) {
      drawVideoRenderSceneV1(canvas.context, program, run.scene, paths, images);
      for (let repeat = 0; repeat < run.frames; repeat += 1) {
        if (cancelled()) throw new VideoRenderExportErrorV1("cancelled", `${frame}/${total}`);
        if (failure !== null) throw new VideoRenderExportErrorV1("encoder", String(failure));
        if (session) {
          session.encode(canvas.source, Math.round(frame * frameMicros), Math.round(frameMicros), frame % keyInterval === 0 || repeat === 0);
          while (session.queueSize > 4) await environment.yieldToHost();
        } else if (firstParty) {
          samples.push(repeat === 0 ? firstParty.encode(canvas.context.getImageData(0, 0, program.width, program.height).data) : firstParty.repeat());
        }
        frame += 1;
        progress(frame, total);
        if (repeat === 0 || environment.now() - yieldedAt >= VIDEO_RENDER_YIELD_BUDGET_MS) {
          await environment.yieldToHost();
          yieldedAt = environment.now();
        }
      }
    }
    if (session) await session.flush();
    if (failure !== null) throw new VideoRenderExportErrorV1("encoder", String(failure));
  } finally {
    session?.close();
  }
  const configuration: AvcDecoderConfiguration | null = firstParty ? firstParty.configuration() : description ? avcConfigurationFromRecord(description) : null;
  if (configuration === null || samples.length !== total) throw new VideoRenderExportErrorV1("encoder", `encoder answered ${samples.length} of ${total} frames${configuration === null ? " and no avcC" : ""}`);
  return { bytes: writeAvcMp4(parameters, configuration, samples), tier: session ? "platform" : "first-party", frames: total };
}

/** 🌍️ The page's own environment: `OffscreenCanvas` (or a detached `<canvas>`), `Path2D`, `createImageBitmap` over a
 * same-origin `fetch`, WebCodecs when present. */
export function browserVideoRenderEnvironmentV1(): VideoRenderEnvironmentV1 {
  const scope = globalThis as unknown as {
    readonly OffscreenCanvas?: new (width: number, height: number) => { getContext(kind: "2d", options: { willReadFrequently: boolean }): VideoRenderContext2dV1 | null };
    readonly VideoEncoder?: { new (init: { output: (chunk: { readonly type: string; readonly byteLength: number; copyTo(target: Uint8Array): void }, metadata?: { readonly decoderConfig?: { readonly description?: AllowSharedBufferSource } }) => void; error: (error: unknown) => void }): { configure(config: object): void; encode(frame: unknown, options: { keyFrame: boolean }): void; readonly encodeQueueSize: number; flush(): Promise<void>; close(): void; readonly state: string }; isConfigSupported(config: object): Promise<{ readonly supported?: boolean }> };
    readonly VideoFrame?: new (source: unknown, init: { timestamp: number; duration: number }) => { close(): void };
    readonly document?: { createElement(tag: "canvas"): { width: number; height: number; getContext(kind: "2d", options: { willReadFrequently: boolean }): VideoRenderContext2dV1 | null } };
  };
  const VideoEncoderCtor = scope.VideoEncoder;
  const VideoFrameCtor = scope.VideoFrame;
  const webCodecs: WebCodecsVideoEncoderPortV1 | null =
    VideoEncoderCtor && VideoFrameCtor
      ? {
          isConfigSupported: async (config) => (await VideoEncoderCtor.isConfigSupported(config)).supported === true,
          open: (config, output, error) => {
            const encoder = new VideoEncoderCtor({
              output: (chunk, metadata) => {
                const data = new Uint8Array(chunk.byteLength);
                chunk.copyTo(data);
                const description = metadata?.decoderConfig?.description;
                output({ data, sync: chunk.type === "key" }, description ? new Uint8Array(ArrayBuffer.isView(description) ? description.buffer.slice(description.byteOffset, description.byteOffset + description.byteLength) : description) : null);
              },
              error,
            });
            encoder.configure({ ...config, latencyMode: "quality" });
            return {
              encode: (source, timestamp, duration, keyFrame) => {
                const frame = new VideoFrameCtor(source, { timestamp, duration });
                try {
                  encoder.encode(frame, { keyFrame });
                } finally {
                  frame.close();
                }
              },
              get queueSize() {
                return encoder.encodeQueueSize;
              },
              flush: () => encoder.flush(),
              close: () => {
                if (encoder.state !== "closed") encoder.close();
              },
            };
          },
        }
      : null;
  return {
    createCanvas: (width, height) => {
      if (scope.OffscreenCanvas) {
        const source = new scope.OffscreenCanvas(width, height);
        const context = source.getContext("2d", { willReadFrequently: webCodecs === null });
        if (context) return { context, source };
      }
      const element = scope.document?.createElement("canvas");
      if (!element) throw new VideoRenderExportErrorV1("encoder", "no 2D canvas in this environment");
      element.width = width;
      element.height = height;
      const context = element.getContext("2d", { willReadFrequently: webCodecs === null });
      if (!context) throw new VideoRenderExportErrorV1("encoder", "no 2D canvas context in this environment");
      return { context, source: element };
    },
    createPath: () => new Path2D(),
    loadImage: async (url) => {
      const response = await fetch(url);
      if (!response.ok) throw new VideoRenderExportErrorV1("encoder", `image ${url} answered ${response.status}`);
      const bitmap = await createImageBitmap(await response.blob());
      return { source: bitmap, width: bitmap.width, height: bitmap.height };
    },
    webCodecs,
    yieldToHost: () => new Promise((resolve) => setTimeout(resolve, 0)),
    now: () => performance.now(),
  };
}
//#endregion 🎞️Encode

//#region 🧵️Jobs
/** 🧵️ How many job events this tab keeps readable after the ledger has folded them (the ledger itself is the snapshot). */
export const VIDEO_RENDER_JOB_LOG_TAIL = 256;

const ledger = new VideoRenderJobLedger();
const log: VideoRenderJobEvent[] = [];
const listeners = new Set<() => void>();

/** ➕️ The ONE write door of this host's task state: folds `event` into the ledger and keeps it in the log tail; a refused
 * event throws (a host that emits an illegal fact has a bug, never a user-visible state). */
function record(event: VideoRenderJobEvent): void {
  const refused = ledger.apply(event);
  if (refused !== null) throw new Error(`video render job event refused (${refused}): ${JSON.stringify(event)}`);
  log.push(event);
  if (log.length > VIDEO_RENDER_JOB_LOG_TAIL) log.splice(0, log.length - VIDEO_RENDER_JOB_LOG_TAIL);
  for (const listener of [...listeners]) listener();
}

/** 🧵️ Every running export of this tab, the ledger's fold (stable identity between changes). */
export function videoRenderExportJobsSnapshotV1(): readonly VideoRenderJobRow[] {
  return ledger.running();
}

/** 📜️ The most recent job events of this tab, oldest first. */
export function videoRenderExportJobLogV1(): readonly VideoRenderJobEvent[] {
  return log.slice();
}

/** 🧵️ Notifies `listener` after every job event. */
export function subscribeVideoRenderExportJobsV1(listener: () => void): () => void {
  listeners.add(listener);
  return () => listeners.delete(listener);
}

/** 🏷️ The prefix of every Task Manager id this host owns. */
export const VIDEO_RENDER_EXPORT_TASK_PREFIX = "export:video#";

/** 🏷️ The Task Manager id of job `job`. */
export function videoRenderExportTaskIdV1(job: number): string {
  return `${VIDEO_RENDER_EXPORT_TASK_PREFIX}${job}`;
}

/** 🛑️ The cancel command: appends `cancelRequested` for the job behind Task Manager id `taskId`; `false` when no such job
 * runs or it is already cancelling. */
export function cancelVideoRenderExportV1(taskId: string): boolean {
  const job = taskId.startsWith(VIDEO_RENDER_EXPORT_TASK_PREFIX) ? Number(taskId.slice(VIDEO_RENDER_EXPORT_TASK_PREFIX.length)) : Number.NaN;
  const row = Number.isInteger(job) ? ledger.row(job) : undefined;
  if (!row || row.cancelling) return false;
  record({ kind: "cancelRequested", job: row.job });
  return true;
}

/** 🏁️ How an export ended, with the user-facing sentence the host shows for it. */
export interface VideoRenderExportOutcomeV1 {
  readonly job: number;
  readonly outcome: VideoRenderJobOutcome;
  readonly text: string;
}

/** ⏱️ The shortest gap between two `progressed` events of one job; the last frame always reports. */
export const VIDEO_RENDER_PROGRESS_EVENT_MS = 100;

/** 🎥️ The whole host answer to one `videoRenderExport` effect: starts a job, admits it (the plugin's declared
 * {@link MEDIA_VIDEO_RENDER_CAPABILITY}, then the kernel's program rule), renders + encodes, hands the MP4 to `deliver`,
 * and finishes the job with its outcome. Never rejects — every failure is a finished job with a stated outcome. */
export async function runVideoRenderExportV1(
  request: { readonly filename: string; readonly owner: string; readonly program: VideoRenderProgram; readonly capabilities: readonly string[] },
  host: { readonly environment: VideoRenderEnvironmentV1; readonly deliver: (filename: string, bytes: Uint8Array) => void; readonly locale: () => string; readonly announce?: (text: string) => void },
): Promise<VideoRenderExportOutcomeV1> {
  const job = ledger.lastJob() + 1;
  const total = videoRenderFrameCount(request.program);
  record({ kind: "started", job, owner: request.owner, filename: request.filename, frames: Number.isSafeInteger(total) && total >= 0 ? total : 0, atMs: Date.now() });
  const finish = (outcome: VideoRenderJobOutcome, text: string): VideoRenderExportOutcomeV1 => {
    record({ kind: "finished", job, outcome });
    return { job, outcome, text };
  };
  const locale = host.locale();
  if (!request.capabilities.includes(MEDIA_VIDEO_RENDER_CAPABILITY)) return finish({ status: "refused", code: "capability" }, videoRenderExportTextV1("refusedCapability", locale, { filename: request.filename, owner: request.owner, capability: MEDIA_VIDEO_RENDER_CAPABILITY }));
  const problem = videoRenderProgramProblem(request.program);
  if (problem !== null) return finish({ status: "refused", code: problem }, videoRenderExportTextV1("refused", locale, { filename: request.filename, code: problem }));
  host.announce?.(videoRenderExportTextV1("started", locale, { filename: request.filename }));
  let reportedAt = Number.NEGATIVE_INFINITY;
  let completed = 0;
  try {
    const result = await renderVideoProgramV1(
      request.program,
      host.environment,
      (frame) => {
        completed = frame;
        const now = host.environment.now();
        if (frame !== total && now - reportedAt < VIDEO_RENDER_PROGRESS_EVENT_MS) return;
        reportedAt = now;
        record({ kind: "progressed", job, completed: frame });
      },
      () => ledger.row(job)?.cancelling === true,
    );
    host.deliver(request.filename, result.bytes);
    return finish({ status: "done", bytes: result.bytes.length, tier: result.tier }, videoRenderExportTextV1("done", host.locale(), { filename: request.filename, frames: result.frames }));
  } catch (error) {
    if (error instanceof VideoRenderExportErrorV1 && error.code === "cancelled") return finish({ status: "cancelled", completed }, videoRenderExportTextV1("cancelled", host.locale(), { filename: request.filename }));
    const reason = error instanceof VideoRenderExportErrorV1 ? error.detail : error instanceof Error ? error.message : String(error);
    return finish({ status: "failed", reason }, videoRenderExportTextV1("failed", host.locale(), { filename: request.filename, reason }));
  }
}
//#endregion 🧵️Jobs
