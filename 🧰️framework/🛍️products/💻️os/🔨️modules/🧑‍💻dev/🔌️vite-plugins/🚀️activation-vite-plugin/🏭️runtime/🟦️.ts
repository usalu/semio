/** 🚀️ The runtime of the plugin activation Vite plugin: receipt observation, freshness reporting and lazy plugin materialization. The plugin shell loads it at server-configure time so the shell stays config-shaped. */
import { scriptCompilerAdmissionControlV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import { receiveScriptProcessInvocation } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { checkResourceLeaseControl } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import { createHash } from "node:crypto";
import { existsSync, readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { DEV_STREAM_ROUTES } from "@semio-tech/framework-os";
import type { PluginSourceEvent } from "@semio-tech/framework";
import { MODULE_BRIDGE_FILE } from "@semio-tech/plugin-registry/deployment";
import { resolveBootSourceContentHashes, SOURCE_FRESHNESS_COMPONENT_CONCURRENCY, mapBoundedV1, stagedModuleMtime, writeStagedSourceFreshness } from "@semio-tech/framework-plugin/build/freshness";
import { ACTIVATION_RECEIPT_FILE, activationSourceFreshnessDirectory, activationSourceFreshnessKey, nextActivationReceipt, observeActivationReceipts, publishActivationReceipt, readActivationReceipt, stagedModuleReportLines, stagedModuleVerdict, type ActivationReceipt, type StagedModuleFacts } from "../../../♻️activation/🟦️.ts";
import { STREAM_MUX_BOUNDS_V1, type StreamMuxJobV1, type StreamMuxJsonV1 } from "@semio-tech/framework/io/stream-mux";
import { REPO_ROOT } from "../../🧭️repo-root/🟦️.ts";
import { devStreamMuxServer, type DevUpgradeServer } from "../../🔀️stream-mux-server/🟦️.ts";

import type { ActivationComponentSpec, ActivationVitePluginOptions } from "../🟦️.ts";
import type { BackboneServerRequest, BackboneServerResponse } from "../../🧱️backbone-vite-plugin/🟦️.ts";

/** 📥️ Filename owned by plugin store installation; inlined so the vite-plugin graph does not pull materialization. */
const EXTENSION_INSTALL_META = "📥️install.json";

/** 🖥️ The Vite dev-server surface the activation runtime reads. */
export type ActivationServer = {
  middlewares: { use: (handler: (req: BackboneServerRequest, res: BackboneServerResponse, next: () => void) => void) => void };
  httpServer?: (DevUpgradeServer & { listening: boolean; once: (event: "close" | "listening", listener: () => void) => unknown }) | null;
  ws?: { send: (message: { type: "full-reload" }) => void };
};

//#region 🔌️PluginActivationRuntime
/** 🔎️ Re-runs the staged-module freshness rule against the receipt the dev server just observed and
 * resolves to one `[stale]` line per component whose served bytes are behind — the live half of the serve-start
 * pass in `📜️script.ts`. A restage that lands while the server runs therefore retires its own warning
 * without a restart, and one that never lands keeps saying so. Asynchronous and bounded (the server keeps answering
 * while it walks); `signal` cancels a pass a newer receipt superseded. */
export async function reportActivationFreshness(receipt: ActivationReceipt, options: { readonly moduleRoot: string; readonly installRoot: string; readonly freshnessRoot?: string; readonly components: readonly ActivationComponentSpec[]; readonly signal?: AbortSignal }): Promise<readonly string[]> {
  const activatedRows = new Map(receipt.plugins.map((row) => [row.pluginId, row]));
  const facts = await mapBoundedV1(options.components, SOURCE_FRESHNESS_COMPONENT_CONCURRENCY, async (component): Promise<StagedModuleFacts> => {
    const moduleDirectory = join(options.moduleRoot, component.directoryName);
    const receiptRow = activatedRows.get(component.pluginId);
    const installedMeta = join(component.installDirectory ?? join(options.installRoot, component.directoryName), EXTENSION_INSTALL_META);
    let installedPackageHash: string | undefined;
    if (existsSync(installedMeta)) {
      try { installedPackageHash = JSON.parse(readFileSync(installedMeta, "utf8")).packageHash as string; } catch { installedPackageHash = undefined; }
    }
    const hashes = await resolveBootSourceContentHashes({
      sourceRoot: component.sourceRoot,
      moduleDirectory: options.freshnessRoot ? join(options.freshnessRoot, activationSourceFreshnessKey(component.pluginId)) : moduleDirectory,
      receiptSourceContentSha256: (receiptRow as { sourceContentSha256?: string } | undefined)?.sourceContentSha256,
      signal: options.signal,
    });
    return {
      pluginId: component.pluginId,
      role: component.role,
      activationTracked: true,
      stagedAtMs: stagedModuleMtime(moduleDirectory),
      newestSourcePath: hashes.newestSourcePath ? relative(REPO_ROOT, hashes.newestSourcePath).split(/[\\/]/).join("/") : undefined,
      sourceContentSha256: hashes.sourceContentSha256,
      stagedSourceContentSha256: hashes.stagedSourceContentSha256,
      receiptArtifactSha256: receiptRow?.artifactSha256,
      installedPackageHash,
    }
  }, options.signal);
  return stagedModuleReportLines(facts.map(stagedModuleVerdict), `bun repo run @semio-tech/framework-os-dev:activate-${receipt.variant}-react-${receipt.profile}`);
}

/** 📡️ Subscribes one dev server to activation receipts and lazy plugin materialization; the returned function releases every server-owned subscription. */
export function activateServer(server: ActivationServer, options: ActivationVitePluginOptions, publishStaleness: (lines: readonly string[]) => void): () => void {
  let dispose = (): void => {};
  const mux = devStreamMuxServer(server.httpServer);
  let previous: ActivationReceipt | undefined;
  const send = (event: PluginSourceEvent): void => mux.publish(DEV_STREAM_ROUTES.pluginModules, "", event as unknown as StreamMuxJsonV1);
  let freshness: AbortController | null = null;
  const observer = observeActivationReceipts(options.receiptDirectory, (receipt) => {
    const apply = (): void => {
      freshness?.abort();
      const pass = (freshness = new AbortController());
      void reportActivationFreshness(receipt, { ...options, freshnessRoot: join(dirname(options.receiptDirectory), "🔖️"), signal: pass.signal }).then((lines) => {
        if (pass.signal.aborted) return;
        publishStaleness(lines);
        for (const line of lines) console.warn(line);
      }, (error: unknown) => {
        if (!pass.signal.aborted) console.warn(`[stale] freshness check unavailable: ${String(error)}`);
      });
      if (previous) {
        if (previous.plugins.map((row) => row.pluginId).join() !== receipt.plugins.map((row) => row.pluginId).join()) server.ws?.send({ type: "full-reload" });
        const prior = new Map(previous.plugins.map((row) => [row.pluginId, row.artifactSha256]));
        for (const row of receipt.plugins) if (prior.get(row.pluginId) !== row.artifactSha256) send({ kind: "built", pluginId: row.pluginId, rebuiltAt: row.rebuiltAt });
      }
      previous = receipt;
    };
    // Serve-start owns the boot freshness pass. Skip the duplicate sync walk on first listen.
    if (!server.httpServer?.listening) {
      previous = receipt;
      return;
    }
    if (previous === undefined) {
      previous = receipt;
      return;
    }
    apply();
  }, (error) => console.error("Activation receipt failed:", error));
  const unrouteWatch = mux.route(DEV_STREAM_ROUTES.pluginModules, {
    admit: (key) => key === "",
    snapshot: () => ({ kind: "snapshot", plugins: observer.snapshot().plugins.map(({ pluginId, rebuiltAt }) => ({ pluginId, rebuiltAt })) }),
    coalesce: (event) => String((event as { readonly pluginId?: StreamMuxJsonV1 }).pluginId ?? "snapshot"),
  });
  dispose = (): void => {
    freshness?.abort();
    observer.close();
    unrouteWatch();
  };
  server.httpServer?.once("close", dispose);

  const jobs = new Map<string, { readonly controller: AbortController; readonly promise: Promise<void>; readonly listeners: Set<(line: string) => void>; owners: number }>();
  const profile: "prototype" | "release" = /(?:^|\/)release(?:\/|$)/.test(options.moduleRoot.replaceAll("\\", "/")) ? "release" : "prototype";

  const publishOne = async (pluginId: string): Promise<void> => {
    const component = options.components.find((row) => row.pluginId === pluginId);
    if (!component) throw new Error(`Unknown plugin ${pluginId}`);
    const moduleDirectory = join(options.moduleRoot, component.directoryName);
    if (!existsSync(join(moduleDirectory, MODULE_BRIDGE_FILE))) throw new Error(`Module still missing after materialize: ${pluginId}`);
    const sourceContentSha256 = await writeStagedSourceFreshness(activationSourceFreshnessDirectory(dirname(options.receiptDirectory), pluginId), component.sourceRoot);
    const previous = existsSync(join(options.receiptDirectory, ACTIVATION_RECEIPT_FILE)) ? readActivationReceipt(options.receiptDirectory) : undefined;
    const artifactSha256 = createHash("sha256").update(readFileSync(join(moduleDirectory, MODULE_BRIDGE_FILE))).digest("hex");
    const completed = [
      ...(previous?.plugins.filter((row) => row.pluginId !== pluginId) ?? []).map((row) => ({ pluginId: row.pluginId, artifactSha256: row.artifactSha256, sourceContentSha256: (row as { sourceContentSha256?: string }).sourceContentSha256 })),
      { pluginId, artifactSha256, sourceContentSha256 },
    ];
    const variant = previous?.variant ?? process.env.SEMIO_PLUGIN ?? "s";
    const receiptProfile = previous?.profile ?? profile;
    const receipt = nextActivationReceipt(variant, receiptProfile, completed, previous);
    publishActivationReceipt(options.receiptDirectory, receipt);
  };

  const leaseMaterialization = (pluginId: string, line?: (text: string) => void): { readonly done: Promise<void>; release(): void } => {
    let job = jobs.get(pluginId);
    if (job === undefined) {
      const component = options.components.find((row) => row.pluginId === pluginId);
      if (!component) return { done: Promise.reject(new Error(`Unknown plugin ${pluginId}`)), release: () => undefined };
      const controller = new AbortController();
      const listeners = new Set<(text: string) => void>();
      const promise = (async () => {
        console.log(`[lazy-activate] build ${pluginId} through the build driver`);
        const { buildComponentClosureV1 } = await import("@semio-tech/framework-plugin/build/driver-stages");
        await receiveScriptProcessInvocation(process.env,async original=>{
          const {repositoryCargoNetworkV1}=await import("@semio-tech/repo-lib/cargo");
          const parent=scriptCompilerAdmissionControlV1(original,"vite:lazy-activate:"+pluginId,0,controller.signal);
          const control={...parent,publish:async(progress:Parameters<typeof parent.publish>[0])=>{await parent.publish(progress);for(const listener of listeners)listener(`Waiting for compiler capacity (${progress.elapsedMs}ms)`);}};
          await buildComponentClosureV1(REPO_ROOT,new Set([component.pluginId]),{facet:"browser",invocation:original,control,network:repositoryCargoNetworkV1(original,REPO_ROOT),log:text=>{const trimmed=text.trim();if(trimmed)for(const listener of listeners)listener(trimmed.slice(0,STREAM_MUX_BOUNDS_V1.maxDetailLength));}});
          checkResourceLeaseControl(control);
          await publishOne(pluginId);
          checkResourceLeaseControl(control);
          const rebuiltAt=readActivationReceipt(options.receiptDirectory).plugins.find((row)=>row.pluginId===pluginId)?.rebuiltAt??Date.now();
          send({kind:"built",pluginId,rebuiltAt});
        }).catch((error:unknown)=>{throw controller.signal.aborted?new Error(`cancelled ${pluginId}`):error;});
      })().finally(() => { jobs.delete(pluginId); });
      job = { controller, promise, listeners, owners: 0 };
      jobs.set(pluginId, job);
    }
    const held = job;
    held.owners += 1;
    if (line !== undefined) held.listeners.add(line);
    let released = false;
    return {
      done: held.promise,
      release: () => {
        if (released) return;
        released = true;
        held.owners -= 1;
        if (line !== undefined) held.listeners.delete(line);
        if (held.owners === 0 && jobs.get(pluginId) === held) held.controller.abort(new Error(`released ${pluginId}`));
      },
    };
  };

  const unrouteActivation = mux.route(DEV_STREAM_ROUTES.pluginActivation, {
    admit: (pluginId) => options.components.some((row) => row.pluginId === pluginId),
    run: (pluginId: string, job: StreamMuxJobV1) =>
      new Promise<void>((resolvePromise, reject) => {
        const component = options.components.find((row) => row.pluginId === pluginId)!;
        if (existsSync(join(options.moduleRoot, component.directoryName, MODULE_BRIDGE_FILE))) return resolvePromise();
        let lines = 0;
        const lease = leaseMaterialization(pluginId, (text) => job.progress((lines += 1), null, text));
        job.signal.addEventListener("abort", () => {
          lease.release();
          reject(job.signal.reason);
        }, { once: true });
        lease.done.then(
          () => {
            lease.release();
            resolvePromise();
          },
          (error: unknown) => {
            lease.release();
            reject(error);
          },
        );
      }),
  });

  const prefetch = (): void => {
    const pending = options.components.filter((row) => !existsSync(join(options.moduleRoot, row.directoryName, MODULE_BRIDGE_FILE)));
    void (async () => {
      for (const row of [...pending].sort((a, b) => (a.pluginId < b.pluginId ? -1 : 1))) {
        if (existsSync(join(options.moduleRoot, row.directoryName, MODULE_BRIDGE_FILE))) continue;
        const lease = leaseMaterialization(row.pluginId);
        try { await lease.done; }
        catch (error) { console.warn(`[lazy-activate] prefetch ${row.pluginId}: ${String(error)}`); }
        finally { lease.release(); }
      }
    })();
  };
  server.httpServer?.once("listening", prefetch);
  const priorDispose = dispose;
  dispose = (): void => {
    unrouteActivation();
    for (const job of jobs.values()) job.controller.abort(new Error("dev server closing"));
    jobs.clear();
    priorDispose();
  };
  return () => dispose();
}
//#endregion 🔌️PluginActivationRuntime
