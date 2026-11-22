import { AsyncLocalStorage } from "node:async_hooks";
import schema from "./🧬️schema/🔣️.json" with { type: "json" };

const POLICY = schema.definitions.Policy.const;
const scope = new AsyncLocalStorage();

/** 📜️ The name under which the generator contracts travel as a `global` row. */
export const GENERATOR_CONTRACTS_FACT_V1 = POLICY.name;

/** 🔭️ A view of the generator contracts that tells `onRead(segments, fact)` every primitive an inference reads of them (the property path and the value; `{absentMarker: true}` for an absent property), every listing of keys (`keysMarker`, the keys) and every `in` probe (`hasPrefix` + name, the answer), deep through the nested objects, and keeps them as a tree: the owner that filters the contracts by its root reads the fields its filter needs of every contract and the rest only of the contracts that concern it, so what it read is exactly what its result depends on. Identity of the nested views is stable (one view per object).
 * @see ./🧬️schema/🔣️.json */
export function trackGeneratorContractsV1(contracts, onRead = () => {}) {
  const tree = {}, views = new WeakMap();
  const nodeOf = path => { let node = tree; for (const key of path) { if (node[key] === undefined || node[key] === null || typeof node[key] !== "object") node[key] = {}; node = node[key]; } return node; };
  const note = (path, key, fact) => { nodeOf(path)[key] = fact; onRead([...path, key], fact); };
  const wrap = (target, path) => {
    const known = views.get(target);
    if (known !== undefined) return known;
    const view = new Proxy(target, {
      get(object, key, receiver) {
        const value = Reflect.get(object, key, receiver);
        if (typeof key !== "string" || typeof value === "function") return value;
        if (value !== null && typeof value === "object") return wrap(value, [...path, key]);
        note(path, key, value === undefined ? { [POLICY.absentMarker]: true } : value);
        return value;
      },
      has(object, key) { const held = Reflect.has(object, key); if (typeof key === "string") note(path, POLICY.hasPrefix + key, held); return held; },
      ownKeys(object) { const keys = Reflect.ownKeys(object); note(path, POLICY.keysMarker, keys.filter(key => typeof key === "string")); return keys; },
    });
    views.set(target, view);
    return view;
  };
  return Object.freeze({ view: wrap(contracts, []), tree: () => tree });
}

/** ✅️ Whether one read (the property path `segments` of the contracts and the `fact` that was read there) has the same answer in `contracts`.
 * @see ./🧪️tests/🟦️.ts */
export function generatorContractReadHoldsV1(contracts, segments, fact) {
  if (!Array.isArray(segments) || segments.length === 0) return false;
  let target = contracts;
  for (const key of segments.slice(0, -1)) { if (target === null || typeof target !== "object") return false; target = target[key]; }
  if (target === null || typeof target !== "object") return false;
  const last = segments.at(-1);
  if (last === POLICY.keysMarker) return JSON.stringify(Reflect.ownKeys(target).filter(item => typeof item === "string")) === JSON.stringify(fact);
  if (last.startsWith(POLICY.hasPrefix)) return Reflect.has(target, last.slice(POLICY.hasPrefix.length)) === fact;
  const actual = target[last];
  return fact !== null && typeof fact === "object" && POLICY.absentMarker in fact ? actual === undefined : actual === fact;
}

/** ✅️ Whether everything an inference read of the contracts (a ReadsTree) has the same answer in `contracts`. */
export function generatorContractReadsHoldV1(contracts, tree) {
  const leaves = (node, path) => Object.entries(node).flatMap(([key, value]) => value !== null && typeof value === "object" && !Array.isArray(value) && !(POLICY.absentMarker in value) ? leaves(value, [...path, key]) : [[[...path, key], value]]);
  return tree !== null && typeof tree === "object" && leaves(tree, []).every(([segments, fact]) => generatorContractReadHoldsV1(contracts, segments, fact));
}

/** 🧭️ Runs `work` with `contracts` as the contracts that generated-output lookups of this asynchronous scope consult. */
export function withGeneratorContractsScopeV1(contracts, work) { return scope.run(contracts, work); }

/** 🧭️ The contracts of the running scope, `fallback` outside any. */
export function generatorContractsInScopeV1(fallback) { return scope.getStore() ?? fallback; }

/** 🔑️ The head of an owner record key: the facts that are shared by every owner of a read and that no row carries (the registry providers, the Cargo command inputs of a native owner, the native or pipeline closure of the owner), then the reference format; nothing about the generator contracts (their reads are a row) and nothing about the kind of read (a whole read and a selected read of one owner key it alike).
 * @see ./🧪️tests/🟦️.ts */
export function ownerRecordHeadV1({ registryDigest, commandRevision, late }, { native, pipeline, assemblyAuthority }) {
  return [registryDigest, native ? commandRevision : null, native || pipeline ? assemblyAuthority : null, ...(late ? [POLICY.referencesMarker] : [])];
}
