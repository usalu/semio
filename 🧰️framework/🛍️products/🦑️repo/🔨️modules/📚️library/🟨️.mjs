import { existsSync, readFileSync, readdirSync } from "node:fs";
import { dirname, join, relative, resolve } from "node:path";
import { createRequire } from "node:module";
import { fileURLToPath } from "node:url";

const PROJECT_BASENAME = "📋️project.json";
const SCRIPT_BASENAME = "📜️script.ts";
const DEFAULT_EXECUTOR = "nx:run-commands";
const TEST_TARGET = "test";
const TEST_LEVELS = ["quick", "long", "exhaustive"];

const LIBRARY_ROOT = dirname(fileURLToPath(import.meta.url));
const POLICY = JSON.parse(readFileSync(join(LIBRARY_ROOT, "⚡️caching/🔣️policy.json"), "utf8"));
const nxPath = (path) => path.split("\\").join("/");
const owned = (path, root) => root === "." || path === root || path.startsWith(`${root}/`);
const matchesCommand = (name, commands) => commands.some((command) => name === command || name.startsWith(`${command}-`));

/** 🦀️ Tokenizes module syntax without interpreting strings and comments as declarations. */
function rustInputTokens(source) {
  const tokens = [];
  for (let i = 0; i < source.length;) {
    if (/\s/.test(source[i])) { i++; continue; }
    if (source.startsWith("//", i)) { const end = source.indexOf("\n", i); i = end < 0 ? source.length : end; continue; }
    if (source.startsWith("/*", i)) {
      let depth = 1; i += 2;
      while (i < source.length && depth) {
        if (source.startsWith("/*", i)) { depth++; i += 2; }
        else if (source.startsWith("*/", i)) { depth--; i += 2; }
        else i++;
      }
      continue;
    }
    const raw = /^(?:b|c)?r(#+)?"/.exec(source.slice(i));
    if (raw) {
      const start = i + raw[0].length, delimiter = `"${raw[1] ?? ""}`, end = source.indexOf(delimiter, start);
      if (end < 0) throw new Error("Unterminated Rust raw string");
      tokens.push({ text: source.slice(start, end), string: true }); i = end + delimiter.length; continue;
    }
    if (source[i] === '"' || /^[bc]"/.test(source.slice(i))) {
      const start = source[i] === '"' ? i : i + 1; i = start + 1;
      while (i < source.length && source[i] !== '"') { i += source[i] === "\\" ? 2 : 1; }
      const literal = source.slice(start, ++i);
      try { tokens.push({ text: JSON.parse(literal), string: true }); }
      catch { tokens.push({ text: literal.slice(1, -1), string: true }); }
      continue;
    }
    const character = /^(?:b)?'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'/u.exec(source.slice(i));
    if (character) { tokens.push({ text: character[0], string: true }); i += character[0].length; continue; }
    const identifier = /^(?:r#)?[\p{ID_Start}_][\p{ID_Continue}]*/u.exec(source.slice(i));
    if (identifier) { tokens.push({ text: identifier[0], identifier: true }); i += identifier[0].length; continue; }
    tokens.push({ text: source[i++] });
  }
  return tokens;
}

/** 📥️ Resolves Cargo entry points, inline/path modules and literal include assets without running a compiler. */
function rustSourceFiles(entries, manifestRoot, facts = new Map()) {
  const files = new Set(), visited = new Set();
  const visit = (file, moduleBase = dirname(file)) => {
    files.add(file);
    const identity = `${file}\0${moduleBase}`;
    if (!existsSync(file) || visited.has(identity)) return;
    visited.add(identity);
    if (!file.endsWith(".rs")) return;
    if (!facts.has(file)) {
      const tokens = rustInputTokens(readFileSync(file, "utf8")), pairs = new Map(), stack = [];
      for (let i = 0; i < tokens.length; i++) {
        if (tokens[i].string) continue;
        if (["(", "[", "{"].includes(tokens[i].text)) stack.push(i);
        else if ([")", "]", "}"].includes(tokens[i].text)) { const open = stack.pop(); if (open !== undefined) pairs.set(open, i); }
      }
      facts.set(file, { tokens, pairs });
    }
    const { tokens, pairs } = facts.get(file);
    const literal = (start, end) => {
      if (tokens[start]?.string && start + 1 === end) return tokens[start].text;
      if (tokens[start]?.text === "env" && tokens[start + 1]?.text === "!" && tokens[start + 3]?.text === "CARGO_MANIFEST_DIR") return manifestRoot;
      if (tokens[start]?.text === "concat" && tokens[start + 1]?.text === "!") {
        const parts = []; let from = start + 3;
        for (let i = from; i < end - 1; i++) {
          if (pairs.has(i)) { i = pairs.get(i); continue; }
          if (tokens[i].text === ",") { parts.push(literal(from, i)); from = i + 1; }
        }
        if (from < end - 1) parts.push(literal(from, end - 1));
        return parts.join("");
      }
      throw new Error(`Dynamic Rust include requires explicit source inputs: ${file}`);
    };
    for (let i = 0; i < tokens.length - 3; i++) {
      if (!tokens[i].identifier || !["include", "include_str", "include_bytes"].includes(tokens[i].text) || tokens[i + 1].text !== "!") continue;
      const end = pairs.get(i + 2);
      if (end !== undefined) visit(resolve(dirname(file), literal(i + 3, tokens[end - 1]?.text === "," ? end - 1 : end)));
    }
    const scope = (start, end, base, fileScope) => {
      for (let i = start; i < end;) {
        let path;
        while (tokens[i]?.text === "#") {
          const open = tokens[i + 1]?.text === "!" ? i + 2 : i + 1, close = pairs.get(open);
          if (close === undefined) break;
          if (tokens[open + 1]?.text === "cfg_attr" && tokens.slice(open + 2, close).some((token) => token.text === "path")) throw new Error(`Conditional Rust mount requires conservative source inputs: ${file}`);
          if (tokens[open + 1]?.text === "path" && tokens[open + 2]?.text === "=" && tokens[open + 3]?.string) path = tokens[open + 3].text;
          i = close + 1;
        }
        if (tokens[i]?.text === "pub") { i++; if (tokens[i]?.text === "(") i = (pairs.get(i) ?? i) + 1; }
        if (tokens[i]?.text === "mod" && tokens[i + 1]?.identifier) {
          const name = tokens[i + 1].text.replace(/^r#/, ""), boundary = i + 2;
          if (tokens[boundary]?.text === "{") {
            const close = pairs.get(boundary);
            if (close !== undefined) { scope(boundary + 1, close, resolve(base, path ?? name), false); i = close + 1; continue; }
          }
          if (tokens[boundary]?.text === ";") {
            const candidates = path === undefined ? [join(base, `${name}.rs`), join(base, name, "mod.rs")] : [resolve(fileScope ? dirname(file) : base, path)];
            if (!candidates.some(existsSync)) for (const child of candidates) files.add(child);
            for (const child of candidates) if (existsSync(child)) visit(child, path === undefined && child.endsWith(`/${name}.rs`) ? join(dirname(child), name) : dirname(child));
            i = boundary + 1; continue;
          }
        }
        const macro = tokens[i]?.text === "macro_rules";
        while (i < end && tokens[i].text !== ";" && tokens[i].text !== "{") { if (pairs.has(i)) i = pairs.get(i); i++; }
        if (macro && tokens.slice(i, pairs.get(i) ?? i).some((token) => !token.string && token.text === "mod")) throw new Error(`Generated Rust modules require conservative source inputs: ${file}`);
        i = (pairs.get(i) ?? i) + 1;
      }
    };
    scope(0, tokens.length, moduleBase, true);
  };
  for (const entry of entries) visit(resolve(entry));
  return [...files].sort();
}

/** 🧭️ Cargo owns compilation; Nx tracks every statically mounted source and embedded asset. */
function cargoSourceInputs(root, workspaceRoot, facts) {
  const directory = join(workspaceRoot, root), manifest = join(directory, "Cargo.toml");
  if (!existsSync(manifest)) return undefined;
  const cargo = readToml(manifest), entries = [cargo.lib?.path ?? "src/lib.rs", "src/main.rs", ...(cargo.bin ?? []).map((target) => target.path), ...(cargo.test ?? []).map((target) => target.path), ...(cargo.bench ?? []).map((target) => target.path), ...(cargo.example ?? []).map((target) => target.path), ...(cargo.package?.build === false ? [] : [typeof cargo.package?.build === "string" ? cargo.package.build : "build.rs"])].filter(Boolean).map((path) => resolve(directory, path)).filter(existsSync);
  if (cargo.package?.build !== false && existsSync(resolve(directory, typeof cargo.package?.build === "string" ? cargo.package.build : "build.rs"))) return undefined;
  for (const source of ["src/bin", "tests", "examples", "benches"]) {
    if (!existsSync(join(directory, source))) continue;
    for (const entry of readdirSync(join(directory, source), { withFileTypes: true })) {
      const path = join(directory, source, entry.name, ...(entry.isDirectory() ? ["main.rs"] : []));
      if (path.endsWith(".rs") && existsSync(path)) entries.push(path);
    }
  }
  if (!entries.length) return undefined;
  try { return rustSourceFiles(entries, directory, facts).map((file) => `{workspaceRoot}/${nxPath(relative(workspaceRoot, file))}`); }
  catch { return undefined; }
}

/** 🛡️ Side effects and live processes cannot be replayed as completed task results. */
function targetPolicy(name, target, policy = POLICY) {
  if (matchesCommand(name, policy.continuous)) return { ...target, cache: false, continuous: true };
  if (matchesCommand(name, policy.uncached) || name === "test-exhaustive") return { ...target, cache: false };
  if (/^(test(?:-|$)|check(?:-|$)|lint(?:-|$)|typecheck$|format-check$)/.test(name)) return { inputs: ["default", "^default"], outputs: [], cache: true, ...target };
  if (/^(build(?:-|$)|wasm$|native-build$|package$|extension-package$)/.test(name) && !target.outputs) return { ...target, cache: false };
  return target;
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

/** 📥️ Shared command implementation and host identity are inputs of every script-backed task. */
function projectInputs(json, root, workspaceRoot, facts) {
  const tools = ["javascript"];
  if (existsSync(join(workspaceRoot, root, "Cargo.toml")) || json.targets?.wasm) tools.push("cargo");
  if (json.targets?.wasm || json.targets?.["extension-package"] || (tools.includes("cargo") && json.targets?.package)) tools.push("wasm");
  if (goManifest(root, workspaceRoot)) tools.push("go");
  if (existsSync(join(workspaceRoot, root, "pyproject.toml"))) tools.push("python");
  if (readdirSync(join(workspaceRoot, root)).some((file) => /\.[cfv]sproj$/.test(file))) tools.push("dotnet");
  if (existsSync(join(workspaceRoot, root, "CMakeLists.txt"))) tools.push("cmake");
  const runner = nxPath(relative(workspaceRoot, LIBRARY_ROOT));
  const inputs = [
    "{projectRoot}/**/*",
    "{workspaceRoot}/📜️script.ts",
    `{workspaceRoot}/${runner}/**/*.{ts,tsx,js,mjs,cjs,json}`,
    `{workspaceRoot}/${runner}/🟨️.mjs`,
    `{workspaceRoot}/${runner}/⚡️caching/**/*`,
    { runtime: 'node -p "process.platform.concat(process.arch)"' },
  ];
  const owner = root.includes("/📦️packages/") ? root.split("/📦️packages/")[0] : root;
  if (owner !== root) {
    const extensions = tools.includes("cargo") ? "{rs,toml,json,semio,wit,wgsl,glsl,h,c,cpp}" : tools.includes("go") ? "{go,mod,sum,json,ts}" : tools.includes("dotnet") ? "{cs,fs,vb,csproj,fsproj,vbproj,props,targets,resx,json}" : tools.includes("python") ? "{py,pyi,toml,json}" : "{ts,tsx,js,jsx,mjs,cjs,json,css,scss,html,svg,wit}";
    const native = tools.includes("cargo") ? cargoSourceInputs(root, workspaceRoot, facts) : undefined;
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
  inputs.push(`!{workspaceRoot}/${runner}/**/🧪️tests/**/*`, `!{workspaceRoot}/${runner}/**/*.test.ts`);
  const production = ["default", "!{projectRoot}/**/*.{spec,test}.{ts,tsx,js,jsx,mjs,cjs}", "!{projectRoot}/**/🧪️tests/**/*", "!{projectRoot}/**/🧫️fixtures/**/*", "!{projectRoot}/**/*.feature", "!{projectRoot}/**/*.stories.{ts,tsx}"];
  if (owner !== root) production.push(`!{workspaceRoot}/${owner}/**/🧪️tests/**/*`, `!{workspaceRoot}/${owner}/**/🧫️fixtures/**/*`);
  const declarations = json.namedInputs ?? {};
  const exclusions = [];
  for (const target of Object.values(json.targets ?? {})) for (const output of target.outputs ?? []) {
    if (!output.startsWith("{projectRoot}/") && !output.startsWith("{workspaceRoot}/")) continue;
    const absolute = resolve(workspaceRoot, output.replace("{projectRoot}", root).replace("{workspaceRoot}", "."));
    const path = nxPath(relative(workspaceRoot, absolute));
    if (path === "" || path.startsWith("../")) throw new Error(`Invalid output ownership for ${json.name}: ${output}`);
    exclusions.push(`!{workspaceRoot}/${path}`, `!{workspaceRoot}/${path}/**/*`);
    if (owned(path, root)) {
      const local = nxPath(relative(join(workspaceRoot, root), absolute));
      exclusions.push(`!{projectRoot}/${local}`, `!{projectRoot}/${local}/**/*`);
    }
  }
  return { ...declarations, default: [...inputs, ...(declarations.default ?? []), ...exclusions], production: [...production, ...(declarations.production ?? [])] };
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
function projectWithDefaults(json, root, projectDir, workspaceRoot, contracts = {}, facts) {
  const ownsScript = existsSync(join(projectDir, SCRIPT_BASENAME));
  const declared = { ...(root === "." && ownsScript ? rootCommandTargets(join(projectDir, SCRIPT_BASENAME)) : {}), ...json.targets };
  for (const contract of Object.values(contracts)) {
    if (contract.ownership !== "owned" || contract.ownerPath !== root) continue;
    const name = contract.target.slice(contract.target.lastIndexOf(":") + 1), target = declared[name];
    if (!target || contract.target !== `${json.name}:${name}`) throw new Error(`Generator target has no project owner: ${contract.target}`);
    const inputs = [...(target.inputs ?? ["default", "^default"]), ...contract.inputPatterns.map((path) => `{workspaceRoot}/${path}`)];
    if (contract.inputDiscovery) inputs.push({ runtime: `bun ${JSON.stringify(nxPath(relative(workspaceRoot, join(LIBRARY_ROOT, "⚡️caching/📜️script.ts"))))} generator-inputs ${contract.inputDiscovery.kind}` });
    declared[name] = { ...target, inputs, outputs: contract.outputRoots.map((output) => `{workspaceRoot}/${output.path}`) };
    if (contract.checkTarget) {
      const check = contract.checkTarget.slice(contract.checkTarget.lastIndexOf(":") + 1);
      if (declared[check]) declared[check] = { ...declared[check], cache: false };
    }
  }
  const normalized = {};
  for (const [name, target] of Object.entries(withLeveledTestTargets(declared))) {
    const policy = targetPolicy(name, targetWithDefaults({ ...(POLICY.targetDefaults?.[name] ?? {}), ...target }, root, ownsScript));
    if (existsSync(join(projectDir, "Cargo.toml")) && /^(build|wasm|native|test(?:-(?:quick|long|exhaustive))?$|lint|check$)/.test(name)) policy.parallelism ??= false;
    normalized[name] = root === "." || json.name === "@semio-tech/repo-test-domain" ? { ...policy, cache: policy.cache === false ? false : target.cache ?? false } : policy;
  }
  return { ...json, name: json.name, root, namedInputs: projectInputs({ ...json, targets: declared }, root, workspaceRoot, facts), targets: normalized };
}

/** 🚪️ Exposes registered workspace commands without inferring forwarding package scripts. */
function rootCommandTargets(script) {
  const ts = createRequire(import.meta.url)("typescript");
  const source = ts.createSourceFile(script, readFileSync(script, "utf8"), ts.ScriptTarget.Latest, true);
  const targets = {};
  const inspect = (node) => {
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) && node.expression.name.text === "register" && ts.isStringLiteral(node.arguments[0])) {
      const name = node.arguments[0].text;
      if (name !== "nx") targets[name] = { executor: DEFAULT_EXECUTOR, cache: false, options: { command: `bun ./📜️script.ts ${name}`, forwardAllArgs: true } };
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
  const rootsByName = new Map(), facts = new Map();
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
      return [configFile, { projects: { [name]: projectWithDefaults(json, root, projectDir, workspaceRoot, contracts, facts) } }];
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
    const script = nxPath(relative(workspaceRoot, join(LIBRARY_ROOT, "⚡️caching/📜️script.ts")));
    const targets = Object.fromEntries(["build", "check", "test"].map((target) => [target, {
      executor: DEFAULT_EXECUTOR,
      cache: true,
      parallelism: false,
      outputs: target === "build" ? ["{projectRoot}/dist/build"] : [],
      inputs: [target === "build" ? "production" : "default", target === "build" ? "^production" : "^default"],
      options: { cwd: ".", command: `bun ${JSON.stringify(script)} native cargo ${target} --manifest ${JSON.stringify(configFile)}` },
    }]));
    const project = { name, root, projectType: "library", tags: ["language:cargo", "discovery:manifest"], targets };
    results.push([configFile, { projects: { [name]: { ...project, namedInputs: projectInputs(project, root, workspaceRoot, facts) } } }]);
  }
  return results;
}

/** 🕸️ Native manifests and package imports contribute edges without spawning a build or installer. */
export function createDependencies(_options, context) {
  const { workspaceRoot, projects } = context;
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
    if (manifest) for (const dependency of Object.keys({ ...manifest.dependencies, ...manifest.devDependencies, ...manifest.peerDependencies, ...manifest.optionalDependencies })) add(name, byPackage.get(dependency), nxPath(relative(workspaceRoot, join(root, "package.json"))));
    for (const file of name === "workspace" ? [] : context.fileMap?.projectFileMap?.[name] ?? []) {
      if (!/\.[cm]?[jt]sx?$/.test(file.file) || file.file.endsWith(SCRIPT_BASENAME)) continue;
      const text = readFileSync(join(workspaceRoot, file.file), "utf8");
      for (const match of text.matchAll(/(?:\bfrom\s*|\bimport\s*(?:\(\s*)?|\brequire\s*\(\s*)["']([^"']+)["']/g)) {
        const specifier = match[1];
        const packageName = specifier.startsWith("@") ? specifier.split("/").slice(0, 2).join("/") : specifier.split("/")[0];
        if (!specifier.startsWith(".")) add(name, byPackage.get(packageName), file.file);
        else {
          const path = nxPath(relative(workspaceRoot, resolve(dirname(join(workspaceRoot, file.file)), specifier)));
          const target = Object.entries(projects).filter(([candidate, p]) => candidate !== "workspace" && owned(path, p.root)).sort((a, b) => b[1].root.length - a[1].root.length)[0]?.[0];
          add(name, target, file.file);
        }
      }
    }
  }
  return [...edges.values()];
}

export default {
  name: "@repo/emoji-project-json",
  createNodesV2: [`**/{${PROJECT_BASENAME},Cargo.toml}`, emojiProjectJsonNodes],
  createDependencies,
};

export const cacheInternals = { targetPolicy, nativeDependencies, goDependencies, rustSourceFiles, projectInputs, rootCommandTargets };
