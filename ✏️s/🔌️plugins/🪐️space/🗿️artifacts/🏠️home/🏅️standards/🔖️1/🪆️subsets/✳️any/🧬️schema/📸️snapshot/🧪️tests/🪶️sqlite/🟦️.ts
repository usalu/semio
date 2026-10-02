import {expect,test} from "bun:test";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import {parseSHomeSnapshot} from "../../🟦️.ts";
import {parseSHomeArtifact} from "../../../🟦️.ts";
import {parseSHomeDiff} from "../../../🔺️diff/🟦️.ts";
import {inverse} from "../../../🧬️mutations/🔢️change-catalog-generation/↩️inverse/🟦️.ts";
import {diff} from "../../../🧬️mutations/🔢️change-catalog-generation/🔺️diff/🟦️.ts";
import {Database} from "bun:sqlite";
import {homeSnapshotToSqliteDatabase,homeSnapshotFromSqliteDatabase,validateHomeSnapshotSqliteDialect,HOME_SQLITE_SCHEMA} from "../../🪶️sqlite/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
test("Home snapshot accepts every native unsigned64 generation exactly",()=>{
 for(const row of fixture.cases){const snapshot={schema:row.schema,catalogGeneration:BigInt(row.catalogGeneration)};expect(parseSHomeSnapshot(snapshot)).toEqual(snapshot);}
 for(const value of fixture.invalidGenerations)expect(()=>parseSHomeSnapshot({schema:"owned",catalogGeneration:BigInt(value)})).toThrow();
});
test("Home BMP and supplementary Unicode schemas admit true interior cancellation",async()=>{
 for(const seed of fixture.schemaScanSeeds){const controller=new AbortController();let interior=false;await expect(homeSnapshotToSqliteDatabase({schema:seed.repeat(fixture.schemaScanRepeat),catalogGeneration:0n},{signal:controller.signal,onProgress(event){if(event.phase==="projectSnapshot"&&event.completed>=65536&&event.completed<event.total){interior=true;controller.abort();}}})).rejects.toThrow();expect(interior).toBe(true);}
});
test("Home artifact, sparse diff and mutation callers retain unsigned64 generation",()=>{
 for(const row of fixture.cases){const generation=BigInt(row.catalogGeneration);expect(parseSHomeArtifact({schema:row.schema,catalogGeneration:generation}).catalogGeneration).toBe(generation);expect(parseSHomeDiff({catalogGeneration:generation})).toEqual({schema:undefined,catalogGeneration:generation});expect(inverse({newCatalogGeneration:0n},{catalogGeneration:generation})).toEqual([{newCatalogGeneration:generation}]);expect(diff({newCatalogGeneration:generation})).toEqual({catalogGeneration:generation});}
});
test("Home semantic SQLite fields are queryable, editable and independently reserializable",async()=>{
 for(const row of fixture.cases){const snapshot={schema:row.schema,catalogGeneration:BigInt(row.catalogGeneration)};const database=await homeSnapshotToSqliteDatabase(snapshot);const oracle=Database.deserialize(await exportSqliteDatabase(database));try{
  expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(oracle.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(oracle.query("SELECT schema,catalog_generation_high AS high,catalog_generation_low AS low FROM home_document").get()).toEqual({schema:row.schema,high:Number(snapshot.catalogGeneration>>32n),low:Number(snapshot.catalogGeneration&0xffffffffn)});
  for(const sql of fixture.renumberSql)oracle.run(sql);const aliased=await importSqliteDatabase(oracle.serialize());expect(await homeSnapshotFromSqliteDatabase(aliased)).toEqual(snapshot);await expect(validateHomeSnapshotSqliteDialect(snapshot,fixture.dialect,aliased)).resolves.toBeUndefined();
  oracle.query("UPDATE home_document SET schema=?,catalog_generation_high=?,catalog_generation_low=?").run(fixture.editedSchema,fixture.editedGenerationHigh,fixture.editedGenerationLow);const edited=await importSqliteDatabase(oracle.serialize());expect(await homeSnapshotFromSqliteDatabase(edited)).toEqual({schema:fixture.editedSchema,catalogGeneration:(BigInt(fixture.editedGenerationHigh)<<32n)|BigInt(fixture.editedGenerationLow)});await expect(validateHomeSnapshotSqliteDialect(snapshot,fixture.dialect,edited)).rejects.toThrow();
 }finally{oracle.close();}}
});
test("Home import independently authored SQLite and reject wrong ownership and unsigned words",async()=>{
 const oracle=new Database(":memory:");try{oracle.run("PRAGMA application_id=1397576526;PRAGMA user_version=1;"+HOME_SQLITE_SCHEMA);oracle.query("INSERT INTO home_document VALUES (?,?,?,?)").run(-71,"independent Home",4294967295,4294967295);const database=await importSqliteDatabase(oracle.serialize());expect(await homeSnapshotFromSqliteDatabase(database)).toEqual({schema:"independent Home",catalogGeneration:0xffffffffffffffffn});oracle.run("PRAGMA ignore_check_constraints=ON;UPDATE home_document SET catalog_generation_high=4294967296");expect(oracle.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(homeSnapshotFromSqliteDatabase(await importSqliteDatabase(oracle.serialize()))).rejects.toThrow();}finally{oracle.close();}
});
test("Home exact declared coordinates and resource/cancellation gates precede materialization",async()=>{
 const row=fixture.cases[2]!,snapshot={schema:row.schema,catalogGeneration:BigInt(row.catalogGeneration)},database=await homeSnapshotToSqliteDatabase(snapshot);for(const dialect of fixture.invalidDialects)await expect(validateHomeSnapshotSqliteDialect(snapshot,dialect,database)).rejects.toThrow();
 const cancelledBefore=new AbortController();cancelledBefore.abort();for(const options of[{maxRows:0},{maxValueBytes:8},{signal:cancelledBefore.signal}]){await expect(homeSnapshotToSqliteDatabase(snapshot,options)).rejects.toThrow();await expect(homeSnapshotFromSqliteDatabase(database,options)).rejects.toThrow();}
 const large={schema:"世界".repeat(65536),catalogGeneration:17n},controller=new AbortController();let cancelled=false;await expect(homeSnapshotToSqliteDatabase(large,{signal:controller.signal,onProgress(event){if(event.phase==="projectSnapshot"&&event.completed>0&&event.total>1){cancelled=true;controller.abort();}}})).rejects.toThrow();expect(cancelled).toBe(true);
});
