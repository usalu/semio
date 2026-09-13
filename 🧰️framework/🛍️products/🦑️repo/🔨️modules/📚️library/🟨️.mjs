import { existsSync, readFileSync, readdirSync, realpathSync, statSync } from "node:fs";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import { fileURLToPath } from "node:url";

const PROJECT_BASENAME = "📋️project.json";
const SCRIPT_BASENAME = "📜️script.ts";
const DEFAULT_EXECUTOR = "nx:run-commands";
const TEST_TARGET = "test";
const TEST_LEVELS = ["quick", "long", "exhaustive"];

const LIBRARY_ROOT = dirname(fileURLToPath(import.meta.url));
const RUNTIME_COMPONENT_MODULE = "🕸️dependencies/🧩️runtime/🟨️.mjs";
const runtimeRevision = createHash("sha256").update(readFileSync(join(LIBRARY_ROOT, RUNTIME_COMPONENT_MODULE))).digest("hex");
const { runtimeComponentClosure } = await import(new URL(`./${RUNTIME_COMPONENT_MODULE}?revision=${runtimeRevision}`, import.meta.url).href);
const SOURCE_INPUT_MODULE = "🕸️dependencies/🟦️typescript/🟨️.mjs";
const sourceInputRevision = createHash("sha256").update(readFileSync(join(LIBRARY_ROOT, SOURCE_INPUT_MODULE))).digest("hex");
const { readSourceInputContract, relativeSourceInputs } = await import(new URL(`./${SOURCE_INPUT_MODULE}?revision=${sourceInputRevision}`, import.meta.url).href);
const POLICY = JSON.parse(readFileSync(join(LIBRARY_ROOT, "⚡️caching/🔣️policy.json"), "utf8"));
const TAXONOMY = JSON.parse(readFileSync(join(LIBRARY_ROOT, "🔣️taxonomy.json"), "utf8"));
const IMPLEMENTATION_REVISION = new URL(import.meta.url).searchParams.get("revision") ?? implementationRevision();
const nxPath = (path) => path.split("\\").join("/");
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
  return [...files].sort();
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

const SCRIPT_IMPORT_CACHE = new Map();

/** 🗂️ Shares source reads and command closures only within one graph construction. */
function createScriptInputCache() {
  return { files: new Map(), closures: new Map() };
}

/** 🔗️ Collects executable import expressions while excluding erased TypeScript declarations. */
function commandImports(path, source, compiler) {
  const previous = SCRIPT_IMPORT_CACHE.get(path);
  if (previous?.source === source) return previous.imports;
  const imports = new Set(), add = (node) => { if (node && compiler.isStringLiteralLike(node) && node.text.startsWith(".")) imports.add(node.text); };
  const visit = (node) => {
    if (compiler.isImportTypeNode(node)) return;
    if (compiler.isImportDeclaration(node)) {
      const clause = node.importClause, bindings = clause?.namedBindings;
      if (!clause?.isTypeOnly && !(bindings && compiler.isNamedImports(bindings) && !clause.name && bindings.elements.length && bindings.elements.every(element => element.isTypeOnly))) add(node.moduleSpecifier);
      return;
    }
    if (compiler.isExportDeclaration(node)) {
      if (!node.isTypeOnly) add(node.moduleSpecifier);
      return;
    }
    if (compiler.isImportEqualsDeclaration(node)) {
      if (!node.isTypeOnly && compiler.isExternalModuleReference(node.moduleReference)) add(node.moduleReference.expression);
      return;
    }
    if (compiler.isCallExpression(node)) {
      const expression = node.expression;
      if (expression.kind === compiler.SyntaxKind.ImportKeyword || compiler.isIdentifier(expression) && expression.text === "require" || compiler.isCallExpression(expression) && compiler.isIdentifier(expression.expression) && expression.expression.text === "createRequire") add(node.arguments[0]);
    }
    compiler.forEachChild(node, visit);
  };
  if (!/\.(?:json|d\.[cm]?ts)$/.test(path)) visit(compiler.createSourceFile(path, source, compiler.ScriptTarget.Latest, false));
  const result = [...imports];
  SCRIPT_IMPORT_CACHE.set(path, { source, imports: result });
  return result;
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
      for (const entry of commandImports(path, readFileSync(path, "utf8"), compiler)) {
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
    { json: "{workspaceRoot}/package.json", fields: ["name"] },
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
  return typeof command === "string" && command.includes(" native cargo metadata ") ? ["{workspaceRoot}/**/Cargo.toml", "{workspaceRoot}/Cargo.lock"] : [];
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
 * output — but a proven Nx contract (see the `⚡️caching/📜️script.ts` `CacheVerifyScript` fixture family) is
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
    if (!existsSync(manifestPath)) throw new Error(`Native dependency has no Cargo manifest: ${manifestPath}`);
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
    if (!/^(?:build|check|lint|test|wasm|native|component|extension-package|package|font-tool|bench)(?:-|$)/.test(name) || generatorTargets.has(`${project.name}:${name}`)) continue;
    const tests = /^(?:test|bench)(?:-|$)/.test(name);
    if (!plans.has(tests)) plans.set(tests, nativePreparation(nativeRoot, workspaceRoot, contracts, tests, cache, closures));
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
    const extensions = tools.includes("cargo") ? "{rs,toml,json,semio,wit,wgsl,glsl,h,c,cpp}" : tools.includes("go") ? "{go,mod,sum,json,ts}" : tools.includes("dotnet") ? "{cs,fs,vb,csproj,fsproj,vbproj,props,targets,resx,json}" : tools.includes("python") ? "{py,pyi,toml,json}" : "{ts,tsx,js,jsx,mjs,cjs,json,css,scss,html,svg,wit}";
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
  const production = ["default", "!{projectRoot}/**/🧪️tests/**/*", "!{projectRoot}/**/🧫️fixtures/**/*", "!{workspaceRoot}/**/🧫️fixtures/**/*", "!{projectRoot}/**/*.feature", "!{projectRoot}/**/*.stories.{ts,tsx}"];
  if (owner !== root) production.push(`!{workspaceRoot}/${owner}/**/🧪️tests/**/*`, `!{workspaceRoot}/${owner}/**/🧫️fixtures/**/*`);
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
  const native = (sources, productionOnly = false) => !tools.includes("cargo") ? ["production"] : [...new Set(sources ? [`{workspaceRoot}/${nativeRoot}/Cargo.toml`, ...sources] : ["{projectRoot}/**/*", ...(owner !== root ? [`{workspaceRoot}/${owner}/**/*`] : [])]), ...(productionOnly ? ["!{projectRoot}/**/🧪️tests/**/*", "!{projectRoot}/**/🧫️fixtures/**/*", "!{workspaceRoot}/**/🧫️fixtures/**/*", ...(owner !== root ? [`!{workspaceRoot}/${owner}/**/🧪️tests/**/*`] : [])] : []), ...POLICY.generatedDirectories.flatMap((directory) => [`!{projectRoot}/**/${directory}/**/*`, ...(owner !== root ? [`!{workspaceRoot}/${owner}/**/${directory}/**/*`] : [])]), ...exclusions];
  const artifactTypeScript = json.tags?.includes("role:artifact") && json.tags.includes("language:typescript") && owner !== root && existsSync(join(workspaceRoot, root, SCRIPT_BASENAME));
  const artifactSource = join(workspaceRoot, owner, "🟦️.ts");
  const artifactSources = artifactTypeScript ? [...relativeScriptInputs([artifactSource], workspaceRoot, scripts), "{projectRoot}/package.json", ...exclusions] : [];
  const javascript = POLICY.toolchains.javascript;
  const artifactCommandSources = artifactTypeScript ? [...relativeScriptInputs([join(workspaceRoot, root, SCRIPT_BASENAME)], workspaceRoot, scripts), `{workspaceRoot}/bunfig.toml`, { externalDependencies: ["typescript"] }, ...javascript.environment.map((env) => ({ env })), ...javascript.commands.map((runtime) => ({ runtime })), { runtime: 'node -p "process.platform.concat(process.arch)"' }] : [];
  return { ...declarations, ...declaredSourceInputs(json, workspaceRoot), default: [...inputs, ...(declarations.default ?? []), ...exclusions], production: [...production, ...(declarations.production ?? [])], nativeSources: [...native(nativeSources, true), ...(declarations.nativeSources ?? [])], nativeTestSources: [...native(nativeTests), ...(declarations.nativeSources ?? []), ...(declarations.nativeTestSources ?? [])], ...(artifactTypeScript ? { artifactSources: [...artifactSources, ...(declarations.artifactSources ?? [])], artifactCommandSources: [...artifactCommandSources, ...(declarations.artifactCommandSources ?? [])] } : {}) };
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
  for (const [name, target] of Object.entries(withLeveledTestTargets(declared))) {
    const policy = targetPolicy(name, targetWithDefaults({ ...(POLICY.targetDefaults?.[name] ?? {}), ...target }, root, ownsScript));
    const nativeTarget = nativeProject && /^(build|wasm|native|test(?:-(?:quick|long|exhaustive))?$|lint|check$)/.test(name) || policy.options?.command?.includes("⚡️caching/🦀️cargo/📜️script.ts");
    const artifactTarget = artifactTypeScript && /^(?:build|check|test(?:-(?:quick|long|exhaustive))?)$/.test(name);
    if (nativeTarget) {
      policy.parallelism ??= false;
      policy.inputs = [name.startsWith("test") ? "nativeTestSources" : "nativeSources", name.startsWith("test") ? "^nativeTestSources" : "^nativeSources", ...nativeTargetCommandInputs(policy, workspaceRoot, commandInputs, scripts), ...(name.startsWith("component-") ? [{ env: "SEMIO_PLUGIN_SYMBOLS" }] : []), ...nativeLockInputs(policy.options?.command)];
    }
    if (artifactTarget) policy.inputs = ["artifactSources", "artifactCommandSources"];
    if (!nativeTarget && !artifactTarget && policy.cache) policy.inputs = [...(policy.inputs ?? ["default", "^default"]), ...genericTargetCommandInputs(policy, workspaceRoot, genericFallback, scripts)];
    normalized[name] = policy;
  }
  for (const contract of Object.values(contracts)) {
    if (contract.ownership !== "owned" || contract.ownerPath !== root || !contract.checkTarget) continue;
    const check = contract.checkTarget.slice(contract.checkTarget.lastIndexOf(":") + 1), target = normalized[check];
    if (!target) continue;
    const discovery = generatorContractInputs(contract);
    normalized[check] = { ...target, cache: true, inputs: [...target.inputs, ...discovery.inputs, ...contract.outputRoots.map((output) => outputRootInputs(output, workspaceRoot))], ...(discovery.dependsOn.length ? { dependsOn: [...new Set([...(target.dependsOn ?? []), ...discovery.dependsOn])] } : {}) };
  }
  for (const [name, target] of Object.entries(normalized)) {
    if (target.cache !== true) continue;
    const extra = generatorOutputCouplingInputs(name, target, declared, root, workspaceRoot);
    if (extra.length) normalized[name] = { ...target, inputs: [...target.inputs, ...extra] };
  }
  return { ...json, name: json.name, root, namedInputs: projectInputs({ ...json, targets: declared }, root, workspaceRoot, facts, scripts), targets: normalized };
}

/** 🛠️ Exposes wasm-pack's immutable optimizer preparation to Nx before compiler execution. */
function withWasmTooling(targets, source) {
  if (!targets.wasm || !/\brunWasmPackWebBuild\s*\(/.test(source)) return targets;
  return { ...targets, wasm: { ...targets.wasm, dependsOn: [...new Set([...(targets.wasm.dependsOn ?? []), "workspace:deps-wasm-opt"])] } };
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
  if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.role)) return {};
  if (!manifest.lib?.["crate-type"]?.includes("cdylib")) throw new Error(`Component ${metadata.component.package} needs a cdylib target: ${path}`);
  const script = nxPath(relative(workspaceRoot, join(LIBRARY_ROOT, "⚡️caching/🦀️cargo/📜️script.ts")));
  const webRoot = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript";
  const webSourceRoot = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐️browser-bundle";
  const deployment = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment";
  const moduleCatalog = JSON.parse(readFileSync(join(workspaceRoot, deployment, "🗺️catalog.json"), "utf8"));
  const moduleDirectory = moduleCatalog.modules.find((row) => metadata.component.package === `semio:${row.pluginId}`)?.directoryName;
  if (!moduleDirectory || moduleDirectory.includes("/") || moduleDirectory.includes("\\") || [".", ".."].includes(moduleDirectory)) throw new Error(`Component needs an authored deployment directory: ${path}`);
  commandInputs ??= nativeCommandInputs(workspaceRoot);
  return Object.fromEntries(["dev", "release"].flatMap((profile) => [[`component-${profile}`, {
    executor: DEFAULT_EXECUTOR,
    cache: true,
    parallelism: false,
    inputs: ["nativeSources", "^nativeSources", ...commandInputs, { env: "SEMIO_PLUGIN_SYMBOLS" }],
    outputs: [`{projectRoot}/dist/component-${profile}`],
    options: { cwd: ".", command: `bun ${JSON.stringify(script)} native component ${profile} --manifest ${JSON.stringify(nxPath(relative(workspaceRoot, path)))}` },
  }], [`materialize-${profile}`, {
    executor: DEFAULT_EXECUTOR,
    cache: true,
    dependsOn: [`component-${profile}`, `@semio-tech/framework-plugin-web:support-${profile}`, ...(profile === "release" ? ["workspace:deps-wasm-opt"] : [])],
    inputs: ["production", "^production", { dependentTasksOutputFiles: "**/*" }, `{workspaceRoot}/${webRoot}/**/*.{ts,json}`, `{workspaceRoot}/${webSourceRoot}/**/*.ts`, `!{workspaceRoot}/${webSourceRoot}/**/🧪️tests/**/*.ts`, `{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🛂️actor-exports/🔣️.json`, `{workspaceRoot}/${deployment}/*.json`, `!{workspaceRoot}/${webRoot}/dist/**/*`],
    outputs: [`{workspaceRoot}/${webRoot}/dist/${profile}/🔌️plugin-modules/${moduleDirectory}`],
    options: { cwd: ".", command: `bun ${JSON.stringify(`${webRoot}/📜️script.ts`)} materialize ${profile} --manifest ${JSON.stringify(nxPath(relative(workspaceRoot, path)))}` },
  }]]));
}

/** 🎮️ Gives each Cargo-declared playground session an independent generated artifact. */
function playgroundSessionTargets(configFiles, workspaceRoot) {
  const variants = new Set();
  for (const file of configFiles) {
    if (!file.endsWith("Cargo.toml") || file.includes("\uFFFD") || file.includes(".🧬semio") || file.startsWith("compose/") || file.startsWith("temp/compose/") || POLICY.generatedDirectories.some((directory) => file.split("/").includes(directory))) continue;
    const metadata = readToml(join(workspaceRoot, file)).package?.metadata;
    if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.role)) continue;
    for (const playground of metadata.semio.playground ?? []) {
      const variant = playground.variant;
      if (typeof variant !== "string" || !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(variant) || variants.has(variant)) throw new Error(`Invalid or duplicate playground variant in ${file}: ${variant}`);
      variants.add(variant);
    }
  }
  return Object.fromEntries([...variants].sort().map((variant) => [`session-${variant}`, {
    cache: true,
    inputs: ["default", { dependentTasksOutputFiles: "**/*" }],
    dependsOn: ["generate"],
    outputs: [`{projectRoot}/dist/sessions/${variant}`],
    options: { command: `bun ./📜️script.ts session ${variant}` },
  }]));
}

/** 🎮️ Declares completed runtime prerequisites, cacheable validation and profile-specific browser producers.
 * React activation owns its restorable runtime directory. WGPU activation publishes live extensions,
 * fonts and reload notifications, so it must execute after every cacheable preparation. */
function playgroundPreparationTargets(configFiles, workspaceRoot, projectRoot) {
  const components = new Map(), playgrounds = [];
  const projectAt = (root) => {
    const path = join(workspaceRoot, root, PROJECT_BASENAME);
    return existsSync(path) ? JSON.parse(readFileSync(path, "utf8")) : undefined;
  };
  for (const path of configFiles) {
    if (!path.endsWith("Cargo.toml") || path.includes("\uFFFD") || path.includes(".🧬semio") || path.startsWith("compose/") || path.startsWith("temp/compose/") || POLICY.generatedDirectories.some((directory) => path.split("/").includes(directory))) continue;
    const manifest = readToml(join(workspaceRoot, path)), metadata = manifest.package?.metadata;
    if (!metadata?.component?.package || !["plugin", "extension"].includes(metadata.semio?.role)) continue;
    const id = metadata.component.package.slice(6), root = nxPath(dirname(path));
    if (components.has(id)) throw new Error(`Duplicate component identity: ${id}`);
    components.set(id, { ...metadata.semio, project: projectAt(root)?.name ?? manifest.package.name });
    for (const row of metadata.semio.playground ?? []) playgrounds.push({ ...row, pluginId: id });
  }
  const composition = JSON.parse(readFileSync(join(workspaceRoot, projectRoot, "package.json"), "utf8"));
  const baseline = ["🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust", "🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust", "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🫀️core/📦️packages/🦀️rust"];
  const linked = (composition.semio?.browserSessionFactories ?? []).map((row) => row.engine);
  const wgpuRoot = nxPath("🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🟦️typescript"), wgpuProject = projectAt(wgpuRoot);
  if (!wgpuProject?.name || !wgpuProject.targets?.wasm || !wgpuProject.targets?.["wasm-release"]) throw new Error(`WGPU renderer must name both authored wasm profile producers: ${wgpuRoot}`);
  const result = {};
  for (const playground of playgrounds) {
    const own = components.get(playground.pluginId), selected = runtimeComponentClosure([...components].map(([pluginId, row]) => ({ ...row, pluginId, dependsOn: [...(row.extends ? [row.extends] : []), ...(row["depends-on"] ?? [])] })), [playground.pluginId]);
    const engines = new Set([...baseline, ...linked, ...(own.host ? playgrounds : [playground]).flatMap((row) => row.engines ?? [])].map((path) => {
      const root = nxPath(relative(workspaceRoot, resolve(workspaceRoot, path))), project = projectAt(root);
      if (root.startsWith("../") || !project?.name || !project.targets?.wasm) throw new Error(`Playground engine must name an authored wasm producer: ${path}`);
      return `${project.name}:wasm`;
    }));
    for (const profile of ["dev", "release"]) {
      for (const command of ["serve", "dev"]) result[`${command}-${playground.variant}-react-${profile}`] = { cache: false, continuous: true, outputs: [], dependsOn: [`activate-${playground.variant}-react-${profile}`], options: { command: `bun ./📜️script.ts serve ${playground.variant} react ${profile}` } };
      result[`activate-${playground.variant}-react-${profile}`] = {
      cache: true,
      parallelism: false,
      outputs: [`{projectRoot}/dist/runtime/${profile}/${playground.variant}`],
      inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
      dependsOn: [`prepare-${playground.variant}-react-${profile}`],
      options: { command: `bun ./📜️script.ts activate ${playground.variant} react ${profile}` },
      };
      result[`prepare-${playground.variant}-react-${profile}`] = {
      cache: true,
      outputs: [],
      inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, `@semio-tech/framework-plugin-web:support-${profile}`, "semio-framework-os-infinite:fonts", ...engines, ...[...selected].sort().map((id) => `${components.get(id).project}:materialize-${profile}`)],
      options: { command: `bun ./📜️script.ts prepare ${playground.variant} react ${profile}` },
      };
      for (const command of ["serve", "dev"]) result[`${command}-${playground.variant}-wgpu-${profile}`] = { cache: false, continuous: true, outputs: [], dependsOn: [`activate-${playground.variant}-wgpu-${profile}`], options: { command: `bun ../../../📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🌐️server/📜️script.ts serve ${playground.variant} ${profile}` } };
      result[`activate-${playground.variant}-wgpu-${profile}`] = {
      cache: false,
      parallelism: false,
      outputs: [],
      inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
      dependsOn: [`prepare-${playground.variant}-wgpu-${profile}`],
      options: { command: `bun ./📜️script.ts activate ${playground.variant} wgpu ${profile}` },
      };
      result[`prepare-${playground.variant}-wgpu-${profile}`] = {
      cache: true,
      outputs: [],
      inputs: [{ dependentTasksOutputFiles: "**/*", transitive: true }],
      dependsOn: [`@semio-tech/plugin-registry:session-${playground.variant}`, `@semio-tech/framework-plugin-web:support-${profile}`, "semio-framework-os-infinite:fonts", `${wgpuProject.name}:${profile === "release" ? "wasm-release" : "wasm"}`, `${wgpuProject.name}:generate-browser-boot`, `${wgpuProject.name}:generate-frame-worker`, ...[...selected].sort().map((id) => `${components.get(id).project}:materialize-${profile}`)],
      options: { command: `bun ./📜️script.ts prepare ${playground.variant} wgpu ${profile}` },
      };
    }
    const name = `build-${playground.variant}-react-release`;
    result[name] = {
      cache: true,
      outputs: [`{projectRoot}/dist/${name}`],
      inputs: ["production", "^production", { dependentTasksOutputFiles: "**/*", transitive: true }, { runtime: `bun ${JSON.stringify(nxPath(relative(workspaceRoot, resolve(workspaceRoot, projectRoot, "../../🚚️distribution/📜️script.ts"))))} inputs` }],
      dependsOn: [...result[`prepare-${playground.variant}-react-release`].dependsOn, "@semio-tech/assets:build"],
      options: { command: `bun ../../🚚️distribution/📜️script.ts build ${playground.variant} react release`, forwardAllArgs: true },
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
function emojiProjectJsonNodes(configFiles, _options, context) {
  const { workspaceRoot } = context;
  const rootsByName = new Map(), facts = new Map(), scripts = createScriptInputCache();
  const commandInputs = configFiles.some((path) => path.endsWith("Cargo.toml") && !path.includes(".🧬semio") && !POLICY.generatedDirectories.some((name) => path.split("/").includes(name))) ? nativeCommandInputs(workspaceRoot, scripts) : undefined;
  const contractPath = join(workspaceRoot, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json");
  const contracts = existsSync(contractPath) ? JSON.parse(readFileSync(contractPath, "utf8")).generatorContracts ?? {} : {};

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
      return [configFile, { projects: { [name]: projectWithDefaults(json, root, projectDir, workspaceRoot, contracts, facts, commandInputs, scripts) } }];
    })
    .filter(Boolean);
  const declaredRoots = new Set([...rootsByName.values()]);
  for (const configFile of configFiles.filter((file) => file.endsWith("Cargo.toml"))) {
    if (configFile.includes("\uFFFD") || configFile.startsWith("compose/") || configFile.startsWith("temp/compose/") || configFile.includes(".🧬semio") || POLICY.generatedDirectories.some((name) => configFile.split("/").includes(name))) continue;
    const projectDir = dirname(join(workspaceRoot, configFile));
    const root = nxPath(relative(workspaceRoot, projectDir)) || ".";
    if (declaredRoots.has(root)) continue;
    const manifest = readToml(join(workspaceRoot, configFile));
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

/** 🕸️ Native manifests and package imports contribute edges without spawning a build or installer. */
function createDependenciesImplementation(_options, context) {
  const { workspaceRoot, projects } = context;
  const locked = _options?.analyzeLockfile && existsSync(join(workspaceRoot, "bun.lock")) ? readBunLockGraph(workspaceRoot) : undefined;
  const byRoot = new Map(Object.entries(projects).map(([name, project]) => [resolve(workspaceRoot, project.root), name]));
  const byPackage = new Map();
  const manifests = new Map();
  const goModules = new Map();
  const goManifests = new Map();
  const edges = new Map();
  const workspace = existsSync(join(workspaceRoot, "Cargo.toml")) ? readToml(join(workspaceRoot, "Cargo.toml")).workspace ?? {} : {};
  const add = (source, target, sourceFile) => {
    if (target && source !== target) edges.set(`${source}\0${target}\0${sourceFile}`, { source, target, sourceFile, type: "static" });
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
  for (const [name, project] of Object.entries(projects)) {
    const root = resolve(workspaceRoot, project.root);
    const cargo = join(root, "Cargo.toml");
    const go = goManifests.get(name);
    if (go) {
      const provenance = owned(nxPath(relative(workspaceRoot, go.path)), project.root) ? go.path : join(root, SCRIPT_BASENAME);
      for (const dependency of go.requires) add(name, goModules.get(dependency), nxPath(relative(workspaceRoot, provenance)));
      for (const replacement of go.replacements) add(name, goModules.get(replacement.module) ?? byRoot.get(resolve(dirname(go.path), replacement.path)), nxPath(relative(workspaceRoot, provenance)));
    }
    if (existsSync(cargo)) {
      for (const dependency of nativeDependencies(readToml(cargo), workspace)) {
        add(name, byRoot.get(resolve(dependency.workspace ? workspaceRoot : root, dependency.path)), nxPath(relative(workspaceRoot, cargo)));
      }
    }
    const manifest = manifests.get(name);
    if (manifest) for (const dependency of Object.keys({ ...manifest.dependencies, ...manifest.devDependencies, ...manifest.peerDependencies, ...manifest.optionalDependencies })) {
      const key = locked?.resolve(locked.workspacePackages.has(manifest.name) ? manifest.name : "", dependency);
      add(name, byPackage.get(dependency) ?? (locked?.workspacePackages.has(key) ? byRoot.get(resolve(workspaceRoot, locked.workspacePackages.get(key))) : key && locked.externalNodes[`npm:${key}`] ? `npm:${key}` : undefined), nxPath(relative(workspaceRoot, join(root, "package.json"))));
    }
    for (const file of name === "workspace" ? [] : context.fileMap?.projectFileMap?.[name] ?? []) {
      if (!/\.[cm]?[jt]sx?$/.test(file.file) || file.file.endsWith(SCRIPT_BASENAME)) continue;
      let text;
      try {
        text = readFileSync(join(workspaceRoot, file.file), "utf8");
      } catch (error) {
        if (error.code === "ENOENT") continue;
        throw error;
      }
      for (const match of text.matchAll(/(?:\bfrom\s*|\bimport\s*(?:\(\s*)?|\brequire\s*\(\s*)["']([^"']+)["']/g)) {
        const specifier = match[1];
        const packageName = specifier.startsWith("@") ? specifier.split("/").slice(0, 2).join("/") : specifier.split("/")[0];
        if (!specifier.startsWith(".")) {
          const key = locked?.resolveImport(file.file, packageName);
          add(name, byPackage.get(packageName) ?? (key && locked.externalNodes[`npm:${key}`] ? `npm:${key}` : undefined), file.file);
        }
        else {
          const path = nxPath(relative(workspaceRoot, resolve(dirname(join(workspaceRoot, file.file)), specifier)));
          const target = Object.entries(projects).filter(([candidate, p]) => candidate !== "workspace" && owned(path, p.root)).sort((a, b) => b[1].root.length - a[1].root.length)[0]?.[0];
          add(name, target, file.file);
        }
      }
    }
  }
  return [...edges.values(), ...(locked?.dependencies ?? [])];
}

/** ♻️ Reloads authored graph code and policy while retaining Nx's daemon and task cache. */
function implementationRevision() {
  return createHash("sha256").update(readFileSync(fileURLToPath(import.meta.url))).update(readFileSync(join(LIBRARY_ROOT, "⚡️caching/🔣️policy.json"))).update(readFileSync(join(LIBRARY_ROOT, RUNTIME_COMPONENT_MODULE))).update(readFileSync(join(LIBRARY_ROOT, SOURCE_INPUT_MODULE))).digest("hex");
}

function invokeCurrentImplementation(kind, args) {
  const revision = implementationRevision();
  if (revision === IMPLEMENTATION_REVISION) return kind === "nodes" ? emojiProjectJsonNodes(...args) : createDependenciesImplementation(...args);
  const url = new URL(import.meta.url);
  url.searchParams.set("revision", revision);
  return import(url.href).then((module) => kind === "nodes" ? module.default.createNodesV2[1](...args) : module.createDependencies(...args));
}

export function createDependencies(...args) { return invokeCurrentImplementation("dependencies", args); }

export default {
  name: "@repo/emoji-project-json",
  createNodesV2: [`**/{${PROJECT_BASENAME},Cargo.toml,bun.lock,*.patch}`, (...args) => invokeCurrentImplementation("nodes", args)],
  createDependencies,
};

export const cacheInternals = { declaredSourceInputs, nativeLockInputs, withWasmTooling, runtimeComponentClosure, playgroundPreparationTargets, bunLockGraph, printDocumentTargets, targetPolicy, matchesUncached, cacheableFamily, mutatingName, liveName, verifyCommand, nativeDependencies, nativeDependencyRoots, nativePreparation, withNativePreparation, cargoTargets, goDependencies, rustSourceFiles, createRustSourceCache, relativeScriptInputs, nativeTargetCommandInputs, targetScriptClosure, genericTargetCommandInputs, genericCommandFallbackInputs, generatorContractInputs, outputRootInputs, resolveOutputPath, generatorOutputCouplingInputs, projectInputs, rootCommandTargets };
