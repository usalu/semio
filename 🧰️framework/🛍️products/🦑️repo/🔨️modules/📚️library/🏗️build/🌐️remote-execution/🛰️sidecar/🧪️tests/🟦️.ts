import {afterEach,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {dirname} from "node:path";
import {cacheAreaDirectoryV1,cacheLedgerV1} from "../../../../⚡️caching/💾️storage/🧾️ledger/🟦️.ts";
import {createServer} from "node:net";
import {join} from "node:path";
import {registeredToolExecutableV1} from "../../../../../../../../🔨️modules/🏃️process/🧰️tools/🏃️run/🟦️.ts";
import {testScratchRootV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {publishFleetCacheFactsV1} from "../../../🌐️remote-cache/🛰️fleet/📣️publication/🟦️.ts";
import {fleetHeadersV1,fleetSignedFetchV1,readFleetOriginKeyV1,syncFleetIdentityV1} from "../../../🌐️remote-cache/🛰️fleet/🪪️identity/🟦️.ts";
import {createSealedFetchV1} from "../../../🌐️remote-cache/🔐️sealed/🟦️.ts";
import {refreshFleetCacheFactsV1,remoteCacheFetchV1} from "../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import {readRemoteExecutionFleetFactsV1} from "../../🚢️status/🟦️.ts";
import {connectGrpcV1,GRPC_CODES_V1} from "../../📡️wire/🚪️grpc/🟦️.ts";
import {eventuallyV1,NAMESPACE_V1,writeHostViewV1} from "../../🧪️tests/🧰️cluster/🟦️.ts";
import {stretchedBudgetMillisecondsV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎚️budget/🟦️.ts";
import {remoteExecutionPortsV1,remoteExecutionWorkerApiV1} from "../🟦️.ts";

import topology from "../../../../../🤝️shared-worktree/🗺️topology/🔣️.json" with {type:"json"};
process.env.SEMIO_FLEET_DISCOVERY="off";
const seed=(root:string,service:"cache"|"execution",endpoint:string)=>{mkdirSync(join(root,".cache","f"),{recursive:true});const file=join(root,".cache","f","last-good.json");let known:Record<string,string>={};try{known=JSON.parse(readFileSync(file,"utf8"));}catch{}writeFileSync(file,JSON.stringify({...known,[`g1-server/${service}`]:endpoint}));};
const bun=registeredToolExecutableV1("bun"),sidecar=join(import.meta.dir,"..","🟦️.ts"),scratch:string[]=[],children:ReturnType<typeof Bun.spawn>[]=[];
let cache:RemoteCacheServerV1|undefined;
const detachers:(()=>void)[]=[];
const attachPartitions=(worktree:string,areas:readonly string[]):void=>{
 if(process.platform!=="darwin")return;
 const ledger=cacheLedgerV1(),mounts=new Set<string>();
 for(const id of areas){
  const area=ledger.areas.find(row=>row.id===id)!,found=cacheAreaDirectoryV1(worktree,ledger,area)!,depth=area.directory===undefined?0:area.directory.split("/").filter(Boolean).length;
  let mount=found;for(let step=0;step<depth;step++)mount=dirname(mount);
  mounts.add(mount);
 }
 for(const mount of mounts){
  mkdirSync(mount,{recursive:true});
  const image=`${mount}.test-partition.sparseimage`;
  expect(spawnSync("hdiutil",["create","-type","SPARSE","-size","24g","-fs","APFS","-volname","semio-test","-ov",image]).status,"hdiutil create").toBe(0);
  expect(spawnSync("hdiutil",["attach",image,"-mountpoint",mount,"-nobrowse","-noverify"]).status,"hdiutil attach").toBe(0);
  detachers.push(()=>{spawnSync("hdiutil",["detach",mount,"-force"]);});
 }
};
afterEach(async()=>{for(const child of children.splice(0))child.kill("SIGKILL");await Bun.sleep(300);for(const detach of detachers.splice(0))detach();await cache?.stop().catch(()=>{});cache=undefined;for(const path of scratch.splice(0))rmSync(path,{recursive:true,force:true});});
const freePort=async():Promise<number>=>{
 for(;;){
  const port=21000+Math.floor(Math.random()*20000);
  if(await new Promise<boolean>(done=>{const server=createServer();server.once("error",()=>{done(false);});server.listen(port,"0.0.0.0",()=>{server.close(()=>{done(true);});});}))return port;
 }
};

test("the scheduler role publishes its endpoint, serves only signed requests on the fleet interface; the agent role finds it through the facts, joins with its own signature, reports its load, and a forged or unsigned caller is refused; both leave in order",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("rexec-sidecar"),"s-"));scratch.push(directory);
 const worktree=join(directory,"worktree"),member=join(directory,"member"),store=join(directory,"store");
 attachPartitions(member,["remote-execution-worker"]);
 for(const [root,origin] of[[worktree,"g1-server"],[member,"g1-m09"]] as const){mkdirSync(join(root,".cache","f"),{recursive:true});mkdirSync(join(store,"v1","origins",origin),{recursive:true});writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:"g1",storeRoot:store,key:"0".repeat(32)}));expect((await syncFleetIdentityV1({root})).error).toBeUndefined();}
 cache=await startRemoteCacheServerV1({directory:join(directory,"cache"),host:"127.0.0.1",port:0,maximumBytes:1<<30,maximumEntryBytes:1<<28,maximumAgeDays:30,defaultInstance:"main",tokens:{},fleet:{storeRoot:store,group:"g1",networks:["127.0.0.0/8"],registryTtlMilliseconds:200}});
 await publishFleetCacheFactsV1({admin:{role:"admin",originId:"g1-server"},storeRoot:store,directory:join(directory,"cache-data"),group:"g1",instance:"main",port:Number(new URL(cache.url).port)});
 const ports=[await freePort(),await freePort()];
 for(const root of[worktree,member]){seed(root,"cache",cache.url);seed(root,"execution",`http://127.0.0.1:${ports[0]}`);}
 for(const root of[worktree,member])expect((await refreshFleetCacheFactsV1({root,force:true})).state).toBe("updated");
 const environment={...process.env,SEMIO_FLEET_ROOT:worktree,SEMIO_WORKSPACE_ROOT:worktree,SEMIO_FLEET_ORIGIN:"g1-server",SEMIO_FLEET_GROUP:"g1",SEMIO_FLEET_STORE_ROOT:store,SEMIO_FLEET_NETWORK:"127.0.0.0/8",SEMIO_REMOTE_EXECUTION_PORTS:ports.join(","),SEMIO_REMOTE_EXECUTION_HOST_VIEW:writeHostViewV1(join(directory,"view.json"))};
 expect(remoteExecutionPortsV1(environment)).toEqual({grpc:ports[0]!,worker:ports[1]!});
 const factsPath=join(store,"v1","fleet","cache.json"),execution=():{origin:string;port:number;protocolVersion:number}|undefined=>(JSON.parse(readFileSync(factsPath,"utf8")) as {execution?:{origin:string;port:number;protocolVersion:number}}).execution,endpoint=`http://127.0.0.1:${ports[0]}`;
 const scheduler=Bun.spawn([bun,sidecar,"scheduler"],{stdout:"ignore",stderr:"ignore",cwd:directory,env:environment});children.push(scheduler);
 await eventuallyV1(()=>execution()!==undefined,stretchedBudgetMillisecondsV1(30000),"the scheduler must publish its endpoint");
 expect(execution()).toEqual({origin:"g1-server",port:ports[0]!,protocolVersion:1});
 expect(readFileSync(factsPath,"utf8").toLowerCase(),"the facts carry no secret").not.toContain("token");
 const api=remoteExecutionWorkerApiV1(endpoint,ports[1]!),anonymous=await fetch(`${api}/v1/fleet/load`);expect(anonymous.status,"the load table needs a signature").toBe(401);
 expect((await fetch(`${api}/v1/fleet/load`,{headers:{authorization:"Bearer "+"A".repeat(43)}})).status,"a bearer token is no credential").toBe(401);
 const grpc=connectGrpcV1(endpoint);
 try{await grpc.unary(`${NAMESPACE_V1}.Capabilities/GetCapabilities`,{});throw Error("an unsigned gRPC call must fail");}catch(error){expect((error as {code:number}).code).toBe(GRPC_CODES_V1.UNAUTHENTICATED);}finally{grpc.close();}
 await refreshFleetCacheFactsV1({root:member,force:true});
 const agentEnvironment={...environment,SEMIO_FLEET_ROOT:member,SEMIO_WORKSPACE_ROOT:member,SEMIO_FLEET_ORIGIN:"g1-m09"};
 const agent=Bun.spawn([bun,sidecar,"agent"],{stdout:"ignore",stderr:"ignore",cwd:directory,env:agentEnvironment});children.push(agent);
 const signed=remoteCacheFetchV1({SEMIO_WORKSPACE_ROOT:member},{root:member,audience:"execution",roleOf:()=>"submit"});
 const load=async():Promise<any>=>(await signed(`${api}/v1/fleet/load`)).json();
 await eventuallyV1(async()=>(await load()).machines.some((row:any)=>row.origin==="g1-m09"&&row.worker.state==="idle"),stretchedBudgetMillisecondsV1(40000),"the agent must join and report");
 const machine=(await load()).machines.find((row:any)=>row.origin==="g1-m09");
 expect(machine).toMatchObject({platform:{os:expect.any(String)},worker:{confinement:expect.stringMatching(/full|none/u),network:expect.stringMatching(/denied|unenforced/u)}});
 const facts=await readRemoteExecutionFleetFactsV1({root:member,environment:{SEMIO_REMOTE_EXECUTION_PORTS:ports.join(",")}});
 expect(facts?.machines.some(row=>row.origin==="g1-m09"),"`repo fleet status` reads the table with the signed scheme through the published facts").toBe(true);
 const forged=JSON.parse(JSON.stringify(machine));forged.origin="g1-server";delete forged.ageSeconds;
 const lie=await signed(`${api}/v1/fleet/load`,{method:"POST",headers:{"content-type":"application/json"},body:JSON.stringify(forged)});
 expect([400,403],"a report that names another origin than the signer is refused").toContain(lie.status);
 const rpc=(content?:string)=>connectGrpcV1(endpoint,{sign:(method,hash)=>fleetHeadersV1(member,"submit","execution","POST",`http://x/${method}`,content??hash)});
 const honest=rpc();try{expect((await honest.unary(`${NAMESPACE_V1}.Capabilities/GetCapabilities`,{})).execution_capabilities.exec_enabled).toBe(true);}finally{honest.close();}
 const swapped=rpc("a".repeat(64));try{await swapped.unary(`${NAMESPACE_V1}.Capabilities/GetCapabilities`,{});throw Error("messages differing from the signed hash must be refused");}catch(error){expect((error as {code:number}).code,"signed hash of other messages").toBe(GRPC_CODES_V1.UNAUTHENTICATED);}finally{swapped.close();}
 scheduler.kill("SIGKILL");await scheduler.exited;await Bun.sleep(3000);
 const restarted=Bun.spawn([bun,sidecar,"scheduler"],{stdout:"ignore",stderr:"ignore",cwd:directory,env:environment});children.push(restarted);
 await eventuallyV1(async()=>{try{return (await load()).machines.some((row:any)=>row.origin==="g1-m09"&&row.worker.state==="idle");}catch{return false;}},stretchedBudgetMillisecondsV1(60000),"after a scheduler restart the agent's worker registers again by itself (no 403 loop on its old id)");
 expect(agent.exitCode,"the agent did not restart").toBeNull();
 agent.kill("SIGTERM");expect(await agent.exited).toBe(0);
 await eventuallyV1(async()=>!(await load()).machines.some((row:any)=>row.origin==="g1-m09"&&row.worker.state!=="absent"),stretchedBudgetMillisecondsV1(20000),"the agent must leave the scheduler");
 restarted.kill("SIGTERM");expect(await restarted.exited).toBe(0);
 expect(execution(),"a scheduler that stops in order withdraws its endpoint").toBeUndefined();
},stretchedBudgetMillisecondsV1(120000));

test("an agent whose lease nobody renews leaves by itself, and one whose lease is renewed stays",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("rexec-sidecar"),"p-"));scratch.push(directory);
 const lease=join(directory,"agent.lease");let held="";const write=():void=>{if(held!=="")writeFileSync(held,JSON.stringify({version:1,pid:process.pid,wallMs:Date.now()}));};
 const agent=Bun.spawn([bun,sidecar,"agent"],{stdout:"ignore",stderr:"pipe",cwd:directory,env:{...process.env,SEMIO_FLEET_ROOT:directory,SEMIO_WORKSPACE_ROOT:directory,SEMIO_FLEET_SIDECAR_LEASE:lease,SEMIO_FLEET_SIDECAR_LEASE_MS:"4000"}});children.push(agent);
 held=join(directory,`agent.${agent.pid}.lease`);write();
 for(let round=0;round<6;round++){await Bun.sleep(1000);write();}
 expect(agent.exitCode,"a renewed lease keeps the agent").toBeNull();
 const result=await Promise.race([agent.exited,Bun.sleep(30000).then(()=>"alive")]);
 expect(result,"the agent ends within seconds of the last renewal").toBe(0);
});

test("the cache role is a supervised server of the current code: it binds the fleet interface only, trusts the registered origins, is the only publisher of the facts and keeps its index in the local state",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("rexec-sidecar"),"c-"));scratch.push(directory);
 const worktree=join(directory,"worktree"),member=join(directory,"member"),store=join(directory,"store");
 attachPartitions(worktree,["fleet-cache-index"]);
 for(const [root,origin] of[[worktree,"g1-server"],[member,"g1-m09"]] as const){mkdirSync(join(root,".cache","f"),{recursive:true});mkdirSync(join(store,"v1","origins",origin),{recursive:true});writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:"g1",storeRoot:store,key:"0".repeat(32)}));expect((await syncFleetIdentityV1({root})).error).toBeUndefined();}
 const port=await freePort(),environment={...process.env,SEMIO_FLEET_ROOT:worktree,SEMIO_WORKSPACE_ROOT:worktree,SEMIO_FLEET_NETWORK:"127.0.0.0/8",SEMIO_REMOTE_CACHE_FLEET_PORT:String(port)};
 const factsPath=join(store,"v1","fleet","cache.json"),child=Bun.spawn([bun,sidecar,"cache"],{stdout:"ignore",stderr:"ignore",cwd:directory,env:environment});children.push(child);
 await eventuallyV1(()=>{try{return JSON.parse(readFileSync(factsPath,"utf8")).publisher.port===port;}catch{return false;}},30000,"the cache sidecar must publish the facts");
 const facts=JSON.parse(readFileSync(factsPath,"utf8"));
 expect(facts.publisher.directory).toBe(join(worktree,".cache","b","p","m","fleet","cache-server"));expect(facts.publisher.origin).toBe("g1-server");expect(readFileSync(factsPath,"utf8").toLowerCase()).not.toContain("token");
 expect((await fetch(`http://127.0.0.1:${port}/status`)).status,"unsigned").toBe(401);
 seed(member,"cache",`http://127.0.0.1:${port}`);await refreshFleetCacheFactsV1({root:member,force:true});
 const signed=remoteCacheFetchV1({SEMIO_WORKSPACE_ROOT:member},{root:member});
 let status=0;for(let wait=0;wait<60&&status!==200;wait++){status=(await signed(`http://127.0.0.1:${port}/status`)).status;if(status!==200)await Bun.sleep(100);}
 expect(status,"a registered member of the group store").toBe(200);
 const claim=await signed(`http://127.0.0.1:${port}/main/claim/nx/abc123`,{method:"POST",body:JSON.stringify({owner:"live-1",ttlMilliseconds:30000})});
 expect(claim.status,"the claim routes exist in the current code").toBe(200);
 const document=await (await signed(`http://127.0.0.1:${port}/status`)).json() as {claims:{active:number};fleet:{origins:Record<string,number>}};
 expect(document.claims.active).toBe(1);
 const slot=await signed(`http://127.0.0.1:${port}/main/v1/slots`,{method:"POST",body:JSON.stringify({entries:[{kind:"nx",key:"abc123",slot:"a".repeat(64)}]})});
 expect(slot.status,"the slot route exists and answers a signed claim").toBe(200);
 child.kill("SIGTERM");expect(await child.exited).toBe(0);
},120000);

test("the cache role takes its port from the topology row of its group, serves a peer group's reader and writes no address of its own into the code",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("rexec-sidecar"),"t-"));scratch.push(directory);
 const worktree=join(directory,"worktree"),peerWorktree=join(directory,"peer"),store=join(directory,"store"),peerStore=join(directory,"peer-store"),port=await freePort();
 attachPartitions(worktree,["fleet-cache-index"]);
 const row=(group:any)=>group.id==="g1"?{...group,ports:{...group.ports,cache:port},nfs:{...group.nfs,export:"/srv/semio-store"},server:{...group.server,platform:"linux"},peers:{...group.peers,g2:peerStore}}:group;
 const folder=join(worktree,"🧰️framework","🛍️products","🦑️repo","🔨️modules","🤝️shared-worktree","🗺️topology");mkdirSync(folder,{recursive:true});
 writeFileSync(join(folder,"🔣️.json"),JSON.stringify({...topology,groups:topology.groups.map(row)}));
 for(const [root,origin,group,where] of[[worktree,"g1-server","g1",store],[peerWorktree,"g2-m01","g2",peerStore]] as const){mkdirSync(join(root,".cache","f"),{recursive:true});mkdirSync(join(where,"v1","origins",origin),{recursive:true});writeFileSync(join(root,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:group,storeRoot:where,key:"0".repeat(32)}));expect((await syncFleetIdentityV1({root})).error).toBeUndefined();}
 const environment={...process.env,SEMIO_FLEET_ROOT:worktree,SEMIO_WORKSPACE_ROOT:worktree,SEMIO_FLEET_NETWORK:"127.0.0.0/8"};
 delete (environment as Record<string,string|undefined>).SEMIO_REMOTE_CACHE_FLEET_PORT;
 const factsPath=join(store,"v1","fleet","cache.json"),child=Bun.spawn([bun,sidecar,"cache"],{stdout:"ignore",stderr:"ignore",cwd:directory,env:environment});children.push(child);
 await eventuallyV1(()=>{try{return JSON.parse(readFileSync(factsPath,"utf8")).publisher.port===port;}catch{return false;}},30000,"the cache sidecar must publish the facts on the port of the topology");
 const facts=JSON.parse(readFileSync(factsPath,"utf8")) as {cache:{transport?:string}};
 const stranger=await fetch(`http://127.0.0.1:${port}/status`);expect(stranger.status).toBe(401);
 await Bun.sleep(1200);
 const hash="a".repeat(64),plain=fleetSignedFetchV1({root:peerWorktree,audience:"cache"}),reader=fleetSignedFetchV1({root:peerWorktree,audience:"cache",fetch:createSealedFetchV1({group:"g1",serverKey:()=>readFleetOriginKeyV1([store],"g1-server")})});
 expect(facts.cache.transport,"the group's policy for its own machines is published").toBe("trusted-lan");
 let status=0;for(let wait=0;wait<60&&status!==404;wait++){status=(await reader(`http://127.0.0.1:${port}/main/cas/${hash}`)).status;if(status!==404)await Bun.sleep(100);}
 expect(status,"a machine of the peer group, known only through the mounted peer store, may read inside the sealed exchange").toBe(404);
 expect((await plain(`http://127.0.0.1:${port}/main/cas/${hash}`)).status,"and not outside it").toBe(403);
 expect((await reader(`http://127.0.0.1:${port}/main/cas/${hash}`,{method:"PUT",body:"x"})).status,"but not write").toBe(403);
 child.kill("SIGTERM");expect(await child.exited).toBe(0);
},120000);

test("a stalled event loop is reported with its length and the open files, in both languages, and a loop that keeps its beat says nothing",async()=>{
 const {createLoopLagWatchV1}=await import("../🧰️shared/🟦️.ts");
 let clock=0;const lines:string[]=[],watch=createLoopLagWatchV1({intervalMilliseconds:1000,thresholdMilliseconds:1500,now:()=>clock,openFiles:()=>830,report:line=>lines.push(line)});
 for(let beat=0;beat<5;beat++){clock+=1100;watch.tick();}
 expect(lines).toEqual([]);
 clock+=10_000;watch.tick();
 expect(lines.length).toBe(1);expect(lines[0]).toContain("stalled for 9000 ms");expect(lines[0]).toContain("830 files open");expect(lines[0]).toContain("stand 9000 ms still");
});

test("a server that answers hundreds of requests a second wakes its restart check only when a restart waits for the last request, and at most once a second",async()=>{
 const {createActivityWakeV1}=await import("../🧰️shared/🟦️.ts");
 let clock=0,wakes=0;const work={busy:false,restartPending:false,wake:()=>{wakes++;}},activity=createActivityWakeV1(work,{now:()=>clock});
 for(let request=0;request<500;request++){activity(1);clock+=2;activity(0);}
 expect(wakes,"nothing waits for the server: the code is never scanned").toBe(0);expect(work.busy).toBe(false);
 work.restartPending=true;
 for(let request=0;request<500;request++){activity(1);expect(work.busy).toBe(true);clock+=2;activity(0);}
 expect(wakes,"1000 ms of requests wake the check once, not 500 times").toBe(1);
 clock+=1500;activity(1);activity(0);expect(wakes).toBe(2);
 activity(2);activity(1);expect(wakes,"still busy: no wake").toBe(2);
 expect(()=>createActivityWakeV1(undefined)(1)).not.toThrow();
});

test("a late event loop names what it was busy in: lines below the stall threshold are rate limited, a stall always says it, and the real sampling profiler sees the function that kept the loop",async()=>{
 const {createLoopLagWatchV1,createStallProfilerV1}=await import("../🧰️shared/🟦️.ts");
 let clock=0;const lines:string[]=[],watch=createLoopLagWatchV1({intervalMilliseconds:1000,thresholdMilliseconds:1500,attributeMilliseconds:400,attributeEveryMilliseconds:5000,now:()=>clock,openFiles:()=>12,profile:()=>()=>"7x spin@a/b.ts:3",report:line=>lines.push(line)});
 clock+=1100;watch.tick();expect(lines).toEqual([]);
 clock+=1600;watch.tick();expect(lines.length).toBe(1);expect(lines[0]).toContain("late");expect(lines[0]).toContain("busy in 7x spin@a/b.ts:3");expect(lines[0]).toContain("beschäftigt in 7x spin@a/b.ts:3");
 clock+=1600;watch.tick();expect(lines.length,"rate limited").toBe(1);
 clock+=6000;watch.tick();expect(lines.length).toBe(2);
 clock+=3000;watch.tick();expect(lines.length).toBe(3);expect(lines[2]).toContain("stalled for 2000 ms");expect(lines[2]).toContain("12 files open");expect(lines[2]).toContain("busy in 7x spin@a/b.ts:3");
 const profile=await createStallProfilerV1();
 if(profile===undefined)return;
 profile();
 function keepTheLoopBusy():number{let sum=0;for(let index=0;index<8e7;index++)sum+=index%7;return sum;}
 expect(keepTheLoopBusy()).toBeGreaterThan(0);
 expect(profile()()).toContain("keepTheLoopBusy");
});
