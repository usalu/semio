/** 🎚️ Canonical host adapter for persisted local OS UI preferences. */

import { OsShellConfig, type StoragePort } from "@semio-tech/framework";
import { parseUiDriver, parseUiTheme, type ElementsSurfaceAppearance, type UiChromeLayout, type UiDriver, type UiLocale, type UiTheme } from "@semio-tech/ui-react";
import { applyUiPreferencesConfigMutation, type UiPreferencesConfigMutation } from "../../../../🎚️config/🧬️schema/🧬️mutations/🟦️.ts";
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
  const events = [...readUiPreferenceEvents(storage), mutation];
  const next = replayUiPreferenceEvents(events);
  new OsShellConfig(storage).setPreference(UI_PREFERENCES_CONFIG_SCHEMA, JSON.stringify({ version: 1, events } satisfies UiPreferencesEventLog));
  publishUiPreferences();
  return next;
}

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
}
