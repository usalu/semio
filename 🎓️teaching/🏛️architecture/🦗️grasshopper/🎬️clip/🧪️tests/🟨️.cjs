/** 🧬 Independent JSON Schema and data-tree oracle for the portable Grasshopper teaching model. */
const fs = require("node:fs");
const path = require("node:path");
const assert = require("node:assert/strict");
const Ajv = require("ajv/dist/2020").default;
const clip = path.resolve(__dirname, "..");
const read = file => JSON.parse(fs.readFileSync(path.join(clip, file), "utf8"));
const schema = read("🧱️model/🧬️schema/🔣️.schema.json");
const model = read("🧱️model/🔣️.json");
const cases = read("🧪️cases/🔣️.json");

const resolve = (root, steps) => steps.reduce((value, step) => {
  if (step !== null && typeof step === "object") {
    const found = value.find(item => Object.entries(step).every(([key, expected]) => item[key] === expected));
    assert.ok(found, "selector " + JSON.stringify(step) + " matches nothing");
    return found;
  }
  assert.ok(value !== undefined && value !== null && step in Object(value), "path step " + step + " does not exist");
  return value[step];
}, root);

const validate = new Ajv({strict: true, allErrors: true}).compile(schema);
assert.equal(validate(model), true, JSON.stringify(validate.errors));
for (const test of cases.invalid_models) {
  const candidate = structuredClone(model);
  const parent = resolve(candidate, test.path.slice(0, -1));
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
console.log("Grasshopper schema: authored model accepted; " + cases.invalid_models.length + " invalid models rejected.");

for (const [name, canvas] of Object.entries(model.canvases)) {
  const ids = new Set(canvas.objects.map(o => o.id));
  assert.equal(ids.size, canvas.objects.length, name + ": unique ids");
  const spec = id => model.catalogue[canvas.objects.find(o => o.id === id).component];
  for (const object of canvas.objects) assert.ok(model.catalogue[object.component], name + ": component " + object.component);
  for (const wire of canvas.wires) {
    const [left, right] = wire.split(">");
    const [from, out] = [left.slice(0, left.indexOf(".")), left.slice(left.indexOf(".") + 1)];
    const [to, inp] = [right.slice(0, right.indexOf(".")), right.slice(right.indexOf(".") + 1)];
    assert.ok(ids.has(from) && ids.has(to), name + ": wire endpoints " + wire);
    assert.ok(spec(from).outputs.some(p => p.name === out), name + ": output " + wire);
    assert.ok(spec(to).inputs.some(p => p.name === inp), name + ": input " + wire);
  }
  for (const [view, v] of Object.entries(canvas.views)) {
    assert.ok(v.objects.length <= 12, name + "/" + view + ": at most 12 objects");
    assert.deepEqual(Object.keys(v.layout).sort(), [...v.objects].sort(), name + "/" + view + ": layout covers the objects");
    for (const id of v.objects) assert.ok(ids.has(id), name + "/" + view + ": " + id);
  }
}
console.log("Grasshopper canvases: wires, parameters and views consistent in " + Object.keys(model.canvases).length + " canvases.");

const keyOf = p => "{" + p.join(";") + "}";
const pathOf = k => (k.match(/\d+/g) || []).map(Number);
const toMap = tree => new Map(Object.entries(tree).map(([k, v]) => [k, v.slice()]));
const toObj = map => Object.fromEntries(map);
const flatten = (tree, to) => new Map([[keyOf(to || [0]), [...toMap(tree).values()].flat()]]);
const graft = tree => new Map([...toMap(tree)].flatMap(([k, v]) => v.map((x, i) => [keyOf([...pathOf(k), i]), [x]])));
const flip = tree => {
  const rows = [...toMap(tree).values()];
  const width = Math.max(...rows.map(r => r.length));
  return new Map(Array.from({length: width}, (_, j) => [keyOf([j]), rows.map(r => (j < r.length ? r[j] : null))]));
};
const simplify = (tree, mode = "all") => {
  const keys = [...toMap(tree).keys()];
  if (keys.length === 1) return new Map([["{0}", toMap(tree).get(keys[0])]]);
  const paths = keys.map(pathOf);
  const width = paths[0].length;
  const constant = Array.from({length: width}, (_, c) => new Set(paths.map(p => p[c])).size === 1);
  let lead = 0, trail = 0;
  if (mode !== "back") while (lead < width - 1 && constant[lead]) lead++;
  if (mode !== "front") while (trail < width - 1 - lead && constant[width - 1 - trail]) trail++;
  return new Map(keys.map((k, i) => [keyOf(paths[i].slice(lead, width - trail)), toMap(tree).get(k)]));
};
const entwine = trees => new Map(trees.flatMap((t, k) => [...toMap(t)].map(([p, v]) => [keyOf([...pathOf(p), k]), v])));
const mapper = (tree, mapping) => {
  const m = mapping.match(/^\{(.*?)\}(?:\((.*?)\))?\s*(?:->|→)\s*\{(.*?)\}(?:\((.*?)\))?$/);
  const names = m[1].split(";").map(s => s.trim());
  const targets = m[3].trim() ? m[3].split(";").map(s => s.trim()) : [];
  const groups = new Map();
  [...toMap(tree)].forEach(([k, items], n) => {
    const p = pathOf(k);
    if (p.length !== names.length) return;
    if (names.some((name, i) => /^\d+$/.test(name) && Number(name) !== p[i])) return;
    const env = {path_count: Object.keys(tree).length, item_count: items.length, path_index: n};
    names.forEach((name, i) => { if (!/^\d+$/.test(name)) env[name] = p[i]; });
    items.forEach((item, i) => {
      const local = {...env, ...(m[2] ? {[m[2].trim()]: i} : {})};
      const evaluate = expr => Function(...Object.keys(local), "return Math.floor(" + expr.replace(/\//g, "/") + ");")(...Object.values(local));
      const target = keyOf(targets.map(evaluate));
      const order = m[4] ? evaluate(m[4]) : 0;
      if (!groups.has(target)) groups.set(target, []);
      groups.get(target).push([order, item]);
    });
  });
  return new Map([...groups].map(([k, rows]) => [k, (m[4] ? rows.map((r, i) => [r[0], i, r[1]]).sort((a, b) => a[0] - b[0] || a[1] - b[1]).map(r => r[2]) : rows.map(r => r[1]))]));
};

let checked = 0;
for (const test of cases.trees) {
  if (test.error || !test.tree) continue;
  const ops = {flatten: () => flatten(test.tree, test.args && test.args.path), graft: () => graft(test.tree), flip: () => flip(test.tree),
    simplify: () => simplify(test.tree, test.args && test.args.mode), entwine: () => entwine(test.trees)};
  if (test.op === "entwine") {
    assert.deepEqual(toObj(entwine(test.trees)), test.expected, test.id);
    checked++;
  } else if (ops[test.op]) {
    assert.deepEqual(toObj(ops[test.op]()), test.expected, test.id);
    checked++;
  }
}
for (const test of cases.mapper) {
  assert.deepEqual(toObj(mapper(test.tree, test.mapping)), test.expected, test.id);
  checked++;
}
console.log("Grasshopper trees: " + checked + " tree and Path Mapper cases reproduced by an independent JavaScript oracle.");

const roundHalfEven = x => {
  const f = Math.floor(x), d = x - f;
  return d < 0.5 ? f : d > 0.5 ? f + 1 : (f % 2 === 0 ? f : f + 1);
};
let casts = 0;
for (const test of cases.casts.filter(t => t.target === "Integer" && typeof t.value === "number")) {
  assert.equal(roundHalfEven(test.value), test.expected, test.id);
  casts++;
}
for (const test of cases.casts.filter(t => t.target === "Number" && typeof t.value === "string" && t.ok && !t.culture)) {
  assert.ok(/^\s*[-+]?(\d+\.?\d*|\.\d+)([eE][-+]?\d+)?\s*$/.test(test.value) && Number(test.value) === test.expected, test.id);
  casts++;
}
for (const test of cases.casts.filter(t => !t.ok && t.target === "Number" && typeof t.value === "string" && !t.culture)) {
  assert.ok(!/^\s*[-+]?(\d+\.?\d*|\.\d+)([eE][-+]?\d+)?\s*$/.test(test.value), test.id);
  casts++;
}
for (const test of cases.casts.filter(t => t.target === "Domain" && typeof t.value === "string" && t.ok)) {
  const m = test.value.match(/^\s*(\S+)\s+to\s+(\S+)\s*$/i);
  assert.deepEqual([Number(m[1]), Number(m[2])], test.expected, test.id);
  casts++;
}
const ruleIds = new Set(Object.keys(model.casting.rules));
for (const test of cases.casts) assert.ok(ruleIds.has(test.rule), test.id + ": rule " + test.rule + " is documented");
console.log("Grasshopper casting: " + casts + " rounding, number and domain casts reproduced; " + cases.casts.length + " cases name a documented rule.");
