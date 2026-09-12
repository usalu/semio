/** 📌️ What a host→guest view context may carry, as laws.
 *
 * 🏁️ Measured live at `http://127.0.0.1:6018/?plugin=generation3d` on 2026-09-12: the React shell
 * put its aggregated plugin-contributions closure (248 635 characters) into `viewState` on every
 * refresh, against the 65 536-character long-field bound the ONE language-neutral view-context
 * schema declares (`🛂️manifest/🪟️view-context/🧬️schema/🔣️.json`). The admission answered
 * `view context: invalid panel data` and every window body of the app became a fault card
 * (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
 *
 * ⚖️ The fix is architectural, not a wider cap: contributions are INSTALLED into the guest by the
 * paged `setContributions` run the publisher owns (`🛠️ShellHelpers/🧩️contributions/🟦️.ts`) and the
 * guest folds them into its own registry, so the crossing context carries neither the closure nor a
 * reference to it. What remains is `panelJson`, host-owned panel state the shell never fills with a
 * roster. */

import { describe, expect, it } from "vitest";
import { parseResolvedPluginViewState, VIEW_CONTEXT_LONG_STRING_CHARS } from "@semio-tech/framework";
import { buildSpacePanelState, panelJsonFromState, parsePanelState } from "../../🧱️elements/🛠️ShellHelpers/📌️panel/🟦️.ts";
import fixture from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/📌️panel-carriage/🔣️.json";

const spawnedRoster = (count: number) =>
  Array.from({ length: count }, (_, index) => ({
    id: `${index}`.padEnd(fixture.spawned.idChars, "i"),
    pluginId: `${index}`.padEnd(fixture.spawned.pluginIdChars, "p"),
    instanceId: index,
    appId: `${index}`.padEnd(fixture.spawned.appIdChars, "a"),
    label: `${index}`.padEnd(fixture.spawned.labelChars, "l"),
    breadcrumb: Array.from({ length: fixture.spawned.breadcrumbSegments }, (_, segment) => `${segment}`.padEnd(fixture.spawned.breadcrumbSegmentChars, "b")),
  }));

describe("view-state carriage", () => {
  it("refuses contributions as a view-state field", () => {
    const resolved = { locale: "en", terminology: "native" } as const;
    expect(() => parseResolvedPluginViewState(resolved)).not.toThrow();
    expect(() => parseResolvedPluginViewState({ ...resolved, contributionsJson: "[]" })).toThrow(/explicit supported preferences required/);
    expect(() => parseResolvedPluginViewState({ ...resolved, contributionsJson: "n".repeat(fixture.measured.contributionsJsonChars) })).toThrow();
  });

  it("keeps panelJson the only long field, bounded at the schema capacity", () => {
    const resolved = { locale: "en", terminology: "native" } as const;
    expect(VIEW_CONTEXT_LONG_STRING_CHARS).toBe(fixture.capacityChars);
    expect(() => parseResolvedPluginViewState({ ...resolved, panelJson: "p".repeat(fixture.capacityChars) })).not.toThrow();
    expect(() => parseResolvedPluginViewState({ ...resolved, panelJson: "p".repeat(fixture.capacityChars + 1) })).toThrow(/invalid panel data at panelJson/);
  });

  it("never puts a program roster in the panel carriage, and stays inside the bound at window capacity", () => {
    // 🪐️ The shell's app roster crosses as its own `setAppRegistrations` hint-push, never as panel
    // state, so `programs` is empty in every panel the shell builds — the roster grows with the
    // plugin closure and would be exactly the payload that broke this bound.
    const empty = buildSpacePanelState([], []);
    expect(empty.programs).toEqual([]);
    expect(fixture.programsAreEmpty).toBe(true);

    const panel = buildSpacePanelState([], spawnedRoster(fixture.windowInstanceCapacity));
    const json = panelJsonFromState(panel);
    expect(parsePanelState({ panelJson: json })?.spawnedApps.length).toBe(fixture.windowInstanceCapacity);
    expect(json.length).toBeLessThanOrEqual(fixture.capacityChars);
    expect(() => parseResolvedPluginViewState({ locale: "en", terminology: "native", panelJson: json })).not.toThrow();
  });
});
