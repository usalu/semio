import type { OriginalBrowserFrameNative } from "../🎟️original/🟦️.ts";
import type { RuntimeAssetBindings } from "../../../../../../../../../🔨️modules/🖼️canvas/🔌️runtime-assets/🟦️.ts";
import { pluginHandleForBridge } from "../../🐚️plugin-bridge/🟦️.ts";

/** 🔢️ `generation` and `sequence` are `u64` on the renderer's own `#[wasm_bindgen]` exports
 * (`🌐️browser-worker/🦀️.rs` `enqueue_batch`/`tick`), and wasm-bindgen lowers a `u64` parameter
 * straight into the wasm `i64` slot — a JS `number` there throws
 * `TypeError: Cannot convert <n> to a BigInt` out of the generated glue, faulting the Worker on its
 * FIRST frame batch (`frame-runtime-fault`). Declaring them `bigint` here is what makes that ABI
 * checkable at the seam instead of at runtime; the transport's own counters stay `number`, which is
 * what a structured-cloned protocol message carries. */
export type BrowserRendererWorkerHandle = OriginalBrowserFrameNative & {
  enqueueBatch(eventsJson: string, generation: bigint): void;
  assetDecodeStep(operation:bigint,generation:bigint,epoch:bigint,items:number,copy:number,capacity:number,release:number,depth:number,fuel:bigint,deadline:bigint):number;
  pollAssetRequest(): string;
  assetResponseCurrent(): boolean;
  beginReferenceImage(width: number, height: number, digest: string): number;
  pushReferenceImageRows(offset: number, pixels: Uint8Array): boolean;
  sealReferenceImage(): boolean;
  reserveAssetResponse(byteCredits: number): boolean;
  pushAssetResponsePage(bytes: Uint8Array): void;
  sealAssetResponse(): boolean;
  abortAssetResponse(): boolean;
  rejectAssetResponse(): boolean;
  closeStep(): boolean;
};

type BrowserRendererBootstrapHandle = {
  step(): string;
  bootShell(): Promise<BrowserRendererBootstrapHandle>;
  finish(): BrowserRendererWorkerHandle;
};

export type BrowserRendererBootStep = { readonly stage: string; readonly progress: number; readonly shellBoot: boolean; readonly complete: boolean; readonly elapsedUs: number };


export type RendererBindings = RuntimeAssetBindings & {
  default?: (moduleOrPath?: WebAssembly.Module | RequestInfo | URL) => Promise<unknown>;
  dumpStructure?: (windowId?: string) => string;
  dumpFrameStats?: (windowId?: string) => string;
  /** ♿️ The window's accessibility tree, for the UI isolate's ARIA mirror — production, not a probe. */
  dumpAccessibility?: (windowId?: string) => string;
  /** 🧊️ Every World3d surface's PUBLISHED mesh payload, per role — the oracle a committed fixture is compared against. */
  dumpMeshStats?: (windowId?: string) => string;
  /** 🎲️ Every Board2d surface's published board (page rect, camera, positions, selection, counts) — the twin of React's
   * Board2dHost `data-board-*` vitals, answered on request only. */
  dumpBoard2d?: (windowId?: string) => string;
  /** 🎯️ The shell chrome's pointer registry and its dispatched-action ledger — the chrome twin of the DOM a
   * React parity probe reads. Diagnostics-gated in the renderer, so an unarmed page answers `armed: false`. */
  dumpChrome?: (windowId?: string) => string;
  /** 🛰️ The shell's agent-bridge offer scope (`{hubOrigin, spaceId, agentPrincipalIds}` or `null`) for the page's offer
   * watcher — production, not a probe; the session capability stays in this isolate. */
  dumpAgentBridgeOfferScope?: () => string;
  /** 🧭️ The ONE boot-axis door of the renderer wasm (`🧊️renderer/🦀️.rs` `WgpuBootDescriptor`), fed the
   * JSON `../🧭️boot-descriptor/🟦️.ts` resolved. It replaced four per-axis setters, which is what let the
   * three wgpu entry points carry three different subsets of the same vocabulary. */
  semioWgpuSetPluginCatalog?: (serialized:string)=>void;
  semioWgpuSetBootDescriptor?: (descriptorJson: string) => void;
  semioWgpuSetHubEnv?: (hubUrl: string, user: string, dataDir: string) => void;
  /** 🌓️ The appearance door (`🧊️renderer/🦀️.rs`, region 🌓️HostAppearance). This realm owns no
   * `window`, so it can neither read `prefers-color-scheme` nor replay the persisted
   * `os.config.ui-preferences` log; the UI isolate makes both reads and hands them here — at boot and
   * again on every change. Without it every browser boot resolved DARK where React resolved LIGHT. */
  semioWgpuSetHostAppearance?: (preference: string, systemDark: boolean) => void;
  /** 🗣️ The locale door (`🧊️renderer/🦀️.rs`, region 🗣️HostLocale). This realm's own `navigator.language`
   * is not the page's promise, so the UI isolate's read (the boot message's `locale`) crosses here once,
   * before the shell resolves its tongue: a lock, then the stored preference, then this. */
  semioWgpuSetHostLocale?: (locale: string) => void;
  semioWgpuSetHostIntroductionSuppressed?: (suppressed: boolean) => void;
  /** 🫥️ The hidden-page door (`⚙️EngineCanvas/🎯️targets/🧊️wgpu/✍️text-editor/🦀️.rs` `semioWgpuHostPageHidden`): the page was hidden or left, so
   * every open text-editor typing run ends (`hidden`) on the next frame turn. */
  semioWgpuHostPageHidden?: () => void;
  /** 🫥️ The window-blur door (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` `semioWgpuHostWindowBlur`): the page's `window` lost focus, so
   * every open typing run ends (`blur`) and the shell blurs the focused pane for its program on the next frame turn. */
  semioWgpuHostWindowBlur?: () => void;
  /** 🛰️ The agent-bridge door (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` `semioWgpuSetAgentBridgeConfig`): the page's discovered
   * offer; an empty url clears it and parks the bridge in `Disabled`. */
  semioWgpuSetAgentBridgeConfig?: (url: string, admissionProof: string) => void;
  /** ⌨️ The platform door (`🧊️renderer/🦀️.rs`, region ⌨️HostPlatform). The renderer's own answer is
   * `cfg!(target_os = "macos")`, which is FALSE in every wasm build, so a macOS browser formatted `mod`
   * as `Ctrl` where React formatted `⌘️`. `userAgentData.platform` is page-thread-only, so the UI
   * isolate makes the read and hands it here once, with the boot. */
  semioWgpuSetHostPlatform?: (platform: string) => void;
  /** 🗄️ The preference door (`🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs`, region 🚪️StorageDoor). This realm owns
   * no `localStorage`, so every `prefs_get`/`prefs_set` used to read empty and write to nothing. The UI
   * isolate hands the whole carried census over here BEFORE the shell boots — which is what makes the
   * first frame's appearance, tour-seen and dock-skeleton reads synchronous — and the shell writes back
   * through the host-io door. */
  semioWgpuSetHostStorage?: (snapshotJson: string) => void;
  /** 🩺️ Arms the renderer's own per-frame `[TRACE]` dumps — `semio_framework_trace::set_runtime_diagnostics`,
   * the wasm door of the `SEMIO_RUNTIME_DIAGNOSTICS` switch a native process reads from its environment
   * and React reads from `localStorage`. */
  semioWgpuSetRuntimeDiagnostics?: (enabled: boolean) => void;
  semioWgpuWorkerBootstrap?: (
    canvas: OffscreenCanvas,
    plugins: readonly { readonly pluginId: string; readonly handle: ReturnType<typeof pluginHandleForBridge> }[],
    pluginFilter: string,
    width: number,
    height: number,
    dpr: number,
    wake: () => void,
  ) => Promise<BrowserRendererBootstrapHandle>;
};
