/** 🧪️ Procedure document facets compose the shared flow and text child identities. */
import assert from "node:assert/strict";
import { join } from "node:path";
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/🪪️document-contract/🟦️.ts";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import { parseProcedureArtifact } from "../../🟦️.ts";
import { parseProcedureSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parseProcedureDiff } from "../../🔺️diff/🟦️.ts";
import vectors from "../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };

/** 🪆️ Compares first-party parsers with Ajv and every committed Procedure mutation document. */
export function testProcedureDocumentContractOracle(): void {
  assertDocumentContractOracle({
    name: "Procedure", dependencies: [ioSchema, childSchema],
    artifact: { schema: artifactSchema, parse: parseProcedureArtifact },
    snapshot: { schema: snapshotSchema, parse: parseProcedureSnapshot },
    diff: { schema: diffSchema, parse: parseProcedureDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }],
    invalidDocuments: vectors.invalidDocuments, invalidDiffs: vectors.invalidDiffs,
    mutationRoots: [join(import.meta.dir, "../../../🧫️fixtures/🧬️mutations")],
    committed: { snapshots: 8, diffs: 2 },
  });
  assert.deepEqual(parseProcedureDiff(vectors.diff), vectors.diff);
  assert.deepEqual(parseProcedureDiff({}), { artifact: null, schema: null, flow: null, text: null });
}
