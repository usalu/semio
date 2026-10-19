import type { ScriptInvocation } from "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import { readdirSync, statSync } from "node:fs";
import { join } from "node:path";
import { defineLint, type FolderLinter } from "./🚨️policy/🧩️linters/🟦️.ts";
import { getWorkspaceRoot } from "./🗂️workspaces/🟦️.ts";
import { loadCatalogTaxonomy } from "./🔍️discovery/🔐️taxonomy-authority/🟦️.ts";

export const policy = defineLint("repo-lib-folder", (l: FolderLinter) => {
  const root = getWorkspaceRoot();
  const dir = join(root, l.path());
  const taxonomy = loadCatalogTaxonomy();
  const generated = new Set([taxonomy.schemaExportResolution.catalogPath, taxonomy.schemaExportResolution.catalogDocumentPath].map((path) => path.slice(path.lastIndexOf("/") + 1)));
  const big: string[] = [];
  for (const name of readdirSync(dir, { withFileTypes: true })) {
    if (!name.isFile() || generated.has(name.name)) continue;
    const p = join(dir, name.name);
    if (statSync(p).size > 1 << 20) big.push(name.name);
  }
  if (big.length === 0) return [];
  return [
    l.breach({
      id: "big-child",
      summary: `Authored child files exceed 1 MiB: ${big.join(", ")}`,
      kind: "lint/folder/child-size",
      priority: "medium",
    }),
  ];
});

