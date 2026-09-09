/** 🧪️ DAG document facets use the native shared graph-child identity. */
import assert from "node:assert/strict";
import { join } from "node:path";
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🧪️testkit/🪪️document-contract/🟦️.ts";
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
    dependencies: [ioSchema, childSchema],
    artifact: { schema: artifactSchema, parse: parseDagArtifact },
    snapshot: { schema: snapshotSchema, parse: parseDagSnapshot },
    diff: { schema: diffSchema, parse: parseDagDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }],
    invalidDocuments: vectors.invalidDocuments,
    invalidDiffs: vectors.invalidDiffs,
    mutationRoots: [join(import.meta.dir, "../../../🧫️fixtures/🧬️mutations")],
    committed: { snapshots: 28, diffs: 0 },
  });
  assert.deepEqual(parseDagDiff(vectors.diff), vectors.diff);
  const child = parseDagArtifact(vectors.document).content;
  assert.equal(child.target.dialect.artifactKind, "s.stdio.semio");
  assert.equal(child.target.dialect.subset, "graph");
}
