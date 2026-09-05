//#region Imports
import { expect, test } from "bun:test";
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
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

/**
 * Every real offset below was surfaced by a live `clean taxonomy plan --scope "🧰️framework/🔨️modules"`
 * run against this ticket's baseline on 2026-08-29, back when the registered contract only ever
 * authorized `mappings[29]`'s five fields: any OTHER `mappings[N][0]` (sourcePath) or `mappings[N][10]`
 * (destinationPath) token that a rename touched was reported `frozen-coordinate-evidence-unowned`,
 * even though the whole-document digest was untouched and the value is genuine census data. Offsets
 * are the reported token start (inside the opening quote), one per distinct `mappings` row. The
 * registration has since been widened — a row-index wildcard over column 0, plus one explicit
 * destinationPath pointer for row 69 — to cover all of them at once; this file asserts that
 * widening is exactly what is now live, and exactly why it is sound.
 */
const observedUnownedOffsets = [
  47215, 48068, 48811, 51282, 56737, 58391, 59214, 60031, 60839, 61626, 62431, 63302, 64981, 65514,
  65748, 66613, 67607, 69849, 73599, 79613, 81704, 83286, 89156, 90055, 97490, 106638, 108510, 115075,
  123021, 126596, 127308, 130192, 130922, 133529, 134344, 138434, 141697, 142556, 143407, 145073,
];
//#endregion Authority

//#region Tests
test("the fixture on disk still matches the registered whole-document digest", () => {
  expect(sha(goldenBytes)).toBe(registered.sha256);
  expect(registered.coordinates).toHaveLength(6);
});

test("every observed offset sits inside the mappings array, and column 0 is a safe wildcard while column 10 is not", () => {
  for (const offset of observedUnownedOffsets) expect(/"mappings"\s*:\s*\[[\s\S]*$/u.test(goldenBytes.toString("utf8", 0, offset)), `offset ${offset} must sit inside the mappings array`).toBe(true);
  // Structural proof that column 0 (sourcePath) is safe to wildcard across all 229 rows, while
  // column 10 (destinationPath) is not: exactly one row — the "unresolved" census tail — has no
  // destinationPath yet, which is why the live registration keeps that column explicit.
  const mappings = golden.mappings as unknown[][];
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

test("the live registration resolves every observed offset without widening ownership over row 29's other four fields or the one row with no destinationPath", () => {
  expect(validateFrozenCoordinateEvidenceContracts({ [contractId]: registered })).toEqual([]);
  const actual = frozenCoordinateEvidenceCoordinates(registered.path, goldenBytes, { [contractId]: registered })!;
  expect(actual).toHaveLength(229 + 5);
  const covered = new Set(actual.map((row) => row.start));
  for (const offset of observedUnownedOffsets) expect(covered.has(offset), `offset ${offset} must now be covered`).toBe(true);
  // The four fields kept explicit for row 29 are still exactly the ones the identity test owns —
  // the wildcard only ever widens column 0, never 3/4/5/6, so mapping[29]'s existing coverage for
  // those columns is untouched by this patch.
  expect(actual.filter((row) => row.pointer.startsWith("/mappings/29/")).map((row) => row.pointer).sort()).toEqual(["/mappings/29/0", "/mappings/29/3", "/mappings/29/4", "/mappings/29/5", "/mappings/29/6"]);
});

test("a full wildcard on column 10 would be unsound — one row's destinationPath is null", () => {
  const unsound = { ...registered, coordinates: [{ pointer: "/mappings/*/10", kind: "destination" }] };
  expect(() => frozenCoordinateEvidenceCoordinates(unsound.path, goldenBytes, { [contractId]: unsound })).toThrow(/must be a physical repository-relative path string/u);
});
//#endregion Tests
