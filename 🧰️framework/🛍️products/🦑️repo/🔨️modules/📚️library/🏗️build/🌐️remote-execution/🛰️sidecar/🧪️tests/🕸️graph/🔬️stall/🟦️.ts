import{expect,test}from"bun:test";
import{chmodSync,mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{wantedServerPolicyV1}from"../../../../../🪪️graph-state/🙋️wanted/🛠️server/🟦️.ts";
import{wantedTextV1}from"../../../../../🪪️graph-state/🙋️wanted/🟦️.ts";
import{inferGraphReadV1}from"../../../🕸️graph/🟦️.ts";
import{awaitGraphReadLeasesV1,describeGraphReadStallV1,graphReadBlockersV1,graphRefreshCustodyHeldV1}from"../../../🕸️graph/🔬️stall/🟦️.ts";
import{Database}from"bun:sqlite";
import refreshCases from "./refresh.json" with {type:"json"};

/** 🔬️ Contract: the serving read of the group server never waits silently: a start behind an exclusive hold of another command is told with the holder and bounded, a read that prints nothing is explained from the lease registry and the capacity gate, and the read runs in the highest class. */
const NOW=1_794_722_160_000,MINUTE=60_000;
const entry=(pid:number,command:string,extra:object={})=>({version:1,pid,startedAtMs:NOW-900_000,updatedAtMs:NOW-1000,command,directory:"/w/.cache/b/l/m/data",standing:{version:1,priority:"agent",phase:"x",progressAtMs:NOW-1000,parentPid:1},holds:[],waits:[],...extra}) as never;
const IMAGE="native-fixed-image:/w/.cache/b/d",OTHER="native-fixed-image:/w/.cache/b/o";

test("refresh suppression needs live held structural custody in the exact generation root",()=>{
 const root=mkdtempSync(join(tmpdir(),"r-")),generationsRoot=join(root,"g"),leaseDirectory=join(root,"l"),directory=join(generationsRoot,"s"),token="a".repeat(24),file=join(directory,token+".json");
 const input={generationsRoot,leaseDirectory,environment:{},signal:new AbortController().signal},db=new Database(":memory:"),pid=30229,startedAtMs=NOW-900_000;
 try{
  mkdirSync(directory,{recursive:true});
  for(const row of refreshCases.claims){
   const resource="graph-custody:"+(row.mode==="other-root"?join(root,"elsewhere"):generationsRoot)+":"+token;
   const claim={version:1,token:row.mode==="other-token"?"b".repeat(24):token,ticket:1,queuedMs:NOW-1000,state:row.mode==="waiting"?"waiting":"held",structural:row.mode!=="selected",owners:row.mode==="selected"?["only-one"]:[],holder:{pid:row.mode==="other-holder"?pid+1:pid,startedMs:row.mode==="reused-pid"?startedAtMs+10000:startedAtMs,command:"show projects",digest:""}};
   rmSync(file,{force:true});
   const ask=join(directory,token+".ask.json");
   rmSync(ask,{force:true});
   if(row.mode==="deposed")writeFileSync(ask,JSON.stringify({version:1,reason:"stalled",atMs:Date.now()-600000,by:{pid:pid+1,digest:""}}));
   if(row.mode==="substituted-file"){
    if(process.platform==="win32")continue;
    const outside=join(root,"outside");
    writeFileSync(outside,JSON.stringify(claim));symlinkSync(outside,file);
   }else if(row.mode==="substituted-directory"){
    const outside=join(root,"aliased-claims");
    mkdirSync(outside);writeFileSync(join(outside,token+".json"),JSON.stringify(claim));
    rmSync(directory,{recursive:true});symlinkSync(outside,directory,"junction");
   }else if(row.mode!=="released")writeFileSync(file,JSON.stringify(claim));
   const entries=()=>[entry(pid,"show projects",{directory:leaseDirectory,holds:[{resource,mode:"exclusive",sinceMs:NOW-1000,task:1}]})];
   const oracle=db.query("select case when ?1 and ?2 and ?3 and ?4 then 1 else 0 end defer").get(Number(claim.structural&&claim.state==="held"),Number(resource==="graph-custody:"+generationsRoot+":"+claim.token),Number(claim.holder.pid===pid&&claim.holder.startedMs===startedAtMs),Number(row.mode!=="released"&&!row.mode.startsWith("substituted-")&&row.mode!=="deposed")) as {defer:number};
   expect(Boolean(oracle.defer),row.mode).toBe(row.defer);
   expect(graphRefreshCustodyHeldV1(input,entries),row.mode).toBe(row.defer);
   if(row.mode==="substituted-directory"){rmSync(directory);mkdirSync(directory);}
  }
  const cancelled=AbortSignal.abort();
  expect(graphRefreshCustodyHeldV1({...input,signal:cancelled},()=>{throw Error("Unexpected observer");})).toBe(false);
  expect(graphRefreshCustodyHeldV1(input,()=>{throw Error("Unavailable registry");})).toBe(false);
 }finally{db.close();rmSync(root,{recursive:true,force:true});}
});
const setup=entry(30229,"ea3915ce tools setup all",{standing:{version:1,priority:"agent",phase:"javascript-installed-paths",progressAtMs:NOW-2000,parentPid:30192},holds:[{resource:OTHER,mode:"shared",sinceMs:NOW-789_000,task:11},{resource:IMAGE,mode:"exclusive",sinceMs:NOW-736_000,task:14}]});
const read=entry(36610,"b8dbe158 show projects",{standing:{version:1,priority:"agent",phase:"native-fixed-image:/w/.cache/b/d",progressAtMs:NOW-71_000,parentPid:36489},holds:[{resource:OTHER,mode:"shared",sinceMs:NOW-71_000,task:11}],waits:[{resource:IMAGE,mode:"shared",sinceMs:NOW-71_000,task:32,queued:true}]});
const other=entry(31523,"19913804 show projects",{standing:{version:1,priority:"agent",phase:"p",progressAtMs:NOW-701_000,parentPid:31500},waits:[{resource:IMAGE,mode:"shared",sinceMs:NOW-701_000,task:32,queued:true}]});

test("the blockers of a start are the exclusive holds of other live commands on the native image, nothing else",()=>{
 const found=graphReadBlockersV1({entries:[setup,read,other],prefixes:["native-fixed-image:"],ownPid:1,nowMs:NOW});
 expect(found).toEqual([{pid:30229,command:"ea3915ce tools setup all",resource:IMAGE,heldMilliseconds:736_000,phase:"javascript-installed-paths",progressMilliseconds:2000}]);
 expect(graphReadBlockersV1({entries:[setup,read,other],prefixes:["native-fixed-image:"],ownPid:30229,nowMs:NOW})).toEqual([]);
 expect(graphReadBlockersV1({entries:[setup],prefixes:["something-else:"],ownPid:1,nowMs:NOW})).toEqual([]);
});

const host={pressure:"normal",availableMegabytes:17795,totalMegabytes:32768};
test("a silent read is explained: the lease it waits for, the command that holds it with its phase, and the capacity grant it holds",()=>{
 const lines=describeGraphReadStallV1({rootPid:36489,entries:[setup,read,other],view:{host,grants:[{id:"Mt-",pid:36489,reservedMegabytes:3346}],waiters:[]},nowMs:NOW,silentMilliseconds:130_000,lastOutput:"",language:"en"});
 expect(lines[0]).toBe(wantedTextV1("stallHead","en",{pid:36489,seconds:130}));
 expect(lines[1]).toBe(wantedTextV1("stallLease","en",{resource:IMAGE,mode:"shared",seconds:71,holder:30229,command:"ea3915ce tools setup all",phase:"javascript-installed-paths",held:736,progress:2}));
 expect(lines[2]).toBe(wantedTextV1("stallGrant","en",{grant:"Mt-",reserved:3346,grants:1,waiters:0,pressure:"normal",free:17795,total:32768}));
 expect(lines).toHaveLength(3);
 const german=describeGraphReadStallV1({rootPid:36489,entries:[setup,read],view:{host,grants:[{id:"Mt-",pid:36489,reservedMegabytes:3346}],waiters:[]},nowMs:NOW,silentMilliseconds:130_000,lastOutput:"",language:"de"});
 expect(german[1]).toContain("30229");expect(german[1]).toContain("javascript-installed-paths");expect(german[1]).not.toBe(lines[1]);
});

test("a silent read that waits in the capacity queue is told with its place; one without any wait says so and quotes its last output",()=>{
 const waiter=(pid:number,id:string,at:number)=>({pid,arrivedAtMs:at,request:{id,priority:"batch"}});
 const queued=describeGraphReadStallV1({rootPid:36489,entries:[],view:{host,grants:[],waiters:[waiter(1,"a",NOW-9000),waiter(36489,"me",NOW-5000),waiter(2,"b",NOW-1000)]},nowMs:NOW,silentMilliseconds:200_000,lastOutput:"",language:"en"});
 expect(queued[1]).toBe(wantedTextV1("stallQueued","en",{id:"me",class:"batch",ahead:1,pressure:"normal",free:17795,total:32768}));
 const none=describeGraphReadStallV1({rootPid:36489,entries:[read],view:undefined,nowMs:NOW,silentMilliseconds:200_000,lastOutput:"[repo] still working after 500 s; last phase started: source.served",language:"en"});
 expect(none.some(line=>line.includes("source.served"))).toBe(true);
 const quiet=describeGraphReadStallV1({rootPid:99,entries:[setup],view:undefined,nowMs:NOW,silentMilliseconds:200_000,lastOutput:"",language:"en"});
 expect(quiet).toEqual([wantedTextV1("stallHead","en",{pid:99,seconds:200}),wantedTextV1("stallNothing","en")]);
});

test("a start behind an exclusive hold waits with a notice every minute that names the holder, and starts at once when it is gone",async()=>{
 const lines:string[]=[];let time=0,blockers=[{pid:30229,command:"tools setup all",resource:IMAGE,heldMilliseconds:100_000,phase:"javascript-installed-paths",progressMilliseconds:1000}];
 const waited=await awaitGraphReadLeasesV1({blockers:()=>blockers,language:"en",say:line=>lines.push(line),signal:undefined,now:()=>time,sleep:async milliseconds=>{time+=milliseconds;if(time>=150_000)blockers=[];},policy:{...wantedServerPolicyV1,leaseWaitMilliseconds:20*MINUTE}});
 expect(waited).toBeGreaterThanOrEqual(150_000);expect(waited).toBeLessThan(160_000);
 expect(lines.filter(line=>line.includes("30229"))).toHaveLength(3);
 expect(lines.at(-1)).toBe(wantedTextV1("leaseWaitFree","en",{resource:IMAGE,seconds:Math.round(waited/1000)}));
});

test("the wait behind a hold is bounded: after the limit the read starts anyway and says so; a cancelled start returns at once",async()=>{
 const lines:string[]=[];let time=0;
 const blockers=()=>[{pid:30229,command:"tools setup all",resource:IMAGE,heldMilliseconds:1,phase:undefined,progressMilliseconds:1}];
 const waited=await awaitGraphReadLeasesV1({blockers,language:"de",say:line=>lines.push(line),signal:undefined,now:()=>time,sleep:async milliseconds=>{time+=milliseconds;},policy:{...wantedServerPolicyV1,leaseWaitMilliseconds:5*MINUTE}});
 expect(waited).toBeGreaterThanOrEqual(5*MINUTE);expect(waited).toBeLessThan(5*MINUTE+10_000);
 expect(lines.at(-1)).toBe(wantedTextV1("leaseWaitLimit","de",{resource:IMAGE,seconds:Math.round(waited/1000)}));
 const controller=new AbortController();controller.abort();
 expect(await awaitGraphReadLeasesV1({blockers,language:"en",say:()=>{},signal:controller.signal,now:()=>time,sleep:async()=>{throw new Error("must not sleep");}})).toBe(0);
});

test("a start that finds no blocker says nothing, however long looking took",async()=>{
 const lines:string[]=[];let time=0;
 expect(await awaitGraphReadLeasesV1({blockers:()=>{time+=2000;return[];},language:"en",say:line=>lines.push(line),signal:undefined,now:()=>time,sleep:async()=>{throw new Error("must not sleep");}})).toBe(0);
 expect(lines).toEqual([]);
});

const fakeRepo=(script:string)=>{const dir=mkdtempSync(join(tmpdir(),"graph-stall-")),path=join(dir,"repo");writeFileSync(path,"#!/bin/sh\n"+script+"\n");chmodSync(path,0o755);return{dir,path};};
const run=(command:string,over:object={})=>inferGraphReadV1({command,cwd:tmpdir(),environment:{},reason:"request",signal:undefined,language:"en",policy:{...wantedServerPolicyV1,inferStallMilliseconds:900,watchMilliseconds:40},...over});

test("the read of a request runs in the highest class, a background refresh in the lowest",async()=>{
 const{dir,path}=fakeRepo('echo "$SEMIO_LEASE_PRIORITY" > "$OUT"');
 try{
  const out=join(dir,"class");
  await run(path,{environment:{OUT:out}});expect(readFileSync(out,"utf8").trim()).toBe("interactive");
  await run(path,{environment:{OUT:out},reason:"refresh"});expect(readFileSync(out,"utf8").trim()).toBe("background");
 }finally{rmSync(dir,{recursive:true,force:true});}
},30000);

test("a read that prints nothing for the dump limit produces the diagnostic dump once, with the pid and the last output, long before it is stopped",async()=>{
 const{dir,path}=fakeRepo('echo "[repo] still working after 5 s; last phase started: source.served" >&2\nexec sleep 30');
 try{
  const said:Array<{at:number;line:string}>=[],calls:Array<{pid:number;silentMilliseconds:number;lastOutput:string}>=[],started=Date.now();
  const result=await run(path,{say:(line:string)=>said.push({at:Date.now()-started,line}),policy:{...wantedServerPolicyV1,inferStallMilliseconds:1500,watchMilliseconds:40,stallDumpMilliseconds:300,stallDumpRepeatMilliseconds:600_000},diagnose:async(input:{pid:number;silentMilliseconds:number;lastOutput:string})=>{calls.push(input);return["DUMP "+input.pid];}});
  expect(result.ok).toBe(false);
  expect(calls).toHaveLength(1);expect(calls[0]!.pid).toBeGreaterThan(1);expect(calls[0]!.lastOutput).toContain("source.served");expect(calls[0]!.silentMilliseconds).toBeGreaterThanOrEqual(300);
  expect(said.filter(row=>row.line.startsWith("DUMP"))).toHaveLength(1);expect(said.find(row=>row.line.startsWith("DUMP"))!.at).toBeLessThan(1400);
 }finally{rmSync(dir,{recursive:true,force:true});}
},30000);
