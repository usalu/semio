import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "../../../🧫️fixtures/🪶️sqlite/🔢️numbers/🔣️.json";
import { binary64 } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import { exportSqliteDatabase,importSqliteDatabase,type SqliteRow } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import { DwgNumberRow,encodeDwgNumberCells,dwgUnsignedWords } from "../../../🪶️sqlite/🔢️number/🟦️.ts";

test("DWG authored numeric classes preserve native identities through independent SQLite",async()=>{
  const words=[binary64(1.25).bits,0x8000000000000000n,0x7ff8000000000000n,0x7ff0000000000000n,0xfff0000000000000n,...fixture.float64Bits.map(word=>BigInt("0x"+word))];
  const rows:SqliteRow[]=words.map((bits,index)=>({rowid:BigInt(index+1),values:[BigInt(index+1),...encodeDwgNumberCells("dwg_annotation_scale",["scale",{bits},{bits},0n])]}));
  const bytes=await exportSqliteDatabase({tables:[{name:"dwg_annotation_scale",sql:"CREATE TABLE dwg_annotation_scale (id INTEGER PRIMARY KEY,name TEXT NOT NULL,paper_units_class TEXT,paper_units_ieee754_bits INTEGER,paper_units REAL,drawing_units_class TEXT,drawing_units_ieee754_bits INTEGER,drawing_units REAL,is_unit_scale INTEGER NOT NULL)",rows}]});
  const independent=Database.deserialize(bytes);
  try{
    expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
    expect(independent.query("SELECT paper_units_class AS kind FROM dwg_annotation_scale ORDER BY id").all().slice(0,5)).toEqual(fixture.cases.map(value=>({kind:value.class})));
    const restored=await importSqliteDatabase(independent.serialize());
    restored.tables[0]!.rows.forEach((row,index)=>{const logical=new DwgNumberRow("dwg_annotation_scale",row);expect(logical.real(2).bits).toBe(words[index]!);expect(logical.real(3).bits).toBe(words[index]!);});
    independent.run("UPDATE dwg_annotation_scale SET paper_units_class='negative_zero',paper_units_ieee754_bits=-9223372036854775808,paper_units=NULL WHERE id=1");
    const edited=await importSqliteDatabase(independent.serialize());expect(new DwgNumberRow("dwg_annotation_scale",edited.tables[0]!.rows[0]!).real(2).bits).toBe(0x8000000000000000n);
    independent.run("UPDATE dwg_annotation_scale SET paper_units_class='nan',paper_units_ieee754_bits=0,paper_units=NULL WHERE id=1");
    expect(independent.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
    const invalid=await importSqliteDatabase(independent.serialize());expect(()=>new DwgNumberRow("dwg_annotation_scale",invalid.tables[0]!.rows[0]!).real(2)).toThrow();
  }finally{independent.close();}
},30000);

test("DWG unsigned word pairs and authored numeric positions reject lossy inputs",()=>{
  expect(dwgUnsignedWords(18446744073709551615n)).toEqual([4294967295n,4294967295n]);
  for(const value of [-1n,18446744073709551616n,9007199254740992])expect(()=>dwgUnsignedWords(value as bigint)).toThrow();
  expect(()=>encodeDwgNumberCells("dwg_annotation_scale",["scale",binary64(0),binary64(0),0n],{maxColumns:8})).toThrow();
  expect(()=>encodeDwgNumberCells("dwg_annotation_scale",["scale",binary64(0),binary64(0),0n],{maxValueBytes:1})).toThrow();
  expect(()=>encodeDwgNumberCells("dwg_dictionary_variable",[binary64(0)])).toThrow();
},30000);
