import { readdirSync } from "node:fs";
import { join } from "node:path";
import { loadCatalogTaxonomy } from "../../../🔍️discovery/🟦️.ts";

export interface MutationDirectoryView {
  children(root: string, relativePath: string): readonly { readonly name: string; readonly isDirectory: boolean }[];
}

export const MUTATION_DIRECTORY_VIEW: MutationDirectoryView = {
  children(root, relativePath) {
    try {
      return readdirSync(join(root, relativePath), { withFileTypes: true }).map((entry) => ({ name: entry.name, isDirectory: entry.isDirectory() }));
    } catch {
      return [];
    }
  },
};

/** 🔎️ Lists physical direct mutation owners through an injectable no-follow directory view. */
export function policyListMutationDirs(repoRoot: string, mutationsRel: string, view: MutationDirectoryView = MUTATION_DIRECTORY_VIEW): string[] {
  const taxonomy = loadCatalogTaxonomy(),
    domains = taxonomy.mutationDomainOwners[mutationsRel];
  const skipped = new Set(["compose", "node_modules", ".git", ".🧬semio", "target", "dist", "build", "coverage", "🤖️generated", ".claude", "vendor", ".venv", ".turbo", ".nx", ".storybook", "storybook-static"]);
  const children = (relativePath: string) => view.children(repoRoot, relativePath).filter((entry) => entry.isDirectory && !skipped.has(entry.name));
  if (domains)
    return Object.entries(domains)
      .flatMap(([domain, operations]) =>
        children(`${mutationsRel}/${domain}`)
          .filter((entry) => Object.hasOwn(operations, entry.name))
          .map((entry) => `${domain}/${entry.name}`),
      )
      .sort();
  const reserved = new Set<string>(["📚️examples", "💾️binary", "📝️text"]);
  return children(mutationsRel)
    .filter((entry) => !reserved.has(entry.name) && !entry.name.startsWith("."))
    .map((entry) => entry.name)
    .sort();
}
