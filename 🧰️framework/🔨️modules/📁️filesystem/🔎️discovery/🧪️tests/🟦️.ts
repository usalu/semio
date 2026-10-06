/** 🧫️ Checks the shared graph corpus against independent graphlib reachability. */
import { expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { Graph, alg } from "graphlib";
import Ajv from "ajv/dist/2020.js";
import { discover } from "../🟦️.ts";

const fixture = JSON.parse(readFileSync(new URL("../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));

test("cycle-safe discovery preserves closed physical membership and cancellation", () => {
  for (const row of fixture.cases) {
    const actual = discover(row.root, key => row.nodes[key].identity, key => row.nodes[key].entries, key => key.endsWith(".grammar.semio"), count => row.cancelAfter === null || count < row.cancelAfter);
    expect({ ...actual, files: [...actual.files].sort() }).toEqual(row.expected);
    const graph = new Graph({ directed: true }), byIdentity = new Map<string, any>();
    for (const node of Object.values(row.nodes) as any[]) {
      graph.setNode(node.identity);
      byIdentity.set(node.identity, node);
    }
    for (const node of Object.values(row.nodes) as any[]) for (const entry of node.entries) if (entry.directory) graph.setEdge(node.identity, row.nodes[entry.value].identity);
    const reachable = alg.preorder(graph, row.nodes[row.root].identity).slice(0, row.cancelAfter ?? Infinity);
    const files = new Set(reachable.flatMap(key => byIdentity.get(key).entries.filter(entry => !entry.directory && entry.value.endsWith(".grammar.semio")).map(entry => entry.value)));
    expect({ files: [...files].sort(), directories: reachable.length, cancelled: row.cancelAfter !== null && reachable.length < alg.preorder(graph, row.nodes[row.root].identity).length }).toEqual(row.expected);
  }
});
