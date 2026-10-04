import { expect, test } from "bun:test";
import Ajv from "ajv";
import { Database } from "bun:sqlite";
import fixture from "../../🧫️fixtures/🧮️allocation/🔣️.json";
import schema from "../../🧫️fixtures/🧮️allocation/🧬️schema/🔣️.json";
import { validateJsonSchemaSubset } from "../../../../../🧬️schema/✅️validator/🟦️.ts";

function payloads(): { text: string; blob: Uint8Array } {
  return { text: fixture.textUnit.repeat(fixture.textRepeats), blob: Uint8Array.from({ length: fixture.blobBytes }, (_, index) => fixture.blobUnit[index % fixture.blobUnit.length]!) };
}

test("shared provider allocation corpus is closed against independent Ajv", () => {
  const admit = new Ajv({ strict: true }).compile(schema);
  expect(validateJsonSchemaSubset(schema, fixture)).toEqual([]);
  expect(admit(fixture)).toBe(true);
  for (const hostile of [
    { ...fixture, allocatorAbiBytes: 32 },
    { ...fixture, tinyAllocationBytes: [0] },
    { ...fixture, semanticBytes: fixture.allocationBytes },
    { ...fixture, expectedOwnershipKind: "allocationFailed" },
    { ...fixture, owners: fixture.owners.map(owner => ({ ...owner, legacy: true })) },
    { ...fixture, textBytes: fixture.textBytes - 1 },
    { ...fixture, duplicateOrdinals: fixture.ordinalInput },
  ]) {
    expect(admit(hostile)).toBe(false);
    expect(validateJsonSchemaSubset(schema, hostile).length).toBeGreaterThan(0);
  }
  console.log("[DEBUG] Shared provider closed fixture: 25 required fields, seven independent hostile cases");
});

test("shared provider authored SQLite tables preserve IEEE bits and ordered relations independently", () => {
  const database = new Database(":memory:", { safeIntegers: true });
  const view = new DataView(new ArrayBuffer(8));
  try {
    database.exec("PRAGMA foreign_keys=ON");
    database.exec(fixture.schemaSql);
    for (const owner of fixture.owners) {
      const bits = BigInt("0x" + owner.binary64Bits);
      view.setBigUint64(0, bits);
      const scalar = view.getFloat64(0);
      expect(Number.isNaN(scalar) ? "nan" : "finite").toBe(owner.numericClass);
      database.run("INSERT INTO provider_owner VALUES(?,?,?,?,?,?)", [owner.id, owner.label, new Uint8Array(owner.blob), Number.isNaN(scalar) ? null : scalar, BigInt.asIntN(64, bits), owner.numericClass]);
    }
    for (const [index, ordinal] of fixture.ordinalInput.entries()) database.run("INSERT INTO provider_relation VALUES(?,?,?,?)", [index + 1, 1, ordinal, 7]);
    const ordered = database.query("SELECT id FROM provider_relation ORDER BY ordinal").all() as { id: bigint }[];
    expect(ordered.map(row => Number(row.id))).toEqual(fixture.ordinalExpectedIds);
    const rows = database.query("SELECT id,label,payload,scalar,scalar_ieee754_bits AS bits,scalar_numeric_class AS class,typeof(label) AS textStorage,typeof(payload) AS blobStorage,typeof(scalar_ieee754_bits) AS bitStorage FROM provider_owner ORDER BY id").all() as { id: bigint; label: string; payload: Uint8Array; scalar: number | null; bits: bigint; class: string; textStorage: string; blobStorage: string; bitStorage: string }[];
    for (const [index, row] of rows.entries()) {
      const expected = fixture.owners[index]!;
      expect(row.label).toBe(expected.label);
      expect([...row.payload]).toEqual(expected.blob);
      expect(row.textStorage).toBe("text");
      expect(row.blobStorage).toBe("blob");
      expect(row.bitStorage).toBe("integer");
      expect(BigInt.asUintN(64, row.bits).toString(16).padStart(16, "0")).toBe(expected.binary64Bits);
      view.setBigInt64(0, row.bits);
      const scalar = view.getFloat64(0);
      expect(Number.isNaN(scalar) ? "nan" : "finite").toBe(row.class);
      if (Number.isNaN(scalar)) expect(row.scalar).toBeNull();
      else { expect(Object.is(scalar, -0)).toBe(true); expect(row.scalar).toBe(0); }
    }
    expect(database.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(database.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    const bytes = database.serialize();
    const reopened = Database.deserialize(bytes, { safeIntegers: true });
    try {
      expect(reopened.query("SELECT r.id,o.label,r.score FROM provider_relation r JOIN provider_owner o ON o.id=r.owner_id ORDER BY r.ordinal").all()).toEqual(fixture.ordinalExpectedIds.map(id => ({ id: BigInt(id), label: "", score: 7n })));
      expect(reopened.query("SELECT scalar_ieee754_bits FROM provider_owner ORDER BY id").all()).toEqual(fixture.owners.map(owner => ({ scalar_ieee754_bits: BigInt.asIntN(64, BigInt("0x" + owner.binary64Bits)) })));
    } finally { reopened.close(); }
    database.run("DELETE FROM provider_relation");
    for (const [index, ordinal] of fixture.duplicateOrdinals.entries()) database.run("INSERT INTO provider_relation VALUES(?,?,?,?)", [index + 1, 1, ordinal, 7]);
    expect(database.query("SELECT ordinal,count(*) AS count FROM provider_relation GROUP BY ordinal HAVING count(*)>1").all()).toEqual([{ ordinal: 0n, count: 2n }]);
  } finally { database.close(); }
  console.log("[DEBUG] Independent Bun SQLite and DataView: two tables, two IEEE owners, three ordered relations, duplicate ordinal, reopen");
});

test("shared provider semantic bytes and late copy frontiers have independent SQLite authority", () => {
  const { text, blob } = payloads();
  expect(Buffer.byteLength(text, "utf8")).toBe(fixture.textBytes);
  expect(blob.byteLength).toBe(fixture.blobBytes);
  const utf8 = Buffer.from(text, "utf8");
  expect(Buffer.byteLength(utf8.subarray(0, fixture.copyFrontier).toString("utf8"))).toBe(fixture.copyFrontier);
  expect(fixture.copyFrontier).toBeLessThan(text.length * 2);
  expect(fixture.copyFrontier).toBeLessThan(blob.byteLength);
  const database = new Database(":memory:");
  try {
    database.exec("CREATE TABLE copy_authority(id INTEGER PRIMARY KEY,text_value TEXT NOT NULL,blob_value BLOB NOT NULL); CREATE TABLE retired_admission(id INTEGER PRIMARY KEY,requested INTEGER NOT NULL)");
    database.run("INSERT INTO copy_authority VALUES(?,?,?)", [1, text, blob]);
    expect(database.query("SELECT length(CAST(text_value AS BLOB)) AS textBytes,length(blob_value) AS blobBytes,typeof(text_value) AS textStorage,typeof(blob_value) AS blobStorage FROM copy_authority").get()).toEqual({ textBytes: fixture.textBytes, blobBytes: fixture.blobBytes, textStorage: "text", blobStorage: "blob" });
    const bytes = Buffer.byteLength(fixture.repeatedText);
    for (let index = 0; index < 2; index++) database.run("INSERT INTO retired_admission VALUES(?,?)", [index, bytes]);
    expect(database.query("SELECT sum(requested) AS bytes FROM retired_admission").get()).toEqual({ bytes: fixture.repeatedAllocationBytes });
    expect(database.query("SELECT CASE WHEN sum(requested)+? > ? THEN 'ownershipLimit' ELSE 'admitted' END AS kind FROM retired_admission").get(bytes, fixture.repeatedAllocationBytes)).toEqual({ kind: fixture.expectedOwnershipKind });
  } finally { database.close(); }
  console.log("[DEBUG] Independent UTF-8/octet authority: 100000 bytes each, 65536 interior frontier, repeated retired admission 8 bytes");
});
