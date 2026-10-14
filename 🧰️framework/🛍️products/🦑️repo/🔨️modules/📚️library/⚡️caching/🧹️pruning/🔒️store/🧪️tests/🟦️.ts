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

test("Cargo cache maintenance excludes every live compiler profile and preserves cancellation",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Original ticket artifact directory required");
 mkdirSync(output,{recursive:true});const root=mkdtempSync(join(output,"m-")),build=join(root,"b"),leaseRoot=join(process.cwd(),".cache/framework/leases");mkdirSync(leaseRoot,{recursive:true});const leases=mkdtempSync(join(leaseRoot,"m-")),controller=new AbortController(),held:Awaited<ReturnType<typeof leaseApi.acquireCargoBuildLeaseV1>>[]=[];
 const control=(signal:AbortSignal)=>phaseCompilerAdmissionControlV1(signal,30000,()=>undefined,()=>new Promise(done=>setImmediate(done)));
 let operation:Promise<unknown>|undefined;
 try{
  expect(typeof(leaseApi as any).acquireCargoStoreLeaseV1).toBe("function");expect(typeof(pruneApi as any).withCargoCacheMaintenanceV1).toBe("function");
  for(const row of fixture.paths){const path=join(build,row.path);mkdirSync(dirname(path),{recursive:true});writeFileSync(path,row.bytes);utimesSync(path,1,1);}
  for(const profile of fixture.profiles)held.push(await leaseApi.acquireCargoBuildLeaseV1({directory:leases,buildDirectory:build,args:["--profile",profile],signal:controller.signal,control:control(controller.signal)}));
  const identity=leaseApi.cargoBuildLeaseIdentityV1(build,[]),resource=JSON.stringify([schema.definitions.StoreProtocol.const.namespace,identity.buildDirectory]),database=new Database(join(leases,createHash("sha256").update(resource).digest("hex")+".sqlite"));
  try{database.exec("PRAGMA busy_timeout=0");expect(()=>database.exec("BEGIN EXCLUSIVE")).toThrow();}finally{database.close();}
  let entered=false;const maintenance=async()=>{entered=true;const units=pruneApi.scanCargoBuildUnits(build,controller.signal),plan=pruneApi.planCachePrune([{name:"cargo",budgetBytes:null,unusedAgeMs:fixture.olderThanMs,units}],Date.now(),0);for(const unit of plan.areas[0]!.ageDeletions)pruneApi.deleteUnit(build,unit);};
  const cancelled=new AbortController();operation=(pruneApi as any).withCargoCacheMaintenanceV1({buildDirectory:build,leaseDirectory:leases,control:control(cancelled.signal)},maintenance);await Bun.sleep(fixture.cancelAfterMs);cancelled.abort(Error("Original cache maintenance cancelled"));await expect(operation).rejects.toThrow("Original cache maintenance cancelled");operation=undefined;
  expect(entered).toBe(false);expect(fg.sync("**/data",{cwd:build})).toHaveLength(fixture.expected.beforeRelease);
  operation=(pruneApi as any).withCargoCacheMaintenanceV1({buildDirectory:build,leaseDirectory:leases,control:control(controller.signal)},maintenance);await Bun.sleep(fixture.cancelAfterMs);expect(entered).toBe(false);held.pop()!.release();await Bun.sleep(fixture.cancelAfterMs);expect(entered).toBe(false);held.pop()!.release();await operation;operation=undefined;
  expect(entered).toBe(true);expect(fg.sync("**/data",{cwd:build})).toHaveLength(fixture.expected.afterPrune);
  const reacquired=await leaseApi.acquireCargoBuildLeaseV1({directory:leases,buildDirectory:build,args:["--profile",fixture.profiles[0]!],signal:controller.signal,control:control(controller.signal)});reacquired.release();
  console.log("[DEBUG] Cargo shared store distinctProfiles=parallel SQLiteMaintenance=excluded cancellation=noDeletion terminalRelease=reacquired");
 }finally{controller.abort();for(const lease of held)lease.release();await operation?.catch(()=>{});rmSync(root,{recursive:true,force:true});rmSync(leases,{recursive:true,force:true});}
},30000);
