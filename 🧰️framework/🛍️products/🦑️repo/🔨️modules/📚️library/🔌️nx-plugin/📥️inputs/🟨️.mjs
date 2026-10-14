import { createRequire } from "node:module";
import { createHash } from "node:crypto";
import { mkdirSync, writeFileSync, existsSync as persistedExists } from "node:fs";
import { fileURLToPath } from "node:url";
import { dirname, isAbsolute, join, relative, resolve } from "node:path";
import { observedFs } from "../👁️observation/🟨️.mjs";

const { existsSync, readdirSync } = observedFs;

/** 🧮️ Interns equivalent project/workspace coordinates before Nx compiles native glob sets. */
export function deduplicateProjectInputs(inputs, root) {
  const keys = new Set(), projectRoot = root === "." ? "{workspaceRoot}" : `{workspaceRoot}/${root}`;
  return inputs.filter(input => {
    const key = typeof input === "string" ? input.replaceAll("{projectRoot}", projectRoot) : JSON.stringify(input);
    if (keys.has(key)) return false;
    keys.add(key);
    return true;
  });
}

/** 🔒️ A lock projection is batched by the same native hasher without compiler subprocesses. */
export function cargoLockInput(manifest, workspaceRoot) {
  const contract = JSON.stringify({ version: 1, manifest }), key = createHash("sha256").update(contract).digest("hex"), directory = join(workspaceRoot, ".🧬semio/🦑️repo/⚡️cache/nx-lock-closures"), path = join(directory, `${key}.json`);
  if (!persistedExists(path)) { mkdirSync(directory, { recursive: true }); try { writeFileSync(path, contract, { flag: "wx" }); } catch (error) { if (error.code !== "EEXIST") throw error; } }
  return { env: `SEMIO_CARGO_LOCK_${key}` };
}

/** 🧮️ Shares exact inventories while Nx hashes their current byte digest once per unique closure. */
export function compactSourceInputs(inputs, workspaceRoot, root = ".") {
  const candidates = [...new Set(inputs.filter(input => typeof input === "string" && input.startsWith("{workspaceRoot}/") && !/[!*?{}[\]]/u.test(input.slice("{workspaceRoot}/".length))))].sort();
  const selected = new Set(candidates), remaining = inputs.filter(input => !selected.has(input));
  const residual = remaining.some(input => typeof input === "string" && !input.startsWith("!")) ? remaining : remaining.filter(input => typeof input !== "string" || !input.startsWith("!"));
  let exact = candidates;
  if (exact.length < 1) return inputs;
  const projectRoot = root === "." ? "." : root, normalize = input => input.replaceAll("{projectRoot}", projectRoot).replaceAll("{workspaceRoot}/", ""), exclusions = inputs.filter(input => typeof input === "string" && input.startsWith("!") && !input.startsWith("!^"));
  if (exclusions.length) {
    const { matchGlobPaths } = createRequire(import.meta.url)("nx/src/native");
    const paths = exact.map(normalize), excluded = paths.map(() => false);
    for (const input of exclusions) for (const [index, matched] of matchGlobPaths([normalize(input.slice(1))], paths).entries()) excluded[index] ||= matched;
    exact = exact.filter((input, index) => !excluded[index]);
  }
  if (exact.length < 1) return residual;
  const helpers = ["./🧮️digest/🟨️.mjs", "./🧮️digest/🔣️schema.json", "./🔒️lock/📥️read/🟨️.mjs", "./🔒️lock/🟨️.mjs"].map(path => relative(workspaceRoot, fileURLToPath(new URL(path, import.meta.url))).split("\\").join("/"));
  const files = [...new Set([...exact.map(input => input.slice("{workspaceRoot}/".length)), ...helpers])].sort(), contract = JSON.stringify({ version: 1, files });
  const key = createHash("sha256").update(contract).digest("hex"), directory = join(workspaceRoot, ".🧬semio/🦑️repo/⚡️cache/nx-input-closures"), manifest = join(directory, `${key}.json`);
  if (!persistedExists(manifest)) { mkdirSync(directory, { recursive: true }); try { writeFileSync(manifest, contract, { flag: "wx" }); } catch (error) { if (error.code !== "EEXIST") throw error; } }
  const script = fileURLToPath(new URL("./🧮️digest/🟨️.mjs", import.meta.url)), schema = fileURLToPath(new URL("./🧮️digest/🔣️schema.json", import.meta.url));
  return [...residual, `{workspaceRoot}/${relative(workspaceRoot, script).split("\\").join("/")}`, `{workspaceRoot}/${relative(workspaceRoot, schema).split("\\").join("/")}`, { env: `SEMIO_SOURCE_CLOSURE_${key}` }];
}

/** 🧬️ Keeps output negations only where a positive source scope can observe those outputs. */
export function sourceOutputOverlaps(output, inputs, root) {
  return inputs.some(input => {
    if (typeof input !== "string" || input.startsWith("!") || !/^\{(?:projectRoot|workspaceRoot)\}\//u.test(input)) return false;
    const source = input.replaceAll("{projectRoot}/", root === "." ? "" : `${root}/`).replaceAll("{workspaceRoot}/", ""), wildcard = source.search(/[!*?{}[\]]/u), prefix = wildcard < 0 ? source : source.slice(0, wildcard).replace(/[^/]*$/u, "").replace(/\/$/u, "");
    return !prefix || output === prefix || output.startsWith(prefix + "/") || prefix.startsWith(output + "/");
  });
}

/** 🗂️ Shares repeated arrays inside one project while retaining native Nx batching and exact membership. */
export function internProjectInputGroups(groups) {
  const names = Object.keys(groups), entries = new Map();
  for (const name of names) for (const input of groups[name]) {
    const key = JSON.stringify(input), entry = entries.get(key) ?? { input, names: [] };
    if (!entry.names.includes(name)) entry.names.push(name);
    entries.set(key, entry);
  }
  const shared = new Map();
  for (const [key, entry] of entries) if (entry.names.length > 1) {
    const membership = JSON.stringify(entry.names), group = shared.get(membership) ?? { names: entry.names, keys: new Set(), inputs: [] };
    group.keys.add(key); group.inputs.push(entry.input); shared.set(membership, group);
  }
  const result = { ...groups };
  for (const group of shared.values()) {
    if (group.inputs.length < 4) continue;
    const name = `sharedSources${createHash("sha256").update(JSON.stringify(group.inputs)).digest("hex").slice(0, 16)}`;
    if (Object.hasOwn(result, name)) throw Error("Shared source group identity collision");
    result[name] = group.inputs;
    for (const owner of group.names) result[owner] = [...result[owner].filter(input => !group.keys.has(JSON.stringify(input))), name];
  }
  return result;
}

/** 📥️ Projects source and compiler input ownership independently of target orchestration. */
export function inferProjectInputs(json, root, workspaceRoot, facts, scripts, context) {
  const { POLICY, TAXONOMY, LIBRARY_ROOT, SCRIPT_BASENAME, cargoSourceInputs, goManifest, nxPath, relativeScriptInputs, resolveOutputPath, exampleCollectionInputs, workspaceNegationClosed, declaredSourceInputs, owned } = context;
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
    const extensions = tools.includes("cargo") ? "{rs,toml,json,semio,wit,wgsl,glsl,h,c,cpp,ts,tsx,js,jsx,mjs,cjs,css,scss,html,svg,sql}" : tools.includes("go") ? "{go,mod,sum,json,ts}" : tools.includes("dotnet") ? "{cs,fs,vb,csproj,fsproj,vbproj,props,targets,resx,json}" : tools.includes("python") ? "{py,pyi,toml,json}" : "{ts,tsx,js,jsx,mjs,cjs,json,css,scss,html,svg,wit,sql}";
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
  const declarations = json.namedInputs ?? {}, sourceContracts = declaredSourceInputs(json, workspaceRoot);
  const sourceScopes = [...inputs, ...Object.values(declarations).flat(), ...Object.values(sourceContracts).flat(), ...(nativeSources ?? []), ...(nativeTests ?? [])];
  const exclusions = [];
  for (const target of Object.values(json.targets ?? {})) for (const output of target.outputs ?? []) {
    const absolute = resolveOutputPath(output, root, workspaceRoot);
    if (!absolute) continue;
    const path = nxPath(relative(workspaceRoot, absolute));
    if (path === "" || path.startsWith("../")) throw new Error(`Invalid output ownership for ${json.name}: ${output}`);
    if (!sourceOutputOverlaps(path, sourceScopes, root)) continue;
    if ((owned(path, root) || owned(path, owner)) && path.split("/").some(part => POLICY.generatedDirectories.includes(part))) continue;
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
  const groups = { ...declarations, ...sourceContracts, default: [...inputs, ...(declarations.default ?? []), ...exclusions], production: [...inputs, ...(declarations.default ?? []), ...exclusions, ...production, ...(declarations.production ?? [])], nativeSources: workspaceNegationClosed([...native(nativeSources, true), ...(declarations.nativeSources ?? [])]), nativeTestSources: workspaceNegationClosed([...native(nativeTests), ...(declarations.nativeSources ?? []), ...(declarations.nativeTestSources ?? [])]), ...(artifactTypeScript ? { artifactSources: [...artifactSources, ...(declarations.artifactSources ?? [])], artifactCommandSources: [...artifactCommandSources, ...(declarations.artifactCommandSources ?? [])] } : {}) };
  return internProjectInputGroups(Object.fromEntries(Object.entries(groups).map(([name, inputs]) => [name, compactSourceInputs(deduplicateProjectInputs(inputs, root), workspaceRoot, root)])));
}

