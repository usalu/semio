import{printArtifactNxOutputV1}from"../../../../../../📓️print/📦️artifacts/🟦️.ts";
import{checkResourceLeaseControl}from"../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {frameworkOsEngineSourceOwnersV1} from "../../../../../../💻️os/🔨️modules/🧑‍💻dev/🏗️builder/🌐️vite/📥️artifacts/🕸️engines/🟦️.ts";
import {repositoryWasmPackageNxOutputV1} from "../../../⚡️caching/📦️artifacts/🕸️wasm/📋️contract/🟦️.ts";
import {repositoryCachePartitionRootDataV1} from "../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📂️paths/🟦️.ts";
import {withHostPlatformOutputsV1} from "../../🎯️host-platform/🟦️.ts";
import {componentModelDirectoryKeyV1} from "../../../../../../💻️os/🔨️modules/🔌️plugin/📦️model/📋️contract/🟦️.ts";
import {conformsOwnerContractV1} from "../../📤️arguments/🟨️.mjs";
import {repositoryBuildGraphDataDirectoryV1,repositoryBuildGraphWriterDirectoryV1,withRepositoryBuildGraphFactStoreV1} from "../../📥️inputs/💾️storage/🟨️.mjs";
import {loadCatalogBuildProvidersV1,catalogBuildContributionForManifestV1} from "../../../../../../💻️os/🔨️modules/🔌️plugin/📇️registry/📣️publication/🏗️build/🟦️.ts";
import { mkdirSync, writeFileSync, statSync as rawStatSync } from "node:fs";
import { dirname, isAbsolute, sep, join, relative, resolve } from "node:path";
import { createRequire, isBuiltin } from "node:module";
import { createHash } from "node:crypto";
import { fileURLToPath,pathToFileURL } from "node:url";
import * as loadedCommandInputs from "../../../⚡️caching/📥️inference/🟨️.mjs";
import * as loadedProjectInputs from "../../../🏗️build/📥️inputs/🟨️.mjs";
import * as loadedBuildMode from "../../../🏗️build/🎚️mode/🟨️.mjs";
import {receiveScriptProcessInvocation,checkScriptInvocation,createIssuedResourceControlV1,requireIssuedResourceControlInvocationV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {advanceScriptInvocation} from "../../../../../../../🔨️modules/🏃️process/🧭️routing/📥️invocation/🟦️.ts";
import { buildModesV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
import {declaredNativeOwnerCommandV1,literalCommandTokensV1,bindBuildCommandV1} from "../../../🏗️build/📤️arguments/🟨️.mjs";
import {readCargoExecutionContractV1,selectCargoExecutionOperationV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🦀️execution/🟦️.ts";
import { memoizedObservedComputationV1, memoizedObservedAsyncComputationV1, memoizedFileRecord, registerSourceAnalysisV1, observedSourceAnalysisV1, observedSourceAnalysisAsyncV1, observeDependencies, observedFs, observedRequireResolve, observedTypeScriptHost, observeInference, observeProjectInferenceV1, settleObservedComputationV1 } from "../../../🏗️build/👁️observation/🟨️.mjs";
import { availableParallelism } from "node:os";

const { existsSync, lstatSync, readFileSync, readdirSync, realpathSync, statSync } = observedFs;

const PROJECT_BASENAME = "📋️project.json";
const SCRIPT_BASENAME = "📜️script.ts";
const DEFAULT_EXECUTOR = "repo:script";
const TARGET_EXECUTION_SCHEMA = "🏗️build/🏃️execution/🧬️schema/🔣️.json";
const targetExecutionSchema = JSON.parse(readFileSync(new URL(`../../../${TARGET_EXECUTION_SCHEMA}`, import.meta.url), "utf8"));

const LIBRARY_ROOT = resolve(dirname(fileURLToPath(import.meta.url)), "../../..");
const LIBRARY_URL = pathToFileURL(join(LIBRARY_ROOT,"🟨️.mjs")).href;
const RUNTIME_COMPONENT_MODULE = "🕸️dependencies/🧩️runtime/🟨️.mjs";
const NATIVE_HOST_MODULE="🎮️playground/🖥️native-host/🟨️.mjs";
const COMPONENT_DEPLOYMENT_MODULE = "📇️catalog/🚚️deployment/🟨️.mjs";
const INSTALLATION_IDENTITY_MODULE = "../../../../🔨️modules/🪪️identity/📁️installation/🟨️.mjs";
const INSTALLATION_IDENTITY_SCHEMA = "../../../../🔨️modules/🪪️identity/📁️installation/🧬️schema/🔣️.json";
const BROWSER_SESSION_MODULE = "../../../💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs";
const PLAYGROUND_COMPOSITION_MODULE = "../../../💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧩️composition/🟨️.mjs";
const SOURCE_INPUT_MODULE = "🕸️dependencies/🟦️typescript/🟨️.mjs";
const COMMAND_INPUT_MODULE = "⚡️caching/📥️inference/🟨️.mjs";
const OWNER_ARGUMENTS_MODULE = "🏗️build/📤️arguments/🟨️.mjs";
const PROJECT_INPUT_MODULE = "🏗️build/📥️inputs/🟨️.mjs";
const BUILD_MODE_MODULE = "🏗️build/🎚️mode/🟨️.mjs";
const BUILD_MODE_API = "../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎚️mode/🟦️.ts";
const OBSERVATION_MODULE = "🏗️build/👁️observation/🟨️.mjs";
const CARGO_EXECUTION_MODULE="../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🏃️execution/🟦️.ts";
const CARGO_EXECUTION_SCHEMA="../../../../🔨️modules/🏃️process/🧪️testing/🦀️cargo/🏃️execution/🧬️schema/🔣️.json";

/** 🏛️ Requires current real ancestors and a real leaf before an authority source is consumed. */
function requirePhysicalSourceV1(path) {
  const ancestors = [];
  for (let directory = dirname(path); ; directory = dirname(directory)) { ancestors.push(directory); if (directory === dirname(directory)) break; }
  for (const directory of ancestors.reverse()) { const value = lstatSync(directory); if (!value.isDirectory() || value.isSymbolicLink()) throw Error(`Build inference requires real authority ancestry: ${directory}`); }
  const value = lstatSync(path);
  if (!value.isFile() || value.isSymbolicLink()) throw Error(`Build inference requires real authority file: ${path}`);
}

/** 📄️ Reads policy and implementation authority through current real ancestors and leaf. */
function readPhysicalSource(path) {
  requirePhysicalSourceV1(path);
  return readFileSync(path);
}

/** 🗺️ The only taxonomy facts graph inference consumes, so edits elsewhere in the taxonomy keep every inference record and parser revision valid. */
function taxonomyProjectionV1(taxonomy) {
  return { testDeliveryScopeDirectoryNames: taxonomy.testDeliveryScopeDirectoryNames, generatorContracts: taxonomy.generatorContracts ?? {} };
}

const TAXONOMY_PROJECTION = registerSourceAnalysisV1({ id: `build:taxonomy-projection:v1:${createHash("sha256").update(import.meta.url).digest("hex")}`, revision: createHash("sha256").update(taxonomyProjectionV1.toString()).digest("hex"), analyze: (_path, bytes) => taxonomyProjectionV1(JSON.parse(bytes.toString("utf8"))) });

/** 🧭️ Reads the consumed taxonomy projection through physical authority ancestry as a semantic fact rather than whole-file bytes. */
function readTaxonomyProjectionV1(path) {
  requirePhysicalSourceV1(path);
  return observedSourceAnalysisV1(TAXONOMY_PROJECTION, path);
}

/** 🔁️ Evicts Bun's canonical module entry and retains Node's revision-specific ESM identity. */
async function importRevision(source, revision) {
  const require = createRequire(import.meta.url), url = new URL(source);
  delete require.cache[require.resolve(fileURLToPath(url))];
  url.searchParams.set("revision", revision);
  return import(url.href);
}

/** 🧷️ Admits only the command input parsers whose bytes are the current authority. */
function currentCommandInputs(commandInputs) {
  if (commandInputs.sourceHash !== commandInputHash) throw Error("Stale command input parser implementation");
  return commandInputs;
}

const commandInputHash = createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, COMMAND_INPUT_MODULE))).digest("hex");
let commandSourceImports = (path, source) => currentCommandInputs(loadedCommandInputs).commandSourceImports(path, source);
let moduleSourceImports = (path, source) => currentCommandInputs(loadedCommandInputs).moduleSourceImports(path, source);
let productionModuleSourceImports = (path,source) => currentCommandInputs(loadedCommandInputs).productionModuleSourceImports(path,source);
const moduleGraphPublications=new Map();
const moduleGraphPublicationKeyV1=root=>JSON.stringify([resolve(root),repositoryBuildGraphDataDirectoryV1(root,"inference")]);

let declaredBrowserSessionEnginesV1;
let playgroundCompositionPathV1;
let admitPlaygroundNativeHostV1;
let nativeHostSourceFactsV1;
let nativeHostScriptAdmissionV1;
let runtimeComponentClosure;
let runtimeInputAdmissionV1;
let readSourceInputContract;
let relativeSourceInputs;
let relativeSourceInputsAsyncV1;
let componentDeploymentDirectoryV1;
let { inferProjectInputs, compactSourceInputs, cargoLockInput, deduplicateProjectInputs } = loadedProjectInputs;
let { targetBuildModes } = loadedBuildMode;
const libraryBootstrap = (loadedCommandInputs.sourceHash === commandInputHash ? Promise.resolve(loadedCommandInputs) : importRevision(new URL(`./${COMMAND_INPUT_MODULE}`, LIBRARY_URL), commandInputHash)).then((commandInputs) => {
  const inputRevision = createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, PROJECT_INPUT_MODULE))).digest("hex");
  const modeRevision = createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, BUILD_MODE_MODULE))).digest("hex");
  return Promise.all([importRevision(new URL(`./${PROJECT_INPUT_MODULE}`, LIBRARY_URL), inputRevision), importRevision(new URL(`./${BUILD_MODE_MODULE}`, LIBRARY_URL), modeRevision)]).then(([inputs, mode]) => {
    ({ inferProjectInputs, compactSourceInputs, cargoLockInput, deduplicateProjectInputs } = inputs); ({ targetBuildModes } = mode);
    return commandInputs;
  });
}).then((commandInputs) => {
  const admitted = currentCommandInputs(commandInputs);
  commandSourceImports = admitted.commandSourceImports;
  moduleSourceImports = admitted.moduleSourceImports;
  productionModuleSourceImports = admitted.productionModuleSourceImports;
  return importRevision(new URL(`./${RUNTIME_COMPONENT_MODULE}`, LIBRARY_URL), createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, RUNTIME_COMPONENT_MODULE))).digest("hex"));
}).then((runtime) => {
  runtimeComponentClosure = runtime.runtimeComponentClosure;
  runtimeInputAdmissionV1 = runtime.runtimeInputAdmissionV1;
  return importRevision(new URL(`./${SOURCE_INPUT_MODULE}`, LIBRARY_URL), createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, SOURCE_INPUT_MODULE))).digest("hex"));
}).then((source) => {
  readSourceInputContract = source.readSourceInputContract;
  relativeSourceInputs = source.relativeSourceInputs;
  relativeSourceInputsAsyncV1 = source.relativeSourceInputsAsyncV1;
  return importRevision(new URL(`./${NATIVE_HOST_MODULE}`, LIBRARY_URL),createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT,NATIVE_HOST_MODULE))).digest("hex"));
 }).then(async nativeHost => {
  admitPlaygroundNativeHostV1 = nativeHost.admitPlaygroundNativeHostV1;
  nativeHostSourceFactsV1 = nativeHost.nativeHostSourceFactsV1;
  nativeHostScriptAdmissionV1 = nativeHost.nativeHostScriptAdmissionV1;
  const revision = createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, COMPONENT_DEPLOYMENT_MODULE))).update(readPhysicalSource(join(LIBRARY_ROOT, INSTALLATION_IDENTITY_MODULE))).update(readPhysicalSource(join(LIBRARY_ROOT, INSTALLATION_IDENTITY_SCHEMA))).digest("hex");
  const identity = await importRevision(new URL(INSTALLATION_IDENTITY_MODULE, LIBRARY_URL), revision);
  const admit = identity.installationDirectoryParserV1(JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, INSTALLATION_IDENTITY_SCHEMA)).toString("utf8")));
  const deployment = await importRevision(new URL(`./${COMPONENT_DEPLOYMENT_MODULE}`, LIBRARY_URL), revision);
  componentDeploymentDirectoryV1 = metadata => deployment.componentDeploymentDirectoryV1(metadata, admit);
  const browser = await importRevision(new URL(BROWSER_SESSION_MODULE, LIBRARY_URL), createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, BROWSER_SESSION_MODULE))).digest("hex"));
  declaredBrowserSessionEnginesV1 = browser.declaredBrowserSessionEnginesV1;
  const composition = await importRevision(new URL(PLAYGROUND_COMPOSITION_MODULE, LIBRARY_URL), createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, PLAYGROUND_COMPOSITION_MODULE))).digest("hex"));
  playgroundCompositionPathV1 = composition.playgroundCompositionPathV1;
});

const POLICY = JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, "⚡️caching/🔣️policy.json")).toString("utf8"));
/** 🗂️ The one plugin-modules root every build driver materializes component modules into. */
const componentModelOutputV1=(cratePath,mode="{options.env.SEMIO_BUILD_MODE}")=>`{workspaceRoot}/.cache/b/p/m/browser/${mode}/components/${componentModelDirectoryKeyV1(cratePath)}`;
const PLUGIN_MODULES_ROOT = ".cache/b/p/m/browser/{options.env.SEMIO_BUILD_MODE}/modules";
const WORKSPACE_OWNERSHIP = JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, "🗂️workspaces/🔣️policy.json")).toString("utf8"));
const EXAMPLE_COLLECTIONS = WORKSPACE_OWNERSHIP.collections.names;
const TAXONOMY = readTaxonomyProjectionV1(join(LIBRARY_ROOT, "🔣️taxonomy.json"));
const IMPLEMENTATION_REVISION = new URL(import.meta.url).searchParams.get("revision") ?? implementationRevision();
const buildPath = (path) => path.split("\\").join("/");
/** 🧭️ Picks an indexed dependency source file through the original portable file map. */
function buildTrackedSourceFile(context, projectName, preferred, basenameHint) {
  const { workspaceRoot } = context;
  const exists = (rel) => {
    try { return existsSync(join(workspaceRoot, rel)); } catch { return false; }
  };
  if (exists(preferred)) return preferred;
  const files = context.fileMap?.projectFileMap?.[projectName];
  const hint = basenameHint ?? preferred.split("/").pop();
  if (files?.length) {
    const exact = files.find((entry) => entry.file === preferred);
    if (exact && exists(exact.file)) return exact.file;
    const suffix = `/${hint}`;
    const matches = files.filter((entry) => (entry.file === hint || entry.file.endsWith(suffix)) && exists(entry.file));
    if (matches.length === 1) return matches[0].file;
  }
  return preferred;
}
/** 🦀️ Lists every workspace Cargo manifest through taxonomy-respecting discovery. */
function walkCargoToml(workspaceRoot) {
  const found = [];
  const walk = (directory) => {
    for (const entry of readdirSync(join(workspaceRoot, directory), { withFileTypes: true })) {
      const path = buildPath(join(directory, entry.name));
      if (entry.isSymbolicLink() || path === "compose" || path === "temp/compose" || entry.name === ".🧬semio" || POLICY.generatedDirectories.includes(entry.name)) continue;
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile() && entry.name === "Cargo.toml") found.push(path);
    }
  };
  walk("");
  return found.sort();
}

/** 🦀️ Discovers the same complete canonical manifest scope through the caller's bounded continuation. */
async function walkCargoTomlControlledV1(workspaceRoot, control) {
  const found = [], directories = [""];let visited=0;
  while(directories.length){
    control.check();if(visited%128===0)await control.advance("manifest-discovery");
    const directory=directories.pop();
    for(const entry of readdirSync(join(workspaceRoot,directory),{withFileTypes:true})){
      const path=buildPath(join(directory,entry.name));
      if(entry.isSymbolicLink()||path==="compose"||path==="temp/compose"||entry.name===".🧬semio"||POLICY.generatedDirectories.includes(entry.name))continue;
      if(entry.isDirectory())directories.push(path);else if(entry.isFile()&&entry.name==="Cargo.toml")found.push(path);
    }
    visited++;
  }
  await control.advance(`manifest-discovery:${visited}`);
  return found.sort();
}
const owned = (path, root) => root === "." || path === root || path.startsWith(`${root}/`);
const matchesCommand = (name, commands) => commands.some((command) => name === command || name.startsWith(`${command}-`));
const matchesUncached = (name, policy) => policy.uncachedExact.includes(name) || matchesCommand(name, policy.uncached);
const mutatingName = (name) => /(?:^|-)(?:clean|gc|prune|setup|fuzz)(?:-|$)/.test(name);
const liveName = (name) => /(?:^|-)(?:e2e|live)(?:-|$)/.test(name);
const verifyCommand = (target) => /(?:^|[\s"'])verify(?:[\s"']|$)/.test(target?.options?.command ?? "");
const cacheableFamily = (name) => /^(test(?:-|$)|check(?:-|$)|lint(?:-|$)|typecheck(?:-|$)|format-check(?:-|$)|generate(?:-|$)|schema(?:-|$)|verify(?:-|$)|stdio(?:-|$)|build(?:-|$)|wasm$|native-build$|package$|extension-package$|cpp-(?:configure|build|test)$|graph-check$|policy-check$|artifact-check$|artifact-package-contract$|toolchain$)/.test(name) || /(?:^|-)contract(?:-|$)/.test(name) || /^generator(?:-|$)/.test(name) && !name.includes("generator-inputs");

/** 🧶️ Models Bun's locked package locations without merging distinct dependency contexts. */
function bunLockGraph(lock, patches = {}) {
  if (![1, 2, 3].includes(lock.lockfileVersion)) throw new Error(`Unsupported Bun lockfile version ${lock.lockfileVersion}`);
  const object = (value) => value && typeof value === "object" && !Array.isArray(value);
  if (!object(lock.packages) || !object(lock.workspaces)) throw new Error("Bun lockfile requires packages and workspaces");
  const entries = new Map(), externalNodes = {}, dependencies = [], workspacePackages = new Map();
  const parts = (key) => {
    if (!key) return [];
    const result = [], segments = key.split("/");
    for (let i = 0; i < segments.length; i++) {
      const name = segments[i].startsWith("@") ? `${segments[i]}/${segments[++i] ?? ""}` : segments[i];
      if (!/^(?:@[^/@\\:]+\/)?[^/@\\:]+$/.test(name) || name.split("/").some((part) => [".", "..", "@.", "@.."].includes(part))) throw new Error(`Invalid Bun package location ${key}`);
      result.push(name);
    }
    return result;
  };
  const canonical = (value) => Array.isArray(value) ? value.map(canonical) : object(value) ? Object.fromEntries(Object.keys(value).sort().map((key) => [key, canonical(value[key])])) : value;
  for (const [key, data] of Object.entries(lock.packages)) {
    parts(key);
    if (!Array.isArray(data) || typeof data[0] !== "string") throw new Error(`Invalid Bun package ${key}`);
    const match = data[0].match(/^(@[^/]+\/[^@]+|[^@]+)@(.+)$/);
    if (!match) throw new Error(`Invalid Bun resolution ${key}`);
    const [, packageName, version] = match;
    entries.set(key, { key, packageName, version, data });
    if (version.startsWith("workspace:")) { workspacePackages.set(key, version.slice(10)); continue; }
    if (!/^\d+\.\d+\.\d+(?:[-+].+)?$/.test(version)) throw new Error(`Unsupported Bun resolution ${key}: ${version}`);
    const patch = lock.patchedDependencies?.[`${packageName}@${version}`];
    if (patch !== undefined && (typeof patch !== "string" || !Object.hasOwn(patches, patch))) throw new Error(`Missing Bun patch bytes for ${packageName}@${version}`);
    const hash = createHash("sha256").update(JSON.stringify(canonical(data)));
    if (patch !== undefined) hash.update("\0patch\0").update(patches[patch]);
    const name = `npm:${key}`;
    externalNodes[name] = { type: "npm", name, data: { packageName, version, hash: hash.digest("hex") } };
  }
  const resolveDependency = (from, name) => {
    if (parts(name).length !== 1) throw new Error(`Invalid Bun dependency name ${name}`);
    const parents = parts(from);
    for (;;) {
      const key = [...parents, name].join("/");
      if (entries.has(key)) return key;
      if (!parents.length) return undefined;
      parents.pop();
    }
  };
  const workspacePaths = [...workspacePackages].map(([key, path]) => ({ key, path })).sort((a, b) => b.path.length - a.path.length);
  const resolveImport = (file, name) => /^(?:@[^/@\\:]+\/)?[^/@\\:]+$/.test(name) ? resolveDependency(workspacePaths.find(({ path }) => file === path || file.startsWith(path + "/"))?.key ?? "", name) : undefined;
  for (const [key, entry] of entries) {
    if (workspacePackages.has(key)) continue;
    const metadata = entry.data.find((value) => object(value)) ?? {}, targets = new Set();
    for (const type of ["dependencies", "optionalDependencies", "peerDependencies"]) {
      if (metadata[type] !== undefined && !object(metadata[type])) throw new Error(`Invalid Bun ${type} for ${key}`);
      for (const name of Object.keys(metadata[type] ?? {})) {
        const target = resolveDependency(key, name);
        if (target === undefined) {
          if (type === "dependencies" && !Object.hasOwn(metadata.optionalDependencies ?? {}, name)) throw new Error(`Cannot resolve Bun dependency ${name} from ${key}`);
          continue;
        }
        if (workspacePackages.has(target)) throw new Error(`External Bun package ${key} depends on workspace ${target}`);
        targets.add(target);
      }
    }
    for (const target of [...targets].sort()) if (target !== key) dependencies.push({ source: `npm:${key}`, target: `npm:${target}`, type: "static" });
  }
  return { externalNodes, dependencies, workspacePackages, resolve: resolveDependency, resolveImport };
}

let BUN_LOCK_CACHE;

/** 🔐️ Reads locked identities and authored patch bytes without inspecting installed packages. */
function readBunLockGraph(workspaceRoot) {
  const source = readFileSync(join(workspaceRoot, "bun.lock"), "utf8"), root = realpathSync(workspaceRoot);
  const parsed = BUN_LOCK_CACHE?.root === root && BUN_LOCK_CACHE.source === source ? BUN_LOCK_CACHE.parsed : createRequire(import.meta.url)("typescript").parseConfigFileTextToJson("bun.lock", source);
  if (parsed.error) throw new Error("Invalid Bun lockfile JSON");
  const patches = {}, digest = createHash("sha256").update(source);
  for (const path of [...new Set(Object.values(parsed.config.patchedDependencies ?? {}))].sort()) {
    if (typeof path !== "string") throw new Error("Invalid Bun patch path");
    const file = realpathSync(resolve(root, path)), local = relative(root, file);
    if (isAbsolute(local) || local === ".." || buildPath(local).startsWith("../") || !statSync(file).isFile()) throw new Error(`Bun patch must be an owned file: ${path}`);
    patches[path] = readFileSync(file); digest.update("\0" + path + "\0").update(patches[path]);
  }
  const hash = digest.digest("hex");
  if (BUN_LOCK_CACHE?.root === root && BUN_LOCK_CACHE.hash === hash) return BUN_LOCK_CACHE.model;
  const model = bunLockGraph(parsed.config, patches);
  BUN_LOCK_CACHE = { root, source, parsed, hash, model };
  return model;
}

/** 🦀️ Tokenizes module syntax without interpreting strings and comments as declarations. */
function rustInputTokens(source) {
  const tokens = [], rawPattern = /(?:b|c)?r(#+)?"/y, characterPattern = /(?:b)?'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'/uy, identifierPattern = /(?:r#)?[\p{ID_Start}_][\p{ID_Continue}]*/uy;
  const match = (pattern, at) => { pattern.lastIndex = at; return pattern.exec(source); };
  for (let i = 0; i < source.length;) {
    const char = source[i], code = source.charCodeAt(i);
    if (code === 32 || code >= 9 && code <= 13 || code > 127 && /\s/u.test(char)) { i++; continue; }
    if (char === "/" && source[i + 1] === "/") { const end = source.indexOf("\n", i); i = end < 0 ? source.length : end; continue; }
    if (char === "/" && source[i + 1] === "*") {
      let depth = 1; i += 2;
      while (i < source.length && depth) {
        if (source.startsWith("/*", i)) { depth++; i += 2; }
        else if (source.startsWith("*/", i)) { depth--; i += 2; }
        else i++;
      }
      continue;
    }
    const raw = (char === "r" || char === "b" || char === "c") && match(rawPattern, i);
    if (raw) {
      const start = i + raw[0].length, delimiter = '"' + (raw[1] ?? ""), end = source.indexOf(delimiter, start);
      if (end < 0) throw new Error("Unterminated Rust raw string");
      tokens.push({ text: source.slice(start, end), string: true }); i = end + delimiter.length; continue;
    }
    if (char === '"' || (char === "b" || char === "c") && source[i + 1] === '"') {
      const start = char === '"' ? i : i + 1; i = start + 1;
      while (i < source.length && source[i] !== '"') { i += source[i] === "\\" ? 2 : 1; }
      const literal = source.slice(start, ++i);
      try { tokens.push({ text: JSON.parse(literal), string: true }); }
      catch { tokens.push({ text: literal.slice(1, -1), string: true }); }
      continue;
    }
    const character = (char === "'" || char === "b") && match(characterPattern, i);
    if (character) { tokens.push({ text: character[0], string: true }); i += character[0].length; continue; }
    const identifier = (code > 127 || char === "_" || code >= 65 && code <= 90 || code >= 97 && code <= 122) && match(identifierPattern, i);
    if (identifier) { tokens.push({ text: identifier[0], identifier: true }); i += identifier[0].length; continue; }
    tokens.push({ text: source[i++] });
  }
  return tokens;
}

/** 🧾️ Retains only module mounts and include expressions, independently of a file's location. */
function rustSourceReferences(source) {
  const tokens = rustInputTokens(source), pairs = new Map(), stack = [], includes = [];
  for (let i = 0; i < tokens.length; i++) {
    if (tokens[i].string) continue;
    if (["(", "[", "{"].includes(tokens[i].text)) stack.push(i);
    else if ([")", "]", "}"].includes(tokens[i].text)) { const open = stack.pop(); if (open !== undefined) pairs.set(open, i); }
  }
  const literal = (start, end) => {
    if (tokens[start]?.string && start + 1 === end) return tokens[start].text;
    if (tokens[start]?.text === "env" && tokens[start + 1]?.text === "!" && tokens[start + 3]?.text === "CARGO_MANIFEST_DIR") return { manifest: true };
    if (tokens[start]?.text === "concat" && tokens[start + 1]?.text === "!") {
      const parts = []; let from = start + 3;
      for (let i = from; i < end - 1; i++) {
        if (pairs.has(i)) { i = pairs.get(i); continue; }
        if (tokens[i].text === ",") { parts.push(literal(from, i)); from = i + 1; }
      }
      if (from < end - 1) parts.push(literal(from, end - 1));
      return parts;
    }
    throw new Error("Dynamic Rust include requires explicit source inputs");
  };
  for (let i = 0; i < tokens.length - 3; i++) {
    if (!tokens[i].identifier || !["include", "include_str", "include_bytes"].includes(tokens[i].text) || tokens[i + 1].text !== "!") continue;
    const end = pairs.get(i + 2);
    if (end !== undefined) includes.push(literal(i + 3, tokens[end - 1]?.text === "," ? end - 1 : end));
  }
  const scope = (start, end) => {
    const modules = [];
    for (let i = start; i < end;) {
      let path;
      while (tokens[i]?.text === "#") {
        const open = tokens[i + 1]?.text === "!" ? i + 2 : i + 1, close = pairs.get(open);
        if (close === undefined) break;
        if (tokens[open + 1]?.text === "cfg_attr" && tokens.slice(open + 2, close).some((token) => token.text === "path")) throw new Error("Conditional Rust mount requires conservative source inputs");
        if (tokens[open + 1]?.text === "path" && tokens[open + 2]?.text === "=" && tokens[open + 3]?.string) path = tokens[open + 3].text;
        i = close + 1;
      }
      if (tokens[i]?.text === "pub") { i++; if (tokens[i]?.text === "(") i = (pairs.get(i) ?? i) + 1; }
      if (tokens[i]?.text === "mod" && tokens[i + 1]?.identifier) {
        const name = tokens[i + 1].text.replace(/^r#/, ""), boundary = i + 2;
        if (tokens[boundary]?.text === "{") {
          const close = pairs.get(boundary);
          if (close !== undefined) { modules.push({ name, path, modules: scope(boundary + 1, close) }); i = close + 1; continue; }
        }
        if (tokens[boundary]?.text === ";") { modules.push({ name, path }); i = boundary + 1; continue; }
      }
      const macro = tokens[i]?.text === "macro_rules";
      while (i < end && tokens[i].text !== ";" && tokens[i].text !== "{") { if (pairs.has(i)) i = pairs.get(i); i++; }
      if (macro && tokens.slice(i, pairs.get(i) ?? i).some((token) => !token.string && token.text === "mod")) throw new Error("Generated Rust modules require conservative source inputs");
      i = (pairs.get(i) ?? i) + 1;
    }
    return modules;
  };
  return { includes, modules: scope(0, tokens.length) };
}

/** 🪶️ Bounds parsed source facts by serialized bytes plus per-entry bookkeeping. */
function createRustSourceCache(limit = 16 * 1024 * 1024) {
  if (!Number.isSafeInteger(limit) || limit < 1) throw new Error("Rust source cache requires a positive byte budget");
  return { entries: new Map(), bytes: 0, limit, hits: 0, misses: 0 };
}
const RUST_SOURCE_CACHE = createRustSourceCache();
const RUST_REFERENCE_NAMESPACE = `rust-source-references-v1:${createHash("sha256").update(rustInputTokens.toString()).update(rustSourceReferences.toString()).digest("hex")}`;
const RUST_INPUT_SNAPSHOTS = new WeakMap();
let sourceAnalysesV1;

/** 🪪️ Names the exact evaluated module URL and its originally bound implementation revision. */
function sourceAnalysisOwnerIdentityV1(url,revision){return createHash("sha256").update(JSON.stringify([url,revision])).digest("hex");}

/** 🧬️ Binds structural parsers to their evaluated implementation and compiler bytes. */
export function prepareSourceAnalysesV1(){
  if(!sourceAnalysesV1){
    const commandParser=commandSourceImports,moduleParser=moduleSourceImports,productionParser=productionModuleSourceImports,compiler=observedRequireResolve(fileURLToPath(import.meta.url),"typescript"),revision=createHash("sha256").update(IMPLEMENTATION_REVISION).update(commandInputHash).update(readPhysicalSource(compiler)).digest("hex");
    const owner=sourceAnalysisOwnerIdentityV1(import.meta.url,IMPLEMENTATION_REVISION),identity=role=>`build:${role}:v1:${owner}`;
    sourceAnalysesV1={
      command:registerSourceAnalysisV1({id:identity("command-imports"),revision,analyze:(path,bytes)=>commandParser(path,bytes.toString("utf8"))}),
      module:registerSourceAnalysisV1({id:identity("module-imports"),revision,analyze:(path,bytes)=>({imports:moduleParser(path,bytes.toString("utf8")),productionImports:productionParser(path,bytes.toString("utf8"))})}),
      script:registerSourceAnalysisV1({id:identity("script-requirements"),revision:IMPLEMENTATION_REVISION,analyze:(_path,bytes)=>{const text=bytes.toString("utf8");return {go:/["']go["']\s*,|\brunGo\w*\(/.test(text),wasm:/\b(?:buildWasmWebV1|buildRepositoryWasmWebV1)\s*\(/.test(text)};}}),
      nativeScript:registerSourceAnalysisV1({id:identity("native-script-admission"),revision:IMPLEMENTATION_REVISION,analyze:(_path,bytes)=>nativeHostScriptAdmissionV1(bytes.toString("utf8"))}),
      rust:registerSourceAnalysisV1({id:identity("rust-references"),revision:createHash("sha256").update(IMPLEMENTATION_REVISION).update(RUST_REFERENCE_NAMESPACE).digest("hex"),analyze:(_path,bytes)=>boundedRustReferences(bytes,RUST_SOURCE_CACHE)})
    };
  }
  return sourceAnalysesV1;
}


/** ♻️ Content-addressed discovery never trusts source timestamps or retains token streams. */
function cachedRustReferences(file, cache, snapshots) {
  return memoizedObservedComputationV1(snapshots,file,()=>{
  const references = observedSourceAnalysisV1(prepareSourceAnalysesV1().rust,file);
  if(cache!==RUST_SOURCE_CACHE){
    const digest=createHash("sha256").update(JSON.stringify(references)).digest("hex"),previous=cache.entries.get(digest);
    if(previous){cache.hits++;cache.entries.delete(digest);cache.entries.set(digest,previous);}
    else{cache.misses++;const bytes=Buffer.byteLength(JSON.stringify(references))+192;if(bytes<=cache.limit){while(cache.bytes+bytes>cache.limit){const oldest=cache.entries.keys().next().value;cache.bytes-=cache.entries.get(oldest).bytes;cache.entries.delete(oldest);}cache.entries.set(digest,{references,bytes});cache.bytes+=bytes;}}
  }
  return references;
  });
}

/** 🪶️ Parses one source's references through the bounded in-process cache keyed by its content digest. */
function boundedRustReferences(source, cache) {
  const digest = createHash("sha256").update(source).digest("hex");
  let entry = cache.entries.get(digest);
  if (entry) { cache.hits++; cache.entries.delete(digest); cache.entries.set(digest, entry); }
  else {
    cache.misses++;
    let references;
    try { references = rustSourceReferences(source.toString("utf8")); }
    catch (error) { references = { error: error.message }; }
    entry = { references, bytes: Buffer.byteLength(JSON.stringify(references)) + 192 };
    if (entry.bytes <= cache.limit) {
      while (cache.bytes + entry.bytes > cache.limit) {
        const key = cache.entries.keys().next().value;
        cache.bytes -= cache.entries.get(key).bytes;
        cache.entries.delete(key);
      }
      cache.entries.set(digest, entry); cache.bytes += entry.bytes;
    }
  }
  return entry.references;
}

/** 📥️ Resolves Cargo entry points and compact source references without running a compiler. */
function rustSourceFiles(entries, manifestRoot, cache = RUST_SOURCE_CACHE, snapshots = new Map()) {
  let closures = RUST_INPUT_SNAPSHOTS.get(snapshots);
  if (!closures) { closures = new Map(); RUST_INPUT_SNAPSHOTS.set(snapshots, closures); }
  const key = JSON.stringify([manifestRoot, entries.map(entry => resolve(entry))]);
  return [...memoizedObservedComputationV1(closures,key,()=>{
  const files = new Set(), visited = new Set();
  const literal = (value) => typeof value === "string" ? value : Array.isArray(value) ? value.map(literal).join("") : manifestRoot;
  const visit = (file, moduleBase = dirname(file)) => {
    files.add(file);
    const identity = file + "\0" + moduleBase;
    if (!existsSync(file) || visited.has(identity)) return;
    visited.add(identity);
    if (!file.endsWith(".rs")) return;
    const references = cachedRustReferences(file, cache, snapshots);
    if (references.error) throw new Error(references.error + ": " + file);
    for (const expression of references.includes) visit(resolve(dirname(file), literal(expression)));
    const scope = (modules, base, fileScope) => {
      for (const { name, path, modules: children } of modules) {
        if (children) { scope(children, resolve(base, path ?? name), false); continue; }
        const candidates = path === undefined ? [join(base, name + ".rs"), join(base, name, "mod.rs")] : [resolve(fileScope ? dirname(file) : base, path)];
        if (!candidates.some(existsSync)) for (const child of candidates) files.add(child);
        for (const child of candidates) if (existsSync(child)) visit(child, path === undefined && buildPath(child).endsWith("/" + name + ".rs") ? join(dirname(child), name) : dirname(child));
      }
    };
    scope(references.modules, moduleBase, true);
  };
  for (const entry of entries) visit(resolve(entry));
  const result = [...files].sort();
  return result;
  })];
}

/** 🧭️ Cargo owns compilation while the build graph tracks mounted sources and embedded assets. */
function cargoSourceInputs(root, workspaceRoot, facts, includeTests = true) {
  const directory = join(workspaceRoot, root), manifest = join(directory, "Cargo.toml");
  if (!existsSync(manifest)) return undefined;
  const cargo = readToml(manifest), entries = [cargo.lib?.path ?? "src/lib.rs", "src/main.rs", ...(cargo.bin ?? []).map((target) => target.path), ...(includeTests ? [...(cargo.test ?? []), ...(cargo.bench ?? []), ...(cargo.example ?? [])].map((target) => target.path) : []), ...(cargo.package?.build === false ? [] : [typeof cargo.package?.build === "string" ? cargo.package.build : "build.rs"])].filter(Boolean).map((path) => resolve(directory, path)).filter(existsSync);
  if (cargo.package?.build !== false && existsSync(resolve(directory, typeof cargo.package?.build === "string" ? cargo.package.build : "build.rs"))) return undefined;
  for (const source of ["src/bin", ...(includeTests ? ["tests", "examples", "benches"] : [])]) {
    if (!existsSync(join(directory, source))) continue;
    for (const entry of readdirSync(join(directory, source), { withFileTypes: true })) {
      const path = join(directory, source, entry.name, ...(entry.isDirectory() ? ["main.rs"] : []));
      if (path.endsWith(".rs") && existsSync(path)) entries.push(path);
    }
  }
  if (!entries.length) return undefined;
  try { return rustSourceFiles(entries, directory, RUST_SOURCE_CACHE, facts).map((file) => `{workspaceRoot}/${buildPath(relative(workspaceRoot, file))}`); }
  catch { return undefined; }
}

/** 🗂️ Shares source reads and command closures only within one graph construction. */
export function createScriptInputCache(context={}) {
  if(context.invocation){requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkResourceLeaseControl(context.phaseControl);}
  return { files: new Map(), closures: new Map(), commandDependencies: new Map(), modules:new Set(), resources:new Set(),context };
}

/** 🧿️ Keeps a logical import's interpretation separate from its physical file identity. */
function buildImportResourceV1(containing,specifier){
 if(!specifier.startsWith("."))return {specifier,suffix:"",source:true};
 const url=new URL(specifier,pathToFileURL(containing)),query=url.searchParams;
 return {specifier:fileURLToPath(url),suffix:url.search+url.hash,source:!query.has("raw")&&(!query.has("url")||query.has("worker")||query.has("sharedworker"))};
}

/** 🧭️ Tracks literal relative command imports through the existing TypeScript tooling boundary. */
async function relativeScriptInputs(entries, workspaceRoot, cache = createScriptInputCache(), requiredSources=false) {
  const roots = [...new Set(entries.map(entry => resolve(entry)))].sort(), key = JSON.stringify([resolve(workspaceRoot), roots,requiredSources]);
  let steps=0;const checkpoint=(force=false)=>yieldInferenceV1(cache.context,"build:command-source-imports",++steps,force);
  return memoizedObservedAsyncComputationV1(cache.closures,key,async()=>{
  const compiler = createRequire(import.meta.url)("typescript"), host = observedTypeScriptHost(compiler.sys), files = new Set(),compiled=new Set();
  const visit = async(path,source=true) => {
    await checkpoint();
    if (buildPath(relative(workspaceRoot, path)).startsWith("../")) throw new Error(`Command import escapes workspace: ${path}`);
    const seen=files.has(path);
    files.add(path);
    if(requiredSources)readPhysicalSource(path);
    else if (!existsSync(path)) return;
    if(!source||!/\.[cm]?[jt]sx?$/u.test(path)){cache.resources.add(path);if(!seen)readPhysicalSource(path);return;}
    if(compiled.has(path))return;
    compiled.add(path);cache.modules.add(path);
    const imports = await memoizedObservedAsyncComputationV1(cache.files,JSON.stringify([path,requiredSources]),async()=>{
      const imports = [];
      for (const entry of observedSourceAnalysisV1(prepareSourceAnalysesV1().command,path)) {
        await checkpoint();
        const resource=buildImportResourceV1(path,entry);
        let resolved;
        try { resolved = observedRequireResolve(path, resource.specifier); }
        catch (error) {
          if (error.code !== "MODULE_NOT_FOUND") throw error;
          resolved = compiler.resolveModuleName(resource.specifier, path, { moduleResolution: compiler.ModuleResolutionKind.Bundler, allowJs: true, resolveJsonModule: true }, host).resolvedModule?.resolvedFileName;
          if (!resolved) {if(requiredSources||resource.suffix||!/\.[cm]?[jt]sx?$/u.test(resource.specifier))throw error;continue;}
          if (/\.d\.[cm]?ts$/.test(resolved)) throw error;
        }
        imports.push({path:resolved,source:resource.source});
      }
      return imports;
    },checkpoint);
    for (const imported of imports) await visit(imported.path,imported.source);
  };
  for (const entry of roots) await visit(entry);
  const result = [...files].map((file) => `{workspaceRoot}/${buildPath(relative(workspaceRoot, file))}`).sort();
  return result;
  },checkpoint);
}

/** 🔧️ Native leaves hash their compiler contract and exact command implementation, independently of UI selection. */
async function nativeCommandInputs(workspaceRoot, scripts) {
  const script = join(LIBRARY_ROOT, "⚡️caching/🦀️cargo/📜️script.ts");
  const cargo = POLICY.toolchains.cargo, javascript = POLICY.toolchains.javascript;
  return [
    ...await relativeScriptInputs([script], workspaceRoot, scripts),
    ...[...javascript.files.filter((file) => !["package.json", "bun.lock"].includes(file)), ...cargo.files].map((file) => `{workspaceRoot}/${file}`),
    { externalDependencies: ["@iarna/toml"] },
    ...cargo.environment.map((env) => ({ env })),
    ...[...javascript.commands, ...cargo.commands].map((runtime) => ({ runtime })),
    { runtime: 'node -p "process.platform.concat(process.arch)"' },
  ];
}

/** 🧭️ Captures outer scripts and explicitly declared child scripts in their own working directories. */
async function targetScriptSources(target, workspaceRoot, scripts, projectRoot = ".",nativeManifest) {
 const entries=new Set();let fallback=false;
 const retain=bound=>{if(bound.script){readPhysicalSource(bound.script);entries.add(bound.script);}else fallback=true;};
 const local=path=>{const value=buildPath(relative(workspaceRoot,path));if(value===".."||value.startsWith("../")||isAbsolute(value))throw Error("Declared command Source escapes workspace");return value||".";};
 for(const options of targetCommandRecipes(target)){
  const commands=typeof options.command==="string"?[options.command]:options.commands??[];if(!commands.length)fallback=true;
  for(const operation of commands){
   const command=typeof operation==="string"?operation:operation.command,arguments_=options.forwardAllArgs!==false&&(typeof operation==="string"||operation.forwardAllArgs!==false)?options.args??[]:[],bound=bindBuildCommandV1(command,workspaceRoot,options.cwd??".",projectRoot,arguments_);retain(bound);
   const owner=options.nativeOwnerCommand,selected=owner?bindBuildCommandV1([owner.program,...owner.arguments].map(value=>JSON.stringify(value)).join(" "),workspaceRoot,owner.workingDirectory,projectRoot,arguments_):bound;if(owner)retain(selected);
   const explicitManifest=owner?.manifest??nativeManifest,manifest=resolve(workspaceRoot,explicitManifest??join(projectRoot,"Cargo.toml"));local(manifest);if(explicitManifest===undefined&&!existsSync(manifest))continue;
   const cargo=readToml(manifest),reference=cargo.package?.metadata?.semio?.["cargo-execution"];if(reference===undefined)continue;if(typeof reference!=="string"||!reference||isAbsolute(reference))throw Error("Relative Cargo execution declaration required");
   const declarationPath=resolve(dirname(manifest),reference);local(declarationPath);readPhysicalSource(manifest);const declaration=readCargoExecutionContractV1("declaration",JSON.parse(readPhysicalSource(declarationPath).toString("utf8")));entries.add(manifest);entries.add(declarationPath);
   const selectedOperation=selectCargoExecutionOperationV1(declaration,{program:selected.program,cwd:owner?.workingDirectory??options.cwd??".",arguments:selected.arguments},workspaceRoot);if(!selectedOperation?.continuation)continue;
   const child=selectedOperation.continuation,childBound=bindBuildCommandV1([child.program,...child.arguments].map(value=>JSON.stringify(value)).join(" "),workspaceRoot,child.cwd,projectRoot,[]);if(!childBound.script)throw Error("Declared Cargo continuation requires an explicit script Source");retain(childBound);
  }
 }
 return {closure:entries.size?await relativeScriptInputs([...entries],workspaceRoot,scripts,true):undefined,fallback};
}

/** ⚙️ Keeps configured recipe inheritance identical for command and lock source discovery. */
function targetCommandRecipes(target) {
  return [target.options ?? {}, ...Object.values(target.configurations ?? {}).map(options => ({ ...target.options, ...options }))];
}

/** 🗂️ Keeps configured executable imports in the target's invariant source closure. */
async function targetScriptClosure(target, workspaceRoot, scripts, projectRoot = ".") {
  return (await targetScriptSources(target,workspaceRoot,scripts,projectRoot)).closure;
}

/** 🔐️ `cargo metadata --locked` validates the shared lock against every workspace manifest, so its replay must hash all of them. */
function nativeLockInputs(command) {
  if(command && typeof command === "object")return [...new Set(targetCommandRecipes(command).flatMap(options=>(typeof options.command==="string"?[options.command]:options.commands??[]).flatMap(operation=>nativeLockInputs(typeof operation==="string"?operation:operation.command))))];
  return typeof command === "string" && command.includes(" native cargo metadata ") ? ["{workspaceRoot}/**/Cargo.toml", "{workspaceRoot}/**/Cargo.lock"] : [];
}

/** 🧭️ Adds the selected native target's local router and executable import closure. */
async function nativeTargetCommandInputs(target, workspaceRoot, fallback, scripts, projectRoot = ".") {
  fallback??=await nativeCommandInputs(workspaceRoot,scripts);
  const closure = await targetScriptClosure(target, workspaceRoot, scripts, projectRoot);
  return closure ? [...new Set([...closure, ...fallback])] : fallback;
}

/** 🎯️ Hashes any script-backed target's own command closure, exactly, instead of the whole router. */
async function genericTargetCommandInputs(target, workspaceRoot, fallback, scripts = createScriptInputCache(), projectRoot = ".") {
  const sources = await targetScriptSources(target, workspaceRoot, scripts, projectRoot),closure=sources.closure;
  if (!closure) return [...fallback, "{workspaceRoot}/package.json", "{workspaceRoot}/bun.lock"];
  const key=JSON.stringify([closure,sources.fallback?fallback:null]);
  let index=0;const checkpoint=(force=false)=>yieldInferenceV1(scripts.context,"build:command-source-packages",++index,force);
  return memoizedObservedAsyncComputationV1(scripts.commandDependencies,key,async()=>{
  const packages = new Set(), graph = existsSync(join(workspaceRoot, "bun.lock")) ? readBunLockGraph(workspaceRoot) : undefined;
  for (const input of closure) {
    await checkpoint();
    const file = resolve(workspaceRoot, input.slice("{workspaceRoot}/".length));
    if (!existsSync(file)) continue;
    for (const specifier of observedSourceAnalysisV1(prepareSourceAnalysesV1().module,file).imports) {
      if (specifier.startsWith(".") || specifier.startsWith("/") || isBuiltin(specifier) || specifier.startsWith("bun:")) continue;
      const name = specifier.startsWith("@") ? specifier.split("/").slice(0, 2).join("/") : specifier.split("/")[0];
      if (!graph) { packages.add(name); continue; }
      const key = graph.resolveImport(buildPath(relative(workspaceRoot, file)), name);
      if (key && graph.externalNodes[`npm:${key}`]) packages.add(`npm:${key}`);
    }
  }
  const inputs = [...new Set([...closure,...(sources.fallback?[...fallback,"{workspaceRoot}/package.json","{workspaceRoot}/bun.lock"]:[])]), { externalDependencies: [...packages].sort() }];
  return inputs;
  },checkpoint);
}

/** 🪢️ The previous blanket router contract, kept only as a correctness fallback for a command naming no script. */
function genericCommandFallbackInputs(workspaceRoot) {
  const runner = buildPath(relative(workspaceRoot, LIBRARY_ROOT));
  return [
    "{workspaceRoot}/📜️script.ts",
    `{workspaceRoot}/${runner}/**/*.{ts,tsx,js,mjs,cjs,json}`,
    `{workspaceRoot}/${runner}/🟨️.mjs`,
    `{workspaceRoot}/${runner}/⚡️caching/**/*`,
  ];
}

/** 🧬️ A generator contract's own declared source inputs, plus the input-discovery fingerprint dependency it authors, if any. */
function generatorContractInputs(contract) {
  const fingerprint = contract.inputDiscovery ? POLICY.generatorInputs[contract.inputDiscovery.kind] : undefined;
  if (contract.inputDiscovery && !fingerprint) throw new Error(`Generator input discovery has no fingerprint owner: ${contract.inputDiscovery.kind}`);
  return {
    inputs: [...contract.inputPatterns.map((path) => `{workspaceRoot}/${path}`), ...(fingerprint ? [{ dependentTasksOutputFiles: fingerprint.output }] : [])],
    dependsOn: fingerprint ? [fingerprint.target] : [],
  };
}

/** 🧬️ Assigns each output to its physical Build producer while retaining the generator's complete entry point. */
function generatorOutputOwners(contracts, root, project, targets) {
  const outputs = new Map();
  for (const contract of Object.values(contracts)) {
    if (contract.ownership !== "owned") continue;
    if (contract.ownerPath === root) outputs.set(contract.target, outputs.get(contract.target) ?? []);
    for (const output of contract.outputRoots) {
      const owner = output.producer ?? contract;
      if (owner.ownerPath !== root) continue;
      if (!owner.target?.startsWith(`${project}:`) || !targets[owner.target.slice(project.length + 1)]) throw new Error(`Generator output has no Build producer: ${output.path}`);
      if (!outputs.has(owner.target)) outputs.set(owner.target, []);
      outputs.get(owner.target).push(`{workspaceRoot}/${output.path}`);
    }
  }
  return outputs;
}

/** 🌲️ Reads one root's current bytes into a stable digest, independently of directory-vs-file shape or a missing path. */
const OUTPUT_ROOT_DIGEST_SCRIPT = 'const c=require("crypto"),f=require("fs"),p=require("path"),r=process.argv[1],h=c.createHash("sha256");(function walk(x){if(!f.existsSync(x))return;const s=f.lstatSync(x);if(s.isDirectory())for(const e of f.readdirSync(x).sort())walk(p.join(x,e));else if(s.isFile()){h.update(p.relative(r,x).split(p.sep).join("/")+"\\0");h.update(f.readFileSync(x));}})(r);process.stdout.write(h.digest("hex"));';

/**
 * 🧮️ A committed generated output root as a live positive input. Nx's own fileset hashing only ever
 * sees files its native git-based walker can see, which silently excludes anything `.gitignore` covers —
 * exactly every `outputRoots` entry with `inclusion: "ignored"`. A `runtime` input instead spawns a fresh
 * read of the current bytes at every hash computation, so it is immune to that exclusion in either direction.
 */
function outputRootInputs(output, workspaceRoot) {
  const absolute = resolve(workspaceRoot, output.path);
  return { runtime: `node -e ${JSON.stringify(OUTPUT_ROOT_DIGEST_SCRIPT)} ${JSON.stringify(absolute)}` };
}

/** 📍️ Resolves a declared `{projectRoot}`/`{workspaceRoot}` output string to its absolute path, or `undefined` for an unrecognized shape. */
function resolveOutputPath(output, root, workspaceRoot) {
  if (!output.startsWith("{projectRoot}/") && !output.startsWith("{workspaceRoot}/")) return undefined;
  return resolve(workspaceRoot, output.replace("{projectRoot}", root).replace("{workspaceRoot}", "."));
}

/**
 * 🪞 A `check-*` target verifying a same-project `generate-*` producer's bytes still needs those CURRENT
 * bytes to invalidate its own cache, whether or not it also `dependsOn` the producer — `dependsOn` alone only
 * orders execution here (every authored instance in this repo names its target fully qualified, even for a
 * same-project producer) and never by itself feeds the producer's bytes into the hash; only a paired
 * `dependentTasksOutputFiles` input would, and this repo does not author that pairing for hand-written
 * `check-*`/`generate-*` pairs. `projectInputs` excludes every declared output from every named bucket
 * project-wide (`default`, `nativeSources`, …) to stop a producer from hashing its own freshly-written
 * output — but a proven Nx contract (see the `⚡️caching/🔁️verification/📋️orchestration/🟦️.ts` `CacheVerifyScript` fixture family) is
 * that once a bucket carrying that exclusion is also referenced, a plain positive glob for the same exact
 * path added elsewhere in the same `inputs` array is suppressed too; only a `runtime` digest (immune to
 * fileset inclusion/exclusion entirely) reliably restores visibility. Reused here uniformly for tracked and
 * gitignored outputs alike, exactly like `outputRootInputs`; deduped against whatever digest a generator
 * contract's own `checkTarget` wiring already added for the same absolute path.
 */
function generatorOutputCouplingInputs(name, target, targets, root, workspaceRoot) {
  if (!/^check(?:-|$)/.test(name)) return [];
  const existing = new Set((target.inputs ?? []).filter((entry) => typeof entry === "object" && typeof entry?.runtime === "string").map((entry) => entry.runtime));
  const inputs = [];
  for (const [sibling, siblingTarget] of Object.entries(targets)) {
    if (sibling === name || !/^generate(?:-|$)/.test(sibling)) continue;
    for (const output of siblingTarget.outputs ?? []) {
      const absolute = resolveOutputPath(output, root, workspaceRoot);
      if (!absolute) continue;
      const runtime = `node -e ${JSON.stringify(OUTPUT_ROOT_DIGEST_SCRIPT)} ${JSON.stringify(absolute)}`;
      if (!existing.has(runtime)) inputs.push({ runtime });
    }
  }
  return inputs;
}

/** 🛡️ Side effects and live processes cannot be replayed as completed task results. */
function targetPolicy(name, target, policy = POLICY) {
  if (matchesCommand(name, policy.continuous)) return { ...target, cache: false, continuous: true };
  if (target.cache === false) return { inputs: ["default"], outputs: [], ...target, cache: false };
  if (matchesUncached(name, policy)) return { ...target, cache: false };
  if (policy.cachedExact.includes(name)) return { inputs: ["default", "^default"], outputs: [], ...target, cache: true };
  if (mutatingName(name) || liveName(name)) return { ...target, cache: false };
  if (cacheableFamily(name) || verifyCommand(target)) return { inputs: ["default", "^default"], outputs: [], ...target, cache: true };
  return { ...target, cache: target.cache !== false };
}

let tomlParsing;

/** 🧾️ The tooling TOML boundary exposes only repository-owned plain records, memoized per parser version (resolved on first use). */
function readToml(path) {
  tomlParsing ??= ((require) => ({ namespace: `iarna-toml-parse-v1:${require("@iarna/toml/package.json").version}`, parse: require("@iarna/toml").parse }))(createRequire(import.meta.url));
  return memoizedFileRecord(tomlParsing.namespace, path, (bytes) => {
    const source=bytes.toString("utf8");
    try{return tomlParsing.parse(source);}catch(cause){
      const integer=value=>Number.isInteger(value)&&value>=0?value:null,tomlObservation={version:1,path:resolve(path),namespace:tomlParsing.namespace,sha256:createHash("sha256").update(bytes).digest("hex"),bytes:bytes.byteLength,line:integer(cause?.line),column:integer(cause?.col),position:integer(cause?.pos),sourceLine:(source.split(/\r?\n/u)[integer(cause?.line)??0]??"").slice(0,512)};
      const error=Object.assign(new Error("Observed TOML parse refused: "+JSON.stringify(tomlObservation),{cause}),{code:"BUILD_SOURCE_UNPARSEABLE",source:tomlObservation.path,sha256:tomlObservation.sha256});error.tomlObservation=tomlObservation;console.error("[DEBUG] "+error.message);throw error;
    }
  });
}

/** 🦀️ Resolves path dependencies, including aliases, target tables and workspace inheritance. */
function nativeDependencies(manifest, workspace) {
  const result = [];
  for (const table of [manifest, ...Object.values(manifest.target ?? {})]) {
    for (const kind of ["dependencies", "build-dependencies", "dev-dependencies"]) {
      for (const [alias, declaration] of Object.entries(table[kind] ?? {})) {
        if (typeof declaration !== "object") continue;
        const inherited = declaration.workspace === true;
        const dependency = inherited ? workspace.dependencies?.[alias] : declaration;
        if (dependency?.path) result.push({ name: dependency.package ?? alias, path: dependency.path, workspace: inherited, kind });
      }
    }
  }
  return result;
}

/** 🦀️ Resolves each crate's own Cargo workspace before interpreting inherited dependency paths. */
function nativeWorkspaceV1(directory,workspaceRoot,manifest=readToml(join(directory,"Cargo.toml"))){
 let root=manifest.package?.workspace?resolve(directory,manifest.package.workspace):directory;
 for(;;){
  const local=relative(workspaceRoot,root);if(isAbsolute(local)||local===".."||local.startsWith(".."+sep))throw Error(`Native workspace escapes repository: ${directory}`);
  const path=join(root,"Cargo.toml");if(existsSync(path)){const workspace=readToml(path).workspace;if(workspace)return {root,declaration:workspace};}
  if(root===workspaceRoot)return {root,declaration:{}};
  root=dirname(root);
 }
}

const NATIVE_DIRECTORY_CLOSURES = new WeakMap();

/** 🦀️ Resolves Cargo's local compilation inputs, admitting development dependencies only at the selected test root. Every non-root crate's
 * closure is shared across roots of one graph construction; a cyclic path graph falls back to one visited-set traversal. */
function nativeDependencyRoots(root, workspaceRoot, tests = false, cache = new Map(), closures = new Map()) {
  const key = `${root}\0${tests}`;
  return memoizedObservedComputationV1(closures,key,()=>{
  const read = path => memoizedObservedComputationV1(cache,path,()=>readToml(path));
  const children = function* (directory, includeTests) {
    const manifestPath = join(directory, "Cargo.toml");
    if (!existsSync(manifestPath)) { const error = new Error(`Native dependency has no Cargo manifest: ${manifestPath}`); error.code = "NATIVE_INPUT_ADMISSION"; error.manifest=manifestPath; throw error; }
    const manifest = read(manifestPath);
    let workspace = directory;
    if (manifest.package?.workspace) workspace = resolve(directory, manifest.package.workspace);
    else while (workspace !== workspaceRoot && !read(join(workspace, "Cargo.toml"))?.workspace) {
      workspace = dirname(workspace);
      while (workspace !== workspaceRoot && !existsSync(join(workspace, "Cargo.toml"))) workspace = dirname(workspace);
      if (buildPath(relative(workspaceRoot, workspace)).startsWith("../")) throw new Error(`Native workspace escapes repository: ${directory}`);
    }
    if (buildPath(relative(workspaceRoot, workspace)).startsWith("../")) throw new Error(`Native workspace escapes repository: ${directory}`);
    const authority = existsSync(join(workspace, "Cargo.toml")) ? read(join(workspace, "Cargo.toml")).workspace ?? {} : {};
    for (const dependency of nativeDependencies(manifest, authority)) {
      if (dependency.kind === "dev-dependencies" && !includeTests) continue;
      const child = resolve(dependency.workspace ? workspace : directory, dependency.path);
      if (buildPath(relative(workspaceRoot, child)).startsWith("../")) throw new Error(`Native dependency escapes repository: ${child}`);
      yield child;
    }
  };
  let shared = NATIVE_DIRECTORY_CLOSURES.get(closures);
  if (!shared) NATIVE_DIRECTORY_CLOSURES.set(closures, shared = new Map());
  const closure = (directory, includeTests, active) => {
    if (active.has(directory)) return null;
    const compute=()=>{
    active.add(directory);
    const members = new Set([directory]);
    for (const child of children(directory, includeTests)) {
      const reached = closure(child, false, active);
      if (reached === null) return null;
      for (const member of reached) members.add(member);
    }
    active.delete(directory);
    return members;
    };
    return includeTests?compute():memoizedObservedComputationV1(shared,directory,compute);
  };
  let visited = closure(resolve(workspaceRoot, root), tests, new Set());
  if (visited === null) {
    visited = new Set();
    const visit = (directory, includeTests) => {
      if (visited.has(directory)) return;
      visited.add(directory);
      for (const child of children(directory, includeTests)) visit(child, false);
    };
    visit(resolve(workspaceRoot, root), tests);
  }
  const result = [...visited].map((path) => buildPath(relative(workspaceRoot, path)) || ".").sort();
  return result;
  });
}

/** 👁️ Projects actual Cargo/Nx compilation and owner generator inputs for live source watching. */
export async function nativeSourceWatchPlanV1(root, workspaceRoot) {
  await libraryBootstrap;
  const roots = nativeDependencyRoots(root, workspaceRoot), includes = new Set(), excludes = new Set(), files = new Set();
  const taxonomyPath = join(workspaceRoot, buildPath(relative(workspaceRoot, join(LIBRARY_ROOT, "🔣️taxonomy.json"))));
  const contracts = existsSync(taxonomyPath) ? readTaxonomyProjectionV1(taxonomyPath).generatorContracts : {};
  for (const nativeRoot of roots) {
    const manifest = readToml(join(workspaceRoot, nativeRoot, "Cargo.toml"));
    let owner = resolve(workspaceRoot, nativeRoot);
    if (manifest.package?.workspace) owner = resolve(owner, manifest.package.workspace);
    else while (owner !== workspaceRoot && (!existsSync(join(owner, "Cargo.toml")) || !readToml(join(owner, "Cargo.toml")).workspace)) owner = dirname(owner);
    for (const name of ["Cargo.toml", "Cargo.lock"]) {
      const file = join(owner, name), local = buildPath(relative(workspaceRoot, file));
      if (local.startsWith("../")) throw Error("Native watch workspace authority escapes repository");
      includes.add(local);
    }
    for (const path of POLICY.toolchains.cargo.files) includes.add(path);
    const projectPath = join(workspaceRoot, nativeRoot, PROJECT_BASENAME);
    const project = existsSync(projectPath) ? JSON.parse(readFileSync(projectPath, "utf8")) : {};
    for (const path of cargoSourceInputs(nativeRoot, workspaceRoot, new Map(), false) ?? []) files.add(path.replace(/^\{workspaceRoot\}\//u, ""));
    const declarations = await projectInputs(project, nativeRoot, workspaceRoot, new Map(), createScriptInputCache(), "source");
    const visit = (values, seen = new Set()) => {
      for (const value of values) {
        if (typeof value !== "string") continue;
        if (declarations[value]) {
          if (seen.has(value)) throw Error("Native watch named input cycle: " + value);
          visit(declarations[value], new Set([...seen, value])); continue;
        }
        const negative = value.startsWith("!"), path = (negative ? value.slice(1) : value).replaceAll("{projectRoot}", nativeRoot).replace(/^\{workspaceRoot\}\//u, "");
        if (path.includes("{") && /\{(?:workspaceRoot|projectRoot)\}/u.test(path)) throw Error("Unresolved native watch source coordinate");
        if (isAbsolute(path) || path.split("/").includes("..") || path.includes("\\")) throw Error("Native watch source input escapes workspace");
        (negative ? excludes : includes).add(path);
      }
    };
    visit(declarations.nativeSources);
  }
  for (const contract of Object.values(contracts)) if (contract.nativeConsumers?.some(path => roots.includes(path))) {
    for (const pattern of contract.inputPatterns ?? []) (pattern.startsWith("!") ? excludes : includes).add(pattern.replace(/^!/u, ""));
  }
  if (!includes.size) throw Error("Native source watch requires admitted compilation inputs");
  const ordered = values => [...values].sort((a, b) => Buffer.from(a).compare(Buffer.from(b)));
  return { schema: "semio.framework.os.plugin.source-watch-plan/v1", includes: ordered(includes), excludes: ordered(excludes), files: ordered(files) };
}

/** 🧬️ Selects declared generators across Cargo's compilation closure without scheduling native dependencies twice. */
/** 🏃️ Admits one declared source or native command owner against its closed production contract. */
function targetExecutionKindV1(target, identity) {
  const value = target.metadata?.semio?.execution;
  if (value === undefined) return undefined;
  const schema = Object.values(targetExecutionSchema.definitions).find(schema => schema.properties.kind.const === value?.kind);
  if (!schema || !conformsOwnerContractV1(schema,value)) throw Error(`Repository target execution contract refused${identity ? `: ${identity}` : ""}`);
  return value.kind;
}

/** 🎯️ Declares the execution service owned by one generated command factory. */
function executionTargetV1(kind, target, scope) {
  const execution = { version: 1, kind, ...(scope === undefined ? {} : { scope }) }, declared = { ...target, metadata: { ...target.metadata, semio: { ...target.metadata?.semio, ...(kind === "orchestration" ? { orchestrator: true } : {}), execution } } };
  targetExecutionKindV1(declared);
  return declared;
}

function nativePreparation(root, workspaceRoot, contracts, tests = false, cache = new Map(), closures = new Map()) {
  const roots = new Set(nativeDependencyRoots(root, workspaceRoot, tests, cache, closures));
  return Object.values(contracts).filter((contract) => {
    if (!contract.nativeConsumers?.some((path) => roots.has(path))) return false;
    if (contract.ownership !== "owned" || !contract.target) throw new Error(`Native prerequisite needs an owned generator: ${root}`);
    return true;
  }).sort((a, b) => a.target.localeCompare(b.target));
}

/** 🏗️ Attaches generation to native leaves in the outer task graph, including transitive Cargo consumers. */
function withNativePreparation(project, workspaceRoot, contracts, cache = new Map(), projectsByRoot = new Map(), closures = new Map()) {
  const nativeRoot = project.metadata?.nativeRoot ?? project.root, manifest = join(workspaceRoot, nativeRoot, "Cargo.toml");
  if (!existsSync(manifest) || !readToml(manifest).package) return project;
  const generatorTargets = new Set(Object.values(contracts).flatMap((contract) => [contract.target, contract.previewTarget, contract.checkTarget]));
  const plans = new Map();
  for (const [name, target] of Object.entries(project.targets)) {
    const execution = targetExecutionKindV1(target);
    if (execution === "source" || execution === undefined && !/^(?:build|check|lint|test|wasm|native|component|describe|extension-package|package|font-tool|bench)(?:-|$)/.test(name) || generatorTargets.has(`${project.name}:${name}`)) continue;
    const tests = execution === "native" ? target.metadata.semio.execution.scope === "test" : execution === "orchestration" ? false : /^(?:test|bench)(?:-|$)/.test(name);
    if (!plans.has(tests)) {
      try { plans.set(tests, nativePreparation(nativeRoot, workspaceRoot, contracts, tests, cache, closures)); }
      catch (error) {
        if (error.code !== "NATIVE_INPUT_ADMISSION") throw error;
        const admission = "native-input-admission";
        project.metadata = { ...project.metadata, nativeInputAdmission: { manifest: buildPath(relative(workspaceRoot, manifest)), diagnostic: error.message } };
        project.targets[admission] = executionTargetV1("source", { executor: "repo:script", cache: false, options: { cwd: ".", command: `bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📜️script.ts native-input-check --manifest ${JSON.stringify(buildPath(relative(workspaceRoot, manifest)))}` } });
        plans.set(tests, { admission });
      }
    }
    if (plans.get(tests).admission) { const name = `${project.name}:${plans.get(tests).admission}`; target.dependsOn = [...(target.dependsOn ?? []), name]; continue; }
    const selected = plans.get(tests);
    if (projectsByRoot.size && target.inputs?.some((input) => input === "^nativeSources" || input === "^nativeTestSources")) {
      const dependencies = nativeDependencyRoots(nativeRoot, workspaceRoot, tests, cache, closures).filter((root) => root !== nativeRoot).map((root) => {
        const owner = projectsByRoot.get(root);
        if (!owner){const error=new Error(`Native dependency has no build project owner: ${root} (required by ${project.name}:${name}; candidates=${projectsByRoot.size})`);error.code="BUILD_SOURCE_AUTHORITY";throw error;}
        return owner;
      }).sort();
      target.inputs = target.inputs.flatMap((input) => input === "^nativeSources" || input === "^nativeTestSources" ? dependencies.length ? [{ input: "nativeSources", projects: dependencies }] : [] : [input]);
    }
    if (!selected.length) continue;
    target.dependsOn = [...(target.dependsOn ?? []), ...selected.map((contract) => contract.target).filter((name) => !target.dependsOn?.includes(name))];
    target.inputs = [...(target.inputs ?? [tests ? "default" : "production", tests ? "^default" : "^production"]), { dependentTasksOutputFiles: "**/*", transitive: true }];
  }
  return project;
}

/** 🐹️ Reads module identities and replacement paths without executing Go during graph construction. */
function goDependencies(text) {
  const result = { module: "", requires: [], replacements: [] };
  let block;
  for (const source of text.split("\n")) {
    const tokens = source.replace(/\/\/.*$/, "").trim().match(/"[^"]*"|\S+/g)?.map((value) => value.replace(/^"|"$/g, "")) ?? [];
    if (!tokens.length) continue;
    if (tokens[0] === ")") { block = undefined; continue; }
    if (tokens[1] === "(") { block = tokens[0]; continue; }
    const kind = block ?? tokens.shift();
    if (kind === "module") result.module = tokens[0];
    if (kind === "require") result.requires.push(tokens[0]);
    if (kind === "replace" && tokens.includes("=>")) result.replacements.push({ module: tokens[0], path: tokens[tokens.indexOf("=>") + 1] });
  }
  return result;
}

/** 🗺️ Resolves a language's source owner for package directories and their sibling domain sources. */
function goManifest(root, workspaceRoot) {
  const owner = root.includes("/📦️packages/") ? root.split("/📦️packages/")[0] : root;
  const script = join(workspaceRoot, root, SCRIPT_BASENAME);
  const goScript = root.includes("🐹️go") || (existsSync(script) && observedSourceAnalysisV1(prepareSourceAnalysesV1().script,script).go);
  const local = join(workspaceRoot, root, "go.mod");
  if (existsSync(local)) return local;
  const manifest = join(workspaceRoot, owner, "go.mod");
  return goScript && existsSync(manifest) ? manifest : undefined;
}

/** 🧬️ Derives named task inputs from explicit relative source-graph contracts. */
async function declaredSourceInputs(json,workspaceRoot,context={}) {
  const groups={};
  for(const[name,path]of Object.entries(json.metadata?.sourceInputs??{})){
    if(["default","production","nativeSources","nativeTestSources","artifactSources","artifactCommandSources"].includes(name)||Object.hasOwn(json.namedInputs??{},name))throw Error(`Duplicate source input group ${name}`);
    if(typeof path!=="string"||isAbsolute(path)||path.split(/[\\/]/).includes(".."))throw Error(`Invalid source input contract path ${path}`);
    const contractPath=resolve(workspaceRoot,path),contract=readSourceInputContract(contractPath),sources=await relativeSourceInputsAsyncV1(contract,workspaceRoot,()=>yieldInferenceV1(context,"build:source-contracts",0));
    groups[name]=[...new Set([contractPath,join(LIBRARY_ROOT,SOURCE_INPUT_MODULE),join(LIBRARY_ROOT,"🕸️dependencies/🟦️typescript/🔣️schema.json"),...sources.files])].map(file=>`{workspaceRoot}/${buildPath(relative(workspaceRoot,file))}`).concat([{externalDependencies:sources.externalDependencies}]);
  }
  return groups;
}

/**
 * 🚧️ Nx plans a task's `!{workspaceRoot}/…` negations against the workspace positives of its expanded inputs; with none, a
 * negation plans every other workspace file. A native source list names its files literally, so without a workspace positive
 * (a workspace-root crate) it keeps only its project-rooted negations.
 * @see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/⚡️workspace-negation-closure/🟦️.ts
 */
function workspaceNegationClosed(inputs) {
  return inputs.some((input) => typeof input === "string" && input.startsWith("{workspaceRoot}/")) ? inputs : inputs.filter((input) => typeof input !== "string" || !input.startsWith("!{workspaceRoot}/"));
}

/** 🧫️ Exact example ancestry preserves a genuine module name and refuses nested collections. */
function exampleCollectionPath(path) {
  const parts = buildPath(path).split("/");
  return parts.some((part, index) => EXAMPLE_COLLECTIONS.includes(part) && parts[index - 1] !== WORKSPACE_OWNERSHIP.collections.moduleMember);
}

/** 🧮️ Nx minimatch exclusions carry the same exact collection and module ancestry. */
function exampleCollectionInputs(root) {
  const names = `{${EXAMPLE_COLLECTIONS.join(",")}}`;
  return [`!${root}/${names}/**/*`, `!${root}/**/!(${WORKSPACE_OWNERSHIP.collections.moduleMember})/${names}/**/*`];
}

/** 📥️ Shared command implementation and host identity are inputs of every script-backed task. */
async function projectInputs(json, root, workspaceRoot, facts, scripts=createScriptInputCache(), sourceRepresentation = "hash", sourceGroups) {
  return inferProjectInputs(json, root, workspaceRoot, facts, scripts, { POLICY, TAXONOMY, LIBRARY_ROOT, SCRIPT_BASENAME, cargoSourceInputs, goManifest, buildPath, relativeScriptInputs, resolveOutputPath, exampleCollectionInputs, workspaceNegationClosed, declaredSourceInputs:sourceGroups===undefined?(json,root)=>declaredSourceInputs(json,root,scripts.context):()=>sourceGroups, owned, sourceRepresentation });
}

/**
 * 🎚️ Applies the workspace target convention: every target runs through
 * {@link DEFAULT_EXECUTOR}, and a project that owns a `📜️script.ts` runs from
 * its own root, so a `📋️project.json` only spells out what actually differs.
 * Projects without one keep the workspace-root default and thereby delegate to
 * the root `📜️script.ts`.
 * @param {Record<string, any>} target
 * @param {string} root
 * @param {boolean} ownsScript
 */
function targetWithDefaults(target, root, ownsScript) {
  if(target.command!==undefined){const {command,...rest}=target;if(typeof command!=="string"||target.options?.command!==undefined||target.options?.commands!==undefined)throw Error("Build command shorthand requires one command form");target={...rest,options:{...target.options,command}};}
  const executor = target.executor ?? DEFAULT_EXECUTOR;
  if (executor !== DEFAULT_EXECUTOR) return { ...target, executor };
  if (!ownsScript) return { ...target, executor };
  return { ...target, executor, options: { cwd: root, ...(target.options ?? {}) } };
}


/**
 * @param {Record<string, any>} json
 * @param {string} root
 * @param {string} projectDir
 */
async function projectWithDefaults(json, root, projectDir, workspaceRoot, contracts = {}, facts, commandInputs, scripts=createScriptInputCache(), sourceGroups, sourceRepresentation = "hash", selectedTargets) {
  const ownsScript = existsSync(join(projectDir, SCRIPT_BASENAME));
  const nativeRoot = json.metadata?.nativeRoot ?? root;
  if (json.metadata?.nativeRoot && (typeof nativeRoot !== "string" || isAbsolute(nativeRoot) || nativeRoot.split(/[\\/]/).some(part => ["", ".", ".."].includes(part)) || !existsSync(join(workspaceRoot, nativeRoot, "Cargo.toml")))) throw new Error(`Invalid native package root for ${json.name}: ${nativeRoot}`);
  const nativeProject = existsSync(join(workspaceRoot, nativeRoot, "Cargo.toml"));
  const artifactTypeScript = json.tags?.includes("role:artifact") && json.tags.includes("language:typescript");
  const declared = withWasmTooling({ ...(root === "." && ownsScript ? rootCommandTargets(join(projectDir, SCRIPT_BASENAME)) : {}), ...await cargoTargets(root, workspaceRoot, commandInputs,scripts), ...json.targets, ...componentBuildTargets(root, workspaceRoot), ...await printDocumentTargets(json, root, workspaceRoot, scripts) }, ownsScript && json.targets?.wasm ? observedSourceAnalysisV1(prepareSourceAnalysesV1().script,join(projectDir,SCRIPT_BASENAME)).wasm : false);
  for (const contract of Object.values(contracts)) {
    if (contract.ownership !== "owned" || contract.ownerPath !== root) continue;
    const name = contract.target.slice(contract.target.lastIndexOf(":") + 1), target = declared[name];
    if (!target || contract.target !== `${json.name}:${name}`) throw new Error(`Generator target has no project owner: ${contract.target}`);
    const discovery = generatorContractInputs(contract);
    const producers = [...new Set(contract.outputRoots.flatMap((output) => output.producer && output.producer.target !== contract.target ? [output.producer.target] : []))].sort();
    const inputs = [...(target.inputs ?? ["default", "^default"]), ...discovery.inputs, ...(producers.length ? [{ dependentTasksOutputFiles: "**/*" }] : [])];
    const dependencies = [...discovery.dependsOn, ...producers];
    declared[name] = { ...target, inputs, ...(dependencies.length ? { dependsOn: [...new Set([...(target.dependsOn ?? []), ...dependencies])] } : {}) };
  }
  for (const [target, outputs] of generatorOutputOwners(contracts, root, json.name, declared)) declared[target.slice(json.name.length + 1)] = { ...declared[target.slice(json.name.length + 1)], outputs };
  const admitted=selectedTargets===undefined?declared:Object.fromEntries(Object.entries(declared).filter(([name])=>selectedTargets.includes(name)));
  const normalized = {};
  const genericFallback = genericCommandFallbackInputs(workspaceRoot);
  const commandSourceGroups = new Map();
  const internCommandSources = (inputs) => {
    if (!Array.isArray(inputs) || !inputs.length) return [];
    const key = JSON.stringify(inputs);
    let group = commandSourceGroups.get(key);
    if (!group) {
      group = { name: `commandSources${commandSourceGroups.size}`, inputs };
      commandSourceGroups.set(key, group);
    }
    return [group.name];
  };
  for (const [name, target] of Object.entries(admitted)) {
    const inheritedBase=targetWithDefaults({ ...(POLICY.targetDefaults?.[name] ?? {}), ...target,...(target.cache===false&&target.inputs===undefined?{inputs:["default"]}:{}) }, root, ownsScript);
    const inherited=inheritedBase.metadata?.semio?.hostPlatformOutputsV1?withHostPlatformOutputsV1(inheritedBase,process.platform):inheritedBase;
    if(typeof inherited.options?.cwd==="string")inherited.options={...inherited.options,cwd:inherited.options.cwd.replaceAll("{projectRoot}",root).replaceAll("{workspaceRoot}",workspaceRoot)};
    const policy = targetPolicy(name,inherited);
    const execution = targetExecutionKindV1(target,`${json.name}:${name}`), orchestrator = execution === "orchestration" || target.metadata?.semio?.orchestrator === true;
    if (execution === "native" && !nativeProject) throw Error(`Native execution contract requires a native project owner: ${json.name}:${name}`);
    const nativePolicyTarget = execution !== "source" && !orchestrator && nativeProject && (execution === "native" || (await nativeTargetCommandInputs({...policy,configurations:undefined},workspaceRoot,commandInputs,scripts,root)).some(path=>path.includes("/🏃️process/🧪️testing/🦀️cargo/")||path.includes("/🏃️process/📦️artifacts/🏗️native-build/")||path.includes("/🏃️process/📦️artifacts/🕸️wasm-build/")));
    const nativeTarget = execution !== "source" && !orchestrator && (nativePolicyTarget || nativeProject && /^(?:build|wasm|native|test|lint|check|verify)(?:-|$)/.test(name) || policy.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts"));
    const nativeTests = execution === "native" ? target.metadata.semio.execution.scope === "test" : name.startsWith("test");
    const artifactTarget = artifactTypeScript && /^(?:build|check|test)$/.test(name);
    if(nativePolicyTarget){
      const command=policy.options?.command;
      if(typeof command!=="string"||!command.startsWith("bun "))throw Error(`Cargo owner test requires one Bun script command: ${json.name}:${name}`);
      const driver=buildPath(relative(workspaceRoot,join(LIBRARY_ROOT,"⚡️caching/🦀️cargo/📜️script.ts")));
      const ownerCwd=policy.options?.cwd??root;
      const boundCommand=bindBuildCommandV1(command,workspaceRoot,ownerCwd,root,[]).command;
      policy.executor="repo:script";
      let nativeOwnerCommand;try{nativeOwnerCommand=declaredNativeOwnerCommandV1(`${nativeRoot}/Cargo.toml`,ownerCwd,boundCommand);}catch(error){throw new Error(`Original discovered native command owner ${json.name}:${name}: ${error.message}`,{cause:error});}
      policy.options={...policy.options,ownerArguments:true,cwd:".",nativeOwnerCommand,command:`bun ${JSON.stringify(driver)} native owner-command --manifest ${JSON.stringify(`${nativeRoot}/Cargo.toml`)} --cwd ${JSON.stringify(ownerCwd)} -- ${boundCommand}`};
      const manifest=readToml(join(workspaceRoot,nativeRoot,"Cargo.toml"));let scope=resolve(workspaceRoot,nativeRoot);
      if(manifest.package?.workspace)scope=resolve(scope,manifest.package.workspace);
      else while(scope!==workspaceRoot && (!existsSync(join(scope,"Cargo.toml"))||!readToml(join(scope,"Cargo.toml")).workspace))scope=dirname(scope);
      const config=buildPath(relative(workspaceRoot,join(scope,".config","nextest.toml")));
      if(config.startsWith("../"))throw Error(`Cargo test policy escapes workspace: ${nativeRoot}`);
      policy.cargoTestPolicyInputs=[`{workspaceRoot}/${config}`];
    }
    if (execution !== "source" && !orchestrator && !/ native owner-command /u.test(policy.options?.command??"") && (await targetScriptClosure({...policy,configurations:undefined}, workspaceRoot, scripts, root))?.some(path=>path.includes("/🏃️process/🧪️testing/🧪️vitest/")||path.includes("/🏃️process/📋️context/"))) {
      const command=policy.options?.command,ownerCwd=policy.options?.cwd??root;
      if(typeof command!=="string"||!command.startsWith("bun "))throw Error(`Process owner requires one Bun script command: ${json.name}:${name}`);
      const driver=buildPath(relative(workspaceRoot,join(LIBRARY_ROOT,"🏃️process/🧭️routing/📥️owner-command/📜️script.ts")));
      const bound=command.replace(/^bun\s+("[^"\n]+"|'[^'\n]+'|[^\s]+)/u,(_,source)=>`bun ${JSON.stringify(buildPath(resolve(workspaceRoot,ownerCwd,source.replace(/^["']|["']$/g,""))))}`);
      policy.executor="repo:script";
      policy.options={...policy.options,ownerArguments:true,cwd:".",command:`bun ${JSON.stringify(driver)} owner-command --cwd ${JSON.stringify(ownerCwd)} -- ${bound}`};
    }
    if (nativeTarget) {
      policy.inputs = [nativeTests ? "nativeTestSources" : "nativeSources", nativeTests ? "^nativeTestSources" : "^nativeSources", ...internCommandSources(await nativeTargetCommandInputs(policy, workspaceRoot, commandInputs, scripts, root)), ...nativeLockInputs(policy),...(policy.cargoTestPolicyInputs??[])];
      delete policy.cargoTestPolicyInputs;
    }
    if (nativeTarget && nativeProject && readToml(join(workspaceRoot, nativeRoot, "Cargo.toml")).package) {
      const lockOwner = join(LIBRARY_ROOT, "🏗️build/📥️inputs/🔒️lock/📜️script.ts");
      policy.inputs.push(...await relativeScriptInputs([lockOwner], workspaceRoot, scripts), `{workspaceRoot}/${buildPath(relative(workspaceRoot, join(dirname(lockOwner), "🔣️schema.json")))}`, cargoLockInput(`${nativeRoot}/Cargo.toml`, workspaceRoot));
    }
    if (artifactTarget) policy.inputs = [...new Set([...(name === "build" ? [] : ["default", "^default"]), "artifactSources", "artifactCommandSources", ...(target.inputs ?? [])])];
    if (!nativeTarget && !artifactTarget && policy.cache) policy.inputs = [...(policy.inputs ?? ["default", "^default"]), ...internCommandSources(await genericTargetCommandInputs(policy, workspaceRoot, genericFallback, scripts, root))];
    if (artifactTarget && ["build", "test"].includes(name)) { policy.cache = false; policy.outputs = ["{projectRoot}/📤️dist/{options.env.SEMIO_BUILD_MODE}", "{projectRoot}/dist"]; }
    if(policy.metadata?.semio?.wasmPackageSourceV1){policy.cache=false;policy.outputs=[...new Set([repositoryWasmPackageNxOutputV1(workspaceRoot,repositoryCachePartitionRootDataV1(workspaceRoot,"products"),policy.metadata.semio.wasmPackageSourceV1),...(target.outputs??[])])];}
    else if (nativeProject && json.name !== "@semio-tech/framework-renderer-wgpu" && /^wasm(?:-release)?$/.test(name)) { policy.cache = false; policy.outputs = ["{projectRoot}/dist/wasm/{options.env.SEMIO_BUILD_MODE}"]; }
    if (POLICY.serialTargets?.includes(name)) policy.parallelism = false;
    normalized[name] = targetBuildModes(name, policy);
  }
  for (const contract of Object.values(contracts)) {
    if (contract.ownership !== "owned" || contract.ownerPath !== root || !contract.checkTarget) continue;
    const check = contract.checkTarget.slice(contract.checkTarget.lastIndexOf(":") + 1), target = normalized[check];
    if (!target) continue;
    const discovery = generatorContractInputs(contract);
    normalized[check] = { ...target, cache: target.cache !== false, inputs: [...(target.inputs ?? ["default", "^default"]), ...discovery.inputs, ...contract.outputRoots.map((output) => outputRootInputs(output, workspaceRoot))], ...(discovery.dependsOn.length ? { dependsOn: [...new Set([...(target.dependsOn ?? []), ...discovery.dependsOn])] } : {}) };
  }
  for (const [name, target] of Object.entries(normalized)) {
    if (target.cache !== true) continue;
    const extra = generatorOutputCouplingInputs(name, target, declared, root, workspaceRoot);
    if (extra.length) normalized[name] = { ...target, inputs: [...target.inputs, ...extra] };
  }
  const namedInputs = await projectInputs({ ...json, targets: admitted }, root, workspaceRoot, facts, scripts,sourceRepresentation,sourceGroups);
  for (const { name, inputs } of commandSourceGroups.values()) {
    if (Object.hasOwn(namedInputs, name)) throw new Error(`Duplicate command source group ${name}`);
    namedInputs[name] = sourceRepresentation === "source" ? deduplicateProjectInputs(inputs, root) : compactSourceInputs(inputs, workspaceRoot, root);
  }
  return { ...json, name: json.name, root, namedInputs, targets: normalized };
}

/** 🔎️ Queries source-project normalization through the same production stages without publishing graph facts. */
export async function inferReadOnlyBuildProjectSourcesV1(json, root, workspaceRoot, invocation,phaseControl) {
  requireIssuedResourceControlInvocationV1(phaseControl,invocation);checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  await libraryBootstrap;checkResourceLeaseControl(phaseControl);
  if(existsSync(join(workspaceRoot,json.metadata?.nativeRoot??root,"Cargo.toml")))throw Error("Read-only source project normalization requires a non-Cargo owner");
  await advanceScriptInvocation(invocation,"build:project-sources","running");
  const taxonomy=join(workspaceRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"),contracts=existsSync(taxonomy)?readTaxonomyProjectionV1(taxonomy).generatorContracts:{};
  const result=await projectWithDefaults(json,root,join(workspaceRoot,root),workspaceRoot,contracts,new Map(),undefined,createScriptInputCache({workspaceRoot,invocation,phaseControl}),undefined,"source");
  checkResourceLeaseControl(phaseControl);checkScriptInvocation(invocation);
  return result;
}

/** 🛠️ Exposes wasm-pack's immutable optimizer preparation to Nx before compiler execution. */
function withWasmTooling(targets, required) {
  if (!targets.wasm || !required) return targets;
  return { ...targets, wasm: { ...targets.wasm, options: { ...targets.wasm.options, env: { ...targets.wasm.options?.env, SEMIO_WASM_BUILD_REQUIRED: "1" } }, dependsOn: [...new Set([...(targets.wasm.dependsOn ?? []), "workspace:deps-wasm-opt"])] } };
}

/** 📄️ Projects a document catalog into separately owned PDF tasks without executing a compiler. */
async function printDocumentTargets(project, root, workspaceRoot, scripts) {
  const path = project.metadata?.printCatalog;
  if (!path) return {};
  const catalog = JSON.parse(readFileSync(join(workspaceRoot, path), "utf8")), compiler = dirname(dirname(path));
  if (catalog.version !== 1 || !Array.isArray(catalog.librarySources) || !catalog.librarySources.length || !Array.isArray(catalog.documents) || !catalog.documents.length) throw new Error(`Invalid Print catalog: ${path}`);
  const product = dirname(dirname(compiler)), script = `${compiler}/${SCRIPT_BASENAME}`, targets = {}, ids = new Set(), sources = new Set();
  const commandInputs = await relativeScriptInputs([join(workspaceRoot, script)], workspaceRoot, scripts);
  const libraryInputs = catalog.librarySources.map(source => {
    if (!source || source.includes("\\") || source.startsWith("/") || source.split("/").some(part => [".", "..", ""].includes(part))) throw new Error(`Invalid Print library source: ${source}`);
    const absolute = `${product}/${source}`;
    return `{workspaceRoot}/${absolute}${statSync(join(workspaceRoot, absolute)).isDirectory() ? "/**/*" : ""}`;
  });
  const inputs = [...commandInputs, ...project.targets.fonts.inputs, `{workspaceRoot}/${path}`, `{workspaceRoot}/${compiler}/🔧️toolchain/**/*`, `{workspaceRoot}/${compiler}/📚️bundle/🔒️dependencies.json`, ...libraryInputs, `{workspaceRoot}/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json`, "{workspaceRoot}/bunfig.toml", { externalDependencies: ["pdfjs-dist", "sharp"] }, { runtime: "bun --version" }, { runtime: 'node -p "process.platform.concat(process.arch)"' }];
  for (const document of catalog.documents) {
    if (!/^[a-z]+(?:-[a-z0-9]+)*$/.test(document.id) || ids.has(document.id) || sources.has(document.texPath) || !["templates", "visualizations"].includes(document.collection)) throw new Error(`Invalid Print document owner: ${document.id}`);
    ids.add(document.id); sources.add(document.texPath);
    const paths = [...new Set([document.texPath, ...document.sources])].map(path => {
      if (!path || path.includes("\\") || path.startsWith("/") || path.split("/").some(part => [".", "..", ""].includes(part))) throw new Error(`Invalid Print source: ${path}`);
      const source = `${product}/${path}`;
      return `{workspaceRoot}/${source}${statSync(join(workspaceRoot, source)).isDirectory() ? "/**/*" : ""}`;
    });
    targets[`build-${document.id}`] = executionTargetV1("source", { executor: DEFAULT_EXECUTOR, cache: true, outputs: [printArtifactNxOutputV1(workspaceRoot,{version:1,kind:"document",mode:"prototype",namespace:"print",id:document.id})], inputs: [...inputs, ...paths], dependsOn: ["fonts", "deps-tectonic", "deps-tex", "generate"], options: { cwd: root, command: `bun ${buildPath(relative(root, script))} build ${document.id}`, forwardAllArgs: false } });
    targets[`watch-${document.id}`] = executionTargetV1("source", { executor: DEFAULT_EXECUTOR, cache: false, continuous: true, outputs: [], dependsOn: [`build-${document.id}`], options: { cwd: root, command: `bun ${buildPath(relative(root, script))} watch`, forwardAllArgs: false } });
  }
  for (const [name, collection] of [["build", "templates"], ["build-viz", "visualizations"]]) targets[name] = executionTargetV1("source", { ...project.targets[name], cache: false, outputs: [], dependsOn: catalog.documents.filter(document => document.collection === collection).map(document => `build-${document.id}`) });
  return targets;
}

/** 🔨️ Every concrete Cargo package exposes native leaves even when its authored metadata only defines generators. `build` queues on the Cargo build lease and may overlap other work; `check` and `test` run Cargo directly and stay `parallelism: false`. */
async function cargoTargets(root, workspaceRoot, commandInputs,scripts) {
  const manifest = join(workspaceRoot, root, "Cargo.toml");
  if (!existsSync(manifest) || !readToml(manifest).package) return {};
  const script = buildPath(relative(workspaceRoot, join(LIBRARY_ROOT, "⚡️caching/🦀️cargo/📜️script.ts")));
  commandInputs ??= [];
  return Object.fromEntries(["build", "check", "test"].map((target) => [target, targetBuildModes(target, executionTargetV1("native", {
    executor: DEFAULT_EXECUTOR,
    cache: true,
    ...(target === "build" ? {} : { parallelism: false }),
    outputs: target === "build" ? ["{projectRoot}/dist/build/{options.env.SEMIO_BUILD_MODE}"] : [],
    inputs: [target === "test" ? "nativeTestSources" : "nativeSources", target === "test" ? "^nativeTestSources" : "^nativeSources", ...commandInputs],
    options: { cwd: ".", command: `bun ${JSON.stringify(script)} native cargo ${target} --manifest ${JSON.stringify(buildPath(relative(workspaceRoot, manifest)))}` },
  }, target === "test" ? "test" : "production"))]));
}

/** 🧩️ The one on-demand target of every plugin and extension crate: `component` builds exactly that component through the build driver (its Cargo pass
 * restricted to the one package, then describe, receipt, transpile, `wasm-opt`, probe and module) for the tests and tools that load it. It is uncached
 * (Cargo freshness and the stage cache are the authorities) and never part of a release graph, which compiles every component inside its one driver task.
 * @see ../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🟦️.ts */
function componentBuildTargets(root, workspaceRoot) {
  const path = join(workspaceRoot, root, "Cargo.toml");
  if (!existsSync(path)) return {};
  const manifest = readToml(path), metadata = manifest.package?.metadata;
  if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.["component-kind"])) return {};
  if (manifest.lib?.["crate-type"]?.length !== 1 || manifest.lib["crate-type"][0] !== "cdylib") throw new Error(`Component ${metadata.component.package} must be a cdylib-only component crate: ${path}`);
  const script = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🧩️driver-stages/📜️script.ts", moduleDirectory = componentDeploymentDirectoryV1(metadata.semio);
  return { component: executionTargetV1("orchestration", {
    executor: DEFAULT_EXECUTOR,
    cache: false,
    metadata: { semio: { orchestrator: true } },
    dependsOn: ["@semio-tech/os-plugin-describe-rs:build", "@semio-tech/framework-plugin-web:support", "workspace:deps-wasm-opt"],
    outputs: [componentModelOutputV1(buildPath(root)), ...(moduleDirectory === undefined ? [] : [`{workspaceRoot}/${PLUGIN_MODULES_ROOT}/${moduleDirectory}`])],
    options: { cwd: ".", command: `bun ${JSON.stringify(script)} component --manifest ${JSON.stringify(buildPath(relative(workspaceRoot, path)))}`, forwardAllArgs: false },
  }) };
}

/** 🧩️ The crate roots of every plugin and extension component, the crates a release orchestrator's driver compiles. */
function componentCrateRoots(configFiles, workspaceRoot) {
  return configFiles.filter((path) => path.endsWith("Cargo.toml") && !path.includes("\uFFFD") && !path.includes(".🧬semio") && !path.startsWith("compose/") && !path.startsWith("temp/compose/") && !POLICY.generatedDirectories.some((directory) => path.split("/").includes(directory))).flatMap((path) => {
    const metadata = readToml(join(workspaceRoot, path)).package?.metadata;
    return metadata?.component?.package && ["plugin", "extension"].includes(metadata.semio?.["component-kind"]) ? [buildPath(dirname(path))] : [];
  }).sort();
}

/** 🚀️ Gives every release orchestrator target (`metadata.semio.release`, a `PipelineV1` declaration) the Nx prerequisites its driver consumes and nothing else:
 * the declared prerequisites (toolchains, catalog generation) and the native generators of every component crate and every job root, all cached at t = 0. The
 * driver itself compiles, so the release graph is these generators, the orchestrator, and nothing per component.
 * @see ../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🧩️component-pass/🧬️schema/🔣️.json */
function withReleasePipelinePreparation(project, workspaceRoot, contracts, componentRoots, cache = new Map(), closures = new Map()) {
  for (const target of Object.values(project.targets ?? {})) {
    const pipeline = target.metadata?.semio?.release;
    if (!pipeline) continue;
    if (target.metadata.semio.orchestrator !== true || target.cache !== false || !Array.isArray(pipeline.prerequisites) || !Array.isArray(pipeline.jobs)) throw new Error(`Release pipeline must be an uncached orchestrator with prerequisites and jobs: ${project.name}`);
    const roots = [...new Set([...componentRoots, ...pipeline.jobs.map((job) => buildPath(job.root)).filter((root) => existsSync(join(workspaceRoot, root, "Cargo.toml")) && readToml(join(workspaceRoot, root, "Cargo.toml")).package)])];
    const generators = roots.flatMap((root) => nativePreparation(root, workspaceRoot, contracts, false, cache, closures).map((contract) => contract.target));
    target.dependsOn = [...new Set([...(target.dependsOn ?? []), ...pipeline.prerequisites, ...generators])].sort();
  }
  return project;
}

/** 🎮️ Flattens every plugin playground row from Cargo manifests (with authored `distDir`). */
function collectPlaygroundCatalog(configFiles, workspaceRoot,catalogContributions=[]) {
  if(!catalogContributions.length)return [];
  const components = new Map(), playgrounds = [], nativeFiles = new Map(), nativeClosures = new Map();
  for (const path of configFiles) {
    if (!path.endsWith("Cargo.toml") || path.includes("\uFFFD") || path.includes(".🧬semio") || path.startsWith("compose/") || path.startsWith("temp/compose/") || POLICY.generatedDirectories.some((directory) => path.split("/").includes(directory))) continue;
    const buildCatalog=catalogBuildContributionForManifestV1(catalogContributions,path);if(!buildCatalog)continue;
    const manifest = readToml(join(workspaceRoot, path)), metadata = manifest.package?.metadata;
    if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.["component-kind"])) continue;
    const id = metadata.component.package.slice(6), root = buildPath(dirname(path));
    if (components.has(id)) throw new Error(`Duplicate component identity: ${id}`);
    components.set(id, metadata.semio);
    for (const row of metadata.semio.playground ?? []) playgrounds.push({ ...row, pluginId: id, cratePath: root,buildCatalog });
  }
  return playgrounds;
}

/** 🌐️ Per-component `build` / `build-<variant>-site` targets that publish CDN trees: each variant runs the distribution build of its
 * playground row declared on the component crate, and `build` is the union of them.
 * @see ../../../💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📜️script.ts */
function pluginSiteTargetsForCrate(root, allPlaygrounds, stagingProjectRoot, workspaceRoot) {
  const rows = allPlaygrounds.filter((row) => row.cratePath === root);
  if (rows.length === 0) return {};
  if (rows.some(row => !row.distDir) && !stagingProjectRoot) throw Error("Playground site output requires the release producer's staging project");
  const output = row => {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(row.variant)) throw Error("Invalid distribution variant");
    if (row.distDir !== undefined && (typeof row.distDir !== "string" || !row.distDir || /[\\:\u0000-\u001f\u007f]/u.test(row.distDir) || row.distDir.split("/").some(part => !part || part === "." || part === ".."))) throw Error("Invalid distribution output directory");
    return `{workspaceRoot}/.cache/b/p/m/vite/{options.env.SEMIO_BUILD_MODE}/${createHash("sha256").update("os-dev:"+row.variant+":react").digest("hex").slice(0,16)}/out`;
  };
  const targets = {}, distribution = buildPath(relative(workspaceRoot, join(LIBRARY_ROOT, "../../../💻️os/🔨️modules/🧑‍💻dev/🚚️distribution/📜️script.ts")));
  for (const row of rows) {
    const site = `build-${row.variant}-site`;
    const build = `build-${row.variant}-react`;
    targets[site] = executionTargetV1("source", {
      cache: false,
      outputs: [output(row)],
      executor: "repo:noop",
      dependsOn: [`@semio-tech/framework-os-dev:${build}`],
      options: {},
    });
  }
  targets.build = executionTargetV1("source", { executor: "repo:noop", cache: false, dependsOn: rows.map((row) => `build-${row.variant}-site`), outputs: rows.map(output) });
  return targets;
}

/** 🏞️ Constructs admitted site targets only through their physical canonical release producer. */
function playgroundSiteTargetsForAdmissionV1(root,playgrounds,workspaceRoot,registry,selectedTargets){
 const rows=playgrounds.filter(row=>row.cratePath===root),names=new Set(["build",...rows.map(row=>"build-"+row.variant+"-site")]);
 if(!rows.length||selectedTargets!==undefined&&!selectedTargets.some(name=>names.has(name)))return {};
 const declaration=createRequire(import.meta.url)("../🚪️admission/🟦️.ts").readBuildOwnerRegistryV1(registry),owner=declaration.owners.find(owner=>owner.name==="@semio-tech/framework-os-dev");
 if(!owner||owner.manifest!==(owner.root==="."?PROJECT_BASENAME:owner.root+"/"+PROJECT_BASENAME))throw Error("Playground site output requires the release producer's staging project");
 const project=JSON.parse(readPhysicalSource(join(workspaceRoot,owner.manifest)).toString("utf8"));
 if(project.name!==owner.name||project.root!==undefined&&project.root!==owner.root)throw Error("Canonical playground release staging producer differs");
 return pluginSiteTargetsForCrate(root,rows,owner.root,workspaceRoot);
}

/** 🎮️ Gives each Cargo-declared playground session an independent generated artifact: a pure function of the committed
 * semantic descriptors of its runtime closure, so no session waits on a component build.
 * @returns {GeneratedProject["targets"]} */
function playgroundSessionTargets(configFiles, workspaceRoot,catalogContributions=[]) {
  if(!catalogContributions.length)return {};
  const components = new Map(), playgrounds = new Map();
  for (const file of configFiles) {
    if (!file.endsWith("Cargo.toml") || file.includes("\uFFFD") || file.includes(".🧬semio") || file.startsWith("compose/") || file.startsWith("temp/compose/") || POLICY.generatedDirectories.some(directory => file.split("/").includes(directory))) continue;
    const buildCatalog=catalogBuildContributionForManifestV1(catalogContributions,file);if(!buildCatalog)continue;
    const manifest=readToml(join(workspaceRoot,file)), metadata=manifest.package?.metadata;
    if(!metadata?.component?.package || !["plugin","extension"].includes(metadata.semio?.["component-kind"])) continue;
    const pluginId=metadata.component.package.slice(6), root=buildPath(dirname(file)), projectFile=join(workspaceRoot,root,PROJECT_BASENAME);
    components.set(pluginId,{...metadata.semio,pluginId,ownerRoot:buildPath(join(root,"..","..")),project:existsSync(projectFile)?JSON.parse(readFileSync(projectFile,"utf8")).name:manifest.package.name,dependsOn:[...(metadata.semio.extends?[metadata.semio.extends]:[]),...(metadata.semio["depends-on"]??[])]});
    for(const row of metadata.semio.playground??[]) {
      if(typeof row.variant!=="string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(row.variant) || playgrounds.has(row.variant)) throw new Error("Invalid or duplicate playground variant in "+file);
      playgrounds.set(row.variant,{...row,pluginId,buildCatalog});
    }
  }
  return Object.fromEntries([...playgrounds].sort(([a],[b])=>a.localeCompare(b)).map(([variant,row])=>["session-"+variant,executionTargetV1("source",{
    cache:true,
    inputs:["default",{dependentTasksOutputFiles:"**/*",transitive:true},...[...runtimeComponentClosure([...components.values()],[{id:row.pluginId,appScoped:row.app!==undefined}])].sort().flatMap(id=>["🛂️.descriptor.semio","🔣️.json"].map(name=>`{workspaceRoot}/${components.get(id).ownerRoot}/${name}`))],
    dependsOn:[row.buildCatalog.project+":"+row.buildCatalog.generateTarget],
    outputs:["{workspaceRoot}/.cache/b/p/m/browser/{options.env.SEMIO_BUILD_MODE}/sessions/"+variant],
    options:{command:"bun ./📜️script.ts session "+variant,env:{SEMIO_PLUGIN_CATALOG:join(workspaceRoot,row.buildCatalog.publication.outputRoot)}},
  })]));
}

/** 🎮️ Declares completed runtime prerequisites, cacheable validation and profile-specific browser producers.
 * React activation owns its restorable runtime directory. WGPU activation publishes live extensions,
 * fonts and reload notifications, so it must execute after every cacheable preparation. */
/** 🎯️ Generated Nx targets for every playground preparation, activation, install and serve command.
 * @param {readonly string[]} configFiles
 * @param {string} workspaceRoot
 * @param {string} projectRoot
 * @returns {Record<string, { cache: boolean, continuous?: boolean, dependsOn: string[], outputs: string[], inputs?: unknown[], options: { command: string, forwardAllArgs?: boolean } }>}
 */
async function playgroundPreparationTargets(configFiles, workspaceRoot, projectRoot, contracts = {},catalogContributions=[],context) {
  if(!catalogContributions.length)return {};
  if(!context?.invocation&&!context?.preparationControl)throw Error("Original playground preparation invocation required");
  const advance=phase=>context.preparationControl?context.preparationControl.advance(phase):advanceScriptInvocation(context.invocation,phase,"running");
  await advance("build:playground-components");
  const components = new Map(), playgrounds = [], nativeFiles = new Map(), nativeClosures = new Map();
  const projectOwners = new Map(), nativeHostOwners = new Map(), nativePlans = new Map();
  const componentPrerequisites=async id=>{if(!nativePlans.has(id)){await advance("build:playground-native");nativePlans.set(id,nativePreparation(components.get(id).cratePath,workspaceRoot,contracts,false,nativeFiles,nativeClosures).map(contract=>contract.target));}return nativePlans.get(id);};
  const closurePrerequisites=async(ids,facet)=>{const prerequisites=["@semio-tech/os-plugin-describe-rs:build",...(facet==="browser"?["workspace:deps-wasm-opt"]:[])];for(const id of ids)prerequisites.push(...await componentPrerequisites(id));return [...new Set(prerequisites)].sort();};
  const closureOutputs=(ids,facet="browser",mode="{options.env.SEMIO_BUILD_MODE}")=>[...ids].sort().flatMap(id=>{const row=components.get(id),directory=componentDeploymentDirectoryV1(row);return[componentModelOutputV1(row.cratePath,mode),...(facet==="browser"&&directory!==undefined?[`{workspaceRoot}/${PLUGIN_MODULES_ROOT}/${directory}`]:[])];});
  const orchestrator = { semio: { orchestrator: true } };
  const projectAt = (root) => {
    const path = join(workspaceRoot, root, PROJECT_BASENAME);
    if (!projectOwners.has(root)) projectOwners.set(root, existsSync(path) ? JSON.parse(readFileSync(path, "utf8")) : undefined);
    return projectOwners.get(root);
  };
  let manifestIndex=0;for (const path of configFiles) {
    if(++manifestIndex%16===0)await advance("build:playground-components");
    if (!path.endsWith("Cargo.toml") || path.includes("\uFFFD") || path.includes(".🧬semio") || path.startsWith("compose/") || path.startsWith("temp/compose/") || POLICY.generatedDirectories.some((directory) => path.split("/").includes(directory))) continue;
    const buildCatalog=catalogBuildContributionForManifestV1(catalogContributions,path);if(!buildCatalog)continue;
    const manifest = readToml(join(workspaceRoot, path)), metadata = manifest.package?.metadata;
    if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.["component-kind"])) continue;
    const id = metadata.component.package.slice(6), root = buildPath(dirname(path));
    if (components.has(id)) throw new Error(`Duplicate component identity: ${id}`);
    components.set(id, { ...metadata.semio, cratePath: root, project: projectAt(root)?.name ?? manifest.package.name });
    for (const row of metadata.semio.playground ?? []) playgrounds.push({ ...row, pluginId: id, cratePath: root,buildCatalog });
  }

  const wgpuRoot = buildPath("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript"), wgpuProject = projectAt(wgpuRoot);
  if (!wgpuProject?.name || !["prototype","release"].every(mode=>wgpuProject.targets?.wasm?.configurations?.[mode]?.env?.SEMIO_BUILD_MODE===mode)) throw new Error(`WGPU renderer must name both authored wasm profile producers: ${wgpuRoot}`);
  const flowRoot="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust", flowProject=projectAt(flowRoot);
  if(!flowProject?.name || !flowProject.targets?.wasm)throw new Error(`Flow must name its authored browser producer: ${flowRoot}`);
  const enginePrerequisites = owners => owners.map(root=>{const project=projectAt(root);if(!project?.name||!project.targets?.wasm)throw Error(`Engine must name its authored producer: ${root}`);return `${project.name}:wasm`;});
  const rendererEngines=enginePrerequisites(frameworkOsEngineSourceOwnersV1("renderer")),distributionEngines=enginePrerequisites(frameworkOsEngineSourceOwnersV1("distribution"));
  const nativeHostView={kind:(path)=>{try{const stat=lstatSync(join(workspaceRoot,path));return stat.isSymbolicLink()?"symlink":stat.isDirectory()?"directory":stat.isFile()?"file":null;}catch(error){if(error.code==="ENOENT")return null;throw error;}},readText:(path)=>readFileSync(join(workspaceRoot,path),"utf8"),admitScript:(path)=>observedSourceAnalysisV1(prepareSourceAnalysesV1().nativeScript,join(workspaceRoot,path))};
  const nativeHostOwner = root => { if (!nativeHostOwners.has(root)) nativeHostOwners.set(root, nativeHostSourceFactsV1(root, nativeHostView)); return nativeHostOwners.get(root); };
  const result = {};
  for (const playground of playgrounds) {
    await advance("build:playground-variants");
    if (typeof playground.variant !== "string" || playground.variant.length > 256 || !/^[a-z0-9]+(?:-[a-z0-9]+)*$(?![\s\S])/u.test(playground.variant)) throw Error("Invalid playground variant");
    const composition = playground.compositionConfigPath === undefined ? undefined : playgroundCompositionPathV1(playground.compositionConfigPath), wgpuPort = playground.ports?.wgpu;
    if (!Number.isSafeInteger(wgpuPort) || wgpuPort < 1 || wgpuPort > 65535) throw Error("Invalid declared WGPU listener port");
    const componentRows = [...components].map(([pluginId,row])=>({...row,pluginId,dependsOn:[...(row.extends?[row.extends]:[]),...(row["depends-on"]??[])]}));
    const sources=[...(playground.engines??[]).map(path=>buildPath(relative(workspaceRoot,resolve(workspaceRoot,path,PROJECT_BASENAME)))),...[playground.nativeHost,playground.mcpHost].filter(Boolean).flatMap(host=>[`${host.cratePath}/Cargo.toml`,`${host.cratePath}/${PROJECT_BASENAME}`]),...(playground.devContribution?[playground.devContribution]:[]),...(composition===undefined?[]:[composition])];
    const request={component:playground.pluginId,appScoped:playground.app!==undefined,sources};
    let admission=runtimeInputAdmissionV1(componentRows,[{id:request.component,appScoped:request.appScoped}],sources,path=>existsSync(join(workspaceRoot,path)));
    if(admission.status==="admitted")for(const id of admission.selected){try{await componentPrerequisites(id);}catch(error){if(error.code!=="NATIVE_INPUT_ADMISSION")throw error;const missing=buildPath(relative(workspaceRoot,error.manifest));sources.push(missing);admission={schemaVersion:1,status:"refused",missing:{kind:"source",value:missing}};break;}}
    if(admission.status==="refused") {
      const command=`bun ${JSON.stringify(join(workspaceRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📜️script.ts"))} runtime-input-check ${Buffer.from(JSON.stringify(request)).toString("base64")}`;
      const target=executionTargetV1("source",{cache:false,outputs:[],dependsOn:[],inputs:["{workspaceRoot}/**/Cargo.toml","{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs","{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/**/*"],metadata:{semio:{runtimeInputAdmission:admission}},options:{command,forwardAllArgs:false}});
      const names=[`runtime-input-admission-${playground.variant}`,`build-${playground.variant}-react-release`];
      for(const profile of ["prototype","release"]){for(const engine of ["react","wgpu"])for(const operation of ["prepare","activate","serve","dev"])names.push(`${operation}-${playground.variant}-${engine}-${profile}`);names.push(`prepare-${playground.variant}-native-${profile}`,`run-${playground.variant}-native-${profile}`,`smoke-${playground.variant}-native-${profile}`);if(playground.mcpHost)for(const transport of ["stdio","http"])names.push(`mcp-${playground.variant}-${transport}-${profile}`);}
      for(const name of names)result[name]=target;
      continue;
    }
    const nativeHost=admitPlaygroundNativeHostV1(playground.nativeHost,nativeHostOwner);
    const mcpHost=admitPlaygroundNativeHostV1(playground.mcpHost,nativeHostOwner);
    if(mcpHost)for(const profile of ["prototype","release"])for(const transport of ["stdio","http"])result[`mcp-${playground.variant}-${transport}-${profile}`]={cache:false,continuous:true,outputs:[],dependsOn:[`${mcpHost.project}:${mcpHost.target}`],options:{command:`bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts mcp ${playground.variant} ${profile} ${transport}`,forwardAllArgs:true}};
    // 🏠️ Boot closure: host sessions must serve before the full catalog materializes. `appScoped: true`
    // keeps the host's own depends-on/consumes without the host-fanout that otherwise selects every crate.
    const bootSelected = runtimeComponentClosure(componentRows, [{ id: playground.pluginId, appScoped: true }]);
    const selected = runtimeComponentClosure(componentRows, [{ id: playground.pluginId, appScoped: playground.app !== undefined }]);
    const bootPrerequisites=await closurePrerequisites(bootSelected,"browser"),selectedPrerequisites=await closurePrerequisites(selected,"browser"),nativePrerequisites=await closurePrerequisites(selected,"model");
    const engines = new Set([...new Set([...(playground.engines ?? []), ...declaredBrowserSessionEnginesV1(workspaceRoot, playground.devContribution)])].map((path) => {
      const root = buildPath(relative(workspaceRoot, resolve(workspaceRoot, path))), project = projectAt(root);
      if (root.startsWith("../") || !project?.name || !project.targets?.wasm) throw new Error(`Playground engine must name an authored wasm producer: ${path}`);
      return `${project.name}:wasm`;
    }));
    for (const profile of ["prototype", "release"]) {
      const nativeScript = "bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts";
      result[`prepare-${playground.variant}-native-${profile}`] = {
        cache: false,
        metadata: orchestrator,
        outputs: [`{workspaceRoot}/.cache/b/p/m/browser/${profile}/native/${playground.variant}`, ...closureOutputs(selected,"model",profile)],
        dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, ...nativePrerequisites],
        options: { command: `bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/📜️script.ts publish ${playground.variant} ${profile}`, forwardAllArgs: false },
      };
      for (const operation of ["run", "smoke"]) result[`${operation}-${playground.variant}-native-${profile}`] = {
        cache: false, continuous: false, outputs: [],
        dependsOn: [`prepare-${playground.variant}-native-${profile}`, `${nativeHost?.project??wgpuProject.name}:${nativeHost?.target??"native-build"}`],
        options: { command: `${nativeScript} run ${playground.variant} ${profile}${operation === "smoke" ? " --smoke" : ""}`, forwardAllArgs: true },
      };
      // ⚡ `serve` must not wait on activate: warm `served` boots from already-staged modules (content-hash
      // freshness inside ServeScript) in seconds. `dev` still depends on boot-only activate so the host is
      // materialized before Vite, then the activation Vite plugin prefetches the rest on demand.
      result[`serve-${playground.variant}-react-${profile}`] = { cache: false, continuous: true, outputs: [], dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`], options: { command: `bun ./📜️script.ts serve ${playground.variant} react ${profile}` } };
      result[`dev-${playground.variant}-react-${profile}`] = { cache: false, continuous: true, outputs: [], dependsOn: [`activate-${playground.variant}-react-${profile}`], options: { command: `bun ./📜️script.ts serve ${playground.variant} react ${profile}` } };
      result[`activate-${playground.variant}-react-${profile}`] = {
      cache: true,
      parallelism: false,
      outputs: [`{workspaceRoot}/.cache/b/p/m/browser/${profile}/runtime/react/${playground.variant}`],
      inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
      dependsOn: [`prepare-${playground.variant}-react-${profile}`],
      options: { command: `bun ./📜️script.ts activate ${playground.variant} react ${profile}` },
      };
      result[`prepare-${playground.variant}-react-${profile}`] = {
      cache: false,
      metadata: orchestrator,
      outputs: closureOutputs(bootSelected),
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, "@semio-tech/framework-plugin-web:support", "semio-framework-os-infinite:fonts", ...rendererEngines, ...engines, ...bootPrerequisites],
      options: { command: `bun ./📜️script.ts prepare ${playground.variant} react ${profile}` },
      };
      for (const command of ["serve", "dev"]) result[`${command}-${playground.variant}-wgpu-${profile}`] = { cache: false, continuous: true, outputs: [], dependsOn: [`activate-${playground.variant}-wgpu-${profile}`], options: { command: `bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts serve ${playground.variant} ${profile} --port ${wgpuPort}`, ...(composition===undefined?{}:{env:{SEMIO_WGPU_COMPOSITION_PATH:composition}}) } };
      result[`activate-${playground.variant}-wgpu-${profile}`] = {
      cache: false,
      outputs: [`{workspaceRoot}/.cache/b/p/m/browser/${profile}/runtime/wgpu/${playground.variant}`],
      inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
      dependsOn: [`prepare-${playground.variant}-wgpu-${profile}`],
      options: { command: `bun ./📜️script.ts activate ${playground.variant} wgpu ${profile}` },
      };
      result[`prepare-${playground.variant}-wgpu-${profile}`] = {
      cache: false,
      metadata: orchestrator,
      outputs: closureOutputs(bootSelected),
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, "@semio-tech/framework-plugin-web:support", "semio-framework-os-infinite:fonts", `${wgpuProject.name}:wasm`, `${wgpuProject.name}:generate-browser-boot`, `${wgpuProject.name}:generate-frame-worker`, `${wgpuProject.name}:generate-renderer-boot`, ...bootPrerequisites],
      options: { command: `bun ./📜️script.ts prepare ${playground.variant} wgpu ${profile}` },
      };
    }
    const name = `build-${playground.variant}-react-release`;
    const buildIdentity=createHash("sha256").update("os-dev:"+playground.variant+":react").digest("hex").slice(0,16);
    const buildRoot = `{workspaceRoot}/.cache/b/p/m/vite`;
    const buildOutput = `${buildRoot}/release/${buildIdentity}/out`;
    result[name] = {
      cache: false,
      metadata: orchestrator,
      outputs: [buildOutput],
      // 🌐️ `S_HUB_URL` and `S_DATA_DIR` are baked into the bundle at Vite `define` time
      // (`🏗️builder/🌐️vite/🟦️.ts:237`, `import.meta.env.VITE_S_HUB_URL`), so two bundles that differ
      // only by which hub they sign in against are DIFFERENT artifacts. Without these env inputs the
      // cache key ignores them and a cached bundle silently answers for the wrong hub.
      inputs: ["production", "^production", { dependentTasksOutputFiles: "**/*", transitive: true }, { env: "S_HUB_URL" }, { env: "S_DATA_DIR" }, { runtime: `bun ${JSON.stringify(buildPath(relative(workspaceRoot, resolve(workspaceRoot, projectRoot, "../../🚚️distribution/📜️script.ts"))))} inputs` }],
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, "@semio-tech/framework-plugin-web:support", "semio-framework-os-infinite:fonts", "semio-framework-canvas:stage-runtime-assets", `${flowProject.name}:wasm`, ...distributionEngines, ...engines, ...selectedPrerequisites, "@semio-tech/assets:build"],
      options: { command: `bun ../../🚚️distribution/📜️script.ts build ${playground.variant} react release`, forwardAllArgs: true },
    };
    result[`build-${playground.variant}-react`] = {
      ...result[name], outputs: [`${buildRoot}/{options.env.SEMIO_BUILD_MODE}/${buildIdentity}/out`],
      configurations: {
        prototype: { command: `bun ../../🚚️distribution/📜️script.ts build ${playground.variant} react prototype` },
        release: { command: `bun ../../🚚️distribution/📜️script.ts build ${playground.variant} react release` },
      },
    };
  }
  for (const playground of playgrounds) for (const renderer of ["react", "wgpu"]) {
    const prototype = result[`prepare-${playground.variant}-${renderer}-prototype`];
    const release = result[`prepare-${playground.variant}-${renderer}-release`];
    if (!prototype || !release) continue;
    result[`prepare-${playground.variant}-${renderer}`] = {
      ...prototype,
      dependsOn: prototype.dependsOn.map(target => target.replace(/:wasm-release$/, ":wasm")),
      configurations: {
        prototype: { command: prototype.options.command },
        release: { command: release.options.command },
      },
    };
  }
  const pluginSiteReleases = playgrounds.filter((row) => row.distDir !== undefined).map((row) => `build-${row.variant}-react`);
  if (pluginSiteReleases.length > 0) {
    result["build-all-playground-cdn-sites"] = {
      cache: false,
      metadata: orchestrator,
      dependsOn: pluginSiteReleases,
      outputs: [],
      options: { command: "bun ../../🚚️distribution/📜️script.ts complete", forwardAllArgs: false },
    };
  }
  return Object.fromEntries(Object.entries(result).map(([name,target])=>{
    const owner=playgrounds.filter(row=>name.includes("-"+row.variant+"-")||name.endsWith("-"+row.variant)).sort((a,b)=>b.variant.length-a.variant.length)[0]?.buildCatalog;
    if(owner)target={...target,options:{...target.options,env:{...target.options?.env,SEMIO_PLUGIN_CATALOG:join(workspaceRoot,owner.publication.outputRoot)}}};
    return [name,target.metadata?.semio?.execution ? target : executionTargetV1("orchestration",target)];
  }));
}

/** 🚪️ Exposes registered workspace command coordinators without inferring forwarding package scripts.
 * @returns {GeneratedProject["targets"]} */
function rootCommandTargets(script) {
  const ts = createRequire(import.meta.url)("typescript");
  const source = ts.createSourceFile(script, readFileSync(script, "utf8"), ts.ScriptTarget.Latest, true);
  const targets = {};
  const inspect = (node) => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "register" && ts.isStringLiteral(node.arguments[0])) {
      const name = node.arguments[0].text;
      if (!["nx", "repo"].includes(name)) targets[name] = executionTargetV1("orchestration", { executor: DEFAULT_EXECUTOR, outputs: [], options: { command: `bun ./📜️script.ts ${name}`, forwardAllArgs: true } });
      inspect(node.expression.expression);
    }
  };
  for (const statement of source.statements) {
    if (!ts.isVariableStatement(statement)) continue;
    for (const declaration of statement.declarationList.declarations) if (declaration.name.getText(source) === "router" && declaration.initializer) inspect(declaration.initializer);
  }
  return targets;
}

/**
 * @param {string[]} configFiles
 * @param {unknown} options
 * @param {{ workspaceRoot: string }} context
 */
/** 🛡️ Admits callback candidates through fresh raw physical workspace ancestry. */
function inferenceCandidates(configFiles, workspaceRoot) {
  if (typeof workspaceRoot !== "string" || !workspaceRoot || workspaceRoot.includes("\0") || workspaceRoot.split(/[\\/]/u).some(part => part === "." || part === "..")) throw Error("Build inference requires raw real workspace ancestry");
  const root = resolve(workspaceRoot), ancestors = [];
  for (let path = root; ; path = dirname(path)) { ancestors.push(path); if (path === dirname(path)) break; }
  for (const path of ancestors.reverse()) { const stat = lstatSync(path); if (!stat.isDirectory() || stat.isSymbolicLink()) throw Error(`Build inference requires real workspace ancestry: ${path}`); }
  const directories = new Map([[root, true]]), files = new Set();
  for (const supplied of configFiles) {
    if (typeof supplied !== "string" || !supplied || supplied.includes("\0") || supplied.includes("\uFFFD")) continue;
    const path = buildPath(supplied), parts = path.split("/");
    if (exampleCollectionPath(path) || isAbsolute(path) || /^[A-Za-z]:/u.test(path) || parts.some(part => !part || part === "." || part === ".." || part === ".🧬semio" || POLICY.generatedDirectories.includes(part)) || path.startsWith("compose/") || path.startsWith("temp/compose/")) continue;
    let directory = root, admitted = true;
    for (const part of parts.slice(0, -1)) {
      directory = join(directory, part);
      if (!directories.has(directory)) {
        try { const value = lstatSync(directory); directories.set(directory, value.isDirectory() && !value.isSymbolicLink()); }
        catch (error) { if (!["ENOENT", "ENOTDIR"].includes(error.code)) throw error; directories.set(directory, false); }
      }
      if (!directories.get(directory)) { admitted = false; break; }
    }
    if (!admitted) continue;
    try { const value = lstatSync(join(root, path)); if (value.isFile() && !value.isSymbolicLink()) files.add(path); }
    catch (error) { if (!["ENOENT", "ENOTDIR"].includes(error.code)) throw error; }
  }
  return [...files].sort((left, right) => Buffer.compare(Buffer.from(left), Buffer.from(right)));
}

/** 🦉️ Projects native catalog manifest candidates from canonical owner identities. */
async function catalogNativeManifestCandidatesV1(workspaceRoot,registry,accept,context){
 const manifest=root=>root==="."?"Cargo.toml":root+"/Cargo.toml",declared=new Set(registry.owners.flatMap(owner=>[...(owner.manifest.endsWith("Cargo.toml")?[owner.manifest]:[]),...(owner.nativeRoots??[]).map(manifest)])),candidates=[...new Set([...declared,...registry.owners.map(owner=>manifest(owner.root))])].filter(accept).sort(),result=[];let index=0;
 for(const path of candidates){await yieldInferenceV1(context,"build:catalog-manifests",++index);try{const entry=lstatSync(join(workspaceRoot,path));if(!entry.isFile()||entry.isSymbolicLink())throw Error("Catalog native manifest must be a real file: "+path);}catch(error){if(!declared.has(path)&&["ENOENT","ENOTDIR"].includes(error.code))continue;throw error;}result.push(path);}
 return result;
}

/** 🔌️ Loads only the caller-declared optional providers and verifies current producer authority. */
async function catalogBuildContributionsV1(workspaceRoot,providers) {
  const catalogContributions=await loadCatalogBuildProvidersV1(providers,{
    readText(path){return readFileSync(join(workspaceRoot,path),"utf8");},
    kind(path){try{const stat=lstatSync(join(workspaceRoot,path));return stat.isSymbolicLink()?"symlink":stat.isDirectory()?"directory":stat.isFile()?"file":null;}catch(error){if(error.code==="ENOENT")return null;throw error;}},
    async load(path){const file=join(workspaceRoot,path),source=readPhysicalSource(file),revision=createHash("sha256").update(source).digest("hex"),module=await importRevision(pathToFileURL(file),revision);if(createHash("sha256").update(readPhysicalSource(file)).digest("hex")!==revision)throw Object.assign(Error("Catalog provider source changed during admission"),{code:"BUILD_SOURCE_AUTHORITY"});return module;},
  });
  for(const owner of catalogContributions){for(const input of owner.inputs)readFileSync(join(workspaceRoot,input));const project=JSON.parse(readFileSync(join(workspaceRoot,owner.projectRoot,PROJECT_BASENAME),"utf8"));if(project.name!==owner.project||!project.targets?.[owner.generateTarget])throw Error("Catalog build producer is absent from its owner's project");}
  return catalogContributions;
}

/** 🧭️ Reads the explicit catalog options declared on this workspace plugin. */
async function workspaceCatalogBuildContributionsV1(workspaceRoot) {
  const config=JSON.parse(readFileSync(join(workspaceRoot,"📋️build-owners.json"),"utf8"));
  return catalogBuildContributionsV1(workspaceRoot,config.catalogProviders.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath})));
}

/** 🧩️ Invalidates project assembly only when its actual native owners or component pipeline change. */
function projectAssemblyAuthorityV1(workspaceRoot,nativeRoot,projectsByRoot,componentRoots,nativeCache=new Map(),nativeClosures=new Map()){
 let nativeOwners;
 if(nativeRoot!==undefined){try{nativeOwners=nativeDependencyRoots(nativeRoot,workspaceRoot,true,nativeCache,nativeClosures).map(owner=>[owner,projectsByRoot.get(owner)??null]);}catch(error){if(error.code!=="NATIVE_INPUT_ADMISSION")throw error;nativeOwners=[error.manifest,error.message];}}
 return createHash("sha256").update(JSON.stringify([nativeOwners??null,componentRoots??null])).digest("hex");
}

/** ⏳️ Delegates bounded inference work to the caller's original progress and continuation control. */
async function yieldInferenceV1(context,phase,index,force=false){if(context.invocation){requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkScriptInvocation(context.invocation);checkResourceLeaseControl(context.phaseControl);if(force||index%16===0){await advanceScriptInvocation(context.invocation,phase,"running");checkResourceLeaseControl(context.phaseControl);}}}

/** 🪪️ Expands actual native owners through the same read-only phase and original caller controls. */
export async function discoverNativeBuildOwnersV1(configFiles,workspaceRoot,context){
  configFiles = inferenceCandidates([...configFiles,...configFiles.filter(file=>file.endsWith("Cargo.toml")).map(file=>buildPath(join(dirname(file),PROJECT_BASENAME)))], workspaceRoot);
  const nativeCache=new Map(),nativeClosures=new Map(),nativeCandidates=new Set(configFiles);let candidateIndex=0;
  for(const file of configFiles.filter(file=>file.endsWith("Cargo.toml"))){
    await yieldInferenceV1(context,"build:cargo-owners",++candidateIndex);
    try{for(const root of nativeDependencyRoots(buildPath(dirname(file)),workspaceRoot,true,nativeCache,nativeClosures)){nativeCandidates.add(`${root}/Cargo.toml`);nativeCandidates.add(`${root}/${PROJECT_BASENAME}`);}}
    catch(error){if(error.code!=="NATIVE_INPUT_ADMISSION")throw error;}
  }
  return {files:inferenceCandidates([...nativeCandidates],workspaceRoot),nativeCache,nativeClosures};
}

/** 🚪️ Observes one canonical owner and only its selected executable source closures. */
export async function discoverBuildOwnerAdmissionFactsV1(owner,targets,registry,context,inputNames=[]){
 await libraryBootstrap;const {workspaceRoot}=context,nativeCache=context.nativeCache??new Map(),nativeClosures=context.nativeClosures??new Map();
 if(context.invocation){requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkResourceLeaseControl(context.phaseControl);checkScriptInvocation(context.invocation);}
 const stage=async phase=>{if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true")console.error("[DEBUG] selected-owner-stage "+JSON.stringify({owner:owner.name,phase}));await yieldInferenceV1(context,"build:selected-"+phase,0,true);};
 await stage("declaration");
 const path=join(workspaceRoot,owner.manifest),source=readPhysicalSource(path);
 const declaration=owner.manifest.endsWith("Cargo.toml")?readToml(path):JSON.parse(source.toString("utf8"));
 const authoredTargets=owner.manifest.endsWith("Cargo.toml")?{}:declaration.targets??{},scripts=createScriptInputCache(context),json=owner.manifest.endsWith("Cargo.toml")?{name:declaration.package?.name,root:owner.root,targets:await cargoTargets(owner.root,workspaceRoot,undefined,scripts)}:{...declaration,root:owner.root};
 if(json.name!==owner.name)throw Error("Admitted canonical build owner differs: "+owner.manifest);
 json.targets={...await cargoTargets(owner.root,workspaceRoot,undefined,scripts),...json.targets,...componentBuildTargets(owner.root,workspaceRoot)};
 const contractsPath=join(workspaceRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json"),contracts=existsSync(contractsPath)?readTaxonomyProjectionV1(contractsPath).generatorContracts:{},providers=registry.catalogProviders.filter(provider=>targets.length&&provider.consumerRoots.some(root=>owned(owner.root,root))||inputNames.includes("{workspaceRoot}/"+provider.sourcePath)),catalogContributions=providers.length?await catalogBuildContributionsV1(workspaceRoot,providers.map(({ownerRoot,sourcePath})=>({ownerRoot,sourcePath}))):[];
 const catalogFiles=catalogContributions.length?await catalogNativeManifestCandidatesV1(workspaceRoot,registry,path=>catalogBuildContributionForManifestV1(catalogContributions,path),context):[];
 await stage("catalog-targets");
 if(catalogContributions.length&&targets.length){
  if(json.name==="@semio-tech/plugin-registry")json.targets={...json.targets,...playgroundSessionTargets(catalogFiles,workspaceRoot,catalogContributions)};
  if(json.name==="@semio-tech/framework-os-dev")json.targets={...json.targets,...await playgroundPreparationTargets(catalogFiles,workspaceRoot,owner.root,contracts,catalogContributions,context)};
  const catalog=collectPlaygroundCatalog(catalogFiles,workspaceRoot,catalogContributions);json.targets={...json.targets,...playgroundSiteTargetsForAdmissionV1(owner.root,catalog,workspaceRoot,registry,targets),...authoredTargets};
 }
 const selected=Object.fromEntries(targets.map(name=>{if(!json.targets?.[name])throw Error("Unknown admitted build target: "+owner.name+":"+name);const target=targetWithDefaults(json.targets[name],owner.root,existsSync(join(workspaceRoot,owner.root,SCRIPT_BASENAME))),modes=targetBuildModes(name,target),configuration=(context.selection?.kind==="tasks"?context.selection.tasks.find(task=>task.project===owner.name&&task.target===name)?.configuration:undefined)??modes.defaultConfiguration,overrides=modes.configurations?.[configuration];if(!overrides)throw Error("Unknown selected build configuration: "+configuration);if(Object.keys(overrides).some(key=>["dependsOn","inputs","outputs","configurations","executor"].includes(key)))throw Error("Build configuration cannot change task identity");return [name,target];})),files=new Set([owner.manifest]),dependencies=new Set(),dependencyInputs=new Map(),caretInputs=new Set();
 const byPackage=new Map(registry.owners.flatMap(owner=>owner.packages.map(name=>[name,owner.name]))),byRoot=new Map(registry.owners.flatMap(owner=>[owner.root,...owner.nativeRoots??[]].map(root=>[root,owner.name]))),owners=registry.owners.flatMap(owner=>[owner.root,...owner.nativeRoots??[]].map(root=>({name:owner.name,root}))).sort((a,b)=>b.root.length-a.root.length);
 const addFile=path=>files.add(path),addDependency=path=>{const owner=owners.find(owner=>owned(path,owner.root));if(owner&&owner.name!==json.name)dependencies.add(owner.name);},addInput=(project,input)=>{if(!registry.owners.some(owner=>owner.name===project)||typeof input!=="string"||!input)throw Error("Canonical named input owner required");const selected=dependencyInputs.get(project)??new Set();selected.add(input);dependencyInputs.set(project,selected);};
 for(const provider of providers)addFile(provider.sourcePath);for(const file of catalogFiles){readPhysicalSource(join(workspaceRoot,file));addFile(file);}for(const contribution of catalogContributions){for(const input of contribution.inputs)addFile(input);addFile(contribution.projectRoot+"/"+PROJECT_BASENAME);if(contribution.project!==json.name)dependencies.add(contribution.project);}
 await stage("command-sources");
 for(const target of Object.values(selected)){
  const sources=await targetScriptSources(target,workspaceRoot,scripts,owner.root,owner.manifest.endsWith("Cargo.toml")?owner.manifest:json.metadata?.nativeRoot?buildPath(join(json.metadata.nativeRoot,"Cargo.toml")):undefined);
  for(const input of sources.closure??[])if(typeof input==="string"&&input.startsWith("{workspaceRoot}/"))addFile(input.slice("{workspaceRoot}/".length));

 }
 const packageFile=join(workspaceRoot,owner.root,"package.json");if(existsSync(packageFile)){const manifest=JSON.parse(readFileSync(packageFile,"utf8"));addFile(buildPath(relative(workspaceRoot,packageFile)));for(const name of Object.keys({...manifest.dependencies,...manifest.peerDependencies,...manifest.optionalDependencies})){const owner=byPackage.get(name);if(owner&&owner!==json.name)dependencies.add(owner);}}
 const nativeRoot=json.metadata?.nativeRoot??owner.root,nativeManifest=join(workspaceRoot,nativeRoot,"Cargo.toml");
 await stage("native-closure");
 if(existsSync(nativeManifest))for(const root of nativeDependencyRoots(nativeRoot,workspaceRoot,true,nativeCache,nativeClosures)){addFile(root+"/Cargo.toml");const dependency=byRoot.get(root);if(!dependency)throw Error("Canonical native dependency owner required: "+root);if(dependency!==json.name)dependencies.add(dependency);}
 if(targets.length||inputNames.length){
  await stage("normalization");const normalized=await projectWithDefaults({...json,targets:{...json.targets,...selected}},owner.root,join(workspaceRoot,owner.root),workspaceRoot,contracts,new Map(),undefined,scripts,undefined,"source",targets);await stage("native-preparation");withNativePreparation(normalized,workspaceRoot,contracts,nativeCache,byRoot,nativeClosures);
  const named={...JSON.parse(readFileSync(join(LIBRARY_ROOT,"🏗️build/🔣️policy.json"),"utf8")).namedInputs,...normalized.namedInputs},patterns=[],active=new Set();
  const expand=values=>{for(const value of values){if(typeof value!=="string"){
   if(value===null||typeof value!=="object"||Array.isArray(value))throw Error("Closed selected source input required");
   const keys=Object.keys(value);
   if(typeof value.input==="string"&&keys.every(key=>["input","projects","dependencies"].includes(key))){if(value.projects!==undefined){if(!Array.isArray(value.projects)||value.dependencies!==undefined)throw Error("Canonical named input project list required");for(const project of value.projects)addInput(project,value.input);}else if(value.dependencies===true)caretInputs.add(value.input);else if(value.dependencies===undefined||value.dependencies===false)expand([value.input]);else throw Error("Closed named input dependency required");}
   else if(typeof value.fileset==="string"&&keys.length===1)expand([value.fileset]);
   else if(keys.length===1&&typeof(value.env??value.runtime)==="string"||keys.length===1&&Array.isArray(value.externalDependencies)&&value.externalDependencies.every(name=>typeof name==="string"))continue;
   else if(typeof value.dependentTasksOutputFiles==="string"&&keys.every(key=>["dependentTasksOutputFiles","transitive"].includes(key))&&(value.transitive===undefined||typeof value.transitive==="boolean"))continue;
   else throw Error("Unsupported selected source input object");
   continue;
  }if(value.startsWith("^")){caretInputs.add(value.slice(1));continue;}if(Object.hasOwn(named,value)){if(active.has(value))throw Error("Selected named input cycle: "+value);active.add(value);expand(named[value]);active.delete(value);}else if(value.includes("{workspaceRoot}")||value.includes("{projectRoot}"))patterns.push(value.replaceAll("{workspaceRoot}/","").replaceAll("{projectRoot}",owner.root));else throw Error("Unknown selected named source input: "+value);}};
  for(const name of targets){json.targets[name]=normalized.targets[name];expand(normalized.targets[name].inputs??["default"]);}
  expand(inputNames);
  const exclusions=patterns.filter(value=>value.startsWith("!")).map(value=>value.slice(1)),matched=new Set(),walked=new Map(),matchers=new Map(),matches=(path,pattern)=>{let matcher=matchers.get(pattern);if(!matcher){matcher=loadedProjectInputs.createBuildInputPathMatcherV1(pattern);matchers.set(pattern,matcher);}return matcher(path);};let visited=0;
  const walk=async root=>{if(walked.has(root))return walked.get(root);const pending=[root],found=[];while(pending.length){await yieldInferenceV1(context,"build:selected-source-inputs",++visited);const directory=pending.pop();if(!existsSync(join(workspaceRoot,directory)))continue;for(const entry of readdirSync(join(workspaceRoot,directory),{withFileTypes:true})){await yieldInferenceV1(context,"build:selected-source-inputs",++visited);const path=buildPath(join(directory,entry.name));if(entry.isSymbolicLink())continue;if(entry.isDirectory()){if(![...POLICY.generatedDirectories,".git",".🧬semio",".nx",".cache","node_modules","dist","target","🗑️generated"].includes(entry.name)&&!(EXAMPLE_COLLECTIONS.includes(entry.name)&&directory.split("/").at(-1)!==WORKSPACE_OWNERSHIP.collections.moduleMember))pending.push(path);}else if(entry.isFile())found.push(path);}}walked.set(root,found);return found;};
  for(const pattern of patterns.filter(value=>!value.startsWith("!"))){await yieldInferenceV1(context,"build:selected-source-inputs",++visited);const index=pattern.search(/[?*[{(!]/u),prefix=index<0?pattern:pattern.slice(0,index),root=index<0?dirname(prefix):prefix.endsWith("/")?prefix.slice(0,-1):dirname(prefix),candidates=index<0?(existsSync(join(workspaceRoot,pattern))?[pattern]:[]):await walk(root||".");for(const path of candidates){await yieldInferenceV1(context,"build:selected-source-inputs",++visited);if(matches(path,pattern)&&!exclusions.some(pattern=>matches(path,pattern)))matched.add(path);}}
  for(const path of matched)addFile(path);
  await stage("source-closure");
  const modules=[...matched].filter(path=>/\.[cm]?[jt]sx?$/u.test(path)).map(path=>join(workspaceRoot,path)).filter(path=>!scripts.resources.has(path)||scripts.modules.has(path));
  for(const input of modules.length?await relativeScriptInputs(modules,workspaceRoot,scripts):[])if(typeof input==="string"&&input.startsWith("{workspaceRoot}/"))addFile(input.slice("{workspaceRoot}/".length));
 }
 const compiler=createRequire(import.meta.url)("typescript"),host=observedTypeScriptHost(compiler.sys),pending=[...scripts.modules].map(path=>buildPath(relative(workspaceRoot,path))),visited=new Set();let index=0;
 await stage("compile-closure");
 while(pending.length){await yieldInferenceV1(context,"build:selected-compile-imports",++index);const path=pending.pop();if(visited.has(path))continue;visited.add(path);const absolute=join(workspaceRoot,path);readPhysicalSource(absolute);
  for(const specifier of observedSourceAnalysisV1(prepareSourceAnalysesV1().module,absolute).imports){
   await yieldInferenceV1(context,"build:selected-compile-imports",++index);
   if(!specifier.startsWith(".")){const name=specifier.startsWith("@")?specifier.split("/").slice(0,2).join("/"):specifier.split("/")[0],dependency=byPackage.get(name);if(dependency&&dependency!==json.name)dependencies.add(dependency);continue;}
   const resource=buildImportResourceV1(absolute,specifier);let resolved;try{resolved=observedRequireResolve(absolute,resource.specifier);}catch(error){if(error.code!=="MODULE_NOT_FOUND")throw error;resolved=compiler.resolveModuleName(resource.specifier,absolute,{moduleResolution:compiler.ModuleResolutionKind.Bundler,allowJs:true,resolveJsonModule:true},host).resolvedModule?.resolvedFileName;if(!resolved)throw Error("Selected compile source import missing: "+path+" -> "+specifier,{cause:error});}
   const typed=resource.source?compiler.resolveModuleName(resource.specifier,absolute,{moduleResolution:compiler.ModuleResolutionKind.Bundler,allowJs:true,resolveJsonModule:true},host).resolvedModule?.resolvedFileName:undefined;
   for(const file of new Set([resolved,...typed?[typed]:[]])){await yieldInferenceV1(context,"build:selected-compile-imports",++index);const local=buildPath(relative(workspaceRoot,file));if(local===".."||local.startsWith("../")||isAbsolute(local))throw Error("Selected compile source escapes workspace: "+local);readPhysicalSource(file);addFile(local);if(resource.source&&/\.[cm]?[jt]sx?$/u.test(local)){addDependency(local);pending.push(local);}}
  }
 }
 for(const dependency of dependencies)for(const input of caretInputs)addInput(dependency,input);
 return {project:json,files:[...files].sort(),dependencies:[...dependencies].sort(),dependencyInputs:Object.fromEntries([...dependencyInputs].sort(([a],[b])=>a.localeCompare(b)).map(([name,inputs])=>[name,[...inputs].sort()]))};
}

/** 📜️ Reads the workspace's canonical owner declaration through original physical Source ancestry. */
export function readBuildOwnerRegistrySourceV1(workspaceRoot){return readPhysicalSource(join(workspaceRoot,"📋️build-owners.json")).toString("utf8");}

/** ⏱️ A source modified this recently may be half-written, so its parse failure is retried by settlement instead of withholding its project (the observation racy rule). */
const UNSETTLED_SOURCE_MS = 2000;

/** 🚧️ Withholds one project whose consumed source does not parse, naming the exact bytes the snapshot observed; a freshly modified source rethrows so settlement waits for it. */
function unavailableProjectV1(root, project, workspaceRoot, error) {
  let modified = Infinity;
  try { modified = rawStatSync(error.source).mtimeMs; } catch {}
  if (modified >= Date.now() - UNSETTLED_SOURCE_MS) throw error;
  return { root, ...(project ? { project } : {}), source: buildPath(relative(workspaceRoot, error.source)), sha256: error.sha256, reason: String(error.message).slice(0, 2048) };
}

/** 🧵️ Bounds concurrently inferred projects; one worker is the serial reference order. */
function projectConcurrencyV1(options) {
  const value = options?.projectConcurrency ?? Math.max(1, Math.min(8, availableParallelism()));
  if (!Number.isSafeInteger(value) || value < 1 || value > 64) throw Error("Build project inference concurrency must be an integer from 1 to 64");
  return value;
}

/** 🪨️ Reports and cancels inference settlement rounds through the original invocation. */
function settlementOptionsV1(context) {
  return { signal: context.phaseControl?.signal, onRound: async ({ round, conflict, failure }) => { if (process.env.SEMIO_BUILD_TRACE_AUTHORITY === "true") console.error("[DEBUG] build inference settlement round " + JSON.stringify({ round, conflict, failure: failure === undefined ? undefined : String(failure) })); await yieldInferenceV1(context,"build:settle",0,true); } };
}

async function emojiProjectJsonNodes(configFiles, options, context) {
  const { workspaceRoot } = context, unavailable = new Map();
  const catalogContributions=await catalogBuildContributionsV1(workspaceRoot,options?.catalogProviders??[]);
  const owners=await discoverNativeBuildOwnersV1(configFiles,workspaceRoot,context),{nativeCache,nativeClosures}=owners;configFiles=owners.files;
  const rootsByName = new Map(), scripts = createScriptInputCache(context);
  if(context.invocation)await advanceScriptInvocation(context.invocation,"build:commands","running");
  const commandInputs = !options?.ownerAdmission&&configFiles.some((path) => path.endsWith("Cargo.toml") && !path.includes(".🧬semio") && !POLICY.generatedDirectories.some((name) => path.split("/").includes(name))) ? await nativeCommandInputs(workspaceRoot, scripts) : undefined;
  if(context.invocation)await advanceScriptInvocation(context.invocation,"build:commands","complete");
  const contractPath = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
  const contracts = existsSync(contractPath) ? readTaxonomyProjectionV1(contractPath).generatorContracts : {};
  const projectAuthority=createHash("sha256").update(JSON.stringify([IMPLEMENTATION_REVISION,contracts])).digest("hex");
  const commandRevision=createHash("sha256").update(JSON.stringify(commandInputs??null)).digest("hex"),registryDigest=createHash("sha256").update(JSON.stringify(context.registry??null)).digest("hex");
  const playgroundCatalog = collectPlaygroundCatalog(configFiles, workspaceRoot,catalogContributions);

  const results = (await Promise.all(configFiles
    .filter((file) => file.endsWith(PROJECT_BASENAME))
    .filter((configFile) => {
      // 🛡️ Nx's native walker sometimes hands back lossy paths with U+FFFD where a
      // multi-byte emoji used to be; those are unopenable duplicates of a real file.
      if (configFile.includes("\uFFFD")) return false;
      if (configFile.includes(".🧬semio") || POLICY.generatedDirectories.some((name) => configFile.split("/").includes(name))) {
        return false;
      }
      return true;
    })
    .map(async (configFile) => {
      const abs = join(workspaceRoot, configFile);
      if (!existsSync(abs)) return null;
      const bytes = readFileSync(abs);
      let json;
      try {
        json = JSON.parse(bytes.toString("utf8"));
      } catch (error) {
        const root = buildPath(relative(workspaceRoot, dirname(abs))) || ".";
        unavailable.set(root, unavailableProjectV1(root, undefined, workspaceRoot, Object.assign(Error(`Invalid Build project metadata ${configFile}: ${error.message}`), { code: "BUILD_SOURCE_UNPARSEABLE", source: abs, sha256: createHash("sha256").update(bytes).digest("hex") })));
        return null;
      }
      const name = json.name;
      if (!name) throw new Error(`Build project metadata ${configFile} requires a name`);
      const projectDir = dirname(abs);
      const root = buildPath(relative(workspaceRoot, projectDir)) || ".";
      const prior = rootsByName.get(name);
      if (prior !== undefined && prior !== root) throw new Error(`Duplicate Build project ${name}: ${prior} and ${root}`);
      if (prior === undefined) rootsByName.set(name, root);
      if (name === "@semio-tech/plugin-registry") json.targets = { ...json.targets, ...playgroundSessionTargets(configFiles, workspaceRoot,catalogContributions) };
      if (name === "@semio-tech/framework-os-dev") json.targets = { ...json.targets, ...await playgroundPreparationTargets(configFiles, workspaceRoot, root, contracts,catalogContributions,context) };
      if (playgroundCatalog.some(row => row.cratePath === root)) {
        json.targets = { ...playgroundSiteTargetsForAdmissionV1(root,playgroundCatalog,workspaceRoot,context.registry,options?.ownerAdmission?options.ownerAdmission.targets[name]??[]:undefined), ...json.targets };
      }
      const canonicalConfig = buildPath(relative(workspaceRoot, join(projectDir, PROJECT_BASENAME)));
      return [canonicalConfig, { projects: { [name]: {...json,root} } }];
    })))
    .filter(Boolean);
  const declaredRoots = new Set([...rootsByName.values()]);let cargoIndex=0;
  for (const configFile of configFiles.filter((file) => file.endsWith("Cargo.toml"))) {
    await yieldInferenceV1(context,"build:cargo-targets",++cargoIndex);
    if (configFile.includes("\uFFFD") || configFile.startsWith("compose/") || configFile.startsWith("temp/compose/") || configFile.includes(".🧬semio") || POLICY.generatedDirectories.some((name) => configFile.split("/").includes(name))) continue;
    if (!existsSync(join(workspaceRoot, configFile))) continue;
    const projectDir = dirname(join(workspaceRoot, configFile));
    const root = buildPath(relative(workspaceRoot, projectDir)) || ".";
    if (declaredRoots.has(root)) continue;
    let manifest;
    try { manifest = readToml(join(projectDir, "Cargo.toml")); }
    catch (error) { if (error?.code !== "BUILD_SOURCE_UNPARSEABLE") throw error; unavailable.set(root, unavailableProjectV1(root, undefined, workspaceRoot, error)); continue; }
    if (!manifest.package?.name) continue;
    const name = manifest.package.name;
    if (rootsByName.has(name)) throw new Error(`Duplicate Build project ${name}: ${rootsByName.get(name)} and ${root}`);
    rootsByName.set(name, root);
    const targets = await cargoTargets(root, workspaceRoot, commandInputs,scripts);
    const project = { name, root, projectType: "library", tags: ["language:cargo", "discovery:manifest"], targets: { ...targets, ...componentBuildTargets(root, workspaceRoot), ...playgroundSiteTargetsForAdmissionV1(root,playgroundCatalog,workspaceRoot,context.registry,options?.ownerAdmission?options.ownerAdmission.targets[name]??[]:undefined) } };
    results.push([configFile, { projects: { [name]: project } }]);
  }
  const projects = results.flatMap(([, result]) => Object.values(result.projects));
  const projectsByRoot = new Map(projects.flatMap(project=>[[project.root,project.name],...(project.metadata?.nativeRoot?[[project.metadata.nativeRoot,project.name]]:[])]));
  const componentRoots = componentCrateRoots(configFiles, workspaceRoot);

  const jobs=results.flatMap(([,result])=>Object.entries(result.projects).map(([name,json])=>({result,name,json})));let projectIndex=0,cursor=0,stopped=false;
  const inferProject=async({result,name,json})=>{
    const started=performance.now();
    await yieldInferenceV1(context,"build:projects",++projectIndex);
    if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true")console.error("[DEBUG] build project start "+JSON.stringify({index:projectIndex,total:projects.length,project:name,root:json.root}));
    const inactive=options?.ownerAdmission?Object.fromEntries(Object.entries(json.targets??{}).filter(([name])=>!options.ownerAdmission.targets[json.name]?.includes(name))):{};
    const selectedTargets=options?.ownerAdmission?.targets[json.name],activeJson=options?.ownerAdmission?{...json,targets:Object.fromEntries(Object.entries(json.targets??{}).filter(([name])=>selectedTargets?.includes(name)))}:json;
    const root=json.root, nativeRoot=json.metadata?.nativeRoot??root, native=existsSync(join(workspaceRoot,nativeRoot,"Cargo.toml"))||Object.values(activeJson.targets??{}).some(target=>target.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts")), pipeline=Object.values(activeJson.targets??{}).some(target=>target.metadata?.semio?.release);
    const assemblyAuthority=native||pipeline?projectAssemblyAuthorityV1(workspaceRoot,native?nativeRoot:undefined,projectsByRoot,pipeline?componentRoots:undefined,nativeCache,nativeClosures):null;
    const authorityMilliseconds=performance.now()-started;
    const cachePath=join(workspaceDataDirectory(workspaceRoot),"np",createHash("sha256").update(root).digest("hex").slice(0,24)+".json");
    result.projects[name]=await observeProjectInferenceV1({cachePath,key:[projectAuthority,registryDigest,native?commandRevision:null,native||pipeline?assemblyAuthority:null,root,json,selectedTargets??null],owner:root},async()=>{
      try{
      const sourceGroups=await declaredSourceInputs(activeJson,workspaceRoot,context);
      await yieldInferenceV1(context,"build:project-assembly",0);
      const project=await projectWithDefaults(json,root,join(workspaceRoot,root),workspaceRoot,contracts,new Map(),commandInputs,scripts,sourceGroups,"hash",options?.ownerAdmission?selectedTargets??[]:undefined),preparationCache=nativeCache,dependencyRootsCache=nativeClosures;
      withNativePreparation(project,workspaceRoot,contracts,preparationCache,projectsByRoot,dependencyRootsCache);
      withReleasePipelinePreparation(project,workspaceRoot,contracts,componentRoots,preparationCache,dependencyRootsCache);
      for(const[name,target]of Object.entries(project.targets))project.targets[name]=targetBuildModes(name,target);
      return project;
      }catch(error){if(error?.code!=="BUILD_SOURCE_UNPARSEABLE")throw error;return {unavailable:unavailableProjectV1(root,name,workspaceRoot,error)};}
    });
    if(result.projects[name].unavailable){unavailable.set(root,result.projects[name].unavailable);delete result.projects[name];return;}
    result.projects[name].targets={...inactive,...result.projects[name].targets};
    if(process.env.SEMIO_BUILD_TRACE_AUTHORITY==="true")console.error("[DEBUG] build project complete "+JSON.stringify({index:projectIndex,total:projects.length,project:name,authorityMilliseconds:Math.round(authorityMilliseconds),elapsedMilliseconds:Math.round(performance.now()-started)}));
  };
  const workers=Array.from({length:Math.min(projectConcurrencyV1(options),jobs.length)},async()=>{while(!stopped&&cursor<jobs.length)await inferProject(jobs[cursor++]);});
  await Promise.all(workers).catch(async error=>{stopped=true;await Promise.allSettled(workers);throw error;});
  for(const row of [...unavailable.values()].sort((a,b)=>a.root<b.root?-1:a.root>b.root?1:0))results.push([row.source,{projects:{},unavailable:[row]}]);
  if (options?.analyzeLockfile && existsSync(join(workspaceRoot, "bun.lock"))) results.push(["bun.lock", { externalNodes: readBunLockGraph(workspaceRoot).externalNodes }]);
  return results;
}

/** 🗂️ Locates immutable build inference source facts. */
function workspaceDataDirectory(workspaceRoot) {
  return repositoryBuildGraphWriterDirectoryV1(workspaceRoot,"inference");
}

/** 📚️ Admits one packed semantic record directory only when every physical temporary and pack path fits. */
function moduleSourceRecordCacheRootV1(workspaceRoot){
 const root=resolve(workspaceDataDirectory(workspaceRoot),"mr"),path=join(root,"0".repeat(64)+".json");
 return path.length<=256&&[...path].length<=256?root:undefined;
}

/** 🏛️ Snapshots current project roots once, in deterministic ownership precedence. */
function importProjectOwners(projects) {
  return Object.entries(projects).filter(([name]) => name !== "workspace").map(([name, project]) => ({ name, root: buildPath(project.root) })).sort((a, b) => b.root.length - a.root.length || a.name.localeCompare(b.name));
}

/** 🧭️ Uses bare and explicit core loader identity so bundled packages retain their package identity. */
function coreModuleImport(specifier) {
  try {
    const loader = createRequire(import.meta.url);
    if (specifier.startsWith("node:")) return loader.resolve(specifier) === specifier;
    if (!isBuiltin(specifier)) return false;
    const core = "node:" + specifier;
    return loader.resolve(specifier) === specifier && loader.resolve(core) === core;
  } catch { return false; }
}

/** 🔗️ Resolves immutable runtime import strings against current ownership, package names, and lock scope. */
function importTargetsFromFacts(imports, file, workspaceRoot, owners, byPackage, locked) {
  const targets = new Set();
  for (const specifier of imports) {
    if (specifier.startsWith(".")) {
      const path = buildPath(relative(workspaceRoot, resolve(dirname(join(workspaceRoot, file)), specifier)));
      const target = owners.find(project => owned(path, project.root))?.name;
      if (target) targets.add(target);
      continue;
    }
    const packageName = specifier.startsWith("@") ? specifier.split("/").slice(0, 2).join("/") : specifier.split("/")[0];
    if (coreModuleImport(packageName)) continue;
    const workspaceTarget = byPackage.get(packageName);
    if (workspaceTarget) { targets.add(workspaceTarget); continue; }
    const key = locked?.resolveImport?.(file, packageName);
    if (key && locked?.externalNodes?.[`npm:${key}`]) targets.add(`npm:${key}`);
  }
  return [...targets].sort();
}

/** 🔍️ Parses one module through the canonical owned string interface and current project authority. */
function importTargetsFromSource(text, file, workspaceRoot, projects, byPackage, locked) {
  return importTargetsFromFacts(moduleSourceImports(file, text), buildPath(file), workspaceRoot, importProjectOwners(projects), byPackage, locked);
}

/** ⚡️ Reuses bounded source facts while resolving every full or incremental file against current authority. */
async function collectImportEdges(workspaceRoot, projectFiles, projects, byPackage, locked, add, publication, scheduledFiles,context={}) {
  const owners=importProjectOwners(projects),jobs=[];
  for(const[name,files]of Object.entries(projectFiles))for(const file of files)if(/\.[cm]?[jt]sx?$/.test(file.file))jobs.push({name,file});
  let cursor=0;
  await Promise.all(Array.from({length:Math.min(32,jobs.length)},async()=>{
    for(;;){
      const index=cursor++;if(index>=jobs.length)return;await yieldInferenceV1(context,"build:modules",index+1);
      const{name,file}=jobs[index],sourceFile=buildPath(file.file);
      let fact;try{fact=await observedSourceAnalysisAsyncV1(prepareSourceAnalysesV1().module,join(workspaceRoot,sourceFile));}catch(error){if(error.code==="ENOENT")continue;throw error;}
      const{imports,productionImports}=fact;
      if(publication)(publication[name]??={version:1,files:[]}).files.push({file:sourceFile,imports,productionImports});
      if(name!=="workspace"&&(!scheduledFiles||scheduledFiles.has(sourceFile)))for(const target of importTargetsFromFacts(context.completeSourceImports?imports:productionImports,sourceFile,workspaceRoot,owners,byPackage,locked))add(name,target,sourceFile);
    }
  }));
  if(publication)for(const value of Object.values(publication))value.files.sort((a,b)=>a.file.localeCompare(b.file));
}

/** 📇️ Publishes complete captured module facts after the original dependency phase. */
function createMetadataImplementation(graph,_options,context){
  const publication=moduleGraphPublications.get(moduleGraphPublicationKeyV1(context.workspaceRoot));
  const owners=Object.entries(graph.nodes).filter(([,node])=>node.type!=="npm").map(([name,node])=>[name,buildPath(node.data.root)]).sort(([a],[b])=>a.localeCompare(b));
  if(!publication||publication.parser!==commandInputHash||JSON.stringify(publication.owners)!==JSON.stringify(owners))throw Error("Original full module graph producer required");
  return Object.fromEntries(owners.map(([name])=>[name,{metadata:{semio:{...graph.nodes[name].data.metadata?.semio,moduleGraph:structuredClone(publication.projects[name]??{version:1,files:[]})}}}]));
}

/**
 * 🧭️ Selects changed module files from the original file map, or the complete owned file roster.
 * @param {{ fileMap?: { projectFileMap?: Record<string, { file: string, hash?: string }[]> }, filesToProcess?: { projectFileMap?: Record<string, { file: string, hash?: string }[]> } }} context
 */
function projectFilesToProcess(context) {
  return context.filesToProcess?.projectFileMap ?? context.fileMap?.projectFileMap ?? {};
}

/** 🏭️ Validates complete authored producers independently of the selected executable projection. */
async function validateGeneratorOutputProducersV1(contracts,projects,workspaceRoot,selection,registry,context){
 const check=()=>{if(context.workspaceRoot!==workspaceRoot)throw Error("Actual generator inference workspace context required");if(context.invocation){requireIssuedResourceControlInvocationV1(context.phaseControl,context.invocation);checkScriptInvocation(context.invocation);checkResourceLeaseControl(context.phaseControl);}};check();
 await libraryBootstrap;check();const admission=createRequire(import.meta.url)("../🚪️admission/🟦️.ts"),selected=admission.readBuildGraphSelectionV1(selection).kind==="tasks",owners=selected?admission.readBuildOwnerRegistryV1(registry):undefined,byRoot=new Map(Object.entries(projects).flatMap(([name,project])=>[[resolve(workspaceRoot,project.root),name],...(project.metadata?.nativeRoot?[[resolve(workspaceRoot,project.metadata.nativeRoot),name]]:[])])),authored=new Map();let steps=0;
 for(const contract of Object.values(contracts)){
  const participants=[contract.ownerPath,...contract.outputRoots.flatMap(output=>output.producer?[output.producer.ownerPath]:[])];if(!participants.some(root=>typeof root==="string"&&byRoot.has(resolve(workspaceRoot,root))))continue;
  for(const output of contract.outputRoots){if(!output.producer)continue;await yieldInferenceV1(context,"build:generator-producers",++steps);const producer=output.producer,separator=producer.target.lastIndexOf(":"),name=producer.target.slice(0,separator),target=producer.target.slice(separator+1);let valid;
   if(selected){
    const owner=owners.owners.find(owner=>owner.name===name&&(owner.root===producer.ownerPath||owner.nativeRoots?.includes(producer.ownerPath)));if(!owner)throw Error(`Generator output has no Build producer: ${output.path}`);
    let project=authored.get(name);if(!project){const path=join(workspaceRoot,owner.manifest),bytes=readPhysicalSource(path),cargo=owner.manifest.endsWith("Cargo.toml"),declared=cargo?readToml(path):JSON.parse(bytes.toString("utf8"));project=cargo?{name:declared.package?.name,root:owner.root,targets:await cargoTargets(owner.root,workspaceRoot,undefined,createScriptInputCache(context))}:declared;check();if(project.name!==owner.name||project.root!==undefined&&project.root!==owner.root)throw Error(`Generator output has no Build producer: ${output.path}`);authored.set(name,project);}
    valid=Boolean(project.targets?.[target]);
   }else valid=byRoot.get(resolve(workspaceRoot,producer.ownerPath))===name&&Boolean(projects[name]?.targets?.[target]);
   if(!valid)throw Error(`Generator output has no Build producer: ${output.path}`);
  }
 }
 check();
}

/** 🕸️ Native manifests and package imports contribute edges without spawning a build or installer. */
async function createDependenciesImplementation(_options, context) {
  const { workspaceRoot, projects } = context;
  const locked = _options?.analyzeLockfile && existsSync(join(workspaceRoot, "bun.lock")) ? readBunLockGraph(workspaceRoot) : undefined;
  const byRoot = new Map(Object.entries(projects).flatMap(([name, project]) => [[resolve(workspaceRoot, project.root), name],...(project.metadata?.nativeRoot?[[resolve(workspaceRoot,project.metadata.nativeRoot),name]]:[])]));
  const byPackage = new Map();
  const manifests = new Map();
  const goModules = new Map();
  const goManifests = new Map();
  const edges = new Map();
  const workspace = existsSync(join(workspaceRoot, "Cargo.toml")) ? readToml(join(workspaceRoot, "Cargo.toml")).workspace ?? {} : {};
  const add = (source, target, sourceFile, basenameHint) => {
    if (!target || source === target) return;
    const tracked = buildTrackedSourceFile(context, source, sourceFile, basenameHint);
    if (!existsSync(join(workspaceRoot, tracked))) return;
    edges.set(`${source}\0${target}\0${tracked}`, { source, target, sourceFile: tracked, type: "static" });
  };
  const authorityPath = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
  const generators = existsSync(authorityPath) ? readTaxonomyProjectionV1(authorityPath).generatorContracts : {};
  await validateGeneratorOutputProducersV1(generators,projects,workspaceRoot,context.selection??{kind:"all"},context.registry,context);
  for (const [name, project] of Object.entries(projects)) {
    const go = goManifest(project.root, workspaceRoot);
    if (go) {
      const manifest = goDependencies(readFileSync(go, "utf8"));
      goManifests.set(name, { path: go, ...manifest });
      goModules.set(manifest.module, name);
    }
    const packageFile = join(workspaceRoot, project.root, "package.json");
    if (existsSync(packageFile)) {
      const manifest = JSON.parse(readFileSync(packageFile, "utf8"));
      manifests.set(name, manifest);
      if (manifest.name) byPackage.set(manifest.name, name);
    }
  }
  const projectFiles = projectFilesToProcess(context);
  for (const [name, files] of Object.entries(projectFiles)) {
    const project = projects[name];
    if (!project || name === "workspace") continue;
    const root = resolve(workspaceRoot, project.root);
    for (const file of files) {
      if (file.file.endsWith("Cargo.toml")) {
        const cargo = join(workspaceRoot, file.file);
        if (!existsSync(cargo)) continue;
        const manifest=readToml(cargo),directory=dirname(cargo),authority=nativeWorkspaceV1(directory,workspaceRoot,manifest);
        for (const dependency of nativeDependencies(manifest, authority.declaration)) {
          if(dependency.kind === "dev-dependencies")continue;
          add(name, byRoot.get(resolve(dependency.workspace ? authority.root : directory, dependency.path)), file.file);
        }
      }
      if (file.file.endsWith("package.json")) {
        const manifest = manifests.get(name) ?? (existsSync(join(workspaceRoot, file.file)) ? JSON.parse(readFileSync(join(workspaceRoot, file.file), "utf8")) : undefined);
        if (!manifest) continue;
        for (const dependency of Object.keys({ ...manifest.dependencies, ...manifest.peerDependencies, ...manifest.optionalDependencies })) {
          const key = locked?.resolve(locked.workspacePackages.has(manifest.name) ? manifest.name : "", dependency);
          add(name, byPackage.get(dependency) ?? (locked?.workspacePackages.has(key) ? byRoot.get(resolve(workspaceRoot, locked.workspacePackages.get(key))) : key && locked.externalNodes[`npm:${key}`] ? `npm:${key}` : undefined), file.file);
        }
      }
    }
    const go = goManifests.get(name);
    if (go) {
      const provenance = buildPath(relative(workspaceRoot, go.path));
      if (files.some((file) => file.file === provenance || file.file.endsWith("go.mod"))) {
        const sourceFile = owned(provenance, project.root) ? provenance : buildPath(relative(workspaceRoot, join(root, SCRIPT_BASENAME)));
        for (const dependency of go.requires) add(name, goModules.get(dependency), sourceFile);
        for (const replacement of go.replacements) add(name, goModules.get(replacement.module) ?? byRoot.get(resolve(dirname(go.path), replacement.path)), sourceFile);
      }
    }
  }
  const publication={},fullFiles=context.fileMap?.projectFileMap??projectFiles;
  await collectImportEdges(workspaceRoot,fullFiles,projects,byPackage,locked,add,publication,new Set(Object.values(projectFiles).flat().map(file=>buildPath(file.file))),context);
  moduleGraphPublications.set(moduleGraphPublicationKeyV1(workspaceRoot),{parser:commandInputHash,owners:Object.entries(projects).map(([name,project])=>[name,buildPath(project.root)]).sort(([a],[b])=>a.localeCompare(b)),projects:publication});
  return [...edges.values(), ...(locked?.dependencies ?? [])];
}

/** ♻️ Identifies the complete authored graph implementation and policy authority. */
function implementationRevision() {
  const hash = createHash("sha256").update(readPhysicalSource(fileURLToPath(import.meta.url)));
  for (const path of ["⚡️caching/🔣️policy.json", "🔣️taxonomy.json", TARGET_EXECUTION_SCHEMA, COMMAND_INPUT_MODULE, OWNER_ARGUMENTS_MODULE, OBSERVATION_MODULE, CARGO_EXECUTION_MODULE, CARGO_EXECUTION_SCHEMA, BUILD_MODE_MODULE, BUILD_MODE_API, PROJECT_INPUT_MODULE, "🏗️build/📥️inputs/💾️storage/🟨️.mjs", "🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json", "🏗️build/📥️inputs/💾️storage/🧬️schema/📦️dataset.json", "🏗️build/📥️inputs/💾️storage/🧬️schema/💾️quota.json", "⚡️caching/🗃️records/🟦️.ts", "⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json", RUNTIME_COMPONENT_MODULE, SOURCE_INPUT_MODULE, BROWSER_SESSION_MODULE, PLAYGROUND_COMPOSITION_MODULE, NATIVE_HOST_MODULE, COMPONENT_DEPLOYMENT_MODULE, "../../../../🔨️modules/🪪️identity/📁️installation/🟨️.mjs", "../../../../🔨️modules/🪪️identity/📁️installation/🧬️schema/🔣️.json"]) hash.update(path === "🔣️taxonomy.json" ? JSON.stringify(readTaxonomyProjectionV1(join(LIBRARY_ROOT, path))) : readPhysicalSource(join(LIBRARY_ROOT, path)));
  hash.update(JSON.stringify(buildModesV1()));
  return hash.digest("hex");
}

let reloadedImplementation;

function invokeCurrentImplementation(kind, args) {
  if(kind!=="nodes"&&kind!=="dependencies")return invokeCurrentImplementationV1(kind,args);
  const index=kind==="nodes"?2:1,context=args[index],run=(invocation,phaseControl)=>withRepositoryBuildGraphFactStoreV1(context.workspaceRoot,invocation,phaseControl,(_store,actualControl)=>{const current=[...args];current[index]={...context,invocation,phaseControl:actualControl};return invokeCurrentImplementationV1(kind,current);});
  return context.invocation?run(context.invocation,context.phaseControl):receiveScriptProcessInvocation(process.env,invocation=>run(invocation,createIssuedResourceControlV1(invocation,"graph:inference-entry",0)));
}

function invokeCurrentImplementationV1(kind, args) {
  return libraryBootstrap.then(() => {
    const revision = implementationRevision();
    if (revision === IMPLEMENTATION_REVISION && kind === "metadata")return createMetadataImplementation(...args);
    if (revision === IMPLEMENTATION_REVISION) return kind === "nodes" ? observeInference({ cachePath: join(workspaceDataDirectory(args[2].workspaceRoot), "inference", "emoji-project-json.json"), key: [revision, args[0], args[1] ?? null, resolve(args[2].workspaceRoot)] }, () => settleObservedComputationV1(() => emojiProjectJsonNodes(...args), settlementOptionsV1(args[2]))) : kind === "authority" ? dependencyResolutionAuthorityImplementation(...args) : observeDependencies({ receiptPath: join(workspaceDataDirectory(args[1].workspaceRoot), "inference", "emoji-project-json.receipt.json"), recordDirectory:moduleSourceRecordCacheRootV1(args[1].workspaceRoot) }, () => settleObservedComputationV1(() => createDependenciesImplementation(...args), settlementOptionsV1(args[1])));
    if (reloadedImplementation?.revision !== revision) {
      const entry = { revision, module: importRevision(import.meta.url, revision).catch((error) => { if (reloadedImplementation === entry) reloadedImplementation = undefined; throw error; }) };
      reloadedImplementation = entry;
    }
    return reloadedImplementation.module.then((module) => {
      if (module.cacheInternals === cacheInternals) throw new Error("Graph runtime retained an obsolete implementation");
      return kind === "metadata" ? module.createMetadata(...args) : kind === "nodes" ? module.default.createNodesV2[1](...args) : kind === "authority" ? module.dependencyResolutionAuthority(...args) : module.createDependencies(...args);
    });
  });
}

/** 🧬️ Identifies current dependency resolution authority using only owned immutable source facts.
 * @param {string} workspaceRoot
 * @param {Record<string, { root: string }>} projects
 * @returns {string}
 */
function dependencyResolutionAuthorityImplementation(workspaceRoot, projects) {
  const hash = createHash("sha256").update(implementationRevision());
  const source = (path) => { hash.update(JSON.stringify(buildPath(relative(workspaceRoot, path)))); hash.update(existsSync(path) ? readPhysicalSource(path) : "\0absent\0"); };
  for (const path of ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json", "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧬️schema/🔣️.json", "Cargo.toml", "go.work", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🟨️.mjs"]) source(join(workspaceRoot, path));
  const recipe = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools"), manifest = join(recipe, "package.json");
  for (const path of [join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts"), join(recipe, "📜️script.ts"), manifest, join(recipe, "bun.lock")]) source(path);
  if (existsSync(manifest)) for (const path of [...new Set(Object.values(JSON.parse(readPhysicalSource(manifest)).semio?.toolPatches ?? {}))].sort()) {
    if (typeof path !== "string") throw new Error("Invalid tooling authority patch path");
    const file = resolve(workspaceRoot, path), local = buildPath(relative(workspaceRoot, file));
    if (isAbsolute(local) || local === ".." || local.startsWith("../")) throw new Error("Tooling authority patch must belong to this workspace");
    source(file);
  }
  if (existsSync(join(workspaceRoot, "bun.lock"))) { readBunLockGraph(workspaceRoot); hash.update(BUN_LOCK_CACHE.hash); }
  else hash.update("\0no-bun-lock\0");
  for (const [name, project] of Object.entries(projects).sort(([a], [b]) => a.localeCompare(b))) {
    hash.update(JSON.stringify([name, buildPath(project.root)]));
    for (const filename of ["package.json", "📋️project.json", "Cargo.toml"]) source(join(workspaceRoot, project.root, filename));
    const go = goManifest(project.root, workspaceRoot);
    if (go) source(go);
  }
  return hash.digest("hex");
}

/** 🏛️ Supplies current roots, manifests, lock scope, and actual implementation identity behind an owned digest interface. */
export function dependencyResolutionAuthority(...args) { return invokeCurrentImplementation("authority", args); }


export function createDependencies(...args) { return invokeCurrentImplementation("dependencies", args); }

/** 📇️ Exposes original complete module ownership independently of scheduling dependencies. */
export function createMetadata(...args){return invokeCurrentImplementation("metadata",args);}

/**
 * 🧩️ One Build project configuration this plugin generates, as its consumers read it.
 * @typedef {{ targets: Record<string, { cache?: boolean, continuous?: boolean, dependsOn?: string[], outputs?: string[], inputs?: unknown[], namedInputs?: Record<string, unknown[]>, options?: Record<string, unknown>, metadata?: Record<string, unknown> }>, root?: string, name?: string, projectType?: string, tags?: string[], sourceRoot?: string }} GeneratedProject
 */

/**
 * 🧩️ The `createNodesV2` result: one entry per matched file, each carrying the projects it contributes.
 * @typedef {[string, { projects: Record<string, GeneratedProject> }][]} GeneratedNodes
 */

/**
 * 🎛️ Receives the OS invocation or preserves an explicitly supplied Original and its mandatory issued phase.
 * @typedef {{workspaceRoot:string} & ({invocation?:undefined,phaseControl?:undefined}|{invocation:import("../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts").ScriptInvocation,phaseControl:import("../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts").ResourceLeaseControl})} BuildInferenceContextV1
 */

export default {
  name: "@repo/emoji-project-json",
  createNodesV2: /** @type {[string, (files: readonly string[], options: unknown, context: BuildInferenceContextV1) => Promise<GeneratedNodes>]} */ ([`**/{${PROJECT_BASENAME},Cargo.toml,bun.lock,*.patch}`, (...args) => invokeCurrentImplementation("nodes", args)]),
  createDependencies,
  createMetadata,
};

export { libraryBootstrap };

export const cacheInternals = { sourceAnalysisOwnerIdentityV1, importRevision, validateGeneratorOutputProducersV1, targetScriptSources, playgroundSiteTargetsForAdmissionV1, catalogNativeManifestCandidatesV1, targetWithDefaults, projectAssemblyAuthorityV1, implementationRevision, catalogBuildContributionsV1, workspaceCatalogBuildContributionsV1, walkCargoTomlControlledV1, targetExecutionKindV1, inferenceCandidates, async declaredSourceInputs(...args) { await libraryBootstrap; return declaredSourceInputs(...args); }, nativeLockInputs, withWasmTooling, get runtimeComponentClosure() { return runtimeComponentClosure; }, async playgroundSessionTargets(...args) { await libraryBootstrap; return playgroundSessionTargets(...args); }, playgroundPreparationTargets, collectPlaygroundCatalog, pluginSiteTargetsForCrate, bunLockGraph, readBunLockGraph, printDocumentTargets, targetPolicy, matchesUncached, cacheableFamily, mutatingName, liveName, verifyCommand, nativeDependencies, nativeDependencyRoots, nativePreparation, withNativePreparation, withReleasePipelinePreparation, componentCrateRoots, async componentBuildTargets(...args) { await libraryBootstrap; return componentBuildTargets(...args); }, cargoTargets, goDependencies, rustSourceFiles, createRustSourceCache, relativeScriptInputs, nativeTargetCommandInputs, targetScriptClosure, genericTargetCommandInputs, genericCommandFallbackInputs, generatorContractInputs, outputRootInputs, resolveOutputPath, generatorOutputCouplingInputs, projectInputs, rootCommandTargets, createDependenciesImplementation, createMetadataImplementation, importTargetsFromSource, collectImportEdges, projectFilesToProcess, moduleSourceRecordCacheRootV1, buildTrackedSourceFile, walkCargoToml };

/** 🏗️ Discovers owned project contributions without loading a task-runner runtime. */
export async function discoverBuildProjectsV1(files, options, context) {
  return invokeCurrentImplementation("nodes",[files,options,context]);
}

/** 🪢️ Resolves owned dependency contributions without loading a task-runner runtime. */
export async function discoverBuildDependenciesV1(options, context) {
  return invokeCurrentImplementation("dependencies",[options,context]);
}
