/** 🫧️ The leftover world overlay per window instance and scope tests. */
import { interactionViewFromLeftoverOutput, leftoverInteractionStateV1 } from "../../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🎮️leftover-views/🟦️.ts";
import { expect, it } from "vitest";
import { leftoverOverlayCarryingSelectionV1, leftoverSelectIdsMustNameHoverPickV1, leftoverOverlayCarryingUtilityV1, leftoverTreeItemSelectedV1, leftoverWorldOverlayAppliesV1, mergeWorldInteractionWithLeftoverV1, mergeWorldSelectionWithLeftoverV1 } from "../../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts";
import { leftoverInspectionPanelHash, leftoverInspectionRefreshScope, uiRefreshSectionUnchanged } from "../../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";

/** 🎛 Registers the per window instance overlay, equal republish, arm authority, mode tool and selection retirement tests. */
export function registerLeftoverOverlayScopeTests(): void {
  it("a republished equal leftover keeps every pane's overlay object and notifies nobody; one pane's hover changes only its own", async () => {
    const world = await import("../../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts");
    const [a, b] = ["puzzle3d#perspective", "puzzle3d#top"];
    const idle = { ids: ["seed-left-001"] as readonly string[], hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "select", activeToolId: null };
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
    world.publishLeftoverWorldSelectionV1(idle, { kind: "document" });
    world.publishLeftoverWorldSelectionV1({ ...idle, hoveredId: "seed-left-001:v0", hoveredDomain: "vortex" }, { kind: "window", windowId: a });
    const documentBefore = world.leftoverWorldSelectionOverlayV1();
    const aBefore = world.leftoverWorldWindowOverlayV1(a);
    const bBefore = world.leftoverWorldWindowOverlayV1(b);
    expect(world.leftoverWorldWindowOverlayV1(a), "a pane's overlay is one object between publications").toBe(aBefore);
    let notified = 0;
    const unsubscribe = world.subscribeLeftoverWorldSelectionV1(() => (notified += 1));
    world.publishLeftoverWorldSelectionV1({ ...idle, ids: [...idle.ids], hoveredId: "seed-left-001:v0", hoveredDomain: "vortex" }, { kind: "window", windowId: a });
    expect(notified, "an equal republication with a fresh ids array changes nothing").toBe(0);
    expect(world.leftoverWorldSelectionOverlayV1()).toBe(documentBefore);
    expect(world.leftoverWorldWindowOverlayV1(a)).toBe(aBefore);
    expect(world.leftoverWorldWindowOverlayV1(b)).toBe(bBefore);
    world.publishLeftoverWorldSelectionV1({ ...idle, hoveredId: null, hoveredDomain: null }, { kind: "window", windowId: a });
    expect(notified, "the hover leaving pane a is one change").toBe(1);
    expect(world.leftoverWorldWindowOverlayV1(a)?.hoveredId).toBeNull();
    expect(world.leftoverWorldWindowOverlayV1(a)).not.toBe(aBefore);
    expect(world.leftoverWorldWindowOverlayV1(b), "the sibling pane keeps its object, so it does not re-render").toBe(bBefore);
    expect(world.leftoverWorldSelectionOverlayV1(), "nor do the document readers").toBe(documentBefore);
    unsubscribe();
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
  });
  it("the first leftover a pane publishes never lends its hover or utility to the document slot, so a sibling pane without its own overlay reads none", async () => {
    const world = await import("../../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts");
    const [a, b] = ["puzzle3d#perspective", "puzzle3d#top"];
    const hovered = { ids: [] as readonly string[], hoveredId: "seed-left-001", hoveredDomain: "object", gumballActive: false, gumballAnchorId: null, activeUtility: "brush" };
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
    world.publishLeftoverWorldSelectionV1(hovered, { kind: "window", windowId: a });
    expect(world.leftoverWorldWindowOverlayV1(a)).toMatchObject({ hoveredId: "seed-left-001", activeUtility: "brush" });
    expect(world.leftoverWorldWindowOverlayV1(b), "the sibling pane paints no hover and no arm of pane a").toMatchObject({ hoveredId: null, hoveredDomain: null, activeUtility: null });
    expect(world.leftoverWorldSelectionOverlayV1()?.hoveredId, "nor do the document readers").toBeNull();
    world.publishLeftoverWorldSelectionV1({ ...hovered, hoveredId: null, hoveredDomain: null }, { kind: "window", windowId: a });
    expect(world.leftoverWorldWindowOverlayV1(b)?.hoveredId, "the hover leaving pane a leaves nothing behind in pane b").toBeNull();
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
  });
  it("one arm authority publishes the armed pane's own overlay through explicit window scope", async () => {
    const world = await import("../../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts");
    const a = "generation3d-preview#a";
    const b = "generation3d-preview#b";
    const documentSelection = { ids: ["solid-a"], hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "select" } as const;
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
    world.publishLeftoverWorldSelectionV1(documentSelection, { kind: "document" });
    world.publishLeftoverWorldSelectionV1({ ...documentSelection, activeUtility: "brush" }, { kind: "window", windowId: a });
    world.publishLeftoverWorldSelectionV1({ ...documentSelection, activeUtility: "volumeBrush" }, { kind: "window", windowId: b });
    expect(world.leftoverWorldWindowOverlayV1(a)).toMatchObject({ ids: ["solid-a"], activeUtility: "brush" });
    expect(world.leftoverWorldWindowOverlayV1(b)).toMatchObject({ ids: ["solid-a"], activeUtility: "volumeBrush" });
    expect(world.leftoverWorldSelectionOverlayV1()).toMatchObject({ ids: ["solid-a"], activeUtility: "select" });
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
    expect(world.leftoverWorldWindowOverlayV1(a)).toBeNull();
    expect(world.leftoverWorldWindowOverlayV1(b)).toBeNull();
  });
  it("carries an armed mode tool id across hover leftovers without overriding the published utility", () => {
    const leftover = { ids: [] as const, hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "select", activeToolId: "tool-a" };
    expect(leftoverWorldOverlayAppliesV1(leftover)).toBe(true);
    expect(mergeWorldInteractionWithLeftoverV1({ activeUtility: "brush" }, leftover).activeUtility).toBe("select");
    const hoverOnly = { ids: [] as const, hoveredId: "seed-left-001:v0", hoveredDomain: "vortex" as const, gumballActive: false, gumballAnchorId: null };
    const carried = leftoverOverlayCarryingUtilityV1(hoverOnly, leftover);
    expect(carried.activeToolId).toBe("tool-a");
    expect(mergeWorldInteractionWithLeftoverV1({}, carried).activeUtility).toBe("select");
  });
  it("first leftover pick keeps selection across hover leftover and busts the Inspection hash skip", () => {
    const hoverOnly = { ids: [] as const, hoveredId: "seed-left-001", hoveredDomain: "vortex" as const, gumballActive: false, gumballAnchorId: null };
    const priorPick = { ids: ["seed-left-001"], hoveredId: null, gumballActive: true, gumballAnchorId: "seed-left-001" };
    const carried = leftoverOverlayCarryingSelectionV1(hoverOnly, priorPick);
    expect(carried.ids).toEqual(["seed-left-001"]);
    const emptyNoHover = leftoverOverlayCarryingSelectionV1({ ids: [] as const, hoveredId: null, hoveredDomain: null, gumballActive: false, gumballAnchorId: null }, priorPick, true);
    expect(emptyNoHover.ids).toEqual([]);
    expect(mergeWorldSelectionWithLeftoverV1({ ids: ["seed-left-001"], activeObjectId: "seed-left-001", gumballActive: true }, { ids: [], hoveredId: null, gumballActive: false, gumballAnchorId: null, selectionCleared: true }).ids).toEqual([]);
    expect(leftoverTreeItemSelectedV1("seed-left-001", carried.ids)).toBe(true);
    expect(leftoverTreeItemSelectedV1("surface/seed-left-001", carried.ids)).toBe(true);
    expect(leftoverTreeItemSelectedV1("other", carried.ids)).toBe(false);
    const idsOnly = interactionViewFromLeftoverOutput({
      interactionView: {
        selectedIds: ["seed-left-001"],
        hoverTarget: null,
        locked: {},
        gumball: { active: true, anchorId: "seed-left-001" },
        selection: {},
        hover: {},
        activeMode: { vortex: "single" },
        activeGranularity: { vortex: "object" },
      },
    });
    expect(leftoverInteractionStateV1(idsOnly!).selection.vortex?.ids).toEqual(["seed-left-001"]);
    const emptyVortex = interactionViewFromLeftoverOutput({
      interactionView: {
        selectedIds: ["seed-left-001"],
        hoverTarget: null,
        locked: {},
        gumball: { active: true, anchorId: "seed-left-001" },
        selection: { vortex: { granularity: "object", ids: [] } },
        hover: {},
        activeMode: { vortex: "single" },
        activeGranularity: { vortex: "object" },
      },
    });
    expect(leftoverInteractionStateV1(emptyVortex!).selection.vortex?.ids).toEqual(["seed-left-001"]);
    expect(leftoverInspectionRefreshScope(["seed-left-001"])).toEqual({ kind: "full" });
    expect(leftoverInspectionPanelHash(["seed-left-001"], "abc")).toBeUndefined();
    expect(leftoverInspectionPanelHash([], "abc")).toBe("abc");
    expect(uiRefreshSectionUnchanged(leftoverInspectionPanelHash(["seed-left-001"], "abc"), { root: 0, hash: "abc" })).toBe(false);
    expect(uiRefreshSectionUnchanged("abc", { root: 0, hash: "abc" })).toBe(true);
  });
  it("empty-target interactionSelect clears selectedIds while hover may remain; hover leftover does not invent a pick", () => {
    const emptyWhileHover = leftoverSelectIdsMustNameHoverPickV1([], "seed-left-001");
    expect(emptyWhileHover).toBe(false);
    const published = interactionViewFromLeftoverOutput({
      interactionView: {
        selectedIds: ["seed-left-001"],
        hoverTarget: { id: "seed-left-001", domain: "vortex", channel: "pointer" },
        locked: {},
        gumball: { active: true, anchorId: "seed-left-001" },
        selection: {},
        hover: {},
        activeMode: { vortex: "single" },
        activeGranularity: { vortex: "object" },
      },
    });
    expect(published?.selectedIds).toEqual(["seed-left-001"]);
    expect(published?.hoverTarget?.id).toBe("seed-left-001");
    expect(leftoverSelectIdsMustNameHoverPickV1(published?.selectedIds, published?.hoverTarget?.id)).toBe(true);
    const hoverOnly = leftoverOverlayCarryingSelectionV1({ ids: [] as const, hoveredId: "seed-left-001", hoveredDomain: "vortex" as const, gumballActive: false, gumballAnchorId: null }, null);
    expect(hoverOnly.ids).toEqual([]);
    expect(leftoverSelectIdsMustNameHoverPickV1(hoverOnly.ids, hoverOnly.hoveredId)).toBe(false);
  });
  it("retires a stale selection overlay against the pane's own document, and keeps the pick it exists for", async () => {
    const world = await import("../../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts");
    const previewWindow = "generation3d-play-window-preview#1";
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
    const picked = interactionViewFromLeftoverOutput({
      interactionView: {
        selectedIds: ["shell@solid"],
        hoverTarget: null,
        locked: {},
        gumball: { active: false, anchorId: null },
        selection: { graph: { granularity: "object", ids: ["shell@solid"] } },
        hover: {},
        activeMode: {},
        activeGranularity: {},
        windowId: previewWindow,
      },
    });
    expect(picked?.selectedIds).toEqual(["shell@solid"]);
    world.publishLeftoverWorldSelectionV1(leftoverOverlayCarryingSelectionV1({ ids: picked!.selectedIds, hoveredId: null, hoveredDomain: null, gumballActive: false, gumballAnchorId: null }, null, false), { kind: "window", windowId: previewWindow });

    const boxShell = [{ id: "shell@solid#0", interactionId: "shell@solid", meshId: "eval-shell@solid#0" }];
    const guestSilent = { ids: [] as string[], activeObjectId: null };
    const covered = mergeWorldSelectionWithLeftoverV1(guestSilent, world.leftoverWorldWindowOverlayV1(previewWindow), boxShell);
    expect(covered.ids, "the cover the overlay exists for").toEqual(["shell@solid"]);
    expect(covered.activeObjectId).toBe("shell@solid#0");

    const sphereCut = [{ id: "brep_bool_cut_5@solid#0", interactionId: "brep_bool_cut_5@solid", meshId: "eval-brep_bool_cut_5@solid#0" }];
    expect(world.leftoverWorldOverlayIdsInDocumentV1(["shell@solid"], sphereCut)).toEqual([]);
    const afterSwitch = mergeWorldSelectionWithLeftoverV1(guestSilent, world.leftoverWorldWindowOverlayV1(previewWindow), sphereCut);
    expect(afterSwitch.ids, "a selection made in another document is not this pane's").toEqual([]);
    expect(afterSwitch.activeObjectId).toBeNull();
    const published = world.worldSurfaceSelectionDomV1(afterSwitch, mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, world.leftoverWorldWindowOverlayV1(previewWindow)));
    expect(published.selectedIds).toEqual([]);
    expect(published.activeObjectId).toBeNull();

    const repicked = leftoverOverlayCarryingSelectionV1({ ids: ["brep_bool_cut_5@solid", "shell@solid"], hoveredId: null, hoveredDomain: null, gumballActive: false, gumballAnchorId: null }, world.leftoverWorldWindowOverlayV1(previewWindow), false);
    world.publishLeftoverWorldSelectionV1(repicked, { kind: "window", windowId: previewWindow });
    const afterRepick = mergeWorldSelectionWithLeftoverV1(guestSilent, world.leftoverWorldWindowOverlayV1(previewWindow), sphereCut);
    expect(afterRepick.ids).toEqual(["brep_bool_cut_5@solid"]);
    expect(afterRepick.activeObjectId).toBe("brep_bool_cut_5@solid#0");

    expect(world.leftoverWorldOverlayIdsInDocumentV1(["shell@solid"], [])).toEqual(["shell@solid"]);
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
  });
}
