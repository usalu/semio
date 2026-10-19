/** 📚️ The example offer, ownership and switching tests. */
import { SET_ACTIVE_EXAMPLE_ACTION_ID, appSwitchesExamples, appOffersRegisteredExamples, frameworkOwnsExampleSwitch, navbarExampleIdFromHistoryUpserts, rememberedExampleIdFromDispatchV1 } from "../../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts";
import exampleOfferFixture from "../../../🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/📚️example-offer/🔣️.json";
import exampleOfferSchema from "../../../🧱️elements/🐚️Shell/📚️example-offer/🔣️.json";
import Ajv2020 from "ajv/dist/2020";
import { expect, it } from "vitest";

/** 🔀 Registers the example offer, neutral ownership vector, switch admission and remembered example tests. */
export function registerExampleOffersAndSwitchingTests(): void {
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
    expect(rememberedExampleIdFromDispatchV1({ action: SET_ACTIVE_EXAMPLE_ACTION_ID, args: { exampleId: "" } }, remembered)).toBe("nakagin-capsule-tower");
    expect(rememberedExampleIdFromDispatchV1({ action: SET_ACTIVE_EXAMPLE_ACTION_ID }, remembered)).toBe("nakagin-capsule-tower");
    const popped = [{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, label: "Set Active Example", revertible: false }];
    const live = [{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, label: "Set Active Example", revertible: true }];
    expect(navbarExampleIdFromHistoryUpserts(popped, remembered, "concrete-forest")).toBe("concrete-forest");
    expect(navbarExampleIdFromHistoryUpserts(live, remembered, "concrete-forest")).toBe("nakagin-capsule-tower");
    expect(navbarExampleIdFromHistoryUpserts(live, "", "concrete-forest")).toBeUndefined();
  });
  it("navbar example id follows Set Active Example revertible, on the action id alone", () => {
    expect(navbarExampleIdFromHistoryUpserts([{ actionId: "shell.windowResize", revertible: false }], "nakagin", "forest")).toBeUndefined();
    expect(navbarExampleIdFromHistoryUpserts([{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, revertible: true }], "nakagin", "forest")).toBe("nakagin");
    expect(navbarExampleIdFromHistoryUpserts([{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, revertible: false }], "nakagin", "forest")).toBe("forest");
  });
}
