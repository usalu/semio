import { lstatSync, readFileSync } from "node:fs";
import { dirname, isAbsolute, relative, resolve } from "node:path";
import { getWorkspaceRoot } from "../../🗂️workspaces/🟦️.ts";
import { cargoProviderTomlParser, discoverCatalogPackages, loadCatalogTaxonomy } from "../../🔍️discovery/🟦️.ts";

export type PlaygroundSelection = {
  readonly variant: string;
  readonly pluginId: string;
  readonly cratePath: string;
  readonly aliases: readonly string[];
  readonly ports: { readonly react: number; readonly wgpu: number };
};

/** 🧭️ Resolves public selections from authored manifests before any generated output exists. */
export function loadFrameworkOsPlaygroundSelections(repoRoot = getWorkspaceRoot(), manifestPaths?: readonly string[]): readonly PlaygroundSelection[] {
  const paths = manifestPaths ?? discoverCatalogPackages(repoRoot, loadCatalogTaxonomy()).filter((entry) => entry.lang === "🦀️rust" && ["plugin", "extension"].includes(entry.role)).map((entry) => entry.manifestPath);
  const selections: PlaygroundSelection[] = [], identities = new Set<string>();
  for (const path of paths) {
    const absolute = resolve(repoRoot, path), local = relative(repoRoot, absolute).replaceAll("\\", "/");
    if (isAbsolute(path) || local !== path || local.startsWith("../") || !local.endsWith("/Cargo.toml")) throw new Error(`Playground manifest is outside its source owner: ${path}`);
    for (let node = absolute; node !== resolve(repoRoot); node = dirname(node)) if (lstatSync(node).isSymbolicLink()) throw new Error(`Playground manifest source is a symlink: ${path}`);
    const metadata = (cargoProviderTomlParser.parse(readFileSync(absolute, "utf8")) as { package?: { metadata?: { component?: { package?: string }; semio?: { role?: string; playground?: PlaygroundSelection[] } } } }).package?.metadata;
    if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.role ?? "")) continue;
    if (!/^semio:[a-z0-9]+(?:-[a-z0-9]+)*$/.test(metadata.component.package)) throw new Error(`Invalid playground component identity: ${path}`);
    for (const row of metadata.semio?.playground ?? []) {
      if (typeof row.variant !== "string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(row.variant)) throw new Error(`Invalid playground variant: ${path}`);
      const aliases = row.aliases ?? [];
      if (!Array.isArray(aliases) || aliases.some((alias) => typeof alias !== "string" || !alias.trim())) throw new Error(`Invalid playground aliases: ${path}`);
      for (const identity of [row.variant, ...aliases.filter((alias) => alias !== row.variant)]) {
        if (identities.has(identity)) throw new Error(`Duplicate playground selection: ${identity}`);
        identities.add(identity);
      }
      for (const renderer of ["react", "wgpu"] as const) if (!Number.isSafeInteger(row.ports?.[renderer]) || row.ports[renderer] < 1 || row.ports[renderer] > 65535) throw new Error(`Invalid playground ${renderer} port: ${path}`);
      selections.push({ variant: row.variant, aliases, ports: row.ports, pluginId: metadata.component.package.slice(6), cratePath: dirname(local).replaceAll("\\", "/") });
    }
  }
  return selections.sort((a, b) => a.variant.localeCompare(b.variant));
}

