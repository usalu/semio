/** 🟠️ The collects live mirror mutations tests of the framework renderer hosts suite. */
import { boardTestSession } from "../../../🧱️elements/🪪️WasmSessionLoader/🔮️oracles/🪪️session-double/🟦️.ts";
import * as flowSessionLoader from "../../../🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { expect, it } from "vitest";
import { beginBoardPeerGesture, collectBoardLiveMirrorMutations, endBoardPeerGesture, notifyBoardPeersGestureEnded, board2dPeers, boardPeerOwnsGesture, pushBoardLiveMirrorMutations, registerBoard2dPeer, unregisterBoard2dPeer } from "../../../🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";

/** 🟡 Registers the collects live mirror mutations tests of the framework renderer hosts suite. */
export function registerCollectsLiveMirrorMutationsTests(): void {
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
}
