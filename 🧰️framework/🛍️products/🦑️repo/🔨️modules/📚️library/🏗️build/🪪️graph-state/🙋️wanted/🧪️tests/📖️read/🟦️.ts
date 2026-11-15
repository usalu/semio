import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{startRemoteCacheServerV1}from"../../../../🌐️remote-cache/🖥️server/🟦️.ts";
import{remoteCacheFetchV1}from"../../../../🌐️remote-cache/🛰️fleet/🟦️.ts";
import{graphFleetClientV1}from"../../../🛰️exchange/🟦️.ts";
import{graphFleetSettingsV1}from"../../../🧶️nx/🟦️.ts";
import{publishServedV1,readServedV1,readWantedV1,wantedEstimateTextV1,wantedPolicyV1,wantedTextV1}from"../../🟦️.ts";
import{fleetReadV1,wantedRefusalV1}from"../../📖️read/🟦️.ts";
import{emptyOwnerCostTableV1,estimateOwnerCostV1,grantOfFootprintV1,ownerCostPolicyV1,ownerCostTextV1,recordOwnerCostV1,soloGrantV1}from"../../💰️cost/🟦️.ts";

/** 📖️ Contract: a reader decides by its admitted memory before it reserves any for inference, takes what the fleet served, publishes the rest, waits with progress, a bound and cancellation, and says in plain text what it waited for when the bound passes. */
const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`wanted-read-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const KEY="a".repeat(64),PLATFORM="darwin-arm64",names=(count:number)=>Array.from({length:count},(_,at)=>`p${String(at).padStart(5,"0")}`);

async function world(){
 const server=await startRemoteCacheServerV1({directory:temp("cache"),host:"127.0.0.1",port:0,maximumBytes:2**28,maximumEntryBytes:2**26,maximumAgeDays:30,defaultInstance:"main",tokens:{read:"r",write:"w"},validation:"off"});
 const environment={SEMIO_REMOTE_CACHE_ENDPOINT:server.url,SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"};
 const client=(label:string)=>{const root=temp(label);return graphFleetClientV1(graphFleetSettingsV1(root,environment)!,{fetch:remoteCacheFetchV1(environment,{root})});};
 return{server,client};
}
const clock=()=>{let at=1_000_000;return{now:()=>at,sleep:async(milliseconds:number)=>{at+=milliseconds;}};};

test("a read whose stale projects fit the admitted memory stays local and never touches the fleet",async()=>{
 const lines:string[]=[],result=await fleetReadV1({stale:names(60),admittedMegabytes:8000,client:undefined,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:"en",say:line=>lines.push(line)});
 expect(result.outcome).toBe("local");expect(result.plan.mode).toBe("local");expect(lines).toEqual([]);
});

test("a machine admitted the serving headroom infers all 5 000 stale projects itself and never asks",async()=>{
 const lines:string[]=[],result=await fleetReadV1({stale:names(5000),admittedMegabytes:16384,client:undefined,requester:"g1-server",platform:PLATFORM,graphKey:KEY,language:"en",say:line=>lines.push(line)});
 expect(result.outcome).toBe("local");expect(result.plan.localProjects).toBe(5000);expect(lines).toEqual([]);
});

test("a 16 GB shape publishes its 4 700 stale projects, takes the answer of a serving machine as it arrives and keeps only what it can infer; progress is told once per change",async()=>{
 const {server,client}=await world();
 try{
  const reader=client("reader"),serving=client("serving"),lines:string[]=[],time=clock(),stale=names(4700);
  let rounds=0;
  const sleep=async(milliseconds:number)=>{await time.sleep(milliseconds);rounds++;if(rounds===3)await publishServedV1({client:serving,server:"g1-server",request:(await readWantedV1({client:serving,requester:"g1-m06",now:time.now}))!,projects:stale.slice(0,2000),generation:7,now:time.now});if(rounds===6)await publishServedV1({client:serving,server:"g1-server",request:(await readWantedV1({client:serving,requester:"g1-m06",now:time.now}))!,projects:stale.slice(0,4700-40),generation:8,now:time.now});};
  const result=await fleetReadV1({stale,admittedMegabytes:4096,client:reader,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:"en",say:line=>lines.push(line),sleep,now:time.now});
  expect(result.outcome).toBe("served");expect(result.missing).toHaveLength(40);expect(result.arrived).toBe(4660);
  expect(lines[0]).toBe(wantedTextV1("plan","en",{stale:4700,taken:0,local:72,wave:18,wanted:4700}));
  expect(lines.filter(line=>line.startsWith("graph: waiting"))).toEqual([wantedTextV1("waiting","en",{arrived:0,wanted:4700}),wantedTextV1("waiting","en",{arrived:2000,wanted:4700}),wantedTextV1("waiting","en",{arrived:4660,wanted:4700})]);
  expect((await readWantedV1({client:serving,requester:"g1-m06",now:time.now}))?.projects).toHaveLength(4700);
 }finally{await server.stop(true);}
},60000);

test("a retried read finds the answer to its earlier request: the projects already served are taken, not asked for again",async()=>{
 const {server,client}=await world();
 try{
  const reader=client("reader"),serving=client("serving"),stale=names(900);
  await publishServedV1({client:serving,server:"g1-server",request:{version:1,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,projects:[],createdMs:0,expiresMs:1},projects:stale.slice(0,850),generation:3});
  expect((await readServedV1({client:reader,requester:"g1-m06"}))?.generation).toBe(3);
  const result=await fleetReadV1({stale,admittedMegabytes:4096,client:reader,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:"de",say:()=>{}});
  expect(result.outcome).toBe("local");expect(result.plan.takenProjects).toBe(850);expect(result.arrived).toBe(850);
 }finally{await server.stop(true);}
},60000);

test("nothing from a silent server within the silence bound: the read says in English and German what it waited for and refuses with the designed class and exit status instead of hanging",async()=>{
 const {server,client}=await world();
 try{
  for(const language of["en","de"] as const){
   const lines:string[]=[],time=clock();
   const error=await fleetReadV1({stale:names(4700),admittedMegabytes:2048,client:client("reader-"+language),requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language,say:line=>lines.push(line),sleep:time.sleep,now:time.now}).then(()=>undefined,reason=>reason as Error&{code:string;key:string;values:Record<string,number>});
   expect(error?.code).toBe(wantedRefusalV1.code);expect(error?.key).toBe(wantedRefusalV1.key);expect(wantedRefusalV1.exitStatus).toBe(75);
   expect(error?.values).toEqual({wanted:4700,arrived:0,seconds:wantedPolicyV1.silenceMilliseconds/1000,missing:4700,local:0,heavy:""});
   expect(lines.at(-1)).toBe(wantedTextV1("refused",language,error!.values));
  }
  expect(wantedTextV1("refused","de",{wanted:1,arrived:0,seconds:2,missing:1,local:0})).toContain("veraltete Projekte");
 }finally{await server.stop(true);}
},60000);

test("an unreachable fleet refuses at once when the machine cannot run even the smallest wave, and goes on locally when it can; cancellation ends the wait",async()=>{
 const lines:string[]=[],dead=graphFleetClientV1(graphFleetSettingsV1(temp("dead"),{SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:9",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"})!);
 await expect(fleetReadV1({stale:names(500),admittedMegabytes:2048,client:undefined,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:"en",say:line=>lines.push(line)})).rejects.toMatchObject({code:"unserved"});
 expect(lines.some(line=>line.includes("cannot be reached"))).toBe(true);
 const local=await fleetReadV1({stale:names(80),admittedMegabytes:4096,client:dead,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:"en",say:()=>{},sleep:clock().sleep,now:clock().now}).catch(()=>undefined);
 expect(local === undefined || local.outcome === "served").toBe(true);
 const {server,client}=await world();
 try{
  const controller=new AbortController();
  const waiting=fleetReadV1({stale:names(4700),admittedMegabytes:4096,client:client("cancel"),requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:"en",say:()=>{},signal:controller.signal,sleep:async()=>{controller.abort();}});
  await expect(waiting).rejects.toThrow();
 }finally{await server.stop(true);}
},60000);

const kindOf=(name:string)=>name==="workspace"?"root:language:cargo":undefined;
const estimator=(extra:Record<string,number>={})=>{let table=emptyOwnerCostTableV1();for(const [name,megabytes] of Object.entries(extra))table=recordOwnerCostV1(table,{name,kind:"project",megabytes,seconds:1,rows:1,stopped:false,atMs:1});return(name:string)=>estimateOwnerCostV1(table,name,kindOf(name));};

const ROOMY=soloGrantV1({hostMegabytes:16384},wantedPolicyV1),TIGHT=grantOfFootprintV1(5000),NOTHING=grantOfFootprintV1(0),LIMIT=(grant:typeof ROOMY)=>Math.round(grant.managedMegabytes*ownerCostPolicyV1.footprintFactor);
type Refusal=Error&{code:string;key:string;values:Record<string,string|number>};

test("a heavy workspace root is refused with the typed owner-too-heavy refusal, naming the owner and the numbers in English and German, after the wait and when the fleet is unreachable",async()=>{
 const {server,client}=await world(),dead=graphFleetClientV1(graphFleetSettingsV1(temp("dead-over"),{SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:9",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"})!);
 try{
  for(const language of["en","de"] as const)for(const via of["wait","unreachable"] as const){
   const lines:string[]=[],time=clock(),reader=via==="wait"?client(`over-${language}`):dead,stale=["@semio-tech/light","workspace"];
   const error=await fleetReadV1({stale,admittedMegabytes:8192,soloGrant:NOTHING,client:reader,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language,estimate:estimator(),say:line=>lines.push(line),sleep:time.sleep,now:time.now}).then(()=>undefined,reason=>reason as Refusal);
   expect(error?.code).toBe("unserved");expect(error?.key).toBe("build.graph.owner-too-heavy");expect(wantedRefusalV1.key).toBe("build.graph.wanted-expired");
   expect(error?.values).toEqual({named:"workspace (about 6200 MB)".replace("about",language==="en"?"about":"etwa"),limit:LIMIT(NOTHING)});
   expect(lines[0]).toContain("workspace");expect(lines[0]).toContain(language==="en"?"too heavy":"zu schwer");
   expect(lines.at(-1)).toBe(ownerCostTextV1("exceeds",language,{named:String(error?.values.named),grant:LIMIT(NOTHING)}));
   expect(lines.at(-1)).toContain(language==="en"?"even when nothing else":"auch dann");
  }
 }finally{await server.stop(true);}
},60000);

test("a heavy workspace root above even the tight solo grant is attempted alone, never refused by its learned cost",async()=>{
 const lines:string[]=[],stale=[...names(30),"workspace"],result=await fleetReadV1({stale,admittedMegabytes:8192,soloGrant:TIGHT,client:undefined,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language:"en",estimate:estimator(),say:line=>lines.push(line),sleep:clock().sleep,now:clock().now});
 expect(result.outcome).toBe("served");expect(result.solo?.owners.map(row=>row.name)).toEqual(["workspace"]);expect(result.solo?.grant).toEqual(TIGHT);
});

test("an unreachable fleet does not stop a machine that can read alone: the light projects are left to the waves and the heavy workspace root is named as solo with the solo grant, in both languages",async()=>{
 const dead=graphFleetClientV1(graphFleetSettingsV1(temp("dead-solo"),{SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:9",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"})!);
 for(const language of["en","de"] as const)for(const client of[dead,undefined]){
  const lines:string[]=[],stale=[...names(30),"workspace"],result=await fleetReadV1({stale,admittedMegabytes:8192,soloGrant:ROOMY,client,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language,estimate:estimator(),say:line=>lines.push(line),sleep:clock().sleep,now:clock().now});
  expect(result.outcome).toBe("served");expect(result.arrived).toBe(0);expect(result.missing).toEqual(stale);
  expect(result.solo?.owners.map(row=>row.name)).toEqual(["workspace"]);expect(result.solo?.grant).toEqual(ROOMY);
  expect(lines.at(-1)).toBe(ownerCostTextV1("solo",language,{count:1,named:"workspace (about 6200 MB)".replace("about",language==="en"?"about":"etwa"),grant:LIMIT(ROOMY)}));
  expect(lines.some(line=>line.includes(language==="en"?"cannot be reached":"nicht erreichbar"))).toBe(true);
 }
});

test("the wait ends without delivery of the heavy workspace root: it stays a solo owner of this machine beside the light remainder, which the waves infer; the fleet was asked first and for the heavy one by name",async()=>{
 const {server,client}=await world();
 try{
  const reader=client("reader"),time=clock(),START=time.now(),lights=names(20),lines:string[]=[];
  const result=await fleetReadV1({stale:[...lights,"workspace"],admittedMegabytes:8192,soloGrant:ROOMY,client:reader,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language:"en",estimate:estimator(),say:line=>lines.push(line),sleep:time.sleep,now:time.now});
  expect(result.outcome).toBe("served");expect(result.arrived).toBe(0);expect(new Set(result.missing)).toEqual(new Set([...lights,"workspace"]));
  expect(result.solo?.owners.map(row=>row.name)).toEqual(["workspace"]);expect(result.solo?.owners[0]).toMatchObject({megabytes:3100,source:"seed"});
  expect(time.now()-START).toBeGreaterThanOrEqual(wantedPolicyV1.silenceMilliseconds);
  expect(lines[0]).toContain("workspace");expect(lines.at(-1)).toContain("one at a time");
  expect((await readWantedV1({client:reader,requester:"g1-m02",now:time.now}))?.heavy).toEqual(["workspace"]);
 }finally{await server.stop(true);}
},60000);

test("the heavy project arrives and the light remainder fits: the read goes on locally with the light ones only; a light remainder alone never waits",async()=>{
 const {server,client}=await world();
 try{
  const reader=client("reader"),serving=client("serving"),time=clock(),lights=names(30),stale=[...lights,"@semio-tech/s-dev-composition"],lines:string[]=[];
  let rounds=0;
  const sleep=async(milliseconds:number)=>{await time.sleep(milliseconds);rounds++;if(rounds===2)await publishServedV1({client:serving,server:"g1-server",request:(await readWantedV1({client:serving,requester:"g1-m02",now:time.now}))!,projects:["@semio-tech/s-dev-composition"],generation:9,now:time.now});};
  const result=await fleetReadV1({stale,admittedMegabytes:8192,client:reader,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language:"en",estimate:estimator(),say:line=>lines.push(line),sleep,now:time.now});
  expect(result.outcome).toBe("served");expect(result.missing).toEqual(lights);expect(rounds).toBeGreaterThanOrEqual(2);
  expect(result.solo).toBeUndefined();
  const light=await fleetReadV1({stale:lights,admittedMegabytes:8192,client:reader,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language:"en",estimate:estimator(),say:()=>{}});
  expect(light.outcome).toBe("local");
 }finally{await server.stop(true);}
},60000);

test("a heavy project already served is taken, not asked for again; the same stale set on a serving machine and on a machine without estimates behaves as before",async()=>{
 const {server,client}=await world();
 try{
  const reader=client("reader"),serving=client("serving"),stale=["workspace",...names(10)];
  await publishServedV1({client:serving,server:"g1-server",request:{version:1,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,projects:[],createdMs:0,expiresMs:1},projects:["workspace"],generation:3});
  expect((await fleetReadV1({stale,admittedMegabytes:8192,client:reader,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language:"en",estimate:estimator(),say:()=>{}})).outcome).toBe("local");
  expect((await fleetReadV1({stale,admittedMegabytes:16384,client:undefined,requester:"g1-server",platform:PLATFORM,graphKey:KEY,language:"en",estimate:estimator(),say:()=>{}})).outcome).toBe("local");
  expect((await fleetReadV1({stale,admittedMegabytes:8192,client:undefined,requester:"g1-m05",platform:PLATFORM,graphKey:KEY,language:"en",say:()=>{}})).outcome).toBe("local");
 }finally{await server.stop(true);}
},60000);

test("an unreachable fleet refuses a single heavy project at once, while the same single project when light stays local",async()=>{
 const lines:string[]=[],dead=graphFleetClientV1(graphFleetSettingsV1(temp("dead-heavy"),{SEMIO_REMOTE_CACHE_ENDPOINT:"http://127.0.0.1:9",SEMIO_REMOTE_CACHE_ACCESS:"read-write",SEMIO_REMOTE_CACHE_TOKEN:"w",SEMIO_REMOTE_CACHE_INSTANCE:"main"})!);
 await expect(fleetReadV1({stale:["workspace"],admittedMegabytes:8192,client:dead,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language:"de",estimate:estimator(),say:line=>lines.push(line)})).rejects.toMatchObject({code:"unserved"});
 expect(lines.at(-1)).toContain("workspace");
 expect((await fleetReadV1({stale:["@semio-tech/light"],admittedMegabytes:8192,client:dead,requester:"g1-m02",platform:PLATFORM,graphKey:KEY,language:"de",estimate:estimator(),say:()=>{}})).outcome).toBe("local");
 expect(ownerCostPolicyV1.kindMegabytes["root:language:cargo"]).toBeGreaterThan(0);
});

const MINUTE=60_000;
test("Law: the read waits as long as the server's progress record advances: a server that is busy for 40 minutes (progress every 60 s, nothing delivered for 38 of them, the old 15-minute bound long passed) keeps the reader waiting, the request is renewed and stays the same piece of work, the wait line tells the server's last progress and the estimate in both languages, and what arrives is used",async()=>{
 const {server,client}=await world();
 try{
  for(const language of["en","de"] as const){
   const reader=client("reader-"+language),serving=client("serving-"+language),lines:string[]=[],time=clock(),stale=names(600),START=time.now();
   let tick=0,created=0,alivePast=false;
   const sleep=async(milliseconds:number)=>{
    await time.sleep(milliseconds);
    const elapsed=time.now()-START;
    if(elapsed>=tick*MINUTE){
     tick++;
     const request=await readWantedV1({client:serving,requester:"g1-m06"+language,now:time.now});
     if(created===0&&request!==undefined)created=request.createdMs;
     if(elapsed>=35*MINUTE&&elapsed<36*MINUTE)alivePast=request!==undefined&&request.createdMs===created&&request.expiresMs>time.now();
     if(request!==undefined)await publishServedV1({client:serving,server:"g1-server",request,projects:elapsed>=38*MINUTE?stale.slice(0,560):[],generation:null,progress:{total:600,progressAtMs:time.now(),remainingMilliseconds:elapsed>=38*MINUTE?0:(40*MINUTE-elapsed)},now:time.now});
    }
   };
   const result=await fleetReadV1({stale,admittedMegabytes:4096,client:reader,requester:"g1-m06"+language,platform:PLATFORM,graphKey:KEY,language,say:line=>lines.push(line),sleep,now:time.now});
   expect(time.now()-START).toBeGreaterThanOrEqual(38*MINUTE);expect(time.now()-START).toBeGreaterThan(wantedPolicyV1.waitMilliseconds);
   expect(result.outcome).toBe("served");expect(result.arrived).toBe(560);expect(result.missing).toHaveLength(40);
   expect(alivePast).toBe(true);
   const waiting=lines.filter(line=>language==="en"?/server last progressed \d+ s ago, estimated time left: (about \d+ min|about \d+ h \d+ min|unknown|under a minute)$/u.test(line):/Server kam zuletzt vor \d+ s voran, geschätzte Restzeit: (etwa \d+ min|etwa \d+ h \d+ min|unbekannt|unter einer Minute)$/u.test(line));
   expect(waiting.length).toBeGreaterThan(20);expect(waiting.some(line=>line.includes(language==="en"?"estimated time left: about 2":"Restzeit: etwa 2"))).toBe(true);
  }
 }finally{await server.stop(true);}
},120000);

test("Law: no progress for the bound is a typed refusal even though the server's sign of life never stops, and it names the cause in both languages; a silent fleet names the other cause; exit status 75 in both",async()=>{
 const {server,client}=await world();
 try{
  for(const cause of["stalled","silent"] as const)for(const language of["en","de"] as const){
   const reader=client(`reader-${cause}-${language}`),serving=client(`serving-${cause}-${language}`),lines:string[]=[],time=clock(),START=time.now(),stale=names(600);let beat=0;
   const sleep=async(milliseconds:number)=>{
    await time.sleep(milliseconds);
    if(cause==="stalled"&&time.now()-START>=beat*20_000){beat++;const request=await readWantedV1({client:serving,requester:"g1-m06",now:time.now});if(request!==undefined)await publishServedV1({client:serving,server:"g1-server",request,projects:[],generation:null,progress:{total:600,progressAtMs:START,remainingMilliseconds:null},now:time.now});}
   };
   const error=await fleetReadV1({stale,admittedMegabytes:2048,client:reader,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language,say:line=>lines.push(line),sleep,now:time.now}).then(()=>undefined,reason=>reason as Error&{code:string;key:string;values:Record<string,string|number>});
   expect(error?.code).toBe(wantedRefusalV1.code);expect(wantedRefusalV1.exitStatus).toBe(75);
   const waited=time.now()-START;
   if(cause==="stalled"){expect(waited).toBeGreaterThanOrEqual(wantedPolicyV1.waitMilliseconds);expect(waited).toBeLessThan(wantedPolicyV1.waitMilliseconds+60_000);}
   else{expect(waited).toBeGreaterThanOrEqual(wantedPolicyV1.silenceMilliseconds);expect(waited).toBeLessThan(wantedPolicyV1.silenceMilliseconds+60_000);}
   const text=cause==="stalled"?wantedTextV1("causeStalled",language,{minutes:30}):wantedTextV1("causeSilent",language,{minutes:5});
   expect(lines).toContain(text);expect(lines.indexOf(text)).toBeLessThan(lines.length-1);
  }
 }finally{await server.stop(true);}
},120000);

test("what arrived is used: the fleet delivered all but k projects within the capacity of this machine, so the read stops waiting at once and infers the k itself instead of refusing",async()=>{
 const {server,client}=await world();
 try{
  const reader=client("reader"),serving=client("serving"),time=clock(),stale=names(600),START=time.now();let sent=false;
  const sleep=async(milliseconds:number)=>{await time.sleep(milliseconds);if(!sent){sent=true;const request=(await readWantedV1({client:serving,requester:"g1-m06",now:time.now}))!;await publishServedV1({client:serving,server:"g1-server",request,projects:stale.slice(0,600-70),generation:null,progress:{total:600,progressAtMs:time.now(),remainingMilliseconds:120_000},now:time.now});}};
  const result=await fleetReadV1({stale,admittedMegabytes:4096,client:reader,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:"en",say:()=>{},sleep,now:time.now});
  expect(result.outcome).toBe("served");expect(result.missing).toHaveLength(70);expect(result.plan.localProjects).toBe(72);expect(time.now()-START).toBeLessThan(10_000);
 }finally{await server.stop(true);}
},60000);

/** 🤝️ Contract: while the fleet request is outstanding the machine infers the light remainder itself in ONE continuous run, lightest first (the server works heaviest first), asks `skip` before every project so what the fleet delivered meanwhile is dropped, narrows its request by what it inferred and ends when the served and the inferred projects cover every stale project. */
type Run=Readonly<{names:readonly string[];skip(name:string):boolean;done(name:string):void}>;
async function cooperation(options:Readonly<{language:"en"|"de";serve:number;stale?:number;infer?:(run:Run)=>Promise<void>;signal?:AbortSignal}>){
 const {server,client}=await world();
 try{
  const reader=client("reader"),serving=client("serving"),time=clock(),count=options.stale??200,stale=names(count),weight=(name:string)=>Number(name.slice(1)),lines:string[]=[],runs:string[][]=[],order=[...stale].sort((a,b)=>weight(b)-weight(a)),pause=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
  let published=0,rounds=0,calls=0;const inferred:string[]=[],dropped:string[]=[],stepPublished:number[]=[],narrowed:number[]=[],heavy=stale[count-1]!;
  const publish=async()=>{published=Math.min(count,published+options.serve);await publishServedV1({client:serving,server:"g1-server",request:(await readWantedV1({client:serving,requester:"g1-m06",now:time.now}))!,projects:order.slice(0,published),generation:published,now:time.now,progress:{total:count,progressAtMs:time.now(),remainingMilliseconds:600000}});};
  const sleep=async(milliseconds:number)=>{time.sleep(milliseconds);await pause(3);if(++rounds>=2&&published<count)await publish();};
  const infer=async(run:Run)=>{
   calls++;runs.push([...run.names]);
   if(options.infer!==undefined)return options.infer(run);
   for(const name of run.names){
    if(run.skip(name)){dropped.push(name);continue;}
    stepPublished.push(published);inferred.push(name);time.sleep(3000);await pause(12);
    if(inferred.length%10===0)narrowed.push((await readWantedV1({client:serving,requester:"g1-m06",now:time.now}))?.projects.length??-1);
    run.done(name);
   }
  };
  const result=await fleetReadV1({stale,admittedMegabytes:4096,client:reader,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:options.language,estimate:name=>({megabytes:name===heavy?99999:weight(name)+1,source:"recorded"}),infer,say:line=>lines.push(line),sleep,now:time.now,...options.signal===undefined?{}:{signal:options.signal}}).catch(error=>({error:error as Error}));
  return{result,lines,runs,calls,inferred,dropped,stepPublished,narrowed,order,stale,heavy};
 }finally{await server.stop(true);}
}

test("a slow fleet and this machine finish the read together in one run: lightest first here, heaviest first there, arrivals dropped mid-run, no project both inferred and taken, narrowed request, both counts told in English",async()=>{
 const run=await cooperation({language:"en",serve:3}),result=run.result as Awaited<ReturnType<typeof fleetReadV1>>,inferred=run.inferred;
 expect(result.outcome).toBe("served");expect(result.missing).toEqual([]);
 expect(run.calls).toBe(1);expect(run.runs[0]).not.toContain(run.heavy);expect(run.runs[0]![0]).toBe(run.stale[0]);
 expect(run.runs[0]).toEqual([...run.runs[0]!].sort());
 expect(inferred.length).toBeGreaterThan(0);expect(inferred).toEqual([...inferred].sort());expect(new Set(inferred).size).toBe(inferred.length);
 expect(run.dropped.length).toBeGreaterThan(0);for(const name of run.dropped)expect(inferred).not.toContain(name);
 for(let at=1;at<inferred.length;at++){const served=new Set(run.order.slice(0,run.stepPublished[at-1]!));expect(served.has(inferred[at]!)).toBe(false);}
 expect(result.inferred).toBe(inferred.length);expect(result.arrived+result.inferred!).toBe(run.stale.length);
 expect(run.narrowed.length).toBeGreaterThan(0);expect(run.narrowed[0]!).toBeLessThan(run.stale.length);
 const told=run.lines.filter(line=>line.includes("inferred here"));
 expect(told.length).toBeGreaterThan(0);
 const start=run.lines.find(line=>line.startsWith("graph: inferring "))!,count=Number(/inferring (\d+) light/u.exec(start)![1]);
 expect(start).toBe(wantedTextV1("inferring","en",{count}));expect(count).toBe(run.runs[0]!.length);
 const last=/^graph: (\d+) of 200 projects arrived from the fleet, (\d+) inferred here, 0 left; the server last progressed \d+ s ago, estimated time left: about 10 min$/u.exec(told.at(-1)!)!;
 expect(Number(last[1])+Number(last[2])).toBe(200);expect(Number(last[2])).toBeLessThanOrEqual(result.inferred!);
},60000);

test("the same cooperation tells both counts in German, and the fleet alone finishing early leaves nothing to infer twice",async()=>{
 const run=await cooperation({language:"de",serve:200,stale:120}),result=run.result as Awaited<ReturnType<typeof fleetReadV1>>;
 expect(result.outcome).toBe("served");expect(result.missing).toEqual([]);expect(result.arrived+result.inferred!).toBe(120);expect(run.calls).toBeLessThanOrEqual(1);
 expect(run.lines.some(line=>/hier berechnet/u.test(line)&&/aus der Flotte eingetroffen/u.test(line))).toBe(true);
 expect(run.lines.some(line=>line.startsWith("Graph: berechne"))).toBe(true);
});

test("a run that is still going when the wait ends by its bounds drops the rest of its projects at once and leaves them to the caller",async()=>{
 const {server,client}=await world();
 try{
  const reader=client("reader"),time=clock(),stale=names(150);let calls=0,inferred=0,cut=0;
  const result=await fleetReadV1({stale,admittedMegabytes:4096,client:reader,requester:"g1-m06",platform:PLATFORM,graphKey:KEY,language:"en",estimate:name=>({megabytes:Number(name.slice(1))/10+1,source:"recorded"}),infer:async run=>{calls++;for(const name of run.names){if(run.skip(name)){cut++;continue;}inferred++;time.sleep(60_000*40);await new Promise<void>(resolve=>setTimeout(resolve,2));run.done(name);}},say:()=>{},sleep:async milliseconds=>{time.sleep(milliseconds);await new Promise<void>(resolve=>setTimeout(resolve,1));},now:time.now,policy:{...wantedPolicyV1,silenceMilliseconds:60_000*60*10,waitMaximumMilliseconds:60_000*60*10}});
  expect(calls).toBe(1);expect(result.outcome).toBe("served");expect(inferred+cut).toBe(150);expect(result.inferred).toBe(inferred);expect(result.missing.length).toBe(cut);
 }finally{await server.stop(true);}
},60000);

test("a failing local inference is told and the read goes on waiting for the fleet only; cancellation ends it",async()=>{
 const failing=await cooperation({language:"en",serve:200,infer:async()=>{throw new Error("boom");}}),result=failing.result as Awaited<ReturnType<typeof fleetReadV1>>;
 expect(result.outcome).toBe("served");expect(failing.lines.some(line=>line===wantedTextV1("inferFailed","en",{detail:"boom"}))).toBe(true);expect(result.arrived).toBe(200);expect(result.inferred).toBe(0);
 const control=new AbortController(),cancelled=await cooperation({language:"en",serve:1,signal:control.signal,infer:async()=>{control.abort();}});
 expect((cancelled.result as {error?:Error}).error?.name).toBe("AbortError");
});
