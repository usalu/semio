// #region 🧲️Header
// 2026 Ueli Saluz <ueli@semio-tech.com>
// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.
// #endregion 🧲️Header

/**
 * 🌐️ The SECOND ECOSYSTEM half of `s.puzzle.2d@1`'s third-party evidence.
 *
 * The sibling case `🕸️third-party-puzzle-2d-1` answers the same questions with `networkx`,
 * `shapely`, `jsonschema` and `jsonpatch` on CPython. This one answers the topology, payload-shape
 * and diff questions again, in bun, with three libraries from an unrelated ecosystem written by
 * unrelated authors: `graphology`, `jsonschema` (the npm package, not Python's) and
 * `fast-json-patch`. Two independent engine families agreeing is the point — a shared bug in one
 * graph library would otherwise look like agreement.
 *
 * `ajv` was the obvious npm schema validator and is DECLINED here, for a reason this repository
 * enforces mechanically: `ajv` is declared `production-runtime` by five packages in this tree, and
 * `verify dependencies literal-external` counts an oracle package that production can reach as an
 * `oracle-conflict`. An oracle must be test-only or it is comparing this repository with itself.
 * The npm `jsonschema` package carries the same draft-07 role with no production reachability.
 *
 * Geometry is deliberately NOT re-answered here: `shapely` sits on GEOS and its affine algebra is
 * strictly richer than anything this ecosystem offers for the same job, so a second, weaker
 * geometry check would add engine count without adding evidence.
 *
 * @see ../🕸️third-party-puzzle-2d-1/🐍️.py
 * @see ../../🔮️oracles/🔣️.json
 */

// #region 🔌️Adapters
import { readdirSync, readFileSync, existsSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import Graph from "graphology";
import { compare, applyPatch, deepClone, type Operation } from "fast-json-patch";
import { Validator } from "jsonschema";
import { defineTestAdapter, type AdapterContext, type AdapterOutcome } from "../../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";
// #endregion 🔌️Adapters

// #region 🧫️Vectors
/** 🧫️ The declared fixture. Its DIRECTORY is the mutation vocabulary; every vector is found beneath it. */
const VECTOR_ROOT_URI = "shared://🧬️mutations/🔣️.json";
const SCENARIOS_DIR = "🧪️tests";
const LEAF_SCHEMA = "🧬️schema/🔣️.json";
const MEMBERS = ["schema", "camera", "nodes", "edges", "meta"] as const;
/** 🕸️ The seven kinds whose correctness is topological — the only ones a graph library can speak to. */
const GRAPH_KINDS = ["create-node", "delete-node", "add-node-handle", "remove-node-handle", "replace-node-handle", "connect-handles", "disconnect-handles"];
const RELATION_KINDS = ["connect-kind-compatibility", "disconnect-kind-compatibility"];
const COMPATIBILITY_ATTRS = ["bidirectional", "important", "specificity"];

type Json = Record<string, any>;
type Vector = { id: string; kind: string; schema: Json | null; before: Json | null; after: Json | null; mutation: Json | null; diff: Json | null; outcome: Json | null };

/** 📜️ One committed JSON leaf, or `null` when the vector deliberately does not carry it. */
function readJson(...parts: string[]): Json | null {
  const path = join(...parts);
  return existsSync(path) ? (JSON.parse(readFileSync(path, "utf8")) as Json) : null;
}

/** 🏷️ The mutation kind a vocabulary directory names, with its identity emoji stripped. */
function kindOf(leaf: string): string {
  const at = [...leaf].findIndex((character) => /[a-z0-9]/.test(character) && character.charCodeAt(0) < 128);
  return at === -1 ? leaf : [...leaf].slice(at).join("");
}

/** 🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words. */
function tagOf(kind: string): string {
  const [head, ...rest] = kind.split("-");
  return head + rest.map((word) => word.slice(0, 1).toUpperCase() + word.slice(1)).join("");
}

/**
 * 🧫️ Every committed vector under the declared vocabulary root, in a stable order.
 *
 * Discovery rather than enumeration is deliberate: this artifact's scenario directories are being
 * authored and renamed continuously, and the sibling case's hand-written Examples table pointed at
 * four directories that no longer existed.
 */
/** 🗃️ Per-process vector cache: four scenarios read the same tree, and reading it once is enough. */
const DISCOVERED = new Map<string, Vector[]>();

function vectors(ctx: AdapterContext): Vector[] {
  const root = dirname(ctx.fixture(VECTOR_ROOT_URI));
  const cached = DISCOVERED.get(root);
  if (cached !== undefined) return cached;
  const found: Vector[] = [];
  for (const leaf of readdirSync(root).sort()) {
    const scenarios = join(root, leaf, SCENARIOS_DIR);
    if (!existsSync(scenarios) || !statSync(scenarios).isDirectory()) continue;
    const schema = readJson(root, leaf, LEAF_SCHEMA);
    for (const scenario of readdirSync(scenarios).sort()) {
      const directory = join(scenarios, scenario);
      if (!statSync(directory).isDirectory()) continue;
      found.push({
        id: `${leaf}/${scenario}`,
        kind: kindOf(leaf),
        schema,
        before: readJson(directory, "📸️snapshot", "⬅️before", "🔣️.json"),
        after: readJson(directory, "📸️snapshot", "➡️after", "🔣️.json"),
        mutation: readJson(directory, "🦠️mutation", "🔣️.json"),
        diff: readJson(directory, "🔺️diff", "🔣️.json"),
        outcome: readJson(directory, "🎯️outcome", "🔣️.json"),
      });
    }
  }
  if (found.length === 0) throw new Error(`no committed vector was discovered under ${root} — the mutation vocabulary cannot be empty`);
  DISCOVERED.set(root, found);
  return found;
}

/** 🦠️ The committed payload's arguments — the discriminator removed, what the handlers consume. */
function payloadOf(vector: Vector): Json {
  return Object.fromEntries(Object.entries(vector.mutation ?? {}).filter(([key]) => key !== "mutation"));
}

/** 🚦️ Whether the committed outcome itself records that the mutation had nothing to do. */
function declaresNoOp(vector: Vector): boolean {
  return ((vector.outcome?.messages ?? []) as Json[]).some((message) => message.code === "mutation.no-op");
}

/**
 * 🚦️ Whether the committed outcome says this vector moves the board at all.
 *
 * A vector is a rejection (`status` other than `applied`, with a `code` naming why) or an applied
 * no-op or an applied change; only the third may move the document.
 */
function movesDocument(vector: Vector): boolean {
  return vector.outcome?.status === "applied" && !declaresNoOp(vector);
}

/** 📤️ One scenario's answer: an ordered, per-vector record plus the count it actually checked. */
function report(scenario: string, rows: Json[], failures: string[]): AdapterOutcome {
  if (failures.length > 0) throw new Error(`${scenario}: ${failures.length} disagreement(s)\n${failures.join("\n")}`);
  return { projection: { scenario, checked: rows.reduce((total, row) => total + (row.checks as number), 0), vectors: rows } };
}
// #endregion 🧫️Vectors

// #region 🕸️Graph
/**
 * 🕸️ The board as a graphology multi-directed graph over NODE and HANDLE vertices.
 *
 * An `owns` edge (node → handle) carries a node's handle list as graph incidence instead of array
 * membership; a `wire` edge joins two HANDLE vertices, so an edge's endpoints really are ports owned
 * by a node. Cascade-on-delete is then `Graph.dropNode`'s own behaviour, not code written here.
 */
function boardGraph(document: Json): Graph {
  const graph = new Graph({ type: "directed", multi: true, allowSelfLoops: true });
  for (const node of document.nodes as Json[]) {
    graph.addNode(`node:${node.id}`, Object.fromEntries(Object.entries(node).filter(([key]) => key !== "handles")));
    for (const handle of node.handles as Json[]) {
      graph.addNode(`handle:${handle.id}`, { ...handle });
      graph.addDirectedEdgeWithKey(`owns:${handle.id}`, `node:${node.id}`, `handle:${handle.id}`, { rel: "owns" });
    }
  }
  for (const edge of document.edges as Json[]) graph.addDirectedEdgeWithKey(`wire:${edge.id}`, `handle:${edge.source}`, `handle:${edge.target}`, { rel: "wire", ...edge });
  return graph;
}

/**
 * 🧭️ graphology's OWN serialization, canonically ordered.
 *
 * `Graph.export` is the library's serializer; ordering it by key is the only thing added, so what is
 * compared is what graphology says the graph is rather than a structure this file rebuilt.
 */
function canonical(graph: Graph): string {
  const exported = graph.export() as { nodes: { key: string }[]; edges: { key: string }[] };
  const order = (left: { key: string }, right: { key: string }): number => (left.key < right.key ? -1 : left.key > right.key ? 1 : 0);
  return JSON.stringify({ nodes: [...exported.nodes].sort(order), edges: [...exported.edges].sort(order) });
}

/** 🔗 The handle vertices a node owns, read off graph incidence rather than the nested array. */
function ownedHandles(graph: Graph, nodeId: string): string[] {
  return graph.outEdges(`node:${nodeId}`).filter((edge) => graph.getEdgeAttribute(edge, "rel") === "owns").map((edge) => graph.target(edge));
}

/** 🧭️ Every handle vertex is owned by exactly one node, and no wire dangles. */
function ownershipInvariant(graph: Graph, where: string, failures: string[]): void {
  graph.forEachNode((vertex) => {
    if (!vertex.startsWith("handle:")) return;
    const owners = graph.inEdges(vertex).filter((edge) => graph.getEdgeAttribute(edge, "rel") === "owns");
    if (owners.length !== 1) failures.push(`${where}: handle vertex ${vertex} is owned by ${owners.length} nodes`);
  });
  graph.forEachEdge((edge, attributes, source, target) => {
    if (attributes.rel !== "wire") return;
    if (!source.startsWith("handle:") || !target.startsWith("handle:")) failures.push(`${where}: wire ${source} → ${target} does not join two handle vertices`);
  });
}

/**
 * 🕸️ Applies one topological kind USING THE LIBRARY'S OWN vertex and edge removal.
 *
 * `dropNode` severs a vertex's incident edges because that is what a graph is; the two cascades this
 * artifact is defined by are therefore asserted, never restated.
 */
function applyTopology(graph: Graph, vector: Vector): boolean {
  const payload = payloadOf(vector);
  switch (vector.kind) {
    case "create-node": {
      const node = payload.node as Json;
      graph.addNode(`node:${node.id}`, Object.fromEntries(Object.entries(node).filter(([key]) => key !== "handles")));
      for (const handle of node.handles as Json[]) {
        graph.addNode(`handle:${handle.id}`, { ...handle });
        graph.addDirectedEdgeWithKey(`owns:${handle.id}`, `node:${node.id}`, `handle:${handle.id}`, { rel: "owns" });
      }
      return true;
    }
    case "delete-node":
      for (const handle of ownedHandles(graph, payload.id as string)) graph.dropNode(handle);
      graph.dropNode(`node:${payload.id}`);
      return true;
    case "add-node-handle": {
      const handle = payload.handle as Json;
      graph.addNode(`handle:${handle.id}`, { ...handle });
      graph.addDirectedEdgeWithKey(`owns:${handle.id}`, `node:${payload.nodeId}`, `handle:${handle.id}`, { rel: "owns" });
      return true;
    }
    case "remove-node-handle":
      graph.dropNode(`handle:${payload.handleId}`);
      return true;
    case "replace-node-handle": {
      const old = `handle:${payload.handleId}`;
      if (!graph.hasNode(old)) return false;
      const handle = payload.newHandle as Json;
      const next = `handle:${handle.id}`;
      if (next === old) {
        graph.replaceNodeAttributes(old, { ...handle });
        return true;
      }
      // 🔁️graphology's core has no vertex relabel, so a re-identified port is expressed as the
      // wires it carried, a `dropNode`, and the same wires re-attached. The Python half, whose
      // library DOES have `relabel_nodes`, is what adjudicates that carry-over independently.
      const owner = graph.source(graph.inEdges(old).find((edge) => graph.getEdgeAttribute(edge, "rel") === "owns")!);
      const wires = graph.edges(old).filter((edge) => graph.getEdgeAttribute(edge, "rel") === "wire").map((edge) => ({ key: edge, source: graph.source(edge), target: graph.target(edge), attributes: graph.getEdgeAttributes(edge) }));
      graph.dropNode(old);
      graph.addNode(next, { ...handle });
      graph.addDirectedEdgeWithKey(`owns:${handle.id}`, owner, next, { rel: "owns" });
      for (const wire of wires) graph.addDirectedEdgeWithKey(wire.key, wire.source === old ? next : wire.source, wire.target === old ? next : wire.target, wire.attributes);
      return true;
    }
    case "connect-handles":
      // 🫥️A JSON carrier writes no member for an absent optional, so a null argument becomes an
      // absent attribute rather than a null one. That is a carrier fact, not a verb rule.
      graph.addDirectedEdgeWithKey(`wire:${payload.id}`, `handle:${payload.source}`, `handle:${payload.target}`, { rel: "wire", ...Object.fromEntries(Object.entries(payload).filter(([, value]) => value !== null)) });
      return true;
    case "disconnect-handles":
      if (!graph.hasEdge(`wire:${payload.id}`)) return false;
      graph.dropEdge(`wire:${payload.id}`);
      return true;
    default:
      return false;
  }
}

/** 🕸️ graphology answers the seven topological kinds, and the ownership invariant on every board. */
function graphCascade(ctx: AdapterContext): AdapterOutcome {
  const rows: Json[] = [];
  const failures: string[] = [];
  for (const vector of vectors(ctx)) {
    if (vector.before === null) {
      rows.push({ id: vector.id, kind: vector.kind, checks: 0, note: "no before-snapshot" });
      continue;
    }
    const before = boardGraph(vector.before);
    ownershipInvariant(before, `${vector.id} before`, failures);
    let checks = 1;
    let state = "invariant-only";
    if (GRAPH_KINDS.includes(vector.kind) && vector.after !== null) {
      const after = boardGraph(vector.after);
      ownershipInvariant(after, `${vector.id} after`, failures);
      checks += 2;
      if (!movesDocument(vector)) {
        state = `declared-${vector.outcome?.status ?? "unstated"}`;
        if (canonical(before) !== canonical(after)) failures.push(`${vector.id}: the committed outcome is ${JSON.stringify(vector.outcome)}, yet the board's graph moved`);
      } else if (applyTopology(before, vector)) {
        state = "topology-reproduced";
        if (canonical(before) !== canonical(after)) failures.push(`${vector.id}: the graph graphology computed from the before-snapshot does not equal the graph of the committed after-snapshot (${before.order}/${after.order} vertices, ${before.size}/${after.size} edges)`);
      } else failures.push(`${vector.id}: kind ${vector.kind} is topological and declared applied, and this oracle applied nothing`);
    }
    rows.push({ id: vector.id, kind: vector.kind, checks, state, handles: before.filterNodes((vertex) => vertex.startsWith("handle:")).length, wires: before.filterEdges((_edge, attributes) => attributes.rel === "wire").length });
  }
  return report("graph-cascade", rows, failures);
}

/** 🤝 The kind-compatibility relation as a simple directed graph over KIND LABELS. */
function relationGraph(document: Json): Graph {
  const graph = new Graph({ type: "directed", multi: false, allowSelfLoops: true });
  for (const record of ((document.meta.kindCompatibility ?? []) as Json[])) {
    graph.mergeNode(record.source as string);
    graph.mergeNode(record.target as string);
    // 🔑️Keyed by the ordered pair, so the comparison below is over the relation and never over
    // graphology's auto-generated edge identifiers.
    graph.mergeEdgeWithKey(`pair:${record.source}→${record.target}`, record.source as string, record.target as string, Object.fromEntries(COMPATIBILITY_ATTRS.filter((key) => key in record).map((key) => [key, record[key]])));
  }
  return graph;
}

/** 🤝 graphology answers the compatibility relation: uniqueness, and the two relation verbs. */
function kindCompatibility(ctx: AdapterContext): AdapterOutcome {
  const rows: Json[] = [];
  const failures: string[] = [];
  for (const vector of vectors(ctx)) {
    if (vector.before === null) continue;
    const before = relationGraph(vector.before);
    const records = (vector.before.meta.kindCompatibility ?? []) as Json[];
    let checks = 1;
    if (before.size !== records.length) failures.push(`${vector.id}: the before-snapshot declares ${records.length} compatibility records but only ${before.size} distinct ordered pairs`);
    if (vector.after !== null) {
      const after = relationGraph(vector.after);
      const payload = movesDocument(vector) ? payloadOf(vector) : {};
      checks += 1;
      if (vector.kind === "connect-kind-compatibility" && payload.source !== undefined) {
        before.mergeNode(payload.source as string);
        before.mergeNode(payload.target as string);
        before.mergeEdgeWithKey(`pair:${payload.source}→${payload.target}`, payload.source as string, payload.target as string, Object.fromEntries(COMPATIBILITY_ATTRS.filter((key) => key in payload).map((key) => [key, payload[key]])));
      } else if (vector.kind === "disconnect-kind-compatibility" && payload.source !== undefined) {
        if (before.hasEdge(payload.source as string, payload.target as string)) before.dropEdge(payload.source as string, payload.target as string);
        // 🏷️A kind label exists in this relation only while some pair still names it, so the vertex
        // set is derived from the edge set by graphology's own degree bookkeeping.
        for (const vertex of before.filterNodes((candidate) => before.degree(candidate) === 0)) before.dropNode(vertex);
      }
      if (canonical(before) !== canonical(after)) failures.push(`${vector.id}: the compatibility relation graphology computed does not equal the committed after-snapshot's relation`);
      else if (!RELATION_KINDS.includes(vector.kind) && vector.kind !== "replace-kind-catalogs" && canonical(relationGraph(vector.before)) !== canonical(after)) failures.push(`${vector.id}: kind ${vector.kind} moved the compatibility relation, which only ${RELATION_KINDS.join(" or ")} may do`);
    }
    rows.push({ id: vector.id, kind: vector.kind, checks, pairs: before.size });
  }
  return report("kind-compatibility", rows, failures);
}
// #endregion 🕸️Graph

// #region 🧬️Schema
/** 🧬️ The npm `jsonschema` validator answers every committed payload against its leaf draft-07 schema. */
function payloadSchemas(ctx: AdapterContext): AdapterOutcome {
  const rows: Json[] = [];
  const failures: string[] = [];
  const validator = new Validator();
  for (const vector of vectors(ctx)) {
    if (vector.mutation === null) continue;
    let checks = 1;
    if (vector.mutation.mutation !== tagOf(vector.kind)) failures.push(`${vector.id}: the committed payload is tagged ${JSON.stringify(vector.mutation.mutation)} where its leaf declares ${vector.kind}`);
    if (vector.schema === null) {
      failures.push(`${vector.id}: the mutation leaf carries no ${LEAF_SCHEMA}`);
      rows.push({ id: vector.id, kind: vector.kind, checks });
      continue;
    }
    checks += 1;
    for (const error of validator.validate(vector.mutation, vector.schema).errors) failures.push(`${vector.id}: jsonschema rejects the committed payload at ${error.property} — ${error.message}`);
    // 🧪️A validator that accepts everything would accept the payload too. The probe proves the
    // opposite by handing it a member the schema does not declare.
    if (vector.schema.additionalProperties === false) {
      checks += 1;
      if (validator.validate({ ...(vector.mutation as Record<string, Json>), semioThirdPartyOracleProbe: true }, vector.schema).valid) failures.push(`${vector.id}: the leaf schema declares additionalProperties false yet jsonschema accepted an undeclared member`);
    }
    rows.push({ id: vector.id, kind: vector.kind, checks, draft: vector.schema.$schema ?? "", title: vector.schema.title ?? "" });
  }
  return report("payload-schemas", rows, failures);
}
// #endregion 🧬️Schema

// #region 🔺️Diff
/** 🔺 The top-level snapshot members an RFC 6902 patch reaches, read off its op PATHS. */
function touchedMembers(patch: readonly Operation[]): string[] {
  return [...new Set(patch.map((operation) => operation.path.replace(/^\//, "").split("/")[0]!))].sort();
}

/** 🆔 The ids a snapshot collection carries, in board order. */
function collectionIds(document: Json, member: string): string[] {
  return (document[member] as Json[]).map((record) => record.id as string);
}

/**
 * 🔺 The ids fast-json-patch says CHANGED while surviving — the typed diff's `patched` set.
 *
 * Computed per RECORD rather than by reading indices off the whole-document patch: removing an entry
 * from the middle of a collection shifts every later index, and a whole-document patch then
 * expresses a removal as field edits on records that did not change at all.
 */
function patchedIds(before: Json, after: Json, member: string): Set<string> {
  const survivors = new Map((after[member] as Json[]).map((record) => [record.id as string, record]));
  const reached = new Set<string>();
  for (const record of before[member] as Json[]) {
    const twin = survivors.get(record.id as string);
    if (twin !== undefined && compare(record, twin).length > 0) reached.add(record.id as string);
  }
  return reached;
}

/** 🔺 fast-json-patch reproduces every committed after-snapshot and holds the typed diff to its ops. */
function diffReproduction(ctx: AdapterContext): AdapterOutcome {
  const rows: Json[] = [];
  const failures: string[] = [];
  const sorted = (values: Iterable<string>): string[] => [...values].sort();
  for (const vector of vectors(ctx)) {
    if (vector.before === null || vector.after === null) continue;
    const patch = compare(vector.before, vector.after);
    let checks = 1;
    // 🔁️Equality is asked of the library itself — an empty patch between the reproduction and the
    // committed after-snapshot — because `ordered-json-v1` holds array order significant and key
    // order never, which a string comparison would get wrong.
    if (compare(applyPatch(deepClone(vector.before), patch).newDocument, vector.after).length > 0) failures.push(`${vector.id}: applying the RFC 6902 patch fast-json-patch derived does not reproduce the committed after-snapshot`);
    checks += 1;
    if (!movesDocument(vector) && patch.length > 0) failures.push(`${vector.id}: the committed outcome is ${JSON.stringify(vector.outcome)} yet fast-json-patch derived ${patch.length} op(s)`);
    if (movesDocument(vector) && patch.length === 0) failures.push(`${vector.id}: the committed vector declares the kind applied yet fast-json-patch found no change`);
    if (vector.diff === null) {
      rows.push({ id: vector.id, kind: vector.kind, checks, ops: patch.length, note: "no committed diff" });
      continue;
    }
    const declared = sorted(MEMBERS.filter((member) => vector.diff![member] !== null && vector.diff![member] !== undefined));
    checks += 1;
    if (JSON.stringify(declared) !== JSON.stringify(touchedMembers(patch))) failures.push(`${vector.id}: the typed diff declares ${JSON.stringify(declared)}, the RFC 6902 patch touches ${JSON.stringify(touchedMembers(patch))}`);
    for (const member of ["nodes", "edges"]) {
      const delta = vector.diff[member] as Json | null | undefined;
      if (delta === null || delta === undefined) continue;
      const beforeIds = new Set(collectionIds(vector.before, member));
      const afterIds = new Set(collectionIds(vector.after, member));
      checks += 3;
      const removed = sorted([...beforeIds].filter((id) => !afterIds.has(id)));
      const added = sorted([...afterIds].filter((id) => !beforeIds.has(id)));
      if (JSON.stringify(sorted((delta.removed ?? []) as string[])) !== JSON.stringify(removed)) failures.push(`${vector.id}: the typed diff removes ${JSON.stringify(sorted((delta.removed ?? []) as string[]))} from ${member}, the two snapshots differ by ${JSON.stringify(removed)}`);
      if (JSON.stringify(sorted(((delta.added ?? []) as Json[]).map((record) => record.id as string))) !== JSON.stringify(added)) failures.push(`${vector.id}: the typed diff adds ${JSON.stringify(sorted(((delta.added ?? []) as Json[]).map((record) => record.id as string)))} to ${member}, the two snapshots differ by ${JSON.stringify(added)}`);
      const reached = sorted(patchedIds(vector.before, vector.after, member));
      if (JSON.stringify(sorted(((delta.patched ?? []) as Json[]).map((entry) => entry.id as string))) !== JSON.stringify(reached)) failures.push(`${vector.id}: the typed diff patches ${JSON.stringify(sorted(((delta.patched ?? []) as Json[]).map((entry) => entry.id as string)))} in ${member}, fast-json-patch needs operations for ${JSON.stringify(reached)}`);
      for (const entry of (delta.patched ?? []) as Json[]) {
        const replacement = entry.patch?.replacement;
        if (replacement === undefined || replacement === null) continue;
        checks += 1;
        if (JSON.stringify(replacement) !== JSON.stringify((vector.after[member] as Json[]).find((record) => record.id === entry.id))) failures.push(`${vector.id}: the typed diff's replacement for ${member} ${JSON.stringify(entry.id)} does not equal the committed after-snapshot's record`);
      }
    }
    rows.push({ id: vector.id, kind: vector.kind, checks, ops: patch.length, members: declared });
  }
  return report("diff-reproduction", rows, failures);
}
// #endregion 🔺️Diff

// #region 🧭️Adapter
/**
 * 🧭️ Registration in the ORACLE role only. These libraries are the reference; this repository's Rust
 * and its Python second implementation are the subjects, and neither is registered here.
 */
export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "graph-cascade": { oracle: graphCascade },
    "kind-compatibility": { oracle: kindCompatibility },
    "payload-schemas": { oracle: payloadSchemas },
    "diff-reproduction": { oracle: diffReproduction },
  },
});
// #endregion 🧭️Adapter
