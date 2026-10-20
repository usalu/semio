/** 🟠️ The scopes suggestion menu ownership tests of the framework renderer hosts suite. */
import { expect, it } from "vitest";
import { worldInstanceHighlighted } from "../../../🧱️elements/🌐️World3dHost/🎨️mesh-render/🖌️mesh-style-palette/🟦️.ts";
import { surfaceContextMenuTitleKey, worldGhostMeshUrl, resolveMeshStyle, resolveMeshSelectionPreviewStyle, semanticColorsFromPalette, celebrateWorldInstances, isWorldInstanceCelebrating, isCurveOnlyWorldMesh, meshBoundsCorners, worldMeshMaterialRevision, worldVortexMaterialRevision, resolveWorldMergeMode, resolveWorldContextMenuTarget, world3dContextMenuSurfaceV1 } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { suggestionMenuOwnsWindow } from "../../../🧱️elements/🎣️suggestion-submenu/🟦️.ts";

/** 🟡 Registers the scopes suggestion menu ownership tests of the framework renderer hosts suite. */
export function registerScopesSuggestionMenuOwnershipTests(): void {
  it("scopes suggestion menu ownership to the opening world window so sibling panes stay interactive", () => {
    expect(suggestionMenuOwnsWindow(null, "puzzle3d-main-top")).toBe(false);
    expect(suggestionMenuOwnsWindow({ open: false, windowId: "puzzle3d-main-top" }, "puzzle3d-main-top")).toBe(false);
    expect(suggestionMenuOwnsWindow({ open: true, windowId: "puzzle3d-main-top" }, "puzzle3d-main-top")).toBe(true);
    expect(suggestionMenuOwnsWindow({ open: true, windowId: "puzzle3d-main-top" }, "puzzle3d-main-perspective")).toBe(false);
    expect(suggestionMenuOwnsWindow({ open: true }, "puzzle3d-main-perspective")).toBe(true);
  });
  it("revisions vortex materials when selection or hover state changes", () => {
    expect(worldVortexMaterialRevision()).toBe("neutral");
    expect(worldVortexMaterialRevision(false, true)).toBe("hovered");
    expect(worldVortexMaterialRevision(true, true)).toBe("selected");
  });
  it("revisions world mesh materials when style kind changes so deselection clears selected paint", () => {
    expect(worldMeshMaterialRevision("selected")).toBe("selected");
    expect(worldMeshMaterialRevision("neutral")).toBe("neutral");
    expect(worldMeshMaterialRevision("hovered")).toBe("hovered");
    expect(worldMeshMaterialRevision(resolveMeshStyle({ selected: false, hovered: false }))).toBe("neutral");
  });
  it("uses the world surface selection mode instead of a stale shared invertive mode", () => {
    expect(resolveWorldMergeMode("replace", {}, "invertive")).toBe("replace");
    expect(resolveWorldMergeMode("replace", { shiftKey: true }, "invertive")).toBe("additive");
    expect(resolveWorldMergeMode("invertive", {}, "invertive")).toBe("invertive");
  });
  it("resolves mesh style by priority: disabled > celebrated > selected > highlighted > hovered > neutral", () => {
    expect(resolveMeshStyle({})).toBe("neutral");
    expect(resolveMeshStyle({ hovered: true })).toBe("hovered");
    expect(resolveMeshStyle({ hovered: true, highlighted: true })).toBe("highlighted");
    expect(resolveMeshStyle({ highlighted: true, selected: true })).toBe("selected");
    expect(resolveMeshStyle({ selected: true, celebrating: true })).toBe("celebrated");
    expect(resolveMeshStyle({ celebrating: true, disabled: true })).toBe("disabled");
    expect(resolveMeshStyle({ selected: true, disabled: true })).toBe("disabled");
    expect(resolveMeshStyle({ disabled: true, selected: true, highlighted: true, hovered: true, celebrating: true })).toBe("disabled");
  });
  it("paints the guest-stamped highlight (a history draft's reference) whether or not the host's own kind hover is on", () => {
    expect(worldInstanceHighlighted(false, { highlighted: true })).toBe(true);
    expect(worldInstanceHighlighted(true, {})).toBe(true);
    expect(worldInstanceHighlighted(false, {})).toBe(false);
    expect(resolveMeshSelectionPreviewStyle({ hovered: true, highlighted: worldInstanceHighlighted(false, { highlighted: true }) })).toBe("highlighted");
  });
  it("celebrateWorldInstances stamps ids and cancel clears them so paint prefers celebrated over selected", () => {
    const cancel = celebrateWorldInstances(["drop-1"], 60_000);
    expect(isWorldInstanceCelebrating("drop-1")).toBe(true);
    expect(resolveMeshStyle({ selected: true, celebrating: isWorldInstanceCelebrating("drop-1") })).toBe("celebrated");
    cancel();
    expect(isWorldInstanceCelebrating("drop-1")).toBe(false);
    expect(resolveMeshStyle({ selected: true, celebrating: isWorldInstanceCelebrating("drop-1") })).toBe("selected");
  });
  it("maps edge hover to line paint so coplanar edges stay distinct from face hover fill", () => {
    const palette = {
      neutral: { meshColor: "#111111", lineColor: "#222222", emissiveIntensity: 0, opacity: 1 },
      hovered: { meshColor: "#aaaaaa", lineColor: "#333333", emissiveIntensity: 0.08, opacity: 1 },
      selected: { meshColor: "#0000ff", lineColor: "#0000ff", emissiveIntensity: 0.35, opacity: 1 },
      highlighted: { meshColor: "#00ff00", lineColor: "#00ff00", emissiveIntensity: 0.2, opacity: 1 },
      celebrated: { meshColor: "#ff00ff", lineColor: "#ff00ff", emissiveIntensity: 0.55, opacity: 1 },
      disabled: { meshColor: "#999999", lineColor: "#888888", emissiveIntensity: 0, opacity: 0.45 },
    } as Parameters<typeof semanticColorsFromPalette>[0];
    const colors = semanticColorsFromPalette(palette);
    expect(colors.hover).toBe("#00ff00");
    expect(colors.edgeHover).toBe("#00ff00");
    expect(colors.edgeHover).toBe(colors.hover);
    expect(colors.select).toBe("#0000ff");
  });
  it("treats centerline meshes without shaded triangles as curve-only instances", () => {
    expect(isCurveOnlyWorldMesh({ indices: [], edgePositions: [0, 0, 0, 1, 0, 0] })).toBe(true);
    expect(isCurveOnlyWorldMesh({ indices: [0, 1, 2], edgePositions: [0, 0, 0, 1, 0, 0] })).toBe(false);
    expect(isCurveOnlyWorldMesh({ indices: [], edgePositions: [] })).toBe(false);
  });
  it("derives marquee bounds from edge samples when positions are empty", () => {
    const corners = meshBoundsCorners({
      positions: [],
      normals: [],
      indices: [],
      edgePositions: [0, 0, 0, 2, 4, 6],
    } as Parameters<typeof meshBoundsCorners>[0]);
    expect(corners).toContainEqual([0, 0, 0]);
    expect(corners).toContainEqual([2, 4, 6]);
  });
  it("renders the new group selection as active and only objects leaving the old selection as highlighted", () => {
    expect(resolveMeshSelectionPreviewStyle({ selected: false }, true)).toBe("selected");
    expect(resolveMeshSelectionPreviewStyle({ selected: true }, true)).toBe("selected");
    expect(resolveMeshSelectionPreviewStyle({ selected: true }, false)).toBe("highlighted");
    expect(resolveMeshSelectionPreviewStyle({ selected: false }, false)).toBe("neutral");
    expect(resolveMeshSelectionPreviewStyle({ selected: true, disabled: true }, false)).toBe("disabled");
  });
  it("resolves catalogue-drop ghost mesh URLs even when the kind is not yet among scene meshes", () => {
    expect(worldGhostMeshUrl({ meshUrl: "/meshes/new-kind.glb" }, [])).toBe("/meshes/new-kind.glb");
    expect(worldGhostMeshUrl({ meshUrl: "/meshes/placed.glb" }, [{ url: "/meshes/placed.glb" }])).toBe("/meshes/placed.glb");
    expect(worldGhostMeshUrl({}, [{ url: "/meshes/placed.glb" }])).toBeUndefined();
  });
  it("resolves the right-click context menu target by priority: vortex, then object, then reference", () => {
    expect(resolveWorldContextMenuTarget({ hoveredVortexFullId: "seed-left-001:v0" }, { hoveredComponent: { objectId: "obj-1" }, hoveredId: "reference:ref-1" })).toEqual({
      kind: "vortex",
      id: "seed-left-001:v0",
    });
    expect(resolveWorldContextMenuTarget({}, { hoveredComponent: { objectId: "obj-1" }, hoveredId: "reference:ref-1" })).toEqual({ kind: "object", id: "obj-1" });
    expect(resolveWorldContextMenuTarget({}, { hoveredId: "reference:ref-1" })).toEqual({ kind: "reference", id: "ref-1" });
    expect(resolveWorldContextMenuTarget({}, { hoveredId: "obj-1" })).toEqual({ kind: "object", id: "obj-1" });
    expect(resolveWorldContextMenuTarget({}, {})).toBeNull();
  });
  it("carries the right-clicked world entity as the request's own hit, so no target-recording dispatch precedes the menu", () => {
    expect(world3dContextMenuSurfaceV1({ kind: "vortex", id: "seed-left-001:v0" }, { ids: ["seed-left-001"], componentIds: [] })).toEqual({
      hits: [{ domain: "vortex", id: "seed-left-001:v0" }],
      selection: [{ domain: "object", ids: ["seed-left-001"] }],
    });
    expect(world3dContextMenuSurfaceV1(null, { ids: [], componentIds: [7] })).toEqual({ hits: [], selection: [{ domain: "feature", ids: ["7"] }] });
    expect(world3dContextMenuSurfaceV1(null, { ids: [], componentIds: [] })).toEqual({ hits: [], selection: [] });
  });
  it("titles context menus from the specific hit before falling back to the surface", () => {
    const request = (domain?: string, kind = "world3d") => ({
      menu: { id: kind, args: null },
      surface: { surfaceId: "surface", kind, hits: domain ? [{ domain, id: "target" }] : [] },
    });

    expect(surfaceContextMenuTitleKey(request("vortex"))).toBe("ui.surfaceContextMenu.vortex");
    expect(surfaceContextMenuTitleKey(request("object"))).toBe("ui.surfaceContextMenu.object");
    expect(surfaceContextMenuTitleKey(request("reference"))).toBe("ui.surfaceContextMenu.reference");
    expect(surfaceContextMenuTitleKey(request())).toBe("ui.surfaceContextMenu.scene");
    expect(surfaceContextMenuTitleKey(request(undefined, "board2d"))).toBe("ui.surfaceContextMenu.board");
  });
  it("covers every target domain emitted by current surface pickers", () => {
    const domains = ["architecture", "attraction", "block", "edge", "entry", "feature", "group", "handle", "layer", "node", "object", "part", "path", "pixel", "position", "reference", "route", "row", "slider", "vortex"];
    for (const domain of domains) {
      expect(surfaceContextMenuTitleKey({ menu: { id: "surface", args: null }, surface: { surfaceId: "surface", kind: "unknown", hits: [{ domain, id: "target" }] } })).toBe(`ui.surfaceContextMenu.${domain}`);
    }
  });
}
