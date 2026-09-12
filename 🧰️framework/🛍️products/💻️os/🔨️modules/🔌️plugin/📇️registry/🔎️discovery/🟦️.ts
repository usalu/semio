import { readFileSync } from "node:fs";
import { dirname, join, relative } from "node:path";
import type { AreaState, DiscoveredPackage, PackageRole, RegistryCatalogInputView } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { canonicalPrimaryFilenameForKind, discoverCatalogPackages, getWorkspaceRoot, loadCatalogTaxonomy, registryCatalogInputView } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { moduleDirectoryName } from "../📦️deployment/🟦️.ts";
import { runtimeComponentClosure } from "../../../../../🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";



//#region 🔖️PluginRegistryEntry
export type PluginHostMetadata = {
  readonly landingAppId: string;
  readonly hostAppId: string;
};


/** 🛂️ `#️⃣PackageHashes` mirror (`🛂️manifest/🦀️.rs`) — content hashes the `check` gate
 * verifies against the built wasm. Present only once a crate has a `🔣️.json`. */
export type PluginDescriptorHashes = {
  readonly wasmSha256: string;
  readonly coreWasmSha256: string;
  readonly descriptorSha256: string;
};


export type PluginRegistryEntry = {
  readonly pluginId: string;
  readonly packageId: string;
  readonly cratePath: string;
  readonly packageName: string;
  readonly wasmOut: string;
  readonly role: "plugin" | "extension";
  readonly extends?: string;
  readonly capabilities: readonly string[];
  readonly contributes: readonly string[];
  readonly consumes: readonly string[];
  /** 🔗️ The RUNTIME actor dependencies this crate declares in `[package.metadata.semio].depends-on`
   * — sibling plugins whose own actor must be loaded beside this one (it embeds their surfaces,
   * contributes onto their artifacts, or exchanges messages with them), mirroring the same set the
   * crate's builder declares through `.depends_on(id, VersionReq)`. A Cargo `[dependencies]` entry on
   * `semio-s-plugin-<id>` is a BUILD-TIME rlib link (codecs, schema types, shared geometry) and is
   * deliberately never read here — see {@link parseSemioDependsOnIds}. For an extension, `extends` is
   * always `dependsOn[0]` (contract freeze §4 rule 1). Consumed by
   * `resolveRegistryPluginIdsForFilter` to close a dev session's plugin set transitively. */
  readonly dependsOn: readonly string[];
  readonly host?: PluginHostMetadata;
  /** 🎬️ `kernel::ActivationEvent` rows, flattened to `📓️design-abi.md` §2's canonical dash-separated
   * strings (`on-command:<id>`, `on-view-visible:<id>`, `on-file-type:<ext>`, `on-artifact-kind:<kind>`,
   * `on-extension-request:<point>`, `on-startup-finished`) — sourced from `🔣️.json`, empty
   * for a crate that has none yet (E1-describe lands ahead of the W3 plugin migrations that produce
   * one per crate — see `parsePluginCargo`'s own doc for the fallback rule). */
  readonly activationEvents: readonly string[];
  /** 🧩️ `ExtensionPointDeclaration.id` rows this package PUBLISHES for others to attach to — empty
   * for crates with none declared or no descriptor yet. */
  readonly extensionPoints: readonly string[];
  /** 🚦️ `ExecutionMode` (`declarative`|`linked`|`isolated`|`exclusive`|`cold`) — `undefined` for a
   * crate with no descriptor yet. */
  readonly executionMode?: string;
  /** #️⃣ `undefined` for a crate with no descriptor yet — see `check`'s hash-verification gate. */
  readonly hashes?: PluginDescriptorHashes;
};


export const COMPONENT_MANIFEST_MAX_BYTES = 64 * 1024;

export const CATALOG_ID = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;

export const COMPONENT_PACKAGE_ID = /^semio:[a-z0-9]+(?:-[a-z0-9]+)*$/;


/** 🧭️ Parses the one exact canonical component package identity without prefix inference. */
export function parseComponentPackageId(text: string, manifestPath: string): string {
  if (Buffer.byteLength(text) > COMPONENT_MANIFEST_MAX_BYTES) throw new Error(`${manifestPath} exceeds the 64 KiB component-contract boundary`);
  let inComponent = false;
  let componentSeen = false;
  let packageId: string | undefined;
  for (const sourceLine of text.split(/\r?\n/u)) {
    const line = sourceLine.trim();
    if (line.startsWith("[")) {
      inComponent = line === "[package.metadata.component]";
      if (inComponent) {
        if (componentSeen) throw new Error(`${manifestPath} repeats [package.metadata.component]`);
        componentSeen = true;
      }
      continue;
    }
    if (!inComponent || line === "" || line.startsWith("#")) continue;
    const separator = line.indexOf("=");
    if (separator < 0 || line.slice(0, separator).trim() !== "package") continue;
    if (packageId !== undefined) throw new Error(`${manifestPath} repeats the component package key`);
    const match = line.slice(separator + 1).trim().match(/^"([^"]+)"\s*(?:#.*)?$/u);
    if (!match) throw new Error(`${manifestPath} component package must be one quoted string`);
    packageId = match[1];
  }
  if (!componentSeen || !packageId) throw new Error(`missing [package.metadata.component].package in ${manifestPath}`);
  if (!COMPONENT_PACKAGE_ID.test(packageId)) throw new Error(`${manifestPath} component package must match semio:<lowercase-alnum-or-hyphen>`);
  return packageId;
}


//#region 🏛️DiscoveryContract
/** @emoji 🔣️ The one shared taxonomy vocabulary (`🦑️repo/📚️library`'s `🔣️taxonomy.json`), read once. Every
 * directory-name, manifest-filename, role and area literal this script used to hardcode as a path regex
 * now comes from here, so registry discovery can never drift from the root policy script's or the SDK
 * testkit's view of the same contract — see mechanism ticket
 * `26/08/06/MECHANISM-VOCABULARY-AND-DISCOVERY-LIBRARY`. */
export const TAXONOMY = loadCatalogTaxonomy();


/** 📄️ Resolves the taxonomy-ordered primary filename for consumers that require one component leaf. */
export function primaryFilenameForKind(kindId: string): string {
  return canonicalPrimaryFilenameForKind(kindId, TAXONOMY);
}


/** @emoji 🗺️ Every area root that may hold a plugin crate, cross-checked against `taxonomy.areas` at
 * load time so none of these literals can outlive a vocabulary rename. Membership across this array —
 * never equality against one hand-picked literal — is how every plugin-tree path test below decides
 * "is this under a plugin area"; its dedicated taxonomy-tree state decides whether findings warn or
 * fail (see `PLUGIN_AREAS_STATE`). */
export const PLUGIN_AREAS: readonly string[] = TAXONOMY.pluginAreas;

if (!Array.isArray(PLUGIN_AREAS) || PLUGIN_AREAS.length === 0) throw new Error(`📇️registry: 🔣️taxonomy.json must declare a non-empty "pluginAreas" array`);

for (const area of PLUGIN_AREAS) {
  if (!(area in TAXONOMY.areas)) throw new Error(`📇️registry: "${area}" is not a declared area in 🔣️taxonomy.json (${Object.keys(TAXONOMY.areas).join(", ")})`);
}


/** @emoji 🗺️ Merges every declared plugin area's `AreaState` to the most permissive member, so one
 * still-migrating area can never be silently masked by a sibling area that already reached `clean`. */
export function mergeAreaStates(states: readonly AreaState[]): AreaState {
  if (states.includes("legacy")) return "legacy";
  if (states.includes("mixed")) return "mixed";
  return "clean";
}


/** @emoji 🌳️ Declared taxonomy-tree maturity across every plugin area, independent of the package-layout
 * maturity in `areas`. `legacy`/`mixed` ⇒ findings are warn-only; `clean` ⇒ they fail the gate. */
export const PLUGIN_AREAS_STATE: AreaState = mergeAreaStates(PLUGIN_AREAS.map((area) => TAXONOMY.areas[area]));


/** @emoji 📚️ Artifact-scoped example data dir (`artifactChildDirs`, not owner root). */
export const EXAMPLES_DIRNAME = "📚️examples";

if (!TAXONOMY.artifactChildDirs.includes(EXAMPLES_DIRNAME)) {
  throw new Error(`📇️registry: "${EXAMPLES_DIRNAME}" must be listed in 🔣️taxonomy.json artifactChildDirs (${TAXONOMY.artifactChildDirs.join(", ")})`);
}

export const EXAMPLE_ASSETS_DIRNAME = TAXONOMY.exampleAssetsDirName ?? "🖼️assets";

export const EXAMPLE_TESTS_DIRNAME = TAXONOMY.exampleTestsDirName ?? "🧪️tests";

export const EXAMPLE_RUST_LEAF = primaryFilenameForKind(TAXONOMY.exampleFileKinds["🦀️rust"]);

export const EXAMPLE_TS_LEAF = primaryFilenameForKind(TAXONOMY.exampleFileKinds["🟦️typescript"]);

export const EXAMPLE_SLUG_RE = new RegExp(TAXONOMY.exampleSlugPattern ?? "^.+\uFE0F[a-z0-9]+(?:-[a-z0-9]+)*$", "u");

export const FORBIDDEN_EXAMPLE_PLURAL_DIRS = TAXONOMY.forbiddenExamplePluralDirs ?? [];

export const FORBIDDEN_EXAMPLE_SLUGS = new Set(TAXONOMY.forbiddenExampleSlugs ?? []);


/** @emoji ✅️ True when `name` is an emoji+VS16+kebab example slug (and not a forbidden placeholder). */
export function isExampleSlugName(name: string): boolean {
  return EXAMPLE_SLUG_RE.test(name) && !FORBIDDEN_EXAMPLE_SLUGS.has(name);
}


export const RUST_LANG = "🦀️rust";


/** @emoji 🧩️ Roles whose packages may carry a `[package.metadata.component]` wasm component and thus
 * belong in the plugin catalog: the plugin itself and the extensions it contributes. Every other role
 * (`framework`, `tool`, `s-module`, …) is filtered out by `tryParsePluginCargo` anyway — listing them
 * here keeps the intent explicit instead of implicit in a downstream parse failure. */
export const COMPONENT_ROLES: ReadonlySet<PackageRole> = new Set<PackageRole>(["plugin", "extension"]);


/** @emoji 📦️ Every rust package in the repo that declares a component-bearing role, via the shared
 * `discoverPackages()` walk (two-level `📦️packages/🦀️rust/` and three-level `🎯️targets/<t>/` shapes
 * alike). Replaces the two hand-written "new contract" path regexes this script used to carry. */
export function discoverComponentPackages(repoRoot: string, packages: readonly DiscoveredPackage[] = discoverCatalogPackages(repoRoot, TAXONOMY)): DiscoveredPackage[] {
  return packages.filter((pkg) => pkg.lang === RUST_LANG && COMPONENT_ROLES.has(pkg.role));
}


//#endregion 🏛️DiscoveryContract

/** @emoji 🧭️ Every manifest that may contribute a row to the plugin catalog, via the shared package
 * discovery contract. The pre-Shape-V2 legacy sandwich shape this used to also admit was removed once
 * every declared plugin area reached `clean` — see `PLUGIN_AREAS_STATE`. */
export function findPluginCargoFiles(root: string, packages?: readonly DiscoveredPackage[]): string[] {
  return discoverComponentPackages(root, packages)
    .map((pkg) => join(root, pkg.manifestPath))
    .sort();
}


/** 🔣️ Where a crate's static descriptor (`semio-framework-plugin-describe`'s output) lives, relative
 * to its own Cargo.toml directory: two levels up (out of `📦️packages/🦀️rust`) into the crate's OWNER
 * root — sibling of the tracked `🛂️manifest.json`, with NO further `🤖️generated/` segment.
 *
 * 🐛️ MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME (D0): this used to append `🤖️generated/`, by analogy
 * with `🎭️actor`'s generated TS bindings. But `🤖️generated/**` is globally gitignored, so a
 * descriptor written there can never survive a commit — and a descriptor's whole purpose is to be
 * the checked-in, static answer to "what does this package contribute" that the registry reads
 * WITHOUT instantiating any wasm. The analogy was to a directory holding regenerable build output;
 * a descriptor is a tracked artifact, so it inherits the opposite convention.
 *
 * Consequence while the paths disagreed: `plugin-registry:check` reported `🗒️note` as having no
 * descriptor while a real, fresh, committed one sat at the owner root and `descriptor_is_fresh()`
 * passed against it. The gate and the test were reading different files and both looked green.
 * `descriptor_is_fresh()` (`🔌️plugin/🦀️.rs`), the dev `📜️script.ts` build step, and every
 * plugin crate's own `📜️script.ts describe` command all use the owner root; this is the last leg. */
export const DESCRIPTOR_JSON_REL_PATH = TAXONOMY.generatorContracts["plugin-registry"].inputDiscovery!.descriptorRelativePath.split("/");


/** 🎬️ `kernel::ActivationEvent`'s default (externally tagged) serde JSON shape, decoded into
 * `📓️design-abi.md` §2's canonical dash-separated string form. Unit variant `OnStartupFinished`
 * serializes as the bare string `"onStartupFinished"`; every other variant as
 * `{ "<camelTag>": { ...fields } }`. */
export function formatActivationEvent(raw: unknown): string | undefined {
  if (typeof raw === "string") {
    return raw === "onStartupFinished" ? "on-startup-finished" : undefined;
  }
  if (raw === null || typeof raw !== "object") return undefined;
  const entries = Object.entries(raw as Record<string, unknown>);
  if (entries.length !== 1) return undefined;
  const [tag, body] = entries[0];
  const field = body !== null && typeof body === "object" ? (body as Record<string, unknown>) : {};
  switch (tag) {
    case "onCommand":
      return typeof field.id === "string" ? `on-command:${field.id}` : undefined;
    case "onViewVisible":
      return typeof field.id === "string" ? `on-view-visible:${field.id}` : undefined;
    case "onFileType":
      return typeof field.ext === "string" ? `on-file-type:${field.ext}` : undefined;
    case "onArtifactKind":
      return typeof field.kind === "string" ? `on-artifact-kind:${field.kind}` : undefined;
    case "onExtensionRequest":
      return typeof field.point === "string" ? `on-extension-request:${field.point}` : undefined;
    default:
      return undefined;
  }
}


/** 🔣️ Reads and loosely-shapes `<cratePath>/🤖️generated/🔣️.json` (the
 * `semio-framework-plugin-describe` emitter's JSON mirror of `PackageDescriptor`) — `undefined` when
 * the crate has none yet (every crate today: E1-describe lands ahead of the W3 plugin migrations
 * that produce one per crate; see `parsePluginCargo`'s doc for the fallback this enables). Loosely
 * typed (no schema validation) on purpose — `check`'s own gate is what enforces shape, not the parser. */
export function readDescriptorJson(repoRoot: string, cratePath: string, view: RegistryCatalogInputView = registryCatalogInputView(repoRoot, TAXONOMY)): Record<string, unknown> | undefined {
  const path = join(repoRoot, cratePath, ...DESCRIPTOR_JSON_REL_PATH);
  const inputPath = relative(repoRoot, path).replaceAll("\\", "/");
  const kind = view.kind(inputPath);
  if (kind === "symlink") throw new Error(`Registry descriptor is a symlink: ${inputPath}`);
  if (kind === null) return undefined;
  try {
    return JSON.parse(view.readText(inputPath));
  } catch {
    return undefined;
  }
}


/**
 * @emoji 🔣️ Parses one plugin/extension crate manifest into its catalog row. `📓️design-abi.md` §3:
 * when `<cratePath>/🤖️generated/🔣️.json` exists, `capabilities`/`contributes`/
 * `activationEvents`/`extensionPoints`/`executionMode`/`hashes` are read from it — Cargo
 * `[package.metadata.semio]` no longer carries `contributes` for a migrated crate (kept only for
 * `role`/`extends`/`mode`/playground rows, per the design doc). **Transitional fallback**: no plugin
 * crate has been migrated to emit a descriptor yet (that is W3's `M0`…`M8`, dispatched after this
 * packet) — for a crate with no descriptor, `capabilities`/`contributes` still come from the OLD
 * Cargo `contributes` TOML array exactly as before, so today's catalog (0/N crates migrated) is
 * byte-identical to pre-E1 behaviour. `consumes` is ALWAYS read from Cargo metadata regardless: the
 * static descriptor has no typed "what a package wants to receive" concept (`PackageDescriptor` only
 * has `topic_contributions`, i.e. what a package PUBLISHES) — a real gap, not silently papered over,
 * see `📓️terra-E1-describe-report.md`.
 */
export function parsePluginCargo(manifestPath: string, repoRoot: string, view?: RegistryCatalogInputView, ownerDescriptors: "required" | "ignored" = "required"): PluginRegistryEntry {
  const text = view ? view.readText(relative(repoRoot, manifestPath).replaceAll("\\", "/")) : readFileSync(manifestPath, "utf8");
  const packageName = text.match(/^name = "([^"]+)"/m)?.[1];
  if (!packageName) throw new Error(`missing package name in ${manifestPath}`);
  const packageId = parseComponentPackageId(text, manifestPath);
  const pluginId = packageId.slice("semio:".length);
  moduleDirectoryName(pluginId);
  const cratePath = relative(repoRoot, dirname(manifestPath));
  const wasmOut = `${packageName.replace(/-/g, "_")}.wasm`;
  const semioBlock = tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[package.metadata.semio]")[0];
  const semioText = semioBlock?.join("\n") ?? "";
  const consumes = parseTomlStringArray(semioText, "consumes");
  const roleRaw = semioText.match(/^role\s*=\s*"([^"]+)"/m)?.[1];
  const role: PluginRegistryEntry["role"] = roleRaw === "extension" ? "extension" : "plugin";
  const extendsHost = semioText.match(/^extends\s*=\s*"([^"]+)"/m)?.[1];
  const hostBlock = semioText.match(/^host\s*=\s*\{([^}]*)\}/m)?.[1];
  const landingAppId = hostBlock?.match(/landing\s*=\s*"([^"]+)"/)?.[1];
  const hostAppId = hostBlock?.match(/shell\s*=\s*"([^"]+)"/)?.[1];
  const host = landingAppId && hostAppId ? { landingAppId, hostAppId } : undefined;
  const declaredDependsOnIds = parseSemioDependsOnIds(semioText, pluginId, manifestPath);
  // 🔗️ contract freeze §4 rule 1: for an extension, `extends` is always dependsOn[0].
  const dependsOn = extendsHost ? [extendsHost, ...declaredDependsOnIds.filter((id) => id !== extendsHost)] : declaredDependsOnIds;

  const descriptor = ownerDescriptors === "required" ? readDescriptorJson(repoRoot, cratePath, view) : undefined;
  let capabilities: string[];
  let contributes: string[];
  let activationEvents: string[] = [];
  let extensionPoints: string[] = [];
  let executionMode: string | undefined;
  let hashes: PluginDescriptorHashes | undefined;
  if (descriptor) {
    const capabilityRequests = Array.isArray(descriptor.capabilityRequests) ? descriptor.capabilityRequests : [];
    capabilities = capabilityRequests.map((row) => (row as { id?: unknown }).id).filter((id): id is string => typeof id === "string");
    const contributions = descriptor.contributions as Record<string, unknown> | undefined;
    const topicContributions = Array.isArray(contributions?.topicContributions) ? (contributions!.topicContributions as unknown[]) : [];
    contributes = topicContributions.map((row) => (row as { topic?: unknown }).topic).filter((topic): topic is string => typeof topic === "string");
    const rawActivationEvents = Array.isArray(descriptor.activationEvents) ? descriptor.activationEvents : [];
    activationEvents = rawActivationEvents.map(formatActivationEvent).filter((event): event is string => event !== undefined);
    const rawExtensionPoints = Array.isArray(descriptor.extensionPoints) ? descriptor.extensionPoints : [];
    extensionPoints = rawExtensionPoints.map((row) => (row as { id?: unknown }).id).filter((id): id is string => typeof id === "string");
    executionMode = typeof descriptor.execution === "string" ? descriptor.execution : undefined;
    const rawHashes = descriptor.hashes as Record<string, unknown> | undefined;
    if (rawHashes && typeof rawHashes.wasmSha256 === "string" && typeof rawHashes.coreWasmSha256 === "string" && typeof rawHashes.descriptorSha256 === "string") {
      hashes = { wasmSha256: rawHashes.wasmSha256, coreWasmSha256: rawHashes.coreWasmSha256, descriptorSha256: rawHashes.descriptorSha256 };
    }
  } else {
    contributes = parseTomlStringArray(semioText, "contributes");
    capabilities = contributes;
  }

  return {
    pluginId,
    packageId,
    cratePath,
    packageName,
    wasmOut,
    role,
    capabilities,
    contributes,
    consumes,
    dependsOn,
    activationEvents,
    extensionPoints,
    ...(extendsHost ? { extends: extendsHost } : {}),
    ...(host ? { host } : {}),
    ...(executionMode ? { executionMode } : {}),
    ...(hashes ? { hashes } : {}),
  };
}



export function tryParsePluginCargo(manifestPath: string, repoRoot: string, view?: RegistryCatalogInputView): PluginRegistryEntry | undefined {
  try {
    return parsePluginCargo(manifestPath, repoRoot, view);
  } catch {
    return undefined;
  }
}



export function generatePluginRegistry(repoRoot = getWorkspaceRoot(), options: GeneratePluginRegistryOptions = {}): PluginRegistryEntry[] {
  const filterPlaygroundPlugin = options.filterPlaygroundPlugin;
  const filterIds = filterPlaygroundPlugin && !isHostPluginFilter(filterPlaygroundPlugin) ? resolveRegistryPluginIdsForFilter(filterPlaygroundPlugin) : undefined;
  const manifestPaths = filterIds ? findPluginCargoPathsForIds(repoRoot, filterIds) : findPluginCargoFiles(repoRoot, options.packages ?? (options.view ? discoverCatalogPackages(repoRoot, TAXONOMY, options.view) : undefined));
  const entries: PluginRegistryEntry[] = [];
  for (const path of manifestPaths) {
    const entry = tryParsePluginCargo(path, repoRoot, options.view);
    if (entry) entries.push(entry);
  }
  entries.sort((a, b) => a.pluginId.localeCompare(b.pluginId));
  return entries;
}



export function tomlBlocksAfterHeader(lines: readonly string[], headerTest: (line: string) => boolean): string[][] {
  const blocks: string[][] = [];
  for (let i = 0; i < lines.length; i++) {
    if (!headerTest(lines[i].trim())) continue;
    const body: string[] = [];
    for (let j = i + 1; j < lines.length; j++) {
      if (lines[j].trim().startsWith("[")) break;
      body.push(lines[j]);
    }
    blocks.push(body);
  }
  return blocks;
}



export function parseTomlStringArray(block: string, key: string): string[] {
  const match = block.match(new RegExp(`^${key}\\s*=\\s*\\[([^\\]]*)\\]`, "m"));
  if (!match) return [];
  return [...match[1].matchAll(/"([^"]*)"/g)].map((m) => m[1]);
}



/**
 * @emoji 🔗️ The runtime actor dependencies one crate DECLARES, read from
 * `[package.metadata.semio].depends-on` — the same plugin-id set its builder passes to
 * `.depends_on(id, VersionReq)` (`🔌️plugin/🦀️.rs`), kept in the Cargo manifest as well because the
 * registry is generated BEFORE any wasm build and therefore cannot read the descriptor a build
 * emits. Mirrors the root policy script's `policySemioMetadataDependsOnIds` so the derived catalog
 * and the `plugin-dependency/parity` gate can never read two different dependency sets.
 *
 * A Cargo `[dependencies]` line on `semio-s-plugin-<id>` is deliberately NOT a source here: it is a
 * build-time rlib link (codecs, schema types, shared geometry called in-process) and says nothing
 * about needing that plugin's own actor loaded. Deriving the load graph from it made every crate
 * that links `stdio`'s codecs pull `stdio` into the browser's load set — and cascade-fail with it —
 * and minted phantom ids for linked sub-crates that are not plugins at all (`draw-fsm`,
 * `imperative-control`). `extends` supplies an extension's host edge and is prepended by
 * {@link parsePluginCargo}, so an extension never repeats its host here.
 */
export function parseSemioDependsOnIds(semioText: string, ownId: string, manifestPath: string): string[] {
  const ids = parseTomlStringArray(semioText, "depends-on");
  for (const id of ids) {
    if (!CATALOG_ID.test(id)) throw new Error(`${manifestPath} metadata.semio.depends-on holds the malformed plugin id ${JSON.stringify(id)}`);
    if (id === ownId) throw new Error(`${manifestPath} metadata.semio.depends-on names its own plugin id`);
  }
  if (new Set(ids).size !== ids.length) throw new Error(`${manifestPath} metadata.semio.depends-on repeats a plugin id`);
  return ids;
}



//#endregion

export type GeneratePluginRegistryOptions = {
  readonly filterPlaygroundPlugin?: string;
  readonly packages?: readonly DiscoveredPackage[];
  readonly view?: RegistryCatalogInputView;
};



/** @emoji 🎯️ Resolves a playground variant/alias or bare plugin id to its wasm registry plugin id. */
export function resolveRegistryPluginIdForFilter(pluginFilter: string, repoRoot = getWorkspaceRoot()): string {
  for (const manifestPath of findPluginCargoFiles(repoRoot)) {
    const text = readFileSync(manifestPath, "utf8");
    let componentPackage: string;
    try {
      componentPackage = parseComponentPackageId(text, manifestPath).slice("semio:".length);
    } catch {
      continue;
    }
    for (const block of tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[[package.metadata.semio.playground]]")) {
      const body = block.join("\n");
      const variant = body.match(/^variant\s*=\s*"([^"]+)"/m)?.[1];
      if (!variant) continue;
      const aliases = parseTomlStringArray(body, "aliases");
      if (variant === pluginFilter || aliases.includes(pluginFilter)) return componentPackage;
    }
  }
  return pluginFilter;
}



export function pluginEntryHasHost(pluginId: string, repoRoot: string): boolean {
  for (const manifestPath of findPluginCargoFiles(repoRoot)) {
    const entry = tryParsePluginCargo(manifestPath, repoRoot);
    if (entry?.pluginId === pluginId) return entry.host !== undefined;
  }
  return false;
}



/** @emoji 🏠️ True when the filter resolves to a plugin crate that declares `[package.metadata.semio].host`. */
export function isHostPluginFilter(pluginFilter?: string, repoRoot = getWorkspaceRoot()): boolean {
  if (!pluginFilter) return true;
  return pluginEntryHasHost(resolveRegistryPluginIdForFilter(pluginFilter, repoRoot), repoRoot);
}



/** 🎯️ Resolves aliases and runtime dependencies within the supplied catalog, or discovers an omitted catalog. */
export function resolveRegistryPluginIdsForFilter(filterPlaygroundPlugin: string, allEntries: readonly PluginRegistryEntry[] = generatePluginRegistry(getWorkspaceRoot()), playgrounds?: readonly { readonly variant: string; readonly aliases: readonly string[]; readonly pluginId: string }[]): readonly string[] {
  const variantRow = playgrounds?.find((p) => p.variant === filterPlaygroundPlugin || p.aliases.includes(filterPlaygroundPlugin));
  const targetPluginId = variantRow?.pluginId ?? (playgrounds === undefined ? resolveRegistryPluginIdForFilter(filterPlaygroundPlugin) : filterPlaygroundPlugin);
  return allEntries.some(row => row.pluginId === targetPluginId) ? runtimeComponentClosure(allEntries, [targetPluginId]) : [];
}



export function findPluginCargoPathsForIds(repoRoot: string, pluginIds: readonly string[]): string[] {
  const idSet = new Set(pluginIds);
  return findPluginCargoFiles(repoRoot).filter((path) => {
    const entry = tryParsePluginCargo(path, repoRoot);
    return entry !== undefined && idSet.has(entry.pluginId);
  });
}
