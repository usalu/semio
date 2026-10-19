/** 🖼️ The flow frame readiness, engine selection and presence peer tests. */
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { expect, it } from "vitest";
import { NodeGraphHost, flowSurfaceRenderAllowed, isFlowGraphScene } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { noopAction } from "../../🧰️engine-contract-helpers/🟦️.ts";

/** 🪟 Registers the flow frame readiness, flow engine selection and scene payload presence peer tests. */
export function registerFlowFrameAndPresenceTests(): void {
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
}
