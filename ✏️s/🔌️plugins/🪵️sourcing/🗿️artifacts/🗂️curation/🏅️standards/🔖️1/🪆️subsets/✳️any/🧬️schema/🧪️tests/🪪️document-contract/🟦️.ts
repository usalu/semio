import { artifactReferenceSchema as artifactReferenceSchema, artifactChildSchema as childSchema } from "@semio-tech/framework/schema/artifact-reference";
import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { assertDocumentContractOracle, declaredChildDialectSchema } from "@semio-tech/framework/schema/oracles/document";
import { ioSchema as ioSchema } from "@semio-tech/framework/io/schema";
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import * as artifact from "../../🟦️.ts";
import * as snapshot from "../../📸️snapshot/🟦️.ts";
import * as diff from "../../🔺️diff/🟦️.ts";
import vectors from "../../../🧪️tests/🧫️fixtures/🔬️contract/🪪️document-contract/🔣️.json" with { type: "json" };

const semioChildSchema = declaredChildDialectSchema("https://json.schemas.assets.semio-tech.com/s/stdio/semio/v1/base/child.json", childSchema.$id, {kit: { artifactKind: "s.stdio.semio", standard: "v1", subset: "kit" }});
/** 🪪️ Curation owns catalog content references and rejects editor settings in documents. */
export function testCurationDocumentContractOracle(): void {
  for (const catalog of vectors.invalidChildren) assert.throws(() => artifact.parseCurationArtifact({ ...vectors.document, catalog }));
  assertDocumentContractOracle({
    name: "Curation", dependencies: [ioSchema, childSchema, semioChildSchema,artifactReferenceSchema],
    artifact: { schema: artifactSchema, parse: artifact.parseCurationArtifact },
    snapshot: { schema: snapshotSchema, parse: snapshot.parseCurationSnapshot },
    diff: { schema: diffSchema, parse: diff.parseCurationDiff },
    validDocuments: [vectors.document, ...vectors.validChildren.map((catalog) => ({ ...vectors.document, catalog })), ...vectors.geometryDocuments].map((input) => ({ input, output: input })),
    invalidDocuments: [...vectors.invalidChildren.map((catalog) => ({ ...vectors.document, catalog })), { ...vectors.document, filters: {} }, { ...vectors.document, curated: [{ objectId: "beam", count: -1 }] }],
    validDiffs: vectors.validDiffs.map((input) => ({ input, output: input })), invalidDiffs: vectors.invalidDiffs,
    mutationRoots: [fileURLToPath(new URL("../../../🧪️tests/🧫️fixtures/🧬️mutations", import.meta.url))], committed: { snapshots: 6, diffs: 3 },
  });
}
