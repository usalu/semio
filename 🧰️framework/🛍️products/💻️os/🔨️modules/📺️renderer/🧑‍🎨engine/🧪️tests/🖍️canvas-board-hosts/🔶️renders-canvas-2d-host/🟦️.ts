/** 🔶️ The renders canvas 2d host tests of the framework renderer hosts suite. */
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import { cleanup } from "@semio-tech/ui-react/test";
import { afterEach, expect, it } from "vitest";
import { Canvas2dHost, Board2dHost, applyBoard2dHighlightedIds, board2dCameraActionArgs, coalesceBoard2dEvents } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { noopAction } from "../../🧰️engine-contract-helpers/🟦️.ts";

/** 🔷 Registers the renders canvas 2d host tests of the framework renderer hosts suite. */
export function registerRendersCanvas2dHostTests(): void {
  afterEach(() => cleanup());
  it("renders canvas 2d host with infinite canvas session", () => {
    const markup = renderToStaticMarkup(
      createElement(Canvas2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "draw.play.canvas",
          controllerId: "draw-play",
          componentKind: "canvas-2d",
          canvas2d: {
            cameraX: 0,
            cameraY: 0,
            zoom: 1,
            layersJson: JSON.stringify([{ id: "layer-1", name: "Layer 1", x: 0, y: 0, width: 120, height: 80 }]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-canvas-2d-host");
  });
  it("renders canvas 2d host with draw gradient/blend/overlay/meta scene records", () => {
    const markup = renderToStaticMarkup(
      createElement(Canvas2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "draw.play.canvas",
          controllerId: "draw-play",
          componentKind: "canvas-2d",
          canvas2d: {
            cameraX: 0,
            cameraY: 0,
            zoom: 1,
            layersJson: JSON.stringify([
              { id: "meta:utility", role: "meta", utility: "selectDirect" },
              {
                id: "shape-1",
                transform: [1, 0, 0, 1, 0, 0],
                segments: [{ kind: "move", to: [0, 0] }, { kind: "line", to: [10, 0] }, { kind: "line", to: [10, 10] }, { kind: "close" }],
                fill: {
                  kind: "linearGradient",
                  x1: 0,
                  y1: 0,
                  x2: 10,
                  y2: 10,
                  stops: [
                    { offset: 0, color: [1, 0, 0, 1] },
                    { offset: 1, color: [0, 0, 1, 1] },
                  ],
                },
                stroke: { color: [0, 0, 0, 1], width: 1, cap: "round", join: "round" },
                opacity: 1,
                blendMode: "multiply",
                visible: true,
                fillRule: "evenodd",
              },
              {
                id: "overlay:sel:shape-1",
                role: "overlay",
                transform: [1, 0, 0, 1, 0, 0],
                segments: [{ kind: "move", to: [0, 0] }, { kind: "line", to: [10, 0] }, { kind: "close" }],
                fill: { kind: "solid", color: [0.98, 0.75, 0.14, 0.16] },
                stroke: { color: [0.98, 0.75, 0.14, 0.95], width: 2 },
                opacity: 1,
                blendMode: "normal",
                visible: true,
                fillRule: "evenodd",
              },
            ]),
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-canvas-2d-host");
  });
  it("renders puzzle 2d board host shell", () => {
    const markup = renderToStaticMarkup(
      createElement(Board2dHost, {
        node: {
          type: "componentScene",
          surfaceId: "puzzle2d.play.composite.2d-overview",
          controllerId: "puzzle2d-play",
          componentKind: "board-2d",
          board2d: {
            snapshotJson: JSON.stringify({ nodes: [], edges: [], camera: { x: 0, y: 0, zoom: 1 } }),
            cameraJson: '{"x":0,"y":0,"zoom":1}',
            glyphCatalogsJson: "{}",
            selectionJson: "[]",
            interactive: true,
            selectionMethod: "rectangle",
            gridSnapEnabled: false,
            gridFactor: 1,
            suggestionOffset: 0,
            brushWeightsJson: "{}",
            placementCompatibilityJson: "[]",
            lodMode: "automatic",
          },
        },
        onAction: noopAction,
      }),
    );
    expect(markup).toContain("semio-board-2d-host");
  });
  it("projects the ids a time-travel draft references onto the puzzle 2d board host", () => {
    const board2d = {
      snapshotJson: JSON.stringify({ nodes: [], edges: [], camera: { x: 0, y: 0, zoom: 1 } }),
      cameraJson: '{"x":0,"y":0,"zoom":1}',
      glyphCatalogsJson: "{}",
      selectionJson: "[]",
      interactive: true,
      selectionMethod: "rectangle",
      gridSnapEnabled: false,
      gridFactor: 1,
      suggestionOffset: 0,
      brushWeightsJson: "{}",
      placementCompatibilityJson: "[]",
      lodMode: "automatic",
    };
    const mount = (highlightedIdsJson?: string) =>
      renderToStaticMarkup(createElement(Board2dHost, { node: { type: "componentScene", surfaceId: "puzzle2d.play.composite.2d-overview", controllerId: "puzzle2d-play", componentKind: "board-2d", board2d: { ...board2d, highlightedIdsJson } }, onAction: noopAction }));
    expect(mount('["mid","left"]')).toContain('data-board-highlighted-ids-json="[&quot;mid&quot;,&quot;left&quot;]"');
    expect(mount(undefined)).toContain('data-board-highlighted-ids-json="[]"');
  });
  it("forwards the ids a time-travel draft references to the puzzle 2d board session as the wgpu board sync does", () => {
    type Session = Parameters<typeof applyBoard2dHighlightedIds>[0];
    const forwarded: string[] = [];
    const session = { setHighlightedIdsJson: (json: string) => void forwarded.push(json) } as unknown as Session;
    applyBoard2dHighlightedIds(session, { highlightedIdsJson: '["mid","left"]' });
    applyBoard2dHighlightedIds(session, {});
    applyBoard2dHighlightedIds(null, { highlightedIdsJson: '["ghost"]' });
    applyBoard2dHighlightedIds({} as unknown as Session, { highlightedIdsJson: '["ghost"]' });
    applyBoard2dHighlightedIds({ setHighlightedIdsJson: () => { throw new Error("session not ready"); } } as unknown as Session, { highlightedIdsJson: '["ghost"]' });
    expect(forwarded).toEqual(['["mid","left"]', "[]"]);
  });
  it("uses the live puzzle 2d board camera for wheel persistence actions", () => {
    expect(board2dCameraActionArgs('{"x":345,"y":-123,"zoom":4.25}')).toEqual({
      camera: { x: 345, y: -123, zoom: 4.25 },
    });
  });
  it("coalesces puzzle 2d board events: drops transients and live nodeMove frames, takes the latest camera out for the view lane", () => {
    const rows = [
      { name: "preselect", payload: { ids: ["a"] } },
      { name: "camera", payload: { x: 1, y: 1, zoom: 1 } },
      { name: "nodeMove", payload: { id: "alpha", x: 10, y: 10 } },
      { name: "camera", payload: { x: 2, y: 2, zoom: 1.5 } },
      { name: "nodeMove", payload: { id: "beta", x: 5, y: 5 } },
    ];
    const { flushNow, eventsJson, camera } = coalesceBoard2dEvents(rows);
    expect(flushNow).toBe(false);
    expect(JSON.parse(eventsJson)).toEqual([]);
    expect(camera).toEqual({ x: 2, y: 2, zoom: 1.5 });
  });
}
