/** 🪢️ Loads, validates and wires one revision of a component graph: every component of a revision is imported under the same revision token and receives its dependencies from a registry instead of importing its siblings, so a reload can never mix stale and current components. */

const REVISION = /^[a-f0-9]{64}$/u;
const NAME = /^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$/u;
const PATH = /^\.\/(?:[^/\\?#.][^/\\?#]*\/)*[^/\\?#.][^/\\?#]*$/u;
const wired = new WeakSet();

/** 🔏️ Admits one revision token before it can name a module graph. */
function requireRevision(revision) {
  if (typeof revision !== "string" || !REVISION.test(revision)) throw Error("Revision graph requires a lowercase sha256 revision");
  return revision;
}

/** 🧾️ Admits the authored component roster: unique names, portable relative paths and needs that only name earlier components, which makes the graph acyclic by construction. */
export function readComponentRoster(manifest) {
  if (manifest?.version !== 1 || !Array.isArray(manifest.components) || !manifest.components.length) throw Error("Component roster version 1 with components required");
  const order = new Map();
  const roster = manifest.components.map((entry, index) => {
    if (!entry || typeof entry !== "object" || !NAME.test(entry.name) || typeof entry.path !== "string" || !PATH.test(entry.path) || !Array.isArray(entry.needs)) throw Error(`Invalid component roster entry ${index}`);
    if (order.has(entry.name)) throw Error(`Duplicate component ${entry.name}`);
    if (new Set(entry.needs).size !== entry.needs.length) throw Error(`Component ${entry.name} repeats a need`);
    for (const need of entry.needs) if (!order.has(need)) throw Error(`Component ${entry.name} needs ${need}, which is not an earlier component`);
    order.set(entry.name, index);
    return Object.freeze({ name: entry.name, path: entry.path, needs: Object.freeze([...entry.needs]) });
  });
  return Object.freeze(roster);
}

/** 🔑️ Names the registry property of one component. */
export function registryKey(name) {
  return name.replace(/-([a-z0-9])/gu, (_, character) => character.toUpperCase());
}

/** 🪪️ Pins the module URL and the revision a graph was built for; components read the revision only after it is bound. */
export function createRevisionIdentity(url) {
  let revision;
  return Object.freeze({
    url,
    get revision() { if (revision === undefined) throw Error("Revision identity is not bound yet"); return revision; },
    bind(value) { requireRevision(value); if (revision !== undefined && revision !== value) throw Error("Revision identity is already bound"); revision = value; },
  });
}

/** 📥️ Imports every component of a roster under one revision token; `load` must resolve relative specifiers against the roster's own directory. */
export async function importRevisionedComponents({ roster, revision, load }) {
  requireRevision(revision);
  const modules = await Promise.all(roster.map((entry) => load(`${entry.path}?revision=${revision}`)));
  return Object.freeze(Object.fromEntries(roster.map((entry, index) => [entry.name, modules[index]])));
}

/** 🔌️ Wires components in roster order, each seeing only the registry entries it declares as needs plus the shared services; a graph that failed to wire completely leaves its components free to be wired again. */
export function wireRevisionedComponents({ roster, modules, services, revisionIdentity, register }) {
  if (Object.keys(modules).length !== roster.length) throw Error("Component modules differ from the roster");
  for (const entry of roster) {
    const module = modules[entry.name];
    if (!module || module.component !== entry.name || typeof module.wire !== "function") throw Error(`Component ${entry.name} does not match its roster entry`);
    if (wired.has(module)) throw Error(`Component ${entry.name} is already wired into another revision`);
  }
  for (const entry of roster) {
    const visible = { services, revisionIdentity, register };
    for (const need of entry.needs) visible[registryKey(need)] = modules[need];
    modules[entry.name].wire(Object.freeze(visible));
  }
  for (const entry of roster) wired.add(modules[entry.name]);
  return Object.freeze({ modules });
}

/** ♻️ Caches one wired graph per revision so a revisited revision reuses its components and never registers its analyses twice; a failed load is forgotten. */
export function createRevisionLoader({ roster, load, services, register, url }) {
  const scopes = new Map();
  return Object.freeze({
    adopt(revision, scope) {
      requireRevision(revision);
      if (scopes.has(revision)) throw Error("Revision graph is already loaded");
      scopes.set(revision, Promise.resolve(scope));
    },
    load(revision) {
      requireRevision(revision);
      let entry = scopes.get(revision);
      if (!entry) {
        entry = importRevisionedComponents({ roster, revision, load }).then((modules) => {
          const revisionIdentity = createRevisionIdentity(url);
          revisionIdentity.bind(revision);
          return wireRevisionedComponents({ roster, modules, services, revisionIdentity, register });
        });
        scopes.set(revision, entry);
        entry.catch(() => { if (scopes.get(revision) === entry) scopes.delete(revision); });
      }
      return entry;
    },
  });
}
