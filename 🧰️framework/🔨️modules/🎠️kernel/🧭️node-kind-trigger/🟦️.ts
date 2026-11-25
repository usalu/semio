/** 🧭️ The node-kind trigger: a flow document or example is scanned for its node kinds, each namespace activates its deferred extension through the shared {@link OnDemandActivator}, and only then is the document handed to the guest.
 *
 * 🧱️ Child module of `🎠️kernel`; importers import this module directly, and the kernel package entry {@link ../📦️packages/🟦️typescript/🟦️.ts} lists its public names.
 *
 * 🔗️ Vocabulary: {@link ../📚️plugin-catalog/🟦️.ts | `on-node-kind:<namespace>` activation events} and {@link ../⏳️on-demand-activation/🟦️.ts | the activator}. Both hosts call {@link openDocumentAfterActivation}; the guest asks for the same activation with the wire request `{kind:"node-kind", nodeKind}` when it meets a kind the document scan could not see.
 */

import { nodeKindActivationEvent } from "../📚️plugin-catalog/🟦️.ts";
import { admitOnDemandRequest, onDemandStatusAnnouncement, type OnDemandActivator, type OnDemandRequest, type OnDemandUnavailableReason } from "../⏳️on-demand-activation/🟦️.ts";
import type { PluginRegistryEntry } from "../🗂️plugin-registry/🟦️.ts";

// #region 🎠️Kernel
//#region 🔎️NodeKindScan
/** 🔎️ A node kind is the value of a `neuron-kind` attribute (DSL `neuron-kind=brep.bool.cut`) or member (JSON `"neuronKind":"brep.bool.cut"`, also `neuron-kind` and `neuron_kind`). */
const NODE_KIND_ATTRIBUTE = /neuron[-_]?kind"?\s*[=:]\s*"?([^\s"\],}]+)/giu;

/** 🔎️ Every distinct node kind a flow document names, in order of first appearance; text of any other shape names none. */
export function scanNodeKinds(text: string): readonly string[] {
  const kinds = new Set<string>();
  for (const match of text.matchAll(NODE_KIND_ATTRIBUTE)) if (nodeKindActivationEvent(match[1]!) !== undefined) kinds.add(match[1]!);
  return [...kinds];
}

/** 🧩️ The distinct `on-node-kind` requests of `nodeKinds`: one per leading namespace, ascending, carrying the first kind that named it. */
export function nodeKindRequests(nodeKinds: readonly string[]): readonly (OnDemandRequest & { readonly kind: "node-kind" })[] {
  const byEvent = new Map<string, string>();
  for (const nodeKind of nodeKinds) {
    const event = nodeKindActivationEvent(nodeKind);
    if (event !== undefined && !byEvent.has(event)) byEvent.set(event, nodeKind);
  }
  return [...byEvent.entries()].sort(([left], [right]) => left.localeCompare(right)).map(([, nodeKind]) => ({ kind: "node-kind", nodeKind }));
}

/** 🗂️ The deferred plugins that serve `nodeKinds`, ascending by id: the pure mapping `node kind -> namespace -> activation event -> owner`. */
export function documentExtensionOwners(nodeKinds: readonly string[], candidates: readonly PluginRegistryEntry[]): readonly string[] {
  const events = new Set(nodeKindRequests(nodeKinds).map((request) => nodeKindActivationEvent(request.nodeKind)));
  return candidates.filter((entry) => (entry.activationEvents ?? []).some((event) => events.has(event))).map((entry) => entry.pluginId).sort((left, right) => left.localeCompare(right));
}
//#endregion 🔎️NodeKindScan

//#region 🚦️DocumentActivation
/** 🚫️ A node kind whose extension could not be activated; the document opens anyway and the node is shown unavailable. */
export type UnavailableNodeKind = { readonly nodeKind: string; readonly pluginId: string; readonly reason?: OnDemandUnavailableReason };

/** 🏁️ The settled answer of activating a document's extensions: `ready` (all active, or nothing deferred), `partial` (some unavailable) or `cancelled`. */
export type DocumentActivationReport = {
  readonly state: "ready" | "partial" | "cancelled";
  readonly nodeKinds: readonly string[];
  readonly active: readonly string[];
  readonly unavailable: readonly UnavailableNodeKind[];
  readonly cancelled: readonly string[];
  /** 🪶️ Kinds no deferred plugin claims; an active plugin serves them or the guest reports them unknown. */
  readonly unowned: readonly string[];
};

export type DocumentActivationOptions = {
  readonly nodeKinds: readonly string[];
  readonly activator: Pick<OnDemandActivator, "request">;
  readonly signal?: AbortSignal;
  readonly progress?: (completed: number, total: number) => void;
};

/** 🚦️ Activates the extension of every namespace `nodeKinds` names, concurrently, with progress as namespaces settle. Never rejects; a caller abort answers `cancelled` and withdraws the requests it started. */
export async function activateDocumentNodeKinds(options: DocumentActivationOptions): Promise<DocumentActivationReport> {
  const requests = nodeKindRequests(options.nodeKinds);
  const empty = { nodeKinds: options.nodeKinds, active: [], unavailable: [], cancelled: [], unowned: [] } as const;
  if (options.signal?.aborted) return { ...empty, state: "cancelled" };
  let settled = 0;
  options.progress?.(0, requests.length);
  const outcomes = await Promise.all(requests.map(async (request) => {
    const outcome = await options.activator.request(request, options.signal);
    options.progress?.(++settled, requests.length);
    return { request, outcome };
  }));
  const inNamespace = (request: { readonly nodeKind: string }) => options.nodeKinds.filter((kind) => nodeKindActivationEvent(kind) === nodeKindActivationEvent(request.nodeKind));
  const unavailable = outcomes.flatMap(({ request, outcome }) => outcome.unavailable.flatMap((status) => inNamespace(request).map((nodeKind) => ({ nodeKind, pluginId: status.pluginId, ...(status.reason ? { reason: status.reason } : {}) }))));
  const cancelled = outcomes.flatMap(({ outcome }) => outcome.cancelled);
  const unowned = outcomes.filter(({ outcome }) => outcome.requested.length === 0).flatMap(({ request }) => inNamespace(request));
  const active = [...new Set(outcomes.flatMap(({ outcome }) => outcome.active))].sort((left, right) => left.localeCompare(right));
  return { state: cancelled.length > 0 || options.signal?.aborted ? "cancelled" : unavailable.length > 0 ? "partial" : "ready", nodeKinds: options.nodeKinds, active, unavailable, cancelled, unowned };
}
//#endregion 🚦️DocumentActivation

//#region 📨️GuestRequest
/** 🧭️ The topic a guest publishes (`Effect::PublishEvent`) to ask its host to activate a deferred extension; `$defs.onDemandRequestTopic` of the OS engine's `🧬️schema/🧩️plugin-modules/🔣️.json`, and the Rust `ON_DEMAND_REQUEST_TOPIC` of the flow host. */
export const ON_DEMAND_REQUEST_TOPIC = "os.on-demand.request";

/** 📨️ The activation request a published guest event carries: none for another topic, the admitted request for this one, `on-demand.request-invalid` for a payload that is not UTF-8 JSON of one. */
export function guestActivationRequest(event: { readonly topic: string; readonly payload: ArrayLike<number> }): OnDemandRequest | undefined {
  if (event.topic !== ON_DEMAND_REQUEST_TOPIC) return undefined;
  try { return admitOnDemandRequest(JSON.parse(new TextDecoder("utf-8", { fatal: true }).decode(Uint8Array.from(event.payload))) as unknown); }
  catch { throw new Error("on-demand.request-invalid: payload"); }
}
//#endregion 📨️GuestRequest

//#region 🏷️DocumentLabels
/** 📢️ EN/DE texts; `{kinds}` and `{plugins}` are comma lists, `{completed}` and `{total}` the namespace position. There is no default language, an unknown locale tells English. */
export const DOCUMENT_ACTIVATION_LABELS = {
  preparing: { en: "Preparing extensions for this document ({completed} of {total})…", de: "Erweiterungen für dieses Dokument werden vorbereitet ({completed} von {total})…" },
  unavailable: { en: "The document opened with unavailable nodes: {kinds}. Extension {plugins} could not be loaded. Everything else works.", de: "Das Dokument wurde mit nicht verfügbaren Knoten geöffnet: {kinds}. Erweiterung {plugins} konnte nicht geladen werden. Alles andere funktioniert." },
  cancelled: { en: "Opening the document was cancelled while loading extension {plugins}.", de: "Das Öffnen des Dokuments wurde beim Laden der Erweiterung {plugins} abgebrochen." },
  openingExample: { en: "Opening example {example}… Large examples can take a few minutes.", de: "Beispiel {example} wird geöffnet… Große Beispiele können einige Minuten dauern." },
  exampleRefused: { en: "Example {example} could not be opened: {reason}", de: "Beispiel {example} konnte nicht geöffnet werden: {reason}" },
} as const;

/** 🗣️ The text of `key` in `locale` (`en`, `de` or a region tag of them) with its placeholders filled. */
export function documentActivationLabel(key: keyof typeof DOCUMENT_ACTIVATION_LABELS, locale: string, values: Readonly<Record<string, string | number>> = {}): string {
  const row = DOCUMENT_ACTIVATION_LABELS[key];
  const template: string = locale.toLowerCase().startsWith("de") ? row.de : row.en;
  return Object.entries(values).reduce((text, [name, value]) => text.replaceAll(`{${name}}`, String(value)), template);
}

/** 📣️ What a host tells the person about a settled report, with the ARIA announcement; none when everything is ready. */
export function documentActivationNotice(report: DocumentActivationReport, locale: string): { readonly text: string; readonly announcement: ReturnType<typeof onDemandStatusAnnouncement> } | undefined {
  const list = (items: readonly string[]) => [...new Set(items)].join(", ");
  if (report.state === "cancelled") return { text: documentActivationLabel("cancelled", locale, { plugins: list(report.cancelled) || "…" }), announcement: onDemandStatusAnnouncement({ phase: "cancelled" }) };
  if (report.state === "partial") return { text: documentActivationLabel("unavailable", locale, { kinds: list(report.unavailable.map((row) => row.nodeKind)), plugins: list(report.unavailable.map((row) => row.pluginId)) }), announcement: onDemandStatusAnnouncement({ phase: "unavailable" }) };
  return undefined;
}
//#endregion 🏷️DocumentLabels

//#region 🚪️DocumentHandoff
/** 📨️ What the handoff learns: the report, so the guest can mark the unavailable kinds instead of failing on them. */
export type DocumentHandoffContext = { readonly report: DocumentActivationReport };

export type OpenDocumentOptions<T> = {
  readonly text: string;
  readonly activator: Pick<OnDemandActivator, "request">;
  readonly handoff: (context: DocumentHandoffContext) => Promise<T> | T;
  readonly locale: string;
  readonly signal?: AbortSignal;
  readonly progress?: (completed: number, total: number, label: string) => void;
  readonly notify?: (notice: NonNullable<ReturnType<typeof documentActivationNotice>>) => void;
};

export type OpenDocumentResult<T> =
  | { readonly state: "opened"; readonly report: DocumentActivationReport; readonly result: T }
  | { readonly state: "cancelled"; readonly report: DocumentActivationReport };

/** 🚪️ Scans `text`, activates the extensions of its node kinds, then calls `handoff` exactly once, after every activation settled. An unavailable extension still hands the document off with the report; a cancellation hands nothing off. */
export async function openDocumentAfterActivation<T>(options: OpenDocumentOptions<T>): Promise<OpenDocumentResult<T>> {
  const report = await activateDocumentNodeKinds({
    nodeKinds: scanNodeKinds(options.text), activator: options.activator, ...(options.signal ? { signal: options.signal } : {}),
    progress: (completed, total) => options.progress?.(completed, total, documentActivationLabel("preparing", options.locale, { completed, total })),
  });
  const notice = documentActivationNotice(report, options.locale);
  if (notice) options.notify?.(notice);
  if (report.state === "cancelled") return { state: "cancelled", report };
  return { state: "opened", report, result: await options.handoff({ report }) };
}

/** 📚️ The authored body of example `exampleId` among a manifest's examples: the inline document an example carries before any guest reads it, which is the earliest host-visible form of a bundled example. */
export function exampleDocumentText(examples: readonly { readonly id: string; readonly documentJson?: string }[], exampleId: string): string | undefined {
  return examples.find((example) => example.id === exampleId)?.documentJson;
}

/** 📚️ {@link openDocumentAfterActivation} for a bundled example picked by id; an example without a host-visible body has no kinds to scan and is handed off at once, its unknown kinds being the guest's to request. */
export function openExampleAfterActivation<T>(options: Omit<OpenDocumentOptions<T>, "text"> & { readonly examples: readonly { readonly id: string; readonly documentJson?: string }[]; readonly exampleId: string }): Promise<OpenDocumentResult<T>> {
  return openDocumentAfterActivation({ ...options, text: exampleDocumentText(options.examples, options.exampleId) ?? "" });
}
//#endregion 🚪️DocumentHandoff
// #endregion 🎠️Kernel
