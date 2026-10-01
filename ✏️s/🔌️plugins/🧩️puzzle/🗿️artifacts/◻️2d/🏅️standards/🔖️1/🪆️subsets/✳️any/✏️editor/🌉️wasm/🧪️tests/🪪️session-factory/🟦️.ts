import { act as reactAct, createElement, type ReactElement } from "react";
import { cleanup, fireEvent, render, waitFor } from "@semio-tech/ui-react/test";
import { beforeEach, afterEach, describe, expect, it, vi } from "vitest";
import { boardTestSession } from "../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🔮️oracles/🪪️session-double/🟦️.ts";
import { semioSchemaAjvV1 } from "../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🧪️tests/🧬️schema-oracle/🟦️.ts";
import * as flowSessionLoader from "../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🪪️WasmSessionLoader/🟦️.tsx";
import { Board2dHost, beginPuzzle2dPeerGesture, endPuzzle2dPeerGesture, unregisterBoard2dPeer, puzzle2dPeerOwnsGesture } from "../../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/⚛️react/📦️packages/🟦️typescript/🟦️.tsx";
import boardSessionFixture from "../../🧫️fixtures/🔣️session-factory.json";
import boardSessionSchema from "../../../../🧬️schema/🔣️.json";

const noopAction = () => {};
beforeEach(() => { vi.stubGlobal("ResizeObserver", class { observe() {} unobserve() {} disconnect() {} }); });
afterEach(() => { cleanup(); vi.restoreAllMocks(); vi.unstubAllGlobals(); });

//#region 🧩️AppOwnedSurfaceSession

function boardTestHost(factory: flowSessionLoader.ScopedBoardSessionFactory, surfaceId: string): ReactElement {
  return createElement(
    flowSessionLoader.BoardSessionFactoryContext.Provider,
    { value: factory },
    createElement(Board2dHost, {
      node: {
        type: "componentScene",
        surfaceId,
        controllerId: boardSessionFixture.isolation.controllerId,
        componentKind: "board-2d",
        board2d: {
          fixtureJson: '{"nodes":[],"edges":[]}',
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
      onAction: vi.fn(),
    }),
  );
}

describe("app-owned surface session factories", () => {
  it.each(boardSessionFixture.retryFailures)("deduplicates module loads and retries an exact failed $0 attempt", async (failure) => {
    const first = Promise.withResolvers<object>();
    const second = Promise.withResolvers<object>();
    const load = vi.fn().mockReturnValueOnce(first.promise).mockReturnValueOnce(second.promise);
    const cached = flowSessionLoader.createWasmModuleLoader<object>(load);
    const a = cached();
    const b = cached();
    expect(a).toBe(b);
    const failed = Promise.allSettled([a, b]);
    first.reject(new Error(failure));
    expect((await failed).map((result) => result.status)).toEqual(["rejected", "rejected"]);
    const retry = cached();
    expect(retry).not.toBe(a);
    expect(cached()).toBe(retry);
    const module = {};
    second.resolve(module);
    expect(await retry).toBe(module);
    expect(await cached()).toBe(module);
    expect(load).toHaveBeenCalledTimes(2);
  });

  it("keeps identical Board controller/surface keys isolated across mounted shell scopes", async () => {
    const a = flowSessionLoader.createBoardPeerScope();
    const b = flowSessionLoader.createBoardPeerScope();
    const sessions = [boardTestSession(), boardTestSession(), boardTestSession(), boardTestSession()];
    const createA = vi.fn().mockResolvedValueOnce(sessions[0]).mockResolvedValueOnce(sessions[1]);
    const createB = vi.fn().mockResolvedValueOnce(sessions[2]).mockResolvedValueOnce(sessions[3]);
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
    const content = (scope: flowSessionLoader.BoardPeerScope, create: () => Promise<flowSessionLoader.Board2dWasmSession>, id: string) => boardTestHost({ pluginId: "puzzle", appId: "s.puzzle2d@1/*#editor", instanceId: 1, scope, create }, id);
    const views = [render(content(a, createA, "pane.a")), render(content(a, createA, "pane.b")), render(content(b, createB, "pane.a")), render(content(b, createB, "pane.b"))];
    try {
      await waitFor(() => sessions.forEach((session) => expect(session.attach_canvas).toHaveBeenCalledOnce()));
      await waitFor(() => sessions.forEach((session) => expect(session.setSelectionIdsJsonSilent).toHaveBeenCalledWith("[]")));
      sessions.forEach((session) => vi.mocked(session.setSelectionIdsJsonSilent!).mockClear());
      vi.mocked(sessions[0]!.drainEventsJson).mockReturnValueOnce(JSON.stringify([{ name: "select", payload: { ids: boardSessionFixture.isolation.selection } }]));
      const canvas = views[0]!.container.querySelector("canvas")!;
      fireEvent.pointerMove(canvas, { clientX: 2, clientY: 3 });
      expect(sessions[1]!.setSelectionIdsJsonSilent).toHaveBeenCalledWith(JSON.stringify(boardSessionFixture.isolation.selection));
      expect(sessions[2]!.setSelectionIdsJsonSilent).not.toHaveBeenCalled();
      expect(sessions[3]!.setSelectionIdsJsonSilent).not.toHaveBeenCalled();
      fireEvent.pointerDown(canvas, { clientX: 2, clientY: 3, button: 0 });
      expect(puzzle2dPeerOwnsGesture(a, "board", "pane.b")).toBe(true);
      expect(puzzle2dPeerOwnsGesture(b, "board", "pane.b")).toBe(false);
      views[0]!.unmount();
      expect(a.peers.get("board")?.has("pane.a")).toBe(false);
      expect(b.peers.get("board")?.has("pane.a")).toBe(true);
    } finally {
      views.forEach((view) => view.unmount());
      bounds.mockRestore();
    }
    sessions.forEach((session) => expect(session.free).toHaveBeenCalledOnce());
    expect(a.peers.size + a.gestures.size + b.peers.size + b.gestures.size).toBe(0);
  });

  it("keeps a remounted peer and gesture registered after the old attachment rejects", async () => {
    const scope = flowSessionLoader.createBoardPeerScope();
    const oldSession = boardTestSession();
    const newSession = boardTestSession();
    const pending = Promise.withResolvers<void>();
    vi.mocked(oldSession.attach_canvas).mockReturnValue(pending.promise);
    const factory = { pluginId: "puzzle", appId: "s.puzzle2d@1/*#editor", instanceId: 1, scope, create: vi.fn().mockResolvedValueOnce(oldSession).mockResolvedValueOnce(newSession) };
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
    const first = render(boardTestHost(factory, "pane.a"));
    await waitFor(() => expect(oldSession.attach_canvas).toHaveBeenCalledOnce());
    const oldPeer = scope.peers.get("board")!.get("pane.a")!;
    first.unmount();
    const next = render(boardTestHost(factory, "pane.a"));
    try {
      await waitFor(() => expect(newSession.attach_canvas).toHaveBeenCalledOnce());
      const successor = scope.peers.get("board")!.get("pane.a")!;
      beginPuzzle2dPeerGesture(scope, "board", "pane.a", successor);
      unregisterBoard2dPeer(scope, "board", "pane.a", oldPeer);
      endPuzzle2dPeerGesture(scope, "board", "pane.a", oldPeer);
      await reactAct(async () => {
        pending.reject(new Error("old attachment"));
        await pending.promise.catch(() => {});
      });
      expect(scope.peers.get("board")!.get("pane.a")).toBe(successor);
      expect(scope.gestures.get("board")?.peer).toBe(successor);
      expect(oldSession.free).toHaveBeenCalledOnce();
      expect(newSession.free).not.toHaveBeenCalled();
    } finally {
      next.unmount();
      bounds.mockRestore();
    }
    expect(newSession.free).toHaveBeenCalledOnce();
    expect(scope.peers.size + scope.gestures.size).toBe(0);
  });

  it.each(boardSessionFixture.cancellation)("retires the exact mounted Board session once after $phase cancellation", async (vector) => {
    const constructed = Promise.withResolvers<flowSessionLoader.Board2dWasmSession>();
    const attached = Promise.withResolvers<void>();
    const session: flowSessionLoader.Board2dWasmSession = {
      attach_canvas: vi.fn(() => attached.promise),
      setSize: vi.fn(),
      renderFrame: vi.fn(),
      parseFixtureJson: () => true,
      syncDescriptorJson: vi.fn(),
      setKindCatalogsJson: vi.fn(),
      setCamera: vi.fn(),
      setSelectionIdsJson: vi.fn(),
      setCanvasThemeJson: vi.fn(),
      pointerDownScreen: vi.fn(),
      pointerMoveScreen: vi.fn(),
      pointerUpScreen: vi.fn(),
      pointerCancelScreen: vi.fn(),
      wheelScreen: vi.fn(),
      drainEventsJson: () => "[]",
      cameraJson: () => '{"x":0,"y":0,"zoom":1}',
      gpuReady: () => true,
      free: vi.fn(),
    };
    const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
    const view = render(
      createElement(
        flowSessionLoader.BoardSessionFactoryContext.Provider,
        { value: { pluginId: "puzzle", appId: "s.puzzle2d@1/*#editor", instanceId: 1, create: () => constructed.promise, scope: flowSessionLoader.createBoardPeerScope() } },
        createElement(Board2dHost, {
          node: {
            type: "componentScene",
            surfaceId: "board.lifecycle",
            controllerId: "board",
            componentKind: "board-2d",
            board2d: {
              fixtureJson: '{"nodes":[],"edges":[]}',
              cameraJson: '{"x":0,"y":0,"zoom":1}',
              glyphCatalogsJson: "{}",
              selectionJson: "[]",
              interactive: false,
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
      ),
    );
    try {
      if (vector.phase !== "constructing") {
        await reactAct(async () => {
          constructed.resolve(session);
          await constructed.promise;
        });
        expect(session.attach_canvas).toHaveBeenCalledOnce();
      }
      if (vector.phase === "ready")
        await reactAct(async () => {
          attached.resolve();
          await attached.promise;
        });
      view.unmount();
      expect(session.free).toHaveBeenCalledTimes(vector.freeBeforeSettle);
      await reactAct(async () => {
        constructed.resolve(session);
        attached.resolve();
        await constructed.promise;
        await attached.promise;
      });
      expect(session.attach_canvas).toHaveBeenCalledTimes(vector.attachCalls);
      expect(session.free).toHaveBeenCalledTimes(vector.freeAfterSettle);
    } finally {
      view.unmount();
      constructed.resolve(session);
      attached.resolve();
      bounds.mockRestore();
    }
  });

  it("joins exact plugin and app ownership while keeping instance scopes distinct", () => {
    const validate = semioSchemaAjvV1({ strict: true, allErrors: true })
      .addFormat("double", true)
      .addFormat("int64", true)
      .addFormat("uint32", true)
      .addSchema(boardSessionSchema)
      .compile({ $ref: `${boardSessionSchema.$id}#/$defs/Puzzle2dWasmSessionFactory` });
    expect(validate(boardSessionFixture)).toBe(true);
    expect(validate({ ...boardSessionFixture, globalFactory: true })).toBe(false);
    const create = vi.fn(async (): Promise<flowSessionLoader.Board2dWasmSession> => {
      throw new Error("A lookup must not construct a session");
    });
    const registrations = boardSessionFixture.appIds.map((appId) => ({ kind: "board-2d" as const, pluginId: boardSessionFixture.pluginId, appId, create }));
    for (const scope of boardSessionFixture.scopes) {
      const resolved = flowSessionLoader.resolveAppSurfaceSessionFactory(registrations, scope);
      expect(resolved !== null).toBe(scope.matches);
      if (resolved) {
        expect(resolved.pluginId).toBe(scope.pluginId);
        expect(resolved.appId).toBe(scope.appId);
        expect(resolved.instanceId).toBe(scope.instanceId);
        expect(resolved.create).toBe(create);
      }
    }
    const scope = boardSessionFixture.scopes[0]!;
    const first = flowSessionLoader.resolveAppSurfaceSessionFactory(registrations, scope);
    const second = flowSessionLoader.resolveAppSurfaceSessionFactory(registrations, { ...scope, instanceId: 4294967295 });
    expect(first?.instanceId).not.toBe(second?.instanceId);
    expect(first).not.toBe(second);
    expect(() => flowSessionLoader.resolveAppSurfaceSessionFactory([...registrations, registrations[0]!], scope)).toThrow();
    expect(flowSessionLoader.resolveAppSurfaceSessionFactory(registrations, null)).toBeNull();
    expect(create).not.toHaveBeenCalled();
  });
});
//#endregion 🧩️AppOwnedSurfaceSession
