/** 🔁️ Command > Reset panels restores the panel dock AND every window a user closed: the dock reset also drops the shell's own window layout so the app's default layout is seeded again. */
import { describe, expect, it } from "vitest";
import { shellLayoutReducer } from "../../🧱️elements/🐚️Shell/♻️slice-reducers/🟦️.ts";

describe("reset panels", () => {
  it("clears the dock override and the closed-window layout together", () => {
    const closedLayout = { kind: "stack", children: [{ kind: "window", windowKindId: "model" }] };
    const state = { panels: {}, dockOverride: { stale: true }, shellLayout: closedLayout, panelPathMemory: { a: ["b"] }, treeOpenStates: { c: true } } as never;
    const next = shellLayoutReducer(state, { type: "RESET_DOCK" });
    expect(next.shellLayout).toBeNull();
    expect(next.dockOverride).toBeNull();
    expect(next.panelPathMemory).toEqual({});
    expect(next.treeOpenStates).toEqual({});
  });
});
