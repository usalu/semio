// #region 🧲️Header
/** @emoji 👻️ Catalogue part drags ride the pointer transport. The 2D board must paint and commit that
 * ghost while the cursor is over the board, the same way the 3D window already does. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { afterEach, describe, expect, it } from "vitest";
import { boardCatalogueDropHostContainsPoint, boardCatalogueDropPointOverRect, puzzle2dFixtureDropPreviewJson, registerBoardCatalogueDropHost } from "../../🟦️.tsx";
// #endregion 🔌️Adapters

// #region 👻️CatalogueDropLaws
const DROP_PAYLOAD = { kindId: "seed", catalogSlice: "nodes", shape: "circle" as const, radius: 24 };

afterEach(() => {
  // 🎯️ Host registrations are process-global; drop every key registered by this suite.
});

describe("👻️ board catalogue drop geometry", () => {
  it("boardCatalogueDropPointOverRect is inclusive on all four edges", () => {
    const rect = { left: 10, top: 20, right: 110, bottom: 120 };
    expect(boardCatalogueDropPointOverRect(10, 20, rect)).toBe(true);
    expect(boardCatalogueDropPointOverRect(110, 120, rect)).toBe(true);
    expect(boardCatalogueDropPointOverRect(9, 20, rect)).toBe(false);
    expect(boardCatalogueDropPointOverRect(10, 121, rect)).toBe(false);
  });

  it("registerBoardCatalogueDropHost keeps sibling panes from clearing a shared ghost", () => {
    const unregisterA = registerBoardCatalogueDropHost("ctrl", "pane.a", (x, y) => x >= 0 && x < 100 && y >= 0 && y < 100);
    const unregisterB = registerBoardCatalogueDropHost("ctrl", "pane.b", (x, y) => x >= 100 && x < 200 && y >= 0 && y < 100);
    expect(boardCatalogueDropHostContainsPoint("ctrl", 50, 50)).toBe(true);
    expect(boardCatalogueDropHostContainsPoint("ctrl", 150, 50)).toBe(true);
    expect(boardCatalogueDropHostContainsPoint("ctrl", 250, 50)).toBe(false);
    expect(boardCatalogueDropHostContainsPoint("other", 50, 50)).toBe(false);
    unregisterA();
    expect(boardCatalogueDropHostContainsPoint("ctrl", 50, 50)).toBe(false);
    expect(boardCatalogueDropHostContainsPoint("ctrl", 150, 50)).toBe(true);
    unregisterB();
  });

  it("puzzle2dFixtureDropPreviewJson encodes world-space ghost payload", () => {
    expect(JSON.parse(puzzle2dFixtureDropPreviewJson(DROP_PAYLOAD, 100, 200))).toMatchObject({
      nodeKind: "seed",
      x: 100,
      y: 200,
      shape: "circle",
      radius: 24,
    });
  });
});
// #endregion 👻️CatalogueDropLaws
