#!/usr/bin/env bun
/**
 * 📜️ `@semio-tech/plugin-registry` — single-source plugin/playground/framework catalog codegen from
 * workspace packages. Discovery is the shared repo-wide contract (`🔣️taxonomy.json` +
 * `discoverPackages()` in `🦑️repo/📚️library`), not path regexes local to this script; every plugin area
 * root comes from `taxonomy.pluginAreas`, and each declared `AreaState` decides whether the taxonomy
 * tree audit warns or hard-fails.
 *
 * `generate` writes `🤖️generated/*` plus `.vscode/launch.json` (both derived from the same playground
 * catalog); `check` byte-compares every one of those artifacts and never writes.
 *
 * @see .🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️06/REGISTRY-SCRIPT-REFACTOR-TO-VOCABULARY-DISCOVERY-LIBRARY
 */
import { createHash } from "node:crypto";
import { closeSync, existsSync, fstatSync, lstatSync, mkdirSync, openSync, readdirSync, readFileSync, readSync, realpathSync, rmSync, statSync, writeFileSync } from "node:fs";
import { isAbsolute, basename, dirname, join, relative, resolve } from "node:path";
import { isDeepStrictEqual } from "node:util";
import type { AreaState, ArtifactScaffoldLeaf, ArtifactScaffoldOptions, ArtifactScaffoldResult, DiscoveredPackage, PackageRole, RegistryCatalogInputView } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { authorArtifactScaffold, BundleScript, canonicalPrimaryFilenameForKind, discoverCatalogPackages, discoverPackageProblems, discoverPackages, getWorkspaceRoot, loadCatalogTaxonomy, parseRegistryCatalogProjection, registryCatalogInputView, registryCatalogProjectedInputView, registryExampleCatalog, runBundleScriptMain, runVitest, ScriptRouter, validateGeneratorContractsAgainstWorkspace } from "../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🟦️.ts";
import { decodePackValue, encodePackValue } from "../../../🟦️.ts";
import { generateLaunchJson, LAUNCH_OUTPUT_REL_PATH } from "./🖥️launch.ts";

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
  /** 🔗️ Every sibling `semio-s-plugin-<id>` Cargo dependency this crate declares, derived straight
   * from its manifest — the ground-truth dependency edge set (contract freeze §4 rule 2) ahead of
   * the runtime `.depends_on(...)` API's rollout (ticket 26/08/16/PLUGIN-DEPENDENCIES-ARTIFACT-
   * CONTRIBUTIONS-AND-COMPOSITE-MUTATIONS). For an extension, `extends` is always `dependsOn[0]`
   * (contract freeze §4 rule 1). Consumed by `resolveRegistryPluginIdsForFilter` to close a dev
   * session's plugin set transitively. */
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

const COMPONENT_MANIFEST_MAX_BYTES = 64 * 1024;
const COMPONENT_PACKAGE_ID = /^semio:[a-z0-9]+(?:-[a-z0-9]+)*$/;

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
const TAXONOMY = loadCatalogTaxonomy();

/** 📄️ Resolves the taxonomy-ordered primary filename for consumers that require one component leaf. */
function primaryFilenameForKind(kindId: string): string {
  return canonicalPrimaryFilenameForKind(kindId, TAXONOMY);
}

/** @emoji 🗺️ Every area root that may hold a plugin crate, cross-checked against `taxonomy.areas` at
 * load time so none of these literals can outlive a vocabulary rename. Membership across this array —
 * never equality against one hand-picked literal — is how every plugin-tree path test below decides
 * "is this under a plugin area"; its dedicated taxonomy-tree state decides whether findings warn or
 * fail (see `PLUGIN_AREAS_STATE`). */
const PLUGIN_AREAS: readonly string[] = TAXONOMY.pluginAreas;
if (!Array.isArray(PLUGIN_AREAS) || PLUGIN_AREAS.length === 0) throw new Error(`📇️registry: 🔣️taxonomy.json must declare a non-empty "pluginAreas" array`);
for (const area of PLUGIN_AREAS) {
  if (!(area in TAXONOMY.areas)) throw new Error(`📇️registry: "${area}" is not a declared area in 🔣️taxonomy.json (${Object.keys(TAXONOMY.areas).join(", ")})`);
}

/** @emoji 🗺️ Merges every declared plugin area's `AreaState` to the most permissive member, so one
 * still-migrating area can never be silently masked by a sibling area that already reached `clean`. */
function mergeAreaStates(states: readonly AreaState[]): AreaState {
  if (states.includes("legacy")) return "legacy";
  if (states.includes("mixed")) return "mixed";
  return "clean";
}

/** @emoji 🌳️ Declared taxonomy-tree maturity across every plugin area, independent of the package-layout
 * maturity in `areas`. `legacy`/`mixed` ⇒ findings are warn-only; `clean` ⇒ they fail the gate. */
const PLUGIN_AREAS_STATE: AreaState = mergeAreaStates(PLUGIN_AREAS.map((area) => TAXONOMY.areas[area]));

/** @emoji 📚️ Artifact-scoped example data dir (`artifactChildDirs`, not owner root). */
const EXAMPLES_DIRNAME = "📚️examples";
if (!TAXONOMY.artifactChildDirs.includes(EXAMPLES_DIRNAME)) {
  throw new Error(`📇️registry: "${EXAMPLES_DIRNAME}" must be listed in 🔣️taxonomy.json artifactChildDirs (${TAXONOMY.artifactChildDirs.join(", ")})`);
}
const EXAMPLE_ASSETS_DIRNAME = TAXONOMY.exampleAssetsDirName ?? "🖼️assets";
const EXAMPLE_TESTS_DIRNAME = TAXONOMY.exampleTestsDirName ?? "🧪️tests";
const EXAMPLE_RUST_LEAF = primaryFilenameForKind(TAXONOMY.exampleFileKinds["🦀️rust"]);
const EXAMPLE_TS_LEAF = primaryFilenameForKind(TAXONOMY.exampleFileKinds["🟦️typescript"]);
const EXAMPLE_SLUG_RE = new RegExp(TAXONOMY.exampleSlugPattern ?? "^.+\uFE0F[a-z0-9]+(?:-[a-z0-9]+)*$", "u");
const FORBIDDEN_EXAMPLE_PLURAL_DIRS = TAXONOMY.forbiddenExamplePluralDirs ?? [];
const FORBIDDEN_EXAMPLE_SLUGS = new Set(TAXONOMY.forbiddenExampleSlugs ?? []);

/** @emoji ✅️ True when `name` is an emoji+VS16+kebab example slug (and not a forbidden placeholder). */
function isExampleSlugName(name: string): boolean {
  return EXAMPLE_SLUG_RE.test(name) && !FORBIDDEN_EXAMPLE_SLUGS.has(name);
}

const RUST_LANG = "🦀️rust";

/** @emoji 🧩️ Roles whose packages may carry a `[package.metadata.component]` wasm component and thus
 * belong in the plugin catalog: the plugin itself and the extensions it contributes. Every other role
 * (`framework`, `tool`, `s-module`, …) is filtered out by `tryParsePluginCargo` anyway — listing them
 * here keeps the intent explicit instead of implicit in a downstream parse failure. */
const COMPONENT_ROLES: ReadonlySet<PackageRole> = new Set<PackageRole>(["plugin", "extension"]);

/** @emoji 📦️ Every rust package in the repo that declares a component-bearing role, via the shared
 * `discoverPackages()` walk (two-level `📦️packages/🦀️rust/` and three-level `🎯️targets/<t>/` shapes
 * alike). Replaces the two hand-written "new contract" path regexes this script used to carry. */
function discoverComponentPackages(repoRoot: string, packages: readonly DiscoveredPackage[] = discoverCatalogPackages(repoRoot, TAXONOMY)): DiscoveredPackage[] {
  return packages.filter((pkg) => pkg.lang === RUST_LANG && COMPONENT_ROLES.has(pkg.role));
}

//#endregion 🏛️DiscoveryContract

/** @emoji 🧭️ Every manifest that may contribute a row to the plugin catalog, via the shared package
 * discovery contract. The pre-Shape-V2 legacy sandwich shape this used to also admit was removed once
 * every declared plugin area reached `clean` — see `PLUGIN_AREAS_STATE`. */
function findPluginCargoFiles(root: string, packages?: readonly DiscoveredPackage[]): string[] {
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
const DESCRIPTOR_JSON_REL_PATH = TAXONOMY.generatorContracts["plugin-registry"].inputDiscovery!.descriptorRelativePath.split("/");

/** 🎬️ `kernel::ActivationEvent`'s default (externally tagged) serde JSON shape, decoded into
 * `📓️design-abi.md` §2's canonical dash-separated string form. Unit variant `OnStartupFinished`
 * serializes as the bare string `"onStartupFinished"`; every other variant as
 * `{ "<camelTag>": { ...fields } }`. */
function formatActivationEvent(raw: unknown): string | undefined {
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
function readDescriptorJson(repoRoot: string, cratePath: string, view: RegistryCatalogInputView = registryCatalogInputView(repoRoot, TAXONOMY)): Record<string, unknown> | undefined {
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
function parsePluginCargo(manifestPath: string, repoRoot: string, view?: RegistryCatalogInputView, ownerDescriptors: "required" | "ignored" = "required"): PluginRegistryEntry {
  const text = view ? view.readText(relative(repoRoot, manifestPath).replaceAll("\\", "/")) : readFileSync(manifestPath, "utf8");
  const packageName = text.match(/^name = "([^"]+)"/m)?.[1];
  if (!packageName) throw new Error(`missing package name in ${manifestPath}`);
  const packageId = parseComponentPackageId(text, manifestPath);
  const pluginId = packageId.slice("semio:".length);
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
  const cargoDependsOnIds = parseCargoPluginDependencyIds(text, pluginId);
  // 🔗️ contract freeze §4 rule 1: for an extension, `extends` is always dependsOn[0].
  const dependsOn = extendsHost ? [extendsHost, ...cargoDependsOnIds.filter((id) => id !== extendsHost)] : cargoDependsOnIds;

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

//#region 🔖️PlaygroundEntry
/** @emoji 🗂️ One `[[package.metadata.semio.assets]]` row: a dev-time asset-serving need declared by a
 * plugin crate. `app` optionally scopes the row to one playground variant of a multi-app crate (unset
 * ⇒ every variant of the crate). Mirrors the TS discriminated union emitted for consumers as
 * `PlaygroundAssetSpec` (see `emitPlaygroundsTypeScript`). */
export type AssetSpecRow = {
  readonly kind: "tile-proxy" | "static-dir" | "mesh-collection";
  readonly route: string;
  readonly app?: string;
  readonly upstream?: string;
  readonly cache?: string;
  readonly root?: string;
  readonly roots?: readonly string[];
  readonly placeholder?: string;
  readonly filterFromExamples?: boolean;
};

/** @emoji 🎮️ One `[[package.metadata.semio.playground]]` row scoped to its owning plugin crate. */
export type PlaygroundEntry = {
  readonly variant: string;
  readonly pluginId: string;
  readonly cratePath: string;
  readonly app?: string;
  /** @emoji 🏷️ Shell brand id (see `framework/os/dev/brand`) this variant ships as. */
  readonly brand?: string;
  readonly aliases: readonly string[];
  readonly ports: { readonly react: number; readonly wgpu: number };
  /** @emoji 👥️ Extra per-user dev ports for a multi-user collaborative session (e.g. hub-backed `s`
   * studio dev launchers) — one port per concurrent user, over and above the single-user `ports` row. */
  readonly userPorts?: { readonly react: readonly number[]; readonly wgpu: readonly number[] };
  readonly examples: readonly string[];
  /** @emoji 🔌️ Crate paths whose `wasm` build target must run for this playground variant. */
  readonly engines: readonly string[];
  /** @emoji 🗂️ Dev-time asset-serving needs for this variant. */
  readonly assets: readonly AssetSpecRow[];
};

function tomlBlocksAfterHeader(lines: readonly string[], headerTest: (line: string) => boolean): string[][] {
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

function parseTomlStringArray(block: string, key: string): string[] {
  const match = block.match(new RegExp(`^${key}\\s*=\\s*\\[([^\\]]*)\\]`, "m"));
  if (!match) return [];
  return [...match[1].matchAll(/"([^"]*)"/g)].map((m) => m[1]);
}

function parseTomlBoolField(block: string, key: string): boolean {
  return new RegExp(`^${key}\\s*=\\s*true\\s*$`, "m").test(block);
}

/** @emoji 🔢️ Every integer in a `key = [1, 2]` inline TOML array found inside `text` (used for
 * sub-blocks like `user_ports = { react = [...], wgpu = [...] }` where `react`/`wgpu` aren't at the
 * start of a line). */
function parseTomlInlineNumberArray(text: string, key: string): number[] {
  const match = text.match(new RegExp(`${key}\\s*=\\s*\\[([^\\]]*)\\]`));
  if (!match) return [];
  return [...match[1].matchAll(/\d+/g)].map((m) => Number(m[0]));
}

/** @emoji 🔗️ Every `semio-s-plugin-<id>` entry in one crate's own Cargo manifest text, both the
 * `key = { …, package = "semio-s-plugin-x" }` renamed-dependency shape and the plain
 * `semio-s-plugin-x = { … }` shape — mirrors the root policy script's
 * `policyCargoPluginDependencyIds` (`📜️script.ts:7562`) so the derived catalog and the
 * `plugin-dependency/parity` gate can never read two different dependency sets from the same file.
 * `ownId` (this crate's own `[package.metadata.component]` plugin id) is excluded so a crate can
 * never be listed as depending on itself. */
function parseCargoPluginDependencyIds(manifestText: string, ownId: string): string[] {
  const ids = new Set<string>();
  for (const match of manifestText.matchAll(/(?:^|\n)\s*(?:[\w-]+\s*=\s*\{[^}]*?)?package\s*=\s*"semio-s-plugin-([a-z0-9-]+)"/g)) {
    ids.add(match[1]!);
  }
  for (const match of manifestText.matchAll(/(?:^|\n)\s*semio-s-plugin-([a-z0-9-]+)\s*=/g)) {
    ids.add(match[1]!);
  }
  ids.delete(ownId);
  return [...ids].sort();
}

/** 📦️ Reads exact Cargo package identities so the strict gate never equates package and plugin ids. */
function parseCargoPluginDependencyPackageNames(manifestText: string, ownPackageName: string): string[] {
  const names = new Set<string>();
  for (const match of manifestText.matchAll(/(?:^|\n)\s*(?:[\w-]+\s*=\s*\{[^}]*?)?package\s*=\s*"(semio-s-plugin-[a-z0-9-]+)"/g)) names.add(match[1]!);
  for (const match of manifestText.matchAll(/(?:^|\n)\s*(semio-s-plugin-[a-z0-9-]+)\s*=/g)) names.add(match[1]!);
  names.delete(ownPackageName);
  return [...names].sort();
}

function parsePlaygroundBlock(block: string, pluginId: string, cratePath: string): PlaygroundEntry | undefined {
  const variant = block.match(/^variant\s*=\s*"([^"]+)"/m)?.[1];
  if (!variant) return undefined;
  const app = block.match(/^app\s*=\s*"([^"]+)"/m)?.[1];
  const brand = block.match(/^brand\s*=\s*"([^"]+)"/m)?.[1];
  const aliases = parseTomlStringArray(block, "aliases");
  const portsBlock = block.match(/^ports\s*=\s*\{([^}]*)\}/m)?.[1];
  const react = portsBlock?.match(/react\s*=\s*(\d+)/)?.[1];
  const wgpu = portsBlock?.match(/wgpu\s*=\s*(\d+)/)?.[1];
  if (!react || !wgpu) return undefined;
  const userPortsBlock = block.match(/^user_ports\s*=\s*\{([^}]*)\}/m)?.[1];
  const userPortsReact = userPortsBlock ? parseTomlInlineNumberArray(userPortsBlock, "react") : [];
  const userPortsWgpu = userPortsBlock ? parseTomlInlineNumberArray(userPortsBlock, "wgpu") : [];
  const userPorts = userPortsReact.length > 0 && userPortsWgpu.length > 0 ? { react: userPortsReact, wgpu: userPortsWgpu } : undefined;
  const engines = parseTomlStringArray(block, "engines");
  return { variant, pluginId, cratePath, app, brand, aliases, ports: { react: Number(react), wgpu: Number(wgpu) }, ...(userPorts ? { userPorts } : {}), examples: [], engines, assets: [] };
}

/** @emoji 🗂️ Parses every `[[package.metadata.semio.assets]]` row for one crate manifest. */
function parseAssetsForCrate(manifestPath: string, repoRoot: string, view?: RegistryCatalogInputView): AssetSpecRow[] {
  const path = relative(repoRoot, manifestPath).replaceAll("\\", "/");
  if (view ? view.kind(path) === null : !existsSync(manifestPath)) return [];
  const text = view ? view.readText(path) : readFileSync(manifestPath, "utf8");
  const blocks = tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[[package.metadata.semio.assets]]");
  const rows: AssetSpecRow[] = [];
  for (const blockLines of blocks) {
    const block = blockLines.join("\n");
    const kind = block.match(/^kind\s*=\s*"([^"]+)"/m)?.[1] as AssetSpecRow["kind"] | undefined;
    const route = block.match(/^route\s*=\s*"([^"]+)"/m)?.[1];
    if (!kind || !route) {
      continue;
    }
    const app = block.match(/^app\s*=\s*"([^"]+)"/m)?.[1];
    const upstream = block.match(/^upstream\s*=\s*"([^"]+)"/m)?.[1];
    const cache = block.match(/^cache\s*=\s*"([^"]+)"/m)?.[1];
    const root = block.match(/^root\s*=\s*"([^"]+)"/m)?.[1];
    const roots = parseTomlStringArray(block, "roots");
    const placeholder = block.match(/^placeholder\s*=\s*"([^"]+)"/m)?.[1];
    const filterFromExamples = parseTomlBoolField(block, "filter_from_examples");
    rows.push({
      kind,
      route,
      ...(app ? { app } : {}),
      ...(upstream ? { upstream } : {}),
      ...(cache ? { cache } : {}),
      ...(root ? { root } : {}),
      ...(roots.length ? { roots } : {}),
      ...(placeholder ? { placeholder } : {}),
      ...(filterFromExamples ? { filterFromExamples: true } : {}),
    });
  }
  return rows;
}

/**
 * @emoji 🖼️ Example ids for one playground row: emoji-slug dirs under `🗿️artifacts/<a>/📚️examples/` and
 * every `👁️viewer`/`✏️editor` surface's `📚️examples/` that carry a definition leaf. Falls back to the
 * variant-suffix surface (matched by its subset name) when artifact/surface scans are empty.
 */
function discoverExamplesForPlayground(repoRoot: string, cratePath: string, _pluginId: string, _variant: string, view?: RegistryCatalogInputView): string[] {
  return registryExampleCatalog(repoRoot, cratePath, TAXONOMY, view);
}


function parsePlaygroundsForCrate(manifestPath: string, pluginId: string, cratePath: string, repoRoot: string, view?: RegistryCatalogInputView): PlaygroundEntry[] {
  const text = view ? view.readText(relative(repoRoot, manifestPath).replaceAll("\\", "/")) : readFileSync(manifestPath, "utf8");
  const blocks = tomlBlocksAfterHeader(text.split("\n"), (line) => line === "[[package.metadata.semio.playground]]");
  const entries: PlaygroundEntry[] = [];
  for (const block of blocks) {
    const entry = parsePlaygroundBlock(block.join("\n"), pluginId, cratePath);
    if (entry) entries.push(entry);
  }
  return entries;
}

/** @emoji 🕹️ Scans every plugin/module crate for `[[package.metadata.semio.playground]]` rows and flattens them into one repo-wide catalog. */
export function generatePlaygroundRegistry(repoRoot = getWorkspaceRoot(), options: GeneratePluginRegistryOptions = {}): PlaygroundEntry[] {
  const entries = generatePluginRegistry(repoRoot, options);
  const playgrounds: PlaygroundEntry[] = [];
  for (const entry of entries) {
    const manifestPath = join(repoRoot, entry.cratePath, "Cargo.toml");
    const crateAssets = parseAssetsForCrate(manifestPath, repoRoot, options.view);
    for (const playground of parsePlaygroundsForCrate(manifestPath, entry.pluginId, entry.cratePath, repoRoot, options.view)) {
      const assets = crateAssets.filter((asset) => asset.app === undefined || asset.app === playground.app);
      playgrounds.push({ ...playground, examples: discoverExamplesForPlayground(repoRoot, entry.cratePath, entry.pluginId, playground.variant, options.view), assets });
    }
  }
  for (let i = 0; i < playgrounds.length; i++) {
    const row = playgrounds[i];
    if (!row.brand || row.examples.length > 0) continue;
    const donor = playgrounds.find((other) => other !== row && other.cratePath === row.cratePath && other.app === row.app && other.examples.length > 0);
    if (donor) playgrounds[i] = { ...row, examples: donor.examples, engines: row.engines.length > 0 ? row.engines : donor.engines };
  }
  playgrounds.sort((a, b) => a.variant.localeCompare(b.variant));
  return playgrounds;
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

function pluginEntryHasHost(pluginId: string, repoRoot: string): boolean {
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

/**
 * 🎯️ Resolves a raw playground filter (a variant id like "puzzle5d", or an already-bare crate
 * pluginId like "note") to the set of crate pluginIds that must be built for one dev session: the
 * target crate itself, plus every crate whose declared `contributes` intersects the target crate
 * `consumes` (per `[package.metadata.semio]` in each crate Cargo.toml — no more registry-id
 * indirection through framework/core/js), plus the FULL TRANSITIVE `dependsOn` closure of everything
 * gathered so far (contract freeze §4/§5's dependency graph — a dev session for one plugin must also
 * load every plugin/extension it depends on, however many hops deep, not just its direct Cargo
 * dependencies). The two membership rules are additive, not a replacement of one by the other: some
 * topic-based consumption (e.g. `demonstrator` consuming `forms.questionKind`) is not backed by a
 * Cargo dependency edge at all, so dropping the topic scan would silently shrink existing dev
 * sessions.
 */
export function resolveRegistryPluginIdsForFilter(filterPlaygroundPlugin: string): readonly string[] {
  const repoRoot = getWorkspaceRoot();
  const allEntries = generatePluginRegistry(repoRoot);
  const playgrounds = generatePlaygroundRegistry(repoRoot);
  const variantRow = playgrounds.find((p) => p.variant === filterPlaygroundPlugin);
  const targetPluginId = variantRow?.pluginId ?? filterPlaygroundPlugin;
  const byId = new Map(allEntries.map((entry) => [entry.pluginId, entry]));
  const targetEntry = byId.get(targetPluginId);
  const ids = new Set<string>([targetPluginId]);
  if (targetEntry) {
    for (const entry of allEntries) {
      if (entry.pluginId === targetPluginId) continue;
      if (entry.contributes.some((topic) => targetEntry.consumes.includes(topic))) ids.add(entry.pluginId);
    }
  }
  // 🔗️ Transitive dependsOn closure over whatever the topic scan already gathered — a BFS/DFS-order-
  // agnostic worklist since `ids` only ever grows and every id is pushed at most once.
  const queue = [...ids];
  while (queue.length > 0) {
    const id = queue.pop()!;
    for (const depId of byId.get(id)?.dependsOn ?? []) {
      if (ids.has(depId)) continue;
      ids.add(depId);
      queue.push(depId);
    }
  }
  return [...ids];
}

function findPluginCargoPathsForIds(repoRoot: string, pluginIds: readonly string[]): string[] {
  const idSet = new Set(pluginIds);
  return findPluginCargoFiles(repoRoot).filter((path) => {
    const entry = tryParsePluginCargo(path, repoRoot);
    return entry !== undefined && idSet.has(entry.pluginId);
  });
}

function tryParsePluginCargo(manifestPath: string, repoRoot: string, view?: RegistryCatalogInputView): PluginRegistryEntry | undefined {
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

/** @emoji 🏠️ Resolves the one playground variant that boots as the host/shell session: the data-driven
 * replacement for the previous hardcoded `"s"` literal. Exactly one plugin crate in the catalog may
 * declare `[package.metadata.semio].host` (see `parsePluginCargo`'s `host`/`shell` parse) — this scans
 * for that crate and returns its own `[[package.metadata.semio.playground]]` variant id, throwing a
 * clear error if zero or more than one plugin crate declares the host table. */
export function resolveDefaultHostVariant(repoRoot = getWorkspaceRoot()): string {
  const packages = discoverCatalogPackages(repoRoot, TAXONOMY);
  return defaultHostVariant(generatePluginRegistry(repoRoot, { packages }), generatePlaygroundRegistry(repoRoot, { packages }));
}

/** 🏠️ One host identity resolved from the same already-rendered catalog rows. */
function defaultHostVariant(entries: readonly PluginRegistryEntry[], playgrounds: readonly PlaygroundEntry[]): string {
  const hostEntries = entries.filter((entry) => entry.host !== undefined);
  if (hostEntries.length !== 1) {
    throw new Error(`📇️registry: expected exactly one plugin crate to declare [package.metadata.semio].host, found ${hostEntries.length}${hostEntries.length > 0 ? ` (${hostEntries.map((entry) => entry.pluginId).join(", ")})` : ""}`);
  }
  const hostPluginId = hostEntries[0].pluginId;
  const hostPlayground = playgrounds.find((entry) => entry.pluginId === hostPluginId);
  if (!hostPlayground) throw new Error(`📇️registry: host plugin "${hostPluginId}" declares no [[package.metadata.semio.playground]] variant`);
  return hostPlayground.variant;
}

function emitTypeScript(entries: PluginRegistryEntry[]): string {
  const pluginEntries = entries.filter((entry) => entry.role === "plugin");
  const extensionEntries = entries.filter((entry) => entry.role === "extension");
  const hostRows = entries
    .filter((entry) => entry.host)
    .map((entry) => `\t{ pluginId: ${JSON.stringify(entry.pluginId)}, landingAppId: ${JSON.stringify(entry.host!.landingAppId)}, hostAppId: ${JSON.stringify(entry.host!.hostAppId)} },`)
    .join("\n");
  const formatTargetRow = (entry: PluginRegistryEntry) => {
    const host = entry.host ? `, host: { landingAppId: ${JSON.stringify(entry.host.landingAppId)}, hostAppId: ${JSON.stringify(entry.host.hostAppId)} }` : "";
    const extendsHost = entry.extends ? `, extends: ${JSON.stringify(entry.extends)}` : "";
    const executionMode = entry.executionMode ? `, executionMode: ${JSON.stringify(entry.executionMode)}` : "";
    const hashes = entry.hashes ? `, hashes: { wasmSha256: ${JSON.stringify(entry.hashes.wasmSha256)}, coreWasmSha256: ${JSON.stringify(entry.hashes.coreWasmSha256)}, descriptorSha256: ${JSON.stringify(entry.hashes.descriptorSha256)} }` : "";
    return `\t{ pluginId: ${JSON.stringify(entry.pluginId)}, packageId: ${JSON.stringify(entry.packageId)}, cratePath: ${JSON.stringify(entry.cratePath)}, wasmOut: ${JSON.stringify(entry.wasmOut)}, role: ${JSON.stringify(entry.role)}, capabilities: ${JSON.stringify(entry.capabilities)}, contributes: ${JSON.stringify(entry.contributes)}, consumes: ${JSON.stringify(entry.consumes)}, dependsOn: ${JSON.stringify(entry.dependsOn)}, activationEvents: ${JSON.stringify(entry.activationEvents)}, extensionPoints: ${JSON.stringify(entry.extensionPoints)}${extendsHost}${host}${executionMode}${hashes} },`;
  };
  const pluginRows = pluginEntries.map(formatTargetRow).join("\n");
  const extensionRows = extensionEntries.map(formatTargetRow).join("\n");
  return `/** @generated by framework/plugin/registry/script.ts — do not edit. */
export type PluginHostMetadata = {
\treadonly landingAppId: string;
\treadonly hostAppId: string;
};

export type PluginHostConfig = PluginHostMetadata & {
\treadonly pluginId: string;
};

export type PluginDescriptorHashes = {
\treadonly wasmSha256: string;
\treadonly coreWasmSha256: string;
\treadonly descriptorSha256: string;
};

export type PluginBuildTarget = {
\treadonly pluginId: string;
\treadonly packageId: string;
\treadonly cratePath: string;
\treadonly wasmOut: string;
\treadonly role: "plugin" | "extension";
\treadonly extends?: string;
\treadonly capabilities: readonly string[];
\treadonly contributes: readonly string[];
\treadonly consumes: readonly string[];
\t/** @emoji 🔗️ Every sibling \`semio-s-plugin-<id>\` Cargo dependency this crate declares (extension's
\t * \`extends\` target always first) — see \`PluginRegistryEntry.dependsOn\` in
\t * \`📇️registry/📜️script.ts\`. */
\treadonly dependsOn: readonly string[];
\treadonly host?: PluginHostMetadata;
\t/** @emoji 🎬️ See \`PluginRegistryEntry.activationEvents\` in \`📇️registry/📜️script.ts\`. */
\treadonly activationEvents: readonly string[];
\t/** @emoji 🧩️ See \`PluginRegistryEntry.extensionPoints\` in \`📇️registry/📜️script.ts\`. */
\treadonly extensionPoints: readonly string[];
\treadonly executionMode?: string;
\treadonly hashes?: PluginDescriptorHashes;
};

export const PLUGIN_HOST_CONFIGS: readonly PluginHostConfig[] = [
${hostRows}
];

export const PLUGIN_BUILD_TARGETS: readonly PluginBuildTarget[] = [
${pluginRows}
];

export const EXTENSION_TARGETS: readonly PluginBuildTarget[] = [
${extensionRows}
];

export const PROGRAM_TARGETS = PLUGIN_BUILD_TARGETS.map((target) => ({
\tpluginId: target.pluginId,
\tmoduleUrl: \`/plugin-modules/\${target.pluginId}/\${target.wasmOut.replace(/\\.wasm$/, ".js")}\`,
}));

export const pluginModuleUrl = (pluginId: string, fileName: string) =>
\t\`/plugin-modules/\${pluginId}/\${fileName.replace(/\\.wasm$/, ".js")}\`;

export const extensionModuleUrl = (extensionId: string, fileName: string) =>
\t\`/extensions/\${extensionId}/\${fileName.replace(/\\.wasm$/, ".js")}\`;
`;
}

function emitAssetSpecTypeScript(asset: AssetSpecRow): string {
  const fields = [`kind: ${JSON.stringify(asset.kind)}`, `route: ${JSON.stringify(asset.route)}`];
  if (asset.upstream !== undefined) fields.push(`upstream: ${JSON.stringify(asset.upstream)}`);
  if (asset.cache !== undefined) fields.push(`cache: ${JSON.stringify(asset.cache)}`);
  if (asset.root !== undefined) fields.push(`root: ${JSON.stringify(asset.root)}`);
  if (asset.roots !== undefined) fields.push(`roots: ${JSON.stringify(asset.roots)}`);
  if (asset.placeholder !== undefined) fields.push(`placeholder: ${JSON.stringify(asset.placeholder)}`);
  if (asset.filterFromExamples) fields.push(`filterFromExamples: true`);
  return `{ ${fields.join(", ")} }`;
}

function emitPlaygroundsTypeScript(playgrounds: PlaygroundEntry[], defaultHostVariant: string): string {
  const rows = playgrounds
    .map((entry) => {
      const app = entry.app !== undefined ? `, app: ${JSON.stringify(entry.app)}` : "";
      const brand = entry.brand !== undefined ? `, brand: ${JSON.stringify(entry.brand)}` : "";
      const userPorts = entry.userPorts !== undefined ? `, userPorts: { react: ${JSON.stringify(entry.userPorts.react)}, wgpu: ${JSON.stringify(entry.userPorts.wgpu)} }` : "";
      const assets = entry.assets.map(emitAssetSpecTypeScript).join(", ");
      return `\t{ variant: ${JSON.stringify(entry.variant)}, pluginId: ${JSON.stringify(entry.pluginId)}, cratePath: ${JSON.stringify(entry.cratePath)}${app}${brand}, aliases: ${JSON.stringify(entry.aliases)}, ports: { react: ${entry.ports.react}, wgpu: ${entry.ports.wgpu} }${userPorts}, examples: ${JSON.stringify(entry.examples)}, engines: ${JSON.stringify(entry.engines)}, assets: [${assets}] },`;
    })
    .join("\n");
  return `/** @generated by framework/plugin/registry/script.ts — do not edit. */
export type PlaygroundAssetSpec =
\t| { readonly kind: "tile-proxy"; readonly route: string; readonly upstream: string; readonly cache: string }
\t| { readonly kind: "static-dir"; readonly route: string; readonly root: string }
\t| { readonly kind: "mesh-collection"; readonly route: string; readonly roots: readonly string[]; readonly placeholder: string; readonly filterFromExamples?: boolean };

export type PlaygroundBuildTarget = {
\treadonly variant: string;
\treadonly pluginId: string;
\treadonly cratePath: string;
\treadonly app?: string;
\treadonly brand?: string;
\treadonly aliases: readonly string[];
\treadonly ports: { readonly react: number; readonly wgpu: number };
\treadonly userPorts?: { readonly react: readonly number[]; readonly wgpu: readonly number[] };
\treadonly examples: readonly string[];
\treadonly engines: readonly string[];
\treadonly assets: readonly PlaygroundAssetSpec[];
};

export const PLAYGROUND_BUILD_TARGETS: readonly PlaygroundBuildTarget[] = [
${rows}
];

/** @emoji 🏠️ The playground variant that boots as the host/shell session — see
 * \`resolveDefaultHostVariant\`. Replaces every hardcoded \`"s"\` default-variant literal downstream. */
export const DEFAULT_HOST_VARIANT = ${JSON.stringify(defaultHostVariant)};
`;
}

//#region 🏛️FrameworkPackageCatalog
/** @emoji 🏛️ One framework package as seen by the shared discovery contract (`role = "framework"`).
 * The framework families are not wasm components, so they never enter `PLUGIN_BUILD_TARGETS`; this is
 * their own catalog section — the consumable answer to "which framework packages exist, in which
 * language/render target, and how far has their owner migrated" that every downstream mechanism
 * (workspaces generator, storybook scopes, dep-cruiser) previously had to rediscover by hand. */
export type FrameworkPackageEntry = {
  readonly id: string;
  readonly ownerPath: string;
  readonly packagePath: string;
  readonly lang: string;
  readonly target?: string;
  readonly area: string;
  readonly maturity: string;
};

/** @emoji 🏛️ Framework-role half of the shared `discoverPackages()` walk (three-level `🎯️targets`
 * aware), flattened into a stable catalog. Plugin and framework catalogs therefore come from one
 * traversal and one vocabulary, and can never drift apart. */
export function generateFrameworkPackageRegistry(repoRoot = getWorkspaceRoot(), packages: readonly DiscoveredPackage[] = discoverCatalogPackages(repoRoot, TAXONOMY)): FrameworkPackageEntry[] {
  return packages
    .filter((pkg) => pkg.role === "framework")
    .map((pkg) => ({
      id: pkg.id,
      ownerPath: pkg.ownerRel,
      packagePath: pkg.packageRel,
      lang: pkg.lang,
      ...(pkg.target ? { target: pkg.target } : {}),
      area: pkg.area,
      maturity: pkg.maturity,
    }))
    .sort((a, b) => a.id.localeCompare(b.id) || a.packagePath.localeCompare(b.packagePath));
}

function emitFrameworkPackagesTypeScript(entries: FrameworkPackageEntry[]): string {
  const rows = entries
    .map((entry) => {
      const target = entry.target !== undefined ? `, target: ${JSON.stringify(entry.target)}` : "";
      return `\t{ id: ${JSON.stringify(entry.id)}, ownerPath: ${JSON.stringify(entry.ownerPath)}, packagePath: ${JSON.stringify(entry.packagePath)}, lang: ${JSON.stringify(entry.lang)}${target}, area: ${JSON.stringify(entry.area)}, maturity: ${JSON.stringify(entry.maturity)} },`;
    })
    .join("\n");
  return `/** @generated by framework/plugin/registry/script.ts — do not edit. */
export type FrameworkPackage = {
\treadonly id: string;
\treadonly ownerPath: string;
\treadonly packagePath: string;
\treadonly lang: string;
\treadonly target?: string;
\treadonly area: string;
\treadonly maturity: string;
};

export const FRAMEWORK_PACKAGES: readonly FrameworkPackage[] = [
${rows}
];
`;
}
//#endregion 🏛️FrameworkPackageCatalog

//#region 🎮️PlaygroundSession
export type PlaygroundSessionPlugin = {
  readonly pluginId: string;
  readonly moduleUrl: string;
  readonly contributes: readonly string[];
  readonly consumes: readonly string[];
};

export type PlaygroundSession = {
  readonly variant: string;
  readonly registryPluginId: string;
  readonly defaultAppId?: string;
  readonly hostMode: boolean;
  readonly host?: PluginHostMetadata;
  readonly plugins: readonly PlaygroundSessionPlugin[];
};

/** @emoji 🎮️ Builds the pre-expanded plugin list and host metadata for one playground launch. */
export function buildPlaygroundSession(variant: string, repoRoot = getWorkspaceRoot()): PlaygroundSession {
  const hostMode = isHostPluginFilter(variant, repoRoot);
  const registryPluginId = resolveRegistryPluginIdForFilter(variant, repoRoot);
  const playgrounds = generatePlaygroundRegistry(repoRoot);
  const playground = playgrounds.find((entry) => entry.variant === variant || entry.aliases.includes(variant));
  const entries = generatePluginRegistry(repoRoot, hostMode ? {} : { filterPlaygroundPlugin: registryPluginId });
  const host = entries.find((entry) => entry.pluginId === registryPluginId)?.host;
  return {
    variant,
    registryPluginId,
    defaultAppId: playground?.app,
    hostMode,
    ...(host ? { host } : {}),
    plugins: entries.map((entry) => ({
      pluginId: entry.pluginId,
      moduleUrl: `/plugin-modules/${entry.pluginId}/${entry.wasmOut.replace(/\.wasm$/, ".js")}`,
      contributes: entry.contributes,
      consumes: entry.consumes,
    })),
  };
}

function emitSessionTypeScript(session: PlaygroundSession): string {
  const host = session.host ? `{ landingAppId: ${JSON.stringify(session.host.landingAppId)}, hostAppId: ${JSON.stringify(session.host.hostAppId)} }` : "undefined";
  const defaultAppId = session.defaultAppId !== undefined ? JSON.stringify(session.defaultAppId) : "undefined";
  const pluginRows = session.plugins
    .map((entry) => `\t{ pluginId: ${JSON.stringify(entry.pluginId)}, moduleUrl: ${JSON.stringify(entry.moduleUrl)}, contributes: ${JSON.stringify(entry.contributes)}, consumes: ${JSON.stringify(entry.consumes)} },`)
    .join("\n");
  return `/** @generated by framework/plugin/registry/script.ts — do not edit. */
export type PlaygroundSessionPlugin = {
\treadonly pluginId: string;
\treadonly moduleUrl: string;
\treadonly contributes: readonly string[];
\treadonly consumes: readonly string[];
};

export type PlaygroundSession = {
\treadonly variant: string;
\treadonly registryPluginId: string;
\treadonly defaultAppId?: string;
\treadonly hostMode: boolean;
\treadonly host?: { readonly landingAppId: string; readonly hostAppId: string };
\treadonly plugins: readonly PlaygroundSessionPlugin[];
};

export const PLAYGROUND_SESSION: PlaygroundSession = {
\tvariant: ${JSON.stringify(session.variant)},
\tregistryPluginId: ${JSON.stringify(session.registryPluginId)},
\tdefaultAppId: ${defaultAppId},
\thostMode: ${session.hostMode},
\thost: ${host},
\tplugins: [
${pluginRows}
\t],
};
`;
}

function emitRustHosts(entries: PluginRegistryEntry[], playgrounds: PlaygroundEntry[]): string {
  const hostRows = entries
    .filter((entry) => entry.host)
    .map((entry) => `    PluginHostConfig { plugin_id: ${JSON.stringify(entry.pluginId)}, landing_app_id: ${JSON.stringify(entry.host!.landingAppId)}, host_app_id: ${JSON.stringify(entry.host!.hostAppId)} },`)
    .join("\n");
  const variantRows = playgrounds.map((entry) => `    (${JSON.stringify(entry.variant)}, ${JSON.stringify(entry.pluginId)}),`).join("\n");
  const aliasRows = playgrounds.flatMap((entry) => entry.aliases.map((alias) => `    (${JSON.stringify(alias)}, ${JSON.stringify(entry.pluginId)}),`)).join("\n");
  const variantAppRows = playgrounds.filter((entry) => entry.app).map((entry) => `    (${JSON.stringify(entry.variant)}, ${JSON.stringify(entry.app)}),`).join("\n");
  const aliasAppRows = playgrounds.flatMap((entry) => entry.app ? entry.aliases.map((alias) => `    (${JSON.stringify(alias)}, ${JSON.stringify(entry.app)}),`) : []).join("\n");
  return `// @generated by framework/plugin/registry/script.ts — do not edit.

pub struct PluginHostConfig {
    pub plugin_id: &'static str,
    pub landing_app_id: &'static str,
    pub host_app_id: &'static str,
}

pub const PLUGIN_HOST_CONFIGS: &[PluginHostConfig] = &[
${hostRows}
];

const PLAYGROUND_VARIANT_REGISTRY_IDS: &[(&str, &str)] = &[
${variantRows}
${aliasRows}
];

const PLAYGROUND_VARIANT_APP_IDS: &[(&str, &str)] = &[
${variantAppRows}
${aliasAppRows}
];

pub fn resolve_registry_plugin_id(plugin_filter: &str) -> &str {
    for (variant, plugin_id) in PLAYGROUND_VARIANT_REGISTRY_IDS {
        if *variant == plugin_filter {
            return plugin_id;
        }
    }
    plugin_filter
}

pub fn resolve_playground_app_id(plugin_filter: &str) -> Option<&'static str> {
    PLAYGROUND_VARIANT_APP_IDS.iter().find_map(|(variant, app_id)| (*variant == plugin_filter).then_some(*app_id))
}

pub fn resolve_plugin_host_config(plugin_filter: &str) -> Option<&'static PluginHostConfig> {
    let registry_id = resolve_registry_plugin_id(plugin_filter);
    PLUGIN_HOST_CONFIGS.iter().find(|entry| entry.plugin_id == registry_id)
}

pub fn is_space_mode(plugin_filter: &str) -> bool {
    resolve_plugin_host_config(plugin_filter).is_some()
}
`;
}

/** @emoji 🗂️ Emits plugin wasm artifact constants for headless `semio-framework-os-run`.
 * Paths are profile-relative; `resolve_plugin_paths` tries `debug` then `wasm-release`. */
function emitRustArtifacts(entries: PluginRegistryEntry[]): string {
  const rows = entries.map((entry) => `    (${JSON.stringify(entry.pluginId)}, ${JSON.stringify(entry.wasmOut)}),`).join("\n");
  return `// @generated by framework/plugin/registry/script.ts — do not edit.

pub const PLUGIN_WASM_TARGET_DIR: &str = "target/wasm32-wasip2";
pub const PLUGIN_WASM_PROFILE_DIRS: &[&str] = &["debug", "wasm-release"];
pub const PLUGIN_WASM_ARTIFACTS: &[(&str, &str)] = &[
${rows}
];
`;
}

/** @emoji 💾️ Writes the per-launch playground session artifact consumed by os/dev and wgpu boot. */
export function writePlaygroundSession(variant: string, outPath: string, repoRoot = getWorkspaceRoot()): PlaygroundSession {
  const session = buildPlaygroundSession(variant, repoRoot);
  mkdirSync(dirname(outPath), { recursive: true });
  writeFileSync(outPath, emitSessionTypeScript(session));
  return session;
}
//#endregion 🎮️PlaygroundSession

/** @emoji 🚦️ Cross-checks the flattened playground catalog for global uniqueness, multi-app crate discipline, and resolvable file-backed asset declarations; returns human-readable violations. */
function validatePlaygroundRegistry(playgrounds: PlaygroundEntry[], repoRoot: string): string[] {
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
        if (!asset.roots?.length) errors.push(`playground variant "${entry.variant}" declares mesh-collection route "${asset.route}" without roots`);
        for (const root of asset.roots ?? []) {
          const path = join(repoRoot, root);
          if (!existsSync(path) || !statSync(path).isDirectory()) errors.push(`playground variant "${entry.variant}" declares missing mesh-collection root "${root}"`);
        }
        const placeholder = asset.placeholder ? join(repoRoot, asset.placeholder) : undefined;
        if (!placeholder || !existsSync(placeholder) || !statSync(placeholder).isFile()) errors.push(`playground variant "${entry.variant}" declares missing mesh-collection placeholder "${asset.placeholder ?? ""}"`);
      }
    }
    entriesByCrate.set(entry.cratePath, [...(entriesByCrate.get(entry.cratePath) ?? []), entry]);
  }
  for (const group of entriesByCrate.values()) {
    if (group.length <= 1) continue;
    for (const entry of group) {
      if (!entry.app) errors.push(`playground variant "${entry.variant}" in ${entry.cratePath} must set "app" (crate declares ${group.length} playground entries)`);
    }
  }
  return errors;
}

//#region 🗿️TaxonomyValidator
/** @emoji 🗿️ Every artifact node must carry the completeness taxonomy component slots (incl. `🧬️mutations` + `⚙️engine`) — sourced from
 * `🔣️taxonomy.json` (single vocabulary source of truth, see master ticket
 * `26/08/05/CRATE-CONSOLIDATION-AND-PLUGIN-TAXONOMY-RESTRUCTURE`; this used to be an independently
 * hand-maintained copy, which is exactly the drift `🔣️taxonomy.json` exists to prevent). */
const TAXONOMY_ARTIFACT_COMPONENTS = TAXONOMY.artifactComponentDirs;
const TAXONOMY_MUTATION_COMPONENT_FILENAME = primaryFilenameForKind(TAXONOMY.mutationComponentFileKindId);
const TAXONOMY_MUTATION_DESCRIPTOR_FILENAME = primaryFilenameForKind(TAXONOMY.mutationDescriptorFileKindId);
const TAXONOMY_MUTATION_FACET_DIRS = [...TAXONOMY.mutationBehaviorFacetDirs, ...TAXONOMY.mutationOrganizationalFacetDirs];
const TAXONOMY_SCHEMA_CHILD_DIRS = TAXONOMY.schemaChildDirs ?? [];
const TAXONOMY_REPRESENTATION_DIRS = TAXONOMY.representationDirs ?? [];
const TAXONOMY_CONFIG_CHILD_DIRS = TAXONOMY.configChildDirs ?? [];
const TAXONOMY_PRESENCE_CHILD_DIRS = TAXONOMY.presenceChildDirs ?? [];
/** @emoji 🎭️ A mode owns its windows plus its own three state lanes — the completeness set the
 * taxonomy declares for every `🎭️modes/<mode>/` node. */
const TAXONOMY_MODE_CHILDREN = TAXONOMY.modeChildDirs ?? [];
const TAXONOMY_SCHEMA_FILENAMES = Object.values(TAXONOMY.schemaFormats).map((format) => primaryFilenameForKind(format.fileKindId));
const MUTATIONS_FACET_DIR = "🧬️mutations";
const ENGINE_FACET_DIR = "⚙️engine";
const SNAPSHOT_FACET_DIR = "📸️snapshot";
const DIFF_FACET_DIR = "🔺️diff";
const SCHEMA_FACET_DIR = "🧬️schema";
const CONFIG_FACET_DIR = "🎚️config";
const PRESENCE_FACET_DIR = "👥️presence";
const IO_FACET_DIR = "🚪️io";
const LEGACY_CONFIG_FACET_DIR = "🧮️config";
const TAXONOMY_IO_DIRECTION_DIRS = TAXONOMY.ioDirectionDirs ?? [];
const TAXONOMY_IO_DIRECTION_CHILD_DIRS = TAXONOMY.ioDirectionChildDirs ?? {};
const BUILDER_FACET_DIR = "🏗️builder";
const DECOMPOSER_FACET_DIR = "🪓️decomposer";
const LEGACY_WASM_DIR = "🕸️wasm";
const TAXONOMY_TS_LEAF_FILENAME = primaryFilenameForKind(TAXONOMY.ecosystems["🟦️typescript"].componentFileKindId);
/** @emoji 🪟️ A window dir may only contain these children, each itself a `🦀️.rs` leaf. */
const TAXONOMY_WINDOW_CHILDREN = new Set(TAXONOMY.windowChildDirs);
const TAXONOMY_LEAF_FILENAME = primaryFilenameForKind(TAXONOMY.ecosystems[RUST_LANG].componentFileKindId);
/** @emoji 🚪️ Rust entry filename and its Shape V2 home relative to the owner root. */
const RUST_LIBRARY_ENTRY_CONTRACT_ID = TAXONOMY.ecosystems[RUST_LANG].entryContractIds.find((contractId) => TAXONOMY.configurableEntryContracts[contractId]?.role === "library");
if (!RUST_LIBRARY_ENTRY_CONTRACT_ID) throw new Error("📇️registry: Rust ecosystem must declare a library entry contract");
const RUST_ENTRY_FILENAME = TAXONOMY.configurableEntryContracts[RUST_LIBRARY_ENTRY_CONTRACT_ID].filename;
const RUST_ENTRY_DIR_FROM_OWNER = TAXONOMY.rustEntryPathRules.entryDirFromOwner.split("/");
const WINDOW_EMPTY_FACET_FILENAME = primaryFilenameForKind(TAXONOMY.windowEmptyFacetFileKindId);

/** @emoji 🧭️ Plugin roots discovered via the shared package contract (`role = "plugin"`, rust, owner
 * sitting directly under one of `taxonomy.pluginAreas`). Drives the taxonomy tree audit below. */
function findNewContractPluginRoots(repoRoot: string): { pluginId: string; pluginRoot: string }[] {
  return discoverPackages(repoRoot, TAXONOMY)
    .filter((pkg) => pkg.role === "plugin" && pkg.lang === RUST_LANG && PLUGIN_AREAS.includes(dirname(pkg.ownerRel)))
    .map((pkg) => ({ pluginId: basename(pkg.ownerRel), pluginRoot: join(repoRoot, pkg.ownerRel) }))
    .sort((a, b) => a.pluginId.localeCompare(b.pluginId));
}

function listDirs(dir: string): string[] {
  if (!existsSync(dir)) return [];
  return readdirSync(dir).filter((name) => statSync(join(dir, name)).isDirectory());
}

/** @emoji 👁️✏️ Every `👁️viewer`/`✏️editor` surface dir under one plugin's
 * `🗿️artifacts/<a>/🏅️standards/<s>/🪆️subsets/<sub>/` tree — the W3 dissolution replacement for the old
 * `🎛️apps/<app>/` root. Paired with a `"<subset>/<roleDirName>"` label for findings. */
function surfaceDirsForPlugin(pluginRoot: string): { abs: string; label: string }[] {
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

/** @emoji 🧱️ True when an inline Rust module's scope continues beyond its declaration line. */
function moduleScopeContinues(line: string): boolean {
  return (line.match(/\{/g) ?? []).length > (line.match(/\}/g) ?? []).length;
}

/** @emoji 🚦️ Structural audit of one migrated plugin's taxonomy tree, entirely against
 * `🔣️taxonomy.json`'s vocabulary. Severity is decided by the caller from the plugin area's declared
 * maturity: warn while it is `legacy`/`mixed`, hard failure once it is `clean`. */
function validateTaxonomyTree(pluginRoot: string, pluginId: string): string[] {
  const findings: string[] = [];

  const artifactsDir = join(pluginRoot, TAXONOMY.artifactsDirName);
  for (const artifact of listDirs(artifactsDir)) {
    for (const component of TAXONOMY_ARTIFACT_COMPONENTS) {
      const facetDir = join(artifactsDir, artifact, component);
      // Soft-require builder/decomposer until W5/W6 migrate every artifact (vocabulary is already strict).
      if (component === BUILDER_FACET_DIR || component === DECOMPOSER_FACET_DIR) {
        if (existsSync(facetDir)) {
          if (!existsSync(join(facetDir, TAXONOMY_LEAF_FILENAME))) {
            findings.push(`${pluginId}: artifact "${artifact}" is missing ${component}/${TAXONOMY_LEAF_FILENAME}`);
          }
          if (!existsSync(join(facetDir, TAXONOMY_TS_LEAF_FILENAME))) {
            findings.push(`${pluginId}: artifact "${artifact}" is missing ${component}/${TAXONOMY_TS_LEAF_FILENAME}`);
          }
        }
        continue;
      }
      if (component === IO_FACET_DIR) {
        if (!existsSync(facetDir)) {
          findings.push(`${pluginId}: artifact "${artifact}" is missing ${component}/`);
        } else if (!existsSync(join(facetDir, TAXONOMY_LEAF_FILENAME))) {
          findings.push(`${pluginId}: artifact "${artifact}" is missing ${component}/${TAXONOMY_LEAF_FILENAME}`);
        }
        continue;
      }
      if (component === SCHEMA_FACET_DIR) {
        if (!existsSync(facetDir)) {
          findings.push(`${pluginId}: artifact "${artifact}" is missing ${component}/`);
        }
        continue;
      }
      if (!existsSync(join(facetDir, TAXONOMY_LEAF_FILENAME))) {
        findings.push(`${pluginId}: artifact "${artifact}" is missing ${component}/${TAXONOMY_LEAF_FILENAME}`);
      }
      if (!existsSync(join(facetDir, TAXONOMY_TS_LEAF_FILENAME))) {
        findings.push(`${pluginId}: artifact "${artifact}" is missing ${component}/${TAXONOMY_TS_LEAF_FILENAME}`);
      }
    }
    //#region NestedFacetWalk
    // Presence-tolerant schema tree + io direction shape (full leaf matrix gated by stdio policies / W5–W6).
    const schemaDir = join(artifactsDir, artifact, SCHEMA_FACET_DIR);
    if (existsSync(schemaDir)) {
      for (const filename of TAXONOMY_SCHEMA_FILENAMES) {
        if (!existsSync(join(schemaDir, filename))) {
          findings.push(`${pluginId}: artifact "${artifact}" is missing ${SCHEMA_FACET_DIR}/${filename}`);
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
                findings.push(`${pluginId}: artifact "${artifact}" mutation "${rep}" is missing ${SCHEMA_FACET_DIR}/${MUTATIONS_FACET_DIR}/${rep}/${TAXONOMY_MUTATION_COMPONENT_FILENAME}`);
              }
              if (!existsSync(join(mutationDir, TAXONOMY_MUTATION_DESCRIPTOR_FILENAME))) {
                findings.push(`${pluginId}: artifact "${artifact}" mutation "${rep}" is missing ${SCHEMA_FACET_DIR}/${MUTATIONS_FACET_DIR}/${rep}/${TAXONOMY_MUTATION_DESCRIPTOR_FILENAME}`);
              }
              for (const facet of TAXONOMY_MUTATION_FACET_DIRS) {
                const facetDir = join(mutationDir, facet);
                if (existsSync(facetDir) && !existsSync(join(facetDir, TAXONOMY_LEAF_FILENAME))) {
                  findings.push(`${pluginId}: artifact "${artifact}" mutation "${rep}" optional facet "${facet}" is missing ${SCHEMA_FACET_DIR}/${MUTATIONS_FACET_DIR}/${rep}/${facet}/${TAXONOMY_LEAF_FILENAME}`);
                }
              }
              continue;
            }
            findings.push(`${pluginId}: artifact "${artifact}" has undeclared ${SCHEMA_FACET_DIR}/${child}/${rep}`);
          }
          continue;
        }
        if (child === TAXONOMY.packagesDirName) continue;
        // allow schema format leaves at schema root; dirs must be schemaChildDirs
        findings.push(`${pluginId}: artifact "${artifact}" has undeclared ${SCHEMA_FACET_DIR}/${child}`);
      }
    }
    const ioFacetDir = join(artifactsDir, artifact, IO_FACET_DIR);
    if (existsSync(ioFacetDir)) {
      for (const direction of listDirs(ioFacetDir)) {
        if (!TAXONOMY_IO_DIRECTION_DIRS.includes(direction)) {
          findings.push(`${pluginId}: artifact "${artifact}" has undeclared ${IO_FACET_DIR}/${direction}`);
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
                  findings.push(`${pluginId}: artifact "${artifact}" is missing ${IO_FACET_DIR}/${direction}/${codec}/${TAXONOMY.artifactsDirName}/${stdioArt}/${TAXONOMY_LEAF_FILENAME}`);
                }
              }
            }
            continue;
          }
          findings.push(`${pluginId}: artifact "${artifact}" has undeclared ${IO_FACET_DIR}/${direction}/${codec}`);
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
          findings.push(`${pluginId}: artifact "${artifact}" mutation "${mutation}" is missing ${MUTATIONS_FACET_DIR}/${mutation}/${TAXONOMY_MUTATION_COMPONENT_FILENAME}`);
        }
        if (!existsSync(join(mutationDir, TAXONOMY_MUTATION_DESCRIPTOR_FILENAME))) {
          findings.push(`${pluginId}: artifact "${artifact}" mutation "${mutation}" is missing ${MUTATIONS_FACET_DIR}/${mutation}/${TAXONOMY_MUTATION_DESCRIPTOR_FILENAME}`);
        }
        for (const facet of TAXONOMY_MUTATION_FACET_DIRS) {
          const facetDir = join(mutationDir, facet);
          if (existsSync(facetDir) && !existsSync(join(facetDir, TAXONOMY_LEAF_FILENAME))) {
            findings.push(`${pluginId}: artifact "${artifact}" mutation "${mutation}" optional facet "${facet}" is missing ${MUTATIONS_FACET_DIR}/${mutation}/${facet}/${TAXONOMY_LEAF_FILENAME}`);
          }
        }
      }
    }
    // ⚙️engine presence is enforced via TAXONOMY_ARTIFACT_COMPONENTS (completeness); keep an explicit finding if the facet dir itself is absent.
    if (!existsSync(join(artifactsDir, artifact, ENGINE_FACET_DIR))) {
      findings.push(`${pluginId}: artifact "${artifact}" is missing ${ENGINE_FACET_DIR}/`);
    }
    //#endregion DirectMutations
    const examplesRoot = join(artifactsDir, artifact, EXAMPLES_DIRNAME);
    if (!existsSync(examplesRoot)) {
      findings.push(`${pluginId}: artifact "${artifact}" is missing ${EXAMPLES_DIRNAME}/`);
      continue;
    }
    const exampleSets = listDirs(examplesRoot);
    if (exampleSets.length === 0) {
      findings.push(`${pluginId}: artifact "${artifact}" ${EXAMPLES_DIRNAME} has no example slug`);
      continue;
    }
    for (const exampleSet of exampleSets) {
      if (!isExampleSlugName(exampleSet)) {
        findings.push(`${pluginId}: artifact "${artifact}" example "${exampleSet}" is not a valid emoji+VS16+kebab slug`);
      }
      for (const plural of FORBIDDEN_EXAMPLE_PLURAL_DIRS) {
        if (existsSync(join(examplesRoot, exampleSet, plural))) {
          findings.push(`${pluginId}: artifact "${artifact}" example "${exampleSet}" still has plural ${plural}/`);
        }
      }
      if (!existsSync(join(examplesRoot, exampleSet, EXAMPLE_RUST_LEAF))) {
        findings.push(`${pluginId}: artifact "${artifact}" example "${exampleSet}" is missing ${EXAMPLE_RUST_LEAF}`);
      }
      if (!existsSync(join(examplesRoot, exampleSet, EXAMPLE_TS_LEAF))) {
        findings.push(`${pluginId}: artifact "${artifact}" example "${exampleSet}" is missing ${EXAMPLE_TS_LEAF}`);
      }
      if (!existsSync(join(examplesRoot, exampleSet, EXAMPLE_ASSETS_DIRNAME))) {
        findings.push(`${pluginId}: artifact "${artifact}" example "${exampleSet}" is missing ${EXAMPLE_ASSETS_DIRNAME}/`);
      }
      if (!existsSync(join(examplesRoot, exampleSet, EXAMPLE_TESTS_DIRNAME))) {
        findings.push(`${pluginId}: artifact "${artifact}" example "${exampleSet}" is missing ${EXAMPLE_TESTS_DIRNAME}/`);
      }
    }
  }

  if (existsSync(join(pluginRoot, EXAMPLES_DIRNAME))) {
    findings.push(`${pluginId}: plugin-root ${EXAMPLES_DIRNAME}/ is forbidden — relocate under 🗿️artifacts/<artifact>/${EXAMPLES_DIRNAME}`);
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
    const surfaceSets = listDirs(surfaceExamples);
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

  // 🪟️ windows live under <surface>/modes/<mode>/windows/<w> and may only contain the fixed child set.
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
      if (name.startsWith(".") || name === "target" || name === "node_modules") continue;
      const path = join(dir, name);
      if (statSync(path).isDirectory()) {
        walkPluginTree(path);
        continue;
      }
      if (!name.endsWith(".rs")) continue;
      if (name === TAXONOMY_LEAF_FILENAME || name === EXAMPLE_RUST_LEAF) {
        componentFiles.push(path);
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

  // 📦️ glue.rs mod/#[path] cross-check: every component.rs on disk must be declared, and no declared
  // #[path] target may dangle (point at a file that doesn't exist) — reported as separate findings.
  // 🌳️ The configurable Rust library entry lives beneath the taxonomy-declared package entry directory.
  //
  // 🧮️ #[path] resolution is CUMULATIVE, not always-relative-to-the-raw-file: each nested `pub mod X`
  // (or leaf `mod X;`) resolves its own `#[path]` string relative to its immediately enclosing mod's
  // *already-resolved* directory (defaulting to `<enclosing dir>/X` when no `#[path]` is given at all,
  // and to "no change" when the string is exactly `"."`) — confirmed empirically against a real,
  // compiling plugin (🖨️raster) that resets the base ONCE via `#[path = "../../."]` on an outer
  // grouping module and lets every nested `#[path = "."]` inherit it, as well as plugins (🏛️architect,
  // 📸️remodel, 🖍️draw) that instead prefix every LEAF path with `../../` and leave every nested `"."`
  // unprefixed — both are valid, and a flat "resolve every #[path] against the raw file directory"
  // approach mis-resolves the first style. So: walk the file's brace structure with a resolved-base
  // stack, seeded with the file's own directory.
  const v1LibRsPath = join(pluginRoot, RUST_ENTRY_FILENAME);
  const v2LibRsPath = join(pluginRoot, ...RUST_ENTRY_DIR_FROM_OWNER, RUST_ENTRY_FILENAME);
  const libRsPath = existsSync(v2LibRsPath) ? v2LibRsPath : v1LibRsPath;
  if (existsSync(libRsPath)) {
    const libDir = dirname(libRsPath);
    const libText = readFileSync(libRsPath, "utf8");
    const declaredAbs = new Set<string>();
    const danglingLeafPaths: string[] = [];

    // 🥞️ One stack frame per open `{` that followed a `mod`/`pub mod` declaration, holding that
    // scope's resolved base dir. A pending `#[path = "…"]` applies to the NEXT `mod` line only.
    const baseStack: string[] = [libDir];
    let pendingPath: string | null = null;
    const lines = libText.split("\n");
    for (const rawLine of lines) {
      const line = rawLine.trim();
      const pathMatch = line.match(/#\[path\s*=\s*"([^"]+)"\]/);
      if (pathMatch) {
        pendingPath = pathMatch[1];
        continue;
      }
      const modMatch = line.match(/^(?:pub\s+)?mod\s+(\w+)\s*(\{|;)/);
      if (modMatch) {
        const parentBase = baseStack[baseStack.length - 1];
        const rawTarget = pendingPath ?? modMatch[1]; // no #[path] ⇒ default splice of the mod's own name
        const resolved = join(parentBase, rawTarget); // node:path's join already normalizes "." / ".." segments
        pendingPath = null;
        if (modMatch[2] === ";") {
          // Leaf: either a real component file (ends .rs) or a `mod tests;`-style non-path leaf — only
          // cross-check paths that look like a file (the taxonomy only ever points #[path] at .rs files).
          if (pendingPathLooksLikeFile(rawTarget)) {
            declaredAbs.add(resolved);
            if (!existsSync(resolved)) danglingLeafPaths.push(rawTarget);
          }
        } else if (moduleScopeContinues(line)) {
          baseStack.push(resolved);
        }
        continue;
      }
      // Count bare closing braces against open mod scopes (lib.rs is wiring-only, so every `{`/`}` in
      // the file belongs to a mod block or the trailing semio_plugin! macro call — once the stack is
      // back to just the file base, further closes belong to the macro call and are ignored).
      const closes = (line.match(/\}/g) ?? []).length;
      const opens = (line.match(/\{/g) ?? []).length;
      for (let i = 0; i < closes - opens; i++) {
        if (baseStack.length > 1) baseStack.pop();
      }
    }

    function pendingPathLooksLikeFile(p: string): boolean {
      return p.endsWith(".rs");
    }

    for (const file of componentFiles) {
      if (!declaredAbs.has(file)) findings.push(`${pluginId}: ${relative(pluginRoot, file)} is not declared by any #[path] in ${RUST_ENTRY_FILENAME}`);
    }
    for (const p of danglingLeafPaths) {
      findings.push(`${pluginId}: ${RUST_ENTRY_FILENAME} declares #[path = "${p}"] but the file does not exist on disk`);
    }
  } else {
    findings.push(`${pluginId}: missing ${RUST_ENTRY_FILENAME} (checked plugin root and ${TAXONOMY.rustEntryPathRules.entryDirFromOwner}/)`);
  }

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

  const pluginChildDirs = TAXONOMY.pluginChildDirs;
  const nestedPluginContract = join(pluginRoot, "🔌️plugin");
  if (existsSync(nestedPluginContract)) {
    findings.push(`${pluginId}: move the redundant 🔌️plugin contract and facets directly into the plugin root, then remove 🔌️plugin/`);
  }
  if (!existsSync(join(pluginRoot, TAXONOMY_LEAF_FILENAME))) {
    findings.push(`${pluginId}: plugin root is missing ${TAXONOMY_LEAF_FILENAME}`);
  }
  for (const child of pluginChildDirs) {
    if (!existsSync(join(pluginRoot, child, TAXONOMY_LEAF_FILENAME))) {
      findings.push(`${pluginId}: plugin root is missing ${child}/${TAXONOMY_LEAF_FILENAME}`);
    }
  }

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
        for (const filename of TAXONOMY_SCHEMA_FILENAMES) {
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
        for (const filename of TAXONOMY_SCHEMA_FILENAMES) {
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
//#endregion 🗿️TaxonomyValidator

//#region 🔖️SurfaceScaffolder
/**
 * 🏗️ `new surface` — the permanent, taxonomy-derived scaffolder for the 286-surface tree (143 subsets
 * × `{viewer, editor}`), ticket `26/08/16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET`. Every path segment
 * below comes from `🔣️taxonomy.json` except the default mode/window ids (`SURFACE_DEFAULT_MODE_DIRNAME`
 * / `SURFACE_DEFAULT_WINDOW_DIRNAME`), which are not taxonomy vocabulary — a *specific* mode/window is
 * per-subset authoring content the owning W2 packet picks; this only supplies the placeholder shape a
 * fresh surface must start from. Every generated component leaf carries the `SCAFFOLD` marker so
 * `policySubsetSurfaceCompletenessBreaches` (root `📜️script.ts`) can flag scaffold residue distinctly
 * from a genuinely missing surface.
 */
const SURFACE_SCAFFOLD_TICKET_PATH = ".🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️16/ARTIFACT-VIEWERS-AND-EDITORS-PER-SUBSET";
/** @emoji 🚧️ Marker every scaffolded component leaf carries; scanned for by the completeness policy. */
const SCAFFOLD_MARKER = "SCAFFOLD";
/** @emoji 🎭️ Default mode dir a freshly scaffolded surface gets — mirrors the pre-existing `🎛️apps`
 * convention (`✏️edit` for editors) onto the read-only side (`👁️view` for viewers). Not taxonomy
 * vocabulary (see the region docstring). */
const SURFACE_DEFAULT_MODE_DIRNAME: Readonly<Record<string, string>> = { viewer: "👁️view", editor: "✏️edit" };
/** @emoji 🪟️ Default window dir a freshly scaffolded mode gets — one obviously-generic placeholder
 * window, replaced by real, per-subset window ids as the owning W2 packet fills the scaffold in. */
const SURFACE_DEFAULT_WINDOW_DIRNAME = "🪟️main";

/** @emoji 🧹️ Drops every non-ASCII codepoint (emoji + variation selectors) — `"📐️cad"` -> `"cad"` — so a
 * bare CLI id (typed without emoji) can match the real on-disk directory name. Mirrors root
 * `📜️script.ts`'s `policyStripEmoji`; duplicated here because the two scripts are separate bundles with
 * no shared import path for this one-line helper. */
function surfaceStripEmoji(segment: string): string {
  return segment.replace(/[^\x00-\x7f]/g, "");
}

/** @emoji 🔎️ Resolves a bare CLI id to the real emoji-prefixed child directory name of `parentAbs`. */
function surfaceResolveChildDir(parentAbs: string, wantStripped: string): string | undefined {
  if (!existsSync(parentAbs)) return undefined;
  for (const name of readdirSync(parentAbs)) {
    if (!statSync(join(parentAbs, name)).isDirectory()) continue;
    if (surfaceStripEmoji(name) === wantStripped) return name;
  }
  return undefined;
}

/** @emoji 🧭️ Resolves `<plugin> <kind> <standard> <subset>` CLI args to the subset's repo-relative path,
 * throwing a precise error naming the failing segment rather than silently creating the wrong tree.
 * `subsetArg === taxonomy.subsetAnyId` (`"*"`) is accepted as an alias for `subsetAnyDirName`
 * (`"✳️any"`), mirroring the taxonomy's own alias. */
function resolveSubsetRel(repoRoot: string, pluginArg: string, kindArg: string, standardArg: string, subsetArg: string): string {
  let area: string | undefined;
  let pluginDir: string | undefined;
  for (const candidate of PLUGIN_AREAS) {
    const found = surfaceResolveChildDir(join(repoRoot, candidate), pluginArg);
    if (found) {
      area = candidate;
      pluginDir = found;
      break;
    }
  }
  if (!area || !pluginDir) throw new Error(`new surface: no plugin "${pluginArg}" under ${PLUGIN_AREAS.join(", ")}`);
  const artifactsAbs = join(repoRoot, area, pluginDir, TAXONOMY.artifactsDirName);
  const kindDir = surfaceResolveChildDir(artifactsAbs, kindArg);
  if (!kindDir) throw new Error(`new surface: no artifact kind "${kindArg}" under ${area}/${pluginDir}/${TAXONOMY.artifactsDirName}`);
  const standardsAbs = join(artifactsAbs, kindDir, TAXONOMY.standardsDirName);
  const standardDir = surfaceResolveChildDir(standardsAbs, standardArg);
  if (!standardDir) throw new Error(`new surface: no standard "${standardArg}" under .../${kindDir}/${TAXONOMY.standardsDirName}`);
  const subsetsAbs = join(standardsAbs, standardDir, TAXONOMY.subsetsDirName);
  const wantSubset = subsetArg === TAXONOMY.subsetAnyId ? surfaceStripEmoji(TAXONOMY.subsetAnyDirName ?? "") : subsetArg;
  const subsetDir = surfaceResolveChildDir(subsetsAbs, wantSubset);
  if (!subsetDir) throw new Error(`new surface: no subset "${subsetArg}" under .../${standardDir}/${TAXONOMY.subsetsDirName}`);
  return relative(repoRoot, join(subsetsAbs, subsetDir)).replaceAll("\\", "/");
}

/** @emoji 🪆️ Every subset dir across every plugin area whose `🧬️schema` facet is present — the "owned"
 * predicate this ticket freezes (contract §6): schema presence alone, independent of `🚪️io`, because
 * the 286-surface target (143 subsets × 2 roles) only holds when every schema-bearing subset counts,
 * including the one subset (🧩️assembly) that has no `🚪️io` yet. */
function discoverOwnedSubsetRels(repoRoot: string): string[] {
  const out: string[] = [];
  for (const area of PLUGIN_AREAS) {
    const areaAbs = join(repoRoot, area);
    for (const plugin of listDirs(areaAbs)) {
      const artifactsAbs = join(areaAbs, plugin, TAXONOMY.artifactsDirName);
      for (const kind of listDirs(artifactsAbs)) {
        const standardsAbs = join(artifactsAbs, kind, TAXONOMY.standardsDirName);
        for (const std of listDirs(standardsAbs)) {
          const subsetsAbs = join(standardsAbs, std, TAXONOMY.subsetsDirName);
          for (const sub of listDirs(subsetsAbs)) {
            const subsetAbs = join(subsetsAbs, sub);
            if (!existsSync(join(subsetAbs, SCHEMA_FACET_DIR))) continue;
            out.push(relative(repoRoot, subsetAbs).replaceAll("\\", "/"));
          }
        }
      }
    }
  }
  return out.sort();
}

function scaffoldRustLeaf(label: string): string {
  return `//! 🚧️ ${SCAFFOLD_MARKER}: ${label} — generated by \`bun ./📜️script.ts new surface\`, not implemented.\n//! @see ${SURFACE_SCAFFOLD_TICKET_PATH}\npub const SCAFFOLD: bool = true;\n`;
}

function scaffoldTsLeaf(label: string): string {
  return `// 🚧️ ${SCAFFOLD_MARKER}: ${label} — generated by \`bun ./📜️script.ts new surface\`, not implemented.\n// @see ${SURFACE_SCAFFOLD_TICKET_PATH}\nexport const SCAFFOLD = true;\n`;
}

function scaffoldEmptyFacetMarkdown(facetLabel: string): string {
  return `# Empty ${surfaceStripEmoji(facetLabel)} Facet\n\nThis facet currently declares no specific items. Authored by \`bun ./📜️script.ts new surface\`.\n`;
}

function scaffoldLeafFilename(lang: string): string {
  const kindId = TAXONOMY.ecosystems[lang]?.componentFileKindId;
  if (!kindId) throw new Error(`new surface: 🔣️taxonomy.json ecosystems has no component file kind for "${lang}"`);
  return primaryFilenameForKind(kindId);
}

function scaffoldLeafContentForLang(lang: string, label: string): string {
  return lang === "🦀️rust" ? scaffoldRustLeaf(label) : scaffoldTsLeaf(label);
}

type SurfaceScaffoldResult = ArtifactScaffoldResult;

/**
 * 🏗️ Creates one surface's full scaffold shape under `<subsetRel>/<viewerDirName|editorDirName>` per
 * the frozen tree (ticket Deliverable A): 2 surface leaves + 4 surface facets + 1 mode leaf + 4 mode
 * facets + 2 window leaves + 6 window facets = 19 files. Idempotent — never overwrites.
 */
export function scaffoldSurfaceTree(repoRoot: string, subsetRel: string, role: string, dryRun: boolean, options: ArtifactScaffoldOptions = {}): SurfaceScaffoldResult {
  const leaves: ArtifactScaffoldLeaf[] = [];
  const add = (path: string, content: string): void => { leaves.push({ path, content }); };
  const surfaceRel = `${subsetRel}/${TAXONOMY.surfaceDirNames[role]}`;
  const label = `${role} surface`;

  for (const lang of TAXONOMY.surfaceComponentLangs) {
    add(`${surfaceRel}/${scaffoldLeafFilename(lang)}`, scaffoldLeafContentForLang(lang, label));
  }
  for (const facet of TAXONOMY.surfaceRequiredChildDirs) {
    if (facet === TAXONOMY.modesDirName) continue;
    add(`${surfaceRel}/${facet}/${WINDOW_EMPTY_FACET_FILENAME}`, scaffoldEmptyFacetMarkdown(`Surface ${facet}`));
  }

  const modeRel = `${surfaceRel}/${TAXONOMY.modesDirName}/${SURFACE_DEFAULT_MODE_DIRNAME[role]}`;
  add(`${modeRel}/${scaffoldLeafFilename("🦀️rust")}`, scaffoldRustLeaf(`${role} mode`));
  for (const facet of TAXONOMY.modeRequiredChildDirs ?? []) {
    if (facet === TAXONOMY.windowsDirName) continue;
    add(`${modeRel}/${facet}/${WINDOW_EMPTY_FACET_FILENAME}`, scaffoldEmptyFacetMarkdown(`Mode ${facet}`));
  }

  const windowRel = `${modeRel}/${TAXONOMY.windowsDirName}/${SURFACE_DEFAULT_WINDOW_DIRNAME}`;
  for (const lang of TAXONOMY.windowLeafLangs) {
    add(`${windowRel}/${scaffoldLeafFilename(lang)}`, scaffoldLeafContentForLang(lang, `${role} window`));
  }
  for (const facet of TAXONOMY.windowRequiredChildDirs) {
    add(`${windowRel}/${facet}/${WINDOW_EMPTY_FACET_FILENAME}`, scaffoldEmptyFacetMarkdown(`Window ${facet}`));
  }

  return authorArtifactScaffold(repoRoot, { kind: "surface", subsetPath: subsetRel, role }, leaves, TAXONOMY, { ...options, dryRun });
}

function reportSurfaceScaffoldResult(label: string, result: SurfaceScaffoldResult, dryRun: boolean): void {
  const verb = dryRun ? "would create" : "created";
  console.log(`${label}: ${verb} ${result.created.length} file(s), ${result.skipped.length} already present`);
  for (const path of result.created) console.log(`  ${dryRun ? "+ (dry-run)" : "+"} ${path}`);
}

/** @emoji 🌊️ `new surface --all`: walks every owned subset on disk and scaffolds whatever surface is
 * missing, idempotently. Reports surface-granularity totals (subset × role pairs touched) alongside
 * the raw file count, so a dry-run answers "how many of the 286 surfaces still need scaffolding". */
function runSurfaceScaffoldAll(repoRoot: string, dryRun: boolean): void {
  const subsetRels = discoverOwnedSubsetRels(repoRoot);
  let surfacesTouched = 0;
  let filesCreated = 0;
  let filesSkipped = 0;
  for (const subsetRel of subsetRels) {
    for (const role of TAXONOMY.surfaceRoles) {
      const result = scaffoldSurfaceTree(repoRoot, subsetRel, role, dryRun);
      filesCreated += result.created.length;
      filesSkipped += result.skipped.length;
      if (result.created.length > 0) surfacesTouched += 1;
    }
  }
  const totalSurfaces = subsetRels.length * TAXONOMY.surfaceRoles.length;
  const verb = dryRun ? "would scaffold" : "scaffolded";
  console.log(`new surface --all: ${verb} ${surfacesTouched}/${totalSurfaces} surface(s) across ${subsetRels.length} owned subset(s) (${filesCreated} file(s) ${dryRun ? "would be created" : "created"}, ${filesSkipped} already present).`);
}

/** @emoji 🚪️ `new surface` CLI: single surface (`<plugin> <kind> <standard> <subset> <role>`) or batch
 * (`--all [--dry-run]`). Registered as `bun ./📜️script.ts new surface …` via `ScriptRouter`. */
class NewScript extends BundleScript {
  run(segments: string[]): void {
    if (segments[0] !== "surface") {
      console.error("usage: bun ./📜️script.ts new surface <plugin> <kind> <standard> <subset> <role>");
      console.error("   or: bun ./📜️script.ts new surface --all [--dry-run]");
      process.exit(1);
    }
    const repoRoot = getWorkspaceRoot();
    const rest = segments.slice(1);
    const dryRun = rest.includes("--dry-run");
    const positional = rest.filter((arg) => arg !== "--dry-run");
    if (positional[0] === "--all") {
      runSurfaceScaffoldAll(repoRoot, dryRun);
      return;
    }
    if (positional.length !== 5) {
      console.error("usage: bun ./📜️script.ts new surface <plugin> <kind> <standard> <subset> <role>");
      process.exit(1);
      return;
    }
    const [pluginArg, kindArg, standardArg, subsetArg, roleArg] = positional;
    if (!TAXONOMY.surfaceRoles.includes(roleArg!)) {
      console.error(`new surface: role must be one of ${TAXONOMY.surfaceRoles.join(", ")}, got "${roleArg}"`);
      process.exit(1);
      return;
    }
    let subsetRel: string;
    try {
      subsetRel = resolveSubsetRel(repoRoot, pluginArg!, kindArg!, standardArg!, subsetArg!);
    } catch (error) {
      console.error((error as Error).message);
      process.exit(1);
      return;
    }
    const result = scaffoldSurfaceTree(repoRoot, subsetRel, roleArg!, dryRun);
    reportSurfaceScaffoldResult(`${subsetRel}#${roleArg}`, result, dryRun);
  }
}
//#endregion 🔖️SurfaceScaffolder

/** @emoji 🧪️ Verifies that representative standalone and studio launches expand to complete sessions,
 * asserting shape rather than hardcoded plugin-id lists/counts so a plugin's crate-name change (or the
 * crate-consolidation restructure itself) can't silently break this check. */
function validatePlaygroundSessions(repoRoot: string): string[] {
  const errors: string[] = [];

  // 🎯️ "standalone variant = the lowest-sorted playground whose plugin does NOT declare
  // [package.metadata.semio].host" — the data-driven replacement for the previous hardcoded
  // `"playbook"` literal, mirroring `resolveDefaultHostVariant`'s host-side resolution below.
  const hostPluginIds = new Set(generatePluginRegistry(repoRoot).filter((entry) => entry.host !== undefined).map((entry) => entry.pluginId));
  const standaloneVariant = generatePlaygroundRegistry(repoRoot)
    .filter((entry) => !hostPluginIds.has(entry.pluginId))
    .map((entry) => entry.variant)
    .sort()[0];
  if (!standaloneVariant) throw new Error(`📇️registry: no playground variant found whose plugin does not declare [package.metadata.semio].host`);
  const standalone = buildPlaygroundSession(standaloneVariant, repoRoot);
  const standalonePluginIds = standalone.plugins.map((entry) => entry.pluginId).sort();
  // 🎯️ "standalone session = target plugin plus every plugin whose `contributes` intersects the
  // target's `consumes`" — exactly what `resolveRegistryPluginIdsForFilter` computes, so re-derive the
  // expectation instead of asserting a hardcoded id list.
  const expectedStandaloneIds = [...resolveRegistryPluginIdsForFilter(standaloneVariant)].sort();
  const expectedRegistryPluginId = resolveRegistryPluginIdForFilter(standaloneVariant, repoRoot);
  if (standalone.registryPluginId !== expectedRegistryPluginId || standalone.hostMode || standalonePluginIds.join(",") !== expectedStandaloneIds.join(",")) {
    errors.push(`standalone session "${standaloneVariant}" resolved unexpectedly (${JSON.stringify({ registryPluginId: standalone.registryPluginId, expectedRegistryPluginId, hostMode: standalone.hostMode, pluginIds: standalonePluginIds, expectedPluginIds: expectedStandaloneIds })})`);
  }

  const studioVariant = resolveDefaultHostVariant(repoRoot);
  const studio = buildPlaygroundSession(studioVariant, repoRoot);
  // 🎯️ "studio/host session has landingAppId==='home', hostAppId==='studio', and includes every
  // registry plugin" — `buildPlaygroundSession` expands studio sessions with no filter, so the exact
  // registry plugin count is the structural expectation, not a magic threshold.
  const totalRegistryPlugins = generatePluginRegistry(repoRoot).length;
  if (!studio.hostMode || studio.host?.landingAppId !== "home" || studio.host.hostAppId !== "studio" || studio.plugins.length !== totalRegistryPlugins) {
    errors.push(`studio session "${studioVariant}" resolved unexpectedly (${JSON.stringify({ hostMode: studio.hostMode, host: studio.host, pluginCount: studio.plugins.length, totalRegistryPlugins })})`);
  }

  if (!isHostPluginFilter(studioVariant, repoRoot) || isHostPluginFilter(standaloneVariant, repoRoot)) {
    errors.push("host filter metadata does not distinguish host and standalone playgrounds");
  }
  return errors;
}

/** @emoji 🗂️ The full generated catalog, rendered in memory once and consumed by both `generate`
 * (writes) and `check` (byte-compares) so the two can never disagree about what belongs in
 * `🤖️generated/`. */
function renderCatalogFiles(repoRoot: string, view?: RegistryCatalogInputView): { files: Record<string, string>; entries: PluginRegistryEntry[]; playgrounds: PlaygroundEntry[]; frameworkPackages: FrameworkPackageEntry[] } {
  const packages = discoverCatalogPackages(repoRoot, TAXONOMY, view);
  const entries = generatePluginRegistry(repoRoot, { packages, view });
  const playgrounds = generatePlaygroundRegistry(repoRoot, { packages, view });
  const frameworkPackages = generateFrameworkPackageRegistry(repoRoot, packages);
  const hostVariant = defaultHostVariant(entries, playgrounds);
  return {
    entries,
    playgrounds,
    frameworkPackages,
    files: {
      "🔣️plugins.json": `${JSON.stringify(entries, null, 2)}\n`,
      "🟦️plugins.ts": emitTypeScript(entries),
      "🔣️playgrounds.json": `${JSON.stringify(playgrounds, null, 2)}\n`,
      "🟦️playgrounds.ts": emitPlaygroundsTypeScript(playgrounds, hostVariant),
      "🔣️framework.json": `${JSON.stringify(frameworkPackages, null, 2)}\n`,
      "🟦️framework.ts": emitFrameworkPackagesTypeScript(frameworkPackages),
      "🦀️hosts.rs": emitRustHosts(entries, playgrounds),
      "🦀️artifacts.rs": emitRustArtifacts(entries),
    },
  };
}

class GenerateScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const { files, entries, playgrounds, frameworkPackages } = renderCatalogFiles(repoRoot);
    const outDir = join(this.root, "🤖️generated");
    mkdirSync(outDir, { recursive: true });
    const expected = new Set(Object.keys(files));
    for (const name of readdirSync(outDir)) if (!expected.has(name)) rmSync(join(outDir, name), { recursive: true, force: true });
    for (const [name, content] of Object.entries(files)) writeFileSync(join(outDir, name), content);
    console.log(`plugin registry catalog refreshed (${entries.length} plugin crates, ${playgrounds.length} playgrounds, ${frameworkPackages.length} framework packages) -> ${outDir}`);
    // 🖥️ `.vscode/launch.json` is the second consumer of the very same playground catalog, so it is
    // regenerated here rather than from a separate entry point — `check` enforces its freshness. Written
    // last so a seed/devLaunchers problem can never leave the catalog itself unwritten.
    const launchPath = join(repoRoot, LAUNCH_OUTPUT_REL_PATH);
    writeFileSync(launchPath, generateLaunchJson(repoRoot, playgrounds));
    console.log(`${LAUNCH_OUTPUT_REL_PATH} regenerated -> ${launchPath}`);
  }
}

/** 🧾️ Emits exact registry/launch bytes and stale removals without touching either output root. */
class PreviewGeneratedScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const protocol = process.env.SEMIO_GENERATOR_PREVIEW_PROTOCOL;
    const authority = TAXONOMY.generatorContracts["plugin-registry"].inputDiscovery!.previewInput;
    if (protocol && protocol !== authority.protocol) throw new Error("Unknown registry preview protocol");
    const native = registryCatalogInputView(repoRoot, TAXONOMY);
    const cancelFile = process.env.SEMIO_GENERATOR_PREVIEW_CANCEL_FILE;
    const cancelPath = cancelFile ? relative(repoRoot, cancelFile).replaceAll("\\", "/") : undefined;
    if (cancelPath) native.kind(cancelPath);
    const checkCancellation = (): void => {
      if (!cancelFile) return;
      try { lstatSync(cancelFile); } catch (error) { if ((error as NodeJS.ErrnoException).code === "ENOENT") return; throw error; }
      throw new Error("Registry preview cancelled");
    };
    const base: RegistryCatalogInputView = {
      entries(path) { checkCancellation(); return native.entries(path); },
      kind(path) { checkCancellation(); return native.kind(path); },
      readText(path) { checkCancellation(); return native.readText(path); },
    };
    let payload = "";
    if (protocol) {
      const chunks: Buffer[] = [];
      let size = 0;
      while (true) {
        checkCancellation();
        const chunk = Buffer.alloc(Math.min(65536, authority.maxBytes - size + 1));
        const length = readSync(0, chunk, 0, chunk.length, null);
        if (!length) break;
        size += length;
        if (size > authority.maxBytes) throw new Error("Registry preview input exceeds its declared byte limit");
        chunks.push(chunk.subarray(0, length));
      }
      payload = Buffer.concat(chunks).toString("utf8");
    }
    const view = protocol ? registryCatalogProjectedInputView(repoRoot, TAXONOMY, parseRegistryCatalogProjection(payload, TAXONOMY), base) : base;
    const { files, playgrounds } = renderCatalogFiles(repoRoot, view);
    const outDir = join(this.root, "🤖️generated");
    const rootPath = relative(repoRoot, outDir).replaceAll("\\", "/").normalize("NFC");
    const launchPath = join(repoRoot, LAUNCH_OUTPUT_REL_PATH);
    const nodes = [
      { bytesBase64: "", mode: 0o755, nodeKind: "directory" as const, path: rootPath },
      ...Object.entries(files).map(([name, content]) => ({ bytesBase64: Buffer.from(content).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: `${rootPath}/${name.normalize("NFC")}` })),
      { bytesBase64: Buffer.from(generateLaunchJson(repoRoot, playgrounds, (path) => view.readText(path))).toString("base64"), mode: 0o644, nodeKind: "file" as const, path: relative(repoRoot, launchPath).replaceAll("\\", "/").normalize("NFC") },
    ].sort((left, right) => Buffer.from(left.path).compare(Buffer.from(right.path)));
    const expected = new Set(Object.keys(files));
    const staleRemovals = (existsSync(outDir) ? readdirSync(outDir) : []).filter((name) => !expected.has(name)).map((name) => `${rootPath}/${name.normalize("NFC")}`).sort((left, right) => Buffer.from(left).compare(Buffer.from(right)));
    process.stdout.write(`${JSON.stringify({ contractId: "plugin-registry", nodes, schemaVersion: 1, staleRemovals })}\n`);
  }
}

/** @emoji 🔎️ Renders the catalog in memory and byte-compares it against `generated/*` plus
 * `.vscode/launch.json` — never writes (a lint/verify step must never let the auto-commit daemon land
 * regenerated files). Launch freshness is folded in here rather than living in a second, unenforced
 * entry point, so one `check` covers every artifact `generate` produces. */
//#region 🔖️DescriptorGate
/** 🦀️ Mirrors `emitRustArtifacts`'s own `PLUGIN_WASM_TARGET_DIR`/`PLUGIN_WASM_PROFILE_DIRS` —
 * kept as a second literal (not imported from the generated file) since this runs at `check` time,
 * before/independent of whether `🤖️generated/🦀️artifacts.rs` itself is fresh. */
const WASM_TARGET_DIR = ["target", "wasm32-wasip2"];
const WASM_PROFILE_DIRS = ["debug", "wasm-release"];

/** #️⃣ Lowercase hex SHA-256 — same algorithm `semio-framework-plugin-describe` uses for
 * `hashes.wasmSha256` (see that crate's own `sha256_hex` doc for why not this repo's usual
 * `blake3`-based `semio-framework-hash`). */
function sha256HexOfFile(path: string): string {
  return createHash("sha256").update(readFileSync(path)).digest("hex");
}

/** 🔎️ Finds the crate's built wasm component under either profile dir, `undefined` if neither exists. */
function findBuiltWasm(repoRoot: string, wasmOut: string): string | undefined {
  for (const profile of WASM_PROFILE_DIRS) {
    const path = join(repoRoot, ...WASM_TARGET_DIR, profile, wasmOut);
    if (existsSync(path)) return path;
  }
  return undefined;
}

/**
 * 🛂️ `📓️design-abi.md` §3's registry `check` extension: a descriptor exists per crate, `pluginId`
 * matches the component package, `extends` matches the first dependency, every
 * `on-extension-request:<point>` activation event an extension declares names a real extension point
 * on its host plugin, and the built wasm's sha256 matches `hashes.wasmSha256`.
 *
 * **Severity, deliberately asymmetric** (documented judgment call, `📓️terra-E1-describe-report.md`):
 * zero plugin crates have been migrated to emit a descriptor yet (W3's `M0`…`M8` land after this
 * packet) — hard-failing "descriptor exists" today would permanently red the gate until every plugin
 * migrates, which is not what a W2 packet's `check` extension is for. So "missing descriptor" and
 * "wasm not built" are **warnings** (mirrors this file's own `PLUGIN_AREAS_STATE`
 * legacy/mixed/clean idiom for the taxonomy-tree audit above); every check that only applies to a
 * crate that DOES have a descriptor (id/extends/extension-point/hash consistency) is a **hard
 * error** — a wrong descriptor is worse than a missing one.
 */
function validateDescriptors(entries: readonly PluginRegistryEntry[], repoRoot: string): { warnings: string[]; errors: string[] } {
  const warnings: string[] = [];
  const errors: string[] = [];
  const byId = new Map(entries.map((entry) => [entry.pluginId, entry]));
  let described = 0;
  for (const entry of entries) {
    const descriptorPath = join(entry.cratePath, ...DESCRIPTOR_JSON_REL_PATH);
    if (entry.hashes === undefined && entry.executionMode === undefined && entry.activationEvents.length === 0 && entry.extensionPoints.length === 0) {
      // 🚧️ No `🔣️.json` (or one too malformed to read) — see `readDescriptorJson`.
      warnings.push(`${entry.pluginId}: no ${descriptorPath} yet — run \`bun ./📜️script.ts describe\` in ${entry.cratePath} after building its wasm32-wasip2 component`);
      continue;
    }
    described++;
    const descriptor = readDescriptorJson(repoRoot, entry.cratePath);
    const packageId = descriptor?.packageId;
    if (packageId !== entry.packageId) {
      errors.push(`${entry.pluginId}: ${descriptorPath} packageId is ${JSON.stringify(packageId)}, expected ${JSON.stringify(entry.packageId)} (the complete [package.metadata.component] package)`);
    }
    const manifestPluginId = (descriptor?.manifest as Record<string, unknown> | undefined)?.pluginId;
    if (manifestPluginId !== entry.pluginId) {
      errors.push(`${entry.pluginId}: ${descriptorPath} manifest.pluginId is ${JSON.stringify(manifestPluginId)}, expected ${JSON.stringify(entry.pluginId)} (the [package.metadata.component] package)`);
    }
    if (entry.extends !== undefined) {
      const manifestDependencies = (descriptor?.manifest as Record<string, unknown> | undefined)?.dependencies;
      const firstDependencyId = Array.isArray(manifestDependencies) ? (manifestDependencies[0] as { pluginId?: unknown } | undefined)?.pluginId : undefined;
      if (firstDependencyId !== entry.extends) {
        errors.push(`${entry.pluginId}: extends ${JSON.stringify(entry.extends)} but manifest.dependencies[0].pluginId is ${JSON.stringify(firstDependencyId)} — contract freeze §4 rule 1 requires these to match`);
      }
    }
    const hostPluginId = entry.extends;
    const hostExtensionPoints = hostPluginId ? new Set(byId.get(hostPluginId)?.extensionPoints ?? []) : undefined;
    for (const activation of entry.activationEvents) {
      const point = activation.startsWith("on-extension-request:") ? activation.slice("on-extension-request:".length) : undefined;
      if (point === undefined) continue;
      if (hostPluginId === undefined) {
        errors.push(`${entry.pluginId}: declares on-extension-request:${point} but has no "extends" host plugin`);
      } else if (!hostExtensionPoints?.has(point)) {
        errors.push(`${entry.pluginId}: declares on-extension-request:${point}, but host plugin ${JSON.stringify(hostPluginId)} declares no extension point ${JSON.stringify(point)} (has: ${[...(hostExtensionPoints ?? [])].join(", ") || "none"})`);
      }
    }
    if (entry.hashes) {
      const builtWasm = findBuiltWasm(repoRoot, entry.wasmOut);
      if (builtWasm === undefined) {
        warnings.push(`${entry.pluginId}: has hashes.wasmSha256 but no built wasm found under ${WASM_TARGET_DIR.join("/")}/{${WASM_PROFILE_DIRS.join(",")}}/${entry.wasmOut} — skipping hash check`);
      } else {
        const actual = sha256HexOfFile(builtWasm);
        if (actual !== entry.hashes.wasmSha256) {
          errors.push(`${entry.pluginId}: hashes.wasmSha256 is ${entry.hashes.wasmSha256} but ${relative(repoRoot, builtWasm)} actually hashes to ${actual} — re-run \`describe\` after the latest build`);
        }
      }
    }
  }
  if (described > 0 || warnings.length === 0) {
    console.log(`descriptor gate: ${described}/${entries.length} crates have a 🔣️.json.`);
  }
  return { warnings, errors };
}
//#endregion 🔖️DescriptorGate

//#region 🔖️CatalogCompleteness
export const CATALOG_NODE_MAX = 256;
export const CATALOG_DEPENDENCY_MAX = 128;
export const CATALOG_ARTIFACT_MAX_BYTES = 64 * 1024 * 1024;
export const CATALOG_DESCRIPTOR_MAX_BYTES = 64 * 1024;
export const CATALOG_DIAGNOSTIC_MAX_BYTES = 4096;
export const CATALOG_COMMIT_MARKER_MAX_BYTES = 64 * 1024;
export const CATALOG_COMMIT_MARKER_FILENAME = "🧾️.catalog-root.json";
const CATALOG_IO_CHUNK_BYTES = 64 * 1024;
const CATALOG_ID = /^[a-z0-9]+(?:-[a-z0-9]+)*$/;
const CATALOG_PACKAGE_ID = COMPONENT_PACKAGE_ID;
const CATALOG_SHA256 = /^[0-9a-f]{64}$/;
const CATALOG_DESCRIPTOR_PACK_FILENAME = "🛂️.descriptor.semio";
const CATALOG_DESCRIPTOR_TOP_LEVEL = new Set(["descriptorVersion", "packageId", "role", "manifest", "activationEvents", "capabilityRequests", "extensionPoints", "execution", "quotas", "contributions", "assets", "hashes"]);
const CATALOG_MANIFEST_FIELDS = new Set(["pluginId", "label", "version", "apps", "examples", "capabilities", "topicContributions", "commands", "artifactKinds", "dependencies", "contributions"]);

export type CatalogVerificationNode = {
  readonly pluginId: string;
  readonly role: "plugin" | "extension";
  readonly dependsOn: readonly string[];
};

export type CatalogVerificationStatus = "verified" | "failed" | "blocked" | "cancelled";

export type CatalogVerificationProgress = {
  readonly schemaVersion: 1;
  readonly completed: number;
  readonly total: number;
  readonly pluginId: string;
  readonly status: CatalogVerificationStatus;
};

export type CatalogVerificationResult = {
  readonly schemaVersion: 1;
  readonly order: readonly string[];
  readonly results: readonly { readonly pluginId: string; readonly status: CatalogVerificationStatus; readonly diagnostic?: string }[];
  readonly publication: "committed" | "withheld" | "failed" | "not-requested";
  readonly publicationDiagnostic?: string;
};

export type CatalogVerificationExecutor<Receipt> = {
  readonly verify: (node: CatalogVerificationNode) => Promise<Receipt>;
  readonly publish?: (verified: readonly { readonly node: CatalogVerificationNode; readonly receipt: Receipt }[]) => Promise<void>;
};

export type CatalogVerificationControl = {
  readonly cancelled: () => boolean;
  readonly progress?: (progress: CatalogVerificationProgress) => void;
};

export type StrictCatalogDescriptor = {
  readonly entry: PluginRegistryEntry;
  readonly jsonPath: string;
  readonly jsonBytes: Uint8Array;
  readonly packPath: string;
  readonly packBytes: Uint8Array;
  readonly descriptor: Readonly<Record<string, unknown>>;
  readonly hashes: PluginDescriptorHashes;
};

export type CatalogSourceIssue = {
  readonly code: "manifest-invalid" | "identity-conflict" | "descriptor-pair-missing" | "descriptor-pair-incomplete" | "descriptor-invalid" | "dependency-invalid";
  readonly path: string;
  readonly pluginId?: string;
  readonly diagnostic: string;
};

export type CatalogSourceAudit = {
  readonly manifestCount: number;
  readonly entries: readonly PluginRegistryEntry[];
  readonly sources: readonly StrictCatalogDescriptor[];
  readonly order: readonly string[];
  readonly issues: readonly CatalogSourceIssue[];
};

export type CatalogArtifactProgress = {
  readonly pluginId: string;
  readonly artifact: "raw" | "core" | "descriptor";
  readonly bytesRead: number;
  readonly totalBytes: number;
};

export type CatalogFileReceipt = {
  readonly path: string;
  readonly bytes: number;
  readonly sha256: string;
};

export type FreshCatalogCommitMarker = {
  readonly schemaVersion: 1;
  readonly packageId: string;
  readonly pluginId: string;
  readonly packageName: string;
  readonly wasmOut: string;
  readonly raw: CatalogFileReceipt;
  readonly core: CatalogFileReceipt;
  readonly descriptor: CatalogFileReceipt;
  readonly descriptorSha256: string;
};

export type CatalogArtifactControl = {
  readonly cancelled?: () => boolean;
  readonly progress?: (progress: CatalogArtifactProgress) => void;
  readonly afterArtifact?: (artifact: CatalogArtifactProgress["artifact"]) => void;
};

export type CatalogArtifactReceipt = {
  readonly pluginId: string;
  readonly rawSha256: string;
  readonly coreSha256: string;
  readonly descriptorSha256: string;
  readonly rawBytes: Uint8Array;
  readonly coreBytes: Uint8Array;
  readonly descriptorBytes: Uint8Array;
};

export interface FreshCatalogBuildVerifier {
  readonly root: string;
  verify(entry: PluginRegistryEntry, control?: CatalogArtifactControl): CatalogArtifactReceipt;
}

/** 🧯️ Retains at most the public catalog diagnostic budget without splitting a Unicode scalar. */
function boundedCatalogDiagnostic(value: unknown): string {
  const message = value instanceof Error ? value.message : String(value);
  if (Buffer.byteLength(message) <= CATALOG_DIAGNOSTIC_MAX_BYTES) return message;
  let output = "";
  for (const scalar of message) {
    if (Buffer.byteLength(output) + Buffer.byteLength(scalar) > CATALOG_DIAGNOSTIC_MAX_BYTES - 3) break;
    output += scalar;
  }
  return `${output}...`;
}

/** 🧭️ Returns true only when `candidate` is `root` or is structurally contained below it. */
function pathIsWithin(root: string, candidate: string): boolean {
  const rel = relative(root, candidate);
  return rel === "" || (!rel.startsWith("..") && !isAbsolute(rel));
}

/** 🧬️ Rejects lossy or unsupported JSON values before they enter the pack identity calculation. */
function isStrictJsonValue(value: unknown): boolean {
  if (value === null || typeof value === "string" || typeof value === "boolean") return true;
  if (typeof value === "number") return Number.isFinite(value) && (!Number.isInteger(value) || Number.isSafeInteger(value));
  if (Array.isArray(value)) return value.every(isStrictJsonValue);
  if (typeof value !== "object") return false;
  return Object.values(value as Record<string, unknown>).every(isStrictJsonValue);
}

/** 🪞️ Normalizes serde's external enum tags and the pack codec's `kind`/`value` tags for comparison. */
function normalizeCatalogDescriptorEnums(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(normalizeCatalogDescriptorEnums);
  if (value === null || typeof value !== "object") return value;
  const record = value as Record<string, unknown>;
  const keys = Object.keys(record);
  if (keys.length === 2 && keys.includes("kind") && keys.includes("value") && typeof record.kind === "string") return { [record.kind]: normalizeCatalogDescriptorEnums(record.value) };
  if (keys.length === 1 && keys[0] === "kind" && typeof record.kind === "string") return record.kind;
  return Object.fromEntries(Object.entries(record).map(([key, child]) => [key, normalizeCatalogDescriptorEnums(child)]));
}

/** 🧱️ Produces one stable dependency-first ordering, with ready plugins before ready extensions. */
export function orderCatalogNodes(nodes: readonly CatalogVerificationNode[]): CatalogVerificationNode[] {
  if (nodes.length === 0 || nodes.length > CATALOG_NODE_MAX) throw new Error(`catalog plan must contain 1..${CATALOG_NODE_MAX} nodes, got ${nodes.length}`);
  const byId = new Map<string, CatalogVerificationNode>();
  for (const node of nodes) {
    if (!CATALOG_ID.test(node.pluginId)) throw new Error(`catalog node has invalid plugin id ${JSON.stringify(node.pluginId)}`);
    if (node.role !== "plugin" && node.role !== "extension") throw new Error(`${node.pluginId}: invalid role ${JSON.stringify(node.role)}`);
    if (node.dependsOn.length > CATALOG_DEPENDENCY_MAX) throw new Error(`${node.pluginId}: dependencies exceed ${CATALOG_DEPENDENCY_MAX}`);
    if (byId.has(node.pluginId)) throw new Error(`catalog plan has duplicate plugin id ${JSON.stringify(node.pluginId)}`);
    if (new Set(node.dependsOn).size !== node.dependsOn.length) throw new Error(`${node.pluginId}: duplicate dependency identity`);
    byId.set(node.pluginId, node);
  }
  for (const node of nodes) {
    for (const dependency of node.dependsOn) {
      if (!CATALOG_ID.test(dependency)) throw new Error(`${node.pluginId}: invalid dependency id ${JSON.stringify(dependency)}`);
      if (!byId.has(dependency)) throw new Error(`${node.pluginId} depends on absent catalog node ${JSON.stringify(dependency)}`);
      if (dependency === node.pluginId) throw new Error(`${node.pluginId}: self dependency is a cycle`);
    }
  }
  const indegree = new Map(nodes.map((node) => [node.pluginId, node.dependsOn.length]));
  const dependents = new Map<string, string[]>();
  for (const node of nodes) for (const dependency of node.dependsOn) dependents.set(dependency, [...(dependents.get(dependency) ?? []), node.pluginId]);
  const compare = (left: CatalogVerificationNode, right: CatalogVerificationNode): number => (left.role === right.role ? left.pluginId.localeCompare(right.pluginId) : left.role === "plugin" ? -1 : 1);
  const ready = nodes.filter((node) => indegree.get(node.pluginId) === 0).sort(compare);
  const ordered: CatalogVerificationNode[] = [];
  while (ready.length > 0) {
    const node = ready.shift()!;
    ordered.push(node);
    for (const dependentId of dependents.get(node.pluginId) ?? []) {
      const next = indegree.get(dependentId)! - 1;
      indegree.set(dependentId, next);
      if (next === 0) {
        ready.push(byId.get(dependentId)!);
        ready.sort(compare);
      }
    }
  }
  if (ordered.length !== nodes.length) throw new Error(`catalog dependency cycle includes ${nodes.filter((node) => !ordered.includes(node)).map(({ pluginId }) => pluginId).sort().join(", ")}`);
  return ordered;
}

/** 🧾️ Verifies a bounded graph and exposes exactly one all-row publication call after full success. */
export async function executeCatalogVerificationPlan<Receipt>(nodes: readonly CatalogVerificationNode[], executor: CatalogVerificationExecutor<Receipt>, control: CatalogVerificationControl): Promise<CatalogVerificationResult> {
  const ordered = orderCatalogNodes(nodes);
  const results: { pluginId: string; status: CatalogVerificationStatus; diagnostic?: string }[] = [];
  const resultById = new Map<string, CatalogVerificationStatus>();
  const verified: { node: CatalogVerificationNode; receipt: Receipt }[] = [];
  for (const node of ordered) {
    let status: CatalogVerificationStatus;
    let diagnostic: string | undefined;
    if (control.cancelled()) {
      status = "cancelled";
      diagnostic = "catalog verification cancelled";
    } else {
      const unavailable = node.dependsOn.find((dependency) => resultById.get(dependency) !== "verified");
      if (unavailable) {
        status = "blocked";
        diagnostic = `dependency ${unavailable} was not verified`;
      } else {
        try {
          verified.push({ node, receipt: await executor.verify(node) });
          status = "verified";
        } catch (error) {
          status = "failed";
          diagnostic = boundedCatalogDiagnostic(error);
        }
      }
    }
    resultById.set(node.pluginId, status);
    results.push({ pluginId: node.pluginId, status, ...(diagnostic ? { diagnostic } : {}) });
    control.progress?.({ schemaVersion: 1, completed: results.length, total: ordered.length, pluginId: node.pluginId, status });
  }
  if (results.some(({ status }) => status !== "verified") || control.cancelled()) return { schemaVersion: 1, order: ordered.map(({ pluginId }) => pluginId), results, publication: "withheld" };
  if (!executor.publish) return { schemaVersion: 1, order: ordered.map(({ pluginId }) => pluginId), results, publication: "not-requested" };
  try {
    await executor.publish(verified);
    return { schemaVersion: 1, order: ordered.map(({ pluginId }) => pluginId), results, publication: "committed" };
  } catch (error) {
    return { schemaVersion: 1, order: ordered.map(({ pluginId }) => pluginId), results, publication: "failed", publicationDiagnostic: boundedCatalogDiagnostic(error) };
  }
}

/** 📄️ Reads one bounded regular non-symlink file and rejects a path that resolves outside `root`. */
function readCatalogFile(path: string, root: string, limit: number): Uint8Array {
  const info = lstatSync(path);
  if (info.isSymbolicLink() || !info.isFile()) throw new Error(`${path} must be a regular non-symlink file`);
  if (info.size > limit) throw new Error(`${path} exceeds ${limit} bytes`);
  const realRoot = realpathSync(root);
  const realPath = realpathSync(path);
  if (!pathIsWithin(realRoot, realPath)) throw new Error(`${path} resolves outside ${root}`);
  return readFileSync(path);
}

/** 🧭️ Rejects duplicated object names before the platform JSON decoder can collapse them. */
function rejectDuplicateJsonObjectNames(source: string): void {
  let index = 0;
  const whitespace = (): void => {
    while (index < source.length && /\s/u.test(source[index]!)) index++;
  };
  const string = (): string => {
    if (source[index] !== '"') throw new Error(`expected JSON string at byte ${index}`);
    const start = index++;
    while (index < source.length) {
      const character = source[index++]!;
      if (character === '"') return JSON.parse(source.slice(start, index)) as string;
      if (character === "\\") index++;
    }
    throw new Error("unterminated JSON string");
  };
  const value = (depth: number): void => {
    if (depth > 128) throw new Error("JSON nesting exceeds 128 levels");
    whitespace();
    if (source[index] === "{") {
      index++;
      whitespace();
      const names = new Set<string>();
      if (source[index] === "}") {
        index++;
        return;
      }
      while (true) {
        whitespace();
        const name = string();
        if (names.has(name)) throw new Error(`duplicate object field ${JSON.stringify(name)}`);
        names.add(name);
        whitespace();
        if (source[index++] !== ":") throw new Error(`expected JSON colon at byte ${index - 1}`);
        value(depth + 1);
        whitespace();
        const separator = source[index++];
        if (separator === "}") return;
        if (separator !== ",") throw new Error(`expected JSON object separator at byte ${index - 1}`);
      }
    }
    if (source[index] === "[") {
      index++;
      whitespace();
      if (source[index] === "]") {
        index++;
        return;
      }
      while (true) {
        value(depth + 1);
        whitespace();
        const separator = source[index++];
        if (separator === "]") return;
        if (separator !== ",") throw new Error(`expected JSON array separator at byte ${index - 1}`);
      }
    }
    if (source[index] === '"') {
      string();
      return;
    }
    const token = source.slice(index).match(/^(?:true|false|null|-?(?:0|[1-9]\d*)(?:\.\d+)?(?:[eE][+-]?\d+)?)/u)?.[0];
    if (!token) throw new Error(`invalid JSON value at byte ${index}`);
    index += token.length;
  };
  value(0);
  whitespace();
  if (index !== source.length) throw new Error(`trailing JSON input at byte ${index}`);
}

function validateCatalogDescriptorValue(entry: PluginRegistryEntry, descriptor: unknown): PluginDescriptorHashes {
  if (descriptor === null || typeof descriptor !== "object" || Array.isArray(descriptor) || !isStrictJsonValue(descriptor)) throw new Error(`${entry.pluginId}: descriptor is not a lossless JSON object`);
  const record = descriptor as Record<string, unknown>;
  const unknownTop = Object.keys(record).filter((key) => !CATALOG_DESCRIPTOR_TOP_LEVEL.has(key));
  if (unknownTop.length > 0) throw new Error(`${entry.pluginId}: descriptor has unknown fields ${unknownTop.sort().join(", ")}`);
  if (record.descriptorVersion !== 1 || record.role !== entry.role) throw new Error(`${entry.pluginId}: descriptor version/role does not match source identity`);
  if (record.packageId !== entry.packageId || !CATALOG_PACKAGE_ID.test(String(record.packageId))) throw new Error(`${entry.pluginId}: descriptor packageId does not match the complete Cargo component identity`);
  const manifest = record.manifest;
  if (manifest === null || typeof manifest !== "object" || Array.isArray(manifest)) throw new Error(`${entry.pluginId}: descriptor manifest must be an object`);
  const manifestRecord = manifest as Record<string, unknown>;
  const unknownManifest = Object.keys(manifestRecord).filter((key) => !CATALOG_MANIFEST_FIELDS.has(key));
  if (unknownManifest.length > 0) throw new Error(`${entry.pluginId}: manifest has unknown fields ${unknownManifest.sort().join(", ")}`);
  if (manifestRecord.pluginId !== entry.pluginId || !CATALOG_ID.test(String(manifestRecord.pluginId))) throw new Error(`${entry.pluginId}: manifest.pluginId does not match the Cargo component identity`);
  if (typeof manifestRecord.label !== "string" || typeof manifestRecord.version !== "string" || !Array.isArray(manifestRecord.apps) || !Array.isArray(manifestRecord.examples)) throw new Error(`${entry.pluginId}: manifest required fields do not decode`);
  for (const field of ["capabilities", "topicContributions", "commands", "artifactKinds", "dependencies", "contributions"] as const) if (manifestRecord[field] !== undefined && !Array.isArray(manifestRecord[field])) throw new Error(`${entry.pluginId}: manifest.${field} must be an array`);
  for (const field of ["activationEvents", "capabilityRequests", "extensionPoints", "assets"] as const) if (record[field] !== undefined && !Array.isArray(record[field])) throw new Error(`${entry.pluginId}: descriptor.${field} must be an array`);
  if (!["declarative", "linked", "isolated", "exclusive", "cold"].includes(String(record.execution))) throw new Error(`${entry.pluginId}: descriptor execution mode does not decode`);
  if (record.quotas === null || typeof record.quotas !== "object" || Array.isArray(record.quotas) || record.contributions === null || typeof record.contributions !== "object" || Array.isArray(record.contributions)) throw new Error(`${entry.pluginId}: descriptor quotas/contributions must be objects`);
  const hashes = record.hashes;
  if (hashes === null || typeof hashes !== "object" || Array.isArray(hashes)) throw new Error(`${entry.pluginId}: descriptor hashes must be an object`);
  const hashRecord = hashes as Record<string, unknown>;
  if (Object.keys(hashRecord).sort().join(",") !== "coreWasmSha256,descriptorSha256,wasmSha256") throw new Error(`${entry.pluginId}: descriptor hashes must contain exactly raw/core/descriptor SHA-256`);
  for (const [name, value] of Object.entries(hashRecord)) if (typeof value !== "string" || !CATALOG_SHA256.test(value)) throw new Error(`${entry.pluginId}: hashes.${name} must be lowercase 64-hex`);
  if (entry.role === "extension" && (!entry.extends || entry.dependsOn[0] !== entry.extends)) throw new Error(`${entry.pluginId}: extension host must be its first dependency`);
  if (entry.role === "plugin" && entry.extends !== undefined) throw new Error(`${entry.pluginId}: plugin cannot declare an extension host`);
  return hashRecord as PluginDescriptorHashes;
}

/** 🔐️ Strict-decodes and cross-checks one owner-root JSON/pack `PackageDescriptor` pair. */
export function validateCatalogDescriptorPair(entry: PluginRegistryEntry, repoRoot: string): StrictCatalogDescriptor {
  const jsonPath = resolve(repoRoot, entry.cratePath, ...DESCRIPTOR_JSON_REL_PATH);
  const ownerRoot = dirname(jsonPath);
  const packPath = join(ownerRoot, CATALOG_DESCRIPTOR_PACK_FILENAME);
  if (!pathIsWithin(resolve(repoRoot), jsonPath) || !pathIsWithin(resolve(repoRoot), packPath)) throw new Error(`${entry.pluginId}: descriptor owner escapes the repository`);
  const jsonBytes = readCatalogFile(jsonPath, repoRoot, CATALOG_DESCRIPTOR_MAX_BYTES);
  const packBytes = readCatalogFile(packPath, repoRoot, CATALOG_DESCRIPTOR_MAX_BYTES);
  let descriptor: unknown;
  let packed: unknown;
  try {
    const json = Buffer.from(jsonBytes).toString("utf8");
    rejectDuplicateJsonObjectNames(json);
    descriptor = JSON.parse(json);
  } catch (error) {
    throw new Error(`${entry.pluginId}: descriptor JSON does not decode: ${boundedCatalogDiagnostic(error)}`);
  }
  try {
    packed = decodePackValue(packBytes);
  } catch (error) {
    throw new Error(`${entry.pluginId}: descriptor pack does not decode: ${boundedCatalogDiagnostic(error)}`);
  }
  if (descriptor === null || typeof descriptor !== "object" || Array.isArray(descriptor) || !isStrictJsonValue(descriptor)) throw new Error(`${entry.pluginId}: descriptor is not a lossless JSON object`);
  const record = descriptor as Record<string, unknown>;
  const unknownTop = Object.keys(record).filter((key) => !CATALOG_DESCRIPTOR_TOP_LEVEL.has(key));
  if (unknownTop.length > 0) throw new Error(`${entry.pluginId}: descriptor has unknown fields ${unknownTop.sort().join(", ")}`);
  if (record.descriptorVersion !== 1 || record.role !== entry.role) throw new Error(`${entry.pluginId}: descriptor version/role does not match source identity`);
  if (record.packageId !== entry.packageId || !CATALOG_PACKAGE_ID.test(String(record.packageId))) throw new Error(`${entry.pluginId}: descriptor packageId does not match the complete Cargo component identity`);
  const manifest = record.manifest;
  if (manifest === null || typeof manifest !== "object" || Array.isArray(manifest)) throw new Error(`${entry.pluginId}: descriptor manifest must be an object`);
  const manifestRecord = manifest as Record<string, unknown>;
  const unknownManifest = Object.keys(manifestRecord).filter((key) => !CATALOG_MANIFEST_FIELDS.has(key));
  if (unknownManifest.length > 0) throw new Error(`${entry.pluginId}: manifest has unknown fields ${unknownManifest.sort().join(", ")}`);
  if (manifestRecord.pluginId !== entry.pluginId || !CATALOG_ID.test(String(manifestRecord.pluginId))) throw new Error(`${entry.pluginId}: manifest.pluginId does not match the Cargo component identity`);
  if (typeof manifestRecord.label !== "string" || typeof manifestRecord.version !== "string" || !Array.isArray(manifestRecord.apps) || !Array.isArray(manifestRecord.examples)) throw new Error(`${entry.pluginId}: manifest required fields do not decode`);
  for (const field of ["capabilities", "topicContributions", "commands", "artifactKinds", "dependencies", "contributions"] as const) if (manifestRecord[field] !== undefined && !Array.isArray(manifestRecord[field])) throw new Error(`${entry.pluginId}: manifest.${field} must be an array`);
  for (const field of ["activationEvents", "capabilityRequests", "extensionPoints", "assets"] as const) if (record[field] !== undefined && !Array.isArray(record[field])) throw new Error(`${entry.pluginId}: descriptor.${field} must be an array`);
  if (!["declarative", "linked", "isolated", "exclusive", "cold"].includes(String(record.execution))) throw new Error(`${entry.pluginId}: descriptor execution mode does not decode`);
  if (record.quotas === null || typeof record.quotas !== "object" || Array.isArray(record.quotas) || record.contributions === null || typeof record.contributions !== "object" || Array.isArray(record.contributions)) throw new Error(`${entry.pluginId}: descriptor quotas/contributions must be objects`);
  const hashes = record.hashes;
  if (hashes === null || typeof hashes !== "object" || Array.isArray(hashes)) throw new Error(`${entry.pluginId}: descriptor hashes must be an object`);
  const hashRecord = hashes as Record<string, unknown>;
  if (Object.keys(hashRecord).sort().join(",") !== "coreWasmSha256,descriptorSha256,wasmSha256") throw new Error(`${entry.pluginId}: descriptor hashes must contain exactly raw/core/descriptor SHA-256`);
  for (const [name, value] of Object.entries(hashRecord)) if (typeof value !== "string" || !CATALOG_SHA256.test(value)) throw new Error(`${entry.pluginId}: hashes.${name} must be lowercase 64-hex`);
  if (!isDeepStrictEqual(normalizeCatalogDescriptorEnums(descriptor), normalizeCatalogDescriptorEnums(packed))) throw new Error(`${entry.pluginId}: descriptor JSON and pack forms disagree`);
  if (!Buffer.from(encodePackValue(packed)).equals(Buffer.from(packBytes))) throw new Error(`${entry.pluginId}: descriptor pack is not canonical or contains trailing bytes`);
  const blanked = structuredClone(packed as Record<string, unknown>);
  (blanked.hashes as Record<string, unknown>).descriptorSha256 = "";
  const actualDescriptorHash = createHash("sha256").update(encodePackValue(blanked)).digest("hex");
  if (actualDescriptorHash !== hashRecord.descriptorSha256) throw new Error(`${entry.pluginId}: descriptor self-hash mismatch`);
  const exactHashes = hashRecord as PluginDescriptorHashes;
  if (!entry.hashes || entry.hashes.wasmSha256 !== exactHashes.wasmSha256 || entry.hashes.coreWasmSha256 !== exactHashes.coreWasmSha256 || entry.hashes.descriptorSha256 !== exactHashes.descriptorSha256) throw new Error(`${entry.pluginId}: rendered registry hashes do not exactly match the source descriptor`);
  if (entry.role === "extension" && (!entry.extends || entry.dependsOn[0] !== entry.extends)) throw new Error(`${entry.pluginId}: extension host must be its first dependency`);
  if (entry.role === "plugin" && entry.extends !== undefined) throw new Error(`${entry.pluginId}: plugin cannot declare an extension host`);
  return { entry, jsonPath, jsonBytes, packPath, packBytes, descriptor: packed as Record<string, unknown>, hashes: exactHashes };
}

/** 🧮️ Independently enumerates discovered component manifests and audits source identities without reading generated catalog files. */
export function auditPluginCatalogSources(repoRoot = getWorkspaceRoot(), control?: { readonly cancelled?: () => boolean; readonly progress?: (completed: number, total: number, path: string) => void; readonly ownerDescriptors?: "required" | "ignored" }): CatalogSourceAudit {
  const manifestPaths = findPluginCargoFiles(repoRoot);
  if (manifestPaths.length === 0 || manifestPaths.length > CATALOG_NODE_MAX) throw new Error(`catalog discovery returned ${manifestPaths.length} manifests; expected 1..${CATALOG_NODE_MAX}`);
  const entries: PluginRegistryEntry[] = [];
  let sources: StrictCatalogDescriptor[] = [];
  const issues: CatalogSourceIssue[] = [];
  const byPlugin = new Map<string, string>();
  const byPackage = new Map<string, string>();
  const pluginByPackage = new Map<string, string>();
  const manifestByPlugin = new Map<string, string>();
  for (let index = 0; index < manifestPaths.length; index++) {
    const manifestPath = manifestPaths[index]!;
    if (control?.cancelled?.()) throw new Error("catalog source audit cancelled");
    let entry: PluginRegistryEntry | undefined;
    try {
      const info = lstatSync(manifestPath);
      if (info.isSymbolicLink() || !info.isFile()) throw new Error("Cargo manifest is not a regular non-symlink file");
      const manifestText = readFileSync(manifestPath, "utf8");
      const semioBlock = tomlBlocksAfterHeader(manifestText.split("\n"), (line) => line === "[package.metadata.semio]")[0]?.join("\n") ?? "";
      const rawRole = semioBlock.match(/^role\s*=\s*"([^"]+)"/m)?.[1];
      if (rawRole !== "plugin" && rawRole !== "extension") throw new Error(`metadata.semio.role must be plugin or extension, got ${JSON.stringify(rawRole)}`);
      entry = parsePluginCargo(manifestPath, repoRoot, undefined, control?.ownerDescriptors);
      if (!CATALOG_ID.test(entry.pluginId) || entry.role !== rawRole) throw new Error("Cargo component/role identity is malformed");
      if (rawRole === "extension" && (!entry.extends || entry.dependsOn[0] !== entry.extends)) throw new Error("extension must declare extends as its first dependency");
      if (rawRole === "plugin" && entry.extends !== undefined) throw new Error("plugin must not declare extends");
      const previousPlugin = byPlugin.get(entry.pluginId);
      const previousPackage = byPackage.get(entry.packageName);
      if (previousPlugin || previousPackage) {
        issues.push({ code: "identity-conflict", path: relative(repoRoot, manifestPath), pluginId: entry.pluginId, diagnostic: boundedCatalogDiagnostic(`duplicate identity conflicts with ${previousPlugin ?? previousPackage}`) });
      } else {
        byPlugin.set(entry.pluginId, manifestPath);
        byPackage.set(entry.packageName, manifestPath);
        pluginByPackage.set(entry.packageName, entry.pluginId);
        manifestByPlugin.set(entry.pluginId, manifestText);
        entries.push(entry);
      }
    } catch (error) {
      issues.push({ code: "manifest-invalid", path: relative(repoRoot, manifestPath), diagnostic: boundedCatalogDiagnostic(error) });
    }
    if (entry && control?.ownerDescriptors !== "ignored") {
      const jsonPath = resolve(repoRoot, entry.cratePath, ...DESCRIPTOR_JSON_REL_PATH);
      const packPath = join(dirname(jsonPath), CATALOG_DESCRIPTOR_PACK_FILENAME);
      const jsonExists = existsSync(jsonPath);
      const packExists = existsSync(packPath);
      if (!jsonExists && !packExists) {
        issues.push({ code: "descriptor-pair-missing", path: relative(repoRoot, dirname(jsonPath)), pluginId: entry.pluginId, diagnostic: "owner-root descriptor JSON and pack are both missing" });
      } else if (!jsonExists || !packExists) {
        issues.push({ code: "descriptor-pair-incomplete", path: relative(repoRoot, dirname(jsonPath)), pluginId: entry.pluginId, diagnostic: `owner-root descriptor ${jsonExists ? "pack" : "JSON"} is missing` });
      } else {
        try {
          sources.push(validateCatalogDescriptorPair(entry, repoRoot));
        } catch (error) {
          issues.push({ code: "descriptor-invalid", path: relative(repoRoot, dirname(jsonPath)), pluginId: entry.pluginId, diagnostic: boundedCatalogDiagnostic(error) });
        }
      }
    }
    control?.progress?.(index + 1, manifestPaths.length, relative(repoRoot, manifestPath));
  }
  const canonicalEntries = entries.map((entry) => {
    const cargoPackages = parseCargoPluginDependencyPackageNames(manifestByPlugin.get(entry.pluginId) ?? "", entry.packageName);
    const mapped: string[] = [];
    for (const packageName of cargoPackages) {
      const pluginId = pluginByPackage.get(packageName);
      if (pluginId) mapped.push(pluginId);
    }
    const dependsOn = entry.extends ? [entry.extends, ...mapped.filter((pluginId) => pluginId !== entry.extends)] : mapped;
    return { ...entry, dependsOn: [...new Set(dependsOn)] };
  });
  const canonicalById = new Map(canonicalEntries.map((entry) => [entry.pluginId, entry]));
  sources = sources.map((source) => ({ ...source, entry: canonicalById.get(source.entry.pluginId) ?? source.entry }));
  let order: string[] = [];
  try {
    order = orderCatalogNodes(canonicalEntries).map(({ pluginId }) => pluginId);
  } catch (error) {
    issues.push({ code: "dependency-invalid", path: "Cargo.toml", diagnostic: boundedCatalogDiagnostic(error) });
  }
  issues.sort((left, right) => `${left.pluginId ?? ""}:${left.code}:${left.path}`.localeCompare(`${right.pluginId ?? ""}:${right.code}:${right.path}`));
  return { manifestCount: manifestPaths.length, entries: canonicalEntries, sources, order, issues };
}

/** #️⃣ Hashes one bounded build artifact in chunks with containment, progress and cancellation checks. */
export function sha256CatalogArtifact(path: string, containmentRoot: string, pluginId = "catalog", artifact: CatalogArtifactProgress["artifact"] = "raw", control: CatalogArtifactControl = {}): string {
  const info = lstatSync(path);
  if (info.isSymbolicLink() || !info.isFile()) throw new Error(`${artifact} artifact must be a regular non-symlink file`);
  if (info.size > CATALOG_ARTIFACT_MAX_BYTES) throw new Error(`${artifact} artifact exceeds ${CATALOG_ARTIFACT_MAX_BYTES} bytes`);
  const realRoot = realpathSync(containmentRoot);
  const realPath = realpathSync(path);
  if (!pathIsWithin(realRoot, realPath)) throw new Error(`${artifact} artifact escapes the fresh build root`);
  const hash = createHash("sha256");
  const buffer = Buffer.allocUnsafe(CATALOG_IO_CHUNK_BYTES);
  const descriptor = openSync(path, "r");
  let bytesRead = 0;
  try {
    while (bytesRead < info.size) {
      if (control.cancelled?.()) throw new Error("catalog artifact verification cancelled");
      const length = readSync(descriptor, buffer, 0, Math.min(buffer.length, info.size - bytesRead), bytesRead);
      if (length === 0) throw new Error(`${artifact} artifact changed while hashing`);
      hash.update(buffer.subarray(0, length));
      bytesRead += length;
      control.progress?.({ pluginId, artifact, bytesRead, totalBytes: info.size });
    }
  } finally {
    closeSync(descriptor);
  }
  if (statSync(path).size !== info.size) throw new Error(`${artifact} artifact changed while hashing`);
  return hash.digest("hex");
}

function readVerifiedCatalogArtifact(path: string, containmentRoot: string, limit: number, pluginId: string, artifact: CatalogArtifactProgress["artifact"], control: CatalogArtifactControl): { readonly bytes: Uint8Array; readonly sha256: string } {
  const pathInfo = lstatSync(path);
  if (pathInfo.isSymbolicLink() || !pathInfo.isFile()) throw new Error(`${artifact} artifact must be a regular non-symlink file`);
  const realRoot = realpathSync(containmentRoot);
  const realPath = realpathSync(path);
  if (!pathIsWithin(realRoot, realPath)) throw new Error(`${artifact} artifact escapes the fresh build root`);
  const descriptor = openSync(path, "r");
  try {
    const before = fstatSync(descriptor);
    if (!before.isFile() || before.size > limit) throw new Error(`${artifact} artifact exceeds ${limit} bytes`);
    const bytes = Buffer.allocUnsafe(before.size);
    const hash = createHash("sha256");
    let bytesRead = 0;
    while (bytesRead < before.size) {
      if (control.cancelled?.()) throw new Error("catalog artifact verification cancelled");
      const length = readSync(descriptor, bytes, bytesRead, Math.min(CATALOG_IO_CHUNK_BYTES, before.size - bytesRead), bytesRead);
      if (length === 0) throw new Error(`${artifact} artifact changed while reading verified bytes`);
      hash.update(bytes.subarray(bytesRead, bytesRead + length));
      bytesRead += length;
      control.progress?.({ pluginId, artifact, bytesRead, totalBytes: before.size });
    }
    const after = fstatSync(descriptor);
    if (after.size !== before.size) throw new Error(`${artifact} artifact changed while reading verified bytes`);
    control.afterArtifact?.(artifact);
    return { bytes, sha256: hash.digest("hex") };
  } finally {
    closeSync(descriptor);
  }
}

function catalogArtifactReceipt(path: string, containmentRoot: string, relativePath: string, pluginId: string, artifact: CatalogArtifactProgress["artifact"], control: CatalogArtifactControl): CatalogFileReceipt {
  const sha256 = sha256CatalogArtifact(path, containmentRoot, pluginId, artifact, control);
  return { path: relativePath, bytes: lstatSync(path).size, sha256 };
}

/** 🧾️ Computes the exact immutable receipt a producer must publish last for one fresh row. */
export function createFreshCatalogCommitMarker(source: StrictCatalogDescriptor, buildRoot: string, control: CatalogArtifactControl = {}): FreshCatalogCommitMarker {
  const exactRoot = realpathSync(resolve(buildRoot));
  const rowRoot = join(exactRoot, source.entry.pluginId);
  const rawRelative = join("raw", source.entry.wasmOut);
  const coreRelative = join("core", source.entry.wasmOut);
  const descriptorRelative = join("descriptor", CATALOG_DESCRIPTOR_PACK_FILENAME);
  const packageId = source.entry.packageId;
  if (!CATALOG_PACKAGE_ID.test(packageId) || source.descriptor.packageId !== packageId) throw new Error(`${source.entry.pluginId}: descriptor packageId cannot identify a commit marker`);
  return {
    schemaVersion: 1,
    packageId,
    pluginId: source.entry.pluginId,
    packageName: source.entry.packageName,
    wasmOut: source.entry.wasmOut,
    raw: catalogArtifactReceipt(join(rowRoot, rawRelative), exactRoot, rawRelative, source.entry.pluginId, "raw", control),
    core: catalogArtifactReceipt(join(rowRoot, coreRelative), exactRoot, coreRelative, source.entry.pluginId, "core", control),
    descriptor: catalogArtifactReceipt(join(rowRoot, descriptorRelative), exactRoot, descriptorRelative, source.entry.pluginId, "descriptor", control),
    descriptorSha256: source.hashes.descriptorSha256,
  };
}

function requireExactCatalogRow(rowRoot: string, wasmOut: string): void {
  const expected = [CATALOG_COMMIT_MARKER_FILENAME, "core", "descriptor", "raw"].sort();
  if (!isDeepStrictEqual(readdirSync(rowRoot).sort(), expected)) throw new Error(`${rowRoot}: catalog row is not the exact committed triplet`);
  for (const [directory, filename] of [["raw", wasmOut], ["core", wasmOut], ["descriptor", CATALOG_DESCRIPTOR_PACK_FILENAME]] as const) {
    const root = join(rowRoot, directory);
    const info = lstatSync(root);
    if (info.isSymbolicLink() || !info.isDirectory() || !isDeepStrictEqual(readdirSync(root), [filename])) throw new Error(`${root}: catalog artifact directory is not exact`);
  }
}

/** 🏗️ Binds completion evidence to a caller-owned isolated root and rejects ambient target/cache authority. */
export function createFreshCatalogBuildVerifier(repoRoot: string, buildRoot: string): FreshCatalogBuildVerifier {
  if (!isAbsolute(buildRoot)) throw new Error("fresh catalog build root must be absolute");
  const resolvedRepo = resolve(repoRoot);
  const resolvedBuild = resolve(buildRoot);
  const sharedTarget = resolve(resolvedRepo, "target");
  const developmentCache = resolve(resolvedRepo, "🧰️framework", "🛍️products", "💻️os", "🔨️modules", "🧑️‍💻️dev", "🔌️plugin-modules");
  if (pathIsWithin(sharedTarget, resolvedBuild)) throw new Error("fresh catalog verification cannot use the ambient shared target");
  if (pathIsWithin(developmentCache, resolvedBuild)) throw new Error("fresh catalog verification cannot use the development cache");
  if (resolvedBuild === resolvedRepo) throw new Error("fresh catalog build root must be a dedicated directory");
  const rootInfo = lstatSync(resolvedBuild);
  if (rootInfo.isSymbolicLink() || !rootInfo.isDirectory()) throw new Error("fresh catalog build root must be a regular non-symlink directory");
  const exactRoot = realpathSync(resolvedBuild);
  return {
    root: exactRoot,
    verify(entry, control = {}) {
      const rowRoot = join(exactRoot, entry.pluginId);
      const markerPath = join(rowRoot, CATALOG_COMMIT_MARKER_FILENAME);
      if (!existsSync(markerPath)) throw new Error(`${entry.pluginId}: catalog commit marker is missing`);
      const markerBytes = readCatalogFile(markerPath, exactRoot, CATALOG_COMMIT_MARKER_MAX_BYTES);
      let marker: unknown;
      try {
        const json = Buffer.from(markerBytes).toString("utf8");
        rejectDuplicateJsonObjectNames(json);
        marker = JSON.parse(json);
      } catch (error) {
        throw new Error(`${entry.pluginId}: catalog commit marker does not decode: ${boundedCatalogDiagnostic(error)}`);
      }
      if (marker === null || typeof marker !== "object" || Array.isArray(marker) || !isStrictJsonValue(marker)) throw new Error(`${entry.pluginId}: catalog commit marker is not an object`);
      const markerRecord = marker as Record<string, unknown>;
      const markerFields = ["core", "descriptor", "descriptorSha256", "packageId", "packageName", "pluginId", "raw", "schemaVersion", "wasmOut"];
      if (!isDeepStrictEqual(Object.keys(markerRecord).sort(), markerFields)) throw new Error(`${entry.pluginId}: catalog commit marker fields are not exact`);
      if (markerRecord.schemaVersion !== 1 || markerRecord.pluginId !== entry.pluginId || markerRecord.packageId !== entry.packageId || markerRecord.packageName !== entry.packageName || markerRecord.wasmOut !== entry.wasmOut || typeof markerRecord.descriptorSha256 !== "string" || !CATALOG_SHA256.test(markerRecord.descriptorSha256)) throw new Error(`${entry.pluginId}: catalog commit marker identity is not the carried Cargo identity`);
      if (!Buffer.from(markerBytes).equals(Buffer.from(`${JSON.stringify(marker)}\n`))) throw new Error(`${entry.pluginId}: catalog commit marker is not canonical JSON`);
      requireExactCatalogRow(rowRoot, entry.wasmOut);
      const rawPath = join(rowRoot, "raw", entry.wasmOut);
      const corePath = join(rowRoot, "core", entry.wasmOut);
      const descriptorPath = join(rowRoot, "descriptor", CATALOG_DESCRIPTOR_PACK_FILENAME);
      const raw = readVerifiedCatalogArtifact(rawPath, exactRoot, CATALOG_ARTIFACT_MAX_BYTES, entry.pluginId, "raw", control);
      const core = readVerifiedCatalogArtifact(corePath, exactRoot, CATALOG_ARTIFACT_MAX_BYTES, entry.pluginId, "core", control);
      const descriptor = readVerifiedCatalogArtifact(descriptorPath, exactRoot, CATALOG_DESCRIPTOR_MAX_BYTES, entry.pluginId, "descriptor", control);
      if (raw.sha256 === core.sha256) throw new Error(`${entry.pluginId}: raw component and extracted core identities are not distinct`);
      const fileReceipt = (path: string, bytes: Uint8Array, sha256: string): CatalogFileReceipt => ({ path, bytes: bytes.byteLength, sha256 });
      const expectedRaw = fileReceipt(join("raw", entry.wasmOut), raw.bytes, raw.sha256);
      const expectedCore = fileReceipt(join("core", entry.wasmOut), core.bytes, core.sha256);
      const descriptorFileSha256 = createHash("sha256").update(descriptor.bytes).digest("hex");
      const expectedDescriptor = fileReceipt(join("descriptor", CATALOG_DESCRIPTOR_PACK_FILENAME), descriptor.bytes, descriptorFileSha256);
      if (!isDeepStrictEqual(markerRecord.raw, expectedRaw) || !isDeepStrictEqual(markerRecord.core, expectedCore) || !isDeepStrictEqual(markerRecord.descriptor, expectedDescriptor)) throw new Error(`${entry.pluginId}: catalog commit marker artifact receipts disagree with the staged row`);
      let packed: unknown;
      try {
        packed = decodePackValue(descriptor.bytes);
      } catch (error) {
        throw new Error(`${entry.pluginId}: staged descriptor pack does not decode: ${boundedCatalogDiagnostic(error)}`);
      }
      if (!Buffer.from(encodePackValue(packed)).equals(Buffer.from(descriptor.bytes))) throw new Error(`${entry.pluginId}: staged descriptor pack is not canonical`);
      const hashes = validateCatalogDescriptorValue(entry, packed);
      const blanked = structuredClone(packed as Record<string, unknown>);
      (blanked.hashes as Record<string, unknown>).descriptorSha256 = "";
      const descriptorSha256 = createHash("sha256").update(encodePackValue(blanked)).digest("hex");
      if (hashes.wasmSha256 !== raw.sha256 || hashes.coreWasmSha256 !== core.sha256 || hashes.descriptorSha256 !== descriptorSha256 || markerRecord.descriptorSha256 !== descriptorSha256) throw new Error(`${entry.pluginId}: staged descriptor identity disagrees with the committed artifacts`);
      if (!Buffer.from(readCatalogFile(markerPath, exactRoot, CATALOG_COMMIT_MARKER_MAX_BYTES)).equals(Buffer.from(markerBytes))) throw new Error(`${entry.pluginId}: catalog commit marker changed during verification`);
      return {
        pluginId: entry.pluginId,
        rawSha256: raw.sha256,
        coreSha256: core.sha256,
        descriptorSha256,
        rawBytes: raw.bytes,
        coreBytes: core.bytes,
        descriptorBytes: descriptor.bytes,
      };
    },
  };
}
//#endregion 🔖️CatalogCompleteness

/** 🛂️ Fails closed unless every discovered source and explicit fresh-root artifact verifies. */
class CatalogCompleteScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const repoRoot = getWorkspaceRoot();
    const option = (name: string): string | undefined => {
      const index = segments.indexOf(name);
      return index < 0 ? undefined : segments[index + 1];
    };
    const buildRoot = option("--build-root") ?? process.env.SEMIO_CATALOG_FRESH_BUILD_ROOT;
    const cancelFile = option("--cancel-file");
    if (!buildRoot || !isAbsolute(buildRoot)) throw new Error("usage: catalog-complete --build-root <absolute fresh build root> [--cancel-file <path>]");
    const cancelled = (): boolean => cancelFile !== undefined && existsSync(cancelFile);
    const audit = auditPluginCatalogSources(repoRoot, {
      cancelled,
      ownerDescriptors: "ignored",
      progress(completed, total, path) { console.log(`catalog-complete source ${completed}/${total}: ${path}`); },
    });
    if (audit.issues.length > 0) {
      console.error(`catalog-complete source preflight failed (${audit.issues.length} issue(s), ${audit.manifestCount} manifests):`);
      for (const issue of audit.issues) console.error(`  - [${issue.code}] ${issue.pluginId ?? issue.path}: ${issue.diagnostic}`);
      throw new Error(`catalog-complete refused unverified source catalog (${audit.issues.length} issue(s))`);
    }
    const verifier = createFreshCatalogBuildVerifier(repoRoot, buildRoot);
    const result = await executeCatalogVerificationPlan(audit.entries, {
      async verify(node) {
        const entry = audit.entries.find(({ pluginId }) => pluginId === node.pluginId);
        if (!entry) throw new Error(`${node.pluginId}: Cargo source identity is absent`);
        return verifier.verify(entry, { cancelled });
      },
    }, {
      cancelled,
      progress(event) { console.log(`catalog-complete artifact ${event.completed}/${event.total}: ${event.pluginId} ${event.status}`); },
    });
    if (result.results.some(({ status }) => status !== "verified")) {
      for (const row of result.results.filter(({ status }) => status !== "verified")) console.error(`  - [${row.status}] ${row.pluginId}: ${row.diagnostic ?? "unverified"}`);
      throw new Error("catalog-complete refused unverified fresh build artifacts");
    }
    console.log(`plugin catalog is complete: ${audit.manifestCount} source identities and fresh raw/core/descriptor artifacts verified from ${verifier.root}.`);
  }
}

/** 🔎️ Checks only bytes produced by generate, without taxonomy, asset or built-WASM diagnostics. */
class CheckGeneratedScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const { files, playgrounds } = renderCatalogFiles(repoRoot);
    const outDir = join(this.root, "🤖️generated");
    const stale = Object.entries(files).filter(([name, content]) => !existsSync(join(outDir, name)) || readFileSync(join(outDir, name), "utf8") !== content).map(([name]) => name);
    if (existsSync(outDir)) stale.push(...readdirSync(outDir).filter((name) => !(name in files)));
    const launchPath = join(repoRoot, LAUNCH_OUTPUT_REL_PATH);
    if (!existsSync(launchPath) || readFileSync(launchPath, "utf8") !== generateLaunchJson(repoRoot, playgrounds)) stale.push(LAUNCH_OUTPUT_REL_PATH);
    if (stale.length) throw new Error(`Generated registry output is stale: ${stale.join(", ")}`);
    console.log("plugin registry generated catalog and launch bytes are fresh.");
  }
}

class CheckScript extends BundleScript {
  run(_segments: string[]): void {
    const repoRoot = getWorkspaceRoot();
    const authorityProblems = validateGeneratorContractsAgainstWorkspace(repoRoot, TAXONOMY);
    if (authorityProblems.length) throw new Error(authorityProblems.join("\n"));
    const { files, entries, playgrounds, frameworkPackages } = renderCatalogFiles(repoRoot);
    const outDir = join(this.root, "🤖️generated");
    const stale = Object.entries(files)
      .filter(([name, content]) => !existsSync(join(outDir, name)) || readFileSync(join(outDir, name), "utf8") !== content)
      .map(([name]) => `generated/${name}`);
    // 🖥️ A seed/devLaunchers mismatch throws out of `generateLaunchJson`; report it as a violation
    // instead of an unhandled exception so the rest of the gate's findings still reach the dev.
    const launchViolations: string[] = [];
    try {
      const launchPath = join(repoRoot, LAUNCH_OUTPUT_REL_PATH);
      const expectedLaunch = generateLaunchJson(repoRoot, playgrounds);
      if (!existsSync(launchPath) || readFileSync(launchPath, "utf8") !== expectedLaunch) stale.push(LAUNCH_OUTPUT_REL_PATH);
    } catch (error) {
      launchViolations.push(`${LAUNCH_OUTPUT_REL_PATH} cannot be rendered: ${(error as Error).message}`);
    }
    if (stale.length > 0) {
      console.error(`plugin registry catalog is stale: ${stale.join(", ")}`);
      console.error("run `bun nx run @semio-tech/plugin-registry:generate` to refresh.");
      process.exit(1);
    }
    const newContractPluginRoots = findNewContractPluginRoots(repoRoot);
    const violations = [...launchViolations, ...validatePlaygroundRegistry(playgrounds, repoRoot), ...validatePlaygroundSessions(repoRoot)];
    if (violations.length > 0) {
      console.error("plugin registry catalog has playground validation errors:");
      for (const violation of violations) console.error(`  - ${violation}`);
      process.exit(1);
    }
    // 🗿️ Taxonomy tree audit for plugins discovered via the shared package contract. Its severity is
    // the plugin areas' declared maturity, not a hand-flipped flag: warn while any area is
    // `legacy`/`mixed` (plugins still mid-migration), hard failure once every area is declared `clean`
    // — the finalization flip is then a one-word edit in `🔣️taxonomy.json`.
    const taxonomyFindings = newContractPluginRoots.flatMap(({ pluginId, pluginRoot }) => validateTaxonomyTree(pluginRoot, pluginId));
    if (taxonomyFindings.length > 0) {
      const areaLabel = PLUGIN_AREAS.join(", ");
      if (PLUGIN_AREAS_STATE === "legacy" || PLUGIN_AREAS_STATE === "mixed") {
        console.warn(`plugin taxonomy tree findings (area(s) "${areaLabel}" is "${PLUGIN_AREAS_STATE}" — not failing the gate yet):`);
        for (const finding of taxonomyFindings) console.warn(`  - ${finding}`);
      } else {
        console.error(`plugin taxonomy tree violations (area(s) "${areaLabel}" is "${PLUGIN_AREAS_STATE}"):`);
        for (const finding of taxonomyFindings) console.error(`  - ${finding}`);
        process.exit(1);
      }
    }
    // 🧭️ Shared-discovery diagnostics: a non-empty `discoverPackageProblems` outside a
    // legacy/mixed/exempt area means a manifest lost its role marker (the failure mode that silently
    // dropped a migrated extension crate from this very catalog) or a `🎯️targets/<target>/` dir is
    // missing its manifest. Warn-only while any area is pre-`clean`.
    const discoveryProblems = discoverPackageProblems(repoRoot, TAXONOMY);
    if (discoveryProblems.length > 0) {
      console.warn("package discovery problems:");
      for (const problem of discoveryProblems) console.warn(`  - [${problem.kind}] ${problem.message}`);
    }
    // 🛂️ `📓️design-abi.md` §3's descriptor gate — see `validateDescriptors`'s own doc for the
    // warn-vs-error severity split.
    const descriptorResult = validateDescriptors(entries, repoRoot);
    if (descriptorResult.warnings.length > 0) {
      console.warn("descriptor gate warnings:");
      for (const warning of descriptorResult.warnings) console.warn(`  - ${warning}`);
    }
    if (descriptorResult.errors.length > 0) {
      console.error("descriptor gate violations:");
      for (const error of descriptorResult.errors) console.error(`  - ${error}`);
      process.exit(1);
    }
    console.log(`plugin registry catalog is fresh (${entries.length} plugin crates, ${playgrounds.length} playgrounds, ${frameworkPackages.length} framework packages); ${LAUNCH_OUTPUT_REL_PATH} is fresh.`);
  }
}

/** 🧪️ Runs the language-neutral generated-launch contract without catalog generation. */
class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await runVitest(this.root, segments, "🧪️tests/🟦️.ts");
  }
}

const router = new ScriptRouter(import.meta.dir).register("generate", GenerateScript).register("preview-generated", PreviewGeneratedScript).register("check-generated", CheckGeneratedScript).register("catalog-complete", CatalogCompleteScript).register("check", CheckScript).register("test", TestScript).register("new", NewScript);

if (import.meta.main) {
  await runBundleScriptMain(router, import.meta.url, { defaultCommand: "generate" });
}
