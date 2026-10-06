import { test, expect } from "bun:test";
import { mkdirSync, mkdtempSync, readFileSync, writeFileSync, existsSync, rmSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import * as toml from "@iarna/toml";
import cases from "../../🧫️fixtures/🧱️production-preparation/🔣️.json";
import { prepareStdioComposition } from "../../🟦️.ts";

const root = resolve(import.meta.dir, "../../../../../..");
const owner = join(root, "🌎️hub/🧩️compositions/🗄️stdio");

for (const row of cases.cases) test(`production preparation isolates examples: ${row.id}`, () => {
  const outputs = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!outputs) throw Error("SEMIO_TEST_ARTIFACT_DIR must name caller-owned test outputs");
  mkdirSync(outputs, { recursive: true });
  const scratch = mkdtempSync(join(outputs, "stdio-production-"));
  try {
    const owners = JSON.parse(readFileSync(join(owner, "🧩️composition/🔣️.json"), "utf8")).owners;
    const paths = ["🧩️composition/🔣️.json", "📇️publication/📜️native-catalog.json", "📇️publication/🔣️.json", ...owners.map((entry: { root: string; main: boolean }) => entry.main ? "📦️packages/🦀️rust/Cargo.toml" : join(dirname(entry.root), "📦️packages/🦀️rust/Cargo.toml"))];
    for (const path of paths) {
      const target = join(scratch, path);
      mkdirSync(dirname(target), { recursive: true });
      writeFileSync(target, readFileSync(join(owner, path)));
    }
    const manifestPath = join(scratch, "📦️packages/🦀️rust/Cargo.toml");
    const manifest = toml.parse(readFileSync(manifestPath, "utf8")) as any;
    const artifacts = manifest.package.metadata.semio.sources.artifacts.map((path: string) => resolve(owner, "📦️packages/🦀️rust", path));
    const original = readFileSync(manifestPath, "utf8");
    const declaration = original.match(/^artifacts\s*=.*$/mu);
    if (!declaration) throw Error("Composition artifact input authority missing");
    writeFileSync(manifestPath, original.replace(declaration[0], "artifacts = " + JSON.stringify(artifacts)));
    const examples = new Map<string, string>();
    if (row.examplesPresent) for (const path of cases.examplePaths) {
      const bytes = readFileSync(join(owner, path), "utf8");
      mkdirSync(dirname(join(scratch, path)), { recursive: true });
      writeFileSync(join(scratch, path), bytes);
      examples.set(path, bytes);
    }
    const result = prepareStdioComposition(root, scratch);
    expect(result.contributions).toBeGreaterThan(0);
    expect(result.apps).toBeGreaterThan(0);
    expect(result.receipts).toBeGreaterThan(0);
    const prepared = readFileSync(manifestPath, "utf8");
    expect(Bun.TOML.parse(prepared)).toEqual(toml.parse(prepared));
    let writes = 0;
    for (const path of cases.examplePaths) {
      if (row.examplesPresent) writes += Number(readFileSync(join(scratch, path), "utf8") !== examples.get(path));
      else writes += Number(existsSync(join(scratch, path)));
    }
    expect(writes).toBe(cases.expectedExampleWrites);
    console.log(`[DEBUG] production preparation case=${row.id} contributions=${result.contributions} apps=${result.apps} receipts=${result.receipts} example-writes=${writes}`);
  } finally {
    rmSync(scratch, { recursive: true, force: true });
  }
});
