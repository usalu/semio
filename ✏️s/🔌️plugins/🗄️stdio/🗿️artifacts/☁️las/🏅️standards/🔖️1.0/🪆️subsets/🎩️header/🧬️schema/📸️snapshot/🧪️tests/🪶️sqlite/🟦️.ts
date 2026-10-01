import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import fixture from "../../🧫️fixtures/🪶️sqlite/🔣️.json";
import type {LasSnapshot} from "../../🟦️.ts";
import {LAS_SQLITE_SCHEMA,lasSnapshotToSqliteDatabase,lasSnapshotFromSqliteDatabase,lasSnapshotValidateSqliteSubset} from "../../🪶️sqlite/🟦️.ts";
import {binary64} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🔢️ieee754/🟦️.ts";
import {exportSqliteDatabase,importSqliteDatabase} from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🟦️.ts";

import {lasSnapshotFixture as snapshot} from "../../🧫️fixtures/🪶️sqlite/🟦️.ts";
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
