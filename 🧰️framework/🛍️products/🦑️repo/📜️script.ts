#!/usr/bin/env bun
/** 🧭️ `@repo` technology policy router: `bun ./📜️script.ts policy`. */
import { defineLint, type TechnologyLinter } from "./🔨️modules/📚️library/🚨️policy/🧩️linters/🟦️.ts";
import { runPolicyOnlyMain } from "./🔨️modules/📚️library/🚨️policy/🧭️entrypoints/🟦️.ts";

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
