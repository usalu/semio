/** 🔢️ Independent SQLite oracle for exact owned IEEE scalar identities. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import { binary64, binary32, parseBinary64, parseBinary32, encodeIeee754Cells, readBinary64, readBinary32 } from "../🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase, parseSqliteDatabaseSchema } from "../../🟦️.ts";
import fixture from "../🧫️fixtures/🔣️.json";
import integerFixture from "../🧫️fixtures/🎯️integer-query.json";

const columns = [{ index: 1, width: 64 }, { index: 2, width: 32 }] as const;
const sql = "CREATE TABLE exact_scalar(id INTEGER PRIMARY KEY, large REAL, small REAL, large_ieee754_bits INTEGER, large_numeric_class TEXT, small_ieee754_bits INTEGER, small_numeric_class TEXT);";

test("owned IEEE identities preserve payloads through independent SQLite numeric affinity", async () => {
  const database = parseSqliteDatabaseSchema(sql);
  const rows = fixture.binary64Bits.map((bits,index) => {
    const id = BigInt(index + 1);
    return { rowid: id, values: encodeIeee754Cells([id, { bits: BigInt("0x" + bits) }, { bits: parseInt(fixture.binary32Bits[index]!,16) }], columns) };
  });
  const db = Database.deserialize(await exportSqliteDatabase({ tables: [{ ...database.tables[0]!, rows }] }));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("SELECT large_numeric_class,small_numeric_class FROM exact_scalar ORDER BY id").all()).toEqual(fixture.classes.map(kind => ({ large_numeric_class: kind, small_numeric_class: kind })));
    const restored = await importSqliteDatabase(new Uint8Array(db.serialize()));
    for (const [index,row] of restored.tables[0]!.rows.entries()) {
      expect(readBinary64(row,1,columns).bits).toBe(BigInt("0x" + fixture.binary64Bits[index]!));
      expect(readBinary32(row,2,columns).bits).toBe(parseInt(fixture.binary32Bits[index]!,16));
    }
    db.run("UPDATE exact_scalar SET large=1,large_numeric_class='nan' WHERE id=3");
    const wrong = (await importSqliteDatabase(new Uint8Array(db.serialize()))).tables[0]!.rows[2]!;
    expect(() => readBinary64(wrong,1,columns)).toThrow();
  } finally { db.close(); }
});

test("owned scalar types reject invalid widths and derive exact ordinary numeric words", () => {
  expect(binary64(-0).bits).toBe(0x8000000000000000n);
  expect(binary32(-0).bits).toBe(0x80000000);
  for (const value of [0, { bits: -1n }, { bits: 18446744073709551616n }, { bits: 0 }]) expect(() => parseBinary64(value)).toThrow();
  for (const value of [0, { bits: -1 }, { bits: 4294967296 }, { bits: 0n }]) expect(() => parseBinary32(value)).toThrow();
});

test("integer query scalars must agree with IEEE words without rounding", async () => {
  const db = new Database(":memory:", { safeIntegers: true });
  const view = new DataView(new ArrayBuffer(8));
  try {
    db.run("PRAGMA application_id=1397576526");
    db.run("PRAGMA user_version=1");
    db.run("CREATE TABLE exact_integer(id INTEGER PRIMARY KEY,large BLOB,small BLOB,large_ieee754_bits INTEGER,large_numeric_class TEXT,small_ieee754_bits INTEGER,small_numeric_class TEXT)");
    for (const [index,item] of integerFixture.cases.entries()) {
      const value = BigInt(item.integer), large = BigInt("0x" + item.binary64Bits), small = parseInt(item.binary32Bits,16);
      view.setBigUint64(0,large);
      expect(BigInt(view.getFloat64(0)) === value).toBe(item.accept64);
      view.setUint32(0,small);
      expect(BigInt(view.getFloat32(0)) === value).toBe(item.accept32);
      db.run("INSERT INTO exact_integer VALUES(?,?,?,?,?,?,?)", [index+1,value,value,BigInt.asIntN(64,large),"finite",small,"finite"]);
    }
    const rows = (await importSqliteDatabase(new Uint8Array(db.serialize()))).tables[0]!.rows;
    for (const [index,row] of rows.entries()) {
      const item = integerFixture.cases[index]!;
      expect(row.values[1]).toBe(BigInt(item.integer));
      if (item.accept64) expect(readBinary64(row,1,columns).bits).toBe(BigInt("0x"+item.binary64Bits));
      else expect(() => readBinary64(row,1,columns)).toThrow();
      if (item.accept32) expect(readBinary32(row,2,columns).bits).toBe(parseInt(item.binary32Bits,16));
      else expect(() => readBinary32(row,2,columns)).toThrow();
    }
  } finally { db.close(); }
});
