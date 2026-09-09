import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { createHash } from "node:crypto";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import { dirname, join } from "node:path";
import { resolveFixtures, repoRootFromHere, TAXONOMY_REL_PATH, type DiscoveredCase } from "../../📦️packages/🟦️typescript/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import vectors from "../../🧫️fixtures/🧭️fixture-resolution/🔣️.json";

describe("fixture and asset resolution", () => {
  test("neutral cases conform to the owning schema", () => {
    const validate = new Ajv({ strict: true }).compile(schema.$defs.FixtureResolutionCases);
    expect(validate(vectors)).toBe(true);
    expect(validate.errors).toBeNull();
  });
  for (const row of vectors.cases) test(row.id, () => {
    const root = mkdtempSync(join(tmpdir(), "semio-fixture-resolution-"));
    const owner = "domain";
    const discovered: DiscoveredCase = { owner, ownerName: owner, case: "🧪️case", caseDir: `${owner}/🧪️tests/🧪️case`, featurePath: `${owner}/🧪️tests/🧪️case/🥒️.feature`, adapters: {}, sharedFixtureDir: `${owner}/🧫️fixtures`, projectName: "fixture-resolution" };
    try {
      mkdirSync(dirname(join(root, TAXONOMY_REL_PATH)), { recursive: true });
      writeFileSync(join(root, TAXONOMY_REL_PATH), readFileSync(join(repoRootFromHere(), TAXONOMY_REL_PATH)));
      for (const [path, body] of Object.entries(vectors.files)) {
        mkdirSync(dirname(join(root, owner, path)), { recursive: true });
        writeFileSync(join(root, owner, path), body);
      }
      const result = resolveFixtures(root, discovered, [row.uri]);
      expect(result.diagnostics).toEqual([]);
      if (row.path === null) {
        expect(result.missing).toEqual([row.uri]);
        expect(result.fixtures).toEqual([]);
      } else {
        expect(result.missing).toEqual([]);
        expect(result.fixtures[0]?.path).toBe(`${owner}/${row.path}`);
        const bytes = readFileSync(join(root, owner, row.path));
        expect(result.fixtures[0]?.digest).toBe(createHash("sha256").update(bytes).digest("hex").slice(0, 32));
        const validate = new Ajv({ strict: false }).compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/FixtureRef" });
        expect(validate(result.fixtures[0])).toBe(true);
      }
    } finally { rmSync(root, { recursive: true, force: true }); }
  });
  test("the protocol rejects retired case-local fixture references", () => {
    const validate = new Ajv({ strict: false }).compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/FixtureRef" });
    expect(validate({ uri: "local://x", scope: "local", name: "x", path: "x", digest: "x" })).toBe(false);
  });
});
