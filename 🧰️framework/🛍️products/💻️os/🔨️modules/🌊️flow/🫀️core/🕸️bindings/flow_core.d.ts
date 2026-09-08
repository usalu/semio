/* tslint:disable */
/* eslint-disable */

export class DagSession {
    free(): void;
    [Symbol.dispose](): void;
    attachCanvas(canvas: HTMLCanvasElement, logical_w: number, logical_h: number, dpr: number): Promise<any>;
    drawLodLabel(): string;
    fixtureJson(): string;
    gpuReady(): boolean;
    labelOverlayPaintStateJson(): string;
    loadFixtureJson(json: string): void;
    lodScaleJson(): string;
    constructor();
    nodeOverlaysJson(): string;
    pointerDown(x: number, y: number, extend: boolean): void;
    pointerMove(x: number, y: number): void;
    pointerUp(x: number, y: number): void;
    renderFrame(): void;
    reorganize(options_json: string): void;
    screenToWorld(x: number, y: number): Array<any>;
    setAutomaticLod(enabled: boolean): void;
    setCamera(x: number, y: number, zoom: number): void;
    setCanvasThemeJson(json: string): void;
    setForcedDrawLodLabel(label: string): void;
    setSize(width: number, height: number, dpr: number): void;
    setWheelZoomActive(active: boolean): void;
    takePendingOpenInstanceId(): string | undefined;
}

export class DagSnapshotVcs {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * 🌐️ Constructs the VCS bridge without synchronously blocking the browser host callback.
     */
    static create(): Promise<DagSnapshotVcs>;
    dispatchBinary(command_bytes: Uint8Array): Promise<void>;
    dispatchText(command_text: string): Promise<void>;
    envelopeJson(): Promise<string>;
    generation(): Promise<number>;
    snapshotJson(): Promise<string>;
}

/**
 * 🌉️ wasm-bindgen wrapper around [`Kernel`] for the React-web / wgpu-web hosts (see design
 * §1's three-host list). Every method takes/returns pack-encoded bytes only — this type owns
 * no logic of its own beyond (de)serializing at the boundary and delegating to `Kernel`.
 */
export class KernelHost {
    private constructor();
    free(): void;
    [Symbol.dispose](): void;
    /**
     * ▶️ `activation_bytes` is a pack-encoded `(PackageId, u16 plugin_ordinal, ActorKind, Lane,
     * Option<WindowId>, ActivationEvent)` tuple; returns the pack-encoded fresh `ActorId`.
     */
    activate(activation_bytes: Uint8Array): Promise<Uint8Array>;
    complete(actor_bytes: Uint8Array, turn_result_bytes: Uint8Array, now_ms: bigint): Promise<void>;
    static create(shard_count: number, exclusive_reserve: number, grants_per_tick: number): Promise<KernelHost>;
    metrics(): Promise<Uint8Array>;
    submit(envelope_bytes: Uint8Array): Promise<Uint8Array>;
    tick(now_ms: bigint): Promise<Uint8Array>;
}

/**
 * 🌐️ Generic JSON-RPC bridge for the CAD `SpatialKernel` (see `🧠️semio/🟦️.ts`): dispatches
 * one `BrepKernel` method by name over JSON args, sharing the same in-process `kernel()` the
 * `tessellate`/`dispose` exports above use so handles stay valid across calls.
 */
export function brep_invoke(method: string, args_json: string): string;

export function dispose(handle: string): void;

export function initialize_browser_clock(): void;

export function tessellate(handle: string, tolerance: number): string;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
    readonly memory: WebAssembly.Memory;
    readonly brep_invoke: (a: number, b: number, c: number, d: number, e: number) => void;
    readonly dispose: (a: number, b: number) => void;
    readonly flow_bridge_allocate: (a: number) => number;
    readonly flow_bridge_begin_close: () => void;
    readonly flow_bridge_poll: (a: number, b: number, c: number, d: bigint, e: bigint) => number;
    readonly flow_bridge_release: (a: number, b: number) => void;
    readonly flow_bridge_send: (a: number, b: number, c: number, d: bigint, e: bigint) => number;
    readonly flow_bridge_terminal_is_empty: () => number;
    readonly tessellate: (a: number, b: number, c: number, d: number) => void;
    readonly __wbg_dagsession_free: (a: number, b: number) => void;
    readonly __wbg_dagsnapshotvcs_free: (a: number, b: number) => void;
    readonly dagsession_attachCanvas: (a: number, b: number, c: number, d: number, e: number) => number;
    readonly dagsession_drawLodLabel: (a: number, b: number) => void;
    readonly dagsession_fixtureJson: (a: number, b: number) => void;
    readonly dagsession_gpuReady: (a: number) => number;
    readonly dagsession_labelOverlayPaintStateJson: (a: number, b: number) => void;
    readonly dagsession_loadFixtureJson: (a: number, b: number, c: number, d: number) => void;
    readonly dagsession_lodScaleJson: (a: number, b: number) => void;
    readonly dagsession_new: () => number;
    readonly dagsession_nodeOverlaysJson: (a: number, b: number) => void;
    readonly dagsession_pointerDown: (a: number, b: number, c: number, d: number) => void;
    readonly dagsession_pointerMove: (a: number, b: number, c: number) => void;
    readonly dagsession_pointerUp: (a: number, b: number, c: number) => void;
    readonly dagsession_renderFrame: (a: number, b: number) => void;
    readonly dagsession_reorganize: (a: number, b: number, c: number, d: number) => void;
    readonly dagsession_screenToWorld: (a: number, b: number, c: number) => number;
    readonly dagsession_setAutomaticLod: (a: number, b: number) => void;
    readonly dagsession_setCamera: (a: number, b: number, c: number, d: number) => void;
    readonly dagsession_setCanvasThemeJson: (a: number, b: number, c: number) => void;
    readonly dagsession_setForcedDrawLodLabel: (a: number, b: number, c: number) => void;
    readonly dagsession_setSize: (a: number, b: number, c: number, d: number) => void;
    readonly dagsession_setWheelZoomActive: (a: number, b: number) => void;
    readonly dagsession_takePendingOpenInstanceId: (a: number, b: number) => void;
    readonly dagsnapshotvcs_create: () => number;
    readonly dagsnapshotvcs_dispatchBinary: (a: number, b: number, c: number) => number;
    readonly dagsnapshotvcs_dispatchText: (a: number, b: number, c: number) => number;
    readonly dagsnapshotvcs_envelopeJson: (a: number) => number;
    readonly dagsnapshotvcs_generation: (a: number) => number;
    readonly dagsnapshotvcs_snapshotJson: (a: number) => number;
    readonly __wbg_kernelhost_free: (a: number, b: number) => void;
    readonly kernelhost_activate: (a: number, b: number, c: number) => number;
    readonly kernelhost_complete: (a: number, b: number, c: number, d: number, e: number, f: bigint) => number;
    readonly kernelhost_create: (a: number, b: number, c: number) => number;
    readonly kernelhost_metrics: (a: number) => number;
    readonly kernelhost_submit: (a: number, b: number, c: number) => number;
    readonly kernelhost_tick: (a: number, b: bigint) => number;
    readonly initialize_browser_clock: () => void;
    readonly __wasm_bindgen_func_elem_16864: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_16866: (a: number, b: number, c: number, d: number) => void;
    readonly __wasm_bindgen_func_elem_13549: (a: number, b: number, c: number) => void;
    readonly __wbindgen_export: (a: number, b: number) => number;
    readonly __wbindgen_export2: (a: number, b: number, c: number, d: number) => number;
    readonly __wbindgen_export3: (a: number) => void;
    readonly __wbindgen_export4: (a: number, b: number) => void;
    readonly __wbindgen_export5: (a: number, b: number, c: number) => void;
    readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
    readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
 * Instantiates the given `module`, which can either be bytes or
 * a precompiled `WebAssembly.Module`.
 *
 * @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
 *
 * @returns {InitOutput}
 */
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
 * If `module_or_path` is {RequestInfo} or {URL}, makes a request and
 * for everything else, calls `WebAssembly.instantiate` directly.
 *
 * @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
 *
 * @returns {Promise<InitOutput>}
 */
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
