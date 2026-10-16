/** ⚡️ The `Effect` union a guest turn requests from the host, and the service operation payload.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { type VideoRenderProgram } from "../🎥️video-render-program/🟦️.ts";
import { type JobPlacement } from "../🛠️spawned-job/🟦️.ts";

// #region 🎠️Kernel
//#region InvocationResponse
/**
 * 🐚️ A typed side effect the guest emits toward the host. Mirrors the Rust `Effect` enum
 * (`🎠️kernel/🦀️.rs` `🔖️Effect` region — replaces `HostEffect` now that plugins and
 * extensions share one `actor` world; externally tagged: unit variants are the plain tag string,
 * struct variants are a single-key object keyed by the camelCase variant name). `openWindow`/
 * `requestFileOpen`/`requestMediaFrames`/`spawnPluginInstance`/`openDialog`/`dispatchAction` gained
 * `req` now that they complete; `invokeExtension` lost `responseAction` and gained `req`.
 */
export type Effect =
  | "requestSync"
  | { readonly openWindow: { readonly req: number; readonly kind: string; readonly params: unknown } }
  | { readonly closeWindow: { readonly window: number } }
  | { readonly notify: { readonly message: string } }
  | { readonly navigate: { readonly uri: string } }
  /** 📂️ Replaces the active app instance's document with pack+spr bytes — host-owned
   * counterpart of `loadAppArtifactPack` for catalog/example studio opens. */
  | { readonly loadDocument: { readonly pack: readonly number[]; readonly spr: readonly number[] } }
  | { readonly openExternalUrl: { readonly url: string } }
  | { readonly setPanel: { readonly panelJson: string } }
  | { readonly downloadMediaExport: { readonly filename: string; readonly mimeType: string; readonly data: string; readonly encoding?: string } }
  | { readonly iconRenderExport: { readonly items: readonly { readonly filename: string; readonly request: unknown }[] } }
  /** 🎥️ Asks the host to render `program` frame by frame, encode it as H.264 in an MP4 and download it as `filename`,
   * as an event-sourced job with progress and cancellation, for a plugin holding {@link MEDIA_VIDEO_RENDER_CAPABILITY}.
   * Twin of Rust `Effect::VideoRenderExport`. */
  | { readonly videoRenderExport: { readonly filename: string; readonly program: VideoRenderProgram } }
  | { readonly requestFileOpen: { readonly req: number; readonly accept: string; readonly readAs?: string; readonly importAction: string; readonly args?: unknown; readonly multiple?: boolean } }
  /** 🎞️ Asks the shell to decode a video (file picker, or `payload` bytes already in hand)
   * and re-dispatch `frameAction` once per sampled frame with `{payload: dataUrl(image/jpeg), name,
   * frameIndex, timestampMs, index, total, width, height, ...args}`, then `doneAction` once with
   * `{name, durationMs, frameCount, sampledCount, width, height, codec, ...args}`; if the host can't
   * decode it, `fallbackAction` fires once with `{payload: dataUrl(raw bytes), name, ...args}`. The
   * numeric hints (`sampleStride`/`maxFrames`/`maxLongEdgePx`/`fpsHint`) are 0 when the caller wants
   * the host default. */
  | {
      readonly requestMediaFrames: {
        readonly req: number;
        readonly accept: string;
        readonly frameAction: string;
        readonly doneAction: string;
        readonly fallbackAction: string;
        readonly sampleStride?: number;
        readonly maxFrames?: number;
        readonly maxLongEdgePx?: number;
        readonly fpsHint?: number;
        readonly payload?: string;
        readonly args?: unknown;
      };
    }
  | { readonly spawnPluginInstance: { readonly req: number; readonly pluginId: string; readonly appId: string; readonly osInstanceId?: string; readonly label?: string; readonly artifactJson?: string } }
  | { readonly openPluginInstance: { readonly pluginId: string; readonly appId: string; readonly osInstanceId?: string } }
  | { readonly setActiveUtility: { readonly windowId: string; readonly utilityId: string } }
  /** 🛠️ Programmatically switches the host-owned active tool of the active mode — the effect form of
   * `setActiveTool`. Empty `toolId` deactivates the current tool. */
  | { readonly setActiveTool: { readonly toolId: string } }
  | { readonly openDialog: { readonly req: number; readonly dialogId: string; readonly args?: Record<string, unknown> } }
  /** 🔁️ Re-dispatches `action` onto the same plugin instance after `delayMs` — lets a program
   * advance staged/progressive work over several ticks without blocking the host; the response's own
   * `requestedEffects` are fed back through `applyHostEffects` recursively. */
  | { readonly dispatchAction: { readonly req: number; readonly action: string; readonly args?: unknown; readonly delayMs: number } }
  | { readonly clipboardWrite: { readonly fragment: unknown } }
  | { readonly replayShellCommand: { readonly actionId: string; readonly args?: unknown } }
  /** 🔁️ Asks the shell to invoke an extension capability — the SDK resumes the awaiting
   * future on a `completed` event carrying the same `req` instead of a `responseAction` redispatch. */
  | {
      readonly invokeExtension: {
        readonly req: bigint;
        readonly extensionId: string;
        readonly capability: string;
        readonly requestJson: string;
      };
    }
  // --- new variants (📓️design-abi.md §2's table; nothing constructs these yet) ---
  | { readonly sendMessage: { readonly target: unknown; readonly payload: readonly number[] } }
  | { readonly publishEvent: { readonly topic: string; readonly payload: readonly number[] } }
  | { readonly blobWrite: { readonly req: number; readonly mediaType: unknown; readonly bytes: readonly number[] } }
  | { readonly blobLoad: { readonly req: number; readonly hash: string } }
  | { readonly httpRequest: { readonly req: number; readonly method: string; readonly url: string; readonly headers?: readonly (readonly [string, string])[]; readonly body?: readonly number[]; readonly stream?: boolean } }
  | { readonly documentRead: { readonly req: number; readonly doc: string; readonly lane: string } }
  | { readonly documentWrite: { readonly req: number; readonly doc: string; readonly lane: string; readonly ops: readonly number[] } }
  | { readonly linkResolve: { readonly req: number; readonly link: string } }
  | { readonly registryQuery: { readonly req: number; readonly kind: string; readonly filter?: unknown } }
  | { readonly ioCompose: { readonly req: number; readonly key: string; readonly sources: readonly string[] } }
  | { readonly cacheDerive: { readonly req: number; readonly engineId: string; readonly input: readonly number[] } }
  | { readonly cacheRead: { readonly req: number; readonly engineId: string; readonly key: string } }
  | { readonly setTimer: { readonly id: number; readonly afterMs: number; readonly repeat?: boolean } }
  /** 🧵️ Asks the host to run one job on this actor and answer with `Event::JobCompleted`.
   * `job` is a WIT `u64` and therefore a `bigint` — it IS the parked request id the guest correlates
   * on, so narrowing it to a `number` would silently mis-resolve a long-lived actor's futures; the
   * `startJob`/`stepJob` door refuses a `number` outright. Every framework reserved tool verb
   * (`interactionSelect`/`interactionHover`/`clearSelection`) reaches its host as this effect and
   * nothing else. */
  | { readonly spawnJob: { readonly job: bigint; readonly kind: string; readonly input: Uint8Array; readonly placement: JobPlacement } }
  | { readonly cancelJob: { readonly job: bigint } }
  /** ↩️ Answers ONE inbound `Event::Request { req, … }` — the only `req`-bearing effect that
   * completes someone else's request instead of opening its own. `result` keeps the WIT
   * `respond-result` arm names (`ok`/`fault`, not Rust's `RequestOutcome::{Ok,Err}`) because this is
   * the shape the wire carries and the shape the extension-completion door already takes. */
  | { readonly respond: { readonly req: bigint; readonly result: { readonly ok: Uint8Array } | { readonly fault: Uint8Array } } }
  | { readonly storageRead: { readonly req: number; readonly key: string } }
  | { readonly storageWrite: { readonly req: number; readonly key: string; readonly bytes: readonly number[] } }
  | { readonly storageDelete: { readonly req: number; readonly key: string } }
  | { readonly requestCapability: { readonly req: number; readonly capability: unknown } }
  | { readonly releaseCapability: { readonly id: unknown } }
  | { readonly subscribe: { readonly topic: string } }
  | { readonly unsubscribe: { readonly topic: string } }
  /** 🔌 Asks the installed owner service to operate on the host-owned active document scope. */
  | { readonly requestServiceOperation: { readonly owner: string; readonly serviceId: string; readonly action: string; readonly payload: ServiceOperationPayloadV1 } };

export type ServiceOperationPayloadV1 = null | boolean | number | string | readonly ServiceOperationPayloadV1[] | { readonly [key: string]: ServiceOperationPayloadV1 };
//#endregion InvocationResponse
// #endregion 🎠️Kernel
