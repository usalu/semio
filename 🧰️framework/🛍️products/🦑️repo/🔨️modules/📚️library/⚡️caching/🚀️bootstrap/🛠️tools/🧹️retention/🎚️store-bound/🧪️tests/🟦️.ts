import {expect,test} from "bun:test";
import {Database} from "bun:sqlite";
import Ajv from "ajv";
import {runToolSyncV1,startToolV1} from "../../../../../../../../../../🔨️modules/🏃️process/🧰️tools/🟦️.ts";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,statSync,symlinkSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {cacheTextV1,type CacheLanguageV1} from "../../../../../💾️storage/🧾️ledger/🟦️.ts";
import texts from "../../../../../💾️storage/🧾️ledger/📖️texts/🔣️.json" with {type:"json"};
import {planNxToolingRetentionV1} from "../../🟦️.ts";
import {NX_TOOLING_IDLE_WAIT_ENVIRONMENT_V1,nxToolingStoreBoundPolicyV1,planNxToolingStoreBoundV1,readNxToolingStoreBoundPolicyV1,resolveNxToolingStoreBoundPolicyV1,type NxToolingStoreBoundInputV1,type NxToolingStoreBoundPolicyV1} from "../🟦️.ts";
import {censusNxToolingStoreV1,formatNxToolingStoreBoundV1,runNxToolingStoreBoundV1,type NxToolingStoreBoundPortsV1} from "../🏃️run/🟦️.ts";
import {probeNxToolingHeldV1,probeNxToolingImageIdleV1} from "../🏃️run/🔌️host/🟦️.ts";
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import policy from "../🔣️policy.json" with {type:"json"};

const clone=<T>(value:T):T=>JSON.parse(JSON.stringify(value)),declared=nxToolingStoreBoundPolicyV1(),oracle=join(import.meta.dir,"🐍️.py"),holder=join(import.meta.dir,"🔐️holder","🟦️.ts");
const identityOf=(seed:number):string=>createHash("sha256").update("identity:"+seed).digest("hex");
function generator(seed:number):()=>number{let state=seed>>>0;return()=>{state=(state+0x6d2b79f5)>>>0;let value=state;value=Math.imul(value^value>>>15,value|1);value^=value+Math.imul(value^value>>>7,value|61);return((value^value>>>14)>>>0)/4294967296;};}

type Case=NxToolingStoreBoundInputV1&{policy:NxToolingStoreBoundPolicyV1};
function corpus(count:number):Case[]{
 const random=generator(20261026),cases:Case[]=[];
 for(let index=0;index<count;index++){
  const size=1+Math.floor(random()*(index%10===0?160:30)),keepNewest=1+Math.floor(random()*12),stamps=2+Math.floor(random()*40),nowMs=1_000_000+Math.floor(random()*100),images=Array.from({length:size},(_,row)=>({identity:identityOf(index*1000+row),createdAt:Math.floor(random()*stamps)*60_000,held:random()<0.12}));
  cases.push({nowMs:nowMs+stamps*60_000,current:images[Math.floor(random()*size)]!.identity,pressure:random()<0.4,images,policy:{...declared,keepNewest,keepNewestUnderPressure:1+Math.floor(random()*keepNewest),minimumAgeMs:Math.floor(random()*8)*60_000,maximumPerRun:1+Math.floor(random()*(size+3))}});
 }
 return cases;
}
const cases=corpus(400);
function pythonPlans(rows:Case[]):any[]{
 const directory=mkdtempSync(join(tmpdir(),"nxret-")),file=join(directory,"cases.json");
 try{
  writeFileSync(file,JSON.stringify({cases:rows}));
  const run=runToolSyncV1("python",["-I",oracle,file],{encoding:"utf8",maxBuffer:1<<28});if(run.status!==0)throw Error(run.stderr);
  return JSON.parse(run.stdout);
 }finally{rmSync(directory,{recursive:true,force:true});}
}

test("the authored policy is closed, Ajv agrees with the validator and every forged policy is refused in both languages",()=>{
 expect(readNxToolingStoreBoundPolicyV1(policy)).toEqual(declared);
 const validate=new Ajv({strict:true}).compile(schema);
 expect(validate(policy)).toBe(true);
 const probes:Array<(document:any)=>void>=[document=>{delete document.keepNewest;},document=>{document.version=2;},document=>{document.extra=1;},document=>{document.keepNewest=0;},document=>{document.keepNewest=33;},document=>{document.maximumPerRun=0;},document=>{document.minimumAgeMs=-1;},document=>{document.intervalMs=1;},document=>{document.maximumCensus=0;},document=>{document.maximumCensus="1";}];
 for(const probe of probes){const document=clone(policy);probe(document);expect(validate(document)).toBe(false);expect(()=>readNxToolingStoreBoundPolicyV1(document)).toThrow("Closed Nx toolkit store bound policy required");expect(()=>readNxToolingStoreBoundPolicyV1(document)).toThrow("Geschlossene Richtlinie");}
 const wider=clone(policy);wider.keepNewestUnderPressure=wider.keepNewest+1;expect(()=>readNxToolingStoreBoundPolicyV1(wider)).toThrow("Closed Nx toolkit store bound policy required");
 expect(declared.keepNewestUnderPressure).toBeLessThanOrEqual(declared.keepNewest);
});

test("the plan equals the independent Python oracle on 400 seeded stores",()=>{
 const expected=pythonPlans(cases);
 cases.forEach((row,index)=>{const {policy:declaredRow,...input}=row,plan=planNxToolingStoreBoundV1(input,declaredRow);expect(JSON.parse(JSON.stringify(plan)),"case "+index).toEqual(expected[index]);});
});

test("laws of the plan: the current, held, newest and young generations stay, the rest goes oldest first, a second run finds nothing",()=>{
 for(const row of cases){
  const {policy:rowPolicy,...input}=row,plan=planNxToolingStoreBoundV1(input,rowPolicy),byIdentity=new Map(input.images.map(image=>[image.identity,image])),reasons=new Map(plan.retained.map(entry=>[entry.identity,entry.reason]));
  expect(plan.retained.length+plan.retire.length).toBe(input.images.length);expect(plan.limit).toBe(rowPolicy.maximumPerRun);
  expect(reasons.get(input.current)).toBe("current");
  for(const identity of plan.retire){const image=byIdentity.get(identity)!;expect(image.held).toBe(false);expect(identity).not.toBe(input.current);expect(input.nowMs-image.createdAt).toBeGreaterThanOrEqual(rowPolicy.minimumAgeMs);}
  for(const image of input.images)if(image.held)expect(plan.retire).not.toContain(image.identity);
  const times=plan.retire.map(identity=>byIdentity.get(identity)!.createdAt);expect(times).toEqual([...times].sort((left,right)=>left-right));
  expect(plan.keepNewest).toBe(input.pressure?rowPolicy.keepNewestUnderPressure:rowPolicy.keepNewest);
  const newest=[...input.images].sort((left,right)=>right.createdAt-left.createdAt||(left.identity<right.identity?-1:1)).slice(0,plan.keepNewest).map(image=>image.identity);
  for(const identity of newest)expect(plan.retire).not.toContain(identity);
  const left=input.images.filter(image=>!plan.retire.includes(image.identity)),again=planNxToolingStoreBoundV1({...input,images:left},rowPolicy);expect(again.retire).toEqual([]);
  const calmer=planNxToolingStoreBoundV1({...input,pressure:false},rowPolicy),tighter=planNxToolingStoreBoundV1({...input,pressure:true},rowPolicy);
  expect(tighter.retire).toEqual(expect.arrayContaining(calmer.retire));
 }
});

test("without readers, young generations or a limit the plan is exactly the dead count rule, ordered by an independent SQLite",()=>{
 const database=new Database(":memory:");
 try{
  database.exec("CREATE TABLE images(identity TEXT PRIMARY KEY,createdAt REAL NOT NULL)");
  for(const row of cases.slice(0,120)){
   database.exec("DELETE FROM images");for(const image of row.images)database.query("INSERT INTO images VALUES (?1,?2)").run(image.identity,image.createdAt);
   const keep=row.policy.keepNewest,open={...row.policy,minimumAgeMs:0,maximumPerRun:1024},input={nowMs:row.nowMs,current:row.current,pressure:false,images:row.images.map(image=>({...image,held:false}))};
   const independent=database.query("SELECT identity FROM images WHERE identity<>?1 AND identity NOT IN (SELECT identity FROM images ORDER BY createdAt DESC,identity ASC LIMIT ?2) ORDER BY createdAt ASC,identity ASC").all(row.current,keep).map(entry=>(entry as {identity:string}).identity);
   expect(planNxToolingStoreBoundV1(input,open).retire).toEqual(independent);
   expect([...planNxToolingRetentionV1(row.images.map(({identity,createdAt})=>({identity,createdAt})),row.current,keep)].sort()).toEqual([...independent].sort());
  }
 }finally{database.close();}
 expect(()=>planNxToolingStoreBoundV1({nowMs:0,current:identityOf(1),pressure:false,images:[{identity:identityOf(2),createdAt:0,held:false}]})).toThrow();
 expect(()=>planNxToolingStoreBoundV1({nowMs:Number.NaN,current:identityOf(1),pressure:false,images:[{identity:identityOf(1),createdAt:0,held:false}]})).toThrow();
});

function fixtureStore(count:number,stamp=(index:number)=>1_700_000_000+index*100):{workspace:string;root:string;identities:string[];cleanup():void}{
 const workspace=mkdtempSync(join(tmpdir(),"nxret-ws-")),root=join(workspace,"tools"),identities=Array.from({length:count},(_,index)=>identityOf(7000+index));
 mkdirSync(join(root,"nx"),{recursive:true});mkdirSync(join(workspace,".nx"),{recursive:true});
 identities.forEach((identity,index)=>{mkdirSync(join(root,"nx",identity));utimesSync(join(root,"nx",identity),stamp(index),stamp(index));});
 mkdirSync(join(root,"nx",".stage-copy-x"));mkdirSync(join(root,"nx","not-an-identity"));
 symlinkSync(join(root,"nx",identities[count-1]!),join(workspace,".nx","installation"));
 return {workspace,root,identities,cleanup:()=>rmSync(workspace,{recursive:true,force:true})};
}
const quiet={signal:new AbortController().signal,progress:async()=>{}};

test("the census reads generations with their birth time, ignores strangers and refuses a missing, foreign or oversized world",async()=>{
 const store=fixtureStore(12);
 try{
  const census=await censusNxToolingStoreV1(store.workspace,store.root,quiet);
  expect(census.kind).toBe("ok");if(census.kind!=="ok")return;
  expect(census.current).toBe(store.identities[11]);
  expect(census.images.map(row=>row.identity).sort()).toEqual([...store.identities].sort());
  for(const row of census.images){const facts=statSync(join(store.root,"nx",row.identity));expect(row.createdAt).toBe(Math.max(0,facts.birthtimeMs>0?facts.birthtimeMs:facts.mtimeMs));}
  expect((await censusNxToolingStoreV1(store.workspace,join(store.workspace,"nowhere"),quiet)).kind).toBe("absent");
  rmSync(join(store.workspace,".nx","installation"));expect((await censusNxToolingStoreV1(store.workspace,store.root,quiet)).kind).toBe("noCurrent");
  const foreign=mkdtempSync(join(tmpdir(),"nxret-foreign-"));try{symlinkSync(foreign,join(store.workspace,".nx","installation"));expect((await censusNxToolingStoreV1(store.workspace,store.root,quiet)).kind).toBe("noCurrent");}finally{rmSync(foreign,{recursive:true,force:true});}
  rmSync(join(store.workspace,".nx","installation"));symlinkSync(join(store.root,"nx",store.identities[3]!),join(store.workspace,".nx","installation"));
  await expect(censusNxToolingStoreV1(store.workspace,store.root,quiet,{...declared,maximumCensus:5})).rejects.toThrow("Mehr als 5");
  const controller=new AbortController();controller.abort();
  await expect(censusNxToolingStoreV1(store.workspace,store.root,{signal:controller.signal,progress:async()=>{}})).rejects.toThrow();
 }finally{store.cleanup();}
});

function ports(store:{workspace:string;root:string},options:{held?:ReadonlySet<string>;pressure?:boolean;dryRun?:boolean;gone?:Set<string>;busy?:boolean;stages?:string[];probes?:string[];signal?:AbortSignal;nowMs?:number;stubborn?:ReadonlySet<string>;busyPolls?:number;slept?:number[]}={}):NxToolingStoreBoundPortsV1{
 const gone=options.gone??new Set<string>(),stages=options.stages??[];
 return {
  signal:options.signal??new AbortController().signal,now:()=>options.nowMs??Date.now()+10*86400000,progress:async stage=>{stages.push(stage);},
  census:async()=>{const census=await censusNxToolingStoreV1(store.workspace,store.root,quiet);return census.kind==="ok"?{...census,images:census.images.filter(row=>!gone.has(row.identity))}:census;},
  held:async identity=>{options.probes?.push(identity);return options.held?.has(identity)??false;},
  pressure:async()=>options.pressure??false,freeBytes:async()=>1<<30,idle:async()=>(options.busyPolls=(options.busyPolls??0)-1)<0,sleep:async milliseconds=>{options.slept?.push(milliseconds);},
  ...(options.dryRun?{}:{retire:async(identities,limit,progress)=>{if(options.busy)return "busy" as const;const retired:string[]=[],held:string[]=[],kept:string[]=[];for(const[index,identity]of identities.entries()){if(retired.length>=limit)break;await progress(identity,index,identities.length);if(options.held?.has(identity))held.push(identity);else if(options.stubborn?.has(identity))kept.push(identity);else{gone.add(identity);retired.push(identity);}}return{retired,held,kept};}})
 };
}

test("a dry run plans without touching anything, an enforcing run retires oldest first around the live readers, a second run finds nothing",async()=>{
 const store=fixtureStore(30),wide={...declared,maximumPerRun:64};
 try{
  const census=await censusNxToolingStoreV1(store.workspace,store.root,quiet);if(census.kind!=="ok")throw Error("census");
  const born=new Map(census.images.map(row=>[row.identity,row.createdAt])),first=await runNxToolingStoreBoundV1(true,ports(store,{dryRun:true}),wide);
  expect(first.state).toBe("planned");
  const unheld=first.plan!.retire,held=new Set([unheld[1]!,unheld[7]!]),probes:string[]=[],stages:string[]=[],gone=new Set<string>();
  const newestEight=[...census.images].sort((left,right)=>right.createdAt-left.createdAt||(left.identity<right.identity?-1:1)).slice(0,declared.keepNewest).map(row=>row.identity),stay=declared.keepNewest+(newestEight.includes(census.current)?0:1);
  expect(unheld.length).toBe(30-stay);
  const times=unheld.map(identity=>born.get(identity)!);expect(times).toEqual([...times].sort((left,right)=>left-right));
  const dry=await runNxToolingStoreBoundV1(true,ports(store,{held,probes,stages,gone}),wide);
  expect(dry.state).toBe("planned");expect(gone.size).toBe(0);
  expect(dry.plan!.retire).toEqual(unheld.filter(identity=>!held.has(identity)));
  expect(dry.plan!.retained.filter(row=>row.reason==="held").map(row=>row.identity).sort()).toEqual([...held].sort());
  expect(new Set(probes).size).toBe(probes.length);expect([...probes].sort()).toEqual([...unheld].sort());
  expect(stages.some(stage=>stage.startsWith("nx-tooling:probe:"))).toBe(true);
  const done=await runNxToolingStoreBoundV1(false,ports(store,{held,stages,gone}),wide);
  expect(done.state).toBe("done");expect(done.outcome!.retired).toEqual(dry.plan!.retire);expect([...gone].sort()).toEqual([...dry.plan!.retire].sort());
  expect(stages.filter(stage=>stage.startsWith("nx-tooling:retire:")).length).toBe(dry.plan!.retire.length);
  const again=await runNxToolingStoreBoundV1(false,ports(store,{held,gone}),wide);
  expect(again.state).toBe("within");expect(again.plan!.retire).toEqual([]);
  const young=await runNxToolingStoreBoundV1(true,ports(store,{dryRun:true,nowMs:Math.min(...census.images.map(row=>row.createdAt))+1000}),wide);
  expect(young.plan!.retire).toEqual([]);expect(young.plan!.retained.filter(row=>row.reason==="young").length).toBe(30-stay);
 }finally{store.cleanup();}
});

test("the per-run limit counts actual retirements, so generations that stay never starve the younger ones, and repeated runs converge to the bound",async()=>{
 const store=fixtureStore(40);
 try{
  const census=await censusNxToolingStoreV1(store.workspace,store.root,quiet);if(census.kind!=="ok")throw Error("census");
  const gone=new Set<string>(),first=await runNxToolingStoreBoundV1(false,ports(store,{gone}));
  expect(first.plan!.limit).toBe(declared.maximumPerRun);expect(first.plan!.retire.length).toBe(40-first.plan!.retained.length);expect(first.outcome!.retired.length).toBe(declared.maximumPerRun);
  expect(first.outcome!.retired).toEqual(first.plan!.retire.slice(0,declared.maximumPerRun));
  const stubborn=new Set(first.plan!.retire.slice(declared.maximumPerRun,declared.maximumPerRun+3)),second=await runNxToolingStoreBoundV1(false,ports(store,{gone,stubborn}));
  expect(second.outcome!.kept.length).toBe(3);expect(second.outcome!.retired.length).toBe(Math.min(declared.maximumPerRun,second.plan!.retire.length-3));
  expect(second.outcome!.retired).not.toContain([...stubborn][0]);
  expect(formatNxToolingStoreBoundV1("en",second).join("\n")).toContain("3 kept");
  let runs=2;for(let again=await runNxToolingStoreBoundV1(false,ports(store,{gone,stubborn}));again.state==="done"&&runs<10;again=await runNxToolingStoreBoundV1(false,ports(store,{gone,stubborn})))runs++;
  const last=await runNxToolingStoreBoundV1(false,ports(store,{gone,stubborn}));
  expect(last.plan!.retire.every(identity=>stubborn.has(identity))).toBe(true);expect(gone.size).toBe(40-last.plan!.retained.length-stubborn.size);
 }finally{store.cleanup();}
});

test("the run waits for an idle image by polling, gives up after the bounded time, and a wait can be cancelled",async()=>{
 const store=fixtureStore(20);
 try{
  const slept:number[]=[],stages:string[]=[],gone=new Set<string>();
  const patient=await runNxToolingStoreBoundV1(false,ports(store,{busyPolls:3,slept,stages,gone}));
  expect(patient.state).toBe("done");expect(slept).toEqual([declared.idlePollMs,declared.idlePollMs,declared.idlePollMs]);expect(stages.filter(stage=>stage.startsWith("nx-tooling:wait:")).length).toBe(3);expect(gone.size).toBeGreaterThan(0);
  const stuck=fixtureStore(20),gave:number[]=[],again=await runNxToolingStoreBoundV1(false,ports(stuck,{busyPolls:1e9,slept:gave}));
  try{expect(again.state).toBe("busy");expect(again.outcome).toBeNull();expect(gave.length*declared.idlePollMs).toBe(declared.idleWaitMs);expect(formatNxToolingStoreBoundV1("de",again).join("\n")).toContain("blieb in Benutzung");}finally{stuck.cleanup();}
  const zero=await runNxToolingStoreBoundV1(false,ports(store,{busyPolls:5}),{...declared,idleWaitMs:0});expect(zero.state).toBe("busy");
  const controller=new AbortController(),aborting=ports(store,{busyPolls:1e9,signal:controller.signal}),retired=new Set<string>();
  await expect(runNxToolingStoreBoundV1(false,{...aborting,sleep:async()=>{controller.abort();},retire:async()=>{retired.add("called");return "busy" as const;}})).rejects.toThrow();expect(retired.size).toBe(0);
 }finally{store.cleanup();}
});

test("pressure tightens the bound, a busy image retires nothing and cancellation stops before any effect",async()=>{
 const store=fixtureStore(20);
 try{
  const calm=await runNxToolingStoreBoundV1(true,ports(store,{dryRun:true})),pressed=await runNxToolingStoreBoundV1(true,ports(store,{dryRun:true,pressure:true}));
  expect(pressed.plan!.retire.length).toBeGreaterThan(calm.plan!.retire.length);expect(pressed.plan!.keepNewest).toBe(declared.keepNewestUnderPressure);expect(pressed.pressure).toBe(true);
  const busy=await runNxToolingStoreBoundV1(false,ports(store,{busy:true}));expect(busy.state).toBe("busy");expect(busy.outcome).toBeNull();
  const controller=new AbortController(),gone=new Set<string>(),aborting=ports(store,{signal:controller.signal,gone});
  await expect(runNxToolingStoreBoundV1(false,{...aborting,held:async()=>{controller.abort();return false;}})).rejects.toThrow();
  expect(gone.size).toBe(0);
  rmSync(join(store.workspace,".nx","installation"));
  expect((await runNxToolingStoreBoundV1(false,ports(store))).state).toBe("noCurrent");
  expect((await runNxToolingStoreBoundV1(false,ports({workspace:store.workspace,root:join(store.workspace,"nowhere")}))).state).toBe("absent");
 }finally{store.cleanup();}
});

test("every report is the same story in English and German",async()=>{
 const store=fixtureStore(16);
 try{
  const reports=[await runNxToolingStoreBoundV1(true,ports(store,{dryRun:true})),await runNxToolingStoreBoundV1(false,ports(store,{pressure:true})),await runNxToolingStoreBoundV1(false,ports(store,{busy:true})),await runNxToolingStoreBoundV1(false,ports(store)),await runNxToolingStoreBoundV1(false,ports({workspace:store.workspace,root:join(store.workspace,"nowhere")}))];
  for(const report of reports){
   const english=formatNxToolingStoreBoundV1("en",report),german=formatNxToolingStoreBoundV1("de",report);
   expect(english.length).toBe(german.length);
   for(const line of [...english,...german])expect(line).not.toMatch(/[{}]|undefined|NaN/u);
   expect(english.join("\n")).not.toBe(german.join("\n"));
   const numbers=(lines:readonly string[])=>lines.join(" ").match(/\d+/gu);expect(numbers(english)).toEqual(numbers(german));
  }
  const done=reports[1]!;expect(done.state).toBe("done");expect(formatNxToolingStoreBoundV1("en",done).join("\n")).toContain("Retired "+done.outcome!.retired.length+" of "+done.plan!.retire.length);
  expect(formatNxToolingStoreBoundV1("de",done).join("\n")).toContain("Entfernt: "+done.outcome!.retired.length+" von "+done.plan!.retire.length);
 }finally{store.cleanup();}
 for(const key of Object.keys(texts.messages).filter(key=>key.startsWith("nxTooling."))){const row=(texts.messages as any)[key];expect(row.en.length>0&&row.de.length>0,key).toBe(true);for(const language of ["en","de"] as CacheLanguageV1[])expect(cacheTextV1(language,key,{reason:"r",count:1,total:1,keep:1,held:0,young:0,free:"1 B",oldest:"abc",limit:1,remaining:0,retired:1,planned:1,kept:0,before:"1 B",after:"2 B",message:"m"})).not.toMatch(/[{}]/u);}
});

test("the lease probe sees a real reader in another process and nothing else",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"nxret-lease-")),identity=identityOf(9001),other=identityOf(9002),signal=new AbortController().signal;
 let child:ReturnType<typeof startToolV1>|undefined;
 try{
  expect(await probeNxToolingHeldV1(directory,identity,signal)).toBe(false);
  child=startToolV1("bun",[holder,directory,"nx-tooling:"+identity],{stdio:["pipe","pipe","inherit"]});
  await new Promise<void>((done,fail)=>{child!.stdout!.on("data",data=>{if(String(data).includes("READY"))done();});child!.on("error",fail);child!.on("exit",code=>fail(Error("holder exited "+code)));});
  expect(await probeNxToolingHeldV1(directory,identity,signal)).toBe(true);
  expect(await probeNxToolingHeldV1(directory,other,signal)).toBe(false);
  const exited=new Promise<void>(done=>child!.on("exit",()=>done()));child.stdin!.end();await exited;child=undefined;
  expect(await probeNxToolingHeldV1(directory,identity,signal)).toBe(false);
 }finally{try{child?.kill();}catch{}rmSync(directory,{recursive:true,force:true});}
},60000);

test("the image probe sees a real reader of the whole image in another process and a free image otherwise",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"nxret-image-")),image="/tmp/nxret-fake-image",signal=new AbortController().signal;
 let child:ReturnType<typeof startToolV1>|undefined;
 try{
  expect(await probeNxToolingImageIdleV1(directory,image,signal)).toBe(true);
  child=startToolV1("bun",[holder,directory,"native-fixed-image:"+image],{stdio:["pipe","pipe","inherit"]});
  await new Promise<void>((done,fail)=>{child!.stdout!.on("data",data=>{if(String(data).includes("READY"))done();});child!.on("error",fail);child!.on("exit",code=>fail(Error("holder exited "+code)));});
  expect(await probeNxToolingImageIdleV1(directory,image,signal)).toBe(false);
  expect(await probeNxToolingImageIdleV1(directory,image+"-other",signal)).toBe(true);
  const exited=new Promise<void>(done=>child!.on("exit",()=>done()));child.stdin!.end();await exited;child=undefined;
  expect(await probeNxToolingImageIdleV1(directory,image,signal)).toBe(true);
 }finally{try{child?.kill();}catch{}rmSync(directory,{recursive:true,force:true});}
},60000);

test("the idle wait follows the machine's override and refuses anything but whole milliseconds within the bound, in both languages",()=>{
 const name=NX_TOOLING_IDLE_WAIT_ENVIRONMENT_V1;
 expect(resolveNxToolingStoreBoundPolicyV1({})).toEqual(declared);expect(resolveNxToolingStoreBoundPolicyV1({[name]:""})).toEqual(declared);
 expect(resolveNxToolingStoreBoundPolicyV1({[name]:"600000"}).idleWaitMs).toBe(600000);expect(resolveNxToolingStoreBoundPolicyV1({[name]:"0"}).idleWaitMs).toBe(0);expect(resolveNxToolingStoreBoundPolicyV1({[name]:"1500"})).toEqual({...declared,idleWaitMs:1500});
 for(const bad of ["-1","1.5","abc","600001","1e3","01"," 5","5 ","0x10","9999999999"]){expect(()=>resolveNxToolingStoreBoundPolicyV1({[name]:bad}),bad).toThrow("Closed Nx toolkit store bound policy required");expect(()=>resolveNxToolingStoreBoundPolicyV1({[name]:bad}),bad).toThrow("Geschlossene Richtlinie");}
});
