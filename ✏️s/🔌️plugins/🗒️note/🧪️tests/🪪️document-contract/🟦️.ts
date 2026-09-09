/** 🧪️ Note document boundaries preserve native block payloads and shared link identity. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import { readdirSync, existsSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { assertDocumentContractOracle } from "../../../../../🧰️framework/🔨️modules/🧬️schema/🧪️testkit/🪪️document-contract/🟦️.ts";
import ioSchema from "../../../../../🧰️framework/🔨️modules/🚪️io/🧬️schema/🔣️.json" with { type: "json" };
import childSchema from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🪆️child/🧬️schema/🔣️.json" with { type: "json" };
import linkSchema from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️link/🧬️schema/🔣️.json" with { type: "json" };
import blobSchema from "../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️blob/🧬️schema/🔣️.json" with { type: "json" };
import vectors from "./../../🧫️fixtures/🪪️document-contract/🔣️.json" with { type: "json" };

/** 🗒️ Compares production parsers with Ajv and all committed native mutation records. */
export async function testNoteDocumentContractOracle(): Promise<void> {
  const root = join(dirname(fileURLToPath(import.meta.url)), "../../🗿️artifacts/🗒️note/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema");
  const facets = await Promise.all(["", "📸️snapshot", "🔺️diff"].map(async (facet) => ({ schema: (await import(pathToFileURL(join(root, facet, "🔣️.json")).href)).default, module: await import(pathToFileURL(join(root, facet, "🟦️.ts")).href) })));
  const dependencies = [ioSchema, childSchema, blobSchema, linkSchema];
  assertDocumentContractOracle({
    name: "Note",
    dependencies,
    artifact: { schema: facets[0].schema, parse: facets[0].module.parseNoteArtifact },
    snapshot: { schema: facets[1].schema, parse: facets[1].module.parseNoteSnapshot },
    diff: { schema: facets[2].schema, parse: facets[2].module.parseNoteDiff },
    validDocuments: [{ input: vectors.document, output: vectors.document }],
    invalidDocuments: [...Object.entries(vectors.invalidFields).map(([field, value]) => ({ ...vectors.document, [field]: value })), ...vectors.invalidBlocks.map((block) => ({ ...vectors.document, blocks: [block] }))],
    invalidDiffs: Object.entries(vectors.invalidFields).map(([field, value]) => ({ [field]: value })),
    mutationRoots: readdirSync(join(root, "../..")).map((subset) => join(root, "../..", subset, "🧫️fixtures/🧬️mutations")).filter(existsSync),
    committed: vectors.committed,
  });
  const ajv = new Ajv({ strict: false, validateFormats: false });
  for (const schema of [...dependencies, facets[0].schema]) ajv.addSchema(schema);
  const validate = ajv.compile(facets[2].schema);
  for (const diff of vectors.validDiffs) {
    assert.equal(validate(diff), true, JSON.stringify(validate.errors));
    assert.deepEqual(facets[2].module.parseNoteDiff(diff), diff);
  }
  console.log(`[DEBUG] Note shared link and identified block additions matched ${vectors.validDiffs.length} independent delta vectors`);
}
