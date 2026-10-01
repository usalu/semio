import {stlSnapshotToSqliteDatabase,stlSnapshotFromSqliteDatabase,stlSnapshotValidateSqliteSubset,STL_SQLITE_SCHEMA,type StlSnapshot} from "@semio-tech/stdio-stl";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import ieee from "../../../🧫️fixtures/🪶️sqlite/🔢️ieee754/🔣️.json";
test("STL public package exposes typed queryable SQLite snapshot I/O",async()=>{
 const snapshot:StlSnapshot={schema:"public STL",solidName:"queryable",triangles:[]};
 const database=await stlSnapshotToSqliteDatabase(snapshot);
 expect(STL_SQLITE_SCHEMA).toContain("CREATE TABLE");
 await stlSnapshotValidateSqliteSubset(snapshot,ieee.sqliteDialect,database);
 const oracle=Database.deserialize(await exportSqliteDatabase(database));
 try{
  expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(oracle.query("SELECT schema,name FROM stl_solid").all()).toEqual([{schema:"public STL",name:"queryable"}]);
  expect(await stlSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).toEqual(snapshot);
 }finally{oracle.close();}
});
