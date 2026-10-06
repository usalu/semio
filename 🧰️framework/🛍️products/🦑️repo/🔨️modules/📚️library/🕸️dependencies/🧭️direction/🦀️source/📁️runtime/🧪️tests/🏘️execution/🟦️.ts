/** 🏘️ Exercises actual policy loading and the authored source census with Specific present and absent. */
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, realpathSync, rmSync } from "node:fs";
import { join } from "node:path";
import glob from "fast-glob";
import Ajv from "ajv/dist/2020.js";
import { writeRustLayerOracle } from "../../../../../../🧪️tests/🧱️rust-source-direction/🔮️oracle/🟦️.ts";
import { inspectRustSourceDirection, verifyRustSourceDirection } from "../../../🏃️execution/🟦️.ts";
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../../../../../../🧬️schema/🧱️rust-source-direction/🔣️.json", import.meta.url), "utf8"));
const validateReport = new Ajv({ strict: true }).compile({ $schema: schema.$schema, $defs: schema.$defs, $ref: "#/$defs/report" });
test("actual Rust source-direction execution rejects declared runtime owner paths even after Specific deletion", async () => {
  const output = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!output) throw Error("Runtime path census requires ticket output");
  mkdirSync(output, { recursive: true });
  const root = realpathSync(mkdtempSync(join(output, "runtime-path-census-")));
  try {
    for (const row of fixture.physicalCases) {
      const cwd = join(root, row.id), body = "pub fn f() { std::path::Path::new(" + JSON.stringify(row.value) + "); }";
      writeRustLayerOracle(cwd, { [row.source]: body, "✏️s/🔌️plugins/input.txt": "data" });
      if (row.removeSpecific) rmSync(join(cwd, "✏️s"), { recursive: true });
      const inventory = glob.sync("**/*.rs", { cwd, onlyFiles: true, followSymbolicLinks: false, ignore: ["node_modules/**", "target/**"] });
      expect(inventory).toContain(row.source);
      const report = await inspectRustSourceDirection(cwd);
      expect(validateReport(report), JSON.stringify(validateReport.errors)).toBe(true);
      expect(validateReport({ ...report, runtime: undefined })).toBe(false);
      expect(validateReport({ ...report, runtime: { ...report.runtime, scope: "all-runtime-dependencies" } })).toBe(false);
      expect(report.runtime.references).toBe(1);
      expect(report.runtime.violations).toHaveLength(row.violations);
      expect(report.violations).toEqual([]);
      expect(report.problems).toEqual([]);
      if (row.violations) await expect(verifyRustSourceDirection(cwd)).rejects.toThrow("runtime path");
      else await expect(verifyRustSourceDirection(cwd)).resolves.toBeUndefined();
      console.log("[DEBUG] runtime source census " + JSON.stringify({ id: row.id, runtime: report.runtime }));
    }
  } finally { rmSync(root, { recursive: true, force: true }); }
});
