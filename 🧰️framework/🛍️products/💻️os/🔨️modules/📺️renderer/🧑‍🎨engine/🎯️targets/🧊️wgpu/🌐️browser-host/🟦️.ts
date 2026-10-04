/** 🪆️ Owns one independently rooted browser renderer and its frame Worker lifetime. */
export type WgpuBrowserHostOptions = {
    readonly descriptor: WgpuBootDescriptor;
    readonly locale: "en" | "de";
    readonly mountId: string;
    readonly pageBindings?: boolean;
    readonly rendererModuleUrl?: string;
    readonly rendererWasmUrl?: string;
    readonly frameWorkerUrl?: string;
    readonly plugins?: readonly WgpuPluginModule[];
    readonly pluginRegistrySelection?: WgpuPluginRegistrySelection;
    readonly suppressAutoIntroduction?: boolean;
    readonly signal?: AbortSignal;
    readonly onProgress?: (event: WgpuBootProgress) => void;
};
import { admitWgpuPluginModules, admitWgpuPluginRegistrySelection, type WgpuPluginModule, type WgpuPluginRegistrySelection } from "../🧩️plugin-modules/🛂️admission/🟦️.ts";
import { createBrowserMediaOverlay } from "../../../🎬️media/🌐️browser/🎛️host/🟦️.ts";
import { BrowserFrameTransport, browserFrameEventFromDom, browserFrameEventIsReplaceable, browserFramePointerDomEvent, browserFrameWheelDomEvent, type BrowserFrameDomEvent, type BrowserFrameFallbackState, type BrowserFrameIntrospectionProbe, type BrowserFrameWorkerFaultCode, type BrowserFrameWorkerStepReport, type BrowserHubDocumentRemote } from "../🚚️browser-frame-transport/🟦️.ts";
import { createWgpuPageHostIo } from "../🚪️host-io/🟦️.ts";
import { setInteractiveJobPort } from "../../../../../../../../🔨️modules/🖱️ui/🧱️elements/🔌️Ports/📡️interactive-jobs/🟦️.ts";
import { TURN_DIAGNOSTICS_KEY, setTurnDiagnostics, turnDiagnosticsEnabled } from "../⏱️turn-budget/🟦️.ts";
import { resolveRuntimeDiagnosticsPreference, stampShardWorkerDiagnostics } from "../../../../../../../../🔨️modules/🎭️actor/🩺️diagnostics/🟦️.ts";
import { describeBrowserBootPhase } from "../🫀️boot-liveness/🟦️.ts";
import { WGPU_PREFERS_DARK_MEDIA_QUERY, readWgpuHostStorageSnapshot, resolveWgpuHostAppearance, resolveWgpuHostPlatform, wgpuReadinessBeacon, type WgpuBootDescriptor, type WgpuHostAppearance, type WgpuHostPlatform, type WgpuHostStorageSnapshot } from "../🧭️boot-descriptor/🟦️.ts";
import { createAccessibilityMirror } from "../♿️accessibility-mirror/🟦️.ts";
import { agentBridgeOfferScopeFromJsonV1, watchAgentBridgeOffer } from "../../../🧱️elements/🔗️AgentBridge/🛰️offer/🟦️.ts";
import { browserClipboardPasteCandidate, wireBrowserFullscreen, wireBrowserKeyboard } from "../🎮️input-wire/🟦️.ts";
import bootLifecycleSchema from "../../../🧬️schema/⏳️boot-lifecycle/🔣️.json";

/** ⏳️ Progress belongs to one pending browser mount and its owned Worker. */
export type WgpuBootProgress = { readonly stage: string; readonly progress: number; readonly worker: BrowserFrameWorkerStepReport };

const RENDERER_MODULE_URL = new URL("../renderer-modules/wgpu/semio-framework-os-renderer-wgpu.js", import.meta.url).href;
const RENDERER_WASM_URL = new URL("../renderer-modules/wgpu/semio-framework-os-renderer-wgpu_bg.wasm", import.meta.url).href;
const FRAME_WORKER_URL = new URL("../🎞️frame-worker.js/🟨️.js", import.meta.url);
function armUiTurnDiagnostics(): void {
    const serverDefault = document.querySelector<HTMLMetaElement>('meta[name="semio-runtime-diagnostics"]')?.content;
    try {
        const stored = globalThis.localStorage?.getItem(TURN_DIAGNOSTICS_KEY);
        setTurnDiagnostics(resolveRuntimeDiagnosticsPreference(stored, serverDefault));
    }
    catch {
        setTurnDiagnostics(resolveRuntimeDiagnosticsPreference(undefined, serverDefault));
    }
}
function hostAppearance(): WgpuHostAppearance {
    return resolveWgpuHostAppearance(window);
}
function hostPlatform(): WgpuHostPlatform {
    return resolveWgpuHostPlatform(window);
}
function hostStorage(): WgpuHostStorageSnapshot {
    return readWgpuHostStorageSnapshot(window);
}
export const WGPU_CANVAS_ID = "semio-wgpu-canvas";
function canvasElement(tongue: "en" | "de", id?: string): HTMLCanvasElement {
    const canvas = document.createElement("canvas");
    if (id)
        canvas.id = id;
    canvas.tabIndex = 0;
    canvas.setAttribute("aria-label", tongue === "de" ? "Semio Arbeitsfläche" : "Semio workspace");
    canvas.style.cssText = "display:block;width:100%;height:100%;touch-action:none;outline:none;";
    return canvas;
}
export const WGPU_INTROSPECTION_GLOBAL = "semioWgpuIntrospection";
export const WGPU_HUB_PROJECTION_GLOBAL = "semioWgpuHubProjection";
/** 🔎️ Reads the accepted output of exactly one independently owned renderer. */
export type WgpuIntrospection = {
    readonly dumpStructure: (windowId?: string) => Promise<string>;
    readonly dumpFrameStats: (windowId?: string) => Promise<string>;
    readonly dumpAccessibility: (windowId?: string) => Promise<string>;
    readonly dumpMeshStats: (windowId?: string) => Promise<string>;
    readonly dumpBoard2d: (windowId?: string) => Promise<string>;
    readonly dumpChrome: (windowId?: string) => Promise<string>;
};
type WgpuHubProjection = {
    readonly publishDocumentStatus: (documentKey: string, remote: BrowserHubDocumentRemote | null) => boolean;
};
function mountIntrospection(transport: BrowserFrameTransport): WgpuIntrospection {
    const probe = (kind: BrowserFrameIntrospectionProbe) => async (windowId?: string) => {
        if (transport.status !== "ready") throw new Error("wgpu-mount-retired");
        return (await transport.introspect(kind, windowId)) ?? "";
    };
    return { dumpStructure: probe("structure"), dumpFrameStats: probe("frame-stats"), dumpAccessibility: probe("accessibility"), dumpMeshStats: probe("mesh-stats"), dumpBoard2d: probe("board2d"), dumpChrome: probe("chrome") };
}

function attachIntrospectionBindings(transport: BrowserFrameTransport): () => void {
    const host = window as unknown as {
        semioWgpuIntrospection?: WgpuIntrospection;
        semioWgpuHubProjection?: WgpuHubProjection;
    };
    host.semioWgpuIntrospection = mountIntrospection(transport);
    host.semioWgpuHubProjection = { publishDocumentStatus: (documentKey, remote) => transport.publishHubDocumentStatus(documentKey, remote) };
    return () => {
        delete host.semioWgpuIntrospection;
        delete host.semioWgpuHubProjection;
    };
}
export { WGPU_ACCESSIBILITY_MIRROR_ID } from "../♿️accessibility-mirror/🟦️.ts";
function statusElement(root: HTMLElement): HTMLElement {
    const status = document.createElement("div");
    status.setAttribute("role", "status");
    status.setAttribute("aria-live", "polite");
    status.style.cssText = "position:fixed;left:12px;bottom:12px;padding:6px 9px;background:#001117cc;color:#d7f7ff;font:12px monospace;z-index:9997;";
    root.appendChild(status);
    return status;
}
function fallbackLines(state: BrowserFrameFallbackState | undefined, tongue: "en" | "de"): string {
    if (!state) {
        return tongue === "de"
            ? "Oberfläche: vor der Übergabe der Zeichenfläche an den Worker gescheitert. Kein Frame-Pfad war je aktiv."
            : "Surface: failed before the canvas reached the Worker. No frame path was ever live.";
    }
    const turns = state.uiTurns;
    const steps = state.workerSteps;
    const ledger = `${turns.recordedOverruns}/${turns.sustainedOverruns} (p99 ${turns.p99Ms.toFixed(3)} ms, worst ${turns.worstExecutingMs.toFixed(3)} ms @ ${turns.worstSite || "—"})`;
    const workerLedger = `${steps.recordedOverruns}/${steps.sustainedOverruns} (worst ${steps.worstStepMs.toFixed(3)} ms @ ${steps.worstStepSite || "—"})`;
    const phaseLine = describeBrowserBootPhase(state.bootPhase, state.bootPhaseElapsedMs, tongue);
    if (tongue === "de") {
        return [
            `Oberfläche: ${state.surface}${state.deferredCadence ? " · verzögerte Taktung" : ""}`,
            `Boot-Stufe: ${state.bootStage || "—"} · still seit ${Math.round(state.bootSilentForMs)} ms`,
            phaseLine,
            `UI-Thread-Frames: nicht verfügbar — die Zeichenfläche gehört dem Frame-Worker (OffscreenCanvas übergeben)`,
            `Worker beendet: ${state.workerTerminated ? "ja" : "nein"} · Eingaben angenommen: ${state.inputAccepted ? "ja" : "nein"}`,
            `UI-Takte über dem Budget (erfasst/anhaltend): ${ledger}`,
            `Worker-Schritte über dem Budget (erfasst/anhaltend): ${workerLedger}${steps.degraded ? " · verzögerte Taktung" : ""}`,
        ].join("\n");
    }
    return [
        `Surface: ${state.surface}${state.deferredCadence ? " · deferred cadence" : ""}`,
        `Boot stage: ${state.bootStage || "—"} · silent for ${Math.round(state.bootSilentForMs)} ms`,
        phaseLine,
        `UI-thread frames: unavailable — the canvas belongs to the frame Worker (OffscreenCanvas transferred)`,
        `Worker terminated: ${state.workerTerminated ? "yes" : "no"} · input accepted: ${state.inputAccepted ? "yes" : "no"}`,
        `UI turns over budget (recorded/sustained): ${ledger}`,
        `Worker steps over budget (recorded/sustained): ${workerLedger}${steps.degraded ? " · deferred cadence" : ""}`,
    ].join("\n");
}
function renderFault(root: HTMLElement, code: string, detail: string, state?: BrowserFrameFallbackState): void {
    console.error(`wgpu renderer fault: ${code}: ${detail}`);
    const banner = document.createElement("div");
    banner.setAttribute("role", "alert");
    banner.style.cssText = "position:fixed;inset:0;padding:24px;background:#2a0a0acc;color:#ffb4b4;font:14px monospace;white-space:pre-wrap;overflow:auto;z-index:9999;";
    const tongue = root.getAttribute("lang") === "de" ? "de" : "en";
    const title = tongue === "de" ? "wgpu-Renderer-Fehler" : "wgpu renderer fault";
    banner.textContent = `${title}:\n\n${code}: ${detail}\n\n${fallbackLines(state, tongue)}`;
    root.appendChild(banner);
}
function wireInput(root: HTMLElement, canvas: HTMLCanvasElement, transport: BrowserFrameTransport): () => void {
    const abort = new AbortController();
    const options = { signal: abort.signal };
    const observed = (site: string, startedAt: number) => void transport.observeUiTurn(site, performance.now() - startedAt);
    const admit = (dom: BrowserFrameDomEvent, startedAt: number): void => {
        const event = browserFrameEventFromDom(dom, window.devicePixelRatio || 1);
        if (browserFrameEventIsReplaceable(event))
            transport.enqueueReplaceable(event);
        else
            transport.enqueueLossless(event);
        observed(dom.type, startedAt);
    };
    canvas.addEventListener("pointermove", (event) => admit(browserFramePointerDomEvent(event, "pointermove"), performance.now()), options);
    canvas.addEventListener("pointerdown", (event) => {
        const startedAt = performance.now();
        canvas.focus({ preventScroll: true });
        canvas.setPointerCapture(event.pointerId);
        admit(browserFramePointerDomEvent(event, "pointerdown"), startedAt);
    }, options);
    canvas.addEventListener("pointerup", (event) => admit(browserFramePointerDomEvent(event, "pointerup"), performance.now()), options);
    canvas.addEventListener("pointercancel", (event) => admit(browserFramePointerDomEvent(event, "pointercancel"), performance.now()), options);
    canvas.addEventListener("wheel", (event) => {
        const startedAt = performance.now();
        event.preventDefault();
        admit(browserFrameWheelDomEvent(event), startedAt);
    }, { ...options, passive: false });
    const cleanupKeyboard = wireBrowserKeyboard(root, canvas, event => admit(event, performance.now()));
    canvas.addEventListener("compositionstart", () => {
        const startedAt = performance.now();
        transport.enqueueLossless({ kind: "ime-start" });
        observed("ime-start", startedAt);
    }, options);
    canvas.addEventListener("compositionupdate", (event) => {
        const startedAt = performance.now();
        transport.enqueueLossless({ kind: "ime-update", text: event.data, cursor: event.data.length });
        observed("ime-update", startedAt);
    }, options);
    canvas.addEventListener("compositionend", (event) => {
        const startedAt = performance.now();
        transport.enqueueLossless({ kind: "ime-commit", text: event.data });
        observed("ime-commit", startedAt);
    }, options);
    canvas.addEventListener("paste", (event) => {
        const startedAt = performance.now();
        const candidate = browserClipboardPasteCandidate(event.clipboardData?.items);
        if (candidate) {
            if (candidate.kind === "image") {
                event.preventDefault();
                const reader = new FileReader();
                reader.addEventListener("load", () => {
                    if (typeof reader.result !== "string")
                        return;
                    const handoffStartedAt = performance.now();
                    transport.enqueueLossless({ kind: "paste-image-data-url", text: reader.result });
                    transport.observeUiTurn("paste-image-handoff", performance.now() - handoffStartedAt);
                }, { once: true });
                reader.readAsDataURL(candidate.file);
                observed("paste", startedAt);
                return;
            }
            event.preventDefault();
            candidate.item.getAsString((text) => {
                const handoffStartedAt = performance.now();
                transport.enqueueLossless({ kind: "paste", text });
                transport.observeUiTurn("paste-handoff", performance.now() - handoffStartedAt);
            });
        }
        observed("paste", startedAt);
    }, options);
    root.ownerDocument.addEventListener("visibilitychange", () => {
        if (root.ownerDocument.visibilityState === "hidden")
            transport.setHostPageHidden();
    }, options);
    root.ownerDocument.defaultView?.addEventListener("blur", () => transport.setHostWindowBlur(), options);
    return () => {
        cleanupKeyboard();
        abort.abort();
    };
}
/** 🎓️ One mount's introduction policy, carried before boot and on live host changes. */
export type WgpuBrowserMount = (() => Promise<void>) & { readonly setIntroductionSuppressed: (suppressed: boolean) => void; readonly introspection: WgpuIntrospection };

type RootOwner = { readonly ready: Promise<WgpuBrowserMount>; readonly abort: AbortController };
const mountedRoots = new WeakMap<HTMLElement, RootOwner>();

/** 🧵️ Cancels prior pending ownership and serializes replacement through complete retirement. */
export function mountWgpuBrowserHost(root: HTMLElement, options: WgpuBrowserHostOptions): Promise<WgpuBrowserMount> {
    if (options.signal?.aborted) return Promise.reject(options.signal.reason);
    try { options = { ...options, pluginRegistrySelection: admitWgpuPluginRegistrySelection(options.pluginRegistrySelection), plugins: options.plugins === undefined ? undefined : admitWgpuPluginModules(options.plugins, window.location.href) }; }
    catch (error) { return Promise.reject(error); }
    const abort = new AbortController();
    const relay = () => abort.abort(options.signal?.reason);
    options.signal?.addEventListener("abort", relay, { once: true });
    const detach = () => options.signal?.removeEventListener("abort", relay);
    abort.signal.addEventListener("abort", detach, { once: true });
    const previous = mountedRoots.get(root);
    previous?.abort.abort();
    const retiring = previous ? previous.ready.then(dispose => dispose(), () => {}) : Promise.resolve();
    const ready = retiring.then(async () => {
        abort.signal.throwIfAborted();
        return createWgpuBrowserHost(root, { ...options, signal: abort.signal }, release);
    });
    const owner: RootOwner = { ready, abort };
    const release = () => { detach(); if (mountedRoots.get(root) === owner) mountedRoots.delete(root); };
    mountedRoots.set(root, owner);
    void ready.catch(release);
    return ready;
}

/** 🪆️ Creates one independently rooted browser renderer and awaits its Worker boot. */
async function createWgpuBrowserHost(root: HTMLElement, options: WgpuBrowserHostOptions, onRetired: () => void): Promise<WgpuBrowserMount> {
    options.signal?.throwIfAborted();
    const plugins = options.plugins === undefined ? undefined : admitWgpuPluginModules(options.plugins, window.location.href);
    armUiTurnDiagnostics();
    const descriptor = options.descriptor;
    const introductionSuppressed = (suppressed: boolean) => window.self !== window.top || suppressed;
    const beacon = wgpuReadinessBeacon(options.pageBindings ? document.documentElement : root, descriptor.pluginVariant);
    if (typeof Worker === "undefined")
        throw new Error("worker-unavailable: Dedicated Worker is not supported");
    const canvas = canvasElement(options.locale, options.pageBindings ? WGPU_CANVAS_ID : undefined);
    if (typeof canvas.transferControlToOffscreen !== "function")
        throw new Error("offscreen-canvas-unavailable: OffscreenCanvas transfer is not supported");
    root.style.position = "relative";
    root.setAttribute("lang", options.locale);
    root.replaceChildren(canvas);
    const status = statusElement(root);
    const dpr = window.devicePixelRatio || 1;
    const width = Math.max(1, Math.round(canvas.clientWidth * dpr));
    const height = Math.max(1, Math.round(canvas.clientHeight * dpr));
    canvas.width = width;
    canvas.height = height;
    let offscreen: OffscreenCanvas;
    try {
        offscreen = canvas.transferControlToOffscreen();
    }
    catch (error) {
        throw new Error(`offscreen-transfer-failed: ${error instanceof Error ? error.message : String(error)}`);
    }
    let worker: Worker;
    try {
        worker = new Worker(stampShardWorkerDiagnostics(options.frameWorkerUrl ?? FRAME_WORKER_URL.href, turnDiagnosticsEnabled()), { type: "module", name: "semio-frame-worker" });
    }
    catch (error) {
        throw new Error(`worker-construction-failed: ${error instanceof Error ? error.message : String(error)}`);
    }
    let resolveReady!: () => void;
    let rejectReady!: (error: unknown) => void;
    const ready = new Promise<void>((resolve, reject) => { resolveReady = resolve; rejectReady = reject; });
    let disposed = false;
    let bootComplete = false;
    let cleanupInput = () => { };
    const fullscreenOwner = wireBrowserFullscreen(root, canvas);
    let detachIntrospection = () => { };
    let accessibility: {
        readonly refresh: () => void;
        readonly dispose: () => void;
    } | undefined;
    let mediaOverlay: ReturnType<typeof createBrowserMediaOverlay> | undefined;
    const transport = new BrowserFrameTransport({
        worker,
        boot: { bindingsModuleUrl: options.rendererModuleUrl ?? RENDERER_MODULE_URL, bindingsWasmUrl: options.rendererWasmUrl ?? RENDERER_WASM_URL, canvas: offscreen, width, height, dpr, plugins, pluginRegistrySelection: options.pluginRegistrySelection, locale: options.locale, introductionSuppressed: introductionSuppressed(options.suppressAutoIntroduction === true), descriptor, appearance: hostAppearance(), platform: hostPlatform(), storage: hostStorage() },
        requestAnimationFrame: (callback) => window.requestAnimationFrame(callback),
        cancelAnimationFrame: (handle) => window.cancelAnimationFrame(handle),
        hostIo: createWgpuPageHostIo(),
        onProgress: (stage, progress, worker) => {
            const contract = bootLifecycleSchema["x-semio-lifecycle"];
            if (disposed || bootComplete || options.signal?.aborted || typeof stage !== "string" || stage.length === 0 || stage.length > contract.maximumStageScalars * 2 || [...stage].length > contract.maximumStageScalars || !Number.isFinite(progress) || progress < contract.minimumProgress || progress > contract.maximumProgress) return;
            try { options.onProgress?.({ stage, progress, worker: { ...worker } }); } catch { }
            status.textContent = `${stage} ${Math.round(progress * 100)}%${worker.degraded ? (options.locale === "de" ? " · verzögerte Taktung" : " · deferred cadence") : ""}`;
            status.dataset.workerDegraded = worker.degraded ? "true" : "false";
            status.dataset.workerStepOverruns = String(worker.recordedOverruns);
        },
        onUiTurn: (outcome) => {
            canvas.dataset.uiTurn = `${outcome.verdict}:${outcome.site}:${outcome.executingMs.toFixed(3)}`;
        },
        onReady: () => {
            if (disposed || options.signal?.aborted)
                return;
            bootComplete = true;
            beacon.ready();
            status.remove();
            if (options.pageBindings)
                detachIntrospection = attachIntrospectionBindings(transport);
            mediaOverlay = createBrowserMediaOverlay(root, (slot) => transport.mediaPort(slot.token));
            accessibility = createAccessibilityMirror(root, transport, options.locale, canvas, (surface, node) => mediaOverlay?.owns(surface.windowId, node.nodeId, node.key) === true, options.pageBindings ? undefined : "semio-wgpu-accessibility-" + options.mountId);
            accessibility.refresh();
            cleanupInput = wireInput(root, canvas, transport);
            transport.enqueueReplaceable(browserFrameEventFromDom({ type: "resize", clientWidth: canvas.clientWidth, clientHeight: canvas.clientHeight }, dpr) as Extract<ReturnType<typeof browserFrameEventFromDom>, {
                kind: "resize";
            }>);
            if (options.pageBindings)
                canvas.focus({ preventScroll: true });
            resolveReady();
        },
        onDirectives: ({ cursor, fullscreen, mediaSlots }) => {
            mediaOverlay?.accept(mediaSlots);
            canvas.style.cursor = cursor;
            if (typeof fullscreen === "boolean")
                void fullscreenOwner.set(fullscreen).catch(() => { });
            accessibility?.refresh();
        },
        onDiagnostic: ({ channel, generation, frameSequence, json }) => console.debug(`[TRACE] wgpu ${channel} generation=${generation} frame=${frameSequence} ${json}`),
        onFault: (code: BrowserFrameWorkerFaultCode, detail, fallback) => {
            beacon.error();
            cleanupInput();
            fullscreenOwner.dispose();
            detachIntrospection();
            mediaOverlay?.dispose();
            accessibility?.dispose();
            renderFault(root, code, detail, fallback);
            rejectReady(new Error(code + ": " + detail));
        },
    });
    const previousInteractiveJobPort = options.pageBindings ? setInteractiveJobPort(transport.interactiveJobs) : undefined;
    const publishMetrics = (site: string) => {
        const startedAt = performance.now();
        const nextDpr = window.devicePixelRatio || 1;
        transport.enqueueReplaceable(browserFrameEventFromDom({ type: "resize", clientWidth: canvas.clientWidth, clientHeight: canvas.clientHeight }, nextDpr) as Extract<ReturnType<typeof browserFrameEventFromDom>, {
            kind: "resize";
        }>);
        transport.observeUiTurn(site, performance.now() - startedAt);
    };
    const resize = new ResizeObserver(() => publishMetrics("resize-observer"));
    resize.observe(canvas);
    let resolutionQuery: MediaQueryList | undefined;
    const armResolutionQuery = () => {
        resolutionQuery?.removeEventListener("change", onResolutionChange);
        resolutionQuery = window.matchMedia?.(`(resolution: ${window.devicePixelRatio || 1}dppx)`);
        resolutionQuery?.addEventListener("change", onResolutionChange);
    };
    function onResolutionChange() {
        publishMetrics("resolution-change");
        armResolutionQuery();
    }
    armResolutionQuery();
    const republishAppearance = () => transport.setHostAppearance(hostAppearance());
    const republishHostStorage = () => transport.setHostStorage(hostStorage());
    const darkQuery = window.matchMedia?.(WGPU_PREFERS_DARK_MEDIA_QUERY);
    darkQuery?.addEventListener("change", republishAppearance);
    window.addEventListener("storage", republishAppearance);
    window.addEventListener("storage", republishHostStorage);
    const stopAgentBridgeOffer = watchAgentBridgeOffer((offer) => transport.setHostAgentBridge(offer), { offerScope: async () => agentBridgeOfferScopeFromJsonV1(await transport.introspect("agent-bridge-scope")) });
    let retirement: Promise<void> | undefined;
    const dispose = () => {
        if (retirement)
            return retirement;
        disposed = true;
        if (!bootComplete) rejectReady(new DOMException("Boot retired", "AbortError"));
        options.signal?.removeEventListener("abort", cancel);
        window.removeEventListener("pagehide", dispose);
        stopAgentBridgeOffer();
        beacon.clear();
        resize.disconnect();
        darkQuery?.removeEventListener("change", republishAppearance);
        window.removeEventListener("storage", republishAppearance);
        window.removeEventListener("storage", republishHostStorage);
        resolutionQuery?.removeEventListener("change", onResolutionChange);
        cleanupInput();
        fullscreenOwner.dispose();
        detachIntrospection();
        mediaOverlay?.dispose();
        accessibility?.dispose();
        if (previousInteractiveJobPort)
            setInteractiveJobPort(previousInteractiveJobPort);
        transport.close();
        retirement = transport.retirement.then(() => { root.replaceChildren(); onRetired(); });
        return retirement;
    };
    const cancel = () => {
        if (!bootComplete) rejectReady(options.signal?.reason ?? new DOMException("Boot cancelled", "AbortError"));
        void dispose();
    };
    options.signal?.addEventListener("abort", cancel, { once: true });
    if (options.signal?.aborted) cancel();
    window.addEventListener("pagehide", dispose, { once: true });
    try {
        await ready;
    }
    catch (error) {
        await dispose();
        throw error;
    }
    return Object.assign(dispose, { introspection: mountIntrospection(transport), setIntroductionSuppressed: (suppressed: boolean) => transport.setHostIntroductionSuppressed(introductionSuppressed(suppressed)) });
}
