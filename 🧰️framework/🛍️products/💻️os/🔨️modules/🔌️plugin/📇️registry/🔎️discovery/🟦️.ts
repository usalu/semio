import { readFileSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import type { AreaState, DiscoveredPackage, RegistryCatalogInputView } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { canonicalPrimaryFilenameForKind, declaredComponentKind, declaredComponentDeploymentDirectoryV1, discoverCatalogPackages, getWorkspaceRoot, loadCatalogTaxonomy, registryCatalogInputView } from "../../../../../🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { decodeRegistryDescriptorV1 } from "../🧬️schema/🟦️.ts";
import { runtimeComponentClosure } from "../../../../../🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs";



import { parseComponentSourceRowV1, parseCompiledComponentRowV1, parseDeployedRegistryEntryV1, type ComponentSourceOwnerV1, type CompiledComponentOwnerV1, type DeployedRegistryEntryV1, type PluginDescriptorHashes } from "./🧬️schema/🟦️.ts";
export type { ComponentSourceOwnerV1, CompiledComponentOwnerV1, DeployedRegistryEntryV1, PluginDescriptorHashes, PluginHostMetadata } from "./🧬️schema/🟦️.ts";

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
/** 🔣️ The one shared taxonomy vocabulary (`🦑️repo/📚️library`'s `🔣️taxonomy.json`), read once. Every
 * directory-name, manifest-filename, role and area literal this script used to hardcode as a path regex
 * now comes from here, so registry discovery can never drift from the root policy script's or the SDK
 * testkit's view of the same contract — see mechanism ticket
 * `26/08/06/MECHANISM-VOCABULARY-AND-DISCOVERY-LIBRARY`. */
export const TAXONOMY = loadCatalogTaxonomy();


/** 📄️ Resolves the taxonomy-ordered primary filename for consumers that require one component leaf. */
export function primaryFilenameForKind(kindId: string): string {
  return canonicalPrimaryFilenameForKind(kindId, TAXONOMY);
}


/** 🗺️ Every area root that may hold a plugin crate, cross-checked against `taxonomy.areas` at
 * load time so none of these literals can outlive a vocabulary rename. Membership across this array —
 * never equality against one hand-picked literal — is how every plugin-tree path test below decides
 * "is this under a plugin area"; its dedicated taxonomy-tree state decides whether findings warn or
 * fail (see `PLUGIN_AREAS_STATE`). */
export const PLUGIN_AREAS: readonly string[] = TAXONOMY.pluginAreas;

if (!Array.isArray(PLUGIN_AREAS) || PLUGIN_AREAS.length === 0) throw new Error(`📇️registry: 🔣️taxonomy.json must declare a non-empty "pluginAreas" array`);

for (const area of PLUGIN_AREAS) {
  if (!(area in TAXONOMY.areas)) throw new Error(`📇️registry: "${area}" is not a declared area in 🔣️taxonomy.json (${Object.keys(TAXONOMY.areas).join(", ")})`);
}


/** 🗺️ Merges every declared plugin area's `AreaState` to the most permissive member, so one
 * still-exempt area can never be silently masked by a sibling area that already reached `clean`. */
export function mergeAreaStates(states: readonly AreaState[]): AreaState {
  return states.includes("exempt") ? "exempt" : "clean";
}


/** 🌳️ Declared taxonomy-tree maturity across every plugin area, independent of the package-layout
 * maturity in `areas`. `exempt` ⇒ findings are warn-only; `clean` ⇒ they fail the gate. */
export const PLUGIN_AREAS_STATE: AreaState = mergeAreaStates(PLUGIN_AREAS.map((area) => TAXONOMY.areas[area]));


/** 📚️ Artifact-scoped example data dir (`artifactChildDirs`, not owner root). */
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


/** ✅️ True when `name` is an emoji+VS16+kebab example slug (and not a forbidden placeholder). */
export function isExampleSlugName(name: string): boolean {
  return EXAMPLE_SLUG_RE.test(name) && !FORBIDDEN_EXAMPLE_SLUGS.has(name);
}


export const RUST_LANG = "🦀️rust";


/** 📦️ Discovers explicitly authored component protocols without inferring package ownership. */
export function discoverComponentPackages(repoRoot: string, packages: readonly DiscoveredPackage[] = discoverCatalogPackages(repoRoot, TAXONOMY), view?: RegistryCatalogInputView): DiscoveredPackage[] {
  return packages.filter((pkg) => pkg.lang === RUST_LANG && declaredComponentKind(view ? view.readText(pkg.manifestPath) : readFileSync(join(repoRoot, pkg.manifestPath), "utf8")) !== undefined);
}

//#endregion 🏛️DiscoveryContract

/** 🧭️ Every manifest that may contribute a row to the plugin catalog, via the shared package
 * discovery contract. The pre-Shape-V2 legacy sandwich shape this used to also admit was removed once
 * every declared plugin area reached `clean` — see `PLUGIN_AREAS_STATE`. */
export function findPluginCargoFiles(root: string, packages?: readonly DiscoveredPackage[], view?: RegistryCatalogInputView): string[] {
  return discoverComponentPackages(root, packages, view)
    .filter((pkg) => declaredComponentDeploymentDirectoryV1(view ? view.readText(pkg.manifestPath) : readFileSync(join(root, pkg.manifestPath), "utf8")) !== undefined)
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

function uniqueStrings(values: readonly string[]): string[] {
  return [...new Set(values)];
}

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


export const ON_ARTIFACT_KIND_PREFIX = "on-artifact-kind:";


/** 🗂️ The artifact kinds a descriptor's own app surfaces declare — `manifest.apps[].dialect.artifactKind`,
 * deduped, first-seen order. This is the namespace an opening request speaks (`s.cad.cad@1/*` parses
 * to `s.cad.cad`), which is NOT the namespace a crate's hand-declared `ActivationEvent::OnArtifactKind`
 * uses (`3d.cad`, the `ArtifactKindSpec.id`). Both are declared by the same descriptor, so the catalog
 * carries both as `on-artifact-kind:` rows and a host can resolve an owner from either without having
 * loaded a single wasm module. */
export function descriptorSurfaceArtifactKinds(descriptor: Record<string, unknown> | undefined): string[] {
  const manifest = descriptor?.manifest as Record<string, unknown> | undefined;
  const apps = Array.isArray(manifest?.apps) ? (manifest!.apps as unknown[]) : [];
  const kinds: string[] = [];
  for (const app of apps) {
    const dialect = (app as { dialect?: unknown }).dialect as Record<string, unknown> | undefined;
    const kind = dialect?.artifactKind;
    if (typeof kind !== "string" || kind.trim() !== kind || kind === "" || kinds.includes(kind)) continue;
    kinds.push(kind);
  }
  return kinds;
}


/** 🎬️ Strips every `on-artifact-kind:` row a crate shares with one of its transitive `dependsOn`
 * crates, so exactly one catalog row claims each kind: the owner. This is the build-time twin of the
 * runtime `AppRouter.build` ownership rule ("dependency-first load order, first claim wins") — a
 * contributor that registers a surface on someone else's kind must declare that owner as a dependency
 * (`surface.contribution-not-permitted`), so the dependency edge is exactly what separates the two.
 * Kinds claimed by unrelated crates stay on both rows; the resolvers below then pick the first by
 * ascending `pluginId`, matching the router's own deterministic ordering. */
export function claimOwnedArtifactKinds<T extends CompiledComponentOwnerV1>(entries: readonly T[]): T[] {
  const byId = new Map(entries.map((entry) => [entry.pluginId, entry] as const));
  const kindsOf = (entry: CompiledComponentOwnerV1): Set<string> => new Set(entry.activationEvents.filter((event) => event.startsWith(ON_ARTIFACT_KIND_PREFIX)));
  const transitive = (entry: CompiledComponentOwnerV1): Set<string> => {
    const seen = new Set<string>();
    const claimed = new Set<string>();
    const pending = [...entry.dependsOn];
    while (pending.length > 0) {
      const id = pending.pop()!;
      if (id === entry.pluginId || seen.has(id)) continue;
      seen.add(id);
      const dependency = byId.get(id);
      if (!dependency) continue;
      for (const kind of kindsOf(dependency)) claimed.add(kind);
      pending.push(...dependency.dependsOn);
    }
    return claimed;
  };
  return entries.map((entry) => {
    const inherited = transitive(entry);
    const activationEvents = entry.activationEvents.filter((event) => !inherited.has(event));
    return activationEvents.length === entry.activationEvents.length ? entry : { ...entry, activationEvents };
  });
}


/** 🔣️ Requires the current descriptor at its authored producer root with no-follow ancestry. */
export function readDescriptorJson(repoRoot: string, cratePath: string, view: RegistryCatalogInputView = registryCatalogInputView(repoRoot, TAXONOMY)): Record<string, unknown> {
  const path = resolve(repoRoot, cratePath, ...DESCRIPTOR_JSON_REL_PATH);
  const inputPath = relative(repoRoot, path).replaceAll("\\", "/");
  if (isAbsolute(inputPath) || inputPath.split("/").some(part => !part || part === "." || part === "..")) throw new Error("Registry descriptor escapes the workspace");
  const parts = inputPath.split("/");
  for (let count = 1; count <= parts.length; count++) {
    const prefix = parts.slice(0, count).join("/");
    if (view.kind(prefix) !== (count === parts.length ? "file" : "directory")) throw new Error(`Registry descriptor requires present no-follow ancestry: ${prefix}`);
  }
  return decodeRegistryDescriptorV1(view.readBytes(inputPath));
}


/** 📇️ Reads source identity and, when selected, its required compiled descriptor. */
export function parseComponentSourceOwnerV1(manifestPath: string, repoRoot: string, view?: RegistryCatalogInputView): ComponentSourceOwnerV1 {
  const text = view ? view.readText(relative(repoRoot, manifestPath).replaceAll("\\", "/")) : readFileSync(manifestPath, "utf8");
  const packageName = text.match(/^name = "([^"]+)"/m)?.[1];
  if (!packageName) throw new Error(`missing package name in ${manifestPath}`);
  const packageId = parseComponentPackageId(text, manifestPath);
  const pluginId = packageId.slice("semio:".length);
  const cratePath = relative(repoRoot, dirname(manifestPath)).replaceAll("\\", "/");
  const wasmOut = `${packageName.replace(/-/g, "_")}.wasm`;
  const directoryName = declaredComponentDeploymentDirectoryV1(text);
  const semioBlock = tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[package.metadata.semio]")[0];
  const semioText = semioBlock?.join("\n") ?? "";
  const consumes = parseTomlStringArray(semioText, "consumes");
  const role = declaredComponentKind(text);
  if (!role) throw new Error("Missing authored component kind in " + manifestPath);
  const extendsHost = semioText.match(/^extends\s*=\s*"([^"]+)"/m)?.[1];
  const hostBlock = semioText.match(/^host\s*=\s*\{([^}]*)\}/m)?.[1];
  const landingAppId = hostBlock?.match(/landing\s*=\s*"([^"]+)"/)?.[1];
  const hostAppId = hostBlock?.match(/shell\s*=\s*"([^"]+)"/)?.[1];
  const host = landingAppId && hostAppId ? { landingAppId, hostAppId } : undefined;
  const declaredDependsOnIds = parseSemioDependsOnIds(semioText, pluginId, manifestPath);
  const dependsOn = extendsHost ? [extendsHost, ...declaredDependsOnIds.filter((id) => id !== extendsHost)] : declaredDependsOnIds;

  return parseComponentSourceRowV1({ pluginId, packageId, cratePath, packageName, wasmOut,
    ...(directoryName === undefined ? {} : { directoryName }), role, consumes, dependsOn,
    ...(extendsHost ? { extends: extendsHost } : {}), ...(host ? { host } : {}) });
}

/** 🛂️Reads a compiled descriptor only after admitting its exact source identity. */
export function parseCompiledComponentOwnerV1(manifestPath: string, repoRoot: string, view?: RegistryCatalogInputView): CompiledComponentOwnerV1 {
  const source = parseComponentSourceOwnerV1(manifestPath, repoRoot, view);
  const descriptor = readDescriptorJson(repoRoot, source.cratePath, view);
  let capabilities: string[];
  let contributes: string[];
  let activationEvents: string[] = [];
  let extensionPoints: string[] = [];
  let executionMode: string | undefined;
  let hashes: PluginDescriptorHashes | undefined;
  {
    if (descriptor.role !== source.role || descriptor.packageId !== source.packageId || (descriptor.manifest as { pluginId: string }).pluginId !== source.pluginId) throw new Error("Compiled descriptor differs from its authored component identity in " + manifestPath);
    const capabilityRequests = Array.isArray(descriptor.capabilityRequests) ? descriptor.capabilityRequests : [];
    capabilities = uniqueStrings(capabilityRequests.map((row) => (row as { id?: unknown }).id).filter((id): id is string => typeof id === "string"));
    const contributions = descriptor.contributions as Record<string, unknown> | undefined;
    const topicContributions = Array.isArray(contributions?.topicContributions) ? (contributions!.topicContributions as unknown[]) : [];
    contributes = uniqueStrings(topicContributions.map((row) => (row as { topic?: unknown }).topic).filter((topic): topic is string => typeof topic === "string"));
    const rawActivationEvents = Array.isArray(descriptor.activationEvents) ? descriptor.activationEvents : [];
    activationEvents = rawActivationEvents.map(formatActivationEvent).filter((event): event is string => event !== undefined);
    for (const kind of descriptorSurfaceArtifactKinds(descriptor)) {
      const event = `${ON_ARTIFACT_KIND_PREFIX}${kind}`;
      if (!activationEvents.includes(event)) activationEvents.push(event);
    }
    const rawExtensionPoints = Array.isArray(descriptor.extensionPoints) ? descriptor.extensionPoints : [];
    extensionPoints = uniqueStrings(rawExtensionPoints.map((row) => (row as { id?: unknown }).id).filter((id): id is string => typeof id === "string"));
    activationEvents = uniqueStrings(activationEvents);
    executionMode = typeof descriptor.execution === "string" ? descriptor.execution : undefined;
    const rawHashes = descriptor.hashes as Record<string, unknown> | undefined;
    if (rawHashes && typeof rawHashes.wasmSha256 === "string" && typeof rawHashes.coreWasmSha256 === "string" && typeof rawHashes.descriptorSha256 === "string") {
      hashes = { wasmSha256: rawHashes.wasmSha256, coreWasmSha256: rawHashes.coreWasmSha256, descriptorSha256: rawHashes.descriptorSha256 };
    }
  }
  return parseCompiledComponentRowV1({ ...source, capabilities, contributes, activationEvents, extensionPoints,
    ...(executionMode ? { executionMode } : {}), ...(hashes ? { hashes } : {}) });
}



export function generatePluginRegistry(repoRoot = getWorkspaceRoot(), options: GeneratePluginRegistryOptions = {}): DeployedRegistryEntryV1[] {
  const filterPlaygroundPlugin = options.filterPlaygroundPlugin;
  const filterIds = filterPlaygroundPlugin && !isHostPluginFilter(filterPlaygroundPlugin) ? resolveRegistryPluginIdsForFilter(filterPlaygroundPlugin) : undefined;
  const manifestPaths = filterIds ? findPluginCargoPathsForIds(repoRoot, filterIds) : findPluginCargoFiles(repoRoot, options.packages ?? (options.view ? discoverCatalogPackages(repoRoot, TAXONOMY, options.view) : undefined), options.view);
  const entries: DeployedRegistryEntryV1[] = [];
  for (const path of manifestPaths) {
    entries.push(parseDeployedRegistryEntryV1(parseCompiledComponentOwnerV1(path, repoRoot, options.view)));
  }
  entries.sort((a, b) => a.pluginId.localeCompare(b.pluginId));
  return claimOwnedArtifactKinds(entries);
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
 * 🔗️ The runtime actor dependencies one crate DECLARES, read from
 * `[package.metadata.semio].depends-on` — the same plugin-id set its builder passes to
 * `.depends_on(id, VersionPin)` (`🔌️plugin/🦀️.rs`), kept in the Cargo manifest as well because the
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
 * {@link parseComponentSourceOwnerV1}, so an extension never repeats its host here.
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



/** 🎮️ The owning plugin id and the app scope of the playground row a variant/alias names, or
 * `undefined` for a filter that names no playground row. A row carrying `app` boots that ONE artifact
 * app; a row without one boots its crate's default session (the OS shell, for the host crate). */
export function resolvePlaygroundFilterRow(pluginFilter: string, repoRoot = getWorkspaceRoot()): { readonly pluginId: string; readonly app?: string } | undefined {
  for (const manifestPath of findPluginCargoFiles(repoRoot)) {
    const text = readFileSync(manifestPath, "utf8");
    const componentPackage = parseComponentPackageId(text, manifestPath).slice("semio:".length);
    for (const block of tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[[package.metadata.semio.playground]]")) {
      const body = block.join("\n");
      const variant = body.match(/^variant\s*=\s*"([^"]+)"/m)?.[1];
      if (!variant) continue;
      const aliases = parseTomlStringArray(body, "aliases");
      const app = body.match(/^app\s*=\s*"([^"]+)"/m)?.[1];
      if (variant === pluginFilter || aliases.includes(pluginFilter)) return { pluginId: componentPackage, ...(app ? { app } : {}) };
    }
  }
  return undefined;
}


/** 🎯️ Resolves a playground variant/alias or bare plugin id to its wasm registry plugin id. */
export function resolveRegistryPluginIdForFilter(pluginFilter: string, repoRoot = getWorkspaceRoot()): string {
  return resolvePlaygroundFilterRow(pluginFilter, repoRoot)?.pluginId ?? pluginFilter;
}



export function pluginEntryHasHost(pluginId: string, repoRoot: string): boolean {
  for (const manifestPath of findPluginCargoFiles(repoRoot)) {
    const entry = parseComponentSourceOwnerV1(manifestPath, repoRoot);
    if (entry.pluginId === pluginId) return entry.host !== undefined;
  }
  return false;
}



/** 🏠️ True when the filter boots the host SESSION: a playground row of the crate that declares
 * `[package.metadata.semio].host` AND names no `app`. The host crate also ships ordinary artifact apps
 * (`🪐️space`'s Home and Space); a row naming one of them is a single-app playground like any other. */
export function isHostPluginFilter(pluginFilter?: string, repoRoot = getWorkspaceRoot()): boolean {
  if (!pluginFilter) return true;
  const row = resolvePlaygroundFilterRow(pluginFilter, repoRoot);
  if (row?.app !== undefined) return false;
  return pluginEntryHasHost(row?.pluginId ?? pluginFilter, repoRoot);
}



/** 🎯️ Resolves aliases and runtime dependencies within the supplied catalog, or discovers an omitted catalog. */
export function resolveRegistryPluginIdsForFilter(filterPlaygroundPlugin: string, allEntries: readonly ComponentSourceOwnerV1[] = generatePluginRegistry(getWorkspaceRoot()), playgrounds?: readonly { readonly variant: string; readonly aliases: readonly string[]; readonly pluginId: string; readonly app?: string }[]): readonly string[] {
  const variantRow = playgrounds?.find((p) => p.variant === filterPlaygroundPlugin || p.aliases.includes(filterPlaygroundPlugin)) ?? (playgrounds === undefined ? resolvePlaygroundFilterRow(filterPlaygroundPlugin) : undefined);
  const targetPluginId = variantRow?.pluginId ?? filterPlaygroundPlugin;
  return allEntries.some(row => row.pluginId === targetPluginId) ? runtimeComponentClosure(allEntries, [{ id: targetPluginId, appScoped: variantRow?.app !== undefined }]) : [];
}



export function findPluginCargoPathsForIds(repoRoot: string, pluginIds: readonly string[]): string[] {
  const idSet = new Set(pluginIds);
  return findPluginCargoFiles(repoRoot).filter((path) => {
    const entry = parseComponentSourceOwnerV1(path, repoRoot);
    return idSet.has(entry.pluginId);
  });
}
