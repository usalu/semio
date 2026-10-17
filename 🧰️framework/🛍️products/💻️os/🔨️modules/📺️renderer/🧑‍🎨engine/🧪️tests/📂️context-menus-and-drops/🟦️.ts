/** 📂 Context menu mapping, tiled-map snapshot ABI and catalogue drag payloads. */
import { uiI18n } from "@semio-tech/ui-react";
import { contextMenuItemClassName } from "../../../../../../../🔨️modules/🖱️ui/🧱️elements/🖱️ContextMenu/🟦️.tsx";
import { cleanup } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { uiDataLabel, formatKeybindingShortcut, buildKeysByActionId } from "@semio-tech/ui-react";
import { mapContextMenuSpecs, suggestionMenuItems, parseBoardCatalogueDragPayload, boardDropPreviewJson, resolveMapInteractionSync, parsePuzzle3dCatalogueDragPayload, snapWorldPointToGrid } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import "../🧰️engine-contract-helpers/🟦️.ts";

describe("framework renderer hosts", () => {
  afterEach(() => cleanup());

  it("maps context menu specs onto UI items with icons, colors, hover, and select handlers", () => {
    const dispatch = vi.fn();
    const items = mapContextMenuSpecs(
      [
        { id: "hide", label: "Hide", icon: "eye-off", color: "#ff0000", action: "setFlag", args: { flag: "hidden" }, hoverAction: "hoverFlag", hoverArgs: { index: 1 } },
        { id: "sep", separator: true },
        { id: "delete", label: "Delete", icon: "trash", destructive: true, action: "deleteSelection" },
      ],
      dispatch,
    );
    expect(items[0]).toMatchObject({ id: "hide", label: "Hide", icon: "eye-off", color: "#ff0000" });
    items[0]?.onSelect?.(new Event("select"));
    items[0]?.onHover?.();
    expect(dispatch).toHaveBeenCalledWith("setFlag", { flag: "hidden" });
    expect(dispatch).toHaveBeenCalledWith("hoverFlag", { index: 1 });
    expect(items[1]).toMatchObject({ id: "sep", separator: true });
    expect(items[2]).toMatchObject({ id: "delete", destructive: true, icon: "trash" });
  });

  it("keeps exportSnapshot and importSnapshot menu rows on the same leaf onSelect bind", () => {
    const dispatch = vi.fn();
    const items = mapContextMenuSpecs(
      [
        { id: "shell-menu.action.exportSnapshot", label: "Export", action: "exportSnapshot" },
        { id: "shell-menu.action.importSnapshot", label: "Import", action: "importSnapshot" },
        { id: "shell-menu.action.openImportSnapshot", label: "Import…", action: "openImportSnapshot" },
      ],
      dispatch,
    );
    expect(items.map((item) => ({ id: item.id, action: item.action, hasSelect: typeof item.onSelect === "function", children: item.children?.length ?? 0 }))).toEqual([
      { id: "shell-menu.action.exportSnapshot", action: "exportSnapshot", hasSelect: true, children: 0 },
      { id: "shell-menu.action.importSnapshot", action: "importSnapshot", hasSelect: true, children: 0 },
      { id: "shell-menu.action.openImportSnapshot", action: "openImportSnapshot", hasSelect: true, children: 0 },
    ]);
    items[0]?.onSelect?.(new Event("select"));
    items[1]?.onSelect?.(new Event("select"));
    expect(dispatch).toHaveBeenCalledWith("exportSnapshot", {});
    expect(dispatch).toHaveBeenCalledWith("importSnapshot", {});
  });

  it("keeps context-menu leaf rows pointer-hittable under dimmed window chrome", () => {
    expect(contextMenuItemClassName({})).toContain("pointer-events-auto");
  });

  it("maps suggestion-style specs without a color swatch field", () => {
    const dispatch = vi.fn();
    const items = mapContextMenuSpecs(
      [
        {
          id: "suggestion-0",
          label: "Capsule · vortex 0",
          icon: "box",
          checked: true,
          action: "acceptSuggestion",
          args: { index: 0, fullId: "obj:v0" },
          hoverAction: "hoverSuggestion",
          hoverArgs: { index: 0 },
        },
      ],
      dispatch,
    );
    expect(items[0]).toMatchObject({ id: "suggestion-0", icon: "box", checked: true });
    expect(items[0]).not.toHaveProperty("color", expect.anything());
    expect(items[0]?.color).toBeUndefined();
  });

  it("enriches context menu shortcuts from app keybindings", () => {
    const keysByActionId = buildKeysByActionId([
      { action: { action: "deleteSelection" }, keys: "delete,backspace" },
      { action: { action: "duplicateSelection" }, keys: "mod+d" },
    ]);
    const items = mapContextMenuSpecs(
      [
        { id: "duplicate", label: "Duplicate", action: "duplicateSelection" },
        { id: "delete", label: "Delete", action: "deleteSelection" },
        { id: "custom", label: "Custom", action: "customAction", shortcut: "F2" },
      ],
      vi.fn(),
      keysByActionId,
    );
    expect(items[0]?.shortcut).toMatch(/D$/);
    expect(items[1]?.shortcut).toBe("⌦️");
    expect(items[2]?.shortcut).toBe("F2");
  });

  it("formats keybinding chords for menu shortcut labels", () => {
    expect(formatKeybindingShortcut("backspace")).toBe("⌫️");
    expect(formatKeybindingShortcut("delete,backspace")).toBe("⌦️");
    expect(formatKeybindingShortcut("mod+d")).toMatch(/D$/);
  });

  it("numbers suggestion menu rows with digit shortcuts for the first nine candidates", () => {
    const items = suggestionMenuItems(
      {
        pending: false,
        candidates: [
          { index: 2, objectLabel: "Capsule", vortexLabel: "port-a", icon: "box" },
          { index: 5, objectLabel: "Box", vortexLabel: "port-b", icon: "box" },
        ],
        vortexFullId: "obj:v0",
      },
      2,
      { checkingPlacement: uiDataLabel("Checking placement…"), noPlacement: uiDataLabel("No placement") },
    );
    expect(items[0]).toMatchObject({ checked: true });
    expect(items[1]).toMatchObject({ checked: false });
    expect(items[0]?.shortcut).toBeUndefined();
  });

  it("resolves taxonomy group rows from the chrome ribbon-parent bundle in both locales", async () => {
    // 🗂️ The guest emits `menu.group.<category>` with `label: undefined` by contract (the host owns the
    // chrome taxonomy vocabulary), and the React shell used to render the raw id — `menu.group.history`,
    // `menu.group.selection`, … — where the wgpu target resolved a real label through
    // `ribbon_parent_label`. The label must come from the SAME `ui.ribbon.parent.*` bundle the ribbon
    // reads, so EN and DE both follow the active locale with no second string table.
    const specs = [
      { id: "menu.group.history", children: [{ id: "undo", label: "Undo", action: "undo" }] },
      { id: "menu.group.selection", children: [{ id: "duplicate", label: "Duplicate", action: "duplicateSelection" }] },
      { id: "menu.group.more", children: [{ id: "fit", label: "Fit", action: "fitWorld" }] },
    ];
    await uiI18n.changeLanguage("en");
    const english = mapContextMenuSpecs(specs, () => {});
    expect(english.map((item) => item.label)).toEqual(["History", "Selection", "More"]);
    expect(english.map((item) => item.icon)).toEqual(["folder", "folder", "folder"]);
    expect(english[0]?.children?.[0]).toMatchObject({ id: "undo", label: "Undo" });

    await uiI18n.changeLanguage("de");
    expect(mapContextMenuSpecs(specs, () => {}).map((item) => item.label)).toEqual(["Verlauf", "Auswahl", "Mehr"]);
    await uiI18n.changeLanguage("en");

    // 🗂️ Only group rows are resolved: an ordinary label-less row (a bare separator) keeps none, and an
    // unknown category resolves to nothing rather than inventing chrome vocabulary.
    const untouched = mapContextMenuSpecs(
      [
        { id: "sep", separator: true },
        { id: "menu.group.not-a-category", children: [{ id: "x", label: "X" }] },
      ],
      () => {},
    );
    expect(untouched[0]?.label).toBeUndefined();
    expect(untouched[1]?.label).toBeUndefined();
  });

  it("enriches context menu shortcuts from app keybindings via mapContextMenuSpecs", () => {
    const keys = new Map([["deleteSelection", "delete,backspace"]]);
    const items = mapContextMenuSpecs([{ id: "delete-selection", label: "Delete Selection (8 nodes and 13 edges)", action: "deleteSelection", destructive: true }], () => {}, keys);
    expect(items[0]?.shortcut).toBeTruthy();
    expect(items[0]?.label).toContain("8 nodes and 13 edges");
  });

  it("maps tiled-map interaction snapshots onto the current wasm sync ABI", () => {
    const mapLayers = [{ id: "positions", geometry: "point", granularity: "position" }, { id: "routes", geometry: "line", granularity: "route" }];
    expect(resolveMapInteractionSync('{"positions":["position-1"],"routes":[]}', '{"kind":"position","id":"position-2"}', mapLayers)).toEqual({
      granularity: "position",
      selectedIdsJson: '["position-1"]',
      hoveredId: "position-2",
    });
    expect(resolveMapInteractionSync('{"positions":[],"routes":["route-1"]}', '{"kind":"position","id":"position-2"}', mapLayers)).toEqual({
      granularity: "route",
      selectedIdsJson: '["route-1"]',
    });
  });

  it("parses a catalogue drag payload and builds a drop-preview JSON", () => {
    const encoded = JSON.stringify({ kindId: "seed", catalogSlice: "nodes", shape: "circle", radius: 24 });
    const payload = parseBoardCatalogueDragPayload(encoded);
    expect(payload).toEqual({ kindId: "seed", catalogSlice: "nodes", shape: "circle", radius: 24, width: undefined, height: undefined, iconKind: undefined });
    expect(payload).not.toBeNull();
    expect(JSON.parse(boardDropPreviewJson(payload!, 100, 200))).toMatchObject({ nodeKind: "seed", x: 100, y: 200, shape: "circle", radius: 24 });
  });

  it("rejects a catalogue drag payload without a kindId", () => {
    expect(parseBoardCatalogueDragPayload(JSON.stringify({ catalogSlice: "nodes" }))).toBeNull();
    expect(parseBoardCatalogueDragPayload(null)).toBeNull();
  });

  it("parses a puzzle 3d catalogue drag payload and snaps drop origins to the grid", () => {
    const encoded = JSON.stringify({ objectKind: "Capsule", meshUrl: "puzzle3d://capsule" });
    expect(parsePuzzle3dCatalogueDragPayload(encoded)).toEqual({ objectKind: "Capsule", meshUrl: "puzzle3d://capsule" });
    expect(snapWorldPointToGrid([1.2, 2.7, 0.0], true, 1)).toEqual([1, 3, 0]);
    expect(snapWorldPointToGrid([1.2, 2.7, 0.0], false, 1)).toEqual([1.2, 2.7, 0]);
    expect(parsePuzzle3dCatalogueDragPayload(JSON.stringify({ meshUrl: "puzzle3d://capsule" }))).toBeNull();
  });
});
