/** 🏭 Registry-derived utilities, command resolution, tool tabs and tutorial commands. */
import gumballTargetsFixture from "../../🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🧭️gesture-targets.json";
import gumballTargetsSchema from "../../🧱️elements/🌐️World3dHost/🧬️schema/🧭️gesture-targets/🔣️.json";
import gumballLiveProtocolFixture from "../../🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🛠️gumball-live-protocol.json";
import { worldGumballStep, WORLD_GUMBALL_IDLE, type WorldGumballEvent, type WorldGumballTargets } from "../../🧱️elements/🌐️World3dHost/🧮️gumball-math/🟦️.ts";
import { type ReactElement } from "react";
import { hostArmedViewContext, windowViewContext } from "@semio-tech/framework-manifest/view-state-and-public-invocation";
import Ajv2020 from "ajv";
import labelResolutionFixture from "../../🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/🔣️label-resolution.json";
import * as THREE from "three";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { render, waitFor } from "@semio-tech/ui-react/test";
import { describe, expect, it, vi } from "vitest";
import { deriveUtilityNodes, actionSemanticsForKind, resolveWindowActions, resolveModeTools, type ActionDefinition, type AppDefinition, type AppModeDefinition, type AppWindowKindDefinition, type CommandDefinition, type UtilityDefinition, type WindowMeasure } from "@semio-tech/framework";
import { progressPanelTabSelection, resolvePanelBranchBodyLeaf, type PanelTabNode, type TreeDataSection } from "@semio-tech/ui-react";
import { builtNodeToSnapshot, snapWorldPointToGrid, UiDocumentStore, findPressedUtilityLeafId, resolveUtilityNodes, resolveUtilities, panelTabDefinitionToNode, panelAnchorForGroup, partitionFrameworkHistoryPanelTab, shellLabel, syncShellLabelLocale, resolveUtilityActivation, isWorldTransformGumballMode, worldGumballConfigForProjection, gumballTransformDeltaBetweenPoses, gumballIdentityDelta, worldPaintStep, WORLD_PAINT_IDLE, world3dGumballSelectionArgsV1, world3dRelocateDragTargetV1, world3dRelocateDispatchArgsV1, world3dVolumeBrushOriginV1, world3dVolumeBrushCommits, gumballLivePreviewDeltaBetweenPoses, applyGumballLivePreviewDeltaToPose, resolveCommands, commandAddressKey, commandCategories, buildOsCommands, createCoalescingActionDispatcher, dispatchOsCommand, type ResolvedToolDefinition, buildToolTabs, toolCategoryOpenPath, toolLeafInactiveRepress, toolIdFromPanelTabId, reconcileToolTabSelection, toolPanelTreeContentRevision, type ToolTabSelection } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { programArmedToolRevealV1 } from "../../🧱️elements/🛠️ShellHelpers/🎛️command-registry/🟦️.tsx";
import { buildContractNode, panelTreePanelHost } from "../🧰️engine-contract-helpers/🟦️.ts";

describe("registry-derived utilities and activation (P5)", () => {
  const utilities: UtilityDefinition[] = [
    { id: "select", label: "Select", iconId: "mouse-pointer", category: "selection", allowsActionsWhileActive: true },
    { id: "brush", label: "Brush", iconId: "paintbrush", group: "paint", category: "utilities", allowsActionsWhileActive: false },
    { id: "erase", label: "Erase", iconId: "eraser", group: "paint", category: "utilities", allowsActionsWhileActive: false },
  ];
  const app = { controllerId: "draw", utilities } satisfies Pick<AppDefinition, "controllerId" | "utilities">;

  it("resolveUtilities scopes to the window kind's refs, falling back to all app utilities when unset", () => {
    expect(resolveUtilities(app, { utilities: ["brush"] } as Pick<AppWindowKindDefinition, "utilities">).map((t) => t.id)).toEqual(["brush"]);
    expect(resolveUtilities(app, { utilities: [] } as unknown as Pick<AppWindowKindDefinition, "utilities">).map((t) => t.id)).toEqual(["select", "brush", "erase"]);
  });

  it("derives grouped utility nodes with the active utility pressed and a setActiveUtility onChange tagged by window", () => {
    const nodes = resolveUtilityNodes(app, { utilities: [] } as unknown as Pick<AppWindowKindDefinition, "utilities">, "brush", "w1");
    const select = nodes.find((node) => node.id === "select");
    expect(select && select.kind === "toggle" ? select.pressed : undefined).toBe(false);
    const paint = nodes.find((node) => node.id === "group:paint");
    expect(paint?.kind).toBe("collection");
    const brush = paint && paint.kind === "collection" ? paint.children.find((child) => child.id === "brush") : undefined;
    expect(brush && brush.kind === "toggle" ? brush.pressed : undefined).toBe(true);
    expect(brush && brush.kind === "toggle" && "onChange" in brush ? brush.onChange : undefined).toEqual({ controllerId: "draw", action: "setActiveUtility", args: { utilityId: "brush", windowId: "w1" } });
  });

  it("deriveUtilityNodes twin marks exactly the active utility pressed", () => {
    const nodes = deriveUtilityNodes(
      "draw",
      [
        { id: "a", label: "A", iconId: "x" },
        { id: "b", label: "B", iconId: "plus" },
      ],
      "b",
    );
    expect(nodes.map((node) => (node.kind === "toggle" ? node.pressed : undefined))).toEqual([false, true]);
  });

  it("deriveUtilityNodes hoists a single-child group to a top-level toggle", () => {
    const nodes = deriveUtilityNodes(
      "puzzle",
      [
        { id: "transform", label: "Transform", iconId: "move-3d", group: "transform" },
        { id: "brush", label: "Brush", iconId: "paintbrush" },
      ],
      "transform",
    );
    expect(nodes.map((node) => node.id)).toEqual(["transform", "brush"]);
    expect(nodes[0]?.kind).toBe("toggle");
    expect(nodes[0] && nodes[0].kind === "toggle" ? nodes[0].pressed : undefined).toBe(true);
  });

  it("resolveUtilityActivation toggles: click activates, re-click or empty deactivates", () => {
    expect(resolveUtilityActivation(null, "brush")).toBe("brush");
    expect(resolveUtilityActivation("brush", "erase")).toBe("erase");
    expect(resolveUtilityActivation("brush", "brush")).toBeNull();
    expect(resolveUtilityActivation("brush", "")).toBeNull();
    expect(resolveUtilityActivation(undefined, "")).toBeNull();
  });

  it("findPressedUtilityLeafId walks nested collections", () => {
    expect(
      findPressedUtilityLeafId([
        {
          id: "group:transform",
          kind: "collection",
          iconId: "move",
          children: [
            { id: "move", kind: "toggle", iconId: "move", pressed: false, onChange: { controllerId: "x", action: "setActiveUtility", args: { utilityId: "move" } } },
            { id: "rotate", kind: "toggle", iconId: "rotate-cw", pressed: true, onChange: { controllerId: "x", action: "setActiveUtility", args: { utilityId: "rotate" } } },
          ],
        },
      ]),
    ).toBe("rotate");
    expect(findPressedUtilityLeafId([{ id: "brush", kind: "toggle", iconId: "paintbrush", pressed: false, onChange: { controllerId: "x", action: "setActiveUtility", args: { utilityId: "brush" } } }])).toBeUndefined();
  });

  it("isWorldTransformGumballMode requires an explicit move/rotate/scale/transform mode", () => {
    expect(isWorldTransformGumballMode("move")).toBe(true);
    expect(isWorldTransformGumballMode("rotate")).toBe(true);
    expect(isWorldTransformGumballMode("scale")).toBe(true);
    expect(isWorldTransformGumballMode("transform")).toBe(true);
    expect(isWorldTransformGumballMode(undefined)).toBe(false);
    expect(isWorldTransformGumballMode("brush")).toBe(false);
    expect(isWorldTransformGumballMode("")).toBe(false);
  });

  it("worldGumballConfigForProjection intersects transform mode with planar window projections", () => {
    expect(worldGumballConfigForProjection("move", { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "top" } })).toEqual({
      moveAxes: true,
      movePlanes: true,
      rotate: false,
      scaleAxes: false,
      scalePlanes: false,
      scaleUniform: false,
      plane: "xy",
    });
    expect(worldGumballConfigForProjection("rotate", { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "front" } }).plane).toBe("xz");
    expect(worldGumballConfigForProjection("scale", { mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "left" } }).plane).toBe("yz");
    expect(worldGumballConfigForProjection("move", { mode: { kind: "threePoint", fov: 50 }, orientation: { type: "free" } }).plane).toBeUndefined();
    expect(worldGumballConfigForProjection("transform", undefined).plane).toBeUndefined();
  });

  it("gumballIdentityDelta names the handle's verb with a pose delta that moves nothing (a live commit's empty tail, an abort's verb)", () => {
    const base = { mode: "object", ids: ["n1"] };
    expect(gumballIdentityDelta("move", base)).toEqual({ action: "translateSelection", args: { ...base, dx: 0, dy: 0, dz: 0 } });
    expect(gumballIdentityDelta("transform", base, "rotateY")).toEqual({ action: "rotateSelection", args: { ...base, ax: 0, ay: 0, az: 1, angle: 0 } });
    expect(gumballIdentityDelta("scale", base)).toEqual({ action: "scaleSelection", args: { ...base, sx: 1, sy: 1, sz: 1 } });
    expect(gumballTransformDeltaBetweenPoses("move", { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, base)).toBeNull();
  });

  it("worldPaintStep streams a gesture's dabs into one transaction, commits on release, swallows the click and aborts with zero trace (the wgpu world engine's press/move/release protocol)", () => {
    const steps = (events: Parameters<typeof worldPaintStep>[1][]) => {
      let gesture = WORLD_PAINT_IDLE;
      const dispatched: (Record<string, unknown> | null)[] = [];
      for (const event of events) {
        const next = worldPaintStep(gesture, event);
        gesture = next.gesture;
        dispatched.push(next.dispatch);
      }
      return { gesture, dispatched };
    };
    const dab = (origin: "press" | "drag" | "click", u: number) => ({ kind: "dab" as const, origin, objectId: "obj-1", u, v: 0.5 });
    const drag = steps([dab("press", 0.1), { kind: "press" }, dab("drag", 0.2), { kind: "release" }, dab("click", 0.2)]);
    expect(drag.dispatched).toEqual([{ objectId: "obj-1", u: 0.1, v: 0.5, phase: "stream" }, null, { objectId: "obj-1", u: 0.2, v: 0.5, phase: "stream" }, { phase: "commit" }, null]);
    expect(drag.gesture).toEqual(WORLD_PAINT_IDLE);
    expect(steps([{ kind: "press" }, { kind: "release" }, dab("click", 0.4)]).dispatched).toEqual([null, null, { objectId: "obj-1", u: 0.4, v: 0.5 }]);
    expect(steps([dab("drag", 0.3), { kind: "cancel", reason: "blur" }, { kind: "release" }]).dispatched).toEqual([{ objectId: "obj-1", u: 0.3, v: 0.5, phase: "stream" }, { phase: "abort", reason: "blur" }, null]);
    expect(steps([{ kind: "cancel", reason: "captureLost" }]).dispatched).toEqual([null]);
  });

  it("gumballTransformDeltaBetweenPoses emits incremental translate/rotate/scale args", () => {
    const base = { mode: "mesh", ids: ["obj-1"] };
    expect(gumballTransformDeltaBetweenPoses("move", { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [2, -1, 0.5], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, base)).toEqual({
      action: "translateSelection",
      args: { ...base, dx: 2, dy: -1, dz: 0.5 },
    });
    expect(gumballTransformDeltaBetweenPoses("move", { position: [1, 1, 1], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [1, 1, 1], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, base)).toBeNull();
    const rotate = gumballTransformDeltaBetweenPoses("rotate", { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [0, 0, 0], quaternion: [0, 0.7071067811865476, 0, 0.7071067811865476], scale: [1, 1, 1] }, base);
    expect(rotate?.action).toBe("rotateSelection");
    expect(rotate?.args.angle).toBeCloseTo(Math.PI / 2, 5);
    const scale = gumballTransformDeltaBetweenPoses("scale", { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [2, 3, 4] }, base);
    expect(scale).toEqual({ action: "scaleSelection", args: { ...base, sx: 2, sy: 3, sz: 4 } });
    expect(gumballTransformDeltaBetweenPoses("transform", { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [1, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, base, "moveX")).toEqual({
      action: "translateSelection",
      args: { ...base, dx: 1, dy: 0, dz: 0 },
    });
    const transformRotate = gumballTransformDeltaBetweenPoses(
      "transform",
      { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] },
      { position: [0, 0, 0], quaternion: [0, 0.7071067811865476, 0, 0.7071067811865476], scale: [1, 1, 1] },
      base,
      "rotateY",
    );
    expect(transformRotate?.action).toBe("rotateSelection");
    expect(transformRotate?.args.angle).toBeCloseTo(Math.PI / 2, 5);
  });

  it("worldGumballStep answers every step of the language-agnostic gumball live protocol exactly (local one-shot, live stream/commit/abort, no fabricated axis step)", () => {
    for (const testCase of gumballLiveProtocolFixture.cases) {
      const targets = testCase.targets as WorldGumballTargets;
      let gesture = WORLD_GUMBALL_IDLE;
      for (const [index, step] of testCase.steps.entries()) {
        const raw = step.event as Record<string, unknown>;
        const event = (raw.kind === "start" ? { ...raw, targets, live: testCase.live, transformMode: testCase.transformMode } : raw.kind === "release" ? { ...raw, targets } : raw) as WorldGumballEvent;
        const next = worldGumballStep(gesture, event);
        expect(next.dispatch, `${testCase.name} — step ${index} (${raw.kind})`).toEqual(step.dispatch);
        expect(next.skipped, `${testCase.name} — step ${index} skip`).toBe((step as { readonly skipped?: string }).skipped);
        gesture = next.gesture;
      }
      expect(gesture.targets, `${testCase.name} ends at rest`).toBeNull();
    }
  });

  it("gumball gesture targets pin opaque component IDs independently of rendered instances", () => {
    const validate = new Ajv2020({ strict: true }).compile(gumballTargetsSchema);
    for (const fixture of gumballTargetsFixture.cases) {
      expect(validate(fixture.selection)).toBe(true);
      const selection = JSON.parse(JSON.stringify(fixture.selection));
      const pinned = world3dGumballSelectionArgsV1(selection);
      expect(pinned).toEqual(fixture.expected);
      if (selection.ids) selection.ids.splice(0, selection.ids.length, "changed-instance");
      if (selection.gumballSelectionIds) selection.gumballSelectionIds.splice(0, selection.gumballSelectionIds.length, "changed-component");
      expect(pinned).toEqual(fixture.expected);
      const before = { position: gumballTargetsFixture.translation.before as [number, number, number], quaternion: [0, 0, 0, 1] as [number, number, number, number], scale: [1, 1, 1] as [number, number, number] };
      const after = { ...before, position: gumballTargetsFixture.translation.after as [number, number, number] };
      const dispatch = gumballTransformDeltaBetweenPoses("translate", before, after, pinned, "moveX");
      expect(dispatch?.args.ids).toEqual(fixture.expected.ids);
      const delta = new THREE.Vector3(...after.position).sub(new THREE.Vector3(...before.position));
      expect([dispatch?.args.dx, dispatch?.args.dy, dispatch?.args.dz]).toEqual(delta.toArray());
    }
  });

  it("gumballLivePreviewDeltaBetweenPoses applies local start→current deltas for instant mid-drag preview", () => {
    expect(gumballLivePreviewDeltaBetweenPoses("move", { position: [1, 2, 3], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [4, 2, 5], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] })).toEqual({ kind: "translate", dx: 3, dy: 0, dz: 2 });
    const rotate = gumballLivePreviewDeltaBetweenPoses("rotate", { position: [0, 0, 0], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { position: [0, 0, 0], quaternion: [0, 0.7071067811865476, 0, 0.7071067811865476], scale: [1, 1, 1] });
    expect(rotate?.kind).toBe("rotate");
    const scaled = applyGumballLivePreviewDeltaToPose({ position: [10, 0, 0], quaternion: [0, 0, 0, 1], scale: [2, 2, 2] }, { kind: "scale", sx: 1.5, sy: 1, sz: 2 });
    expect(scaled).toEqual({ position: [10, 0, 0], quaternion: [0, 0, 0, 1], scale: [3, 2, 4] });
    const translated = applyGumballLivePreviewDeltaToPose({ position: [1, 1, 1], quaternion: [0, 0, 0, 1], scale: [1, 1, 1] }, { kind: "translate", dx: 2, dy: -3, dz: 0.5 });
    expect(translated.position).toEqual([3, -2, 1.5]);
  });

  it("Relocate-utility drag grabs by selection gate and commits one absolute worldRelocate", () => {
    expect(world3dRelocateDragTargetV1("obj-1", [])).toBe("obj-1");
    expect(world3dRelocateDragTargetV1("obj-1", ["obj-1", "obj-2"])).toBe("obj-1");
    expect(world3dRelocateDragTargetV1("obj-3", ["obj-1", "obj-2"])).toBeNull();
    expect(world3dRelocateDragTargetV1(null, [])).toBeNull();
    expect(world3dRelocateDragTargetV1(null, ["seed-left-001"])).toBe("seed-left-001");
    expect(world3dRelocateDragTargetV1(undefined, ["obj-1", "obj-2"])).toBe("obj-1");
    const selected = world3dGumballSelectionArgsV1({ ids: ["obj-1"], selectionMode: "object" });
    expect(world3dRelocateDragTargetV1("obj-1", selected.ids)).toBe("obj-1");
    expect(world3dRelocateDispatchArgsV1("obj-1", [1, 2, 3], [10, 10, 0], [14, 7, 0])).toEqual({ objectId: "obj-1", position: [5, -1, 3] });
    expect(world3dRelocateDispatchArgsV1("obj-2", [0, 0, 0], [0, 0, 0], [1.4, -2.6, 0], { gridSnapEnabled: true, gridFactor: 1 })).toEqual({ objectId: "obj-2", position: [1, -3, 0] });
    expect(world3dRelocateDispatchArgsV1("obj-2", [0, 0, 0], [0, 0, 0], [1.4, -2.6, 0], { gridSnapEnabled: false, gridFactor: 1 })).toEqual({ objectId: "obj-2", position: [1.4, -2.6, 0] });
    expect(world3dRelocateDispatchArgsV1("obj-1", [1, 2, 3], [10, 10, 0], [10, 10, 0])).toBeNull();
    expect(world3dRelocateDispatchArgsV1("obj-1", [1, 2, 3], [10, 10, 0], [10.0000001, 10, 0])).toBeNull();
  });

  it("every self-gating world lane returns the controlled round trip it coalesces", async () => {
    const first = Promise.withResolvers<void>();
    const sent: string[] = [];
    const lane = createCoalescingActionDispatcher<string>((value) => {
      sent.push(value);
      return sent.length === 1 ? first.promise : Promise.resolve();
    });
    lane("marker-a");
    lane("marker-b");
    lane("marker-c");
    expect(sent).toEqual(["marker-a"]);
    first.resolve();
    await waitFor(() => expect(sent).toEqual(["marker-a", "marker-c"]));

    const { existsSync } = await import("node:fs");
    const { resolve } = await import("node:path");
    const relative = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx";
    let root = process.cwd();
    for (let hop = 0; hop < 12 && !existsSync(resolve(root, relative)); hop += 1) root = resolve(root, "..");
    const source = readModuleTreeSource(resolve(root, relative));
    for (const action of ["interactionHover", "referenceHover", "worldVortexHover"]) expect(source).toMatch(new RegExp(`return dispatchSettled\\(\\"${action}\\"`));
    expect(source).toMatch(/createCoalescingActionDispatcher<string \| null>\(\(fullId\) => dispatchSettled\("targetBrushSuggestions"/);
    expect(source).not.toMatch(/return dispatch\("(?:interactionHover|targetBrushSuggestions|worldVortexHover)"/);
  });

  it("the world host has no brush suggestion tick and no brush ghost; the tool run trace replaces both", async () => {
    const { readFileSync, existsSync } = await import("node:fs");
    const { resolve } = await import("node:path");
    const relative = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🟦️.tsx";
    let root = process.cwd();
    for (let hop = 0; hop < 12 && !existsSync(resolve(root, relative)); hop += 1) root = resolve(root, "..");
    const source = readModuleTreeSource(resolve(root, relative));
    for (const retired of ["suggestionsTick", "createInFlightSkippingInterval(", "brushPreviewJson", "BrushPreviewGhost", "WorldBrushPreviewRecord", "addBrushObject"]) {
      expect(source.includes(retired), `[TRACE] ${retired} is the retired brush ghost path`).toBe(false);
    }
    expect(source).toContain("<WorldToolRunTrace lane={scene.toolRunTrace}");
    expect(source).toContain("if (brushSuggestionsTargetSentRef.current === brushSuggestionsTarget) return;");
    expect(source).toContain('if (fullId) dispatch("acceptSuggestion", { fullId });');
  });

  it("Volume-Brush Alt+click reads the ground through the host, not an occludable canvas plane", () => {
    expect(world3dVolumeBrushCommits(true, true)).toBe(true);
    expect(world3dVolumeBrushCommits(true, false)).toBe(false);
    expect(world3dVolumeBrushCommits(false, true)).toBe(false);
    expect(world3dVolumeBrushOriginV1([1.4, -2.6, 0], 1)).toEqual([1, -3, 0]);
    expect(world3dVolumeBrushOriginV1([1.4, -2.6, 0], 2)).toEqual([2, -2, 0]);
    expect(world3dVolumeBrushOriginV1(null, 1)).toBeNull();
    expect(world3dVolumeBrushOriginV1([1.4, -2.6, 0], 1)).toEqual(snapWorldPointToGrid([1.4, -2.6, 0], true, 1));
  });

  it("resolveWindowActions preserves every definition owned by the window", () => {
    const actionsApp = {
      controllerId: "draw",
      windowKinds: [
        {
          actions: [
            { id: "extrude", label: "Extrude", iconId: "box", semantics: actionSemanticsForKind("mutation"), kind: "mutation", inPalette: true, args: [] },
            { id: "undo", label: "Undo", iconId: "undo", semantics: actionSemanticsForKind("history"), kind: "history", inPalette: true, args: [] },
            { id: "setActiveUtility", label: "Set Active Utility", iconId: "wrench", semantics: actionSemanticsForKind("view"), kind: "view", inPalette: false, args: [] },
          ] satisfies ActionDefinition[],
        },
      ],
    };
    const resolved = resolveWindowActions(actionsApp, actionsApp.windowKinds[0]!);
    expect(resolved.map((action) => action.id)).toEqual(["extrude", "undo", "setActiveUtility"]);
  });

  it("panelTabDefinitionToNode maps the framework-injected History panel tab through its rendered body", () => {
    const emptyAppLabelsOverlay = {
      windowKindLabels: {},
      panelTabLabels: {},
      modeLabels: {},
      actionLabels: {},
      utilityLabels: {},
      exampleLabels: {},
      actionArgLabels: {},
      dialogLabels: {},
      introductionLabels: {},
      groupLabels: {},
    };
    const historyTab = { kind: { kind: "app" as const, id: "framework.panel.history" }, label: "History", group: "settings" as const, bodyKey: "framework.body.history", children: [] };
    const historyUiNode = buildContractNode({
      key: "framework.history",
      component: { type: "tree", interactionDomain: null },
      children: [
        {
          key: "framework.history.commands",
          component: { type: "treeSection", label: null, defaultOpen: true, headerToolbar: null, window: null },
          children: [
            { key: "framework.history.entry.1", component: { type: "treeItem", label: "Increment", description: null, icon: null, defaultOpen: null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: [], target: null } },
          ],
        },
      ],
    });
    const historyStore = new UiDocumentStore("panel:framework.panel.history");
    historyStore.loadSnapshot(builtNodeToSnapshot("panel:framework.panel.history", historyUiNode));
    const node = panelTabDefinitionToNode(historyTab, "settings", { "framework.panel.history": historyStore }, () => {}, 1, emptyAppLabelsOverlay);
    expect(node.kind).toBe("leaf");
    if (node.kind !== "leaf") return;
    expect(node.id).toBe("framework.panel.history");
    const source = node.trees[0].tree;
    const config = "resolveTree" in source ? source.resolveTree() : source;
    const rendered = render(panelTreePanelHost(config));
    expect(rendered.getByText("Increment")).toBeTruthy();
  });

  it("anchors every declarable panel group on one of the four corners the dock fills from the session's own panel tabs", () => {
    expect(panelAnchorForGroup("workbench")).toBe("top-left");
    expect(panelAnchorForGroup("document")).toBe("top-left");
    expect(panelAnchorForGroup("details")).toBe("top-right");
    expect(panelAnchorForGroup("display")).toBe("bottom-left");
    expect(panelAnchorForGroup("settings")).toBe("bottom-right");
    const dockFilledFromSession = new Set(["top-left", "top-right", "bottom-left", "bottom-right"]);
    for (const group of ["workbench", "document", "details", "display", "settings", "unknown-group"]) {
      expect(dockFilledFromSession.has(panelAnchorForGroup(group))).toBe(true);
    }
  });

  it("docks the framework History tab as its own bottom-right leaf and nests only the app's Settings tabs in the Settings branch", () => {
    const panelTabs = [
      { kind: { kind: "app" as const, id: "puzzle3d.panel.settings" }, group: "settings" as const },
      { kind: { kind: "app" as const, id: "framework.panel.history" }, group: "settings" as const },
    ];
    const { history, rest } = partitionFrameworkHistoryPanelTab(panelTabs.filter((tab) => panelAnchorForGroup(tab.group) === "bottom-right"));
    expect(history?.kind.id).toBe("framework.panel.history");
    expect(rest.map((tab) => tab.kind.id)).toEqual(["puzzle3d.panel.settings"]);
    expect(partitionFrameworkHistoryPanelTab([panelTabs[0]!]).history).toBeUndefined();
  });

  it("moves every out-of-hook chrome label when the in-app language switch runs, and restores them on the way back", async () => {
    const keys = ["ui.panel.artifact", "ui.settings.tab.general", "ui.settings.tab.language"] as const;
    const english = keys.map((key) => String(shellLabel(key)));
    syncShellLabelLocale("de");
    const german = keys.map((key) => String(shellLabel(key)));
    expect(german).not.toEqual(english);
    for (const [index, label] of german.entries()) expect(label).not.toBe(english[index]);
    syncShellLabelLocale("en");
    expect(keys.map((key) => String(shellLabel(key)))).toEqual(english);
  });
});

describe("resolveCommands / commandCategories (footer command panel registry)", () => {
  const command = (id: string, label: string, category: string): CommandDefinition => ({ id, label, category, iconId: "wrench", semantics: actionSemanticsForKind("shell"), kind: "shell", inPalette: true, args: [], keybindings: [] });
  const osCommands: CommandDefinition[] = [command("os.setThemeId", "Set Theme", "appearance")];
  const pluginManifest = { pluginId: "fixture", commands: [command("export", "Export", "document")] };
  const app = {
    id: "canvas",
    commands: [command("resetGrid", "Reset Grid", "document")],
    modes: [
      { id: "edit", label: "Edit", commands: [command("focus", "Focus", "view")] },
      { id: "paint", label: "Paint", commands: [command("paintOnly", "Paint Only", "view")] },
    ] as AppModeDefinition[],
  };

  it("aggregates os + program + app-scope + active-mode's mode-scope commands, excluding other modes' mode-scope commands", () => {
    const resolved = resolveCommands(osCommands, pluginManifest, app, "edit");
    expect(resolved.map((entry) => entry.definition.id)).toEqual(["os.setThemeId", "export", "resetGrid", "focus"]);
    expect(resolved.find((entry) => entry.definition.id === "os.setThemeId")?.address).toEqual({ owner: "os", commandId: "os.setThemeId" });
    expect(resolved.find((entry) => entry.definition.id === "resetGrid")?.address).toEqual({ owner: { app: { pluginId: "fixture", appId: "canvas" } }, commandId: "resetGrid" });
    expect(resolved.find((entry) => entry.definition.id === "focus")?.address).toEqual({ owner: { mode: { pluginId: "fixture", appId: "canvas", modeId: "edit" } }, commandId: "focus" });
  });

  it("switching the active mode swaps which mode-scope commands resolve", () => {
    const resolved = resolveCommands(osCommands, pluginManifest, app, "paint");
    expect(resolved.map((entry) => entry.definition.id)).toEqual(["os.setThemeId", "export", "resetGrid", "paintOnly"]);
  });

  it("resolves only os commands with no session (null program manifest / app)", () => {
    const resolved = resolveCommands(osCommands, null, null, "");
    expect(resolved.map((entry) => entry.definition.id)).toEqual(["os.setThemeId"]);
  });

  it("owner-qualifies identical local command ids into collision-free UI keys", () => {
    const duplicateId = "refresh";
    const resolved = resolveCommands(
      [command(duplicateId, "Refresh Shell", "general")],
      { pluginId: "fixture", commands: [command(duplicateId, "Refresh Plugin", "general")] },
      {
        id: "canvas",
        commands: [command(duplicateId, "Refresh App", "general")],
        modes: [{ id: "edit", label: "Edit", commands: [command(duplicateId, "Refresh Mode", "general")] }] as AppModeDefinition[],
      },
      "edit",
    );
    const keys = resolved.map((entry) => commandAddressKey(entry.address));
    expect(new Set(keys).size).toBe(4);
    expect(keys).toEqual(["os:refresh", "plugin:fixture:refresh", "app:fixture:canvas:refresh", "mode:fixture:canvas:edit:refresh"]);
  });

  it("commandCategories orders and dedupes categories by first appearance", () => {
    const resolved = resolveCommands(osCommands, pluginManifest, app, "edit");
    expect(commandCategories(resolved)).toEqual([
      { id: "appearance", label: "Appearance" },
      { id: "document", label: "Document" },
      { id: "view", label: "View" },
    ]);
  });
});

describe("resolveModeTools / buildToolTabs (footer tool panel registry)", () => {
  const toolApp = {
    tools: [
      { id: "fill", label: "Fill", iconId: "paint-bucket" },
      { id: "brush", label: "Brush", iconId: "paintbrush" },
    ] satisfies ResolvedToolDefinition[],
    modes: [
      { id: "edit", label: "Edit", tools: ["fill", "brush"] },
      { id: "view", label: "View", tools: [] },
    ] as AppModeDefinition[],
  };

  it("resolves the active mode's tools in declared order", () => {
    expect(resolveModeTools(toolApp, "edit").map((tool) => tool.id)).toEqual(["fill", "brush"]);
  });

  it("tools are opt-in per mode — no orphan fallback for a mode that declares none", () => {
    expect(resolveModeTools(toolApp, "view")).toEqual([]);
  });

  it("resolves nothing for an app/mode that doesn't exist", () => {
    expect(resolveModeTools(undefined, "edit")).toEqual([]);
    expect(resolveModeTools(toolApp, "nonexistent")).toEqual([]);
  });

  it("buildToolTabs builds one leaf per resolved tool, whose lazily-resolved tree carries that tool's own measures", () => {
    const toolMeasuresByToolIdRef = {
      current: { fill: [{ kind: "slider", id: "puzzle3d-fill-count", label: "Count", value: 3, min: 0, max: 100, onChange: { controllerId: "c", action: "setFillCount" } }] } as Readonly<Record<string, readonly WindowMeasure[]>>,
    };
    const onAction = vi.fn();
    const tabs = buildToolTabs(toolApp.tools, toolMeasuresByToolIdRef, onAction);
    expect(tabs.map((tab) => tab.id)).toEqual(["tool.fill", "tool.brush"]);
    const fillTab = tabs[0] as Extract<PanelTabNode, { kind: "leaf" }>;
    const fillTree = fillTab.trees[0]!.tree as { resolveTree: () => { sections: TreeDataSection[]; sortableSections: false } };
    const fillResolved = fillTree.resolveTree();
    expect(fillResolved.sortableSections).toBe(false);
    expect(fillResolved.sections).toHaveLength(1);
    expect(fillResolved.sections[0]!.id).toBe("tool.fill.options");
    expect(fillResolved.sections[0]!.items).toHaveLength(1);
    expect(fillResolved.sections[0]!.items![0]!.label).toBe("Count");
    expect(fillResolved.sections[0]!.items![0]!.control).toBeTruthy();

    const brushTab = tabs[1] as Extract<PanelTabNode, { kind: "leaf" }>;
    const brushTree = brushTab.trees[0]!.tree as { resolveTree: () => { sections: TreeDataSection[] } };
    const brushResolved = brushTree.resolveTree();
    expect(brushResolved.sections).toHaveLength(1);
    expect(brushResolved.sections[0]!.id).toBe("tool.brush.options");
    expect(brushResolved.sections[0]!.items).toEqual([]);
  });

  it("no tool tree carries a control that duplicates its own leaf tab id", () => {
    const toolMeasuresByToolIdRef = { current: {} as Readonly<Record<string, readonly WindowMeasure[]>> };
    const tabs = buildToolTabs(toolApp.tools, toolMeasuresByToolIdRef, vi.fn());
    for (const tab of tabs as Extract<PanelTabNode, { kind: "leaf" }>[]) {
      const tree = tab.trees[0]!.tree as { resolveTree: () => { sections: TreeDataSection[] } };
      for (const section of tree.resolveTree().sections) {
        for (const item of section.items ?? []) {
          const control = item.control as ReactElement<{ id?: string }> | undefined;
          expect(control?.props?.id).not.toBe(tab.id);
        }
      }
    }
  });

  it("reconcileToolTabSelection arms the tool a restored dock arrangement already selects", () => {
    const restored = ["framework.category.tool", "tool.fill"];
    const hydrate = reconcileToolTabSelection(null, null, toolIdFromPanelTabId(restored[restored.length - 1]!));
    expect(hydrate.effect).toEqual({ kind: "activate", toolId: "fill" });
    expect(hydrate.next).toEqual({ toolId: "fill", selected: "fill" });
    expect(reconcileToolTabSelection(hydrate.next, "fill", "fill").effect).toEqual({ kind: "idle" });
  });

  it("windowed context-menu view overlays the armed tool like handleAction", () => {
    const view = {
      locale: "en",
      terminology: "native",
      windowInstances: [{ id: "left", windowKindId: "graph" }],
      activeUtilityByWindowId: { left: "pan" },
    };
    expect(windowViewContext(view, "left")?.activeToolId).toBeUndefined();
    expect(hostArmedViewContext(view, "fill", "left")?.activeToolId).toBe("fill");
  });

  it("reconcileToolTabSelection disarms the tool when its leaf collapses, and re-arms on the next press", () => {
    const armed: ToolTabSelection = { toolId: "fill", selected: "fill" };
    const collapsed = reconcileToolTabSelection(armed, "fill", null);
    expect(collapsed.effect).toEqual({ kind: "activate", toolId: null });
    expect(reconcileToolTabSelection(collapsed.next, null, "fill").effect).toEqual({ kind: "activate", toolId: "fill" });
  });

  it("reconcileToolTabSelection selects the leaf of a tool armed by the program, and collapses when a utility clears it", () => {
    const idle: ToolTabSelection = { toolId: null, selected: null };
    const armedByProgram = reconcileToolTabSelection(idle, "fill", null);
    expect(armedByProgram.effect).toEqual({ kind: "select", toolId: "fill" });
    expect(reconcileToolTabSelection(armedByProgram.next, null, "fill").effect).toEqual({ kind: "select", toolId: null });
  });

  it("a tool the program arms reveals its own leaf tab, and a tool the user browsed away from does not", () => {
    expect(programArmedToolRevealV1(null, "fill", false)).toBe(true);
    expect(programArmedToolRevealV1(null, "fill", true)).toBe(false);
    expect(programArmedToolRevealV1("fill", "fill", false)).toBe(false);
    expect(programArmedToolRevealV1("fill", null, false)).toBe(false);
    expect(programArmedToolRevealV1("fill", "brush", false)).toBe(true);
    expect(programArmedToolRevealV1(null, null, false)).toBe(false);
  });

  it("reconcileToolTabSelection self-heals a refused activation instead of looping", () => {
    const refused = reconcileToolTabSelection(null, null, "fill");
    expect(refused.effect).toEqual({ kind: "activate", toolId: "fill" });
    expect(reconcileToolTabSelection(refused.next, null, "fill").effect).toEqual({ kind: "select", toolId: null });
  });

  it("one press on the Tool category reveals its remembered tool leaf and arms that tool", () => {
    const tabs: PanelTabNode[] = [{ kind: "branch", id: "framework.category.tool", icon: () => null, name: "Tool", children: buildToolTabs(toolApp.tools, { current: {} }, vi.fn()) }];
    const opened = progressPanelTabSelection(tabs, [], ["framework.category.tool"], { "framework.category.tool": "tool.fill" });
    expect(opened.fold).toBe(false);
    expect(opened.path).toEqual(["framework.category.tool", "tool.fill"]);
    expect(reconcileToolTabSelection(null, null, toolIdFromPanelTabId(opened.path[opened.path.length - 1])).effect).toEqual({ kind: "activate", toolId: "fill" });
  });

  it("toolIdFromPanelTabId extracts the mode tool id from a tool leaf tab id", () => {
    expect(toolIdFromPanelTabId("tool.fill")).toBe("fill");
    expect(toolIdFromPanelTabId("tool.brush")).toBe("brush");
    expect(toolIdFromPanelTabId("framework.category.tool")).toBeNull();
    expect(toolIdFromPanelTabId("tool.")).toBeNull();
    expect(toolIdFromPanelTabId(undefined)).toBeNull();
  });

  it("late-arriving fill measures change the tool panel tree revision so the memoized pane re-resolves", () => {
    const toolMeasuresByToolIdRef = { current: {} as Readonly<Record<string, readonly WindowMeasure[]>> };
    const tabs = buildToolTabs(toolApp.tools, toolMeasuresByToolIdRef, vi.fn());
    const fillTree = (tabs[0] as Extract<PanelTabNode, { kind: "leaf" }>).trees[0]!.tree as { resolveTree: () => { sections: TreeDataSection[] } };
    expect(fillTree.resolveTree().sections[0]!.items).toEqual([]);
    const emptyRevision = toolPanelTreeContentRevision(null, toolMeasuresByToolIdRef.current, {});
    toolMeasuresByToolIdRef.current = { fill: [{ kind: "slider", id: "puzzle3d-fill-count", label: "Count", value: 0, min: 0, max: 100, onChange: { controllerId: "c", action: "setFillCount" } }] };
    expect(fillTree.resolveTree().sections[0]!.items).toHaveLength(1);
    const landedRevision = toolPanelTreeContentRevision(null, toolMeasuresByToolIdRef.current, {});
    expect(landedRevision).not.toEqual(emptyRevision);
    expect(landedRevision.toolMeasuresByToolId.fill).toHaveLength(1);
  });

  it("one press on an inactive selected Fill leaf arms the tool instead of collapsing it", () => {
    const previous = ["framework.category.tool", "tool.fill"];
    const collapsed = progressPanelTabSelection(
      [{ kind: "branch", id: "framework.category.tool", icon: () => null, name: "Tool", children: buildToolTabs(toolApp.tools, { current: {} }, vi.fn()) }],
      previous,
      ["framework.category.tool", "tool.fill"],
      { "framework.category.tool": "tool.fill" },
    );
    expect(collapsed.path).toEqual(["framework.category.tool"]);
    const repress = toolLeafInactiveRepress(previous, collapsed.path, null);
    expect(repress).toEqual({ path: previous, toolId: "fill" });
    expect(toolLeafInactiveRepress(previous, collapsed.path, "fill")).toBeNull();
  });

  it("one press that opens the Tool category selects Fill when no leaf is remembered", () => {
    const tabs = buildToolTabs(toolApp.tools, { current: {} }, vi.fn());
    const opened = progressPanelTabSelection([{ kind: "branch", id: "framework.category.tool", icon: () => null, name: "Tool", children: tabs }], [], ["framework.category.tool"], {});
    expect(opened.fold).toBe(false);
    expect(opened.path).toEqual(["framework.category.tool"]);
    expect(
      toolCategoryOpenPath(
        opened.path,
        opened.memory,
        tabs.map((tab) => tab.id),
      ),
    ).toEqual(["framework.category.tool", "tool.fill"]);
  });

  it("a Tool category branch still exposes the remembered Fill leaf trees as the panel body", () => {
    const fill = buildToolTabs(toolApp.tools, { current: {} }, vi.fn())[0]!;
    const branch: PanelTabNode = { kind: "branch", id: "framework.category.tool", icon: () => null, name: "Tool", children: [fill] };
    const body = resolvePanelBranchBodyLeaf(branch, { "framework.category.tool": "tool.fill" });
    expect(body?.id).toBe("tool.fill");
    expect(body?.kind).toBe("leaf");
    expect(body && body.kind === "leaf" ? body.trees.length : 0).toBeGreaterThan(0);
    expect(resolvePanelBranchBodyLeaf(branch, {})?.id).toBe("tool.fill");
  });
});

describe("Introduce App command", () => {
  it("is available only for apps with an introduction", () => {
    expect(buildOsCommands([], [], true).find((command) => command.id === "os.introduceApp")).toMatchObject({ label: "Introduce App", category: "app", args: [] });
    expect(buildOsCommands([], [], false).some((command) => command.id === "os.introduceApp")).toBe(false);
  });

  it("starts the introduction at its first step", () => {
    const dispatch = vi.fn();
    dispatchOsCommand("os.introduceApp", undefined, vi.fn(), dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never);
    expect(dispatch).toHaveBeenCalledWith({ type: "SET_INTRODUCTION_STEP", value: 0 });
  });
});

describe("Play/Record Tutorial commands", () => {
  it("os.playTutorial appears only when at least one tutorial is declared, offering each as a Select option", () => {
    expect(buildOsCommands([], [], false).some((command) => command.id === "os.playTutorial")).toBe(false);
    const withTutorials = buildOsCommands([], [], false, undefined, undefined, [{ id: "welcome-tour", title: "Welcome Tour" }]);
    const playTutorial = withTutorials.find((command) => command.id === "os.playTutorial");
    expect(playTutorial).toMatchObject({ label: "Play Tutorial", category: "app" });
    expect(playTutorial?.args[0]).toMatchObject({ id: "tutorialId", required: true, schema: { kind: "string", options: [{ value: "welcome-tour", label: "Welcome Tour" }] } });
    const localized = buildOsCommands([], [], false, undefined, undefined, [{ id: "localized-tour", title: labelResolutionFixture.matrix }], false, "reuse", "de");
    expect(localized.find((command) => command.id === "os.playTutorial")?.args[0]).toMatchObject({ schema: { kind: "string", options: [{ value: "localized-tour", label: "Bauteil" }] } });
  });

  it("os.recordTutorial appears only when the recorder is available (dev/studio), independent of declared tutorials", () => {
    expect(buildOsCommands([], [], false, undefined, undefined, [], false).some((command) => command.id === "os.recordTutorial")).toBe(false);
    expect(buildOsCommands([], [], false, undefined, undefined, [], true).some((command) => command.id === "os.recordTutorial")).toBe(true);
  });

  it("os-scope Play/Record Tutorial commands are NOT handled by dispatchOsCommand (routed earlier, through the shell's own startTutorialRef/toggleTutorialRecordingRef bridge)", () => {
    const dispatch = vi.fn();
    dispatchOsCommand("os.playTutorial", { tutorialId: "welcome-tour" }, vi.fn(), dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never);
    dispatchOsCommand("os.recordTutorial", undefined, vi.fn(), dispatch, { reset: vi.fn() } as never, { reset: vi.fn() } as never);
    expect(dispatch).not.toHaveBeenCalled();
  });
});
