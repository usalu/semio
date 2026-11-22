/** 🏊️ Contract: the project farm. The plan, the languages, the texts and the heap environment of the farm equal an independent model; a pool of real worker processes answers the same for any worker count, completion order and crash; the leaves staged by any number of workers are adopted into one tree whose digest is the same for every worker count, assignment and order (and the digest of the independent model), however many adopters race for the same leaf. */
import {afterAll,afterEach,expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,statSync,writeFileSync} from "node:fs";
import {uptime} from "node:os";
import {dirname,join,relative,sep} from "node:path";
import {fileURLToPath,pathToFileURL} from "node:url";
import {createHash} from "node:crypto";
import {brotliCompressSync,brotliDecompressSync} from "node:zlib";
import Ajv from "ajv";
import fixture from "./🔣️.json";
import fixtureSchema from "./🧬️schema/🔣️.json";
import schema from "../🏊️pool/🧬️schema/🔣️.json";
import {awaitGrowingV1,recycleReasonV1,choosePoolV1,grantedPoolV1,ownGrantPoolV1,GRAPH_FARM_MESSAGES_V1,GRAPH_FARM_POLICY_V1,createFarmV1,graphFarmRuntimeEnvironmentV1,planGraphFarmV1,renderGraphFarmMessagesV1,selectGraphFarmLanguagesV1} from "../🏊️pool/🟨️.mjs";
import {EventEmitter} from "node:events";
import {closeProjectFarmsV1,openProjectFarmV1 as openFarmV1} from "../🟨️.mjs";
import {inferenceRecordIdentityV1} from "../../../../../👁️observation/🟨️.mjs";
import {graphFarmIdentityV1} from "../🪆️captured/🟨️.mjs";
import {adoptStagedGraphLeavesV1} from "../../../../../📥️inputs/🟨️.mjs";
import {withBuildGraphStagingV1} from "../../../../../📥️inputs/💾️storage/🟨️.mjs";
const openProjectFarmV1=(options:Parameters<typeof openFarmV1>[0])=>openFarmV1({adopt:(()=>({published:1,present:0,declined:false})) as never,...options});

const here=dirname(fileURLToPath(import.meta.url)),python=testPythonV1(),oracle=join(here,"🐍️.py"),fixturePath=join(here,"🔣️.json"),schemaPath=join(here,"..","🏊️pool","🧬️schema","🔣️.json");
const ask=(command:string)=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,fixturePath,schemaPath,command]).stdout.toString());
const roots:string[]=[],scratch=()=>{const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Owned test artifacts directory required");mkdirSync(artifacts,{recursive:true});const root=mkdtempSync(join(realpathSync(artifacts),"f-"));roots.push(root);return root;};
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
afterEach(async()=>{await closeProjectFarmsV1();});
const sha=(data:string|Buffer)=>createHash("sha256").update(data).digest("hex");
const worker=join(here,"..","👷️worker","🟨️.mjs"),service=pathToFileURL(join(here,"🧫️fixtures","🛰️service","🟨️.mjs")).href,adopter=join(here,"🧫️fixtures","🛰️adopter","🟨️.mjs");
const corpus=fixture.corpus;

test("the fixture and the declared farm are closed under their schemas, validated by an independent implementation",()=>{
 const ajv=new Ajv({strict:true});
 expect(ajv.validate(fixtureSchema,fixture),JSON.stringify(ajv.errors)).toBe(true);
 const messages=schema.$defs.Messages.const as Record<string,Record<string,string>>;
 expect(Object.keys(messages.en).sort()).toEqual(Object.keys(messages.de).sort());
 for(const key of Object.keys(messages.en)){
  const fields=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
  expect(fields(messages.de[key]!),key).toEqual(fields(messages.en[key]!));
 }
 expect(GRAPH_FARM_POLICY_V1.languages).toEqual(["en","de"]);
});

test("the plan of the farm equals the independent model for every case",()=>{
 const expected=ask("plan");
 fixture.plan.forEach((row,index)=>{
  const want=expected[index];
  if(want.error){expect(()=>planGraphFarmV1({projects:row.projects,environment:row.environment,cores:row.cores}),JSON.stringify(row)).toThrow();return;}
  expect(planGraphFarmV1({projects:row.projects,environment:row.environment,cores:row.cores}),JSON.stringify(row)).toEqual(want);
 });
});

test("the languages and the texts equal the independent model, English first and German second, with no default language",()=>{
 const languages=ask("languages");
 fixture.languages.forEach((row,index)=>expect(selectGraphFarmLanguagesV1(row.environment),JSON.stringify(row)).toEqual(languages[index]));
 expect(selectGraphFarmLanguagesV1({})).toEqual(["en","de"]);
 const rendered=ask("messages");
 fixture.messages.forEach((row,index)=>[{},{LANG:"de_DE.UTF-8"},{LANG:"en_GB"}].forEach((environment,at)=>expect(renderGraphFarmMessagesV1(row.key,Object.fromEntries(Object.entries(row.fields).flatMap(([key,value])=>value===undefined?[]:[[key,value]])),environment),row.key).toEqual(rendered[index][at])));
 expect(()=>renderGraphFarmMessagesV1("missing" as never,{},{})).toThrow();
 expect(Object.keys(GRAPH_FARM_MESSAGES_V1.en)).toEqual(Object.keys(GRAPH_FARM_MESSAGES_V1.de));
});

test("the heap environment of a worker equals the independent model for every runtime",()=>{
 const expected=ask("runtime");
 fixture.runtime.forEach((row,index)=>expect(graphFarmRuntimeEnvironmentV1(row.runtime,row.megabytes),JSON.stringify(row)).toEqual(expected[index]));
});

test("the worker pools a process starts equal the independent model: the pool of its own grant, the pool of any host grant and the pool that takes the next job",()=>{
 const expected=ask("pools");
 fixture.pools.own.forEach((row,index)=>{if((row as {refused?:boolean}).refused)expect(()=>ownGrantPoolV1(row.environment),row.name).toThrow();else expect(ownGrantPoolV1(row.environment),row.name).toEqual(expected.own[index]??undefined);expect(expected.own[index],row.name).toEqual(row.expect);});
 fixture.pools.granted.forEach((row,index)=>{expect(grantedPoolV1(row.granted??undefined),row.name).toEqual(expected.granted[index]??undefined);expect(expected.granted[index],row.name).toEqual(row.expect);});
 fixture.pools.choose.forEach((row,index)=>{expect(choosePoolV1(row.pools),row.name).toBe(expected.choose[index]);expect(row.expect,row.name).toBe(expected.choose[index]);});
 expect(GRAPH_FARM_POLICY_V1.ownWorkers).toBeGreaterThanOrEqual(1);expect(GRAPH_FARM_POLICY_V1.minimumGrantedWorkers).toBe(1);
});

const farm=(options:Partial<Parameters<typeof createFarmV1>[0]>={})=>createFarmV1({entry:worker,service,session:{identity:"law"},workers:2,megabytes:100000,environment:{...process.env},accept:(identity:unknown)=>identity==="law",...options} as never);

test("a pool of real worker processes answers every job with the same value for any worker count and completion order",async()=>{
 for(const workers of corpus.workers){
  const pool=farm({workers}),outcomes=await Promise.all(fixture.jobs.map(job=>pool.run({kind:"echo",n:job.n,delay:job.delay})));
  await pool.close();
  outcomes.forEach((outcome,index)=>{expect(outcome.reply,`workers ${workers} job ${index}`).toBeDefined();expect((outcome.reply as {n:number;value:number}).n).toBe(fixture.jobs[index]!.n);expect((outcome.reply as {value:number}).value).toBe(fixture.jobs[index]!.value);});
  const pids=new Set(outcomes.map(outcome=>(outcome.reply as {pid:number}).pid));
  expect(pids.size).toBeLessThanOrEqual(workers);expect(pool.stats().replies).toBe(fixture.jobs.length);
 }
},120000);

test("a job that throws is a failure with the error of the service, a lost process is retried on a fresh one, and a job that kills every process it meets is lost",async()=>{
 const root=scratch(),lost:string[]=[],pool=farm({workers:2,report:(event:string,fields:{reason:string})=>{if(event==="lost")lost.push(fields.reason);}});
 const failure=await pool.run({kind:"throw",n:3});
 expect(failure).toEqual({failure:{name:"Error",message:"boom 3",code:"BOOM"}});
 const marker=join(root,"crashed"),retried=await pool.run({kind:"crash",n:4,marker});
 expect((retried.reply as {value:number}).value).toBe((4*4+7)%1013);expect(existsSync(marker)).toBe(true);expect(lost.length).toBe(1);
 const dead=await pool.run({kind:"crash-always",n:5});
 expect(dead.lost).toBeDefined();expect(pool.stats().lost).toBe(1+GRAPH_FARM_POLICY_V1.maximumAttempts);
 const after=await pool.run({kind:"echo",n:6});
 expect((after.reply as {value:number}).value).toBe((6*6+7)%1013);
 await pool.close();
},120000);

test("a job that never answers is lost after the job timeout on every attempt, without a timer that outlives the pool",async()=>{
 const policy={...GRAPH_FARM_POLICY_V1,jobTimeoutMilliseconds:400},pool=farm({workers:1,policy});
 const started=Date.now(),outcome=await pool.run({kind:"hang",n:1});
 expect(outcome.lost).toBe("job timeout");expect(Date.now()-started).toBeGreaterThanOrEqual(400*policy.maximumAttempts-50);
 await pool.close();
},60000);

test("a worker whose identity differs disables the pool and every job is lost, never run",async()=>{
 const reasons:string[]=[],pool=farm({accept:()=>false,report:(event:string,fields:{reason:string})=>{if(event==="disabled")reasons.push(fields.reason);}});
 expect(await pool.ready).toBe(false);
 expect(reasons).toEqual(["identity differs"]);
 const outcome=await pool.run({kind:"echo",n:1});
 expect(outcome.lost).toBe("disabled: identity differs");expect(pool.usable).toBe(false);
 await pool.close();
},60000);

test("a worker over its memory is replaced after its job and every process that left is reported to reclaim what it left",async()=>{
 const gone:string[]=[],pool=farm({workers:1,megabytes:1,policy:{...GRAPH_FARM_POLICY_V1,recycleCeilingFactor:0},retired:(slot:number,generation:number)=>gone.push(slot+":"+generation)});
 for(let n=0;n<3;n++)expect(((await pool.run({kind:"echo",n})).reply as {n:number}).n).toBe(n);
 expect(pool.stats().recycled).toBe(3);
 await pool.close();
 await Bun.sleep(200);
 expect(gone.sort()).toEqual(["0:1","0:2","0:3"]);
},60000);

test("cancelling the signal settles a waiting job as lost, stops every worker process and refuses new jobs",async()=>{
 const controller=new AbortController(),pool=farm({workers:1,signal:controller.signal}),pid=((await pool.run({kind:"echo",n:1})).reply as {pid:number}).pid,pending=pool.run({kind:"hang",n:2});
 await Bun.sleep(500);
 controller.abort(new Error("cancelled by law"));
 expect((await pending).lost).toBe("closed");
 await Bun.sleep(500);
 expect(()=>process.kill(pid,0)).toThrow();
 expect((await pool.run({kind:"echo",n:3})).lost).toBe("closed");
 expect(()=>pool.run({kind:"echo",n:4},{signal:controller.signal})).not.toThrow();
 await pool.close();
},60000);

const hex24=(text:string)=>sha(text).slice(0,24);
const stage=(root:string,project:typeof corpus.projects[number],written:Set<string>)=>{
 const names:string[]=[],put=(name:string,bytes:Buffer|string)=>{if(written.has(name))return;written.add(name);names.push(name);mkdirSync(dirname(join(root,name)),{recursive:true});writeFileSync(join(root,name),bytes);};
 for(const index of project.closures){const row=corpus.closures[index]!;put(`${row.namespace}/${sha(row.text)}.json`,row.text);}
 for(const index of project.blocks)put(`inference/np/ob/${sha(corpus.blocks[index]!)}.br`,brotliCompressSync(Buffer.from(corpus.blocks[index]!)));
 for(const receipt of project.receipts)put(`inference/${receipt.name}`,receipt.text);
 for(const pack of project.packs)put(`inference/${pack.name}`,pack.text);
 put(`inference/np/${hex24(project.root)}.json`,project.record);
 return names;
};
const walk=(root:string,at=root):string[]=>readdirSync(at,{withFileTypes:true}).flatMap(entry=>entry.isDirectory()?walk(root,join(at,entry.name)):[relative(root,join(at,entry.name)).split(sep).join("/")]);
const treeDigest=(root:string)=>{
 const lines=walk(root).sort().map(name=>{const bytes=readFileSync(join(root,name));return `${name}\0${sha(/^inference\/np\/ob\/[0-9a-f]{64}\.br$/u.test(name)?brotliDecompressSync(bytes):bytes)}\n`;});
 return {digest:sha(lines.join("")),names:lines.map(line=>line.split("\0")[0]!)};
};
const random=(seed:number)=>()=>{seed|=0;seed=seed+0x6D2B79F5|0;let value=Math.imul(seed^seed>>>15,1|seed);value=value+Math.imul(value^value>>>7,61|value)^value;return((value^value>>>14)>>>0)/4294967296;};
const shuffled=<T>(items:readonly T[],next:()=>number)=>{const out=[...items];for(let index=out.length-1;index>0;index--){const at=Math.floor(next()*(index+1));[out[index],out[at]]=[out[at]!,out[index]!];}return out;};
const adopt=(target:string,staging:string,names:readonly string[])=>withBuildGraphStagingV1({root:target},()=>adoptStagedGraphLeavesV1("/unused",staging,names)) as {published:number;present:number;declined:boolean};

test("the leaves staged by k workers are adopted into the tree of the independent model for every worker count, assignment and order",()=>{
 const expected=ask("tree");
 for(const workers of corpus.workers)for(const seed of corpus.seeds){
  const next=random(seed),stagings=Array.from({length:workers},()=>scratch()),writtenBy=stagings.map(()=>new Set<string>()),target=scratch();
  const jobs=shuffled(corpus.projects.map(project=>({project,worker:Math.floor(next()*workers)})),next).map(job=>({...job,names:[] as string[]}));
  const adoptedFrom=new Map<number,number>();
  for(const job of jobs){job.names=stage(stagings[job.worker]!,job.project,writtenBy[job.worker]!);}
  for(const job of shuffled(jobs,next)){const plan=adopt(target,stagings[job.worker]!,job.names);expect(plan.declined).toBe(false);adoptedFrom.set(job.worker,(adoptedFrom.get(job.worker)??0)+plan.published);}
  const actual=treeDigest(target);
  expect(actual.names,`workers ${workers} seed ${seed}`).toEqual(expected.names);
  expect(actual.digest,`workers ${workers} seed ${seed}`).toBe(expected.digest);
 }
});

test("adopting the same leaves again publishes nothing and no leaf that stood is rewritten",()=>{
 const staging=scratch(),target=scratch(),written=new Set<string>(),names=corpus.projects.flatMap(project=>stage(staging,project,written));
 const first=adopt(target,staging,names),leaves=names.filter(name=>name.startsWith("inference/")),stamp=(name:string)=>statSync(join(target,name)).mtimeMs,before=leaves.map(stamp);
 const second=adopt(target,staging,names);
 expect(first.declined||second.declined).toBe(false);expect(first.published).toBe(leaves.length);expect(second.published).toBe(0);expect(second.present).toBe(leaves.length);
 expect(leaves.map(stamp)).toEqual(before);expect(treeDigest(target).digest).toBe(ask("tree").digest);
});

test.skipIf(process.platform==="win32")("the record that names the other leaves is published last, so a failing batch never leaves a record whose leaves are missing",()=>{
 const staging=scratch(),target=scratch(),project=corpus.projects[0]!,names=stage(staging,project,new Set<string>()),record=names.find(name=>/^inference\/np\/[0-9a-f]{24}\.json$/u.test(name))!;
 mkdirSync(join(target,"inference","rr"),{recursive:true});chmodSync(join(target,"inference","rr"),0o500);
 try{expect(()=>adopt(target,staging,[record,...names.filter(name=>name!==record)])).toThrow();expect(existsSync(join(target,record))).toBe(false);}
 finally{chmodSync(join(target,"inference","rr"),0o700);}
});

test("a receipt or pack that stands is the first writer's and a record is replaced by the newer one",()=>{
 const first=scratch(),second=scratch(),target=scratch(),project=corpus.projects[0]!,receipt=`inference/${project.receipts[0]!.name}`,record=`inference/np/${hex24(project.root)}.json`;
 const names=stage(first,project,new Set<string>());stage(second,project,new Set<string>());
 writeFileSync(join(second,receipt),"another writer");writeFileSync(join(second,record),"a newer record");
 adopt(target,first,names);
 const plan=adopt(target,second,[receipt,record]);
 expect(readFileSync(join(target,receipt),"utf8")).toBe(project.receipts[0]!.text);
 expect(readFileSync(join(target,record),"utf8")).toBe("a newer record");
 expect(plan.present).toBe(1);expect(plan.published).toBe(1);
});

test("a staged leaf that does not hold what its name says is refused, and a damaged block or closure that stood is repaired by the verified copy",()=>{
 const staging=scratch(),target=scratch(),good=corpus.blocks[0]!,name=`inference/np/ob/${sha(good)}.br`;
 mkdirSync(join(staging,"build-input-closures"),{recursive:true});mkdirSync(join(staging,"inference","np","ob"),{recursive:true});
 writeFileSync(join(staging,"build-input-closures",`${sha("one")}.json`),"two");
 expect(()=>adopt(target,staging,[`build-input-closures/${sha("one")}.json`])).toThrow("Closure content differs from its name");
 writeFileSync(join(staging,"inference","np","ob",`${sha("one")}.br`),brotliCompressSync(Buffer.from("two")));
 expect(()=>adopt(target,staging,[`inference/np/ob/${sha("one")}.br`])).toThrow("Block content differs from its name");
 for(const bad of["inference/../escape","build-input-closures/../x.json","unknown/leaf","inference","inference//x"])expect(()=>adopt(target,staging,[bad]),bad).toThrow("Relative staged graph leaf of a declared namespace required");
 writeFileSync(join(staging,name),brotliCompressSync(Buffer.from(good)));
 mkdirSync(join(target,"inference","np","ob"),{recursive:true});writeFileSync(join(target,name),"damaged");
 expect(adopt(target,staging,[name]).published).toBe(1);
 expect(sha(brotliDecompressSync(readFileSync(join(target,name))))).toBe(sha(good));
 const closure=corpus.closures[0]!,closureName=`${closure.namespace}/${sha(closure.text)}.json`;
 mkdirSync(join(staging,closure.namespace),{recursive:true});writeFileSync(join(staging,closureName),closure.text);
 mkdirSync(join(target,closure.namespace),{recursive:true});writeFileSync(join(target,closureName),closure.text+" ");
 expect(()=>adopt(target,staging,[closureName])).not.toThrow();expect(readFileSync(join(target,closureName),"utf8")).toBe(closure.text);
});

test("four processes that adopt every leaf of the corpus at the same moment leave exactly one copy of each leaf and nothing else",async()=>{
 for(let round=0;round<3;round++){
  const staging=scratch(),target=scratch(),written=new Set<string>(),names=corpus.projects.flatMap(project=>stage(staging,project,written)),startAt=Date.now()+1500;
  const children=Array.from({length:4},()=>Bun.spawn([process.execPath,adopter,JSON.stringify({stagingRoot:staging,targetRoot:target,names,startAt})],{stdout:"pipe",stderr:"pipe"}));
  const results=await Promise.all(children.map(async child=>({code:await child.exited,out:await new Response(child.stdout).text(),err:await new Response(child.stderr).text()})));
  for(const result of results)expect(result.code,result.err).toBe(0);
  const actual=treeDigest(target),expected=ask("tree");
  expect(actual.names).toEqual(expected.names);expect(actual.digest).toBe(expected.digest);
  expect(walk(target).filter(name=>/\.tmp$|~/u.test(name))).toEqual([]);
 }
},120000);

test("a loop started while the others are awaited is awaited too, and a rejection of any of them rejects",async()=>{
 const done:string[]=[],loops:Promise<void>[]=[],late=()=>loops.push((async()=>{await Bun.sleep(60);done.push("late");})());
 loops.push((async()=>{await Bun.sleep(20);late();done.push("first");})());
 await awaitGrowingV1(loops);
 expect(done.sort()).toEqual(["first","late"]);
 const failing:Promise<void>[]=[(async()=>{await Bun.sleep(10);throw new Error("loop failed");})()];
 await expect(awaitGrowingV1(failing)).rejects.toThrow("loop failed");
});

/** 🧪️ A scripted child process: announces the identity of this implementation and every `run` with an empty reply, so the farm wiring runs without the real inference. */
class ScriptedChild extends EventEmitter{
 connected=true;exitCode:number|null=null;signalCode:string|null=null;jobs:number[]=[];
 constructor(readonly tag:string,readonly log:string[]){super();setImmediate(()=>this.announce());}
 announce(){this.emit("message",{type:"ready",pid:1,identity:graphFarmIdentityV1(inferenceRecordIdentityV1())});}
 send(message:{type:string;id?:number}){
  if(message.type==="run"){this.jobs.push(message.id!);this.log.push(this.tag+":run");setImmediate(()=>this.emit("message",{type:"result",id:message.id,reply:{names:[],cpuMilliseconds:1},rssMegabytes:1}));}
  else if(message.type==="shutdown"){this.exitCode=0;setImmediate(()=>this.emit("exit"));}
  return true;
 }
 kill(){this.exitCode=0;setImmediate(()=>this.emit("exit"));return true;}
}

test("the farm starts the worker its own capacity grant pays for, accepts a host grant of one worker as a second pool, and sends jobs to the pool with an idle worker",async()=>{
 const root=scratch(),log:string[]=[],children:ScriptedChild[]=[],lines:string[]=[];
 let grant!:(value:{jobs:number;megabytes:number}|undefined)=>void;
 const released:string[]=[],asked:number[]=[];
 const open=(environment:Record<string,string>)=>openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment,processed:()=>24,cores:10,write:line=>lines.push(line),
  spawnProcess:((_command:string,_args:readonly string[],options:{env:Record<string,string>})=>{const child=new ScriptedChild(String(children.length),log);children.push(child);return child;}) as never,
  request:(((input:{preferredJobs:number}) => {asked.push(input.preferredJobs);let cancel!:(value:undefined)=>void;const granted=new Promise<{jobs:number;megabytes:number}|undefined>(done=>{grant=done;cancel=done;});return {granted,release:()=>{released.push("host");cancel(undefined);}};}) as never)})!;
 const id="farm_live_0001",capacity=join(root,"capacity"),plan=JSON.stringify({version:1,id,jobs:1,cpu:1,memoryMegabytes:2048,fixedMegabytes:1536,floorMegabytes:1981});
 mkdirSync(join(capacity,"grants"),{recursive:true});
 writeFileSync(join(capacity,"grants",id+".json"),JSON.stringify({id,operation:"run",jobs:1,cpu:1,reservedMegabytes:7168,admittedAtMs:Date.now(),observedMegabytes:null,pid:process.pid,bootAtMs:Math.round((Date.now()-uptime()*1000)/1000)*1000}));

 const farm=open({LANG:"en_US.UTF-8",SEMIO_SLICE_LEAN:"off",SEMIO_HOST_CAPACITY_DIRECTORY:capacity,SEMIO_HOST_CAPACITY_PLAN:plan});
 const job=(name:string)=>({name,root:name,json:{},native:false,pipeline:false,nativeRoot:name,selectedInputs:[],head:[],cacheKey:"0".repeat(24)});

 for(let index=0;index<23;index++)expect(await farm.prepare(job("miss"+index))).toBe(false);
 expect(children.length).toBe(0);

 expect(await farm.prepare(job("engage"))).toBe(false);
 expect(children.length).toBe(1);
 expect(lines.some(line=>line.includes("inside the capacity grant of this process (2048 MB)"))).toBe(true);

 const loops=await new Promise<number>(done=>farm.onReady(done));
 expect(loops).toBe(GRAPH_FARM_POLICY_V1.loopsPerWorker);

 await farm.prepare(job("first"));
 expect(log).toEqual(["0:run"]);

 grant({jobs:1,megabytes:3584});await Bun.sleep(50);
 const grown=await new Promise<number>(done=>farm.onReady(total=>{if(total>loops)done(total);}));
 expect(grown).toBe(2*GRAPH_FARM_POLICY_V1.loopsPerWorker);expect(children.length).toBe(2);

 await Promise.all([farm.prepare(job("a")),farm.prepare(job("b"))]);
 expect(log.filter(entry=>entry==="1:run").length).toBeGreaterThanOrEqual(1);

 await farm.close();
 expect(released,"the pools outlive the construction").toEqual([]);
 await closeProjectFarmsV1();
 expect(released).toEqual(["host","host"]);
 expect(asked.length).toBe(2);expect(asked[1]).toBe(asked[0]!-2);
 expect(lines.some(line=>line.includes("3 of 566")||line.includes("projects inferred by 2 worker processes"))).toBe(true);
});

test("a cold generation starts the owned worker before a quarter of the projects miss",async()=>{
 const root=scratch(),children:ScriptedChild[]=[],lines:string[]=[],log:string[]=[];
 const id="farm_cold_00001",capacity=join(root,"capacity"),plan=JSON.stringify({version:1,id,jobs:1,cpu:1,memoryMegabytes:2048,fixedMegabytes:1536,floorMegabytes:1981});
 mkdirSync(join(capacity,"grants"),{recursive:true});
 writeFileSync(join(capacity,"grants",id+".json"),JSON.stringify({id,operation:"run",jobs:1,cpu:1,reservedMegabytes:7168,admittedAtMs:Date.now(),observedMegabytes:null,pid:process.pid,bootAtMs:Math.round((Date.now()-uptime()*1000)/1000)*1000}));
 let grant!:(value:{jobs:number;megabytes:number}|undefined)=>void;
 const farm=openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8",SEMIO_SLICE_LEAN:"off",SEMIO_HOST_CAPACITY_DIRECTORY:capacity,SEMIO_HOST_CAPACITY_PLAN:plan},processed:()=>400,cores:10,write:line=>lines.push(line),engage:true,spawnProcess:(()=>{const child=new ScriptedChild("cold",log);children.push(child);return child;}) as never,request:(()=>({granted:new Promise<{jobs:number;megabytes:number}|undefined>(done=>{grant=done;}),release:()=>undefined})) as never})!;
 expect(children.length).toBe(1);
 expect(lines.some(line=>line.includes("inside the capacity grant of this process"))).toBe(true);
 const job={name:"p",root:"p",json:{},native:false,pipeline:false,nativeRoot:"p",selectedInputs:[],head:[],cacheKey:"0".repeat(24)};
 expect(await farm.prepare(job),"the owner waited for the worker and was adopted from it, never inferred by the parent").toBe(true);
 await new Promise<number>(done=>farm.onReady(done));
 await farm.prepare(job);
 expect(log,"the first project waited for the worker the grant pays for and ran on it").toEqual(["cold:run","cold:run"]);
 grant(undefined);
 await farm.close();await closeProjectFarmsV1();
});

test("a farm without a capacity grant of its own and a host that grants nothing infers in the parent, and a host grant of zero jobs starts no pool",async()=>{
 const root=scratch(),children:ScriptedChild[]=[],lines:string[]=[];
 let grant!:(value:{jobs:number;megabytes:number}|undefined)=>void;
 const farm=openProjectFarmV1({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],environment:{LANG:"en_US.UTF-8",SEMIO_SLICE_LEAN:"off"},processed:()=>24,cores:10,write:line=>lines.push(line),
  spawnProcess:(()=>{const child=new ScriptedChild("x",[]);children.push(child);return child;}) as never,
  request:(() => ({granted:new Promise<{jobs:number;megabytes:number}|undefined>(done=>{grant=done;}),release:()=>undefined})) as never})!;
 const job={name:"p",root:"p",json:{},native:false,pipeline:false,nativeRoot:"p",selectedInputs:[],head:[],cacheKey:"0".repeat(24)};
 for(let index=0;index<24;index++)await farm.prepare(job);
 grant({jobs:0,megabytes:2048});
 await Bun.sleep(20);
 expect(children.length).toBe(0);expect(await farm.prepare(job)).toBe(false);
 expect(lines.some(line=>line.includes("in this process (host capacity)"))).toBe(true);
 await farm.close();
});

test("a worker is replaced after the declared share of its memory or the declared number of projects, as the independent model decides",()=>{
 const expected=ask("recycle");
 fixture.recycle.forEach((row,index)=>expect(recycleReasonV1(row)??null,JSON.stringify(row)).toBe(expected[index]));
 expect(expected.filter((value:unknown)=>value==="memory").length).toBeGreaterThan(3);
 expect(expected.filter((value:unknown)=>value==="projects").length).toBeGreaterThan(1);
 expect(expected.filter((value:unknown)=>value===null).length).toBeGreaterThan(3);
});

test("a farm leaves no abort listener on the long-lived signal of its invocation once it is closed",async()=>{
 const listeners=new Set<unknown>(),signal={aborted:false,reason:undefined,throwIfAborted(){},addEventListener:(_type:string,listener:unknown)=>listeners.add(listener),removeEventListener:(_type:string,listener:unknown)=>listeners.delete(listener)} as unknown as AbortSignal;
 const children:ScriptedChild[]=[],made=farm({workers:1,signal,spawnProcess:(()=>{const child=new ScriptedChild("x",[]);children.push(child);return child;}) as never});
 expect(listeners.size).toBe(1);
 await made.close();
 expect(listeners.size).toBe(0);
});

test("a worker without a measured baseline is still recycled by the memory ratio, as the independent model decides",()=>{
 const ratio=GRAPH_FARM_POLICY_V1.recycleMemoryRatio,megabytes=2048,rows=[1,Math.floor(megabytes*ratio),Math.floor(megabytes*ratio)+1,megabytes,megabytes+1].flatMap(heldMegabytes=>[undefined,Number.NaN,"x"].map(baselineMegabytes=>({heldMegabytes,jobs:1,megabytes,baselineMegabytes}))),
 expected=rows.map(row=>row.heldMegabytes>megabytes||row.heldMegabytes>megabytes*ratio?"memory":null);
 rows.forEach((row,index)=>expect(recycleReasonV1(row as never)??null,JSON.stringify(row)).toBe(expected[index]));
 expect(expected.filter(value=>value==="memory").length).toBeGreaterThan(3);
});

class SilentChild extends ScriptedChild{override announce(){}override send(){return true;}}
class GatedChild extends ScriptedChild{constructor(tag:string,log:string[],readonly gate:Promise<void>){super(tag,log);}override announce(){void this.gate.then(()=>super.announce());}}

const grantFile=(root:string,id:string)=>{
 const capacity=join(root,"capacity"),plan=JSON.stringify({version:1,id,jobs:1,cpu:1,memoryMegabytes:2048,fixedMegabytes:1536,floorMegabytes:1981});
 mkdirSync(join(capacity,"grants"),{recursive:true});
 writeFileSync(join(capacity,"grants",id+".json"),JSON.stringify({id,operation:"run",jobs:1,cpu:1,reservedMegabytes:7168,admittedAtMs:Date.now(),observedMegabytes:null,pid:process.pid,bootAtMs:Math.round((Date.now()-uptime()*1000)/1000)*1000}));
 return {LANG:"en_US.UTF-8",SEMIO_SLICE_LEAN:"off",SEMIO_HOST_CAPACITY_DIRECTORY:capacity,SEMIO_HOST_CAPACITY_PLAN:plan};
};
const pendingRequest=(released:string[]=[])=>(()=>{let cancel!:(value:undefined)=>void;return{granted:new Promise<undefined>(done=>{cancel=done;}),release:()=>{released.push("host");cancel(undefined);}};}) as never;
const holdJob=(name:string)=>({name,root:name,json:{},native:false,pipeline:false,nativeRoot:name,selectedInputs:[],head:[],cacheKey:"0".repeat(24)});
const session=(root:string,extra:Record<string,unknown>={})=>({workspaceRoot:root,context:{registry:null},contracts:{},launch:null,projects:566,commandInputs:null,projectsByRoot:new Map(),componentRoots:[],cores:10,...extra});

test("while the workers start, the parent infers no project: every project waits for the pool and is adopted from the workers once it is ready",async()=>{
 const root=scratch(),children:ScriptedChild[]=[],log:string[]=[],lines:string[]=[];let open!:()=>void;const gate=new Promise<void>(done=>{open=done;});
 const farm=openProjectFarmV1({...session(root),environment:grantFile(root,"farm_hold_00001"),processed:()=>400,write:line=>lines.push(line),engage:true,
  spawnProcess:(()=>{const child=new GatedChild("held",log,gate);children.push(child);return child;}) as never,request:(()=>{let cancel!:(value:undefined)=>void;return{granted:new Promise<undefined>(done=>{cancel=done;}),release:()=>cancel(undefined)};}) as never})!;
 expect("parentWhileStarting" in GRAPH_FARM_POLICY_V1).toBe(false);
 expect(lines.filter(line=>line.includes("holds them for the workers")).length,"told once, and only once something is held").toBe(0);
 const held=Array.from({length:8},(_,index)=>{let settled=false;const promise=farm.prepare(holdJob("held"+index)).then(value=>{settled=true;return value;});return{promise,settled:()=>settled};});
 await Bun.sleep(300);
 expect(held.every(row=>!row.settled()),"eight projects wait for the pool, none is inferred by the parent").toBe(true);
 expect(lines.filter(line=>line.includes("holds them for the workers; a project that waits longer than 190 s says why")).length).toBe(1);
 const ready=new Promise<number>(done=>farm.onReady(done));open();await ready;
 expect(await Promise.all(held.map(row=>row.promise))).toEqual(Array(8).fill(true));
 expect(log.filter(entry=>entry==="held:run").length,"the pool took all of them").toBe(8);
 await farm.close();
},60000);

test("a pool that never starts keeps the projects in the queue with the reason shown for as long as the farm exists (no hold time ends it); the host's answer is waited for as long as it takes, never for the grant wait only: an owner a worker could run does not fall back to the parent",async()=>{
 const policy={...GRAPH_FARM_POLICY_V1,holdMilliseconds:300,grantWaitMilliseconds:200},root=scratch(),lines:string[]=[];
 const farm=openProjectFarmV1({...session(root),policy,environment:grantFile(root,"farm_hold_00002"),processed:()=>400,write:line=>lines.push(line),engage:true,
  spawnProcess:(()=>new SilentChild("never",[])) as never,request:pendingRequest()})!;
 let answer:boolean|undefined;const queued=farm.prepare(holdJob("c")).then(value=>{answer=value;});
 await Bun.sleep(policy.holdMilliseconds*3);
 expect(answer,"three hold times later the owner is still queued").toBeUndefined();
 expect(farm.exists()).toBe(true);expect(lines.some(line=>line.includes("waits in the queue")&&line.includes("Warteschlange"))).toBe(true);
 await farm.close();await closeProjectFarmsV1();await queued;
 expect(answer,"the farm is gone: the owner is released").toBe(false);
 const bare=scratch(),quiet=openProjectFarmV1({...session(bare),policy,environment:{LANG:"en_US.UTF-8",SEMIO_SLICE_LEAN:"off"},processed:()=>400,write:()=>undefined,engage:true,spawnProcess:(()=>{throw new Error("no worker may start");}) as never,request:(()=>{let cancel!:(value:undefined)=>void;setTimeout(()=>cancel(undefined),policy.holdMilliseconds*3);return{granted:new Promise<undefined>(done=>{cancel=done;}),release:()=>cancel(undefined)};}) as never})!;
 const waited=Date.now();expect(await quiet.prepare(holdJob("e"))).toBe(false);
 expect(Date.now()-waited,"the pending grant is waited for beyond the grant wait and the hold time").toBeGreaterThanOrEqual(policy.holdMilliseconds*3-50);
 await quiet.close();
},30000);

test("the worker pools belong to the session: the next construction of the session uses the workers that are running, a changed session retires them, and closing the farms ends the workers and returns the grants",async()=>{
 const root=scratch(),children:ScriptedChild[]=[],log:string[]=[],lines:string[]=[],released:string[]=[];
 const environment=grantFile(root,"farm_hub_000001"),make=(extra:Record<string,unknown>={})=>openProjectFarmV1({...session(root,extra),environment,processed:()=>400,write:line=>lines.push(line),engage:true,
  spawnProcess:(()=>{const child=new ScriptedChild(String(children.length),log);children.push(child);return child;}) as never,request:pendingRequest(released)})!;
 const first=make();
 const loops=await new Promise<number>(done=>first.onReady(done));
 expect(loops).toBe(GRAPH_FARM_POLICY_V1.loopsPerWorker);
 await first.prepare(holdJob("a"));await first.close();
 expect(children.length).toBe(1);expect(children[0]!.exitCode,"the worker keeps running when its construction ends").toBeNull();
 const second=make();
 expect(await new Promise<number>(done=>second.onReady(done)),"ready at once, nothing to start").toBe(loops);
 expect(lines.some(line=>line.includes("worker processes of the earlier waves are ready; the 566 projects of this wave go to them"))).toBe(true);
 await second.prepare(holdJob("b"));
 expect(children.length,"no second worker was started").toBe(1);expect(log).toEqual(["0:run","0:run"]);
 await second.close();
 const other=make({contracts:{changed:{outputs:["x"],inputs:["y"]}}});
 await Bun.sleep(30);
 expect(children[0]!.exitCode,"a changed session retires the pools of the old one").toBe(0);expect(children.length).toBe(2);
 await other.close();
 await closeProjectFarmsV1();
 expect(children[1]!.exitCode).toBe(0);
},30000);

test("a real worker starts from its environment while the event loop of the parent is blocked, so the parent never delays it",async()=>{
 const root=scratch(),opened=join(root,"opened.txt"),pool=farm({workers:2,session:{identity:"law",openedFile:opened}});
 const blocked=Date.now()+2500;while(Date.now()<blocked){}
 const stamps=readFileSync(opened,"utf8").trim().split("\n").map(Number);
 expect(stamps.length,"both workers opened their session during the block").toBe(2);
 for(const stamp of stamps)expect(stamp).toBeLessThan(blocked);
 expect(await pool.ready).toBe(true);
 const outcomes=await Promise.all([3,4,5,6].map(n=>pool.run({kind:"echo",n})));
 const expected=JSON.parse(Bun.spawnSync([python!,"-I","-c","import json\nprint(json.dumps([(n*n+7)%1013 for n in [3,4,5,6]]))"]).stdout.toString());
 expect(outcomes.map(outcome=>(outcome.reply as {value:number}).value)).toEqual(expected);
 await pool.close();
},60000);

test("a job whose construction is cancelled is settled as aborted, retires its worker without counting a lost process, and the pool goes on",async()=>{
 const pool=farm({workers:1}),controller=new AbortController();
 const first=pool.run({kind:"echo",n:1}),pid=((await first).reply as {pid:number}).pid;
 const pending=pool.run({kind:"hang",n:2},{signal:controller.signal});await Bun.sleep(100);
 controller.abort();
 expect((await pending).lost).toBe("aborted");
 const after=(await pool.run({kind:"echo",n:3})).reply as {pid:number;value:number};
 expect(after.value).toBe((3*3+7)%1013);expect(after.pid).not.toBe(pid);
 expect(pool.stats().restarts,"a cancelled job is no lost process").toBe(0);expect(pool.stats().lost).toBe(0);
 await pool.close();
},60000);

test("a worker that boots above its budget but does not grow is kept, one that grows past the ceiling over its first reply is replaced",async()=>{
 const pool=farm({workers:1,megabytes:1});
 for(let n=0;n<3;n++)expect(((await pool.run({kind:"echo",n})).reply as {n:number}).n).toBe(n);
 expect(pool.stats().recycled).toBe(0);
 await pool.close();
},60000);
