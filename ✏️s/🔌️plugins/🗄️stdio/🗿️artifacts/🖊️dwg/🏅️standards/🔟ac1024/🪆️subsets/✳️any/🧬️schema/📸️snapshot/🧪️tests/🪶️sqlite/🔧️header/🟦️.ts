import { expect,test } from "bun:test";
import { Database } from "bun:sqlite";
import fixture from "../../../🧫️fixtures/🪶️sqlite/🔢️numbers/🔣️.json";
import type { DwgHeaderVariables,DwgHeaderSpaceGeometry } from "../../../../🟦️.ts";
import { DWG_SQLITE_SCHEMA } from "../../../🪶️sqlite/🟦️.ts";
import { dwgProjectHeader,dwgReconstructHeader } from "../../../🪶️sqlite/🔧️header/🟦️.ts";
import { DwgProjection } from "../../../🪶️sqlite/🔢️number/🟦️.ts";
import { DwgReader } from "../../../🪶️sqlite/🫳️reader/🟦️.ts";
import { binary64 } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import { exportSqliteDatabase,importSqliteDatabase,type SqliteDatabase } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

import { dwgHeaderFixture as header } from "../../../🧫️fixtures/🪶️sqlite/🔧️header/🟦️.ts";
async function project(value:DwgHeaderVariables){const p=await DwgProjection.create(DWG_SQLITE_SCHEMA);await p.insert("dwg_document",["owned","AC1024",2n,1252n]);await dwgProjectHeader(p,value);return p.finish();}
async function reconstruct(database:SqliteDatabase){const r=await DwgReader.create(database,DWG_SQLITE_SCHEMA);await r.one("dwg_document");const result=await dwgReconstructHeader(r);await r.finish();return result;}

const words=[binary64(1.25).bits,0x8000000000000000n,0x7ff0000000000000n,0xfff0000000000000n,...fixture.float64Bits.map(word=>BigInt("0x"+word))];
for(const word of words)test("DWG full header preserves independent SQLite IEEE word "+word.toString(16),async()=>{
    const input=header(word),oracle=Database.deserialize(await exportSqliteDatabase(await project(input)));
    try{
      expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);
      expect(oracle.query("SELECT tree_depth,user_integer5 FROM dwg_header_integer_settings").get()).toEqual({tree_depth:-32768,user_integer5:32767});
      expect(await reconstruct(await importSqliteDatabase(oracle.serialize()))).toEqual(input);
      oracle.run("UPDATE dwg_header_units SET unit2_conversion_class='negative_zero',unit2_conversion_ieee754_bits=-9223372036854775808,unit2_conversion=NULL");const expected=structuredClone(input);expected.units.unit2Conversion={bits:0x8000000000000000n};
      expect(await reconstruct(await importSqliteDatabase(oracle.serialize()))).toEqual(expected);
    }finally{oracle.close();}
  console.log("[DEBUG] DWG TypeScript full typed header preserved exact words, signed widths, ordered spaces and optional unsigned64 references");
},60000);

test("DWG header semantic space and numeric identity edits reject after valid SQLite integrity",async()=>{
  const oracle=Database.deserialize(await exportSqliteDatabase(await project(header(binary64(1.25).bits))));
  try{
    oracle.run("UPDATE dwg_header_space_extents_maximum SET ordinal=0 WHERE id=2");expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(reconstruct(await importSqliteDatabase(oracle.serialize()))).rejects.toThrow();
    oracle.run("UPDATE dwg_header_space_extents_maximum SET ordinal=1 WHERE id=2");oracle.run("UPDATE dwg_header_dimension_settings SET scale_class='nan',scale_ieee754_bits=0,scale=NULL");expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(reconstruct(await importSqliteDatabase(oracle.serialize()))).rejects.toThrow();
  }finally{oracle.close();}
  const invalid=header(0n);invalid.integers.treeDepth=32768;await expect(project(invalid)).rejects.toThrow();
},30000);
