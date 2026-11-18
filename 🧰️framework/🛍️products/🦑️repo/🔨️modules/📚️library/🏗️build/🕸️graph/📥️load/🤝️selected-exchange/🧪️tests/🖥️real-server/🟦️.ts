/** 🖥️ Contract: the exchange works against the real fleet cache server (blobs, head by compare-and-swap, claims with a lease, waiting for a stored head), not only against a model of it.
 * @see ../../🟦️.ts */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{graphFleetClientV1}from"../../../../../🪪️graph-state/🛰️exchange/🟦️.ts";
import{selectedGenerationV1}from"../../🟦️.ts";
import{makeMachine,makeRecords,ownerOfV1,sha}from"../🌍️world/🟦️.ts";

const scratch:string[]=[],temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`selected-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const scope=sha("real-server-scope"),platform="darwin-arm64";

async function start(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const client=()=>graphFleetClientV1({endpoint:server.url.replace(/\/+$/u,""),instance:"main",token:"w",upload:true,stateDirectory:temp("state")});
 return{server,client};
}
const run=(client:ReturnType<typeof graphFleetClientV1>,machine:ReturnType<typeof makeMachine>,extra:Partial<Parameters<typeof selectedGenerationV1>[0]>={})=>selectedGenerationV1({client,scope,platform,host:machine.host,owner:machine.name,ownerOf:ownerOfV1,holds:machine.holds,install:machine.install,read:machine.read,waitSliceMilliseconds:300,waitMaximumMilliseconds:20000,...extra});

test("a second machine installs the generation the first one published to the real server and reads nothing",async()=>{
 const{server,client}=await start();
 try{
  const records=makeRecords(),a=makeMachine({name:"a",count:25,records}),b=makeMachine({name:"b",count:25,records});
  const first=await run(client(),a);
  expect(first.outcome).toBe("read");expect(first.published).toBe(true);
  const second=await run(client(),b);
  expect(second.outcome).toBe("installed");expect(b.counters).toEqual({observations:0,inferences:0,takes:0,reads:0});expect(b.graph).toBe(a.graph.replaceAll(a.root,b.root));
 }finally{await server.stop();}
},60000);

test("two machines start at once on the real server: one reads, the other waits for the stored head and takes it, each owner is inferred once",async()=>{
 const{server,client}=await start();
 try{
  const records=makeRecords(),a=makeMachine({name:"a",count:30,records}),b=makeMachine({name:"b",count:30,records}),said:string[]=[];
  const slow=async(differing:readonly string[]|undefined)=>{await Bun.sleep(1200);return a.read(differing);};
  const[one,two]=await Promise.all([run(client(),a,{read:slow}),run(client(),b,{say:key=>said.push(key)})]);
  expect([one.outcome,two.outcome].sort()).toEqual(["read","waited"]);
  expect(said).toContain("waiting");
  expect([...records.inferences.values()].every(count=>count===1)).toBe(true);expect(a.counters.observations+b.counters.observations).toBe(30);
 }finally{await server.stop();}
},60000);

test("a machine whose tree moved publishes the newest head by compare-and-swap, and the next machine converges on it",async()=>{
 const{server,client}=await start();
 try{
  const records=makeRecords(),a=makeMachine({name:"a",count:20,records}),b=makeMachine({name:"b",count:20,records}),c=makeMachine({name:"c",count:20,records});
  await run(client(),a);
  b.edit(7,"moved on b");
  const second=await run(client(),b);expect(second.outcome).toBe("repaired");expect(second.differing).toEqual(["o7"]);expect(second.published).toBe(true);
  c.edit(7,"moved on b");
  const third=await run(client(),c);expect(third.outcome).toBe("installed");expect(c.counters.reads).toBe(0);
 }finally{await server.stop();}
},60000);
