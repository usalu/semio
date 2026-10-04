/** 🖼️ Complete TIFF operation laws retain their original operation limits. */
import {expect,test} from "bun:test";
import Ajv from "ajv/dist/2020";
import {Database} from "bun:sqlite";
import fixture from "../../🧫️fixtures/💰️operation/🔣️.json";
import schema from "../../🧬️schema/💰️operation/🔣️.json";
import {validateJsonSchemaSubset} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🧬️schema/✅️validator/🟦️.ts";
import {exportSqliteDatabase,SqliteOperation} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {tiffSnapshotToSqliteDatabase,tiffSnapshotToSqliteFile,tiffSnapshotFromSqliteFile,type TiffSnapshot} from "../../🟦️.ts";
test("complete TIFF operation grant corpus is closed and independently admitted",()=>{const admit=new Ajv({strict:true}).compile(schema);expect(admit(fixture)).toBe(true);expect(validateJsonSchemaSubset(schema,fixture)).toEqual([]);for(const hostile of [{...fixture,extra:1},{...fixture,domain:{...fixture.domain,projectionRequests:[4]}}]){expect(admit(hostile)).toBe(false);expect(validateJsonSchemaSubset(schema,hostile).length).toBeGreaterThan(0);}});
test("actual TIFF chunk projector and physical file share one complete operation grant",async()=>{
 const value:TiffSnapshot={schema:fixture.domain.schema,byteOrder:"littleEndian",ifds:fixture.domain.ifds as TiffSnapshot["ifds"]},projection=new SqliteOperation(),projected=await tiffSnapshotToSqliteDatabase(value,projection),physical=new SqliteOperation(),bytes=await exportSqliteDatabase(projected,physical);
 const oracle=Database.deserialize(bytes);try{expect(oracle.query("SELECT hex(payload) AS payload FROM tiff_chunk").get()).toEqual({payload:"00FF4100"});expect(oracle.query("SELECT COUNT(*) AS tables FROM sqlite_schema WHERE type='table'").get()).toEqual({tables:fixture.domain.tableCount});}finally{oracle.close();}
 const grant=physical.ownedBytes;expect(grant).toBeGreaterThan(fixture.domain.projectionRequests.reduce((a,b)=>a+b,0));
 expect(new Uint32Array(fixture.domain.tableSlotCount).byteLength).toBe(fixture.domain.tableSlotBytes);
 expect(fixture.domain.tableSlotCount).toBe(fixture.domain.tableCount*2);
 expect(projection.ownedBytes).toBe(fixture.domain.tableSlotBytes+fixture.domain.projectionRequests.reduce((a,b)=>a+b,0));
 const refused=new SqliteOperation({maxAllocationBytes:grant});await expect(tiffSnapshotToSqliteFile(value,refused)).rejects.toHaveProperty("kind",fixture.refusal);expect(refused.ownedBytes).toBeGreaterThan(projection.ownedBytes);
 const complete=new SqliteOperation({maxAllocationBytes:grant+projection.ownedBytes});expect(await tiffSnapshotToSqliteFile(value,complete)).toEqual(bytes);expect(complete.remainingBytes()).toBe(0);expect(await tiffSnapshotFromSqliteFile(bytes)).toEqual(value);
});
