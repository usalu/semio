import {stlSnapshotToSqliteDatabase,stlSnapshotFromSqliteDatabase,stlSnapshotValidateSqliteSubset,STL_SQLITE_SCHEMA,type StlSnapshot} from "@semio-tech/stdio-stl";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import ieee from "../../../../🏅️standards/🔖️ascii/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧫️fixtures/🔢️ieee754/🔣️.json";
test("STL public package exposes typed queryable SQLite snapshot I/O",async()=>{
 const bits=BigInt("0x"+ieee.binary64Bits[6]!);const scalar={bits};
 const snapshot:StlSnapshot={schema:"public STL",solidName:"queryable",triangles:[{normal:[scalar,scalar,scalar],vertices:[[scalar,scalar,scalar],[scalar,scalar,scalar],[scalar,scalar,scalar]]}]};
 const database=await stlSnapshotToSqliteDatabase(snapshot);
 expect(STL_SQLITE_SCHEMA).toContain("CREATE TABLE");
 await stlSnapshotValidateSqliteSubset(snapshot,ieee.sqliteDialect,database);
 const oracle=Database.deserialize(await exportSqliteDatabase(database));
 try{
  expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});
  expect(oracle.query("SELECT schema,name FROM stl_solid").all()).toEqual([{schema:"public STL",name:"queryable"}]);
  expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);
  expect(oracle.query("SELECT CAST(normal_x_ieee754_bits AS TEXT) AS bits,normal_x,normal_x_numeric_class FROM stl_facet").get()).toEqual({bits:BigInt.asIntN(64,bits).toString(),normal_x:null,normal_x_numeric_class:"nan"});
  oracle.run("UPDATE stl_solid SET id=-19;UPDATE stl_facet SET solid_id=-19,id=-id-100;UPDATE stl_vertex SET facet_id=-facet_id-100,id=-id-300");
  const aliased=await importSqliteDatabase(oracle.serialize());
  await expect(stlSnapshotValidateSqliteSubset(snapshot,ieee.sqliteDialect,aliased)).resolves.toBeUndefined();
  expect(await stlSnapshotFromSqliteDatabase(aliased)).toEqual(snapshot);
 }finally{oracle.close();}
});
