import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import { DwgReader,dwgFullUnsigned,dwgOptionalUnsigned,dwgUnsigned } from "../../🫳️reader/🟦️.ts";
import { DwgNumberRow } from "../../🔢️number/🟦️.ts";
import { exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

const sql="CREATE TABLE dwg_document (id INTEGER PRIMARY KEY,schema TEXT NOT NULL);CREATE TABLE dwg_named_reference (id INTEGER PRIMARY KEY,document_id INTEGER NOT NULL REFERENCES dwg_document(id),ordinal INTEGER NOT NULL,name TEXT NOT NULL,handle_high INTEGER,handle_low INTEGER)";
const database:SqliteDatabase={tables:[{name:"dwg_document",sql:"CREATE TABLE dwg_document (id INTEGER PRIMARY KEY,schema TEXT NOT NULL)",rows:[{rowid:1n,values:[1n,"owned"]}]},{name:"dwg_named_reference",sql:"CREATE TABLE dwg_named_reference (id INTEGER PRIMARY KEY,document_id INTEGER NOT NULL REFERENCES dwg_document(id),ordinal INTEGER NOT NULL,name TEXT NOT NULL,handle_high INTEGER,handle_low INTEGER)",rows:[{rowid:2n,values:[2n,1n,1n,"second",4294967295n,4294967295n]},{rowid:7n,values:[7n,1n,0n,"first",null,null]}]}]};

test("DWG row ownership follows semantic ordinals after independent SQLite",async()=>{
  const native=Database.deserialize(await exportSqliteDatabase(database));
  try{
    expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
    expect(native.query("PRAGMA foreign_key_check").all()).toEqual([]);
    const reader=await DwgReader.create(await importSqliteDatabase(native.serialize()),sql);
    expect((await reader.one("dwg_document")).text(1)).toBe("owned");
    const rows=await reader.list("dwg_named_reference",1,1n,2);
    expect(rows.map(row=>row.text(3))).toEqual(["first","second"]);
    expect(dwgOptionalUnsigned(rows[0]!,4,5)).toBeUndefined();
    expect(dwgFullUnsigned(rows[1]!,4,5)).toBe(18446744073709551615n);
    await reader.finish();
    await expect(reader.component("dwg_named_reference",7n)).rejects.toThrow();
    native.run("UPDATE dwg_named_reference SET ordinal=0 WHERE id=2");
    const invalid=await DwgReader.create(await importSqliteDatabase(native.serialize()),sql);await invalid.one("dwg_document");
    await expect(invalid.list("dwg_named_reference",1,1n,2)).rejects.toThrow();
  }finally{native.close();}
},30000);

test("DWG incomplete ownership, widths and bounded cancellation reject",async()=>{
  const reader=await DwgReader.create(database,sql);await reader.one("dwg_document");await expect(reader.finish()).rejects.toThrow();
  const bad=new DwgNumberRow("dwg_named_reference",{rowid:1n,values:[1n,4294967296n,null,0n]});
  expect(()=>dwgUnsigned(bad,1)).toThrow();expect(()=>dwgOptionalUnsigned(bad,2,3)).toThrow();
  await expect(DwgReader.create(database,sql,{maxRows:2})).rejects.toThrow();
  const controller=new AbortController();await expect(DwgReader.create(database,sql,{signal:controller.signal,onProgress:()=>controller.abort()})).rejects.toMatchObject({kind:"canceled"});
},30000);
