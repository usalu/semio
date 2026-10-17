/** 🔒 Shell option locks. */
import { shouldStartIntroduction } from "../../../../../../../🔨️modules/🖱️ui/🎓️introduction/🟦️.ts";
import {
  SET_ACTIVE_EXAMPLE_ACTION_ID,
  appSwitchesExamples,
  appOffersRegisteredExamples,
  frameworkOwnsExampleSwitch,
  buildActiveExampleAction,
  navbarExampleIdFromHistoryUpserts,
  rememberedExampleIdFromDispatchV1
} from "../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🔀️example-switching/🟦️.ts";
import {
  interactionViewFromLeftoverOutput,
  leftoverInteractionStateV1,
  leftoverWorldGumballPoseV1
} from "../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🎮️leftover-views/🟦️.ts";
import { FRAMEWORK_HISTORY_BODY_KEY, resolveUiDirtyScope } from "@semio-tech/framework";
import exampleOfferFixture from "../../🧱️elements/🐚️Shell/🧪️tests/🧫️fixtures/📚️example-offer/🔣️.json";
import exampleOfferSchema from "../../🧱️elements/🐚️Shell/📚️example-offer/🔣️.json";
import Ajv2020 from "ajv/dist/2020";
import { manifestExamplePickerFixture as examplePickerFixture } from "../../../../../../../🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/🟦️.ts";
import { examplesForApp, examplesForDialect, surfaceAppId, type AppRole } from "@semio-tech/framework";
import { readModuleTreeSource, sliceModuleTreeSource } from "../../../../../../../🔨️modules/📚️compiler/📖️syntax/🌳️module-tree/🟦️.ts";
import { describe, expect, it, vi } from "vitest";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { leftoverHoveredVortexFullIdV1, leftoverOverlayCarryingSelectionV1, leftoverSelectIdsMustNameHoverPickV1, leftoverOverlayCarryingUtilityV1, leftoverOverlayArmedBrushUtilityV1, leftoverTreeItemSelectedV1, leftoverWorldOverlayAppliesV1, mergeWorldInteractionWithLeftoverV1, mergeWorldSelectionWithLeftoverV1 } from "../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts";
import { leftoverInspectionPanelHash, leftoverInspectionRefreshScope, uiRefreshSectionUnchanged } from "../../🧱️elements/🔌️PluginRuntime/🟦️.tsx";
import { initialShellState, world3dGumballSelectionArgsV1, buildOsCommands, dispatchOsCommand, mergeShellLockSources, resolveBootExampleId, resolveShellDefaults, resolveShellLocks, shouldPersistIntroductionSeen, shouldReplayIntroductionOnLoad, isEphemeralShellBrand, clearDurableShellStorage } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import {
  browserActorDispatchUiScopeV1,
  browserActorWindowConfigDispatchUiScopeV1,
  hostEffectRefreshScopeV1,
  typedOperationCompletionRefreshV1
} from "../../🧱️elements/🛠️ShellHelpers/🔄️ui-refresh/🟦️.ts";
import { existsSync } from "node:fs";
import "../🧰️engine-contract-helpers/🟦️.ts";

describe("shell option locks (SEMIO_LOCKED_*)", () => {
  it("resolves valid locale/appearance and falls back with a warning on invalid values while staying locked", () => {
    const warn = vi.spyOn(console, "warn").mockImplementation(() => {});
    expect(resolveShellLocks({ locale: "de" })).toMatchObject({ locale: "de" });
    expect(resolveShellLocks({ locale: "fr" })).toMatchObject({ locale: "en" });
    expect(resolveShellLocks({ appearance: "dark" })).toMatchObject({ appearance: "dark" });
    expect(resolveShellLocks({ appearance: "bogus" })).toMatchObject({ appearance: "system" });
    expect(warn).toHaveBeenCalled();
    warn.mockRestore();
  });

  it("accepts any non-empty terminology id verbatim (app-declared ids can't be validated at boot)", () => {
    expect(resolveShellLocks({ terminology: "reuse" })).toMatchObject({ terminology: "reuse" });
    expect(resolveShellLocks({ terminology: "some-app-declared-id" })).toMatchObject({ terminology: "some-app-declared-id" });
    expect(resolveShellLocks({ terminology: "" })).toEqual({});
  });

  it("returns an empty object for undefined locks", () => {
    expect(resolveShellLocks(undefined)).toEqual({});
  });

  it("initialShellState applies locked values over stored/default prefs", () => {
    const state = initialShellState({ locale: "en", plugins: [], locks: { exampleId: "concrete-forest", locale: "de", terminology: "reuse", themeId: "semio", appearance: "dark" }, storage: createMemoryStoragePort() });
    expect(state.layout.activeExampleId).toBe("concrete-forest");
    expect(state.uiPrefs.uiLocale).toBe("de");
    expect(state.uiPrefs.uiTerminology).toBe("reuse");
    expect(state.uiPrefs.uiThemeId).toBe("semio");
    expect(state.uiPrefs.uiAppearance).toBe("dark");
  });

  it("mergeShellLockSources keeps brand locks locked and lets a defined env lock win per key", () => {
    expect(mergeShellLockSources({ locale: "de", terminology: "reuse" }, { locale: "en", themeId: undefined })).toEqual({ locale: "en", terminology: "reuse" });
    expect(mergeShellLockSources({ locale: "de" }, undefined)).toEqual({ locale: "de" });
    expect(mergeShellLockSources(undefined, { locale: "en" })).toEqual({ locale: "en" });
    expect(mergeShellLockSources(undefined, undefined)).toBeUndefined();
  });

  it("resolveShellDefaults prefers env defaults over brand defaults and initialShellState seeds without locking", () => {
    const brand = { id: "entwerfen-mit-bestand-aggregator", windowTitle: "Entwerfen mit Bestand · Aggregator", defaults: { exampleId: "concrete-forest" } };
    expect(resolveShellDefaults(brand, { exampleId: "nakagin-capsule-tower" })).toEqual({ exampleId: "nakagin-capsule-tower" });
    expect(resolveShellDefaults(brand, undefined)).toEqual({ exampleId: "concrete-forest" });
    expect(resolveShellDefaults(undefined, undefined)).toEqual({ exampleId: undefined });
    const state = initialShellState({ locale: "en", plugins: [], defaults: { exampleId: "concrete-forest" }, storage: createMemoryStoragePort() });
    expect(state.layout.activeExampleId).toBe("concrete-forest");
    const locked = initialShellState({ locale: "en", plugins: [], locks: { exampleId: "nakagin-capsule-tower" }, defaults: { exampleId: "concrete-forest" }, storage: createMemoryStoragePort() });
    expect(locked.layout.activeExampleId).toBe("nakagin-capsule-tower");
  });

  it("resolveBootExampleId seeds the first registered example when nothing is active or defaulted", () => {
    const options = [{ id: "hexagonal-mushroom-column" }, { id: "rectangle-extrude-volume" }, { id: "sphere-cut-with-torus" }];
    expect(resolveBootExampleId("", options)).toBe("hexagonal-mushroom-column");
    expect(resolveBootExampleId("", options, "sphere-cut-with-torus")).toBe("sphere-cut-with-torus");
    expect(resolveBootExampleId("rectangle-extrude-volume", options, "sphere-cut-with-torus")).toBe("rectangle-extrude-volume");
    expect(resolveBootExampleId("missing", options)).toBe("hexagonal-mushroom-column");
    expect(resolveBootExampleId("", [])).toBe("");
  });

  // 🎨️ The REDO half of the navbar label, measured red on :6013 in wave B26: undo relabelled the picker
  // (a popped `Set Active Example` row answers with the boot example, which needs no memory) and redo
  // silently could not, because the only writer of the remembered id was `NavbarExampleSelect`'s own
  // `onValueChange` — `navbar example from history {"navbarExample":"concrete-forest","remembered":""}`.
  // Every dispatch of the verb now teaches the memory, so a row redone from anywhere relabels.
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
    // ↩️ undo pops the row → the boot example; ↪️ redo makes it live again → exactly what was remembered.
    const popped = [{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, label: "Set Active Example", revertible: false }];
    const live = [{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, label: "Set Active Example", revertible: true }];
    expect(navbarExampleIdFromHistoryUpserts(popped, remembered, "concrete-forest")).toBe("concrete-forest");
    expect(navbarExampleIdFromHistoryUpserts(live, remembered, "concrete-forest")).toBe("nakagin-capsule-tower");
    expect(navbarExampleIdFromHistoryUpserts(live, "", "concrete-forest")).toBeUndefined();
  });

  it("navbar example id follows Set Active Example revertible, on the action id alone", () => {
    // 🪪️ The row is recognised by its action id, never by its label's English prose — a history
    // label is a locale matrix now, so a German shell must reach the same verdict. Ticket 26/09/18 U3.
    expect(navbarExampleIdFromHistoryUpserts([{ actionId: "shell.windowResize", revertible: false }], "nakagin", "forest")).toBeUndefined();
    expect(navbarExampleIdFromHistoryUpserts([{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, revertible: true }], "nakagin", "forest")).toBe("nakagin");
    expect(navbarExampleIdFromHistoryUpserts([{ actionId: SET_ACTIVE_EXAMPLE_ACTION_ID, revertible: false }], "nakagin", "forest")).toBe("forest");
  });

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
    const { default: identity } = await import("../../🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🪪️world-surface-identity.json");
    const { surfaceHostIdentityV1 } = await import("../../🧱️elements/🗣️Interpreter/🔌️surface-bridge/🟦️.tsx");
    const { worldSurfaceSelectionDomV1, worldSurfaceGuestSelectionDomV1 } = await import("../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts");

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
    const { default: identity } = await import("../../🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🪪️world-surface-identity.json");
    const { surfaceHostIdentityV1 } = await import("../../🧱️elements/🗣️Interpreter/🔌️surface-bridge/🟦️.tsx");
    const world = await import("../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts");
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

  it("a republished equal leftover keeps every pane's overlay object and notifies nobody; one pane's hover changes only its own", async () => {
    const world = await import("../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts");
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
    const world = await import("../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts");
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
    const world = await import("../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts");
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
    // 🧹️ Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, lane `selection-prune-interact`. Loading another
    // bundled example replaces the whole document and the guest's own lane goes correctly empty, but
    // the host's leftover overlay is written only by the pick route and no document-replacing command
    // can retire it — a job-routed command answers through `TypedOperationCompletion`, which has no
    // leftover `output` field. Measured on generation3d 2026-09-14: `data-guest-selection-json` `[]`
    // against `data-selection-json` `["shell@solid"]` for six of eight examples, and the next plain
    // REPLACE click then read back `["fuse@solid","shell@solid"]`.
    const world = await import("../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts");
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

    // 🎯️ The pick's OWN document: the overlay is exactly the cover it exists for, so it must stand
    // even though the guest's lane has not answered yet.
    const boxShell = [{ id: "shell@solid#0", interactionId: "shell@solid", meshId: "eval-shell@solid#0" }];
    const guestSilent = { ids: [] as string[], activeObjectId: null };
    const covered = mergeWorldSelectionWithLeftoverV1(guestSilent, world.leftoverWorldWindowOverlayV1(previewWindow), boxShell);
    expect(covered.ids, "the cover the overlay exists for").toEqual(["shell@solid"]);
    expect(covered.activeObjectId).toBe("shell@solid#0");

    // 🧹️ The NEXT example's document offers none of those ids, so the same overlay is covering nothing.
    const sphereCut = [{ id: "brep_bool_cut_5@solid#0", interactionId: "brep_bool_cut_5@solid", meshId: "eval-brep_bool_cut_5@solid#0" }];
    expect(world.leftoverWorldOverlayIdsInDocumentV1(["shell@solid"], sphereCut)).toEqual([]);
    const afterSwitch = mergeWorldSelectionWithLeftoverV1(guestSilent, world.leftoverWorldWindowOverlayV1(previewWindow), sphereCut);
    expect(afterSwitch.ids, "a selection made in another document is not this pane's").toEqual([]);
    expect(afterSwitch.activeObjectId).toBeNull();
    const published = world.worldSurfaceSelectionDomV1(afterSwitch, mergeWorldInteractionWithLeftoverV1({ activeUtility: "select" }, world.leftoverWorldWindowOverlayV1(previewWindow)));
    expect(published.selectedIds).toEqual([]);
    expect(published.activeObjectId).toBeNull();

    // 🎯️ A plain click REPLACES: the new pick is the whole selection, the dead id does not ride along.
    const repicked = leftoverOverlayCarryingSelectionV1({ ids: ["brep_bool_cut_5@solid", "shell@solid"], hoveredId: null, hoveredDomain: null, gumballActive: false, gumballAnchorId: null }, world.leftoverWorldWindowOverlayV1(previewWindow), false);
    world.publishLeftoverWorldSelectionV1(repicked, { kind: "window", windowId: previewWindow });
    const afterRepick = mergeWorldSelectionWithLeftoverV1(guestSilent, world.leftoverWorldWindowOverlayV1(previewWindow), sphereCut);
    expect(afterRepick.ids).toEqual(["brep_bool_cut_5@solid"]);
    expect(afterRepick.activeObjectId).toBe("brep_bool_cut_5@solid#0");

    // 🕳️ A pane that has drawn nothing yet has no membership information and keeps every id, the same
    // rule the guest's topology pruning uses for a domain it has no entry for.
    expect(world.leftoverWorldOverlayIdsInDocumentV1(["shell@solid"], [])).toEqual(["shell@solid"]);
    world.publishLeftoverWorldSelectionV1(null, { kind: "allWindows" });
  });

  it("dirties the whole shell when a direct browser-actor dispatch applied a mutation", () => {
    // 🖼️ Wave B9 lane 4: the actor handoff answers `{outcome, mutationCount}` and carries no
    // `UiDirtyScope`, and the verbs that commit inline on it (`paste`, `undo`, a gumball commit) publish
    // no `OperationCompleted` frame either — so an applied mutation must dirty the shell on the strength
    // of `mutationCount` alone, or the world lane repaints only when some later action happens to.
    expect(browserActorDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 1 })).toEqual({ kind: "full" });
    expect(browserActorDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 7 })).toEqual({ kind: "full" });
    expect(browserActorDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 })).toEqual({ kind: "none" });
    expect(browserActorDispatchUiScopeV1({ outcome: "rejected", mutationCount: 3 })).toEqual({ kind: "none" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 }, "setGridVisible")).toEqual({ kind: "full" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 }, "setGridSpacing")).toEqual({ kind: "full" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 }, "setVortexShow")).toEqual({ kind: "full" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 }, "worldPointerDown")).toEqual({ kind: "none" });
    expect(browserActorWindowConfigDispatchUiScopeV1({ outcome: "rejected", mutationCount: 0 }, "setGridVisible")).toEqual({ kind: "none" });
  });

  it("refreshes a typed operation on its completion's own scope and pays nothing for a completion that dirtied nothing", () => {
    // 🏁️ Wave B27 §2. Every typed operation is merely ADMITTED on the reply that carries `mutationCount`
    // — its edit publishes and logs on later continuations — so `browserActorDispatchUiScopeV1` above
    // answers `none` for all of them and the refresh has to follow the COMPLETION frame instead. What the
    // completion carries is the mutation's own scope, verbatim; a history patch with an empty scope still
    // owes the reserved history body its row (wave B21 put the patch on the terminal completion).
    const admission = browserActorDispatchUiScopeV1({ outcome: "guest-applied", mutationCount: 0 });
    expect(admission).toEqual({ kind: "none" });
    const documentScope = { kind: "partial", panelBodies: ["puzzle.3d.play.artifact", "puzzle.3d.play.inspector"], measures: true } as const;
    expect(typedOperationCompletionRefreshV1({ uiScope: documentScope, historyPatch: { cursor: 3, upserts: [{ seq: 3 }] }, requestedEffects: [] })).toEqual(documentScope);
    expect(typedOperationCompletionRefreshV1({ uiScope: undefined, historyPatch: undefined, requestedEffects: [] })).toEqual({ kind: "full" });
    expect(typedOperationCompletionRefreshV1({ uiScope: { kind: "none" }, historyPatch: { cursor: 4, upserts: [{ seq: 4 }] }, requestedEffects: [] })).toEqual({
      kind: "partial",
      panelBodies: [FRAMEWORK_HISTORY_BODY_KEY],
    });
    // 🫥️ …and the other half: a retained operation completes on every drain poll, measured live on the
    // puzzle 3d shell at 18 completions per idle 10 s, 12 of them empty. Those owe NO pass — paying for
    // them is a refresh storm on View-kind completions. An effect still buys the effect pass, with a
    // `none` scope so nothing is re-taken for it.
    expect(typedOperationCompletionRefreshV1({ uiScope: { kind: "none" }, historyPatch: undefined, requestedEffects: [] })).toBeNull();
    expect(typedOperationCompletionRefreshV1({ uiScope: { kind: "none" }, historyPatch: undefined, requestedEffects: ["requestSync"] })).toEqual({ kind: "none" });
  });

  it("makes an effect that rewrites a guest render input earn its own pass, and leaves host chrome paying nothing", () => {
    // 🧰️ Wave B37, the measured defect. `engagementSubmit`'s typed `brush` verb dirties NOTHING in the
    // guest — it arms `scene.active_utility`, whose only carrier to the next render is the host's own
    // utility map — so the completion declares `none`, `typedOperationCompletionRefreshV1` forwards that
    // `none` (correctly), and `refreshUi({kind:"none"})` asks for nothing at all. Live console on :6013,
    // wasm #53: `engagementSubmit … effects:1` → the host armed `puzzle3d-main-perspective → brush` →
    // the next `refresh-ui` crossing was 30 s later on an unrelated keystroke, with BOTH world panes
    // publishing `activeUtility=select` for the whole interval (`engagement-brush-verb FAIL
    // activeUtility=select waitedMs=30347`).
    const bodyKeys = ["puzzle.3d.play.viewport"] as const;
    const declared = typedOperationCompletionRefreshV1({ uiScope: { kind: "none" }, historyPatch: undefined, requestedEffects: [{ setActiveUtility: { windowId: "puzzle3d-main-perspective", utilityId: "brush" } }] });
    expect(declared, "the completion's own scope is still what the GUEST dirtied").toEqual({ kind: "none" });
    const earned = hostEffectRefreshScopeV1([{ setActiveUtility: { windowId: "puzzle3d-main-perspective", utilityId: "brush" } }], declared!, bodyKeys);
    expect(earned, "arming a utility is a host-owned render input — the pass that applies it owes the render that publishes it").toEqual({
      kind: "partial",
      windowBodies: ["puzzle.3d.play.viewport"],
      panelBodies: [],
      utilities: true,
      tools: true,
      engagements: false,
      measures: true,
      labels: false,
    });
    expect(hostEffectRefreshScopeV1([{ setActiveTool: { toolId: "fill" } }], { kind: "none" }, bodyKeys), "a programmatic tool switch is the same shape as a utility arm").toEqual(earned);
    expect(hostEffectRefreshScopeV1([{ setPanel: { panelJson: "{}" } }], { kind: "none" }, bodyKeys), "`setPanel` names no body of its own — `panelJson` feeds every section").toEqual({ kind: "full" });

    // 🫥️ …and the no-storm half, which is why this is a filter and not `{kind:"full"}` for every effect
    // pass: a `notify` banner is host React state, a clipboard write never reaches a projection, and a
    // `dispatchAction` comes back through shell dispatch carrying its own scope. None of them may upgrade
    // a `none` completion into a pass — a retained operation completes on every drain poll (18 per idle
    // 10 s, wave B27).
    expect(hostEffectRefreshScopeV1([{ notify: { message: "refused" } }], { kind: "none" }, bodyKeys)).toEqual({ kind: "none" });
    expect(hostEffectRefreshScopeV1([{ clipboardWrite: { text: "x" } }, { dispatchAction: { actionId: "x" } }, "requestSync"], { kind: "none" }, bodyKeys)).toEqual({ kind: "none" });
    const documentScope = { kind: "partial", panelBodies: ["puzzle.3d.play.artifact"], measures: true } as const;
    expect(hostEffectRefreshScopeV1([{ notify: { message: "done" } }], documentScope, bodyKeys), "an effect that earns nothing must leave a declared scope exactly as it is").toBe(documentScope);
    expect(hostEffectRefreshScopeV1([{ setActiveUtility: { windowId: "w", utilityId: "brush" } }], documentScope, bodyKeys), "the earned scope is UNIONED with the declared one, never substituted for it").toEqual({
      kind: "partial",
      windowBodies: ["puzzle.3d.play.viewport"],
      panelBodies: ["puzzle.3d.play.artifact"],
      utilities: true,
      tools: true,
      engagements: false,
      measures: true,
      labels: false,
    });
  });

  /** 🔁️ A pass's OWED effects are applied under the scope that pass declared — never a wider one.
   *
   * 🐛️ `ShellHost`'s ui-refresh lane handed `{ kind: "full" }` to `applyHostEffects` for every owed
   * application, so each React pick paid a whole extra full guest re-render on top of the narrowed
   * scope the interaction had just derived: `[TRACE] refreshUi lane {"decision":"owed","scope":{"kind":"full"}}`,
   * four of them per pick, which is why narrowing the pick's own scope moved its wall not at all
   * (`📓️interaction-scope-narrowing-2026-09-15.md` §5.3, whose owner this law names). The effects a
   * pass owes are `pending_effects` — `dispatchAction` shapes — and this proves they earn NOTHING of
   * their own, so `full` was the hardcode and not a derivation.
   */
  it("applies a pass's owed effects under that pass's own scope, never a hardcoded full one", () => {
    const bodyKeys = ["procedural.play.main", "procedural.play.preview", "procedural.play.generate-preview"] as const;
    const passScope = { kind: "partial", windowBodies: ["procedural.play.main", "procedural.play.preview"], panelBodies: [], utilities: false, tools: false, engagements: false, measures: false, labels: false } as const;
    const owedEffects = [{ dispatchAction: { req: 103, action: "flowEvalTick", args: {}, delayMs: 0 } }, { dispatchAction: { req: 104, action: "toolRunStart", args: { toolId: "previewEval" }, delayMs: 0 } }];
    expect(hostEffectRefreshScopeV1(owedEffects, passScope, bodyKeys), "a pass's owed pending_effects earn no scope of their own").toBe(passScope);
    expect(hostEffectRefreshScopeV1(owedEffects, { kind: "full" }, bodyKeys), "so the only thing that ever made the owed pass full was the caller").toEqual({ kind: "full" });

    const shellHostRelative = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx";
    let shellHostRoot = process.cwd();
    for (let hop = 0; hop < 12 && !existsSync(`${shellHostRoot}/${shellHostRelative}`); hop += 1) shellHostRoot = `${shellHostRoot}/..`;
    expect(existsSync(`${shellHostRoot}/${shellHostRelative}`)).toBe(true);
    const shellHost = readModuleTreeSource(`${shellHostRoot}/${shellHostRelative}`);
    const lane = sliceModuleTreeSource(`${shellHostRoot}/${shellHostRelative}`, "uiRefreshLaneRef.current = createUiRefreshCoalescerV1", "const refreshUi = useCallback");
    expect(lane, "the lane applies the scope the pass recorded").toContain("applyHostEffectsRef.current(owedEffects.effects, owedEffects.session, owedEffects.scope, owedEffects.owner)");
    expect(lane.includes('{ kind: "full" }'), "and names no scope of its own").toBe(false);
    expect(shellHost, "the owed slot records the pass's scope when it records the effects").toContain("owedPassEffectsRef.current = { effects: pendingRefreshEffects, session: nextSession, owner: refreshOwner, scope: scopeArg }");
  });

  // 📚️ The language-agnostic half of the picker: which examples a surface may offer at all. Rust
  // `manifest::examples_for_dialect` answers the SAME rows in
  // `🛂️manifest/🧪️tests/🔬️example-picker/🦀️.rs` (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
  it("resolves the example picker by dialect, so an editor and its viewer offer exactly the same examples", () => {
    
    
    for (const useCase of examplePickerFixture.cases) {
      expect(surfaceAppId(useCase.app.dialect, useCase.app.role as AppRole), useCase.name).toBe(useCase.app.id);
      expect(
        examplesForDialect(examplePickerFixture.examples, useCase.app.dialect).map((example) => example.id),
        useCase.name,
      ).toEqual(useCase.expected);
      expect(
        examplesForApp(examplePickerFixture.examples, useCase.app).map((example) => example.id),
        useCase.name,
      ).toEqual(useCase.expected);
    }
    const editor = examplePickerFixture.cases.find((useCase) => useCase.name === "editor-of-the-dialect-offers-every-example-of-it")!;
    const viewer = examplePickerFixture.cases.find((useCase) => useCase.name === "viewer-of-the-same-dialect-offers-exactly-the-same-picker")!;
    expect(viewer.expected).toEqual(editor.expected);
    expect(viewer.app.id).not.toBe(editor.app.id);
    // 🧭️ An app whose dialect is absent (a pre-registration stub) resolves no picker rather than the
    // whole manifest — the empty-picker branch `ShellHost`'s `exampleOptions` relies on.
    expect(examplesForApp(examplePickerFixture.examples, {})).toEqual([]);
  });

  it("the example picker dispatches the chosen example id and its completion refreshes the whole shell", () => {
    // 🎨️ The picker writes the LABEL through `SET_ACTIVE_EXAMPLE_ID` and reaches the program through
    // exactly one descriptor — boot #11 changed the label while the flow window kept the previous
    // fixture, so both halves are pinned here (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    expect(buildActiveExampleAction("s.procedural.generation3d@1/*#editor", "box-shell-preview")).toEqual({
      controllerId: "s.procedural.generation3d@1/*#editor",
      action: SET_ACTIVE_EXAMPLE_ACTION_ID,
      args: { exampleId: "box-shell-preview" },
    });
    // 👁️ The read-only surface takes the exact same descriptor, addressed at ITS controller — the
    // viewer declares `setActiveExample` as a `View` action and loads the example onto its own
    // config lane (ticket 26/09/09/PROCEDURAL-3D-END-TO-END).
    expect(buildActiveExampleAction("s.procedural.generation3d@1/*#viewer", "box-shell-preview")).toEqual({
      controllerId: "s.procedural.generation3d@1/*#viewer",
      action: SET_ACTIVE_EXAMPLE_ACTION_ID,
      args: { exampleId: "box-shell-preview" },
    });
    expect(buildActiveExampleAction("c", "")).toEqual({ controllerId: "c", action: "setActiveExample", args: { exampleId: "" } });
    // 🏁️ `setActiveExample` is a retained typed operation: `handleAction` only admits it, and the shell's
    // `subscribeOperationCompletions` handler is the ONLY carrier of its outcome — it must resolve the
    // completion's own scope, and an absent scope must still repaint every surface.
    expect(resolveUiDirtyScope({ kind: "full" })).toEqual({ kind: "full" });
    expect(resolveUiDirtyScope(undefined)).toEqual({ kind: "full" });
    expect(resolveUiDirtyScope({ kind: "none" })).toEqual({ kind: "none" });
  });

  it("shouldReplayIntroductionOnLoad opts a brand into replaying its tour after every window refresh", () => {
    expect(shouldReplayIntroductionOnLoad(undefined)).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "plain", windowTitle: "Plain" })).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "plain", windowTitle: "Plain", replayIntroductionOnLoad: false })).toBe(false);
    expect(shouldReplayIntroductionOnLoad({ id: "entwerfen-mit-bestand-aggregator", windowTitle: "Entwerfen mit Bestand · Aggregator", replayIntroductionOnLoad: true })).toBe(true);
    expect(shouldPersistIntroductionSeen({ id: "plain", windowTitle: "Plain" })).toBe(true);
    expect(shouldPersistIntroductionSeen({ id: "entwerfen-mit-bestand-aggregator", windowTitle: "Entwerfen mit Bestand · Aggregator", replayIntroductionOnLoad: true })).toBe(false);
  });

  it("shouldStartIntroduction offers an app's tour once per session and never re-arms a veil the user dismissed", () => {
    const base = { appId: "puzzle-3d-play", hasIntroduction: true, tutorialActive: false, suppressed: false, replayOnLoad: false, seenOnDevice: false, dismissedInSession: false };
    expect(shouldStartIntroduction(base)).toBe(true);
    // 🎓️ The blocking veil owns every pointer in the app, so a dismissed tour must stay dismissed for the
    // session even though the app definition is republished (a new object, same content) on every full
    // refresh, hot-swap and re-established session — B47 §5.1 measured a full-viewport
    // `div.ui-veil.z-tutorial` at `pointer-events: auto` swallowing every press after Skip.
    const dismissedAppIds = new Set(["puzzle-3d-play"]);
    const dismissed = { ...base, dismissedInSession: dismissedAppIds.has(base.appId) };
    expect(shouldStartIntroduction(dismissed)).toBe(false);
    // …and a device-local seen flag that cannot be written (ephemeral brand ⇒ in-memory StoragePort ⇒
    // `replayOnLoad`) must not resurrect it either.
    expect(shouldStartIntroduction({ ...dismissed, replayOnLoad: true })).toBe(false);
    // A genuinely DIFFERENT app still introduces itself in the same shell (demonstrator grid, app switch).
    expect(shouldStartIntroduction({ ...dismissed, appId: "puzzle-3d-view", dismissedInSession: dismissedAppIds.has("puzzle-3d-view") })).toBe(true);
    // The device-local seen flag keeps its own meaning, and replay-on-load brands still ignore it.
    expect(shouldStartIntroduction({ ...base, seenOnDevice: true })).toBe(false);
    expect(shouldStartIntroduction({ ...base, seenOnDevice: true, replayOnLoad: true })).toBe(true);
    // An app id that has not arrived yet must never arm a tour: the "" key persists nothing, so the tour
    // would arm a second time the moment the real id lands.
    expect(shouldStartIntroduction({ ...base, appId: "" })).toBe(false);
    expect(shouldStartIntroduction({ ...base, hasIntroduction: false })).toBe(false);
    expect(shouldStartIntroduction({ ...base, tutorialActive: true })).toBe(false);
    expect(shouldStartIntroduction({ ...base, suppressed: true })).toBe(false);
  });

  
  it("shouldStartIntroduction offers an app's tour once per session and never re-arms a veil the user dismissed", () => {
    const base = { appId: "puzzle-3d-play", hasIntroduction: true, tutorialActive: false, suppressed: false, replayOnLoad: false, seenOnDevice: false, dismissedInSession:false };
    expect(shouldStartIntroduction(base)).toBe(true);
    // 🎓️ The blocking veil owns every pointer in the app, so a dismissed tour must stay dismissed for the
    // session even though the app definition is republished (a new object, same content) on every full
    // refresh, hot-swap and re-established session — B47 §5.1 measured a full-viewport
    // `div.ui-veil.z-tutorial` at `pointer-events: auto` swallowing every press after Skip.
    const dismissed = { ...base, dismissedInSession:true };
    expect(shouldStartIntroduction(dismissed)).toBe(false);
    // …and a device-local seen flag that cannot be written (ephemeral brand ⇒ in-memory StoragePort ⇒
    // `replayOnLoad`) must not resurrect it either.
    expect(shouldStartIntroduction({ ...dismissed, replayOnLoad: true })).toBe(false);
    // A genuinely DIFFERENT app still introduces itself in the same shell (demonstrator grid, app switch).
    expect(shouldStartIntroduction({ ...dismissed, appId: "puzzle-3d-view", dismissedInSession:false })).toBe(true);
    // The device-local seen flag keeps its own meaning, and replay-on-load brands still ignore it.
    expect(shouldStartIntroduction({ ...base, seenOnDevice: true })).toBe(false);
    expect(shouldStartIntroduction({ ...base, seenOnDevice: true, replayOnLoad: true })).toBe(true);
    // An app id that has not arrived yet must never arm a tour: the "" key persists nothing, so the tour
    // would arm a second time the moment the real id lands.
    expect(shouldStartIntroduction({ ...base, appId: "" })).toBe(false);
    expect(shouldStartIntroduction({ ...base, hasIntroduction: false })).toBe(false);
    expect(shouldStartIntroduction({ ...base, tutorialActive: true })).toBe(false);
    expect(shouldStartIntroduction({ ...base, suppressed: true })).toBe(false);
  });
it("isEphemeralShellBrand skips durable shell state so a refresh boots from brand defaults only", () => {
    expect(isEphemeralShellBrand(undefined)).toBe(false);
    expect(isEphemeralShellBrand({ id: "plain", windowTitle: "Plain" })).toBe(false);
    expect(isEphemeralShellBrand({ id: "plain", windowTitle: "Plain", ephemeral: true })).toBe(true);
    const ephemeralState = initialShellState({
      locale: "en",
      plugins: [],
      locks: { locale: "de", terminology: "reuse", themeId: "semio" },
      defaults: { exampleId: "concrete-forest" },
      // 🐚️ An in-memory storage port is now the direct analogue of the old `ephemeral: true` flag —
      // nothing persists, so every unlocked pref reads back its own built-in default, same as before.
      storage: createMemoryStoragePort(),
    });
    expect(ephemeralState.layout.activeExampleId).toBe("concrete-forest");
    expect(ephemeralState.uiPrefs.uiLocale).toBe("de");
    expect(ephemeralState.uiPrefs.uiTerminology).toBe("reuse");
    expect(ephemeralState.uiPrefs.uiThemeId).toBe("semio");
    expect(ephemeralState.uiPrefs.uiAppearance).toBe("system");
    expect(ephemeralState.uiPrefs.uiLayout).toBe("desktop");
    expect(ephemeralState.uiPrefs.uiCustomThemes).toEqual({});
    expect(ephemeralState.layout.dockOverride).toBeNull();
    expect(ephemeralState.layout.shellLayout).toBeNull();
    localStorage.setItem("ui.chrome.appearance", "dark");
    localStorage.setItem("semio.os.dock", "{}");
    localStorage.setItem("ui.introduction.seen.entwerfen-mit-bestand-aggregator:puzzle3d-play", "true");
    clearDurableShellStorage();
    expect(localStorage.getItem("ui.chrome.appearance")).toBeNull();
    expect(localStorage.getItem("semio.os.dock")).toBeNull();
    expect(localStorage.getItem("ui.introduction.seen.entwerfen-mit-bestand-aggregator:puzzle3d-play")).toBeNull();
  });

  it("buildOsCommands omits only the commands for locked prefs", () => {
    const ids = buildOsCommands([], [], false, { locale: "de", appearance: "dark" }).map((c) => c.id);
    expect(ids).not.toContain("os.setLocale");
    expect(ids).not.toContain("os.setAppearance");
    expect(ids).toContain("os.setTerminology");
    expect(ids).toContain("os.setThemeId");
  });

  it("dispatchOsCommand is a no-operation for a locked pref even if invoked directly", () => {
    const dispatch = vi.fn();
    const commit = vi.fn();
    dispatchOsCommand("os.setLocale", { locale: "de" }, commit, dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never, { locale: "en" });
    expect(commit).not.toHaveBeenCalled();
    expect(dispatch).not.toHaveBeenCalled();
  });
});
