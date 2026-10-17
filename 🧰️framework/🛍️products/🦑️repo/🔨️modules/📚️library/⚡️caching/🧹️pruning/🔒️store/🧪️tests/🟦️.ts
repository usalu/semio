import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,writeFileSync,utimesSync,rmSync} from "node:fs";
import {dirname,join} from "node:path";
import fg from "fast-glob";
import fixture from "./🧫️fixtures/🔣️.json";
import schema from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🧬️schema/🔣️.json";
import * as leaseApi from "../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🔒️lease/🟦️.ts";
import * as pruneApi from "../../🟦️.ts";
import {phaseCompilerAdmissionControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";

test("Cargo cache maintenance excludes every live compiler profile and preserves cancellation",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Original ticket artifact directory required");
 mkdirSync(output,{recursive:true});const root=mkdtempSync(join(output,"m-")),build=join(root,"b"),leaseRoot=join(process.cwd(),".cache/b/l/m/data");mkdirSync(leaseRoot,{recursive:true});const leases=mkdtempSync(join(leaseRoot,"m-")),controller=new AbortController(),held:Awaited<ReturnType<typeof leaseApi.acquireCargoBuildLeaseV1>>[]=[];
 const control=(signal:AbortSignal)=>phaseCompilerAdmissionControlV1(signal,30000,()=>undefined,()=>new Promise(done=>setImmediate(done)));
 let operation:Promise<unknown>|undefined;
 try{
  expect(typeof(leaseApi as any).acquireCargoStoreLeaseV1).toBe("function");expect(typeof(pruneApi as any).withCargoCacheMaintenanceV1).toBe("function");
  for(const row of fixture.paths){const path=join(build,row.path);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,row.bytes);utimesSync(path,1,1);}
  for(const profile of fixture.profiles)held.push(await leaseApi.acquireCargoBuildLeaseV1({directory:leases,buildDirectory:build,targetDirectory:build,args:["--profile",profile],signal:controller.signal,control:control(controller.signal)}));
  const identity=leaseApi.cargoBuildLeaseIdentityV1(build,[]),resource=JSON.stringify([schema.definitions.StoreProtocol.const.namespace,identity.buildDirectory]),databasePath=join(leases,createHash("sha256").update(resource).digest("hex")+".sqlite"),database=new Database(databasePath);
  try{database.exec("PRAGMA busy_timeout=0");expect(()=>database.exec("BEGIN EXCLUSIVE")).toThrow();}finally{database.close();}
  const nodeOracle=async()=>JSON.parse(await captureOwnedCommandV1("node",["--no-warnings","-e","const{DatabaseSync}=require('node:sqlite'),db=new DatabaseSync(process.argv[1]);let blocked=false;try{db.exec('PRAGMA busy_timeout=0');try{db.exec('BEGIN EXCLUSIVE');db.exec('ROLLBACK')}catch(error){if(error.errcode!==5)throw error;blocked=true}process.stdout.write(JSON.stringify({blocked}))}finally{db.close()}",databasePath],process.cwd(),{control:control(controller.signal),env:process.env}));
  expect(await nodeOracle()).toEqual({blocked:true});
  let entered=false;const maintenance=async()=>{entered=true;const originalControl=control(controller.signal),units=await pruneApi.scanCargoBuildUnits(build,originalControl),plan=pruneApi.planCachePrune([{name:"cargo",budgetBytes:null,unusedAgeMs:fixture.olderThanMs,units}],Date.now(),0);for(const unit of plan.areas[0]!.ageDeletions)await pruneApi.deleteUnit(build,unit,originalControl);};
  const cancelled=new AbortController();operation=(pruneApi as any).withCargoCacheMaintenanceV1({buildDirectory:build,leaseDirectory:leases,control:control(cancelled.signal),wait:true},maintenance);await Bun.sleep(fixture.cancelAfterMs);cancelled.abort(Error("Original cache maintenance cancelled"));await expect(operation).rejects.toThrow("Original cache maintenance cancelled");operation=undefined;
  expect(entered).toBe(false);expect(fg.sync("**/data",{cwd:build})).toHaveLength(fixture.expected.beforeRelease);
  operation=(pruneApi as any).withCargoCacheMaintenanceV1({buildDirectory:build,leaseDirectory:leases,control:control(controller.signal),wait:true},maintenance);await Bun.sleep(fixture.cancelAfterMs);expect(entered).toBe(false);held.pop()!.release();await Bun.sleep(fixture.cancelAfterMs);expect(entered).toBe(false);held.pop()!.release();await operation;operation=undefined;
  expect(entered).toBe(true);expect(fg.sync("**/data",{cwd:build})).toHaveLength(fixture.expected.afterPrune);
  expect(await nodeOracle()).toEqual({blocked:false});
  const reacquired=await leaseApi.acquireCargoBuildLeaseV1({directory:leases,buildDirectory:build,targetDirectory:build,args:["--profile",fixture.profiles[0]!],signal:controller.signal,control:control(controller.signal)});reacquired.release();
 }finally{controller.abort();for(const lease of held)lease.release();await operation?.catch(()=>{});rmSync(root,{recursive:true,force:true});rmSync(leases,{recursive:true,force:true});}
},30000);
