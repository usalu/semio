/** 🔗️ The catalogue row activation and the semantic intent action channel tests. */
import { createElement } from "react";
import { cleanup, fireEvent, render } from "@semio-tech/ui-react/test";
import { expect, it } from "vitest";
import { type UiIntent, type ActionDescriptor } from "@semio-tech/framework";
import { resolveHostSnapshotWidgetInstanceId, UiPresenceOverlayContext, type UiPresenceOverlayEntry, uiIntentToActionDescriptor, uiNodeToTreePanelConfig } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { buildContractNode, panelTreePanelHost } from "../../🧰️engine-contract-helpers/🟦️.ts";

/** 📡 Registers the expandable catalogue row activation, intent bridge, scalar payload, outliner row action and fixture widget id tests. */
export function registerCatalogueRowAndIntentChannelTests(): void {
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
    expect(row?.getAttribute("data-activatable")).toBe("true");
    const expected = { controllerId: "puzzle3d-play", action: "addObjectKind", args: { objectKind: "Hexagonal Cut Concrete Forest Left" } };
    fireEvent.click(row as Element);
    expect(dispatched).toEqual([expected]);
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
    expect(uiIntentToActionDescriptor({ ...base, trigger: "delta", input: -0.5 } as UiIntent)).toEqual({
      controllerId: "puzzle3d-play",
      action: "setGridSpacing",
      args: { windowId: "puzzle3d-main-top", delta: -0.5 },
    });
    expect(uiIntentToActionDescriptor({ ...base, args: null, trigger: "commit", input: "fill 12" } as UiIntent)).toEqual({
      controllerId: "puzzle3d-play",
      action: "setGridSpacing",
      args: { value: "fill 12" },
    });
  });
  it("carries an outliner row action's explicit ids to the action channel whatever else is selected", () => {
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
}
