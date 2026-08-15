//#region 🧲️Header
// 2025-2026 Ueli Saluz <ueli@semio-tech.com>
// AGPL-3.0 — @semio-tech/repo-lib/js: shared taxonomy vocabulary + repo-wide package discovery contract.
//#endregion 🧲️Header

//#region 🔌️Adapters
import { ephemeralMap, ephemeralBox } from "@semio-tech/framework";
import { createHash } from "node:crypto";
import { existsSync, readFileSync, readdirSync, realpathSync, statSync } from "node:fs";
import { basename, dirname, extname, join, relative, resolve } from "node:path";
import { fileURLToPath } from "node:url";
//#endregion 🔌️Adapters

const __dirname = dirname(fileURLToPath(import.meta.url));

//#region 🔣️Taxonomy
/** 🗺️ Declared migration state of an area root (`taxonomy.areas` values) — replaces the registry script's `LEGACY_LAYOUT_TOLERANT` boolean with per-area granularity. */
export type AreaState = "legacy" | "mixed" | "clean" | "exempt";

/** 📈️ Derived (never declared) state of one already-migrated owner: `clean` once nothing but the taxonomy shape is left, `mixed` while residuals survive — see `discoverOwners`. */
export type PackageMaturity = "clean" | "mixed";

/** 🧩️ Semantic responsibility owned by one collection member. */
export type SemanticKind = "inference" | "mutation" | "io" | "module" | "artifact" | "standard" | "subset" | "plugin" | "product" | "extension" | "capability" | "ui" | "action" | "app" | "command";

/** 🧭️ Lowest legal owner of a reusable module. */
export type SemanticOwnerLevel = "subset" | "standard" | "artifact" | "app" | "plugin" | "product" | "s" | "framework";

/** 🗂️ Schema entry describing a recognized semantic collection directory. */
export interface SemanticCollectionSpec {
  readonly kind: SemanticKind;
  readonly direction?: "import" | "export";
}

/** 🏷️ One exact child declared by a collection root's canonical `🔣️component.json`. */
export interface SemanticMember {
  readonly directory: string;
  readonly id: string;
  readonly kind: SemanticKind;
  readonly responsibility: string;
  readonly generator?: string;
  readonly inference?: { readonly inputs: readonly string[]; readonly target: string };
  readonly mutation?: { readonly command: string; readonly event: string };
  readonly io?: { readonly format: string; readonly direction: "import" | "export" };
  readonly module?: { readonly productionConsumers: readonly string[] };
}

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
  readonly semanticManifestFilename: string;
  readonly semanticExtensionKey: string;
  readonly semanticConsumerMinimum: number;
  readonly semanticAllowedOwnerLevels: readonly SemanticOwnerLevel[];
  readonly semanticCollections: Readonly<Record<string, SemanticCollectionSpec>>;
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
  readonly standardsDirName: string;
  readonly subsetsDirName: string;
  readonly standardDirPrefix?: string;
  readonly standardSlugPattern?: string;
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
  /** 🚪️ Dedicated collection below `🚪️io/` for codecs of derived inference results. */
  readonly ioInferenceCollectionDirName: string;
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
  /** 🫧️ Required children of each `🫧️transient/` facet: its schema. The ephemeral local-only UI lane, fourth and last of the state mechanisms. */
  readonly transientChildDirs: readonly string[];
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
  /** 🎭️ STRUCTURAL set: every directory allowed directly below a mode. */
  readonly modeChildDirs: readonly string[];
  /** 🎭️ COMPLETENESS set: mode children that must exist even when empty. */
  readonly modeRequiredChildDirs: readonly string[];
  readonly semioDataLeafPrefix: string;
  readonly semioFileExtension: string;
  /** 📖️ Normative `.semio` spec filename per constitutional artifact facet (`artifactComponentDirs` keys only). */
  readonly artifactSpecFilenames: Readonly<Record<string, string>>;
  /** 🪟️ STRUCTURAL set: every directory allowed directly below a window. */
  readonly windowChildDirs: readonly string[];
  /** 🪟️ COMPLETENESS set: capability directories every window must carry, empty modules allowed. */
  readonly windowRequiredChildDirs: readonly string[];
  /** 🌐️ IMPLEMENTATION set: language component leaves every concrete capability member must carry. */
  readonly windowComponentLangs: readonly string[];
  /** 📌️ Tracked marker used only when a required window facet has no specific items. */
  readonly windowEmptyFacetFilename: string;
  readonly taxonomyLeafParentDirs: readonly string[];
  /** 🍃️ Leaf component filename, keyed by target when a package has one (e.g. `"🧊️wgpu"`), else by lang (e.g. `"🦀️rust"` for plugins, which never have a target level). */
  readonly taxonomyLeafFilenames: Readonly<Record<string, string>>;
  /** 🚪️ PRIMARY entry filename per lang; `ecosystems.<lang>.entryFilenames` carries the full accepted set. */
  readonly entryFilenames: Readonly<Record<string, string>>;
  /** 🧪️ Co-located story leaf filename (does not match a `*.stories.*` glob — callers must list it literally). */
  readonly storyLeafFilename: string;
  readonly libWiringLineBudget: number;
  readonly forbiddenPathSegments: readonly string[];
  /** 🔌️ Structural facet folders allowed directly under each plugin root. */
  readonly pluginChildDirs: readonly string[];
  readonly pluginRequiredChildDirs: readonly string[];
  /** 💻️ Structural facet folders owned directly by the OS product. */
  readonly osChildDirs: readonly string[];
  readonly osRequiredChildDirs: readonly string[];
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
  /** 🔌️ Area roots whose package owners contribute plugins. */
  readonly pluginAreas: readonly string[];
  /** 🌳️ Independent graduation state of each plugin area's taxonomy-tree contract. */
  readonly pluginTaxonomyStates: Readonly<Record<string, AreaState>>;
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

/**
 * 🂡 Whether a dir name is an emoji-prefixed slug — an Extended_Pictographic codepoint at the
 * start, U+FE0F variation selector optional.
 *
 * The selector must be optional: real slug dirs on disk are predominantly bare. Among `🧬️mutations`
 * slugs, `📄set-snapshot` and `➕create-node` carry no U+FE0F; among `💡️inferences` slugs, neither do
 * `📦bounds`, `🧭topology`, `⏱duration` or `🧾outline`. Requiring it rejected the majority of genuine
 * slugs as undeclared. Anchoring at the start also matches this predicate's own name — the previous
 * unanchored form accepted an emoji occurring anywhere in the name.
 */
function isEmojiPrefixedSlugDir(name: string): boolean {
  return /^\p{Extended_Pictographic}\uFE0F?/u.test(name);
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
    if (root === "🚪️io") return { kind: "fixed", dirs: [...(taxonomy.ioDirectionDirs ?? []), taxonomy.ioInferenceCollectionDirName] };
    return { kind: "none" };
  }
  if (root === "🧬️schema") {
    if (parents.length === 2 && (taxonomy.schemaChildDirs ?? []).includes(a!)) {
      if (a === "🧬️mutations") return { kind: "fixed", dirs: [...(taxonomy.representationDirs ?? []), "*"] };
      if (a === "💡️inferences") return { kind: "fixed", dirs: ["*"] };
      return { kind: "fixed", dirs: taxonomy.representationDirs ?? [] };
    }
    if (parents.length === 3 && a === "🧬️mutations") {
      if ((taxonomy.representationDirs ?? []).includes(b!)) return { kind: "none" };
      return { kind: "fixed", dirs: taxonomy.mutationChildDirs ?? [] };
    }
    if (parents.length === 3 && a === "💡️inferences") return { kind: "none" };
    if (parents.length === 3 && (taxonomy.representationDirs ?? []).includes(b!)) return { kind: "none" };
    if (parents.length === 4 && a === "🧬️mutations") return { kind: "none" };
    return { kind: "none" };
  }
  if (root === "🚪️io") {
    const directions = taxonomy.ioDirectionDirs ?? [];
    const childMap = taxonomy.ioDirectionChildDirs ?? {};
    if (parents.length === 2 && a === taxonomy.ioInferenceCollectionDirName) return { kind: "fixed", dirs: taxonomy.representationDirs ?? [] };
    if (parents.length === 3 && a === taxonomy.ioInferenceCollectionDirName && (taxonomy.representationDirs ?? []).includes(b!)) return { kind: "none" };
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
  if (facet === "🫧️transient") return taxonomy.transientChildDirs ?? [];
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
    if (parents.length === 2 && parents[0] === "🧬️schema" && parents[1] === "💡️inferences" && (taxonomy.representationDirs ?? []).includes(segment)) return false;
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
  for (const area of taxonomy.pluginAreas) {
    if (!(area in taxonomy.areas)) problems.push(`pluginAreas member "${area}" is missing from areas.`);
    const state = taxonomy.pluginTaxonomyStates[area];
    if (!state) problems.push(`pluginTaxonomyStates is missing plugin area "${area}".`);
    else if (!areaStates.has(state)) problems.push(`pluginTaxonomyStates["${area}"] = "${state}" is not one of areaStates (${taxonomy.areaStates.join(", ")}).`);
  }
  for (const area of Object.keys(taxonomy.pluginTaxonomyStates)) {
    if (!taxonomy.pluginAreas.includes(area)) problems.push(`pluginTaxonomyStates declares non-plugin area "${area}".`);
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
  //#region WindowShapeContract
  if (!Array.isArray(taxonomy.windowRequiredChildDirs) || taxonomy.windowRequiredChildDirs.length === 0) {
    problems.push(`windowRequiredChildDirs must be a non-empty array.`);
  } else {
    const seen = new Set<string>();
    for (const dir of taxonomy.windowRequiredChildDirs) {
      if (seen.has(dir)) problems.push(`windowRequiredChildDirs contains duplicate member "${dir}".`);
      seen.add(dir);
      if (!taxonomy.windowChildDirs.includes(dir)) {
        problems.push(`windowRequiredChildDirs member "${dir}" is missing from windowChildDirs — the structural set must be a superset of the completeness set.`);
      }
      if (!taxonomy.taxonomyLeafParentDirs.includes(dir)) {
        problems.push(`windowRequiredChildDirs member "${dir}" is missing from taxonomyLeafParentDirs.`);
      }
    }
  }
  if (!Array.isArray(taxonomy.windowComponentLangs) || taxonomy.windowComponentLangs.length === 0) {
    problems.push(`windowComponentLangs must be a non-empty array.`);
  } else {
    const seen = new Set<string>();
    for (const lang of taxonomy.windowComponentLangs) {
      if (seen.has(lang)) problems.push(`windowComponentLangs contains duplicate member "${lang}".`);
      seen.add(lang);
      if (!taxonomy.taxonomyLeafFilenames[lang]) {
        problems.push(`windowComponentLangs member "${lang}" has no taxonomyLeafFilenames entry.`);
      }
    }
  }
  if (typeof taxonomy.windowEmptyFacetFilename !== "string" || taxonomy.windowEmptyFacetFilename.length === 0) {
    problems.push(`windowEmptyFacetFilename must be a non-empty string.`);
  }
  //#endregion WindowShapeContract
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
  if (!taxonomy.ioInferenceCollectionDirName || !isEmojiPrefixedSlugDir(taxonomy.ioInferenceCollectionDirName)) {
    problems.push(`ioInferenceCollectionDirName must be one emoji-prefixed collection directory.`);
  }
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
  for (const required of ["📥️import", "📤️export", "🚪️io", taxonomy.ioInferenceCollectionDirName, "\ud83e\udde9\ufe0fdeserializers", "\ud83e\uddf5\ufe0fserializers"] as const) {
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
    ["transientChildDirs", taxonomy.transientChildDirs],
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
  for (const [owner, structural, required] of [
    ["plugin", taxonomy.pluginChildDirs, taxonomy.pluginRequiredChildDirs],
    ["os", taxonomy.osChildDirs, taxonomy.osRequiredChildDirs],
  ] as const) {
    if (!Array.isArray(required) || required.length === 0) {
      problems.push(`${owner}RequiredChildDirs must be a non-empty array.`);
      continue;
    }
    for (const dir of new Set(required)) {
      if (!structural.includes(dir)) problems.push(`${owner}RequiredChildDirs member "${dir}" is missing from ${owner}ChildDirs.`);
      if (!taxonomy.taxonomyLeafParentDirs.includes(dir)) problems.push(`${owner}RequiredChildDirs member "${dir}" is missing from taxonomyLeafParentDirs.`);
    }
    if (new Set(required).size !== required.length) problems.push(`${owner}RequiredChildDirs contains duplicate members.`);
  }
  //#region StateLaneContract
  // 🫧️ The four state mechanisms are exhaustive, so every state-owning scope (app, mode, window)
  // declares the same three non-artifact lanes; artifacts themselves are the fourth.
  const stateLaneDirs = ["🎚️config", "👥️presence", "🫧️transient"] as const;
  if (!Array.isArray(taxonomy.modeChildDirs) || taxonomy.modeChildDirs.length === 0) {
    problems.push(`modeChildDirs must be a non-empty array.`);
  } else {
    if (!taxonomy.modeChildDirs.includes(taxonomy.windowsDirName)) {
      problems.push(`modeChildDirs must include "${taxonomy.windowsDirName}" — a mode owns its windows.`);
    }
    for (const dir of taxonomy.modeChildDirs) {
      if (!dir) problems.push(`modeChildDirs contains an empty entry.`);
      else if (dir !== taxonomy.windowsDirName && !taxonomy.taxonomyLeafParentDirs.includes(dir)) {
        problems.push(`modeChildDirs member "${dir}" is missing from taxonomyLeafParentDirs.`);
      }
    }
  }
  if (!Array.isArray(taxonomy.modeRequiredChildDirs) || taxonomy.modeRequiredChildDirs.length === 0) {
    problems.push(`modeRequiredChildDirs must be a non-empty array.`);
  } else {
    for (const dir of taxonomy.modeRequiredChildDirs) {
      if (!taxonomy.modeChildDirs.includes(dir)) problems.push(`modeRequiredChildDirs member "${dir}" is missing from modeChildDirs.`);
    }
  }
  for (const lane of stateLaneDirs) {
    if (!taxonomy.appChildDirs.includes(lane)) problems.push(`appChildDirs must include the state lane "${lane}".`);
    if (!taxonomy.modeChildDirs?.includes(lane)) problems.push(`modeChildDirs must include the state lane "${lane}".`);
    if (!taxonomy.windowChildDirs.includes(lane)) problems.push(`windowChildDirs must include the state lane "${lane}".`);
    if (!taxonomy.windowRequiredChildDirs.includes(lane)) problems.push(`windowRequiredChildDirs must include the state lane "${lane}".`);
  }
  //#endregion StateLaneContract
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
  const commandsDir = "🎮️commands";
  for (const [owner, dirs] of [
    ["appChildDirs", taxonomy.appChildDirs],
    ["modeChildDirs", taxonomy.modeChildDirs],
    ["pluginChildDirs", taxonomy.pluginChildDirs],
    ["osChildDirs", taxonomy.osChildDirs],
  ] as const) {
    if (!Array.isArray(dirs) || !dirs.includes(commandsDir)) problems.push(`${owner} must include "${commandsDir}" — commands are owned at every command scope.`);
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
  //#region SemanticCollectionContract
  if (taxonomy.semanticManifestFilename !== "🔣️component.json") problems.push(`semanticManifestFilename must be the canonical "🔣️component.json".`);
  if (taxonomy.semanticExtensionKey !== "x-semio") problems.push(`semanticExtensionKey must be "x-semio".`);
  if (!Number.isInteger(taxonomy.semanticConsumerMinimum) || taxonomy.semanticConsumerMinimum < 2) problems.push(`semanticConsumerMinimum must be an integer of at least two.`);
  const ownerLevels = new Set<SemanticOwnerLevel>(["subset", "standard", "artifact", "app", "plugin", "product", "s", "framework"]);
  if (new Set(taxonomy.semanticAllowedOwnerLevels).size !== taxonomy.semanticAllowedOwnerLevels.length) problems.push(`semanticAllowedOwnerLevels contains duplicate members.`);
  for (const level of taxonomy.semanticAllowedOwnerLevels) if (!ownerLevels.has(level)) problems.push(`semanticAllowedOwnerLevels contains unknown level "${level}".`);
  for (const required of ["🔨️modules", "💡️inferences", "🧬️mutations", "🧩️deserializers", "🧵️serializers", `🚪️io/${taxonomy.ioInferenceCollectionDirName}`] as const) {
    if (!taxonomy.semanticCollections[required]) problems.push(`semanticCollections must declare "${required}".`);
  }
  for (const [directory, spec] of Object.entries(taxonomy.semanticCollections)) {
    if (!directory || !directory.split("/").every(isEmojiPrefixedSlugDir)) problems.push(`semanticCollections key ${JSON.stringify(directory)} must be one or more emoji-prefixed directories.`);
    if (spec.kind === "io" && !spec.direction) problems.push(`semanticCollections["${directory}"] must declare an io direction.`);
    if (spec.kind !== "io" && spec.direction) problems.push(`semanticCollections["${directory}"] declares a direction but is not io.`);
  }
  //#endregion SemanticCollectionContract
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

//#region 🧩️SemanticCollections
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
const SEMANTIC_NON_PRODUCTION_SEGMENTS = new Set(["🧪️tests", "tests", "test", "__tests__", "📚️examples", "🧪️examples", "examples", "fixtures", "🧪️fixtures", "🤖️generated"]);

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
  return new Set([...Object.values(taxonomy.ecosystems).map((ecosystem) => ecosystem.sourceExtension), ".tsx", ".jsx", ".c", ".cc", ".cpp", ".h", ".hpp", ".proto", ".graphql", ".csproj"]);
}

function semanticWalk(root: string): string[] {
  const files: string[] = [];
  const visited = new Set<string>();
  const walk = (dir: string): void => {
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
      if (entry.isDirectory()) {
        if (!entry.name.startsWith(".") && !SEMANTIC_SKIP_DIRS.has(entry.name)) walk(path);
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

/** 🗺️ Taxonomy-derived active roots; legacy and exempt areas are absent structurally. */
export function semanticActiveRoots(repoRoot: string, taxonomy: Taxonomy = loadTaxonomy()): string[] {
  const active = Object.entries(taxonomy.areas)
    .filter(([, state]) => state === "clean" || state === "mixed")
    .map(([path]) => path)
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
  if (!existsSync(path)) {
    problems.push({ code: "collection-manifest-missing", severity: "error", path: collectionPath, message: `Collection is missing canonical ${taxonomy.semanticManifestFilename}.` });
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
    problems.push({ code: "collection-manifest-invalid", severity: "error", path: collectionPath, message: `${taxonomy.semanticManifestFilename} is not valid JSON: ${(error as Error).message}` });
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
function semanticProductionConsumer(source: SemanticSource): boolean {
  return source.production && basename(source.abs) !== "📦️glue.rs" && !semanticAssemblyOnly(source.content, extname(source.abs));
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

/** 🦀️ Relative Rust namespaces are imports too; they must resolve to their semantic member, not a crate barrel. */
function semanticRustUseSpecs(source: SemanticSource): string[] {
  if (!source.rel.endsWith(".rs")) return [];
  const specs: string[] = [];
  for (const match of source.content.matchAll(/\b(?:pub\s+)?use\s+((?:super|self)(?:::[^;]+)+)\s*;/gu)) if (match[1]) specs.push(match[1].replace(/\s+/gu, " ").trim());
  return semanticUnique(specs);
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
      else if (["import", "default", "types", "bun", "node"].includes(key) && !result.has(prefix)) semanticFlattenExports(child, prefix, result);
    }
  }
  return result;
}

function semanticResolverIndex(allFiles: readonly string[]): SemanticResolverIndex {
  const packageRoots = new Map<string, string>();
  const packageExports = new Map<string, ReadonlyMap<string, string>>();
  const goModules = new Map<string, string>();
  const pythonRoots: string[] = [];
  const tsPaths: { readonly root: string; readonly pattern: string; readonly targets: readonly string[] }[] = [];
  for (const file of allFiles) {
    if (basename(file) === "package.json") {
      const manifest = semanticJson(file);
      if (typeof manifest?.name === "string") {
        packageRoots.set(manifest.name, dirname(file));
        packageExports.set(manifest.name, semanticFlattenExports(manifest.exports ?? manifest.module ?? manifest.main ?? "./🟦️glue.ts"));
      }
    } else if (basename(file) === "go.mod") {
      const module = readFileSync(file, "utf8").match(/^module\s+(\S+)/mu)?.[1];
      if (module) goModules.set(module, dirname(file));
    } else if (basename(file) === "pyproject.toml") {
      pythonRoots.push(dirname(file));
    } else if (basename(file) === "tsconfig.json") {
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

/** 🦀️ Resolves nested Rust path attributes with the enclosing module's cumulative base. */
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

/** 🦀️ Resolves `use super::…` through emoji-prefixed sibling directories to immediate semantic component leaves. */
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
      const child = readdirSafe(base).find((entry) => entry.isDirectory() && stripEmoji(entry.name).replaceAll("-", "_") === segment);
      if (!child) break;
      base = join(base, child.name);
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

function semanticResolveCandidate(from: SemanticSource, specifier: string, fileIndex: ReadonlyMap<string, string>, extensions: ReadonlySet<string>, resolvers: SemanticResolverIndex, leafFilenames: readonly string[]): string | null {
  let normalized = specifier.replace(/[?#].*$/u, "");
  if (from.rel.endsWith(".py") && normalized.startsWith(".")) normalized = normalized.replace(/^\.+/u, "./").replaceAll(".", "/");
  const bases: string[] = [];
  if (normalized.startsWith(".") || normalized.startsWith("/")) bases.push(resolve(dirname(from.abs), normalized));
  else {
    for (const [name, root] of [...resolvers.packageRoots.entries()].sort((a, b) => b[0].length - a[0].length || semanticCompare(a[0], b[0]))) {
      if (normalized !== name && !normalized.startsWith(`${name}/`)) continue;
      const subpath = normalized === name ? "." : `./${normalized.slice(name.length + 1)}`;
      const target = resolvers.packageExports.get(name)?.get(subpath) ?? (subpath === "." ? "./🟦️glue.ts" : subpath);
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
    for (const extension of extensions) candidates.push(join(base, `index${extension}`));
    candidates.push(join(base, "__init__.py"));
    for (const filename of leafFilenames) candidates.push(join(base, filename));
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

/** 🕸️ Follows reverse module edges until independent non-module production components; intermediary modules never qualify. */
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

/** 📊️ Builds the timestamp-free semantic census from taxonomy-defined active scope. */
export function buildSemanticCensus(repoRoot: string, options: { readonly scope?: string } = {}, taxonomy: Taxonomy = loadTaxonomy()): SemanticCensus {
  repoRoot = realpathSync(repoRoot);
  const problems: SemanticProblem[] = validateTaxonomy(taxonomy).map((message) => ({ code: "taxonomy-schema", severity: "error", path: semanticRel(repoRoot, join(__dirname, "../🔣️taxonomy.json")), message }));
  const extensions = semanticSourceExtensions(taxonomy);
  const allFiles = semanticActiveRoots(repoRoot, taxonomy).flatMap((active) => semanticWalk(realpathSync(join(repoRoot, active))));
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
    const manifest = semanticReadManifest(join(collectionAbs, taxonomy.semanticManifestFilename), taxonomy, problems, collectionPath);
    const actualChildren = readdirSafe(collectionAbs)
      .filter((entry) => entry.isDirectory() && !entry.name.startsWith(".") && !SEMANTIC_SKIP_DIRS.has(entry.name) && entry.name !== taxonomy.packagesDirName && entry.name !== "🤖️generated")
      .map((entry) => entry.name)
      .sort(semanticCompare);
    const declaredMembers = manifest?.members ?? [];
    const declaredDirs = declaredMembers.map((member) => member.directory);
    if (actualChildren.length === 0) problems.push({ code: "collection-empty", severity: "error", path: collectionPath, message: "Semantic collection has no specific members." });
    for (const duplicate of semanticUnique(declaredDirs.filter((directory, index) => declaredDirs.indexOf(directory) !== index))) problems.push({ code: "member-directory-duplicate", severity: "error", path: collectionPath, message: `Manifest declares directory ${JSON.stringify(duplicate)} more than once.` });
    const ids = declaredMembers.map((member) => member.id);
    for (const duplicate of semanticUnique(ids.filter((id, index) => ids.indexOf(id) !== index))) problems.push({ code: "member-id-duplicate", severity: "error", path: collectionPath, message: `Manifest declares semantic id ${JSON.stringify(duplicate)} more than once.` });
    for (const directory of actualChildren.filter((directory) => !declaredDirs.includes(directory))) problems.push({ code: "manifest-child-missing", severity: "error", path: `${collectionPath}/${directory}`, message: `Direct child is not declared in ${taxonomy.semanticManifestFilename}.` });
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
      const leafNames = new Set(Object.values(taxonomy.taxonomyLeafFilenames));
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
  const leafNames = new Set(Object.values(taxonomy.taxonomyLeafFilenames));
  for (const source of sourceFiles) {
    const owner = memberRoots.find(([root]) => source.abs === root || source.abs.startsWith(`${root}/`));
    if (owner) {
      sourceToComponent.set(source.abs, owner[1]);
      sourceComponentRoots.set(source.abs, owner[0]);
      if (dirname(source.abs) === owner[0] && leafNames.has(basename(source.abs)) && source.rel.endsWith(".rs")) componentLeaves.set(owner[0], source.abs);
    } else if (leafNames.has(basename(source.abs)) && !source.rel.includes(`/${taxonomy.packagesDirName}/`)) problems.push({ code: "unclassified-component-leaf", severity: "error", path: source.rel, message: "Authored component leaf is not owned by a recognized <collection>/<specific> member." });
  }
  const fileIndex = new Map(sourceFiles.map((source) => [source.abs, source.abs] as const));
  const resolvers = semanticResolverIndex(allFiles);
  const draftById = new Map(drafts.map((draft) => [draft.id, draft] as const));
  const edges: SemanticConsumerEdge[] = [];
  for (const source of sourceFiles) {
    const from = sourceToComponent.get(source.abs);
    if (!from) continue;
    const production = semanticProductionConsumer(source);
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
      const targetAbs = semanticResolveCandidate(source, specifier, fileIndex, extensions, resolvers, Object.values(taxonomy.taxonomyLeafFilenames));
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
    const languageMirrors = semanticUnique(draft.sourceFiles.map((source) => Object.entries(taxonomy.taxonomyLeafFilenames).find(([, filename]) => filename === basename(source.abs))?.[0]).filter((value): value is string => Boolean(value)));
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
  const scopedRecords = options.scope ? records.filter((record) => record.id.includes(options.scope!) || record.currentPath.includes(options.scope!)) : records;
  const scopedIds = new Set(scopedRecords.map((record) => record.id));
  const scopedProblems = problems.filter((problem) => !options.scope || problem.path.includes(options.scope) || problem.componentId?.includes(options.scope));
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

/** 🧬️ Stable machine-readable duplicate-candidate representation. */
export function renderSemanticDuplicatesJson(census: SemanticCensus): string {
  return `${JSON.stringify({ duplicates: census.duplicates }, null, 2)}\n`;
}

/** 📓️ Stable duplicate evidence companion without semantic conclusions. */
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
//#endregion 🧩️SemanticCollections
