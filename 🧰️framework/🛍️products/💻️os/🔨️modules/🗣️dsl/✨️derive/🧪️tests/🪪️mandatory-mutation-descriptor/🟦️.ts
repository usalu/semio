/** 🔍️ Checks shared production schema-search specimens through an independent filesystem glob. */
import { test, expect } from "bun:test";
import fg from "fast-glob";
import { mkdirSync, mkdtempSync, readFileSync, rmSync, writeFileSync } from "node:fs";
import { dirname, join, resolve } from "node:path";

test("production schema search specimens match the independent filesystem oracle", async () => {
  const vector = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧬️schema-search/🔣️.json", import.meta.url), "utf8"));
  const artifactRoot = process.env.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot) throw Error("Ticket-owned schema search output required");
  mkdirSync(artifactRoot, { recursive: true });
  const base = mkdtempSync(join(artifactRoot, "dsl-schema-search-oracle-"));
  const collections = new Set(["🧫️fixtures", "🧪️fixtures", "🧪️tests"]);
  const isCollection = (path: string) => path.split("/").some((part, ordinal, parts) => collections.has(part) && parts[ordinal - 1] !== "🔨️modules");
  try {
    for (const [ordinal, item] of vector.cases.entries()) {
      const root = join(base, String(ordinal), item.root);
      mkdirSync(root, { recursive: true });
      for (const [path, document] of Object.entries(item.files)) {
        const destination = resolve(root, path);
        mkdirSync(dirname(destination), { recursive: true });
        writeFileSync(destination, JSON.stringify(document));
      }
      const files = await fg("**/🧬️schema/**/*.json", { cwd: root, onlyFiles: true, dot: false, ignore: ["**/target/**", "**/node_modules/**", "**/dist/**", "**/🗑️generated/**", "**/📦️packages/**"] });
      const ids = isCollection(item.root) ? [] : files.filter(path => !isCollection(path)).map(path => JSON.parse(readFileSync(join(root, path), "utf8")).$id).sort();
      expect(ids).toEqual(item.expected.ids);
      console.log("[DEBUG] independent fast-glob schema index case=" + item.id + " indexed=" + ids.length);
    }
  } finally {
    rmSync(base, { recursive: true, force: true });
  }
});
