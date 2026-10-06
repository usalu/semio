import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { resolveTestInputs, repoRootFromHere, TAXONOMY_REL_PATH, type DiscoveredCase } from "../../📦️packages/🟦️typescript/🟦️.ts";
import schema from "../../../../../../🔨️modules/🧪️test/🧬️schema/🔣️.json";
import vectors from "../../🧫️fixtures/🧭️fixture-resolution/🔣️.json";

describe("fixture and asset resolution", () => {
  for (const row of vectors.cases) test(row.id, () => {
    const root = realpathSync(mkdtempSync(join(tmpdir(), "semio-fixture-resolution-")));
    const owner = "domain";
    const discovered: DiscoveredCase = { owner, ownerName: owner, case: "🧪️case", caseDir: `${owner}/🧪️tests/🧪️case`, featurePath: `${owner}/🧪️tests/🧪️case/🥒️.feature`, adapters: {}, sharedFixtureDir: `${owner}/🧫️fixtures`, projectName: "fixture-resolution" };
    try {
      mkdirSync(dirname(join(root, TAXONOMY_REL_PATH)), { recursive: true });
      writeFileSync(join(root, TAXONOMY_REL_PATH), readFileSync(join(repoRootFromHere(), TAXONOMY_REL_PATH)));
      for (const [path, body] of Object.entries(vectors.files)) {
        mkdirSync(dirname(join(root, owner, path)), { recursive: true });
        writeFileSync(join(root, owner, path), body);
      }
      const result = resolveTestInputs(root, discovered, [row.uri]);
      expect(result.diagnostics).toEqual([]);
      if (row.path === null) {
        expect(result.missing).toEqual([row.uri]);
        expect(result.inputs).toEqual([]);
      } else {
        expect(result.missing).toEqual([]);
        expect(result.inputs[0]?.path).toBe(`${owner}/${row.path}`);
        const bytes = readFileSync(join(root, owner, row.path));
        expect(result.inputs[0]?.digest).toBe(createHash("sha256").update(bytes).digest("hex").slice(0, 32));
        const validate = new Ajv({ strict: false }).compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/TestInput" });
        expect(validate(result.inputs[0])).toBe(true);
      }
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
  test("the protocol rejects retired case-local fixture references", () => {
    const validate = new Ajv({ strict: false }).compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/TestInput" });
    expect(validate({ uri: "local://x", scope: "local", name: "x", path: "x", digest: "x" })).toBe(false);
  });
});
