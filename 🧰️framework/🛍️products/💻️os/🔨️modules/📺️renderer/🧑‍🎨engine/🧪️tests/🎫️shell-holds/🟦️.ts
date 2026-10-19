/** 🎫 The shell says what it holds, plus gizmo, virtual file system, replay refusal and locale source. */
import { readUiPreferences } from "../../🧱️elements/🎚️UiPreferences/🟦️.ts";
import explicitLocaleFixture from "../../🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/⏳️compute-locale.json";
import { gizmoTipBoundsFixture, sceneListTransferFixture, treeDragHandleFixture, vfsDescriptorFixture } from "../../../../../../../🔨️modules/🖱️ui/🧪️tests/🧫️fixtures/🟦️.ts";
import { renderVirtualFileSystemDescriptorCell, type DescriptorKind, type FileNodeDescriptorValue } from "@semio-tech/ui-react";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { COMPACT_UI_DRIVER, DEFAULT_UI_DRIVER, TreeContext, TreeItem, UiDriverProvider, closestCenter, deriveTreeDragRoles, uiI18n } from "@semio-tech/ui-react";
import { BlockListHost } from "../../🧱️elements/🧩️BlockListHost/🟦️.tsx";
import replayRefusalVocabulary from "../../🧱️elements/🏛️ShellHost/📣️replay-refusal/🔣️.json";
import { REPLAY_REFUSAL_LABELS_V1, replayRefusalCodeV1, replayRefusalNoticeTextV1, type ReplayRefusalReasonV1 } from "../../🧱️elements/🏛️ShellHost/📣️replay-refusal/🟦️.ts";
import * as THREE from "three";
import { worldSceneLightingFixture as world3dLightingFixture, worldSceneShadowsFixture as world3dShadowFixture, sceneShadowParityFixture as world3dShadowParityFixture } from "../../../../♾️infinite/🧪️tests/🧫️fixtures/🟦️.ts";
import { readModuleTreeSource } from "@semio-tech/framework-compiler-ts/syntax/module-tree";
import { fireEvent, render } from "@semio-tech/ui-react/test";
import { describe, expect, it, vi } from "vitest";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { TableHost, initialShellState } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import "../🧰️engine-contract-helpers/🟦️.ts";


//#region 🎫️RemainingReds
/** 🎫️ Ticket 26/09/09/PROCEDURAL-3D-END-TO-END, lane `react-remaining-reds`. Four surfaces of the shell
 * that a user reads or clicks and that published nothing — or published over each other — measured on
 * the generation3d React serve (:6018) and closed on the layer that owns each one. */
describe("🎫️ the shell says what it holds", () => {
  it("Escape on a graph surface clears the FRAMEWORK selection, not a deleted media-graph verb", async () => {
    const { readFileSync } = await import("node:fs");
    const { dirname, resolve } = await import("node:path");
    const { fileURLToPath } = await import("node:url");
    const repoRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../../../../../../../..");
    const nodeGraph = readModuleTreeSource(resolve(repoRoot, "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx"));
    const manifest = readFileSync(resolve(repoRoot, "🧰️framework/🔨️modules/🛂️manifest/🔘️definitions-action-ids-and-shell-brand/🟦️.ts"), "utf8");
    const { nodeGraphActions } = await import("../../../../../../../🔨️modules/🖱️ui/🎬️scene/🕸️node-graph/🟦️.ts");

    const reserved = /export const CLEAR_SELECTION_ACTION_ID = "([^"]+)";/u.exec(manifest)?.[1];
    expect(reserved, "the framework declares the reserved clear verb").toBe("clearSelection");
    expect(nodeGraphActions.clearSelection, "and the graph scene's action table names that very verb").toBe(reserved);

    const handler = nodeGraph.slice(nodeGraph.indexOf("function handleGraphKeyboard"));
    const body = handler.slice(0, handler.indexOf("\n}"));
    expect(body, "Escape must dispatch the reserved clear verb").toContain("dispatch(nodeGraphActions.clearSelection)");
    expect(nodeGraph, "`setMediaNodeSelection` was deleted with the first-class selection mechanism — no window kind declares it, so every press was dropped").not.toMatch(/dispatch\(\s*"setMediaNodeSelection"/u);
  });

  it("a graph surface publishes the selection it paints, in the same shape the 3D pane does", async () => {
    const { nodeGraphSurfaceSelectionDomV1 } = await import("@semio-tech/framework-os-node-graph-rs/index");
    expect(nodeGraphSurfaceSelectionDomV1(undefined)).toEqual({ selectedIds: [], highlightedIds: [], hoverTarget: null, editable: true });
    const picked = nodeGraphSurfaceSelectionDomV1({
      nodes: [],
      edges: [],
      editable: true,
      selection: ["height"],
      highlighted: ["height@number"],
      hover: { nodeId: "profile", portId: "radius" },
    });
    expect(picked.selectedIds, "the node the Artifact panel's outline row picked").toEqual(["height"]);
    expect(picked.highlightedIds).toEqual(["height@number"]);
    expect(picked.hoverTarget).toEqual({ nodeId: "profile", portId: "radius" });
    const hoverOnly = nodeGraphSurfaceSelectionDomV1({ nodes: [], edges: [], hover: { nodeId: "extrude" } });
    expect(hoverOnly.hoverTarget).toEqual({ nodeId: "extrude", portId: null });
    expect(hoverOnly.selectedIds, "an unhovered graph publishes an EMPTY lane, never a missing one").toEqual([]);
  });

  it("the shell publishes the framework history cursor undo and redo are decided by, in the locale it is showing", async () => {
    const { shellHistoryCursorDomV1, SHELL_HISTORY_DOM_LABELS } = await import("../../🧱️elements/🛠️ShellHelpers/🔩️plumbing/🟦️.ts");
    // 🌐️ A history row carries the whole locale matrix, exactly as Rust `LocalizedLabel` serialises
    // it — the projection resolves it, so the SAME ledger publishes German labels for a German shell
    // with no re-read of the guest's history. Ticket 26/09/18 slice U3.
    const bilingual = (en: string, de: string) => ({ native: { en, de }, reuse: { en, de } }) as const;
    const entry = (seq: number, en: string, de: string) => [seq, { seq, label: bilingual(en, de), actionId: `action-${seq}`, kind: "app", timestamp: "0" }] as const;
    const english = { terminology: "native", locale: "en" } as const;
    const german = { terminology: "native", locale: "de" } as const;

    const empty = shellHistoryCursorDomV1({ cursor: 0, entries: {}, canUndo: false, canRedo: false }, english);
    expect(empty).toEqual({ cursor: 0, canUndo: false, canRedo: false, entries: 0, currentCheckpointId: null, labels: [], actionIds: [], undoLabel: null, redoLabel: null });

    const entries = Object.fromEntries([entry(1, "Add Widget", "Widget hinzufügen"), entry(2, "Move Widget", "Widget verschieben"), entry(3, "Delete Selection", "Auswahl löschen")]);
    const midway = shellHistoryCursorDomV1({ cursor: 2, entries, canUndo: true, canRedo: true, currentCheckpointId: "check-1" }, english);
    expect(midway.cursor).toBe(2);
    expect(midway.undoLabel, "undo reverts the newest entry at or below the cursor").toBe("Move Widget");
    expect(midway.redoLabel, "redo re-applies the first entry above it").toBe("Delete Selection");
    expect(midway.labels).toEqual(["Add Widget", "Move Widget", "Delete Selection"]);
    expect(midway.currentCheckpointId).toBe("check-1");

    const auf_deutsch = shellHistoryCursorDomV1({ cursor: 2, entries, canUndo: true, canRedo: true, currentCheckpointId: "check-1" }, german);
    expect(auf_deutsch.undoLabel, "the same ledger, resolved for a German shell").toBe("Widget verschieben");
    expect(auf_deutsch.redoLabel).toBe("Auswahl löschen");
    expect(auf_deutsch.labels, "no English leaks into a German projection").toEqual(["Widget hinzufügen", "Widget verschieben", "Auswahl löschen"]);
    expect(auf_deutsch.actionIds, "ids are locale-invariant").toEqual(midway.actionIds);

    const bounded = shellHistoryCursorDomV1({ cursor: 40, canUndo: true, canRedo: false, entries: Object.fromEntries(Array.from({ length: 40 }, (_, index) => entry(index + 1, `Step ${index + 1}`, `Schritt ${index + 1}`))) }, english);
    expect((bounded.labels as readonly string[]).length, "a DOM attribute carries a bounded tail, never the whole log").toBe(SHELL_HISTORY_DOM_LABELS);
    expect((bounded.labels as readonly string[])[SHELL_HISTORY_DOM_LABELS - 1]).toBe("Step 40");
    expect(bounded.redoLabel, "nothing above the cursor to redo").toBeNull();
  });

  it("a cleared leftover overlay carries no ids, so an outline row it named goes idle on the clearing turn", async () => {
    const { leftoverOverlayCarryingSelectionV1, leftoverTreeItemSelectedV1, leftoverWorldOverlayAppliesV1, mergeWorldSelectionWithLeftoverV1 } = await import("../../🧱️elements/🌐️World3dHost/🔗️instance-selection/🟦️.ts");
    const prior = { ids: ["extrusion-axis"], hoveredId: null, gumballActive: true, gumballAnchorId: "extrusion-axis", activeUtility: "select" };
    /** 🧹️ The publication generation3d's `clearSelection` really sends, measured on 6018:
     * `selectionCleared: true` alongside the ids it has just retired. */
    const cleared = leftoverOverlayCarryingSelectionV1({ ids: ["extrusion-axis"], hoveredId: null, gumballActive: false, gumballAnchorId: null, selectionCleared: true }, prior, true);
    expect(cleared.selectionCleared).toBe(true);
    expect(cleared.ids, "a clear is an ABSENCE — it cannot carry the ids it retired").toEqual([]);
    expect(cleared.gumballActive).toBe(false);
    expect(leftoverWorldOverlayAppliesV1(cleared), "and it still applies, or the one publication that REMOVES a selection would be the one nothing acts on").toBe(true);
    expect(leftoverTreeItemSelectedV1("panel:procedural-play-graph/extrusion-axis", cleared.ids), "the outline row the clear retired reads idle").toBe(false);
    expect(leftoverTreeItemSelectedV1("panel:procedural-play-graph/extrusion-axis", prior.ids), "fails-before: the same row read selected off the overlay's retained ids").toBe(true);
    expect(mergeWorldSelectionWithLeftoverV1({ ids: ["extrusion-axis"], activeObjectId: "extrusion-axis", gumballActive: true }, cleared).ids).toEqual([]);

    const kept = leftoverOverlayCarryingSelectionV1({ ids: ["height"], hoveredId: null, gumballActive: false, gumballAnchorId: null }, prior);
    expect(kept.ids, "an ordinary pick is untouched").toEqual(["height"]);
  });

  it("an open chrome-hosted dock lays its fold control BESIDE the tab strip, never under it", async () => {
    const { chromeHostedPanelCapRowStyle, uiSpacingPx } = await import("@semio-tech/ui-react");
    /** 📐️ The measured 6018 geometry (`🐍️remaining-reds-recon.mjs` → `🗑️generated/react-reds/recon/recon.json`):
     * a 300 px dock body, a 238 px tab strip, a 64 px `Collapse` control and a 92 px navbar trailing-end
     * reserve — 394 px of content for 300 px of body. */
    const body = 300,
      strip = 238,
      controls = 64,
      reserve = 92;
    const style = chromeHostedPanelCapRowStyle("top-right", reserve);
    expect(style.paddingInlineStart, "the navbar's trailing end is still cleared").toBe(`${reserve + uiSpacingPx(1)}px`);
    expect(style.width, "the cap row sizes to its content").toBe("max-content");
    expect(style.minWidth, "and never shrinks below the body it caps").toBe("100%");

    /** 📐️ One inline axis, laid out right-to-left from the dock's own right edge, the way the panel root's
     * `dir="rtl"` lays a right-anchored cap out. Before the fix the row was pinned to `body`, so the strip
     * was allotted `body - controls - reserve` and painted its full width anyway — over the controls. */
    const rowWidth = Math.max(body, reserve + strip + controls);
    const stripBox = { start: rowWidth - reserve - strip, end: rowWidth - reserve };
    const controlsBox = { start: rowWidth - reserve - strip - controls, end: rowWidth - reserve - strip };
    expect(controlsBox.end, "the fold control ends exactly where the strip begins").toBe(stripBox.start);
    expect(Math.max(stripBox.start, controlsBox.start) < Math.min(stripBox.end, controlsBox.end), "no overlap").toBe(false);

    const pinnedStripBox = { start: body - reserve - strip, end: body - reserve };
    const pinnedControlsBox = { start: 0, end: controls };
    expect(Math.max(pinnedStripBox.start, pinnedControlsBox.start) < Math.min(pinnedStripBox.end, pinnedControlsBox.end), "fails-before: pinned to the body, the strip painted over the fold control").toBe(true);
  });

  it("the shared World3d lighting fixture builds React's actual light and standard-material values", async () => {
    
    

    const { sunPositionFromAzimuthElevation } = await import("@semio-tech/ui-react");
    const request = world3dLightingFixture.iconRenderRequest;
    const environment = world3dLightingFixture.worldEnvironment;
    const ambient = new THREE.AmbientLight(request.lights.ambientColor, request.lights.ambientIntensity);
    const sun = new THREE.DirectionalLight(request.lights.sunColor, request.lights.sunIntensity);
    sun.position.fromArray(sunPositionFromAzimuthElevation(request.lights.sunAzimuth, request.lights.sunElevation));
    const material = new THREE.MeshStandardMaterial({
      color: request.material.color,
      metalness: request.material.metalness,
      roughness: request.material.roughness,
      emissive: request.material.emissive,
      emissiveIntensity: request.material.emissiveIntensity,
    });

    expect(ambient.intensity).toBe(environment.ambient.intensity);
    expect(ambient.color.getHexString()).toBe(environment.ambient.color.slice(1));
    expect(sun.intensity).toBe(environment.sun.intensity);
    expect(sun.color.getHexString()).toBe(environment.sun.color.slice(1));
    expect(sun.position.clone().normalize().toArray()).toEqual(
      [
        Math.cos(THREE.MathUtils.degToRad(environment.sun.elevation)) * Math.cos(THREE.MathUtils.degToRad(environment.sun.azimuth)),
        Math.cos(THREE.MathUtils.degToRad(environment.sun.elevation)) * Math.sin(THREE.MathUtils.degToRad(environment.sun.azimuth)),
        Math.sin(THREE.MathUtils.degToRad(environment.sun.elevation)),
      ].map((value) => expect.closeTo(value, 12)),
    );
    expect(material.metalness).toBe(environment.material.metalness);
    expect(material.roughness).toBe(environment.material.roughness);
    expect(material.emissive.getHexString()).toBe(environment.material.emissive.slice(1));
    expect(material.emissiveIntensity).toBe(environment.material.emissiveIntensity);
    expect(
      material.emissive
        .clone()
        .multiplyScalar(material.emissiveIntensity)
        .toArray()
        .some((channel) => channel > 0),
      "the oracle's neutral emissive contributes radiance",
    ).toBe(true);
  });

  it("the shared Tree drag fixture matches React's handle and surface drivers", () => {
    for (const row of treeDragHandleFixture.rows) {
      const dragData = "dragData" in row ? row.dragData : undefined;
      const roles = deriveTreeDragRoles({ draggable: row.draggable, dragData }, false);
      expect(roles).toEqual(row.role ? [row.role] : []);
      // 🧭️ Annotated, not inferred: the two fixture rows differ in which id lists are empty, so the
      // imported JSON types one `labelStarts`/`visibleHandles` as `never[]` and `.includes(id)` refuses.
      const driverCases: readonly { readonly id: string; readonly drag: string; readonly visibleHandles: readonly string[]; readonly labelStarts: readonly string[]; readonly handleStarts: readonly string[] }[] = treeDragHandleFixture.drivers;
      for (const driverCase of driverCases) {
        const driver = driverCase.drag === "surface" ? COMPACT_UI_DRIVER : DEFAULT_UI_DRIVER;
        const markup = renderToStaticMarkup(
          createElement(
            UiDriverProvider,
            { driver },
            createElement(
              TreeContext.Provider,
              { value: { level: 0, isLastAtLevel: [], showLines: true, isTree: true, indentMultiplier: 1 } },
              createElement(TreeItem, {
                id: row.id,
                label: row.label,
                draggable: row.draggable,
                dragRoles: roles,
                dragInitiation: "handle",
                dragData,
              }),
            ),
          ),
        );
        const visible = driverCase.visibleHandles.includes(row.id);
        const labelStarts = driverCase.labelStarts.includes(row.id);
        expect(markup.includes('data-slot="drag-handle"')).toBe(visible);
        expect(/data-slot="tree-item-row"[^>]*\sdraggable="true"/.test(markup), `${driverCase.id}:${row.id}`).toBe(labelStarts);
        if (visible) {
          expect(markup).toContain(`data-drag-role="${row.role}"`);
        }
      }
    }
  });

  it("the shared scene-list transfer fixture matches React data transfer and dnd-kit geometry", async () => {
    const fixture = sceneListTransferFixture;
    
    
    await uiI18n.changeLanguage("en");

    const transferValues = new Map<string, string>();
    const transferTypes: string[] = [];
    const dataTransfer = {
      effectAllowed: "uninitialized",
      dropEffect: "none",
      types: transferTypes,
      setData(type: string, value: string) {
        if (!transferValues.has(type)) transferTypes.push(type);
        transferValues.set(type, value);
      },
      getData(type: string) {
        return transferValues.get(type) ?? "";
      },
      clearData(type?: string) {
        if (type === undefined) {
          transferValues.clear();
          transferTypes.splice(0);
        } else {
          transferValues.delete(type);
          const index = transferTypes.indexOf(type);
          if (index >= 0) transferTypes.splice(index, 1);
        }
      },
      files: [],
      items: [],
      setDragImage() {},
    } as unknown as DataTransfer;
    const tableSource = fixture.table.source;
    const sourceNode = {
      surfaceId: tableSource.surfaceId,
      controllerId: "controller.table-a",
      table: {
        columnsJson: JSON.stringify([{ id: "label", label: "Label" }]),
        rowsJson: JSON.stringify([{ id: tableSource.rowId, label: "Asset 7", _drag: tableSource.payload }]),
        rowDragMime: tableSource.mime,
      },
    };
    const destinationNode = {
      surfaceId: fixture.table.destination.surfaceId,
      controllerId: fixture.table.destination.dropAction.controllerId,
      table: {
        columnsJson: JSON.stringify([{ id: "label", label: "Label" }]),
        rowsJson: JSON.stringify([]),
        dropActionJson: JSON.stringify(fixture.table.destination.dropAction),
      },
    };
    const destinationAction = vi.fn();
    const sourceView = render(createElement(UiDriverProvider, { driver: DEFAULT_UI_DRIVER }, createElement(TableHost, { node: sourceNode as never, onAction: vi.fn() })));
    const destinationView = render(createElement(UiDriverProvider, { driver: DEFAULT_UI_DRIVER }, createElement(TableHost, { node: destinationNode as never, onAction: destinationAction })));
    try {
      const row = sourceView.container.querySelector("tbody tr") as HTMLTableRowElement;
      const handle = row.querySelector('[data-slot="drag-handle"]') as HTMLElement;
      expect(handle).not.toBeNull();
      expect(row.draggable, "Handle leaves the row inert before its handle arms").toBe(false);
      fireEvent.pointerDown(handle, { button: 0, isPrimary: true });
      expect(row.draggable, "the real native-drag arm makes the row draggable").toBe(true);
      fireEvent.dragStart(row, { dataTransfer });
      expect(dataTransfer.types).toEqual([tableSource.mime]);
      expect(JSON.parse(dataTransfer.getData(tableSource.mime))).toEqual(tableSource.payload);
      fireEvent.drop(destinationView.container.querySelector(".semio-table-host") as HTMLElement, { dataTransfer });
      expect(destinationAction).toHaveBeenCalledTimes(1);
      expect(destinationAction.mock.calls[0]![0]).toEqual(fixture.table.expectedAction);
    } finally {
      sourceView.unmount();
      destinationView.unmount();
    }

    const blockNode = {
      surfaceId: fixture.blockList.surfaceId,
      controllerId: "controller.block-list",
      blockList: {
        steps: fixture.blockList.steps,
        palette: fixture.blockList.palette,
      },
    };
    for (const driverCase of fixture.drivers) {
      const driver = driverCase.drag === "surface" ? COMPACT_UI_DRIVER : DEFAULT_UI_DRIVER;
      const actions = vi.fn();
      const view = render(createElement(UiDriverProvider, { driver }, createElement(BlockListHost, { node: blockNode as never, onAction: actions })));
      try {
        const expectedHandleCount = driverCase.drag === "handle" ? fixture.blockList.steps.length + fixture.blockList.steps.reduce((count, step) => count + step.blocks.length, 0) + fixture.blockList.palette.length : 0;
        expect(view.container.querySelectorAll('[data-slot="drag-handle"]')).toHaveLength(expectedHandleCount);
        const paletteRow = view.container.querySelector(".semio-palette > div") as HTMLDivElement;
        expect(paletteRow.draggable).toBe(driverCase.drag === "surface");
      } finally {
        view.unmount();
      }
    }

    const rect = (top: number) => ({ top, bottom: top + 20, left: 0, right: 100, width: 100, height: 20 });
    const collisionWinner = (ids: readonly string[], targetIndex: number) => {
      const droppableRects = new Map(ids.map((id, index) => [id, rect(index * 40)]));
      const droppableContainers = ids.map((id) => ({ id, disabled: false, data: { current: {} }, node: { current: null }, rect: { current: null } }));
      return closestCenter({ collisionRect: rect(targetIndex * 40), droppableRects, droppableContainers } as never)[0]?.id;
    };
    expect(
      collisionWinner(
        fixture.blockList.steps.map((step) => step.id),
        1,
      ),
    ).toBe("publish");
    expect(
      collisionWinner(
        fixture.blockList.steps[0]!.blocks.map((block) => block.id),
        1,
      ),
    ).toBe("clean");
    expect(fixture.journeys.find((journey) => journey.id === "step-prepare-after-publish")?.expectedAction).toEqual({ controllerId: "controller.block-list", action: "moveStep", args: { stepId: "prepare", index: 1 } });
    expect(fixture.journeys.find((journey) => journey.id === "block-load-after-clean")?.expectedAction).toEqual({
      controllerId: "controller.block-list",
      action: "moveBlock",
      args: { blockId: "load", fromStepId: "prepare", toStepId: "prepare", index: 1 },
    });
  });

  it("the shared shadow fixture matches Three's directional camera and IconRender caster contract", async () => {
    
    

    const { sunPositionFromAzimuthElevation } = await import("@semio-tech/ui-react");
    const environment = world3dShadowFixture.worldEnvironment;
    const sun = new THREE.DirectionalLight(environment.sun.color, environment.sun.intensity);
    sun.position.fromArray(sunPositionFromAzimuthElevation(environment.sun.azimuth, environment.sun.elevation));
    sun.castShadow = environment.shadow.enabled;
    sun.shadow.mapSize.set(world3dShadowFixture.iconRender.mapSize, world3dShadowFixture.iconRender.mapSize);
    sun.updateMatrixWorld();
    sun.target.updateMatrixWorld();
    sun.shadow.updateMatrices(sun);

    expect(sun.castShadow).toBe(true);
    expect(sun.shadow.mapSize.toArray()).toEqual([1024, 1024]);
    expect(sun.shadow.matrix.elements).toEqual(world3dShadowFixture.oracle.shadowMatrix.map((value) => expect.closeTo(value, 12)));
    expect([sun.shadow.camera.left, sun.shadow.camera.right, sun.shadow.camera.top, sun.shadow.camera.bottom, sun.shadow.camera.near, sun.shadow.camera.far]).toEqual([-5, 5, 5, -5, 0.5, 500]);

    const subject = new THREE.Mesh(new THREE.BoxGeometry(), new THREE.MeshStandardMaterial());
    subject.castShadow = environment.shadow.enabled && world3dShadowFixture.iconRender.materialPresent;
    subject.receiveShadow = environment.shadow.enabled && world3dShadowFixture.iconRender.materialPresent;
    expect([subject.castShadow, subject.receiveShadow]).toEqual([true, true]);
  });

  it("the neutral exact-shadow corpus matches React roles and current Three PCF", () => {
    const fixture = world3dShadowParityFixture;
    
    
    const worldSun = new THREE.DirectionalLight();
    expect(worldSun.shadow.mapSize.toArray()).toEqual([fixture.profiles.world.mapSize, fixture.profiles.world.mapSize]);
    expect(worldSun.shadow.bias).toBe(fixture.profiles.world.bias);
    expect(worldSun.shadow.normalBias).toBe(fixture.profiles.world.normalBias);
    expect(worldSun.shadow.radius).toBe(fixture.profiles.world.radius);
    expect(worldSun.shadow.intensity).toBe(fixture.profiles.world.intensity);
    expect(worldSun.shadow.intensity).not.toBe(fixture.profiles.world.opacityInput);
    expect(worldSun.shadow.radius).not.toBe(fixture.profiles.world.softnessInput);
    expect(fixture.profiles.world.consumesOpacity).toBe(false);
    expect(fixture.profiles.world.consumesSoftness).toBe(false);

    const iconSun = new THREE.DirectionalLight();
    iconSun.shadow.mapSize.set(fixture.profiles.iconPng.mapSize, fixture.profiles.iconPng.mapSize);
    expect(iconSun.shadow.mapSize.toArray()).toEqual([1024, 1024]);
    expect(fixture.profiles.iconPng.usesShadowMap).toBe(true);
    expect(fixture.profiles.iconSvg.usesShadowMap).toBe(false);

    for (const row of fixture.roles) {
      const mesh = new THREE.Mesh(new THREE.BoxGeometry(), new THREE.MeshStandardMaterial({ transparent: row.transparent }));
      if (row.kind === "glb") {
        mesh.castShadow = true;
        mesh.receiveShadow = true;
      } else if (row.kind === "terrain") {
        mesh.receiveShadow = true;
      } else if (row.kind === "icon" && row.materialPresent) {
        mesh.castShadow = true;
        mesh.receiveShadow = true;
      }
      expect([mesh.castShadow, mesh.receiveShadow], row.kind).toEqual([row.casts, row.receives]);
    }

    const cameraRow = fixture.frusta.mainCamera;
    const camera = new THREE.PerspectiveCamera(cameraRow.fov, cameraRow.aspect, cameraRow.near, cameraRow.far);
    camera.position.fromArray(cameraRow.position);
    camera.up.fromArray(cameraRow.up);
    camera.lookAt(new THREE.Vector3().fromArray(cameraRow.target));
    camera.updateProjectionMatrix();
    camera.updateMatrixWorld();
    const mainFrustum = new THREE.Frustum().setFromProjectionMatrix(new THREE.Matrix4().multiplyMatrices(camera.projectionMatrix, camera.matrixWorldInverse));

    const sunRow = fixture.frusta.sun;
    const azimuth = THREE.MathUtils.degToRad(sunRow.azimuth);
    const elevation = THREE.MathUtils.degToRad(sunRow.elevation);
    const shadowSun = new THREE.DirectionalLight();
    shadowSun.position.set(Math.cos(elevation) * Math.cos(azimuth) * sunRow.distance, Math.cos(elevation) * Math.sin(azimuth) * sunRow.distance, Math.sin(elevation) * sunRow.distance);
    shadowSun.updateMatrixWorld();
    shadowSun.target.updateMatrixWorld();
    shadowSun.shadow.updateMatrices(shadowSun);
    const lightFrustum = shadowSun.shadow.getFrustum();
    const box = (bounds: { readonly min: readonly number[]; readonly max: readonly number[] }) => new THREE.Box3(new THREE.Vector3().fromArray(bounds.min), new THREE.Vector3().fromArray(bounds.max));
    const caster = box(fixture.frusta.offscreenCasterBounds);
    const receiver = box(fixture.frusta.visibleReceiverBounds);
    expect(mainFrustum.intersectsBox(caster)).toBe(fixture.frusta.offscreenCasterMainVisible);
    expect(lightFrustum.intersectsBox(caster)).toBe(fixture.frusta.offscreenCasterLightVisible);
    expect(mainFrustum.intersectsBox(receiver)).toBe(fixture.frusta.visibleReceiverMainVisible);
    expect(lightFrustum.intersectsBox(receiver)).toBe(fixture.frusta.visibleReceiverLightVisible);

    const pcfSource = THREE.ShaderChunk.shadowmap_pars_fragment;
    expect(pcfSource).toContain("interleavedGradientNoise( gl_FragCoord.xy )");
    expect(pcfSource).toContain("vogelDiskSample( 4, 5, phi )");
    expect(pcfSource).toContain("shadowCoord.x >= 0.0 && shadowCoord.x <= 1.0");
    expect(pcfSource).toContain("shadowCoord.z <= 1.0");
    const fract = (value: number) => value - Math.floor(value);
    const [fragmentX, fragmentY] = fixture.pcf.fragmentPosition;
    const noise = fract(52.9829189 * fract(fragmentX * 0.06711056 + fragmentY * 0.00583715));
    const rotation = noise * 6.28318530718;
    const offsets = Array.from({ length: fixture.pcf.sampleCount }, (_, index) => {
      const radius = Math.sqrt((index + 0.5) / fixture.pcf.sampleCount);
      const theta = index * 2.399963229728653 + rotation;
      return [Math.cos(theta) * radius, Math.sin(theta) * radius];
    });
    expect(noise).toBeCloseTo(fixture.pcf.interleavedGradientNoise, 12);
    expect(rotation).toBeCloseTo(fixture.pcf.rotation, 12);
    expect(offsets).toEqual(fixture.pcf.unitOffsets.map((row) => row.map((value) => expect.closeTo(value, 12))));
  });
});
//#endregion 🎫️RemainingReds

it("the neutral gizmo head corpus matches Three sprite scale and circle bounds", () => {
  const camera = new THREE.PerspectiveCamera();
  camera.position.fromArray(gizmoTipBoundsFixture.camera.position);
  camera.up.fromArray(gizmoTipBoundsFixture.camera.up);
  camera.lookAt(new THREE.Vector3().fromArray(gizmoTipBoundsFixture.camera.target));
  camera.updateMatrixWorld(true);
  const viewRotation = new THREE.Matrix4().extractRotation(camera.matrixWorldInverse);
  const axes = [
    [1, 0, 0],
    [-1, 0, 0],
    [0, 1, 0],
    [0, -1, 0],
    [0, 0, 1],
    [0, 0, -1],
  ];
  for (const [index, axis] of axes.entries()) {
    const point = new THREE.Vector3().fromArray(axis).applyMatrix4(viewRotation);
    expect([point.x * gizmoTipBoundsFixture.groupScale, -point.y * gizmoTipBoundsFixture.groupScale, point.z]).toEqual(gizmoTipBoundsFixture.axisOffsets[index].map((value) => expect.closeTo(value, 6)));
  }
  for (const row of gizmoTipBoundsFixture.cases) {
    const sprite = new THREE.Sprite();
    const group = new THREE.Group();
    group.scale.setScalar(gizmoTipBoundsFixture.groupScale);
    sprite.scale.setScalar((row.prominent ? 1 : 0.65) * gizmoTipBoundsFixture.axisHeadScale * (row.hovered ? 1.1 : 1));
    group.add(sprite);
    group.updateMatrixWorld(true);
    const radius = (sprite.getWorldScale(new THREE.Vector3()).x * (row.prominent ? 16 : 12)) / gizmoTipBoundsFixture.textureSize;
    expect(radius).toBeCloseTo(row.radius, 8);
    const geometry = new THREE.CircleGeometry(radius, 64);
    geometry.computeBoundingBox();
    const size = geometry.boundingBox!.getSize(new THREE.Vector3());
    expect(size.x).toBeCloseTo(row.radius * 2, 5);
    expect(size.y).toBeCloseTo(row.radius * 2, 5);
    expect(size.x).toBeLessThanOrEqual(gizmoTipBoundsFixture.maxHeadDiameter + 0.00001);
    geometry.dispose();
    sprite.material.dispose();
  }
});

describe("shared VFS descriptor presentation", () => {
  for (const law of vfsDescriptorFixture.cases)
    it(`renders the actual React VFS descriptor ${law.id}`, () => {
      const kind = vfsDescriptorFixture.kinds[law.kind as keyof typeof vfsDescriptorFixture.kinds] as DescriptorKind;
      const value = law.value === null ? undefined : (law.value as FileNodeDescriptorValue);
      const view = render(createElement("div", null, renderVirtualFileSystemDescriptorCell(kind, value, "en-US")));
      try {
        expect(view.container.textContent).toBe(law.expect.text);
        if (value?.presentation === "avatar" && kind.presentation === "avatar") {
          expect(view.container.querySelector('[data-slot="avatar-fallback"]')?.getAttribute("aria-label")).toBe(law.expect.name);
          expect(view.container.querySelector('[data-slot="avatar"]')?.classList.contains("rounded-full")).toBe(true);
          if (value.icon) expect(view.container.querySelector("img")?.getAttribute("src")).toBe(law.expect.icon);
        }
      } finally {
        view.unmount();
      }
    });
});

describe("📣️ replay refusal vocabulary", () => {
  it("names every reason in both tongues from the shared vocabulary the wgpu shell reads", () => {
    expect(Object.keys(REPLAY_REFUSAL_LABELS_V1).sort()).toEqual(Object.keys(replayRefusalVocabulary.reasons).sort());
    for (const label of Object.values(REPLAY_REFUSAL_LABELS_V1)) expect(label.en !== "" && label.de !== "" && label.en !== label.de).toBe(true);
  });

  it("answers the shared vectors the wgpu shell's law answers", () => {
    expect(replayRefusalVocabulary.vectors.length).toBeGreaterThan(0);
    for (const vector of replayRefusalVocabulary.vectors) {
      const reason = vector.reason as ReplayRefusalReasonV1;
      expect([replayRefusalNoticeTextV1(reason, vector.locale), replayRefusalCodeV1(reason)]).toEqual([vector.text, vector.code]);
    }
  });
});

describe("shell explicit locale source", () => {
  it("initializes each unlocked pane from its own explicit user language", () => {
    for (const row of explicitLocaleFixture.locales) {
      const locale = new Intl.Locale(row.locale).language as "en" | "de";
      const storage = createMemoryStoragePort();
      const state = initialShellState({locale, plugins:[], storage});
      expect(state.uiPrefs.uiLocale).toBe(locale);
      expect(readUiPreferences(storage).locale).toBeNull();
    }
  });
});
