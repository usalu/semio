/// <reference types="vitest/importMeta" />
/** 🔌️ Plugin sources: the development, bundled and extension sources, their events, module acquisition and multiplexing.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 */

import { pluginDescriptorUrl } from "../🛂️descriptor-admission/🟦️.ts";
import { dependsOnToPluginDependencies, type PluginRegistryEntry } from "../🗂️plugin-registry/🟦️.ts";
import { type PluginCatalog } from "../📚️plugin-catalog/🟦️.ts";

// #region 🎠️Kernel
//#region 🔌️PluginSource
/** 🔌️ One entry of an availability stream: either the full set of currently-built plugins sent
 * once on connect (a reconnecting/late-connecting browser must not miss builds that already finished),
 * or a single plugin's rebuild landing. `rebuiltAt` is the artifact's build timestamp and doubles as
 * the cache-busting query value {@link PluginSource.moduleUrl} mints. */
export type PluginSourceEvent = { readonly kind: "snapshot"; readonly plugins: readonly { readonly pluginId: string; readonly rebuiltAt?: number }[] } | { readonly kind: "built"; readonly pluginId: string; readonly rebuiltAt: number };

/** 📈️ Bytes one module acquisition has verified so far, out of every byte it must verify. */
export type PluginModuleAcquisitionProgress = {
  readonly completedBytes: number;
  readonly totalBytes: number;
  /** 🔁️ Present while one file is fetched again after a transient answer: this attempt and the policy's maximum. */
  readonly retry?: { readonly attempt: number; readonly of: number };
};

/** 🛑️ How a caller bounds one module acquisition: its cancellation and its progress listener. */
export type PluginModuleAcquisition = { readonly signal: AbortSignal; readonly onProgress?: (progress: PluginModuleAcquisitionProgress) => void };

/** 📦️ One acquired module: the URL to load and the build stamp the loaded module is at least as new as.
 * The stamp is what later availability events are compared against, so the connect-time `snapshot` of
 * the very build a boot install just loaded is recognised as a replay instead of a newer build. */
export type PluginModuleAcquired = { readonly moduleUrl: string; readonly rebuiltAt: number | undefined };

/** 🚫️ A source that cannot serve `pluginId`'s module on this device — not listed, or its module is not
 * served here. A multiplexed source asks the next one; every other failure stays a failure. */
export class PluginModuleUnavailableError extends Error {
  override readonly name = "PluginModuleUnavailableError";
  readonly sourceId: string;
  readonly pluginId: string;
  constructor(sourceId: string, pluginId: string, detail: string) {
    super(`plugin module ${pluginId} is unavailable from source ${sourceId}: ${detail}`);
    this.sourceId = sourceId;
    this.pluginId = pluginId;
  }
}

/** 🔎️ Answers whether a locally served module is actually served here: its descriptor must answer. A
 * device that never staged a plugin answers 404 for it, and that is an unavailability, not a fault. */
async function requireServedPluginModule(sourceId: string, pluginId: string, moduleUrl: string, signal: AbortSignal): Promise<void> {
  const response = await fetch(pluginDescriptorUrl(moduleUrl), { method: "HEAD", signal, cache: "no-store" });
  if (!response.ok) throw new PluginModuleUnavailableError(sourceId, pluginId, `descriptor answered HTTP ${response.status}`);
}

/**
 * 🔌️ Where the shell's incremental plugin runtime (install/uninstall/reload — see the react
 * renderer's plugin panel) gets its catalog, its modules and its availability notifications from. The
 * dev, bundled and extension sources serve modules staged beside the shell; the hub source
 * (`🔌️plugin/📇️registry/🌎️hub-source`) installs trusted catalog modules on first use. The shell only
 * ever depends on this interface.
 */
export interface PluginSource {
  readonly id: string;
  /** Every plugin this source can currently install (built or not — the panel shows "available"
   * entries that haven't finished their first build yet). */
  list(): Promise<readonly PluginRegistryEntry[]>;
  /** Makes one install/reload of `pluginId` importable and answers its concrete module URL and stamp: a
   * local source checks the module is served and cache-busts it with `rebuiltAt` (its own boot version
   * before any `built` event, which then is the stamp); a remote source downloads and verifies it first,
   * reporting progress and honouring cancellation, and stamps it with the moment it was acquired.
   * {@link PluginModuleUnavailableError} means "not from here". */
  acquireModule(pluginId: string, rebuiltAt: number | undefined, acquisition: PluginModuleAcquisition): Promise<PluginModuleAcquired>;
  /** Subscribes to availability events; returns an unsubscribe function. Fires an immediate `snapshot`
   * on subscribe against sources that support it (the dev source's watch stream sends one on every fresh
   * open, including after a reconnect that could not resume). */
  subscribe(listener: (event: PluginSourceEvent) => void): () => void;
}

/** 📡️ One availability stream the source's owner opened for it: every raw event goes to `listener`; answers the
 * unsubscribe. The owner decides the transport (the `s` shell: one route of its page's stream channel, so no watch holds an
 * HTTP/1.1 connection of its origin); the kernel only reads events. */
export type PluginSourceWatch = (listener: (event: unknown) => void) => () => void;

const pluginSourceEventOf = (event: unknown): PluginSourceEvent | undefined => {
  if (typeof event !== "object" || event === null) return undefined;
  const row = event as { readonly kind?: unknown; readonly plugins?: unknown; readonly pluginId?: unknown; readonly rebuiltAt?: unknown };
  if (row.kind === "snapshot" && Array.isArray(row.plugins)) return event as PluginSourceEvent;
  if (row.kind === "built" && typeof row.pluginId === "string" && typeof row.rebuiltAt === "number") return event as PluginSourceEvent;
  return undefined;
};

/** 🔌️ `PluginSource` backed by an injected dev catalog and the availability stream its owner opens for it
 * ({@link PluginSourceWatch}); every subscription is its own stream, and an event that is not an availability event is
 * refused with one warning. */
export function createDevPluginSource(registry: readonly PluginRegistryEntry[], watch: PluginSourceWatch): PluginSource {
  const byId = new Map(registry.map((entry) => [entry.pluginId, entry] as const));
  const bootVersion = Date.now();
  return {
    id: "dev",
    async list() {
      return registry;
    },
    async acquireModule(pluginId, rebuiltAt, acquisition) {
      const entry = byId.get(pluginId);
      if (!entry) throw new PluginModuleUnavailableError("dev", pluginId, "no registry entry");
      await requireServedPluginModule("dev", pluginId, entry.moduleUrl, acquisition.signal);
      const separator = entry.moduleUrl.includes("?") ? "&" : "?";
      const stamp = rebuiltAt ?? bootVersion;
      return { moduleUrl: `${entry.moduleUrl}${separator}v=${stamp}`, rebuiltAt: stamp };
    },
    subscribe(listener) {
      return watch((event) => {
        const availability = pluginSourceEventOf(event);
        if (availability === undefined) console.warn("plugin source \"dev\" refused a watch event that is not an availability event", event);
        else listener(availability);
      });
    },
  };
}

/** 📦️ `PluginSource` for shipped static hosts: every registry entry is already materialized
 * beside the HTML bundle, so there is no dev-server SSE `/watch` endpoint to announce availability.
 * `subscribe` replays one immediate `snapshot` (same shape the dev endpoint sends on connect) so the
 * shell's install pump loads the full expanded registry — plugins and bundled flow extensions — without
 * waiting on `EventSource`. */
export function createBundledPluginSource(registry: readonly PluginRegistryEntry[]): PluginSource {
  const byId = new Map(registry.map((entry) => [entry.pluginId, entry] as const));
  const bootVersion = Date.now();
  // 🚫️ No per-plugin `rebuiltAt` on the connect-time snapshot — static hosts replay the same build the
  // primary boot already installed; naming a timestamp would hot-swap the session-owning plugin
  // (`pluginAvailabilityRouteV1`). First-time installs still cache-bust via `moduleUrl`'s `bootVersion`.
  const snapshot: PluginSourceEvent = { kind: "snapshot", plugins: registry.map((entry) => ({ pluginId: entry.pluginId, rebuiltAt: undefined })) };
  return {
    id: "bundled",
    async list() {
      return registry;
    },
    async acquireModule(pluginId, rebuiltAt, acquisition) {
      const entry = byId.get(pluginId);
      if (!entry) throw new PluginModuleUnavailableError("bundled", pluginId, "no registry entry");
      await requireServedPluginModule("bundled", pluginId, entry.moduleUrl, acquisition.signal);
      const separator = entry.moduleUrl.includes("?") ? "&" : "?";
      const stamp = rebuiltAt ?? bootVersion;
      return { moduleUrl: `${entry.moduleUrl}${separator}v=${stamp}`, rebuiltAt: stamp };
    },
    subscribe(listener) {
      queueMicrotask(() => listener(snapshot));
      return () => {};
    },
  };
}

type ExtensionSourceWireEvent =
  | { readonly kind: "snapshot"; readonly extensions: readonly { readonly extensionId: string; readonly installedAt: number }[] }
  | { readonly kind: "installed"; readonly extensionId: string; readonly installedAt: number }
  | { readonly kind: "uninstalled"; readonly extensionId: string };

/** 🔁️ Converts the extension store's install vocabulary into the runtime's plugin
 * availability vocabulary. Uninstall events have no availability equivalent and are ignored. */
export function extensionSourceEventToPluginSourceEvent(event: ExtensionSourceWireEvent): PluginSourceEvent | undefined {
  if (event.kind === "snapshot") {
    if (!Array.isArray(event.extensions)) throw new Error("snapshot extensions must be an array");
    return { kind: "snapshot", plugins: event.extensions.map((extension) => ({ pluginId: extension.extensionId, rebuiltAt: extension.installedAt })) };
  }
  if (event.kind === "installed") return { kind: "built", pluginId: event.extensionId, rebuiltAt: event.installedAt };
  if (event.kind === "uninstalled") return undefined;
  throw new Error("unknown extension source event kind");
}

/** 🧩️ Registry rows for flow extensions — shared by {@link createExtensionSource} and static
 * {@link createBundledPluginSource} hosts. */
export function extensionRegistryFromCatalog(catalog: PluginCatalog): readonly PluginRegistryEntry[] {
  return catalog.extensions.map((target) => ({
    pluginId: target.pluginId,
    moduleUrl: catalog.extensionModuleUrl(target.pluginId),
    contributes: target.contributes,
    consumes: target.consumes,
    dependencies: dependsOnToPluginDependencies(target.dependsOn),
  }));
}

/** 🧩️ `PluginSource` backed by an extension catalog and the install stream its owner opens for it
 * ({@link PluginSourceWatch}). Catalog rows come from the injected {@link PluginCatalog}'s `extensions`; runtime installs
 * add artifacts under each extension id without changing this list. `subscribe` normalizes the extension wire vocabulary
 * per listener. */
export function createExtensionSource(catalog: PluginCatalog, watch: PluginSourceWatch): PluginSource {
  const registry = extensionRegistryFromCatalog(catalog);
  const byId = new Map(registry.map((entry) => [entry.pluginId, entry] as const));
  return {
    id: "extensions",
    async list() {
      return registry;
    },
    async acquireModule(pluginId, rebuiltAt) {
      const entry = byId.get(pluginId);
      if (!entry) throw new PluginModuleUnavailableError("extensions", pluginId, "no registry entry");
      return { moduleUrl: rebuiltAt === undefined ? entry.moduleUrl : `${entry.moduleUrl}?v=${rebuiltAt}`, rebuiltAt };
    },
    subscribe(listener) {
      return watch((event) => {
        let normalized: PluginSourceEvent | undefined;
        try {
          normalized = extensionSourceEventToPluginSourceEvent(event as ExtensionSourceWireEvent);
        } catch (error) {
          console.warn("plugin source \"extensions\" refused a watch event that is not an install event", error);
          return;
        }
        if (normalized) listener(normalized);
      });
    },
  };
}

/** 🔌️ Merges multiple {@link PluginSource} implementations into one catalog the shell's
 * incremental runtime can treat as a single source. */
export function multiplexPluginSources(...sources: readonly PluginSource[]): PluginSource {
  if (sources.length === 0) throw new Error("multiplexPluginSources requires at least one source");
  if (sources.length === 1) { const source = sources[0]; if (!source) throw new Error("multiplexPluginSources requires an occupied source"); return source; }
  return {
    id: sources.map((source) => source.id).join("+"),
    async list() {
      const merged = new Map<string, PluginRegistryEntry>();
      for (const entries of await Promise.all(sources.map((source) => source.list()))) {
        for (const entry of entries) merged.set(entry.pluginId, entry);
      }
      return [...merged.values()];
    },
    async acquireModule(pluginId, rebuiltAt, acquisition) {
      const refusals: string[] = [];
      for (const source of sources) {
        try {
          return await source.acquireModule(pluginId, rebuiltAt, acquisition);
        } catch (error) {
          if (!(error instanceof PluginModuleUnavailableError)) throw error;
          refusals.push(error.message);
        }
      }
      throw new PluginModuleUnavailableError(sources.map((source) => source.id).join("+"), pluginId, refusals.join("; "));
    },
    subscribe(listener) {
      const unsubscribes = sources.map((source) => source.subscribe(listener));
      return () => {
        for (const unsubscribe of unsubscribes) unsubscribe();
      };
    },
  };
}
//#endregion 🔌️PluginSource
// #endregion 🎠️Kernel

//#region 🧪️SharedWatchStreamTests
if (import.meta.vitest) {
  const { registerTests6 } = await import("../🧪️tests/🧪️createturnoutcomebroadcast/🟦️.ts");
  await registerTests6(import.meta.vitest, { createBundledPluginSource, createDevPluginSource, createExtensionSource }, { directory: (await import("node:path")).dirname((await import("node:path")).dirname((await import("node:url")).fileURLToPath(import.meta.url))), url: import.meta.url });
}
//#endregion 🧪️SharedWatchStreamTests
