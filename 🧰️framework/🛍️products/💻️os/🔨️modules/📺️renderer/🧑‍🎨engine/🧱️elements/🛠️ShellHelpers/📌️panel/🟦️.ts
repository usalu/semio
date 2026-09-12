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

import { packValueFromBase64, packValueToBase64 } from "@semio-tech/framework-os";
import type { SpacePanelState, SpaceProgramEntry, SpawnedAppEntry } from "../../🐚️Shell/🟦️.tsx";
import type { PluginViewState as ViewModel } from "@semio-tech/framework";

/** 📌️ Builds a panel. `programs` is the roster the shell never fills — it is kept as an explicit,
 * always-empty parameter so a caller that tries to fill it is visible at the call site. */
export function buildSpacePanelState(programs: readonly SpaceProgramEntry[], spawnedApps: readonly SpawnedAppEntry[], activePanelTab = "s-play-catalogue", activeSpawnedId?: string): SpacePanelState {
  return { activePanelTab, programs, spawnedApps, activeSpawnedId };
}

/** 📦️ Encodes a panel for `ViewModel.panelJson`. */
export function panelJsonFromState(state: SpacePanelState): string {
  return packValueToBase64(state);
}

/** 📦️ Decodes the panel a view state carries; an absent or unreadable payload is no panel. */
export function parsePanelState(viewState: ViewModel): SpacePanelState | null {
  if (!viewState.panelJson) return null;
  try {
    return packValueFromBase64(viewState.panelJson) as SpacePanelState;
  } catch {
    return null;
  }
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
  return buildSpacePanelState(panel.programs, spawnedApps, panel.activePanelTab, spawned.id);
}

/** @emoji 🐚️ Commits a studio panel into a view state's `panelJson` for a single host-effect session write. */
export function viewStateWithSpacePanel(viewState: ViewModel, panel: SpacePanelState): ViewModel {
  return { ...viewState, panelJson: panelJsonFromState(panel) };
}
