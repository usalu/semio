/** 🧪️ Graphlib independently verifies the recursive replacement publication vectors. */
import assert from "node:assert/strict";
import { testOwnedChildEmissionRefusalOracle } from "./📨️emission/🟦️.ts";
import { testDeclaredChildProjectionOracle } from "./🔎️projection/🟦️.ts";
import { createRequire } from "node:module";
import vectors from "../../🧫️fixtures/🧩️composition/🔣️.json" with { type: "json" };
import archiveVectors from "../../../../🧫️fixtures/📡️channel/🗃️document-archive/🔣️.json" with { type: "json" };


const graphlib = createRequire(import.meta.url)("graphlib");

type Member = { artifactId: string; parentId: string; slot: string; children: string[] };
type Case = {
  id: string;
  mode: "complete" | "missing" | "extra" | "duplicate";
  admissionOrder: number[];
  members: Member[];
  cancelAt: null | "memberOpen" | "closure" | "viewPreparation";
  staleAuthority: boolean;
  published: boolean;
};

type HistoryDocument = {
  artifactId: string;
  edits: string[];
  changes: { id: string; edits: string[] }[];
  applied: string[];
  redo: string[];
};

type HistoryCase = {
  id: string;
  documents: HistoryDocument[];
  accepted: boolean;
};

type ArchiveMember = {
  ordinal: number;
  reference: { artifact_id: string };
  owner: { parent: { artifact_id: string }; slot: string; child_id: string };
};

type ArchiveMembership = { parent: string; slot: string; childId: string; target: string };

function archiveClosureAccepted(rootId: string, members: readonly ArchiveMember[], declared: readonly ArchiveMembership[]): boolean {
  if (declared.length !== members.length) return false;
  const graph = new graphlib.Graph({ directed: true });
  graph.setNode(rootId);
  const identities = new Set<string>();
  for (const [ordinal, member] of members.entries()) {
    if (member.ordinal !== ordinal || !declared.some(row => row.parent === member.owner.parent.artifact_id && row.slot === member.owner.slot && row.childId === member.owner.child_id && row.target === member.reference.artifact_id) || identities.has(member.reference.artifact_id)) return false;
    identities.add(member.reference.artifact_id);
    graph.setNode(member.reference.artifact_id);
  }
  for (const member of members) {
    if (!graph.hasNode(member.owner.parent.artifact_id)) return false;
    graph.setEdge(member.owner.parent.artifact_id, member.reference.artifact_id);
  }
  return graphlib.alg.isAcyclic(graph) && graphlib.alg.preorder(graph, rootId).length === members.length + 1;
}

function topologyAccepted(rootId: string, rootChildren: string[], members: Member[]): boolean {
  const graph = new graphlib.Graph({ directed: true });
  graph.setNode(rootId);
  const identities = new Set<string>();
  for (const member of members) {
    if (identities.has(member.artifactId)) return false;
    identities.add(member.artifactId);
    graph.setNode(member.artifactId);
  }
  for (const member of members) {
    if (!graph.hasNode(member.parentId)) return false;
    graph.setEdge(member.parentId, member.artifactId);
  }
  if (!graphlib.alg.isAcyclic(graph) || graphlib.alg.preorder(graph, rootId).length !== members.length + 1) return false;
  const declaredByParent = new Map<string, string[]>([[rootId, rootChildren]]);
  for (const member of members) declaredByParent.set(member.artifactId, member.children);
  for (const [parentId, declared] of declaredByParent) {
    const owned = members.filter((member) => member.parentId === parentId).map((member) => member.artifactId).sort();
    if (JSON.stringify([...declared].sort()) !== JSON.stringify(owned)) return false;
  }
  return true;
}

function historyAccepted(row: HistoryCase): boolean {
  const artifacts = new Set<string>();
  for (const document of row.documents) {
    if (artifacts.has(document.artifactId)) return false;
    artifacts.add(document.artifactId);
    const edits = new Set(document.edits);
    if (edits.size !== document.edits.length) return false;
    const changes = new Set<string>();
    for (const change of document.changes) {
      if (changes.has(change.id) || new Set(change.edits).size !== change.edits.length || change.edits.some((edit) => !edits.has(edit))) return false;
      changes.add(change.id);
    }
    if (
      new Set(document.applied).size !== document.applied.length
      || new Set(document.redo).size !== document.redo.length
      || document.applied.some((edit) => !edits.has(edit))
      || document.redo.some((edit) => !edits.has(edit) || document.applied.includes(edit))
    ) return false;
  }
  return row.documents.length > 0;
}

/** 🌳️ Publication is possible only for one complete, live, uncancelled decoded closure. */
export function testRecursiveOwnedDocumentReplacementOracle(): void {
  testDeclaredChildProjectionOracle();
  testOwnedChildEmissionRefusalOracle();
  const declared: ArchiveMembership[] = [{ parent: "root-1", slot: "children", childId: "child-1", target: "child-1" }, { parent: "child-1", slot: "nested", childId: "grandchild-1", target: "grandchild-1" }];
  assert(archiveClosureAccepted(archiveVectors.rootArtifactId, archiveVectors.archive.members, declared));
  const independent = structuredClone(archiveVectors.archive);
  independent.members[0]!.owner.child_id = "logical-child-1";
  assert(archiveClosureAccepted(archiveVectors.rootArtifactId, independent.members, [{ parent: "root-1", slot: "children", childId: "logical-child-1", target: "child-1" }, declared[1]!]));
  assert(!archiveClosureAccepted(archiveVectors.rootArtifactId, independent.members, declared));
  for (const row of vectors.historyCases as HistoryCase[]) assert.equal(historyAccepted(row), row.accepted, row.id);
  for (const row of archiveVectors.invalid) {
    const archive = structuredClone(archiveVectors.archive);
    const member = archive.members[row.mutation.member]!;
    if (row.mutation.field === "ordinal") member.ordinal = row.mutation.value as number;
    else if (row.mutation.field === "owner.child_id") member.owner.child_id = row.mutation.value as string;
    else member.owner.parent.artifact_id = row.mutation.value as string;
    assert(!archiveClosureAccepted(archiveVectors.rootArtifactId, archive.members, declared), row.id);
  }
  const ids = new Set<string>();
  for (const row of vectors.cases as Case[]) {
    assert(!ids.has(row.id), `duplicate case id ${row.id}`);
    ids.add(row.id);
    assert.deepEqual([...row.admissionOrder].sort((left, right) => left - right), Array.from({ length: row.members.length }, (_, index) => index), `${row.id} admission permutation`);
    const complete = topologyAccepted(vectors.root.artifactId, vectors.root.children, row.members);
    assert.equal(complete && row.cancelAt === null && !row.staleAuthority, row.published, row.id);
  }
}
