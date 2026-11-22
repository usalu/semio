const fs = require("node:fs");
const { dirname, isAbsolute, join, relative, resolve } = require("node:path");
const { ownsPayload } = require("../📦️payload/🟨️.cjs");
const { createHash } = require("node:crypto");
const limits = Object.freeze({ ...require("./🧬️schema/🔣️.json").$defs.PhysicalLimits.const });
const frontierReplayProtocol = Object.freeze({ ...require("./🧬️schema/🔣️.json").$defs.FrontierReplayProtocol.const });
const slash = (path) => path.replaceAll("\\", "/");
const churn = Object.freeze(require("../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🧬️schema/🔣️.json").definitions.Protocol.const);
/** 🧨️ A peer edit of an owner manifest, the ownership policy or the membership while this custody was open: the plain error carries the code, reason, path and layer rank the churn policy declares, so the step that owns the snapshot takes a new one. */
const edited = (message, reason) => { const split = message.indexOf(": "); return Object.assign(Error(message), { code: churn.concurrentEditCode, reason: reason ?? (/membership|manifests changed/u.test(message) ? "roster" : "bytes"), path: split < 0 ? "bun workspace ownership" : message.slice(split + 2).split(";")[0], rank: churn.profiles.snapshot.rank - 1 }); };
const policySourceFields = Object.freeze([...require("./🧬️schema/🔣️.json").$defs.PhysicalPolicySource.required].sort());
const {workspaceTestingCollectionRoot: collectionRoot} = require("../🟨️.cjs");
const opaque = new Set(require("./🧬️schema/🔣️.json").$defs.OutputDirectories.const);
const outputPattern = new RegExp(require("./🧬️schema/🔣️.json").$defs.OutputDirectoryPattern.const, "u");
const outputName = name => opaque.has(name) || outputPattern.test(name);
const outputDirectory = (root, directory) => relative(root, directory).split(/[\\/]/u).some(outputName);

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
  if (full.length > limits.maximumPathUnits || local === ".." || local.startsWith("../") || local.startsWith("..\\") || isAbsolute(local)) throw Error(`Bun workspace escapes repository or path capacity: ${path}`);
  return full;
};

const physicalBudget = () => ({ texts: new Map(), bytes: 0, lists: new Map(), entries: 0 });
const stateKindNames = Object.freeze(["missing", "file", "directory", "symlink", "other"]), stateKindCodes = Object.freeze({ missing: 0, file: 1, directory: 2, symlink: 3, other: 4 });
const listingKindChars = Object.freeze({ file: "f", directory: "d", symlink: "l", other: "o" }), listingKindNames = Object.freeze({ f: "file", d: "directory", l: "symlink", o: "other" });
const internedNameCapacity = 262144, internedNames = new Map();
/** 🏷️ Shares one string per distinct directory entry name, so a million listing rows hold a few thousand name strings. */
function internName(name) {
  let shared = internedNames.get(name);
  if (shared === undefined) { if (internedNames.size >= internedNameCapacity) internedNames.clear(); internedNames.set(name, name); shared = name; }
  return shared;
}
const emptyListing = Object.freeze({ names: Object.freeze([]), kinds: "", length: 0 });
const isListing = value => value !== null && typeof value === "object" && typeof value.kinds === "string" && Array.isArray(value.names);
/** 🗜️ Packs ordered `{name, kind}` rows into interned names beside one kind character per row; the packed listing is immutable. */
function packListing(entries) {
  if (!entries.length) return emptyListing;
  const names = new Array(entries.length); let kinds = "";
  for (let index = 0; index < entries.length; index++) { const kind = listingKindChars[entries[index].kind]; if (kind === undefined) throw Error("Closed Bun directory entry kind required"); names[index] = internName(entries[index].name); kinds += kind; }
  return Object.freeze({ names, kinds, length: names.length });
}
/** 📖️ Unpacks a listing into fresh `{name, kind}` rows in their original order. */
function listingEntries(listing, frozen = false) {
  const rows = new Array(listing.length);
  for (let index = 0; index < listing.length; index++) { const row = { name: listing.names[index], kind: listingKindNames[listing.kinds[index]] }; rows[index] = frozen ? Object.freeze(row) : row; }
  return frozen ? Object.freeze(rows) : rows;
}
/** ⚖️ Compares two packed listings row by row. */
function listingEquals(left, right) {
  if (left === right) return true;
  if (left.length !== right.length || left.kinds !== right.kinds) return false;
  for (let index = 0; index < left.length; index++) if (left.names[index] !== right.names[index]) return false;
  return true;
}
/** 🪶️ Holds every observed path, its physical state, its listing, its frontier stamp and the global fact order in flat typed arrays and one name per path, instead of an object graph per path. */
class PackedObservations {
  constructor() {
    this.index = new Map(); this.paths = []; this.lists = [];
    this.kind = new Uint8Array(1024); this.dev = new BigUint64Array(1024); this.ino = new BigUint64Array(1024); this.mode = new Uint32Array(1024);
    this.mtime = new BigInt64Array(1024); this.ctime = new BigInt64Array(1024); this.settled = new Uint8Array(1024);
    this.facts = new Uint32Array(2048); this.factCount = 0;
  }
  get size() { return this.paths.length; }
  reserve(count) {
    if (count <= this.kind.length) return;
    const capacity = Math.max(count, this.kind.length * 2);
    for (const name of ["kind", "dev", "ino", "mode", "mtime", "ctime", "settled"]) { const next = new this[name].constructor(capacity); next.set(this[name]); this[name] = next; }
  }
  addState(path, tuple) {
    const id = this.paths.length, code = tuple[0] === "missing" ? 0 : Object.hasOwn(stateKindCodes, tuple[0]) ? stateKindCodes[tuple[0]] : undefined;
    if (code === undefined || tuple.length !== (code === 0 ? 1 : 4)) throw Error("Closed Bun ownership observation kind required");
    let dev = 0n, ino = 0n, mode = 0;
    if (code !== 0) {
      dev = BigInt.asUintN(64, BigInt(tuple[1])); ino = BigInt.asUintN(64, BigInt(tuple[2])); mode = tuple[3];
      if (String(dev) !== tuple[1] || String(ino) !== tuple[2] || !Number.isInteger(mode) || mode < 0 || mode > 4294967295) throw Error("Closed Bun ownership observation kind required");
    }
    this.reserve(id + 1); this.kind[id] = code; this.dev[id] = dev; this.ino[id] = ino; this.mode[id] = mode; this.settled[id] = 0;
    this.index.set(path, id); this.paths.push(path); this.lists.push(undefined); return id;
  }
  pushFact(id, operation) {
    if (this.factCount >= this.facts.length) { const next = new Uint32Array(this.facts.length * 2); next.set(this.facts); this.facts = next; }
    this.facts[this.factCount++] = id * 2 + (operation === "list" ? 1 : 0);
  }
  stateTuple(id) { const code = this.kind[id]; return code === 0 ? ["missing"] : [stateKindNames[code], String(this.dev[id]), String(this.ino[id]), this.mode[id]]; }
  stateEquals(id, tuple) {
    const code = this.kind[id];
    if (code === 0) return tuple.length === 1 && tuple[0] === "missing";
    return tuple.length === 4 && tuple[0] === stateKindNames[code] && tuple[1] === String(this.dev[id]) && tuple[2] === String(this.ino[id]) && tuple[3] === this.mode[id];
  }
  stateMatches(id, facts) {
    const code = this.kind[id];
    if (!facts) return code === 0;
    return code === stateKindCodes[facts.isSymbolicLink() ? "symlink" : facts.isDirectory() ? "directory" : facts.isFile() ? "file" : "other"] && this.dev[id] === BigInt.asUintN(64, facts.dev) && this.ino[id] === BigInt.asUintN(64, facts.ino) && this.mode[id] === Number(facts.mode);
  }
  setFrontier(path, facts, settled) { const id = this.index.get(path); this.mtime[id] = BigInt.asIntN(64, facts.mtimeNs); this.ctime[id] = BigInt.asIntN(64, facts.ctimeNs); this.settled[id] = settled ? 2 : 1; }
  clearFrontier(path) { const id = this.index.get(path); if (id !== undefined) this.settled[id] = 0; }
  frontierMoved(id, facts) { return this.mtime[id] !== BigInt.asIntN(64, facts.mtimeNs) || this.ctime[id] !== BigInt.asIntN(64, facts.ctimeNs); }
  rows() { return observedRows(this); }
  factRows() { const rows = []; for (let index = 0; index < this.factCount; index++) rows.push([this.paths[this.facts[index] >>> 1], (this.facts[index] & 1) === 1 ? "list" : "state"]); return rows; }
  record(path) { const id = this.index.get(path); return id === undefined ? undefined : packedRow(this, id); }
}
/** ⏱️ Collects the frontier stamps one replay proved in flat arrays and applies them only once the whole replay holds. */
class FrontierProof {
  constructor(size) { this.mtime = new BigInt64Array(size); this.ctime = new BigInt64Array(size); this.settled = new Uint8Array(size); }
  set(id, facts, settled) { this.mtime[id] = BigInt.asIntN(64, facts.mtimeNs); this.ctime[id] = BigInt.asIntN(64, facts.ctimeNs); this.settled[id] = settled ? 2 : 1; }
  applyTo(observations) { for (let id = 0; id < this.settled.length; id++) if (this.settled[id] !== 0) { observations.mtime[id] = this.mtime[id]; observations.ctime[id] = this.ctime[id]; observations.settled[id] = this.settled[id]; } }
}
/** 🧱️ Materializes one frozen `{path, state, list?}` row from the packed arrays of an observation set. */
function packedRow(observations, id) {
  const list = observations.lists[id];
  return Object.freeze({ path: observations.paths[id], state: Object.freeze(observations.stateTuple(id)), ...(list ? { list: listingEntries(list, true) } : {}) });
}
/** 👁️ Presents a snapshot of the packed observations as a read-only array whose rows are built on access and never retained. */
function observedRows(observations) {
  const { paths, lists, kind, dev, ino, mode } = observations, count = paths.length, snapshot = { paths, lists, kind, dev, ino, mode, stateTuple: PackedObservations.prototype.stateTuple };
  const isIndex = key => typeof key === "string" && /^(?:0|[1-9][0-9]*)$/u.test(key) && Number(key) < count;
  let recent = -1, row;
  const at = id => { if (id !== recent) { row = packedRow(snapshot, id); recent = id; } return row; };
  return new Proxy([], {
    get: (target, key, receiver) => key === "length" ? count : isIndex(key) ? at(Number(key)) : Reflect.get(target, key, receiver),
    has: (target, key) => isIndex(key) || Reflect.has(target, key),
    ownKeys: () => [...Array.from({ length: count }, (_, index) => String(index)), "length"],
    getOwnPropertyDescriptor: (target, key) => isIndex(key) ? { value: at(Number(key)), writable: false, enumerable: true, configurable: true } : key === "length" ? { value: count, writable: true, enumerable: false, configurable: false } : Reflect.getOwnPropertyDescriptor(target, key),
    set: () => false, defineProperty: () => false, deleteProperty: () => false,
  });
}
const physicalObservations = () => new PackedObservations();
const context = root => ({ root, inventory: new Map(), documents: new Map(), texts: new Map(), selected: new Map(), observations: physicalObservations(), physical: physicalBudget() });
function sameManifest(left, right) { return ["dev", "ino", "mode", "nlink", "size", "mtimeNs", "ctimeNs", "birthtimeNs"].every(key => left[key] === right[key]); }
/** 📏️ Reads a bounded actual descriptor and refuses substitution, truncation or growth before returning text. */
function readManifestText(path, budget, maximum = limits.maximumManifestBytes) {
  if (typeof path !== "string" || path.length > limits.maximumPathUnits) throw Error("Bun workspace physical path capacity exceeded");
  const before = fs.lstatSync(path, { bigint: true });
  if (!before.isFile()) throw Error("Bun workspace requires a regular manifest: " + path);
  if (before.size < 0n || before.size > BigInt(maximum)) throw Error("Bun manifest byte capacity exceeded: " + path);
  const size = Number(before.size), prior = budget?.texts.get(path) ?? 0, total = (budget?.bytes ?? 0) - prior + size;
  if (budget && (!budget.texts.has(path) && budget.texts.size >= limits.maximumTextDocuments || total > limits.maximumAggregateManifestBytes)) throw Error("Bun aggregate manifest byte capacity exceeded: " + path);
  const descriptor = fs.openSync(path, fs.constants.O_RDONLY | (fs.constants.O_NOFOLLOW ?? 0));
  let failed = false;
  try {
    if (!sameManifest(before, fs.fstatSync(descriptor, { bigint: true }))) throw edited("Bun current manifest changed during descriptor admission: " + path);
    const bytes = Buffer.alloc(size), probe = Buffer.alloc(1); let offset = 0;
    while (offset < size) { const count = fs.readSync(descriptor, bytes, offset, Math.min(size - offset, limits.readChunkBytes), offset); if (!count) throw edited("Bun current manifest changed during descriptor read: " + path); offset += count; }
    if (fs.readSync(descriptor, probe, 0, 1, size) || !sameManifest(before, fs.fstatSync(descriptor, { bigint: true })) || !sameManifest(before, fs.lstatSync(path, { bigint: true }))) throw edited("Bun current manifest changed during descriptor read: " + path);
    const text = new TextDecoder("utf-8", { fatal: true }).decode(bytes);
    if (budget) { budget.texts.set(path, size); budget.bytes = total; }
    return text;
  } catch (error) { failed = true; throw error; }
  finally { try { fs.closeSync(descriptor); } catch (error) { if (!failed) throw error; } }
}
/** 🎯️ Admits one explicit immutable ownership selection without creating operation authority. */
function ownershipSelection(value) {
  if (!value || typeof value !== "object" || Array.isArray(value)) throw Error("Explicit Bun ownership selection required");
  const keys = Object.keys(value);
  if (value.kind === "all" && keys.length === 1 && keys[0] === "kind") return Object.freeze({ kind: "all" });
  if (value.kind === "scope" && keys.length === 2 && keys.includes("kind") && keys.includes("owner") && typeof value.owner === "string" && /^(?:\.|[^./\\:\u0000][^/\\:\u0000]*)$/u.test(value.owner)) return Object.freeze({ kind: "scope", owner: value.owner });
  if (value.kind === "closure" && keys.length === 2 && keys.includes("kind") && keys.includes("roots") && Array.isArray(value.roots) && value.roots.length <= 256 && value.roots.every(root => typeof root === "string" && /^(?:\.|[^./\\:\u0000][^/\\:\u0000]*)$/u.test(root)) && new Set(value.roots).size === value.roots.length) return Object.freeze({ kind: "closure", roots: Object.freeze([...value.roots].sort()) });
  throw Error("Explicit Bun ownership selection required");
}
function* documentPlan(owner, path) {
  if (!owner.documents.has(path)) {
    const text = yield { operation: "readText", path: bounded(owner.root, path) };
    owner.texts.set(path, text); owner.documents.set(path, JSON.parse(text));
  }
  return owner.documents.get(path);
}
const noCandidates = Object.freeze({ paths: Object.freeze([]), collections: Object.freeze([]) });
function* walkPlan(owner, directory) {
  if (outputDirectory(owner.root, directory)) return { paths: [], collections: [] };
  const collection = collectionRoot(owner.root, directory);
  if (collection) return { paths: [], collections: [collection] };
  if (owner.inventory.has(directory)) return owner.inventory.get(directory);
  if ((yield { operation: "state", path: directory }) !== "directory") throw Error("Bun current source owner must be a real directory: " + directory);
  const entries = yield { operation: "list", path: directory }, paths = [], collections = [];
  if (entries.some(entry => entry.name === "package.json")) paths.push(directory);
  for (const entry of entries) if (!entry.name.startsWith(".") && !outputName(entry.name)) {
    if (entry.kind === "symlink") throw Error("Bun source directory is a symlink: " + join(directory, entry.name));
    if (entry.kind === "directory") { const child = yield* walkPlan(owner, join(directory, entry.name)); paths.push(...child.paths); collections.push(...child.collections); }
  }
  const result = paths.length || collections.length ? { paths, collections } : noCandidates; owner.inventory.set(directory, result); return result;
}
/** 📁️ Expands fresh physical ownership through one resumable computation. */
function* candidatesPlan(owner, scope, excluded = new Set()) {
  const positive = scope.declaration.members.filter(path => !path.startsWith("!")), negative = scope.declaration.members.filter(path => path.startsWith("!")).map(path => patternRegex(path.slice(1))), found = new Set();
  for (const pattern of positive) {
    const selector = patternRegex(pattern), start = bounded(owner.root, join(scope.directory, sourcePrefix(pattern)));
    if (outputDirectory(owner.root, start)) continue;
    const state = yield { operation: "state", path: start };
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
      if (parentOwner) { retained = !ownsPayload(parentOwner, candidate); break; }
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
function* scopesPlan(owner, selection) {
  const document = yield* documentPlan(owner, "package.json"), declaration = parseBunWorkspaceDeclaration(document.semio?.workspace), paths = new Set(["package.json"]);
  if (selection.kind === "scope" && selection.owner !== ".") {
    const path = join(owner.root, selection.owner);
    if (!declaration.owners.includes("*/package.json") || opaque.has(selection.owner) || collectionRoot(owner.root, path)) throw Error("Current authored Bun installation scope required");
    paths.clear(); paths.add(selection.owner + "/package.json");
  }
  for (const pattern of selection.kind === "all" || selection.kind === "closure" ? declaration.owners : []) {
    if (pattern !== "*/package.json") throw Error("Unsupported physical owner recipe: " + pattern);
    const entries = yield { operation: "list", path: owner.root };
    for (const entry of entries) if (!entry.name.startsWith(".") && !outputName(entry.name) && !collectionRoot(owner.root, join(owner.root, entry.name))) {
      if (entry.kind !== "directory" && entry.kind !== "symlink" || (yield { operation: "state", path: join(owner.root, entry.name, "package.json") }) === "missing") continue;
      if (entry.kind === "symlink") throw Error("Bun installation owner is a symlink: " + entry.name);
      paths.add(entry.name + "/package.json");
    }
  }
  const scopes = [];
  for (const path of [...paths].sort()) {
    const row = yield* documentPlan(owner, path);
    if (row.semio?.workspace === undefined) { if (selection.kind === "scope") throw Error("Current authored Bun installation scope required"); continue; }
    const d = dirname(path), directory = d === "." ? "." : slash(d), admitted = parseBunWorkspaceDeclaration(row.semio.workspace);
    for (const pattern of admitted.members) bounded(owner.root, join(directory, sourcePrefix(pattern.replace(/^!/u, ""))));
    scopes.push({ directory, manifest: path, lock: directory === "." ? "bun.lock" : directory + "/bun.lock", declaration: admitted });
  }
  owner.scopeDirectories = scopes.map(scope => scope.directory);
  return selection.kind === "closure" ? yield* closurePlan(owner, scopes, selection.roots) : scopes;
}
/** 🧭️ Names the deepest declared scope directory that contains the path (the path itself or a descendant), the root scope when none does; the one ownership rule of closures and of the manifests a walk retained. */
function deepestScopeDirectory(directories, path) {
  const depth = directory => directory === "." ? 0 : directory.split("/").length;
  return directories.reduce((best, directory) => (directory === "." || path === directory || path.startsWith(directory + "/")) && depth(directory) > depth(best) ? directory : best, ".");
}
/** 🧬️ Narrows the declared scopes to the root scope, the requested roots and, breadth first, the deepest declaring scope of every member of a reached scope; no scope outside that closure has its members walked. */
function* closurePlan(owner, scopes, roots) {
  const byDirectory = new Map(scopes.map(scope => [scope.directory, scope]));
  if (!byDirectory.has(".") || roots.some(root => !byDirectory.has(root))) throw Error("Current authored Bun installation scope required");
  const ownerOf = path => deepestScopeDirectory(scopes.map(scope => scope.directory), path);
  const reached = new Set([".", ...roots]), queue = [...reached];
  for (let index = 0; index < queue.length; index++) {
    const scope = byDirectory.get(queue[index]);
    for (const path of [scope.directory, ...(yield* packagesPlan(owner, scope))]) { const next = ownerOf(path); if (!reached.has(next)) { reached.add(next); queue.push(next); } }
  }
  return scopes.filter(scope => reached.has(scope.directory));
}
/** 🗂️ Computes packages only for the already admitted current installation scopes. */
function* membershipPlan(owner, selection) {
  const scopes = yield* scopesPlan(owner, selection), all = new Set();
  for (const scope of scopes) (yield* packagesPlan(owner, scope)).forEach(path => all.add(path));
  return { scopes, packages: [...all].sort((a, b) => a.localeCompare(b)) };
}
function executeOperation(operations, request) { return request.operation === "selection" ? undefined : operations[request.operation](request.path); }
function drain(plan, operations) {
  let step = plan.next();
  try { while (!step.done) step = plan.next(executeOperation(operations, step.value)); return step.value; }
  finally { plan.return(); }
}
/** 🔎️ Describes at most eight changed direct children without changing the original observation fence. */
function ownershipChange(operation, path, previous, current) {
  if (operation !== "list") return { operation, path, previous, current };
  const added = [], removed = [], typeChanged = []; let left = 0, right = 0, total = 0;
  const retain = (rows, row) => { if (total++ < 8) rows.push(row); };
  while (left < previous.length || right < current.length) {
    const before = previous[left], after = current[right], order = before && after ? Buffer.from(before.name).compare(Buffer.from(after.name)) : before ? -1 : 1;
    if (order < 0) { retain(removed, before); left++; }
    else if (order > 0) { retain(added, after); right++; }
    else { if (before.kind !== after.kind) retain(typeChanged, { name: before.name, before: before.kind, after: after.kind }); left++; right++; }
  }
  return { operation, path, added, removed, typeChanged, total, omitted: Math.max(0, total - 8) };
}
/** 🧾️ Retains original path facts with independent path/fact bounds and global reference order. */
function retainOwnershipObservationV1(observations, operation, path, value) {
  if (!observations) return;
  if (operation !== "state" && operation !== "list") throw Error("Closed Bun ownership observation kind required");
  const id = observations.index.get(path), listing = operation === "list" ? isListing(value) ? value : packListing(value) : undefined;
  if (id !== undefined && (operation === "state" || observations.lists[id] !== undefined)) {
    if (operation === "state" ? observations.stateEquals(id, value) : listingEquals(observations.lists[id], listing)) return;
    const previous = operation === "state" ? observations.stateTuple(id) : listingEntries(observations.lists[id]), current = operation === "state" ? value : listingEntries(listing);
    throw edited("Bun current ownership changed during verification: " + path + "; change=" + JSON.stringify(ownershipChange(operation, path, previous, current)));
  }
  if (operation === "list" && (id === undefined || observations.kind[id] !== stateKindCodes.directory)) throw Error("Original Bun directory state observation required");
  if (id === undefined && observations.size >= limits.maximumObservationPaths) throw Error("Bun ownership physical path observation capacity exceeded");
  if (observations.factCount >= limits.maximumObservationFacts) throw Error("Bun ownership observation fact capacity exceeded");
  const target = id === undefined ? observations.addState(path, value) : id;
  if (operation === "list") observations.lists[target] = listing;
  observations.pushFact(target, operation);
}
/** 🧊️ Remembers one directory listing under the directory's physical stamp, so a later derivation re-lists only directories whose entries could have changed. */
const listingCache = new Map();
const directoryStamp = entry => [entry.dev, entry.ino, entry.mode, entry.mtimeNs, entry.ctimeNs].map(String).join(":");
const settledDirectory = (entry, now) => entry.mtimeNs < BigInt(now - limits.listingSettleMilliseconds) * 1000000n && entry.ctimeNs < BigInt(now - limits.listingSettleMilliseconds) * 1000000n;
const ownershipStateV1 = entry => entry ? [kindOf(entry), String(entry.dev), String(entry.ino), Number(entry.mode)] : ["missing"];
function ownershipFrontierDecisionV1(before, after, settled) { return !before || !after ? "derive" : settled && before === after ? "reuse" : "relist"; }
const byName = (a, b) => Buffer.from(a.name).compare(Buffer.from(b.name));
const kindOf = entry => entry.isSymbolicLink() ? "symlink" : entry.isDirectory() ? "directory" : entry.isFile() ? "file" : "other";
/** 🔬️ Projects the only directory entries that can change ownership, so a listing digest decides whether a cached derivation still holds. */
const projectListing = (root, path, entries) => entries.filter(entry => entry.name === "package.json" || !entry.name.startsWith(".") && !outputName(entry.name) && (entry.kind === "symlink" || entry.kind === "directory" && !collectionRoot(root, join(path, entry.name))));
/** 🪄️ Projects a packed listing exactly as `projectListing` projects its rows and shares the packed listing when nothing is dropped. */
function projectPacked(root, path, listing) {
  const names = [], kinds = [];
  for (let index = 0; index < listing.length; index++) {
    const name = listing.names[index], kind = listing.kinds[index];
    if (name === "package.json" || !name.startsWith(".") && !outputName(name) && (kind === "l" || kind === "d" && !collectionRoot(root, join(path, name)))) { names.push(name); kinds.push(kind); }
  }
  return names.length === listing.length ? listing : names.length ? Object.freeze({ names, kinds: kinds.join(""), length: names.length }) : emptyListing;
}
/** 📋️ Lists one directory exactly as the ownership walk does and returns its projection. */
function readListingProjection(root, path) {
  const entries = fs.readdirSync(path, { withFileTypes: true }).map(entry => ({ name: entry.name, kind: kindOf(entry) }));
  entries.sort(byName);
  return projectListing(root, path, entries);
}
/** 📤️ Exports the settled directory listings this process retained under one workspace, so a later process can reuse them under their stamps. */
function exportOwnershipIndexV1(root) {
  const rows = [];
  for (const [path, record] of listingCache) if (record.root === root) rows.push({ path, stamp: record.stamp, count: record.count, entries: listingEntries(record.projected ??= projectPacked(root, path, record.listing)) });
  return rows;
}
/** 📥️ Seeds settled directory listings persisted by an earlier process; each is trusted only while the live directory carries exactly its stamp. */
function seedOwnershipIndexV1(root, records) {
  for (const record of records) {
    if (listingCache.size >= limits.maximumListingCacheDirectories) break;
    if (!listingCache.has(record.path)) { const listing = packListing(record.entries); listingCache.set(record.path, { stamp: record.stamp, root, listing, count: record.count ?? listing.length, projected: listing }); }
  }
}
function nativeOperations(observations, root, budget = physicalBudget()) {
  const retain = (operation, path, value) => retainOwnershipObservationV1(observations, operation, path, value);
  let recent;
  const observe = path => {
    if (typeof path !== "string" || path.length > limits.maximumPathUnits) throw Error("Bun workspace physical path capacity exceeded");
    try {
      const entry = fs.lstatSync(path, { bigint: true }), kind = kindOf(entry);
      retain("state", path, ownershipStateV1(entry)); return { kind, entry };
    } catch (error) { if (error.code === "ENOENT" || error.code === "ENOTDIR") { retain("state", path, ["missing"]); return { kind: "missing" }; } throw error; }
  };
  const operations = {
    state(path) {
      const { kind, entry } = observe(path);
      recent = kind === "directory" ? { path, entry } : undefined; return kind;
    },
    list(path) {
      let before = recent?.path === path ? recent.entry : undefined; recent = undefined;
      if (!before) { const current = observe(path); if (current.kind !== "directory") throw Error("Bun workspace requires a real directory: " + path); before = current.entry; }
      const listedAt = Date.now(), stamp = directoryStamp(before), prior = budget.lists.get(path) ?? 0, cached = listingCache.get(path);
      let record = cached?.stamp === stamp && cached.root === root ? cached : undefined, latest = before;
      let entries;
      if (!record) {
        entries = []; const directory = fs.opendirSync(path, { bufferSize: limits.directoryReadBatch }); let failed = false;
        try { for (let entry; (entry = directory.readSync()) !== null;) { if (entries.length >= limits.maximumDirectoryEntries || budget.entries - prior + entries.length + 1 > limits.maximumAggregateDirectoryEntries) throw Error("Bun directory observation capacity exceeded: " + path); entries.push({ name: entry.name, kind: kindOf(entry) }); } }
        catch (error) { failed = true; throw error; } finally { try { directory.closeSync(); } catch (error) { if (!failed) throw error; } }
        entries.sort(byName);
        const after = observe(path);
        if (after.kind !== "directory") throw Error("Bun workspace requires a real directory: " + path);
        latest = after.entry;
        record = { stamp, root, listing: packListing(entries), count: entries.length, projected: undefined };
        if (directoryStamp(after.entry) === stamp && settledDirectory(after.entry, listedAt)) { if (listingCache.size >= limits.maximumListingCacheDirectories) listingCache.clear(); listingCache.set(path, record); }
        else listingCache.delete(path);
      } else if (record.count > limits.maximumDirectoryEntries || budget.entries - prior + record.count > limits.maximumAggregateDirectoryEntries) throw Error("Bun directory observation capacity exceeded: " + path);
      budget.entries = budget.entries - prior + record.count; budget.lists.set(path, record.count);
      if (observations) {
        retain("list", path, record.projected ??= projectPacked(root, path, record.listing));
        if (latest.isDirectory() && directoryStamp(latest) === stamp) observations.setFrontier(path, latest, settledDirectory(latest, listedAt));
        else observations.clearFrontier(path);
      }
      return entries ?? listingEntries(record.listing);
    },
    readText(path) {
      recent = undefined;
      const ancestry = [];
      for (let current = dirname(path); ; current = dirname(current)) { ancestry.push(current); if (current === dirname(current)) break; }
      for (const current of ancestry.reverse()) if (operations.state(current) !== "directory") throw Error("Bun workspace ancestor must be a real directory: " + current);
      if (operations.state(path) !== "file") throw Error("Bun workspace requires a regular manifest: " + path);
      recent = undefined;
      return readManifestText(path, budget);
    },
  };
  return operations;
}
const readBunRepositoryMembership = (root, operations) => drain(membershipPlan(context(root), { kind: "all" }), operations);
const bunRepositoryMembership = root => readBunRepositoryMembership(root, nativeOperations());
const bunWorkspacePackages = (root, scope) => drain(packagesPlan(context(root), scope), nativeOperations());
const bunWorkspaceNativePatterns = (root, scope) => drain(nativePatternsPlan(context(root), scope), nativeOperations());
function* publicationPlan(owner, selection, verifyPublished) {
  const membership = yield* membershipPlan(owner, selection), scopes = [];
  for (const scope of membership.scopes) {
    const patterns = yield* nativePatternsPlan(owner, scope), packages = yield* packagesPlan(owner, scope), document = owner.documents.get(scope.manifest);
    if (verifyPublished && JSON.stringify(patterns) !== JSON.stringify(document.workspaces)) throw Error("Bun published workspace patterns drift: " + scope.manifest);
    scopes.push({ manifest: scope.manifest, beforeSha256: createHash("sha256").update(owner.texts.get(scope.manifest)).digest("hex"), patterns, packages });
  }
  for (const [path, text] of owner.texts) if ((yield { operation: "readText", path: bounded(owner.root, path) }) !== text) throw edited("Bun current manifest changed during verification: " + path);
  return { version: 1, scopes, packages: membership.packages };
}
/** 🔭️ Exposes the exact path states, directory listing projections and manifest digests one ownership computation retained, so a later process can prove the same topology by stamps; `textOwners` names, per retained manifest, the declared installation scope that owns it. */
function observedOwnership(owner) {
  const paths = owner.observations.rows(), texts = [...owner.texts].map(([path, text]) => Object.freeze({ path, sha256: createHash("sha256").update(text).digest("hex") }));
  const directories = owner.scopeDirectories ?? [], textOwners = Object.freeze(Object.fromEntries(texts.map(({ path }) => [path, deepestScopeDirectory(directories, path.includes("/") ? path.slice(0, path.lastIndexOf("/")) : ".")])));
  return Object.freeze({ root: owner.root, paths, texts: Object.freeze(texts), textOwners });
}
/** 🧭️ Remembers the first path a replay saw move and the fact that moved there (existence, identity, listing or content digest), so a refusal can name it; always answers false. */
function ownershipMoved(owner, path, fact, reason) { owner.moved ??= { path, fact, reason }; return false; }
/** 🪪️ Names the fact of a state record that no longer holds: the path appeared or vanished, or its kind, device, inode or mode changed. */
function ownershipStateMoved(owner, record, facts) { return ownershipMoved(owner, record.path, facts === undefined || record.state[0] === "missing" ? "existence" : "identity", facts === undefined ? "vanished" : record.state[0] === "missing" ? "appeared" : "identity"); }
/** ⚡️ Replays the complete original decision frontier and fences the final awaited continuation without rediscovering packages. */
async function replayCurrentMembershipV1(owner, control, progress) {
  let continuationFailed = false, visited = 0;
  const observations = owner.observations, proof = new FrontierProof(observations?.size ?? 0), operations = nativeOperations(undefined, owner.root, owner.physical), read = path => {
    try { return fs.lstatSync(path, { bigint: true }); } catch (error) { if (error.code === "ENOENT" || error.code === "ENOTDIR") return undefined; throw error; }
  };
  try {
    if (!observations?.size) return false;
    for (let id = 0; id < observations.size; id++) {
      control.signal.throwIfAborted();
      const completedOperations = ++progress.completedOperations;
      if (++visited === observations.size || completedOperations % (control.frontierBatchPaths ?? 1) === 0) try { await control.advance({ operation: "frontier", path: observations.paths[id], completedOperations }); } catch (error) { continuationFailed = true; throw error; }
      control.signal.throwIfAborted();
    }
    if (readManifestText(owner.ownership.ownershipPath, undefined, limits.maximumPolicyBytes) !== owner.ownership.ownershipSource) return ownershipMoved(owner, owner.ownership.ownershipPath, "content digest", "bytes");
    for (const [path, text] of owner.texts) {
      control.signal.throwIfAborted();
      if (readManifestText(bounded(owner.root, path), owner.physical) !== text) return ownershipMoved(owner, path, "content digest", "bytes");
    }
    for (let id = 0; id < observations.size; id++) {
      const path = observations.paths[id];
      control.signal.throwIfAborted(); const facts = read(path);
      if (!observations.stateMatches(id, facts)) return ownershipStateMoved(owner, { path, state: [stateKindNames[observations.kind[id]]] }, facts);
      const list = observations.lists[id];
      if (list) {
        if (!facts?.isDirectory()) return ownershipMoved(owner, path, "identity", "identity");
        const known = observations.settled[id] !== 0, decision = ownershipFrontierDecisionV1(known ? "known" : undefined, known && !observations.frontierMoved(id, facts) ? "known" : "moved", known && observations.settled[id] === 2 && settledDirectory(facts, Date.now()));
        if (decision === "derive") return ownershipMoved(owner, path, "identity", "identity");
        if (decision === "relist" && (!listingEquals(packListing(projectListing(owner.root, path, operations.list(path))), list) || directoryStamp(read(path)) !== directoryStamp(facts))) return ownershipMoved(owner, path, "listing", "roster");
        proof.set(id, facts, settledDirectory(facts, Date.now()));
      }
    }
    if (readManifestText(owner.ownership.ownershipPath, undefined, limits.maximumPolicyBytes) !== owner.ownership.ownershipSource) return ownershipMoved(owner, owner.ownership.ownershipPath, "content digest", "bytes");
    proof.applyTo(observations);
    return true;
  } catch (error) { control.signal.throwIfAborted(); if (continuationFailed) throw error; return ownershipMoved(owner, typeof error?.path === "string" ? error.path : owner.root, "content digest", "bytes"); }
}
/** 🧷️ Authenticates the complete candidate after the caller continuation without another await. */
function authenticateCurrentMembershipV1(owner, control) {
  const observations = owner.observations, operations = nativeOperations(undefined, owner.root, owner.physical), proof = new FrontierProof(observations?.size ?? 0), read = path => {
    try { return fs.lstatSync(path, { bigint: true }); } catch (error) { if (error.code === "ENOENT" || error.code === "ENOTDIR") return undefined; throw error; }
  };
  if (!observations?.size) return false;
  if (readManifestText(owner.ownership.ownershipPath, undefined, limits.maximumPolicyBytes) !== owner.ownership.ownershipSource) return ownershipMoved(owner, owner.ownership.ownershipPath, "content digest", "bytes");
  for (let id = 0; id < observations.size; id++) {
    const path = observations.paths[id];
    control.signal.throwIfAborted();
    const facts = read(path);
    if (!observations.stateMatches(id, facts)) return ownershipStateMoved(owner, { path, state: [stateKindNames[observations.kind[id]]] }, facts);
    const list = observations.lists[id];
    if (list) {
      if (!facts?.isDirectory()) return ownershipMoved(owner, path, "identity", "identity");
      const known = observations.settled[id] !== 0, decision = ownershipFrontierDecisionV1(known ? "known" : undefined, known && !observations.frontierMoved(id, facts) ? "known" : "moved", known && observations.settled[id] === 2 && settledDirectory(facts, Date.now()));
      if (decision === "derive") return ownershipMoved(owner, path, "identity", "identity");
      if (decision === "relist" && (!listingEquals(packListing(projectListing(owner.root, path, operations.list(path))), list) || directoryStamp(read(path)) !== directoryStamp(facts))) return ownershipMoved(owner, path, "listing", "roster");
      proof.set(id, facts, settledDirectory(facts, Date.now()));
    }
  }
  for (const [path, text] of owner.texts) {
    control.signal.throwIfAborted();
    if (readManifestText(bounded(owner.root, path), owner.physical) !== text) return ownershipMoved(owner, path, "content digest", "bytes");
  }
  control.signal.throwIfAborted();
  if (readManifestText(owner.ownership.ownershipPath, undefined, limits.maximumPolicyBytes) !== owner.ownership.ownershipSource) return ownershipMoved(owner, owner.ownership.ownershipPath, "content digest", "bytes");
  proof.applyTo(observations);
  return true;
}
/** 🔎️ Names the first scope manifest, package manifest or authored pattern row on which two ownership reports differ, and whether it appeared, vanished or changed in content. */
function firstOwnershipDifference(left, right) {
  const rows = value => new Map(Array.isArray(value) ? value.map(scope => ["scope:" + scope.manifest, scope.manifest]) : [...value.scopes.map(scope => ["scope:" + scope.manifest, JSON.stringify([scope.patterns, scope.packages])]), ...value.packages.map(path => ["package:" + path, path])]), before = rows(left), after = rows(right);
  const key = [...new Set([...before.keys(), ...after.keys()])].sort().find(name => before.get(name) !== after.get(name));
  return key === undefined ? ["package.json", "content digest"] : [key.replace(/^scope:/u, "").replace(/^package:(.*)$/u, "$1/package.json"), before.has(key) && after.has(key) ? "content digest" : "existence"];
}
/** 🧮️ Rederives the selected membership, at most `maximumRederivations` times while the replayed frontier keeps moving, and refuses at once when the scopes, packages or any consumed manifest differ from the verified ones (naming the first path and fact), so unrelated concurrent Source edits stay admissible and a refusal after the bound names the first path that moved in the last replay. */
async function requireCurrentMembershipV1(owner, control, progress) {
  owner.moved = undefined;
  if (await replayCurrentMembershipV1(owner, control, progress)) return;
  const shape = value => JSON.stringify(Array.isArray(value) ? value.map(scope => scope.manifest) : { scopes: value.scopes.map(({ manifest, patterns, packages }) => ({ manifest, patterns, packages })), packages: value.packages }), baseline = new Map(owner.texts);
  for (let attempt = 1; ; attempt++) {
    const fresh = Object.assign(context(owner.root), { ownership: owner.ownership, physical: owner.physical }), report = await drainControlledPlanV1(owner.computation(fresh, owner.selection, owner.verifyPublished), nativeOperations(fresh.observations, fresh.root, fresh.physical), control, progress);
    if (shape(report) !== shape(owner.report)) { const [path, fact] = firstOwnershipDifference(report, owner.report); throw edited("Bun current workspace membership changed during verification: " + path + "; " + fact, "roster"); }
    for (const [path, text] of fresh.texts) if (baseline.has(path) && baseline.get(path) !== text) throw edited("Bun current manifest changed during verification: " + path + "; content digest");
    const missing = [...owner.texts.keys()].find(path => !fresh.texts.has(path) && path !== "package.json");
    if (missing !== undefined) throw edited("Bun current workspace manifests changed during verification: " + missing + "; existence");
    for (const [path, text] of fresh.texts) if (!baseline.has(path)) baseline.set(path, text);
    fresh.texts = new Map(baseline);
    if (await replayCurrentMembershipV1(fresh, control, progress) && authenticateCurrentMembershipV1(fresh, control)) { Object.assign(owner.observations, fresh.observations); owner.texts = fresh.texts; return; }
    const moved = fresh.moved ?? { path: owner.root, fact: "identity", reason: "identity" };
    if (attempt >= frontierReplayProtocol.maximumRederivations) throw edited("Bun current ownership changed after refreshed continuation: " + moved.path + "; " + moved.fact, moved.reason);
  }
}
/** 🛡️ Fences the last caller continuation before native publication or a completed report. */
async function currentFence(owner, operations, control, progress) {
  const {ownershipPath,ownershipSource}=owner.ownership;
  const { signal } = control;
  signal.throwIfAborted();
  if (readManifestText(ownershipPath, undefined, limits.maximumPolicyBytes) !== ownershipSource) throw edited("Bun current ownership policy changed during publication");
  if (owner.report) await requireCurrentMembershipV1(owner, control, progress);
  for (const [path, text] of owner.texts) {
    signal.throwIfAborted();
    if (operations.readText(bounded(owner.root, path)) !== text) throw edited("Bun current manifest changed during publication: " + path);
  }
}
/** ⏳️ Advances actual initial and fresh fence plans under one original signal and cumulative progress counter. */
async function drainControlledPlanV1(plan, operations, control, progress) {
  let step;
  try {
    control.signal.throwIfAborted(); step = plan.next();
    while (!step.done) {
      control.signal.throwIfAborted();
      const value = executeOperation(operations, step.value);
      await control.advance({ ...step.value, completedOperations: ++progress.completedOperations });
      control.signal.throwIfAborted();
      step = plan.next(value);
    }
    control.signal.throwIfAborted();
    return step.value;
  } finally { plan.return(); }
}
/** 🎛️ Computes complete current ownership through the original caller control. */
async function controlledOwnership(ownership, root, selection, control, verifyPublished, computation = publicationPlan) {
  const {ownershipPath,ownershipSource}=ownership;
  selection = ownershipSelection(selection);
  if (!control || !(control.signal instanceof AbortSignal) || typeof control.advance !== "function") throw Error("Original Bun workspace control required");
  if (readManifestText(ownershipPath, undefined, limits.maximumPolicyBytes) !== ownershipSource) throw edited("Bun current ownership policy changed during verification");
  if (control.frontierBatchPaths !== undefined && (!Number.isInteger(control.frontierBatchPaths) || control.frontierBatchPaths < 1 || control.frontierBatchPaths > frontierReplayProtocol.progressBatchPaths)) throw Error("Bounded Bun frontier continuation batch required");
  const owner = Object.assign(context(root), { ownership, computation, selection, verifyPublished }), operations = nativeOperations(owner.observations, owner.root, owner.physical), current = { owner, completedOperations: 0 };
  current.report = await drainControlledPlanV1(computation(owner, selection, verifyPublished), operations, control, current);
  if (readManifestText(ownershipPath, undefined, limits.maximumPolicyBytes) !== ownershipSource) throw edited("Bun current ownership policy changed during verification");
  owner.report = current.report;
  await currentFence(owner, operations, control, current);
  return current;
}
/** 🎛️ Verifies the complete exact current native projection. */
async function inspectBunWorkspaceOwnership(ownership, root, selection, control) { return (await controlledOwnership(ownership, root, selection, control, true)).report; }
/** 📣️ Publishes every affected scope before installation while preserving current Source guards. */
async function publishCurrentOwnership(ownership, root, selection, control) {
  const {ownershipPath,ownershipSource}=ownership;
  const current = await controlledOwnership(ownership, root, selection, control, false), { report, owner } = current, operations = nativeOperations(owner.observations, owner.root, owner.physical);
  const validate = async () => {
    control.signal.throwIfAborted();
    if (readManifestText(ownershipPath, undefined, limits.maximumPolicyBytes) !== ownershipSource) throw edited("Bun current ownership policy changed during publication");
    await control.advance({ operation: "membership", path: owner.root, completedOperations: ++current.completedOperations });
    control.signal.throwIfAborted();
    for (const [path, text] of owner.texts) {
      control.signal.throwIfAborted(); const target = bounded(root, path);
      if (operations.readText(target) !== text) throw edited("Bun current manifest changed during publication: " + path);
      await control.advance({ operation: "readText", path: target, completedOperations: ++current.completedOperations });
      control.signal.throwIfAborted();
      if (operations.readText(target) !== text) throw edited("Bun current manifest changed during publication: " + path);
    }
  };
  for (const scope of report.scopes) {
    const before = owner.texts.get(scope.manifest), document = owner.documents.get(scope.manifest);
    if (JSON.stringify(document.workspaces) === JSON.stringify(scope.patterns)) continue;
    await validate();
    await currentFence(owner, operations, control, current);
    const path = bounded(root, scope.manifest), after = JSON.stringify({ ...document, workspaces: scope.patterns }, null, 2) + "\n";
    fs.writeFileSync(path, after); owner.texts.set(scope.manifest, after);
    await control.advance({ operation: "publication", path, completedOperations: ++current.completedOperations });
    control.signal.throwIfAborted();
  }
  await validate();
  await currentFence(owner, operations, control, current);
  return { report, owner, recheck: async () => { await validate(); await currentFence(owner, operations, control, current); } };
}
/** 📣️ Publishes current selected native ownership without issuing authority through the report. */
async function publishBunWorkspaceOwnership(ownership, root, selection, control) { return (await publishCurrentOwnership(ownership, root, selection, control)).report; }
/** 🪢️ Holds original full Source and topology observations through the complete installation callback. */
async function withPublishedBunWorkspaceOwnershipV1(ownership, root, selection, control, body) {
  if (typeof body !== "function") throw Error("Original Bun installation callback required");
  const current = await publishCurrentOwnership(ownership, root, selection, control), scopes = Object.freeze(current.report.scopes.map(scope => Object.freeze({ manifest: scope.manifest, packages: Object.freeze([...scope.packages]) })));
  let live = true;
  const recheck = async () => { if (!live) throw Error("Original Bun installation callback released"); await current.recheck(); if (!live) throw Error("Original Bun installation callback released"); control.signal.throwIfAborted(); };
  try { const result = await body(Object.freeze({ scopes, recheck, observed: () => observedOwnership(current.owner) })); await recheck(); return result; }
  finally { live = false; }
}
/** 👓️ Retains the original verified Source observations without publishing or writing. */
async function withCurrentBunWorkspaceOwnershipV1(ownership, root, selection, control, body) {
  const {ownershipPath,ownershipSource}=ownership;
  if (typeof body !== "function") throw Error("Original Bun readonly callback required");
  const current = await controlledOwnership(ownership, root, selection, control, true), owner = current.owner, operations = nativeOperations(owner.observations, owner.root, owner.physical);
  const scopes = Object.freeze(current.report.scopes.map(scope => Object.freeze({ manifest: scope.manifest, packages: Object.freeze([...scope.packages]) }))), pending = new Set();
  let live = true, accepting = true, acceptedFailed = false, acceptedFailure;
  const guard = () => { if (!live) throw Error("Original Bun readonly callback released"); control.signal.throwIfAborted(); };
  const validate = async () => {
    guard();
    if (readManifestText(ownershipPath, undefined, limits.maximumPolicyBytes) !== ownershipSource) throw edited("Bun current ownership policy changed during verification");
    await control.advance({ operation: "membership", path: owner.root, completedOperations: ++current.completedOperations }); guard();
    for (const [path, text] of owner.texts) {
      guard(); const target = bounded(root, path);
      if (operations.readText(target) !== text) throw edited("Bun current manifest changed during verification: " + path);
      await control.advance({ operation: "readText", path: target, completedOperations: ++current.completedOperations }); guard();
      if (operations.readText(target) !== text) throw edited("Bun current manifest changed during verification: " + path);
    }
    guard(); await currentFence(owner, operations, control, current); guard();
  };
  const recheck = () => {
    if (!live || !accepting) return Promise.reject(Error("Original Bun readonly callback released"));
    if (pending.size >= limits.maximumPendingRechecks) return Promise.reject(Error("Bun readonly pending recheck capacity exceeded"));
    const task = Promise.resolve().then(validate); pending.add(task);
    void task.then(() => { pending.delete(task); }, error => { pending.delete(task); if (!acceptedFailed) { acceptedFailed = true; acceptedFailure = error; } });
    return task;
  };
  let bodyFailed = false, bodyFailure, result;
  try { result = await body(Object.freeze({ scopes, recheck, observed: () => observedOwnership(owner) })); } catch (error) { bodyFailed = true; bodyFailure = error; }
  accepting = false;
  try {
    while (pending.size) await Promise.allSettled([...pending]);
    if (bodyFailed) throw bodyFailure;
    if (acceptedFailed) throw acceptedFailure;
    await validate(); return result;
  } finally { live = false; }
}
/** 🧭️ Enumerates current declared owner manifests without scanning their package trees. */
async function discoverCurrentBunWorkspaceScopesV1(ownership, root, selection, control) {
  const computation = function* (owner, selected) {
    const scopes = yield* scopesPlan(owner, selected);
    for (const [path, text] of owner.texts) if ((yield { operation: "readText", path: bounded(owner.root, path) }) !== text) throw edited("Bun current manifest changed during verification: " + path);
    return scopes;
  };
  const current = await controlledOwnership(ownership, root, selection, control, false, computation);
  return Object.freeze(current.report.map(scope => Object.freeze({ ...scope, declaration: Object.freeze({ ...scope.declaration, members: Object.freeze([...scope.declaration.members]), owners: Object.freeze([...scope.declaration.owners]) }) })));
}
/** 🪢️ Binds captured policy DATA to one independent physical ownership implementation. */
function createBunWorkspaceOwnershipV1(value) {
  if (!value || typeof value !== "object" || Object.getPrototypeOf(value) !== Object.prototype) throw Error("Closed physical Bun policy Source required");
  const rows = Object.getOwnPropertyDescriptors(value);
  if (Reflect.ownKeys(value).length !== 2 || Object.keys(rows).sort().join() !== policySourceFields.join() || Object.values(rows).some(row => !("value" in row) || !row.enumerable)) throw Error("Closed physical Bun policy Source required");
  const ownershipPath = rows.ownershipPath.value, ownershipSource = rows.ownershipSource.value;
  if (typeof ownershipPath !== "string" || !isAbsolute(ownershipPath) || resolve(ownershipPath) !== ownershipPath || ownershipPath.length > limits.maximumPathUnits || typeof ownershipSource !== "string" || Buffer.byteLength(ownershipSource) > limits.maximumPolicyBytes) throw Error("Finite physical Bun policy Source required");
  if (readManifestText(ownershipPath, undefined, limits.maximumPolicyBytes) !== ownershipSource) throw edited("Bun captured ownership policy changed before binding");
  const ownership = Object.freeze({ ownershipPath, ownershipSource });
  return Object.freeze({...{ exportOwnershipIndexV1, seedOwnershipIndexV1, readListingProjection, withCurrentBunWorkspaceOwnershipV1, withPublishedBunWorkspaceOwnershipV1, discoverCurrentBunWorkspaceScopesV1, parseBunWorkspaceDeclaration, readBunRepositoryMembership, bunRepositoryMembership, bunWorkspacePackages, bunWorkspaceNativePatterns, inspectBunWorkspaceOwnership, publishBunWorkspaceOwnership, discoverBunWorkspaces: root => bunRepositoryMembership(root).scopes, bunRepositoryPackages: root => bunRepositoryMembership(root).packages }, withCurrentBunWorkspaceOwnershipV1:withCurrentBunWorkspaceOwnershipV1.bind(undefined,ownership),withPublishedBunWorkspaceOwnershipV1:withPublishedBunWorkspaceOwnershipV1.bind(undefined,ownership),discoverCurrentBunWorkspaceScopesV1:discoverCurrentBunWorkspaceScopesV1.bind(undefined,ownership),inspectBunWorkspaceOwnership:inspectBunWorkspaceOwnership.bind(undefined,ownership),publishBunWorkspaceOwnership:publishBunWorkspaceOwnership.bind(undefined,ownership)});
}
module.exports = {createBunWorkspaceOwnershipV1,readBunRepositoryMembership};
