/** 🔑️ Laws of JSON duplicate keys: the fixtures agree with their schema, the streaming detector reports exactly the fixture duplicates and agrees with the standard Python JSON parser, no JSON file of the repository repeats an object key, and the breach record names the file, line and key. */
import { expect, test } from "bun:test";
import Ajv from "ajv";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { runToolSyncV1, ToolErrorV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { testCheckoutRootV1 } from "../../../🧪️testing/🧫️private-repository/🟦️.ts";
import { findJsonDuplicateKeysV1, jsonDuplicateKeyReportV1, listJsonFilesV1, policyJsonDuplicateKeyBreaches } from "../🟦️.ts";
import fixtures from "./🧫️fixtures/🔣️.json" with { type: "json" };
import schema from "../🧬️schema/🔣️.json" with { type: "json" };

const here = dirname(fileURLToPath(import.meta.url));
const oracle = join(here, "🐍️oracle", "🐍️.py");
const python = (args: string[]): string | undefined => {
  try {
    return runToolSyncV1("python", [oracle, ...args], { encoding: "utf8", check: true, maxBuffer: 256 * 1024 * 1024 }).stdout;
  } catch (error) {
    if (error instanceof ToolErrorV1 && error.code === "tool-missing") return undefined;
    throw error;
  }
};
const scratch = <T>(body: (directory: string) => T): T => {
  const directory = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? resolve(here), "jdk-"));
  try {
    return body(directory);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
};
const key = (rows: readonly { path: string; key: string }[]) => rows.map(({ path, key }) => ({ path, key })).sort((a, b) => a.path.localeCompare(b.path) || a.key.localeCompare(b.key));

test("the fixtures agree with their schema", () => {
  expect(new Ajv({ strict: false, allErrors: true }).validate(schema, fixtures)).toBe(true);
});

test("the detector finds exactly the fixture duplicates", () => {
  for (const fixture of fixtures.cases) expect({ name: fixture.name, duplicates: findJsonDuplicateKeysV1(fixture.source) }).toEqual({ name: fixture.name, duplicates: fixture.duplicates as never });
});

test("the standard Python JSON parser agrees with the detector on the fixtures", () => {
  scratch((directory) => {
    const cases = join(directory, "fixtures.json");
    writeFileSync(cases, JSON.stringify(fixtures));
    const output = python(["fixtures", cases]);
    if (output === undefined) return;
    expect(JSON.parse(output)).toEqual(fixtures.cases.filter((fixture) => (fixture as { oracle?: boolean }).oracle !== false).map((fixture) => ({ name: fixture.name, duplicates: key(fixture.duplicates) })));
  });
});

test("no sampled JSON file of the repository repeats an object key and the Python parser agrees", () => {
  const root = testCheckoutRootV1();
  const files = listJsonFilesV1(root);
  expect(files.length).toBeGreaterThan(1000);
  const sample = files.filter((_, index) => index % 8 === 0);
  const report = jsonDuplicateKeyReportV1(root, sample);
  expect([...report].map(([path, rows]) => `${path}: ${rows.map((row) => `${row.path}:${row.line}`).join(", ")}`)).toEqual([]);
  scratch((directory) => {
    const listed = join(directory, "files.json");
    writeFileSync(listed, JSON.stringify(sample));
    const output = python(["files", root, listed]);
    if (output === undefined) return;
    expect(JSON.parse(output)).toEqual(Object.fromEntries([...report].map(([path, rows]) => [path, key(rows)])));
  });
}, 240000);

test("the breach record names the file, line and key and a clean tree has none", () => {
  scratch((directory) => {
    mkdirSync(join(directory, "a"), { recursive: true });
    writeFileSync(join(directory, "a", "bad.json"), '{\n  "x": {"k": 1,\n "k": 2}\n}');
    writeFileSync(join(directory, "good.json"), '{"k":1,"j":{"k":2}}');
    const breaches = policyJsonDuplicateKeyBreaches(directory);
    expect(breaches).toHaveLength(1);
    expect(breaches[0]).toMatchObject({ kind: "repo/json-duplicate-key", scope: "a/bad.json", line: 3, priority: "high" });
    expect(breaches[0]!.summary).toContain('a/bad.json:3 repeats the key "k" at $.x.k');
    rmSync(join(directory, "a"), { recursive: true });
    expect(policyJsonDuplicateKeyBreaches(directory)).toEqual([]);
  });
});
