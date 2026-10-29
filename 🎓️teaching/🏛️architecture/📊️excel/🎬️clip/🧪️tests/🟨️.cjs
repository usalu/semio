/** 🧬 Independent JSON Schema, A1, copy-translation and number-format oracle for the portable Excel teaching workbook. */
const fs = require("node:fs");
const path = require("node:path");
const assert = require("node:assert/strict");
const Ajv = require("ajv/dist/2020").default;
const clip = path.resolve(__dirname, "..");
const read = file => JSON.parse(fs.readFileSync(path.join(clip, file), "utf8"));
const schema = read("🧱️model/🧬️schema/🔣️.schema.json");
const model = read("🧱️model/🔣️.json");
const cases = read("🧪️cases/🔣️.json");
const nakaginFile = path.resolve(clip, "../../🏢️nakagin/🔣️.json");

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
console.log("Excel schema: authored workbook accepted; " + cases.invalid_models.length + " invalid workbooks rejected.");

const sheetNames = model.sheets.map(sheet => sheet.name);
assert.deepEqual(sheetNames, cases.workbook.sheets);
assert.equal(new Set(sheetNames.map(name => name.toLowerCase())).size, sheetNames.length, "sheet names are unique");
const germanTabs = model.sheets.map(sheet => sheet.title.de);
assert.deepEqual(germanTabs, ["Programm", "Typ A", "Typ B", "Typ C", "Typ D", "Klima", "Energie"]);
model.sheets.forEach(sheet => assert.equal(sheet.title.en, sheet.name, "English tab label is the canonical sheet name"));
for (const [name, texts] of Object.entries(cases.workbook.tabs)) assert.equal(model.sheets.find(sheet => sheet.name === name).title.de, texts.de);
const cellOf = (sheetName, address) => {
  const sheet = model.sheets.find(item => item.name === sheetName);
  assert.ok(sheet, "sheet " + sheetName);
  return sheet.cells[address];
};
for (const [name, refersTo] of Object.entries(model.names)) {
  const [sheetName, address] = refersTo.split("!");
  assert.ok(sheetNames.includes(sheetName), name + " points to an existing sheet");
  assert.ok(cellOf(sheetName, address.replaceAll("$", "")), name + " points to an authored cell");
}
const refSheets = /(?:'([^']+)'|([A-Za-z_][\w.]*))!/g;
for (const sheet of model.sheets) {
  const formulas = Object.values(sheet.cells).filter(cell => cell.formula).map(cell => cell.formula);
  if (sheet.table) formulas.push(...sheet.table.columns.filter(column => column.formula).map(column => column.formula));
  for (const formula of formulas) {
    for (const match of formula.matchAll(refSheets)) {
      for (const part of (match[1] ?? match[2]).split(":")) assert.ok(sheetNames.includes(part), sheet.name + ": " + formula + " names sheet " + part);
    }
  }
}

const nakagin = JSON.parse(fs.readFileSync(nakaginFile, "utf8"));
const walk = path => path.split(".").reduce((node, step) => {
  if (Array.isArray(node)) return node.find(item => item && item.key === step) ?? node[Number(step)];
  return node[step];
}, nakagin);
let bound = 0;
for (const sheet of model.sheets) {
  for (const [address, cell] of Object.entries(sheet.cells)) {
    if (!cell.from) continue;
    const leaf = walk(cell.from);
    assert.ok(leaf !== undefined, sheet.name + "!" + address + " binds to " + cell.from);
    assert.equal(cell.value, leaf.value, sheet.name + "!" + address);
    assert.equal(cell.basis, leaf.basis, sheet.name + "!" + address + " basis");
    bound += 1;
  }
}
assert.ok(bound > 80, "bound inputs: " + bound);
console.log("Excel workbook: " + bound + " inputs equal the shared Nakagin model values.");

for (const sheet of model.sheets) {
  for (const [address, cell] of Object.entries(sheet.cells)) assert.notEqual(typeof cell.value, "string", sheet.name + "!" + address + " is a text constant without German label");
  for (const column of sheet.table?.columns ?? []) if (column.labels) for (const label of Object.values(column.labels)) assert.ok(label.en && label.de);
}
const program = model.sheets.find(sheet => sheet.name === "Program");
const typeColumn = program.table.columns.find(column => column.key === "type");
assert.deepEqual(Object.values(typeColumn.labels).map(label => label.de), ["Typ A", "Typ B", "Typ C", "Typ D"]);
assert.equal(program.table.columns.find(column => column.key === "facade").labels.E.de, "O");
const rows = program.table.rows;
assert.equal(rows.length, 140);
const typeCounts = {};
for (const row of rows) typeCounts[row[4]] = (typeCounts[row[4]] ?? 0) + 1;
const expected = cases.workbook.values;
["A", "B", "C", "D"].forEach((key, index) => assert.equal(typeCounts["Type " + key], expected["Program!B" + (4 + index)], "count Type " + key));
assert.equal(rows.length, expected["Program!B8"]);
const towerCounts = {};
for (const row of rows) towerCounts[row[0]] = (towerCounts[row[0]] ?? 0) + 1;
assert.deepEqual(towerCounts, {A: 54, B: 86});
const interior = nakagin.capsule.interior;
const area = interior.width.value * interior.length.value;
assert.ok(Math.abs(area - expected["Type A!B9"]) < 1e-12);
assert.ok(Math.abs(rows.length * area - expected["Program!D8"]) < 1e-9);

assert.equal(program.table.name, "Capsules");
assert.deepEqual(program.table.title, {en: "Capsules", de: "Kapseln"});
const headers = program.table.columns.map(column => column.header.en);
const structured = /(?:Capsules)?\[(?:\[#This Row\],\[|@)?([^\[\]#@]+)\]/g;
let structuredCount = 0;
for (const sheet of model.sheets) {
  const formulas = [...Object.values(sheet.cells).filter(cell => cell.formula).map(cell => cell.formula), ...(sheet.table?.columns ?? []).filter(column => column.formula).map(column => column.formula)];
  for (const formula of formulas) for (const match of formula.matchAll(structured)) { assert.ok(headers.includes(match[1]), sheet.name + ": " + formula + " names column " + match[1]); structuredCount += 1; }
}
assert.ok(structuredCount >= 12, "structured references: " + structuredCount);
for (const column of program.table.columns) assert.ok(column.header.de, "German header " + column.key);
console.log("Excel table: " + structuredCount + " structured references name existing columns of table Capsules.");

const letters = (number) => { let text = ""; while (number) { const rest = (number - 1) % 26; text = String.fromCharCode(65 + rest) + text; number = (number - 1 - rest) / 26; } return text; };
const number = text => [...text].reduce((total, char) => total * 26 + char.charCodeAt(0) - 64, 0);
for (const [text, value] of cases.references.columns) {
  assert.equal(number(text), value);
  assert.equal(letters(value), text);
}

const CELL = /(\$?)([A-Z]{1,3})(\$?)(\d+)/y;
const shiftCell = (text, dc, dr) => {
  const [, ca, col, ra, row] = text.match(/^(\$?)([A-Z]{1,3})(\$?)(\d+)$/);
  const c = ca ? number(col) : number(col) + dc;
  const r = ra ? Number(row) : Number(row) + dr;
  if (c < 1 || c > 16384 || r < 1 || r > 1048576) return null;
  return ca + letters(c) + ra + r;
};
const translate = (formula, from, to) => {
  const [, fc, fr] = from.match(/^([A-Z]+)(\d+)$/);
  const [, tc, tr] = to.match(/^([A-Z]+)(\d+)$/);
  const dc = number(tc) - number(fc), dr = Number(tr) - Number(fr);
  return formula.replace(/(\$?[A-Z]{1,3}\$?\d+)(?::(\$?[A-Z]{1,3}\$?\d+))?/g, (all, a, b) => {
    const first = shiftCell(a, dc, dr), second = b ? shiftCell(b, dc, dr) : "";
    return first === null || second === null ? "#REF!" : b ? first + ":" + second : first;
  });
};
const simple = cases.translate.filter(item => item.simple);
assert.ok(simple.length >= 8);
for (const item of simple) assert.equal(translate(item.formula, item.from, item.to), item.expected, item.formula);
console.log("Excel A1: " + simple.length + " copy translations reproduced by an independent implementation.");

const intl = cases.display.filter(item => item.intl);
assert.ok(intl.length >= 3);
for (const item of intl) {
  const options = {minimumFractionDigits: item.intl.min, maximumFractionDigits: item.intl.max, useGrouping: item.intl.group};
  assert.equal(new Intl.NumberFormat("en-US", options).format(item.value), item.en, "en " + item.format);
  assert.equal(new Intl.NumberFormat("de-DE", options).format(item.value), item.de, "de " + item.format);
}
console.log("Excel display: " + intl.length + " number formats equal ICU en-US and de-DE output.");
