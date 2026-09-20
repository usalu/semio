import { existsSync, mkdirSync, readdirSync, readFileSync, rmSync, statSync, writeFileSync } from "node:fs";
import { basename, dirname, join, relative, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import { BundleScript, discoverPackages, getWorkspaceRoot, inspectRustModuleGraph, inspectRustModuleGraphFacts, isDiscoverySkipDirectory, schemaFacetFormatEntries } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { registrySchemaValidator } from "../✅️catalog-verification/🟦️.ts";
import { EXAMPLES_DIRNAME, EXAMPLE_ASSETS_DIRNAME, EXAMPLE_RUST_LEAF, EXAMPLE_TESTS_DIRNAME, EXAMPLE_TS_LEAF, FORBIDDEN_EXAMPLE_PLURAL_DIRS, PLUGIN_AREAS, RUST_LANG, TAXONOMY, isExampleSlugName, primaryFilenameForKind } from "../🔎️discovery/🟦️.ts";
import { PlaygroundEntry } from "../🎮️playground/🔎️discovery/🟦️.ts";
import type { SemanticFacetPrimaryFileProjectionContract } from "../../../../../🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts";


//#endregion 🎮️PlaygroundSession

/** @emoji 🚦️ Cross-checks the flattened playground catalog for global uniqueness, multi-app crate discipline, and resolvable file-backed asset declarations; returns human-readable violations.
 *
 * `hostPluginIds` names the plugin(s) whose crate declares `[package.metadata.semio].host`. The
 * multi-app rule below must not fire on that crate's SHELL row: `defaultHostVariant`
 * (`🎮️playground/🔎️discovery/🟦️.ts:210-214`) states the opposite contract in its own comment — "the
 * host crate also ships ordinary artifact apps as their own single-app playgrounds (Home, Space); the
 * row that boots the SHELL is the one naming no `app`" — and it *requires* exactly one app-less row
 * there. Without this exemption the two laws contradict each other and the whole gate aborts before
 * a single taxonomy row is rendered, which is what `🪐️space` did the moment its Home/Space app
 * variants were declared beside the `s` shell. */
export function validatePlaygroundRegistry(playgrounds: PlaygroundEntry[], repoRoot: string, hostPluginIds: ReadonlySet<string>): string[] {
  const errors: string[] = [];
  const variantOwners = new Map<string, string>();
  const aliasOwners = new Map<string, string>();
  const portOwners = new Map<string, string>();
  /** @emoji 🌐️ Every individual port (single-user `ports.react`/`ports.wgpu` plus every
   * `userPorts.react[]`/`userPorts.wgpu[]` entry) across the whole catalog, keyed by the raw port
   * number — a dev machine has exactly one TCP namespace, so no two rows may ever claim the same port
   * regardless of which renderer or user slot it's for. */
  const globalPortOwners = new Map<number, string>();
  const claimGlobalPort = (port: number, label: string): void => {
    const owner = globalPortOwners.get(port);
    if (owner) errors.push(`duplicate playground port ${port} (${owner} and ${label})`);
    else globalPortOwners.set(port, label);
  };
  const entriesByCrate = new Map<string, PlaygroundEntry[]>();
  for (const entry of playgrounds) {
    claimGlobalPort(entry.ports.react, `${entry.variant} react`);
    claimGlobalPort(entry.ports.wgpu, `${entry.variant} wgpu`);
    entry.userPorts?.react.forEach((port, index) => claimGlobalPort(port, `${entry.variant} user${index + 1} react`));
    entry.userPorts?.wgpu.forEach((port, index) => claimGlobalPort(port, `${entry.variant} user${index + 1} wgpu`));
    if (variantOwners.has(entry.variant)) {
      errors.push(`duplicate playground variant "${entry.variant}" (${variantOwners.get(entry.variant)} and ${entry.cratePath})`);
    } else {
      variantOwners.set(entry.variant, entry.cratePath);
    }
    for (const alias of entry.aliases) {
      if (aliasOwners.has(alias)) {
        errors.push(`duplicate playground alias "${alias}" (variants "${aliasOwners.get(alias)}" and "${entry.variant}")`);
      } else {
        aliasOwners.set(alias, entry.variant);
      }
    }
    const portKey = `${entry.ports.react}:${entry.ports.wgpu}`;
    if (portOwners.has(portKey)) {
      errors.push(`duplicate playground ports react=${entry.ports.react}/wgpu=${entry.ports.wgpu} (variants "${portOwners.get(portKey)}" and "${entry.variant}")`);
    } else {
      portOwners.set(portKey, entry.variant);
    }
    for (const asset of entry.assets) {
      if (asset.kind === "static-dir") {
        const root = asset.root ? join(repoRoot, asset.root) : undefined;
        if (!root || !existsSync(root) || !statSync(root).isDirectory()) errors.push(`playground variant "${entry.variant}" declares missing static-dir root "${asset.root ?? ""}"`);
      }
      if (asset.kind === "mesh-collection") {
        const catalog = asset.catalog ? join(repoRoot, asset.catalog) : undefined;
        if (asset.route !== "/mesh" || !catalog || !existsSync(catalog) || !statSync(catalog).isFile()) errors.push(`playground variant "${entry.variant}" declares invalid mesh-collection catalog "${asset.catalog ?? ""}"`);
      }
    }
    entriesByCrate.set(entry.cratePath, [...(entriesByCrate.get(entry.cratePath) ?? []), entry]);
  }
  for (const group of entriesByCrate.values()) {
    if (group.length <= 1) continue;
    // 🏠️ On the host crate the single app-less row IS the shell session, required by
    // `defaultHostVariant`; a SECOND app-less row there is still a violation, and so is any app-less
    // row on a crate that declares no host table.
    const hostShellRows = group.filter((entry) => hostPluginIds.has(entry.pluginId) && !entry.app);
    for (const entry of group) {
      if (entry.app) continue;
      if (hostShellRows.length === 1 && hostShellRows[0] === entry) continue;
      errors.push(`playground variant "${entry.variant}" in ${entry.cratePath} must set "app" (crate declares ${group.length} playground entries)`);
    }
  }
  return errors;
}


//#region 🗿️TaxonomyValidator
/** @emoji 🗿️ Every artifact node must carry the completeness taxonomy component slots (incl. `🧬️mutations` + `⚙️engine`) — sourced from
 * `🔣️taxonomy.json` (single vocabulary source of truth, see master ticket
 * `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`; this used to be an independently
 * hand-maintained copy, which is exactly the drift `🔣️taxonomy.json` exists to prevent). */
export const TAXONOMY_ARTIFACT_COMPONENTS = TAXONOMY.artifactComponentDirs;

/** @emoji 🪆️ The facets a SUBSET owns (`🔣️taxonomy.json` `subsetComponentDirs`) — where `🧬️schema`
 * and `🚪️io` actually live since the W3 standards/subsets nesting. */
export const TAXONOMY_SUBSET_COMPONENTS = TAXONOMY.subsetComponentDirs;

export const TAXONOMY_MUTATION_COMPONENT_FILENAME = primaryFilenameForKind(TAXONOMY.mutationComponentFileKindId);

export const TAXONOMY_MUTATION_DESCRIPTOR_FILENAME = primaryFilenameForKind(TAXONOMY.mutationDescriptorFileKindId);

export const TAXONOMY_MUTATION_FACET_DIRS = [...TAXONOMY.mutationBehaviorFacetDirs, ...TAXONOMY.mutationOrganizationalFacetDirs];

export const TAXONOMY_SCHEMA_CHILD_DIRS = TAXONOMY.schemaChildDirs ?? [];

export const TAXONOMY_REPRESENTATION_DIRS = TAXONOMY.representationDirs ?? [];

export const TAXONOMY_CONFIG_CHILD_DIRS = TAXONOMY.configChildDirs ?? [];

export const TAXONOMY_PRESENCE_CHILD_DIRS = TAXONOMY.presenceChildDirs ?? [];

/** @emoji 🎭️ A mode owns its windows plus its own three state lanes — the completeness set the
 * taxonomy declares for every `🎭️modes/<mode>/` node. */
export const TAXONOMY_MODE_CHILDREN = TAXONOMY.modeChildDirs ?? [];

/** @emoji 🧬️ Leaf filenames one `🧬️schema` facet must carry, resolved through the facet's **declared
 * kind** (`🔣️taxonomy.json` `schemaFacetKinds`), never through the flat `schemaFormats` map. A
 * `🧬️data` facet owns the five data projections; `📜️.wit` belongs to the `📜️interface` kind, which the
 * taxonomy grants to exactly the two enumerated `facetPathIdentities` — so demanding a `📜️.wit` leaf
 * from every `🎚️config`/`👥️presence`/artifact schema facet contradicts the taxonomy that defines them.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️.ts `schemaFacetFormatEntries` */
export function taxonomySchemaFilenames(facetAbs: string): string[] {
  const facetRel = relative(getWorkspaceRoot(), facetAbs).replaceAll("\\", "/");
  return schemaFacetFormatEntries(facetRel, TAXONOMY).map(([, format]) => primaryFilenameForKind(format.fileKindId));
}

export const MUTATIONS_FACET_DIR = "🧬️mutations";

export const ENGINE_FACET_DIR = "⚙️engine";

export const SNAPSHOT_FACET_DIR = "📸️snapshot";

export const DIFF_FACET_DIR = "🔺️diff";

export const SCHEMA_FACET_DIR = "🧬️schema";

export const CONFIG_FACET_DIR = "🎚️config";

export const PRESENCE_FACET_DIR = "👥️presence";

export const IO_FACET_DIR = "🚪️io";

export const LEGACY_CONFIG_FACET_DIR = "🧮️config";

export const TAXONOMY_IO_DIRECTION_DIRS = TAXONOMY.ioDirectionDirs ?? [];

export const TAXONOMY_IO_DIRECTION_CHILD_DIRS = TAXONOMY.ioDirectionChildDirs ?? {};

export const BUILDER_FACET_DIR = "🏗️builder";

export const DECOMPOSER_FACET_DIR = "🪓️decomposer";

export const LEGACY_WASM_DIR = "🕸️wasm";

export const TAXONOMY_TS_LEAF_FILENAME = primaryFilenameForKind(TAXONOMY.ecosystems["🟦️typescript"].componentFileKindId);

/** @emoji 🪟️ A window dir may only contain these children, each itself a `🦀️.rs` leaf. */
export const TAXONOMY_WINDOW_CHILDREN = new Set(TAXONOMY.windowChildDirs);

/** @emoji 🧪️ The test-ownership dirs any taxonomy owner may carry beside its facets. `🔣️taxonomy.json`
 * `testOwnerKinds` names `🪟️windows` an owner kind, so a window owns its own `🧪️tests`/`🧫️fixtures`
 * exactly as a subset or a surface does; they are ownership dirs, never window facets. */
export const TAXONOMY_TEST_OWNERSHIP_DIRS: ReadonlySet<string> = new Set(TAXONOMY.testOwnerKinds.includes(TAXONOMY.windowsDirName) ? [TAXONOMY.testsDirName, TAXONOMY.testFixturesDirName] : []);

export const TAXONOMY_LEAF_FILENAME = primaryFilenameForKind(TAXONOMY.ecosystems[RUST_LANG].componentFileKindId);

/** 🥒️ Gherkin feature leaf that marks a directory as a repository test-platform case. */
export const TEST_FEATURE_FILENAME = primaryFilenameForKind(TAXONOMY.testFeatureFileKindId);

/** @emoji 🧭️ One `semantic-facet-primary-file` projection contract, read through its own discriminant.
 *
 * `semanticOwnedFileProjectionContracts` holds a union, and its `exact-owner-path-catalog` member
 * names `sourceBasenames` rather than one `sourceFilename` — so a lookup that simply reads
 * `.sourceFilename` is asking a contract that may not have one. Refusing by name here keeps the
 * taxonomy the single authority and makes a retired contract id a loud failure. */
function semanticFacetPrimaryProjection(contractId: string): SemanticFacetPrimaryFileProjectionContract {
  const contract = TAXONOMY.semanticOwnedFileProjectionContracts[contractId];
  if (contract === undefined) throw new Error(`taxonomy declares no ${contractId} owned-file projection contract`);
  if (contract.contractKind !== "semantic-facet-primary-file") throw new Error(`${contractId} is a ${contract.contractKind} contract, not a semantic-facet-primary-file one`);
  return contract;
}

/** 📌️ The tracked marker that makes a lane DELIBERATELY empty rather than missing.
 *
 * Resolved from `semanticOwnedFileProjectionContracts["artifact-empty-facet-primary-markdown-v1"]`
 * (`🔣️taxonomy.json:16824-16828`), never spelled here, so the marker stays one piece of vocabulary.
 * A plugin-root lane carrying only this marker publishes nothing on purpose: 33 of the 34 plugins
 * declare no plugin-scope command, and a stub `🦀️.rs` for each would put 33 entries into
 * `PluginManifest::commands` that no plugin actually offers. */
export const PLUGIN_EMPTY_LANE_FILENAME = semanticFacetPrimaryProjection("artifact-empty-facet-primary-markdown-v1").sourceFilename;

/** @emoji 🚪️ Rust entry filename and its Shape V2 home relative to the owner root. */
export const RUST_LIBRARY_ENTRY_CONTRACT_ID = TAXONOMY.ecosystems[RUST_LANG].entryContractIds.find((contractId) => TAXONOMY.configurableEntryContracts[contractId]?.role === "library");

if (!RUST_LIBRARY_ENTRY_CONTRACT_ID) throw new Error("📇️registry: Rust ecosystem must declare a library entry contract");

export const RUST_ENTRY_FILENAME = TAXONOMY.configurableEntryContracts[RUST_LIBRARY_ENTRY_CONTRACT_ID].filename;

export const RUST_ENTRY_DIR_FROM_OWNER = TAXONOMY.rustEntryPathRules.entryDirFromOwner.split("/");

/** @emoji 📌️ The empty-facet marker a surface/mode/window lane carries, resolved from the SAME
 * projection contract as `PLUGIN_EMPTY_LANE_FILENAME`. `windowEmptyFacetFileKindId` names the file
 * KIND (`markdown`), whose primary filename is the generic `📝️.md`; the contract names the FILE
 * (`📌️.empty.md`) and cites that kind as its `fileKindAuthority`. Deriving the name from the kind
 * dropped the contract's own spelling, so `new surface` wrote a marker no lane on disk uses — 4265
 * markers are `📌️.empty.md` and the only `📝️.md` lanes in the plugin tree came from that drift. */
export const WINDOW_EMPTY_FACET_FILENAME = PLUGIN_EMPTY_LANE_FILENAME;


/** @emoji 🧭️ Plugin roots discovered via the shared package contract (`role = "plugin"`, rust, owner
 * sitting directly under one of `taxonomy.pluginAreas`). Drives the taxonomy tree audit below. */
export function findNewContractPluginRoots(repoRoot: string): { pluginId: string; pluginRoot: string }[] {
  return discoverPackages(repoRoot, TAXONOMY)
    .filter((pkg) => pkg.role === "plugin" && pkg.lang === RUST_LANG && PLUGIN_AREAS.includes(dirname(pkg.ownerRel)))
    .map((pkg) => ({ pluginId: basename(pkg.ownerRel), pluginRoot: join(repoRoot, pkg.ownerRel) }))
    .sort((a, b) => a.pluginId.localeCompare(b.pluginId));
}


export function listDirs(dir: string): string[] {
  if (!existsSync(dir)) return [];
  return readdirSync(dir).filter((name) => statSync(join(dir, name)).isDirectory());
}


/** @emoji 👁️✏️ Every `👁️viewer`/`✏️editor` surface dir under one plugin's
 * `🗿️artifacts/<a>/🏅️standards/<s>/🪆️subsets/<sub>/` tree — the W3 dissolution replacement for the old
 * `🎛️apps/<app>/` root. Paired with a `"<subset>/<roleDirName>"` label for findings. */
export function surfaceDirsForPlugin(pluginRoot: string): { abs: string; label: string }[] {
  const out: { abs: string; label: string }[] = [];
  const artifactsAbs = join(pluginRoot, TAXONOMY.artifactsDirName);
  for (const kind of listDirs(artifactsAbs)) {
    const standardsAbs = join(artifactsAbs, kind, TAXONOMY.standardsDirName);
    for (const std of listDirs(standardsAbs)) {
      const subsetsAbs = join(standardsAbs, std, TAXONOMY.subsetsDirName);
      for (const sub of listDirs(subsetsAbs)) {
        for (const role of TAXONOMY.surfaceRoles) {
          const dirName = TAXONOMY.surfaceDirNames[role];
          const abs = join(subsetsAbs, sub, dirName);
          if (existsSync(abs)) out.push({ abs, label: `${sub}/${dirName}` });
        }
      }
    }
  }
  return out;
}


/** 🪃️ Whether one `#[path]` mount leaves the owner root it is declared under.
 *
 * `validateRustTaxonomyMounts` builds its module graph from the files under ONE owner root, so a
 * mount that climbs out of that root (a plugin artifact mounting a shared `✏️s/🔨️modules/<m>/⚙️engine`
 * leaf, for example) can never appear in `graph.targets` no matter how correct it is. Judging such a
 * mount here would report a file that exists on disk as missing; its membership belongs to the audit
 * of the area that owns it. */
function escapesOwnerRoot(ownerRoot: string, mountingFileRel: string, pathTarget: string): boolean {
  const resolved = relative(ownerRoot, resolve(dirname(join(ownerRoot, mountingFileRel)), pathTarget));
  return resolved === ".." || resolved.startsWith("../") || resolved.startsWith("..\\");
}


/** 🥒️ The repository test platform's own case shape: a directory directly under a `🧪️tests` owner dir
 * that carries the feature file. `discoverTestCases`
 * (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟦️.ts:545`) recognises exactly this, and the Rust
 * adapter beside the feature is compiled by a GENERATED cache-local crate that mounts it by absolute
 * `#[path]` (`…/🧪️test/🖥️host/🏗️materialization/🟦️.ts:132`) and links `semio-repo-test-host`. No plugin
 * Cargo manifest owns it — by design, since the host crate must be able to link an oracle role
 * WITHOUT the subject — so Cargo reachability is the wrong question to ask of it here. */
function isRepositoryTestCaseAdapter(caseDir: string): boolean {
  return basename(dirname(caseDir)) === TAXONOMY.testsDirName && existsSync(join(caseDir, TEST_FEATURE_FILENAME));
}


/** 📎️ Every `.rs` file textually expanded into an already-owned source by `include!`, transitively.
 *
 * `include!` is a compilation edge the module graph cannot carry: the expansion becomes part of the
 * including file's module rather than a `mod` of its own, so the included leaf never appears in
 * `graph.contexts` however correctly it compiles. Judging it as unreachable would report a file
 * `rustc` demonstrably compiles. `rustc` resolves each literal against the directory of the file the
 * invocation sits in, which is what `dirname(current)` reproduces here. */
function includeClosure(pluginRoot: string, ownedSources: readonly string[], known: ReadonlySet<string>): Set<string> {
  const reached = new Set<string>();
  const pending = [...ownedSources];
  for (let index = 0; index < pending.length; index++) {
    const current = pending[index]!;
    let facts;
    try {
      facts = inspectRustModuleGraphFacts(readFileSync(join(pluginRoot, current), "utf8"));
    } catch {
      continue;
    }
    for (const include of facts.includes) {
      const target = relative(pluginRoot, resolve(join(pluginRoot, dirname(current)), include.path)).replaceAll("\\", "/");
      if (target.startsWith("..") || !known.has(target) || reached.has(target)) continue;
      reached.add(target);
      pending.push(target);
    }
  }
  return reached;
}


/** 🕸️ Verifies taxonomy components through the exact Cargo-owned recursive module graph.
 *
 * `manifestFiles` names every Cargo manifest that owns compilation under this root — the plugin's
 * own `📦️packages/🦀️rust/Cargo.toml` plus each nested artifact package's, since an artifact that
 * ships as its own crate mounts its subtree from its own `[lib]` and never from the plugin's.
 * Defaults to the owner manifest alone, which is what the single-crate mount oracle fixtures feed. */
export function validateRustTaxonomyMounts(pluginRoot: string, pluginId: string, componentFiles: readonly string[], sourceFiles: readonly string[], manifestFiles?: readonly string[]): string[] {
  const manifest = [...RUST_ENTRY_DIR_FROM_OWNER, "Cargo.toml"].join("/");
  if (!existsSync(join(pluginRoot, manifest))) return [pluginId + ": missing Cargo manifest " + manifest];
  const manifests = [...new Set([manifest, ...(manifestFiles ?? [])])];
  const sources = sourceFiles.map((path) => relative(pluginRoot, path).replaceAll("\\", "/"));
  const graph = inspectRustModuleGraph([...sources, ...manifests], (path) => readFileSync(join(pluginRoot, path), "utf8"), { strictManifests: true });
  if (graph.invalidManifests.has(manifest)) return [pluginId + ": invalid Cargo manifest " + manifest];
  const owned = (context: { manifestPath: string | null }): boolean => context.manifestPath !== null && manifests.includes(context.manifestPath);
  const expanded = includeClosure(pluginRoot, [...graph.contexts].filter(([, rows]) => rows.some(owned)).map(([path]) => path), new Set(sources));
  const findings = new Set<string>();
  if (![...graph.contexts.values()].some((rows) => rows.some((context) => context.manifestPath === manifest))) findings.add(pluginId + ": missing module target for Cargo library " + manifest);
  for (const file of componentFiles) {
    const path = relative(pluginRoot, file).replaceAll("\\", "/");
    if (!graph.contexts.get(path)?.some(owned) && !expanded.has(path)) findings.add(pluginId + ": " + path + " is not reachable from Cargo manifest " + manifest);
  }
  for (const [path, contexts] of graph.contexts) {
    const facts = inspectRustModuleGraphFacts(readFileSync(join(pluginRoot, path), "utf8"));
    for (const context of contexts.filter(owned)) {
      for (const module of facts.modules) {
        if (module.inline || module.conditional || module.modulePath.length !== context.sourceScope.length + 1 || module.modulePath.slice(0, -1).join("::") !== context.sourceScope.join("::")) continue;
        if (module.pathTarget !== null && escapesOwnerRoot(pluginRoot, path, module.pathTarget)) continue;
        const key = context.crateRoot + "\0" + [...context.modulePath, module.name].join("::");
        if (!graph.targets.has(key)) findings.add(pluginId + ": missing module target " + JSON.stringify(module.pathTarget ?? module.name) + " from " + path);
      }
    }
  }
  return [...findings].sort();
}


/** 🌳️ Validates the direct plugin contract owner against the taxonomy. */
export function validatePluginContractRoot(pluginRoot: string, pluginId: string): string[] {
  const findings: string[] = [];
  const nestedPluginContract = join(pluginRoot, "🔌️plugin");
  if (existsSync(nestedPluginContract)) {
    findings.push(`${pluginId}: move the redundant 🔌️plugin contract and facets directly into the plugin root, then remove 🔌️plugin/`);
  }
  if (!existsSync(join(pluginRoot, TAXONOMY_LEAF_FILENAME))) {
    findings.push(`${pluginId}: plugin root is missing ${TAXONOMY_LEAF_FILENAME}`);
  }
  for (const child of TAXONOMY.pluginRequiredChildDirs) {
    const lane = join(pluginRoot, child);
    if (existsSync(join(lane, TAXONOMY_LEAF_FILENAME)) || existsSync(join(lane, PLUGIN_EMPTY_LANE_FILENAME))) continue;
    findings.push(`${pluginId}: plugin root is missing ${child}/${TAXONOMY_LEAF_FILENAME} or ${child}/${PLUGIN_EMPTY_LANE_FILENAME}`);
  }

  return findings;
}


/** @emoji 🚦️ Structural audit of one migrated plugin's taxonomy tree, entirely against
 * `🔣️taxonomy.json`'s vocabulary. Severity is decided by the caller from the plugin area's declared
 * maturity: warn while it is `legacy`/`mixed`, hard failure once it is `clean`. */
export function validateTaxonomyTree(pluginRoot: string, pluginId: string): string[] {
  const findings: string[] = [];

  const artifactsDir = join(pluginRoot, TAXONOMY.artifactsDirName);
  for (const artifact of listDirs(artifactsDir)) {
    // 🏅️ `🔣️taxonomy.json` states the ownership chain and this walk follows it exactly: artifacts own
    // standards only (`newArtifactComponentDirs`), standards own subsets only
    // (`standardComponentDirs`), and the SUBSET owns schema, IO and examples
    // (`subsetComponentDirs`/`subsetChildDirs`). `_standardsSubsetsComment` also forbids an artifact
    // ⚙️engine outright — "Subsets own schema, IO, and examples — never an engine … behaviour belongs
    // to the app that edits it (26/08/12/ENGINELESS-ARTIFACTS-AND-APP-STATE-MACHINES)" — which is why
    // no engine facet is required here, matching the surface walk's own note below.
    const standardsDir = join(artifactsDir, artifact, TAXONOMY.standardsDirName);
    if (!existsSync(standardsDir)) {
      findings.push(`${pluginId}: artifact "${artifact}" is missing ${TAXONOMY.standardsDirName}/`);
      continue;
    }
    for (const standard of listDirs(standardsDir)) {
      if (standard === TAXONOMY.packagesDirName) continue;
      const subsetsDir = join(standardsDir, standard, TAXONOMY.subsetsDirName);
      if (!existsSync(subsetsDir)) {
        findings.push(`${pluginId}: artifact "${artifact}" standard "${standard}" is missing ${TAXONOMY.subsetsDirName}/`);
        continue;
      }
      for (const subset of listDirs(subsetsDir)) {
        const subsetDir = join(subsetsDir, subset);
        const owner = `artifact "${artifact}" subset "${standard}/${subset}"`;
        for (const component of TAXONOMY_SUBSET_COMPONENTS) {
          const facetDir = join(subsetDir, component);
          // Soft-require builder/decomposer until W5/W6 migrate every artifact (vocabulary is already strict).
          if (component === BUILDER_FACET_DIR || component === DECOMPOSER_FACET_DIR) {
            if (existsSync(facetDir)) {
              if (!existsSync(join(facetDir, TAXONOMY_LEAF_FILENAME))) {
                findings.push(`${pluginId}: ${owner} is missing ${component}/${TAXONOMY_LEAF_FILENAME}`);
              }
              if (!existsSync(join(facetDir, TAXONOMY_TS_LEAF_FILENAME))) {
                findings.push(`${pluginId}: ${owner} is missing ${component}/${TAXONOMY_TS_LEAF_FILENAME}`);
              }
            }
            continue;
          }
          if (component === IO_FACET_DIR) {
            if (!existsSync(facetDir)) {
              findings.push(`${pluginId}: ${owner} is missing ${component}/`);
            } else if (!existsSync(join(facetDir, TAXONOMY_LEAF_FILENAME))) {
              findings.push(`${pluginId}: ${owner} is missing ${component}/${TAXONOMY_LEAF_FILENAME}`);
            }
            continue;
          }
          if (component === SCHEMA_FACET_DIR) {
            if (!existsSync(facetDir)) {
              findings.push(`${pluginId}: ${owner} is missing ${component}/`);
            }
            continue;
          }
          if (!existsSync(join(facetDir, TAXONOMY_LEAF_FILENAME))) {
            findings.push(`${pluginId}: ${owner} is missing ${component}/${TAXONOMY_LEAF_FILENAME}`);
          }
          if (!existsSync(join(facetDir, TAXONOMY_TS_LEAF_FILENAME))) {
            findings.push(`${pluginId}: ${owner} is missing ${component}/${TAXONOMY_TS_LEAF_FILENAME}`);
          }
        }
        //#region NestedFacetWalk
        // 🚧️ Presence-tolerant schema tree + io direction shape. STILL ARTIFACT-ROOTED, deliberately:
        // the nested matrix below (io codec leaves directly under `<format>/`, one Rust leaf per
        // mutation facet, no `🧪️tests`/`🧫️fixtures` anywhere) describes the pre-W3 tree, while the live
        // subset carries `<format>/🔖️<standard>/✳️<subset>/🦀️.rs`, json-kind mutation `🧬️schema/`
        // payloads and test dirs throughout. Re-pointing it at `subsetDir` before it is rewritten
        // turns 92 stale findings into ~9,000 across all 33 plugins — see
        // `26/09/09/PROCEDURAL-3D-END-TO-END/📓️taxonomy-fix-2026-09-10.md` §3 for the measurement.
        const schemaDir = join(artifactsDir, artifact, SCHEMA_FACET_DIR);
        if (existsSync(schemaDir)) {
          for (const filename of taxonomySchemaFilenames(schemaDir)) {
            if (!existsSync(join(schemaDir, filename))) {
              findings.push(`${pluginId}: ${owner} is missing ${SCHEMA_FACET_DIR}/${filename}`);
            }
          }
          for (const child of listDirs(schemaDir)) {
            if (TAXONOMY_SCHEMA_CHILD_DIRS.includes(child)) {
              const childDir = join(schemaDir, child);
              for (const rep of listDirs(childDir)) {
                if (TAXONOMY_REPRESENTATION_DIRS.includes(rep)) continue;
                if (child === MUTATIONS_FACET_DIR) {
                  const mutationDir = join(childDir, rep);
                  if (!existsSync(join(mutationDir, TAXONOMY_MUTATION_COMPONENT_FILENAME))) {
                    findings.push(`${pluginId}: ${owner} mutation "${rep}" is missing ${SCHEMA_FACET_DIR}/${MUTATIONS_FACET_DIR}/${rep}/${TAXONOMY_MUTATION_COMPONENT_FILENAME}`);
                  }
                  if (!existsSync(join(mutationDir, TAXONOMY_MUTATION_DESCRIPTOR_FILENAME))) {
                    findings.push(`${pluginId}: ${owner} mutation "${rep}" is missing ${SCHEMA_FACET_DIR}/${MUTATIONS_FACET_DIR}/${rep}/${TAXONOMY_MUTATION_DESCRIPTOR_FILENAME}`);
                  }
                  for (const facet of TAXONOMY_MUTATION_FACET_DIRS) {
                    const facetDir = join(mutationDir, facet);
                    if (existsSync(facetDir) && !existsSync(join(facetDir, TAXONOMY_LEAF_FILENAME))) {
                      findings.push(`${pluginId}: ${owner} mutation "${rep}" optional facet "${facet}" is missing ${SCHEMA_FACET_DIR}/${MUTATIONS_FACET_DIR}/${rep}/${facet}/${TAXONOMY_LEAF_FILENAME}`);
                    }
                  }
                  continue;
                }
                findings.push(`${pluginId}: ${owner} has undeclared ${SCHEMA_FACET_DIR}/${child}/${rep}`);
              }
              continue;
            }
            if (child === TAXONOMY.packagesDirName) continue;
            // allow schema format leaves at schema root; dirs must be schemaChildDirs
            findings.push(`${pluginId}: ${owner} has undeclared ${SCHEMA_FACET_DIR}/${child}`);
          }
        }
        const ioFacetDir = join(artifactsDir, artifact, IO_FACET_DIR);
        if (existsSync(ioFacetDir)) {
          for (const direction of listDirs(ioFacetDir)) {
            if (!TAXONOMY_IO_DIRECTION_DIRS.includes(direction)) {
              findings.push(`${pluginId}: ${owner} has undeclared ${IO_FACET_DIR}/${direction}`);
              continue;
            }
            const expected = TAXONOMY_IO_DIRECTION_CHILD_DIRS[direction];
            const directionDir = join(ioFacetDir, direction);
            for (const codec of listDirs(directionDir)) {
              if (expected && codec === expected) {
                const artsDir = join(directionDir, codec, TAXONOMY.artifactsDirName);
                if (existsSync(artsDir)) {
                  for (const stdioArt of listDirs(artsDir)) {
                    if (!existsSync(join(artsDir, stdioArt, TAXONOMY_LEAF_FILENAME))) {
                      findings.push(`${pluginId}: ${owner} is missing ${IO_FACET_DIR}/${direction}/${codec}/${TAXONOMY.artifactsDirName}/${stdioArt}/${TAXONOMY_LEAF_FILENAME}`);
                    }
                  }
                }
                continue;
              }
              findings.push(`${pluginId}: ${owner} has undeclared ${IO_FACET_DIR}/${direction}/${codec}`);
            }
          }
        }
        //#endregion NestedFacetWalk
        //#region DirectMutations
        // 🧬️ Walk 🧬️schema/🧬️mutations/<semantic-mutation>/ with one mandatory direct component.
        const mutationsRoot = join(artifactsDir, artifact, SCHEMA_FACET_DIR, MUTATIONS_FACET_DIR);
        if (existsSync(mutationsRoot)) {
          for (const mutation of listDirs(mutationsRoot)) {
            if (mutation === TAXONOMY.packagesDirName) continue;
            const mutationDir = join(mutationsRoot, mutation);
            if (!existsSync(join(mutationDir, TAXONOMY_MUTATION_COMPONENT_FILENAME))) {
              findings.push(`${pluginId}: ${owner} mutation "${mutation}" is missing ${MUTATIONS_FACET_DIR}/${mutation}/${TAXONOMY_MUTATION_COMPONENT_FILENAME}`);
            }
            if (!existsSync(join(mutationDir, TAXONOMY_MUTATION_DESCRIPTOR_FILENAME))) {
              findings.push(`${pluginId}: ${owner} mutation "${mutation}" is missing ${MUTATIONS_FACET_DIR}/${mutation}/${TAXONOMY_MUTATION_DESCRIPTOR_FILENAME}`);
            }
            for (const facet of TAXONOMY_MUTATION_FACET_DIRS) {
              const facetDir = join(mutationDir, facet);
              if (existsSync(facetDir) && !existsSync(join(facetDir, TAXONOMY_LEAF_FILENAME))) {
                findings.push(`${pluginId}: ${owner} mutation "${mutation}" optional facet "${facet}" is missing ${MUTATIONS_FACET_DIR}/${mutation}/${facet}/${TAXONOMY_LEAF_FILENAME}`);
              }
            }
          }
        }
        // ⚙️engine is FORBIDDEN here, not required: `🔣️taxonomy.json`'s `_standardsSubsetsComment` says
        // a subset owns "schema, IO, and examples — never an engine", and `subsetChildDirs` carries no
        // entry for one. The word stays globally legal (`taxonomyLeafParentDirs`) only one level up, in
        // a module's own ⚙️engine.
        if (existsSync(join(subsetDir, ENGINE_FACET_DIR))) {
          findings.push(`${pluginId}: ${owner} has a forbidden ${ENGINE_FACET_DIR}/ — an artifact is data plus pure transforms; move the algorithm into the owning 🔨️modules/<module>/${ENGINE_FACET_DIR}/`);
        }
        //#endregion DirectMutations
        const examplesRoot = join(subsetDir, EXAMPLES_DIRNAME);
        if (!existsSync(examplesRoot)) {
          findings.push(`${pluginId}: ${owner} is missing ${EXAMPLES_DIRNAME}/`);
          continue;
        }
        const exampleSets = listDirs(examplesRoot).filter((name) => name !== TAXONOMY.testsDirName && name !== TAXONOMY.testFixturesDirName);
        if (exampleSets.length === 0) {
          findings.push(`${pluginId}: ${owner} ${EXAMPLES_DIRNAME} has no example slug`);
          continue;
        }
        for (const exampleSet of exampleSets) {
          if (!isExampleSlugName(exampleSet)) {
            findings.push(`${pluginId}: ${owner} example "${exampleSet}" is not a valid emoji+VS16+kebab slug`);
          }
          for (const plural of FORBIDDEN_EXAMPLE_PLURAL_DIRS) {
            if (existsSync(join(examplesRoot, exampleSet, plural))) {
              findings.push(`${pluginId}: ${owner} example "${exampleSet}" still has plural ${plural}/`);
            }
          }
          if (!existsSync(join(examplesRoot, exampleSet, EXAMPLE_RUST_LEAF))) {
            findings.push(`${pluginId}: ${owner} example "${exampleSet}" is missing ${EXAMPLE_RUST_LEAF}`);
          }
          if (!existsSync(join(examplesRoot, exampleSet, EXAMPLE_TS_LEAF))) {
            findings.push(`${pluginId}: ${owner} example "${exampleSet}" is missing ${EXAMPLE_TS_LEAF}`);
          }
          if (!existsSync(join(examplesRoot, exampleSet, EXAMPLE_ASSETS_DIRNAME))) {
            findings.push(`${pluginId}: ${owner} example "${exampleSet}" is missing ${EXAMPLE_ASSETS_DIRNAME}/`);
          }
          if (!existsSync(join(examplesRoot, exampleSet, EXAMPLE_TESTS_DIRNAME))) {
            findings.push(`${pluginId}: ${owner} example "${exampleSet}" is missing ${EXAMPLE_TESTS_DIRNAME}/`);
          }
        }
      }
    }
  }

  if (existsSync(join(pluginRoot, EXAMPLES_DIRNAME))) {
    findings.push(`${pluginId}: plugin-root ${EXAMPLES_DIRNAME}/ is forbidden — relocate under 🗿️artifacts/<artifact>/${TAXONOMY.standardsDirName}/<standard>/${TAXONOMY.subsetsDirName}/<subset>/${EXAMPLES_DIRNAME}`);
  }

  // 👁️✏️ Surfaces replace 🎛️apps (W3 dissolution, ticket 26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-
  // SUBSET). No ⚙️engine facet requirement — the ENGINELESS ticket moved engine ownership to the
  // module level, and surfaceChildDirs carries no ⚙️engine entry. `📚️examples` is optional (contract
  // §7.5, not in surfaceRequiredChildDirs) — only its CONTENTS are validated when present, never its
  // absence.
  const surfaceDirs = surfaceDirsForPlugin(pluginRoot);
  for (const { abs: surfaceAbs, label } of surfaceDirs) {
    const surfaceExamples = join(surfaceAbs, EXAMPLES_DIRNAME);
    if (!existsSync(surfaceExamples)) continue;
    // 🧪️ `🧪️tests`/`🧫️fixtures` beside the example slugs are the examples' OWN test owners, not a
    // fourth example — the same exclusion the subset-level walk makes below. Without it a surface's
    // `📚️examples/🧪️tests/` is asked for a `🦀️.rs`, a `🟦️.ts`, an `🖼️assets/` and a `🧪️tests/` of its
    // own, and `🧪️tests` even passes the emoji+VS16+kebab slug pattern, so nothing else catches it.
    const surfaceSets = listDirs(surfaceExamples).filter((name) => name !== TAXONOMY.testsDirName && name !== TAXONOMY.testFixturesDirName);
    if (surfaceSets.length === 0) {
      findings.push(`${pluginId}: surface "${label}" ${EXAMPLES_DIRNAME} has no example slug`);
    }
    for (const exampleSet of surfaceSets) {
      if (!isExampleSlugName(exampleSet)) {
        findings.push(`${pluginId}: surface "${label}" example "${exampleSet}" is not a valid emoji+VS16+kebab slug`);
      }
      for (const plural of FORBIDDEN_EXAMPLE_PLURAL_DIRS) {
        if (existsSync(join(surfaceExamples, exampleSet, plural))) {
          findings.push(`${pluginId}: surface "${label}" example "${exampleSet}" still has plural ${plural}/`);
        }
      }
      if (!existsSync(join(surfaceExamples, exampleSet, EXAMPLE_RUST_LEAF))) {
        findings.push(`${pluginId}: surface "${label}" example "${exampleSet}" is missing ${EXAMPLE_RUST_LEAF}`);
      }
      if (!existsSync(join(surfaceExamples, exampleSet, EXAMPLE_TS_LEAF))) {
        findings.push(`${pluginId}: surface "${label}" example "${exampleSet}" is missing ${EXAMPLE_TS_LEAF}`);
      }
      if (!existsSync(join(surfaceExamples, exampleSet, EXAMPLE_ASSETS_DIRNAME))) {
        findings.push(`${pluginId}: surface "${label}" example "${exampleSet}" is missing ${EXAMPLE_ASSETS_DIRNAME}/`);
      }
      if (!existsSync(join(surfaceExamples, exampleSet, EXAMPLE_TESTS_DIRNAME))) {
        findings.push(`${pluginId}: surface "${label}" example "${exampleSet}" is missing ${EXAMPLE_TESTS_DIRNAME}/`);
      }
    }
  }

  // 🪟️ windows live under <surface>/modes/<mode>/🪟️windows/<w> and may only contain the fixed child set.
  for (const { abs: surfaceAbs, label } of surfaceDirs) {
    const modesDir = join(surfaceAbs, TAXONOMY.modesDirName);
    for (const mode of listDirs(modesDir)) {
      // 🎭️ A mode declares its windows plus its own 🎚️config / 👥️presence / 🫧️transient lanes; an
      // empty lane is valid (it carries only the tracked marker), an absent lane is not.
      for (const child of TAXONOMY_MODE_CHILDREN) {
        if (existsSync(join(modesDir, mode, child))) continue;
        findings.push(`${pluginId}: mode "${label}/${mode}" is missing required child "${child}"`);
      }
      const windowsDir = join(modesDir, mode, TAXONOMY.windowsDirName);
      for (const w of listDirs(windowsDir)) {
        for (const child of listDirs(join(windowsDir, w))) {
          if (TAXONOMY_TEST_OWNERSHIP_DIRS.has(child)) continue;
          if (!TAXONOMY_WINDOW_CHILDREN.has(child)) {
            findings.push(`${pluginId}: window "${label}/${mode}/${w}" has unexpected child "${child}" (expected one of ${[...TAXONOMY_WINDOW_CHILDREN].join(", ")})`);
          }
        }
      }
    }
  }

  // 🦀️ collect every actual component.rs on disk (for the lib.rs cross-check below) and flag any
  // taxonomy leaf file that isn't literally named `component.rs`.
  const componentFiles: string[] = [];
  const sourceFiles: string[] = [];
  const manifestFiles: string[] = [];
  const taxonomyIoChildDirs = Object.values(TAXONOMY_IO_DIRECTION_CHILD_DIRS).flatMap((v) =>
    Array.isArray(v) ? v : [String(v)],
  );
  const taxonomyLeafParents = new Set<string>([
    ...TAXONOMY_ARTIFACT_COMPONENTS,
    ...TAXONOMY_WINDOW_CHILDREN,
    ...TAXONOMY_MUTATION_FACET_DIRS,
    ...TAXONOMY_SCHEMA_CHILD_DIRS,
    ...TAXONOMY_REPRESENTATION_DIRS,
    ...taxonomyIoChildDirs,
  ]);
  function walkPluginTree(dir: string) {
    for (const name of readdirSync(dir)) {
      if (name.startsWith(".") || isDiscoverySkipDirectory(name)) continue;
      const path = join(dir, name);
      if (statSync(path).isDirectory()) {
        walkPluginTree(path);
        continue;
      }
      if (name === "Cargo.toml") manifestFiles.push(relative(pluginRoot, path).replaceAll("\\", "/"));
      if (!name.endsWith(".rs")) continue;
      sourceFiles.push(path);
      if (name === TAXONOMY_LEAF_FILENAME || name === EXAMPLE_RUST_LEAF) {
        if (!isRepositoryTestCaseAdapter(dir)) componentFiles.push(path);
      } else {
        const parts = dir.replaceAll("\\", "/").split("/");
        const parent = parts[parts.length - 1] ?? "";
        const isExampleSlugParent = parts.length >= 2 && parts[parts.length - 2] === EXAMPLES_DIRNAME;
        if (taxonomyLeafParents.has(parent) || isExampleSlugParent) {
          const expected = isExampleSlugParent ? EXAMPLE_RUST_LEAF : TAXONOMY_LEAF_FILENAME;
          if (name !== expected) {
            findings.push(`${pluginId}: taxonomy leaf file must be named ${expected}, found ${relative(pluginRoot, path)}`);
          }
        }
      }
    }
  }
  walkPluginTree(pluginRoot);

  findings.push(...validateRustTaxonomyMounts(pluginRoot, pluginId, componentFiles, sourceFiles, manifestFiles));

  // 🚫️ no `📡️protocol` path segment may remain under a migrated plugin (renamed to `📡️spr`).
  function containsProtocolSegment(dir: string): boolean {
    for (const name of readdirSync(dir)) {
      if (name.startsWith(".") || name === "target" || name === "node_modules") continue;
      const path = join(dir, name);
      if (!statSync(path).isDirectory()) continue;
      if (name === "📡️protocol" || containsProtocolSegment(path)) return true;
    }
    return false;
  }
  if (containsProtocolSegment(pluginRoot)) findings.push(`${pluginId}: found a "📡️protocol" path segment under the plugin dir (renamed to 📡️spr)`);

  findings.push(...validatePluginContractRoot(pluginRoot, pluginId));

  //#region SurfaceFacetWalk
  // 🎛 Walk every 🎚️config owner (surface-level and plugin-level) and its sibling 👥️presence, requiring all five schemaFormats leaves.
  const assertAppSchemaOwner = (ownerLabel: string, parentAbs: string): void => {
    const configAbs = join(parentAbs, CONFIG_FACET_DIR);
    if (!existsSync(configAbs)) return;
    for (const child of TAXONOMY_CONFIG_CHILD_DIRS) {
      const childAbs = join(configAbs, child);
      if (!existsSync(childAbs)) {
        findings.push(`${pluginId}: ${ownerLabel} is missing ${CONFIG_FACET_DIR}/${child}/`);
        continue;
      }
      if (child === SCHEMA_FACET_DIR) {
        for (const filename of taxonomySchemaFilenames(childAbs)) {
          if (!existsSync(join(childAbs, filename))) {
            findings.push(`${pluginId}: ${ownerLabel} is missing ${CONFIG_FACET_DIR}/${child}/${filename}`);
          }
        }
      }
    }
    const presenceAbs = join(parentAbs, PRESENCE_FACET_DIR);
    if (!existsSync(presenceAbs)) {
      findings.push(`${pluginId}: ${ownerLabel} is missing ${PRESENCE_FACET_DIR}/`);
      return;
    }
    for (const child of TAXONOMY_PRESENCE_CHILD_DIRS) {
      const childAbs = join(presenceAbs, child);
      if (!existsSync(childAbs)) {
        findings.push(`${pluginId}: ${ownerLabel} is missing ${PRESENCE_FACET_DIR}/${child}/`);
        continue;
      }
      if (child === SCHEMA_FACET_DIR) {
        for (const filename of taxonomySchemaFilenames(childAbs)) {
          if (!existsSync(join(childAbs, filename))) {
            findings.push(`${pluginId}: ${ownerLabel} is missing ${PRESENCE_FACET_DIR}/${child}/${filename}`);
          }
        }
      }
    }
  };
  for (const { abs: surfaceAbs, label } of surfaceDirs) {
    if (existsSync(join(surfaceAbs, LEGACY_CONFIG_FACET_DIR))) {
      findings.push(`${pluginId}: surface "${label}" still has ${LEGACY_CONFIG_FACET_DIR}/ — rename to ${CONFIG_FACET_DIR}/`);
    }
    if (existsSync(join(surfaceAbs, LEGACY_WASM_DIR))) {
      findings.push(`${pluginId}: surface "${label}" still has ${LEGACY_WASM_DIR}/ — rename to 🌉️wasm/`);
    }
    assertAppSchemaOwner(`surface "${label}"`, surfaceAbs);
  }
  if (existsSync(join(pluginRoot, LEGACY_CONFIG_FACET_DIR))) {
    findings.push(`${pluginId}: plugin-root still has ${LEGACY_CONFIG_FACET_DIR}/ — rename to ${CONFIG_FACET_DIR}/`);
  }
  assertAppSchemaOwner(`plugin-root`, pluginRoot);
  //#endregion SurfaceFacetWalk

  return findings;
}



/** 🧪️ Compares exact registry mounts with Rust compiler dependency membership over neutral trees. */
export class RustTaxonomyMountsCheckScript extends BundleScript {
  async run(): Promise<void> {
    const fixtureRoot = join(import.meta.dir, "..", "🧫️fixtures/🕸️rust-taxonomy-mounts");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as { cases: { id: string; files: Record<string, string>; neighbourFiles?: Record<string, string>; expectedCodes: string[]; expectedUnmounted: string[]; rustcSuccess: boolean }[] };
    const validate = await registrySchemaValidator("RustTaxonomyMountsV1");
    if (!validate(fixture)) throw new Error(JSON.stringify(validate.errors));
    const capture = join(this.root, "dist/rust-taxonomy-mounts-check");
    rmSync(capture, { recursive: true, force: true });
    mkdirSync(capture, { recursive: true });
    const failures: string[] = [];
    for (const row of fixture.cases) {
      const pluginRoot = join(capture, row.id);
      // 🪃️ `neighbourFiles` are written BESIDE the owner root, never inside it, exactly as a shared
      // `✏️s/🔨️modules/<m>/` leaf sits beside the plugin that mounts it: `rustc` compiles them, while
      // the registry walk — which only ever descends the owner root — never sees them.
      for (const [path, content] of Object.entries({ ...row.files, ...Object.fromEntries(Object.entries(row.neighbourFiles ?? {}).map(([rel, body]) => [join("..", row.id + "-neighbour", rel), body])) })) {
        const destination = join(pluginRoot, path);
        mkdirSync(dirname(destination), { recursive: true });
        writeFileSync(destination, content);
      }
      const sourceFiles = Object.keys(row.files).filter((path) => path.endsWith(".rs"));
      const entry = [...RUST_ENTRY_DIR_FROM_OWNER, RUST_ENTRY_FILENAME].join("/");
      const child = Bun.spawn(["rustc", "--crate-name", "taxonomy_mount_fixture", "--crate-type", "lib", "--edition", "2021", "--emit=metadata=fixture.rmeta,dep-info=fixture.d", entry], { cwd: pluginRoot, stdout: "pipe", stderr: "pipe", env: { ...process.env, RUST_BACKTRACE: "0" } });
      let cancelled = false;
      let timedOut = false;
      const cancel = (): void => { cancelled = true; child.kill(); };
      process.once("SIGINT", cancel);
      process.once("SIGTERM", cancel);
      const timeout = setTimeout(() => { timedOut = true; child.kill(); }, 30_000);
      const [status, stdout, stderr] = await Promise.all([child.exited, new Response(child.stdout).text(), new Response(child.stderr).text()]).finally(() => {
        clearTimeout(timeout);
        process.off("SIGINT", cancel);
        process.off("SIGTERM", cancel);
      });
      if (cancelled || timedOut) throw new Error(row.id + (cancelled ? ": compiler cancelled" : ": compiler exceeded 30s budget"));
      writeFileSync(join(pluginRoot, "compiler.log"), stdout + stderr);
      if ((status === 0) !== row.rustcSuccess) failures.push(row.id + ": compiler status differs: " + status);
      const referenceCodes: string[] = [];
      let referenceUnmounted: string[] = [];
      if (status !== 0) {
        if (!stderr.includes("couldn't read") && !stderr.includes("file not found for module")) throw new Error(row.id + ": non-membership compiler failure: " + stderr);
        referenceCodes.push("missing-target");
      } else {
        const dependencies = readFileSync(join(pluginRoot, "fixture.d"), "utf8").replaceAll("\\\n", "");
        const mounted = dependencies.split(/\r?\n/u).filter((line) => line.endsWith(":")).map((line) => {
          const path = line.slice(0, -1).replace(/\\([ #\\])/gu, "$1").replaceAll("$", "$").replaceAll("\\", "/");
          return relative(pluginRoot, resolve(pluginRoot, path)).replaceAll("\\", "/");
        });
        referenceUnmounted = sourceFiles.filter((path) => !mounted.includes(path)).sort();
        if (referenceUnmounted.length) referenceCodes.push("unmounted");
      }
      const findings = validateRustTaxonomyMounts(pluginRoot, row.id, sourceFiles.map((path) => join(pluginRoot, path)), sourceFiles.map((path) => join(pluginRoot, path)));
      const actualCodes = [...new Set(findings.map((finding) => finding.includes("is not declared") || finding.includes("is not reachable") ? "unmounted" : finding.includes("does not exist") || finding.includes("missing module target") ? "missing-target" : finding.includes("invalid Cargo") ? "invalid-manifest" : "missing-manifest"))].sort();
      const actualUnmounted = sourceFiles.filter((path) => findings.includes(row.id + ": " + path + " is not reachable from Cargo manifest " + [...RUST_ENTRY_DIR_FROM_OWNER, "Cargo.toml"].join("/"))).sort();
      if (!isDeepStrictEqual(referenceUnmounted, row.expectedUnmounted)) failures.push(row.id + ": compiler membership differs: " + JSON.stringify(referenceUnmounted));
      if (!isDeepStrictEqual(actualUnmounted, row.expectedUnmounted)) failures.push(row.id + ": registry membership differs: " + JSON.stringify(actualUnmounted));
      if (!isDeepStrictEqual(referenceCodes.sort(), row.expectedCodes)) failures.push(row.id + ": Rust reference differs: " + JSON.stringify(referenceCodes));
      if (!isDeepStrictEqual(actualCodes, row.expectedCodes)) failures.push(row.id + ": registry differs: " + JSON.stringify({ actualCodes, findings }));
    }
    if (failures.length) throw new Error(failures.join("\n"));
    console.log("registry-rust-mounts-oracle cases=" + fixture.cases.length + " ajv=1 compiler=" + fixture.cases.length);
  }
}


/** 🪴️ Checks required root ownership with neutral trees and an independent SQLite relation. */
export class PluginRootOwnershipCheckScript extends BundleScript {
  async run(): Promise<void> {
    const fixtureRoot = join(import.meta.dir, "..", "🧫️fixtures/🌳️plugin-root-ownership");
    const fixture = JSON.parse(readFileSync(join(fixtureRoot, "🔣️.json"), "utf8")) as { cases: { id: string; files: string[]; expected: string[] }[] };
    const validate = await registrySchemaValidator("PluginRootOwnershipV1");
    if (!validate(fixture)) throw new Error(JSON.stringify(validate.errors));
    const capture = join(this.root, "dist/plugin-root-ownership-check");
    rmSync(capture, { recursive: true, force: true });
    mkdirSync(capture, { recursive: true });
    const { Database } = await import("bun:sqlite");
    const oracle = new Database(":memory:");
    const failures: string[] = [];
    try {
      oracle.run("CREATE TABLE required (path TEXT PRIMARY KEY)");
      oracle.run("CREATE TABLE actual (path TEXT PRIMARY KEY)");
      for (const path of [TAXONOMY_LEAF_FILENAME, ...TAXONOMY.pluginRequiredChildDirs.map((child) => child + "/" + TAXONOMY_LEAF_FILENAME)]) {
        oracle.run("INSERT INTO required VALUES (?1)", [path]);
      }
      for (const row of fixture.cases) {
        oracle.run("DELETE FROM actual");
        const root = join(capture, row.id);
        for (const path of row.files) {
          oracle.run("INSERT INTO actual VALUES (?1)", [path]);
          const destination = join(root, path);
          mkdirSync(dirname(destination), { recursive: true });
          writeFileSync(destination, "");
        }
        const reference = (oracle.query("SELECT 'missing:' || path AS code FROM required WHERE path NOT IN (SELECT path FROM actual) UNION SELECT 'nested-contract' AS code WHERE EXISTS (SELECT 1 FROM actual WHERE path LIKE '🔌️plugin/%') ORDER BY code").all() as { code: string }[]).map(({ code }) => code);
        const findings = validatePluginContractRoot(root, row.id);
        const actual = findings.map((finding) => finding.includes("redundant 🔌️plugin") ? "nested-contract" : "missing:" + finding.slice(finding.indexOf("plugin root is missing ") + "plugin root is missing ".length)).sort();
        if (!isDeepStrictEqual(reference, row.expected)) failures.push(row.id + ": SQLite=" + JSON.stringify(reference));
        if (!isDeepStrictEqual(actual, row.expected)) failures.push(row.id + ": registry=" + JSON.stringify(actual));
      }
    } finally {
      oracle.close();
    }
    if (failures.length) throw new Error(failures.join("\n"));
    console.log("registry-plugin-root-ownership cases=" + fixture.cases.length + " ajv=1 sqlite=" + fixture.cases.length + " capture=" + capture);
  }
}
