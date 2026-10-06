import {objSnapshotToSqliteDatabase,objSnapshotFromSqliteDatabase,objSnapshotValidateSqliteSubset,OBJ_SQLITE_SCHEMA,type ObjSnapshot} from "@semio-tech/stdio-obj";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import ieee from "../../../../🏅️standards/🔖️3.0/🪆️subsets/📐️geometry/🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔢️ieee754/🔣️.json";
test("OBJ public package exposes typed queryable SQLite snapshot I/O",async()=>{
 const snapshot:ObjSnapshot={schema:"public OBJ",vertices:[],texcoords:[],normals:[],faces:[],groups:[],objects:[],usemtl:[],smoothingGroups:[],unknownStatements:[{lineIndex:18446744073709551615n,raw:"public source"}]};
 const database=await objSnapshotToSqliteDatabase(snapshot);
 expect(OBJ_SQLITE_SCHEMA).toContain("CREATE TABLE");
 await objSnapshotValidateSqliteSubset(snapshot,ieee.sqliteDialect,database);
 const oracle=Database.deserialize(await exportSqliteDatabase(database));
 try{
  expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(oracle.query("SELECT source_line_ordinal_high,source_line_ordinal_low,raw FROM obj_unknown_statement").all()).toEqual([{source_line_ordinal_high:4294967295,source_line_ordinal_low:4294967295,raw:"public source"}]);
  expect(await objSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).toEqual(snapshot);
 }finally{oracle.close();}
});
