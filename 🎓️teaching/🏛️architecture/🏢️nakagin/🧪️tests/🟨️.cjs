/** 🧬 Independent AJV schema and count oracle for the portable Nakagin teaching model. */
const fs = require("node:fs");
const path = require("node:path");
const assert = require("node:assert/strict");
const Ajv = require("ajv/dist/2020").default;
const root = path.resolve(__dirname, "..");
const read = file => JSON.parse(fs.readFileSync(path.join(root, file), "utf8"));
const schema = read("🧬️schema/🔣️.schema.json");
const model = read("🔣️.json");
const cases = read("🧪️cases/🔣️.json");

const validate = new Ajv({strict: true, allErrors: true}).compile(schema);
assert.equal(validate(model), true, JSON.stringify(validate.errors));
for (const test of cases.invalid_models) {
  const candidate = structuredClone(model);
  const parent = test.path.slice(0, -1).reduce((value, step) => {
    assert.ok(value !== undefined && value !== null && step in Object(value), test.id + ": path step " + step + " exists");
    return value[step];
  }, candidate);
  const key = test.path.at(-1);
  if (test.remove) {
    assert.ok(key in Object(parent), test.id + ": removed key exists");
    if (Array.isArray(parent)) parent.splice(key, 1); else delete parent[key];
  } else {
    assert.ok(test.add ? !(key in Object(parent)) : key in Object(parent), test.id + ": mutated key " + (test.add ? "is new" : "exists"));
    parent[key] = test.value;
  }
  assert.equal(validate(candidate), false, test.id);
}
console.log("Nakagin schema: authored model accepted; " + cases.invalid_models.length + " invalid models rejected.");

const sourceKeys = new Set(Object.keys(model.sources));
const leaves = (node, out = []) => {
  if (node && typeof node === "object") {
    if ("value" in node && "unit" in node && "basis" in node) out.push(node);
    else Object.values(node).forEach(child => leaves(child, out));
  }
  return out;
};
const all = leaves({...model, sources: undefined});
assert.ok(all.length > 100);
for (const leaf of all) if ("source" in leaf) assert.ok(sourceKeys.has(leaf.source), "unknown source " + leaf.source);
console.log("Nakagin leaves: " + all.length + " values, every source key listed.");

const perFacade = model.items.per_facade.value;
const branches = {};
const typeCounts = {};
const order = model.assignment.order;
const a = model.assignment;
model.towers.forEach((tower, ti) => {
  let n = 0;
  for (let s = 0; s < tower.capsule_storeys.value; s++) {
    const exception = tower.exceptions.find(e => e.storey === s);
    for (const f of exception ? exception.facades : tower.facades) {
      n++;
      for (let i = 0; i < perFacade; i++) {
        const index = a.tower_weight.value * ti + a.storey_weight.value * s + a.facade_weight.value * f + a.item_weight.value * i + a.offset.value;
        const key = order[index % order.length];
        typeCounts[key] = (typeCounts[key] || 0) + 1;
      }
    }
  }
  branches[tower.key] = n;
});
const total = Object.values(branches).reduce((x, y) => x + y, 0);
assert.equal(total, cases.counts.branches);
assert.equal(total * perFacade, cases.counts.slots);
assert.equal(total * perFacade, 140);
assert.deepEqual(branches, cases.counts.branches_per_tower);
assert.deepEqual(typeCounts, cases.counts.per_type);
for (const [key, count] of Object.entries(branches)) assert.equal(count * perFacade, cases.counts.per_tower[key]);
console.log("Nakagin counts: " + total + " branches, " + total * perFacade + " capsules, types " + JSON.stringify(typeCounts) + ".");

const c = model.capsule;
const pitch = model.items.pitch.value;
assert.ok(pitch >= c.width.value && perFacade * pitch <= model.core.side.value + 1e-9, "two capsules fit the core facade");
assert.ok(c.interior.width.value < c.width.value && c.interior.length.value < c.length.value && c.interior.height.value < c.height.value);
for (const t of model.types) {
  const r = t.window_diameter.value / 2;
  assert.ok(Math.abs(t.window_u.value) + r <= c.width.value / 2 && t.window_v.value - r >= 0 && t.window_v.value + r <= c.height.value, "window of " + t.key + " fits");
}
assert.ok(model.core.shaft_side.value < model.core.side.value);
const stagger = model.core.storey_height.value / 3;
assert.ok(Math.abs(model.items.stagger.value - stagger) < 1e-3, "stagger is a third of the storey pitch");
console.log("Nakagin geometry rules: facade fit, windows, interior, stagger hold.");

const e = model.energy;
const months = model.climate.months;
const hdd = months.reduce((sum, m) => sum + Math.max(0, e.heating_base.value - m.mean.value) * m.days.value, 0);
assert.ok(Math.abs(hdd - model.climate.heating_degree_days.value) < 1e-6, "stored HDD equals the monthly sum");
assert.ok(Math.abs(hdd - cases.climate_source.hdd) < 1e-6);
assert.equal(model.climate.source, cases.climate_source.source);
assert.ok(sourceKeys.has(model.climate.source));
const typeA = model.types[0];
const heatingA = names => {
  const u = {...Object.fromEntries(Object.entries(e).map(([k, leaf]) => [k, leaf.value]))};
  for (const name of names) {
    const variant = model.variants.find(v => v.key === name);
    assert.ok(variant, "variant " + name);
    for (const [key, leaf] of Object.entries(variant.overrides)) { assert.ok(key in e, "override key " + key); u[key] = leaf.value; }
  }
  const w = c.width.value, l = c.length.value, h = c.height.value, d = typeA.window_diameter.value;
  const win = Math.PI * d * d / 4;
  const ht = u.u_roof * w * l + u.u_floor * w * l + u.u_wall * 2 * h * l + u.u_wall * (w * h - win) + u.u_window * win + u.u_wall * w * h * u.attached_factor;
  const volume = c.interior.width.value * c.interior.length.value * c.interior.height.value;
  const total = ht + u.air_heat_capacity * u.air_change * volume;
  return {ht, total, annual: total * hdd * 24 / 1000};
};
for (const row of cases.variants.heating_A) {
  const got = heatingA(row.variant || []);
  assert.ok(Math.abs(got.ht - row.h_t) < 1e-9 && Math.abs(got.total - row.h) < 1e-9 && Math.abs(got.annual - row.annual) < 1e-6, JSON.stringify(row.variant));
}
assert.deepEqual(model.variants.map(v => v.key), cases.variants.keys);
for (const v of model.variants) for (const [k, leaf] of Object.entries(v.overrides)) assert.equal(leaf.value, cases.variants.overrides[v.key][k]);
console.log("Nakagin variants: " + model.variants.length + " variants and " + cases.variants.heating_A.length + " heating rows reproduced; HDD " + hdd.toFixed(1) + " K d from " + model.climate.source + ".");
