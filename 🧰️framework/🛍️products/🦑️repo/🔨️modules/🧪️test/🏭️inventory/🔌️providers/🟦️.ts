import { lstatSync, readFileSync } from "node:fs";
import { dirname, isAbsolute, relative, resolve, sep } from "node:path";
import { discoverCargoWorkspaces } from "../../../📚️library/🗂️workspaces/🦀️cargo/🟦️.ts";

export type MutationInventoryProviderV1 = { script: string; roots: string[] };

/** 🧬️ Admits the closed owner-authored runtime mutation provider declaration. */
export function admitMutationInventoryProviderV1(value: unknown): MutationInventoryProviderV1 {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw new Error("Invalid mutation inventory provider");
  const row = value as Record<string, unknown>;
  const path = (value: unknown): value is string => typeof value === "string" && value.length > 0 && value.split("/").every(part => part.length > 0 && part !== "." && part !== ".." && !/[\\:\u0000-\u001f]/u.test(part));
  if (Object.keys(row).length !== 2 || !path(row.script) || !Array.isArray(row.roots) || !row.roots.length || row.roots.some(value => value !== "." && !path(value)) || new Set(row.roots).size !== row.roots.length) throw new Error("Invalid closed mutation inventory provider");
  return { script: row.script, roots: [...row.roots] };
}

function physical(root: string, path: string, optional: boolean): boolean {
  const local = relative(root, path);
  if (isAbsolute(local) || local === ".." || local.startsWith(".." + sep)) throw new Error("Mutation inventory provider escapes its defining owner");
  let current = root;
  for (const part of local.split(sep).filter(Boolean)) {
    current = resolve(current, part);
    let info;
    try { info = lstatSync(current); }
    catch (error) { if (optional && (error as NodeJS.ErrnoException).code === "ENOENT") return false; throw error; }
    if (info.isSymbolicLink()) throw new Error("Mutation inventory provider follows a symlink");
  }
  return true;
}

/** 📇️ Discovers explicit runtime providers contributed by present defining workspaces. */
export function discoverMutationInventoryProvidersV1(repoRoot: string): MutationInventoryProviderV1[] {
  return discoverCargoWorkspaces(repoRoot).flatMap(scope => {
    const manifest = resolve(repoRoot, scope.manifest);
    physical(repoRoot, manifest, false);
    const authority = (Bun.TOML.parse(readFileSync(manifest, "utf8")) as any).workspace?.metadata?.semio?.["mutation-inventory"];
    if (authority === undefined) return [];
    const provider = admitMutationInventoryProviderV1(authority), owner = dirname(manifest), script = resolve(owner, provider.script);
    physical(owner, script, false);
    const info = lstatSync(script);
    if (!info.isFile() || info.size > 1024 * 1024) throw new Error("Mutation inventory provider script is not bounded and regular");
    const roots = provider.roots.map(path => resolve(owner, path)).filter(root => {
      if (!physical(owner, root, true)) return false;
      if (!lstatSync(root).isDirectory()) throw new Error("Mutation inventory scope is not a directory");
      return true;
    });
    return roots.length ? [{ script, roots }] : [];
  });
}

/** 🎯️ Resolves one explicit provider and refuses ambiguous ownership. */
export function selectMutationInventoryProviderV1(providers: readonly MutationInventoryProviderV1[], owner: string): MutationInventoryProviderV1 | undefined {
  const selected = providers.filter(provider => provider.roots.some(root => { const local = relative(root, owner); return !isAbsolute(local) && local !== ".." && !local.startsWith(".." + sep); }));
  if (selected.length > 1) throw new Error("Mutation inventory owner has ambiguous providers");
  return selected[0];
}
