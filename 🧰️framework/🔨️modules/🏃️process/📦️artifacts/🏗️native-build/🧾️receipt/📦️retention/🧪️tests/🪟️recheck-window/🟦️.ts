import {test,expect} from "bun:test";
import {createHash} from "node:crypto";
import {mkdtempSync,mkdirSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {acquireNativeStorageV1,ordinaryNativeStoragePortV1} from "../../../../💾️storage/🟦️.ts";
import {createNativePhysicalGateV1,nativeRecheckWindowV1} from "../../../../💾️storage/🍎️image/📡️roster/🟦️.ts";
import {CargoCompilationSnapshotPoolV1} from "../../📥️snapshot/🟦️.ts";
import {createScriptProcessEnvelope,createIssuedResourceControlV1,withScriptProcessEnvelope} from "../../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../../🔒️leases/🎛️control/🟦️.ts";

const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!artifacts)throw Error("Original test artifact directory required");

/** 🪟️ Retaining N files reproves the kernel image and mapping guard once per recheck window, not N times: a persistent image port gates its full strict check exactly like the gate below, and the pool opens the window around every per-file storage recheck. @see ../../📥️snapshot/🟦️.ts */
test("snapshot pool verification of many retained files runs inside one recheck window and fully reproves storage at most once per window",async()=>{
 const root=mkdtempSync(join(artifacts,"window-")),policy={version:1 as const,root,leaseDirectory:join(root,"leases"),capacityBytes:1<<24,maximumWorkspaceBytes:1<<24,maximumArtifactBytes:1<<24},gate=createNativePhysicalGateV1(),windows:number[]=[];
 const strict=()=>gate.run(nativeRecheckWindowV1(),()=>{windows.push(nativeRecheckWindowV1());},()=>new Promise<void>(done=>setTimeout(done,2)));
 const port={...ordinaryNativeStoragePortV1(),withOperation:<T>(actual:ResourceLeaseControl,run:(control:ResourceLeaseControl)=>Promise<T>)=>run(actual),withJournal:<T>(actual:ResourceLeaseControl,run:(control:ResourceLeaseControl)=>Promise<T>)=>run(actual),observe:async()=>({root,identity:"window-fixture",maximumPhysicalBytes:1<<24}),recheck:strict,measure:strict,syncMetadata:async()=>{},retire:async()=>{}};
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"fixture:window",maximumElapsedMilliseconds:0},{fixture:"window"},Date.now()),async original=>{
  const control=createIssuedResourceControlV1(original,"fixture:window",600000),storage=await acquireNativeStorageV1({policy,control,port,owner:"window",kind:"compiler-workspace",bytes:1<<20});
  const directory=join(root,"retained");mkdirSync(directory,{recursive:true});
  const pool=new CargoCompilationSnapshotPoolV1({workspace:root,mode:"prototype",buildDirectory:join(root,"build"),targetDirectory:join(root,"target"),cargoHome:join(root,"cargo"),directory,maximumBytes:1<<20,storage,control});
  const physical={maxBytes:1<<20,maxWork:1<<20,chunkBytes:4096,cancelled:()=>false,remainingMs:()=>60000,onProgress:()=>{}};
  const count=200,files=Array.from({length:count},(_,index)=>{const bytes=Buffer.from("retained "+index),path=join(directory,"f"+index);writeFileSync(path,bytes);return{path,sha256:createHash("sha256").update(bytes).digest("hex"),byteLength:bytes.length,mode:0o644};});
  windows.length=0;const before=gate.fulls();
  for(const file of files)await pool.verify(file,physical,control);
  const fulls=gate.fulls()-before;
  expect(windows.length,"every file recheck ran its cheap identity check").toBeGreaterThanOrEqual(count);
  expect(windows.every(window=>window>0),"every per-file recheck runs inside a declared window").toBe(true);
  expect(fulls,"full strict reproofs per window, not per file").toBeLessThanOrEqual(Math.ceil(windows.length*0.01));
  const outside=createNativePhysicalGateV1();for(let index=0;index<count;index++)await outside.run(nativeRecheckWindowV1(),()=>{},async()=>{});
  expect(outside.fulls(),"without a window every recheck is a full reproof").toBe(count);
 });
},60000);
