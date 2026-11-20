/** 🌍️ Contract: machines that load the graph of the whole workspace through one real fleet cache server read it once. The first machine reads and offers its generation; every other machine, started at the same time or later, waits or fetches and infers nothing on an identical tree and exactly the changed owners on a changed tree; a question under the policy current fetches a complete head instead of refusing, and refuses on an incomplete one.
 * @see ../../🟦️.ts */
import {afterEach,expect,test} from "bun:test";
import {mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createNodeFilePortV1} from "../../../🟨️.mjs";
import {loadGenerationGraphV1} from "../../../🔌️load/🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";
import {startRemoteCacheServerV1,type RemoteCacheServerV1} from "../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import {graphFleetClientV1} from "../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import {remoteGenerationExchangeV1} from "../../../🌐️remote-exchange/🟦️.ts";
import {wholeReadGateV1} from "../../🟦️.ts";

const files=createNodeFilePortV1(),directories:string[]=[],servers:RemoteCacheServerV1[]=[],platform="darwin-arm64";
const scratch=()=>{const path=mkdtempSync(join(tmpdir(),"whole-gate-"));directories.push(path);return path;};
afterEach(async()=>{for(const server of servers.splice(0))await server.stop();for(const path of directories.splice(0))rmSync(path,{recursive:true,force:true});});
async function serve():Promise<RemoteCacheServerV1>{const server=await startRemoteCacheServerV1({directory:scratch(),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{},validation:"complete",maximumActionResultBytes:4194304});servers.push(server);return server;}
const total=(mocks:readonly ReturnType<typeof createMockHost>[])=>mocks.flatMap(mock=>mock.log.flatMap(row=>row.full)).length;
const computed=(mock:ReturnType<typeof createMockHost>)=>mock.log.flatMap(row=>row.full).sort();
const restored=(mock:ReturnType<typeof createMockHost>)=>mock.said.some(row=>/restored from the remote cache, \d+ documents/u.test(row.text));
function fleet(server:RemoteCacheServerV1,all:readonly string[],options:{gate?:boolean;ownerDelayMs?:number}={}){
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write"},lines:string[]=[];
 const mount=(workspace:string,self:string)=>{
  const mock=createMockHost(workspace,{ownerDelayMs:options.ownerDelayMs}),host=mock.host as any;
  host.exchange=remoteGenerationExchangeV1({environment,workspace,identity:()=>mock.host.identity(),platform,origins:{self,all}});
  if(options.gate!==false)host.wholeGate=(request:{identity:string;look():Promise<any>;signal?:AbortSignal})=>wholeReadGateV1({client:graphFleetClientV1({endpoint:server.url.replace(/\/+$/u,""),instance:"main",token:undefined,upload:true,stateDirectory:scratch()}),identity:request.identity,platform,owner:self,language:"en",look:request.look,signal:request.signal,say:line=>lines.push(self+": "+line),claimTtlMilliseconds:3000,claimHeartbeatMilliseconds:300,waitSliceMilliseconds:150,waitMaximumMilliseconds:60000});
  return mock;
 };
 return{mount,lines,environment};
}

test("four machines with identical trees start at once on an empty fleet: every owner is inferred once on one machine, the others fetch and infer nothing",async()=>{
 const server=await serve(),all=["g1-a","g1-b","g1-c","g1-d"],{mount,lines}=fleet(server,all,{ownerDelayMs:25}),roots=all.map(()=>scratch());
 for(const root of roots)await seed(root,ringFiles(20));
 const mocks=all.map((origin,index)=>mount(roots[index]!,origin));
 const results=await Promise.all(mocks.map(mock=>loadGenerationGraphV1(mock.host)));
 expect(total(mocks)).toBe(20);
 expect(results.filter(row=>row.stats.ownersReinferred===20).length).toBe(1);expect(results.filter(row=>row.stats.ownersReinferred===0).length).toBe(3);
 expect(mocks.filter(restored).length).toBe(3);
 for(const result of results)expect(Object.keys((result.graph as any).projects).length).toBe(20);
 expect(lines.filter(line=>line.includes("this machine reads it once")).length).toBe(1);
 expect(lines.filter(line=>line.includes("reading the whole workspace")).length).toBeGreaterThan(0);
 expect(lines.filter(line=>line.includes("fetching it instead of reading")).length).toBe(3);
},60000);

test("without the gate the same four machines read the whole workspace four times",async()=>{
 const server=await serve(),all=["g1-a","g1-b","g1-c","g1-d"],{mount}=fleet(server,all,{gate:false,ownerDelayMs:25}),roots=all.map(()=>scratch());
 for(const root of roots)await seed(root,ringFiles(20));
 const mocks=all.map((origin,index)=>mount(roots[index]!,origin));
 await Promise.all(mocks.map(mock=>loadGenerationGraphV1(mock.host)));
 expect(total(mocks)).toBeGreaterThan(20);
},60000);

test("a machine that comes later on an identical tree answers from the fleet head without one inference; with k changed files exactly those owners are inferred and offered",async()=>{
 const server=await serve(),all=["g1-a","g1-b","g1-c"],{mount,lines}=fleet(server,all),[a,b,c]=all.map(()=>scratch());
 for(const root of[a,b,c])await seed(root,ringFiles(30));
 const first=mount(a,"g1-a");await loadGenerationGraphV1(first.host);expect(total([first])).toBe(30);
 const same=mount(b,"g1-b"),started=Date.now(),again=await loadGenerationGraphV1(same.host);
 expect(restored(same)).toBe(true);expect(again.stats.ownersReinferred).toBe(0);expect(computed(same)).toEqual([]);expect(Date.now()-started).toBeLessThan(10000);
 await applyOps(c,[{op:"write",path:"pkg/p3/src/index.ts",content:'import "@syn/p1";\nexport const edited=1;\n'},{op:"write",path:"pkg/p7/src/index.ts",content:'import "@syn/p2";\nexport const edited=2;\n'}]);
 const changed=mount(c,"g1-c"),third=await loadGenerationGraphV1(changed.host);
 expect(restored(changed)).toBe(true);expect(third.stats.ownersReinferred).toBe(2);expect(computed(changed)).toEqual(["p3","p7"]);
 expect(lines.filter(line=>/^g1-[bc]/u.test(line)&&line.includes("reads it once"))).toEqual([]);expect(lines.filter(line=>/^g1-[bc]/u.test(line)&&line.includes("fetching it instead of reading")).length).toBe(2);
},60000);

test("a question under the policy current fetches the complete head the fleet holds instead of refusing, infers nothing and states the age; an incomplete head is refused",async()=>{
 const slow=await serve(),all=["g1-a","g1-b","g1-c"],a=scratch(),b=scratch();
 await seed(a,ringFiles(12));await seed(b,ringFiles(12));
 const producer=createMockHost(a),exchange=remoteGenerationExchangeV1({environment:{SEMIO_REMOTE_CACHE_ENDPOINT:slow.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write"},workspace:a,identity:()=>producer.host.identity(),platform,origins:{self:"g1-a",all}})!;
 await loadGenerationGraphV1(producer.host);await exchange.publish({root:join(a,".gen"),fs:files,stale:5});
 const incomplete=fleet(slow,all).mount(b,"g1-b"),refusal=await loadGenerationGraphV1(incomplete.host,{policy:"current"}).then(()=>undefined,error=>error as Error&{code?:string});
 expect(refusal?.code).toBe("GENERATION_NONE_SEALED");expect(computed(incomplete)).toEqual([]);expect(restored(incomplete)).toBe(false);
 const server=await serve(),{mount}=fleet(server,all),c=scratch(),d=scratch();
 await seed(c,ringFiles(12));await seed(d,ringFiles(12));
 await loadGenerationGraphV1(mount(c,"g1-c").host);
 const asking=mount(d,"g1-d"),before=asking.log.length,answered=await loadGenerationGraphV1(asking.host,{policy:"current"});
 expect(Object.keys((answered.graph as any).projects).length).toBe(12);expect(asking.log.length).toBe(before);expect(answered.stats.ownersReinferred).toBe(0);
 expect(asking.said.some(row=>row.key==="sealedAge")).toBe(true);expect(restored(asking)).toBe(true);
 const second=mount(d,"g1-d"),again=await loadGenerationGraphV1(second.host,{policy:"current"});
 expect(Object.keys((again.graph as any).projects).length).toBe(12);expect(restored(second)).toBe(false);
},60000);

test("a question under the policy current with a read going on here is refused as before and never waits for the custody",async()=>{
 const server=await serve(),all=["g1-a","g1-b"],{mount}=fleet(server,all),[a,b]=all.map(()=>scratch());
 for(const root of[a,b])await seed(root,ringFiles(8));
 await loadGenerationGraphV1(mount(a,"g1-a").host);
 const busy=mount(b,"g1-b");(busy.host as any).builderIdle=async()=>false;
 const refusal=await loadGenerationGraphV1(busy.host,{policy:"current"}).then(()=>undefined,error=>error as Error&{code?:string});
 expect(refusal?.code).toBe("GENERATION_NONE_SEALED");expect(busy.events.some(event=>event.startsWith("builder"))).toBe(false);
},60000);
