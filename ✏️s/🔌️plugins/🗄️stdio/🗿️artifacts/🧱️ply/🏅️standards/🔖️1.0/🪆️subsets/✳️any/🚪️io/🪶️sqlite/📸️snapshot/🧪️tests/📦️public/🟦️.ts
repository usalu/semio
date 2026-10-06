import {plySnapshotToSqliteDatabase,plySnapshotFromSqliteDatabase,plySnapshotValidateSqliteSubset,PLY_SQLITE_SCHEMA,type PlySnapshot} from "@semio-tech/stdio-ply";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import ieee from "../../🧫️fixtures/🔢️ieee754/🔣️.json";
import declarations from "../../🧫️fixtures/📋️declaration-state.json";
import {parsePlyElementDiff} from "../../../../../🧬️schema/🔺️diff/🟦️.ts";

test("PLY declaration patches retain unsigned64 independent metadata",()=>{
 for(const value of declarations.declaredCounts){const count=BigInt(value);expect(parsePlyElementDiff({count})).toEqual({count,properties:undefined,rows:undefined});}
 for(const count of [-1n,18446744073709551616n,1])expect(()=>parsePlyElementDiff({count})).toThrow("unsigned64");
});
test("PLY public package exposes typed queryable SQLite snapshot I/O",async()=>{
 const snapshot:PlySnapshot={schema:"public PLY",format:"ascii",comments:[],elements:[{name:"vertex",count:18446744073709551615n,properties:[],rows:[]}]};
 const database=await plySnapshotToSqliteDatabase(snapshot);
 expect(PLY_SQLITE_SCHEMA).toContain("CREATE TABLE");
 await plySnapshotValidateSqliteSubset(snapshot,ieee.sqliteDialect,database);
 const oracle=Database.deserialize(await exportSqliteDatabase(database));
 try{
  expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(oracle.query("SELECT name,declared_count_high,declared_count_low FROM ply_element").all()).toEqual([{name:"vertex",declared_count_high:4294967295,declared_count_low:4294967295}]);
  expect(await plySnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).toEqual(snapshot);
 }finally{oracle.close();}
});
