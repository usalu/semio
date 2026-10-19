#!/usr/bin/env bun
/** 🧭️ `@semio-tech/repo-client` bundle policy router: `bun ./📜️script.ts policy`. */
import { existsSync } from "node:fs";
import { join } from "node:path";
import { defineLint, type BundleLinter, runPolicyOnlyMain } from "../📚️library/📦️packages/🟦️typescript/🚨️policy/🟦️.ts";
import { getWorkspaceRoot } from "../📚️library/📦️packages/🟦️typescript/📇️inventory/🟦️.ts";

export const policy = defineLint("repo-client-bundle", (l: BundleLinter) => {
  const root = getWorkspaceRoot();
  const manifest = join(root, l.root(), "package.json");
  if (existsSync(manifest)) return [];
  return [
    l.breach({
      id: "missing-package-json",
      summary: "Bundle root is missing package.json",
      kind: "lint/bundle/package-json",
      priority: "medium",
    }),
  ];
});

await runPolicyOnlyMain(import.meta.url);
