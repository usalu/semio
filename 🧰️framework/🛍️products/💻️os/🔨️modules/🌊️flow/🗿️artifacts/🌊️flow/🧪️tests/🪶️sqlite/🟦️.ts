import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import Ajv from "ajv";
import { readFileSync } from "node:fs";
const fixture = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🪶️sqlite/🔣️.json", import.meta.url), "utf8"));
const schema = JSON.parse(readFileSync(new URL("../../🧬️schema/📸️snapshot/🪶️sqlite/🔣️.json", import.meta.url), "utf8"));
const sql = readFileSync(new URL("../../🧬️schema/📸️snapshot/🪶️sqlite/🗄️.sql", import.meta.url), "utf8");
test("framework Flow neutral corpus covers actual widgets, chrome and neural values", () => {
  expect(new Ajv({ strict: true }).compile(schema)(fixture)).toBe(true);
  expect(fixture.widgetKinds).toHaveLength(9); expect(fixture.chromeKinds).toHaveLength(5); expect(fixture.neuralVariants).toHaveLength(6); expect(fixture.literal.includes("\0")).toBe(true);
});
test("independent SQLite exposes all thirty-seven literal Flow entity tables", () => {
  const db = new Database(":memory:");
  try {
    db.exec(sql);
    const rows = db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all() as { name: string }[];
    expect(rows.map((row) => row.name)).toEqual(Object.keys(fixture.tableWidths).sort());
    for (const [name, width] of Object.entries(fixture.tableWidths)) {
      const fields = db.query(`PRAGMA table_info('${name}')`).all() as { pk: number; name: string; type: string }[];
      expect(fields).toHaveLength(width as number); expect(fields[0]).toMatchObject({ name: "id", type: "INTEGER", pk: 1 });
      if (!["flow_document", "flow_gui", "flow_tree", "flow_dictionary", "flow_neural_value"].includes(name)) expect(db.query(`PRAGMA foreign_key_list('${name}')`).all().length).toBeGreaterThan(0);
    }
    const columns = db.query("PRAGMA table_info('flow_document')").all() as { name: string }[];
    expect(columns.map((row) => row.name)).toContain("camera_zoom");
  } finally { db.close(); }
});
test("independent SQLite interprets exact decimal words and signed neural integers", () => {
  const db = new Database(":memory:");
  try {
    db.exec(sql);
    for (const [index, hex] of (fixture.binary64Bits as string[]).entries()) {
      const bits = BigInt(`0x${hex}`), view = new DataView(new ArrayBuffer(8)); view.setBigUint64(0, bits, false); const numeric = view.getFloat64(0, false);
      const kind = Number.isNaN(numeric) ? "nan" : numeric === Infinity ? "positiveInfinity" : numeric === -Infinity ? "negativeInfinity" : "finite";
      db.query("INSERT INTO flow_neural_value VALUES (?,?)").run(index + 1, "decimal");
      db.query("INSERT INTO flow_neural_decimal VALUES (?,?,?,?,?)").run(index + 1, index + 1, Number.isNaN(numeric) ? null : numeric, BigInt.asIntN(64, bits), kind);
      const row = db.query("SELECT value_ieee754_bits AS bits,value_numeric_class AS class FROM flow_neural_decimal WHERE id=?").safeIntegers(true).get(index + 1) as { bits: bigint; class: string };
      expect(BigInt.asUintN(64, row.bits).toString(16).padStart(16, "0")).toBe(hex); expect(row.class).toBe(kind);
    }
    for (const [index, decimal] of (fixture.signedWords as string[]).entries()) {
      db.query("INSERT INTO flow_neural_value VALUES (?,?)").run(index + 100, "integer");
      db.query("INSERT INTO flow_neural_integer VALUES (?,?,?)").run(index + 100, index + 100, BigInt(decimal));
      const row = db.query("SELECT value FROM flow_neural_integer WHERE id=?").safeIntegers(true).get(index + 100) as { value: bigint }; expect(row.value.toString()).toBe(decimal);
    }
  } finally { db.close(); }
});

const families = JSON.parse(readFileSync(new URL("../../🧫️fixtures/🪶️sqlite/🚦️owned-families.json", import.meta.url), "utf8"));
const familiesSchema = JSON.parse(readFileSync(new URL("../../🧬️schema/📸️snapshot/🪶️sqlite/🚦️owned-families.json", import.meta.url), "utf8"));
test("closed Flow ownership demands cover actual ordered and recursive neural families", () => {
  const validate = new Ajv({ strict: true }).compile(familiesSchema);
  expect(validate(families)).toBe(true);
  expect(validate({ ...families, unknown: true })).toBe(false);
  for (const key of ["limits", "refusals", "recordFields"]) expect(validate({ ...families, [key]: { ...families[key], unknown: true } })).toBe(false);
  expect(families.families).toEqual(["orderedMap", "orderedSet", "tree", "neuron", "synapse"]);
  expect(families.recordFields.tree).toEqual(["neurons", "synapses"]);
  expect(families.recordFields.neuron).toEqual(["id", "kind", "params", "tree"]);
  expect(families.recordFields.synapse).toEqual(["id", "from", "to", "fromPort", "toPort"]);
  expect(Buffer.byteLength(fixture.literal.repeat(families.limits.repeat))).toBeGreaterThan(families.limits.interiorBytes);
  expect(families.limits.refusedBytes).toBe(0);
  expect(families.limits.recursiveDepth).toBe(fixture.work.recursiveDepth);
  expect(families.refusals).toEqual({ ownership: "OwnershipLimit", cancel: "Canceled", depth: "DepthLimit" });
});
test("independent SQLite validates literal ordered-family keys without map state normalization", () => {
  const db = new Database(":memory:");
  try {
    db.exec("CREATE TABLE keys (key BLOB PRIMARY KEY, ordinal INTEGER NOT NULL) WITHOUT ROWID");
    for (const [ordinal, key] of families.keys.entries()) db.query("INSERT INTO keys VALUES (?,?)").run(Buffer.from(key, "utf8"), ordinal);
    const rows = db.query("SELECT key,ordinal FROM keys ORDER BY key").all() as { key: Uint8Array; ordinal: number }[];
    expect(rows.map(row => Buffer.from(row.key).toString("utf8"))).toEqual(families.orderedKeys);
    for (const row of rows) expect([...row.key]).toEqual([...Buffer.from(families.keys[row.ordinal], "utf8")]);
    expect(db.query("SELECT count(*) AS n FROM keys").get()).toEqual({ n: families.keys.length });
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("WITH RECURSIVE depth(n) AS (SELECT 1 UNION ALL SELECT n+1 FROM depth WHERE n<?) SELECT max(n) AS n FROM depth").get(families.limits.recursiveDepth)).toEqual({ n: families.limits.recursiveDepth });
  } finally { db.close(); }
});

test("neutral wire Text preserves empty endpoints and literal owned ports independently", () => {
  const witness = fixture.wireText;
  expect(witness).toBeDefined();
  const quoted = witness.canonical.match(/"(?:\\.|[^"\\])*"/gu) as string[];
  const decode = (text: string) => JSON.parse(text.replace(/\\u\{([0-9a-f]+)\}/gu, (_: string, hex: string) => "\\u" + hex.padStart(4, "0")));
  expect(quoted.map(decode)).toEqual([witness.from, witness.fromPort, witness.to, witness.toPort]);
  const db = new Database(":memory:");
  try {
    db.exec("CREATE TABLE wire(source TEXT,source_port TEXT,target TEXT,target_port TEXT,syntax TEXT)");
    db.query("INSERT INTO wire VALUES(?,?,?,?,?)").run(witness.from, witness.fromPort, witness.to, witness.toPort, witness.canonical);
    expect(db.query("SELECT source,source_port,target,target_port,syntax FROM wire").get()).toEqual({ source: witness.from, source_port: witness.fromPort, target: witness.to, target_port: witness.toPort, syntax: witness.canonical });
    expect(witness.canonical).toBe('""@"!@/\\u{0}引用😀"->"!@/\\u{0}引用😀"@""');
    expect(new Ajv({ strict: true }).compile(schema)({ ...fixture, wireText: { ...witness, unknown: true } })).toBe(false);
  } finally { db.close(); }
});

test("neutral adjacent record-map keys have closed element delimiters", () => {
  const witness = fixture.recordMapText;
  expect(witness).toBeDefined();
  const db = new Database(":memory:");
  try {
    db.exec("CREATE TABLE entry(key TEXT PRIMARY KEY,field TEXT NOT NULL,literal TEXT NOT NULL)");
    for (const entry of witness.entries) db.query("INSERT INTO entry VALUES(?,?,?)").run(entry.key, entry.field, entry.literal);
    const entries = db.query("SELECT key,field,literal FROM entry ORDER BY key").all() as { key: string; field: string; literal: string }[];
    expect(entries).toEqual(witness.entries);
    expect("{ " + entries.map(entry => entry.key + "={" + entry.field + "=" + entry.literal + "}").join(" ") + " }").toBe(witness.canonical);
    expect(witness.entries.map((entry: { key: string }) => entry.key)).toEqual(["bool", "decimal"]);
    expect(witness.entries[1].literal).toBe("nan64_" + fixture.binary64Bits[6]);
    expect(new Ajv({ strict: true }).compile(schema)({ ...fixture, recordMapText: { ...witness, unknown: true } })).toBe(false);
    expect(new Ajv({ strict: true }).compile(schema)({ ...fixture, recordMapText: { ...witness, entries: [{ ...witness.entries[0], unknown: true }] } })).toBe(false);
  } finally { db.close(); }
});

test("closed named record field retains a quoted reserved literal key", () => {
  const witness = fixture.namedFieldText;
  expect(witness).toBeDefined();
  expect(JSON.parse(witness.canonical.slice(0, witness.canonical.indexOf("=")))).toBe(witness.key);
  expect(JSON.parse(witness.canonical.slice(witness.canonical.indexOf("=") + 1))).toBe(witness.value);
  expect(witness.key).toBe(fixture.neuralVariants[0]);
  const db = new Database(":memory:");
  try {
    db.exec("CREATE TABLE named(key TEXT PRIMARY KEY,value INTEGER NOT NULL)");
    db.query("INSERT INTO named VALUES(?,?)").run(witness.key, Number(witness.value));
    expect(db.query("SELECT key,value FROM named").get()).toEqual({ key: "null", value: 1 });
    expect(new Ajv({ strict: true }).compile(schema)({ ...fixture, namedFieldText: { ...witness, unknown: true } })).toBe(false);
  } finally { db.close(); }
});
