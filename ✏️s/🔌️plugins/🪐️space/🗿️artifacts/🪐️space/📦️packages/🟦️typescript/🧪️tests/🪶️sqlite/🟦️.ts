/** 🪐️ Actual named-package fields remain queryable and editable through independent SQLite. */
import{spaceSnapshotToSqliteDatabase,spaceSnapshotFromSqliteDatabase,validateSpaceSnapshotSqliteDialect,type SSpaceSnapshot}from"@semio-tech/space-space";
import{exportSqliteDatabase,importSqliteDatabase}from"@semio-tech/framework";
import{Database}from"bun:sqlite";
import{expect,test}from"bun:test";
import fixture from"../../../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧫️fixtures/🪶️sqlite/🔣️.json";
test("Space public package retains both full timestamp words, duplicate IDs and aliases",async()=>{
 const snapshot:SSpaceSnapshot={...fixture.snapshot,artifacts:fixture.snapshot.artifacts.map(row=>({...row,createdAtMs:BigInt(row.createdAtMs),updatedAtMs:BigInt(row.updatedAtMs)}))},database=await spaceSnapshotToSqliteDatabase(snapshot),oracle=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("SELECT artifact_id,created_at_ms_high AS high,created_at_ms_low AS low FROM space_artifact ORDER BY ordinal").all()).toEqual([{artifact_id:"duplicate",high:4294967295,low:4294967295},{artifact_id:"duplicate",high:0,low:0}]);
  oracle.exec("BEGIN");for(const sql of fixture.renumberSql)oracle.exec(sql);oracle.exec("COMMIT");const aliased=await importSqliteDatabase(oracle.serialize());await expect(validateSpaceSnapshotSqliteDialect(snapshot,fixture.dialect,aliased)).resolves.toBeUndefined();expect(await spaceSnapshotFromSqliteDatabase(aliased)).toEqual(snapshot);
  oracle.query("UPDATE space_artifact SET name=? WHERE ordinal=0").run(fixture.editedName);const edited=await importSqliteDatabase(oracle.serialize());expect((await spaceSnapshotFromSqliteDatabase(edited)).artifacts[0]!.name).toBe(fixture.editedName);await expect(validateSpaceSnapshotSqliteDialect(snapshot,fixture.dialect,edited)).rejects.toThrow();
 }finally{oracle.close();}
});
