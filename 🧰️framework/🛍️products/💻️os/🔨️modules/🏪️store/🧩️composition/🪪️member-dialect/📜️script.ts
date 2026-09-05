/** 🪪️ Independent closed-coordinate and persisted-envelope admission oracle. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import Ajv2020 from "ajv/dist/2020.js";
import { createRequire } from "node:module";
import { testInitialChildIdentityFixture } from "../🌱️initial/🪪️identity/📜️script.ts";

export function testMemberDialectFixture(): void {
  testInitialChildIdentityFixture();
  const read = (path: string) => readFileSync(new URL(path, import.meta.url), "utf8");
  const fixture = JSON.parse(read("./🧪️tests/🔣️.json"));
  const ajv = new Ajv2020({ strict: true, allErrors: true });
  const validate = ajv.compile(JSON.parse(read("./🧬️schema/🔣️.json")));
  assert(validate(fixture), JSON.stringify(validate.errors));
  const keys = new Set<string>();
  const coordinate = (value: { artifactKind: string; standard: string; subset: string }) => JSON.stringify([value.artifactKind, value.standard, value.subset]);
  const bindings = fixture.bindings.map((binding: { dialect: object; schema: string; variant: string }) => {
    const key = coordinate(binding.dialect as Parameters<typeof coordinate>[0]);
    assert(!keys.has(key), "factory coordinates are unique");
    keys.add(key);
    return { ...binding, matches: ajv.compile({ type: "object", const: binding.dialect }) };
  });
  const ids = new Set<string>();
  for (const row of fixture.publicRestoreCases) {
    assert(!ids.has(row.id), `duplicate case ${row.id}`);
    ids.add(row.id);
    const expected = { slot: "slot", childId: "child-1" };
    const parent = row.parentHasChild ? [expected] : [];
    const candidate = { slot: row.slot, childId: row.childId };
    const admitted = parent.some(ref => ref.slot === candidate.slot && ref.childId === candidate.childId);
    const oracle = ajv.compile({ type: "array", contains: { const: candidate }, minContains: 1, maxContains: 1 });
    assert.equal(oracle(parent), admitted, row.id);
    assert.equal(admitted, row.accepted, row.id);
  }
  for (const row of fixture.cases) {
    assert(!ids.has(row.id), `duplicate case ${row.id}`);
    ids.add(row.id);
    const direct = bindings.find((binding: { dialect: Parameters<typeof coordinate>[0] }) => coordinate(binding.dialect) === coordinate(row.requested));
    const independent = bindings.filter((binding: { matches: (value: unknown) => boolean }) => binding.matches(row.requested));
    assert.equal(independent.length, direct ? 1 : 0, row.id);
    const admitted = direct !== undefined && (row.operation === "create" || (row.persisted.schema === direct.schema && row.persisted.dialect !== null && coordinate(row.persisted.dialect) === coordinate(row.requested)));
    const oracle = independent.length === 1 && (row.operation === "create" || (independent[0].schema === row.persisted.schema && independent[0].matches(row.persisted.dialect)));
    assert.equal(admitted, oracle, row.id);
    assert.equal(admitted, row.accepted, row.id);
    assert.equal(admitted ? direct.variant : null, row.variant, row.id);
  }
  const store = read("../../🦀️.rs");
  for (const row of fixture.projectionCases) {
    assert(!ids.has(row.id), `duplicate case ${row.id}`);
    ids.add(row.id);
    const counts = new Map<string, number>();
    const children = new Set<string>();
    let projected = row.parent.length <= 64;
    for (const ref of row.parent) {
      const slot = fixture.projectionSlots.find((slot: { name: string }) => slot.name === ref.slot);
      const count = (counts.get(ref.slot) ?? 0) + 1;
      projected &&= !!slot && slot.kind === ref.artifactKind && (slot.many || count === 1) && ref.childId === ref.artifactId && !children.has(ref.childId)
        && Object.values(ref).every((value) => typeof value === "string" && new TextEncoder().encode(value).length > 0 && new TextEncoder().encode(value).length <= 256);
      counts.set(ref.slot, count);
      children.add(ref.childId);
    }
    const refSchema = { type: "object", required: ["slot", "childId", "artifactId", "artifactKind", "standard", "subset"], properties: Object.fromEntries(["slot", "childId", "artifactId", "artifactKind", "standard", "subset"].map((name) => [name, { type: "string", minLength: 1 }])), anyOf: fixture.projectionSlots.map((slot: { name: string; kind: string }) => ({ properties: { slot: { const: slot.name }, artifactKind: { const: slot.kind } } })) };
    const independent = ajv.compile({ type: "array", maxItems: 64, items: refSchema, allOf: fixture.projectionSlots.filter((slot: { many: boolean }) => !slot.many).map((slot: { name: string }) => ({ contains: { type: "object", required: ["slot"], properties: { slot: { const: slot.name } } }, minContains: 0, maxContains: 1 })) });
    const oracle = independent(row.parent) && ajv.compile({ type: "array", uniqueItems: true })(row.parent.map((ref: { childId: string }) => ref.childId))
      && row.parent.every((ref: { childId: string; artifactId: string }) => ajv.compile({ const: ref.childId })(ref.artifactId) && Object.values(ref).every((value) => Buffer.byteLength(value) <= 256));
    assert.equal(projected, oracle, row.id);
    assert.equal(projected, row.projected, row.id);
    const canonical = (refs: Array<{ slot: string; childId: string }>) => refs.toSorted((a, b) => JSON.stringify([a.slot, a.childId]).localeCompare(JSON.stringify([b.slot, b.childId])));
    const admitted = projected && row.incoming.length === row.parent.length && row.incoming.every((ref: object, index: number) => row.parent.some((expected: object) => Object.entries(expected).every(([key, value]) => ref[key] === value)) && !row.incoming.slice(0, index).some((prior: { childId: string }) => prior.childId === ref["childId"]));
    assert.equal(admitted, projected && ajv.compile({ const: canonical(row.parent) })(canonical(row.incoming)), row.id);
    assert.equal(admitted, row.accepted, row.id);
  }
  const derive = read("../../../../../../🔨️modules/🧬️schema/✨️derive/🦀️.rs");
  assert(derive.includes("ChildFieldRefs>::MANY") && derive.includes("visit_child_refs"), "schema derive projects marked real fields, including aliases, through typed child cardinality");
  assert(store.includes("pub struct ChildRestoreProjection") && store.includes("pub fn admit_complete"), "restore needs a bounded exact loaded-parent reference projection");
  const owner = { parentId: "parent-1", parentDialect: "s.test.parent@v1/*", slot: "content", childId: "child-1" };
  for (const row of fixture.identityCases) {
    assert(!ids.has(row.id), `duplicate case ${row.id}`);
    ids.add(row.id);
    const expectedOwner = row.expectedOwned ? owner : null;
    const ownMatch = row.persistedOwner === null ? expectedOwner === null : expectedOwner !== null && Object.entries(expectedOwner).every(([key, value]) => row.persistedOwner[key] === value);
    const admitted = row.persistedId === "child-1" && ownMatch;
    const oracle = ajv.compile({ const: { persistedId: "child-1", persistedOwner: expectedOwner } });
    assert.equal(oracle({ persistedId: row.persistedId, persistedOwner: row.persistedOwner }), admitted, row.id);
    assert.equal(admitted, row.accepted, row.id);
  }
  const contract = store.slice(store.indexOf("pub trait MemberFactory:"), store.indexOf("pub enum NoMembers"));
  const graphlib = createRequire(import.meta.url)("graphlib");
  for (const row of fixture.graphCases) {
    assert(!ids.has(row.id), `duplicate case ${row.id}`);
    ids.add(row.id);
    const edges = new Map<string, { parent: string; slot: string; child: string }>(row.edges.map((edge: { child: string }) => [edge.child, edge]));
    const current = edges.get(row.candidate.child);
    let cursor = row.candidate.parent;
    const seen = new Set<string>([row.candidate.child]);
    while (!seen.has(cursor) && edges.has(cursor)) {
      seen.add(cursor);
      cursor = edges.get(cursor)!.parent;
    }
    const accepted = !seen.has(cursor) && (!current || (current.parent === row.candidate.parent && current.slot === row.candidate.slot));
    const graph = new graphlib.Graph({ directed: true });
    for (const edge of row.edges) graph.setEdge(edge.parent, edge.child);
    graph.setEdge(row.candidate.parent, row.candidate.child);
    const exact = current === undefined || ajv.compile({ const: current })(row.candidate);
    assert.equal(accepted, exact && graphlib.alg.isAcyclic(graph), row.id);
    assert.equal(accepted, row.accepted, row.id);
    assert.equal(accepted && !current, row.inserted, row.id);
    const sync = new graphlib.Graph({ directed: true });
    for (const edge of row.edges) if (edge.parent !== row.candidate.parent) sync.setEdge(edge.parent, edge.child);
    sync.setEdge(row.candidate.parent, row.candidate.child);
    const syncAccepted = !seen.has(cursor) && (!current || current.parent === row.candidate.parent);
    assert.equal(syncAccepted, (!current || current.parent === row.candidate.parent) && graphlib.alg.isAcyclic(sync), `${row.id}: sync`);
    assert.equal(syncAccepted, row.syncAccepted, `${row.id}: sync`);
  }
  assert(store.includes("existing_owner != parent_id || existing_slot != slot"), "graph admission rejects same child under a different slot");
  const sync = store.slice(store.indexOf("pub async fn sync_member<P: ArtifactRefs>"), store.indexOf("/// 🧹 Releases at most one retained graph edge"));
  assert(sync.indexOf("let mut next_owns") >= 0 && sync.indexOf("self.owns.retain") > sync.lastIndexOf(".await"), "graph sync completes all fallible preparation before removing prior ownership");
  assert(!contract.includes("kind: &str"), "MemberFactory must receive a full dialect, never an arbitrary discriminator");
  assert(contract.includes("async fn open(expected: &crate::os_io::ArtifactRef, owner: Option<&OwnerRef>"), "member restore requires exact reference and ownership");
  assert(store.includes("if history.doc_id != expected.artifact_id || history.schema != schema || !dialect_matches || !owner_matches"), "persisted member identity, schema, dialect and ownership must match before typed hydration");
  const memberOpen = store.slice(store.indexOf("pub async fn open_member_store"), store.indexOf("fn validate_member_history_identity"));
  assert(memberOpen.indexOf("validate_member_history_identity(&history, schema, expected, owner)?") < memberOpen.indexOf("parse_decoded_document_spr::<P, Mutation>"), "identity preflight must precede typed snapshot and history allocation");
  console.log(`[DEBUG] member dialect source/AJV/Graphlib oracle: ${fixture.bindings.length} closed bindings, ${fixture.cases.length} admission, ${fixture.identityCases.length} identity/ownership, ${fixture.graphCases.length} graph, ${fixture.projectionCases.length} parent-reference and ${fixture.publicRestoreCases.length} public restore vectors; no native assertion claim`);
}
