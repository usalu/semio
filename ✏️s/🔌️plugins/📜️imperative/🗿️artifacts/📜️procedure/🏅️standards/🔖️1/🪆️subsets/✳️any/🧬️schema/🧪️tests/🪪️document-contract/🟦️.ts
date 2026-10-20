/** 🧪️ Procedure document facets compose the shared flow and text child identities. */
import { artifactReferenceSchema as artifactReferenceSchema, artifactChildSchema as childSchema } from "@semio-tech/framework/schema/artifact-reference";
import assert from "node:assert/strict";
import { assertDocumentContractOracle } from "@semio-tech/framework/schema/oracles/document";
import { ioSchema as ioSchema } from "@semio-tech/framework/io/schema";
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import { parseProcedureArtifact } from "../../🟦️.ts";
import { parseProcedureSnapshot } from "../../📸️snapshot/🟦️.ts";
import { parseProcedureDiff } from "../../🔺️diff/🟦️.ts";
import vectors from "../../../🧪️tests/🧫️fixtures/🔬️contract/🪪️document-contract/🔣️.json" with { type: "json" };

/** 🪆️ Compares first-party parsers with Ajv and every committed Procedure mutation document. */
export function testProcedureDocumentContractOracle(): void {
  assertDocumentContractOracle({
    name: "Procedure", dependencies: [ioSchema, childSchema,artifactReferenceSchema],
    artifact: { schema: artifactSchema, parse: parseProcedureArtifact },
    snapshot: { schema: snapshotSchema, parse: parseProcedureSnapshot },
    diff: { schema: diffSchema, parse: parseProcedureDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }],
    invalidDocuments: vectors.invalidDocuments, invalidDiffs: vectors.invalidDiffs,
    mutationRoots: [],
    committed: { snapshots: 0, diffs: 0 },
  });
  assert.deepEqual(parseProcedureDiff(vectors.diff), vectors.diff);
  assert.deepEqual(parseProcedureDiff({}), { schema: null, flow: null, text: null });
}
