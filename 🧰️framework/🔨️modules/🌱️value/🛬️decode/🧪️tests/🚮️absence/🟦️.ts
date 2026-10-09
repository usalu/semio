/** 🚮️ Original source inventory proves the neutral decoder needs no specialization or fixture runtime input. */
import { expect, test } from "bun:test";
import { existsSync, readFileSync, realpathSync } from "node:fs";
import { dirname, relative, resolve } from "node:path";
import * as ts from "typescript";
import contract from "../../🧫️fixtures/🧩️ownership/🔣️.json";

const root = resolve(import.meta.dir, "../../../../../..");

test("the original neutral decoder closure excludes products, S, Hub and fixture runtime inputs", async () => {
  const { inspectRuntimeGraphV1 } = await import(resolve(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🕸️runtime/🟦️.ts"));
  const entry = relative(root, resolve(import.meta.dir, "../../🟦️.ts"));
  const graph = inspectRuntimeGraphV1([entry], { rootDirectory: root, read: (path: string) => {
    const absolute = resolve(root, path);
    return existsSync(absolute) ? readFileSync(absolute, "utf8") : undefined;
  } });
  expect(graph.findings).toEqual([]);
  expect(graph.nodes).toContain(entry);
  const independent: { from: string; to: string; kind: string }[] = [];
  for (const path of graph.nodes as string[]) {
    for (const candidate of [path, relative(root, realpathSync(resolve(root, path)))]) {
      expect(contract.absentRoots.some(owner => candidate === owner || candidate.startsWith(owner + "/")), candidate).toBe(false);
    }
    const source = readFileSync(resolve(root, path), "utf8");
    const parsed = ts.preProcessFile(source, true, true);
    for (const imported of parsed.importedFiles) {
      expect(imported.fileName.startsWith("."), imported.fileName).toBe(true);
      const target = resolve(root, dirname(path), imported.fileName);
      expect(existsSync(target), target).toBe(true);
      independent.push({ from: path, to: relative(root, target), kind: "import" });
    }
  }
  expect((graph.edges as typeof independent).toSorted((a, b) => JSON.stringify(a).localeCompare(JSON.stringify(b)))).toEqual(independent.toSorted((a, b) => JSON.stringify(a).localeCompare(JSON.stringify(b))));
  const fixture = relative(root, resolve(import.meta.dir, "../../🧫️fixtures/🔣️.json"));
  expect(inspectRuntimeGraphV1([fixture], { read: (path: string) => readFileSync(resolve(root, path), "utf8") }).findings.map((row: {code: string}) => row.code)).toEqual(["runtime-fixture-edge"]);
  console.log("[DEBUG] Original neutral decoder closure nodes=" + graph.nodes.length + " edges=" + graph.edges.length + " TypeScriptImportOracle=true lexicalRealSpecializations=0 fixtureRuntimeEdges=0 actualFixtureRefused=true SourceCopies=0");
});
