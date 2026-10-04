//#region Imports
import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { findNodeAtLocation, parseTree } from "jsonc-parser";
import { loadCatalogTaxonomy, validateFrozenCoordinateEvidenceContracts } from "../../🔍️discovery/🟦️.ts";
import { frozenCoordinateEvidenceCoordinates } from "../../🧹️normalization/🟦️.ts";
//#endregion Imports

//#region Authority
const libraryRoot = resolve(import.meta.dir, "../..");
const repoRoot = resolve(libraryRoot, "../../../../..");
const contractId = "remaining-package-purity-history-v1";
const registered = loadCatalogTaxonomy().frozenCoordinateEvidenceContracts[contractId]!;
const goldenPath = join(repoRoot, registered.path);
const goldenBytes = readFileSync(goldenPath);
const sha = (bytes: Uint8Array) => createHash("sha256").update(bytes).digest("hex");
const golden = JSON.parse(goldenBytes.toString("utf8"));

/** 🧭️ The token start (inside the opening quote) of every row's column-0 sourcePath, read from the live sealed bytes by
 * an independent JSON parser. A 2026-08-29 `clean taxonomy plan --scope "🧰️framework/🔨️modules"` reported such tokens
 * `frozen-coordinate-evidence-unowned` while the registration authorized only `mappings[29]`; the registration was widened to a
 * row-index wildcard over column 0 plus one explicit destinationPath pointer for row 69. Deriving the offsets from the live
 * bytes keeps the proof exact across every recorded re-seal (`🧫️frozen-seal-ledger`). */
const mappingsNode = findNodeAtLocation(parseTree(goldenBytes.toString("utf8"))!, ["mappings"])!;
const columnZeroOffsets = mappingsNode.children!.map((row) => row.children![0]!.offset + 1);
//#endregion Authority

//#region Tests
test("the fixture on disk still matches the registered whole-document digest", () => {
  expect(sha(goldenBytes)).toBe(registered.sha256);
  expect(registered.coordinates).toHaveLength(6);
});

test("every column-0 token sits inside the mappings array, and column 0 is a safe wildcard while column 10 is not", () => {
  const mappings = golden.mappings as unknown[][];
  expect(columnZeroOffsets).toHaveLength(mappings.length);
  for (const offset of columnZeroOffsets) expect(/"mappings"\s*:\s*\[[\s\S]*$/u.test(goldenBytes.toString("utf8").slice(0, offset)), `offset ${offset} must sit inside the mappings array`).toBe(true);
  expect(mappings.every((row) => typeof row[0] === "string" && row[0].length > 0)).toBe(true);
  expect(mappings.filter((row) => row[10] === null || row[10] === "")).toHaveLength(1);
});

test("the live registration is exactly the wildcard-plus-one-explicit-row shape, not a hand-enumerated list", () => {
  expect(registered.coordinates).toEqual([
    { pointer: "/mappings/*/0", kind: "source" },
    { pointer: "/mappings/29/3", kind: "source", representation: "recorded-package-owner-identity", identityPrefix: "unmarked:" },
    { pointer: "/mappings/29/4", kind: "source" },
    { pointer: "/mappings/29/5", kind: "source" },
    { pointer: "/mappings/29/6", kind: "destination" },
    { pointer: "/mappings/69/10", kind: "destination" },
  ]);
});

test("the live registration resolves every column-0 token without widening ownership over row 29's other four fields or the one row with no destinationPath", () => {
  expect(validateFrozenCoordinateEvidenceContracts({ [contractId]: registered })).toEqual([]);
  const actual = frozenCoordinateEvidenceCoordinates(registered.path, goldenBytes, { [contractId]: registered })!;
  expect(actual).toHaveLength(golden.mappings.length + 5);
  const covered = new Set(actual.map((row) => row.start));
  for (const offset of columnZeroOffsets) expect(covered.has(offset), `offset ${offset} must be covered`).toBe(true);
  // The four fields kept explicit for row 29 are still exactly the ones the identity test owns —
  // the wildcard only ever widens column 0, never 3/4/5/6, so mapping[29]'s existing coverage for
  // those columns is untouched by this patch.
  expect(actual.filter((row) => row.pointer.startsWith("/mappings/29/")).map((row) => row.pointer).sort()).toEqual(["/mappings/29/0", "/mappings/29/3", "/mappings/29/4", "/mappings/29/5", "/mappings/29/6"]);
});

test("a full wildcard on column 10 would be unsound — one row's destinationPath is null", () => {
  const unsound = { ...registered, coordinates: [{ pointer: "/mappings/*/10", kind: "destination" as const }] };
  expect(() => frozenCoordinateEvidenceCoordinates(unsound.path, goldenBytes, { [contractId]: unsound })).toThrow(/must be a physical repository-relative path string/u);
});
//#endregion Tests
