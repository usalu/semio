import capacity from "../../../../../../🌊️flow/🕸️wasm/🧪️tests/🧫️fixtures/🎯️hover-group-capacity.json";
import { act, renderHook } from "@testing-library/react";
import { expect, it, vi } from "vitest";
import { useFlowPickAndSpotlight } from "../../🎯️flow-pick-spotlight/🟦️.ts";
import { createNodeGraphInteractionLedger } from "../../🫱️interaction-publication/🟦️.ts";
import fixture from "../🧫️fixtures/async-pointer.json";
import { nodeGraphHoverActionArgs } from "../../🧰️graph-core/🟦️.ts";
import { applyFlowSceneHover, cancelFlowTasks, setFlowSessionHover, syncFlowSessionStructureFromScene } from "../../🔄️flow-session-sync/🟦️.ts";

vi.mock("@semio-tech/ui-react", async (load) => ({
  ...(await load<typeof import("@semio-tech/ui-react")>()),
  useCanvasAppearanceSync: () => {},
}));

function deferredTask<T>() {
  let resolve!: (value: T) => void;
  const result = new Promise<T>(done => { resolve = done; });
  return { result, resolve, subscribe: () => () => {}, cancel: vi.fn() };
}

function newGraph(direction: "in" | "out" = "out", scoped = false) {
  const pick = deferredTask<string>();
  const write = deferredTask<void>();
  const immediate = (value?: string) => {
    const task = deferredTask<string | undefined>();
    task.resolve(value);
    return task;
  };
  const session = {
    pickTargetsAtScreenJson: vi.fn(() => pick),
    setHoverChannel: vi.fn(() => write),
    setHover: vi.fn(() => write),
    hoveredWidgetId: vi.fn(() => immediate(fixture.nativeHover.channel.widgetId)),
    hoveredChannelJson: vi.fn(() => immediate(JSON.stringify({ ...fixture.nativeHover, channel: { ...fixture.nativeHover.channel, direction } }))),
    clearGhostWidget: () => immediate(),
  };
  const dispatch = vi.fn();
  const interactionLedger = createNodeGraphInteractionLedger();
  const scene = {
    nodes: fixture.sceneNodes,
    interactionDomain: { id: "graph", ...(scoped ? fixture.semanticNamespace : { nodeTargetPrefix: "", handleTargetPrefix: "", edgeTargetPrefix: "" }) },
  };
  const core = {
    scene, appCatalogue: {}, sessionReady: false, sceneRef: { current: scene },
    sessionRef: { current: session }, hoverRequestRef: { current: 0 },
    containerRef: { current: { getBoundingClientRect: () => fixture.origin } },
    schedulerRef: { current: { invalidate: () => {} } }, interactionLedger, dispatch,
    setSpotlightSections: () => {}, paintOverlays: () => {}, renderFlow: () => {},
  } as unknown as Parameters<typeof useFlowPickAndSpotlight>[0];
  const hook = renderHook(() => useFlowPickAndSpotlight(core));
  return { hook, pick, write, session, dispatch, interactionLedger };
}

it("publishes the native port only after that pointer pick and hover write settle", async () => {
  const { hook, pick, write, session, dispatch } = newGraph();
  act(() => hook.result.current.pickInteraction.onCanvasPointerMove(fixture.client));
  expect(session.pickTargetsAtScreenJson).toHaveBeenCalledWith(...fixture.nativePoint);
  expect(session.setHover).not.toHaveBeenCalled();
  expect(session.setHoverChannel).not.toHaveBeenCalled();
  await act(async () => { pick.resolve(JSON.stringify(fixture.nativeTargets)); });
  expect(session.setHoverChannel).toHaveBeenCalledExactlyOnceWith(...fixture.nativeWrite);
  expect(session.hoveredWidgetId).not.toHaveBeenCalled();
  expect(session.hoveredChannelJson).not.toHaveBeenCalled();
  expect(dispatch).not.toHaveBeenCalled();
  await act(async () => { write.resolve(); });
  await vi.waitFor(() => expect(dispatch).toHaveBeenCalledTimes(1));
  const [action, args] = dispatch.mock.calls[0]!;
  expect(action).toBe("interactionHover");
  expect(JSON.parse(args.targets)).toEqual(fixture.published);
  hook.unmount();
});

it("discards a pending pointer reply when a new projected graph mark arrives", async () => {
  const { hook, pick, session, dispatch, interactionLedger } = newGraph();
  act(() => hook.result.current.pickInteraction.onCanvasPointerMove(fixture.client));
  interactionLedger.reconcileSceneHover(fixture.replacement);
  await act(async () => { pick.resolve(JSON.stringify(fixture.nativeTargets)); });
  expect(session.setHover).not.toHaveBeenCalled();
  expect(session.setHoverChannel).not.toHaveBeenCalled();
  expect(dispatch).not.toHaveBeenCalled();
  hook.unmount();
});

it("never delivers native hover writes that settle after replacement or session close", async () => {
  const first = deferredTask<void>();
  const second = deferredTask<void>();
  const last = deferredTask<void>();
  const tasks = [first, second, last];
  const session = {
    setHover: () => tasks.shift()!,
  } as unknown as Parameters<typeof setFlowSessionHover>[0];
  const publish = vi.fn();
  setFlowSessionHover(session, { hoveredId: "first" }, () => publish("first"));
  setFlowSessionHover(session, { hoveredId: "second" }, () => publish("second"));
  expect(first.cancel).toHaveBeenCalledOnce();
  first.resolve();
  second.resolve();
  await vi.waitFor(() => expect(publish.mock.calls).toEqual([["second"]]));
  setFlowSessionHover(session, { hoveredId: "last" }, () => publish("last"));
  cancelFlowTasks(session);
  expect(last.cancel).toHaveBeenCalledOnce();
  await act(async () => { last.resolve(); });
  expect(publish.mock.calls).toEqual([["second"]]);
});

it("silences a pending hover write as soon as a later pointer pick starts", async () => {
  const { hook, pick, write, session, dispatch } = newGraph();
  act(() => hook.result.current.pickInteraction.onCanvasPointerMove(fixture.client));
  await act(async () => { pick.resolve(JSON.stringify(fixture.nativeTargets)); });
  expect(session.setHoverChannel).toHaveBeenCalledOnce();

  const next = deferredTask<string>();
  session.pickTargetsAtScreenJson.mockImplementationOnce(() => next);
  act(() => hook.result.current.pickInteraction.onCanvasPointerMove({ ...fixture.client, x: fixture.client.x + 9 }));
  await act(async () => { write.resolve(); });
  expect(session.hoveredChannelJson).not.toHaveBeenCalled();
  expect(dispatch).not.toHaveBeenCalled();

  hook.unmount();
  expect(next.cancel).toHaveBeenCalledOnce();
});

it("retains the picked side when input and output have the same authored port", async () => {
  const { hook, pick, write, session, dispatch } = newGraph("in");
  act(() => hook.result.current.pickInteraction.onCanvasPointerMove(fixture.client));
  await act(async () => { pick.resolve(JSON.stringify([fixture.nativeTargets[0], fixture.inputTarget])); });
  expect(session.setHoverChannel).toHaveBeenCalledExactlyOnceWith(...fixture.inputWrite);
  await act(async () => { write.resolve(); });
  await vi.waitFor(() => expect(dispatch).toHaveBeenCalledTimes(1));
  expect(JSON.parse(dispatch.mock.calls[0]![1].targets)).toEqual(fixture.inputPublished);
  hook.unmount();
});

it("restores both typed handles while avoiding stale writes at gesture completion", () => {
  const setSelection = vi.fn(() => deferredTask<void>());
  const setSelectedChannels = vi.fn(() => deferredTask<void>());
  const session = { setSelection, setSelectedChannels } as unknown as Parameters<typeof syncFlowSessionStructureFromScene>[0];
  const scene = {
    nodes: fixture.sceneNodes,
    edges: [],
    selection: [...fixture.inputPublished, ...fixture.published].map(target => target.id),
    selectionGranularity: "handle" as const,
  };
  syncFlowSessionStructureFromScene(session, scene, {} as Parameters<typeof syncFlowSessionStructureFromScene>[2]);
  expect(setSelection).toHaveBeenCalledExactlyOnceWith(JSON.stringify({ nodes: [], edges: [], handles: [] }));
  expect(setSelectedChannels).toHaveBeenCalledExactlyOnceWith(JSON.stringify([fixture.inputTarget.channel, fixture.nativeHover.channel]));
  syncFlowSessionStructureFromScene(session, scene, {} as Parameters<typeof syncFlowSessionStructureFromScene>[2], true);
  expect(setSelection).toHaveBeenCalledTimes(1);
  expect(setSelectedChannels).toHaveBeenCalledTimes(1);
  cancelFlowTasks(session);
});

it("keeps a native handle distinct from a node and wire with the same text", async () => {
  const { hook, pick, write, dispatch } = newGraph("out", true);
  act(() => hook.result.current.pickInteraction.onCanvasPointerMove(fixture.client));
  await act(async () => { pick.resolve(JSON.stringify(fixture.nativeTargets)); });
  await act(async () => { write.resolve(); });
  await vi.waitFor(() => expect(dispatch).toHaveBeenCalledTimes(1));
  const domain = { id: "graph", ...fixture.semanticNamespace };
  const node = nodeGraphHoverActionArgs(domain, fixture.rawCollision);
  const edge = nodeGraphHoverActionArgs(domain, null, null, fixture.rawCollision);
  const marks = [node!.targets, edge!.targets, dispatch.mock.calls[0]![1].targets];
  expect(marks.map(json => JSON.parse(json)[0].id)).toEqual(fixture.scopedCollision);
  hook.unmount();
});

it("projects all authored companion ports in one native hover operation", () => {
  const channelTask = deferredTask<void>();
  const setHoverChannel = vi.fn(() => channelTask);
  const session = { setHoverChannel } as unknown as Parameters<typeof applyFlowSceneHover>[0];
  const scene = { nodes: fixture.sceneNodes, edges: [] };
  const [input, output] = [...fixture.inputPublished, ...fixture.published].map(target => target.id);
  applyFlowSceneHover(session, scene, { hoveredId: "shape", portId: "!imesh", portIds: [input!, output!] });
  expect(setHoverChannel).toHaveBeenCalledExactlyOnceWith({
    widgetId: "shape",
    port: "mesh",
    direction: "in",
    portsJson: JSON.stringify([fixture.inputTarget.channel, fixture.nativeHover.channel]),
  });
  cancelFlowTasks(session);
});

it("projects the complete renderer hover roster into one ABI call", () => {
  const setHoverChannel = vi.fn(() => deferredTask<void>());
  const session = { setHoverChannel } as unknown as Parameters<typeof applyFlowSceneHover>[0];
  const nodes = Array.from({ length: capacity.count }, (_, index) => {
    const head = capacity.widgetPrefix + String(index).padStart(4, "0") + "-" + capacity.escapeSample;
    const id = head.padEnd(capacity.widgetBytes, "q");
    const port = { id: id + (index % 2 === 0 ? "@!i" : "@") + capacity.port, code: capacity.port };
    return { id, ...(index % 2 === 0 ? {inputs: [port]} : {outputs: [port]}) };
  });
  const ids = nodes.map(node => (("inputs" in node ? node.inputs : node.outputs) as readonly { readonly id: string }[])[0]!.id);
  applyFlowSceneHover(session, { nodes, edges: [] } as unknown as Parameters<typeof applyFlowSceneHover>[1], {
    hoveredId: nodes[0].id, portId: "!i" + capacity.port, portIds: ids,
  });
  expect(setHoverChannel).toHaveBeenCalledTimes(1);
  const payload = (setHoverChannel.mock.calls as unknown as [[{portsJson: string}]])[0][0].portsJson;
  const size = new TextEncoder().encode(payload).length;
  expect(size).toBeGreaterThan(capacity.oldLimitBytes);
  expect(size).toBeLessThan(capacity.maximumBodyBytes);
  expect(JSON.parse(payload)).toEqual(nodes.map((node, index) => ({
    widgetId: node.id, port: capacity.port, direction: index % 2 === 0 ? "in" : "out",
  })));
  cancelFlowTasks(session);
});
