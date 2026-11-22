/** 📦️ Law: every non-generated `exports` target of every tracked package.json exists; the detector agrees with its fixtures and with an independent Python reading of the whole repository. */
import { expect, test } from "bun:test";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";
import { runToolSyncV1, ToolErrorV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import { testCheckoutRootV1 } from "../../../🧪️testing/🧫️private-repository/🟦️.ts";
import { listPackageJsonFilesV1, missingPackageExportTargetsV1, packageExportBreachesV1, packageExportReportV1, packageExportTargetsV1 } from "../🟦️.ts";

const here = dirname(fileURLToPath(import.meta.url));

test("the detector reads strings, conditions and arrays, skips patterns and generated output, and reports missing files", () => {
  const exports = { ".": "./src/a.ts", "./b": { import: "./src/b.ts", default: ["./src/c.ts", "./dist/x.js"] }, "./all/*": "./src/*.ts", "./pkg": "not-relative" };
  expect(packageExportTargetsV1(exports)).toEqual(["./src/a.ts", "./src/b.ts", "./src/c.ts"]);
  expect(missingPackageExportTargetsV1(JSON.stringify({ exports }), (target) => target === "./src/a.ts")).toEqual(["./src/b.ts", "./src/c.ts"]);
  expect(missingPackageExportTargetsV1("{", () => false)).toEqual([]);
  expect(missingPackageExportTargetsV1("{}", () => false)).toEqual([]);
});

test("no tracked package.json exports a file that does not exist, and the Python reading agrees", () => {
  const root = testCheckoutRootV1(), files = listPackageJsonFilesV1(root);
  if (files.length === 0) return;
  const report = packageExportReportV1(root, files);
  expect([...report].map(([path, missing]) => `${path}: ${missing.join(", ")}`)).toEqual([]);
  const directory = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? resolve(here), "pet-"));
  try {
    const listing = join(directory, "files.json");
    writeFileSync(listing, JSON.stringify(files));
    let output: string;
    try {
      output = runToolSyncV1("python", [join(here, "🐍️oracle", "🐍️.py"), root, listing], { encoding: "utf8", check: true, maxBuffer: 256 * 1024 * 1024 }).stdout;
    } catch (error) {
      if (error instanceof ToolErrorV1 && error.code === "tool-missing") return;
      throw error;
    }
    expect(JSON.parse(output)).toEqual(Object.fromEntries(report));
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});

test("the gate's breach lines name every missing target of a tracked package.json in a git repository, and none once the files exist", () => {
  const directory = mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR ?? resolve(here), "pet-gate-"));
  try {
    runToolSyncV1("git", ["init", "--quiet"], { cwd: directory, check: true });
    mkdirSync(join(directory, "pkg", "src"), { recursive: true });
    writeFileSync(join(directory, "pkg", "package.json"), JSON.stringify({ exports: { ".": "./src/a.ts", "./b": "./src/b.ts", "./dist": "./dist/x.js" } }));
    writeFileSync(join(directory, "pkg", "src", "a.ts"), "");
    runToolSyncV1("git", ["add", "."], { cwd: directory, check: true });
    expect(packageExportBreachesV1(directory)).toEqual(["pkg/package.json: ./src/b.ts"]);
    writeFileSync(join(directory, "pkg", "src", "b.ts"), "");
    expect(packageExportBreachesV1(directory)).toEqual([]);
    expect(packageExportBreachesV1(join(directory, "pkg", "src"))).toEqual([]);
  } finally {
    rmSync(directory, { recursive: true, force: true });
  }
});
