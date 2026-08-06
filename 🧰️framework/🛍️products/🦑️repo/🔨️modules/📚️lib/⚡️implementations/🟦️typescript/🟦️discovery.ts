//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — @semio-tech/repo-lib/js: shared taxonomy vocabulary + repo-wide package discovery contract.
//#endregion 🧲️Header

//#region 🔌️Adapters
import { existsSync, readFileSync, readdirSync } from "node:fs";
import { join, dirname, relative } from "node:path";
import { fileURLToPath } from "node:url";
//#endregion 🔌️Adapters

const __dirname = dirname(fileURLToPath(import.meta.url));

//#region 🔣️Taxonomy
/** 🗺️ Declared migration state of an area root (`taxonomy.areas` values) — replaces the registry script's `LEGACY_LAYOUT_TOLERANT` boolean with per-area granularity. */
export type AreaState = "legacy" | "mixed" | "clean" | "exempt";

/** 📈️ Derived (never declared) state of one already-migrated owner: `clean` once nothing but the taxonomy shape is left, `mixed` while residuals survive — see `discoverOwners`. */
export type PackageMaturity = "clean" | "mixed";

/** 🏷️ Where an ecosystem's semio role marker lives: a `table` inside the package manifest, addressed dotted (`package.metadata.semio` for TOML, `metadata.semio` for JSON). */
export interface EcosystemMarkerSpec {
  readonly in: "manifest";
  readonly format: "toml" | "json";
  readonly table: string;
  readonly roleKey: string;
  readonly idKey: string;
}

/** 🌐️ Per-language packaging contract: which manifest identifies a package, where its role marker sits, which entry/leaf filenames it uses. `manifestFilename: null` ⇒ discovery-opaque until that ecosystem joins the contract. */
export interface Ecosystem {
  readonly manifestFilename: string | null;
  /** 📍️ File that must sit at the OWNER root rather than in `📦️packages/<lang>/` (Go: `go.mod`, since a Go module root must contain its sources). */
  readonly moduleRootFilename: string | null;
  readonly marker: EcosystemMarkerSpec | null;
  readonly entryFilenames: readonly string[];
  readonly leafFilename: string;
  readonly sourceExtension: string;
}

/** 🎯️ One render/build target of a lang (`📦️packages/<lang>/🎯️targets/<target>/`), with the leaf/entry filenames that target uses. */
export interface TargetSpec {
  readonly lang: string;
  readonly leafFilename: string;
  readonly entryFilenames: readonly string[];
}

/** 🦀️ One valid way of writing the entry file's `#[path]` strings — see `Taxonomy.rustEntryPathRules`. */
export interface RustEntryPathConvention {
  readonly id: string;
  readonly outerReset: string | null;
  readonly groupingReset: string;
  readonly leafPrefix: string;
}

/** 🦀️ Cumulative-`#[path]` base rules for the relocated (Shape V2) rust entry file. */
export interface RustEntryPathRules {
  readonly _comment?: string;
  readonly entryDirFromOwner: string;
  readonly resolution: "cumulative";
  readonly groupingResetPath: string;
  readonly leafPathPrefix: string;
  readonly conventions: readonly RustEntryPathConvention[];
}

/**
 * 🔣️ Shape of `🔣️taxonomy.json` — the single source of truth for taxonomy directory-name/role/lang
 * vocabulary and the package-discovery contract, replacing the two independently hand-maintained copies in
 * framework/os/plugin/registry script.ts (`TAXONOMY_ARTIFACT_COMPONENTS`/`TAXONOMY_WINDOW_CHILDREN`) and
 * root script.ts (`POLICY_ARTIFACT_COMPONENT_DIRS`/`POLICY_WINDOW_CHILD_DIRS`) — see master ticket
 * `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`.
 */
export interface Taxonomy {
  readonly _comment?: string;
  readonly schemaVersion: number;
  readonly roles: readonly string[];
  readonly langs: readonly string[];
  readonly ecosystems: Readonly<Record<string, Ecosystem>>;
  readonly targets: Readonly<Record<string, TargetSpec>>;
  readonly rustEntryPathRules: RustEntryPathRules;
  readonly packagesDirName: string;
  /** 🎯️ Optional per-lang render-target axis: `<owner>/📦️packages/<lang>/🎯️targets/<target>/<manifest>`. */
  readonly targetsDirName: string;
  /** 🧱️ Flat co-location dir holding one subdir per logical element, each with a sibling leaf file per lang/target (see `taxonomyLeafFilenames`). */
  readonly elementsDirName: string;
  /** 🚪️ Shape V2: the entry file lives inside `📦️packages/<lang>/`, no longer at the owner root. */
  readonly entryLocation: "packages" | "owner-root";
  /** 📦️ Exact filenames allowed inside a `📦️packages/<lang>[/🎯️targets/<target>]/` dir — packaging code only, never data. */
  readonly packagingFileNames: readonly string[];
  /** 📦️ Suffix allowlist complementing `packagingFileNames` (tool configs: `*.config.ts`, …). */
  readonly packagingFileSuffixes: readonly string[];
  readonly artifactsDirName: string;
  readonly appsDirName: string;
  readonly modesDirName: string;
  readonly windowsDirName: string;
  /** ✅️ COMPLETENESS set: every `🗿️artifacts/<a>/` must carry each of these as a leaf. Never merge with `artifactChildDirs`. */
  readonly artifactComponentDirs: readonly string[];
  /** 🌳️ STRUCTURAL set: every dir allowed as a child of an artifact (superset of `artifactComponentDirs`; the extra member is `⚙️engine`, allowed but not required). */
  readonly artifactChildDirs: readonly string[];
  readonly exampleComponentDirs: readonly string[];
  readonly appChildDirs: readonly string[];
  readonly semioDataLeafPrefix: string;
  readonly semioFileExtension: string;
  /** 📖️ Normative `.semio` spec filename per constitutional artifact facet (`artifactComponentDirs` keys only). */
  readonly artifactSpecFilenames: Readonly<Record<string, string>>;
  readonly windowChildDirs: readonly string[];
  readonly taxonomyLeafParentDirs: readonly string[];
  /** 🍃️ Leaf component filename, keyed by target when a package has one (e.g. `"🧊️wgpu"`), else by lang (e.g. `"🦀️rust"` for plugins, which never have a target level). */
  readonly taxonomyLeafFilenames: Readonly<Record<string, string>>;
  /** 🚪️ PRIMARY entry filename per lang; `ecosystems.<lang>.entryFilenames` carries the full accepted set. */
  readonly entryFilenames: Readonly<Record<string, string>>;
  /** 🧪️ Co-located story leaf filename (does not match a `*.stories.*` glob — callers must list it literally). */
  readonly storyLeafFilename: string;
  readonly libWiringLineBudget: number;
  readonly forbiddenPathSegments: readonly string[];
  readonly rootDataDirNames: readonly string[];
  readonly rootDataFileNames: readonly string[];
  readonly rootDocFileNames: readonly string[];
  readonly areaStates: readonly AreaState[];
  readonly packageMaturityStates: readonly PackageMaturity[];
  /** 🧭️ How migration is detected — structurally, never from a hand-maintained package list. */
  readonly migratedMarker: "packages-dir-exists";
  readonly areas: Readonly<Record<string, AreaState>>;
}

let cachedTaxonomy: Taxonomy | undefined;

/** 📖️ Reads and parses `🔣️taxonomy.json` (sibling to this module, resolved via `import.meta.url` like `getLibRoot` in `📦️index.ts` of this same package) — cached after the first call. */
export function loadTaxonomy(): Taxonomy {
  if (cachedTaxonomy) return cachedTaxonomy;
  const path = join(__dirname, "🔣️taxonomy.json");
  cachedTaxonomy = JSON.parse(readFileSync(path, "utf8")) as Taxonomy;
  return cachedTaxonomy;
}

/**
 * 🚦️ Internal-consistency audit of the vocabulary itself: the completeness/structural artifact lists must
 * stay in their superset relation, every declared area state and every projection map (`taxonomyLeafFilenames`,
 * `entryFilenames`) must agree with `ecosystems`/`targets`, and every lang must be described. Returns human
 * readable problems (empty = healthy) so a vocabulary edit can never silently blind or flood the rules that
 * consume it.
 */
export function validateTaxonomy(taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const problems: string[] = [];
  const areaStates = new Set<string>(taxonomy.areaStates);
  for (const [area, state] of Object.entries(taxonomy.areas)) {
    if (!areaStates.has(state)) problems.push(`areas["${area}"] = "${state}" is not one of areaStates (${taxonomy.areaStates.join(", ")}).`);
  }
  for (const dir of taxonomy.exampleComponentDirs ?? []) {
    if (!taxonomy.taxonomyLeafParentDirs.includes(dir)) {
      problems.push(`exampleComponentDirs member "${dir}" should appear in taxonomyLeafParentDirs for leaf validation.`);
    }
  }
  for (const dir of taxonomy.artifactComponentDirs) {
    if (!taxonomy.artifactChildDirs.includes(dir)) problems.push(`artifactComponentDirs member "${dir}" is missing from artifactChildDirs — the structural set must be a superset of the completeness set.`);
  }
  for (const lang of taxonomy.langs) {
    const ecosystem = taxonomy.ecosystems[lang];
    if (!ecosystem) {
      problems.push(`langs member "${lang}" has no ecosystems entry.`);
      continue;
    }
    const leaf = taxonomy.taxonomyLeafFilenames[lang];
    if (leaf !== ecosystem.leafFilename) problems.push(`taxonomyLeafFilenames["${lang}"] = ${JSON.stringify(leaf)} disagrees with ecosystems["${lang}"].leafFilename = ${JSON.stringify(ecosystem.leafFilename)}.`);
    const primaryEntry = taxonomy.entryFilenames[lang];
    if (primaryEntry !== undefined && ecosystem.entryFilenames[0] !== primaryEntry) {
      problems.push(`entryFilenames["${lang}"] = ${JSON.stringify(primaryEntry)} is not the first of ecosystems["${lang}"].entryFilenames (${ecosystem.entryFilenames.join(", ")}).`);
    }
  }
  for (const [target, spec] of Object.entries(taxonomy.targets)) {
    if (!taxonomy.langs.includes(spec.lang)) problems.push(`targets["${target}"].lang = "${spec.lang}" is not a declared lang.`);
    const leaf = taxonomy.taxonomyLeafFilenames[target];
    if (leaf !== spec.leafFilename) problems.push(`taxonomyLeafFilenames["${target}"] = ${JSON.stringify(leaf)} disagrees with targets["${target}"].leafFilename = ${JSON.stringify(spec.leafFilename)}.`);
  }
  for (const key of Object.keys(taxonomy.taxonomyLeafFilenames)) {
    if (!taxonomy.ecosystems[key] && !taxonomy.targets[key]) problems.push(`taxonomyLeafFilenames key "${key}" is neither a lang nor a target.`);
  }
  for (const [facet, specName] of Object.entries(taxonomy.artifactSpecFilenames ?? {})) {
    if (!taxonomy.artifactComponentDirs.includes(facet)) {
      problems.push(`artifactSpecFilenames key "${facet}" is not in artifactComponentDirs.`);
    }
    if (!specName.endsWith(`.${taxonomy.semioFileExtension}`)) {
      problems.push(`artifactSpecFilenames["${facet}"] must end with .${taxonomy.semioFileExtension}.`);
    }
  }
  return problems;
}

/** 📖️ Normative spec filename for an artifact facet dir name, if any. */
export function artifactSpecFilename(facetDirName: string, taxonomy: Taxonomy = loadTaxonomy()): string | undefined {
  return taxonomy.artifactSpecFilenames?.[facetDirName];
}

/** 🧭️ Longest-prefix match of a repo-relative path against `taxonomy.areas` keys — `undefined` outside every declared area. */
export function areaOf(repoRelPath: string, taxonomy: Taxonomy = loadTaxonomy()): AreaState | undefined {
  const norm = repoRelPath.replaceAll("\\", "/").replace(/^\.\//, "");
  let bestKey: string | undefined;
  for (const key of Object.keys(taxonomy.areas)) {
    if (norm !== key && !norm.startsWith(`${key}/`)) continue;
    if (!bestKey || key.length > bestKey.length) bestKey = key;
  }
  return bestKey ? taxonomy.areas[bestKey] : undefined;
}
//#endregion 🔣️Taxonomy

//#region 🧭️Discovery
/** 🎭️ Package "kind" declared by the ecosystem's role marker — see `readSemioMarker` and `taxonomy.roles`. */
export type PackageRole = "plugin" | "framework" | "product" | "hub" | "s-module" | "extension" | "testkit" | "tool";

/** 🌐️ Ecosystem a discovered package's manifest belongs to (`taxonomy.langs`). */
export type PackageLang = "🦀️rust" | "🟦️typescript" | "🐹️go" | "🐍️python" | "🔷️dotnet";

/** 🎯️ A lang's render/build target when the package sits under `🎯️targets/<target>/` (three-level shape) — open vocabulary, e.g. `"⚛️react"`, `"🧊️wgpu"`, `"⌨️tui"`. */
export type PackageTarget = string;

/** 📦️ One package discovered under `<owner>/📦️packages/<lang>/` (two-level, e.g. plugins/styling) or `<owner>/📦️packages/<lang>/🎯️targets/<target>/` (three-level, e.g. ui/renderer-engine), with its role/id marker resolved. */
export interface DiscoveredPackage {
  readonly ownerRel: string;
  readonly lang: PackageLang;
  /** 🎯️ Set only for the three-level shape; `undefined` for a direct `📦️packages/<lang>/` package. */
  readonly target?: PackageTarget;
  /** 📁️ Repo-relative dir holding the manifest (i.e. `dirname(manifestPath)`). */
  readonly packageRel: string;
  readonly manifestPath: string;
  readonly role: PackageRole;
  readonly id: string;
  /** 🗺️ Declared state of the enclosing area (`taxonomy.areas`), `""` outside every declared area. */
  readonly area: string;
  /** 📈️ Derived state of this package's OWNER: `mixed` while the owner still carries forbidden-segment dirs or an owner-root entry file, else `clean`. */
  readonly maturity: PackageMaturity;
}

/** 🏠️ One owner (a dir carrying a `📦️packages` folder) with every package it ships plus its derived migration state. */
export interface DiscoveredOwner {
  readonly ownerRel: string;
  readonly area: string;
  readonly maturity: PackageMaturity;
  readonly langs: readonly PackageLang[];
  readonly targets: readonly PackageTarget[];
  readonly roles: readonly PackageRole[];
  readonly packages: readonly DiscoveredPackage[];
  /** 🔥️ Residual `⚡️implementations`/`⚡️implementation` dirs still inside this owner (burn-down counter — must decrease monotonically to 0). */
  readonly residualImplDirs: number;
  /** 🚪️ Shape V1 leftovers: entry files still sitting at the owner root instead of in `📦️packages/<lang>/`. */
  readonly entryFilesAtOwnerRoot: readonly string[];
}

/** ⚠️ A discovery-time problem `discoverPackages` does not fail on but must not stay silent about — see `discoverPackageProblems`. */
export interface DiscoveryProblem {
  readonly kind: "ambiguous-lang-shape" | "target-without-manifest" | "manifest-without-marker" | "unknown-lang" | "unknown-role";
  readonly path: string;
  readonly message: string;
}

/** 🔥️ Migration burn-down snapshot: everything that still has to shrink to zero before the finalization flip, derived from disk on every call (no hand-maintained lists). */
export interface DiscoveryBurndown {
  readonly ownersTotal: number;
  readonly packagesTotal: number;
  readonly cleanOwners: number;
  readonly mixedOwners: readonly DiscoveredOwner[];
  /** 🔥️ Every forbidden-segment dir repo-wide, including those outside any migrated owner. */
  readonly implDirsTotal: number;
  readonly implDirsByArea: Readonly<Record<string, number>>;
  /** 🏷️ `📦️packages/<lang>/` manifests carrying no role marker — invisible to `discoverPackages` until they get one. */
  readonly unmarkedManifests: readonly { readonly path: string; readonly area: string }[];
  /** 📦️ Files inside a package dir that are neither packaging code nor its entry file (data/docs belong at the owner root under Shape V2). */
  readonly packagingViolations: readonly { readonly path: string; readonly ownerRel: string }[];
}

/** 🔍️ Extracts a dotted TOML table's body from manifest text (stops at the next `[…]` header) — simple line-scan, no TOML parser dependency (none is a repo dependency for the TS side; mirrors `hasSemioRole` in framework/os/plugin/registry script.ts). */
function tomlTableBody(text: string, table: string): string | undefined {
  const header = `[${table}]`;
  const lines = text.split("\n");
  const start = lines.findIndex((line) => line.trim() === header);
  if (start === -1) return undefined;
  const body: string[] = [];
  for (let i = start + 1; i < lines.length; i++) {
    if (lines[i].trim().startsWith("[")) break;
    body.push(lines[i]);
  }
  return body.join("\n");
}

/** 🔍️ Walks a dotted key path (`"metadata.semio"`, `"semio"`) into a parsed JSON manifest. */
function jsonTable(value: unknown, table: string): Record<string, unknown> | undefined {
  let current: unknown = value;
  for (const key of table.split(".")) {
    if (typeof current !== "object" || current === null) return undefined;
    current = (current as Record<string, unknown>)[key];
  }
  return typeof current === "object" && current !== null ? (current as Record<string, unknown>) : undefined;
}

/** 🏷️ Package name from a Cargo.toml's `[package] name = "…"` line. */
function rustPackageName(text: string): string | undefined {
  return text.match(/^name\s*=\s*"([^"]+)"/m)?.[1];
}

/**
 * 🏷️ Reads a package's role marker as declared by its ecosystem (`taxonomy.ecosystems.<lang>.marker`):
 * rust `[package.metadata.semio]`, typescript `package.json` `"semio"`, go `📋️project.json`
 * `metadata.semio`, python `pyproject.toml` `[tool.semio]`. `undefined` when the manifest is missing, the
 * ecosystem is discovery-opaque (no marker spec, e.g. dotnet), or no `role` is declared.
 */
export function readSemioMarker(manifestPath: string, lang: PackageLang, taxonomy: Taxonomy = loadTaxonomy()): { role: PackageRole; id?: string } | undefined {
  const spec = taxonomy.ecosystems[lang]?.marker;
  if (!spec || !existsSync(manifestPath)) return undefined;
  let role: string | undefined;
  let id: string | undefined;
  if (spec.format === "toml") {
    const body = tomlTableBody(readFileSync(manifestPath, "utf8"), spec.table);
    if (!body) return undefined;
    role = body.match(new RegExp(`^${spec.roleKey}\\s*=\\s*"([^"]+)"`, "m"))?.[1];
    id = body.match(new RegExp(`^${spec.idKey}\\s*=\\s*"([^"]+)"`, "m"))?.[1];
  } else {
    let parsed: unknown;
    try {
      parsed = JSON.parse(readFileSync(manifestPath, "utf8"));
    } catch {
      return undefined;
    }
    const table = jsonTable(parsed, spec.table);
    if (!table) return undefined;
    role = typeof table[spec.roleKey] === "string" ? (table[spec.roleKey] as string) : undefined;
    id = typeof table[spec.idKey] === "string" ? (table[spec.idKey] as string) : undefined;
  }
  if (!role) return undefined;
  return id ? { role: role as PackageRole, id } : { role: role as PackageRole };
}

//#region 🏷️SemioMarkerSubTable
/** 🔧️ Minimal TOML table-body decoder: plain string values (`key = "..."`) and flat string arrays
 * (`key = ["a", "b"]`) — sufficient for opt-in sub-tables (see `readSemioMarkerSubTable`); deliberately
 * NOT a general TOML parser (no numbers/bools/nested-inline-tables/multiline strings — reach for a real
 * TOML dependency if a future consumer needs more than this). */
function tomlTableValues(body: string): Record<string, unknown> {
  const result: Record<string, unknown> = {};
  for (const rawLine of body.split("\n")) {
    const line = rawLine.trim();
    if (!line || line.startsWith("#")) continue;
    const arrayMatch = line.match(/^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*\[([^\]]*)\]\s*$/);
    if (arrayMatch) {
      result[arrayMatch[1]] = [...arrayMatch[2].matchAll(/"((?:[^"\\]|\\.)*)"/g)].map((m) => m[1]);
      continue;
    }
    const scalarMatch = line.match(/^([A-Za-z_][A-Za-z0-9_]*)\s*=\s*"((?:[^"\\]|\\.)*)"\s*$/);
    if (scalarMatch) result[scalarMatch[1]] = scalarMatch[2];
  }
  return result;
}

/**
 * 🏷️ Reads an arbitrary OPT-IN sub-table nested under a package's own semio marker table — rust
 * `[package.metadata.semio.<subKey>]`, TS `package.json`'s `"semio": {"<subKey>": {...}}` — the generic
 * mechanism per-concern consumers use to opt a package INTO extra behavior beyond the bare role/id every
 * marked package already declares via `readSemioMarker` (first consumer: Storybook coverage, see
 * `26/08/06/GENERATED-STORYBOOK-SCOPES-AND-STORIES-FROM-PACKAGE-CATALOG`). `undefined` when the
 * manifest/table/sub-key is absent — silence here means "not opted in", never an error. Only ONE table
 * per sub-key is read (no TOML array-of-tables support); a package needing more than one entry, or
 * fields this decoder can't express, is expected to stay hand-curated by its consumer instead.
 */
export function readSemioMarkerSubTable(manifestPath: string, lang: PackageLang, subKey: string, taxonomy: Taxonomy = loadTaxonomy()): Record<string, unknown> | undefined {
  const spec = taxonomy.ecosystems[lang]?.marker;
  if (!spec || !existsSync(manifestPath)) return undefined;
  if (spec.format === "toml") {
    const body = tomlTableBody(readFileSync(manifestPath, "utf8"), `${spec.table}.${subKey}`);
    return body === undefined ? undefined : tomlTableValues(body);
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(readFileSync(manifestPath, "utf8"));
  } catch {
    return undefined;
  }
  return jsonTable(parsed, `${spec.table}.${subKey}`);
}
//#endregion 🏷️SemioMarkerSubTable

const DISCOVERY_SKIP_DIRS = new Set(["node_modules", "target", "dist", ".git", ".🦑️repo", "🤖️generated", "🔌️plugin-modules", "pkg", "storybook-static", "temp", "compose"]);

/** 📁️ `readdirSync(dir, { withFileTypes: true })`, defaulting to `[]` for an unreadable/missing dir — a helper (rather than an explicit `ReturnType<typeof readdirSync>` annotation) so the `Dirent<string>` element type infers unambiguously from this specific overload. */
function readdirSafe(absDir: string) {
  try {
    return readdirSync(absDir, { withFileTypes: true });
  } catch {
    return [];
  }
}

/** 🔤️ Drops every non-ASCII codepoint (emoji + variation selectors), e.g. `"✒️writer"` -> `"writer"` — mirrors `policyStripEmoji` in root script.ts. */
function stripEmoji(segment: string): string {
  return segment.replace(/[^\x00-\x7f]/g, "");
}

/** 🆔️ Falls back to the manifest's own package name, else an emoji-stripped dash-joined owner path, when `readSemioMarker` found no explicit `id`. */
function fallbackPackageId(manifestPath: string, lang: PackageLang, ownerRel: string): string {
  try {
    if (lang === "🦀️rust") {
      const name = rustPackageName(readFileSync(manifestPath, "utf8"));
      if (name) return name;
    } else {
      const name = (JSON.parse(readFileSync(manifestPath, "utf8")) as { name?: string }).name;
      if (name) return name;
    }
  } catch {
    /* fall through to path-derived id */
  }
  return ownerRel
    .replaceAll("\\", "/")
    .split("/")
    .map(stripEmoji)
    .filter(Boolean)
    .join("-");
}

/** 🚫️ Areas where a markerless manifest is expected (not yet migrated) and must stay a silent skip for `discoverPackageProblems` — `discoverBurndown` still counts it. */
const DISCOVERY_QUIET_AREAS = new Set(["legacy", "mixed", "exempt", ""]);

interface OwnerAccumulator {
  ownerRel: string;
  area: string;
  packages: DiscoveredPackage[];
  residualImplDirs: number;
  entryFilesAtOwnerRoot: string[];
}

interface DiscoveryScan {
  readonly packages: readonly DiscoveredPackage[];
  readonly owners: readonly DiscoveredOwner[];
  readonly problems: readonly DiscoveryProblem[];
  readonly burndown: DiscoveryBurndown;
}

const scanCache = new Map<string, DiscoveryScan>();

/** 🧹️ Drops the memoized repo scan — call after mutating the tree inside one process (tests, generators). */
export function clearDiscoveryCache(): void {
  scanCache.clear();
}

/**
 * 🗺️ ONE repo walk answering every discovery question. For each `<owner>/📦️packages/<lang>/` it resolves the
 * package as either a direct manifest (two-level — plugins, styling) or a `🎯️targets/<target>/<manifest>` tree
 * (three-level — ui, renderer-engine: one package per render target), reads its role marker via
 * `readSemioMarker`, and — in the same pass — derives each owner's migration state from disk: residual
 * `⚡️implementations`/`⚡️implementation` dirs and Shape V1 entry files still at the owner root. A package's
 * existence IS the migration marker (`taxonomy.migratedMarker`), so no hand-maintained "already migrated" list
 * can drift. A markerless manifest is skipped silently in a `legacy`/`mixed`/`exempt`/undeclared area (not yet
 * migrated) but always shows up in `discoverBurndown`, so nothing vanishes unnoticed.
 */
function scanRepo(repoRoot: string, taxonomy: Taxonomy): DiscoveryScan {
  const packagesDirName = taxonomy.packagesDirName;
  const targetsDirName = taxonomy.targetsDirName;
  const forbiddenSegments = new Set(taxonomy.forbiddenPathSegments);
  const packagingNames = new Set(taxonomy.packagingFileNames);
  const owners = new Map<string, OwnerAccumulator>();
  const problems: DiscoveryProblem[] = [];
  const unmarkedManifests: { path: string; area: string }[] = [];
  const packagingViolations: { path: string; ownerRel: string }[] = [];
  const implDirsByArea: Record<string, number> = {};
  let implDirsTotal = 0;

  const rel = (abs: string): string => relative(repoRoot, abs).replaceAll("\\", "/");

  const isPackagingFile = (name: string, allowedEntries: readonly string[]): boolean =>
    packagingNames.has(name) || allowedEntries.includes(name) || taxonomy.packagingFileSuffixes.some((suffix) => name.endsWith(suffix));

  const collectPackagingViolations = (manifestDirAbs: string, owner: OwnerAccumulator, allowedEntries: readonly string[]): void => {
    for (const entry of readdirSafe(manifestDirAbs)) {
      if (entry.isDirectory() || entry.name.startsWith(".") || isPackagingFile(entry.name, allowedEntries)) continue;
      packagingViolations.push({ path: rel(join(manifestDirAbs, entry.name)), ownerRel: owner.ownerRel });
    }
  };

  const resolveOne = (manifestAbs: string, lang: PackageLang, owner: OwnerAccumulator, target: PackageTarget | undefined): void => {
    const manifestPath = rel(manifestAbs);
    const marker = readSemioMarker(manifestAbs, lang, taxonomy);
    if (!marker) {
      unmarkedManifests.push({ path: manifestPath, area: owner.area });
      if (!DISCOVERY_QUIET_AREAS.has(owner.area)) {
        problems.push({ kind: "manifest-without-marker", path: manifestPath, message: `"${manifestPath}" has no resolvable semio role marker (area "${owner.area}" requires one).` });
      }
      return;
    }
    if (!taxonomy.roles.includes(marker.role)) {
      problems.push({ kind: "unknown-role", path: manifestPath, message: `"${manifestPath}" declares role "${marker.role}", which is not one of taxonomy.roles (${taxonomy.roles.join(", ")}).` });
      return;
    }
    owner.packages.push({
      ownerRel: owner.ownerRel,
      lang,
      target,
      packageRel: rel(dirname(manifestAbs)),
      manifestPath,
      role: marker.role,
      id: marker.id ?? fallbackPackageId(manifestAbs, lang, owner.ownerRel),
      area: owner.area,
      maturity: "clean", // 📈️ replaced once the owner's residual scan finishes (see finalize below)
    });
  };

  const scanPackagesDir = (packagesAbs: string, owner: OwnerAccumulator): void => {
    for (const langEntry of readdirSafe(packagesAbs)) {
      if (!langEntry.isDirectory() || langEntry.name.startsWith(".")) continue;
      const lang = langEntry.name as PackageLang;
      const ecosystem = taxonomy.ecosystems[lang];
      if (!ecosystem) {
        problems.push({ kind: "unknown-lang", path: rel(join(packagesAbs, langEntry.name)), message: `"${rel(join(packagesAbs, langEntry.name))}" is not a declared lang (${taxonomy.langs.join(", ")}).` });
        continue;
      }
      const manifestFilename = ecosystem.manifestFilename;
      if (!manifestFilename) continue; // 🔷️ declared but discovery-opaque until that ecosystem's manifest contract lands
      const langAbs = join(packagesAbs, langEntry.name);
      const directManifestAbs = join(langAbs, manifestFilename);
      const targetsAbs = join(langAbs, targetsDirName);
      const hasDirect = existsSync(directManifestAbs);
      const hasTargets = existsSync(targetsAbs);

      if (hasDirect && hasTargets) {
        problems.push({ kind: "ambiguous-lang-shape", path: rel(langAbs), message: `"${rel(langAbs)}" has both a direct manifest and a ${targetsDirName}/ dir — a lang is either two-level or three-level, never both.` });
        continue;
      }
      if (hasDirect) {
        resolveOne(directManifestAbs, lang, owner, undefined);
        collectPackagingViolations(langAbs, owner, ecosystem.entryFilenames);
        continue;
      }
      if (!hasTargets) continue;
      for (const targetEntry of readdirSafe(targetsAbs)) {
        if (!targetEntry.isDirectory()) continue;
        const targetAbs = join(targetsAbs, targetEntry.name);
        const targetManifestAbs = join(targetAbs, manifestFilename);
        if (!existsSync(targetManifestAbs)) {
          problems.push({ kind: "target-without-manifest", path: rel(targetAbs), message: `"${rel(targetAbs)}" has no ${manifestFilename}.` });
          continue;
        }
        resolveOne(targetManifestAbs, lang, owner, targetEntry.name);
        collectPackagingViolations(targetAbs, owner, taxonomy.targets[targetEntry.name]?.entryFilenames ?? ecosystem.entryFilenames);
      }
    }
  };

  const ownerRootEntryFiles = (entries: readonly { name: string; isDirectory: () => boolean }[]): string[] => {
    const wanted = new Set<string>();
    for (const ecosystem of Object.values(taxonomy.ecosystems)) for (const name of ecosystem.entryFilenames) wanted.add(name);
    for (const spec of Object.values(taxonomy.targets)) for (const name of spec.entryFilenames) wanted.add(name);
    return entries.filter((entry) => !entry.isDirectory() && wanted.has(entry.name)).map((entry) => entry.name);
  };

  const walk = (absDir: string, ownerStack: readonly OwnerAccumulator[]): void => {
    const entries = readdirSafe(absDir);
    let stack = ownerStack;
    if (entries.some((entry) => entry.isDirectory() && entry.name === packagesDirName)) {
      const ownerRel = rel(absDir);
      const owner: OwnerAccumulator = { ownerRel, area: areaOf(ownerRel, taxonomy) ?? "", packages: [], residualImplDirs: 0, entryFilesAtOwnerRoot: ownerRootEntryFiles(entries) };
      owners.set(ownerRel, owner);
      stack = [...ownerStack, owner];
      scanPackagesDir(join(absDir, packagesDirName), owner);
    }
    for (const entry of entries) {
      if (!entry.isDirectory() || entry.name.startsWith(".") || DISCOVERY_SKIP_DIRS.has(entry.name) || entry.name === packagesDirName) continue;
      if (forbiddenSegments.has(entry.name)) {
        implDirsTotal += 1;
        const area = stack.length > 0 ? stack[stack.length - 1].area : areaOf(rel(absDir), taxonomy) ?? "";
        implDirsByArea[area] = (implDirsByArea[area] ?? 0) + 1;
        if (stack.length > 0) stack[stack.length - 1].residualImplDirs += 1;
        continue; // 🔥️ counted as one residual sandwich; its innards are legacy by definition
      }
      walk(join(absDir, entry.name), stack);
    }
  };
  walk(repoRoot, []);

  const discoveredOwners: DiscoveredOwner[] = [...owners.values()]
    .map((owner) => {
      const maturity: PackageMaturity = owner.residualImplDirs === 0 && owner.entryFilesAtOwnerRoot.length === 0 ? "clean" : "mixed";
      const packages = owner.packages.map((pkg) => ({ ...pkg, maturity }));
      return {
        ownerRel: owner.ownerRel,
        area: owner.area,
        maturity,
        langs: [...new Set(packages.map((pkg) => pkg.lang))],
        targets: [...new Set(packages.flatMap((pkg) => (pkg.target ? [pkg.target] : [])))],
        roles: [...new Set(packages.map((pkg) => pkg.role))],
        packages,
        residualImplDirs: owner.residualImplDirs,
        entryFilesAtOwnerRoot: owner.entryFilesAtOwnerRoot,
      };
    })
    .sort((a, b) => a.ownerRel.localeCompare(b.ownerRel));

  const packages = discoveredOwners.flatMap((owner) => owner.packages).sort((a, b) => a.ownerRel.localeCompare(b.ownerRel) || (a.target ?? "").localeCompare(b.target ?? ""));

  return {
    packages,
    owners: discoveredOwners,
    problems,
    burndown: {
      ownersTotal: discoveredOwners.length,
      packagesTotal: packages.length,
      cleanOwners: discoveredOwners.filter((owner) => owner.maturity === "clean").length,
      mixedOwners: discoveredOwners.filter((owner) => owner.maturity === "mixed"),
      implDirsTotal,
      implDirsByArea,
      unmarkedManifests,
      packagingViolations,
    },
  };
}

function scan(repoRoot: string, taxonomy: Taxonomy): DiscoveryScan {
  const cached = scanCache.get(repoRoot);
  if (cached) return cached;
  const result = scanRepo(repoRoot, taxonomy);
  scanCache.set(repoRoot, result);
  return result;
}

/** 📦️ Flat catalog of every marked package in the repo — see `scanRepo`. */
export function discoverPackages(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): DiscoveredPackage[] {
  return [...scan(repoRoot, taxonomy).packages];
}

/** 🏠️ Owner-level view of the same scan: one row per `📦️packages`-carrying dir, with its langs/targets/roles and derived maturity. */
export function discoverOwners(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): DiscoveredOwner[] {
  return [...scan(repoRoot, taxonomy).owners];
}

/** ⚠️ Diagnostics half of the scan (ambiguous shapes, dangling target dirs, unknown langs/roles, unmarked manifests outside legacy areas). */
export function discoverPackageProblems(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): DiscoveryProblem[] {
  return [...scan(repoRoot, taxonomy).problems];
}

/** 🔥️ Burn-down half of the scan: everything that must shrink to zero before the finalization flip. */
export function discoverBurndown(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): DiscoveryBurndown {
  return scan(repoRoot, taxonomy).burndown;
}
//#endregion 🧭️Discovery
