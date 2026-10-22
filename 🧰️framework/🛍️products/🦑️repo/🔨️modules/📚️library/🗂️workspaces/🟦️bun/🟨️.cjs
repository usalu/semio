const fs = require("node:fs");
const { dirname, isAbsolute, join, relative, resolve } = require("node:path");
const { ownsPayload } = require("../📦️payload/🟨️.cjs");
const { createHash } = require("node:crypto");
const limits = Object.freeze({ ...require("./🧬️schema/🔣️.json").$defs.PhysicalLimits.const });
const slash = (path) => path.replaceAll("\\", "/");
const policySourceFields = Object.freeze([...require("./🧬️schema/🔣️.json").$defs.PhysicalPolicySource.required].sort());
const {workspaceTestingCollectionRoot: collectionRoot} = require("../🟨️.cjs");
const opaque = new Set(["🗑️generated", "🤖️generated", "node_modules", "target", "dist", "📤️dist", "build", "storybook-static", "temp", "coverage", "__pycache__", "🔌️plugin-modules", "compose"]);
const outputName = name => opaque.has(name) || /^target(?:-[a-z0-9]+)*$/u.test(name);
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
const physicalObservations = () => ({ paths: new Map(), order: [], frontier: new Map() });
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
    if (!sameManifest(before, fs.fstatSync(descriptor, { bigint: true }))) throw Error("Bun current manifest changed during descriptor admission: " + path);
    const bytes = Buffer.alloc(size), probe = Buffer.alloc(1); let offset = 0;
    while (offset < size) { const count = fs.readSync(descriptor, bytes, offset, Math.min(size - offset, limits.readChunkBytes), offset); if (!count) throw Error("Bun current manifest changed during descriptor read: " + path); offset += count; }
    if (fs.readSync(descriptor, probe, 0, 1, size) || !sameManifest(before, fs.fstatSync(descriptor, { bigint: true })) || !sameManifest(before, fs.lstatSync(path, { bigint: true }))) throw Error("Bun current manifest changed during descriptor read: " + path);
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
  throw Error("Explicit Bun ownership selection required");
}
function* documentPlan(owner, path) {
  if (!owner.documents.has(path)) {
    const text = yield { operation: "readText", path: bounded(owner.root, path) };
    owner.texts.set(path, text); owner.documents.set(path, JSON.parse(text));
  }
  return owner.documents.get(path);
}
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
  const result = { paths, collections }; owner.inventory.set(directory, result); return result;
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
  for (const pattern of selection.kind === "all" ? declaration.owners : []) {
    if (pattern !== "*/package.json") throw Error("Unsupported physical owner recipe: " + pattern);
    const entries = yield { operation: "list", path: owner.root };
    for (const entry of entries) if (!entry.name.startsWith(".") && !outputName(entry.name) && !collectionRoot(owner.root, join(owner.root, entry.name))) {
      if (entry.kind === "symlink") throw Error("Bun installation owner is a symlink: " + entry.name);
      if (entry.kind === "directory" && (yield { operation: "state", path: join(owner.root, entry.name, "package.json") }) !== "missing") paths.add(entry.name + "/package.json");
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
  return scopes;
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
  let record = observations.paths.get(path);
  const previous = record?.[operation];
  if (previous !== undefined) {
    if (JSON.stringify(previous) !== JSON.stringify(value)) throw Error("Bun current ownership changed during verification: " + path + "; change=" + JSON.stringify(ownershipChange(operation, path, previous, value)));
    return;
  }
  if (operation === "list" && record?.state[0] !== "directory") throw Error("Original Bun directory state observation required");
  if (!record && observations.paths.size >= limits.maximumObservationPaths) throw Error("Bun ownership physical path observation capacity exceeded");
  if (observations.order.length >= limits.maximumObservationFacts) throw Error("Bun ownership observation fact capacity exceeded");
  if (operation === "list") value.forEach(Object.freeze);
  Object.freeze(value);
  if (!record) { record = Object.seal({ path, state: value, list: undefined }); observations.paths.set(path, record); }
  else record.list = value;
  observations.order.push(Object.freeze({ record, operation }));
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
/** 📋️ Lists one directory exactly as the ownership walk does and returns its projection. */
function readListingProjection(root, path) {
  const entries = fs.readdirSync(path, { withFileTypes: true }).map(entry => ({ name: entry.name, kind: kindOf(entry) }));
  entries.sort(byName);
  return projectListing(root, path, entries);
}
/** 📤️ Exports the settled directory listings this process retained under one workspace, so a later process can reuse them under their stamps. */
function exportOwnershipIndexV1(root) {
  const rows = [];
  for (const [path, record] of listingCache) if (record.root === root) rows.push({ path, stamp: record.stamp, count: record.count ?? record.entries.length, entries: (record.projected ??= projectListing(root, path, record.entries)).map(({ name, kind }) => ({ name, kind })) });
  return rows;
}
/** 📥️ Seeds settled directory listings persisted by an earlier process; each is trusted only while the live directory carries exactly its stamp. */
function seedOwnershipIndexV1(root, records) {
  for (const record of records) {
    if (listingCache.size >= limits.maximumListingCacheDirectories) break;
    if (!listingCache.has(record.path)) listingCache.set(record.path, { stamp: record.stamp, root, entries: record.entries, count: record.count, projected: record.entries });
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
    } catch (error) { if (error.code === "ENOENT") { retain("state", path, ["missing"]); return { kind: "missing" }; } throw error; }
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
      let record = cached?.stamp === stamp && cached.root === root ? cached : undefined;
      if (!record) {
        const entries = [], directory = fs.opendirSync(path, { bufferSize: limits.directoryReadBatch }); let failed = false;
        try { for (let entry; (entry = directory.readSync()) !== null;) { if (entries.length >= limits.maximumDirectoryEntries || budget.entries - prior + entries.length + 1 > limits.maximumAggregateDirectoryEntries) throw Error("Bun directory observation capacity exceeded: " + path); entries.push({ name: entry.name, kind: kindOf(entry) }); } }
        catch (error) { failed = true; throw error; } finally { try { directory.closeSync(); } catch (error) { if (!failed) throw error; } }
        entries.sort(byName);
        const after = observe(path);
        if (after.kind !== "directory") throw Error("Bun workspace requires a real directory: " + path);
        record = { stamp, root, entries, count: entries.length, projected: undefined };
        if (directoryStamp(after.entry) === stamp && settledDirectory(after.entry, listedAt)) { if (listingCache.size >= limits.maximumListingCacheDirectories) listingCache.clear(); listingCache.set(path, record); }
        else listingCache.delete(path);
      } else if ((record.count ?? record.entries.length) > limits.maximumDirectoryEntries || budget.entries - prior + (record.count ?? record.entries.length) > limits.maximumAggregateDirectoryEntries) throw Error("Bun directory observation capacity exceeded: " + path);
      budget.entries = budget.entries - prior + (record.count ?? record.entries.length); budget.lists.set(path, record.count ?? record.entries.length);
      if (observations) {
        retain("list", path, record.projected ??= projectListing(root, path, record.entries));
        const after = fs.lstatSync(path, { bigint: true });
        if (after.isDirectory() && directoryStamp(after) === stamp) observations.frontier.set(path, { stamp, settled: settledDirectory(after, listedAt) });
        else observations.frontier.delete(path);
      }
      return record.entries;
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
  for (const [path, text] of owner.texts) if ((yield { operation: "readText", path: bounded(owner.root, path) }) !== text) throw Error("Bun current manifest changed during verification: " + path);
  return { version: 1, scopes, packages: membership.packages };
}
/** 🔭️ Exposes the exact path states, directory listing projections and manifest digests one ownership computation retained, so a later process can prove the same topology by stamps. */
function observedOwnership(owner) {
  const paths = [...owner.observations.paths.values()].map(record => Object.freeze({ path: record.path, state: record.state, ...(record.list ? { list: record.list } : {}) })), texts = [...owner.texts].map(([path, text]) => Object.freeze({ path, sha256: createHash("sha256").update(text).digest("hex") }));
  return Object.freeze({ root: owner.root, paths: Object.freeze(paths), texts: Object.freeze(texts) });
}
/** ⚡️ Replays the complete original decision frontier and fences the final awaited continuation without rediscovering packages. */
async function replayCurrentMembershipV1(owner, control, progress) {
  let continuationFailed = false;
  const observations = owner.observations, proof = new Map(), operations = nativeOperations(undefined, owner.root, owner.physical), read = path => {
    try { return fs.lstatSync(path, { bigint: true }); } catch (error) { if (error.code === "ENOENT") return undefined; throw error; }
  };
  try {
    if (!observations?.paths.size || !observations.frontier) return false;
    for (const record of observations.paths.values()) {
      control.signal.throwIfAborted();
      const facts = read(record.path);
      if (JSON.stringify(ownershipStateV1(facts)) !== JSON.stringify(record.state)) return false;
      if (record.list) {
        const before = observations.frontier.get(record.path), stamp = facts?.isDirectory() ? directoryStamp(facts) : null;
        const decision = ownershipFrontierDecisionV1(before?.stamp, stamp, before?.settled && facts && settledDirectory(facts, Date.now()));
        if (decision === "derive") return false;
        if (decision === "relist" && (JSON.stringify(projectListing(owner.root, record.path, operations.list(record.path))) !== JSON.stringify(record.list) || directoryStamp(read(record.path)) !== stamp)) return false;
        proof.set(record.path, { stamp, exact: decision === "relist" });
      }
      try { await control.advance({ operation: "frontier", path: record.path, completedOperations: ++progress.completedOperations }); } catch (error) { continuationFailed = true; throw error; }
      control.signal.throwIfAborted();
    }
    if (readManifestText(owner.ownership.ownershipPath, undefined, limits.maximumPolicyBytes) !== owner.ownership.ownershipSource) return false;
    for (const [path, text] of owner.texts) {
      control.signal.throwIfAborted();
      if (readManifestText(bounded(owner.root, path), owner.physical) !== text) return false;
    }
    for (const record of observations.paths.values()) {
      control.signal.throwIfAborted(); const facts = read(record.path);
      if (JSON.stringify(ownershipStateV1(facts)) !== JSON.stringify(record.state)) return false;
      if (record.list) {
        const before = proof.get(record.path); if (!facts?.isDirectory() || !before) return false;
        const stamp = directoryStamp(facts);
        if ((before.exact || before.stamp !== stamp) && (JSON.stringify(projectListing(owner.root, record.path, operations.list(record.path))) !== JSON.stringify(record.list) || directoryStamp(read(record.path)) !== stamp)) return false;
        proof.set(record.path, { stamp, settled: settledDirectory(facts, Date.now()) });
      }
    }
    if (readManifestText(owner.ownership.ownershipPath, undefined, limits.maximumPolicyBytes) !== owner.ownership.ownershipSource) return false;
    for (const [path, observation] of proof) observations.frontier.set(path, { stamp: observation.stamp, settled: observation.settled });
    return true;
  } catch (error) { control.signal.throwIfAborted(); if (continuationFailed) throw error; return false; }
}
/** 🧷️ Authenticates the complete candidate after the caller continuation without another await. */
function authenticateCurrentMembershipV1(owner, control) {
  const observations = owner.observations, operations = nativeOperations(undefined, owner.root, owner.physical), proof = new Map(), read = path => {
    try { return fs.lstatSync(path, { bigint: true }); } catch (error) { if (error.code === "ENOENT") return undefined; throw error; }
  };
  if (!observations?.paths.size || !observations.frontier) return false;
  if (readManifestText(owner.ownership.ownershipPath, undefined, limits.maximumPolicyBytes) !== owner.ownership.ownershipSource) return false;
  for (const record of observations.paths.values()) {
    control.signal.throwIfAborted();
    const facts = read(record.path);
    if (JSON.stringify(ownershipStateV1(facts)) !== JSON.stringify(record.state)) return false;
    if (record.list) {
      if (!facts?.isDirectory()) return false;
      const before = observations.frontier.get(record.path), stamp = directoryStamp(facts), decision = ownershipFrontierDecisionV1(before?.stamp, stamp, before?.settled && settledDirectory(facts, Date.now()));
      if (decision === "derive") return false;
      if (decision === "relist" && (JSON.stringify(projectListing(owner.root, record.path, operations.list(record.path))) !== JSON.stringify(record.list) || directoryStamp(read(record.path)) !== stamp)) return false;
      proof.set(record.path, { stamp, settled: settledDirectory(facts, Date.now()) });
    }
  }
  for (const [path, text] of owner.texts) {
    control.signal.throwIfAborted();
    if (readManifestText(bounded(owner.root, path), owner.physical) !== text) return false;
  }
  control.signal.throwIfAborted();
  if (readManifestText(owner.ownership.ownershipPath, undefined, limits.maximumPolicyBytes) !== owner.ownership.ownershipSource) return false;
  for (const [path, observation] of proof) observations.frontier.set(path, observation);
  return true;
}
/** 🧮️ Rederives the selected membership and refuses only when scopes, packages or manifests drift, so unrelated concurrent Source edits stay admissible. */
async function requireCurrentMembershipV1(owner, control, progress) {
  if (await replayCurrentMembershipV1(owner, control, progress)) return;
  const fresh = Object.assign(context(owner.root), { ownership: owner.ownership, physical: owner.physical }), report = await drainControlledPlanV1(owner.computation(fresh, owner.selection, owner.verifyPublished), nativeOperations(fresh.observations, fresh.root, fresh.physical), control, progress);
  const shape = value => JSON.stringify(Array.isArray(value) ? value.map(scope => scope.manifest) : { scopes: value.scopes.map(({ manifest, patterns, packages }) => ({ manifest, patterns, packages })), packages: value.packages });
  if (shape(report) !== shape(owner.report)) throw Error("Bun current workspace membership changed during verification");
  for (const [path, text] of fresh.texts) if (owner.texts.has(path) && owner.texts.get(path) !== text) throw Error("Bun current manifest changed during verification: " + path);
  if ([...owner.texts.keys()].some(path => !fresh.texts.has(path) && path !== "package.json")) throw Error("Bun current workspace manifests changed during verification");
  fresh.texts = new Map([...owner.texts, ...fresh.texts]);
  if (!await replayCurrentMembershipV1(fresh, control, progress) || !authenticateCurrentMembershipV1(fresh, control)) throw Error("Bun current ownership changed after refreshed continuation");
  Object.assign(owner.observations, fresh.observations);
  owner.texts = fresh.texts;
}
/** 🛡️ Fences the last caller continuation before native publication or a completed report. */
async function currentFence(owner, operations, control, progress) {
  const {ownershipPath,ownershipSource}=owner.ownership;
  const { signal } = control;
  signal.throwIfAborted();
  if (readManifestText(ownershipPath, undefined, limits.maximumPolicyBytes) !== ownershipSource) throw Error("Bun current ownership policy changed during publication");
  if (owner.report) await requireCurrentMembershipV1(owner, control, progress);
  for (const [path, text] of owner.texts) {
    signal.throwIfAborted();
    if (operations.readText(bounded(owner.root, path)) !== text) throw Error("Bun current manifest changed during publication: " + path);
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
  if (readManifestText(ownershipPath, undefined, limits.maximumPolicyBytes) !== ownershipSource) throw Error("Bun current ownership policy changed during verification");
  const owner = Object.assign(context(root), { ownership, computation, selection, verifyPublished }), operations = nativeOperations(owner.observations, owner.root, owner.physical), current = { owner, completedOperations: 0 };
  current.report = await drainControlledPlanV1(computation(owner, selection, verifyPublished), operations, control, current);
  if (readManifestText(ownershipPath, undefined, limits.maximumPolicyBytes) !== ownershipSource) throw Error("Bun current ownership policy changed during verification");
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
    if (readManifestText(ownershipPath, undefined, limits.maximumPolicyBytes) !== ownershipSource) throw Error("Bun current ownership policy changed during publication");
    await requireCurrentMembershipV1(owner, control, current);
    await control.advance({ operation: "membership", path: owner.root, completedOperations: ++current.completedOperations });
    control.signal.throwIfAborted();
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
    if (readManifestText(ownershipPath, undefined, limits.maximumPolicyBytes) !== ownershipSource) throw Error("Bun current ownership policy changed during verification");
    await requireCurrentMembershipV1(owner, control, current);
    await control.advance({ operation: "membership", path: owner.root, completedOperations: ++current.completedOperations }); guard();
    for (const [path, text] of owner.texts) {
      guard(); const target = bounded(root, path);
      if (operations.readText(target) !== text) throw Error("Bun current manifest changed during verification: " + path);
      await control.advance({ operation: "readText", path: target, completedOperations: ++current.completedOperations }); guard();
      if (operations.readText(target) !== text) throw Error("Bun current manifest changed during verification: " + path);
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
    for (const [path, text] of owner.texts) if ((yield { operation: "readText", path: bounded(owner.root, path) }) !== text) throw Error("Bun current manifest changed during verification: " + path);
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
  if (readManifestText(ownershipPath, undefined, limits.maximumPolicyBytes) !== ownershipSource) throw Error("Bun captured ownership policy changed before binding");
  const ownership = Object.freeze({ ownershipPath, ownershipSource });
  return Object.freeze({...{ exportOwnershipIndexV1, seedOwnershipIndexV1, readListingProjection, withCurrentBunWorkspaceOwnershipV1, withPublishedBunWorkspaceOwnershipV1, discoverCurrentBunWorkspaceScopesV1, parseBunWorkspaceDeclaration, readBunRepositoryMembership, bunRepositoryMembership, bunWorkspacePackages, bunWorkspaceNativePatterns, inspectBunWorkspaceOwnership, publishBunWorkspaceOwnership, discoverBunWorkspaces: root => bunRepositoryMembership(root).scopes, bunRepositoryPackages: root => bunRepositoryMembership(root).packages }, withCurrentBunWorkspaceOwnershipV1:withCurrentBunWorkspaceOwnershipV1.bind(undefined,ownership),withPublishedBunWorkspaceOwnershipV1:withPublishedBunWorkspaceOwnershipV1.bind(undefined,ownership),discoverCurrentBunWorkspaceScopesV1:discoverCurrentBunWorkspaceScopesV1.bind(undefined,ownership),inspectBunWorkspaceOwnership:inspectBunWorkspaceOwnership.bind(undefined,ownership),publishBunWorkspaceOwnership:publishBunWorkspaceOwnership.bind(undefined,ownership)});
}
module.exports = {createBunWorkspaceOwnershipV1,readBunRepositoryMembership};
