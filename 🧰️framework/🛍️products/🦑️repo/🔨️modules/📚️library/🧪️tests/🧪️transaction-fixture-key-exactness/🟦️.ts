import { expect, test } from "bun:test";
import { readFileSync, existsSync } from "node:fs";
import { join, resolve } from "node:path";

/** 🧩️ Guards the split of the former 13-key `transaction-dispositions` blob (see
 *  `🧹️normalization/🟦️.ts` `requireExactKeys("transaction sentinel cases fixture", …)`) into
 *  one kind-only `🔣️.json` leaf per semantic consumer, each carrying exactly its own keys. */
const packagesRoot = resolve(import.meta.dir, "../../📦️packages/🟦️typescript");
const fixturesRoot = join(packagesRoot, "🧫️fixtures");

const groups: readonly { readonly directory: string; readonly exactKeys: readonly string[] }[] = [
  { directory: "🧪️transaction-sentinel-cases", exactKeys: ["schemaVersion", "symlinkFlavorCases", "virtualPathPolicyCases"] },
  { directory: "🧪️transaction-disposition-outcomes", exactKeys: ["affectedStateCases", "expectedDispositionOperations", "negativeDispositionCases", "schemaVersion"] },
  { directory: "🧪️transaction-protocol", exactKeys: ["failureStages", "journalStates", "schemaVersion", "virtualPreimageNodes"] },
  { directory: "🧪️transaction-ledger-boundaries", exactKeys: ["boundaries", "schemaVersion", "transactionLedgers", "workspaceLedgers"] },
];

/** 🧮️ Mirrors `requireExactKeys`: the parsed object's own keys, sorted, must equal the expected set exactly — no more, no fewer. */
function requireExactKeysLike(value: Record<string, unknown>, keys: readonly string[]): void {
  expect(Object.keys(value).sort()).toEqual([...keys].sort());
}

test("the former combined transaction-dispositions fixture no longer exists", () => {
  expect(existsSync(join(fixturesRoot, "🧪️transaction-dispositions"))).toBe(false);
});

for (const group of groups) test(`${group.directory} fixture carries exactly its own consumer's keys`, () => {
  const path = join(fixturesRoot, group.directory, "🔣️.json");
  expect(existsSync(path)).toBe(true);
  const value = JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
  requireExactKeysLike(value, group.exactKeys);
  expect(value.schemaVersion).toBe(1);
});

test("the dead attemptLayout key was dropped, not smuggled into any split fixture", () => {
  for (const group of groups) {
    const path = join(fixturesRoot, group.directory, "🔣️.json");
    const value = JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
    expect(Object.hasOwn(value, "attemptLayout")).toBe(false);
  }
});

test("sentinel case rows keep the exact shape the normalization engine requires", () => {
  const value = JSON.parse(readFileSync(join(fixturesRoot, "🧪️transaction-sentinel-cases/🔣️.json"), "utf8")) as {
    virtualPathPolicyCases: Record<string, unknown>[];
    symlinkFlavorCases: Record<string, unknown>[];
  };
  for (const row of value.virtualPathPolicyCases) requireExactKeysLike(row, ["id", "inputPath", "physicalSourcePath", "expectedViolationCode", "sourceContentHash"]);
  for (const row of value.symlinkFlavorCases) requireExactKeysLike(row, ["id", "repositoryRoot", "target", "owned"]);
});
