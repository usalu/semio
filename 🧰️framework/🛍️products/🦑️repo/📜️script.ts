#!/usr/bin/env repo
/** 🧭️ `@repo` technology policy router: `repo ./📜️script.ts policy`. */
import { defineLint, type TechnologyLinter } from "./🔨️modules/📚️library/📦️packages/🟦️typescript/🚨️policy/🧩️linters/🟦️.ts";
import { runPolicyOnlyMain } from "./🔨️modules/📚️library/📦️packages/🟦️typescript/🚨️policy/🟦️.ts";

export const policy = defineLint("repo-technology", (l: TechnologyLinter) => {
  if (l.bundles().length > 0) return [];
  return [
    l.breach({
      id: "no-bundles",
      summary: "Technology has zero bundles",
      kind: "lint/technology/no-bundles",
      priority: "medium",
    }),
  ];
});

await runPolicyOnlyMain(import.meta.url);
