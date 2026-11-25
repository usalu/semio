// #region 🧲️Header
/** 👻️ Catalogue part drags ride the pointer transport. The 2D board must paint and commit that
 * ghost while the cursor is over the board, the same way the 3D window already does. */
// #endregion 🧲️Header

// #region 🔌️Adapters
import { describe, expect, it, vi } from "vitest";
import { Matrix3, Vector2 } from "three";
import { boardCatalogueDropHostContainsPoint, boardCatalogueDropPointOverRect, boardDropPreviewJson, boardScreenToWorld, registerBoardCatalogueDropHost } from "../../📍️drop-and-camera/🟦️.ts";
import { pushBoardDropPreview, registerBoard2dPeer } from "../../🔄️session-sync/🟦️.ts";
import { createBoardPeerScope, type Board2dPeer } from "../../../🪪️WasmSessionLoader/🟦️.tsx";
// #endregion 🔌️Adapters

// #region 👻️CatalogueDropLaws
const DROP_PAYLOAD = { kindId: "seed", catalogSlice: "nodes", shape: "circle" as const, radius: 24 };

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

  it("boardDropPreviewJson encodes world-space ghost payload", () => {
    expect(JSON.parse(boardDropPreviewJson(DROP_PAYLOAD, 100, 200))).toMatchObject({
      nodeKind: "seed",
      x: 100,
      y: 200,
      shape: "circle",
      radius: 24,
    });
  });

  it("maps through the inverse of the translated and zoomed viewport", () => {
    const camera = { x: 180, y: 40, zoom: 2 };
    const size = { w: 800, h: 600 };
    const origin = { x: 120, y: 60 };
    const client = { x: 260, y: 173 };
    const worldToClient = new Matrix3().set(2, 0, 160, 0, 2, 280, 0, 0, 1);
    const expected = new Vector2(client.x, client.y).applyMatrix3(worldToClient.invert());
    expect(boardScreenToWorld(JSON.stringify(camera), size, { x: client.x - origin.x, y: client.y - origin.y })).toEqual({ x: expected.x, y: expected.y });
  });

  it("updates each peer once per distinct ghost and lets its frame scheduler paint", () => {
    const scope = createBoardPeerScope();
    const session = { setDropPreviewJson: vi.fn(), clearDropPreview: vi.fn(), renderFrame: vi.fn() };
    const peer = { session: session as unknown as Board2dPeer["session"], onPeerGestureEnded: () => {} };
    registerBoard2dPeer(scope, "controller", "source", peer);
    const preview = boardDropPreviewJson(DROP_PAYLOAD, 100, 200);
    pushBoardDropPreview(scope, "controller", preview);
    pushBoardDropPreview(scope, "controller", preview);
    pushBoardDropPreview(scope, "controller", null);
    pushBoardDropPreview(scope, "controller", null);
    expect(session.setDropPreviewJson.mock.calls).toEqual([[preview]]);
    expect(session.clearDropPreview).toHaveBeenCalledTimes(1);
    expect(session.renderFrame).not.toHaveBeenCalled();
  });
});
// #endregion 👻️CatalogueDropLaws
