import { expect, test } from "bun:test";
import Ajv from "ajv";
import fixture from "../../🧫️fixtures/🚚️node-graph-scene-lanes/🔣️.json";
import schema from "../../🧬️schema/🚚️node-graph-scene-lanes/🔣️.json";
import { NODE_GRAPH_SCENE_LANES, nodeGraphSceneFromLanes } from "../../🟦️.ts";

test("node graph lanes match their language-neutral schema", () => {
  expect(new Ajv().compile(schema)(fixture)).toBe(true);
  expect<unknown>(NODE_GRAPH_SCENE_LANES).toEqual(fixture.lanes);
});

test("oversized node graphs preserve typed records and Unicode JSON through lanes", () => {
  const nodes = Array.from({ length: fixture.nodeCount }, (_, index) => ({ ...fixture.node, id: String(index) }));
  const snapshot = JSON.stringify({ label: fixture.hostSnapshotLabel.repeat(fixture.repeatCount) });
  const spine = { nodes: [], edges: [], viewport: { x: 1, y: 2, zoom: 1 } };
  const restored = nodeGraphSceneFromLanes(spine, new Map([
    ["framework.scene.nodeGraph.nodes", JSON.stringify(nodes)],
    ["framework.scene.nodeGraph.hostSnapshot", snapshot],
  ]));
  expect(new TextEncoder().encode(JSON.stringify(nodes)).length).toBeGreaterThan(32768);
  expect(restored.nodes).toEqual(nodes);
  expect(restored.hostSnapshotJson).toBe(snapshot);
  expect(restored.viewport).toEqual(spine.viewport);
  expect(spine.nodes).toEqual([]);
  expect(nodeGraphSceneFromLanes(spine, new Map([["framework.scene.nodeGraph.nodes", "[truncated"]]))).toEqual(spine);
});

test("every node graph field restores from its declared carrier", () => {
  const spine: Record<string, unknown> = { ...fixture.sample };
  const texts = new Map<string, string>();
  for (const lane of fixture.lanes) {
    const value = spine[lane.field];
    texts.set(lane.bodyKey, "encoding" in lane ? JSON.stringify(value) : value as string);
    if (lane.optional) delete spine[lane.field];
    else spine[lane.field] = [];
  }
  expect(nodeGraphSceneFromLanes(spine as unknown as import("../../🟦️.ts").NodeGraphScene, texts)).toEqual(fixture.sample);
});
