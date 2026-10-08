import { expect, test } from "bun:test";
import { schemaScopeIdFromDocumentId } from "../../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";
import Ajv from "ajv";
import { build, transform } from "esbuild";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, symlinkSync, writeFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import { distributionStaticSourcePaths } from "../🟦️.ts";
import { DISTRIBUTION_LAYOUT, parseDistributionManifest, parseDistributionStaticInputs } from "../../🟦️.ts";

const examples = JSON.parse(readFileSync(join(import.meta.dir, "../🧫️fixtures/🔣️.json"), "utf8"));
const schema = JSON.parse(readFileSync(join(import.meta.dir, "../../🧬️schema/🔣️.json"), "utf8"));

test("distribution production coordinates refuse testing collections with genuine schema parity", () => {
  const validate = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/DistributionSourceCoordinate`)!;
  for (const row of examples.coordinates) {
    expect(validate(row.path), row.path).toBe(!row.refused);
    const staticInput = () => parseDistributionStaticInputs({ version: 1, paths: [row.path], moduleEntries: [row.path] });
    const manifest = () => parseDistributionManifest({ version: 1, contractId: "dev-distribution-bundle", layoutSha256: "a".repeat(64), inputs: [{ path: row.path, bytes: 0, sha256: "a".repeat(64) }], outputs: [{ ownerId: DISTRIBUTION_LAYOUT.entry.output, path: DISTRIBUTION_LAYOUT.entry.output, bytes: 0, sha256: "a".repeat(64) }] }, DISTRIBUTION_LAYOUT);
    if (row.refused) { expect(staticInput).toThrow(/fixture|testing/iu); expect(manifest).toThrow(/fixture|testing/iu); }
    else { expect(staticInput().paths).toEqual([row.path]); expect(manifest().inputs[0]!.path).toBe(row.path); }
  }
});

test("distribution source closure excludes inline tests and refuses actual fixture inputs", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("Explicit ticket output required for distribution source verification");
  mkdirSync(output, { recursive: true });
  const work = mkdtempSync(join(output, "distribution-source-"));
  const validate = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/DistributionSourceCoordinate`)!;
  try {
    for (const row of examples.closures) {
      const root = join(work, row.id);
      for (const [path, source] of Object.entries(row.files) as [string, string][]) { const target = join(root, path); mkdirSync(dirname(target), { recursive: true }); writeFileSync(target, source); }
      for (const [path, target] of Object.entries(row.links ?? {}) as [string, string][]) symlinkSync(target, join(root, path));
      const actual = () => distributionStaticSourcePaths(root, row.entries);
      const oracle = await build({ absWorkingDir: root, entryPoints: row.entries, bundle: true, write: false, format: "esm", platform: "browser", metafile: true, define: { "import.meta.vitest": "undefined" }, logLevel: "silent", plugins: [{ name: "independent-production-exclusion", setup(compiler) { compiler.onLoad({ filter: /\.[cm]?[jt]sx?$/ }, async args => ({ contents: (await transform(readFileSync(args.path, "utf8"), { loader: args.path.endsWith("tsx") ? "tsx" : args.path.endsWith("ts") ? "ts" : "js", define: { "import.meta.vitest": "undefined" }, minifySyntax: true })).code, loader: "js" })); } }] });
      const oraclePaths = Object.keys(oracle.metafile!.inputs).map(path => relative(root, join(root, path)).replaceAll("\\", "/")).sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
      expect(oraclePaths.some(path => !validate(path)), row.id).toBe(row.refused);
      if (row.refused) { await expect(actual()).rejects.toThrow(/fixture|testing/iu); continue; }
      const paths = await actual();
      expect(paths, row.id).toEqual([...row.paths].sort((a: string, b: string) => Buffer.compare(Buffer.from(a), Buffer.from(b))));
      expect(paths, row.id).toEqual(oraclePaths);
    }
    console.log("[DEBUG] actual production source closure compared with esbuild; physical fixture aliases refused");
  } finally { rmSync(work, { recursive: true, force: true }); }
});

test("distribution actual static producer inventory contains no testing collections", async () => {
  const root = process.env.REPO_ROOT;
  if (!root) throw Error("Actual repository root required for distribution source verification");
  const authority = parseDistributionStaticInputs(JSON.parse(readFileSync(join(import.meta.dir, "../../🔗️inputs.json"), "utf8")));
  const paths = await distributionStaticSourcePaths(root, authority.moduleEntries);
  const validate = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/DistributionSourceCoordinate`)!;
  expect(paths.length).toBeGreaterThan(authority.moduleEntries.length);
  for (const path of paths) expect(validate(path), path).toBe(true);
  console.log(`[DEBUG] actual distribution static producer inventory: ${paths.length} source/package inputs, zero testing collection inputs`);
});

test("distribution genuine source contract has canonical addressable scope", () => {
  const example = examples.contract;
  expect(schemaScopeIdFromDocumentId(schema.$id)).toBe(example.scopeId);
  const actual = new URL(schema.$id), expected = new URL(example.id);
  expect(actual.origin).toBe(expected.origin);
  expect(actual.pathname.split("/").slice(1, -1).join(".")).toBe(example.scopeId);
  const validate = new Ajv({ strict: true }).addSchema(schema).getSchema(`${example.id}#/$defs/${example.leaf}`);
  expect(validate).toBeDefined();
  expect(validate!(examples.coordinates.find((row: { refused: boolean }) => !row.refused).path)).toBe(true);
  console.log("[DEBUG] actual genuine distribution coordinate contract resolves through owned taxonomy, URL and Ajv");
});
