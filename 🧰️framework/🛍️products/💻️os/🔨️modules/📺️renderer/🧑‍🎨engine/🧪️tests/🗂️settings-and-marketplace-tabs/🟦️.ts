/** 🗂 Display windows tab, settings and marketplace panel tabs and window measure trees. */
import { createElement, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { singleTreeLeaf } from "@semio-tech/ui-react";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { describe, expect, it } from "vitest";
import { type WindowMeasure } from "@semio-tech/framework";
import { integrateAppSettingsPanelTabsIntoFrameworkBranch, shellTabIcon, createFrameworkDisplayPanelTabs, type DisplayHostApi, createFrameworkSettingsPanelTab, createFrameworkMarketplacePanelTab, type MarketplaceExtensionEntry, type MarketplaceHostApi, type MarketplacePluginEntry, type PluginPanelStatus, introductionTargetsWindow, windowMeasureTreeContainsId, renderWindowMeasuresTree } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { decodeWorldProjectionTemplateId } from "@semio-tech/infinite-world-r3f";
import "../🧰️engine-contract-helpers/🟦️.ts";

describe("Display Windows tab — projection drag templates", () => {
  function windowsTreeSections(windowKinds: DisplayHostApi["windowKinds"]) {
    const host: DisplayHostApi = {
      windowKinds,
      namedLayouts: [],
      userLayouts: [],
      saveCurrentLayout: () => {},
      applyNamedLayout: () => {},
      deleteUserLayout: () => {},
      layoutSaveLabel: "",
      setLayoutSaveLabel: () => {},
    };
    const tabs = createFrameworkDisplayPanelTabs(() => host);
    const windowsTab = tabs.find((tab) => tab.id === "framework.display.windows");
    if (!windowsTab || windowsTab.kind !== "leaf") throw new Error("Display fixture requires the windows tree leaf");
    const source = windowsTab.trees[0]?.tree;
    if (!source) throw new Error("Display fixture requires the windows tree source");
    return ("resolveTree" in source ? source.resolveTree() : source).sections;
  }

  type LabeledTreeItem = { readonly id: string; readonly label?: string; readonly icon?: unknown; readonly items?: readonly LabeledTreeItem[]; readonly dragData?: Record<string, string> };
  const byLabel = (items: readonly LabeledTreeItem[], label: string) => items.find((row) => row.label === label);

  it("shows window kind icons on section headers and kind rows", () => {
    const sections = windowsTreeSections([{ id: "puzzle2d-overview", label: "Overview", iconId: "layout-grid", surfaceKind: "canvas-2d" }]);
    expect(sections[0]!.icon).toBeTruthy();
    const items = sections[0]!.items as LabeledTreeItem[];
    expect(items[0]?.icon).toBeTruthy();
  });

  it("nests the full Parallel/Perspective projection taxonomy under a world-3d window kind", () => {
    const sections = windowsTreeSections([{ id: "puzzle3d-main", label: "Puzzle 3D", iconId: "puzzle", surfaceKind: "world-3d" }]);
    expect(sections).toHaveLength(1);
    const items = sections[0]!.items as LabeledTreeItem[];
    expect(items).toHaveLength(3);
    expect(items.some((row) => row.id === "framework.display.windows.puzzle3dMain.kind")).toBe(true);
    const parallel = byLabel(items, "Parallel")!;
    const perspective = byLabel(items, "Perspective")!;
    expect(perspective).toBeDefined();
    expect(parallel.items!.map((row) => row.id.split(".").pop()).sort()).toEqual(["axonometric", "oblique", "orthographic"]);
  });

  it("keeps a flat single drag entry for non-world-3d window kinds", () => {
    const sections = windowsTreeSections([{ id: "puzzle2d-overview", label: "Overview", iconId: "layout-grid", surfaceKind: "canvas-2d" }]);
    expect(sections[0]!.items).toHaveLength(1);
  });

  it('pre-reverses every level so the bottom-anchored (direction="up") Tree\'s own sibling-reversal renders Parallel children top-to-bottom', () => {
    const sections = windowsTreeSections([{ id: "puzzle3d-main", label: "Puzzle 3D", iconId: "puzzle", surfaceKind: "world-3d" }]);
    const items = sections[0]!.items as LabeledTreeItem[];
    const parallel = byLabel(items, "Parallel")!;
    // Raw (pre-render) order is reversed once more so that after the Tree's own "up" reversal on render,
    // Parallel reads Orthographic, Axonometric, Oblique — top to bottom.
    expect([...parallel.items!].reverse().map((row) => row.label)).toEqual(["Orthographic", "Axonometric", "Oblique"]);
    const axonometric = byLabel(parallel.items!, "Axonometric")!;
    expect([...axonometric.items!].reverse().map((row) => row.label)).toEqual(["Isometric", "Dimetric", "Trimetric"]);
  });

  it("each projection leaf's drag payload decodes back to its WorldProjectionSpec", () => {
    const sections = windowsTreeSections([{ id: "puzzle3d-main", label: "Puzzle 3D", iconId: "puzzle", surfaceKind: "world-3d" }]);
    const items = sections[0]!.items as LabeledTreeItem[];
    const parallel = byLabel(items, "Parallel")!;
    const orthographic = byLabel(parallel.items!, "Orthographic")!;
    const payload = JSON.parse(orthographic.dragData!["application/x-compose-window-template"]!) as { windowKindId: string; templateId: string };
    expect(payload.windowKindId).toBe("puzzle3d-main");
    expect(decodeWorldProjectionTemplateId(payload.templateId)).toEqual({ mode: { kind: "orthographic" }, orientation: { type: "cardinal", view: "plan" } });
    const axonometric = byLabel(parallel.items!, "Axonometric")!;
    const isometric = byLabel(axonometric.items!, "Isometric")!;
    const isoPayload = JSON.parse(isometric.dragData!["application/x-compose-window-template"]!) as { windowKindId: string; templateId: string };
    expect(decodeWorldProjectionTemplateId(isoPayload.templateId)).toMatchObject({ mode: { kind: "axonometric", variant: "isometric" } });
  });
});

describe("createFrameworkSettingsPanelTab", () => {
  it("exposes one Settings toggle whose children are General, Theme, and Hotkeys tabs", () => {
    const settings = createFrameworkSettingsPanelTab(() => null);
    expect(settings.kind).toBe("branch");
    if (settings.kind !== "branch") throw new Error("Settings must be a branch");
    expect(settings.id).toBe("framework.settings");
    expect(settings.children.map((child) => child.id)).toEqual(["framework.settings.general", "framework.settings.theme", "framework.settings.keybindings"]);
    expect(settings.children.map((child) => child.name)).toEqual(["General", "Theme", "Hotkeys"]);
  });
});

describe("integrateAppSettingsPanelTabsIntoFrameworkBranch", () => {
  it("nests app Settings-group tabs inside framework.settings so the bottom-right anchor carries one Settings toggle", () => {
    const framework = createFrameworkSettingsPanelTab(() => null);
    const appTab = singleTreeLeaf({
      id: "puzzle3d.panel.settings",
      icon: shellTabIcon("settings"),
      name: "Einstellungen",
      order: 0,
      tree: { resolveTree: () => ({ sections: [{ id: "puzzle3d-play-settings", items: [] }] }) },
    });
    const integrated = integrateAppSettingsPanelTabsIntoFrameworkBranch(framework, [appTab]);
    expect(integrated.kind).toBe("branch");
    if (integrated.kind !== "branch") throw new Error("integrated settings must be a branch");
    expect(integrated.id).toBe("framework.settings");
    expect(integrated.children[0]?.id).toBe("puzzle3d.panel.settings");
    expect(integrated.children.map((child) => child.id)).toEqual(["puzzle3d.panel.settings", "framework.settings.general", "framework.settings.theme", "framework.settings.keybindings"]);
  });
});

describe("createFrameworkMarketplacePanelTab", () => {
  type LabeledTreeItem = { readonly id: string; readonly label?: string; readonly loading?: boolean; readonly items?: readonly LabeledTreeItem[]; readonly control?: ReactElement };
  type MarketplaceTreeSections = { readonly sections: readonly { readonly id: string; readonly label?: string; readonly items?: readonly LabeledTreeItem[] }[] };
  type LeafWithTrees = { readonly trees: readonly { readonly tree: { readonly resolveTree: () => MarketplaceTreeSections } }[] };

  function marketplaceTreeSections(host: MarketplaceHostApi | null) {
    const marketplaceTab = createFrameworkMarketplacePanelTab(() => host) as unknown as LeafWithTrees;
    return marketplaceTab.trees[0]!.tree.resolveTree().sections;
  }

  it("shows an unavailable placeholder when no host is mounted yet", () => {
    const sections = marketplaceTreeSections(null);
    expect(sections).toHaveLength(1);
    expect(sections[0]!.items?.[0]?.id).toBe("unavailable");
  });

  const plugin = (pluginId: string, status: PluginPanelStatus, canUninstall: boolean): MarketplacePluginEntry => ({ pluginId, label: pluginId, version: "1", status, sourceId: "dev", canUninstall });
  const extension = (extensionId: string, extendsHost: string, enabled = true): MarketplaceExtensionEntry => ({ extensionId, label: extensionId, version: "1", extendsHost, enabled, status: "loaded" });
  const host = (overrides: Partial<MarketplaceHostApi> = {}): MarketplaceHostApi => ({
    plugins: [],
    extensions: [],
    installPlugin: () => {},
    uninstallPlugin: () => {},
    reloadPlugin: () => {},
    installExtensionFromUrl: () => {},
    installExtensionFromFile: () => {},
    uninstallExtension: () => {},
    setExtensionEnabled: () => {},
    ...overrides,
  });

  it("groups plugins into one section per source, sorted by pluginId within a source", () => {
    const sections = marketplaceTreeSections(host({ plugins: [plugin("s", "loaded", false), plugin("note", "loaded", true)] }));
    expect(sections.map((section) => section.id)).toEqual(["framework.marketplace.extensions.install", "framework.marketplace.source.dev"]);
    expect(sections[1]!.items?.map((item) => item.id)).toEqual(["framework.marketplace.plugin.note", "framework.marketplace.plugin.s"]);
  });

  it("integrates extensions as children of their owning plugin", () => {
    const sections = marketplaceTreeSections(
      host({
        plugins: [plugin("flow", "loaded", true), plugin("s", "loaded", false)],
        extensions: [extension("flow.brep", "flow"), extension("flow.math", "flow", false)],
      }),
    );
    const flow = sections[1]!.items?.find((item) => item.id === "framework.marketplace.plugin.flow");
    expect(flow?.items?.map((item) => item.id)).toEqual(["framework.marketplace.plugin.flow.extension.flow.brep", "framework.marketplace.plugin.flow.extension.flow.math"]);
    expect(sections.some((section) => section.id.includes("extensions.host"))).toBe(false);
  });

  it("marks installing/reloading rows as loading, and every status is reflected in the row label", () => {
    const items = marketplaceTreeSections(host({ plugins: [plugin("a", "available", true), plugin("b", "installing", true), plugin("c", "loaded", true), plugin("d", "failed", true), plugin("e", "reloading", true)] }))[1]!.items!;
    const byId = (pluginId: string) => items.find((item) => item.id === `framework.marketplace.plugin.${pluginId}`)!;
    expect(byId("a").loading).toBe(false);
    expect(byId("b").loading).toBe(true);
    expect(byId("c").loading).toBe(false);
    expect(byId("d").loading).toBe(false);
    expect(byId("e").loading).toBe(true);
    expect(byId("a").label).toContain("Available");
    expect(byId("b").label).toContain("Installing");
    expect(byId("c").label).toContain("Loaded");
    expect(byId("d").label).toContain("Failed");
    expect(byId("e").label).toContain("Reloading");
  });

  it("routes install/uninstall/reload clicks for one row back through the host without touching others", () => {
    const calls: string[] = [];
    const sections = marketplaceTreeSections(
      host({
        plugins: [plugin("note", "loaded", true)],
        uninstallPlugin: (pluginId) => calls.push(`uninstall:${pluginId}`),
        reloadPlugin: (pluginId) => calls.push(`reload:${pluginId}`),
      }),
    );
    const noteItem = sections[1]!.items![0]!;
    const { getByText } = render(createElement("div", null, noteItem.control));
    fireEvent.click(getByText("Reload"));
    fireEvent.click(getByText("Uninstall"));
    expect(calls).toEqual(["reload:note", "uninstall:note"]);
    cleanup();
  });

  it("disables uninstall for the host/primary plugin and the active session's plugin (canUninstall: false)", () => {
    const sections = marketplaceTreeSections(host({ plugins: [plugin("s", "loaded", false)] }));
    const sItem = sections[1]!.items![0]!;
    const { getByText } = render(createElement("div", null, sItem.control));
    expect((getByText("Uninstall").closest("button") as HTMLButtonElement).disabled).toBe(true);
    cleanup();
  });
});

describe("introductionTargetsWindow", () => {
  it("matches both the window kind and every open instance of that kind", () => {
    expect(introductionTargetsWindow("puzzle3d-main", "puzzle3d-main", "puzzle3d-main")).toBe(true);
    expect(introductionTargetsWindow("puzzle3d-main-top", "puzzle3d-main", "puzzle3d-main")).toBe(true);
    expect(introductionTargetsWindow("puzzle3d-main-perspective", "puzzle3d-main", "puzzle3d-main")).toBe(true);
    expect(introductionTargetsWindow("other-window", "other-window", "puzzle3d-main")).toBe(false);
  });

  it("matches action-rail segments against the kind and its instances", () => {
    expect(introductionTargetsWindow("puzzle3d-main-top", "puzzle3d-main", null, "puzzle3dMain")).toBe(true);
    expect(introductionTargetsWindow("puzzle3d-main", "puzzle3d-main", null, "puzzle3dMain")).toBe(true);
    expect(introductionTargetsWindow("other-window", "other-window", null, "puzzle3dMain")).toBe(false);
  });
});

describe("windowMeasureTreeContainsId", () => {
  it("finds nested measure ids used as introduction targets", () => {
    const measures: WindowMeasure[] = [
      { kind: "select", id: "puzzle3d-play-vortex-show", value: "selected", items: [], onChange: { controllerId: "puzzle", action: "setVortexShow" } },
      {
        kind: "group" as const,
        id: "group",
        label: "Group",
        children: [{ kind: "toggle", id: "nested-toggle", pressed: false, iconId: "eye", onChange: { controllerId: "puzzle", action: "noOperation" } }],
      },
    ];
    expect(windowMeasureTreeContainsId(measures, "puzzle3d-play-vortex-show")).toBe(true);
    expect(windowMeasureTreeContainsId(measures, "nested-toggle")).toBe(true);
    expect(windowMeasureTreeContainsId(measures, "missing")).toBe(false);
  });
});

describe("renderWindowMeasuresTree", () => {
  it("puts toggle icons before labels and uses checkboxes instead of icon toggles", () => {
    const measures: WindowMeasure[] = [
      {
        kind: "toggle",
        id: "grid-visible",
        iconId: "layout-grid",
        label: "Grid",
        pressed: true,
        onChange: { controllerId: "x", action: "setGridVisible" },
      },
    ];
    const markup = renderToStaticMarkup(renderWindowMeasuresTree(measures, () => undefined) as ReactElement);
    const iconIdx = markup.indexOf('data-slot="tree-icon"');
    const labelIdx = markup.indexOf('data-slot="tree-label"');
    const checkboxIdx = markup.indexOf('data-slot="tree-action-checkbox"');
    expect(iconIdx).toBeGreaterThan(-1);
    expect(labelIdx).toBeGreaterThan(iconIdx);
    expect(checkboxIdx).toBeGreaterThan(labelIdx);
    expect(markup).toContain('type="checkbox"');
    expect(markup).toContain('id="grid-visible"');
    expect(markup).toContain('data-icon="layout-grid"');
    expect(markup).toContain("Grid");
    expect(markup).toContain("checked");
    expect(markup).not.toContain('data-state="on"');
  });
});
