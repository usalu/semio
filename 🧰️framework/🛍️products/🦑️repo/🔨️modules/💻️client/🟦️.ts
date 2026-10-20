import type { ScriptInvocation } from "@semio-tech/process/routing";
import { existsSync } from "node:fs";
import { join } from "node:path";
import { defineLint, type BundleLinter } from "@semio-tech/repo-lib/policy";
import { getWorkspaceRoot } from "@semio-tech/repo-lib/inventory";

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

