//#region 🔌️Adapters
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { posix, resolve } from "node:path";
import { packageImplementationDestination } from "../../📏️fixed-contracts/🟦️.ts";
import { loadTaxonomy } from "../../🔣️taxonomy/🟦️.ts";
//#endregion 🔌️Adapters

//#region 🧬️Contract
const repoRoot = resolve(import.meta.dir, "../../../../../../../..");
const vectors = JSON.parse(readFileSync(new URL("../🧫️fixtures/📦️package-implementation-destination/🔣️.json", import.meta.url), "utf8")) as {
  schemaVersion: 1;
  cases: readonly { id: string; ecosystem: "rust" | "typescript"; owner: string; ownerCanonical: string; packageRelative: string; sourceRelative: string; fileKind: string | null; stem: string | null; semanticDirectoryName?: string; expected: string | null }[];
};
const taxonomy = loadTaxonomy({ repoRoot });
const packages = taxonomy.discoverySchema.packagesDirName;
const ecosystem = (needle: string): string => Object.keys(taxonomy.discoverySchema.ecosystems).find((id) => id.endsWith(needle))!;
//#endregion 🧬️Contract

//#region 🧪️Destination
/** 🧭️ A package implementation file keeps its module directories below the package root when it moves onto its owner, so sibling modules never collapse onto one destination. */
describe("package implementation destination keeps the module path", () => {
  test("fixture identifiers and expected destinations are distinct", () => {
    const ids = vectors.cases.map((row) => row.id);
    expect(new Set(ids).size).toBe(ids.length);
    const destinations = vectors.cases.filter((row) => row.expected !== null).map((row) => `${row.ownerCanonical}/${row.expected}/${row.ecosystem}`);
    expect(new Set(destinations).size).toBe(destinations.length);
  });

  for (const row of vectors.cases) test(row.id, () => {
    const packageRoot = posix.join(row.owner, packages, ecosystem(row.ecosystem));
    const source = posix.join(packageRoot, row.sourceRelative);
    const packageCanonical = posix.join(row.ownerCanonical, packages, ecosystem(row.ecosystem));
    const kindOnly = posix.basename(row.sourceRelative);
    const canonicalPath = posix.join(packageCanonical, posix.dirname(row.sourceRelative), row.semanticDirectoryName ?? "", kindOnly);
    const canonical = { path: canonicalPath, fileKind: row.fileKind, stem: row.stem, ...(row.semanticDirectoryName ? { semanticDirectoryName: row.semanticDirectoryName } : {}), violations: [] };
    const directories = new Map([[row.owner, row.ownerCanonical], [packageRoot, packageCanonical]]);
    const actual = packageImplementationDestination(source, canonical, directories, new Map(), taxonomy);
    expect(actual).toBe(row.expected === null ? null : posix.join(row.ownerCanonical, row.expected));
    if (row.expected !== null && row.stem === null) expect(actual).toBe(posix.join(row.ownerCanonical, posix.relative(packageCanonical, canonicalPath)));
  });
});
//#endregion 🧪️Destination
