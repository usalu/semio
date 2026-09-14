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
import { parseResolvedPluginViewState, VIEW_CONTEXT_LONG_STRING_CHARS, type PluginViewState } from "@semio-tech/framework";
import { decodePackValue, encodePackValue, isPackInteger, packValueToBase64, packValueFromBase64, viewContextWireValue } from "@semio-tech/framework-os";
import { buildSpacePanelState, isSpacePanelState, panelJsonFromState, parsePanelState } from "../../🧱️elements/🛠️ShellHelpers/📌️panel/🟦️.ts";
import fixture from "../../🧱️elements/🛠️ShellHelpers/🧫️fixtures/📌️panel-carriage/🔣️.json";
import integerCarriers from "../../../../../../../🔨️modules/🛂️manifest/🪟️view-context/🧫️fixtures/🔢️integer-carriers/🔣️.json";

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
    // state: the carriage declares exactly the fields the fixture names and `isSpacePanelState`
    // refuses every other key, so a roster — which grows with the plugin closure and is exactly the
    // payload that broke this bound — has nowhere to ride.
    const empty = buildSpacePanelState([], fixture.activePanelTab);
    expect(Object.keys(empty).sort()).toEqual([...fixture.carriedFields].sort());
    expect(isSpacePanelState({ ...empty, programs: [] })).toBe(false);

    const panel = buildSpacePanelState(spawnedRoster(fixture.windowInstanceCapacity), fixture.activePanelTab);
    const json = panelJsonFromState(panel);
    expect(parsePanelState({ panelJson: json })?.spawnedApps.length).toBe(fixture.windowInstanceCapacity);
    expect(json.length).toBeLessThanOrEqual(fixture.capacityChars);
    expect(() => parseResolvedPluginViewState({ locale: "en", terminology: "native", panelJson: json })).not.toThrow();
  });
});

/** 🔢️ The React door's half of the integer-carrier law, over the SAME fixture the guest half reads
 * (`🛂️manifest/🧪️tests/🔢️integer-carriers/🦀️.rs`). A JS `number` is an IEEE double and
 * `encodePackValue` writes every one of them as `TAG_F64`, so the first integer-typed view-state
 * field the shell ever carried reached the guest as `Float(1.0)`, failed its `u64` `FromValue` and
 * took EVERY guest turn with it — 608 worker faults and 0 of 23 journey steps on
 * `http://127.0.0.1:6018/?plugin=generation3d`, 2026-09-14 (ticket
 * 26/09/09/PROCEDURAL-3D-END-TO-END). */
describe("view-context integer carriers", () => {
  const hex = (bytes: Uint8Array): string => Array.from(bytes, (byte) => byte.toString(16).padStart(2, "0")).join("");
  const view = integerCarriers.viewContext as PluginViewState;

  it("admits the carried context and reads every integer path as an exact carrier", () => {
    expect(() => parseResolvedPluginViewState(view)).not.toThrow();
    const wire = viewContextWireValue(view) as Record<string, unknown>;
    for (const path of integerCarriers.integerPaths) {
      const carrier = path.reduce<unknown>((node, key) => (node as Record<string, unknown>)[key], wire);
      expect(isPackInteger(carrier), path.join(".")).toBe(true);
    }
  });

  it("hands the guest the same bytes the wgpu door does", () => {
    expect(hex(encodePackValue(viewContextWireValue(view)))).toBe(integerCarriers.packHex);
  });

  it("fails-before: the unprojected context widens every integer onto a float", () => {
    expect(hex(encodePackValue(view))).not.toBe(integerCarriers.packHex);
    const widened = decodePackValue(encodePackValue(view)) as Record<string, Record<string, Record<string, unknown>>>;
    expect(isPackInteger(widened.toolRunTraceCursorByWindowId!["procedural-preview"]!.run)).toBe(false);
  });

  /** 🔁️ The wgpu bridge reaches the SAME `AppChannelClient.command` seam holding a context it decoded
   * straight out of pack — carriers already minted — and a call with no context at all passes
   * `undefined`. Minting a carrier twice throws on the `BigInt` and takes the dispatch with it, which
   * is the failure this projection exists to end, so the seam must be idempotent and total. */
  it("is idempotent over a context that already crossed as pack, and total over one that is not a context", () => {
    const wire = viewContextWireValue(view);
    const decoded = packValueFromBase64(packValueToBase64(wire as never));
    const again = viewContextWireValue(decoded);
    expect(hex(encodePackValue(again))).toBe(integerCarriers.packHex);
    for (const value of [undefined, null, "pk:", 7]) expect(viewContextWireValue(value)).toBe(value);
  });

  it("keeps a non-integral number a float, and never mints a carrier for one", () => {
    const fractional = { ...view, toolRunTraceCursorByWindowId: { "procedural-preview": { run: integerCarriers.nonIntegral, generation: 0, page: 0 } } };
    expect(() => parseResolvedPluginViewState(fractional)).toThrow(/invalid tool run trace cursor/);
    // 🚫️ The mint refuses rather than rounds: a fractional run id has no exact integer carrier, and
    // silently truncating it would resend the wrong trace page instead of failing loudly.
    expect(() => viewContextWireValue(fractional as PluginViewState)).toThrow();
  });
});
