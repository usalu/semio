import { REPO_TEST_DOMAIN_REL } from "../../🧱️contract/🟦️.ts";
import { type BreachRecord } from "../../../📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { existsSync, readdirSync } from "node:fs";
import { dirname, join, resolve } from "node:path";
import { fileURLToPath } from "node:url";

/** 🧹️ Folder policy for this domain: the six generated output roots must never be committed. */
export function policy(): BreachRecord[] {
  const domainRoot = resolve(dirname(fileURLToPath(import.meta.url)), "../..");
  const repoRoot = resolve(domainRoot, "../../../../..");
  const breaches: BreachRecord[] = [];
  for (const name of readdirSync(domainRoot)) {
    if (
      ![
        "🧱️contract",
        "🔍️discovery",
        "🖥️host",
        "🏃️execution",
        "⚖️parity",
        "🧾️contracts",
        "📊️reporting",
        "🕸️dependencies",
        "🏭️inventory",
        "🧾️provenance",
        "📊️coverage",
        "🧹️retention",
        "🩺️environment",
        "⚖️policy",
        "🧬️schema",
        "📇️registry",
        "📦️packages",
        "🧫️fixtures",
        "🧪️tests",
        "📡️protocol",
        "🏃️runner",
        "🔮️oracles",
        "📜️script.ts",
        "📋️project.json",
        "🟨️.mjs",
        "🟦️.ts",
        "🦀️.rs",
        "AGENTS.md",
        "README.md",
        "node_modules",
      ].includes(name)
    ) {
      breaches.push({
        id: "unknown-domain-child",
        kind: "testing/taxonomy",
        scope: `${REPO_TEST_DOMAIN_REL}/${name}`,
        summary: `Unexpected child ${name} in the testing domain root`,
        priority: "medium",
        reason: "The testing domain root holds its schema, registry, packages, fixtures, self-tests and routers — nothing else.",
        solution: "Move it into the owning child directory, or delete it.",
      });
    }
  }
  const cachedInTree = existsSync(join(repoRoot, REPO_TEST_DOMAIN_REL, ".🧬semio"));
  if (cachedInTree)
    breaches.push({
      id: "nested-cache",
      kind: "testing/taxonomy",
      scope: REPO_TEST_DOMAIN_REL,
      summary: "A nested .🧬semio cache exists inside the testing domain",
      priority: "high",
      reason: "Generated test state belongs only under the repository cache root.",
      solution: "Delete the nested cache.",
    });
  return breaches;
}
