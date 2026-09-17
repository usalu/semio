// #region 🧲️Header
/** @emoji 🧪️ Board2dHost pure-function laws: the probe's handle vitals and the transitive kind hover. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { describe, expect, it } from "vitest";
import { board2dKindDomainById, board2dKindHoverFromElementId } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

//#region 🖱️KindHover
const CATALOGS = JSON.stringify({
  nodeKinds: [{ id: "beam" }, { id: "capsule" }],
  handleKinds: [{ id: "b-l" }],
  edgeKinds: [{ id: "weld" }],
});

describe("board 2d kind hover", () => {
  /** 🗂️ The domain map is read off the board's OWN catalogs, so the host names no plugin and no panel. */
  it("reads every catalog slice into the engine's own hover domains", () => {
    const map = board2dKindDomainById(CATALOGS);
    expect([...map.entries()].sort()).toEqual([
      ["b-l", "handle"],
      ["beam", "node"],
      ["capsule", "node"],
      ["weld", "edge"],
    ]);
    expect(board2dKindDomainById("not json").size).toBe(0);
    expect(board2dKindDomainById("{}").size).toBe(0);
  });

  /** 🖱️ A catalogue row is `<sectionId>.<kindId>` in puzzle 2d and a bare kind id in puzzle 3d; both
   * resolve, and only against kinds this board actually carries — so unrelated chrome paints nothing. */
  it("resolves a catalogue row id to its kind, and refuses everything else", () => {
    const map = board2dKindDomainById(CATALOGS);
    expect(board2dKindHoverFromElementId("puzzle2d-play-kinds.nodes.beam", map)).toEqual({ domain: "node", kindId: "beam" });
    expect(board2dKindHoverFromElementId("puzzle2d-play-kinds.handles.b-l", map)).toEqual({ domain: "handle", kindId: "b-l" });
    expect(board2dKindHoverFromElementId("weld", map)).toEqual({ domain: "edge", kindId: "weld" });
    expect(board2dKindHoverFromElementId("puzzle2d-play-inspector.node.locked", map)).toBeNull();
    expect(board2dKindHoverFromElementId("", map)).toBeNull();
    expect(board2dKindHoverFromElementId(null, map)).toBeNull();
  });
});
//#endregion 🖱️KindHover
