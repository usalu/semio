import { applyPatch, compare } from "fast-json-patch";
import { applyDagDiff } from "../../🔺️diff/🟦️.ts";
import semioChildSchema from "../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🪆️child/🔣️.json" with { type: "json" };
/** 🧪️ DAG document facets use the native shared graph-child identity. */
import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/🪪️document-contract/🟦️.ts";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import { parseDagArtifact } from "../../🟦️.ts";
import { parseDagSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parseDagDiff } from "../../🔺️diff/🟦️.ts";
import vectors from "../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };

/** 🪪️ Checks every committed DAG snapshot plus independent embedded-graph and editor-state refusals. */
export function testDagDocumentContractOracle(): void {
  assertDocumentContractOracle({
    name: "DAG",
    dependencies: [ioSchema, childSchema, semioChildSchema],
    childIdentityFields: ["content"],
    artifact: { schema: artifactSchema, parse: parseDagArtifact },
    snapshot: { schema: snapshotSchema, parse: parseDagSnapshot },
    diff: { schema: diffSchema, parse: parseDagDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }],
    invalidDocuments: [...vectors.invalidDocuments, ...vectors.invalidIdentityDocuments],
    invalidDiffs: vectors.invalidDiffs,
    validDiffs: vectors.patchCases.map((item) => ({ input: item.diff, output: item.diff })),
    mutationRoots: [fileURLToPath(new URL("../../../🧫️fixtures/🧬️mutations", import.meta.url))],
    committed: { snapshots: 28, diffs: 0 },
  });
  assert.deepEqual(parseDagDiff(vectors.diff), vectors.diff);
  const child = parseDagArtifact(vectors.document).content;
  assert.equal(child.target.dialect.artifactKind, "s.stdio.semio");
  assert.equal(child.target.dialect.subset, "graph");
  for (const item of vectors.patchCases) {
    const base = parseDagArtifact(item.before);
    assert.deepEqual(applyDagDiff(base, item.diff), item.after, item.name);
    assert.deepEqual(applyPatch(structuredClone(item.before), compare(item.before, item.after)).newDocument, item.after, item.name);
    for (const field of ["content"] as const) if (!Object.hasOwn(item.diff, field)) assert.equal(applyDagDiff(base, item.diff)[field], base[field]);
  }
  console.log("[DEBUG] Dag sparse edit laws matched independent JSON Patch and retained untouched child identities");
}
