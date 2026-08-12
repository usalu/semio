//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — @semio-tech/repo-lib/js: shared taxonomy vocabulary + repo-wide package discovery contract.
//#endregion 🧲️Header

//#region 🔌️Adapters
import { ephemeralMap, ephemeralBox } from "@semio-tech/framework";
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
  /** 🪆️ Dir-name prefix for a subset slug under `🪆️subsets/` (mirrors `standardDirPrefix` one level down). */
  readonly subsetDirPrefix?: string;
  /** 🪆️ Legal shape for a subset id (the logical id, `*`/`subsetAnyId` excepted — that one never matches this pattern by design). */
  readonly subsetSlugPattern?: string;
  /** 🪆️ The logical id every standard's unconstrained base subset carries. */
  readonly subsetAnyId?: string;
  /** 🪆️ The on-disk dir name `subsetAnyId` maps to — canonical single source for the `"*"` ⇔ `✳️any` mapping. */
  readonly subsetAnyDirName?: string;
  /** 🔣️ Filename of the per-standard subset vocabulary manifest at `🏅️standards/🔖️<slug>/🪆️subsets/<this>`. */
  readonly subsetsManifestFilename?: string;
  /** ✅️ COMPLETENESS set: every legacy artifact carries schema, engine, and IO. Lifecycle capabilities are schema-derived. */
  readonly artifactComponentDirs: readonly string[];
  /** 🌳️ STRUCTURAL set: every dir allowed as a child of an artifact (superset of `artifactComponentDirs`; the structural-only extra is `📚️examples`). */
  readonly artifactChildDirs: readonly string[];
  /** 🧬️ Required children of a standards-based artifact. */
  readonly newArtifactComponentDirs: readonly string[];
  readonly newArtifactChildDirs: readonly string[];
  /** 🏅️ Required and allowed children of a standard. */
  readonly standardComponentDirs: readonly string[];
  readonly standardChildDirs: readonly string[];
  /** 🪆️ Required and allowed children of a subset. */
  readonly subsetComponentDirs: readonly string[];
  readonly subsetChildDirs: readonly string[];
  /** 🪆️ Allowed subset archetypes: owning owns types; derived reuses types + conformance gate. */
  readonly subsetArchetypes?: readonly string[];
  /** ⚖️ Allowed IO fidelity class names a subset may declare. */
  readonly ioFidelityClasses?: readonly string[];
  /** 🧬️ Required children of each `🧬️mutations/<mutation>/` dir: mutation struct, per-mutation diff, inverse. */
  readonly mutationChildDirs: readonly string[];
  /** 🧬️ Required children of each `🧬️schema/` facet: snapshot, diff, mutations. */
  readonly schemaChildDirs: readonly string[];
  /** 📝️ Representation nodes under schema snapshot/diff/mutations. */
  readonly representationDirs: readonly string[];
  /** 🚪️ Top-level dirs under `🚪️io/`: import and export. */
  readonly ioDirectionDirs: readonly string[];
  /** 🚪️ Direction to codec folder (import→deserializers, export→serializers). */
  readonly ioDirectionChildDirs: Readonly<Record<string, string>>;
  /** 📖️ Spec leaves required under every text representation node. */
  readonly textSpecFilenames: readonly string[];
  /** 📡️ Spec leaves required under every binary representation node. */
  readonly binarySpecFilenames: readonly string[];
  /**
   * 🧬️ Schema serialisation formats a `🧬️schema` facet must carry, one handcrafted leaf each. `fieldCasing`
   * is the canonical casing a field name takes in that format, which the parity scanners normalise through.
   */
  readonly schemaFormats: Readonly<Record<string, { readonly leafFilename: string; readonly extension: string; readonly fieldCasing: string }>>;
  /** 🔣️ Normative JSON Schema leaf per `🧬️schema` facet path — the twin of `artifactSpecFilenames` for schema facets, which carry no `.semio` spec. */
  readonly artifactSchemaSpecFilenames: Readonly<Record<string, string>>;
  /** ✅️ COMPLETENESS set: every app that owns a config must carry each of these as a leaf. */
  readonly appComponentDirs: readonly string[];
  /** 🎛 Required children of each `🎚️config/` facet: its schema. */
  readonly configChildDirs: readonly string[];
  /** 👥️ Required children of each `👥️presence/` facet: its schema. */
  readonly presenceChildDirs: readonly string[];
  /** 🔣️ Normative JSON Schema leaf per app schema facet path. */
  readonly appSchemaSpecFilenames: Readonly<Record<string, string>>;
  readonly exampleAssetsDirName: string;
  readonly exampleTestsDirName: string;
  readonly exampleSlugPattern: string;
  readonly exampleAssetKindPrefixes: Readonly<Record<string, string>>;
  readonly exampleMediaKindPrefixes: Readonly<Record<string, string>>;
  readonly exampleLeafFilenames: Readonly<Record<string, string>>;
  readonly exampleTestLeafFilenames: Readonly<Record<string, string>>;
  readonly forbiddenExampleSlugs: readonly string[];
  readonly forbiddenExamplePluralDirs: readonly string[];
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
  /** 🔌️ Required facet folders directly under each `✏️s/🔌️plugins/<plugin>/` root. */
  readonly pluginChildDirs: readonly string[];
  /** 🚫️ Emoji-stripped directory/file stems banned repo-wide (e.g. `core`, `shared`). */
  readonly bannedNameStems: readonly string[];
  /** ✅️ Taxonomy directories must start with an emoji prefix that includes U+FE0F. */
  readonly requireEmojiPrefixWithVs16: boolean;
  readonly rootDataDirNames: readonly string[];
  readonly rootDataFileNames: readonly string[];
  readonly rootDocFileNames: readonly string[];
  readonly areaStates: readonly AreaState[];
  readonly packageMaturityStates: readonly PackageMaturity[];
  /** 🧭️ How migration is detected — structurally, never from a hand-maintained package list. */
  readonly migratedMarker: "packages-dir-exists";
  readonly areas: Readonly<Record<string, AreaState>>;
}

const cachedTaxonomy = ephemeralBox<Taxonomy | undefined>("framework.products.repo.modules.lib.discovery.component.ts.cachedTaxonomy", undefined);

/** 📖️ Reads and parses `🔣️taxonomy.json` (sibling to this module, resolved via `import.meta.url` like `getLibRoot` in `📦️index.ts` of this same package) — cached after the first call. */
export function loadTaxonomy(): Taxonomy {
  if (cachedTaxonomy.current) return cachedTaxonomy.current;
  const path = join(__dirname, "../🔣️taxonomy.json");
  cachedTaxonomy.current = JSON.parse(readFileSync(path, "utf8")) as Taxonomy;
  return cachedTaxonomy.current;
}

/** 🌳️ Level descriptor: fixed allowlist or wildcard (`*` = any emoji-prefixed slug dir). */
type ArtifactFacetLevel =
  | { readonly kind: "fixed"; readonly dirs: readonly string[] }
  | { readonly kind: "wildcard" }
  | { readonly kind: "none" };

/** 🂡 Whether a dir name is an emoji-prefixed slug (requires U+FE0F in the emoji prefix). */
function isEmojiPrefixedSlugDir(name: string): boolean {
  return /\p{Extended_Pictographic}\uFE0F/u.test(name);
}

/** 🌳️ Declared child level of a path under an artifact root (parents are `/`-segments already accepted). */
function artifactFacetChildLevel(parents: readonly string[], taxonomy: Taxonomy): ArtifactFacetLevel {
  if (parents.length === 0) return { kind: "fixed", dirs: taxonomy.artifactComponentDirs };
  const root = parents[0]!;
  const a = parents[1];
  const b = parents[2];
  const c = parents[3];
  if (parents.length === 1) {
    if (root === "🧬️schema") return { kind: "fixed", dirs: taxonomy.schemaChildDirs ?? [] };
    if (root === "🚪️io") return { kind: "fixed", dirs: taxonomy.ioDirectionDirs ?? [] };
    return { kind: "none" };
  }
  if (root === "🧬️schema") {
    if (parents.length === 2 && (taxonomy.schemaChildDirs ?? []).includes(a!)) {
      if (a === "🧬️mutations") return { kind: "fixed", dirs: [...(taxonomy.representationDirs ?? []), "*"] };
      return { kind: "fixed", dirs: taxonomy.representationDirs ?? [] };
    }
    if (parents.length === 3 && a === "🧬️mutations") {
      if ((taxonomy.representationDirs ?? []).includes(b!)) return { kind: "none" };
      return { kind: "fixed", dirs: taxonomy.mutationChildDirs ?? [] };
    }
    if (parents.length === 3 && (taxonomy.representationDirs ?? []).includes(b!)) return { kind: "none" };
    if (parents.length === 4 && a === "🧬️mutations") return { kind: "none" };
    return { kind: "none" };
  }
  if (root === "🚪️io") {
    const directions = taxonomy.ioDirectionDirs ?? [];
    const childMap = taxonomy.ioDirectionChildDirs ?? {};
    if (parents.length === 2 && directions.includes(a!)) {
      const child = childMap[a!];
      return child ? { kind: "fixed", dirs: [child] } : { kind: "none" };
    }
    if (parents.length === 3 && directions.includes(a!) && childMap[a!] === b) {
      return { kind: "fixed", dirs: [taxonomy.artifactsDirName] };
    }
    if (parents.length === 4 && b === childMap[a!] && c === taxonomy.artifactsDirName) return { kind: "wildcard" };
    if (parents.length === 5) return { kind: "none" };
    return { kind: "none" };
  }
  return { kind: "none" };
}

/** 🌳️ Declared children of a nesting artifact facet path (`/`-joined parents), empty when leaves-only. */
function artifactFacetChildDirs(facetPath: string, taxonomy: Taxonomy): readonly string[] {
  const parents = facetPath ? facetPath.split("/") : [];
  const level = artifactFacetChildLevel(parents, taxonomy);
  if (level.kind !== "fixed") return [];
  return level.dirs.filter((d) => d !== "*");
}

/** 🌳️ Declared children of a nesting app facet. */
function appFacetChildDirs(facet: string, taxonomy: Taxonomy): readonly string[] {
  if (facet === "🎚️config") return taxonomy.configChildDirs ?? [];
  if (facet === "👥️presence") return taxonomy.presenceChildDirs ?? [];
  return [];
}

/** 🧭️ Whether a `/`-joined facet path such as `🎚️config/🧬️schema` walks only declared dirs from an app (or shared config owner). */
export function appFacetPathIsDeclared(facetPath: string, taxonomy: Taxonomy = loadTaxonomy()): boolean {
  const [root, ...rest] = facetPath.split("/");
  if (!root || !taxonomy.appComponentDirs.includes(root)) return false;
  let parent = root;
  for (const segment of rest) {
    if (!appFacetChildDirs(parent, taxonomy).includes(segment)) return false;
    parent = segment;
  }
  return true;
}

/** 🧭️ Whether a `/`-joined facet path walks only declared dirs from an artifact root (supports `*` wildcard levels). */
export function artifactFacetPathIsDeclared(facetPath: string, taxonomy: Taxonomy = loadTaxonomy()): boolean {
  const [root, ...rest] = facetPath.split("/");
  if (!root || !taxonomy.artifactComponentDirs.includes(root)) return false;
  const parents: string[] = [root];
  for (const segment of rest) {
    const level = artifactFacetChildLevel(parents, taxonomy);
    if (level.kind === "none") return false;
    if (level.kind === "wildcard") {
      if (!isEmojiPrefixedSlugDir(segment)) return false;
    } else {
      const dirs = level.dirs;
      const allowWildcard = dirs.includes("*");
      const fixed = dirs.filter((d) => d !== "*");
      if (!(fixed.includes(segment) || (allowWildcard && isEmojiPrefixedSlugDir(segment)))) return false;
    }
    parents.push(segment);
  }
  return true;
}

/**
 * 🚦️ Internal-consistency audit of the vocabulary itself: the completeness/structural artifact lists must
 * stay in their superset relation, `mutationChildDirs` must be declared and covered by `taxonomyLeafParentDirs`,
 * every declared area state and every projection map (`taxonomyLeafFilenames`,
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
  //#region ExampleShapeContract
  if (!taxonomy.exampleAssetsDirName) problems.push(`exampleAssetsDirName is required.`);
  if (!taxonomy.exampleTestsDirName) problems.push(`exampleTestsDirName is required.`);
  if (!taxonomy.exampleSlugPattern) problems.push(`exampleSlugPattern is required.`);
  try {
    void new RegExp(taxonomy.exampleSlugPattern, "u");
  } catch {
    problems.push(`exampleSlugPattern is not a valid unicode RegExp: ${JSON.stringify(taxonomy.exampleSlugPattern)}`);
  }
  if (!taxonomy.appChildDirs.includes("📚️examples")) {
    problems.push(`appChildDirs must include "📚️examples".`);
  }
  if (!taxonomy.artifactChildDirs.includes("📚️examples")) {
    problems.push(`artifactChildDirs must include "📚️examples".`);
  }
  for (const plural of taxonomy.forbiddenExamplePluralDirs ?? []) {
    if (taxonomy.taxonomyLeafParentDirs.includes(plural)) {
      problems.push(`forbiddenExamplePluralDirs member "${plural}" must not appear in taxonomyLeafParentDirs.`);
    }
  }
  for (const dir of [taxonomy.exampleAssetsDirName, taxonomy.exampleTestsDirName]) {
    if (dir && taxonomy.taxonomyLeafParentDirs.includes(dir)) {
      problems.push(`"${dir}" must not appear in taxonomyLeafParentDirs — example leaves live on emoji-slug dirs, not assets/tests.`);
    }
  }
  for (const kind of ["snapshot-text", "snapshot-binary", "mutations-text", "mutations-binary", "diff-text", "cmd"] as const) {
    if (!taxonomy.exampleAssetKindPrefixes?.[kind]) {
      problems.push(`exampleAssetKindPrefixes must declare "${kind}".`);
    }
  }
  for (const kind of ["image", "mesh", "document", "video"] as const) {
    if (!taxonomy.exampleMediaKindPrefixes?.[kind]) {
      problems.push(`exampleMediaKindPrefixes must declare "${kind}".`);
    }
  }
  for (const lang of ["🦀️rust", "🟦️typescript"] as const) {
    if (!taxonomy.exampleLeafFilenames?.[lang]) problems.push(`exampleLeafFilenames must declare "${lang}".`);
    if (!taxonomy.exampleTestLeafFilenames?.[lang]) problems.push(`exampleTestLeafFilenames must declare "${lang}".`);
  }
  if ("exampleComponentDirs" in taxonomy) {
    problems.push(`exampleComponentDirs is removed — use emoji-slug examples with exampleAssetsDirName/exampleTestsDirName.`);
  }
  //#endregion ExampleShapeContract
  //#region StandardsSubsetsShapeContract
  if (taxonomy.standardDirPrefix) {
    try {
      void new RegExp(taxonomy.standardSlugPattern ?? "", "u");
    } catch {
      problems.push(`standardSlugPattern is not a valid unicode RegExp: ${JSON.stringify(taxonomy.standardSlugPattern)}`);
    }
  }
  if (taxonomy.subsetDirPrefix || taxonomy.subsetSlugPattern || taxonomy.subsetAnyId || taxonomy.subsetAnyDirName || taxonomy.subsetsManifestFilename) {
    if (!taxonomy.subsetDirPrefix) problems.push(`subsetDirPrefix is required once any subset-vocabulary field is declared.`);
    if (!taxonomy.subsetSlugPattern) problems.push(`subsetSlugPattern is required once any subset-vocabulary field is declared.`);
    else {
      try {
        void new RegExp(taxonomy.subsetSlugPattern, "u");
      } catch {
        problems.push(`subsetSlugPattern is not a valid unicode RegExp: ${JSON.stringify(taxonomy.subsetSlugPattern)}`);
      }
    }
    if (!taxonomy.subsetAnyId) problems.push(`subsetAnyId is required once any subset-vocabulary field is declared.`);
    if (!taxonomy.subsetAnyDirName) problems.push(`subsetAnyDirName is required once any subset-vocabulary field is declared.`);
    if (taxonomy.subsetDirPrefix && taxonomy.subsetAnyDirName && taxonomy.subsetAnyDirName !== `${taxonomy.subsetDirPrefix}any`) {
      problems.push(`subsetAnyDirName (${JSON.stringify(taxonomy.subsetAnyDirName)}) must equal subsetDirPrefix + "any" (${JSON.stringify(`${taxonomy.subsetDirPrefix}any`)}).`);
    }
    if (!taxonomy.subsetsManifestFilename) problems.push(`subsetsManifestFilename is required once any subset-vocabulary field is declared.`);
    if ("subsetDirs" in taxonomy) {
      problems.push(`subsetDirs is removed — subset id/dir mapping is now subsetAnyId/subsetAnyDirName (shape) plus each standard's own 🪆️subsets/🔣️component.json (content vocabulary), never a single repo-wide map.`);
    }
  }
  //#endregion StandardsSubsetsShapeContract
  //#region DerivedArtifactFacetsContract
  for (const [name, dirs] of [
    ["newArtifactChildDirs", taxonomy.newArtifactChildDirs],
    ["standardChildDirs", taxonomy.standardChildDirs],
    ["subsetChildDirs", taxonomy.subsetChildDirs],
  ] as const) {
    for (const forbidden of ["🏗️builder", "🧐️analyzer", "🎹️composer"] as const) {
      if (dirs.includes(forbidden)) problems.push(`${name} must not include derived lifecycle facet "${forbidden}".`);
    }
  }
  //#endregion DerivedArtifactFacetsContract
  for (const dir of taxonomy.artifactComponentDirs) {
    if (!taxonomy.artifactChildDirs.includes(dir)) problems.push(`artifactComponentDirs member "${dir}" is missing from artifactChildDirs — the structural set must be a superset of the completeness set.`);
  }
  //#region MutationFacetContract
  for (const required of ["🧬️schema", "🚪️io"] as const) {
    if (!taxonomy.artifactComponentDirs.includes(required)) {
      problems.push(`artifactComponentDirs must include "${required}".`);
    }
    if (!taxonomy.artifactChildDirs.includes(required)) {
      problems.push(`artifactChildDirs must include "${required}".`);
    }
  }
  for (const dirs of [taxonomy.artifactComponentDirs, taxonomy.artifactChildDirs, taxonomy.subsetComponentDirs, taxonomy.subsetChildDirs]) {
    if (dirs.includes("⚙️engine")) {
      problems.push(`artifact/subset dir vocabularies must not include "⚙️engine" — an artifact is a schema (snapshot, diff, mutations, inferences) plus an io system, never an engine. Behaviour belongs to the app that edits the artifact (🎛️apps/<app>/⚙️engine); pure algorithms belong one level up, in a module's ⚙️engine, which taxonomyLeafParentDirs keeps globally legal.`);
    }
  }
  for (const banned of ["🗣️dsl", "🔧️op", "📡️spr", "🔺️diff", "📸️snapshot", "🧬️mutations"] as const) {
    if (taxonomy.artifactComponentDirs.includes(banned)) {
      problems.push(`artifactComponentDirs must not include root "${banned}" — absorbed under 🧬️schema.`);
    }
  }
  if (!Array.isArray(taxonomy.mutationChildDirs) || taxonomy.mutationChildDirs.length === 0) {
    problems.push(`mutationChildDirs must be a non-empty array.`);
  } else {
    for (const dir of taxonomy.mutationChildDirs) {
      if (!dir) {
        problems.push(`mutationChildDirs contains an empty entry.`);
        continue;
      }
      if (!taxonomy.taxonomyLeafParentDirs.includes(dir)) {
        problems.push(`mutationChildDirs member "${dir}" is missing from taxonomyLeafParentDirs.`);
      }
    }
  }
  for (const required of ["🧬️mutations", "🦠️mutation", "↩️inverse"] as const) {
    if (!taxonomy.taxonomyLeafParentDirs.includes(required)) {
      problems.push(`taxonomyLeafParentDirs must include "${required}".`);
    }
  }
  //#endregion MutationFacetContract
  //#region IoFacetContract
  if ("mediaFormatDirs" in taxonomy) problems.push(`mediaFormatDirs is removed — use ioDirectionDirs + ioDirectionChildDirs.`);
  if ("ioFormatChildDirs" in taxonomy) problems.push(`ioFormatChildDirs is removed — use ioDirectionDirs + ioDirectionChildDirs.`);
  if (!Array.isArray(taxonomy.ioDirectionDirs) || taxonomy.ioDirectionDirs.length === 0) {
    problems.push(`ioDirectionDirs must be a non-empty array.`);
  } else {
    for (const dir of taxonomy.ioDirectionDirs) {
      if (!taxonomy.taxonomyLeafParentDirs.includes(dir)) {
        problems.push(`ioDirectionDirs member "${dir}" is missing from taxonomyLeafParentDirs.`);
      }
      if (!taxonomy.ioDirectionChildDirs?.[dir]) {
        problems.push(`ioDirectionChildDirs must declare "${dir}".`);
      }
    }
  }
  const ioChildDirs = taxonomy.ioDirectionChildDirs ?? {};
  for (const [direction, child] of Object.entries(ioChildDirs)) {
    if (!(taxonomy.ioDirectionDirs ?? []).includes(direction)) {
      problems.push(`ioDirectionChildDirs key "${direction}" is not in ioDirectionDirs.`);
    }
    if (!child) problems.push(`ioDirectionChildDirs["${direction}"] is empty.`);
    else if (!taxonomy.taxonomyLeafParentDirs.includes(child)) {
      problems.push(`ioDirectionChildDirs["${direction}"] = "${child}" is missing from taxonomyLeafParentDirs.`);
    }
  }
  for (const required of ["📥️import", "📤️export", "🚪️io", "\ud83e\udde9\ufe0fdeserializers", "\ud83e\uddf5\ufe0fserializers"] as const) {
    if (!taxonomy.taxonomyLeafParentDirs.includes(required)) {
      problems.push(`taxonomyLeafParentDirs must include "${required}".`);
    }
  }
  //#endregion IoFacetContract
  //#region SchemaFacetContract
  if ("snapshotChildDirs" in taxonomy) problems.push(`snapshotChildDirs is removed — use schemaChildDirs + representationDirs.`);
  if ("diffChildDirs" in taxonomy) problems.push(`diffChildDirs is removed — use schemaChildDirs + representationDirs.`);
  if (!Array.isArray(taxonomy.schemaChildDirs) || taxonomy.schemaChildDirs.length === 0) {
    problems.push(`schemaChildDirs must be a non-empty array.`);
  } else {
    for (const required of ["📸️snapshot", "🔺️diff", "🧬️mutations"] as const) {
      if (!taxonomy.schemaChildDirs.includes(required)) problems.push(`schemaChildDirs must include "${required}".`);
    }
  }
  if (!Array.isArray(taxonomy.representationDirs) || taxonomy.representationDirs.length === 0) {
    problems.push(`representationDirs must be a non-empty array.`);
  } else {
    for (const required of ["\ud83d\udcdd\ufe0ftext", "\ud83d\udcbe\ufe0fbinary"] as const) {
      if (!taxonomy.representationDirs.includes(required)) problems.push(`representationDirs must include "${required}".`);
      if (!taxonomy.taxonomyLeafParentDirs.includes(required)) {
        problems.push(`taxonomyLeafParentDirs must include representation "${required}".`);
      }
    }
  }
  if (!Array.isArray(taxonomy.textSpecFilenames) || taxonomy.textSpecFilenames.length !== 8) {
    problems.push(`textSpecFilenames must list exactly 8 leaves.`);
  }
  if (!Array.isArray(taxonomy.binarySpecFilenames) || taxonomy.binarySpecFilenames.length !== 6) {
    problems.push(`binarySpecFilenames must list exactly 6 leaves.`);
  }
  for (const required of ["⚙️engine", "🧬️schema"] as const) {
    if (!taxonomy.taxonomyLeafParentDirs.includes(required)) {
      problems.push(`taxonomyLeafParentDirs must include "${required}".`);
    }
  }
  for (const [key, dirs] of [
    ["configChildDirs", taxonomy.configChildDirs],
    ["presenceChildDirs", taxonomy.presenceChildDirs],
  ] as const) {
    if (!Array.isArray(dirs) || dirs.length === 0) {
      problems.push(`${key} must be a non-empty array.`);
      continue;
    }
    for (const dir of dirs) {
      if (!dir) problems.push(`${key} contains an empty entry.`);
      else if (!taxonomy.taxonomyLeafParentDirs.includes(dir)) problems.push(`${key} member "${dir}" is missing from taxonomyLeafParentDirs.`);
    }
  }
  if (taxonomy.artifactComponentDirs.includes("🎒️pack") || taxonomy.artifactChildDirs.includes("🎒️pack")) {
    problems.push(`a bare "🎒️pack" is not an artifact facet — binary snapshot lives under 🧬️schema/📸️snapshot/💾️binary.`);
  }
  const schemaFormats = taxonomy.schemaFormats ?? {};
  if (Object.keys(schemaFormats).length === 0) problems.push(`schemaFormats must be a non-empty registry.`);
  for (const [formatId, format] of Object.entries(schemaFormats)) {
    if (!format.leafFilename.endsWith(format.extension)) {
      problems.push(`schemaFormats["${formatId}"] leafFilename must end with its extension (${JSON.stringify(format.leafFilename)} vs ${JSON.stringify(format.extension)}).`);
    }
    if (format.fieldCasing !== "snake" && format.fieldCasing !== "camel") {
      problems.push(`schemaFormats["${formatId}"].fieldCasing must be "snake" or "camel", got ${JSON.stringify(format.fieldCasing)}.`);
    }
  }
  const normativeSchemaLeaf = schemaFormats["🔣️jsonschema"]?.leafFilename;
  for (const [facet, specName] of Object.entries(taxonomy.artifactSchemaSpecFilenames ?? {})) {
    if (!(facet === "🧬️schema" || artifactFacetPathIsDeclared(facet, taxonomy))) {
      problems.push(`artifactSchemaSpecFilenames key "${facet}" is not a declared schema facet path.`);
    }
    if (specName !== normativeSchemaLeaf) {
      problems.push(`artifactSchemaSpecFilenames["${facet}"] = ${JSON.stringify(specName)} must be the normative schemaFormats["🔣️jsonschema"] leaf ${JSON.stringify(normativeSchemaLeaf)}.`);
    }
  }
  for (const [facet, specName] of Object.entries(taxonomy.appSchemaSpecFilenames ?? {})) {
    if (!appFacetPathIsDeclared(facet, taxonomy)) {
      problems.push(`appSchemaSpecFilenames key "${facet}" is not a declared app facet path.`);
    }
    if (specName !== normativeSchemaLeaf) {
      problems.push(`appSchemaSpecFilenames["${facet}"] = ${JSON.stringify(specName)} must be the normative schemaFormats["🔣️jsonschema"] leaf ${JSON.stringify(normativeSchemaLeaf)}.`);
    }
  }
  if (!Array.isArray(taxonomy.appComponentDirs) || taxonomy.appComponentDirs.length === 0) {
    problems.push(`appComponentDirs must be a non-empty array.`);
  } else {
    for (const dir of taxonomy.appComponentDirs) {
      if (!taxonomy.appChildDirs.includes(dir)) problems.push(`appComponentDirs member "${dir}" is missing from appChildDirs — the structural set must be a superset of the completeness set.`);
    }
  }
  for (const banned of ["🧮️config", "🕸️wasm"] as const) {
    if (taxonomy.appChildDirs.includes(banned) || taxonomy.appComponentDirs.includes(banned)) {
      problems.push(`a bare "${banned}" is not an app facet — use "🎚️config" and "🌉️wasm".`);
    }
  }
  //#endregion SchemaFacetContract
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
    if (facet === "🎒️pack") {
      problems.push(`a bare "🎒️pack" key is not an artifact facet — key it as "📸️snapshot/🎒️pack".`);
    } else if (!artifactFacetPathIsDeclared(facet, taxonomy)) {
      problems.push(`artifactSpecFilenames key "${facet}" is not a declared artifact facet path.`);
    }
    if (!specName.endsWith(`.${taxonomy.semioFileExtension}`)) {
      problems.push(`artifactSpecFilenames["${facet}"] must end with .${taxonomy.semioFileExtension}.`);
    }
  }
  if (!Array.isArray(taxonomy.pluginChildDirs) || taxonomy.pluginChildDirs.length === 0) {
    problems.push(`pluginChildDirs must be a non-empty array.`);
  } else {
    for (const dir of taxonomy.pluginChildDirs) {
      if (!dir) problems.push(`pluginChildDirs contains an empty entry.`);
    }
  }
  if (!Array.isArray(taxonomy.bannedNameStems) || taxonomy.bannedNameStems.length === 0) {
    problems.push(`bannedNameStems must be a non-empty array.`);
  }
  if (taxonomy.bannedNameStems?.includes("core") !== true) {
    problems.push(`bannedNameStems must include "core".`);
  }
  if (taxonomy.requireEmojiPrefixWithVs16 !== true) {
    problems.push(`requireEmojiPrefixWithVs16 must be true.`);
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

const scanCache = ephemeralMap<string, DiscoveryScan>("framework.products.repo.modules.lib.discovery.component.ts.scanCache");

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
