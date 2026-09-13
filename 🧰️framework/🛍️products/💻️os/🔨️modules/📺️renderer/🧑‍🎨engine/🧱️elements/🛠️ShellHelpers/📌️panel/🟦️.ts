/** 📌️ The host-owned panel carriage — the ONE writer and the ONE reader of `ViewModel.panelJson`,
 * the only long string a host→guest view context may carry
 * (`🛂️manifest/🪟️view-context/🧬️schema/🔣️.json`, 65 536 characters).
 *
 * 🏁️ Its own module, with no React and no shell imports, for the same reason the contributions
 * publisher has one: importing `🛠️ShellHelpers/🟦️.tsx` from a test hits the
 * `ShellHelpers → Shell → ShellHost` cycle and dies with
 * `Cannot access '__vite_ssr_import_8__' before initialization`, so a law could not drive it. The
 * shape types come in as `import type` (erased at runtime), which is why this module adds no edge.
 *
 * 🪶️ The carriage deliberately holds NO roster. The app roster crosses as its own
 * `setAppRegistrations` hint-push and contributions as their own paged `setContributions` run
 * (`🧩️contributions/🟦️.ts`); a roster inside the panel grows with the plugin closure and is exactly
 * the payload that put 248 635 characters against a 65 536-character bound
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END). */

import type { SpacePanelState, SpawnedAppEntry } from "../../🐚️Shell/🟦️.tsx";
import type { PluginViewState as ViewModel } from "@semio-tech/framework";

const PANEL_JSON_CAPACITY = 65_536;
const PANEL_IDENTIFIER_CAPACITY = 256;
const SPAWNED_APP_CAPACITY = 64;
const CONTROL = /[\u0000-\u001f\u007f]/u;

function isIdentifier(value: unknown): value is string {
  return typeof value === "string" && value.length > 0 && Array.from(value).length <= PANEL_IDENTIFIER_CAPACITY && !CONTROL.test(value);
}

function isCarriedText(value: unknown): value is string {
  return typeof value === "string" && Array.from(value).length <= PANEL_JSON_CAPACITY;
}

/** 📌️ Validates the strict host-owned panel carriage independently of its JSON parser. */
export function isSpacePanelState(value: unknown): value is SpacePanelState {
  if (value === null || typeof value !== "object" || Array.isArray(value)) return false;
  const state = value as Record<string, unknown>;
  if (Object.keys(state).some((key) => !["activePanelTab", "spawnedApps", "activeSpawnedId"].includes(key))) return false;
  if (!isIdentifier(state.activePanelTab) || !Array.isArray(state.spawnedApps) || state.spawnedApps.length > SPAWNED_APP_CAPACITY) return false;
  const ids = new Set<string>();
  for (const value of state.spawnedApps) {
    if (value === null || typeof value !== "object" || Array.isArray(value)) return false;
    const entry = value as Record<string, unknown>;
    if (Object.keys(entry).sort().join("|") !== "appId|breadcrumb|id|instanceId|label|pluginId") return false;
    if (!isIdentifier(entry.id) || ids.has(entry.id) || !isIdentifier(entry.pluginId) || !isIdentifier(entry.appId)) return false;
    if (!Number.isInteger(entry.instanceId) || (entry.instanceId as number) < 0 || (entry.instanceId as number) > 0xffff_ffff) return false;
    if (!isCarriedText(entry.label) || !Array.isArray(entry.breadcrumb) || entry.breadcrumb.length > SPAWNED_APP_CAPACITY || entry.breadcrumb.some((part) => !isCarriedText(part))) return false;
    ids.add(entry.id);
  }
  if (state.activeSpawnedId !== undefined && (!isIdentifier(state.activeSpawnedId) || !ids.has(state.activeSpawnedId))) return false;
  return Array.from(JSON.stringify(value)).length <= PANEL_JSON_CAPACITY;
}

/** 📌️ Builds a host panel state without duplicating the installed app catalogue. */
export function buildSpacePanelState(spawnedApps: readonly SpawnedAppEntry[], activePanelTab: string, activeSpawnedId?: string): SpacePanelState {
  return { activePanelTab, spawnedApps, activeSpawnedId };
}

/** 📦️ Encodes a panel for `ViewModel.panelJson`. */
export function panelJsonFromState(state: SpacePanelState): string {
  if (!isSpacePanelState(state)) throw new Error("host panel state violates its strict carriage contract");
  return JSON.stringify(state);
}

/** 📦️ Decodes the strict JSON panel a view state carries; only an absent payload is no panel. */
export function parsePanelState(viewState: ViewModel): SpacePanelState | null {
  if (!viewState.panelJson) return null;
  if (Array.from(viewState.panelJson).length > PANEL_JSON_CAPACITY) throw new Error("host panel JSON exceeds its carriage capacity");
  const state: unknown = JSON.parse(viewState.panelJson);
  if (!isSpacePanelState(state)) throw new Error("host panel JSON violates its strict carriage contract");
  return state;
}

/**
 * @emoji 🪟️ Returns a studio panel with `spawned` present and focused as `activeSpawnedId`.
 * Host-effect application must fold this into the in-flight `nextViewState` before the final
 * `SET_SESSION` write — a separate panel dispatch is overwritten by that write and leaves the shell
 * stuck on the studio surface.
 * @see Effect.openPluginInstance
 */
export function studioPanelFocusingSpawned(panel: SpacePanelState, spawned: SpawnedAppEntry): SpacePanelState {
  const spawnedApps = panel.spawnedApps.some((entry) => entry.id === spawned.id) ? panel.spawnedApps.map((entry) => (entry.id === spawned.id ? spawned : entry)) : [...panel.spawnedApps, spawned];
  return buildSpacePanelState(spawnedApps, panel.activePanelTab, spawned.id);
}

/** @emoji 🐚️ Commits a studio panel into a view state's `panelJson` for a single host-effect session write. */
export function viewStateWithSpacePanel(viewState: ViewModel, panel: SpacePanelState): ViewModel {
  return { ...viewState, panelJson: panelJsonFromState(panel) };
}
