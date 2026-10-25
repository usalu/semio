import { createHash } from "node:crypto";
import { existsSync, readFileSync, realpathSync, statSync } from "node:fs";
import { basename, dirname, extname, join, relative, resolve } from "node:path";
import type { SemanticCollectionSpec, SemanticKind, SemanticMember, SemanticOwnerLevel } from "../📜️taxonomy-contracts/🟦️.ts";
import type { Taxonomy } from "../📑️taxonomy-document/🟦️.ts";
import { CARGO_TARGET_DIR_PATTERN, type DiscoveredPackage, readdirSafe, stripEmoji } from "../🪧️owner-markers/🟦️.ts";
import { exactContractFilename, fixedContractFilename } from "../🎯️taxonomy-path-matching/🟦️.ts";
import { loadTaxonomy, pathIsExcluded, validateTaxonomy } from "../🔐️taxonomy-authority/🟦️.ts";
import { discoverPackageProblems, discoverPackages } from "../🔭️repository-scan/🟦️.ts";
import { componentFilenames } from "../⚖️fixed-contract-resolution/🟦️.ts";
import { fileKindIdForFilename, semanticManifestFilenameForCollection } from "../🗂️file-kind-resolution/🟦️.ts";
import { runtimeIdsV1 } from "../../../../../../📦️packages/🟦️typescript/⚙️runtime/🏷️identity/🟦️.ts";

/** 🔗️ One resolved source dependency between repository-owned semantic components. */
export interface SemanticConsumerEdge {
  readonly from: string;
  readonly to: string;
  readonly source: string;
  readonly target: string;
  readonly mechanism: "static-import" | "path-attribute" | "project-reference" | "runtime-registration";
  readonly production: boolean;
}

/** 🕸️ Deterministic component graph used to prove production independence and module ownership. */
export interface SemanticConsumerGraph {
  readonly nodes: readonly string[];
  readonly edges: readonly SemanticConsumerEdge[];
}

/** 📋️ Structured semantic-policy finding; report and enforce consume the same records. */
export interface SemanticProblem {
  readonly code: string;
  readonly severity: "error" | "warning";
  readonly path: string;
  readonly componentId?: string;
  readonly message: string;
}

/** 📊️ Deterministic census row for one maximally specific semantic component. */
export interface SemanticCensusRecord {
  readonly id: string;
  readonly currentPath: string;
  readonly collectionPath: string;
  readonly kind: SemanticKind;
  readonly responsibility: string;
  readonly ownerAncestry: readonly string[];
  readonly languageMirrors: readonly string[];
  readonly packages: readonly string[];
  readonly provenance: "authored" | "generated" | "vendor" | "test" | "example";
  readonly publicSymbols: readonly string[];
  readonly schemaContracts: readonly string[];
  readonly staticImports: readonly string[];
  readonly runtimeMounts: readonly string[];
  readonly registrations: readonly string[];
  readonly packageEntrypoints: readonly string[];
  readonly reverseDependencies: readonly string[];
  readonly productionConsumers: readonly string[];
  readonly excludedConsumers: readonly string[];
  readonly currentOwner: string;
  readonly computedLowestCommonOwner: string | null;
  readonly proposedDisposition: "retain" | "split" | "inline" | "promote" | "relocate" | "regenerate" | "delete";
  readonly duplicateClusters: readonly string[];
  readonly applicableInstructions: readonly string[];
  readonly dirtyConflicts: readonly string[];
  readonly generatorInputs: readonly string[];
  readonly tests: readonly string[];
  readonly runtimeSurfaces: readonly string[];
  readonly leaseId: string | null;
}

/** 🧬️ Syntax-duplicate evidence; it never implies semantic equivalence or extraction. */
export interface SemanticDuplicateCluster {
  readonly id: string;
  readonly hash: string;
  readonly componentIds: readonly string[];
  readonly paths: readonly string[];
}

/** 🧰️ Complete deterministic semantic inventory and its validation graph. */
export interface SemanticCensus {
  readonly records: readonly SemanticCensusRecord[];
  readonly graph: SemanticConsumerGraph;
  readonly problems: readonly SemanticProblem[];
  readonly duplicates: readonly SemanticDuplicateCluster[];
}

/** 🦀️ A cumulative Rust `#[path]` resolution from one entry source. */
export interface RustResolvedPath {
  readonly specifier: string;
  readonly target: string;
}

interface SemanticSource {
  readonly abs: string;
  readonly rel: string;
  readonly content: string;
  readonly production: boolean;
}

interface SemanticRecordDraft {
  readonly id: string;
  readonly currentPath: string;
  readonly collectionPath: string;
  readonly collectionDirectory: string;
  readonly kind: SemanticKind;
  readonly responsibility: string;
  readonly member?: SemanticMember;
  readonly sourceFiles: readonly SemanticSource[];
  readonly currentOwner: string;
  readonly ownerAncestry: readonly string[];
}

interface SemanticManifestExtension {
  readonly kind: "collection";
  readonly members: readonly SemanticMember[];
}

interface SemanticResolverIndex {
  readonly packageRoots: ReadonlyMap<string, string>;
  readonly packageExports: ReadonlyMap<string, ReadonlyMap<string, string>>;
  readonly goModules: ReadonlyMap<string, string>;
  readonly pythonRoots: readonly string[];
  readonly tsPaths: readonly { readonly root: string; readonly pattern: string; readonly targets: readonly string[] }[];
}

const SEMANTIC_SKIP_DIRS = new Set(["node_modules", "target", "dist", ".git", ".nx", ".cache", "vendor", "pkg", "storybook-static", "temp"]);

const SEMANTIC_NON_PRODUCTION_SEGMENTS = new Set(["🧪️tests", "tests", "test", "__tests__", "📚️examples", "🧪️examples", "examples", "fixtures", "🧫️fixtures", "🤖️generated"]);

function semanticCompare(a: string, b: string): number {
  return a < b ? -1 : a > b ? 1 : 0;
}

function semanticUnique(values: Iterable<string>): string[] {
  return [...new Set(values)].sort(semanticCompare);
}

function semanticRel(repoRoot: string, path: string): string {
  return relative(repoRoot, path).replaceAll("\\", "/");
}

function semanticProductionPath(path: string): boolean {
  return !path.split("/").some((segment) => SEMANTIC_NON_PRODUCTION_SEGMENTS.has(segment) || /^\./u.test(segment));
}

function semanticProvenance(path: string): SemanticCensusRecord["provenance"] {
  const segments = path.split("/");
  if (segments.some((segment) => segment === "node_modules" || segment === "vendor")) return "vendor";
  if (segments.some((segment) => segment === "🤖️generated" || segment === "generated" || segment === "dist" || segment === "target")) return "generated";
  if (segments.some((segment) => segment === "🧪️tests" || segment === "tests" || segment === "test" || segment === "__tests__")) return "test";
  if (segments.some((segment) => segment === "📚️examples" || segment === "🧪️examples" || segment === "examples")) return "example";
  return "authored";
}

function semanticSourceExtensions(taxonomy: Taxonomy): Set<string> {
  const sourceKinds = Object.values(taxonomy.fileKinds).filter((kind) => kind.role === "source");
  return new Set([...sourceKinds.flatMap((kind) => kind.extensionChains.map((chain) => extname(`source${chain}`))), ".c", ".cc", ".cpp", ".h", ".hpp", ".csproj"]);
}

function semanticWalk(repoRoot: string, root: string, taxonomy: Taxonomy): string[] {
  const files: string[] = [];
  const visited = new Set<string>();
  const walk = (dir: string): void => {
    if (pathIsExcluded(repoRoot, dir, taxonomy)) return;
    let real: string;
    try {
      real = realpathSync(dir);
    } catch {
      return;
    }
    if (visited.has(real)) return;
    visited.add(real);
    for (const entry of readdirSafe(real).sort((a, b) => semanticCompare(a.name, b.name))) {
      const path = join(real, entry.name);
      if (pathIsExcluded(repoRoot, path, taxonomy)) continue;
      if (entry.isDirectory()) {
        if (!entry.name.startsWith(".") && !SEMANTIC_SKIP_DIRS.has(entry.name) && !CARGO_TARGET_DIR_PATTERN.test(entry.name)) walk(path);
      } else if (entry.isFile()) files.push(path);
    }
  };
  walk(root);
  return files.sort(semanticCompare);
}

function semanticCollectionAncestors(repoRoot: string, file: string, taxonomy: Taxonomy): string[] {
  const ancestors: string[] = [];
  let current = dirname(file);
  while (current.startsWith(repoRoot) && current !== repoRoot) {
    if (semanticCollectionSpec(current, taxonomy)) ancestors.push(current);
    current = dirname(current);
  }
  return ancestors;
}

/** 🧭️ Chooses the most-specific declared collection suffix for one on-disk collection root. */
function semanticCollectionSpec(path: string, taxonomy: Taxonomy): SemanticCollectionSpec | null {
  const segments = path.replaceAll("\\", "/").split("/").filter(Boolean);
  for (const [key, spec] of Object.entries(taxonomy.semanticCollections).sort(([a], [b]) => b.split("/").length - a.split("/").length || semanticCompare(a, b))) {
    const suffix = key.split("/");
    if (suffix.length <= segments.length && suffix.every((segment, index) => segments[segments.length - suffix.length + index] === segment)) return spec;
  }
  return null;
}

/** 🗺️ Every declared clean area root, with opaque prefixes filtered before existence checks. */
export function semanticActiveRoots(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const active = Object.entries(taxonomy.areas)
    .map(([path]) => path)
    .filter((path) => !pathIsExcluded(repoRoot, join(repoRoot, path), taxonomy))
    .filter((path) => existsSync(join(repoRoot, path)))
    .sort((a, b) => a.split("/").length - b.split("/").length || semanticCompare(a, b));
  return active.filter((path, index) => !active.some((candidate, other) => other < index && (path === candidate || path.startsWith(`${candidate}/`))));
}

function semanticOwnerAncestry(path: string): string[] {
  const segments = path.split("/").filter(Boolean);
  const owners: string[] = [];
  if (segments[0] === "🧰️framework") owners.push(segments[0]);
  if (segments[0] === "✏️s") owners.push(segments[0]);
  const collections = new Set(["🔌️plugins", "🛍️products", "🎛️apps", "🗿️artifacts", "🏅️standards", "🪆️subsets"]);
  for (let index = 0; index < segments.length - 1; index += 1) {
    if (collections.has(segments[index]!)) owners.push(segments.slice(0, index + 2).join("/"));
  }
  return semanticUnique(owners).sort((a, b) => a.split("/").length - b.split("/").length || semanticCompare(a, b));
}

function semanticOwnerLevel(path: string): SemanticOwnerLevel | null {
  const segments = path.split("/");
  const parent = segments.at(-2);
  if (parent === "🪆️subsets") return "subset";
  if (parent === "🏅️standards") return "standard";
  if (parent === "🗿️artifacts") return "artifact";
  if (parent === "🎛️apps") return "app";
  if (parent === "🔌️plugins") return "plugin";
  if (parent === "🛍️products") return "product";
  if (path === "✏️s") return "s";
  if (path === "🧰️framework") return "framework";
  return null;
}

function semanticLowestCommonOwner(records: readonly SemanticRecordDraft[]): string | null {
  if (records.length === 0) return null;
  const common = records[0]!.ownerAncestry.filter((owner) => records.every((record) => record.ownerAncestry.includes(owner)));
  return common.sort((a, b) => b.split("/").length - a.split("/").length || semanticCompare(a, b))[0] ?? null;
}

function semanticReadManifest(path: string, taxonomy: Taxonomy, problems: SemanticProblem[], collectionPath: string): SemanticManifestExtension | null {
  const filename = basename(path);
  if (!existsSync(path)) {
    problems.push({ code: "collection-manifest-missing", severity: "error", path: collectionPath, message: `Collection is missing canonical ${filename}.` });
    return null;
  }
  try {
    const parsed = JSON.parse(readFileSync(path, "utf8")) as Record<string, unknown>;
    const extension = parsed[taxonomy.semanticExtensionKey] as Partial<SemanticManifestExtension> | undefined;
    if (!extension || extension.kind !== "collection" || !Array.isArray(extension.members)) {
      problems.push({ code: "collection-manifest-shape", severity: "error", path: semanticRel(dirname(dirname(path)), path), message: `${taxonomy.semanticExtensionKey} must be { kind: "collection", members: [...] }.` });
      return null;
    }
    return extension as SemanticManifestExtension;
  } catch (error) {
    problems.push({ code: "collection-manifest-invalid", severity: "error", path: collectionPath, message: `${filename} is not valid JSON: ${(error as Error).message}` });
    return null;
  }
}

function semanticMemberProblems(member: SemanticMember, spec: SemanticCollectionSpec, collectionPath: string, taxonomy: Taxonomy): SemanticProblem[] {
  const path = `${collectionPath}/${member.directory}`;
  const problems: SemanticProblem[] = [];
  const add = (code: string, message: string): void => {
    problems.push({ code, severity: "error", path, componentId: member.id, message });
  };
  if (!member.directory || member.directory.includes("*") || member.id.includes("*")) add("member-wildcard", "Member directory and id must be exact, non-wildcard values.");
  if (!member.id.trim()) add("member-id-empty", "Member id must be non-empty.");
  if (!member.responsibility?.trim()) add("member-responsibility-empty", "Member responsibility must be specific and non-empty.");
  if (member.kind !== spec.kind) add("member-kind-mismatch", `Member kind ${JSON.stringify(member.kind)} does not match collection kind ${JSON.stringify(spec.kind)}.`);
  if (member.kind === "inference" && (!member.inference || member.inference.inputs.length === 0 || !member.inference.target.trim())) add("inference-contract-missing", "Inference must declare non-empty inputs and one derived target.");
  if (member.kind === "mutation" && (!member.mutation?.command.trim() || !member.mutation.event.trim())) add("mutation-contract-missing", "Mutation must declare its command and emitted event.");
  if (member.kind === "io" && (!member.io?.format.trim() || !member.io.direction || member.io.direction !== spec.direction)) add("io-contract-missing", `I/O member must declare a format and direction ${JSON.stringify(spec.direction)}.`);
  if (member.kind === "module") {
    const consumers = semanticUnique(member.module?.productionConsumers ?? []);
    if (consumers.length < taxonomy.semanticConsumerMinimum) add("module-consumer-minimum", `Module declares ${consumers.length} independent production consumers; at least ${taxonomy.semanticConsumerMinimum} are required.`);
  }
  const stem = stripEmoji(member.directory).toLowerCase();
  if (taxonomy.bannedNameStems.includes(stem)) add("member-generic-stem", `Specific member uses banned generic stem ${JSON.stringify(stem)}.`);
  return problems;
}

function semanticAssemblyOnly(content: string, extension: string): boolean {
  const lines = content.split(/\r?\n/u).map((line) => line.trim()).filter((line) => line && !/^(\/\/|\/\*|\*|#region|#endregion|\/\/#[a-z])/u.test(line));
  if (extension === ".rs") return lines.every((line) => /^(#\[path\s*=|(?:pub\s+)?mod\s+\w+\s*(?:;|\{)|pub\s+use\s+|[\w:]+!\(|[)};,]+$)/u.test(line));
  if (extension === ".ts" || extension === ".tsx" || extension === ".js" || extension === ".jsx") return lines.every((line) => /^(import\s|export\s(?:\{|\*)|[};,]+$)/u.test(line));
  if (extension === ".py") return lines.every((line) => /^(from\s|import\s|__all__\s*=|[\[\],]+$)/u.test(line));
  return lines.length === 0;
}

/** 🧷️ Mechanical glue and collection assembly establish reachability but never qualify as a production consumer. */
function semanticProductionConsumer(source: SemanticSource, packages: readonly DiscoveredPackage[]): boolean {
  return source.production && !packages.some((pkg) => source.rel.startsWith(`${pkg.packageRel}/`)) && !semanticAssemblyOnly(source.content, extname(source.abs));
}

function semanticPublicSymbols(source: SemanticSource): string[] {
  const symbols: string[] = [];
  const patterns = source.rel.endsWith(".rs")
    ? [/\bpub\s+(?:struct|enum|trait|type|fn|const|static|mod)\s+([A-Za-z_][A-Za-z0-9_]*)/gu]
    : source.rel.endsWith(".go")
      ? [/\b(?:type|func|const|var)\s+([A-Z][A-Za-z0-9_]*)/gu]
      : source.rel.endsWith(".py")
        ? [/^class\s+([A-Za-z_][A-Za-z0-9_]*)/gmu, /^def\s+([A-Za-z_][A-Za-z0-9_]*)/gmu]
        : source.rel.endsWith(".cs")
          ? [/\bpublic\s+(?:class|record|struct|interface|enum)\s+([A-Za-z_][A-Za-z0-9_]*)/gu]
          : [/\bexport\s+(?:default\s+)?(?:class|interface|type|enum|function|const|let|var)\s+([A-Za-z_$][A-Za-z0-9_$]*)/gu];
  for (const pattern of patterns) for (const match of source.content.matchAll(pattern)) if (match[1]) symbols.push(match[1]);
  return semanticUnique(symbols);
}

function semanticImportSpecs(source: SemanticSource): string[] {
  const specs: string[] = [];
  const patterns = source.rel.endsWith(".rs")
    ? [/#\[path\s*=\s*"([^"]+)"\]/gu]
    : source.rel.endsWith(".py")
      ? [/^from\s+(\.+[A-Za-z0-9_.]+)\s+import/gmu]
      : source.rel.endsWith(".csproj")
        ? [/<ProjectReference\s+Include="([^"]+)"/gu]
        : source.rel.endsWith(".go")
          ? [/"([^"\n]+)"/gu]
          : [/(?:import|export)\s+(?:[^"']*?\s+from\s+)?["']([^"']+)["']/gu, /import\s*\(\s*["']([^"']+)["']\s*\)/gu, /require\s*\(\s*["']([^"']+)["']\s*\)/gu];
  for (const pattern of patterns) for (const match of source.content.matchAll(pattern)) if (match[1]) specs.push(match[1]);
  return semanticUnique(specs);
}

/** 📕️ Relative Rust namespaces are imports too; they must resolve to their semantic member, not a crate barrel. */
function semanticRustUseSpecs(source: SemanticSource): string[] {
  if (!source.rel.endsWith(".rs")) return [];
  const specs: string[] = [];
  for (const match of source.content.matchAll(/\b(?:pub\s+)?use\s+((?:super|self)(?:::[^;]+)+)\s*;/gu)) if (match[1]) specs.push(match[1].replace(/\s+/gu, " ").trim());
  return semanticUnique(specs);
}

/** 📇️ Resolves a logical Rust namespace to its closest physical taxonomy directory. */
function semanticRustNamespaceDirectory(base: string, segment: string): string | null {
  let current = base;
  for (;;) {
    const child = readdirSafe(current).find((entry) => entry.isDirectory() && stripEmoji(entry.name).replaceAll("-", "_") === segment);
    if (child) return join(current, child.name);
    if (segment !== "modules") return null;
    const parent = dirname(current);
    if (parent === current) return null;
    current = parent;
  }
}

function semanticJson(path: string): Record<string, unknown> | null {
  try {
    const content = readFileSync(path, "utf8").replace(/\/\*[\s\S]*?\*\//gu, "").replace(/(^|\s)\/\/.*$/gmu, "$1").replace(/,\s*([}\]])/gu, "$1");
    return JSON.parse(content) as Record<string, unknown>;
  } catch {
    return null;
  }
}

function semanticFlattenExports(value: unknown, prefix = ".", result = new Map<string, string>()): ReadonlyMap<string, string> {
  if (typeof value === "string") result.set(prefix, value);
  else if (value && typeof value === "object") {
    for (const [key, child] of Object.entries(value as Record<string, unknown>)) {
      if (key.startsWith(".")) semanticFlattenExports(child, key, result);
      else if (["import", "default", "types", ...runtimeIdsV1()].includes(key) && !result.has(prefix)) semanticFlattenExports(child, prefix, result);
    }
  }
  return result;
}

function semanticResolverIndex(allFiles: readonly string[], taxonomy: Taxonomy): SemanticResolverIndex {
  const packageRoots = new Map<string, string>();
  const packageExports = new Map<string, ReadonlyMap<string, string>>();
  const goModules = new Map<string, string>();
  const pythonRoots: string[] = [];
  const tsPaths: { readonly root: string; readonly pattern: string; readonly targets: readonly string[] }[] = [];
  const nodeManifest = exactContractFilename(taxonomy.ecosystems["🟦️typescript"]?.manifestContractId ?? null, taxonomy);
  const goManifest = exactContractFilename(taxonomy.ecosystems["🐹️go"]?.moduleRootContractId ?? null, taxonomy);
  const pythonManifest = exactContractFilename(taxonomy.ecosystems["🐍️python"]?.manifestContractId ?? null, taxonomy);
  const tsConfigContract = taxonomy.fixedFilenameContracts["typescript-config"];
  const tsConfig = tsConfigContract ? fixedContractFilename(tsConfigContract) : undefined;
  const defaultTypescriptEntry = taxonomy.ecosystems["🟦️typescript"]?.entryContractIds.map((id) => exactContractFilename(id, taxonomy)).find((filename): filename is string => Boolean(filename));
  for (const file of allFiles) {
    if (nodeManifest && basename(file) === nodeManifest) {
      const manifest = semanticJson(file);
      if (typeof manifest?.name === "string") {
        packageRoots.set(manifest.name, dirname(file));
        packageExports.set(manifest.name, semanticFlattenExports(manifest.exports ?? manifest.module ?? manifest.main ?? (defaultTypescriptEntry ? `./${defaultTypescriptEntry}` : undefined)));
      }
    } else if (goManifest && basename(file) === goManifest) {
      const module = readFileSync(file, "utf8").match(/^module\s+(\S+)/mu)?.[1];
      if (module) goModules.set(module, dirname(file));
    } else if (pythonManifest && basename(file) === pythonManifest) {
      pythonRoots.push(dirname(file));
    } else if (tsConfig && basename(file) === tsConfig) {
      const config = semanticJson(file);
      const compiler = config?.compilerOptions as Record<string, unknown> | undefined;
      const base = resolve(dirname(file), typeof compiler?.baseUrl === "string" ? compiler.baseUrl : ".");
      if (compiler?.paths && typeof compiler.paths === "object") {
        for (const [pattern, targets] of Object.entries(compiler.paths as Record<string, unknown>)) if (Array.isArray(targets)) tsPaths.push({ root: base, pattern, targets: targets.filter((target): target is string => typeof target === "string") });
      }
    }
  }
  return { packageRoots, packageExports, goModules, pythonRoots: semanticUnique(pythonRoots), tsPaths: tsPaths.sort((a, b) => b.root.length - a.root.length || semanticCompare(a.pattern, b.pattern)) };
}

function semanticRuntimeEvidence(source: SemanticSource, pattern: RegExp): string[] {
  const evidence: string[] = [];
  for (const [index, line] of source.content.split(/\r?\n/u).entries()) if (pattern.test(line)) evidence.push(`${source.rel}:${index + 1}`);
  return evidence;
}

/** 🪆️ Resolves nested Rust path attributes with the enclosing module's cumulative base. */
export function resolveRustPathAttributes(sourcePath: string, content: string): RustResolvedPath[] {
  const resolved: RustResolvedPath[] = [];
  const scopes: { readonly base: string; readonly depth: number }[] = [{ base: dirname(sourcePath), depth: 0 }];
  let depth = 0;
  let pending: string | null = null;
  for (const line of content.split(/\r?\n/u)) {
    const pathMatch = line.match(/#\[path\s*=\s*"([^"]+)"\]/u);
    if (pathMatch?.[1]) pending = pathMatch[1];
    const moduleMatch = line.match(/(?:pub\s+)?mod\s+([A-Za-z_][A-Za-z0-9_]*)\s*([;{])/u);
    const base = scopes.at(-1)!.base;
    if (moduleMatch) {
      const name = moduleMatch[1]!;
      if (moduleMatch[2] === ";") {
        const specifier = pending ?? `${name}.rs`;
        resolved.push({ specifier, target: resolve(base, specifier) });
      } else {
        scopes.push({ base: resolve(base, pending ?? name), depth: depth + 1 });
      }
      pending = null;
    }
    depth += (line.match(/\{/gu) ?? []).length - (line.match(/\}/gu) ?? []).length;
    while (scopes.length > 1 && scopes.at(-1)!.depth > depth) scopes.pop();
  }
  return resolved.sort((a, b) => semanticCompare(a.target, b.target));
}

/** 🗂️ Resolves `use super::…` through emoji-prefixed sibling directories to immediate semantic component leaves. */
function resolveRustRelativeUses(source: SemanticSource, componentRoot: string, componentLeaves: ReadonlyMap<string, string>): RustResolvedPath[] {
  const resolved: RustResolvedPath[] = [];
  for (const specifier of semanticRustUseSpecs(source)) {
    const segments = specifier.split("::").map((segment) => segment.trim()).filter(Boolean);
    let index = 0;
    let base = componentRoot;
    if (segments[index] === "self") index += 1;
    else {
      while (segments[index] === "super") {
        base = dirname(base);
        index += 1;
      }
      if (index === 0) continue;
    }
    const tail = segments.slice(index).join("::");
    const braceAt = tail.indexOf("{");
    const path = (braceAt < 0 ? tail : tail.slice(0, braceAt)).replace(/::$/u, "");
    for (const segment of path.split("::").map((part) => part.trim()).filter(Boolean)) {
      const child = semanticRustNamespaceDirectory(base, segment);
      if (!child) break;
      base = child;
      const target = componentLeaves.get(base);
      if (target) {
        resolved.push({ specifier, target });
        break;
      }
    }
    if (braceAt >= 0) {
      for (const candidate of tail.slice(braceAt).matchAll(/[a-z][A-Za-z0-9_]*/gu)) {
        const child = readdirSafe(base).find((entry) => entry.isDirectory() && stripEmoji(entry.name).replaceAll("-", "_") === candidate[0]);
        if (!child) continue;
        const target = componentLeaves.get(join(base, child.name));
        if (target) resolved.push({ specifier, target });
      }
    }
  }
  return [...new Map(resolved.map((target) => [`${target.specifier}\0${target.target}`, target])).values()].sort((a, b) => semanticCompare(`${a.specifier}\0${a.target}`, `${b.specifier}\0${b.target}`));
}

function semanticResolveCandidate(from: SemanticSource, specifier: string, fileIndex: ReadonlyMap<string, string>, extensions: ReadonlySet<string>, resolvers: SemanticResolverIndex, taxonomy: Taxonomy): string | null {
  let normalized = specifier.replace(/[?#].*$/u, "");
  if (from.rel.endsWith(".py") && normalized.startsWith(".")) normalized = normalized.replace(/^\.+/u, ".").replaceAll(".", "/");
  const bases: string[] = [];
  if (normalized.startsWith(".") || normalized.startsWith("/")) bases.push(resolve(dirname(from.abs), normalized));
  else {
    for (const [name, root] of [...resolvers.packageRoots.entries()].sort((a, b) => b[0].length - a[0].length || semanticCompare(a[0], b[0]))) {
      if (normalized !== name && !normalized.startsWith(`${name}/`)) continue;
      const subpath = normalized === name ? "." : `./${normalized.slice(name.length + 1)}`;
      const defaultEntry = taxonomy.ecosystems["🟦️typescript"]?.entryContractIds.map((id) => exactContractFilename(id, taxonomy)).find((filename): filename is string => Boolean(filename));
      const target = resolvers.packageExports.get(name)?.get(subpath) ?? (subpath === "." && defaultEntry ? `./${defaultEntry}` : subpath);
      bases.push(resolve(root, target));
    }
    for (const [name, root] of [...resolvers.goModules.entries()].sort((a, b) => b[0].length - a[0].length || semanticCompare(a[0], b[0]))) if (normalized === name || normalized.startsWith(`${name}/`)) bases.push(resolve(root, normalized.slice(name.length).replace(/^\//u, "")));
    for (const mapping of resolvers.tsPaths) {
      if (!from.abs.startsWith(`${mapping.root}/`) && !from.abs.startsWith(`${dirname(mapping.root)}/`)) continue;
      const star = mapping.pattern.indexOf("*");
      const captured = star < 0 ? (normalized === mapping.pattern ? "" : null) : normalized.startsWith(mapping.pattern.slice(0, star)) && normalized.endsWith(mapping.pattern.slice(star + 1)) ? normalized.slice(star, normalized.length - mapping.pattern.slice(star + 1).length) : null;
      if (captured === null) continue;
      for (const target of mapping.targets) bases.push(resolve(mapping.root, target.replace("*", captured)));
    }
    if (from.rel.endsWith(".py")) for (const root of resolvers.pythonRoots.filter((root) => from.abs.startsWith(`${root}/`))) bases.push(resolve(root, normalized.replaceAll(".", "/")));
  }
  const candidates = [...bases];
  for (const base of bases) {
    for (const extension of extensions) candidates.push(`${base}${extension}`);
    for (const filename of componentFilenames(taxonomy)) candidates.push(join(base, filename));
    for (const contract of Object.values(taxonomy.configurableEntryContracts)) candidates.push(join(base, contract.filename));
  }
  for (const candidate of candidates) {
    let real = candidate;
    try {
      if (existsSync(candidate) && statSync(candidate).isFile()) real = realpathSync(candidate);
    } catch {
      continue;
    }
    const indexed = fileIndex.get(real);
    if (indexed) return indexed;
  }
  return null;
}

function semanticInstructions(repoRoot: string, componentPath: string): string[] {
  const instructions: string[] = [];
  let current = join(repoRoot, componentPath);
  while (current.startsWith(repoRoot)) {
    const candidate = join(current, "AGENTS.md");
    if (existsSync(candidate)) instructions.push(semanticRel(repoRoot, candidate));
    if (current === repoRoot) break;
    current = dirname(current);
  }
  return instructions.reverse();
}

function semanticNormalizeDuplicate(content: string): string {
  return content.replace(/\/\*[\s\S]*?\*\//gu, "").replace(/(^|\s)\/\/.*$/gmu, "$1").replace(/(^|\s)#(?!\[).*$/gmu, "$1").replace(/\s+/gu, "").trim();
}

function semanticDisposition(kind: SemanticKind, productionConsumers: readonly string[], currentOwner: string, lca: string | null): SemanticCensusRecord["proposedDisposition"] {
  if (kind !== "module") return "retain";
  if (productionConsumers.length === 0) return "delete";
  if (productionConsumers.length === 1) return "inline";
  return lca === currentOwner ? "retain" : "relocate";
}

/** 🖥️ Follows reverse module edges until independent non-module production components; intermediary modules never qualify. */
function semanticTerminalProductionConsumers(componentId: string, edges: readonly SemanticConsumerEdge[], drafts: ReadonlyMap<string, SemanticRecordDraft>): string[] {
  const incoming = new Map<string, SemanticConsumerEdge[]>();
  for (const edge of edges) incoming.set(edge.to, [...(incoming.get(edge.to) ?? []), edge]);
  const terminals = new Set<string>();
  const visited = new Set<string>([componentId]);
  const visit = (target: string): void => {
    for (const edge of incoming.get(target) ?? []) {
      if (!edge.production || visited.has(edge.from)) continue;
      visited.add(edge.from);
      if (drafts.get(edge.from)?.kind === "module") visit(edge.from);
      else terminals.add(edge.from);
    }
  };
  visit(componentId);
  return semanticUnique(terminals);
}

function semanticScopeMatchesId(id: string, scope: string): boolean {
  return id === scope || id.startsWith(`${scope}.`);
}

function semanticCommonPath(paths: readonly string[]): string | null {
  const [first, ...remaining] = paths.map((path) => path.split("/").filter(Boolean));
  if (!first) return null;
  const common = first.filter((segment, index) => remaining.every((candidate) => candidate[index] === segment));
  return common.length === 0 ? null : common.join("/");
}

/** 🌳️ Resolves a semantic-id scope to its real owner boundary so unclassified collection findings remain visible. */
function semanticScopeRoots(records: readonly SemanticCensusRecord[], scope: string): string[] {
  const matched = records.filter((record) => semanticScopeMatchesId(record.id, scope) || record.currentPath === scope || record.currentPath.startsWith(`${scope}/`));
  if (matched.length === 0) return [];
  const ownerName = scope.split(".").filter(Boolean).at(-1);
  const ownerPaths = ownerName ? matched.flatMap((record) => record.ownerAncestry.filter((owner) => stripEmoji(basename(owner)) === ownerName)) : [];
  const root = semanticCommonPath(ownerPaths.length > 0 ? ownerPaths : matched.map((record) => record.currentPath));
  return root ? [root] : [];
}

function semanticPathInRoots(path: string, roots: readonly string[]): boolean {
  return roots.some((root) => path === root || path.startsWith(`${root}/`));
}

/** 🔨️ Builds the timestamp-free semantic census from taxonomy-defined active scope. */
export function buildSemanticCensus(repoRoot: string, options: { readonly scope?: string } = {}, taxonomy: Taxonomy = loadTaxonomy()): SemanticCensus {
  repoRoot = realpathSync(repoRoot);
  const problems: SemanticProblem[] = validateTaxonomy(taxonomy).map((message) => ({ code: "taxonomy-schema", severity: "error", path: "📋️project.json#metadata.semio.taxonomy", message }));
  for (const pkgProblem of discoverPackageProblems(repoRoot, taxonomy)) {
    problems.push({
      code: pkgProblem.kind,
      severity: "error",
      path: pkgProblem.path,
      message: pkgProblem.message,
    });
  }
  const extensions = semanticSourceExtensions(taxonomy);
  const allFiles = semanticActiveRoots(repoRoot, taxonomy).flatMap((active) => semanticWalk(repoRoot, join(repoRoot, active), taxonomy));
  const sourceFiles: SemanticSource[] = allFiles
    .filter((path) => extensions.has(extname(path)))
    .map((abs) => ({ abs: realpathSync(abs), rel: semanticRel(repoRoot, abs), content: readFileSync(abs, "utf8"), production: semanticProductionPath(semanticRel(repoRoot, abs)) }))
    .sort((a, b) => semanticCompare(a.rel, b.rel));
  const collectionDirs = semanticUnique(allFiles.flatMap((file) => semanticCollectionAncestors(repoRoot, file, taxonomy)).map((dir) => realpathSync(dir)));
  const packages = discoverPackages(repoRoot, taxonomy);
  const drafts: SemanticRecordDraft[] = [];
  for (const collectionAbs of collectionDirs) {
    const collectionPath = semanticRel(repoRoot, collectionAbs);
    const collectionDirectory = basename(collectionAbs);
    const spec = semanticCollectionSpec(collectionAbs, taxonomy)!;
    const manifestFilename = semanticManifestFilenameForCollection(collectionPath, taxonomy);
    const manifest = semanticReadManifest(join(collectionAbs, manifestFilename), taxonomy, problems, collectionPath);
    const actualChildren = readdirSafe(collectionAbs)
      .filter((entry) => entry.isDirectory() && !entry.name.startsWith(".") && !SEMANTIC_SKIP_DIRS.has(entry.name) && !CARGO_TARGET_DIR_PATTERN.test(entry.name) && entry.name !== taxonomy.packagesDirName && entry.name !== "🤖️generated")
      .map((entry) => entry.name)
      .sort(semanticCompare);
    const declaredMembers = manifest?.members ?? [];
    const declaredDirs = declaredMembers.map((member) => member.directory);
    if (actualChildren.length === 0) problems.push({ code: "collection-empty", severity: "error", path: collectionPath, message: "Semantic collection has no specific members." });
    for (const duplicate of semanticUnique(declaredDirs.filter((directory, index) => declaredDirs.indexOf(directory) !== index))) problems.push({ code: "member-directory-duplicate", severity: "error", path: collectionPath, message: `Manifest declares directory ${JSON.stringify(duplicate)} more than once.` });
    const ids = declaredMembers.map((member) => member.id);
    for (const duplicate of semanticUnique(ids.filter((id, index) => ids.indexOf(id) !== index))) problems.push({ code: "member-id-duplicate", severity: "error", path: collectionPath, message: `Manifest declares semantic id ${JSON.stringify(duplicate)} more than once.` });
    for (const directory of actualChildren.filter((directory) => !declaredDirs.includes(directory))) problems.push({ code: "manifest-child-missing", severity: "error", path: `${collectionPath}/${directory}`, message: `Direct child is not declared in ${manifestFilename}.` });
    for (const directory of declaredDirs.filter((directory) => !actualChildren.includes(directory))) problems.push({ code: "manifest-child-extra", severity: "error", path: `${collectionPath}/${directory}`, message: `Manifest member has no exact child directory.` });
    for (const member of declaredMembers) problems.push(...semanticMemberProblems(member, spec, collectionPath, taxonomy));
    const rootSources = sourceFiles.filter((source) => dirname(source.abs) === collectionAbs);
    for (const source of rootSources) if (!semanticAssemblyOnly(source.content, extname(source.abs))) problems.push({ code: "collection-authored-behavior", severity: "error", path: source.rel, message: "Collection language leaf contains authored behavior; list roots may contain generated/mechanical assembly only." });
    for (const directory of actualChildren) {
      const currentPath = `${collectionPath}/${directory}`;
      const member = declaredMembers.find((candidate) => candidate.directory === directory);
      const memberAbs = join(collectionAbs, directory);
      const nestedCollections = collectionDirs.filter((candidate) => candidate !== collectionAbs && candidate.startsWith(`${memberAbs}/`));
      const memberSources = sourceFiles.filter((source) => source.abs.startsWith(`${memberAbs}/`) && !nestedCollections.some((nested) => source.abs === nested || source.abs.startsWith(`${nested}/`)));
      const leafNames = new Set(componentFilenames(taxonomy));
      if (!memberSources.some((source) => dirname(source.abs) === memberAbs && leafNames.has(basename(source.abs)))) problems.push({ code: "member-component-leaf-missing", severity: "error", path: currentPath, componentId: member?.id, message: "Specific member has no immediate canonical component language leaf." });
      if (memberSources.some((source) => semanticProvenance(source.rel) === "generated") && !member?.generator) problems.push({ code: "generated-provenance-missing", severity: "error", path: currentPath, componentId: member?.id, message: "Generated source requires exact generator provenance in the semantic member manifest." });
      const currentOwner = semanticRel(repoRoot, dirname(collectionAbs));
      drafts.push({ id: member?.id || currentPath, currentPath, collectionPath, collectionDirectory, kind: member?.kind ?? spec.kind, responsibility: member?.responsibility ?? stripEmoji(directory), member, sourceFiles: memberSources, currentOwner, ownerAncestry: semanticOwnerAncestry(currentPath) });
    }
  }
  const memberRoots = drafts.map((draft) => [realpathSync(join(repoRoot, draft.currentPath)), draft.id] as const).sort((a, b) => b[0].length - a[0].length || semanticCompare(a[0], b[0]));
  const sourceToComponent = new Map<string, string>();
  const sourceComponentRoots = new Map<string, string>();
  const componentLeaves = new Map<string, string>();
  const leafNames = new Set(componentFilenames(taxonomy));
  for (const source of sourceFiles) {
    const owner = memberRoots.find(([root]) => source.abs === root || source.abs.startsWith(`${root}/`));
    if (owner) {
      sourceToComponent.set(source.abs, owner[1]);
      sourceComponentRoots.set(source.abs, owner[0]);
      if (dirname(source.abs) === owner[0] && leafNames.has(basename(source.abs)) && source.rel.endsWith(".rs")) componentLeaves.set(owner[0], source.abs);
    } else if (leafNames.has(basename(source.abs)) && !source.rel.includes(`/${taxonomy.packagesDirName}/`)) problems.push({ code: "unclassified-component-leaf", severity: "error", path: source.rel, message: "Authored component leaf is not owned by a recognized <collection>/<specific> member." });
  }
  const fileIndex = new Map(sourceFiles.map((source) => [source.abs, source.abs] as const));
  const resolvers = semanticResolverIndex(allFiles, taxonomy);
  const draftById = new Map(drafts.map((draft) => [draft.id, draft] as const));
  const edges: SemanticConsumerEdge[] = [];
  for (const source of sourceFiles) {
    const from = sourceToComponent.get(source.abs);
    if (!from) continue;
    const production = semanticProductionConsumer(source, packages);
    const pathTargets = source.rel.endsWith(".rs") ? resolveRustPathAttributes(source.abs, source.content) : [];
    for (const pathTarget of pathTargets) {
      let targetAbs = pathTarget.target;
      try {
        if (existsSync(targetAbs)) targetAbs = realpathSync(targetAbs);
      } catch {
        continue;
      }
      const to = sourceToComponent.get(targetAbs);
      if (to && to !== from) edges.push({ from, to, source: source.rel, target: semanticRel(repoRoot, targetAbs), mechanism: "path-attribute", production });
    }
    for (const specifier of semanticImportSpecs(source)) {
      const targetAbs = semanticResolveCandidate(source, specifier, fileIndex, extensions, resolvers, taxonomy);
      if (!targetAbs) continue;
      const to = sourceToComponent.get(targetAbs);
      if (to && to !== from) {
        const target = semanticRel(repoRoot, targetAbs);
        edges.push({ from, to, source: source.rel, target, mechanism: source.rel.endsWith(".csproj") ? "project-reference" : "static-import", production });
        if (/\b(?:register|mount)\s*\(/u.test(source.content)) edges.push({ from, to, source: source.rel, target, mechanism: "runtime-registration", production });
      }
    }
    const componentRoot = sourceComponentRoots.get(source.abs);
    if (componentRoot) for (const useTarget of resolveRustRelativeUses(source, componentRoot, componentLeaves)) {
      const to = sourceToComponent.get(useTarget.target);
      if (to && to !== from) edges.push({ from, to, source: source.rel, target: semanticRel(repoRoot, useTarget.target), mechanism: "static-import", production });
    }
  }
  const uniqueEdges = [...new Map(edges.map((edge) => [`${edge.from}\0${edge.to}\0${edge.source}\0${edge.target}\0${edge.mechanism}`, edge])).values()].sort((a, b) => semanticCompare(`${a.from}\0${a.to}\0${a.source}`, `${b.from}\0${b.to}\0${b.source}`));
  const duplicateFiles = new Map<string, SemanticSource[]>();
  for (const source of sourceFiles) {
    const normalized = semanticNormalizeDuplicate(source.content);
    if (normalized.length < 80 || !sourceToComponent.has(source.abs)) continue;
    const hash = createHash("sha256").update(normalized).digest("hex");
    duplicateFiles.set(hash, [...(duplicateFiles.get(hash) ?? []), source]);
  }
  const duplicates: SemanticDuplicateCluster[] = [...duplicateFiles.entries()]
    .map(([hash, sources]) => ({ hash, componentIds: semanticUnique(sources.map((source) => sourceToComponent.get(source.abs)!).filter(Boolean)), paths: semanticUnique(sources.map((source) => source.rel)) }))
    .filter((cluster) => cluster.componentIds.length > 1)
    .map((cluster) => ({ id: `duplicate-${cluster.hash.slice(0, 16)}`, ...cluster }))
    .sort((a, b) => semanticCompare(a.id, b.id));
  const records: SemanticCensusRecord[] = drafts.map((draft) => {
    const incoming = uniqueEdges.filter((edge) => edge.to === draft.id);
    const productionConsumers = draft.kind === "module"
      ? semanticTerminalProductionConsumers(draft.id, uniqueEdges, draftById)
      : semanticUnique(incoming.filter((edge) => edge.production).map((edge) => edge.from));
    const excludedConsumers = semanticUnique(incoming.filter((edge) => !edge.production).map((edge) => edge.from));
    const consumerRecords = productionConsumers.map((id) => draftById.get(id)).filter((record): record is SemanticRecordDraft => Boolean(record));
    const lca = semanticLowestCommonOwner(consumerRecords);
    const declaredConsumers = semanticUnique(draft.member?.module?.productionConsumers ?? []);
    if (draft.kind === "module") {
      const currentLevel = semanticOwnerLevel(draft.currentOwner);
      if (!currentLevel || !taxonomy.semanticAllowedOwnerLevels.includes(currentLevel)) problems.push({ code: "module-owner-level", severity: "error", path: draft.currentPath, componentId: draft.id, message: `Module owner ${JSON.stringify(draft.currentOwner)} is not an allowed semantic owner level.` });
      if (declaredConsumers.join("\0") !== productionConsumers.join("\0")) problems.push({ code: "module-consumer-graph-mismatch", severity: "error", path: draft.currentPath, componentId: draft.id, message: `Declared production consumers (${declaredConsumers.join(", ") || "none"}) do not match resolved graph (${productionConsumers.join(", ") || "none"}).` });
      if (productionConsumers.length < taxonomy.semanticConsumerMinimum) problems.push({ code: "module-production-consumer-minimum", severity: "error", path: draft.currentPath, componentId: draft.id, message: `Resolved reverse closure reaches ${productionConsumers.length} independent production components; ${taxonomy.semanticConsumerMinimum} are required.` });
      if (productionConsumers.length >= taxonomy.semanticConsumerMinimum && lca !== draft.currentOwner) problems.push({ code: "module-lowest-common-owner", severity: "error", path: draft.currentPath, componentId: draft.id, message: `Module is owned by ${JSON.stringify(draft.currentOwner)} but consumers compute ${JSON.stringify(lca)}.` });
    }
    const languageMirrors = semanticUnique(draft.sourceFiles.map((source) => Object.entries(taxonomy.componentFileKinds).find(([, kindId]) => kindId === fileKindIdForFilename(basename(source.abs), taxonomy))?.[0]).filter((value): value is string => Boolean(value)));
    const ownerPackages = packages.filter((pkg) => draft.currentPath === pkg.ownerRel || draft.currentPath.startsWith(`${pkg.ownerRel}/`) || pkg.ownerRel.startsWith(`${draft.currentPath}/`)).map((pkg) => `${pkg.role}:${pkg.ownerRel}${pkg.target ? `#${pkg.target}` : ""}`);
    const duplicateClusters = duplicates.filter((cluster) => cluster.componentIds.includes(draft.id)).map((cluster) => cluster.id);
    const staticImports = semanticUnique(draft.sourceFiles.flatMap((source) => [...semanticImportSpecs(source), ...semanticRustUseSpecs(source)]));
    const runtimeMounts = semanticUnique(draft.sourceFiles.flatMap((source) => semanticRuntimeEvidence(source, /\bmount(?:ed|ing)?\b|\.mount\s*\(/iu)));
    const registrations = semanticUnique(draft.sourceFiles.flatMap((source) => semanticRuntimeEvidence(source, /\bregister(?:ed|ing)?\b|\.register\s*\(|plugin_exports!|inventory::submit/iu)));
    return {
      id: draft.id,
      currentPath: draft.currentPath,
      collectionPath: draft.collectionPath,
      kind: draft.kind,
      responsibility: draft.responsibility,
      ownerAncestry: draft.ownerAncestry,
      languageMirrors,
      packages: semanticUnique(ownerPackages),
      provenance: semanticProvenance(draft.currentPath),
      publicSymbols: semanticUnique(draft.sourceFiles.flatMap(semanticPublicSymbols)),
      schemaContracts: semanticUnique(draft.sourceFiles.filter((source) => [".json", ".proto", ".graphql"].includes(extname(source.abs)) || source.rel.endsWith(".semio")).map((source) => source.rel)),
      staticImports,
      runtimeMounts,
      registrations,
      packageEntrypoints: [],
      reverseDependencies: semanticUnique(incoming.map((edge) => edge.source)),
      productionConsumers,
      excludedConsumers,
      currentOwner: draft.currentOwner,
      computedLowestCommonOwner: lca,
      proposedDisposition: semanticDisposition(draft.kind, productionConsumers, draft.currentOwner, lca),
      duplicateClusters,
      applicableInstructions: semanticInstructions(repoRoot, draft.currentPath),
      dirtyConflicts: [],
      generatorInputs: draft.member?.generator ? [draft.member.generator] : [],
      tests: semanticUnique(draft.sourceFiles.filter((source) => semanticProvenance(source.rel) === "test").map((source) => source.rel)),
      runtimeSurfaces: semanticUnique([...runtimeMounts, ...registrations]),
      leaseId: null,
    };
  }).sort((a, b) => semanticCompare(a.id, b.id));
  const scopedRecords = options.scope ? records.filter((record) => semanticScopeMatchesId(record.id, options.scope!) || record.currentPath === options.scope || record.currentPath.startsWith(`${options.scope}/`)) : records;
  const scopedIds = new Set(scopedRecords.map((record) => record.id));
  const scopedRoots = options.scope ? semanticScopeRoots(records, options.scope) : [];
  const scopedProblems = problems.filter((problem) => !options.scope || semanticPathInRoots(problem.path, scopedRoots) || (problem.componentId !== undefined && semanticScopeMatchesId(problem.componentId, options.scope)));
  return {
    records: scopedRecords,
    graph: { nodes: scopedRecords.map((record) => record.id), edges: uniqueEdges.filter((edge) => scopedIds.has(edge.from) || scopedIds.has(edge.to)) },
    problems: scopedProblems.sort((a, b) => semanticCompare(`${a.path}\0${a.code}\0${a.message}`, `${b.path}\0${b.code}\0${b.message}`)),
    duplicates: duplicates.filter((cluster) => cluster.componentIds.some((id) => scopedIds.has(id))),
  };
}

/** 🗃️ Stable machine-readable census representation. */
export function renderSemanticCensusJson(census: SemanticCensus): string {
  return `${JSON.stringify(census, null, 2)}\n`;
}

function semanticMarkdownCell(value: string): string {
  return value.replaceAll("|", "\\|").replaceAll("\n", " ");
}

/** 📓️ Stable human-readable companion for the machine census. */
export function renderSemanticCensusMarkdown(census: SemanticCensus): string {
  const lines = [
    "# Semantic Census",
    "",
    `- Components: ${census.records.length}`,
    `- Consumer edges: ${census.graph.edges.length}`,
    `- Problems: ${census.problems.length}`,
    `- Duplicate evidence clusters: ${census.duplicates.length}`,
    "",
    "| Semantic ID | Kind | Current path | Owner | Production consumers | Disposition |",
    "|---|---|---|---|---:|---|",
    ...census.records.map((record) => `| ${semanticMarkdownCell(record.id)} | ${record.kind} | ${semanticMarkdownCell(record.currentPath)} | ${semanticMarkdownCell(record.currentOwner)} | ${record.productionConsumers.length} | ${record.proposedDisposition} |`),
    "",
  ];
  return `${lines.join("\n")}\n`;
}

/** 🔣️ Stable machine-readable duplicate-candidate representation. */
export function renderSemanticDuplicatesJson(census: SemanticCensus): string {
  return `${JSON.stringify({ duplicates: census.duplicates }, null, 2)}\n`;
}

/** 🧾️ Stable duplicate evidence companion without semantic conclusions. */
export function renderSemanticDuplicatesMarkdown(census: SemanticCensus): string {
  const lines = ["# Semantic Duplicate Evidence", "", "Similarity is evidence only. It never authorizes extraction, relocation, or deletion.", ""];
  for (const cluster of census.duplicates) {
    lines.push(`## ${cluster.id}`, "", `- SHA-256: \`${cluster.hash}\``, `- Components: ${cluster.componentIds.join(", ")}`, "", ...cluster.paths.map((path) => `- ${path}`), "");
  }
  if (census.duplicates.length === 0) lines.push("No cross-component exact-syntax clusters found.", "");
  return `${lines.join("\n")}\n`;
}

/** 🚦️ Stable report shared by non-blocking report and blocking enforce modes. */
export function renderSemanticTaxonomyReport(census: SemanticCensus, scope?: string): string {
  const lines = ["# Semantic Taxonomy Report", "", `- Mode: report`, `- Scope: ${scope ?? "all active taxonomy areas"}`, `- Components: ${census.records.length}`, `- Errors: ${census.problems.filter((problem) => problem.severity === "error").length}`, `- Warnings: ${census.problems.filter((problem) => problem.severity === "warning").length}`, "", "## Findings", ""];
  if (census.problems.length === 0) lines.push("No findings.");
  else for (const problem of census.problems) lines.push(`- [${problem.severity}] ${problem.code} — ${problem.path}: ${problem.message}`);
  return `${lines.join("\n")}\n`;
}
