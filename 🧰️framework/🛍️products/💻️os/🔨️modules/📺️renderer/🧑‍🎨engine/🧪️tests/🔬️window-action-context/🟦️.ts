import assert from "node:assert/strict";
import { applyPatch } from "fast-json-patch";
import { describe, expect, it } from "vitest";
import { inputActionWindowV1, inputActionWithWindowV1 } from "../../🧱️elements/🏛️ShellHost/🎯️input-ledger/🟦️.ts";
import fixture from "../🧫️fixtures/🔬️window-action-context/🔣️.json";

/** 🎯️ RFC 6902 independently projects the host target onto the shared native action fixtures. */
export function testWindowActionContextOracle(): void {
  assert.notEqual(fixture.activeWindowId, fixture.clickedWindowId);
  for (const test of fixture.cases) {
    const result = applyPatch(structuredClone(test.args ?? {}), [{ op: "add", path: "/windowId", value: fixture.clickedWindowId }], true).newDocument;
    assert.deepEqual(result, test.expected);
  }
}

describe("window action context", () => {
  it("projects the clicked window onto every shared fixture case (RFC 6902 oracle)", () => {
    expect(fixture.cases.length).toBeGreaterThan(0);
    testWindowActionContextOracle();
  });

  it("resolves a control action to its clicked window, never the focused one, without touching domain args", () => {
    for (const test of fixture.cases) {
      const bound = inputActionWithWindowV1({ controllerId: "c", action: "a", args: test.args ?? undefined }, fixture.clickedWindowId);
      expect(inputActionWindowV1(bound, fixture.activeWindowId), JSON.stringify(test.args)).toBe(fixture.clickedWindowId);
      expect(bound.args).toEqual(test.args ?? undefined);
    }
  });

  it("falls back to an explicit args.windowId, then to the focused window", () => {
    expect(inputActionWindowV1({ controllerId: "c", action: "a", args: { windowId: fixture.clickedWindowId } }, fixture.activeWindowId)).toBe(fixture.clickedWindowId);
    expect(inputActionWindowV1({ controllerId: "c", action: "a" }, fixture.activeWindowId)).toBe(fixture.activeWindowId);
    expect(inputActionWindowV1({ controllerId: "c", action: "a" }, null)).toBeNull();
  });
});
