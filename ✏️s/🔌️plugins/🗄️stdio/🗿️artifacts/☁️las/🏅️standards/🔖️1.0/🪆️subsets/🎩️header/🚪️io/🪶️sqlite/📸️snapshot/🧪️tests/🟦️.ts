import Ajv from "ajv";
import cohort from "../🧫️fixtures/🚦️cohort/🔣️.json";

import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import fixture from "../🧫️fixtures/🔣️.json";
import type {LasSnapshot} from "../../../../🧬️schema/📸️snapshot/🟦️.ts";
import {LAS_SQLITE_SCHEMA,lasSnapshotToSqliteDatabase,lasSnapshotFromSqliteDatabase,lasSnapshotValidateSqliteSubset} from "../🟦️.ts";
import {binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

import {lasSnapshotFixture as snapshot} from "🧰️support/🟦️.ts";
const f=(index:number)=>({bits:BigInt("0x"+fixture.float64Bits[index%fixture.float64Bits.length]!)});

test("LAS all owned header/VLR/point fields and exact IEEE/presence states survive independent SQLite",async()=>{
  const native=Database.deserialize(await exportSqliteDatabase(await lasSnapshotToSqliteDatabase(snapshot)));
  try{
    expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(native.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(await lasSnapshotFromSqliteDatabase(await importSqliteDatabase(native.serialize()))).toEqual(snapshot);
    expect(native.query(fixture.independentQuery).all()).toEqual([{ordinal:0,intensity:65535,time_numeric_class:null,red:null},{ordinal:1,intensity:0,time_numeric_class:"nan",red:null},{ordinal:2,intensity:17,time_numeric_class:null,red:0},{ordinal:3,intensity:17,time_numeric_class:"finite",red:65535}]);
    native.run(fixture.independentEdit);const edited=await lasSnapshotFromSqliteDatabase(await importSqliteDatabase(native.serialize()));expect(edited.points[1]!.intensity).toBe(23);expect(edited.vlrs[0]!.data).toEqual([0,42,17,0]);expect(edited.points[3]!.gpsTime).toBeUndefined();expect(edited.header.numberOfPointRecords).toBe(4294967295);
    console.log("[DEBUG] LAS complete typed header, histogram, proprietary VLR octets, point/GPS/RGB SQL joins and edited file verified");
  }finally{native.close();}
},30000);

test("LAS coherent numeric identity and semantic child order reject independent valid SQLite edits",async()=>{
  expect(LAS_SQLITE_SCHEMA).not.toContain("BLOB");for(const sql of fixture.invalidEdits){const n=Database.deserialize(await exportSqliteDatabase(await lasSnapshotToSqliteDatabase(snapshot)));try{n.run(sql);expect(n.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});await expect(lasSnapshotFromSqliteDatabase(await importSqliteDatabase(n.serialize()))).rejects.toThrow();}finally{n.close();}}
},30000);

test("LAS declared coordinates, resource controls and cancellation use the owned semantic provider",async()=>{
  const database=await lasSnapshotToSqliteDatabase(snapshot);for(const dialect of fixture.acceptedDialects)await lasSnapshotValidateSqliteSubset(snapshot,dialect,database);for(const dialect of fixture.rejectedDialects)await expect(lasSnapshotValidateSqliteSubset(snapshot,dialect,database)).rejects.toThrow();
  await expect(lasSnapshotToSqliteDatabase(snapshot,{maxRows:1})).rejects.toThrow();await expect(lasSnapshotFromSqliteDatabase(database,{maxRows:1})).rejects.toThrow();await expect(lasSnapshotToSqliteDatabase(snapshot,{maxValueBytes:1})).rejects.toThrow();await expect(lasSnapshotFromSqliteDatabase(database,{maxValueBytes:1})).rejects.toThrow();const c=new AbortController();c.abort();await expect(lasSnapshotToSqliteDatabase(snapshot,{signal:c.signal})).rejects.toThrow();await expect(lasSnapshotFromSqliteDatabase(database,{signal:c.signal})).rejects.toThrow();
  expect(binary64(1.25)).toEqual(f(3));
},30000);

test("LAS long projection and reconstruction cancel at bounded semantic checkpoints",async()=>{
  const large:LasSnapshot={...snapshot,vlrs:[{...snapshot.vlrs[0]!,data:Array.from({length:1200},(_,i)=>i%256)}],points:Array.from({length:600},()=>({...snapshot.points[0]!}))};
  for(const phase of ["projectSnapshot","reconstructSnapshot"] as const){const c=new AbortController();let reached=false;
    const options={signal:c.signal,onProgress:(p:{phase:string;completed:number;total:number})=>{if(p.phase===phase&&p.completed>=256){reached=true;c.abort();}}};
    if(phase==="projectSnapshot")await expect(lasSnapshotToSqliteDatabase(large,options)).rejects.toThrow(/cancel/i);
    else {const db=await lasSnapshotToSqliteDatabase(large);await expect(lasSnapshotFromSqliteDatabase(db,options)).rejects.toThrow(/cancel/i);}
    expect(reached).toBe(true);
  }
},30000);


test("LAS controlled cohort long literal ownership cancels inside each declared string field",async()=>{
 for(const field of ["schema","systemIdentifier","generatingSoftware","description"] as const){const text="x".repeat(cohort.copyBytes);const value:LasSnapshot={...snapshot,header:{...snapshot.header},vlrs:snapshot.vlrs.map(vlr=>({...vlr}))};if(field==="schema")value.schema=text;else if(field==="description")value.vlrs[0]!.description=text;else value.header[field]=text;
 const database=await lasSnapshotToSqliteDatabase(value);for(const phase of ["projectSnapshot","reconstructSnapshot"] as const){let interior=false;const controller=new AbortController();const options={signal:controller.signal,onProgress:(event:{phase:string,completed:number,total:number})=>{if(event.phase===phase&&event.completed>=cohort.copyCancelAfter&&event.completed<event.total){interior=true;controller.abort();}}};await expect(phase==="projectSnapshot"?lasSnapshotToSqliteDatabase(value,options):lasSnapshotFromSqliteDatabase(database,options)).rejects.toMatchObject({kind:"canceled"});expect(interior).toBe(true);}
 }
});





import NormSemanticAjv from "ajv/dist/2020.js";
import normSemanticContract from "../🧫️fixtures/🎛️semantic.json";

import {lasSnapshotToSqliteDatabase as normSemanticProject,lasSnapshotFromSqliteDatabase as normSemanticRestore} from "../🟦️.ts";
import {exportSqliteDatabase as normSemanticExport,importSqliteDatabase as normSemanticImport} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";
import {independentSqliteExtent as normIndependentExtent} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🔮️semantic-extent/🟦️.ts";


test("LAS controlled cohort literal fields and every raw word have independent SQLite and DataView identity",async()=>{
 
 const value:LasSnapshot={...snapshot,schema:cohort.literalSchema,header:{...snapshot.header,systemIdentifier:cohort.literalText,generatingSoftware:cohort.literalText,numberOfPointRecords:cohort.unsigned32,creationYear:cohort.unsigned16},vlrs:snapshot.vlrs.map(vlr=>({...vlr,description:cohort.literalText})),points:cohort.word64.map((word,index)=>({...snapshot.points[index%snapshot.points.length]!,x:{bits:BigInt("0x"+word)},intensity:cohort.unsigned16,scanAngleRank:cohort.signed8}))};
 const native=Database.deserialize(await exportSqliteDatabase(await lasSnapshotToSqliteDatabase(value)));
 try{expect(native.query("PRAGMA integrity_check").get()).toEqual({integrity_check:"ok"});expect(native.query("PRAGMA foreign_key_check").all()).toEqual([]);expect(native.query("SELECT schema FROM las_document").get()).toEqual({schema:cohort.literalSchema});expect(native.query("SELECT system_identifier,generating_software,number_of_point_records,creation_year FROM las_header").get()).toEqual({system_identifier:cohort.literalText,generating_software:cohort.literalText,number_of_point_records:cohort.unsigned32,creation_year:cohort.unsigned16});
 const rows=native.query("SELECT x_ieee754_bits,intensity,scan_angle_rank FROM las_point ORDER BY ordinal").safeIntegers(true).all() as {x_ieee754_bits:bigint,intensity:bigint,scan_angle_rank:bigint}[];expect(rows.length).toBe(cohort.word64.length);for(const[index,row]of rows.entries()){const view=new DataView(new ArrayBuffer(8));view.setBigInt64(0,row.x_ieee754_bits,true);expect(view.getBigUint64(0,true)).toBe(BigInt("0x"+cohort.word64[index]!));expect(row.intensity).toBe(BigInt(cohort.unsigned16));expect(row.scan_angle_rank).toBe(BigInt(cohort.signed8));}
 expect(await lasSnapshotFromSqliteDatabase(await importSqliteDatabase(native.serialize()))).toEqual(value);
 }finally{native.close();}
});

test("LAS paid owner contract retains exact declared record slots and independently counted semantic relationships",async()=>{
 
 const contract=cohort.ownership;
 expect(contract).toEqual({"authority":"actualLasSnapshot","nativeFields":"directDeclaredRecords","recordSlots":{"snapshot":4,"header":25,"point":14,"rgb":3},"ieee754":"rawBinary64Words","admission":"beforeBacking","retirementRefund":false,"refusalKinds":{"owned":"ownershipLimit","allocator":"allocationFailed","work":"workLimit","cancellation":"canceled"}});
 const db=Database.deserialize(await exportSqliteDatabase(await lasSnapshotToSqliteDatabase(snapshot)));
 try{
  const groups=db.query("SELECT 7+(SELECT count(*) FROM las_vlr)+(SELECT count(*) FROM las_vlr_octet)+(SELECT count(*) FROM las_point)+(SELECT count(*) FROM las_point_gps)+(SELECT count(*) FROM las_point_rgb) AS entities").get() as {entities:number};
  const semanticRows=db.query("SELECT (SELECT count(*) FROM las_header)+(SELECT count(*) FROM las_return_histogram)+(SELECT count(*) FROM las_vlr)+(SELECT count(*) FROM las_vlr_octet)+(SELECT count(*) FROM las_point)+(SELECT count(*) FROM las_point_gps)+(SELECT count(*) FROM las_point_rgb) AS rows").get() as {rows:number};
  expect(groups.entities).toBe(semanticRows.rows+1);
  const text=cohort.literalText.repeat(11);
  expect(Buffer.byteLength(JSON.stringify(text))).toBeLessThanOrEqual(Buffer.byteLength(text)*6+64);
  expect(db.query("SELECT count(*) AS n FROM las_header").get()).toEqual({n:1});
  expect(db.query("SELECT count(*) AS n FROM las_return_histogram").get()).toEqual({n:5});
 }finally{db.close();}
});

test("LAS native input ownership allowance remains distinct from semantic SQL value limits",async()=>{
 expect(cohort["literalSchema"]).toEqual("LAS 世界\u0000 schema");expect(cohort["literalText"]).toEqual("Grüße\t\n\u0000🌠");expect(cohort["workItems"]).toEqual(1024);expect(cohort["copyBytes"]).toEqual(100000);expect(cohort["cancelAfter"]).toEqual(256);expect(cohort["copyCancelAfter"]).toEqual(65536);expect(cohort["word64"]).toEqual(["0000000000000000","8000000000000000","0000000000000001","3ff4000000000000","7ff0000000000000","fff0000000000000","7ff0000000000001","7ff8123456789abc","fff0123456789abc"]);expect(cohort["unsigned32"]).toEqual(4294967295);expect(cohort["unsigned16"]).toEqual(65535);expect(cohort["signed8"]).toEqual(-128);expect(cohort["ownership"]["authority"]).toEqual("actualLasSnapshot");expect(cohort["ownership"]["nativeFields"]).toEqual("directDeclaredRecords");
 const database=await lasSnapshotToSqliteDatabase(snapshot);const independent=Database.deserialize(await exportSqliteDatabase(database));
 try{const schema=independent.query("SELECT schema FROM las_document").get() as {schema:string};expect(Buffer.byteLength(schema.schema)).toBeGreaterThan(1);expect(independent.query("SELECT count(*) AS n FROM las_return_histogram").get()).toEqual({n:5});}finally{independent.close();}
 await expect(lasSnapshotToSqliteDatabase(snapshot,{maxValueBytes:1})).rejects.toMatchObject({kind:"ownershipLimit"});
 await expect(lasSnapshotFromSqliteDatabase(database,{maxValueBytes:1})).rejects.toMatchObject({kind:"ownershipLimit"});
});

test("LAS complete independent cells preserve copied limits and required empty metadata",async()=>{
 expect(normSemanticContract["schemaBytes"]).toEqual(5875);expect(normSemanticContract["tableWidths"]).toEqual({"las_document":2,"las_header":50,"las_point":21,"las_point_gps":4,"las_point_rgb":4,"las_return_histogram":4,"las_vlr":6,"las_vlr_octet":4});expect(normSemanticContract["cases"]).toEqual([{"id":"full","rows":21,"valueBytes":1509},{"id":"metadataRetainingEmpty","rows":7,"valueBytes":520}]);
 for(const item of normSemanticContract.cases){const source=structuredClone(snapshot);if(item.id==="metadataRetainingEmpty"){source.vlrs=[];source.points=[];}const database=await normSemanticProject(source),bytes=await normSemanticExport(database);expect(normIndependentExtent(bytes)).toEqual({rows:item.rows,valueBytes:item.valueBytes,schemaBytes:normSemanticContract.schemaBytes,tableWidths:normSemanticContract.tableWidths});const limits={maxRows:item.rows,maxValueBytes:item.valueBytes,maxSchemaBytes:normSemanticContract.schemaBytes,maxTables:8,maxColumns:50};expect(await normSemanticProject(source,limits)).toEqual(database);expect(await normSemanticRestore(await normSemanticImport(bytes),limits)).toEqual(source);
  for(const short of[{...limits,maxRows:item.rows-1},{...limits,maxValueBytes:item.valueBytes-1},{...limits,maxSchemaBytes:limits.maxSchemaBytes-1},{...limits,maxTables:7},{...limits,maxColumns:49}]){await expect(normSemanticProject(source,short)).rejects.toThrow();await expect(normSemanticRestore(database,short)).rejects.toThrow();}
 }console.log("[DEBUG] LAS complete independent SQLite cells and metadata preserve full and empty semantic limits");
});
