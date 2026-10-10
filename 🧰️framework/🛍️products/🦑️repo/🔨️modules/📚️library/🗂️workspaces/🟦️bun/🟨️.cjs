const fs = require("node:fs");
const { dirname, isAbsolute, join, relative, resolve } = require("node:path");
const { ownsPayloadPlan } = require("../📦️payload/🟨️.cjs");
const { createHash } = require("node:crypto");
const slash = (path) => path.replaceAll("\\", "/");
const ownershipPath = join(__dirname, "../🔣️policy.json"), ownershipSource = fs.readFileSync(ownershipPath, "utf8");
const {workspaceTestingCollectionRoot: collectionRoot} = require("../🟨️.cjs");
const opaque = new Set(["🗑️generated", "node_modules", "target", "dist", "build", "storybook-static", "temp", "coverage", "🔌️plugin-modules", "compose"]);

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
const patternRegex = pattern => {
  const literal = c => c.replace(/[.*+?^${}()|[\]\\]/g, "\\$&");
  const partSource = part => {
    let source = "";
    for (let index = 0; index < part.length; index++) {
      const c = part[index];
      source += c === "*" ? "[^/]*" : c === "?" ? "[^/]" : literal(c);
    }
    return source;
  };
  const parts = pattern.split("/").filter((part, index, all) => part !== "**" || all[index - 1] !== "**"), source = parts.map((part, index) => part === "**" ? index === parts.length - 1 ? index === 0 ? ".*" : "(?:/.*)?" : "(?:[^/]+/)*" : partSource(part) + (index === parts.length - 1 || index === parts.length - 2 && parts[index + 1] === "**" ? "" : "/")).join("");
  return new RegExp("^" + source + "$");
};
const sourcePrefix = pattern => {
  const parts = pattern.split("/"), index = parts.findIndex(part => /[*?]/u.test(part));
  return (index < 0 ? parts : parts.slice(0, index)).join("/").replace(/\/$/u, "") || ".";
};
const bounded = (root, path) => {
  const full = resolve(root, path), local = relative(root, full);
  if (local === ".." || local.startsWith("../") || local.startsWith("..\\") || isAbsolute(local)) throw Error(`Bun workspace escapes repository: ${path}`);
  return full;
};

const context = root => ({ root, inventory: new Map(), documents: new Map(), texts: new Map(), selected: new Map(), observations: new Map() });
function* documentPlan(owner, path) {
  if (!owner.documents.has(path)) {
    const text = yield { operation: "readText", path: bounded(owner.root, path) };
    owner.texts.set(path, text); owner.documents.set(path, JSON.parse(text));
  }
  return owner.documents.get(path);
}
function* walkPlan(owner, directory) {
  const collection = collectionRoot(owner.root, directory);
  if (collection) return { paths: [], collections: [collection] };
  if (owner.inventory.has(directory)) return owner.inventory.get(directory);
  if ((yield { operation: "state", path: directory }) !== "directory") throw Error("Bun current source owner must be a real directory: " + directory);
  const entries = yield { operation: "list", path: directory }, paths = [], collections = [];
  if (entries.some(entry => entry.name === "package.json")) paths.push(directory);
  for (const entry of entries) if (!entry.name.startsWith(".") && !opaque.has(entry.name)) {
    if (entry.kind === "symlink") throw Error("Bun source directory is a symlink: " + join(directory, entry.name));
    if (entry.kind === "directory") { const child = yield* walkPlan(owner, join(directory, entry.name)); paths.push(...child.paths); collections.push(...child.collections); }
  }
  const result = { paths, collections }; owner.inventory.set(directory, result); return result;
}
/** 📁️ Expands fresh physical ownership through one resumable computation. */
function* candidatesPlan(owner, scope, excluded = new Set()) {
  const positive = scope.declaration.members.filter(path => !path.startsWith("!")), negative = scope.declaration.members.filter(path => path.startsWith("!")).map(path => patternRegex(path.slice(1))), found = new Set();
  for (const pattern of positive) {
    const selector = patternRegex(pattern), start = bounded(owner.root, join(scope.directory, sourcePrefix(pattern))), state = yield { operation: "state", path: start };
    if (state === "missing") continue;
    if (state !== "directory") throw Error("Bun source owner must be a real directory: " + start);
    const candidates = yield* walkPlan(owner, start);
    candidates.collections.forEach(path => excluded.add(path));
    for (const directory of candidates.paths) {
      yield { operation: "selection", path: directory };
      const local = slash(relative(resolve(owner.root, scope.directory), directory));
      if (local && selector.test(local) && !negative.some(selector => selector.test(local + "/") || selector.test(local))) found.add(slash(relative(owner.root, directory)));
    }
  }
  return [...found].sort((a, b) => a.localeCompare(b));
}
function* packagesPlan(owner, scope) {
  if (owner.selected.has(scope.manifest)) return owner.selected.get(scope.manifest);
  const paths = yield* candidatesPlan(owner, scope), candidates = [];
  for (const relDir of paths) candidates.push({ relDir, absDir: resolve(owner.root, relDir), ...(yield* documentPlan(owner, relDir + "/package.json")) });
  const byDirectory = new Map(candidates.map(candidate => [candidate.absDir, candidate])), selected = [];
  for (const candidate of candidates) {
    yield { operation: "selection", path: candidate.absDir };
    let retained = true;
    for (let parent = dirname(candidate.absDir); parent !== owner.root && parent !== dirname(parent); parent = dirname(parent)) {
      const parentOwner = byDirectory.get(parent);
      if (parentOwner) { retained = !(yield* ownsPayloadPlan(parentOwner, candidate)); break; }
    }
    if (retained) selected.push(candidate.relDir);
  }
  owner.selected.set(scope.manifest, selected); return selected;
}
function* nativePatternsPlan(owner, scope) {
  return (yield* packagesPlan(owner, scope)).map(path => {
    const literal = slash(relative(resolve(owner.root, scope.directory), resolve(owner.root, path))), local = /[?*\[\]{}]/u.test(literal) ? literal.replace(/[?*\[\]]/g, "\\$&").replace(/[{}]/g, character => "[" + character + "]") + "{,}" : literal;
    return local.startsWith("!") ? "./" + local : local;
  }).sort((a, b) => Buffer.from(a).compare(Buffer.from(b)));
}
/** 🗂️ Admits current scopes and source membership without reading testing collections. */
function* membershipPlan(owner) {
  const document = yield* documentPlan(owner, "package.json"), declaration = parseBunWorkspaceDeclaration(document.semio?.workspace), paths = new Set(["package.json"]);
  for (const pattern of declaration.owners) {
    if (pattern !== "*/package.json") throw Error("Unsupported physical owner recipe: " + pattern);
    const entries = yield { operation: "list", path: owner.root };
    for (const entry of entries) if (!entry.name.startsWith(".") && !opaque.has(entry.name) && !collectionRoot(owner.root, join(owner.root, entry.name))) {
      if (entry.kind === "symlink") throw Error("Bun installation owner is a symlink: " + entry.name);
      if (entry.kind === "directory" && (yield { operation: "state", path: join(owner.root, entry.name, "package.json") }) !== "missing") paths.add(entry.name + "/package.json");
    }
  }
  const scopes = [], all = new Set();
  for (const path of [...paths].sort()) {
    const row = yield* documentPlan(owner, path);
    if (row.semio?.workspace === undefined) continue;
    const d = dirname(path), directory = d === "." ? "." : slash(d), admitted = parseBunWorkspaceDeclaration(row.semio.workspace);
    for (const pattern of admitted.members) bounded(owner.root, join(directory, sourcePrefix(pattern.replace(/^!/u, ""))));
    const scope = { directory, manifest: path, lock: directory === "." ? "bun.lock" : directory + "/bun.lock", declaration: admitted }, packages = yield* packagesPlan(owner, scope);
    packages.forEach(path => all.add(path)); scopes.push(scope);
  }
  return { scopes, packages: [...all].sort((a, b) => a.localeCompare(b)) };
}
function executeOperation(operations, request) { return request.operation === "selection" ? undefined : operations[request.operation](request.path); }
function drain(plan, operations) {
  let step = plan.next();
  try { while (!step.done) step = plan.next(executeOperation(operations, step.value)); return step.value; }
  finally { plan.return(); }
}
function nativeOperations(observations, root) {
  const retain = (operation, path, value) => {
    if (!observations) return;
    const key = JSON.stringify([operation, path]), previous = observations.get(key);
    if (previous && JSON.stringify(previous.value) !== JSON.stringify(value)) throw Error("Bun current ownership changed during verification: " + path);
    if (!previous) observations.set(key, { operation, path, value });
  };
  return {
    state(path) {
      try {
        const entry = fs.lstatSync(path), kind = entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other";
        retain("state", path, [kind, String(entry.dev), String(entry.ino), entry.mode]); return kind;
      } catch (error) { if (error.code === "ENOENT") { retain("state", path, ["missing"]); return "missing"; } throw error; }
    },
    list(path) {
      const entries = fs.readdirSync(path, { withFileTypes: true }).map(entry => ({ name: entry.name, kind: entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other" })).sort((a, b) => Buffer.from(a.name).compare(Buffer.from(b.name)));
      retain("list", path, observations ? entries.filter(entry => entry.name === "package.json" || !entry.name.startsWith(".") && !opaque.has(entry.name) && (entry.kind === "symlink" || entry.kind === "directory" && !collectionRoot(root, join(path, entry.name)))) : entries); return entries;
    },
    readText(path) {
      const ancestry = [];
      for (let current = dirname(path); ; current = dirname(current)) { ancestry.push(current); if (current === dirname(current)) break; }
      for (const current of ancestry.reverse()) if (this.state(current) !== "directory") throw Error("Bun workspace ancestor must be a real directory: " + current);
      if (this.state(path) !== "file") throw Error("Bun workspace requires a regular manifest: " + path);
      return fs.readFileSync(path, "utf8");
    },
  };
}
const readBunRepositoryMembership = (root, operations) => drain(membershipPlan(context(root)), operations);
const bunRepositoryMembership = root => readBunRepositoryMembership(root, nativeOperations());
const bunWorkspacePackages = (root, scope) => drain(packagesPlan(context(root), scope), nativeOperations());
const bunWorkspaceNativePatterns = (root, scope) => drain(nativePatternsPlan(context(root), scope), nativeOperations());
function* publicationPlan(owner, verifyPublished) {
  const membership = yield* membershipPlan(owner), scopes = [];
  for (const scope of membership.scopes) {
    const patterns = yield* nativePatternsPlan(owner, scope), packages = yield* packagesPlan(owner, scope), document = owner.documents.get(scope.manifest);
    if (verifyPublished && JSON.stringify(patterns) !== JSON.stringify(document.workspaces)) throw Error("Bun published workspace patterns drift: " + scope.manifest);
    scopes.push({ manifest: scope.manifest, beforeSha256: createHash("sha256").update(owner.texts.get(scope.manifest)).digest("hex"), patterns, packages });
  }
  for (const [path, text] of owner.texts) if ((yield { operation: "readText", path: bounded(owner.root, path) }) !== text) throw Error("Bun current manifest changed during verification: " + path);
  for (const observation of owner.observations.values()) yield { operation: observation.operation, path: observation.path };
  return { version: 1, scopes, packages: membership.packages };
}
/** 🛡️ Fences the last caller continuation before native publication or a completed report. */
function currentFence(owner, operations, signal) {
  signal.throwIfAborted();
  if (fs.readFileSync(ownershipPath, "utf8") !== ownershipSource) throw Error("Bun current ownership policy changed during publication");
  for (const observation of owner.observations.values()) { signal.throwIfAborted(); executeOperation(operations, observation); }
  for (const [path, text] of owner.texts) {
    signal.throwIfAborted();
    if (operations.readText(bounded(owner.root, path)) !== text) throw Error("Bun current manifest changed during publication: " + path);
  }
}
/** 🎛️ Computes complete current ownership through the original caller control. */
async function controlledOwnership(root, control, verifyPublished) {
  if (!control || !(control.signal instanceof AbortSignal) || typeof control.advance !== "function") throw Error("Original Bun workspace control required");
  if (fs.readFileSync(ownershipPath, "utf8") !== ownershipSource) throw Error("Bun current ownership policy changed during verification");
  const owner = context(root), operations = nativeOperations(owner.observations, owner.root), plan = publicationPlan(owner, verifyPublished); let step, completedOperations = 0;
  try {
    control.signal.throwIfAborted(); step = plan.next();
    while (!step.done) {
      control.signal.throwIfAborted();
      const value = executeOperation(operations, step.value);
      await control.advance({ ...step.value, completedOperations: ++completedOperations });
      control.signal.throwIfAborted();
      step = plan.next(value);
    }
    control.signal.throwIfAborted();
    if (fs.readFileSync(ownershipPath, "utf8") !== ownershipSource) throw Error("Bun current ownership policy changed during verification");
    currentFence(owner, operations, control.signal);
    return { report: step.value, owner, completedOperations };
  } finally { plan.return(); }
}
/** 🎛️ Verifies the complete exact current native projection. */
async function inspectBunWorkspaceOwnership(root, control) { return (await controlledOwnership(root, control, true)).report; }
/** 📣️ Publishes every affected scope before installation while preserving current Source guards. */
async function publishBunWorkspaceOwnership(root, control) {
  const current = await controlledOwnership(root, control, false), { report, owner } = current, operations = nativeOperations(owner.observations, owner.root);
  const validate = async () => {
    control.signal.throwIfAborted();
    if (fs.readFileSync(ownershipPath, "utf8") !== ownershipSource) throw Error("Bun current ownership policy changed during publication");
    for (const observation of owner.observations.values()) {
      control.signal.throwIfAborted(); executeOperation(operations, observation);
      await control.advance({ operation: observation.operation, path: observation.path, completedOperations: ++current.completedOperations });
      control.signal.throwIfAborted();
    }
    for (const [path, text] of owner.texts) {
      control.signal.throwIfAborted(); const target = bounded(root, path);
      if (operations.readText(target) !== text) throw Error("Bun current manifest changed during publication: " + path);
      await control.advance({ operation: "readText", path: target, completedOperations: ++current.completedOperations });
      control.signal.throwIfAborted();
      if (operations.readText(target) !== text) throw Error("Bun current manifest changed during publication: " + path);
    }
  };
  for (const scope of report.scopes) {
    const before = owner.texts.get(scope.manifest), document = owner.documents.get(scope.manifest);
    if (JSON.stringify(document.workspaces) === JSON.stringify(scope.patterns)) continue;
    await validate();
    currentFence(owner, operations, control.signal);
    const path = bounded(root, scope.manifest), after = JSON.stringify({ ...document, workspaces: scope.patterns }, null, 2) + "\n";
    fs.writeFileSync(path, after); owner.texts.set(scope.manifest, after);
    await control.advance({ operation: "publication", path, completedOperations: ++current.completedOperations });
    control.signal.throwIfAborted();
  }
  await validate();
  currentFence(owner, operations, control.signal);
  return report;
}
module.exports = { parseBunWorkspaceDeclaration, readBunRepositoryMembership, bunRepositoryMembership, bunWorkspacePackages, bunWorkspaceNativePatterns, inspectBunWorkspaceOwnership, publishBunWorkspaceOwnership, discoverBunWorkspaces: root => bunRepositoryMembership(root).scopes, bunRepositoryPackages: root => bunRepositoryMembership(root).packages };
