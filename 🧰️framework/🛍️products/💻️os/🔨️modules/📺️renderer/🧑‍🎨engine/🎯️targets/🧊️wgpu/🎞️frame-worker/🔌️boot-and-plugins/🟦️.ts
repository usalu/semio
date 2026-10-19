import { fetchPackageDescriptor, PlaygroundBootPlanner, pluginGraphErrorMessage } from "@semio-tech/framework";
import { loadPluginModule } from "../../🐚️plugin-bridge/🔌️plugin-module-loader/🟦️.ts";
import { pluginHandleForBridge } from "../../🐚️plugin-bridge/🟦️.ts";
import { declaredStep, driveChunks, macrotask, monitoredSuspension, ownedStep, post, progress } from "../⏱️steps-and-reporting/🟦️.ts";
import { bindings, bootAbort, closed, closing, consumedTopics, diagnosticsStamp, explicitPluginModules, frameTurns, frameTurnTasks, lifecycle, originalFrameSource, runtime, scope, setBindings, setBootDeclarationsOpen, setExplicitPluginModules, setFrameInput, setFrameTurns, setFrameTurnTasks, setInteractiveJobs, setLifecycle, setOriginalFrameSource, setRuntime, setWorkerCatalog, suspensionLedger, typedMediaPlugins, workerCatalog } from "../🧵️state/🟦️.ts";
import type { BrowserRendererBootStep, RendererBindings } from "../🧬️bindings-types/🟦️.ts";
import { evictCachedRendererModule, readCachedRendererModule, rendererArtifactTag, writeCachedRendererModule } from "../../🗄️wasm-module-cache/🟦️.ts";
import type { BrowserFrameUiMessage } from "../../🚚️browser-frame-transport/📋️contract/🟦️.ts";
import { fault } from "../🧯️close-and-fault/🟦️.ts";
import { emptyNativeCatalogV1, readNativeCatalogV1 } from "@semio-tech/plugin-registry/runtime";
import { installRuntimeAssets } from "../../../../../../../../../🔨️modules/🖼️canvas/🔌️runtime-assets/🟦️.ts";
import { admitPluginCatalogV1, pluginCatalogV1 } from "../../../../../../🔌️plugin/📇️registry/🟦️.ts";
import { admitWgpuPluginModules, assertWgpuPluginPlan, prepareWgpuPluginModules } from "../../🧩️plugin-modules/🟦️.ts";
import { componentPackageReceiptSource } from "@semio-tech/framework-plugin/receipt";
import { admitWgpuPluginRegistrySelection } from "../../🧩️plugin-modules/🛂️admission/🟦️.ts";
import { frameOriginalWorkOwner } from "../../🐚️plugin-bridge/🧵️shard-and-actor-calls/🟦️.ts";
import { OriginalBrowserFrameSource } from "../🎟️original/🟦️.ts";
import { FrameTurnScheduler, WorkerTurnTaskQueue } from "../../🧵️frame-turn-scheduler/🟦️.ts";
import { runAssetDecodeTurn, runFrameTurn } from "../🎞️frame-and-introspection/🟦️.ts";
import { INTERACTIVE_WORKER_DESCRIPTORS, InteractiveWorkerScheduler } from "../../📇️interactive-job-registry/🟦️.ts";
import { scheduleAssetPump } from "../📦️assets-and-host-io/🟦️.ts";

type PluginHandleMount = { readonly pluginId: string; readonly typedHandle: Awaited<ReturnType<typeof loadPluginModule>>; readonly handle: ReturnType<typeof pluginHandleForBridge> };

/** 🧩️ Mounts every plugin the boot plan names, isolating each one. A module that fails to load, or
 * whose manifest overruns its fixed credits, is reported as a `plugin-fault:` boot-progress stage and
 * skipped — the same per-plugin isolation the React shell's router gives a descriptor fault, rather than
 * taking the whole surface down. The catalogue cache legitimately carries stale or missing descriptors
 * while plugin cores are rebuilt, and a shell that boots only when all of them are fresh never boots.
 * Cancellation (`closed`/`closing`) is re-thrown, never swallowed; an empty result is still fatal upstream. */
async function mountPluginHandles(targets: readonly { readonly pluginId: string; readonly moduleUrl: string }[], descriptors?: ReadonlyMap<string, import("@semio-tech/framework").PluginPackageDescriptor>): Promise<PluginHandleMount[]> {
  const mounted: PluginHandleMount[] = [];
  for (let index = 0; index < targets.length; index++) {
    const target = targets[index]!;
    const share = 0.3 + 0.3 * (index / Math.max(1, targets.length));
    progress(`plugin:${target.pluginId}`, share);
    await macrotask();
    try {
      const module = await monitoredSuspension(`plugin:${target.pluginId}`, () => loadPluginModule(target.pluginId, target.moduleUrl, bootAbort.signal, descriptors?.get(target.pluginId), consumedTopics(target.pluginId)), suspensionLedger);
      typedMediaPlugins.set(target.pluginId, module);
      mounted.push(ownedStep(`plugin-handle:${target.pluginId}`, () => ({ pluginId: target.pluginId, typedHandle: module, handle: pluginHandleForBridge(module) })));
    } catch (error) {
      if (closed || closing || descriptors) throw error;
      progress(`plugin-fault:${target.pluginId}: ${error instanceof Error ? error.message : String(error)}`, share);
    }
  }
  return mounted;
}

/** 📶️ How many buckets the 76 MB download reports itself in — one `boot-progress` every 2 %, which
 * is real streamed progress rather than a stage name that sits still for the whole transfer, and is far
 * below the rate at which the UI isolate's `progress-hook` would become the expensive thing. */
const WASM_FETCH_PROGRESS_BUCKETS = 50;

/** 🧱️ Compiles the renderer wasm WHILE it downloads, reporting the transfer as it goes.
 * `compileStreaming` over a counting `TransformStream` keeps the browser's streaming compilation — the
 * bytes are never buffered into one 76 MB array — while every 2 % of the body posts a `boot-progress`, so
 * the longest phase of the boot stops being a stage name that sits still. Answers `undefined` for every
 * failure (a browser that will not `compileStreaming` a constructed `Response`, a transport fault), which
 * hands the caller back to wasm-bindgen's own URL path rather than failing the boot. */
async function compileRendererModule(url: string): Promise<{ readonly module: WebAssembly.Module; readonly byteLength: number } | undefined> {
  try {
    const response = await fetch(url);
    if (!response.ok || !response.body) return undefined;
    const total = Number(response.headers.get("content-length") ?? 0);
    let received = 0;
    let reported = -1;
    const counted = response.body.pipeThrough(
      new TransformStream<Uint8Array, Uint8Array>({
        transform(chunk, controller) {
          received += chunk.byteLength;
          const share = total > 0 ? received / total : 0;
          const bucket = Math.floor(share * WASM_FETCH_PROGRESS_BUCKETS);
          if (total > 0 && bucket > reported) {
            reported = bucket;
            progress(`wasm-compile ${Math.round(share * 100)} %`, 0.06 + 0.09 * share);
          }
          controller.enqueue(chunk);
        },
      }),
    );
    const module = await WebAssembly.compileStreaming(new Response(counted, { headers: { "content-type": "application/wasm" } }));
    return { module, byteLength: received };
  } catch {
    return undefined;
  }
}

/** 🧱️ Brings the renderer wasm up in DECLARED phases instead of one opaque `init(url)`.
 *
 * 🩸️ What this replaces: `loaded.default(url)` — one call that fetched, compiled, instantiated and linked
 * 76 048 601 B behind a single stage name, reported nothing while it ran, and re-did all of it on every
 * reload. The four phases here each declare themselves to the UI-isolate watchdog before they block
 * (`monitoredSuspension`), the compile reports its own transfer, and the compiled module is kept in
 * IndexedDB (`../🗄️wasm-module-cache/🟦️.ts`) so the second boot on an unchanged artifact skips the compile
 * entirely and goes straight to instantiation. A cache miss costs one `HEAD`; a browser that cannot cache
 * a `WebAssembly.Module` at all costs the same `HEAD` and nothing else. */
async function instantiateRendererWasm(bindings: RendererBindings, url: string): Promise<void> {
  const tag = await monitoredSuspension("wasm-artifact", () => rendererArtifactTag(url), suspensionLedger);
  const cached = tag ? await monitoredSuspension("wasm-cache-read", () => readCachedRendererModule(url, tag), suspensionLedger) : undefined;
  if (cached) {
    progress(`wasm-instantiate:cached ${(cached.byteLength / 1_048_576).toFixed(1)} MB`, 0.18);
    await monitoredSuspension("wasm-instantiate", () => bindings.default!(cached.module), suspensionLedger);
    return;
  }
  progress("wasm-compile", 0.06);
  const compiled = await monitoredSuspension("wasm-compile", () => compileRendererModule(url), suspensionLedger);
  progress("wasm-instantiate", 0.18);
  if (!compiled) {
    await monitoredSuspension("wasm-instantiate", () => bindings.default!(url), suspensionLedger);
    return;
  }
  await monitoredSuspension("wasm-instantiate", () => bindings.default!(compiled.module), suspensionLedger);
  if (tag) void writeCachedRendererModule(url, tag, compiled.module, compiled.byteLength, performance.now());
  else void evictCachedRendererModule(url);
}

export async function boot(message: Extract<BrowserFrameUiMessage, { kind: "boot" }>): Promise<void> {
  if (runtime || lifecycle !== 0) {
    fault("duplicate-boot", "the frame Worker accepts exactly one boot lifecycle");
    return;
  }
  setLifecycle(message.lifecycle);
  try {
    progress("renderer-module", 0.05);
    const loaded = await monitoredSuspension("renderer-module", () => import(/* @vite-ignore */ message.bindingsModuleUrl) as Promise<RendererBindings>, suspensionLedger);
    setBindings(loaded);
    if (loaded.default) await instantiateRendererWasm(loaded, message.bindingsWasmUrl);
    if (!loaded.semioWgpuWorkerBootstrap) throw new Error("renderer bindings missing semioWgpuWorkerBootstrap");
    ownedStep("runtime-environment", () => {
      const nativeCatalog=readNativeCatalogV1(message.nativeCatalog??emptyNativeCatalogV1());
      if(!loaded.semioWgpuSetPluginCatalog&&nativeCatalog.plugins.length)throw Error("Renderer bindings lack supplied native catalog admission");
      loaded.semioWgpuSetPluginCatalog?.(JSON.stringify(nativeCatalog));
      loaded.semioWgpuSetRuntimeDiagnostics?.(diagnosticsStamp === true);
      loaded.semioWgpuSetBootDescriptor?.(JSON.stringify(message.descriptor));
      loaded.semioWgpuSetHostStorage?.(JSON.stringify(message.storage));
      loaded.semioWgpuSetHostAppearance?.(message.appearance.preference, message.appearance.systemDark);
      loaded.semioWgpuSetHostPlatform?.(message.platform);
      loaded.semioWgpuSetHostLocale?.(message.locale);
      loaded.semioWgpuSetHostIntroductionSuppressed?.(message.introductionSuppressed === true);
      if (message.descriptor.hub) loaded.semioWgpuSetHubEnv?.(message.descriptor.hub.hubUrl, message.descriptor.hub.user, message.descriptor.hub.dataDir);
    }, suspensionLedger);
    progress("runtime-assets", 0.2);
    try {
      await monitoredSuspension("runtime-assets", () => installRuntimeAssets(loaded, scope.location.href, (url) => fetch(url)), suspensionLedger);
    } catch (error) {
      if (closed || closing) throw error;
      progress(`runtime-assets-fault: ${error instanceof Error ? error.message : String(error)}`, 0.2);
    }
    progress("plugin-graph", 0.25);
    const catalogRows = admitPluginCatalogV1(message.catalog, { maxBytes: 2097152, maxRows: 512, maxEdges: 4096, maxWork: 65536, deadlineMs: performance.now() + 30000, now: () => performance.now(), cancelled: () => bootAbort.signal.aborted, progress: ({ completed, total }) => progress("catalog-admission", 0.25 + 0.005 * completed / Math.max(1, total)) });
    setWorkerCatalog(pluginCatalogV1(catalogRows));
    setExplicitPluginModules(admitWgpuPluginModules(message.plugins ?? catalogRows.targets.map(({ pluginId, moduleUrl }) => ({ pluginId, moduleUrl })), scope.location.href));
    const prepared = await prepareWgpuPluginModules(workerCatalog!, explicitPluginModules!, componentPackageReceiptSource, {
      signal: bootAbort.signal,
      deadlineMs: performance.now() + 30000,
      now: () => performance.now(),
      maxDescriptors: explicitPluginModules!.length,
      yieldTurn: macrotask,
      readDescriptor: (id, url, source, signal, heartbeat) => monitoredSuspension(`plugin-descriptor:${id}`, () => fetchPackageDescriptor(id, url, source, signal, heartbeat), suspensionLedger),
      progress: (id, index, count) => progress(`plugin-descriptor:${id}`, 0.25 + 0.025 * index / Math.max(1, count)),
    });
    const selection = admitWgpuPluginRegistrySelection(message.pluginRegistrySelection);
    const planner = new PlaygroundBootPlanner(prepared.catalog, message.descriptor.pluginVariant, undefined, selection);
    await driveChunks(planner, 0.25, 0.05);
    const bootPlan = ownedStep("plugin-graph:finish", () => planner.finish());
    assertWgpuPluginPlan(bootPlan, prepared, selection);
    const pluginCredits = explicitPluginModules!.length;
    if (bootPlan.plugins.length > pluginCredits) throw new Error(`plugin-credits: boot plan exceeds ${pluginCredits} plugins`);
    for (const error of bootPlan.dependencyErrors) progress(pluginGraphErrorMessage(error, message.locale), 0.3);
    const plugins = await mountPluginHandles(bootPlan.plugins, prepared.packages);
    if (plugins.length === 0 && (selection !== "all" || explicitPluginModules!.length !== 0)) throw new Error(`no wasm plugin modules found for variant ${message.descriptor.pluginVariant}`);
    progress("renderer-runtime", 0.65);
    let bootstrap = await monitoredSuspension("gpu-platform", () => loaded.semioWgpuWorkerBootstrap!(message.canvas, plugins, bootPlan.variant, message.width, message.height, message.dpr, () => frameTurns?.requestRuntimeWake()), suspensionLedger);
    while (true) {
      await macrotask();
      const step = declaredStep("renderer-bootstrap", () => JSON.parse(bootstrap.step()) as BrowserRendererBootStep, suspensionLedger);
      if (diagnosticsStamp === true) console.debug(`[TRACE] wgpu boot stage=${step.stage} phaseUs=${step.elapsedUs} progress=${step.progress}`);
      progress(step.stage, 0.65 + step.progress * 0.3);
      if (step.shellBoot) {
        bootstrap = await monitoredSuspension("shell-boot", () => bootstrap.bootShell(), suspensionLedger);
        continue;
      }
      if (step.complete) break;
    }
    setRuntime(declaredStep("renderer-finish", () => bootstrap.finish(), suspensionLedger));
    const originalFrameOwner=frameOriginalWorkOwner();
    const originalFrameAdmission=await originalFrameOwner.admit();
    setOriginalFrameSource(new OriginalBrowserFrameSource(runtime!,originalFrameOwner,originalFrameAdmission,()=>BigInt(Math.floor(performance.now()*1000))));
    (scope as unknown as {semioOriginalFrameClockUs:()=>bigint}).semioOriginalFrameClockUs=()=>originalFrameSource!.operationClockUs();
    await originalFrameSource!.admit();
    setFrameTurnTasks(declaredStep("frame-turn-task-owner", () => new WorkerTurnTaskQueue(), suspensionLedger));
    setFrameTurns(declaredStep("frame-turn-scheduler", () => new FrameTurnScheduler(frameTurnTasks!.schedule, runFrameTurn, () => {
      setFrameInput(undefined);
      return true;
    }, runAssetDecodeTurn), suspensionLedger));
    setInteractiveJobs(declaredStep("interactive-job-registry", () => new InteractiveWorkerScheduler(lifecycle, INTERACTIVE_WORKER_DESCRIPTORS, post, (callback) => setTimeout(callback, 0), () => performance.now(), (detail) => fault("interactive-job-fault", detail)), suspensionLedger));
    progress("ready", 1);
    setBootDeclarationsOpen(false);
    post({ kind: "booted", lifecycle });
    scheduleAssetPump();
  } catch (error) {
    if (!closed && !closing) fault("worker-boot-failed", error instanceof Error ? error.message : String(error));
  }
}
