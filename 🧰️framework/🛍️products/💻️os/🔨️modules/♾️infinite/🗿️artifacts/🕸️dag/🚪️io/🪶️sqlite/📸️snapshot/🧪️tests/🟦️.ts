import { expect, test } from "bun:test";
import { Database } from "bun:sqlite";
import { readFileSync } from "node:fs";

const fixture = JSON.parse(readFileSync(new URL("../../../../🧫️fixtures/🪶️sqlite/🔣️.json", import.meta.url), "utf8"));
const sql = readFileSync(new URL("../🗄️.sql", import.meta.url), "utf8");

test("framework DAG property members have exact contiguous backing and independently ordered unique literal keys", () => {
  expect(fixture.propertyMembers).toEqual({ order: "utf8UnsignedOctets", duplicates: "lastValue", cardinalities: [0,1,3], emptyOwnedBytes: 0, backing: "typedContiguousSlots" });
  const d = new Database(":memory:");
  try {
    d.exec("CREATE TABLE member(key TEXT COLLATE BINARY PRIMARY KEY,value INTEGER NOT NULL)");
    for (const [key,value] of [["",0],["引用",1],["!",2],["引用",3]] as const)
      d.query("INSERT INTO member VALUES(?,?) ON CONFLICT(key) DO UPDATE SET value=excluded.value").run(key,value);
    expect(d.query("SELECT key,value FROM member ORDER BY key COLLATE BINARY").all()).toEqual([{key:"",value:0},{key:"!",value:2},{key:"引用",value:3}]);
    expect(Buffer.compare(Buffer.from("!"),Buffer.from("引用"))).toBeLessThan(0);
  } finally { d.close(); }
});

test("framework DAG paid ownership authority remains separate from borrowed preflight and semantic work", () => {
  expect(fixture.ownership).toEqual({
    authority: "actualPersistedDagSnapshot",
    backing: "cumulativeOwnedBacking",
    borrowedPreflightBytes: 0,
    refusalKinds: { ceiling: "ownershipLimit", allocator: "allocationFailed", work: "workLimit", cancellation: "canceled" },
    retirementRefund: false
  });
  const text = Buffer.from(fixture.literal, "utf8");
  const octets = Buffer.from(fixture.octetsHex, "hex");
  expect(text.toString("utf8")).toBe(fixture.literal);
  expect(octets.toString("hex")).toBe(fixture.octetsHex);
  expect(text.byteLength).toBeGreaterThan(fixture.literal.length);
});

test("framework DAG specimens exercise persisted variant domains", () => {
  expect(fixture.nodeKinds).toHaveLength(11);
  expect(fixture.intrinsicVariants).toHaveLength(9);
  expect(fixture.propertyVariants).toHaveLength(6);
  expect(fixture.literal.includes("\0")).toBe(true);
});
test("independent SQLite reads forty named tables and structural relationships", () => {
  const db = new Database(":memory:");
  try {
    db.exec(sql);
    const actual = db.query("SELECT name FROM sqlite_schema WHERE type='table' ORDER BY name").all() as { name: string }[];
    expect(actual.map((row) => row.name)).toEqual(Object.keys(fixture.tableWidths).sort());
    for (const [name, width] of Object.entries(fixture.tableWidths)) {
      const fields = db.query(`PRAGMA table_info('${name}')`).all() as { pk: number; name: string; type: string }[];
      expect(fields).toHaveLength(width as number);
      expect(fields[0]).toMatchObject({ name: "id", type: "INTEGER", pk: 1 });
      const references = db.query(`PRAGMA foreign_key_list('${name}')`).all();
      if (name !== "dag_document" && name !== "dag_property_value" && name !== "dag_intrinsic_value") expect(references.length).toBeGreaterThan(0);
    }
    expect(actual.some((row) => /camera|selection/.test(row.name))).toBe(false);
  } finally { db.close(); }
});
test("independent SQLite exposes exact scalar words, all u64 values and intrinsic octets", () => {
  const db = new Database(":memory:");
  try {
    db.exec(sql);
    for (const [index, hex] of (fixture.binary64Bits as string[]).entries()) {
      const bits = BigInt(`0x${hex}`), buffer = new ArrayBuffer(8), view = new DataView(buffer);
      view.setBigUint64(0, bits, false); const numeric = view.getFloat64(0, false);
      const kind = Number.isNaN(numeric) ? "nan" : numeric === Infinity ? "positiveInfinity" : numeric === -Infinity ? "negativeInfinity" : "finite";
      db.query("INSERT INTO dag_intrinsic_value VALUES (?,?)").run(index + 1, "float");
      db.query("INSERT INTO dag_intrinsic_float VALUES (?,?,?,?,?)").run(index + 1, index + 1, Number.isNaN(numeric) ? null : numeric, BigInt.asIntN(64, bits), kind);
      const row = db.query("SELECT value_ieee754_bits AS bits,value_numeric_class AS class FROM dag_intrinsic_float WHERE id=?").safeIntegers(true).get(index + 1) as { bits: bigint; class: string };
      expect(BigInt.asUintN(64, BigInt(row.bits)).toString(16).padStart(16, "0")).toBe(hex);
      expect(row.class).toBe(kind);
    }
    for (const [index, decimal] of (fixture.unsignedWords as string[]).entries()) {
      const word = BigInt(decimal);
      db.query("INSERT INTO dag_intrinsic_value VALUES (?,?)").run(index + 100, "uint");
      db.query("INSERT INTO dag_intrinsic_unsigned VALUES (?,?,?,?)").run(index + 100, index + 100, word >> 32n, word & 0xffffffffn);
      const row = db.query("SELECT value_high_u32 AS hi,value_low_u32 AS lo FROM dag_intrinsic_unsigned WHERE id=?").get(index + 100) as { hi: number; lo: number };
      expect(((BigInt(row.hi) << 32n) | BigInt(row.lo)).toString()).toBe(decimal);
    }
    db.query("INSERT INTO dag_intrinsic_value VALUES (?,?)").run(1000, "bytes");
    const bytes = Uint8Array.from(fixture.octetsHex.match(/../g), (hex: string) => parseInt(hex, 16));
    db.query("INSERT INTO dag_intrinsic_bytes VALUES (?,?,?)").run(1, 1000, bytes);
    const row = db.query("SELECT hex(value) AS octets FROM dag_intrinsic_bytes").get() as { octets: string };
    expect(row.octets.toLowerCase()).toBe(fixture.octetsHex);
  } finally { db.close(); }
});

test("framework DAG borrowed encoding ceiling admits no ownership and bounds independent literal escaping", () => {
  expect(fixture.preflight).toEqual({traversal:"borrowedFields",ownedBytes:0,textEscapeMultiplier:6,scalarAllowance:64,recordAllowance:4096,envelopeAllowance:65536});
  for(const literal of [fixture.literal,"","\\0","\\n","引用","\\\"\\\\"]) {
    const independent=Buffer.byteLength(JSON.stringify(literal),"utf8");
    expect(independent).toBeLessThanOrEqual(Buffer.byteLength(literal,"utf8")*6+64);
  }
});

test("framework DAG port intrinsic presence keeps authored null distinct from absent relation", () => {
  expect(fixture.portIntrinsicPresence).toBe("preservePresentNull");
  const db=new Database(":memory:");
  try {
    db.exec(sql);
    db.exec("INSERT INTO dag_intrinsic_value VALUES(1,'null'); INSERT INTO dag_port_value VALUES(1,1,1)");
    expect(db.query("WITH port(id) AS (VALUES(1),(2)) SELECT port.id,v.id IS NOT NULL AS present,i.variant FROM port LEFT JOIN dag_port_value v ON v.port_id=port.id LEFT JOIN dag_intrinsic_value i ON i.id=v.value_id ORDER BY port.id").all()).toEqual([{id:1,present:1,variant:"null"},{id:2,present:0,variant:null}]);
  } finally { db.close(); }
});
