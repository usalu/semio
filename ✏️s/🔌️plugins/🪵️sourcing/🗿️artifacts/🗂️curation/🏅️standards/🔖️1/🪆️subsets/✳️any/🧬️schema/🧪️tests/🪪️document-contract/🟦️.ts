import assert from "node:assert/strict";
import { fileURLToPath } from "node:url";
import { assertDocumentContractOracle } from "../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/🔮️oracles/🪪️document-contract/🟦️.ts";
import ioSchema from "../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import semioChildSchema from "../../../../../../../../../../🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✉️base/🧬️schema/🪆️child/🔣️.json" with { type: "json" };
import artifactSchema from "../../🔣️.json" with { type: "json" };
import snapshotSchema from "../../📸️snapshot/🔣️.json" with { type: "json" };
import diffSchema from "../../🔺️diff/🔣️.json" with { type: "json" };
import * as artifact from "../../🟦️.ts";
import * as snapshot from "../../📸️snapshot/🟦️.ts";
import * as diff from "../../🔺️diff/🟦️.ts";
import vectors from "../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };

/** 🪪️ Curation owns catalog content references and rejects editor settings in documents. */
export function testCurationDocumentContractOracle(): void {
  for (const catalog of vectors.invalidChildren) assert.throws(() => artifact.parseCurationArtifact({ ...vectors.document, catalog }));
  assertDocumentContractOracle({
    name: "Curation", dependencies: [ioSchema, childSchema, semioChildSchema], childIdentityFields: ["catalog"],
    artifact: { schema: artifactSchema, parse: artifact.parseCurationArtifact },
    snapshot: { schema: snapshotSchema, parse: snapshot.parseCurationSnapshot },
    diff: { schema: diffSchema, parse: diff.parseCurationDiff },
    validDocuments: [vectors.document, ...vectors.geometryDocuments].map((input) => ({ input, output: input })),
    invalidDocuments: [...vectors.invalidChildren.map((catalog) => ({ ...vectors.document, catalog })), { ...vectors.document, filters: {} }, { ...vectors.document, curated: [{ objectId: "beam", count: -1 }] }],
    validDiffs: vectors.validDiffs.map((input) => ({ input, output: input })), invalidDiffs: vectors.invalidDiffs,
    mutationRoots: [fileURLToPath(new URL("../../../🧫️fixtures/🧬️mutations", import.meta.url))], committed: { snapshots: 6, diffs: 3 },
  });
}
