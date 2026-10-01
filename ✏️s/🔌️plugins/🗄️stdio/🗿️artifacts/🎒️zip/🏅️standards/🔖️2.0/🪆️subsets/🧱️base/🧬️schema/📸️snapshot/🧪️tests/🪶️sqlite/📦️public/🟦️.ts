import {expect,test}from"bun:test";
import{ZIP_SQLITE_SCHEMA,zipSnapshotToSqliteDatabase,zipSnapshotFromSqliteDatabase,zipSnapshotValidateSqliteSubset,checkZipIso21320Conformance,parseZipSnapshot,type ZipSnapshot}from"@semio-tech/stdio-zip";
import fixture from"../../../🧫️fixtures/🪶️sqlite/🔣️.json";

test("ZIP built public package exposes actual typed named semantic guards",async()=>{
 const snapshot:ZipSnapshot=parseZipSnapshot(fixture);const database=await zipSnapshotToSqliteDatabase(snapshot);const restored=await zipSnapshotFromSqliteDatabase(database);expect(restored).toEqual(snapshot);expect(database.tables).toHaveLength(12);expect(ZIP_SQLITE_SCHEMA).toContain("zip_local_header");
 const dialect={artifactKind:"s.stdio.zip",standard:"2.0",subset:"iso21320"};expect(await zipSnapshotValidateSqliteSubset(restored,dialect,database)).toEqual(await checkZipIso21320Conformance(snapshot));
});
