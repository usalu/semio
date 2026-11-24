/** 📚️ The example offer, ownership and switching tests. */
import { EMPTY_EXAMPLE_ID, SET_ACTIVE_EXAMPLE_ACTION_ID, appSwitchesExamples, appOffersRegisteredExamples, buildActiveExampleAction, exampleSelectionAfterInputV1, frameworkOwnsExampleSwitch, navbarExampleIdFromHistoryUpserts, rememberedExampleIdFromDispatchV1 } from "../../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts";
import settlementFixture from "./settlement.json";
import historyFixture from "./history.json";
import exampleOfferFixture from "../../../🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/📚️example-offer/🔣️.json";
import exampleOfferSchema from "../../../🧱️elements/🐚️Shell/📚️example-offer/🔣️.json";
import Ajv2020 from "ajv/dist/2020";
import { expect, it } from "vitest";

/** 🔀 Registers the example offer, neutral ownership vector, switch admission and remembered example tests. */
export function registerExampleOffersAndSwitchingTests(): void {
  it("keeps the selected example after a whole-document command history refresh", () => {
    for (const row of historyFixture) {
      expect(navbarExampleIdFromHistoryUpserts(row.upserts, "concrete-forest", "demo") ?? null, row.name).toBe(row.expected);
    }
  });
  it("publishes only a settled example for the focused program and remembers it for redo", () => {
    const previous = { exampleId: "house", inputSeq: 3 };
    for (const row of settlementFixture) {
      const next = exampleSelectionAfterInputV1({ action: SET_ACTIVE_EXAMPLE_ACTION_ID, args: { exampleId: row.requested } }, row, previous, "demo", row.focused);
      expect(next ?? null, row.name).toEqual(row.expected);
      if (next) {
        expect(navbarExampleIdFromHistoryUpserts([{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, editId: "example", applied: false }], next.memory.exampleId, "demo")).toBe("demo");
        expect(navbarExampleIdFromHistoryUpserts([{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, editId: "example", applied: true }], next.memory.exampleId, "demo")).toBe(next.memory.exampleId);
      }
    }
  });
  it("offers examples only to an app that declares setActiveExample on some window kind", () => {
    expect(
      appSwitchesExamples("s.cad.cad@1/*#editor", [
        { id: "cad-play-shape", actions: [{ id: "select" }] },
        { id: "cad-play-energy", actions: [{ id: SET_ACTIVE_EXAMPLE_ACTION_ID }] },
      ]),
    ).toBe(true);
    expect(appSwitchesExamples("s.vcs.vcs@1/*#editor", [{ id: "vcs-editor", actions: [{ id: "commit" }] }, { id: "vcs-history" }])).toBe(false);
    expect(appSwitchesExamples("s.note.note@1/*#editor", [])).toBe(false);
  });
  it("answers neutral example ownership vectors with the independent schema oracle", () => {
    const oracle = new Ajv2020({ strict: true }).compile(exampleOfferSchema);
    for (const { name, expected, ...row } of exampleOfferFixture.cases) {
      const app = { id: name, role: row.role, actions: row.appActions.map((id) => ({ id })), windowKinds: row.windowActions.map((actions, index) => ({ id: String(index), actions: actions.map((id) => ({ id })) })) };
      expect(oracle(row), name).toBe(expected);
      expect(appOffersRegisteredExamples(app, row.exampleCount), name).toBe(expected);
    }
  });
  it("lists registered examples on every editor and admits the switch without a declared action", () => {
    const editor = { id: "s.lowpoly.lowpoly@1/*#editor", role: "editor", windowKinds: [{ id: "model", actions: [{ id: "select" }] }] };
    expect(appOffersRegisteredExamples(editor, 1)).toBe(true);
    expect(appOffersRegisteredExamples(editor, 0)).toBe(false);
    expect(appSwitchesExamples(editor.id, editor.windowKinds)).toBe(false);
    expect(frameworkOwnsExampleSwitch("editor", SET_ACTIVE_EXAMPLE_ACTION_ID)).toBe(true);
    expect(frameworkOwnsExampleSwitch("viewer", SET_ACTIVE_EXAMPLE_ACTION_ID)).toBe(false);
    expect(frameworkOwnsExampleSwitch("editor", "select")).toBe(false);
    const viewer = { id: "s.lowpoly.lowpoly@1/*#viewer", role: "viewer", windowKinds: [{ id: "model", actions: [{ id: "select" }] }] };
    expect(appOffersRegisteredExamples(viewer, 1)).toBe(false);
    expect(appOffersRegisteredExamples({ ...viewer, windowKinds: [{ id: "model", actions: [{ id: SET_ACTIVE_EXAMPLE_ACTION_ID }] }] }, 1)).toBe(true);
  });
  it("remembers the example id of every setActiveExample dispatch, so a redone row can relabel the picker", () => {
    const remembered = rememberedExampleIdFromDispatchV1({ action: SET_ACTIVE_EXAMPLE_ACTION_ID, args: { exampleId: "nakagin-capsule-tower" } }, "");
    expect(remembered).toBe("nakagin-capsule-tower");
    expect(rememberedExampleIdFromDispatchV1({ action: "setSpacing", args: { exampleId: "concrete-forest" } }, remembered)).toBe("nakagin-capsule-tower");
    expect(rememberedExampleIdFromDispatchV1({ action: SET_ACTIVE_EXAMPLE_ACTION_ID, args: { exampleId: EMPTY_EXAMPLE_ID } }, remembered)).toBe(EMPTY_EXAMPLE_ID);
    expect(rememberedExampleIdFromDispatchV1({ action: SET_ACTIVE_EXAMPLE_ACTION_ID }, remembered)).toBe("nakagin-capsule-tower");
    const popped = [{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, editId: "example", applied: false }];
    const live = [{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, editId: "example", applied: true }];
    expect(navbarExampleIdFromHistoryUpserts(popped, remembered, "concrete-forest")).toBe("concrete-forest");
    expect(navbarExampleIdFromHistoryUpserts(live, remembered, "concrete-forest")).toBe("nakagin-capsule-tower");
    expect(navbarExampleIdFromHistoryUpserts(live, undefined, "concrete-forest")).toBeUndefined();
    expect(navbarExampleIdFromHistoryUpserts(live, EMPTY_EXAMPLE_ID, "concrete-forest")).toBe(EMPTY_EXAMPLE_ID);
  });
  it("keeps the empty document selected through settle, undo and redo, and offers it first-class on every picker", async () => {
    const empty = exampleSelectionAfterInputV1({ action: SET_ACTIVE_EXAMPLE_ACTION_ID, args: { exampleId: EMPTY_EXAMPLE_ID } }, { kind: "applied", inputSeq: 5 }, { exampleId: "demo", inputSeq: 4 }, "demo", true);
    expect(empty).toEqual({ memory: { exampleId: EMPTY_EXAMPLE_ID, inputSeq: 5 }, visibleExampleId: EMPTY_EXAMPLE_ID });
    const back = exampleSelectionAfterInputV1({ action: SET_ACTIVE_EXAMPLE_ACTION_ID, args: { exampleId: "demo" } }, { kind: "applied", inputSeq: 6 }, empty!.memory, "demo", true);
    expect(back?.visibleExampleId).toBe("demo");
    expect(buildActiveExampleAction("c", EMPTY_EXAMPLE_ID).args).toEqual({ exampleId: EMPTY_EXAMPLE_ID });
    const scope = await Bun.file(new URL("../../../🧱️elements/🏛️ShellHost/🔎️command-bar/🔶️scope/🟦️.tsx", import.meta.url)).text();
    expect(scope).toMatch(/<NavbarExampleSelect[^>]*\bincludeNoExample\b/s);
  });
  it("keeps the picker unchanged for history from another action", () => {
    expect(navbarExampleIdFromHistoryUpserts([{ actionId: "shell.windowResize", editId: "resize", applied: false }], "nakagin", "forest")).toBeUndefined();
  });
}
