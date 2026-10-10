import { existsSync, lstatSync, mkdirSync, readFileSync, readdirSync, realpathSync, statSync, writeFileSync } from "node:fs";
import { readFile } from "node:fs/promises";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { createRequire, isBuiltin } from "node:module";
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";
import * as loadedCommandInputs from "./⚡️caching/📥️inference/🟨️.mjs";
import {declaredNativeOwnerCommandV1} from "./🔌️nx-plugin/📤️arguments/🟨️.mjs";

const PROJECT_BASENAME = "📋️project.json";
const SCRIPT_BASENAME = "📜️script.ts";
const DEFAULT_EXECUTOR = "nx:run-commands";
const TEST_TARGET = "test";
const TEST_LEVELS = ["quick", "long", "exhaustive"];

const LIBRARY_ROOT = dirname(fileURLToPath(import.meta.url));
const RUNTIME_COMPONENT_MODULE = "🕸️dependencies/🧩️runtime/🟨️.mjs";
const NATIVE_HOST_MODULE="🎮️playground/🖥️native-host/🟨️.mjs";
const COMPONENT_DEPLOYMENT_MODULE = "📇️catalog/🚚️deployment/🟨️.mjs";
const INSTALLATION_IDENTITY_MODULE = "../../../../🔨️modules/🪪️identity/📁️installation/🟨️.mjs";
const INSTALLATION_IDENTITY_SCHEMA = "../../../../🔨️modules/🪪️identity/📁️installation/🧬️schema/🔣️.json";
const BROWSER_SESSION_MODULE = "../../../💻️os/🔨️modules/🧑‍💻dev/⚙️engine/🧭️selection/🟨️.mjs";
const PLAYGROUND_COMPOSITION_MODULE = "../../../💻️os/🔨️modules/🔌️plugin/📇️registry/🎮️playground/🧩️composition/🟨️.mjs";
const SOURCE_INPUT_MODULE = "🕸️dependencies/🟦️typescript/🟨️.mjs";
const COMMAND_INPUT_MODULE = "⚡️caching/📥️inference/🟨️.mjs";

/** 📄️ Reads policy and implementation authority through current real ancestors and leaf. */
function readPhysicalSource(path) {
  const ancestors = [];
  for (let directory = dirname(path); ; directory = dirname(directory)) { ancestors.push(directory); if (directory === dirname(directory)) break; }
  for (const directory of ancestors.reverse()) { const value = lstatSync(directory); if (!value.isDirectory() || value.isSymbolicLink()) throw Error(`Nx inference requires real authority ancestry: ${directory}`); }
  const value = lstatSync(path);
  if (!value.isFile() || value.isSymbolicLink()) throw Error(`Nx inference requires real authority file: ${path}`);
  return readFileSync(path);
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

let declaredBrowserSessionEnginesV1;
let playgroundCompositionPathV1;
let admitPlaygroundNativeHostV1;
let nativeHostSourceFactsV1;
let runtimeComponentClosure;
let runtimeInputAdmissionV1;
let readSourceInputContract;
let relativeSourceInputs;
let componentDeploymentDirectoryV1;
const libraryBootstrap = (loadedCommandInputs.sourceHash === commandInputHash ? Promise.resolve(loadedCommandInputs) : importRevision(new URL(`./${COMMAND_INPUT_MODULE}`, import.meta.url), commandInputHash)).then((commandInputs) => {
  const admitted = currentCommandInputs(commandInputs);
  commandSourceImports = admitted.commandSourceImports;
  moduleSourceImports = admitted.moduleSourceImports;
  return importRevision(new URL(`./${RUNTIME_COMPONENT_MODULE}`, import.meta.url), createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, RUNTIME_COMPONENT_MODULE))).digest("hex"));
}).then((runtime) => {
  runtimeComponentClosure = runtime.runtimeComponentClosure;
  runtimeInputAdmissionV1 = runtime.runtimeInputAdmissionV1;
  return importRevision(new URL(`./${SOURCE_INPUT_MODULE}`, import.meta.url), createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, SOURCE_INPUT_MODULE))).digest("hex"));
}).then((source) => {
  readSourceInputContract = source.readSourceInputContract;
  relativeSourceInputs = source.relativeSourceInputs;
  return importRevision(new URL(`./${NATIVE_HOST_MODULE}`,import.meta.url),createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT,NATIVE_HOST_MODULE))).digest("hex"));
 }).then(async nativeHost => {
  admitPlaygroundNativeHostV1 = nativeHost.admitPlaygroundNativeHostV1;
  nativeHostSourceFactsV1 = nativeHost.nativeHostSourceFactsV1;
  const revision = createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, COMPONENT_DEPLOYMENT_MODULE))).update(readPhysicalSource(join(LIBRARY_ROOT, INSTALLATION_IDENTITY_MODULE))).update(readPhysicalSource(join(LIBRARY_ROOT, INSTALLATION_IDENTITY_SCHEMA))).digest("hex");
  const identity = await importRevision(new URL(INSTALLATION_IDENTITY_MODULE, import.meta.url), revision);
  const admit = identity.installationDirectoryParserV1(JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, INSTALLATION_IDENTITY_SCHEMA)).toString("utf8")));
  const deployment = await importRevision(new URL(`./${COMPONENT_DEPLOYMENT_MODULE}`, import.meta.url), revision);
  componentDeploymentDirectoryV1 = metadata => deployment.componentDeploymentDirectoryV1(metadata, admit);
  const browser = await importRevision(new URL(BROWSER_SESSION_MODULE, import.meta.url), createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, BROWSER_SESSION_MODULE))).digest("hex"));
  declaredBrowserSessionEnginesV1 = browser.declaredBrowserSessionEnginesV1;
  const composition = await importRevision(new URL(PLAYGROUND_COMPOSITION_MODULE, import.meta.url), createHash("sha256").update(readPhysicalSource(join(LIBRARY_ROOT, PLAYGROUND_COMPOSITION_MODULE))).digest("hex"));
  playgroundCompositionPathV1 = composition.playgroundCompositionPathV1;
});

const POLICY = JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, "⚡️caching/🔣️policy.json")).toString("utf8"));
const WORKSPACE_OWNERSHIP = JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, "🗂️workspaces/🔣️policy.json")).toString("utf8"));
const EXAMPLE_COLLECTIONS = WORKSPACE_OWNERSHIP.collections.names;
const TAXONOMY = JSON.parse(readPhysicalSource(join(LIBRARY_ROOT, "🔣️taxonomy.json")).toString("utf8"));
const IMPLEMENTATION_REVISION = new URL(import.meta.url).searchParams.get("revision") ?? implementationRevision();
const nxPath = (path) => path.split("\\").join("/");
/** 🧭️ Picks a dependency `sourceFile` Nx already indexes so graph validation survives Windows walker encoding drift. */
function nxTrackedSourceFile(context, projectName, preferred, basenameHint) {
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
/** 🦀️ Lists every workspace `Cargo.toml` with taxonomy-respecting discovery instead of Nx glob paths. */
function walkCargoToml(workspaceRoot) {
  const found = [];
  const walk = (directory) => {
    for (const entry of readdirSync(join(workspaceRoot, directory), { withFileTypes: true })) {
      const path = nxPath(join(directory, entry.name));
      if (entry.isSymbolicLink() || path === "compose" || path === "temp/compose" || entry.name === ".🧬semio" || POLICY.generatedDirectories.includes(entry.name)) continue;
      if (entry.isDirectory()) walk(path);
      else if (entry.isFile() && entry.name === "Cargo.toml") found.push(path);
    }
  };
  walk("");
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
  const source = readFileSync(join(workspaceRoot, "bun.lock"), "utf8");
  const parsed = createRequire(import.meta.url)("typescript").parseConfigFileTextToJson("bun.lock", source);
  if (parsed.error) throw new Error("Invalid Bun lockfile JSON");
  const patches = {}, digest = createHash("sha256").update(source), root = realpathSync(workspaceRoot);
  for (const path of [...new Set(Object.values(parsed.config.patchedDependencies ?? {}))].sort()) {
    if (typeof path !== "string") throw new Error("Invalid Bun patch path");
    const file = realpathSync(resolve(root, path)), local = relative(root, file);
    if (isAbsolute(local) || local === ".." || nxPath(local).startsWith("../") || !statSync(file).isFile()) throw new Error(`Bun patch must be an owned file: ${path}`);
    patches[path] = readFileSync(file); digest.update("\0" + path + "\0").update(patches[path]);
  }
  const hash = digest.digest("hex");
  if (BUN_LOCK_CACHE?.root === root && BUN_LOCK_CACHE.hash === hash) return BUN_LOCK_CACHE.model;
  const model = bunLockGraph(parsed.config, patches);
  BUN_LOCK_CACHE = { root, hash, model };
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
const RUST_INPUT_SNAPSHOTS = new WeakMap();

/** ♻️ Content-addressed discovery never trusts source timestamps or retains token streams. */
function cachedRustReferences(file, cache, snapshots) {
  if (snapshots.has(file)) return snapshots.get(file);
  const source = readFileSync(file), digest = createHash("sha256").update(source).digest("hex");
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
  snapshots.set(file, entry.references);
  return entry.references;
}

/** 📥️ Resolves Cargo entry points and compact source references without running a compiler. */
function rustSourceFiles(entries, manifestRoot, cache = RUST_SOURCE_CACHE, snapshots = new Map()) {
  let closures = RUST_INPUT_SNAPSHOTS.get(snapshots);
  if (!closures) { closures = new Map(); RUST_INPUT_SNAPSHOTS.set(snapshots, closures); }
  const key = JSON.stringify([manifestRoot, entries.map(entry => resolve(entry))]);
  if (closures.has(key)) return [...closures.get(key)];
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
        for (const child of candidates) if (existsSync(child)) visit(child, path === undefined && nxPath(child).endsWith("/" + name + ".rs") ? join(dirname(child), name) : dirname(child));
      }
    };
    scope(references.modules, moduleBase, true);
  };
  for (const entry of entries) visit(resolve(entry));
  const result = [...files].sort();
  closures.set(key, result);
  return [...result];
}

/** 🧭️ Cargo owns compilation; Nx tracks every statically mounted source and embedded asset. */
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
  try { return rustSourceFiles(entries, directory, RUST_SOURCE_CACHE, facts).map((file) => `{workspaceRoot}/${nxPath(relative(workspaceRoot, file))}`); }
  catch { return undefined; }
}

/** 🗂️ Shares source reads and command closures only within one graph construction. */
function createScriptInputCache() {
  return { files: new Map(), closures: new Map() };
}

/** 🧭️ Tracks literal relative command imports through the existing TypeScript tooling boundary. */
function relativeScriptInputs(entries, workspaceRoot, cache = createScriptInputCache()) {
  const roots = [...new Set(entries.map(entry => resolve(entry)))].sort(), key = JSON.stringify([resolve(workspaceRoot), roots]);
  const cached = cache.closures.get(key);
  if (cached) return cached;
  const compiler = createRequire(import.meta.url)("typescript"), files = new Set();
  const visit = (path) => {
    if (files.has(path)) return;
    if (nxPath(relative(workspaceRoot, path)).startsWith("../")) throw new Error(`Command import escapes workspace: ${path}`);
    files.add(path);
    let imports = cache.files.get(path);
    if (!imports) {
      imports = [];
      for (const entry of commandSourceImports(path, readFileSync(path, "utf8"))) {
        let resolved;
        try { resolved = createRequire(path).resolve(entry); }
        catch (error) {
          if (error.code !== "MODULE_NOT_FOUND") throw error;
          resolved = compiler.resolveModuleName(entry, path, { moduleResolution: compiler.ModuleResolutionKind.Bundler, allowJs: true, resolveJsonModule: true }, compiler.sys).resolvedModule?.resolvedFileName;
          if (!resolved) continue;
          if (/\.d\.[cm]?ts$/.test(resolved)) throw error;
        }
        imports.push(resolved);
      }
      cache.files.set(path, imports);
    }
    for (const imported of imports) visit(imported);
  };
  for (const entry of roots) visit(entry);
  const result = [...files].map((file) => `{workspaceRoot}/${nxPath(relative(workspaceRoot, file))}`).sort();
  cache.closures.set(key, result);
  return result;
}

/** 🔧️ Native leaves hash their compiler contract and exact command implementation, independently of UI selection. */
function nativeCommandInputs(workspaceRoot, scripts) {
  const script = join(LIBRARY_ROOT, "⚡️caching/🦀️cargo/📜️script.ts");
  const cargo = POLICY.toolchains.cargo, javascript = POLICY.toolchains.javascript;
  return [
    ...relativeScriptInputs([script], workspaceRoot, scripts),
    ...[...javascript.files.filter((file) => !["package.json", "bun.lock"].includes(file)), ...cargo.files].map((file) => `{workspaceRoot}/${file}`),
    "{workspaceRoot}/package.json",
    { externalDependencies: ["@iarna/toml"] },
    ...cargo.environment.map((env) => ({ env })),
    ...[...javascript.commands, ...cargo.commands].map((runtime) => ({ runtime })),
    { runtime: 'node -p "process.platform.concat(process.arch)"' },
  ];
}

/** 🧭️ Parses a target's own script entry points from its command, or reports that none exist. */
function targetScriptClosure(target, workspaceRoot, scripts) {
  const command = target.options?.command;
  if (typeof command !== "string") return undefined;
  const cwd = resolve(workspaceRoot, target.options?.cwd ?? ".");
  const entries = [...command.matchAll(/"([^"\n]+)"|'([^'\n]+)'|([^\s"';&|]+)/g)]
    .map((match) => match[1] ?? match[2] ?? match[3])
    .filter((token) => token.endsWith(SCRIPT_BASENAME))
    .map((token) => resolve(cwd, token))
    .filter((path) => existsSync(path) && !nxPath(relative(workspaceRoot, path)).startsWith("../"));
  return entries.length ? relativeScriptInputs(entries, workspaceRoot, scripts) : undefined;
}

/** 🔐️ `cargo metadata --locked` validates the shared lock against every workspace manifest, so its replay must hash all of them. */
function nativeLockInputs(command) {
  return typeof command === "string" && command.includes(" native cargo metadata ") ? ["{workspaceRoot}/**/Cargo.toml", "{workspaceRoot}/**/Cargo.lock"] : [];
}

/** 🧭️ Adds the selected native target's local router and executable import closure. */
function nativeTargetCommandInputs(target, workspaceRoot, fallback = nativeCommandInputs(workspaceRoot), scripts) {
  const closure = targetScriptClosure(target, workspaceRoot, scripts);
  return closure ? [...new Set([...closure, ...fallback])] : fallback;
}

/** 🎯️ Hashes any script-backed target's own command closure, exactly, instead of the whole router. */
function genericTargetCommandInputs(target, workspaceRoot, fallback, scripts) {
  return targetScriptClosure(target, workspaceRoot, scripts) ?? fallback;
}

/** 🪢️ The previous blanket router contract, kept only as a correctness fallback for a command naming no script. */
function genericCommandFallbackInputs(workspaceRoot) {
  const runner = nxPath(relative(workspaceRoot, LIBRARY_ROOT));
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

/** 🧬️ Assigns each output to its physical Nx producer while retaining the generator's complete entry point. */
function generatorOutputOwners(contracts, root, project, targets) {
  const outputs = new Map();
  for (const contract of Object.values(contracts)) {
    if (contract.ownership !== "owned") continue;
    if (contract.ownerPath === root) outputs.set(contract.target, outputs.get(contract.target) ?? []);
    for (const output of contract.outputRoots) {
      const owner = output.producer ?? contract;
      if (owner.ownerPath !== root) continue;
      if (!owner.target?.startsWith(`${project}:`) || !targets[owner.target.slice(project.length + 1)]) throw new Error(`Generator output has no Nx producer: ${output.path}`);
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
  if (target.cache === false) return { inputs: ["default", "^default"], outputs: [], ...target, cache: false };
  if (matchesUncached(name, policy)) return { ...target, cache: false };
  if (policy.cachedExact.includes(name)) return { inputs: ["default", "^default"], outputs: [], ...target, cache: true };
  if (mutatingName(name) || liveName(name)) return { ...target, cache: false };
  if (cacheableFamily(name) || verifyCommand(target)) return { inputs: ["default", "^default"], outputs: [], ...target, cache: true };
  return { ...target, cache: target.cache !== false };
}

/** 🧾️ The tooling TOML boundary exposes only repository-owned plain records. */
function readToml(path) {
  return createRequire(import.meta.url)("@iarna/toml").parse(readFileSync(path, "utf8"));
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

/** 🦀️ Resolves Cargo's local compilation inputs, admitting development dependencies only at the selected test root. */
function nativeDependencyRoots(root, workspaceRoot, tests = false, cache = new Map(), closures = new Map()) {
  const key = `${root}\0${tests}`;
  if (closures.has(key)) return closures.get(key);
  const read = (path) => { if (!cache.has(path)) cache.set(path, readToml(path)); return cache.get(path); };
  const visited = new Set();
  const visit = (directory, includeTests) => {
    if (visited.has(directory)) return;
    visited.add(directory);
    const manifestPath = join(directory, "Cargo.toml");
    if (!existsSync(manifestPath)) { const error = new Error(`Native dependency has no Cargo manifest: ${manifestPath}`); error.code = "NATIVE_INPUT_ADMISSION"; error.manifest=manifestPath; throw error; }
    const manifest = read(manifestPath);
    let workspace = directory;
    if (manifest.package?.workspace) workspace = resolve(directory, manifest.package.workspace);
    else while (workspace !== workspaceRoot && !read(join(workspace, "Cargo.toml"))?.workspace) {
      workspace = dirname(workspace);
      while (workspace !== workspaceRoot && !existsSync(join(workspace, "Cargo.toml"))) workspace = dirname(workspace);
      if (nxPath(relative(workspaceRoot, workspace)).startsWith("../")) throw new Error(`Native workspace escapes repository: ${directory}`);
    }
    if (nxPath(relative(workspaceRoot, workspace)).startsWith("../")) throw new Error(`Native workspace escapes repository: ${directory}`);
    const authority = existsSync(join(workspace, "Cargo.toml")) ? read(join(workspace, "Cargo.toml")).workspace ?? {} : {};
    for (const dependency of nativeDependencies(manifest, authority)) {
      if (dependency.kind === "dev-dependencies" && !includeTests) continue;
      const child = resolve(dependency.workspace ? workspace : directory, dependency.path);
      if (nxPath(relative(workspaceRoot, child)).startsWith("../")) throw new Error(`Native dependency escapes repository: ${child}`);
      visit(child, false);
    }
  };
  visit(resolve(workspaceRoot, root), tests);
  const result = [...visited].map((path) => nxPath(relative(workspaceRoot, path)) || ".").sort();
  closures.set(key, result);
  return result;
}

/** 👁️ Projects actual Cargo/Nx compilation and owner generator inputs for live source watching. */
export async function nativeSourceWatchPlanV1(root, workspaceRoot) {
  await libraryBootstrap;
  const roots = nativeDependencyRoots(root, workspaceRoot), includes = new Set(), excludes = new Set(), files = new Set();
  const taxonomyPath = join(workspaceRoot, nxPath(relative(workspaceRoot, join(LIBRARY_ROOT, "🔣️taxonomy.json"))));
  const contracts = existsSync(taxonomyPath) ? JSON.parse(readFileSync(taxonomyPath, "utf8")).generatorContracts ?? {} : {};
  for (const nativeRoot of roots) {
    const manifest = readToml(join(workspaceRoot, nativeRoot, "Cargo.toml"));
    let owner = resolve(workspaceRoot, nativeRoot);
    if (manifest.package?.workspace) owner = resolve(owner, manifest.package.workspace);
    else while (owner !== workspaceRoot && (!existsSync(join(owner, "Cargo.toml")) || !readToml(join(owner, "Cargo.toml")).workspace)) owner = dirname(owner);
    for (const name of ["Cargo.toml", "Cargo.lock"]) {
      const file = join(owner, name), local = nxPath(relative(workspaceRoot, file));
      if (local.startsWith("../")) throw Error("Native watch workspace authority escapes repository");
      includes.add(local);
    }
    for (const path of POLICY.toolchains.cargo.files) includes.add(path);
    const projectPath = join(workspaceRoot, nativeRoot, PROJECT_BASENAME);
    const project = existsSync(projectPath) ? JSON.parse(readFileSync(projectPath, "utf8")) : {};
    for (const path of cargoSourceInputs(nativeRoot, workspaceRoot, new Map(), false) ?? []) files.add(path.replace(/^\{workspaceRoot\}\//u, ""));
    const declarations = projectInputs(project, nativeRoot, workspaceRoot, new Map(), createScriptInputCache());
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
    if (!/^(?:build|check|lint|test|wasm|native|component|describe|extension-package|package|font-tool|bench)(?:-|$)/.test(name) || generatorTargets.has(`${project.name}:${name}`)) continue;
    const tests = /^(?:test|bench)(?:-|$)/.test(name);
    if (!plans.has(tests)) {
      try { plans.set(tests, nativePreparation(nativeRoot, workspaceRoot, contracts, tests, cache, closures)); }
      catch (error) {
        if (error.code !== "NATIVE_INPUT_ADMISSION") throw error;
        const admission = "native-input-admission";
        project.metadata = { ...project.metadata, nativeInputAdmission: { manifest: nxPath(relative(workspaceRoot, manifest)), diagnostic: error.message } };
        project.targets[admission] = { executor: "nx:run-commands", cache: false, options: { cwd: ".", command: `bun ./🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📜️script.ts native-input-check --manifest ${JSON.stringify(nxPath(relative(workspaceRoot, manifest)))}` } };
        plans.set(tests, { admission });
      }
    }
    if (plans.get(tests).admission) { const name = `${project.name}:${plans.get(tests).admission}`; target.dependsOn = [...(target.dependsOn ?? []), name]; continue; }
    const selected = plans.get(tests);
    if (projectsByRoot.size && target.inputs?.some((input) => input === "^nativeSources" || input === "^nativeTestSources")) {
      const dependencies = nativeDependencyRoots(nativeRoot, workspaceRoot, tests, cache, closures).filter((root) => root !== nativeRoot).map((root) => {
        const name = projectsByRoot.get(root);
        if (!name) throw new Error(`Native dependency has no Nx project owner: ${root}`);
        return name;
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
  const goScript = root.includes("🐹️go") || (existsSync(script) && /["']go["']\s*,|\brunGo\w*\(/.test(readFileSync(script, "utf8")));
  const local = join(workspaceRoot, root, "go.mod");
  if (existsSync(local)) return local;
  const manifest = join(workspaceRoot, owner, "go.mod");
  return goScript && existsSync(manifest) ? manifest : undefined;
}

/** 🧬️ Derives named task inputs from explicit relative source-graph contracts. */
function declaredSourceInputs(json, workspaceRoot) {
  const groups = {};
  for (const [name, path] of Object.entries(json.metadata?.sourceInputs ?? {})) {
    if (["default", "production", "nativeSources", "nativeTestSources", "artifactSources", "artifactCommandSources"].includes(name) || Object.hasOwn(json.namedInputs ?? {}, name)) throw new Error(`Duplicate source input group ${name}`);
    if (typeof path !== "string" || isAbsolute(path) || path.split(/[\\/]/).includes("..")) throw new Error(`Invalid source input contract path ${path}`);
    const contractPath = resolve(workspaceRoot, path), contract = readSourceInputContract(contractPath), sources = relativeSourceInputs(contract, workspaceRoot);
    groups[name] = [...new Set([contractPath, join(LIBRARY_ROOT, SOURCE_INPUT_MODULE), join(LIBRARY_ROOT, "🕸️dependencies/🟦️typescript/🔣️schema.json"), ...sources.files])].map(file => `{workspaceRoot}/${nxPath(relative(workspaceRoot, file))}`).concat([{ externalDependencies: sources.externalDependencies }]);
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
  const parts = nxPath(path).split("/");
  return parts.some((part, index) => EXAMPLE_COLLECTIONS.includes(part) && parts[index - 1] !== WORKSPACE_OWNERSHIP.collections.moduleMember);
}

/** 🧮️ Nx minimatch exclusions carry the same exact collection and module ancestry. */
function exampleCollectionInputs(root) {
  const names = `{${EXAMPLE_COLLECTIONS.join(",")}}`;
  return [`!${root}/${names}/**/*`, `!${root}/**/!(${WORKSPACE_OWNERSHIP.collections.moduleMember})/${names}/**/*`];
}

/** 📥️ Shared command implementation and host identity are inputs of every script-backed task. */
function projectInputs(json, root, workspaceRoot, facts, scripts) {
  const nativeRoot = json.metadata?.nativeRoot ?? root;
  const tools = ["javascript"];
  if (existsSync(join(workspaceRoot, root, "Cargo.toml")) || json.targets?.wasm) tools.push("cargo");
  if (json.targets?.wasm || json.targets?.["extension-package"] || (tools.includes("cargo") && json.targets?.package)) tools.push("wasm");
  if (goManifest(root, workspaceRoot)) tools.push("go");
  if (existsSync(join(workspaceRoot, root, "pyproject.toml"))) tools.push("python");
  if (readdirSync(join(workspaceRoot, root)).some((file) => /\.[cfv]sproj$/.test(file))) tools.push("dotnet");
  if (existsSync(join(workspaceRoot, root, "CMakeLists.txt"))) tools.push("cmake");
  const nativeSources = tools.includes("cargo") ? cargoSourceInputs(nativeRoot, workspaceRoot, facts, false) : undefined;
  const nativeTests = tools.includes("cargo") ? cargoSourceInputs(nativeRoot, workspaceRoot, facts, true) : undefined;
  const runner = nxPath(relative(workspaceRoot, LIBRARY_ROOT));
  const inputs = [
    "{projectRoot}/**/*",
    { runtime: 'node -p "process.platform.concat(process.arch)"' },
  ];
  const deliveryOffsets = TAXONOMY.testDeliveryScopeDirectoryNames.map((name) => root.indexOf(`/${name}/`)).filter((index) => index >= 0);
  const owner = deliveryOffsets.length > 0 ? root.slice(0, Math.min(...deliveryOffsets)) : root;
  if (owner !== root) {
    const extensions = tools.includes("cargo") ? "{rs,toml,json,semio,wit,wgsl,glsl,h,c,cpp}" : tools.includes("go") ? "{go,mod,sum,json,ts}" : tools.includes("dotnet") ? "{cs,fs,vb,csproj,fsproj,vbproj,props,targets,resx,json}" : tools.includes("python") ? "{py,pyi,toml,json}" : "{ts,tsx,js,jsx,mjs,cjs,json,css,scss,html,svg,wit,sql}";
    const native = nativeTests;
    inputs.push(...(native ? [...native, `{workspaceRoot}/${owner}/**/*.{json,semio,wit,wgsl,glsl,h,c,cpp,ts,tsx,js,mjs,cjs}`] : [`{workspaceRoot}/${owner}/**/*.${extensions}`]));
  }
  for (const tool of tools) {
    const contract = POLICY.toolchains[tool];
    inputs.push(...contract.files.map((file) => `{workspaceRoot}/${file}`));
    inputs.push(...contract.environment.map((env) => ({ env })));
    inputs.push(...contract.commands.map((runtime) => ({ runtime })));
  }
  if (tools.includes("python")) inputs.push({ runtime: `uv run --locked --no-sync --project ${JSON.stringify(root)} python --version` });
  for (const directory of POLICY.generatedDirectories) {
    inputs.push(`!{projectRoot}/**/${directory}/**/*`);
    if (owner !== root) inputs.push(`!{workspaceRoot}/${owner}/**/${directory}/**/*`);
  }
  if (owner !== runner) inputs.push(`!{workspaceRoot}/${runner}/**/🧪️tests/**/*`);
  const examples = [...exampleCollectionInputs("{projectRoot}"), ...exampleCollectionInputs("{workspaceRoot}")];
  const production = ["!{projectRoot}/**/🧪️tests/**/*", ...examples, "!{projectRoot}/**/*.feature", "!{projectRoot}/**/*.stories.{ts,tsx}"];
  if (owner !== root) production.push(`!{workspaceRoot}/${owner}/**/🧪️tests/**/*`);
  const declarations = json.namedInputs ?? {};
  const exclusions = [];
  for (const target of Object.values(json.targets ?? {})) for (const output of target.outputs ?? []) {
    const absolute = resolveOutputPath(output, root, workspaceRoot);
    if (!absolute) continue;
    const path = nxPath(relative(workspaceRoot, absolute));
    if (path === "" || path.startsWith("../")) throw new Error(`Invalid output ownership for ${json.name}: ${output}`);
    exclusions.push(`!{workspaceRoot}/${path}`, `!{workspaceRoot}/${path}/**/*`);
    if (owned(path, root)) {
      const local = nxPath(relative(join(workspaceRoot, root), absolute));
      exclusions.push(`!{projectRoot}/${local}`, `!{projectRoot}/${local}/**/*`);
    }
  }
  if (owner !== root && tools.includes("cargo")) {
    for (const glob of POLICY.ephemeralOwnerGlobs ?? []) {
      exclusions.push(`!{workspaceRoot}/${owner}/${glob}`, `!{workspaceRoot}/${owner}/${glob}/**/*`);
    }
  }
  const native = (sources, productionOnly = false) => !tools.includes("cargo") ? ["production"] : [...new Set(sources ? [`{workspaceRoot}/${nativeRoot}/Cargo.toml`, ...sources] : ["{projectRoot}/**/*", ...(owner !== root ? [`{workspaceRoot}/${owner}/**/*`] : [])]), ...(productionOnly ? ["!{projectRoot}/**/🧪️tests/**/*", ...examples, ...(owner !== root ? [`!{workspaceRoot}/${owner}/**/🧪️tests/**/*`] : [])] : []), ...POLICY.generatedDirectories.flatMap((directory) => [`!{projectRoot}/**/${directory}/**/*`, ...(owner !== root ? [`!{workspaceRoot}/${owner}/**/${directory}/**/*`] : [])]), ...exclusions];
  const artifactTypeScript = json.tags?.includes("role:artifact") && json.tags.includes("language:typescript") && owner !== root && existsSync(join(workspaceRoot, root, SCRIPT_BASENAME));
  const artifactSource = join(workspaceRoot, owner, "🟦️.ts");
  const artifactSources = artifactTypeScript ? [...relativeScriptInputs([artifactSource], workspaceRoot, scripts), "{projectRoot}/package.json", ...exclusions] : [];
  const javascript = POLICY.toolchains.javascript;
  const artifactCommandSources = artifactTypeScript ? [...relativeScriptInputs([join(workspaceRoot, root, SCRIPT_BASENAME)], workspaceRoot, scripts), `{workspaceRoot}/bunfig.toml`, { externalDependencies: ["typescript"] }, ...javascript.environment.map((env) => ({ env })), ...javascript.commands.map((runtime) => ({ runtime })), { runtime: 'node -p "process.platform.concat(process.arch)"' }] : [];
  return { ...declarations, ...declaredSourceInputs(json, workspaceRoot), default: [...inputs, ...(declarations.default ?? []), ...exclusions], production: [...inputs, ...(declarations.default ?? []), ...exclusions, ...production, ...(declarations.production ?? [])], nativeSources: workspaceNegationClosed([...native(nativeSources, true), ...(declarations.nativeSources ?? [])]), nativeTestSources: workspaceNegationClosed([...native(nativeTests), ...(declarations.nativeSources ?? []), ...(declarations.nativeTestSources ?? [])]), ...(artifactTypeScript ? { artifactSources: [...artifactSources, ...(declarations.artifactSources ?? [])], artifactCommandSources: [...artifactCommandSources, ...(declarations.artifactCommandSources ?? [])] } : {}) };
}

/**
 * 🎚️ Applies the workspace target convention: every target runs through
 * {@link DEFAULT_EXECUTOR}, and a project that owns a `📜️script.ts` runs from
 * its own root, so a `📋️project.json` only spells out what actually differs.
 * Projects without one keep nx's workspace-root default and thereby delegate to
 * the root `📜️script.ts`.
 * @param {Record<string, any>} target
 * @param {string} root
 * @param {boolean} ownsScript
 */
function targetWithDefaults(target, root, ownsScript) {
  const executor = target.executor ?? DEFAULT_EXECUTOR;
  if (executor !== DEFAULT_EXECUTOR) return { ...target, executor };
  if (!ownsScript) return { ...target, executor };
  return { ...target, executor, options: { cwd: root, ...(target.options ?? {}) } };
}

/**
 * 🧪 Derives the `test-quick` / `test-long` / `test-exhaustive` siblings from a
 * project's base `test` target, leaving any explicitly declared level untouched.
 * @param {Record<string, any>} targets
 */
function withLeveledTestTargets(targets) {
  const base = targets[TEST_TARGET];
  if (typeof base?.options?.command !== "string") return targets;
  if (base.options.command.includes("⚡️caching/🦀️cargo/📜️script.ts")) return targets;
  const leveled = { ...targets };
  for (const level of TEST_LEVELS) {
    const name = `${TEST_TARGET}-${level}`;
    if (leveled[name]) continue;
    leveled[name] = { ...base, options: { ...base.options, command: `${base.options.command} ${level}` } };
  }
  return leveled;
}


/**
 * @param {Record<string, any>} json
 * @param {string} root
 * @param {string} projectDir
 */
function projectWithDefaults(json, root, projectDir, workspaceRoot, contracts = {}, facts, commandInputs, scripts) {
  const ownsScript = existsSync(join(projectDir, SCRIPT_BASENAME));
  const nativeRoot = json.metadata?.nativeRoot ?? root;
  if (json.metadata?.nativeRoot && (typeof nativeRoot !== "string" || isAbsolute(nativeRoot) || nativeRoot.split(/[\\/]/).some(part => ["", ".", ".."].includes(part)) || !existsSync(join(workspaceRoot, nativeRoot, "Cargo.toml")))) throw new Error(`Invalid native package root for ${json.name}: ${nativeRoot}`);
  const nativeProject = existsSync(join(workspaceRoot, nativeRoot, "Cargo.toml"));
  const artifactTypeScript = json.tags?.includes("role:artifact") && json.tags.includes("language:typescript");
  const declared = withWasmTooling({ ...(root === "." && ownsScript ? rootCommandTargets(join(projectDir, SCRIPT_BASENAME)) : {}), ...cargoTargets(root, workspaceRoot, commandInputs), ...json.targets, ...componentTargets(root, workspaceRoot, commandInputs), ...printDocumentTargets(json, root, workspaceRoot, scripts) }, ownsScript && json.targets?.wasm ? readFileSync(join(projectDir, SCRIPT_BASENAME), "utf8") : "");
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
  for (const [name, target] of Object.entries(withLeveledTestTargets(declared))) {
    const policy = targetPolicy(name, targetWithDefaults({ ...(POLICY.targetDefaults?.[name] ?? {}), ...target }, root, ownsScript));
    const nativePolicyTarget = nativeProject && nativeTargetCommandInputs(policy,workspaceRoot,commandInputs,scripts).some(path=>path.includes("/🏃️process/🧪️testing/🦀️cargo/")||path.includes("/🏃️process/📦️artifacts/🏗️native-build/")||path.includes("/🏃️process/📦️artifacts/🕸️wasm-build/"));
    const nativeTarget = nativePolicyTarget || nativeProject && (/^(?:build|wasm|native|test|lint|check|verify)(?:-|$)/.test(name) || name === "describe" && Boolean(declared["component-dev"])) || policy.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts");
    const artifactTarget = artifactTypeScript && /^(?:build|check|test(?:-(?:quick|long|exhaustive))?)$/.test(name);
    if(nativePolicyTarget){
      const command=policy.options?.command;
      if(typeof command!=="string"||!command.startsWith("bun "))throw Error(`Cargo owner test requires one Bun script command: ${json.name}:${name}`);
      const driver=nxPath(relative(workspaceRoot,join(LIBRARY_ROOT,"⚡️caching/🦀️cargo/📜️script.ts")));
      const ownerCwd=target.options?.cwd??root;
      const boundCommand=command.replace(/^bun\s+("[^"\n]+"|'[^'\n]+'|[^\s]+)/u,(_,source)=>`bun ${JSON.stringify(nxPath(resolve(workspaceRoot,ownerCwd,source.replace(/^["']|["']$/g,""))))}`);
      policy.executor="@semio-tech/repo-lib:owner-command";
      let nativeOwnerCommand;try{nativeOwnerCommand=declaredNativeOwnerCommandV1(`${nativeRoot}/Cargo.toml`,ownerCwd,boundCommand);}catch(error){throw new Error(`Original discovered native command owner ${json.name}:${name}: ${error.message}`,{cause:error});}
      policy.options={...policy.options,cwd:".",nativeOwnerCommand,command:`bun ${JSON.stringify(driver)} native owner-command --manifest ${JSON.stringify(`${nativeRoot}/Cargo.toml`)} --cwd ${JSON.stringify(ownerCwd)} -- ${boundCommand}`};
      const manifest=readToml(join(workspaceRoot,nativeRoot,"Cargo.toml"));let scope=resolve(workspaceRoot,nativeRoot);
      if(manifest.package?.workspace)scope=resolve(scope,manifest.package.workspace);
      else while(scope!==workspaceRoot && (!existsSync(join(scope,"Cargo.toml"))||!readToml(join(scope,"Cargo.toml")).workspace))scope=dirname(scope);
      const config=nxPath(relative(workspaceRoot,join(scope,".config","nextest.toml")));
      if(config.startsWith("../"))throw Error(`Cargo test policy escapes workspace: ${nativeRoot}`);
      policy.cargoTestPolicyInputs=[`{workspaceRoot}/${config}`];
    }
    if (!/ native owner-command /u.test(policy.options?.command??"") && targetScriptClosure(policy, workspaceRoot, scripts)?.some(path=>path.includes("/🏃️process/🧪️testing/🧪️vitest/")||path.includes("/🏃️process/📋️context/"))) {
      const command=policy.options?.command,ownerCwd=target.options?.cwd??root;
      if(typeof command!=="string"||!command.startsWith("bun "))throw Error(`Process owner requires one Bun script command: ${json.name}:${name}`);
      const driver=nxPath(relative(workspaceRoot,join(LIBRARY_ROOT,"📦️packages/🟦️typescript/📜️script.ts")));
      const bound=command.replace(/^bun\s+("[^"\n]+"|'[^'\n]+'|[^\s]+)/u,(_,source)=>`bun ${JSON.stringify(nxPath(resolve(workspaceRoot,ownerCwd,source.replace(/^["']|["']$/g,""))))}`);
      policy.executor="@semio-tech/repo-lib:owner-command";
      policy.options={...policy.options,cwd:".",command:`bun ${JSON.stringify(driver)} owner-command --cwd ${JSON.stringify(ownerCwd)} -- ${bound}`};
    }
    if (nativeTarget) {
      policy.inputs = [name.startsWith("test") ? "nativeTestSources" : "nativeSources", name.startsWith("test") ? "^nativeTestSources" : "^nativeSources", ...internCommandSources(nativeTargetCommandInputs(policy, workspaceRoot, commandInputs, scripts)), ...(name.startsWith("component-") ? [{ env: "SEMIO_PLUGIN_SYMBOLS" }] : []), ...nativeLockInputs(policy.options?.command),...(policy.cargoTestPolicyInputs??[])];
      delete policy.cargoTestPolicyInputs;
    }
    if (artifactTarget) policy.inputs = [...new Set([...(name === "build" ? [] : ["default", "^default"]), "artifactSources", "artifactCommandSources", ...(target.inputs ?? [])])];
    if (!nativeTarget && !artifactTarget && policy.cache) policy.inputs = [...(policy.inputs ?? ["default", "^default"]), ...internCommandSources(genericTargetCommandInputs(policy, workspaceRoot, genericFallback, scripts))];
    if (POLICY.nxSerialTargets?.includes(name)) policy.parallelism = false;
    normalized[name] = policy;
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
  const namedInputs = projectInputs({ ...json, targets: declared }, root, workspaceRoot, facts, scripts);
  for (const { name, inputs } of commandSourceGroups.values()) {
    if (Object.hasOwn(namedInputs, name)) throw new Error(`Duplicate command source group ${name}`);
    namedInputs[name] = inputs;
  }
  return { ...json, name: json.name, root, namedInputs, targets: normalized };
}

/** 🛠️ Exposes wasm-pack's immutable optimizer preparation to Nx before compiler execution. */
function withWasmTooling(targets, source) {
  if (!targets.wasm || !/\b(?:buildWasmWebV1|buildRepositoryWasmWebV1)\s*\(/.test(source)) return targets;
  return { ...targets, wasm: { ...targets.wasm, options: { ...targets.wasm.options, env: { ...targets.wasm.options?.env, SEMIO_WASM_BUILD_REQUIRED: "1" } }, dependsOn: [...new Set([...(targets.wasm.dependsOn ?? []), "workspace:deps-wasm-opt"])] } };
}

/** 📄️ Projects a document catalog into separately owned PDF tasks without executing a compiler. */
function printDocumentTargets(project, root, workspaceRoot, scripts) {
  const path = project.metadata?.printCatalog;
  if (!path) return {};
  const catalog = JSON.parse(readFileSync(join(workspaceRoot, path), "utf8")), compiler = dirname(dirname(path));
  if (catalog.version !== 1 || !Array.isArray(catalog.librarySources) || !catalog.librarySources.length || !Array.isArray(catalog.documents) || !catalog.documents.length) throw new Error(`Invalid Print catalog: ${path}`);
  const product = dirname(dirname(compiler)), script = `${compiler}/${SCRIPT_BASENAME}`, targets = {}, ids = new Set(), sources = new Set();
  const commandInputs = relativeScriptInputs([join(workspaceRoot, script)], workspaceRoot, scripts);
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
    targets[`build-${document.id}`] = { executor: DEFAULT_EXECUTOR, cache: true, outputs: [`{projectRoot}/dist/documents/${document.id}`], inputs: [...inputs, ...paths], dependsOn: ["fonts", "deps-tectonic", "deps-tex", "generate"], options: { cwd: root, command: `bun ${nxPath(relative(root, script))} build ${document.id}`, forwardAllArgs: false } };
    targets[`watch-${document.id}`] = { executor: DEFAULT_EXECUTOR, cache: false, continuous: true, outputs: [], dependsOn: [`build-${document.id}`], options: { cwd: root, command: `bun ${nxPath(relative(root, script))} watch`, forwardAllArgs: false } };
  }
  for (const [name, collection] of [["build", "templates"], ["build-viz", "visualizations"]]) targets[name] = { ...project.targets[name], cache: false, outputs: [], dependsOn: catalog.documents.filter(document => document.collection === collection).map(document => `build-${document.id}`) };
  return targets;
}

/** 🔨️ Every concrete Cargo package exposes native leaves even when its authored metadata only defines generators. */
function cargoTargets(root, workspaceRoot, commandInputs) {
  const manifest = join(workspaceRoot, root, "Cargo.toml");
  if (!existsSync(manifest) || !readToml(manifest).package) return {};
  const script = nxPath(relative(workspaceRoot, join(LIBRARY_ROOT, "⚡️caching/🦀️cargo/📜️script.ts")));
  commandInputs ??= nativeCommandInputs(workspaceRoot);
  return Object.fromEntries(["build", "check", "test"].map((target) => [target, {
    executor: DEFAULT_EXECUTOR,
    cache: true,
    parallelism: false,
    outputs: target === "build" ? ["{projectRoot}/dist/build"] : [],
    inputs: [target === "test" ? "nativeTestSources" : "nativeSources", target === "test" ? "^nativeTestSources" : "^nativeSources", ...commandInputs],
    options: { cwd: ".", command: `bun ${JSON.stringify(script)} native cargo ${target} --manifest ${JSON.stringify(nxPath(relative(workspaceRoot, manifest)))}` },
  }]));
}

/** 🧩️ Every Cargo component owns profile-specific deliverables outside the compiler store. */
function componentTargets(root, workspaceRoot, commandInputs) {
  const path = join(workspaceRoot, root, "Cargo.toml");
  if (!existsSync(path)) return {};
  const manifest = readToml(path), metadata = manifest.package?.metadata;
  if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.["component-kind"])) return {};
  if (!manifest.lib?.["crate-type"]?.includes("cdylib")) throw new Error(`Component ${metadata.component.package} needs a cdylib target: ${path}`);
  const script = nxPath(relative(workspaceRoot, join(LIBRARY_ROOT, "⚡️caching/🦀️cargo/📜️script.ts")));
  const webRoot = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript";
  const webSourceRoot = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle";
  const deployment = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment";
  const moduleDirectory = componentDeploymentDirectoryV1(metadata.semio);
  commandInputs ??= nativeCommandInputs(workspaceRoot);
  const ownerRoot = nxPath(join(root, "..", ".."));
  const describe = {
    executor: DEFAULT_EXECUTOR,
    cache: false,
    dependsOn: ["component-dev", "@semio-tech/os-plugin-describe-rs:build"],
    outputs: [`{workspaceRoot}/${ownerRoot}/🛂️.descriptor.semio`, `{workspaceRoot}/${ownerRoot}/🔣️.json`],
    options: { cwd: ".", command: `bun ${JSON.stringify("🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust/📜️script.ts")} component --manifest ${JSON.stringify(nxPath(relative(workspaceRoot, path)))}` },
  };
  return Object.fromEntries([["describe", describe], ...["dev", "release"].flatMap((profile) => [[`component-${profile}`, {
    executor: DEFAULT_EXECUTOR,
    cache: true,
    parallelism: false,
    inputs: ["nativeSources", "^nativeSources", ...commandInputs, { env: "SEMIO_PLUGIN_SYMBOLS" }],
    outputs: [`{projectRoot}/dist/component-${profile}`],
    options: { cwd: ".", command: `bun ${JSON.stringify(script)} native component ${profile} --manifest ${JSON.stringify(nxPath(relative(workspaceRoot, path)))}` },
  }], ...(moduleDirectory === undefined ? [] : [[`materialize-${profile}`, {
    executor: DEFAULT_EXECUTOR,
    cache: true,
    dependsOn: [`component-${profile}`, `@semio-tech/framework-plugin-web:support-${profile}`, ...(profile === "release" ? ["workspace:deps-wasm-opt"] : [])],
    inputs: ["production", "^production", { dependentTasksOutputFiles: "**/*" }, `{workspaceRoot}/${webRoot}/**/*.{ts,json}`, `{workspaceRoot}/${webSourceRoot}/**/*.ts`, `!{workspaceRoot}/${webSourceRoot}/**/🧪️tests/**/*.ts`, `{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/📜️.wit`, `{workspaceRoot}/${deployment}/*.json`, `!{workspaceRoot}/${webRoot}/dist/**/*`],
    outputs: [`{workspaceRoot}/${webRoot}/dist/${profile}/🔌️plugin-modules/${moduleDirectory}`],
    options: { cwd: ".", command: `bun ${JSON.stringify(`${webRoot}/📜️script.ts`)} materialize ${profile} --manifest ${JSON.stringify(nxPath(relative(workspaceRoot, path)))}` },
  }]])])]);
}

/** 🎮️ Flattens every plugin playground row from Cargo manifests (with authored `distDir`). */
function collectPlaygroundCatalog(configFiles, workspaceRoot) {
  const components = new Map(), playgrounds = [], nativeFiles = new Map(), nativeClosures = new Map();
  for (const path of configFiles) {
    if (!path.endsWith("Cargo.toml") || path.includes("\uFFFD") || path.includes(".🧬semio") || path.startsWith("compose/") || path.startsWith("temp/compose/") || POLICY.generatedDirectories.some((directory) => path.split("/").includes(directory))) continue;
    const manifest = readToml(join(workspaceRoot, path)), metadata = manifest.package?.metadata;
    if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.["component-kind"])) continue;
    const id = metadata.component.package.slice(6), root = nxPath(dirname(path));
    if (components.has(id)) throw new Error(`Duplicate component identity: ${id}`);
    components.set(id, metadata.semio);
    for (const row of metadata.semio.playground ?? []) playgrounds.push({ ...row, pluginId: id, cratePath: root });
  }
  return playgrounds;
}

/** 🌐️ Per-plugin `build` / `build-<variant>-site` targets that publish CDN trees under each plugin's `dist/`. */
function pluginSiteTargetsForCrate(root, allPlaygrounds, stagingProjectRoot) {
  const rows = allPlaygrounds.filter((row) => row.cratePath === root);
  if (rows.length === 0) return {};
  if (rows.some(row => !row.distDir) && !stagingProjectRoot) throw Error("Playground site output requires the release producer's staging project");
  const output = row => {
    if (!/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(row.variant)) throw Error("Invalid distribution variant");
    if (row.distDir !== undefined && (typeof row.distDir !== "string" || !row.distDir || /[\\:\u0000-\u001f\u007f]/u.test(row.distDir) || row.distDir.split("/").some(part => !part || part === "." || part === ".."))) throw Error("Invalid distribution output directory");
    return `{workspaceRoot}/${row.distDir ?? `${stagingProjectRoot}/dist/build-${row.variant}-react-release`}`;
  };
  const targets = {};
  for (const row of rows) {
    const site = `build-${row.variant}-site`;
    const release = `build-${row.variant}-react-release`;
    targets[site] = {
      cache: true,
      outputs: [output(row)],
      dependsOn: [`@semio-tech/framework-os-dev:${release}`],
      options: { cwd: root, command: `bun ./📜️script.ts build ${row.variant}`, forwardAllArgs: false },
    };
  }
  const siteTargets = rows.map((row) => `build-${row.variant}-site`);
  targets.build = {
    cache: true,
    dependsOn: siteTargets,
    outputs: rows.map(output),
    options: { cwd: root, command: rows.length === 1 ? "bun ./📜️script.ts build" : "bun ./📜️script.ts build all", forwardAllArgs: false },
  };
  return targets;
}

/** 🎮️ Gives each Cargo-declared playground session an independent generated artifact. */
function playgroundSessionTargets(configFiles, workspaceRoot) {
  const components = new Map(), playgrounds = new Map();
  for (const file of configFiles) {
    if (!file.endsWith("Cargo.toml") || file.includes("\uFFFD") || file.includes(".🧬semio") || file.startsWith("compose/") || file.startsWith("temp/compose/") || POLICY.generatedDirectories.some(directory => file.split("/").includes(directory))) continue;
    const manifest=readToml(join(workspaceRoot,file)), metadata=manifest.package?.metadata;
    if(!metadata?.component?.package || !["plugin","extension"].includes(metadata.semio?.["component-kind"])) continue;
    const pluginId=metadata.component.package.slice(6), root=nxPath(dirname(file)), projectFile=join(workspaceRoot,root,PROJECT_BASENAME);
    components.set(pluginId,{...metadata.semio,pluginId,project:existsSync(projectFile)?JSON.parse(readFileSync(projectFile,"utf8")).name:manifest.package.name,dependsOn:[...(metadata.semio.extends?[metadata.semio.extends]:[]),...(metadata.semio["depends-on"]??[])]});
    for(const row of metadata.semio.playground??[]) {
      if(typeof row.variant!=="string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(row.variant) || playgrounds.has(row.variant)) throw new Error("Invalid or duplicate playground variant in "+file);
      playgrounds.set(row.variant,{...row,pluginId});
    }
  }
  return Object.fromEntries([...playgrounds].sort(([a],[b])=>a.localeCompare(b)).map(([variant,row])=>["session-"+variant,{
    cache:true,
    inputs:["default",{dependentTasksOutputFiles:"**/*",transitive:true}],
    dependsOn:["generate",...[...runtimeComponentClosure([...components.values()],[{id:row.pluginId,appScoped:row.app!==undefined}])].sort().map(id=>components.get(id).project+":describe")],
    outputs:["{projectRoot}/dist/sessions/"+variant],
    options:{command:"bun ./📜️script.ts session "+variant},
  }]));
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
function playgroundPreparationTargets(configFiles, workspaceRoot, projectRoot) {
  const components = new Map(), playgrounds = [], nativeFiles = new Map(), nativeClosures = new Map();
  const projectAt = (root) => {
    const path = join(workspaceRoot, root, PROJECT_BASENAME);
    return existsSync(path) ? JSON.parse(readFileSync(path, "utf8")) : undefined;
  };
  for (const path of configFiles) {
    if (!path.endsWith("Cargo.toml") || path.includes("\uFFFD") || path.includes(".🧬semio") || path.startsWith("compose/") || path.startsWith("temp/compose/") || POLICY.generatedDirectories.some((directory) => path.split("/").includes(directory))) continue;
    const manifest = readToml(join(workspaceRoot, path)), metadata = manifest.package?.metadata;
    if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.["component-kind"])) continue;
    const id = metadata.component.package.slice(6), root = nxPath(dirname(path));
    if (components.has(id)) throw new Error(`Duplicate component identity: ${id}`);
    components.set(id, { ...metadata.semio, cratePath: root, project: projectAt(root)?.name ?? manifest.package.name });
    for (const row of metadata.semio.playground ?? []) playgrounds.push({ ...row, pluginId: id, cratePath: root });
  }

  const wgpuRoot = nxPath("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript"), wgpuProject = projectAt(wgpuRoot);
  if (!wgpuProject?.name || !wgpuProject.targets?.wasm || !wgpuProject.targets?.["wasm-release"]) throw new Error(`WGPU renderer must name both authored wasm profile producers: ${wgpuRoot}`);
  const flowRoot="🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust", flowProject=projectAt(flowRoot);
  if(!flowProject?.name || !flowProject.targets?.wasm)throw new Error(`Flow must name its authored browser producer: ${flowRoot}`);
  const nativeHostView={kind:(path)=>{try{const stat=lstatSync(join(workspaceRoot,path));return stat.isSymbolicLink()?"symlink":stat.isDirectory()?"directory":stat.isFile()?"file":null;}catch(error){if(error.code==="ENOENT")return null;throw error;}},readText:(path)=>readFileSync(join(workspaceRoot,path),"utf8")};
  const result = {};
  for (const playground of playgrounds) {
    if (typeof playground.variant !== "string" || playground.variant.length > 256 || !/^[a-z0-9]+(?:-[a-z0-9]+)*$(?![\s\S])/u.test(playground.variant)) throw Error("Invalid playground variant");
    const composition = playground.compositionConfigPath === undefined ? undefined : playgroundCompositionPathV1(playground.compositionConfigPath), wgpuPort = playground.ports?.wgpu;
    if (!Number.isSafeInteger(wgpuPort) || wgpuPort < 1 || wgpuPort > 65535) throw Error("Invalid declared WGPU listener port");
    const componentRows = [...components].map(([pluginId,row])=>({...row,pluginId,dependsOn:[...(row.extends?[row.extends]:[]),...(row["depends-on"]??[])]}));
    const sources=[...(playground.engines??[]).map(path=>nxPath(relative(workspaceRoot,resolve(workspaceRoot,path,PROJECT_BASENAME)))),...[playground.nativeHost,playground.mcpHost].filter(Boolean).flatMap(host=>[`${host.cratePath}/Cargo.toml`,`${host.cratePath}/${PROJECT_BASENAME}`]),...(playground.devContribution?[playground.devContribution]:[]),...(composition===undefined?[]:[composition])];
    const request={component:playground.pluginId,appScoped:playground.app!==undefined,sources};
    let admission=runtimeInputAdmissionV1(componentRows,[{id:request.component,appScoped:request.appScoped}],sources,path=>existsSync(join(workspaceRoot,path)));
    if(admission.status==="admitted")for(const id of admission.selected){try{nativeDependencyRoots(components.get(id).cratePath,workspaceRoot,false,nativeFiles,nativeClosures);}catch(error){if(error.code!=="NATIVE_INPUT_ADMISSION")throw error;const missing=nxPath(relative(workspaceRoot,error.manifest));sources.push(missing);admission={schemaVersion:1,status:"refused",missing:{kind:"source",value:missing}};break;}}
    if(admission.status==="refused") {
      const command=`bun ${JSON.stringify(join(workspaceRoot,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📜️script.ts"))} runtime-input-check ${Buffer.from(JSON.stringify(request)).toString("base64")}`;
      const target={cache:false,outputs:[],dependsOn:[],inputs:["{workspaceRoot}/**/Cargo.toml","{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧩️runtime/🟨️.mjs","{workspaceRoot}/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/**/*"],metadata:{semio:{runtimeInputAdmission:admission}},options:{command,forwardAllArgs:false}};
      const names=[`runtime-input-admission-${playground.variant}`,`build-${playground.variant}-react-release`];
      for(const profile of ["dev","release"]){for(const engine of ["react","wgpu"])for(const operation of ["prepare","activate","serve","dev"])names.push(`${operation}-${playground.variant}-${engine}-${profile}`);names.push(`prepare-${playground.variant}-native-${profile}`,`run-${playground.variant}-native-${profile}`,`smoke-${playground.variant}-native-${profile}`);if(playground.mcpHost)for(const transport of ["stdio","http"])names.push(`mcp-${playground.variant}-${transport}-${profile}`);}
      for(const name of names)result[name]=target;
      continue;
    }
    const nativeHost=admitPlaygroundNativeHostV1(playground.nativeHost,(root)=>nativeHostSourceFactsV1(root,nativeHostView));
    const mcpHost=admitPlaygroundNativeHostV1(playground.mcpHost,(root)=>nativeHostSourceFactsV1(root,nativeHostView));
    if(mcpHost)for(const profile of ["dev","release"])for(const transport of ["stdio","http"])result[`mcp-${playground.variant}-${transport}-${profile}`]={cache:false,continuous:true,outputs:[],dependsOn:[`${mcpHost.project}:${mcpHost.target}${profile==="release"?"-release":""}`],options:{command:`bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts mcp ${playground.variant} ${profile} ${transport}`,forwardAllArgs:true}};
    // 🏠️ Boot closure: host sessions must serve before the full catalog materializes. `appScoped: true`
    // keeps the host's own depends-on/consumes without the host-fanout that otherwise selects every crate.
    const bootSelected = runtimeComponentClosure(componentRows, [{ id: playground.pluginId, appScoped: true }]);
    const selected = runtimeComponentClosure(componentRows, [{ id: playground.pluginId, appScoped: playground.app !== undefined }]);
    const engines = new Set([...new Set([...(playground.engines ?? []), ...declaredBrowserSessionEnginesV1(workspaceRoot, playground.devContribution)])].map((path) => {
      const root = nxPath(relative(workspaceRoot, resolve(workspaceRoot, path))), project = projectAt(root);
      if (root.startsWith("../") || !project?.name || !project.targets?.wasm) throw new Error(`Playground engine must name an authored wasm producer: ${path}`);
      return `${project.name}:wasm`;
    }));
    for (const profile of ["dev", "release"]) {
      const nativeScript = "bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📜️script.ts";
      result[`prepare-${playground.variant}-native-${profile}`] = {
        cache: true,
        outputs: [`{projectRoot}/dist/runtime/native/${profile}/${playground.variant}`],
        inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
        dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, ...[...selected].sort().map(id => `${components.get(id).project}:materialize-${profile}`)],
        options: { command: `bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/⌨️native-entrypoint/📦️modules/📜️script.ts publish ${playground.variant} ${profile}`, forwardAllArgs: false },
      };
      for (const operation of ["run", "smoke"]) result[`${operation}-${playground.variant}-native-${profile}`] = {
        cache: false, continuous: false, outputs: [],
        dependsOn: [`prepare-${playground.variant}-native-${profile}`, `${nativeHost?.project??wgpuProject.name}:${nativeHost?.target??"native-build"}${profile === "release" ? "-release" : ""}`],
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
      outputs: [`{projectRoot}/dist/runtime/react/${profile}/${playground.variant}`],
      inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
      dependsOn: [`prepare-${playground.variant}-react-${profile}`],
      options: { command: `bun ./📜️script.ts activate ${playground.variant} react ${profile}` },
      };
      result[`prepare-${playground.variant}-react-${profile}`] = {
      cache: true,
      outputs: [],
      inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, `@semio-tech/framework-plugin-web:support-${profile}`, "semio-framework-os-infinite:fonts", `${flowProject.name}:wasm`, ...engines, ...[...bootSelected].sort().map((id) => `${components.get(id).project}:materialize-${profile}`)],
      options: { command: `bun ./📜️script.ts prepare ${playground.variant} react ${profile}` },
      };
      for (const command of ["serve", "dev"]) result[`${command}-${playground.variant}-wgpu-${profile}`] = { cache: false, continuous: true, outputs: [], dependsOn: [`activate-${playground.variant}-wgpu-${profile}`], options: { command: `bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts serve ${playground.variant} ${profile} --port ${wgpuPort}`, ...(composition===undefined?{}:{env:{SEMIO_WGPU_COMPOSITION_PATH:composition}}) } };
      result[`activate-${playground.variant}-wgpu-${profile}`] = {
      cache: false,
      outputs: [`{projectRoot}/dist/runtime/wgpu/${profile}/${playground.variant}`],
      inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
      dependsOn: [`prepare-${playground.variant}-wgpu-${profile}`],
      options: { command: `bun ./📜️script.ts activate ${playground.variant} wgpu ${profile}` },
      };
      result[`prepare-${playground.variant}-wgpu-${profile}`] = {
      cache: true,
      outputs: [],
      inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, `@semio-tech/framework-plugin-web:support-${profile}`, "semio-framework-os-infinite:fonts", `${flowProject.name}:wasm`, `${wgpuProject.name}:${profile === "release" ? "wasm-release" : "wasm"}`, `${wgpuProject.name}:generate-browser-boot`, `${wgpuProject.name}:generate-frame-worker`, `${wgpuProject.name}:generate-renderer-boot`, ...[...bootSelected].sort().map((id) => `${components.get(id).project}:materialize-${profile}`)],
      options: { command: `bun ./📜️script.ts prepare ${playground.variant} wgpu ${profile}` },
      };
    }
    const name = `build-${playground.variant}-react-release`;
    const buildOutput = playground.distDir ? `{workspaceRoot}/${playground.distDir}` : `{projectRoot}/dist/${name}`;
    result[name] = {
      cache: true,
      outputs: [buildOutput],
      // 🌐️ `S_HUB_URL` and `S_DATA_DIR` are baked into the bundle at Vite `define` time
      // (`🏗️builder/🌐️vite/🟦️.ts:237`, `import.meta.env.VITE_S_HUB_URL`), so two bundles that differ
      // only by which hub they sign in against are DIFFERENT artifacts. Without these env inputs the
      // cache key ignores them and a cached bundle silently answers for the wrong hub.
      inputs: ["production", "^production", { dependentTasksOutputFiles: "**/*", transitive: true }, { env: "S_HUB_URL" }, { env: "S_DATA_DIR" }, { runtime: `bun ${JSON.stringify(nxPath(relative(workspaceRoot, resolve(workspaceRoot, projectRoot, "../../🚚️distribution/📜️script.ts"))))} inputs` }],
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, `@semio-tech/framework-plugin-web:support-release`, "semio-framework-os-infinite:fonts", `${flowProject.name}:wasm`, ...engines, ...[...selected].sort().map((id) => `${components.get(id).project}:materialize-release`), "@semio-tech/assets:build"],
      options: { command: `bun ../../🚚️distribution/📜️script.ts build ${playground.variant} react release`, forwardAllArgs: true },
    };
  }
  const pluginSiteReleases = playgrounds.filter((row) => row.distDir !== undefined).map((row) => `build-${row.variant}-react-release`);
  if (pluginSiteReleases.length > 0) {
    result["build-all-playground-cdn-sites"] = {
      cache: true,
      dependsOn: pluginSiteReleases,
      outputs: [...new Set(playgrounds.filter((row) => row.distDir !== undefined).map((row) => `{workspaceRoot}/${row.distDir}`))],
      options: { command: "node -e \"process.exit(0)\"", forwardAllArgs: false },
    };
  }
  return result;
}

/** 🚪️ Exposes registered workspace commands without inferring forwarding package scripts. */
function rootCommandTargets(script) {
  const ts = createRequire(import.meta.url)("typescript");
  const source = ts.createSourceFile(script, readFileSync(script, "utf8"), ts.ScriptTarget.Latest, true);
  const targets = {};
  const inspect = (node) => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "register" && ts.isStringLiteral(node.arguments[0])) {
      const name = node.arguments[0].text;
      if (name !== "nx") targets[name] = { executor: DEFAULT_EXECUTOR, outputs: [], options: { command: `bun ./📜️script.ts ${name}`, forwardAllArgs: true } };
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
 * @param {unknown} _options
 * @param {{ workspaceRoot: string }} context
 */
/** 🛡️ Admits callback candidates through fresh raw physical workspace ancestry. */
function inferenceCandidates(configFiles, workspaceRoot) {
  if (typeof workspaceRoot !== "string" || !workspaceRoot || workspaceRoot.includes("\0") || workspaceRoot.split(/[\\/]/u).some(part => part === "." || part === "..")) throw Error("Nx inference requires raw real workspace ancestry");
  const root = resolve(workspaceRoot), ancestors = [];
  for (let path = root; ; path = dirname(path)) { ancestors.push(path); if (path === dirname(path)) break; }
  for (const path of ancestors.reverse()) { const stat = lstatSync(path); if (!stat.isDirectory() || stat.isSymbolicLink()) throw Error(`Nx inference requires real workspace ancestry: ${path}`); }
  const directories = new Map([[root, true]]), files = new Set();
  for (const supplied of configFiles) {
    if (typeof supplied !== "string" || !supplied || supplied.includes("\0") || supplied.includes("\uFFFD")) continue;
    const path = nxPath(supplied), parts = path.split("/");
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

function emojiProjectJsonNodes(configFiles, _options, context) {
  const { workspaceRoot } = context;
  configFiles = inferenceCandidates(configFiles, workspaceRoot);
  const rootsByName = new Map(), facts = new Map(), scripts = createScriptInputCache();
  const commandInputs = configFiles.some((path) => path.endsWith("Cargo.toml") && !path.includes(".🧬semio") && !POLICY.generatedDirectories.some((name) => path.split("/").includes(name))) ? nativeCommandInputs(workspaceRoot, scripts) : undefined;
  const contractPath = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
  const contracts = existsSync(contractPath) ? JSON.parse(readFileSync(contractPath, "utf8")).generatorContracts ?? {} : {};
  const playgroundCatalog = collectPlaygroundCatalog(configFiles, workspaceRoot);
  const stagingProjects = configFiles.filter(file => file.endsWith(PROJECT_BASENAME) && existsSync(join(workspaceRoot, file))).filter(file => JSON.parse(readFileSync(join(workspaceRoot, file), "utf8")).name === "@semio-tech/framework-os-dev");
  if (stagingProjects.length > 1) throw Error("Duplicate playground release staging producer");
  const stagingProjectRoot = stagingProjects.length === 1 ? nxPath(dirname(stagingProjects[0])) : undefined;

  const results = configFiles
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
    .map((configFile) => {
      const abs = join(workspaceRoot, configFile);
      if (!existsSync(abs)) return null;
      let json;
      try {
        json = JSON.parse(readFileSync(abs, "utf8"));
      } catch (error) {
        throw new Error(`Invalid Nx project metadata ${configFile}: ${error.message}`);
      }
      const name = json.name;
      if (!name) throw new Error(`Nx project metadata ${configFile} requires a name`);
      const projectDir = dirname(abs);
      const root = nxPath(relative(workspaceRoot, projectDir)) || ".";
      const prior = rootsByName.get(name);
      if (prior !== undefined && prior !== root) throw new Error(`Duplicate Nx project ${name}: ${prior} and ${root}`);
      if (prior === undefined) rootsByName.set(name, root);
      if (name === "@semio-tech/plugin-registry") json.targets = { ...json.targets, ...playgroundSessionTargets(configFiles, workspaceRoot) };
      if (name === "@semio-tech/framework-os-dev") json.targets = { ...json.targets, ...playgroundPreparationTargets(configFiles, workspaceRoot, root) };
      if (playgroundCatalog.some(row => row.cratePath === root)) {
        json.targets = { ...pluginSiteTargetsForCrate(root, playgroundCatalog, stagingProjectRoot), ...json.targets };
      }
      const canonicalConfig = nxPath(relative(workspaceRoot, join(projectDir, PROJECT_BASENAME)));
      return [canonicalConfig, { projects: { [name]: projectWithDefaults(json, root, projectDir, workspaceRoot, contracts, facts, commandInputs, scripts) } }];
    })
    .filter(Boolean);
  const declaredRoots = new Set([...rootsByName.values()]);
  for (const configFile of configFiles.filter((file) => file.endsWith("Cargo.toml"))) {
    if (configFile.includes("\uFFFD") || configFile.startsWith("compose/") || configFile.startsWith("temp/compose/") || configFile.includes(".🧬semio") || POLICY.generatedDirectories.some((name) => configFile.split("/").includes(name))) continue;
    if (!existsSync(join(workspaceRoot, configFile))) continue;
    const projectDir = dirname(join(workspaceRoot, configFile));
    const root = nxPath(relative(workspaceRoot, projectDir)) || ".";
    if (declaredRoots.has(root)) continue;
    const manifest = readToml(join(projectDir, "Cargo.toml"));
    if (!manifest.package?.name) continue;
    const name = manifest.package.name;
    if (rootsByName.has(name)) throw new Error(`Duplicate Nx project ${name}: ${rootsByName.get(name)} and ${root}`);
    rootsByName.set(name, root);
    const targets = cargoTargets(root, workspaceRoot, commandInputs);
    const project = { name, root, projectType: "library", tags: ["language:cargo", "discovery:manifest"], targets: { ...targets, ...componentTargets(root, workspaceRoot, commandInputs) } };
    results.push([configFile, { projects: { [name]: { ...project, namedInputs: projectInputs(project, root, workspaceRoot, facts, scripts) } } }]);
  }
  const preparationCache = new Map(), dependencyRootsCache = new Map(), projects = results.flatMap(([, result]) => Object.values(result.projects));
  const projectsByRoot = new Map(projects.map((project) => [project.root, project.name]));
  for (const project of projects) withNativePreparation(project, workspaceRoot, contracts, preparationCache, projectsByRoot, dependencyRootsCache);
  if (_options?.analyzeLockfile && existsSync(join(workspaceRoot, "bun.lock"))) results.push(["bun.lock", { externalNodes: readBunLockGraph(workspaceRoot).externalNodes }]);
  return results;
}

/** 🗂️ Current Nx workspace-data location for immutable source facts. */
function workspaceDataDirectory(workspaceRoot) {
  return process.env.NX_WORKSPACE_DATA_DIRECTORY || join(workspaceRoot, ".nx", "workspace-data");
}

/** 🧬️ Keeps parser revisions and source identities apart from resolved project authority. */
function moduleSourceFactCacheRoot(workspaceRoot) {
  const root = resolve(workspaceDataDirectory(workspaceRoot), "mf");
  return join(root, "0".repeat(64) + ".json").length <= 256 ? root : undefined;
}

const MODULE_FACT_BYTES = 1024 * 1024;

/** 🔑️ Identifies immutable content at one normalized source path under the actual parser revision. */
function moduleSourceFactIdentity(file, hash) {
  const identity = { parser: commandInputHash, file: nxPath(file), hash: String(hash) };
  return { ...identity, key: createHash("sha256").update(JSON.stringify(identity)).digest("hex") };
}

/** 📥️ Admits a bounded closed source fact record without granting any project target authority. */
function readCachedModuleImports(cacheRoot, identity) {
  if (!cacheRoot) return undefined;
  try {
    const path = join(cacheRoot, `${identity.key}.json`);
    if (statSync(path).size > MODULE_FACT_BYTES) return undefined;
    const value = JSON.parse(readFileSync(path, "utf8"));
    if (!value || Object.keys(value).length !== 4 || value.parser !== identity.parser || value.file !== identity.file || value.hash !== identity.hash || !Array.isArray(value.imports) || value.imports.length > 16384 || !value.imports.every(specifier => typeof specifier === "string")) return undefined;
    return value.imports;
  } catch {
    return undefined;
  }
}

/** 📤️ Persists only bounded source facts; partial cache reads simply cause a fresh parse. */
function writeCachedModuleImports(cacheRoot, identity, imports) {
  if (!cacheRoot || imports.length > 16384) return;
  const value = JSON.stringify({ parser: identity.parser, file: identity.file, hash: identity.hash, imports });
  if (Buffer.byteLength(value) > MODULE_FACT_BYTES) return;
  mkdirSync(cacheRoot, { recursive: true });
  writeFileSync(join(cacheRoot, `${identity.key}.json`), value);
}

/** 🏛️ Snapshots current project roots once, in deterministic ownership precedence. */
function importProjectOwners(projects) {
  return Object.entries(projects).filter(([name]) => name !== "workspace").map(([name, project]) => ({ name, root: nxPath(project.root) })).sort((a, b) => b.root.length - a.root.length || a.name.localeCompare(b.name));
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
      const path = nxPath(relative(workspaceRoot, resolve(dirname(join(workspaceRoot, file)), specifier)));
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
  return importTargetsFromFacts(moduleSourceImports(file, text), nxPath(file), workspaceRoot, importProjectOwners(projects), byPackage, locked);
}

/** ⚡️ Reuses bounded source facts while resolving every full or incremental file against current authority. */
async function collectImportEdges(workspaceRoot, projectFiles, projects, byPackage, locked, add) {
  const cacheRoot = moduleSourceFactCacheRoot(workspaceRoot), owners = importProjectOwners(projects), jobs = [];
  for (const [name, files] of Object.entries(projectFiles)) {
    if (name === "workspace") continue;
    for (const file of files) if (/\.[cm]?[jt]sx?$/.test(file.file)) jobs.push({ name, file });
  }
  let cursor = 0;
  await Promise.all(Array.from({ length: Math.min(32, jobs.length) }, async () => {
    for (;;) {
      const index = cursor++;
      if (index >= jobs.length) return;
      const { name, file } = jobs[index], sourceFile = nxPath(file.file);
      let identity = file.hash ? moduleSourceFactIdentity(sourceFile, file.hash) : undefined;
      let imports = identity ? readCachedModuleImports(cacheRoot, identity) : undefined;
      if (!imports) {
        let text;
        try { text = await readFile(join(workspaceRoot, sourceFile), "utf8"); }
        catch (error) { if (error.code === "ENOENT") continue; throw error; }
        identity ??= moduleSourceFactIdentity(sourceFile, createHash("sha256").update(text).digest("hex"));
        imports = moduleSourceImports(sourceFile, text);
        writeCachedModuleImports(cacheRoot, identity, imports);
      }
      for (const target of importTargetsFromFacts(imports, sourceFile, workspaceRoot, owners, byPackage, locked)) add(name, target, sourceFile);
    }
  }));
}

/**
 * 🧭️ Files Nx asks this plugin to re-parse: changed files when the file-map cache hits, else the full map.
 * @param {{ fileMap?: { projectFileMap?: Record<string, { file: string, hash?: string }[]> }, filesToProcess?: { projectFileMap?: Record<string, { file: string, hash?: string }[]> } }} context
 */
function projectFilesToProcess(context) {
  return context.filesToProcess?.projectFileMap ?? context.fileMap?.projectFileMap ?? {};
}

/** 🕸️ Native manifests and package imports contribute edges without spawning a build or installer. */
async function createDependenciesImplementation(_options, context) {
  const { workspaceRoot, projects } = context;
  const locked = _options?.analyzeLockfile && existsSync(join(workspaceRoot, "bun.lock")) ? readBunLockGraph(workspaceRoot) : undefined;
  const byRoot = new Map(Object.entries(projects).map(([name, project]) => [resolve(workspaceRoot, project.root), name]));
  const byPackage = new Map();
  const manifests = new Map();
  const goModules = new Map();
  const goManifests = new Map();
  const edges = new Map();
  const workspace = existsSync(join(workspaceRoot, "Cargo.toml")) ? readToml(join(workspaceRoot, "Cargo.toml")).workspace ?? {} : {};
  const add = (source, target, sourceFile, basenameHint) => {
    if (!target || source === target) return;
    const tracked = nxTrackedSourceFile(context, source, sourceFile, basenameHint);
    if (!existsSync(join(workspaceRoot, tracked))) return;
    edges.set(`${source}\0${target}\0${tracked}`, { source, target, sourceFile: tracked, type: "static" });
  };
  const authorityPath = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
  const generators = existsSync(authorityPath) ? JSON.parse(readFileSync(authorityPath, "utf8")).generatorContracts ?? {} : {};
  for (const contract of Object.values(generators)) {
    const participants = [contract.ownerPath, ...contract.outputRoots.flatMap((output) => output.producer ? [output.producer.ownerPath] : [])];
    if (!participants.some((root) => typeof root === "string" && byRoot.has(resolve(workspaceRoot, root)))) continue;
    for (const output of contract.outputRoots) {
      if (!output.producer) continue;
      const producer = output.producer, separator = producer.target.lastIndexOf(":"), name = producer.target.slice(0, separator);
      if (byRoot.get(resolve(workspaceRoot, producer.ownerPath)) !== name || !projects[name]?.targets?.[producer.target.slice(separator + 1)]) throw new Error(`Generator output has no Nx producer: ${output.path}`);
    }
  }
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
        for (const dependency of nativeDependencies(readToml(cargo), workspace)) {
          add(name, byRoot.get(resolve(dependency.workspace ? workspaceRoot : root, dependency.path)), file.file);
        }
      }
      if (file.file.endsWith("package.json")) {
        const manifest = manifests.get(name) ?? (existsSync(join(workspaceRoot, file.file)) ? JSON.parse(readFileSync(join(workspaceRoot, file.file), "utf8")) : undefined);
        if (!manifest) continue;
        for (const dependency of Object.keys({ ...manifest.dependencies, ...manifest.devDependencies, ...manifest.peerDependencies, ...manifest.optionalDependencies })) {
          const key = locked?.resolve(locked.workspacePackages.has(manifest.name) ? manifest.name : "", dependency);
          add(name, byPackage.get(dependency) ?? (locked?.workspacePackages.has(key) ? byRoot.get(resolve(workspaceRoot, locked.workspacePackages.get(key))) : key && locked.externalNodes[`npm:${key}`] ? `npm:${key}` : undefined), file.file);
        }
      }
    }
    const go = goManifests.get(name);
    if (go) {
      const provenance = nxPath(relative(workspaceRoot, go.path));
      if (files.some((file) => file.file === provenance || file.file.endsWith("go.mod"))) {
        const sourceFile = owned(provenance, project.root) ? provenance : nxPath(relative(workspaceRoot, join(root, SCRIPT_BASENAME)));
        for (const dependency of go.requires) add(name, goModules.get(dependency), sourceFile);
        for (const replacement of go.replacements) add(name, goModules.get(replacement.module) ?? byRoot.get(resolve(dirname(go.path), replacement.path)), sourceFile);
      }
    }
  }
  await collectImportEdges(workspaceRoot, projectFiles, projects, byPackage, locked, add);
  return [...edges.values(), ...(locked?.dependencies ?? [])];
}

/** ♻️ Reloads authored graph code and policy while retaining Nx's daemon and task cache. */
function implementationRevision() {
  const hash = createHash("sha256").update(readPhysicalSource(fileURLToPath(import.meta.url)));
  for (const path of ["⚡️caching/🔣️policy.json", "🔣️taxonomy.json", COMMAND_INPUT_MODULE, RUNTIME_COMPONENT_MODULE, SOURCE_INPUT_MODULE, BROWSER_SESSION_MODULE, PLAYGROUND_COMPOSITION_MODULE, NATIVE_HOST_MODULE, COMPONENT_DEPLOYMENT_MODULE, "../../../../🔨️modules/🪪️identity/📁️installation/🟨️.mjs", "../../../../🔨️modules/🪪️identity/📁️installation/🧬️schema/🔣️.json"]) hash.update(readPhysicalSource(join(LIBRARY_ROOT, path)));
  return hash.digest("hex");
}

let reloadedImplementation;

function invokeCurrentImplementation(kind, args) {
  return libraryBootstrap.then(() => {
    const revision = implementationRevision();
    if (revision === IMPLEMENTATION_REVISION) return kind === "nodes" ? emojiProjectJsonNodes(...args) : kind === "authority" ? dependencyResolutionAuthorityImplementation(...args) : createDependenciesImplementation(...args);
    if (reloadedImplementation?.revision !== revision) {
      const entry = { revision, module: importRevision(import.meta.url, revision).catch((error) => { if (reloadedImplementation === entry) reloadedImplementation = undefined; throw error; }) };
      reloadedImplementation = entry;
    }
    return reloadedImplementation.module.then((module) => {
      if (module.cacheInternals === cacheInternals) throw new Error("Graph runtime retained an obsolete implementation");
      return kind === "nodes" ? module.default.createNodesV2[1](...args) : kind === "authority" ? module.dependencyResolutionAuthority(...args) : module.createDependencies(...args);
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
  const source = (path) => { hash.update(JSON.stringify(nxPath(relative(workspaceRoot, path)))); hash.update(existsSync(path) ? readPhysicalSource(path) : "\0absent\0"); };
  for (const path of ["nx.json", "Cargo.toml", "go.work", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🟨️.mjs", "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🕸️dependencies/🟨️.mjs"]) source(join(workspaceRoot, path));
  const recipe = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools"), manifest = join(recipe, "package.json");
  for (const path of [join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/📜️script.ts"), join(recipe, "📜️script.ts"), manifest, join(recipe, "bun.lock")]) source(path);
  if (existsSync(manifest)) for (const path of [...new Set(Object.values(JSON.parse(readPhysicalSource(manifest)).semio?.toolPatches ?? {}))].sort()) {
    if (typeof path !== "string") throw new Error("Invalid tooling authority patch path");
    const file = resolve(workspaceRoot, path), local = nxPath(relative(workspaceRoot, file));
    if (isAbsolute(local) || local === ".." || local.startsWith("../")) throw new Error("Tooling authority patch must belong to this workspace");
    source(file);
  }
  if (existsSync(join(workspaceRoot, "bun.lock"))) { readBunLockGraph(workspaceRoot); hash.update(BUN_LOCK_CACHE.hash); }
  else hash.update("\0no-bun-lock\0");
  for (const [name, project] of Object.entries(projects).sort(([a], [b]) => a.localeCompare(b))) {
    hash.update(JSON.stringify([name, nxPath(project.root)]));
    for (const filename of ["package.json", "📋️project.json", "Cargo.toml"]) source(join(workspaceRoot, project.root, filename));
    const go = goManifest(project.root, workspaceRoot);
    if (go) source(go);
  }
  return hash.digest("hex");
}

/** 🏛️ Supplies current roots, manifests, lock scope, and actual implementation identity behind an owned digest interface. */
export function dependencyResolutionAuthority(...args) { return invokeCurrentImplementation("authority", args); }


export function createDependencies(...args) { return invokeCurrentImplementation("dependencies", args); }

/**
 * 🧩️ One Nx project configuration this plugin generates, as its consumers read it.
 * @typedef {{ targets: Record<string, { cache?: boolean, continuous?: boolean, dependsOn?: string[], outputs?: string[], inputs?: unknown[], namedInputs?: Record<string, unknown[]>, options?: Record<string, unknown>, metadata?: Record<string, unknown> }>, root?: string, name?: string, projectType?: string, tags?: string[], sourceRoot?: string }} GeneratedProject
 */

/**
 * 🧩️ The `createNodesV2` result: one entry per matched file, each carrying the projects it contributes.
 * @typedef {[string, { projects: Record<string, GeneratedProject> }][]} GeneratedNodes
 */

export default {
  name: "@repo/emoji-project-json",
  createNodesV2: /** @type {[string, (files: readonly string[], options: unknown, context: { workspaceRoot: string }) => Promise<GeneratedNodes>]} */ ([`**/{${PROJECT_BASENAME},Cargo.toml,bun.lock,*.patch}`, (...args) => invokeCurrentImplementation("nodes", args)]),
  createDependencies,
};

export { libraryBootstrap };

export const cacheInternals = { inferenceCandidates, async declaredSourceInputs(...args) { await libraryBootstrap; return declaredSourceInputs(...args); }, nativeLockInputs, withWasmTooling, get runtimeComponentClosure() { return runtimeComponentClosure; }, async componentTargets(...args) { await libraryBootstrap; return componentTargets(...args); }, async playgroundSessionTargets(...args) { await libraryBootstrap; return playgroundSessionTargets(...args); }, playgroundPreparationTargets, collectPlaygroundCatalog, pluginSiteTargetsForCrate, bunLockGraph, printDocumentTargets, targetPolicy, matchesUncached, cacheableFamily, mutatingName, liveName, verifyCommand, nativeDependencies, nativeDependencyRoots, nativePreparation, withNativePreparation, cargoTargets, goDependencies, rustSourceFiles, createRustSourceCache, relativeScriptInputs, nativeTargetCommandInputs, targetScriptClosure, genericTargetCommandInputs, genericCommandFallbackInputs, generatorContractInputs, outputRootInputs, resolveOutputPath, generatorOutputCouplingInputs, projectInputs, rootCommandTargets, createDependenciesImplementation, importTargetsFromSource, collectImportEdges, projectFilesToProcess, moduleSourceFactCacheRoot, nxTrackedSourceFile, walkCargoToml };
