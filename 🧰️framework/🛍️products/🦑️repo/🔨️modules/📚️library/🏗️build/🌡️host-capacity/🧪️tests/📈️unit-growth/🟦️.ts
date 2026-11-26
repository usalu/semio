/** 📈️ Contract: the port that grows a parent memory grant books the difference beside the grant in the fair queue of the host gate (so every run, the warden thresholds and the interactive reserve see it), holds it until it is released, ends a wait that is never answered at its time bound or when its signal aborts, and reads the other machines of the fleet from the group store without ever taking an unknown fleet for a small one.
 * @see ../../🟦️.ts (`createGrantGrowthPortV1`, `readFleetGrowthHostsV1`) */
import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readHostCapacityPolicyV1,type HostSampleV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🌡️host-capacity/🟦️.ts";
import buildPolicy from "../../../🔣️policy.json" with {type:"json"};
import {createGrantGrowthPortV1,growthHostV1,readFleetGrowthHostsV1} from "../../🟦️.ts";

const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(realpathSync(tmpdir()),"unit-growth-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=(over:Partial<HostSampleV1>={}):HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:10,...over});
const fast=readHostCapacityPolicyV1({...buildPolicy.hostCapacity,wait:{...buildPolicy.hostCapacity.wait,pollMilliseconds:10,progressMilliseconds:20,sampleMaximumAgeMilliseconds:0}});
const grants=(directory:string)=>existsSync(join(directory,"grants"))?readdirSync(join(directory,"grants")):[];
const queue=(directory:string)=>existsSync(join(directory,"queue"))?readdirSync(join(directory,"queue")):[];
const eventually=async(check:()=>boolean)=>{const started=Date.now();while(!check()&&Date.now()-started<3000)await new Promise(done=>setTimeout(done,10));return check();};
const ask=(over:Partial<Parameters<ReturnType<typeof createGrantGrowthPortV1>["acquire"]>[0]>={})=>({unit:"big",extraMegabytes:3364,signal:new AbortController().signal,waitMilliseconds:5000,write:()=>{},...over});

test("the difference is booked as a supplementary grant of its own size and given back on release",async()=>{
 const directory=fresh(),sample=async()=>({host:host(),swappedOutMegabytes:0}),port=createGrantGrowthPortV1({environment:{LANG:"en_US.UTF-8"},directory,policy:fast,sample});
 const answer=await port.acquire(ask());
 expect(answer.granted).toBe(true);
 expect(await eventually(()=>grants(directory).length===1)).toBe(true);
 const row=JSON.parse(readFileSync(join(directory,"grants",grants(directory)[0]!),"utf8"));expect(row.reservedMegabytes).toBe(3364);
 if(answer.granted)answer.release();
 expect(await eventually(()=>grants(directory).length===0)).toBe(true);
},60000);

test("a gate that is off books nothing and grants at once",async()=>{
 const directory=fresh(),answer=await createGrantGrowthPortV1({environment:{LANG:"en_US.UTF-8",SEMIO_HOST_CAPACITY:"off"},directory,policy:fast}).acquire(ask());
 expect(answer.granted).toBe(true);expect(grants(directory)).toEqual([]);
});

test("a wait the gate never answers ends at its time bound as a timeout and leaves no waiter behind",async()=>{
 const directory=fresh(),sample=async()=>({host:host({availableMegabytes:300,pressure:"critical"}),swappedOutMegabytes:0}),answer=await createGrantGrowthPortV1({environment:{LANG:"en_US.UTF-8"},directory,policy:fast,sample}).acquire(ask({waitMilliseconds:300}));
 expect(answer).toEqual({granted:false,reason:"timeout"});
 expect(await eventually(()=>queue(directory).length===0)).toBe(true);expect(grants(directory)).toEqual([]);
},60000);

test("an aborted signal cancels the wait",async()=>{
 const directory=fresh(),controller=new AbortController(),sample=async()=>({host:host({availableMegabytes:300,pressure:"critical"}),swappedOutMegabytes:0}),waiting=createGrantGrowthPortV1({environment:{LANG:"en_US.UTF-8"},directory,policy:fast,sample}).acquire(ask({signal:controller.signal}));
 expect(await eventually(()=>queue(directory).length===1)).toBe(true);
 controller.abort(new Error("cancelled by law"));
 expect(await waiting).toEqual({granted:false,reason:"cancelled"});
 expect(await eventually(()=>queue(directory).length===0)).toBe(true);
},60000);

test("the fleet is read from the group store: fresh mirrors only, an unknown store is unknown and never small",()=>{
 const store=fresh(),now=Date.now(),write=(origin:string,total:number,atMs:number)=>{const dir=join(store,"v1","origins",origin);mkdirSync(dir,{recursive:true});writeFileSync(join(dir,"memory.json"),JSON.stringify({version:1,atMs,host:{totalMegabytes:total}}));};
 write("g1-server",32768,now-1000);write("g1-m01",16384,now-2000);write("g1-m06",16384,now-10*86400000);mkdirSync(join(store,"v1","origins","g1-m09"),{recursive:true});
 const hosts=readFleetGrowthHostsV1(fast,{SEMIO_FLEET_STORE_ROOT:store},now)!;
 expect(hosts.map(row=>row.origin).sort()).toEqual(["g1-m01","g1-server"]);
 expect(hosts.find(row=>row.origin==="g1-server")).toEqual(growthHostV1(fast,{origin:"g1-server",totalMegabytes:32768}));
 expect(readFleetGrowthHostsV1(fast,{},now)).toBeUndefined();
 expect(readFleetGrowthHostsV1(fast,{SEMIO_FLEET_STORE_ROOT:join(store,"missing")},now)).toBeUndefined();
});

test("the facts of a machine follow the policy floor, the interactive reserve and the budget share",()=>{
 const facts=growthHostV1(fast,{origin:"a",totalMegabytes:32768,availableMegabytes:1000});
 expect(facts).toMatchObject({origin:"a",totalMegabytes:32768,availableMegabytes:1000,budgetBasisPoints:fast.memory.budgetBasisPoints});
 expect(facts.floorMegabytes).toBeGreaterThan(0);expect(facts.interactiveReserveMegabytes).toBeGreaterThan(0);
});
