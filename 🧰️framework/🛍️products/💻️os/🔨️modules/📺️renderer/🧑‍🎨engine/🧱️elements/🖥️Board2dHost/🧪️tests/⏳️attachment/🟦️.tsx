import { act, cleanup, render } from "@semio-tech/ui-react/test";
import { Component, createElement, type ReactNode } from "react";
import { afterEach, expect, it, vi } from "vitest";
import type { UiComponentSceneNode } from "@semio-tech/framework";
import type { Board2dWasmSession } from "@semio-tech/framework/canvas/react-renderer";
import { Board2dHost } from "../../🟦️.tsx";
import { BoardSessionFactoryContext, createBoardPeerScope } from "../../../🪪️WasmSessionLoader/🟦️.tsx";
import corpus from "./🧫️fixtures/🔣️.json";

afterEach(() => { cleanup(); vi.restoreAllMocks(); vi.unstubAllGlobals(); });

class AttachmentBoundary extends Component<{ children: ReactNode; errors: string[] }, { error: Error | null }> {
  state = { error: null as Error | null };
  static getDerivedStateFromError(error: Error) { return { error }; }
  componentDidCatch(error: Error) { this.props.errors.push(error.message); }
  render() { return this.state.error ? createElement("div", { role: "alert" }, this.state.error.message) : this.props.children; }
}

it.each(corpus.cases)("keeps the WASM session exclusively borrowed: $name", async row => {
  const calls: string[] = [], violations: string[] = [], errors: string[] = [], selectionOptions: unknown[][] = [];
  let complete!: () => void, rejectAttachment!: (error: Error) => void, borrowed = false, resized: (() => void) | undefined;
  const attachment = new Promise<void>((resolve, reject) => { complete = resolve; rejectAttachment = reject; });
  const session = new Proxy({
    attach_canvas: async () => { borrowed = true; calls.push("attach-start"); try { await attachment; calls.push("attach-complete"); } catch (error) { calls.push("attach-error"); throw error; } finally { borrowed = false; } },
    free: () => { if (borrowed) { violations.push("free"); throw Error("borrowed: free"); } calls.push("free"); if (row.freeFailure) throw Error("free refused"); },
    setSelectionOptions: (...args: unknown[]) => { selectionOptions.push(args); },
    renderFrame: () => { if (row.renderFailure) throw Error("render refused"); },
  }, { get(target, key) {
    if (key in target) return Reflect.get(target, key);
    if (key === "then" || typeof key !== "string") return undefined;
    return () => { if (borrowed) { violations.push(key); throw Error("borrowed: " + key); } return key === "loadBoardSnapshotJson" ? true : key === "drainEventsJson" ? "[]" : key.endsWith("Json") ? "{}" : undefined; };
  } }) as unknown as Board2dWasmSession;
  vi.stubGlobal("ResizeObserver", class { constructor(callback: () => void) { resized = callback; } observe() {} disconnect() {} });
  vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue({ x: 0, y: 0, width: 800, height: 600, top: 0, left: 0, right: 800, bottom: 600, toJSON: () => ({}) });
  const factory = { pluginId: "test", appId: "test", instanceId: 1, create: async () => session, scope: createBoardPeerScope() };
  const scene = (x: number) => ({ type: "componentScene", controllerId: "controller", surfaceId: "surface", componentKind: "board-2d", board2d: { snapshotJson: JSON.stringify({ nodes: [], edges: [], camera: { x, y: 0, zoom: 1 } }), cameraJson: JSON.stringify({ x, y: 0, zoom: 1 }), glyphCatalogsJson: "{}", selectionJson: "[]", interactive: true, selectionMethod: row.selection.method, selectableNodes: row.selection.nodes, selectableEdges: row.selection.edges, selectableHandles: row.selection.handles, gridSnapEnabled: false, gridFactor: 1, suggestionOffset: 0, brushWeightsJson: "{}", placementCompatibilityJson: "{}", lodMode: "automatic" } }) as UiComponentSceneNode;
  const board = (x: number) => createElement(AttachmentBoundary, { errors, children: createElement(BoardSessionFactoryContext.Provider, { value: factory }, createElement(Board2dHost, { node: scene(x), onAction: () => {} })) });
  const view = render(board(0));
  await act(async () => { await Promise.resolve(); });
  expect(calls).toEqual(["attach-start"]);
  if (row.update) { view.rerender(board(10)); act(() => resized?.()); }
  if (row.closeBefore) { view.unmount(); expect(calls).toEqual(["attach-start"]); }
  await act(async () => { if (row.settle === "reject") rejectAttachment(Error("attach refused")); else complete(); await attachment.catch(() => {}); });
  if (row.refreshAfter) await act(async () => { view.rerender(board(10)); await Promise.resolve(); });
  if (!row.closeBefore && row.settle === "resolve" && !row.renderFailure) { expect(view.container.querySelector(".semio-board-2d-host")?.getAttribute("data-board-snapshot-parsed")).toBe("true"); view.unmount(); }
  expect(errors).toEqual(row.errors);
  expect(selectionOptions).toEqual(row.expectedSelectionOptions);
  expect(violations).toEqual([]);
  expect(calls).toEqual(row.expected);
});
