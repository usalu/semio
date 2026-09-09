/** 🧪️ Reusable document-facet oracle over neutral vectors and committed mutation fixtures. */
import assert from "node:assert/strict";
import Ajv from "ajv";
import { readFileSync, readdirSync } from "node:fs";
import { join } from "node:path";

interface Facet {
  schema: object;
  parse: (value: unknown) => unknown;
}

export interface DocumentContractOracle {
  name: string;
  dependencies: readonly object[];
  artifact: Facet;
  snapshot: Facet;
  diff: Facet;
  validDocuments: readonly { input: unknown; output: unknown }[];
  invalidDocuments: readonly unknown[];
  invalidDiffs?: readonly unknown[];
  childIdentityFields?: readonly string[];
  mutationRoots: readonly string[];
  committed: { snapshots: number; diffs: number };
}

/** 🧬️ Validates production parsers independently with Ajv and the owner's committed native inputs. */
export function assertDocumentContractOracle(spec: DocumentContractOracle): void {
  const ajv = new Ajv({ strict: false, allErrors: true, validateFormats: false });
  for (const schema of spec.dependencies) ajv.addSchema(schema);
  ajv.addSchema(spec.artifact.schema);
  const exactIdentities = (input: unknown): boolean => (spec.childIdentityFields ?? []).every((field) => {
    const child = (input as Record<string, unknown>)?.[field] as { childId?: unknown; target?: { artifactId?: unknown } } | null | undefined;
    return child == null || ajv.validate({ type: "object", properties: { childId: { const: child.target?.artifactId } } }, child) === true;
  });
  for (const facet of [spec.artifact, spec.snapshot]) {
    const validate = ajv.compile(facet.schema);
    for (const { input, output } of spec.validDocuments) {
      assert.equal(validate(input) && exactIdentities(input), true, JSON.stringify(validate.errors));
      assert.deepEqual(facet.parse(input), output);
    }
    for (const input of spec.invalidDocuments) {
      assert.equal(validate(input) && exactIdentities(input), false, JSON.stringify(input));
      assert.throws(() => facet.parse(input));
    }
  }
  const validateDiff = ajv.compile(spec.diff.schema), validateSnapshot = ajv.compile(spec.snapshot.schema);
  for (const input of spec.invalidDiffs ?? []) {
    assert.equal(validateDiff(input) && exactIdentities(input), false);
    assert.throws(() => spec.diff.parse(input));
  }
  const paths = spec.mutationRoots.flatMap((root) => readdirSync(root, { recursive: true }).map((path) => ({ path: String(path).replaceAll("\\", "/"), file: join(root, String(path)) })));
  let snapshots = 0, diffs = 0;
  for (const { path, file } of paths) {
    const isSnapshot = path.endsWith("/📸️snapshot/⬅️before/🔣️.json") || path.endsWith("/📸️snapshot/➡️after/🔣️.json");
    const isDiff = path.endsWith("/🔺️diff/🔣️.json");
    if (!isSnapshot && !isDiff) continue;
    const input = JSON.parse(readFileSync(file, "utf8"));
    assert.equal((isSnapshot ? validateSnapshot : validateDiff)(input) && exactIdentities(input), true, path);
    assert.deepEqual((isSnapshot ? spec.snapshot : spec.diff).parse(input), input, path);
    if (isSnapshot) snapshots++; else diffs++;
  }
  assert.deepEqual({ snapshots, diffs }, spec.committed);
  console.log(`[DEBUG] ${spec.name} exact document contracts matched ${snapshots} native snapshots, ${diffs} committed diffs and independent owner/child rejection vectors`);
}
