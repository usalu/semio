/** 🗂️ The schema index excludes test specimens before resolving domain schema documents. */
import { test } from "bun:test";
import { strict as assert } from "node:assert";
import fg from "fast-glob";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { schemaDocumentIndex } from "../../🗂️schema-index/🟦️.ts";
const vector = JSON.parse(readFileSync(new URL("../🧫️fixtures/🧬️schema-search/🔣️.json", import.meta.url), "utf8"));
const collections = new Set(["🧫️fixtures", "🧪️fixtures", "🧫️examples", "🧪️examples", "🧪️tests"]);
const isCollection = (path: string) => path.split("/").some((part, ordinal, parts) => collections.has(part) && parts[ordinal - 1] !== "🔨️modules");
for (const item of vector.cases) test(item.id, async () => {
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw Error("Ticket-owned schema search output required");
  mkdirSync(artifactRoot, { recursive: true });
  const base = mkdtempSync(join(artifactRoot, "dsl-schema-search-oracle-")), root = join(base, item.root);
  try {
    mkdirSync(root, { recursive: true });
    for (const [path, document] of Object.entries(item.files)) {
      const destination = resolve(root, path);
      mkdirSync(dirname(destination), { recursive: true });
      writeFileSync(destination, JSON.stringify(document));
    }
    const files = await fg("**/🧬️schema/**/*.json", { cwd: root, onlyFiles: true, dot: false, followSymbolicLinks: false, ignore: ["**/target/**", "**/node_modules/**", "**/dist/**", "**/🗑️generated/**", "**/📦️packages/**"] });
    const oracle = isCollection(item.root) ? [] : files.filter(path => !isCollection(path)).map(path => JSON.parse(readFileSync(join(root, path), "utf8")).$id).sort();
    const actual = [...schemaDocumentIndex(root).keys()].sort();
    assert.deepEqual(actual, oracle, item.id);
    assert.deepEqual(actual, item.expected.ids, item.id);
  } finally { rmSync(base, { recursive: true, force: true }); }
});
