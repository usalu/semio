/** 🏷 Spawned window chrome, window measure partitioning and the utility ribbon. */
import { createElement, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { describe, expect, it } from "vitest";
import { partitionWindowMeasures, type AppWindowKindDefinition, type WindowMeasure, type UtilityNode } from "@semio-tech/framework";
import { buildUtilityRibbonSegments, dedupeUtilityNodesById, groupUtilityNodesByCategory, reconcileUtilityPath, sortUtilityNodes, spawnedWindowChromeForKind, UtilityTree } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { noopAction } from "../🧰️engine-contract-helpers/🟦️.ts";

describe("spawned window chrome", () => {
  const app = {
    id: "cad-play",
    label: "CAD",
    breadcrumb: ["semio", "cad"],
    controllerId: "cad-play",
    defaultModeId: "edit",
    modes: [
      {
        id: "edit",
        label: "Edit",
        utilities: [{ id: "static-utility", kind: "button" as const, iconId: "save", controllerId: "cad-play", action: "save" }],
      },
    ],
    windowKinds: [
      {
        id: "cad-window-shape",
        label: "Shape",
        bodyKey: "shape",
        options: {
          engagement: {
            kind: "some" as const,
            value: {
              input: {
                id: "engagement-input",
                placeholder: "Action",
                onChange: { controllerId: "cad-play", action: "engagementInput" },
              },
              possibleEngagements: [{ id: "box", label: "Box", action: { controllerId: "cad-play", action: "startBox" } }],
            },
          },
          measures: [{ id: "render-mode", kind: "select" as const, label: "Render Mode", value: "shaded", items: [], onChange: { controllerId: "cad-play", action: "setRenderMode" } }],
        },
      },
    ],
    panelTabs: [],
    keybindings: [],
  };

  it("builds spawned engagement and measures chrome from program contributions", () => {
    const kind = app.windowKinds[0]!;
    const engagements = {
      [kind.id]: {
        input: {
          id: "engagement-input",
          value: "Box",
          placeholder: "Action",
          onChange: { controllerId: "cad-play", action: "engagementInput" },
        },
        possibleEngagements: [{ id: "box", label: "Box", detail: "b", action: { controllerId: "cad-play", action: "startBox" } }],
      },
    };
    const measures = { [kind.id]: kind.options.measures ?? [] };
    const chrome = spawnedWindowChromeForKind(kind, kind.id, engagements, measures, undefined, noopAction);
    expect(chrome.search?.input?.value).toBe("Box");
    expect(chrome.search?.possibles?.[0]?.label).toBe("Box");
    const measuresMarkup = renderToStaticMarkup(chrome.measures as ReactElement);
    expect(measuresMarkup).toContain("Render Mode");
  });
});

describe("partitionWindowMeasures", () => {
  const utilityGroup = (id: string, activeUtilityId: string | undefined, children: WindowMeasure[] = []): WindowMeasure => ({ kind: "group", id, label: id, activeUtilityId, children });
  const slider = (id: string): WindowMeasure => ({ kind: "slider", id, value: 1, min: 0, max: 2, onChange: { controllerId: "c", action: "a" } });

  it("unwraps a tagged group's children into utilityOptions only when its utility is active", () => {
    const measures = [utilityGroup("brush-params", "brush", [slider("size")]), slider("zoom")];
    const active = partitionWindowMeasures(measures, "brush");
    expect(active.utilityOptions.map((m) => m.id)).toEqual(["size"]);
    expect(active.general.map((m) => m.id)).toEqual(["zoom"]);
  });

  it("drops a tagged group from both buckets when a different or no utility is active", () => {
    const measures = [utilityGroup("brush-params", "brush", [slider("size")]), slider("zoom")];
    const other = partitionWindowMeasures(measures, "fill");
    expect(other.utilityOptions).toEqual([]);
    expect(other.general.map((m) => m.id)).toEqual(["zoom"]);
    const none = partitionWindowMeasures(measures, undefined);
    expect(none.utilityOptions).toEqual([]);
    expect(none.general.map((m) => m.id)).toEqual(["zoom"]);
  });

  it("keeps untagged groups and non-group measures in general, unaffected by the active utility", () => {
    const measures = [utilityGroup("grid", undefined), slider("zoom")];
    const { general, utilityOptions } = partitionWindowMeasures(measures, "brush");
    expect(general.map((m) => m.id)).toEqual(["grid", "zoom"]);
    expect(utilityOptions).toEqual([]);
  });

  it("wires a utility-scoped group into spawnedWindowChromeForKind's utilityOptions slot only when its utility is active", () => {
    const kind = { id: "w", label: "W", bodyKey: "b", surfaceKind: "paint-2d", options: { engagement: { kind: "none" as const }, measures: [] } } as unknown as AppWindowKindDefinition;
    const brushGroup: WindowMeasure = {
      kind: "group",
      id: "brush-params",
      label: "Brush",
      defaultOpen: true,
      activeUtilityId: "brush",
      children: [{ kind: "slider", id: "size", label: "Brush size", value: 4, min: 1, max: 10, onChange: { controllerId: "c", action: "setSize" } }],
    };
    const measures = { [kind.id]: [brushGroup] };
    const activeChrome = spawnedWindowChromeForKind(kind, kind.id, {}, measures, "brush", noopAction);
    const activeMarkup = renderToStaticMarkup(activeChrome.utilityOptions as ReactElement);
    expect(activeMarkup).toContain("Brush size");
    expect(activeMarkup).toContain('data-direction="up"');
    expect(activeChrome.measures).toBeUndefined();
    const idleChrome = spawnedWindowChromeForKind(kind, kind.id, {}, measures, "fill", noopAction);
    expect(idleChrome.utilityOptions).toBeUndefined();
    expect(idleChrome.measures).toBeUndefined();
  });

  it("parses the real ui_wgpu camelCase wire JSON and unwraps a utility-scoped fill group into flat utilityOptions (snake_case divergence regression guard)", () => {
    const wireJson =
      '[{"kind":"group","id":"fill-params","label":"Fill","activeUtilityId":"fill","children":[{"kind":"slider","id":"fillCount","label":"Count","value":3,"min":1,"max":9,"step":1,"onChange":{"controllerId":"puzzle","action":"setFillCount"}}]},{"kind":"toggle","id":"grid","iconId":"layout-grid","pressed":true,"onChange":{"controllerId":"puzzle","action":"toggleGrid"}}]';
    const measures = JSON.parse(wireJson) as WindowMeasure[];
    const { general, utilityOptions } = partitionWindowMeasures(measures, "fill");
    expect(utilityOptions.map((m) => m.id)).toEqual(["fillCount"]);
    expect(utilityOptions[0]).toMatchObject({ kind: "slider", id: "fillCount", onChange: { action: "setFillCount" } });
    expect(general.map((m) => m.id)).toEqual(["grid"]);
    const gridToggle = general[0];
    expect(gridToggle.kind === "toggle" && gridToggle.iconId).toBe("layout-grid");

    const legacyJson = wireJson
      .replace(/"activeUtilityId"/g, '"active_utility_id"')
      .replace(/"onChange"/g, '"on_change"')
      .replace(/"iconId"/g, '"icon_id"');
    const legacy = JSON.parse(legacyJson) as WindowMeasure[];
    const legacyPartition = partitionWindowMeasures(legacy, "fill");
    expect(legacyPartition.utilityOptions).toEqual([]);
    expect(legacyPartition.general.map((m) => m.id)).toEqual(["fill-params", "grid"]);
  });
});

describe("utility ribbon", () => {
  it("sorts utility nodes by order", () => {
    const sorted = sortUtilityNodes([
      { id: "b", kind: "button", iconId: "box", order: 2, controllerId: "x", action: "b" },
      { id: "a", kind: "button", iconId: "box", order: 1, controllerId: "x", action: "a" },
    ]);
    expect(sorted.map((node) => node.id)).toEqual(["a", "b"]);
  });

  it("recurses into a collection level only when the path names one of its collections", () => {
    const tree = [
      {
        id: "view",
        kind: "collection",
        iconId: "eye",
        children: [
          {
            id: "view-tools",
            kind: "collection",
            iconId: "zoom-in",
            children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
          },
        ],
      },
      {
        id: "construct",
        kind: "collection",
        iconId: "box",
        children: [
          {
            id: "construct-tools",
            kind: "collection",
            iconId: "box",
            children: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
          },
        ],
      },
    ] satisfies UtilityNode[];

    const noActive = buildUtilityRibbonSegments(tree, []);
    expect(noActive).toEqual([{ kind: "picker", collections: tree, depth: 0 }]);

    const oneActive = buildUtilityRibbonSegments(tree, ["construct"]);
    expect(oneActive[0]).toMatchObject({ kind: "picker", depth: 0 });
    expect(oneActive[1]).toMatchObject({ kind: "picker", depth: 1, collections: tree[1].children });
    expect(oneActive).toHaveLength(2);

    const twoActive = buildUtilityRibbonSegments(tree, ["construct", "construct-tools"]);
    const utilitiesSegment = twoActive.find((segment) => segment.kind === "utilities" && segment.items.some((item) => item.id === "box"));
    expect(utilitiesSegment).toMatchObject({ depth: 2 });
  });

  it("ignores a path entry that no longer names an enabled collection at that level", () => {
    const tree: UtilityNode[] = [
      {
        id: "view",
        kind: "collection",
        iconId: "eye",
        children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
      },
    ];
    expect(buildUtilityRibbonSegments(tree, ["nonexistent"])).toEqual([{ kind: "picker", collections: tree, depth: 0 }]);
  });

  it("emits a picker segment alongside loose leaves at the same depth", () => {
    const segments = buildUtilityRibbonSegments(
      [
        { id: "undo", kind: "button", iconId: "undo", controllerId: "x", action: "undo" },
        {
          id: "view",
          kind: "collection",
          iconId: "eye",
          children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
        },
      ],
      [],
    );
    expect(segments).toEqual([
      { kind: "picker", collections: [expect.objectContaining({ id: "view" })], depth: 0 },
      { kind: "utilities", items: [expect.objectContaining({ id: "undo" })], depth: 0 },
    ]);
  });

  it("reconciles an active path by truncating at the first stale entry instead of substituting a default", () => {
    const tree: UtilityNode[] = [
      {
        id: "a",
        kind: "collection",
        iconId: "box",
        children: [
          { id: "x", kind: "collection", iconId: "box", children: [{ id: "leaf", kind: "button", iconId: "box", controllerId: "c", action: "act" }] },
          { id: "y", kind: "collection", iconId: "box", children: [] },
        ],
      },
      { id: "b", kind: "collection", iconId: "box", children: [] },
    ];
    expect(reconcileUtilityPath(tree, ["a", "x"])).toEqual(["a", "x"]);
    expect(reconcileUtilityPath(tree, ["a", "gone"])).toEqual(["a"]);
    expect(reconcileUtilityPath(tree, ["gone"])).toEqual([]);
    expect(reconcileUtilityPath(tree, [])).toEqual([]);
  });

  it("buckets top-level utility nodes into ordered category collections (uncategorized nodes default to tools now that the Actions category is gone)", () => {
    const grouped = groupUtilityNodesByCategory([
      { id: "sel", kind: "toggle", iconId: "mouse-pointer", controllerId: "x", action: "sel", category: "selection" },
      { id: "hist", kind: "button", iconId: "undo", controllerId: "x", action: "undo", category: "history" },
      { id: "act", kind: "button", iconId: "sparkles", controllerId: "x", action: "run" },
      { id: "tool", kind: "toggle", iconId: "pencil", controllerId: "x", action: "pen" },
      { id: "sync", kind: "toggle", iconId: "cloud", controllerId: "x", action: "sync", category: "sync" },
    ]);
    expect(grouped.map((node) => node.id)).toEqual(["selection", "utilities", "history", "sync"]);
    expect(grouped.every((node) => node.kind === "collection")).toBe(true);
  });

  it("drops separator-only category buckets so an empty group never appears as a picker option", () => {
    const grouped = groupUtilityNodesByCategory([
      { id: "a", kind: "button", iconId: "box", controllerId: "x", action: "a", category: "utilities" },
      { id: "sep", kind: "separator" },
    ]);
    expect(grouped).toHaveLength(1);
    expect(grouped[0].id).toBe("utilities");
  });

  it("reuses a category's single already-meaningful collection instead of re-wrapping it, avoiding a duplicate-looking picker level", () => {
    const selectionCollection: Extract<UtilityNode, { kind: "collection" }> = {
      id: "lowpoly-tools-selection",
      kind: "collection" as const,
      iconId: "mouse-pointer",
      label: "Selection",
      category: "selection" as const,
      children: [{ id: "mesh", kind: "toggle" as const, iconId: "box", controllerId: "x", action: "mesh" }],
    };
    const grouped = groupUtilityNodesByCategory([selectionCollection]);
    expect(grouped).toEqual([{ ...selectionCollection, order: 0 }]);
    const segments = buildUtilityRibbonSegments(grouped, ["lowpoly-tools-selection"]);
    const utilitiesSegment = segments.find((segment) => segment.kind === "utilities" && segment.items.some((item) => item.id === "mesh"));
    expect(utilitiesSegment).toBeTruthy();
  });

  it("still wraps a category with multiple top-level nodes in a synthetic collection", () => {
    const grouped = groupUtilityNodesByCategory([
      { id: "a", kind: "button", iconId: "box", controllerId: "x", action: "a", category: "utilities" },
      { id: "b", kind: "button", iconId: "box", controllerId: "x", action: "b", category: "utilities" },
    ]);
    expect(grouped).toEqual([{ id: "utilities", kind: "collection", iconId: "wrench", text: "utilities", order: 0, category: "utilities", children: expect.any(Array) }]);
  });

  it("scopes grouping to the given categories only", () => {
    const nodes: UtilityNode[] = [
      { id: "sel", kind: "toggle", iconId: "mouse-pointer", controllerId: "x", action: "sel", category: "selection" },
      { id: "hist", kind: "button", iconId: "undo", controllerId: "x", action: "undo", category: "history" },
    ];
    expect(groupUtilityNodesByCategory(nodes, ["selection", "utilities"]).map((node) => node.id)).toEqual(["selection"]);
    expect(groupUtilityNodesByCategory(nodes, ["utilities", "history"]).map((node) => node.id)).toEqual(["history"]);
  });

  it("deduplicates utility nodes by id across window utility lists for a single shared footer entry", () => {
    const history: UtilityNode = { id: "s-play.history", kind: "collection", iconId: "clock", category: "history", children: [] };
    const deduped = dedupeUtilityNodesById([[history, { id: "leaf-a", kind: "button" as const, iconId: "box", controllerId: "x", action: "a" }], [history], []]);
    expect(deduped).toEqual([history, { id: "leaf-a", kind: "button", iconId: "box", controllerId: "x", action: "a" }]);
  });

  it("renders utility ribbon with picker and batched toggles", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        utilities: [
          {
            id: "view",
            kind: "collection",
            iconId: "eye",
            children: [
              {
                id: "view-tools",
                kind: "collection",
                iconId: "eye",
                children: [
                  { id: "show-edges", kind: "toggle", iconId: "box", pressed: true, controllerId: "x", action: "edges" },
                  { id: "show-faces", kind: "toggle", iconId: "square", pressed: false, controllerId: "x", action: "faces" },
                ],
              },
            ],
          },
          {
            id: "construct",
            kind: "collection",
            iconId: "box",
            children: [
              {
                id: "construct-tools",
                kind: "collection",
                iconId: "box",
                children: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
              },
            ],
          },
        ],
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('id="ui.utilities"');
    expect(markup).toContain('data-slot="toggle-group"');
  });

  it("stacks the window utility bar ribbon upward, showing only the base picker row until a group is activated", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        direction: "up",
        utilities: [
          {
            id: "view",
            kind: "collection",
            iconId: "eye",
            children: [{ id: "zoom-in", kind: "button", iconId: "zoom-in", controllerId: "x", action: "zoomIn" }],
          },
          {
            id: "construct",
            kind: "collection",
            iconId: "box",
            children: [
              {
                id: "construct-tools",
                kind: "collection",
                iconId: "box",
                children: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
              },
            ],
          },
        ],
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('data-slot="ribbon"');
    expect(markup).toContain('data-direction="up"');
    expect(markup).toContain("flex-col-reverse");
    expect(markup.match(/data-slot="ribbon-row"/g)?.length).toBe(1);
    expect(markup).toContain('data-slot="toggle-group"');
    expect(markup).not.toContain('id="zoom-in"');
  });

  it("renders UtilityTree with a custom id for per-window namespacing", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        id: "ui.utilities.model",
        utilities: [{ id: "box", kind: "button", iconId: "box", controllerId: "x", action: "box" }],
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('id="ui.utilities.model"');
    expect(markup).not.toContain('id="ui.utilities"');
  });

  it("renders utilityOptions as an extra ribbon row when direction is up", () => {
    const markup = renderToStaticMarkup(
      createElement(UtilityTree, {
        id: "ui.utilities.w",
        direction: "up",
        utilities: [{ id: "brush", kind: "toggle", iconId: "paintbrush", pressed: true, controllerId: "x", action: "setActiveUtility" }],
        utilityOptions: createElement("span", { "data-testid": "brush-options" }, "Brush size"),
        onAction: noopAction,
      }),
    );
    expect(markup).toContain('data-testid="brush-options"');
    expect(markup).toContain("Brush size");
    expect(markup).toContain('data-variable-height="true"');
    expect(markup).toContain("h-auto min-h-medium");
    expect(markup).toContain("items-start");
    const variableZoneClass = markup.match(/data-variable-height="true" class="([^"]*)"/)?.[1].split(" ") ?? [];
    expect(variableZoneClass).toContain("h-auto");
    expect(variableZoneClass).not.toContain("h-medium");
    const optionItemClass = [...markup.matchAll(/data-slot="ribbon-item" class="([^"]*)"/g)].at(-1)?.[1].split(" ") ?? [];
    expect(optionItemClass).toEqual(expect.arrayContaining(["h-auto", "items-start", "w-full", "max-w-full"]));
  });
});
