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
  readonly cancelled: string;
  readonly cutSelection: string;
  readonly hourOne: string;
  readonly hourOther: string;
  readonly minuteOne: string;
  readonly minuteOther: string;
  readonly secondOne: string;
  readonly secondOther: string;
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

/** ✂️ The editor-declared range cut: which action a selection dispatches, with its canonical revision argument, and how many whole units the media holds per second (a rational) out of how many in total. */
export type MediaTransportTrim = {
  readonly actionId: string;
  readonly revision: string;
  readonly unit: "frame" | "sample";
  readonly unitsPerSecondNumerator: number;
  readonly unitsPerSecondDenominator: number;
  readonly totalUnits: number;
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
  readonly trim: MediaTransportTrim | null;
};

export type MediaView = Omit<MediaTransportProps, "schemaVersion" | "labels">;

const LABEL_KEYS = ["play", "pause", "seek", "position", "duration", "selectionStart", "selectionEnd", "loading", "progress", "cancel", "unsupported", "unknownDuration", "cancelled", "cutSelection", "hourOne", "hourOther", "minuteOne", "minuteOther", "secondOne", "secondOther", "audio", "video"] as const;
const PROP_KEYS = ["schemaVersion", "kind", "mediaType", "revision", "durationMs", "positionMs", "selectionStartMs", "selectionEndMs", "locale", "labels", "resource", "capability", "hostContentHeight", "trim"] as const;
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

const TRIM_KEYS = ["actionId", "revision", "unit", "unitsPerSecondNumerator", "unitsPerSecondDenominator", "totalUnits"] as const;

function parseTrim(value: unknown): MediaTransportTrim | null | undefined {
  if (value === null) return null;
  const trim = record(value);
  if (!trim || !exactKeys(trim, TRIM_KEYS)) return undefined;
  const bounded = (text: unknown): text is string => typeof text === "string" && text.length > 0 && [...text].length <= 128;
  const units = (number: unknown): number is number => integer(number, 1) && number <= Number.MAX_SAFE_INTEGER;
  if (!bounded(trim.actionId) || !bounded(trim.revision) || (trim.unit !== "frame" && trim.unit !== "sample") || !units(trim.unitsPerSecondNumerator) || !units(trim.unitsPerSecondDenominator) || !units(trim.totalUnits)) return undefined;
  return trim as MediaTransportTrim;
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
  if (parseTrim(props.trim) === undefined) throw new TypeError("media-transport.trim");
  return props as MediaTransportProps;
}

/** 🌍️ Returns the complete host-owned accessible label set for one explicit locale. */
export function mediaTransportLabels(locale: MediaLocale): MediaTransportLabels {
  return locale === "de"
    ? { play: "Wiedergabe", pause: "Pause", seek: "Position", position: "Position", duration: "Dauer", selectionStart: "Auswahlbeginn", selectionEnd: "Auswahlende", loading: "Medium wird geladen…", progress: "Ladefortschritt", cancel: "Abbrechen", unsupported: "Dieses Medium kann auf diesem Gerät nicht wiedergegeben werden.", unknownDuration: "Dauer unbekannt", cancelled: "Abgebrochen", cutSelection: "Auswahl ausschneiden", hourOne: "Stunde", hourOther: "Stunden", minuteOne: "Minute", minuteOther: "Minuten", secondOne: "Sekunde", secondOther: "Sekunden", audio: "Audio", video: "Video" }
    : { play: "Play", pause: "Pause", seek: "Seek", position: "Position", duration: "Duration", selectionStart: "Selection start", selectionEnd: "Selection end", loading: "Loading media…", progress: "Loading progress", cancel: "Cancel", unsupported: "This media cannot be played on this device.", unknownDuration: "Unknown duration", cancelled: "Cancelled", cutSelection: "Cut selection", hourOne: "hour", hourOther: "hours", minuteOne: "minute", minuteOther: "minutes", secondOne: "second", secondOther: "seconds", audio: "Audio", video: "Video" };
}

/** ⏱️ Formats a media time as `h:mm:ss.mmm` (hours only when present) with the locale's decimal mark, never raw milliseconds. */
export function formatMediaTime(milliseconds: number, locale: MediaLocale): string {
  const total = Math.max(0, Math.trunc(milliseconds));
  const hours = Math.floor(total / 3_600_000);
  const minutes = Math.floor(total / 60_000) % 60;
  const seconds = Math.floor(total / 1000) % 60;
  const pad = (value: number, width: number): string => String(value).padStart(width, "0");
  const mark = locale === "de" ? "," : ".";
  return `${hours > 0 ? `${hours}:${pad(minutes, 2)}` : String(minutes)}:${pad(seconds, 2)}${mark}${pad(total % 1000, 3)}`;
}

/** 🗣️ Spells a media time out for assistive technology (`aria-valuetext`) from the label table, choosing each unit word with the locale's plural rules. */
export function mediaTimeValueText(milliseconds: number, locale: MediaLocale, labels: MediaTransportLabels): string {
  const total = Math.max(0, Math.trunc(milliseconds));
  const hours = Math.floor(total / 3_600_000);
  const minutes = Math.floor(total / 60_000) % 60;
  const plural = new Intl.PluralRules(locale);
  const unit = (value: number | string, one: string, other: string): string => `${value} ${plural.select(Number(value)) === "one" ? one : other}`;
  const seconds = `${Math.floor(total / 1000) % 60}${locale === "de" ? "," : "."}${String(total % 1000).padStart(3, "0")}`;
  return [...(hours > 0 ? [unit(hours, labels.hourOne, labels.hourOther)] : []), ...(minutes > 0 || hours > 0 ? [unit(minutes, labels.minuteOne, labels.minuteOther)] : []), `${seconds} ${Number(seconds.replace(",", ".")) === 1 ? labels.secondOne : labels.secondOther}`].join(" ");
}

/** ✂️ Maps a selected time range to the cut-action value `start-end` in whole units: the start floors, the end ceils, both clamped to the units the media holds; `null` when the range holds no whole unit. */
export function trimValueForSelection(trim: MediaTransportTrim, startMs: number, endMs: number): string | null {
  const units = (milliseconds: number, round: (value: number) => number): number => Math.min(trim.totalUnits, Math.max(0, round((Math.max(0, milliseconds) * trim.unitsPerSecondNumerator) / (trim.unitsPerSecondDenominator * 1000))));
  const start = units(startMs, Math.floor);
  const end = units(endMs, Math.ceil);
  return start < end ? `${start}-${end}` : null;
}

/** ⌨️ The transport commands the keyboard can raise. */
export type MediaKeyCommand = Readonly<{ kind: "toggle" }> | Readonly<{ kind: "seek"; deltaMs: number }> | Readonly<{ kind: "seek-to"; positionMs: number }>;
export type MediaKeyInput = Readonly<{ key: string; shiftKey: boolean; ctrlKey: boolean; altKey: boolean; metaKey: boolean; targetTag: string; targetType: string | null }>;

/** ⌨️ Space or K toggles playback, the arrows seek 5 s (1 s with Shift), Home and End jump; controls that already own the key keep it. Scoped by the caller to the media window. */
export function mediaKeyCommand(input: MediaKeyInput, durationMs: number | null): MediaKeyCommand | null {
  if (input.ctrlKey || input.altKey || input.metaKey) return null;
  const tag = input.targetTag.toUpperCase();
  const ownsArrows = tag === "TEXTAREA" || tag === "SELECT" || (tag === "INPUT" && input.targetType !== null);
  const ownsActivation = tag === "BUTTON" || tag === "A" || tag === "TEXTAREA" || tag === "SELECT" || (tag === "INPUT" && input.targetType !== "range");
  if ((input.key === " " || input.key === "Spacebar" || input.key === "k" || input.key === "K") && !ownsActivation) return { kind: "toggle" };
  if (ownsArrows) return null;
  if (input.key === "ArrowLeft") return { kind: "seek", deltaMs: input.shiftKey ? -1000 : -5000 };
  if (input.key === "ArrowRight") return { kind: "seek", deltaMs: input.shiftKey ? 1000 : 5000 };
  if (input.key === "Home") return { kind: "seek-to", positionMs: 0 };
  if (input.key === "End" && durationMs !== null) return { kind: "seek-to", positionMs: durationMs };
  return null;
}

/** ⌨️ Applies a seek command to a position, clamped to the known duration. */
export function seekedMediaPosition(command: Extract<MediaKeyCommand, { kind: "seek" | "seek-to" }>, positionMs: number, durationMs: number | null): number {
  const target = command.kind === "seek" ? positionMs + command.deltaMs : command.positionMs;
  return Math.max(0, durationMs === null ? target : Math.min(durationMs, target));
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

// #endregion 🎬️MediaWindowKit
