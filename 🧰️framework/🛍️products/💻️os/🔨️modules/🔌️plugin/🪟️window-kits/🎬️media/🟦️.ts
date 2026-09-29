// #region 🎬️MediaWindowKit
/// <reference types="vitest/importMeta" />
import type { AccessibilitySpec, BuiltNode, Component, LayoutSpec, StyleSpec } from "@semio-tech/framework";

/** 🎬️ Stable media window and host-extension identities. */
export const MEDIA_WINDOW_KIND_ID = "framework.window.media";
export const MEDIA_TRANSPORT_EXTENSION_ID = "framework.media.transport@1";
export const MEDIA_PLAYBACK_OUTPUT_PORT = "playback:out";

export type MediaKind = "audio" | "video";
export type MediaLocale = "en" | "de";
export type MediaTransportCapabilityStatus = "ready" | "loading" | "unsupported";

export type MediaTransportLabels = {
  readonly play: string;
  readonly pause: string;
  readonly seek: string;
  readonly position: string;
  readonly duration: string;
  readonly selectionStart: string;
  readonly selectionEnd: string;
  readonly loading: string;
  readonly progress: string;
  readonly cancel: string;
  readonly unsupported: string;
  readonly unknownDuration: string;
  readonly audio: string;
  readonly video: string;
};

export type MediaTransportResource = {
  readonly kind: "artifact-media-export";
  readonly controllerId: string;
  readonly appInstanceId: number;
  readonly parentDocumentId: string;
  readonly outputPort: typeof MEDIA_PLAYBACK_OUTPUT_PORT;
  readonly revision: string;
  readonly generation: string;
};

export type MediaTransportCapability = {
  readonly status: MediaTransportCapabilityStatus;
  readonly reason: string | null;
};

export type MediaTransportProps = {
  readonly schemaVersion: 1;
  readonly kind: MediaKind;
  readonly mediaType: string;
  readonly revision: string;
  readonly durationMs: number | null;
  readonly positionMs: number;
  readonly selectionStartMs: number | null;
  readonly selectionEndMs: number | null;
  readonly locale: MediaLocale;
  readonly labels: MediaTransportLabels;
  readonly resource: MediaTransportResource | null;
  readonly capability: MediaTransportCapability;
  readonly hostContentHeight: number;
};

export type MediaView = Omit<MediaTransportProps, "schemaVersion" | "labels">;

const LABEL_KEYS = ["play", "pause", "seek", "position", "duration", "selectionStart", "selectionEnd", "loading", "progress", "cancel", "unsupported", "unknownDuration", "audio", "video"] as const;
const PROP_KEYS = ["schemaVersion", "kind", "mediaType", "revision", "durationMs", "positionMs", "selectionStartMs", "selectionEndMs", "locale", "labels", "resource", "capability", "hostContentHeight"] as const;
const RESOURCE_KEYS = ["kind", "controllerId", "appInstanceId", "parentDocumentId", "outputPort", "revision", "generation"] as const;
const CAPABILITY_KEYS = ["status", "reason"] as const;
const MAX_U64 = 18_446_744_073_709_551_615n;

const DEFAULT_LAYOUT: LayoutSpec = { kind: "leaf", width: "hug", height: "hug" };
const DEFAULT_STYLE: StyleSpec = { variant: "plain", size: "md", density: "standard", tone: "neutral", emphasis: "regular" };
const DEFAULT_ACCESSIBILITY: AccessibilitySpec = { label: null, description: null, live: "off", shortcut: null, hidden: false };

function record(value: unknown): Record<string, unknown> | null {
  return typeof value === "object" && value !== null && !Array.isArray(value) ? value as Record<string, unknown> : null;
}

function exactKeys(value: Record<string, unknown>, keys: readonly string[]): boolean {
  const actual = Object.keys(value);
  return actual.length === keys.length && keys.every((key) => Object.hasOwn(value, key));
}

function decimalRevision(value: unknown): value is string {
  return typeof value === "string" && /^(0|[1-9][0-9]{0,19})$/.test(value) && BigInt(value) <= MAX_U64;
}

function integer(value: unknown, minimum: number): value is number {
  return typeof value === "number" && Number.isSafeInteger(value) && value >= minimum;
}

function parseLabels(value: unknown): MediaTransportLabels | null {
  const labels = record(value);
  if (!labels || !exactKeys(labels, LABEL_KEYS) || !LABEL_KEYS.every((key) => typeof labels[key] === "string")) return null;
  return labels as MediaTransportLabels;
}

function parseResource(value: unknown, revision: string): MediaTransportResource | null | undefined {
  if (value === null) return null;
  const resource = record(value);
  if (!resource || !exactKeys(resource, RESOURCE_KEYS)) return undefined;
  if (resource.kind !== "artifact-media-export" || typeof resource.controllerId !== "string" || resource.controllerId.length === 0 || [...resource.controllerId].length > 256 || !integer(resource.appInstanceId, 0) || resource.appInstanceId > 4_294_967_295 || typeof resource.parentDocumentId !== "string" || resource.parentDocumentId.length === 0 || [...resource.parentDocumentId].length > 512 || resource.outputPort !== MEDIA_PLAYBACK_OUTPUT_PORT || resource.revision !== revision || !decimalRevision(resource.generation)) return undefined;
  return resource as MediaTransportResource;
}

/** 🧬️ Parses the language-agnostic host contract without accepting unknown or lossy fields. */
export function parseMediaTransportProps(value: unknown): MediaTransportProps {
  const props = record(value);
  if (!props || !exactKeys(props, PROP_KEYS)) throw new TypeError("media-transport.props");
  const revision = props.revision;
  if (props.schemaVersion !== 1 || (props.kind !== "audio" && props.kind !== "video") || typeof props.mediaType !== "string" || props.mediaType.length === 0 || [...props.mediaType].length > 128 || !decimalRevision(revision) || (props.locale !== "en" && props.locale !== "de")) throw new TypeError("media-transport.identity");
  const duration = props.durationMs;
  const position = props.positionMs;
  const start = props.selectionStartMs;
  const end = props.selectionEndMs;
  if (duration !== null && !integer(duration, 1)) throw new TypeError("media-transport.duration");
  if (!integer(position, 0) || (duration !== null && position > duration)) throw new TypeError("media-transport.position");
  if ((start === null) !== (end === null) || (start !== null && (!integer(start, 0) || !integer(end, 1) || start >= end || duration === null || end > duration))) throw new TypeError("media-transport.selection");
  if (duration === null && (position !== 0 || start !== null)) throw new TypeError("media-transport.unknown-duration");
  if (!parseLabels(props.labels)) throw new TypeError("media-transport.labels");
  const resource = parseResource(props.resource, revision);
  if (resource === undefined) throw new TypeError("media-transport.resource");
  const capability = record(props.capability);
  if (!capability || !exactKeys(capability, CAPABILITY_KEYS) || (capability.status !== "ready" && capability.status !== "loading" && capability.status !== "unsupported") || (capability.reason !== null && typeof capability.reason !== "string") || (typeof capability.reason === "string" && [...capability.reason].length > 512) || (capability.status === "ready" && resource === null)) throw new TypeError("media-transport.capability");
  if (typeof props.hostContentHeight !== "number" || !Number.isFinite(props.hostContentHeight) || props.hostContentHeight < 0 || props.hostContentHeight > 4096) throw new TypeError("media-transport.host-height");
  return props as MediaTransportProps;
}

/** 🌍️ Returns the complete host-owned accessible label set for one explicit locale. */
export function mediaTransportLabels(locale: MediaLocale): MediaTransportLabels {
  return locale === "de"
    ? { play: "Wiedergabe", pause: "Pause", seek: "Position", position: "Position", duration: "Dauer", selectionStart: "Auswahlbeginn", selectionEnd: "Auswahlende", loading: "Medium wird geladen…", progress: "Ladefortschritt", cancel: "Abbrechen", unsupported: "Dieses Medium kann auf diesem Gerät nicht wiedergegeben werden.", unknownDuration: "Dauer unbekannt", audio: "Audio", video: "Video" }
    : { play: "Play", pause: "Pause", seek: "Seek", position: "Position", duration: "Duration", selectionStart: "Selection start", selectionEnd: "Selection end", loading: "Loading media…", progress: "Loading progress", cancel: "Cancel", unsupported: "This media cannot be played on this device.", unknownDuration: "Unknown duration", audio: "Audio", video: "Video" };
}

function leafNode(key: string, component: Component): BuiltNode {
  return { key, component, layout: DEFAULT_LAYOUT, style: DEFAULT_STYLE, activity: "idle", disabled: false, accessibility: DEFAULT_ACCESSIBILITY, bindings: [], menu: null, children: [] };
}

/** 🎬️ Projects bounded media timing and a revision-bound export resource to the host transport. */
export function renderMedia(view: MediaView): BuiltNode {
  const durationMs = view.durationMs === null ? null : Math.max(1, Math.trunc(view.durationMs));
  const positionMs = durationMs === null ? 0 : Math.min(durationMs, Math.max(0, Math.trunc(view.positionMs)));
  const selectionStartMs = durationMs !== null && view.selectionStartMs !== null && view.selectionEndMs !== null ? Math.min(durationMs - 1, Math.max(0, Math.trunc(view.selectionStartMs))) : null;
  const selectionEndMs = selectionStartMs !== null && view.selectionEndMs !== null ? Math.min(durationMs!, Math.max(selectionStartMs + 1, Math.trunc(view.selectionEndMs))) : null;
  const props = parseMediaTransportProps({ schemaVersion: 1, ...view, durationMs, positionMs, selectionStartMs, selectionEndMs, labels: mediaTransportLabels(view.locale) });
  return leafNode(MEDIA_WINDOW_KIND_ID, { type: "extension", extension: MEDIA_TRANSPORT_EXTENSION_ID, props });
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🧪️rendermedia/🟦️.ts");
  await registerTests1(import.meta.vitest, { parseMediaTransportProps, renderMedia }, { directory: import.meta.dir, url: import.meta.url });
}
// #endregion 🎬️MediaWindowKit
