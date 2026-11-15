import{expect,test}from"bun:test";
import{chmodSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{wantedServerPolicyV1}from"../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import{remoteExecutionSidecarEntriesV1}from"../../🟦️.ts";
import{graphExchangeEnvironmentV1,graphServerCommandV1,graphServerHeadroomV1,inferGraphReadV1,superviseGraphServerV1}from"../../🕸️graph/🟦️.ts";

/** 🕸️ Contract: the graph server is a role of the build sidecars with a module of its own (its code, and only its code, decides its restarts), and it starts graph reads with the `repo` the machine names. */
test("the graph role loads its own module and watches only that closure",()=>{
 const{entries,staticOnly}=remoteExecutionSidecarEntriesV1("graph");
 expect(entries.map(path=>path.split("/").slice(-2).join("/"))).toEqual(["🛰️sidecar/🟦️.ts","🕸️graph/🟦️.ts"]);
 expect(staticOnly).toEqual([join(entries[0]!)]);
});

test("the command of a graph read is the named repo, else the installed one",()=>{
 expect(graphServerCommandV1({SEMIO_REPO_BIN:"/opt/repo"})).toBe("/opt/repo");
 expect(graphServerCommandV1({})).toMatch(/repo(\.exe)?$/u);
});

test("the graph server beside the agent restarts after a failure without reaching the agent and stays off when switched off",async()=>{
 const controller=new AbortController(),lines:string[]=[];let starts=0;
 const context={root:"/nowhere",environment:{} as Record<string,string>,signal:controller.signal,write:(line:string)=>lines.push(line)};
 await superviseGraphServerV1(context,async()=>{if(++starts===3)controller.abort();throw new Error("no fleet cache");},1);
 expect(starts).toBe(3);expect(lines).toEqual(["graph-server stopped: no fleet cache","graph-server stopped: no fleet cache","graph-server stopped: no fleet cache"]);
 let off=0;await superviseGraphServerV1({...context,signal:new AbortController().signal,environment:{SEMIO_GRAPH_SERVER:"off"}},async()=>{off++;},1);
 expect(off).toBe(0);
});

test("the fleet's 16 GB workers never serve the graph: the serving memory lies above their physical memory and a requested read needs less than a refresh",()=>{
 expect(wantedServerPolicyV1.serveTotalMegabytes).toBeGreaterThan(16384);
 expect(wantedServerPolicyV1.requestHeadroomMegabytes).toBeLessThan(wantedServerPolicyV1.refreshHeadroomMegabytes);
 expect(graphServerHeadroomV1({totalMegabytes:16384,availableMegabytes:7000,pressure:"normal"},wantedServerPolicyV1.serveTotalMegabytes)).toEqual({freeMegabytes:0,pressure:"normal"});
 expect(graphServerHeadroomV1({totalMegabytes:32768,availableMegabytes:7000,pressure:"normal"},wantedServerPolicyV1.serveTotalMegabytes)).toEqual({freeMegabytes:7000,pressure:"normal"});
});

test("a machine below the serving memory offers no headroom, a larger one its available memory",()=>{
 expect(graphServerHeadroomV1({totalMegabytes:16384,availableMegabytes:9992,pressure:"normal"},24000)).toEqual({freeMegabytes:0,pressure:"normal"});
 expect(graphServerHeadroomV1({totalMegabytes:32768,availableMegabytes:9992.4,pressure:"normal"},24000)).toEqual({freeMegabytes:9992,pressure:"normal"});
});

const fakeRepo=(script:string)=>{const dir=mkdtempSync(join(tmpdir(),"graph-infer-")),path=join(dir,"repo");writeFileSync(path,"#!/bin/sh\n"+script+"\n");chmodSync(path,0o755);return{dir,path};};
const readOf=(command:string,over:object={})=>inferGraphReadV1({command,cwd:tmpdir(),environment:{},reason:"refresh",signal:undefined,language:"en",policy:{...wantedServerPolicyV1,inferStallMilliseconds:700,watchMilliseconds:40},...over});

test("a graph read of the server is not cut while it publishes waves, although it runs three times as long as the stall limit",async()=>{
 const{dir,path}=fakeRepo('i=1\nwhile [ $i -le 9 ]; do echo "graph: generation $i published, 48 projects refreshed in 80 s (0 kept their edges)" >&2; i=$((i+1)); sleep 0.25; done\nexit 0');
 try{
  const started=Date.now(),result=await readOf(path);
  expect(Date.now()-started).toBeGreaterThan(2000);
  expect(result).toMatchObject({ok:true,status:0,generation:9,detail:"refresh"});
 }finally{rmSync(dir,{recursive:true,force:true});}
},30000);

test("a graph read that publishes nothing for the stall limit is stopped and says so; the generation of its last wave is kept in the answer",async()=>{
 const{dir,path}=fakeRepo('echo "graph: generation 4 published, 48 projects refreshed in 80 s (0 kept their edges)" >&2\nexec sleep 60');
 try{
  const started=Date.now(),result=await readOf(path);
  expect(Date.now()-started).toBeLessThan(15000);
  expect(result.ok).toBe(false);expect(result.generation).toBe(4);expect(result.detail).toContain("no wave was published for");
  const german=await readOf(path,{language:"de"});expect(german.detail).toContain("seit ");
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("a graph read that fails reports the last line of its output, and one cancelled from outside is stopped at once; the read is started as a background refresh without a fleet request",async()=>{
 const{dir,path}=fakeRepo('echo "SEMIO_GRAPH_BACKGROUND=$SEMIO_GRAPH_BACKGROUND SEMIO_GRAPH_FLEET_READ=$SEMIO_GRAPH_FLEET_READ" >&2\necho "boom: the store is full" >&2\nexit 3');
 try{
  const failed=await readOf(path);expect(failed).toMatchObject({ok:false,status:3,generation:null,detail:"boom: the store is full"});
  const quiet=fakeRepo('echo started >&2\nexec sleep 60'),controller=new AbortController();
  try{const reading=readOf(quiet.path,{signal:controller.signal,policy:{...wantedServerPolicyV1,inferStallMilliseconds:600000,watchMilliseconds:40}});await new Promise(resolve=>setTimeout(resolve,300));controller.abort();const result=await reading;expect(result.ok).toBe(false);}finally{rmSync(quiet.dir,{recursive:true,force:true});}
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("the graph exchange of the server runs in the instance the fleet announces: the agent's explicit remote cache overrides are dropped, everything else stays",()=>{
 const agent={SEMIO_REMOTE_CACHE_ENDPOINT:"http://x:6090",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_INSTANCE:"semio",SEMIO_REMOTE_CACHE_TOKEN:"t",SEMIO_FLEET_CACHE_ENDPOINT:"http://y",SEMIO_FLEET_GROUP:"g1",SEMIO_FLEET_STORE_ROOT:"/store",PATH:"/bin"};
 expect(graphExchangeEnvironmentV1(agent)).toEqual({SEMIO_FLEET_GROUP:"g1",SEMIO_FLEET_STORE_ROOT:"/store",PATH:"/bin"});
 expect(agent.SEMIO_REMOTE_CACHE_INSTANCE).toBe("semio");
});
