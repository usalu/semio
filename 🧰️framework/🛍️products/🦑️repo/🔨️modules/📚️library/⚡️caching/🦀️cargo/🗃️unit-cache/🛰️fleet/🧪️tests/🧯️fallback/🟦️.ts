import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {PROVENANCE_VOLATILE_OUTPUTS_V1} from "../../../../../../🏗️build/🌐️remote-cache/🧾️provenance/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../../🏗️build/🌐️remote-cache/🖥️server/🟦️.ts";
import {createFleetClientV1,readFleetSettingsV1,type FleetClaimV1} from "../../📡️client/🟦️.ts";
import {obtainFleetUnitV1,type FleetUnitV1} from "../../🟦️.ts";
import {FLEET_POLICY_V1} from "../../🪪️identity/🟦️.ts";
import {countFallbackV1,readFallbacksV1} from "../../📊️counters/🟦️.ts";

/** 🧯️ A fleet fault never ends a unit's wait for the fleet at the first try, and every fall-back to a local compile is counted with its reason. */
const scratch:string[]=[],servers:RemoteCacheServerV1[]=[];
const directory=(label:string)=>{const path=realpathSync(mkdtempSync(join(tmpdir(),"fb-"+label+"-")));scratch.push(path);return path;};
afterAll(async()=>{for(const server of servers)await server.stop().catch(()=>{});for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
async function serve():Promise<RemoteCacheServerV1>{
 const server=await startRemoteCacheServerV1({directory:directory("srv"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**24,maximumAgeDays:30,defaultInstance:"main",tokens:{write:"w-token"}});
 servers.push(server);return server;
}
function fakeUnit(server:{url:string},key:string,owner:string,client?:Partial<ReturnType<typeof createFleetClientV1>>):FleetUnitV1{
 const settings=readFleetSettingsV1(directory("fake"),{SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_INSTANCE:"main",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w-token"})!,real=createFleetClientV1(settings);
 return {client:client===undefined?real:{...real,...client},root:settings.stateDirectory,environment:{},localKey:"1".repeat(64),key,slot:"2".repeat(64),slotParts:[],crate:"fake",platform:{version:1,os:"darwin",arch:"arm64",host:"h",rustc:"3".repeat(64),toolchain:"",linker:"",cpu:""},roots:[],cwd:"/w",workspace:undefined,laneRoot:undefined,owner,expected:[]};
}
const down:FleetClaimV1={state:"unavailable",fault:"down"};

test("the volatile manifest name of the server's provenance is the unit-cache manifest name",()=>{
 expect(PROVENANCE_VOLATILE_OUTPUTS_V1).toContain(FLEET_POLICY_V1.manifestName);
});

test("fall-back reasons are counted one byte each, read back by reason, and refuse anything that is not a reason name",()=>{
 const root=directory("count");
 countFallbackV1(root,"claims-down");countFallbackV1(root,"claims-down");countFallbackV1(root,"claims-down");countFallbackV1(root,"rounds");
 countFallbackV1(root,"Bad Reason!");countFallbackV1(root,"../escape");countFallbackV1(undefined,"rounds");
 expect(readFallbacksV1(root)).toEqual({"claims-down":3,rounds:1});
 expect(readFallbacksV1(directory("none"))).toEqual({});
});

test("a claim that fails transiently is retried a bounded number of times before the unit compiles locally, and the reason is the fault",async()=>{
 const server=await serve(),noise=()=>{},policy={retryBackoffMilliseconds:5};
 let calls=0;
 const flaky=fakeUnit(server,sha("flaky"),"flaky:1:aa",{claim:async()=>{calls++;return calls<=FLEET_POLICY_V1.claim.retries?down:{state:"granted",untilMilliseconds:Date.now()+30000};}});
 const recovered=await obtainFleetUnitV1(flaky,noise,policy);
 expect(calls).toBe(FLEET_POLICY_V1.claim.retries+1);expect(recovered.outcome).toBe("compile");
 if(recovered.outcome==="compile"){expect(recovered.reason).toBe("granted");expect(recovered.hold).toBeDefined();await recovered.hold!.release();}
 calls=0;
 const dead=fakeUnit(server,sha("dead"),"dead:1:aa",{claim:async()=>{calls++;return down;}});
 const gave=await obtainFleetUnitV1(dead,noise,policy);
 expect(calls).toBe(FLEET_POLICY_V1.claim.retries+1);expect(gave.outcome==="compile"&&gave.reason).toBe("claims-down");expect(gave.outcome==="compile"&&gave.hold).toBeUndefined();
 calls=0;
 const denied=fakeUnit(server,sha("denied"),"denied:1:aa",{claim:async()=>{calls++;return {state:"unavailable",fault:"denied"};}});
 const refused=await obtainFleetUnitV1(denied,noise,policy);
 expect(calls,"a refused credential is no transient fault").toBe(1);expect(refused.outcome==="compile"&&refused.reason).toBe("claims-denied");
});

test("a wait on a live claim whose long poll fails is retried a bounded number of times, then ends in the await-fault reason",async()=>{
 const server=await serve(),noise=()=>{},key=sha("held"),holder=fakeUnit(server,key,"holder:1:aa");
 expect((await holder.client.claim(key,"holder:1:aa",30000,60000)).state).toBe("granted");
 let waits=0;
 const waiter=fakeUnit(server,key,"waiter:2:bb",{await:async()=>{waits++;return "fault";}});
 const decision=await obtainFleetUnitV1(waiter,noise,{retryBackoffMilliseconds:5});
 expect(waits).toBe(FLEET_POLICY_V1.claim.retries+1);expect(decision.outcome==="compile"&&decision.reason).toBe("await-fault");
 await holder.client.release(key,"holder:1:aa");
},30000);
