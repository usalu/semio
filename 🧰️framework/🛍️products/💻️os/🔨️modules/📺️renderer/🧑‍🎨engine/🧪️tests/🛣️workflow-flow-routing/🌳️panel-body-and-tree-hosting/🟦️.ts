/** 🌳️ The panel body loading, empty state, scope request and tree hosting tests. */
import { uiI18n } from "@semio-tech/ui-react";
import { render } from "@semio-tech/ui-react/test";
import { expect, it } from "vitest";
import { pendingPanelUiNode } from "@semio-tech/framework";
import { resolveTranslationLabel } from "@semio-tech/ui-react";
import { buildUiRefreshRequest, uiNodeToTreePanelConfig } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { buildContractNode, noopAction, panelTreePanelHost } from "../../🧰️engine-contract-helpers/🟦️.ts";

/** 🏠 Registers the loading panel body, idle tree empty state, partial scope request and semantic tree leaf hosting tests. */
export function registerPanelBodyAndTreeHostingTests(): void {
  it("renders a panel body that has not arrived yet as a loading surface, never as a silently empty panel", () => {
    const rendered = render(panelTreePanelHost(uiNodeToTreePanelConfig(pendingPanelUiNode(), noopAction, "framework.panel.inspection")));
    expect(rendered.container.querySelector('[data-ui-status="loading"]')).toBeTruthy();
    expect(rendered.container.querySelector('[aria-busy="true"]')).toBeTruthy();
  });
  it("renders an idle tree body with no sections as an explicit empty state, distinct from loading", () => {
    const idleEmpty = { ...pendingPanelUiNode(), activity: "idle" as const };
    const empty = render(panelTreePanelHost(uiNodeToTreePanelConfig(idleEmpty, noopAction, "framework.panel.inspection")));
    expect(empty.container.querySelector('[data-ui-status="loading"]')).toBeNull();
    expect(empty.container.textContent?.replace(/\u2026/g, "").trim()).not.toBe("");
    expect(empty.container.textContent).toContain(resolveTranslationLabel(uiI18n.t("ui.common.noData")));
  });
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
}
