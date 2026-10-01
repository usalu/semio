import { binary64,binary32,binary64Value,binary32Value,type Binary64 } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
/** 🧫️ Shared PLY generic typed-property corpus with independent scalar and list SQL edits. */
import { Database } from "bun:sqlite";
import { expect, test } from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import ieee from "../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json";
import {plySnapshotValidateSqliteSubset}from"../../🪶️sqlite/🟦️.ts";
import { parsePlySnapshot, type PlySnapshot } from "../../🟦️.ts";
import { plySnapshotToSqliteDatabase, plySnapshotFromSqliteDatabase, PLY_SQLITE_SCHEMA } from "../../🪶️sqlite/🟦️.ts";
import { exportSqliteDatabase, importSqliteDatabase } from "@semio-tech/framework";

function fixtureValue(value:any):import("../../🟦️.ts").PlyValue{switch(value.kind){case"float":return{kind:"float",value:binary32(value.value)};case"double":return{kind:"double",value:binary64(value.value)};case"list":return{kind:"list",value:value.value.map(fixtureValue)};default:return value;}}
const input: PlySnapshot = {...fixture,format:fixture.format as PlySnapshot["format"],elements:fixture.elements.map(element=>({...element,count:BigInt(element.count),properties:element.properties as PlySnapshot["elements"][number]["properties"],rows:element.rows.map(row=>({values:row.values.map(fixtureValue)}))}))};

test("PLY all scalar widths and typed list cells expose shared handwritten relational SQL", async () => {
  expect(PLY_SQLITE_SCHEMA).toBe(await Bun.file(new URL("../../🪶️sqlite/🗄️.sql", import.meta.url)).text());
  expect(parsePlySnapshot(input)).toEqual(input);
  const database = await plySnapshotToSqliteDatabase(input);
  expect(await plySnapshotFromSqliteDatabase(database)).toEqual(input);
  const db = Database.deserialize(await exportSqliteDatabase(database));
  try {
    expect(db.query("PRAGMA integrity_check").get()).toEqual({ integrity_check: "ok" });
    expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
    expect(db.query("SELECT DISTINCT scalar_kind FROM ply_property WHERE form='scalar' ORDER BY scalar_kind").all()).toEqual(["char","double","float","int","short","uchar","uint","ushort"].map(scalar_kind => ({ scalar_kind })));
    expect(db.query("SELECT i.integer_value FROM ply_list_item i JOIN ply_cell c ON c.id=i.cell_id JOIN ply_property p ON p.id=c.property_id WHERE p.name='vertex_indices' ORDER BY i.ordinal").all()).toEqual([{ integer_value: 0 },{ integer_value: 1 },{ integer_value: 0 }]);
    expect(db.query("SELECT format FROM ply_document").get()).toEqual({ format: "binary_big_endian" });
    db.run("UPDATE ply_cell SET real_value=6.125,real_value_ieee754_bits=4618582155356798976 WHERE property_id=(SELECT id FROM ply_property WHERE name='x') AND row_id=1");
    db.run("UPDATE ply_list_item SET integer_value=1 WHERE ordinal=2");
    const edited = await plySnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
    expect(edited.elements[0]!.rows[0]!.values[0]).toEqual({ kind:"double",value:binary64(6.125) });
    expect(edited.elements[1]!.rows[0]!.values[0]).toEqual({ kind: "list", value: [{ kind: "int", value: 0 },{ kind: "int", value: 1 },{ kind: "int", value: 1 }] });
  } finally { db.close(); }
});

test("PLY independently edited ranges, float32 precision, ownership and property completeness reject", async () => {
  for (const edit of [
    "UPDATE ply_cell SET integer_value=256 WHERE kind='uchar'",
    "UPDATE ply_cell SET real_value=0.1 WHERE kind='float'",
    "UPDATE ply_cell SET property_id=999 WHERE id=1",
    "UPDATE ply_cell SET property_id=(SELECT id FROM ply_property WHERE name='vertex_indices') WHERE id=1",
    "DELETE FROM ply_cell WHERE id=1",
    "UPDATE ply_list_item SET ordinal=99 WHERE id=1",
    "UPDATE ply_list_item SET cell_id=1 WHERE id=1",
  ]) {
    const db = Database.deserialize(await exportSqliteDatabase(await plySnapshotToSqliteDatabase(input)));
    try { db.run(edit); await expect(plySnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow(); }
    finally { db.close(); }
  }
});

test("PLY format variants, empty snapshots and bounds preserve domain semantics", async () => {
  for (const format of ["ascii", "binaryLittleEndian", "binaryBigEndian"] as const) {
    const value = { ...input, format };
    expect(await plySnapshotFromSqliteDatabase(await plySnapshotToSqliteDatabase(value as unknown as PlySnapshot))).toEqual(value);
  }
  const empty: PlySnapshot = { schema: "empty.ply", format: "ascii", comments: [], elements: [] };
  expect(await plySnapshotFromSqliteDatabase(await plySnapshotToSqliteDatabase(empty))).toEqual(empty);
  const database = await plySnapshotToSqliteDatabase(input);
  for (const options of [{ maxRows: 0 }, { maxValueBytes: 0 }]) {
    await expect(plySnapshotToSqliteDatabase(input, options)).rejects.toThrow("limit");
    await expect(plySnapshotFromSqliteDatabase(database, options)).rejects.toThrow("limit");
  }
});

test("PLY nested lists and noncanonical float32 values reject before projection", async () => {
  for (const value of [
    { ...input, elements: [{ name: "bad", count: 1n, properties: [{ form: "scalar" as const, name: "x", kind: "float" as const }], rows: [{ values: [{ kind: "float" as const, value: 0.1 }] }] }] },
    { ...input, elements: [{ name: "bad", count: 1n, properties: [{ form: "list" as const, name: "x", countKind: "uChar" as const, valueKind: "int" as const }], rows: [{ values: [{ kind: "list" as const, value: [{ kind: "list" as const, value: [] }] }] }] }] },
  ]) await expect(plySnapshotToSqliteDatabase(value as unknown as PlySnapshot)).rejects.toThrow();
});

test("PLY large list counting and reconstruction observe cancellation", async () => {
  const many: PlySnapshot = { schema: "large.ply", format: "ascii", comments: [], elements: [{ name: "face", count: 1n, properties: [{ form: "list", name: "indices", countKind: "uShort", valueKind: "int" }], rows: [{ values: [{ kind: "list", value: Array.from({ length: 1000 }, () => ({ kind: "int", value: 1 })) }] }] }] };
  const controller = new AbortController();
  let events = 0;
  await expect(plySnapshotToSqliteDatabase(many, { signal: controller.signal, onProgress: () => { if (++events === 2) controller.abort(); } })).rejects.toMatchObject({ name: "AbortError" });
  const database = await plySnapshotToSqliteDatabase(input);
  const reconstruct = new AbortController();
  await expect(plySnapshotFromSqliteDatabase(database, { signal: reconstruct.signal, onProgress: () => reconstruct.abort() })).rejects.toMatchObject({ name: "AbortError" });
});
test("PLY exact scalar words survive independent SQLite affinity, edits and malformed companions", async () => {
  for(const [index,hex] of ieee.binary64Bits.entries()){
    const bits=BigInt("0x"+hex);
    const scalar={bits};const single={bits:parseInt(ieee.binary32Bits[index]!,16)};const snapshot:PlySnapshot={schema:"exact",format:"ascii",comments:[],elements:[{name:"values",count:1n,properties:[{form:"scalar",name:"large",kind:"double"},{form:"scalar",name:"small",kind:"float"},{form:"list",name:"small_list",countKind:"uChar",valueKind:"float"},{form:"list",name:"large_list",countKind:"uChar",valueKind:"double"}],rows:[{values:[{kind:"double",value:scalar},{kind:"float",value:single},{kind:"list",value:[{kind:"float",value:single}]},{kind:"list",value:[{kind:"double",value:scalar}]}]}]}]};
    const db=Database.deserialize(await exportSqliteDatabase(await plySnapshotToSqliteDatabase(snapshot)));
    try{
      expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
      expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);
      expect(db.query("SELECT CAST(real_value_ieee754_bits AS TEXT) AS bits,real_value_numeric_class AS kind FROM ply_cell WHERE id=1").get()).toEqual({bits:BigInt.asIntN(64,bits).toString(),kind:ieee.classes[index]});
      const restored=await plySnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
      expect((restored.elements[0]!.rows[0]!.values[0] as {value:Binary64}).value.bits).toBe(bits);
      expect(restored).toEqual(snapshot);
      db.run("UPDATE ply_cell SET real_value=0,real_value_ieee754_bits=-9223372036854775808,real_value_numeric_class='finite' WHERE id=1");
      const edited=await plySnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())));
      expect((edited.elements[0]!.rows[0]!.values[0] as {value:Binary64}).value.bits).toBe(0x8000000000000000n);
      db.run("UPDATE ply_cell SET real_value=NULL,real_value_numeric_class='nan' WHERE id=1");
      await expect(plySnapshotFromSqliteDatabase(await importSqliteDatabase(new Uint8Array(db.serialize())))).rejects.toThrow();
    }finally{db.close();}
  }
  console.log("[DEBUG] PLY exact binary words and independent SQLite identity laws");
});
test("PLY exact owned dialect, document identity and cancellation laws", async()=>{
  const database=await plySnapshotToSqliteDatabase(input);
  await plySnapshotValidateSqliteSubset(input,ieee.sqliteDialect,database);
  for(const dialect of ieee.invalidSqliteDialects)await expect(plySnapshotValidateSqliteSubset(input,dialect,database)).rejects.toThrow("dialect");
  await expect(plySnapshotValidateSqliteSubset({...input,schema:"different"},ieee.sqliteDialect,database)).rejects.toThrow("identity");
  const controller=new AbortController();controller.abort();
  await expect(plySnapshotValidateSqliteSubset(input,ieee.sqliteDialect,database,{signal:controller.signal})).rejects.toMatchObject({name:"AbortError"});
});

import declarations from "../../🧫️fixtures/🪶️sqlite/📋️declaration-state.json";
test("PLY declaration counts and count scalar kinds remain independent from occurrence rows",async()=>{
 for(const count of declarations.declaredCounts)for(const countKind of declarations.countKinds){
  const snapshot={schema:"owned state",format:"ascii",comments:[],elements:[{name:"face",count:BigInt(count),properties:[{form:"list",name:"indices",countKind,valueKind:"int"}],rows:[{values:[{kind:"list",value:Array.from({length:declarations.listLength},()=>({kind:"int",value:1}))}]}]}]} as PlySnapshot;
  const database=await plySnapshotToSqliteDatabase(snapshot);const db=Database.deserialize(await exportSqliteDatabase(database));try{expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(db.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(db.query("SELECT declared_count_high AS high,declared_count_low AS low FROM ply_element").get()).toEqual({high:Number(BigInt(count)>>32n),low:Number(BigInt(count)&0xffffffffn)});expect(db.query("SELECT count_kind FROM ply_property").get()).toEqual({count_kind:countKind.toLowerCase()});expect(await plySnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()))).toEqual(snapshot);}finally{db.close();}
 }
});

test("PLY independent SQLite declaration edits preserve unsigned64 counts and floating count metadata",async()=>{const db=Database.deserialize(await exportSqliteDatabase(await plySnapshotToSqliteDatabase(input)));try{db.run("UPDATE ply_element SET declared_count_high=4294967295,declared_count_low=4294967295 WHERE id=2");db.run("UPDATE ply_property SET count_kind='double' WHERE form='list'");expect(db.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});const restored=await plySnapshotFromSqliteDatabase(await importSqliteDatabase(db.serialize()));expect(restored.elements[1]!.count).toBe(0xffffffffffffffffn);expect(restored.elements[1]!.properties[0]).toEqual({form:"list",name:"vertex_indices",countKind:"double",valueKind:"int"});expect(restored.elements[1]!.rows).toEqual(input.elements[1]!.rows);}finally{db.close();}});
