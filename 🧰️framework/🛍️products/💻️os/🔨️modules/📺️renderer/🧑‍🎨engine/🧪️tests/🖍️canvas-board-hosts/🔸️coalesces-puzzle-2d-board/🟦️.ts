/** 🔸️ The coalesces puzzle 2d board tests of the framework renderer hosts suite. */
import { expect, it } from "vitest";
import { collectBoardLiveMirrorMutations, board2dGranularityById, board2dHoverActionArgs, latestBoard2dHoverId, parseBoard2dSuggestionMenu, board2dSuggestionMenuOwnsWindow, board2dSuggestionMenuItems, coalesceBoard2dEvents, parseBoard2dTransformFlags, board2dStatusJson } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🔹 Registers the coalesces puzzle 2d board tests of the framework renderer hosts suite. */
export function registerCoalescesPuzzle2dBoardTests(): void {
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
}
