/** ⏳️ On-demand plugin activation: which installed plugin a request (node kind, tag, activation event, plugin id) activates, and the one activator both hosts drive with progress, cancellation and a bilingual state.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 *
 * 🔗️ Vocabulary: {@link ../🗂️plugin-registry/🟦️.ts | dependency activation} and {@link ../📚️plugin-catalog/🟦️.ts | activation events}; the wire shapes live in the OS engine's `🧬️schema/🧩️plugin-modules/🔣️.json`.
 */

import { nodeKindActivationEvent } from "../📚️plugin-catalog/🟦️.ts";
import { onDemandActivationClosure, type PluginRegistryEntry } from "../🗂️plugin-registry/🟦️.ts";

// #region 🎠️Kernel
//#region ⏳️OnDemandActivation
/** 🎯️ What asks for an inactive plugin: a flow node kind, a contributed tag, a declared activation event or the plugin itself. */
export type OnDemandRequest =
  | { readonly kind: "node-kind"; readonly nodeKind: string }
  | { readonly kind: "tag"; readonly tag: string }
  | { readonly kind: "event"; readonly event: string }
  | { readonly kind: "plugin"; readonly pluginId: string };

/** 🚦️ The phases one activation reports; `active`, `unavailable` and `cancelled` are terminal. */
export type OnDemandPhase = "resolving" | "loading" | "active" | "unavailable" | "cancelled";

/** 🧾️ Why a plugin could not be activated. */
export type OnDemandUnavailableReason = "not-installed" | "dependency-missing" | "load-failed";

/** 📣️ One plugin's activation state, as the hosts render it. `completed`/`total` are the closure position, not a byte share. */
export type OnDemandStatus = {
  readonly pluginId: string;
  readonly phase: OnDemandPhase;
  readonly completed: number;
  readonly total: number;
  readonly reason?: OnDemandUnavailableReason;
};

/** 🏁️ The settled answer of one {@link OnDemandActivator.request}: it never rejects, so a missing optional plugin cannot break its caller. */
export type OnDemandOutcome = {
  readonly requested: readonly string[];
  readonly active: readonly string[];
  readonly unavailable: readonly OnDemandStatus[];
  readonly cancelled: readonly string[];
};

/** 🔎️ The installed plugins `request` activates, ascending by plugin id like the artifact-kind owner resolution; an unknown request answers none. */
export function resolveOnDemandOwners(candidates: readonly PluginRegistryEntry[], request: OnDemandRequest): readonly string[] {
  const event = request.kind === "node-kind" ? nodeKindActivationEvent(request.nodeKind) : request.kind === "event" ? request.event : undefined;
  const matches = (entry: PluginRegistryEntry): boolean => {
    if (request.kind === "plugin") return entry.pluginId === request.pluginId;
    if (request.kind === "tag") return (entry.contributes ?? []).includes(request.tag);
    return event !== undefined && (entry.activationEvents ?? []).includes(event);
  };
  return candidates.filter(matches).map((entry) => entry.pluginId).sort((left, right) => left.localeCompare(right));
}

/** 🛂️ Admits a wire request (`🧬️schema/🧩️plugin-modules/🔣️.json` `$defs.onDemandRequest`) before it reaches an activator; refuses anything else with `on-demand.request-invalid`. */
export function admitOnDemandRequest(input: unknown): OnDemandRequest {
  const row = input !== null && typeof input === "object" && !Array.isArray(input) ? input as Record<string, unknown> : undefined;
  const text = (key: string, pattern: RegExp): string => {
    const value = row?.[key];
    if (typeof value !== "string" || value.length === 0 || value.length > 256 || !pattern.test(value) || Object.keys(row!).length !== 2) throw new Error(`on-demand.request-invalid: ${key}`);
    return value;
  };
  const printable = /^[^\u0000-\u001f\u007f]+$/u;
  switch (row?.kind) {
    case "node-kind": return { kind: "node-kind", nodeKind: text("nodeKind", /^[^\u0000-\u001f\u007f.][^\u0000-\u001f\u007f]*$/u) };
    case "tag": return { kind: "tag", tag: text("tag", printable) };
    case "event": return { kind: "event", event: text("event", /^on-[a-z-]+:[^\u0000-\u001f\u007f]+$/u) };
    case "plugin": return { kind: "plugin", pluginId: text("pluginId", printable) };
    default: throw new Error("on-demand.request-invalid: kind");
  }
}

export type OnDemandActivationContext = {
  readonly signal: AbortSignal;
  readonly progress: (completed: number, total: number) => void;
};

export type OnDemandActivatorOptions = {
  /** 🗂️ The deferred entries a request may activate. */
  readonly candidates: () => readonly PluginRegistryEntry[];
  /** 🗃️ Every installed entry, where a candidate's `required` dependencies are looked up; defaults to {@link candidates}. */
  readonly installed?: () => readonly PluginRegistryEntry[];
  readonly active: () => ReadonlySet<string>;
  /** 🚀️ Loads the module and starts the actor of ONE entry; rejects when it cannot. A host with no abortable load honours `signal` by settling early. */
  readonly activate: (entry: PluginRegistryEntry, context: OnDemandActivationContext) => Promise<void>;
  readonly onStatus?: (status: OnDemandStatus) => void;
};

export interface OnDemandActivator {
  request(request: OnDemandRequest, signal?: AbortSignal): Promise<OnDemandOutcome>;
  /** 🔎️ The deferred plugins `request` would start right now, ascending by id: the owners of the request that are not active yet. A host that mounts through its own install path (the wgpu shell) asks this, installs each owner itself, and so owns the progress band and the cancel. */
  owners(request: OnDemandRequest): readonly string[];
  cancel(pluginId?: string): number;
  statuses(): readonly OnDemandStatus[];
  subscribe(listener: (status: OnDemandStatus) => void): () => void;
}

const abortReason = (pluginId: string): Error => new Error(`on-demand.cancelled: ${pluginId}`);

/** 🧩️ Builds the activator over the injected seams. Requests for a plugin already loading join the running activation; a withdrawn request settles at once while a started load runs on into the host's own cache. */
export function createOnDemandActivator(options: OnDemandActivatorOptions): OnDemandActivator {
  const settled = new Set<string>();
  const running = new Map<string, { readonly promise: Promise<OnDemandStatus>; readonly abort: AbortController }>();
  const latest = new Map<string, OnDemandStatus>();
  const listeners = new Set<(status: OnDemandStatus) => void>();
  const publish = (status: OnDemandStatus): OnDemandStatus => {
    latest.set(status.pluginId, status);
    for (const listener of [...listeners, ...(options.onStatus ? [options.onStatus] : [])]) {
      try { listener(status); } catch { }
    }
    return status;
  };
  const activeIds = (): ReadonlySet<string> => new Set([...options.active(), ...settled]);
  const activateOne = (entry: PluginRegistryEntry, completed: number, total: number, callerSignal: AbortSignal | undefined): Promise<OnDemandStatus> => {
    const existing = running.get(entry.pluginId);
    if (existing) return existing.promise;
    const abort = new AbortController();
    const signal = callerSignal ? AbortSignal.any([abort.signal, callerSignal]) : abort.signal;
    publish({ pluginId: entry.pluginId, phase: "resolving", completed, total });
    const withdrawn = new Promise<never>((_, reject) => {
      if (signal.aborted) reject(abortReason(entry.pluginId));
      else signal.addEventListener("abort", () => reject(abortReason(entry.pluginId)), { once: true });
    });
    withdrawn.catch(() => { });
    publish({ pluginId: entry.pluginId, phase: "loading", completed, total });
    const loaded = options.activate(entry, { signal, progress: (done, count) => publish({ pluginId: entry.pluginId, phase: "loading", completed: done, total: count }) });
    const promise: Promise<OnDemandStatus> = Promise.race([loaded, withdrawn]).then(
      () => {
        settled.add(entry.pluginId);
        return publish({ pluginId: entry.pluginId, phase: "active", completed: total, total });
      },
      (error: unknown) => signal.aborted || (error instanceof Error && error.message.startsWith("on-demand.cancelled"))
        ? publish({ pluginId: entry.pluginId, phase: "cancelled", completed, total })
        : publish({ pluginId: entry.pluginId, phase: "unavailable", completed, total, reason: "load-failed" }),
    ).finally(() => { if (running.get(entry.pluginId)?.promise === promise) running.delete(entry.pluginId); });
    running.set(entry.pluginId, { promise, abort });
    return promise;
  };
  const activateOwner = async (pluginId: string, signal: AbortSignal | undefined): Promise<OnDemandStatus> => {
    if (activeIds().has(pluginId)) return publish({ pluginId, phase: "active", completed: 1, total: 1 });
    const closure = onDemandActivationClosure((options.installed ?? options.candidates)(), activeIds(), pluginId);
    if (!closure) return publish({ pluginId, phase: "unavailable", completed: 0, total: 1, reason: options.candidates().some((entry) => entry.pluginId === pluginId) ? "dependency-missing" : "not-installed" });
    let last: OnDemandStatus = { pluginId, phase: "active", completed: 0, total: closure.length };
    for (const [index, entry] of closure.entries()) {
      last = await activateOne(entry, index, closure.length, signal);
      if (last.phase !== "active") return entry.pluginId === pluginId ? last : publish({ pluginId, phase: last.phase, completed: index, total: closure.length, ...(last.reason ? { reason: last.reason } : {}) });
    }
    return last;
  };
  return {
    async request(request, signal) {
      const owners = resolveOnDemandOwners(options.candidates(), request);
      const statuses = await Promise.all(owners.map((owner) => activateOwner(owner, signal)));
      const unknown: OnDemandStatus[] = request.kind === "plugin" && owners.length === 0 ? [publish({ pluginId: request.pluginId, phase: "unavailable", completed: 0, total: 1, reason: "not-installed" })] : [];
      return {
        requested: owners,
        active: statuses.filter((status) => status.phase === "active").map((status) => status.pluginId),
        unavailable: [...statuses.filter((status) => status.phase === "unavailable"), ...unknown],
        cancelled: statuses.filter((status) => status.phase === "cancelled").map((status) => status.pluginId),
      };
    },
    owners: (request) => resolveOnDemandOwners(options.candidates(), request).filter((pluginId) => !activeIds().has(pluginId)),
    cancel(pluginId) {
      const targets = pluginId === undefined ? [...running.values()] : running.has(pluginId) ? [running.get(pluginId)!] : [];
      for (const target of targets) target.abort.abort();
      return targets.length;
    },
    statuses: () => [...latest.values()],
    subscribe(listener) {
      listeners.add(listener);
      return () => { listeners.delete(listener); };
    },
  };
}
//#endregion ⏳️OnDemandActivation

//#region 🏷️OnDemandLabels
/** 📢️ EN/DE state texts, `{name}` the plugin id; there is no default language, an unknown locale tells English. */
export const ON_DEMAND_STATUS_LABELS = {
  resolving: { en: "Preparing extension {name}…", de: "Erweiterung {name} wird vorbereitet…" },
  loading: { en: "Loading extension {name}…", de: "Erweiterung {name} wird geladen…" },
  active: { en: "Extension {name} is ready.", de: "Erweiterung {name} ist bereit." },
  cancelled: { en: "Loading extension {name} was cancelled.", de: "Das Laden der Erweiterung {name} wurde abgebrochen." },
  unavailable: { en: "Extension {name} is not available. The rest keeps working.", de: "Erweiterung {name} ist nicht verfügbar. Alles andere funktioniert weiter." },
} as const;

/** 🗣️ The text of `status` in `locale` (`en`, `de` or a region tag of them). */
export function onDemandStatusLabel(status: Pick<OnDemandStatus, "pluginId" | "phase">, locale: string): string {
  const row = ON_DEMAND_STATUS_LABELS[status.phase];
  return (locale.toLowerCase().startsWith("de") ? row.de : row.en).replace("{name}", status.pluginId);
}

/** ♿️ How a host announces `status` to assistive technology: a polite `status` while it runs, an `alert` when it is unavailable, and `busy` only while it runs. */
export function onDemandStatusAnnouncement(status: Pick<OnDemandStatus, "phase">): { readonly role: "status" | "alert"; readonly ariaLive: "polite" | "assertive"; readonly busy: boolean } {
  return status.phase === "unavailable" ? { role: "alert", ariaLive: "assertive", busy: false } : { role: "status", ariaLive: "polite", busy: status.phase === "resolving" || status.phase === "loading" };
}
//#endregion 🏷️OnDemandLabels
// #endregion 🎠️Kernel
