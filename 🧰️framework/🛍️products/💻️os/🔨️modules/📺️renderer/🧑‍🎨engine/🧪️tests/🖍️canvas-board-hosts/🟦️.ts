/** 🖍 Canvas 2d host, puzzle 2d board host, board event coalescing, gumball and peer presence. */
import { boardTestSession } from "../../🧱️elements/🪪️WasmSessionLoader/🔮️oracles/🪪️session-double/🟦️.ts";
import { createElement } from "react";
import { renderToStaticMarkup } from "react-dom/server";
import * as flowSessionLoader from "../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { cleanup } from "@semio-tech/ui-react/test";
import { afterEach, describe, expect, it } from "vitest";
import { Canvas2dHost, Board2dHost, applyBoard2dHighlightedIds, board2dCameraActionArgs, beginBoardPeerGesture, collectBoardLiveMirrorMutations, board2dGranularityById, board2dHoverActionArgs, latestBoard2dHoverId, parseBoard2dSuggestionMenu, board2dSuggestionMenuOwnsWindow, board2dSuggestionMenuItems, coalesceBoard2dEvents, parseBoard2dTransformFlags, board2dStatusJson, endBoardPeerGesture, notifyBoardPeersGestureEnded, board2dPeers, boardPeerOwnsGesture, pushBoardLiveMirrorMutations, registerBoard2dPeer, unregisterBoard2dPeer } from "../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import { noopAction } from "../🧰️engine-contract-helpers/🟦️.ts";

describe("framework renderer hosts", () => {
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

  it("coalesces puzzle 2d board events: drops transients and live nodeMove frames, keeps the latest camera", () => {
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
    expect(camera).toEqual({x:2,y:2,zoom:1.5});
  });

it("keeps hover out of the board-events batch — it travels on the framework interactionHover lane instead", () => {
    const { eventsJson, flushNow } = coalesceBoard2dEvents([
      { name: "hover", payload: { id: "alpha", kind: null } },
      { name: "camera", payload: { x: 0, y: 0, zoom: 1 } },
    ]);
    expect(JSON.parse(eventsJson) as { name: string }[]).not.toContainEqual(expect.objectContaining({ name: "hover" }));
    expect(flushNow).toBe(false);
  });

  it("reads the LAST hover row of a batch — a string id hovers, an empty/absent id clears, no row leaves it alone", () => {
    expect(
      latestBoard2dHoverId([
        { name: "hover", payload: { id: "alpha" } },
        { name: "hover", payload: { id: "beta" } },
      ]),
    ).toBe("beta");
    expect(
      latestBoard2dHoverId([
        { name: "hover", payload: { id: "alpha" } },
        { name: "hover", payload: { id: null } },
      ]),
    ).toBeNull();
    expect(latestBoard2dHoverId([{ name: "camera", payload: {} }])).toBeUndefined();
  });

  it("classifies every board id into its vortex-domain granularity and publishes it on the interactionHover wire", () => {
    const fixture = JSON.stringify({ nodes: [{ id: "n1", handles: [{ id: "n1:h0" }] }], edges: [{ id: "e1" }] });
    const byId = board2dGranularityById(fixture);
    expect([byId.get("n1"), byId.get("n1:h0"), byId.get("e1"), byId.get("ghost")]).toEqual(["node", "handle", "edge", undefined]);
    expect(board2dHoverActionArgs("vortex", "handle", "n1:h0")).toEqual({ domainId: "vortex", channel: "pointer", targets: JSON.stringify([{ granularity: "handle", id: "n1:h0" }]) });
    expect(JSON.parse(board2dHoverActionArgs("vortex", "node", null).targets)).toEqual([]);
  });

  it("parses the handle-suggestions popup, scopes it to its own window, and renders hover-preview rows distinct from the commit", () => {
    const encoded = JSON.stringify({
      open: true,
      x: 12,
      y: 34,
      windowId: "w1",
      handleId: "n1:h0",
      hoveredIndex: 1,
      pending: false,
      candidates: [
        { index: 0, nodeLabel: "beam", handleLabel: "handle 0" },
        { index: 1, nodeLabel: "slab", handleLabel: "handle 1", icon: "square" },
      ],
    });
    const menu = parseBoard2dSuggestionMenu(encoded)!;
    expect(menu).toMatchObject({ open: true, x: 12, y: 34, windowId: "w1", handleId: "n1:h0", hoveredIndex: 1 });
    expect(board2dSuggestionMenuOwnsWindow(menu, "w1")).toBe(true);
    expect(board2dSuggestionMenuOwnsWindow(menu, "w2")).toBe(false);
    expect(parseBoard2dSuggestionMenu(undefined)).toBeNull();
    expect(parseBoard2dSuggestionMenu(JSON.stringify({ open: false }))).toBeNull();
    const rows = board2dSuggestionMenuItems(menu, { checkingPlacement: "checking", noPlacement: "none" });
    expect(rows.map((row) => row.action)).toEqual(["acceptSuggestion", "acceptSuggestion"]);
    expect(rows.map((row) => row.hoverAction)).toEqual(["hoverSuggestion", "hoverSuggestion"]);
    expect(rows.map((row) => row.checked)).toEqual([false, true]);
    expect(rows[1]?.args).toEqual({ index: 1, handleId: "n1:h0" });
    expect(rows[1]?.hoverArgs).toEqual({ index: 1, handleId: "n1:h0" });
  });

  it("refuses politely: a pending slot shows one disabled checking row, a resolved empty slot one disabled no-placement row", () => {
    const labels = { checkingPlacement: "checking", noPlacement: "none" };
    const pending = board2dSuggestionMenuItems(parseBoard2dSuggestionMenu(JSON.stringify({ open: true, x: 0, y: 0, hoveredIndex: 0, pending: true, candidates: [] }))!, labels);
    expect(pending).toEqual([{ id: "pending", label: "checking", disabled: true }]);
    const empty = board2dSuggestionMenuItems(parseBoard2dSuggestionMenu(JSON.stringify({ open: true, x: 0, y: 0, hoveredIndex: 0, pending: false, candidates: [] }))!, labels);
    expect(empty).toEqual([{ id: "empty", label: "none", disabled: true }]);
  });

  it("coalesces puzzle 2d board events: one drag is its tagged select plus its gesture record, frames dropped", () => {
    const select = { name: "select", payload: { ids: ["alpha"], exitHighlightIds: [], gestureId: "gesture-1" } };
    const record = { name: "gesture", payload: { gestureId: "gesture-1", kind: "drag", targets: ["alpha"], dx: 10, dy: 0, proximity: [] } };
    const { eventsJson, flushNow } = coalesceBoard2dEvents([select, { name: "nodeMove", payload: { id: "alpha", x: 10, y: 10 } }, record]);
    expect(flushNow).toBe(true);
    expect(JSON.parse(eventsJson)).toEqual([select, record]);
    expect(coalesceBoard2dEvents([select]).flushNow).toBe(false);
  });

  it("flushes puzzle 2d board events immediately for gesture/select/brushPlace/edge/delete/region rows, not for camera/nodeMove alone", () => {
    expect(coalesceBoard2dEvents([{ name: "camera", payload: { x: 0, y: 0, zoom: 1 } }]).flushNow).toBe(false);
    expect(coalesceBoard2dEvents([{ name: "nodeMove", payload: { id: "alpha", x: 0, y: 0 } }]).flushNow).toBe(false);
    for (const name of ["gesture", "select", "preselectCancel", "brushCandidates", "brushPlace", "edgeCreate", "edgeDelete", "nodeDelete", "regionCreate", "regionResize"]) {
      expect(coalesceBoard2dEvents([{ name, payload: {} }]).flushNow).toBe(true);
    }
  });

  it("collects live mirror mutations: coalesces nodeMove to the latest per id, ignores unrelated rows", () => {
    const mutations = collectBoardLiveMirrorMutations([
      { name: "camera", payload: { x: 1, y: 1, zoom: 1 } },
      { name: "nodeMove", payload: { id: "alpha", x: 1, y: 1 } },
      { name: "brushPreview", payload: {} },
      { name: "nodeMove", payload: { id: "alpha", x: 9, y: 9 } },
      { name: "nodeMove", payload: { id: "beta", x: 2, y: 2 } },
    ]);
    expect(mutations.positions).toEqual([
      { id: "alpha", x: 9, y: 9 },
      { id: "beta", x: 2, y: 2 },
    ]);
    expect(mutations.selectionIds).toBeNull();
    expect(mutations.preselect).toBeNull();
    expect(mutations.clearPreselect).toBe(false);
  });

  it("board 2d gumball: a rotate record flushes at once and its live preview frames never reach the guest", () => {
    const { flushNow, eventsJson } = coalesceBoard2dEvents([
      { name: "transformPreview", payload: { moves: [{ id: "alpha", x: 1, y: 1 }] } },
      { name: "gesture", payload: { gestureId: "gesture-2", kind: "rotate", targets: ["alpha"], pivotX: 0, pivotY: 0, angle: 0.5, proximity: [] } },
    ]);
    expect(flushNow).toBe(true);
    const events = JSON.parse(eventsJson) as { name: string }[];
    expect(events.map((event) => event.name)).toEqual(["gesture"]);
    expect(coalesceBoard2dEvents([{ name: "transformPreview", payload: { moves: [] } }]).flushNow).toBe(false);
  });

  it("board 2d gumball: transformPreview frames mirror into sibling panes exactly like a drag's final moves", () => {
    const mutations = collectBoardLiveMirrorMutations([
      { name: "transformPreview", payload: { moves: [{ id: "alpha", x: 3, y: 4 }] } },
      {
        name: "transformPreview",
        payload: {
          moves: [
            { id: "alpha", x: 0, y: 5 },
            { id: "beta", x: -5, y: 0 },
          ],
        },
      },
    ]);
    expect(mutations.positions).toEqual([
      { id: "alpha", x: 0, y: 5 },
      { id: "beta", x: -5, y: 0 },
    ]);
    expect(mutations.selectionIds).toBeNull();
  });

  it("board 2d gumball: transformFlags default to move+rotate and never disarm on malformed input", () => {
    expect(parseBoard2dTransformFlags(JSON.stringify({ move: false, rotate: true }))).toEqual({ move: false, rotate: true });
    expect(parseBoard2dTransformFlags(JSON.stringify({ rotate: false }))).toEqual({ move: true, rotate: false });
    for (const encoded of [undefined, null, "", "not json", "{}", JSON.stringify({ move: "yes" })]) {
      expect(parseBoard2dTransformFlags(encoded)).toEqual({ move: true, rotate: true });
    }
  });

  it("board 2d vitals: the status row names the fixture verdict, its size, the refusal and the pending flush", () => {
    expect(JSON.parse(board2dStatusJson({ snapshotParsed: true, snapshotChars: 128, refusalReason: "", pendingEvents: 0, guestRevision: 3 }))).toEqual({
      snapshotParsed: true,
      snapshotChars: 128,
      refusalReason: "",
      pendingEvents: 0,
      guestRevision: 3,
    });
    const refused = JSON.parse(board2dStatusJson({ snapshotParsed: false, snapshotChars: 9, refusalReason: "engine refused the fixture", pendingEvents: 2, guestRevision: 1 })) as { snapshotParsed: boolean; refusalReason: string };
    expect(refused.snapshotParsed).toBe(false);
    expect(refused.refusalReason).toBe("engine refused the fixture");
  });

  it("collects live mirror mutations: preselect sets the live highlight, select/preselectCancel commit selection and clear it", () => {
    expect(collectBoardLiveMirrorMutations([{ name: "preselect", payload: { ids: ["a", "b"], removedIds: ["c"] } }])).toMatchObject({
      preselect: { ids: ["a", "b"], removedIds: ["c"] },
      clearPreselect: false,
      selectionIds: null,
    });
    expect(collectBoardLiveMirrorMutations([{ name: "select", payload: { ids: ["a"] } }])).toMatchObject({
      selectionIds: ["a"],
      preselect: null,
      clearPreselect: true,
    });
    expect(collectBoardLiveMirrorMutations([{ name: "preselectCancel", payload: { ids: ["a", "b"] } }])).toMatchObject({
      selectionIds: ["a", "b"],
      preselect: null,
      clearPreselect: true,
    });
  });

  it("peer registry: registers/unregisters, excludes own surfaceId and other controllerIds", () => {
    const scope = flowSessionLoader.createBoardPeerScope();
    const peerA = { session: {} as never, onPeerGestureEnded: () => {} };
    const peerB = { session: {} as never, onPeerGestureEnded: () => {} };
    const peerOther = { session: {} as never, onPeerGestureEnded: () => {} };
    registerBoard2dPeer(scope, "puzzle2d-play", "pane.a", peerA);
    registerBoard2dPeer(scope, "puzzle2d-play", "pane.b", peerB);
    registerBoard2dPeer(scope, "other-controller", "pane.a", peerOther);

    expect(board2dPeers(scope, "puzzle2d-play", "pane.a")).toEqual([peerB]);
    expect(board2dPeers(scope, "puzzle2d-play", "pane.b")).toEqual([peerA]);
    expect(board2dPeers(scope, "other-controller", "pane.z")).toEqual([peerOther]);

    unregisterBoard2dPeer(scope, "puzzle2d-play", "pane.b", scope.peers.get("puzzle2d-play")?.get("pane.b") ?? null);
    expect(board2dPeers(scope, "puzzle2d-play", "pane.a")).toEqual([]);

    unregisterBoard2dPeer(scope, "puzzle2d-play", "pane.a", scope.peers.get("puzzle2d-play")?.get("pane.a") ?? null);
    unregisterBoard2dPeer(scope, "other-controller", "pane.a", scope.peers.get("other-controller")?.get("pane.a") ?? null);
  });

  it("peer gesture ownership: begin/end tracks the owning surfaceId; a pane never defers against its own gesture", () => {
    const scope = flowSessionLoader.createBoardPeerScope();
    registerBoard2dPeer(scope, "puzzle2d-play", "pane.a", { session: boardTestSession(), onPeerGestureEnded: () => {} });
    expect(boardPeerOwnsGesture(scope, "puzzle2d-play", "pane.a")).toBe(false);
    beginBoardPeerGesture(scope, "puzzle2d-play", "pane.a", scope.peers.get("puzzle2d-play")?.get("pane.a") ?? null);
    expect(boardPeerOwnsGesture(scope, "puzzle2d-play", "pane.a")).toBe(false);
    expect(boardPeerOwnsGesture(scope, "puzzle2d-play", "pane.b")).toBe(true);
    endBoardPeerGesture(scope, "puzzle2d-play", "pane.a", scope.peers.get("puzzle2d-play")?.get("pane.a") ?? null);
    expect(boardPeerOwnsGesture(scope, "puzzle2d-play", "pane.b")).toBe(false);
  });

  it("pushes live mirror mutations into peer sessions, skipping the source pane", () => {
    const scope = flowSessionLoader.createBoardPeerScope();
    const calls: { pane: string; method: string; arg: string }[] = [];
    const makePeer = (pane: string) => ({
      session: {
        setNodePositionsJson: (json: string) => calls.push({ pane, method: "setNodePositionsJson", arg: json }),
        setSelectionIdsJsonSilent: (json: string) => calls.push({ pane, method: "setSelectionIdsJsonSilent", arg: json }),
        setPreselectStateJsonSilent: (json: string) => calls.push({ pane, method: "setPreselectStateJsonSilent", arg: json }),
      } as never,
      onPeerGestureEnded: () => {},
    });
    registerBoard2dPeer(scope, "mirror-test", "pane.source", makePeer("pane.source"));
    registerBoard2dPeer(scope, "mirror-test", "pane.sibling", makePeer("pane.sibling"));

    pushBoardLiveMirrorMutations(scope, "mirror-test", "pane.source", {
      positions: [{ id: "alpha", x: 1, y: 2 }],
      selectionIds: ["alpha"],
      preselect: null,
      clearPreselect: false,
    });

    expect(calls).toEqual([
      { pane: "pane.sibling", method: "setNodePositionsJson", arg: JSON.stringify([{ id: "alpha", x: 1, y: 2 }]) },
      { pane: "pane.sibling", method: "setSelectionIdsJsonSilent", arg: JSON.stringify(["alpha"]) },
    ]);

    unregisterBoard2dPeer(scope, "mirror-test", "pane.source", scope.peers.get("mirror-test")?.get("pane.source") ?? null);
    unregisterBoard2dPeer(scope, "mirror-test", "pane.sibling", scope.peers.get("mirror-test")?.get("pane.sibling") ?? null);
  });

  it("notifies peers when a gesture ends, skipping the source pane, passing whether it flushed", () => {
    const scope = flowSessionLoader.createBoardPeerScope();
    const ended: { pane: string; flushed: boolean }[] = [];
    registerBoard2dPeer(scope, "notify-test", "pane.source", { session: {} as never, onPeerGestureEnded: (flushed) => ended.push({ pane: "pane.source", flushed }) });
    registerBoard2dPeer(scope, "notify-test", "pane.sibling", { session: {} as never, onPeerGestureEnded: (flushed) => ended.push({ pane: "pane.sibling", flushed }) });

    notifyBoardPeersGestureEnded(scope, "notify-test", "pane.source", true);
    expect(ended).toEqual([{ pane: "pane.sibling", flushed: true }]);

    unregisterBoard2dPeer(scope, "notify-test", "pane.source", scope.peers.get("notify-test")?.get("pane.source") ?? null);
    unregisterBoard2dPeer(scope, "notify-test", "pane.sibling", scope.peers.get("notify-test")?.get("pane.sibling") ?? null);
  });
});
