/** 🧪️ Playbook document facets use shared child identities and exact native fields. */
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
}
