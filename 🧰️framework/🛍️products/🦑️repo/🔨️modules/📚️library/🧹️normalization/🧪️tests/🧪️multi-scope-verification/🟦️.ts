/** 🗺️ The multi-scope verification law over `🧫️fixtures/🧫️multi-scope-verification/🔣️.json`: `verifyTaxonomyScopes` reads the repository
 * once for every listed scope, and each verdict it yields — violations in order and cleanliness — equals `verifyTaxonomy` of that scope
 * alone, so the shared source admission, catalog view and incoming-reference scan never change a finding; the `moving` scopes (shared
 * incoming-reference scan) join at the exhaustive test level. A rejected generator preview is
 * compared without the preview subprocess's own exit status and output digests (each run spawns it afresh). */
import { describe, expect, test } from "bun:test";
import { readFileSync } from "node:fs";
import { fileURLToPath } from "node:url";
import { type TaxonomyViolation, verifyTaxonomy, verifyTaxonomyScopes } from "../../🟦️.ts";

const repoRoot = fileURLToPath(new URL("../../../../../../../../", import.meta.url)).replace(/\/$/u, "");
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🧫️multi-scope-verification/🔣️.json", import.meta.url), "utf8")) as { readonly scopes: readonly string[]; readonly moving: readonly string[] };
const scopes = [...fixture.scopes, ...(process.env.SEMIO_TEST_LEVEL === "exhaustive" ? fixture.moving : [])];
const stable = (violations: readonly TaxonomyViolation[]): TaxonomyViolation[] => violations.map((violation) => (violation.code === "generator-preview-invalid" ? { ...violation, message: violation.message.replace(/: status=.*$/su, "") } : violation));

describe("🗺️ multi-scope taxonomy verification", () => {
  test("each multi-scope verdict equals the single-scope verdict", () => {
    const multi = [...verifyTaxonomyScopes({ repoRoot, scopes })];
    expect(multi.map((row) => row.scope).sort()).toEqual([...scopes].sort());
    for (const row of multi) {
      const single = verifyTaxonomy({ repoRoot, scope: row.scope });
      expect({ scope: row.scope, clean: row.clean, violations: stable(row.violations) }).toEqual({ scope: row.scope, clean: single.clean, violations: stable(single.violations) });
    }
  }, process.env.SEMIO_TEST_LEVEL === "exhaustive" ? 1_800_000 : 900_000);
});
