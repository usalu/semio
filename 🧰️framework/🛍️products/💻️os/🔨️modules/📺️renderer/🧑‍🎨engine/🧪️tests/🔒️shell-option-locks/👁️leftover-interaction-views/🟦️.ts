/** 👁️ The leftover interaction view publication tests. */
import { interactionViewFromLeftoverOutput, leftoverInteractionStateV1, leftoverWorldGumballPoseV1 } from "../../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🎮️leftover-views/🟦️.ts";
import { expect, it } from "vitest";
import { leftoverHoveredVortexFullIdV1, leftoverOverlayCarryingSelectionV1, leftoverOverlayCarryingUtilityV1, leftoverOverlayArmedBrushUtilityV1, leftoverWorldOverlayAppliesV1, mergeWorldInteractionWithLeftoverV1, mergeWorldSelectionWithLeftoverV1 } from "../../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts";
import { world3dGumballSelectionArgsV1 } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🪞 Registers the leftover interaction view publication, hover, utility and per pane window surface tests. */
export function registerLeftoverInteractionViewTests(): void {
  it("leftover InteractionView publication populates selection, lock, and gumball", () => {
    const published = interactionViewFromLeftoverOutput({
      interactionView: {
        selectedIds: ["seed-left-001"],
        hoverTarget: { domain: "vortex", channel: "pointer", id: "seed-left-001" },
        locked: { "seed-left-001": false },
        gumball: { active: true, anchorId: "seed-left-001" },
        selection: { vortex: { granularity: "object", ids: ["seed-left-001"] } },
        hover: { vortex: { channel: "pointer", ids: ["seed-left-001"] } },
        activeMode: { vortex: "single" },
        activeGranularity: { vortex: "object" },
      },
    });
    expect(published?.selectedIds).toEqual(["seed-left-001"]);
    expect(published?.locked["seed-left-001"]).toBe(false);
    expect(published?.gumballActive).toBe(true);
    expect(published?.gumballAnchorId).toBe("seed-left-001");
    expect(leftoverInteractionStateV1(published!).selection.vortex?.ids).toEqual(["seed-left-001"]);
    expect(interactionViewFromLeftoverOutput(null)).toBeNull();
    const selectOnly = interactionViewFromLeftoverOutput({
      interactionView: {
        selectedIds: ["seed-left-001"],
        hoverTarget: null,
        locked: {},
        gumball: { active: false, anchorId: null },
        selection: { vortex: { granularity: "object", ids: ["seed-left-001"] } },
        hover: {},
        activeMode: { vortex: "single" },
        activeGranularity: { vortex: "object" },
        activeUtility: "select",
      },
    });
    expect(selectOnly?.gumballActive).toBe(false);
    expect(leftoverWorldGumballPoseV1({ gumballActive: false, gumballAnchorId: null, ids: ["seed-left-001"] }, [{ id: "seed-left-001", position: [1, 2, 3] }]).transformMode).toBeUndefined();
    const pose = leftoverWorldGumballPoseV1({ gumballActive: true, gumballAnchorId: "seed-left-001", ids: ["seed-left-001"] }, [{ id: "seed-left-001", position: [1, 2, 3] }]);
    expect(pose.transformMode).toBe("transform");
    expect(pose.gumballTarget).toEqual([1, 2, 3]);
    const gumballArgs = world3dGumballSelectionArgsV1({ ids: ["seed-left-001"], componentIds: [9] });
    expect(gumballArgs.ids).toEqual(["seed-left-001"]);
    expect(gumballArgs.mode).toBe("object");
  });
  it("hover-only leftover InteractionView overlays vortex hover without selected ids", () => {
    const published = interactionViewFromLeftoverOutput({
      interactionView: {
        selectedIds: [],
        hoverTarget: { domain: "vortex", channel: "pointer", id: "seed-left-001:v0" },
        locked: {},
        gumball: { active: false, anchorId: null },
        selection: {},
        hover: { vortex: { channel: "pointer", ids: ["seed-left-001:v0"] } },
        activeMode: { vortex: "multiple" },
        activeGranularity: { vortex: "vortex" },
      },
    });
    expect(published?.hoverTarget).toEqual({ domain: "vortex", channel: "pointer", id: "seed-left-001:v0" });
    expect(leftoverWorldOverlayAppliesV1({ ids: published!.selectedIds, hoveredId: published!.hoverTarget?.id ?? null, hoveredDomain: published!.hoverTarget?.domain ?? null, gumballActive: false, gumballAnchorId: null })).toBe(true);
    expect(leftoverHoveredVortexFullIdV1({ hoveredId: published!.hoverTarget!.id, hoveredDomain: published!.hoverTarget!.domain })).toBe("seed-left-001:v0");
    expect(mergeWorldInteractionWithLeftoverV1({}, { ids: [], hoveredId: "seed-left-001:v0", hoveredDomain: "vortex", gumballActive: false, gumballAnchorId: null }).hoveredVortexFullId).toBe("seed-left-001:v0");
    expect(mergeWorldSelectionWithLeftoverV1({ method: "rectangle", ids: [] }, { ids: [], hoveredId: "seed-left-001:v0", hoveredDomain: "vortex", gumballActive: false, gumballAnchorId: null }).hoveredId).toBe("seed-left-001:v0");
    expect(leftoverHoveredVortexFullIdV1({ hoveredId: "seed-left-001", hoveredDomain: "vortex" })).toBeUndefined();
  });
  it("leftover activeUtility overlays guest select without a hover id", () => {
    const leftover = { ids: [] as const, hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "brush" };
    expect(leftoverWorldOverlayAppliesV1(leftover)).toBe(true);
    expect(mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, leftover).activeUtility).toBe("brush");
    expect(mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, { ids: [], hoveredId: null, gumballActive: false, gumballAnchorId: null }).activeUtility).toBe("select");
  });
  it("carries the armed utility across an InteractionView leftover republish", () => {
    const armed = { ids: [] as const, hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "brush" };
    const picked = { ids: ["seed-left-001"], hoveredId: "seed-left-001:v0", hoveredDomain: "vortex", gumballActive: true, gumballAnchorId: "seed-left-001" };
    const carried = leftoverOverlayCarryingUtilityV1(picked, armed);
    expect(carried.activeUtility).toBe("brush");
    expect(mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, carried).activeUtility).toBe("brush");
    expect(leftoverOverlayCarryingUtilityV1(picked, null).activeUtility).toBeNull();
    expect(leftoverOverlayArmedBrushUtilityV1("brush")).toBe(true);
    expect(leftoverOverlayArmedBrushUtilityV1("volumeBrush")).toBe(true);
    expect(leftoverOverlayArmedBrushUtilityV1("select")).toBe(false);
    expect(leftoverOverlayCarryingUtilityV1({ ...picked, activeUtility: "select" }, armed).activeUtility).toBe("brush");
    expect(mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, leftoverOverlayCarryingUtilityV1({ ...picked, activeUtility: "select" }, armed)).activeUtility).toBe("brush");
    expect(leftoverOverlayCarryingUtilityV1(picked, { ...armed, activeUtility: "select" }).activeUtility).toBe("select");
  });
  it("hover leftover select keeps an armed brush or volumeBrush", () => {
    const hoverSelect = { ids: [] as const, hoveredId: "seed-left-001:v0", hoveredDomain: "vortex" as const, gumballActive: false, gumballAnchorId: null, activeUtility: "select" };
    const brush = leftoverOverlayCarryingSelectionV1(hoverSelect, { ids: [] as const, hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "brush" });
    expect(brush.activeUtility).toBe("brush");
    expect(mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, brush).activeUtility).toBe("brush");
    const volume = leftoverOverlayCarryingSelectionV1(hoverSelect, { ids: [] as const, hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "volumeBrush" });
    expect(volume.activeUtility).toBe("volumeBrush");
    expect(mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, volume).activeUtility).toBe("volumeBrush");
    const idle = leftoverOverlayCarryingSelectionV1(hoverSelect, { ids: [] as const, hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "select" });
    expect(idle.activeUtility).toBe("select");
  });
  it("every pane of one document names its OWN window surface, and publishes the selection it paints", async () => {
    const { default: identity } = await import("../../../🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🪪️world-surface-identity.json");
    const { surfaceHostIdentityV1 } = await import("../../../🧱️elements/🗣️Interpreter/🔌️surface-bridge/🟦️.tsx");
    const { worldSurfaceSelectionDomV1, worldSurfaceGuestSelectionDomV1 } = await import("../../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts");

    const identities = identity.panes.map((pane) => surfaceHostIdentityV1(pane.documentSurface, pane.recordKey, pane.recordId));
    for (const [index, pane] of identity.panes.entries()) expect(identities[index], `pane ${pane.window}`).toEqual(pane.identity);
    expect(new Set(identities.map((one) => one.surfaceId)).size, "two panes of one document must not share one surface identity").toBe(identity.panes.length);
    expect(new Set(identity.panes.map((pane) => String(pane.recordId))).size, "the record id they used to be keyed on IS the same in both panes — that is the defect").toBe(1);
    expect(surfaceHostIdentityV1(identity.keylessPane.documentSurface, identity.keylessPane.recordKey, identity.keylessPane.recordId)).toEqual({ ...identity.keylessPane.identity, paneId: undefined });

    const publication = identity.selectionPublication;
    const painted = mergeWorldSelectionWithLeftoverV1(publication.guestSelectionJson as never, publication.leftoverOverlay as never, []);
    const interaction = mergeWorldInteractionWithLeftoverV1(publication.guestInteractionJson as never, publication.leftoverOverlay as never);
    expect(worldSurfaceSelectionDomV1(painted, interaction)).toEqual(publication.published);

    const objectHover = publication.objectHoverPublished;
    const objectPainted = mergeWorldSelectionWithLeftoverV1(publication.guestSelectionJson as never, objectHover.leftoverOverlay as never, []);
    const objectInteraction = mergeWorldInteractionWithLeftoverV1(publication.guestInteractionJson as never, objectHover.leftoverOverlay as never);
    const objectDom = worldSurfaceSelectionDomV1(objectPainted, objectInteraction);
    expect(objectDom.hoverTarget).toEqual(objectHover.hoverTarget);
    expect(objectDom.hoveredVortexFullId).toBe(objectHover.hoveredVortexFullId);
    expect(objectDom.activeUtility).toBe(objectHover.activeUtility);
    const guestPublished = worldSurfaceGuestSelectionDomV1(JSON.stringify(publication.guestSelectionJson));
    const { why: _guestWhy, ...guestExpected } = publication.guestPublished;
    expect(guestPublished, "the guest's own lane must be published UNMERGED, or a guest that lost the pick reads identically to one that kept it").toEqual(guestExpected);
    expect(guestPublished.selectedIds, "this fixture's guest sent nothing").toEqual([]);
    expect(publication.published.selectedIds, "while the pane paints the leftover pick — the two attributes are the discriminator").toEqual(["seed-left-001"]);
    const carried = publication.guestCarriedPick;
    const { why: _carriedWhy, ...carriedExpected } = { why: carried.why, ...carried.guestPublished };
    expect(worldSurfaceGuestSelectionDomV1(JSON.stringify(carried.guestSelectionJson)), "a guest that KEPT the pick must say so on its own lane").toEqual(carriedExpected);
  });
  it("the leftover world overlay is per window INSTANCE: arming one pane leaves its sibling's record untouched", async () => {
    const { default: identity } = await import("../../../🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🪪️world-surface-identity.json");
    const { surfaceHostIdentityV1 } = await import("../../../🧱️elements/🗣️Interpreter/🔌️surface-bridge/🟦️.tsx");
    const world = await import("../../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts");
    const [perspective, top] = identity.panes.map((pane) => pane.window);
    const surfaces = identity.panes.map((pane) => surfaceHostIdentityV1(pane.documentSurface, pane.recordKey, pane.recordId).surfaceId);
    expect(new Set(surfaces).size, "two panes of one document bind two records").toBe(2);

    const idle = { ids: [] as readonly string[], hoveredId: null, gumballActive: false, gumballAnchorId: null, activeUtility: "select", activeToolId: null };
    world.publishLeftoverWorldSelectionV1(idle, { kind: "allWindows" });
    world.publishLeftoverWorldSelectionV1({ ...idle, activeUtility: "brush", hoveredId: "seed-left-001:v0", hoveredDomain: "vortex" }, { kind: "window", windowId: perspective });

    const paneRecord = (windowId: string) => world.worldSurfaceSelectionDomV1({ method: "rectangle", ids: [] }, mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, world.leftoverWorldWindowOverlayV1(windowId)));
    expect(paneRecord(perspective).activeUtility, "the armed pane's own record").toBe("brush");
    expect(paneRecord(top).activeUtility, "its sibling must NOT read the arm back — one shared overlay is what made both panes publish one value").toBe("select");
    expect(paneRecord(perspective)).not.toEqual(paneRecord(top));
    expect(world.leftoverWorldWindowOverlayV1(top)?.hoveredId ?? null, "nor the armed pane's hover").toBeNull();
    expect(world.leftoverWorldArmedWindowOverlayV1()?.activeUtility, "the host still finds the one armed pane").toBe("brush");

    world.publishLeftoverWorldSelectionV1({ ...idle, ids: ["seed-left-001"], gumballActive: true, gumballAnchorId: "seed-left-001" }, { kind: "document" });
    expect(paneRecord(perspective).activeUtility, "a windowless leftover carries the document selection, never another pane's utility").toBe("brush");
    expect(paneRecord(perspective).selectedIds, "while the document's selection reaches every pane").toEqual([]);
    expect(world.leftoverWorldWindowOverlayV1(top)?.ids).toEqual(["seed-left-001"]);
    expect(world.leftoverWorldSelectionOverlayV1()?.ids, "the document-scoped readers (Outliner/Inspection) see it too").toEqual(["seed-left-001"]);

    world.publishLeftoverWorldSelectionV1({ ...idle, activeUtility: "fill", activeToolId: "fill" }, { kind: "allWindows" });
    expect(paneRecord(perspective).activeUtility, "a mode-level tool is the ONE authority that speaks for every pane").toBe("fill");
    expect(paneRecord(top).activeUtility).toBe("fill");
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
    expect(world.leftoverWorldWindowOverlayV1(perspective)).toBeNull();
  });
}
