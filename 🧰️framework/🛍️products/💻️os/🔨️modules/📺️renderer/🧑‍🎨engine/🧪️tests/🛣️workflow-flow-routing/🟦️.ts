/** 🛣 Workflow flow routing and ui search. */
import { createElement, type ReactElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { uiI18n } from "@semio-tech/ui-react";
import { automaticCheckinWaitsV1 } from "../../🧱️elements/🛠️ShellHelpers/🧾️transfer-checkin-tutorial/🟦️.ts";
import automaticCheckinCorpus from "../../🧱️elements/🛠️ShellHelpers/🧪️tests/🧫️fixtures/🧫️automatic-checkin/🔣️.json";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it, vi } from "vitest";
import { type UiIntent, pendingPanelUiNode, type ActionDescriptor } from "@semio-tech/framework";
import { resolveTranslationLabel, uiDataLabel } from "@semio-tech/ui-react";
import { NodeGraphHost, catalogueGhostDescriptorJson, flowCatalogueItemDescriptor, flowSurfaceRenderAllowed, flowRankCatalogueSuggestions, flowSpotlightSuggestionListScrollClass, parseCatalogueAppDragPayload, resolveHostSnapshotWidgetInstanceId, UiPresenceOverlayContext, type UiPresenceOverlayEntry, buildUiRefreshRequest, isFlowGraphScene, parseShellRoute, shellActorId, canonicalSurfaceId, reloadRetainsActiveApp, directoryCommandFromAction, mintDirectoryCommandRequestId, retainDirectoryCommandResult, DIRECTORY_COMMAND_RESULT_SLOTS, type DirectoryCommandResultSlotV1, type DirectoryCommandReceiptV1, AUTO_CHECKIN_IDLE_MS, AUTO_CHECKIN_EDIT_THRESHOLD, AutoCheckinScheduler, canCheckIn, computeSyncPillState, syncPillText, ShellFaultBoundary, studioPanelFocusingSpawned, viewStateWithSpacePanel, uiIntentToActionDescriptor, UIFind, UIFindProvider, uiNodeToTreePanelConfig, UISearch, type UISearchItem, useUIFind } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { buildContractNode, noopAction, panelTreePanelHost } from "../🧰️engine-contract-helpers/🟦️.ts";

describe("s workflow flow routing", () => {
  it("does not render a flow frame before its surface is ready", () => {
    expect(flowSurfaceRenderAllowed(false)).toBe(false);
    expect(flowSurfaceRenderAllowed(true)).toBe(true);
  });

  it("selects the flow engine for scenes with engine flow capabilities", () => {
    expect(isFlowGraphScene('{"engine":"flow","spotlight":false,"noteEdit":false}')).toBe(true);
    expect(isFlowGraphScene('{"spotlight":false,"noteEdit":false,"clusters":false}')).toBe(false);
    expect(isFlowGraphScene(undefined)).toBe(false);
  });

  it("renders presence peers from the scene payload", () => {
    const markup = renderToStaticMarkup(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.workflow",
          controllerId: "s-play",
          componentKind: "node-graph",
          nodeGraph: {
            nodes: [],
            edges: [],
            viewport: { x: 0, y: 0, zoom: 1 },
            presencePeersJson: JSON.stringify([{ clientId: "client-b", name: "Ada", selectionCount: 2 }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("Ada");
    expect(markup).toContain("2 selected");
  });

  it("parses a catalogue app drag payload, ignoring extra keys", () => {
    expect(parseCatalogueAppDragPayload(JSON.stringify({ pluginId: "s.system", appId: "draw", label: "Draw", extra: "x" }))).toEqual({
      pluginId: "s.system",
      appId: "draw",
      label: "Draw",
    });
  });

  it("rejects catalogue app drag payloads missing pluginId/appId, and garbage", () => {
    expect(parseCatalogueAppDragPayload(JSON.stringify({ appId: "draw" }))).toBeNull();
    expect(parseCatalogueAppDragPayload(JSON.stringify({ kind: "neuron" }))).toBeNull();
    expect(parseCatalogueAppDragPayload("not json")).toBeNull();
  });

  it("builds a ghost neuron descriptor, preferring label over appId", () => {
    expect(JSON.parse(catalogueGhostDescriptorJson({ pluginId: "s.system", appId: "draw", label: "Draw" }))).toEqual({ kind: "neuron", neuronKind: "Draw" });
    expect(JSON.parse(catalogueGhostDescriptorJson({ pluginId: "s.system", appId: "draw" }))).toEqual({ kind: "neuron", neuronKind: "draw" });
  });

  it("builds addWidget descriptors from catalogue items", () => {
    expect(JSON.parse(flowCatalogueItemDescriptor({ kind: "neuron", neuronKind: "math.add", name: "Add", abbreviation: "Add", icon: "emoji:➕️", summary: "" }))).toEqual({
      kind: "neuron",
      neuronKind: "math.add",
    });
    expect(JSON.parse(flowCatalogueItemDescriptor({ kind: "outputExport", format: "svg", name: "Export SVG", abbreviation: "SVG", icon: "emoji:📤️", summary: "" }))).toEqual({
      kind: "outputExport",
      format: "svg",
    });
    expect(JSON.parse(flowCatalogueItemDescriptor({ kind: "inputSlider", name: "Slider", abbreviation: "Slider", icon: "emoji:🎚️", summary: "" }))).toEqual({ kind: "inputSlider", label: "Slider" });
  });

  it("ranks catalogue suggestions by exact/prefix match with neurons first", () => {
    const sections = [
      {
        id: "inputs",
        title: "Inputs",
        items: [
          { kind: "inputSlider", name: "Slider", abbreviation: "Slider", icon: "emoji:🎚️", summary: "" },
          { kind: "inputNote", name: "Note", abbreviation: "Note", icon: "emoji:📝️", summary: "" },
        ],
      },
      {
        id: "math",
        title: "Math",
        items: [
          { kind: "neuron", neuronKind: "math.add", name: "Add", abbreviation: "Add", icon: "emoji:➕️", summary: "" },
          { kind: "neuron", neuronKind: "math.subtract", name: "Subtract", abbreviation: "Sub", icon: "emoji:➖️", summary: "" },
        ],
      },
    ];
    expect(flowRankCatalogueSuggestions(sections, "add").map((item) => item.neuronKind ?? item.kind)).toEqual(["math.add"]);
    expect(flowRankCatalogueSuggestions(sections, "sl").map((item) => item.kind)).toEqual(["inputSlider"]);
    const brepSections = [
      {
        id: "brep",
        title: "Brep",
        items: [{ kind: "neuron", neuronKind: "brep.prim3d.box", name: "Box", abbreviation: "Box", icon: "emoji:📦️", summary: "Axis-aligned box" }],
      },
    ];
    expect(flowRankCatalogueSuggestions(brepSections, "brep").map((item) => item.neuronKind ?? item.kind)).toEqual(["brep.prim3d.box"]);
    expect(flowRankCatalogueSuggestions(brepSections, "box").map((item) => item.neuronKind ?? item.kind)).toEqual(["brep.prim3d.box"]);
    const empty = flowRankCatalogueSuggestions(sections, "");
    expect(empty[0]?.kind).toBe("neuron");
    expect(empty.some((item) => item.kind === "inputSlider")).toBe(true);
  });

  it("returns every catalogue item for empty query without a 20-item cap", () => {
    const items = Array.from({ length: 25 }, (_, index) => ({
      kind: "neuron",
      neuronKind: `math.operation${index}`,
      name: `Operation ${index}`,
      abbreviation: `Operation${index}`,
      icon: "emoji:➕️",
      summary: "",
    }));
    const sections = [{ id: "math", title: "Math", items }];
    expect(flowRankCatalogueSuggestions(sections, "")).toHaveLength(25);
  });

  it("enables overflow scrolling only when spotlight suggestions are expanded", () => {
    expect(flowSpotlightSuggestionListScrollClass(false)).toContain("overflow-hidden");
    expect(flowSpotlightSuggestionListScrollClass(false)).not.toContain("overflow-y-auto");
    expect(flowSpotlightSuggestionListScrollClass(true)).toContain("overflow-y-auto");
    expect(flowSpotlightSuggestionListScrollClass(true)).toContain("max-h-[min(24rem,70vh)]");
  });

  it("hosts semantic panel bodies full-width via tree emptyState, not a property-layout control wrapper", () => {
    const config = uiNodeToTreePanelConfig(pendingPanelUiNode(), noopAction, "framework.panel.inspection");
    expect(config.sections).toEqual([]);
    expect(config.emptyState).toBeTruthy();
    expect(config.className).toContain("w-full");
  });

  // (measured 2026-09-09 21:05). The guest cannot render an empty inspection body
  // (`selected_object_inspector_renders_that_object_field_group` in the puzzle3d crate proves a pick
  // yields the object field group, and `render` always falls back to the document summary), and the
  // panel leaf above proves a delivered tree body renders. What was left was the third state: a body
  // the shell has NOT received yet is `pendingPanelUiNode()` — a `tree` node with `activity: "loading"`
  // and no children — and `<Tree sections={[]}/>` drew literally nothing for it. Loading, empty and
  // dropped were one and the same blank rectangle.
  it("renders a panel body that has not arrived yet as a loading surface, never as a silently empty panel", () => {
    const rendered = render(panelTreePanelHost(uiNodeToTreePanelConfig(pendingPanelUiNode(), noopAction, "framework.panel.inspection")));
    expect(rendered.container.querySelector('[data-ui-status="loading"]')).toBeTruthy();
    expect(rendered.container.querySelector('[aria-busy="true"]')).toBeTruthy();
  });

  // 🈳️ The blank-panel state: a tree body that IS settled (`activity: "idle"`) and resolves to zero
  // `treeSection` children used to render `<Tree sections={[]}/>`, i.e. literally nothing — no ring, no
  // text, no marker — which is indistinguishable from a body that was dropped on the way in. It now says
  // so. A guest-rendered panel body always carries at least one section (`PanelTreeBuilder::build`), so
  // reaching this state at all means something upstream lost the body.
  it("renders an idle tree body with no sections as an explicit empty state, distinct from loading", () => {
    const idleEmpty = { ...pendingPanelUiNode(), activity: "idle" as const };
    const empty = render(panelTreePanelHost(uiNodeToTreePanelConfig(idleEmpty, noopAction, "framework.panel.inspection")));
    expect(empty.container.querySelector('[data-ui-status="loading"]')).toBeNull();
    expect(empty.container.textContent?.replace(/\u2026/g, "").trim()).not.toBe("");
    expect(empty.container.textContent).toContain(resolveTranslationLabel(uiI18n.t("ui.common.noData")));
  });

  // 🐢️ The host half of the puzzle3d scope table (`puzzle3d_command_scope_class`): a partial scope that
  // NAMES the inspection panel body is what puts that panel in the batched `refresh-ui` request. A
  // partial scope that names only window bodies — which is what every puzzle3d partial scope used to be
  // — leaves the inspector out of the request entirely, so it keeps whatever it last rendered.
  it("a partial scope naming a panel body requests that panel, and one that omits it does not", () => {
    const leaves = [
      { kind: { kind: "app" as const, id: "framework.panel.artifact" }, bodyKey: "puzzle.3d.play.artifact" },
      { kind: { kind: "app" as const, id: "framework.panel.inspection" }, bodyKey: "puzzle.3d.play.inspector" },
    ];
    const windows = [{ id: "puzzle3d-main", bodyKey: "puzzle3d.play.composite" }];
    const named = buildUiRefreshRequest({ kind: "partial", windowBodies: ["puzzle3d.play.composite"], panelBodies: ["puzzle.3d.play.inspector", "puzzle.3d.play.artifact"], measures: true }, windows, leaves, {}, new Map());
    expect(named?.panels?.map((panel) => panel.key)).toEqual(["framework.panel.artifact", "framework.panel.inspection"]);
    const omitted = buildUiRefreshRequest({ kind: "partial", windowBodies: ["puzzle3d.play.composite"], panelBodies: [] }, windows, leaves, {}, new Map());
    expect(omitted?.panels).toEqual([]);
  });

  it("hosts a semantic tree document inside a panel leaf", () => {
    const config = uiNodeToTreePanelConfig(
      buildContractNode({
        key: "catalogue",
        component: { type: "tree", interactionDomain: null },
        children: [
          {
            key: "catalogue.section",
            component: { type: "treeSection", label: "Catalogue", defaultOpen: true, headerToolbar: null, window: null },
            children: [
              {
                key: "s-play-catalogue.document.draw",
                component: {
                  type: "treeItem",
                  label: "Draw",
                  description: null,
                  icon: null,
                  defaultOpen: null,
                  draggable: true,
                  dragData: { "application/x-semio-catalogue-item": '{"pluginId":"s.system","appId":"draw"}' },
                  dimmed: null,
                  selected: null,
                  window: null,
                  granularity: null,
                  inlineToolbar: null,
                  detail: null,
                  rowActions: [], target: null,
                },
              },
            ],
          },
        ],
      }),
      noopAction,
      "framework.panel.catalogue",
    );
    const rendered = render(panelTreePanelHost(config));
    expect(rendered.getByText("Draw")).toBeTruthy();
  });

  // 🛍️ The catalogue row's OWN add gesture: `object_kind_item` (`📌️panels/🛍️catalogue/🦀️.rs`) authors an
  // expandable `treeItem` — the kind's rim-vortex templates are its children — that ALSO binds
  // `activate` to `addObjectKind` with its own `{objectKind}` args. Battery #48 measured
  // `catalogue-add-object-kind before=1 after=1` while `catalogue-drag-drop` (a different route into the
  // same command) passed, so the question this pins is whether a click on an EXPANDABLE authored row
  // reaches the action channel at all, or is swallowed as a fold toggle.
  it("fires an expandable catalogue row's own target activation, args and all, instead of only folding it", () => {
    const dispatched: ActionDescriptor[] = [];
    const config = uiNodeToTreePanelConfig(
      buildContractNode({
        key: "puzzle3d-play-kinds",
        component: { type: "tree", interactionDomain: null },
        children: [
          {
            key: "puzzle3d-play-kinds.objects",
            component: { type: "treeSection", label: "Objects", defaultOpen: true, headerToolbar: null, window: null },
            children: [
              {
                key: "Hexagonal Cut Concrete Forest Left",
                component: {
                  type: "treeItem",
                  label: "Hexagonal Cut Concrete Forest Left",
                  description: "Hexagonal Cut Concrete Forest Left",
                  icon: "box",
                  defaultOpen: false,
                  draggable: true,
                  dragData: { "application/x-semio-catalogue-item": '{"objectKind":"Hexagonal Cut Concrete Forest Left"}' },
                  dimmed: null,
                  selected: null,
                  window: null,
                  granularity: null,
                  inlineToolbar: null,
                  detail: null,
                  rowActions: [], target: { scope: "puzzle3d-play", version: 1, args: { objectKind: "Hexagonal Cut Concrete Forest Left" }, activation: "addObjectKind" },
                },
                children: [
                  {
                    key: "puzzle3d-kind-vortex.0.b-l",
                    component: { type: "treeItem", label: "b-l", description: "[4,4,3]", icon: "circle-dot", defaultOpen: null, draggable: null, dragData: null, dimmed: null, selected: null, window: null, granularity: null, inlineToolbar: null, detail: null, rowActions: [], target: null },
                  },
                ],
              },
            ],
          },
        ],
      }),
      (action) => dispatched.push(action),
      "framework.panel.catalogue",
    );
    const rendered = render(panelTreePanelHost(config));
    const row = rendered.container.querySelector("#panel\\:puzzle3d-play-kinds\\/Hexagonal\\ Cut\\ Concrete\\ Forest\\ Left");
    expect(row).toBeTruthy();
    // 🖱️ The row must announce its own activation in the DOM, so a red says WHICH hop broke rather than
    // "nothing happened" — `data-activatable` is `Boolean(TreeDataItem.onClick)` reaching the shell.
    expect(row?.getAttribute("data-activatable")).toBe("true");
    const expected = { controllerId: "puzzle3d-play", action: "addObjectKind", args: { objectKind: "Hexagonal Cut Concrete Forest Left" } };
    fireEvent.click(row as Element);
    expect(dispatched).toEqual([expected]);
    // 🏷️ And through the LABEL, which is what a real pointer lands on: a click on the row's text must
    // reach the same action rather than being spent folding the group open.
    const label = row?.querySelector('[data-slot="tree-label"]');
    expect(label).toBeTruthy();
    fireEvent.click(label as Element);
    expect(dispatched).toEqual([expected, expected]);
  });

  it("bridges semantic intent scope, version, args, and input into the plugin action channel", () => {
    const intent = {
      surface: "panel:settings",
      revision: 1,
      node: 7,
      nodeKey: "spacing",
      trigger: "change",
      action: { scope: "puzzle3d-play", name: "setSpacing", version: 1 },
      args: { axis: "x", value: 1 },
      input: { value: 2 },
      seq: 1n,
    } as UiIntent;
    expect(uiIntentToActionDescriptor(intent)).toEqual({ controllerId: "puzzle3d-play", action: "setSpacing", args: { axis: "x", value: 2 } });
  });

  it("names a scalar intent payload after its trigger and keeps the authored arguments", () => {
    // 🪜️ A `NumberStepper`'s `onChange` reports the number itself, and the node's authored args carry the
    // window the panel is tuning. The scalar must arrive UNDER `value` — every guest command reads named
    // arguments — and must not evict `windowId` on the way (ticket 26/09/02/PUZZLE-3D-END-TO-END B47 §2).
    const base = {
      surface: "panel:puzzle3d-play-settings",
      revision: 1,
      node: 7,
      nodeKey: "puzzle3d-play-settings.grid-spacing",
      action: { scope: "puzzle3d-play", name: "setGridSpacing", version: 1 },
      args: { windowId: "puzzle3d-main-top" },
      seq: 1n,
    };
    expect(uiIntentToActionDescriptor({ ...base, trigger: "change", input: 10.5 } as UiIntent)).toEqual({
      controllerId: "puzzle3d-play",
      action: "setGridSpacing",
      args: { windowId: "puzzle3d-main-top", value: 10.5 },
    });
    // ➕️➖️ …and a relative bump under `delta`, which is the other half of `puzzle3d_absolute_or_delta`.
    expect(uiIntentToActionDescriptor({ ...base, trigger: "delta", input: -0.5 } as UiIntent)).toEqual({
      controllerId: "puzzle3d-play",
      action: "setGridSpacing",
      args: { windowId: "puzzle3d-main-top", delta: -0.5 },
    });
    // 🧾️ A scalar with no authored args is still named, never a bare primitive.
    expect(uiIntentToActionDescriptor({ ...base, args: null, trigger: "commit", input: "fill 12" } as UiIntent)).toEqual({
      controllerId: "puzzle3d-play",
      action: "setGridSpacing",
      args: { value: "fill 12" },
    });
  });

  it("carries an outliner row action's explicit ids to the action channel whatever else is selected", () => {
    // 🙈️ `with_hide_lock_actions` (`📌️panels/🗿️artifact/🦀️.rs`) authors each inline toggle as a set-verb
    // (`setSelectionHidden`/`setSelectionLocked`) over the row's ONE target `{entity, hidden, ids, locked}` — the explicit
    // next state and the entity it flags — so the guest's explicit branch never consults the live selection. Battery #59 measured `outliner-hide-applies` FAIL with the row's
    // object selected (`historyUpserts: 0, effects: 0`) and PASS with nothing selected, which made "the
    // host dropped the args under a selection" a live candidate (26/09/02/PUZZLE-3D-END-TO-END B47 §6).
    // This pins the host half: the same authored map reaches `ActionDescriptor` with and without a
    // selection overlay, so a future red here means the HOST lost them and a red only in the browser
    // means the guest did.
    const targetArgs = { entity: "object", hidden: true, ids: ["seed-left-001"], locked: true };
    const documentNode = buildContractNode({
      key: "puzzle3d-play-document",
      component: { type: "tree", interactionDomain: "puzzle3d" },
      bindings: [{ trigger: "activate", action: { scope: "puzzle3d-play", name: "interactionSelect", version: 1 }, args: { domainId: "puzzle3d" }, capability: null }],
      children: [
        {
          key: "puzzle3d-play-document.objects",
          component: { type: "treeSection", label: "Objects", defaultOpen: true, headerToolbar: null, window: null },
          children: [
            {
              key: "seed-left-001",
              component: {
                type: "treeItem",
                label: "Hexagonal Cut Concrete Forest Left",
                description: null,
                icon: "box",
                defaultOpen: null,
                draggable: null,
                dragData: null,
                dimmed: null,
                selected: null,
                window: null,
                granularity: "object",
                inlineToolbar: null,
                detail: null,
                rowActions: [{ icon: "eye", label: "Hide", verb: "setSelectionHidden", placement: "row", disabled: false }],
                target: { scope: "puzzle3d-play", version: 1, args: targetArgs, activation: null },
              },
            },
          ],
        },
      ],
    });
    const expected = { controllerId: "puzzle3d-play", action: "setSelectionHidden", args: targetArgs };
    for (const presence of [undefined, { "seed-left-001": { selected: true }, "object-1": { selected: true } }]) {
      const dispatched: ActionDescriptor[] = [];
      const config = uiNodeToTreePanelConfig(documentNode, (action) => dispatched.push(action), "framework.panel.document");
      const host = panelTreePanelHost(config);
      const rendered = render(presence ? createElement(UiPresenceOverlayContext.Provider, { value: { byKey: new Map(Object.entries(presence)) as ReadonlyMap<string, UiPresenceOverlayEntry> } }, host) : host);
      const hide = Array.from(rendered.container.querySelectorAll('[data-slot="action"]')).find((candidate) => (candidate.textContent ?? "").includes("Hide"));
      expect(hide).toBeTruthy();
      fireEvent.click(hide as Element);
      expect(dispatched).toEqual([expected]);
      cleanup();
    }
  });

  it("resolves a fixture widget id to its workflow instance id, independent of selection state", () => {
    const snapshotJson = JSON.stringify({
      widgets: [
        { id: "widget-1", params: { instanceId: "app-1" } },
        { id: "widget-2", params: {} },
      ],
    });
    expect(resolveHostSnapshotWidgetInstanceId(snapshotJson, "widget-1")).toBe("app-1");
    expect(resolveHostSnapshotWidgetInstanceId(snapshotJson, "widget-2")).toBeUndefined();
    expect(resolveHostSnapshotWidgetInstanceId(snapshotJson, "missing-widget")).toBeUndefined();
    expect(resolveHostSnapshotWidgetInstanceId(snapshotJson, undefined)).toBeUndefined();
    expect(resolveHostSnapshotWidgetInstanceId(undefined, "widget-1")).toBeUndefined();
    expect(resolveHostSnapshotWidgetInstanceId("not json", "widget-1")).toBeUndefined();
  });

  it("classifies shell routes into landing, space, and notFound", () => {
    expect(parseShellRoute("/")).toEqual({ kind: "landing" });
    expect(parseShellRoute("/spaces/my-studio")).toEqual({ kind: "space", spaceId: "my-studio", instanceId: undefined });
    expect(parseShellRoute("/spaces/my-studio/instances/inst-1")).toEqual({ kind: "space", spaceId: "my-studio", instanceId: "inst-1" });
    expect(parseShellRoute("/unknown/path")).toEqual({ kind: "notFound", path: "/unknown/path" });
    expect(parseShellRoute("/spaces/my-studio/instances/inst-1/extra")).toEqual({ kind: "notFound", path: "/spaces/my-studio/instances/inst-1/extra" });
  });

  // 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C0/§C3/§C6 — pure-function
  // coverage of the identity/directory helpers `ShellHost/🟦️.tsx` exports for this. Full
  // end-to-end coverage (a real `openDocument` binding snapshot with/without a resolved identity, an
  // `os.open-artifact{documentId}` effect resulting in a worker `open` request with hub+folder
  // bindings) would need `fetch`/`Worker`/`DirectoryClient` mocking this already-huge shared suite has
  // no existing pattern for; not added this lane — see `📓️w2-c-report.md`.
  it("shellActorId mints user:{userId}#{sessionId} once identity resolves, else client-{sessionId}", () => {
    expect(shellActorId("sess-1", null)).toBe("client-sess-1");
    expect(shellActorId("sess-1", { userId: "u-1", email: "u1@semio.dev", displayName: "U1", hubBaseUrl: "http://127.0.0.1:8787", issuedAtMs: 0 })).toBe("user:u-1#sess-1");
  });

  it("canonicalSurfaceId formats <kind>@<standard>/<subset>#<role>", () => {
    expect(canonicalSurfaceId({ artifactKind: "s.space.space", standard: "1", subset: "*" }, "editor")).toBe("s.space.space@1/*#editor");
    expect(canonicalSurfaceId({ artifactKind: "s.space.space", standard: "1", subset: "*" }, "viewer")).toBe("s.space.space@1/*#viewer");
  });

  it("reloadRetainsActiveApp accepts extension-only programs and rejects a dropped active app", () => {
    expect(reloadRetainsActiveApp([], undefined)).toBe(true);
    expect(reloadRetainsActiveApp([{ id: "active" }], "active")).toBe(true);
    expect(reloadRetainsActiveApp([], "active")).toBe(false);
  });

  it("directoryCommandFromAction maps all 7 frozen os.directory.* ids, share-link sugaring to create-invite", () => {
    expect(directoryCommandFromAction("os.directory.create-space", { name: "Atelier", spaceKind: "atelier", visibility: "private" })).toEqual({
      kind: "create-space",
      name: "Atelier",
      spaceKind: "atelier",
      visibility: "private",
    });
    expect(directoryCommandFromAction("os.directory.delete-space", { spaceId: "sp-1" })).toEqual({ kind: "delete-space", spaceId: "sp-1" });
    expect(directoryCommandFromAction("os.directory.rename-space", { spaceId: "sp-1", name: "New" })).toEqual({ kind: "rename-space", spaceId: "sp-1", name: "New" });
    expect(directoryCommandFromAction("os.directory.set-visibility", { spaceId: "sp-1", visibility: "public" })).toEqual({ kind: "set-visibility", spaceId: "sp-1", visibility: "public" });
    expect(directoryCommandFromAction("os.directory.upsert-member", { spaceId: "sp-1", email: "a@b.com", role: "author" })).toEqual({
      kind: "upsert-member",
      spaceId: "sp-1",
      email: "a@b.com",
      role: "author",
    });
    expect(directoryCommandFromAction("os.directory.remove-member", { spaceId: "sp-1", userId: "u-1" })).toEqual({ kind: "remove-member", spaceId: "sp-1", userId: "u-1" });
    expect(directoryCommandFromAction("os.directory.share-link", { spaceId: "sp-1", role: "spectator", ttlSecs: 60 })).toEqual({
      kind: "create-invite",
      spaceId: "sp-1",
      role: "spectator",
      ttlSecs: 60,
    });
    expect(directoryCommandFromAction("os.unknownVerb", {})).toBeNull();
  });

  it("mints a 32-hex nonzero correlation and retains bounded, request-id-keyed command results", () => {
    const first = mintDirectoryCommandRequestId();
    expect(first).toMatch(/^(?!0{32}$)[0-9a-f]{32}$/u);
    expect(mintDirectoryCommandRequestId()).not.toBe(first);

    const slots = new Map<string, DirectoryCommandResultSlotV1>();
    const receipt = {
      schema: "semio.directory.command-receipt.v1",
      requestId: first,
      commandSha256: "a".repeat(64),
      outcome: "accepted",
      events: [],
      result: { kind: "invite", inviteToken: "invite.v1.one-shot" },
      receiptSha256: "b".repeat(64),
    } as unknown as DirectoryCommandReceiptV1;
    retainDirectoryCommandResult(slots, first, { kind: "receipt", receipt });
    expect(slots.get(first)).toEqual({ kind: "receipt", receipt });
    retainDirectoryCommandResult(slots, first, { kind: "failed", code: "forbidden" });
    expect(slots.size).toBe(1);
    expect(slots.get(first)).toEqual({ kind: "failed", code: "forbidden" });

    for (let index = 0; index < DIRECTORY_COMMAND_RESULT_SLOTS + 4; index += 1) {
      retainDirectoryCommandResult(slots, index.toString(16).padStart(32, "c"), { kind: "failed", code: "request-conflict" });
    }
    expect(slots.size).toBe(DIRECTORY_COMMAND_RESULT_SLOTS);
    expect(slots.has(first)).toBe(false);
    expect(slots.has((DIRECTORY_COMMAND_RESULT_SLOTS + 3).toString(16).padStart(32, "c"))).toBe(true);
  });

  // 📇️ ticket 26/08/16/HUB-SPACES-LIVE-PRESENCE-AND-COLLABORATIVE-STUDIOS §C5 — save/check-in policy
  // (lane 3-A). `AutoCheckinScheduler` is deliberately framework-free (see its own doc) so its
  // debounce/storm-guard behaviour is verifiable with fake timers directly, without mounting
  // `ShellHost` — the same "no Worker/fetch/DirectoryClient mocking pattern exists yet" ceiling
  // `📓️w2-c-report.md` already documented applies here too; `ShellHost`'s own wiring of this
  // scheduler (and the pill's rendering into the sync tab) is reviewed, not click/mount-tested.
  describe("AutoCheckinScheduler (§C5 auto check-in)", () => {
    afterEach(() => {
      vi.useRealTimers();
    });

    it("3 edits then idle ⇒ exactly one commitCheckpoint", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(1);
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS - 1);
      scheduler.notify(2);
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS - 1);
      scheduler.notify(3);
      expect(onCheckpoint).not.toHaveBeenCalled();
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
      // 🎯️ "never a storm": more time passing without a fresh `notify` never fires a second time.
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS * 3);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
    });

    it("≥ 200 uncommitted edits ⇒ checkpoint without waiting for idle", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD - 1);
      expect(onCheckpoint).not.toHaveBeenCalled();
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
      // 🎯️ "never a storm": a second `notify` at/above the threshold before the checkpoint's own
      // `notify(0)` lands must not fire again.
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD + 1);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
    });

    it("notify(0) (a landed checkpoint) clears the pending latch for a fresh idle window later", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(AUTO_CHECKIN_EDIT_THRESHOLD);
      expect(onCheckpoint).toHaveBeenCalledTimes(1);
      scheduler.notify(0);
      scheduler.notify(1);
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS);
      expect(onCheckpoint).toHaveBeenCalledTimes(2);
    });

    it("cancel() stops a pending idle timer (unmount/session-switch)", () => {
      vi.useFakeTimers();
      const onCheckpoint = vi.fn();
      const scheduler = new AutoCheckinScheduler(onCheckpoint);
      scheduler.notify(1);
      scheduler.cancel();
      vi.advanceTimersByTime(AUTO_CHECKIN_IDLE_MS * 2);
      expect(onCheckpoint).not.toHaveBeenCalled();
    });

    it("an automatic check-in waits while its document cannot take it and is asked for again, per the shared corpus (live finding O4)", () => {
      for (const row of automaticCheckinCorpus.waits) expect(automaticCheckinWaitsV1(row.document), row.id).toBe(row.waits);
      for (const timeline of automaticCheckinCorpus.timelines) {
        vi.useFakeTimers();
        const start = Date.now();
        let document = { loading: false, attached: false, bound: false };
        const dispatched: number[] = [];
        const deferred: number[] = [];
        const scheduler: AutoCheckinScheduler = new AutoCheckinScheduler(() => {
          if (automaticCheckinWaitsV1(document)) {
            deferred.push(Date.now() - start);
            scheduler.defer();
          } else dispatched.push(Date.now() - start);
        }, automaticCheckinCorpus.idleMs, automaticCheckinCorpus.threshold);
        for (const event of timeline.events as readonly { readonly notify?: number; readonly advance?: number; readonly document?: typeof document }[]) {
          if (event.document !== undefined) document = event.document;
          else if (event.notify !== undefined) scheduler.notify(event.notify);
          else vi.advanceTimersByTime(event.advance ?? 0);
        }
        expect([dispatched, deferred], timeline.id).toEqual([timeline.dispatched, timeline.deferred]);
        scheduler.cancel();
        vi.useRealTimers();
      }
    });
  });

  describe("sync status pill (§C5 status pill, ArtifactSyncStatus → persisted|pending(n)|remote(...))", () => {
    it("persisted: a live remote with nothing pending", () => {
      const state = computeSyncPillState({ persisted: true, pendingMutations: 0, remote: { kind: "live", peerCount: 1 } });
      expect(state).toEqual({ kind: "persisted" });
      expect(syncPillText(state, "en")).toBe("Persisted");
      expect(syncPillText(state, "de")).toBe("Gespeichert");
    });

    it("pending(n): a live remote with unacked mutations", () => {
      const state = computeSyncPillState({ persisted: false, pendingMutations: 3, remote: { kind: "live", peerCount: 1 } });
      expect(state).toEqual({ kind: "pending", count: 3 });
      expect(syncPillText(state, "en")).toBe("Pending (3)");
      expect(syncPillText(state, "de")).toBe("Ausstehend (3)");
    });

    it("remote(connecting|backoff|detached): a non-live remote takes priority over a pending count", () => {
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "connecting" } }), "en")).toBe("Remote: connecting");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "connecting" } }), "de")).toBe("Remote: verbindet");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "backoff", retryInMs: 500 } }), "en")).toBe("Remote: backoff");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 9, remote: { kind: "backoff", retryInMs: 500 } }), "en")).toBe("Remote: backoff");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "detached" } }), "en")).toBe("Remote: detached");
      expect(syncPillText(computeSyncPillState({ persisted: false, pendingMutations: 0, remote: { kind: "detached" } }), "de")).toBe("Remote: getrennt");
    });

    it("no status observed yet reads as remote(detached)", () => {
      expect(computeSyncPillState(null)).toEqual({ kind: "remote", remote: "detached" });
    });
  });

  // 🧪️ §C5 item 5: "viewers never checkpoint" — `canCheckIn` is the SAME predicate `ShellHost` gates
  // both the `#s-checkin`/checkpoint footer items (JSX presence, `!canCheckIn(session.app.role)`) and
  // the auto-checkin scheduler's arming (`isEditorSession`) with, so this one test covers both call
  // sites' logic without needing to mount `ShellHost` itself.
  it("canCheckIn is true only for an editor role — viewer gets no affordance and no auto timer", () => {
    expect(canCheckIn("editor")).toBe(true);
    expect(canCheckIn("viewer")).toBe(false);
    expect(canCheckIn(undefined)).toBe(false);
  });

  it("isolates render faults in ShellFaultBoundary", () => {
    function FaultyChild(): ReactElement {
      throw new Error("boom");
    }
    const { getByRole } = render(createElement(ShellFaultBoundary, { boundaryId: "test", fallbackLabel: uiDataLabel("Fault"), children: createElement(FaultyChild) }));
    expect(getByRole("alert").textContent).toContain("boom");
  });

  it("folds spawned focus into viewState so a subsequent host-effect session write keeps activeSpawnedId", async () => {
    const panel = { activePanelTab: "s-play-catalogue", spawnedApps: [] as const };
    const spawned = { id: "app-draw-1", pluginId: "draw", instanceId: 1, appId: "draw", label: "Semio Emblem", breadcrumb: ["draw"] };
    const focused = studioPanelFocusingSpawned(panel, spawned);
    expect(focused.activeSpawnedId).toBe("app-draw-1");
    expect(focused.spawnedApps).toEqual([spawned]);
    // 🐚️ Simulate applyHostEffects: fold into nextViewState, then a final SET_SESSION commits that
    // viewState (the bug was committing the pre-spawn viewState and wiping activeSpawnedId).
    const baseViewState = { panelJson: JSON.stringify(panel) };
    const nextViewState = viewStateWithSpacePanel(baseViewState, focused);
    const { parsePanelState } = await import("../../🧱️elements/🛠️ShellHelpers/📌️panel/🟦️.ts");
    expect(parsePanelState(nextViewState)?.activeSpawnedId).toBe("app-draw-1");
    const refocused = studioPanelFocusingSpawned(focused, { ...spawned, label: "Renamed" });
    expect(refocused.spawnedApps).toHaveLength(1);
    expect(refocused.spawnedApps[0]?.label).toBe("Renamed");
    expect(refocused.activeSpawnedId).toBe("app-draw-1");
  });
});

describe("ui search/find (fuse re-export from @semio-tech/ui-react)", () => {
  // Command dialogs render via a Radix Portal into `document.body`, not into the render() container, so assertions query `document.body`.
  // This package's vitest config has no shared setupFile, so tests here clean up their own portal-rendered DOM.
  afterEach(async () => {
    const { cleanup } = await import("@semio-tech/ui-react/test");
    cleanup();
  });

  it("UISearch renders all items and fuzzy-filters them through the owned ranker", async () => {
    const { render, fireEvent } = await import("@semio-tech/ui-react/test");
    const selected = vi.fn();
    const openChange = vi.fn();
    const items: UISearchItem[] = [
      { id: "a", label: "Alpha", category: "Test", onSelect: selected },
      { id: "b", label: "Bravo", category: "Test", onSelect: noopAction },
    ];
    render(createElement(UIFindProvider, null, createElement(UISearch, { items, open: true, onOpenChange: openChange })));
    expect(document.body.textContent).toContain("Alpha");
    expect(document.body.textContent).toContain("Bravo");
    const input = document.querySelector('[data-slot="command-input"]') as HTMLInputElement;
    expect(input).not.toBeNull();
    fireEvent.change(input, { target: { value: "alp" } });
    expect(document.body.textContent).toContain("Alpha");
    expect(document.body.textContent).not.toContain("Bravo");
    fireEvent.keyDown(input, { key: "Enter" });
    expect(openChange).toHaveBeenCalledWith(false);
    expect(selected).toHaveBeenCalledTimes(1);
  });

  it("UIFind renders and fuzzy-filters items registered on its context through the owned ranker", async () => {
    const { render, fireEvent, act } = await import("@semio-tech/ui-react/test");
    let contextValue: ReturnType<typeof useUIFind> | undefined;
    const Harness = () => {
      contextValue = useUIFind();
      return createElement(UIFind, { open: true, onOpenChange: noopAction });
    };
    render(createElement(UIFindProvider, null, createElement(Harness)));
    act(() => {
      contextValue!.setFindItems([
        { id: "1", label: "Chair", category: "Test" },
        { id: "2", label: "Table", category: "Test" },
      ]);
    });
    const selected = vi.fn();
    act(() => contextValue!.setOnFindItem(selected));
    expect(document.body.textContent).toContain("Chair");
    expect(document.body.textContent).toContain("Table");
    const input = document.querySelector('[data-slot="command-input"]') as HTMLInputElement;
    expect(input).not.toBeNull();
    fireEvent.change(input, { target: { value: "cha" } });
    expect(document.body.textContent).toContain("Chair");
    expect(document.body.textContent).not.toContain("Table");
    fireEvent.keyDown(input, { key: "Enter" });
    expect(selected).toHaveBeenCalledWith("1");
  });
});
