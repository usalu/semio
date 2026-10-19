/** 🖼️ The node graph host rendering, gesture retirement and viewport action tests. */
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import Ajv2020 from "ajv/dist/2020";
import { uiViewport2dSchema as viewport2dSchema } from "@semio-tech/framework/ui/testing/fixtures";
import { viewportPoseFixture } from "@semio-tech/framework/ui/testing/fixtures";
import * as flowSessionLoader from "../../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { cleanup, fireEvent, render, waitFor } from "@semio-tech/ui-react/test";
import { afterEach, expect, it, vi } from "vitest";
import { NodeGraphHost, nodeGraphViewportActionArgs, parseNodeGraphSessionViewport } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { noopAction } from "../../🧰️engine-contract-helpers/🟦️.ts";

/** 🪟 Registers the workflow scene rendering, mounted gesture retirement, editable find items, live viewport and neutral viewport schema tests. */
export function registerHostRenderingAndGestureTests(): void {
  afterEach(() => cleanup());
  it("renders node graph host from workflow scene json", () => {
    const markup = renderToStaticMarkup(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.workflow",
          controllerId: "s-play",
          componentKind: "node-graph",
          nodeGraph: {
            nodes: [
              {
                id: "node-a",
                instanceId: "app-a",
                label: "Draw",
                x: 10,
                y: 20,
                width: 160,
                height: 80,
                inputs: [{ id: "in", resourceKind: "2d.drawing" }],
                outputs: [{ id: "out", resourceKind: "2d.drawing" }],
              },
            ],
            edges: [],
            viewport: { x: 0, y: 0, zoom: 1 },
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-node-graph-host");
  });
  it("retires a mounted graph gesture without synthesizing an up or graph commit", async () => {
    const attachCanvas = vi.fn(async () => {});
    const pointerDownScreen = vi.fn();
    const pointerUpScreen = vi.fn();
    const pointerCancelScreen = vi.fn();
    const onAction = vi.fn();
    const session = {
      attachCanvas,
      setCaretVisible: vi.fn(),
      setSize: () => {},
      renderFrame: vi.fn(),

      synchronizeScene: () => {},
      setCanvasThemeJson: () => {},
      pointerDownScreen,
      pointerMoveScreen: () => {},
      pointerUpScreen,
      pointerCancelScreen,
      wheelScreen: () => {},
      labelOverlayPaintStateJson: () => '{"labels":[]}',
      sliderOverlayStateJson: () => "{}",
      selectionUnionBoundsScreenJson: () => "{}",
      selectionPreviewPointsJson: () => "[]",
      selectionPreviewCrossing: () => false,
      selectionPreviewMethod: () => "rectangle",
      selectedNodeIdsJson: () => "[]",
      hoveredNodeId: () => null,
      hoveredChannelJson: () => '{"channel":null,"refusal":null}',
      viewport: () => ({ x: 0, y: 0, zoom: 1 }),
      pickTargetsAtScreenJson: () => "[]",
      setHover: () => {},
      setHoverChannel: () => {},
      alignSelection: () => {},
      hostSnapshotJson: () => "{}",
      takePendingOpenInstanceId: () => null,
      free: vi.fn(),
    } as unknown as Awaited<ReturnType<typeof flowSessionLoader.createGraphSession>>;
    const factory = vi.spyOn(flowSessionLoader, "createGraphSession").mockResolvedValue(session);
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
    const canvasContext = vi.spyOn(HTMLCanvasElement.prototype, "getContext").mockReturnValue(null);
    const view = render(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "graph.cancel",
          controllerId: "graph",
          componentKind: "node-graph",
          nodeGraph: {
            nodes: [{ id: "node-a", instanceId: "app-a", label: "Draw", x: 10, y: 20, width: 160, height: 80, inputs: [], outputs: [] }],
            edges: [],
            viewport: { x: 0, y: 0, zoom: 1 },
            editable: true,
            interactionDomain: { id: "graph.cancel", nodeTargetPrefix: "node:", edgeTargetPrefix: "edge:", handleTargetPrefix: "handle:" },
          },
        },
        onAction,
      }),
    );
    try {
      await waitFor(() => expect(attachCanvas).toHaveBeenCalledOnce());
      const surface = view.container.querySelector('.semio-node-graph-host [class*="z-30"]') as HTMLElement;
      await waitFor(() => {
        fireEvent.pointerDown(surface, { pointerId: 11, button: 0, clientX: 20, clientY: 20 });
        expect(pointerDownScreen).toHaveBeenCalled();
      });
      const downsBeforeCancel = pointerDownScreen.mock.calls.length;
      const actionsBeforeCancel = onAction.mock.calls.length;
      fireEvent.pointerCancel(surface, { pointerId: 11, clientX: 30, clientY: 20 });
      expect(pointerCancelScreen).toHaveBeenCalledOnce();
      expect(pointerUpScreen).not.toHaveBeenCalled();
      expect(onAction.mock.calls.slice(actionsBeforeCancel).map(([action]) => action.action)).toEqual(["interactionHover"]);
      fireEvent.pointerDown(surface, { pointerId: 12, button: 0, clientX: 40, clientY: 20 });
      expect(pointerDownScreen).toHaveBeenCalledTimes(downsBeforeCancel + 1);
    } finally {
      view.unmount();
      factory.mockRestore();
      bounds.mockRestore();
      canvasContext.mockRestore();
    }
  });
  it("renders editable node graph host with find items", () => {
    const markup = renderToStaticMarkup(
      createElement(NodeGraphHost, {
        node: {
          type: "componentScene",
          surfaceId: "s.play.workflow",
          controllerId: "s-play",
          componentKind: "node-graph",
          nodeGraph: {
            nodes: [
              {
                id: "node-a",
                instanceId: "app-a",
                label: "Draw",
                x: 10,
                y: 20,
                width: 160,
                height: 80,
                inputs: [{ id: "in", resourceKind: "2d.drawing" }],
                outputs: [{ id: "out", resourceKind: "2d.drawing" }],
              },
            ],
            edges: [],
            viewport: { x: 0, y: 0, zoom: 1 },
            editable: true,
            findItems: [{ id: "app-a", label: "Draw", category: "Workflow" }],
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-node-graph-host");
  });
  it("uses the live session viewport for node graph wheel actions", () => {
    expect(parseNodeGraphSessionViewport({ x: 12, y: 24, zoom: 1.75 })).toEqual({ x: 12, y: 24, zoom: 1.75 });
    expect(nodeGraphViewportActionArgs({ x: 12, y: 24, zoom: 1.75 })).toEqual({
      viewport: { x: 12, y: 24, zoom: 1.75 },
    });
    expect(() => parseNodeGraphSessionViewport('{"x":12,"y":24,"zoom":1.75}')).toThrow();
    expect(() => nodeGraphViewportActionArgs({ x: 0, y: 0, zoom: 0 })).toThrow();
    expect(() => nodeGraphViewportActionArgs({ x: 0, y: 0, zoom: Number.NaN })).toThrow();
    expect(() => nodeGraphViewportActionArgs({ x: 0, y: 0, zoom: 1, extra: true } as never)).toThrow();
  });
  it("matches the shared neutral viewport schema at the node graph action boundary", () => {
    const validate = new Ajv2020({ strict: true, allErrors: true }).compile(viewport2dSchema);
    const cases = viewportPoseFixture.cases.filter((row) => row.dimension === "2d");
    for (const row of cases) {
      expect(validate(row.value), row.name).toBe(row.valid);
      if (row.valid) {
        expect(nodeGraphViewportActionArgs(parseNodeGraphSessionViewport(row.value)), row.name).toEqual({ viewport: row.value });
      } else {
        expect(() => nodeGraphViewportActionArgs(parseNodeGraphSessionViewport(row.value)), row.name).toThrow();
      }
    }
    expect(cases).toHaveLength(8);
  });
}
