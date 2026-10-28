import{expect,test}from"bun:test";
import Ajv from"ajv";
import{join}from"node:path";
import{captureOwnedCommandV1}from"../../../🎛️owned-execution/🟦️.ts";
import{cargoTestPlanV1,type CargoTestPolicyV1}from"../../🦀️cargo/🟦️.ts";
import cargoFixture from"../../🦀️cargo/🧪️tests/🧫️fixtures/🔣️.json";
import{testPythonV1}from"../../🧰️toolchain/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import authored from"../🔣️.json";
import schema from"../🧬️schema/🔣️.json";
import corpus from"./🧫️fixtures/🔣️.json";
import{announceTestWorkersV1,describeTestWorkersV1,planTestWorkersV1,readTestWorkerOverrideV1,readTestWorkerPolicyV1,testWorkerFactsV1,testWorkersV1,type TestRunnerIdV1,type TestWorkerFactsV1,type TestWorkerPolicyV1}from"../🟦️.ts";

const ajv=new Ajv({strict:false,allErrors:true}),ids=schema.definitions.Protocol.const.runners as readonly TestRunnerIdV1[];
const compile=(name:string)=>ajv.compile({...(schema.definitions as Record<string,object>)[name]!,definitions:schema.definitions});
const policy=readTestWorkerPolicyV1(corpus.policy),mine=readTestWorkerPolicyV1(authored);

test("Ajv independently admits the authored policy, the corpus and its closed shape, and the in-repo validator agrees on every refusal",()=>{
 expect(compile("Policy")(authored)).toBe(true);expect(compile("Corpus")(corpus)).toBe(true);
 const refused:unknown[]=[{...authored,version:2},{...authored,extra:1},{...authored,hostShareBasisPoints:0},{...authored,runners:[]},{...authored,runners:[{...authored.runners[0],maximum:65}]},{...authored,runners:[{...authored.runners[0],id:"jest"}]},{...authored,runners:[{...authored.runners[0],override:"workers"}]},{...authored,runners:[{...authored.runners[0],oversubscription:0}]},{...authored,runners:[{...authored.runners[0],wanted:"all"}]}];
 for(const value of refused){expect(compile("Policy")(value),JSON.stringify(value).slice(0,80)).toBe(false);expect(validateJsonSchemaSubset(schema.definitions.Policy,value,schema).length,JSON.stringify(value).slice(0,80)).toBeGreaterThan(0);}
 expect(validateJsonSchemaSubset(schema.definitions.Policy,authored,schema)).toEqual([]);
 expect(()=>readTestWorkerPolicyV1({...authored,runners:[authored.runners[0],authored.runners[0]]})).toThrow("invalid");
 expect(()=>readTestWorkerPolicyV1({...authored,runners:[{...authored.runners[0],wanted:9,maximum:8}]})).toThrow("more than its maximum");
});

test("The authored policy declares every runner once, with its own override variable and sizes a worker can be priced with",()=>{
 expect(mine.runners.map(row=>row.id).sort()).toEqual([...ids].sort());
 expect(new Set(mine.runners.map(row=>row.override)).size).toBe(mine.runners.length);
 for(const row of mine.runners){expect(row.workerMegabytes).toBeGreaterThan(0);expect(row.maximum).toBeGreaterThanOrEqual(typeof row.wanted==="number"?row.wanted:1);}
});

test("Every stored vector of the workers a runner may start equals the Python model, bound included, and a bad override names its variable in the language of the environment",()=>{
 expect(corpus.cases.length).toBeGreaterThan(600);
 const reached=new Set<string>();
 for(const item of corpus.cases){
  const run=()=>planTestWorkersV1(policy,item.runner as TestRunnerIdV1,item.facts as TestWorkerFactsV1,{request:item.request,ratioBasisPoints:item.ratioBasisPoints,override:item.override},{});
  if(item.refuse!==undefined){expect(run,item.name).toThrow(policy.runners.find(row=>row.id===item.runner)!.override);continue;}
  const plan=run();expect({workers:plan.workers,bound:plan.bound as string},item.name).toEqual(item.expect);reached.add(plan.bound);
  expect(compile("Plan")(plan),item.name).toBe(true);
 }
 expect([...reached].sort()).toEqual(["cores","maximum","memory","override","request","wanted"]);
 const english=policy.runners[1]!;
 expect(()=>readTestWorkerOverrideV1(english,"0",{LANG:"en_US.UTF-8"})).toThrow(`${english.override} must be an integer from 1 to ${english.maximum}, not '0'`);
 expect(()=>readTestWorkerOverrideV1(english,"0",{LANG:"de_DE.UTF-8"})).toThrow(`${english.override} muss eine ganze Zahl von 1 bis ${english.maximum} sein, nicht '0'`);
});

test("The facts of a process equal the Python model: the share of one job of the grant it carries, else the host share",()=>{
 for(const item of corpus.facts){expect(testWorkerFactsV1(item.environment,policy,item.host),item.name).toEqual(item.expect as TestWorkerFactsV1);expect(compile("Facts")(item.expect),item.name).toBe(true);}
 expect(()=>testWorkerFactsV1({SEMIO_HOST_CAPACITY_PLAN:"{"},policy,{cores:2,totalMegabytes:4096})).toThrow();
 expect(()=>testWorkerFactsV1({SEMIO_HOST_CAPACITY_PLAN:JSON.stringify({version:1})},policy,{cores:2,totalMegabytes:4096})).toThrow();
});

test("Law: a plan never exceeds what was admitted, grows with memory and cores, shrinks with a learned larger size, and an override never leaves the admitted cores",()=>{
 for(const row of mine.runners){
  let forcedCounts:number[]=[];
  for(const cores of [1,2,4,10,64])for(const memoryMegabytes of [0,512,1536,2048,4096,8192,65536])for(const ratio of [3000,10000,20000]){
   const facts:TestWorkerFactsV1={source:"grant",cores,hostCores:10,memoryMegabytes},plan=planTestWorkersV1(mine,row.id,facts,{ratioBasisPoints:ratio});
   expect(plan.workers).toBeGreaterThanOrEqual(1);expect(plan.workers).toBeLessThanOrEqual(row.maximum);expect(plan.workers).toBeLessThanOrEqual(Math.max(1,cores*row.oversubscription));
   const base=Math.ceil(row.baseMegabytes*ratio/10000);
   if(plan.workers>1)expect(base+plan.workers*plan.workerMegabytes,`${row.id} ${cores} ${memoryMegabytes} ${ratio}`).toBeLessThanOrEqual(memoryMegabytes);
   expect(planTestWorkersV1(mine,row.id,{...facts,memoryMegabytes:memoryMegabytes*2},{ratioBasisPoints:ratio}).workers).toBeGreaterThanOrEqual(plan.workers);
   expect(planTestWorkersV1(mine,row.id,{...facts,cores:cores+1},{ratioBasisPoints:ratio}).workers).toBeGreaterThanOrEqual(plan.workers);
   if(ratio<20000)expect(planTestWorkersV1(mine,row.id,facts,{ratioBasisPoints:ratio+1000}).workers).toBeLessThanOrEqual(plan.workers);
   const forced=planTestWorkersV1(mine,row.id,facts,{override:String(row.maximum)});expect(forced.workers).toBeLessThanOrEqual(Math.max(1,cores*row.oversubscription));forcedCounts=[...forcedCounts,forced.workers];
  }
  expect(forcedCounts.some(count=>count>1)||row.maximum===1,`${row.id} can run more than one worker somewhere`).toBe(true);
 }
});

test("The 16 GB profile: a vitest, cargo and bun test run inside one job of a small grant starts the workers that job can carry, and a roomy host without a grant starts what each runner wants",()=>{
 const job={SEMIO_HOST_CAPACITY_PLAN:JSON.stringify({version:1,id:"k".repeat(22),jobs:1,cpu:1,memoryMegabytes:2048,fixedMegabytes:1536,floorMegabytes:2048})},host={cores:10,totalMegabytes:16384};
 const small=Object.fromEntries(ids.map(id=>[id,testWorkersV1(id,job,{},mine,host)])) as Record<TestRunnerIdV1,ReturnType<typeof testWorkersV1>>;
 expect(small["bun-test"].workers).toBe(1);expect(small["playwright"].workers).toBe(1);expect(small["e2e-stacks"].workers).toBe(1);expect(small["e2e-stacks"].bound).toBe("memory");
 expect(small["vitest"].workers).toBeLessThanOrEqual(4);expect(small["cargo-test"].workers).toBeLessThanOrEqual(1*mine.runners.find(row=>row.id==="cargo-test")!.oversubscription);
 const open=Object.fromEntries(ids.map(id=>[id,testWorkersV1(id,{},{},mine,host)])) as Record<TestRunnerIdV1,ReturnType<typeof testWorkersV1>>;
 expect(open["cargo-test"].workers).toBe(10);expect(open["vitest"].workers).toBe(4);expect(open["e2e-stacks"].workers).toBe(3);expect(open["playwright"].workers).toBe(4);expect(open["bun-test"].workers).toBe(1);
 expect(testWorkersV1("bun-test",{SEMIO_BUN_TEST_WORKERS:"3"},{},mine,host)).toMatchObject({workers:3,bound:"override"});
 expect(testWorkersV1("bun-test",{...job,SEMIO_BUN_TEST_WORKERS:"3"},{},mine,host)).toMatchObject({workers:1,bound:"cores"});
 expect(()=>testWorkersV1("vitest",{SEMIO_VITEST_WORKERS:"nine"},{},mine,host)).toThrow("SEMIO_VITEST_WORKERS");
 expect(testWorkersV1("bun-test",{},{request:4},mine,host)).toMatchObject({workers:4,bound:"request",wanted:4});
});

test("A reduced plan is worded in English and German and an unreduced plan is silent",()=>{
 const reduced=planTestWorkersV1(policy,"vitest",{source:"grant",cores:1,hostCores:10,memoryMegabytes:1100},{}),full=planTestWorkersV1(policy,"vitest",{source:"host",cores:10,hostCores:10,memoryMegabytes:65536},{});
 expect(reduced.workers).toBeLessThan(reduced.wanted);
 expect(describeTestWorkersV1(reduced,{LANG:"en_US.UTF-8"})).toBe(`[tests] Vitest: ${reduced.workers} of ${reduced.wanted} worker(s), bounded by the admitted memory (1100 MB and 1 core(s) of the admitted grant, about 151 MB per worker)`);
 expect(describeTestWorkersV1(reduced,{LANG:"de_DE.UTF-8"})).toBe(`[tests] Vitest: ${reduced.workers} von ${reduced.wanted} Worker(n), begrenzt durch den zugelassenen Speicher (1100 MB und 1 Kern(e) des zugelassenen Anteils, etwa 151 MB je Worker)`);
 expect(describeTestWorkersV1(full,{LANG:"en_US.UTF-8"})).toBeUndefined();
 const lines:string[]=[];announceTestWorkersV1(reduced,{LANG:"en_US.UTF-8"},line=>lines.push(line));announceTestWorkersV1(full,{LANG:"en_US.UTF-8"},line=>lines.push(line));
 expect(lines).toHaveLength(1);
 for(const language of ["en","de"] as const){const words=schema.definitions.Messages.const[language];expect(Object.keys(words.bounds).sort()).toEqual([...schema.definitions.Bound.enum].sort());expect(Object.keys(words.runners).sort()).toEqual([...ids].sort());}
});

test("Law: the Cargo test plan carries the planned test threads exactly once at every level, with and without Nextest and coverage, and a thread count the caller names wins",()=>{
 const base=cargoFixture.policies[0]!,shapes=[{nextest:true,coverage:false},{nextest:false,coverage:false},{nextest:true,coverage:true},{nextest:false,coverage:true}];
 let seen=0;
 for(const level of ["fundamental","quick","long","exhaustive"] as const)for(const shape of shapes){
  const policy={...base,level,nextest:shape.nextest,coverageEnabled:shape.coverage,coveragePath:shape.coverage?"/owner/coverage.lcov":null,assertionThreads:5} as unknown as CargoTestPolicyV1;
  const count=(args:readonly string[])=>args.filter(arg=>arg==="--test-threads"||arg.startsWith("--test-threads=")).length;
  const assertion=(extra:readonly string[])=>cargoTestPlanV1({manifestPath:policy.manifestPath,packages:["owner-package"],cwd:"/owner",extraArgs:extra},policy,"/owner/results/binaries.json").filter(step=>step.phase==="assert")[0]!.args;
  const planned=assertion(["--lib"]);
  expect(count(planned),`${level} ${JSON.stringify(shape)}`).toBe(1);
  expect(planned.some((arg,index)=>arg==="5"&&planned[index-1]==="--test-threads")||planned.includes("--test-threads=5"),`${level} ${JSON.stringify(shape)}`).toBe(true);
  if(shape.nextest){const named=assertion(["--lib","--test-threads","2"]);expect(count(named)).toBe(1);expect(named[named.indexOf("--test-threads")+1]).toBe("2");}
  else{const named=assertion(["--lib","--","--test-threads=2"]);expect(count(named)).toBe(1);expect(named).toContain("--test-threads=2");}
  seen++;
 }
 expect(seen).toBe(16);
});

test("The authored command configuration plans its test threads and passes the closed schema",async()=>{
 const configuration=JSON.parse(await Bun.file(join(import.meta.dir,"../../../../../⚙️configuration/🏃️process/🔣️.json")).text()),shape=JSON.parse(await Bun.file(join(import.meta.dir,"../../../🧭️routing/🎛️command/⚙️configuration/🧬️schema/🔣️.json")).text());
 expect(configuration.assertionThreads).toBe("planned");expect(new Ajv({strict:false}).compile(shape)(configuration)).toBe(true);
 expect(new Ajv({strict:false}).compile(shape)({...configuration,assertionThreads:"available-minus-quarter"})).toBe(false);
});

test("The stored vectors equal the live Python model",async()=>{
 const python=testPythonV1()??Bun.which("python3")??Bun.which("python");
 if(!python)throw Error("Independent Python oracle required");
 const answer=await captureOwnedCommandV1(python,[join(import.meta.dir,"🐍️oracle/🐍️.py"),"--check"],join(import.meta.dir,".."),{output:{maximumBytes:4096,maximumLines:8}});
 expect(answer).toContain("equal the model");
});
