/** 🧶️ Models Bun's locked package locations and authored patch bytes without inspecting installed packages. */
import { createHash } from "node:crypto";
import { createRequire } from "node:module";
import { isAbsolute, join, relative, resolve } from "node:path";
import { observedFs } from "../../../../🏗️build/👁️observation/🟨️.mjs";
const { readFileSync, realpathSync, statSync } = observedFs;

/** 🏷️ Names this component as its roster entry does. */
export const component = "bun-lock";
let buildPath;

/** 🔌️ Binds this component's dependencies from the revision registry once its needs are wired. */
export function wire(registry) {
  ({ buildPath } = registry.coordinates);
}

/** 🧶️ Models Bun's locked package locations without merging distinct dependency contexts. */
export function bunLockGraph(lock, patches = {}) {
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

export let BUN_LOCK_CACHE;

/** 🔐️ Reads locked identities and authored patch bytes without inspecting installed packages. */
export function readBunLockGraph(workspaceRoot) {
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


/** 🔐️ Names the digest of the last lock graph this revision read, which dependency resolution authority consumes after a read. */
export function bunLockCacheHash() { return BUN_LOCK_CACHE.hash; }
