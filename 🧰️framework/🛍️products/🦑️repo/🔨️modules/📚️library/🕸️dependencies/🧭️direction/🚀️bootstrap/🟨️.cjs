const fs = require("node:fs");
const { builtinModules } = require("node:module");
const { createHash } = require("node:crypto");
const { dirname, isAbsolute, join, relative, resolve } = require("node:path");
const { readBunRepositoryMembership } = require("../../../🗂️workspaces/🟦️bun/🟨️.cjs");
const { buildDependencyDirectionPolicy } = require("../🏗️construction/🟨️.cjs");
const slash = path => path.replaceAll("\\", "/");
const digest = text => createHash("sha256").update(text).digest("hex");
function freeze(value) {
  if (value && typeof value === "object" && !Object.isFrozen(value)) { Object.values(value).forEach(freeze); Object.freeze(value); }
  return value;
}
function rootPath(input) {
  if (typeof input !== "string" || !input || input.includes("\0") || /^[A-Za-z]:(?:$|[^\\/])/u.test(input) || input.split(/[\\/]/u).some(segment => segment === "." || segment === "..")) throw Error(`Dependency policy requires an unnormalized safe root: ${JSON.stringify(input)}`);
  const root = resolve(input), ancestry = [];
  for (let current = root; ; current = dirname(current)) { ancestry.push(current); if (current === dirname(current)) break; }
  for (const current of ancestry.reverse()) if (!fs.lstatSync(current).isDirectory() || fs.lstatSync(current).isSymbolicLink()) throw Error(`Dependency policy root ancestry must be a real directory: ${current}`);
  return root;
}
function locator(value) {
  const path = value?.metadata?.semio?.taxonomy;
  if (typeof path !== "string" || !path || /[\\\0]/u.test(path) || path.startsWith("/") || /^[A-Za-z]:/u.test(path) || path.split("/").some(segment => !segment || segment === "." || segment === "..")) throw Error("Dependency policy requires canonical metadata.semio.taxonomy");
  return path;
}

/** 🛡️ Captures fresh no-follow authority and builds policy without module-cached input.
 * @param {string} input
 * @param {import("./🟦️.ts").DependencyPolicyCaptureOptions} [options]
 * @returns {import("./🟦️.ts").DependencyPolicySnapshot}
 */
function loadDependencyDirectionPolicy(input, options = {}) {
  const check = () => { if (options.signal?.aborted) throw Error("Dependency policy authority capture cancelled"); };
  check();
  const root = rootPath(input), sources = new Map(), texts = new Map(), verifiedDirectories = new Set([root]);
  const record = (path, receipt) => { sources.set(path, { path, ...receipt }); if (sources.size % 300 === 0) options.onProgress?.({ phase: "capture", sources: sources.size }); check(); };
  const local = path => {
    const value = slash(relative(root, path));
    if (value === ".." || value.startsWith("../") || isAbsolute(value)) throw Error(`Dependency policy authority escapes root: ${path}`);
    return value;
  };
  const state = (path, freshAncestry = false) => {
    check();
    const parts = local(path).split("/").filter(Boolean); let current = root;
    for (const [index, part] of parts.entries()) {
      current = join(current, part);
      if (!freshAncestry && verifiedDirectories.has(current)) { if (index === parts.length - 1) return "directory"; continue; }
      let entry;
      try { entry = fs.lstatSync(current); } catch (error) { if (error.code === "ENOENT") { record(local(current), { kind: "missing" }); return "missing"; } throw error; }
      if (entry.isSymbolicLink()) throw Error(`Dependency policy authority must not follow a link: ${local(current)}`);
      if (index < parts.length - 1 && !entry.isDirectory()) throw Error(`Dependency policy authority ancestor must be a directory: ${local(current)}`);
      if (entry.isDirectory()) verifiedDirectories.add(current);
      if (index === parts.length - 1) return entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other";
    }
    return "directory";
  };
  const readText = path => {
    const name = local(path);
    if (texts.has(name)) return texts.get(name);
    if (state(path, true) !== "file") throw Error(`Dependency policy requires a regular authority file: ${name}`);
    const before = fs.lstatSync(path), descriptor = fs.openSync(path, fs.constants.O_RDONLY | (fs.constants.O_NOFOLLOW ?? 0));
    try {
      const opened = fs.fstatSync(descriptor);
      if (!opened.isFile() || opened.dev !== before.dev || opened.ino !== before.ino) throw Error(`Dependency policy authority changed while opening: ${name}`);
      const bytes = fs.readFileSync(descriptor), text = bytes.toString("utf8");
      if (state(path, true) !== "file") throw Error(`Dependency policy authority changed while reading: ${name}`);
      const after = fs.lstatSync(path);
      if (after.dev !== opened.dev || after.ino !== opened.ino || !Buffer.from(text).equals(bytes)) throw Error(`Dependency policy authority is not stable UTF-8: ${name}`);
      texts.set(name, text); record(name, { kind: "file", sha256: digest(bytes), bytes: bytes.length }); return text;
    } finally { fs.closeSync(descriptor); }
  };
  const list = path => {
    const name = local(path);
    if (state(path) !== "directory") throw Error(`Dependency policy requires a real authority directory: ${name}`);
    const entries = fs.readdirSync(path, { withFileTypes: true }).map(entry => ({ name: entry.name, kind: entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other" })).sort((a, b) => Buffer.compare(Buffer.from(a.name), Buffer.from(b.name)));
    record(name || ".", { kind: "directory", entries, sha256: digest(JSON.stringify(entries)) }); return entries;
  };
  const document = path => JSON.parse(readText(join(root, path)));
  document("nx.json");
  const project = document("📋️project.json"), taxonomyPath = locator(project), taxonomy = document(taxonomyPath);
  if (!taxonomy || typeof taxonomy !== "object" || Array.isArray(taxonomy) || !taxonomy.areaLayers || !taxonomy.dependencyDirections || !taxonomy.pathExclusions || !taxonomy.implementationLeafPolicy || !Array.isArray(taxonomy.forbiddenPathSegments)) throw Error("Dependency policy requires readable taxonomy authorities");
  const membership = readBunRepositoryMembership(root, { state, readText, list });
  for (const scope of membership.scopes) for (const path of scope.declaration.members) {
    if (path.startsWith("!") || /[*?]/u.test(path)) continue;
    const owner = join(root, scope.directory, path);
    if (state(owner) === "directory") readText(join(owner, "package.json"));
  }
  const excluded = Object.values(taxonomy.pathExclusions).map(row => row.path.replace(/\/$/u, ""));
  const workspacePackages = membership.packages.filter(owner => !excluded.some(prefix => owner === prefix || owner.startsWith(prefix + "/"))).map(owner => {
    const row = document(owner + "/package.json");
    if (typeof row.name !== "string" || !row.name) throw Error(`Workspace owner requires an authored package name: ${owner}`);
    const exports = row.exports && typeof row.exports === "object" && !Array.isArray(row.exports) && Object.keys(row.exports).some(key => key.startsWith(".")) ? Object.keys(row.exports).filter(key => row.exports[key] !== null) : ["."];
    return { owner, name: row.name, exports, ...(row.semio?.dependencyRole ? { dependencyRole: row.semio.dependencyRole } : {}) };
  });
  const pluginRoot = join(root, "✏️s/🔌️plugins"), pluginState = state(pluginRoot);
  if (pluginState !== "directory" && pluginState !== "missing") throw Error("Dependency policy plugin owner must be a real directory");
  const plugins = pluginState === "missing" ? [] : list(pluginRoot).filter(entry => { if (entry.kind === "symlink") throw Error(`Dependency policy plugin must not be linked: ${entry.name}`); return entry.kind === "directory"; }).map(entry => entry.name).sort((a, b) => Buffer.compare(Buffer.from(a), Buffer.from(b)));
  const policy = buildDependencyDirectionPolicy({ taxonomy, plugins, workspacePackages, nodeBuiltins: builtinModules });
  options.onProgress?.({ phase: "complete", sources: sources.size }); check();
  return freeze({ workspaceRoot: root, taxonomyPath, taxonomy, workspacePackages, plugins, policy, sources: [...sources.values()].sort((a, b) => Buffer.compare(Buffer.from(a.path), Buffer.from(b.path))) });
}
module.exports = { loadDependencyDirectionPolicy };
