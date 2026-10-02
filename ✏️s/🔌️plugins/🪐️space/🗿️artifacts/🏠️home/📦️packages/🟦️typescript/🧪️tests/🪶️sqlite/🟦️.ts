import {homeSnapshotToSqliteDatabase,homeSnapshotFromSqliteDatabase,validateHomeSnapshotSqliteDialect,type SHomeSnapshot} from "@semio-tech/space-home";
import {exportSqliteDatabase,importSqliteDatabase} from "@semio-tech/framework";
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import fixture from "../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🔣️.json";
test("Home public package preserves full unsigned64 query fields and structural aliases",async()=>{
 const source=fixture.cases[2]!,snapshot:SHomeSnapshot={schema:source.schema,catalogGeneration:BigInt(source.catalogGeneration)},database=await homeSnapshotToSqliteDatabase(snapshot),oracle=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(oracle.query("SELECT schema,catalog_generation_high AS high,catalog_generation_low AS low FROM home_document").get()).toEqual({schema:source.schema,high:4294967295,low:4294967295});
  for(const sql of fixture.renumberSql)oracle.run(sql);const aliased=await importSqliteDatabase(oracle.serialize());await expect(validateHomeSnapshotSqliteDialect(snapshot,fixture.dialect,aliased)).resolves.toBeUndefined();expect(await homeSnapshotFromSqliteDatabase(aliased)).toEqual(snapshot);
  oracle.query("UPDATE home_document SET schema=?").run(fixture.editedSchema);const edited=await importSqliteDatabase(oracle.serialize());expect((await homeSnapshotFromSqliteDatabase(edited)).schema).toBe(fixture.editedSchema);await expect(validateHomeSnapshotSqliteDialect(snapshot,fixture.dialect,edited)).rejects.toThrow();
 }finally{oracle.close();}
});
