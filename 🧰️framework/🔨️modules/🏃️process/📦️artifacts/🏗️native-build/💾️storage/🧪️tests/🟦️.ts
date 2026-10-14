import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {mkdtempSync,readdirSync} from "node:fs";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import {nativeStorageStateV1,nativeStorageEventV1,nativeStorageUsedBytesV1,acquireNativeStorageV1,ordinaryNativeStoragePortV1,type NativeStorageEventV1} from "../🟦️.ts";
import {buildCargoArtifacts} from "../../🟦️.ts";

const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!artifacts)throw Error("Original test artifact directory required");
const ajv=new Ajv({strict:false});ajv.addSchema(schema);
const validate=ajv.getSchema(schema.$id+"#/definitions/Event")!;
const original=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>10000,publish:async()=>{},yieldContinuation:async()=>{}});

test("neutral storage events agree with independent SQLite and preserve charge until physical retirement",()=>{
 for(const row of fixture.cases){
  let state=nativeStorageStateV1(),refused=false;const db=new Database(":memory:");db.exec("CREATE TABLE allocations(id TEXT PRIMARY KEY, bytes INTEGER NOT NULL, phase TEXT NOT NULL)");
  try{for(const event of row.events){expect(validate(event),JSON.stringify(validate.errors)).toBe(true);const used=Number((db.query("SELECT COALESCE(SUM(bytes),0) AS used FROM allocations").get() as {used:number}).used),oracleRefused=event.type==="reserved"&&used+(event as any).allocation.bytes>fixture.capacity;
   if(oracleRefused){expect(()=>nativeStorageEventV1(state,event as NativeStorageEventV1,fixture.capacity)).toThrow("capacity");refused=true;break;}
   state=nativeStorageEventV1(state,event as NativeStorageEventV1,fixture.capacity);
   if(event.type==="reserved")db.query("INSERT INTO allocations VALUES(?,?,?)").run((event as any).allocation.id,(event as any).allocation.bytes,"reserved");else if(event.type==="retired")db.query("DELETE FROM allocations WHERE id=?").run((event as any).id);else db.query("UPDATE allocations SET phase=? WHERE id=?").run(event.type,(event as any).id);
   expect(nativeStorageUsedBytesV1(state)).toBe(Number((db.query("SELECT COALESCE(SUM(bytes),0) AS used FROM allocations").get() as {used:number}).used));
  }expect([nativeStorageUsedBytesV1(state),refused]).toEqual([row.used,row.refused]);}finally{db.close();}
 }
 console.log("[DEBUG] Native storage six neutral event cases agree with SQLite; reserved/committed/retiring remain charged");
});

test("ordinary filesystem refuses strict storage before ledger, producer or workspace writes",async()=>{
 const root=mkdtempSync(join(artifacts!,"strict-storage-")),policy={version:1 as const,root,leaseDirectory:join(artifacts!,"leases"),capacityBytes:100,maximumWorkspaceBytes:100,maximumArtifactBytes:100};
 await expect(acquireNativeStorageV1({policy,control:original(),port:ordinaryNativeStoragePortV1(),owner:"original",kind:"compiler-workspace",bytes:100})).rejects.toThrow("enforcing storage backend");
 expect(readdirSync(root)).toEqual([]);console.log("[DEBUG] Actual ordinary filesystem strict admission refused before any managed write");
});

test("expired or cancelled caller cannot begin a storage observation",async()=>{
 const root=mkdtempSync(join(artifacts!,"strict-control-"));let observations=0;const port={...ordinaryNativeStoragePortV1(),observe:async()=>{observations++;throw Error("should not observe");}},control=original();
 await expect(acquireNativeStorageV1({policy:{version:1,root,leaseDirectory:join(artifacts!,"leases"),capacityBytes:100,maximumWorkspaceBytes:100,maximumArtifactBytes:100},control:{...control,remainingMilliseconds:()=>0},port,owner:"original",kind:"compiler-workspace",bytes:1})).rejects.toThrow("deadline");expect(observations).toBe(0);
 const abort=new AbortController();abort.abort(Error("original cancellation"));await expect(acquireNativeStorageV1({policy:{version:1,root,leaseDirectory:join(artifacts!,"leases"),capacityBytes:100,maximumWorkspaceBytes:100,maximumArtifactBytes:100},control:{...control,signal:abort.signal},port,owner:"original",kind:"compiler-workspace",bytes:1})).rejects.toThrow("original cancellation");expect(observations).toBe(0);
});

test("actual Cargo dispatch refuses an invented storage receipt before recipe, capture or compiler writes",async()=>{
 const root=mkdtempSync(join(artifacts!,"strict-cargo-")),control=original();let rechecks=0;
 const invented={allocation:{id:"1".repeat(32),owner:"invented",kind:"compiler-workspace",bytes:100,phase:"reserved"},backing:{identity:"invented",root,maximumPhysicalBytes:100},recheck:async()=>{rechecks++;},commit:async()=>{},retire:async()=>{}};
 await expect(buildCargoArtifacts(join(root,"Cargo.toml"),[],{version:1,mode:"prototype",cwd:root,buildDirectory:join(root,"b"),captureDirectory:join(root,"c"),leaseDirectory:join(artifacts!,"leases"),budgetMs:10000},{control,storage:invented as any,sourceBoundary:{version:1,maxSourceBytes:262144,maxSourceFiles:32,maxActiveLines:4000}})).rejects.toThrow("Original issued native storage reservation");expect(rechecks).toBe(0);expect(readdirSync(root)).toEqual([]);
});

test("neutral journal checkpoint preserves every held phase and sequence with independent SQLite",()=>{
 const db=new Database(":memory:");db.exec("CREATE TABLE held(id TEXT PRIMARY KEY,phase TEXT,bytes INTEGER)");let state=nativeStorageStateV1();
 try{for(const [index,phase] of ["reserved","committed","retiring"].entries()){const allocation={id:String(index+1).repeat(32),owner:"original-checkpoint",kind:"compiler-workspace" as const,bytes:index+1,phase:"reserved" as const};state=nativeStorageEventV1(state,{type:"reserved",allocation},100);if(phase!=="reserved")state=nativeStorageEventV1(state,{type:phase as "committed"|"retiring",id:allocation.id},100);db.query("INSERT INTO held VALUES(?,?,?)").run(allocation.id,phase,index+1);}
  const event={type:"checkpoint",state};expect(validate(event),JSON.stringify(validate.errors)).toBe(true);const replay=nativeStorageEventV1(nativeStorageStateV1(),event as any,100);expect(replay).toEqual(state);expect(replay.allocations.map(row=>({id:row.id,phase:row.phase,bytes:row.bytes}))).toEqual(db.query("SELECT id,phase,bytes FROM held ORDER BY id").all());expect(nativeStorageUsedBytesV1(replay)).toBe(6);expect(()=>nativeStorageEventV1(state,event as any,100)).toThrow("checkpoint");expect(()=>nativeStorageEventV1(nativeStorageStateV1(),event as any,5)).toThrow("capacity");
 }finally{db.close();}
 console.log("[DEBUG] Independent SQLite confirms checkpoint retains reserved, committed and retiring charge/sequence");
});
