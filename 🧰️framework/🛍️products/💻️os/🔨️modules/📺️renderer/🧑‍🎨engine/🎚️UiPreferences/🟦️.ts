/** 🎚️ Canonical host adapter for persisted local OS UI preferences. */

import { OsShellConfig, type StoragePort } from "@semio-tech/framework";
import { parseUiDriver, parseUiTheme, type ElementsSurfaceAppearance, type UiChromeLayout, type UiDriver, type UiLocale, type UiTheme } from "@semio-tech/ui-react";
import { applyUiPreferencesConfigMutation, UI_PREFERENCE_MUTATION_KEYS, uiPreferenceMutationDataClassV1, type UiPreferencesConfigMutation } from "../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
import { parseUiPreferences, type UiDriver as CanonicalUiDriver, type UiPreferences, type UiTheme as CanonicalUiTheme } from "../../../../🎚️config/🧬️schema/🟦️.ts";

export const UI_PREFERENCES_CONFIG_SCHEMA = "os.config.ui-preferences";

export interface UiPreferencesEventLog {
  readonly version: 1;
  readonly events: readonly UiPreferencesConfigMutation[];
}

type UiPreferencesSubscriber = {
  readonly storage: StoragePort;
  readonly listener: (preferences: UiPreferences) => void;
  raw: string | undefined;
};

const subscribers = new Set<UiPreferencesSubscriber>();
let browserStorageListenerInstalled = false;

export interface ResolvedUiPreferences {
  readonly appearance: ElementsSurfaceAppearance;
  readonly layout: UiChromeLayout;
  readonly driverId: string;
  readonly customDrivers: Record<string, UiDriver>;
  readonly locale: UiLocale;
  readonly terminology: string;
  readonly themeId: string;
  readonly customThemes: Record<string, UiTheme>;
  readonly keybindingOverrides: Record<string, string>;
}

export const emptyUiPreferences = (): UiPreferences => ({
  appearance: null,
  layout: null,
  driverId: null,
  customDrivers: {},
  locale: null,
  terminology: null,
  themeId: null,
  customThemes: {},
  keybindingOverrides: {},
});

function rawUiPreferencesEventLog(storage: StoragePort): string | undefined {
  return new OsShellConfig(storage).getPreference(UI_PREFERENCES_CONFIG_SCHEMA);
}

function parseUiPreferencesEventLog(raw: string | undefined): UiPreferencesEventLog {
  if (raw === undefined) return { version: 1, events: [] };
  try {
    const value = JSON.parse(raw) as { readonly version?: unknown; readonly events?: unknown };
    if (value.version !== 1 || !Array.isArray(value.events)) return { version: 1, events: [] };
    let projection = emptyUiPreferences();
    for (const event of value.events) {
      projection = parseUiPreferences(applyUiPreferencesConfigMutation(projection, event as UiPreferencesConfigMutation));
    }
    return { version: 1, events: value.events as UiPreferencesConfigMutation[] };
  } catch {
    return { version: 1, events: [] };
  }
}

export function readUiPreferenceEvents(storage: StoragePort): readonly UiPreferencesConfigMutation[] {
  return parseUiPreferencesEventLog(rawUiPreferencesEventLog(storage)).events;
}

export function replayUiPreferenceEvents(events: readonly UiPreferencesConfigMutation[]): UiPreferences {
  return events.reduce((projection, event) => parseUiPreferences(applyUiPreferencesConfigMutation(projection, event)), emptyUiPreferences());
}

export function readUiPreferences(storage: StoragePort): UiPreferences {
  return replayUiPreferenceEvents(readUiPreferenceEvents(storage));
}

function publishUiPreferences(): void {
  for (const subscriber of subscribers) {
    const raw = rawUiPreferencesEventLog(subscriber.storage);
    if (raw === subscriber.raw) continue;
    subscriber.raw = raw;
    subscriber.listener(readUiPreferences(subscriber.storage));
  }
}

function installBrowserStorageListener(): void {
  if (browserStorageListenerInstalled || typeof window === "undefined") return;
  browserStorageListenerInstalled = true;
  window.addEventListener("storage", publishUiPreferences);
}

export function subscribeUiPreferences(storage: StoragePort, listener: (preferences: UiPreferences) => void): () => void {
  installBrowserStorageListener();
  const subscriber: UiPreferencesSubscriber = { storage, listener, raw: rawUiPreferencesEventLog(storage) };
  subscribers.add(subscriber);
  return () => subscribers.delete(subscriber);
}

export function commitUiPreferencesConfigMutation(storage: StoragePort, mutation: UiPreferencesConfigMutation): UiPreferences {
  return appendUiPreferencesConfigMutations(storage, [mutation]);
}

function appendUiPreferencesConfigMutations(storage: StoragePort, mutations: readonly UiPreferencesConfigMutation[]): UiPreferences {
  const events = [...readUiPreferenceEvents(storage), ...mutations];
  const next = replayUiPreferenceEvents(events);
  if (mutations.length === 0) return next;
  new OsShellConfig(storage).setPreference(UI_PREFERENCES_CONFIG_SCHEMA, JSON.stringify({ version: 1, events } satisfies UiPreferencesEventLog));
  publishUiPreferences();
  return next;
}

//#region 🌐️PreferenceLane
/** 🌐️ The preference vocabulary id a `user.preference-recorded` event carries for these preferences. */
export const UI_PREFERENCES_LANE_SCHEMA_V1 = "os.config.ui-preferences.v1";
/** 🌐️ Where a device keeps each lane's frontier and not yet acknowledged changes (`semio.os.config` preferences). */
export const UI_PREFERENCES_LANE_CONFIG_SCHEMA = "os.config.ui-preferences-lane";

/** 🌐️ One change of a `persistedShared` preference waiting for the hub: `id` travels inside the recorded envelope so the
 * device recognizes its own change when the lane hands it back; `requestId` makes every resend the same directory
 * command (`previously-accepted`). */
export type UiPreferenceLaneEntryV1 = { readonly id: string; readonly requestId: string; readonly mutation: UiPreferencesConfigMutation };

/** 🌐️ One device's view of one lane (one hub origin × one user): the hub seq it has folded through and its own
 * unacknowledged changes. */
export type UiPreferenceLaneStateV1 = { readonly lane: string; readonly afterSeq: number; readonly outbox: readonly UiPreferenceLaneEntryV1[] };

type UiPreferenceLaneLogV1 = { readonly version: 1; readonly lanes: Readonly<Record<string, { readonly afterSeq: number; readonly outbox: readonly UiPreferenceLaneEntryV1[] }>> };

/** 🌐️ The lane key of a signed-in user: one hub origin, one user id. */
export function uiPreferenceLaneKeyV1(hubBaseUrl: string, userId: string): string {
  return `${new URL(hubBaseUrl).origin}#${userId}`;
}

/** 🌐️ The canonical JSON text a change is recorded as — `{ id, mutation }` in that order. */
export function uiPreferenceEnvelopeTextV1(entry: Pick<UiPreferenceLaneEntryV1, "id" | "mutation">): string {
  return JSON.stringify({ id: entry.id, mutation: entry.mutation });
}

/** 🌐️ A recorded envelope back as `{ id, mutation }`, or `null` when it is not one of these preferences' shared changes: a
 * foreign vocabulary, a malformed id, a device-local key (never shared) or a mutation the schema refuses. */
export function parseUiPreferenceEnvelopeV1(schema: string, text: string): Pick<UiPreferenceLaneEntryV1, "id" | "mutation"> | null {
  if (schema !== UI_PREFERENCES_LANE_SCHEMA_V1) return null;
  try {
    const value = JSON.parse(text) as { readonly id?: unknown; readonly mutation?: unknown };
    if (value === null || typeof value !== "object" || Array.isArray(value) || Object.keys(value).join() !== "id,mutation") return null;
    if (typeof value.id !== "string" || !/^[0-9a-f]{32}$/u.test(value.id)) return null;
    const mutation = value.mutation as UiPreferencesConfigMutation;
    if (mutation === null || typeof mutation !== "object" || !(mutation.mutation in UI_PREFERENCE_MUTATION_KEYS) || uiPreferenceMutationDataClassV1(mutation) !== "persistedShared") return null;
    parseUiPreferences(applyUiPreferencesConfigMutation(emptyUiPreferences(), mutation));
    return { id: value.id, mutation };
  } catch {
    return null;
  }
}

/** 🎯️ The slot a mutation writes — a key, or a key plus the entry it sets inside a map — so a newer pending change of the
 * same slot supersedes an older one that never reached the hub, and the outbox stays bounded by the slots in use. */
function uiPreferenceSlotV1(mutation: UiPreferencesConfigMutation): string {
  const key = UI_PREFERENCE_MUTATION_KEYS[mutation.mutation];
  if (mutation.mutation === "setKeybindingOverride") return `${key}:${mutation.controlId}`;
  if (mutation.mutation === "setCustomTheme") return `${key}:${mutation.themeId}`;
  if (mutation.mutation === "setCustomDriver") return `${key}:${mutation.driverId}`;
  return key;
}

/** ➕️ Queues one change of a shared preference on a lane (a device-local one never is). */
export function queueUiPreferenceLaneV1(state: UiPreferenceLaneStateV1, entry: UiPreferenceLaneEntryV1): UiPreferenceLaneStateV1 {
  if (uiPreferenceMutationDataClassV1(entry.mutation) !== "persistedShared") return state;
  const slot = uiPreferenceSlotV1(entry.mutation);
  return { ...state, outbox: [...state.outbox.filter((pending) => uiPreferenceSlotV1(pending.mutation) !== slot), entry] };
}

/** 🔁️ Folds a lane page into a device, in hub order, so the local projection always equals "the hub's order, then what
 * this device has not yet had accepted". The device's own changes at the head of the page are only acknowledged — they
 * already stand on top of the local log; from the first change of another device on, every change is appended in hub
 * order (the device's own included, so a later own change still wins over an earlier foreign one), and then the
 * device's still-unacknowledged changes are appended again, so an offline edit stays on top until the hub orders it.
 * Every device therefore converges on the hub's last write per slot. */
export function foldUiPreferenceLaneEventsV1(state: UiPreferenceLaneStateV1, events: readonly { readonly seq: number; readonly schema: string; readonly mutation: string }[], throughSeqInclusive: number): { readonly state: UiPreferenceLaneStateV1; readonly append: readonly UiPreferencesConfigMutation[] } {
  let outbox = state.outbox;
  let afterSeq = state.afterSeq;
  let foreign = false;
  const ordered: UiPreferencesConfigMutation[] = [];
  for (const event of [...events].sort((left, right) => left.seq - right.seq)) {
    if (event.seq <= afterSeq) continue;
    afterSeq = event.seq;
    const envelope = parseUiPreferenceEnvelopeV1(event.schema, event.mutation);
    if (envelope === null) continue;
    const own = outbox.some((pending) => pending.id === envelope.id);
    if (own) outbox = outbox.filter((pending) => pending.id !== envelope.id);
    else foreign = true;
    if (foreign) ordered.push(envelope.mutation);
  }
  afterSeq = Math.max(afterSeq, throughSeqInclusive);
  return { state: { ...state, afterSeq, outbox }, append: foreign ? [...ordered, ...outbox.map((pending) => pending.mutation)] : [] };
}

function readUiPreferenceLaneLog(storage: StoragePort): UiPreferenceLaneLogV1 {
  try {
    const value = JSON.parse(new OsShellConfig(storage).getPreference(UI_PREFERENCES_LANE_CONFIG_SCHEMA) ?? "null") as UiPreferenceLaneLogV1 | null;
    return value !== null && value.version === 1 && typeof value.lanes === "object" && value.lanes !== null ? value : { version: 1, lanes: {} };
  } catch {
    return { version: 1, lanes: {} };
  }
}

/** 📖️ A device's state of one lane (a lane it never joined starts at seq 0 with nothing pending). */
export function readUiPreferenceLane(storage: StoragePort, lane: string): UiPreferenceLaneStateV1 {
  const entry = readUiPreferenceLaneLog(storage).lanes[lane];
  return { lane, afterSeq: entry?.afterSeq ?? 0, outbox: entry?.outbox ?? [] };
}

function writeUiPreferenceLane(storage: StoragePort, state: UiPreferenceLaneStateV1): void {
  const log = readUiPreferenceLaneLog(storage);
  new OsShellConfig(storage).setPreference(UI_PREFERENCES_LANE_CONFIG_SCHEMA, JSON.stringify({ version: 1, lanes: { ...log.lanes, [state.lane]: { afterSeq: state.afterSeq, outbox: state.outbox } } } satisfies UiPreferenceLaneLogV1));
}

/** ✍️ One user change, local-first: applied to this device's log at once and, for a shared preference on a signed-in
 * lane, queued for the hub. Returns the queued entry the caller sends (`null` for a device-local change or no lane). */
export function commitUiPreferenceOnLaneV1(storage: StoragePort, mutation: UiPreferencesConfigMutation, lane: string | null, mintId: () => string): { readonly preferences: UiPreferences; readonly queued: UiPreferenceLaneEntryV1 | null } {
  const preferences = commitUiPreferencesConfigMutation(storage, mutation);
  if (lane === null || uiPreferenceMutationDataClassV1(mutation) !== "persistedShared") return { preferences, queued: null };
  const queued: UiPreferenceLaneEntryV1 = { id: mintId(), requestId: mintId(), mutation };
  writeUiPreferenceLane(storage, queueUiPreferenceLaneV1(readUiPreferenceLane(storage, lane), queued));
  return { preferences, queued };
}

/** 📥️ Folds one lane page into this device's log and lane state ({@link foldUiPreferenceLaneEventsV1}). */
export function foldUiPreferenceLanePageV1(storage: StoragePort, lane: string, page: { readonly throughSeqInclusive: number; readonly events: readonly { readonly seq: number; readonly schema: string; readonly mutation: string }[] }): UiPreferences {
  const folded = foldUiPreferenceLaneEventsV1(readUiPreferenceLane(storage, lane), page.events, page.throughSeqInclusive);
  writeUiPreferenceLane(storage, folded.state);
  return appendUiPreferencesConfigMutations(storage, folded.append);
}
//#endregion 🌐️PreferenceLane

export function canonicalUiDriver(driver: UiDriver): CanonicalUiDriver {
  const { id, label, ...config } = driver;
  return { driverId: id, label, config };
}

export function canonicalUiTheme(theme: UiTheme): CanonicalUiTheme {
  const { id, label, ...config } = theme;
  return { themeId: id, label, config };
}

function resolveCustomDrivers(drivers: UiPreferences["customDrivers"]): Record<string, UiDriver> {
  const resolved: Record<string, UiDriver> = {};
  for (const [id, driver] of Object.entries(drivers)) {
    try {
      resolved[id] = parseUiDriver({ id: driver.driverId, label: driver.label, ...(driver.config as object) });
    } catch {
      continue;
    }
  }
  return resolved;
}

function resolveCustomThemes(themes: UiPreferences["customThemes"]): Record<string, UiTheme> {
  const resolved: Record<string, UiTheme> = {};
  for (const [id, theme] of Object.entries(themes)) {
    try {
      resolved[id] = parseUiTheme({ id: theme.themeId, label: theme.label, ...(theme.config as object) });
    } catch {
      continue;
    }
  }
  return resolved;
}

export function resolveUiPreferences(
  preferences: UiPreferences,
  fallback: Pick<ResolvedUiPreferences, "appearance" | "layout" | "driverId" | "locale" | "terminology" | "themeId">,
): ResolvedUiPreferences {
  return {
    appearance: preferences.appearance ?? fallback.appearance,
    layout: preferences.layout ?? fallback.layout,
    driverId: preferences.driverId ?? fallback.driverId,
    customDrivers: resolveCustomDrivers(preferences.customDrivers),
    locale: preferences.locale ?? fallback.locale,
    terminology: preferences.terminology ?? fallback.terminology,
    themeId: preferences.themeId ?? fallback.themeId,
    customThemes: resolveCustomThemes(preferences.customThemes),
    keybindingOverrides: { ...preferences.keybindingOverrides },
  };
}

if (import.meta.vitest) {
  const { registerTests1 } = await import("./🧪️tests/🎚️canonical-os-ui-preferences/🟦️.ts");
  await registerTests1(import.meta.vitest, { commitUiPreferencesConfigMutation, readUiPreferenceEvents, readUiPreferences, replayUiPreferenceEvents, resolveUiPreferences, subscribeUiPreferences, UI_PREFERENCES_CONFIG_SCHEMA }, { url: import.meta.url });
  const { registerTests1: registerPreferenceLaneTests } = await import("./🧪️tests/🌐️preference-lane/🟦️.ts");
  await registerPreferenceLaneTests(import.meta.vitest, { commitUiPreferenceOnLaneV1, foldUiPreferenceLanePageV1, readUiPreferenceLane, readUiPreferences, replayUiPreferenceEvents, uiPreferenceEnvelopeTextV1, parseUiPreferenceEnvelopeV1, UI_PREFERENCES_LANE_SCHEMA_V1, uiPreferenceLaneKeyV1 }, { url: import.meta.url });
}
