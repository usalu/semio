/** 🔬 Compare portable cache traces with both first-party TypeScript and independent installed LRU oracles. */
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const { pathToFileURL } = require("node:url");
const crypto = require("node:crypto");
const { test, after } = require("node:test");
const base = path.resolve(__dirname, "..");
const cases = JSON.parse(fs.readFileSync(path.join(base, "🧪️cases/🔣️.json"), "utf8"));
const oracleIdentities = {};
function dependency(name, version) {
  const load = candidate => {
    const entry = require.resolve(candidate);
    let directory = path.dirname(entry);
    for (let depth = 0; depth < 10; depth++) {
      const manifest = path.join(directory, "package.json");
      if (fs.existsSync(manifest)) {
        const raw = fs.readFileSync(manifest), data = JSON.parse(raw);
        if (data.name === name) {
          assert.equal(data.version, version);
          oracleIdentities[name] = { version, entry, entry_sha256: crypto.createHash("sha256").update(fs.readFileSync(entry)).digest("hex"), manifest_sha256: crypto.createHash("sha256").update(raw).digest("hex") };
          return require(entry);
        }
      }
      const parent = path.dirname(directory); if (parent === directory) break; directory = parent;
    }
    throw Error("Test oracle package identity missing");
  };
  try { return load(name); } catch {}
  let current = base;
  for (let depth = 0; depth < 10; depth++) {
    const candidate = path.join(current, "node_modules", ".bun", name + "@" + version, "node_modules", name);
    try { return load(candidate); } catch {}
    const parent = path.dirname(current);
    if (parent === current) break;
    current = parent;
  }
  throw Error("Required installed test oracle unavailable: " + name);
}
const load = () => import(pathToFileURL(path.join(base, "🟦️.ts")).href);
const trace = [];
after(() => { const index = process.argv.indexOf("--record"); if (index >= 0) { fs.writeFileSync(process.argv[index + 1], JSON.stringify(trace, null, 2) + "\n"); fs.writeFileSync(path.join(path.dirname(process.argv[index + 1]), "oracle-identities.json"), JSON.stringify(oracleIdentities, null, 2) + "\n"); } });
test("portable schema and deliberately malformed cases", () => {
  const Ajv = dependency("ajv", "8.20.0");
  const validate = new Ajv({ strict: false }).compile(JSON.parse(fs.readFileSync(path.join(base, "🔣️.schema.json"), "utf8")));
  assert.equal(validate(cases), true, JSON.stringify(validate.errors));
  for (const mutate of [c => c.cases[0].policy[0] = true, c => c.cases[0].steps[0].cost[0] = -1, c => c.cases[0].steps[0].extra = 1, c => delete c.cases[0].steps[0].key]) {
    const bad = structuredClone(cases); mutate(bad); assert.equal(validate(bad), false);
  }
});
test("TypeScript executes every portable trace", async () => {
  const C = await load();
  for (const row of cases.cases) {
    const cache = new C.TemplateCache(new C.CachePolicy(...row.policy));
    for (const step of row.steps) {
      if (step.op === "request") {
        const value = { key: step.key };
        const factory = () => { if (!step.reserve) assert.equal(cache.stats().entries, 0); if (step.fail === "factory") throw Error("factory"); return value; };
        const measure = () => { if (step.fail === "measure") throw Error("measure"); return new C.CacheCost(...step.cost); };
        const clone = item => { if (step.fail === "clone") throw Error("clone"); return { ...item }; };
        const options = { valid: () => !step.invalid, cancelled: () => step.fail === "cancel", ...(step.reserve ? { reservation: new C.CacheCost(...step.reserve) } : {}) };
        if (step.outcome === "error") assert.throws(() => cache.resolve(step.key, factory, measure, clone, options));
        else assert.equal(cache.resolve(step.key, factory, measure, clone, options) === value, step.outcome === "original");
      } else if (step.op === "release") cache.release();
      else if (step.op === "invalidate") cache.invalidate(step.key);
      else cache.configure(new C.CachePolicy(...step.policy));
      const stats = cache.stats();
      assert.deepEqual(stats.keys, step.keys, row.name);
      assert.deepEqual([stats.members, stats.points], step.total);
      assert.equal(stats.reserved_members + stats.reserved_points, 0);
      trace.push({ case: row.name, ...stats });
    }
  }
});
test("installed LRU independently validates mixed member and point retention", () => {
  const { LRUCache } = dependency("lru-cache", "11.5.3");
  const row = cases.cases.find(row => row.name === "Independent Budgets And LRU");
  let policy = row.policy;
  let caches;
  const create = () => [1, 2].map((field, i) => new LRUCache({ max: policy[0], maxSize: policy[field], sizeCalculation: value => value[i] }));
  caches = create();
  for (const step of row.steps) {
    if (step.op === "request") for (const oracle of caches) { if (oracle.has(step.key)) oracle.get(step.key); else oracle.set(step.key, step.cost); }
    else if (step.op === "release") for (const oracle of caches) oracle.clear();
    else if (step.op === "configure") {
      const values = [...caches[0].entries()].reverse(); policy = step.policy; caches = create();
      for (const [key, value] of values) for (const oracle of caches) oracle.set(key, value);
    }
    for (const key of new Set(caches.flatMap(oracle => [...oracle.keys()]))) if (!caches.every(oracle => oracle.has(key))) for (const oracle of caches) oracle.delete(key);
    assert.deepEqual([...caches[0].keys()].reverse(), step.keys);
    assert.deepEqual(caches.map(oracle => oracle.calculatedSize), step.total);
  }
});
test("TypeScript policy validation and explicit scope ownership", async () => {
  const C = await load();
  for (const value of [...cases.invalid_numbers, Infinity, NaN]) {
    assert.throws(() => new C.CacheCost(value, 1));
    assert.throws(() => new C.CachePolicy(value, 1, 1, 1));
  }
  const owner = new C.CacheOwner(), outer = new C.TemplateCache();
  outer.resolve("outer", () => ({}), () => new C.CacheCost(1, 1), value => ({ ...value }));
  for (const scenario of cases.ownership) owner.run(() => {
    assert.equal(owner.current, outer);
    let inner;
    const body = () => owner.run(cache => { inner = cache; cache.resolve("a", () => ({}), () => new C.CacheCost(1, 1), value => ({ ...value })); if (scenario === "nested_failure") throw Error("scope"); });
    if (scenario === "borrowed") owner.run(cache => assert.equal(cache, outer), { cache: outer });
    else { if (scenario === "nested_failure") assert.throws(body); else body(); assert.equal(inner.stats().entries, 0); }
    assert.equal(owner.current, outer);
  }, { cache: outer });
  assert.equal(owner.current, undefined);
  assert.equal(outer.stats().entries, 1);
  assert.throws(() => owner.run(() => ({ then() {} })));
  assert.equal(owner.current, undefined);
});
test("TypeScript bounded history, pre-factory release, failure and mutation control", async () => {
  const C = await load();
  const cache = new C.TemplateCache(new C.CachePolicy(8, 100, 100, 2));
  for (let index = 0; index < 9; index++) cache.resolve(String(index), () => ({}), () => new C.CacheCost(1, 1), value => ({ ...value }), { reservation: new C.CacheCost(1, 1) });
  assert.deepEqual(cache.stats().keys, ["1", "2", "3", "4", "5", "6", "7", "8"]);
  assert.deepEqual(cache.stats().history_keys, ["7", "8"]);
  cache.configure(new C.CachePolicy(8, 2, 2, 2));
  cache.resolve("7", () => ({}), () => new C.CacheCost(1, 1), value => ({ ...value }));
  assert.deepEqual(cache.stats().keys, ["8", "7"]);
  cache.invalidate("7");
  cache.resolve("7", () => { assert.equal(cache.stats().entries, 0); return {}; }, () => new C.CacheCost(1, 1), value => ({ ...value }));
  cache.release();
  for (const stage of ["measure", "clone", "late_cancel", "reentry"]) {
    let calls = 0;
    assert.throws(() => cache.resolve(stage, () => ({}), () => { if (stage === "measure") return [1, 1]; if (stage === "reentry") cache.release(); return new C.CacheCost(1, 1); }, () => { if (stage === "clone") throw Error("clone"); return {}; }, { cancelled: () => ++calls === 5 && stage === "late_cancel" }));
    assert.equal(cache.stats().entries, 0);
    assert.equal(cache.stats().reserved_members, 0);
  }
  for (const key of [undefined, true, 2, "", "x".repeat(4097)]) assert.throws(() => cache.resolve(key, () => ({}), () => new C.CacheCost(1, 1), value => value));
});
