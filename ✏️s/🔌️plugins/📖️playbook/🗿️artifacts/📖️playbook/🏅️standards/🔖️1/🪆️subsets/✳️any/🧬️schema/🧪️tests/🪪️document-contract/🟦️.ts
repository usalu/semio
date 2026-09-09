/** 🧪️ Playbook document facets use shared child identities and exact native fields. */
import assert from "node:assert/strict";
import { readdirSync, readFileSync } from "node:fs";
import { join } from "node:path";
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧪️testkit/🪪️document-contract/🟦️.ts";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import * as artifact from "../../🟦️.ts";
import * as snapshot from "../../📸️snapshot/🟦️.ts";
import * as diff from "../../🔺️diff/🟦️.ts";
import vectors from "../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };

function snapshotFiles(root: string): string[] {
  return readdirSync(root, { withFileTypes: true }).flatMap((entry) => {
    const path = join(root, entry.name);
    return entry.isDirectory() ? snapshotFiles(path) : entry.name === "🔣️.json" && path.includes("/📸️snapshot/") ? [path] : [];
  });
}

function childKind(schema: Record<string, any>, field: "document" | "flow"): unknown {
  const property = schema.properties[field];
  return property["x-semio-child-kind"] ?? property.anyOf?.[0]?.["x-semio-child-kind"];
}

/** 🪪️ Checks native title nulls, exact child references and malformed document replacements. */
export function testPlaybookDocumentContractOracle(): void {
  const titled = { ...vectors.document, title: "Welcome" };
  const { title: _, ...missingTitle } = vectors.document;
  assertDocumentContractOracle({
    name: "Playbook",
    dependencies: [ioSchema, childSchema],
    artifact: { schema: artifactSchema, parse: artifact.parsePlaybookArtifact },
    snapshot: { schema: snapshotSchema, parse: snapshot.parsePlaybookSnapshot },
    diff: { schema: diffSchema, parse: diff.parsePlaybookDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }, { input: titled, output: titled }],
    invalidDocuments: [missingTitle, ...Object.entries(vectors.invalidDocumentFields).map(([key, value]) => ({ ...vectors.document, [key]: value })), ...vectors.invalidChildren.map((child) => ({ ...vectors.document, document: child }))],
    invalidDiffs: [{ artifact: { ...vectors.document, steps: [] } }, { document: { childId: "missing-target" } }, { steps: [] }],
    mutationRoots: [join(import.meta.dir, "../../../🧫️fixtures/🧬️mutations")],
    committed: { snapshots: 18, diffs: 5 },
  });
  for (const schema of [artifactSchema, snapshotSchema, diffSchema] as Array<Record<string, any>>) {
    assert.equal(childKind(schema, "document"), "s.stdio.semio");
    assert.equal(childKind(schema, "flow"), "s.stdio.semio");
  }
  const files = snapshotFiles(join(import.meta.dir, "../../../🧫️fixtures/🧬️mutations"));
  assert.equal(files.length, 18);
  for (const path of files) {
    const value = snapshot.parsePlaybookSnapshot(JSON.parse(readFileSync(path, "utf8")));
    for (const field of ["document", "flow"] as const) {
      assert.equal(value[field].target.dialect.artifactKind, "s.stdio.semio", `${path}:${field} kind`);
      assert.equal(value[field].target.artifactId, value[field].childId, `${path}:${field} identity`);
    }
  }
}
