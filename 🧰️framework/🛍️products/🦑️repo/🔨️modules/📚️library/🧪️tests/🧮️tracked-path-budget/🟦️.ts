import { describe, expect, test } from "bun:test";
import { execFileSync } from "node:child_process";
import { lstatSync } from "node:fs";
import { resolve } from "node:path";
import iconv from "iconv-lite";
import fixture from "../🧫️fixtures/🧮️tracked-path-budget/🔣️.json";
import { loadCatalogTaxonomy } from "../../🔍️discovery/🔐️taxonomy-authority/🟦️.ts";
import { pathBudgetCodes, pathUtf16Units } from "../../🔍️discovery/📏️path-budget/🟦️.ts";

const repoRoot = process.env.SEMIO_FIXTURE_REPO_ROOT ?? resolve(import.meta.dir, "../../../../../../../");
const render = (parts: readonly Readonly<{ text: string; count: number }>[]): string => parts.map(({ text, count }) => text.repeat(count)).join("");
const thirdPartyUnits = (value: string): number => iconv.encode(value, "utf16-le").length / 2;
const rootUnits = (id: string): number => fixture.checkoutRoots.find((row) => row.id === id)!.utf16Units;

/** 🗺️ Paths git indexes plus the untracked, not ignored ones, restricted to those that still exist (a rename in flight leaves the old name indexed). */
function trackedPaths(): readonly string[] {
  const listed = execFileSync("git", ["-c", "core.quotepath=false", "ls-files", "-z", "--cached", "--others", "--exclude-standard"], { cwd: repoRoot, maxBuffer: 1 << 30 });
  return [...new Set(listed.toString("utf8").split("\0").filter(Boolean))].filter((path) => {
    try {
      lstatSync(resolve(repoRoot, path));
      return true;
    } catch {
      return false;
    }
  });
}

describe("tracked path budget (language-agnostic fixture)", () => {
  test("the fixture policy is the taxonomy collision policy", () => {
    const policy = loadCatalogTaxonomy().collisionPolicy;
    expect({ maxPathUtf16Units: policy.maxPathUtf16Units, maxComponentUtf8Bytes: policy.maxComponentUtf8Bytes }).toEqual(fixture.policy);
  });

  for (const root of fixture.checkoutRoots)
    test(`checkout root ${root.id} has ${root.utf16Units} units`, () => {
      expect(pathUtf16Units(root.root)).toBe(root.utf16Units);
      expect(thirdPartyUnits(root.root)).toBe(root.utf16Units);
    });

  for (const row of fixture.cases)
    test(`${row.id}: root plus relative path`, () => {
      const path = render(row.parts);
      expect(pathUtf16Units(path)).toBe(row.relativeUtf16Units);
      expect(thirdPartyUnits(path)).toBe(row.relativeUtf16Units);
      expect(rootUnits(row.root) + pathUtf16Units(path) <= fixture.policy.maxPathUtf16Units).toBe(row.fits);
    });

  test("every tracked product path fits under the reference checkout root and every ticket path under the repo-relative limit", () => {
    const reserve = rootUnits(fixture.treeRoot);
    const offenders: string[] = [];
    let longest = { units: 0, path: "" };
    for (const path of trackedPaths()) {
      const units = pathUtf16Units(path);
      const ticket = path.startsWith(fixture.ticketPrefix);
      if (!ticket && units > longest.units) longest = { units, path };
      const fits = ticket ? pathBudgetCodes(path, fixture.policy).length === 0 : reserve + units <= fixture.policy.maxPathUtf16Units && pathBudgetCodes(path, fixture.policy).length === 0;
      if (!fits) offenders.push(`${units} ${path}`);
    }
    expect(thirdPartyUnits(longest.path)).toBe(longest.units);
    expect(offenders.sort().slice(0, 20)).toEqual([]);
  }, 180_000);
});
