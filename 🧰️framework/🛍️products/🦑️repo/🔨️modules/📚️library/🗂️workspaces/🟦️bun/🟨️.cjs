const fs = require("node:fs");
const { dirname, isAbsolute, join, relative, resolve } = require("node:path");
const { ownsPayload } = require("../📦️payload/🟨️.cjs");
const slash = (path) => path.replaceAll("\\", "/");
const opaque = new Set(["🗑️generated", "node_modules", "target", "dist", "build", "storybook-static", "temp", "coverage", "🔌️plugin-modules", "compose", "🧫️fixtures"]);

/** 🧬️ Admits authored native patterns independently of package identity or product roles.
 * @returns {import("./🟦️.ts").BunWorkspaceDeclarationV1}
 */
function parseBunWorkspaceDeclaration(value) {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw Error("Bun workspace declaration requires an object");
  if (Object.keys(value).some(key => !["schemaVersion", "members", "owners"].includes(key)) || value.schemaVersion !== 1 || !Array.isArray(value.members) || !value.members.length || !Array.isArray(value.owners)) throw Error("Invalid Bun workspace declaration");
  for (const [key, values] of [["members", value.members], ["owners", value.owners]]) if (values.some(path => typeof path !== "string" || !path || path === "!" || /^!?\//u.test(path) || /^!?[A-Za-z]:/u.test(path) || path.includes("\\") || path.includes("\0") || key === "owners" && path !== "*/package.json") || new Set(values).size !== values.length) throw Error("Invalid Bun workspace patterns");
  if (!value.members.some(path => !path.startsWith("!"))) throw Error("Bun workspace needs a positive source pattern");
  return { schemaVersion: 1, members: [...value.members], owners: [...value.owners] };
}
const match = (pattern, path) => {
  const parts = pattern.split("/"), source = parts.map((part, index) => part === "**" ? index === parts.length - 1 ? ".*" : "(?:[^/]+/)*" : part.split("").map(c => c === "*" ? "[^/]*" : c === "?" ? "[^/]" : c.replace(/[.*+?^${}()|[\]\\]/g, "\\$&")).join("") + (index === parts.length - 1 ? "" : "/")).join("");
  return new RegExp("^" + source + "$").test(path);
};
const bounded = (root, path) => {
  const full = resolve(root, path), local = relative(root, full);
  if (local === ".." || local.startsWith("../") || local.startsWith("..\\") || isAbsolute(local)) throw Error(`Bun workspace escapes repository: ${path}`);
  return full;
};

/** 📁️ Expands current physical candidates without cached package membership. */
function workspaceCandidates(root, scope, operations, inventory) {
  const positive = scope.declaration.members.filter(path => !path.startsWith("!")), negative = scope.declaration.members.filter(path => path.startsWith("!")).map(path => path.slice(1)), found = new Set();
  for (const pattern of positive) {
    const prefix = pattern.split(/[*?]/u)[0].replace(/\/$/u, ""), start = bounded(root, join(scope.directory, prefix));
    const state = operations.state(start);
    if (state === "missing") continue;
    if (state !== "directory") throw Error(`Bun source owner must be a real directory: ${start}`);
    const walk = (directory) => {
      const known = inventory.get(directory);
      if (known) return known;
      const entries = operations.list(directory), paths = [];
      if (entries.some(entry => entry.name === "package.json")) paths.push(directory);
      for (const entry of entries) if (!entry.name.startsWith(".") && !opaque.has(entry.name)) {
        if (entry.kind === "symlink") throw Error(`Bun source directory is a symlink: ${join(directory, entry.name)}`);
        if (entry.kind === "directory") paths.push(...walk(join(directory, entry.name)));
      }
      inventory.set(directory, paths);
      return paths;
    };
    for (const directory of walk(start)) {
      const local = slash(relative(resolve(root, scope.directory), directory));
      if (match(pattern, local) && !negative.some(path => match(path, local + "/") || match(path, local))) found.add(slash(relative(root, directory)));
    }
  }
  return [...found].sort((a, b) => a.localeCompare(b));
}
function workspacePackages(root, scope, operations, inventory, documents) {
  const document = (path) => {
    if (!documents.has(path)) documents.set(path, JSON.parse(operations.readText(bounded(root, path))));
    return documents.get(path);
  };
  const paths = workspaceCandidates(root, scope, operations, inventory), candidates = paths.map(relDir => ({ relDir, absDir: resolve(root, relDir), ...document(relDir + "/package.json") })), byDirectory = new Map(candidates.map(candidate => [candidate.absDir, candidate]));
  return candidates.filter(candidate => {
    for (let parent = dirname(candidate.absDir); parent !== root && parent !== dirname(parent); parent = dirname(parent)) {
      const owner = byDirectory.get(parent);
      if (owner) return !ownsPayload(owner, candidate, operations);
    }
    return true;
  }).map(candidate => candidate.relDir);
}

/** 🗂️ Resolves source membership through an explicit physical operations interface.
 * @returns {import("./🟦️.ts").BunRepositoryMembership}
 */
function readBunRepositoryMembership(root, operations) {
  const document = JSON.parse(operations.readText(join(root, "package.json"))), declaration = parseBunWorkspaceDeclaration(document.semio?.workspace), paths = new Set(["package.json"]);
  for (const pattern of declaration.owners) {
    if (pattern !== "*/package.json") throw Error(`Unsupported physical owner recipe: ${pattern}`);
    for (const entry of operations.list(root)) if (!entry.name.startsWith(".") && !opaque.has(entry.name)) {
      if (entry.kind === "symlink") throw Error(`Bun installation owner is a symlink: ${entry.name}`);
      if (entry.kind === "directory" && operations.state(join(root, entry.name, "package.json")) !== "missing") paths.add(entry.name + "/package.json");
    }
  }
  const scopes = [], inventory = new Map(), documents = new Map(), all = new Set();
  for (const path of [...paths].sort()) {
    const row = JSON.parse(operations.readText(join(root, path)));
    if (row.semio?.workspace === undefined) continue;
    const d = dirname(path), directory = d === "." ? "." : slash(d), admitted = parseBunWorkspaceDeclaration(row.semio.workspace);
    if (!Array.isArray(row.workspaces) || JSON.stringify(row.workspaces.slice(0, admitted.members.length)) !== JSON.stringify(admitted.members) || row.workspaces.slice(admitted.members.length).some(path => typeof path !== "string" || !path.startsWith("!"))) throw Error(`Bun native source patterns drift: ${path}`);
    for (const pattern of admitted.members) bounded(root, join(directory, pattern.replace(/^!/u, "").split(/[*?]/u)[0]));
    const scope = { directory, manifest: path, lock: directory === "." ? "bun.lock" : directory + "/bun.lock", declaration: admitted };
    const packages = workspacePackages(root, scope, operations, inventory, documents);
    for (const exclusion of row.workspaces.slice(admitted.members.length)) if (packages.some(path => match(exclusion.slice(1), slash(relative(resolve(root, directory), resolve(root, path)))))) throw Error(`Bun native exclusion hides an independent owner: ${path} ${exclusion}`);
    packages.forEach(path => all.add(path)); scopes.push(scope);
  }
  return { scopes, packages: [...all].sort((a, b) => a.localeCompare(b)) };
}
const nativeOperations = {
  state(path) {
    try {
      const entry = fs.lstatSync(path);
      return entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other";
    } catch (error) { if (error.code === "ENOENT") return "missing"; throw error; }
  },
  list: path => fs.readdirSync(path, { withFileTypes: true }).map(entry => ({ name: entry.name, kind: entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other" })),
  readText(path) {
    const ancestry = [];
    for (let current = dirname(path); ; current = dirname(current)) { ancestry.push(current); if (current === dirname(current)) break; }
    for (const current of ancestry.reverse()) if (this.state(current) !== "directory") throw Error(`Bun workspace ancestor must be a real directory: ${current}`);
    if (this.state(path) !== "file") throw Error(`Bun workspace requires a regular manifest: ${path}`);
    return fs.readFileSync(path, "utf8");
  },
};
const bunRepositoryMembership = root => readBunRepositoryMembership(root, nativeOperations);
const bunWorkspacePackages = (root, scope) => workspacePackages(root, scope, nativeOperations, new Map(), new Map());
const bunWorkspaceNativePatterns = (root, scope) => {
  const inventory = new Map(), selected = new Set(workspacePackages(root, scope, nativeOperations, inventory, new Map()));
  return [...scope.declaration.members, ...workspaceCandidates(root, scope, nativeOperations, inventory).filter(path => !selected.has(path)).map(path => "!" + slash(relative(resolve(root, scope.directory), resolve(root, path))).replace(/[?*\[\]{}]/g, "\\$&"))];
};
module.exports = { parseBunWorkspaceDeclaration, readBunRepositoryMembership, bunRepositoryMembership, bunWorkspacePackages, bunWorkspaceNativePatterns, discoverBunWorkspaces: root => bunRepositoryMembership(root).scopes, bunRepositoryPackages: root => bunRepositoryMembership(root).packages };
