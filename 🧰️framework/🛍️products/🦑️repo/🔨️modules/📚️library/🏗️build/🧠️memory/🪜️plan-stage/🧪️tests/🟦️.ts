import{expect,test}from"bun:test";
import Ajv from"ajv";
import{readFileSync}from"node:fs";
import{PLAN_STAGE_MESSAGES_V1,PLAN_STAGE_POLICY_V1,createPlanStageGuardV1,onPlanStageV1,planStageLimitBytesV1,renderPlanStageMessageV1}from"../🟨️.mjs";
import{throwIfOwnerExceededV1}from"../../🎯️owner-scope/🟨️.mjs";

/** 🪜️ Contract: the plan of a wave runs as one owner of the memory governor. Its stages are named and measured in the log in both languages, a plan that grows past what the machine gives a plan is stopped at its next yield point or stage boundary with the typed refusal that names the stage and the size, garbage a collection frees is no growth, a serving machine has no limit, and the declared numbers and the worker message agree with their JSON schema (checked with Ajv).
 * @see ../🧬️schema/🔣️.json */
const schema=JSON.parse(readFileSync(new URL("../🧬️schema/🔣️.json",import.meta.url),"utf8"));
const MB=1048576,wait=(milliseconds:number)=>new Promise<void>(resolve=>setTimeout(resolve,milliseconds));
const harness=(limitMegabytes:number)=>{
 const state={bytes:0,garbage:0,collections:0},lines:string[]=[],seconds=[0];
 const guard=createPlanStageGuardV1({limitBytes:limitMegabytes*MB,ports:{sample:()=>state.bytes,collect:()=>{state.collections++;state.bytes-=state.garbage;state.garbage=0;}},environment:{},report:line=>lines.push(line),now:()=>seconds[0]+=1500,resident:()=>state.bytes+3000*MB,stageMegabytes:{environment:300},usage:()=>({rss:state.bytes+3000*MB,heapTotal:state.bytes,heapUsed:state.bytes,external:0,arrayBuffers:0}),heap:()=>({heapSize:state.bytes,extraMemorySize:1048576,objectCount:7,objectTypeCounts:{string:5,Array:2}}),resources:()=>["FSReqPromise","FSReqPromise","Timeout"],refuse:detail=>Object.assign(new Error("plan too heavy "+JSON.stringify(detail)),{detail})});
 return{state,lines,guard};
};

test("the numbers: a share of the physical memory between the minimum and the maximum, none on a serving machine, the environment may name another number or switch it off",()=>{
 const P=PLAN_STAGE_POLICY_V1;
 expect(planStageLimitBytesV1({},16384)).toBe(Math.floor(16384*0.3)*MB);
 expect(planStageLimitBytesV1({},8192)).toBe(Math.floor(8192*0.3)*MB);
 expect(planStageLimitBytesV1({},4096)).toBe(P.minimumMegabytes*MB);
 expect(planStageLimitBytesV1({},23999)).toBe(Math.floor(23999*0.3)*MB);
 expect(planStageLimitBytesV1({},24000)).toBe(Infinity);
 expect(planStageLimitBytesV1({},0)).toBe(Infinity);
 expect(planStageLimitBytesV1({SEMIO_GRAPH_PLAN_LIMIT_MB:"off"},16384)).toBe(Infinity);
 expect(planStageLimitBytesV1({SEMIO_GRAPH_PLAN_LIMIT_MB:"3000"},16384)).toBe(3000*MB);
 expect(planStageLimitBytesV1({SEMIO_GRAPH_PLAN_LIMIT_MB:"nonsense"},16384)).toBe(Math.floor(16384*0.3)*MB);
});

test("declared numbers, messages and the worker message agree with the schema file; both languages have the same lines and placeholders",()=>{
 const ajv=new Ajv({strict:false}),policy={type:"object",required:["version","hostShare","treeShare","minimumMegabytes","maximumMegabytes","serveHostMegabytes","limitEnvironment","off","owner","stages","stageMegabytes","sampleMilliseconds","recentSamples"],additionalProperties:false,properties:{version:{const:1},hostShare:{type:"number",exclusiveMinimum:0,maximum:1},treeShare:{type:"number",exclusiveMinimum:0,maximum:1},minimumMegabytes:{type:"integer",minimum:1},maximumMegabytes:{type:"integer",minimum:1},serveHostMegabytes:{type:"integer",minimum:1},limitEnvironment:{type:"string",pattern:"^SEMIO_[A-Z_]+$"},off:{type:"string"},owner:{type:"string"},stages:{type:"array",minItems:1,uniqueItems:true,items:{type:"string",pattern:"^[a-z-]+$"}},stageMegabytes:{type:"object",additionalProperties:{type:"integer",minimum:1}},sampleMilliseconds:{type:"integer",minimum:10},recentSamples:{type:"integer",minimum:1}}};
 expect(ajv.validate(policy,schema.definitions.Policy.const),JSON.stringify(ajv.errors)).toBe(true);
 expect(PLAN_STAGE_POLICY_V1.minimumMegabytes).toBeLessThanOrEqual(PLAN_STAGE_POLICY_V1.maximumMegabytes);
 const messages={type:"object",required:["en","de"],additionalProperties:false,properties:{en:{type:"object",required:["started","finished","stopped"],additionalProperties:{type:"string",minLength:10}},de:{type:"object",required:["started","finished","stopped"],additionalProperties:{type:"string",minLength:10}}}};
 expect(ajv.validate(messages,PLAN_STAGE_MESSAGES_V1),JSON.stringify(ajv.errors)).toBe(true);
 const holes=(text:string)=>[...text.matchAll(/\{([a-z]+)\}/gu)].map(match=>match[1]).sort();
 for(const key of Object.keys(PLAN_STAGE_MESSAGES_V1.en) as ("started"|"finished"|"stopped")[])expect(holes(PLAN_STAGE_MESSAGES_V1.de[key])).toEqual(holes(PLAN_STAGE_MESSAGES_V1.en[key]));
 const event=ajv.compile(schema.definitions.StageEvent);
 expect(event({type:"stage",stage:"tests",operation:"fields",managedMegabytes:3,residentMegabytes:4})).toBe(true);
 expect(event({type:"stage",stage:"tests",managedMegabytes:3,residentMegabytes:4})).toBe(true);
 expect(event({type:"stage",stage:"",managedMegabytes:3,residentMegabytes:4})).toBe(false);
 expect(event({type:"stage",stage:"tests",managedMegabytes:3})).toBe(false);
});

test("stages are named, timed and measured in both languages, a stage event reaches the listeners and validates against the schema",async()=>{
 const{state,lines,guard}=harness(100000),events:unknown[]=[],off=onPlanStageV1(event=>events.push(event));
 try{
  await guard.run(async()=>{guard.mark("inference");state.bytes+=100*MB;await wait(1);guard.mark("tests");state.bytes+=50*MB;guard.mark("edges");});
 }finally{off();}
 expect(lines.filter(line=>line.startsWith("graph: plan stage"))).toEqual(["graph: plan stage inference starts (managed 0 MB, resident 3000 MB)","graph: plan stage inference took 1.5 s (managed 100 MB, 100 MB more than at its start, resident 3100 MB)","graph: plan stage tests starts (managed 100 MB, resident 3100 MB)","graph: plan stage tests took 1.5 s (managed 150 MB, 50 MB more than at its start, resident 3150 MB)","graph: plan stage edges starts (managed 150 MB, resident 3150 MB)","graph: plan stage edges took 1.5 s (managed 150 MB, 0 MB more than at its start, resident 3150 MB)"]);
 expect(lines.filter(line=>line.startsWith("Graph: Plan-Stufe"))).toHaveLength(6);
 const ajv=new Ajv({strict:false}),check=ajv.compile(schema.definitions.StageEvent);
 expect(events).toHaveLength(3);for(const event of events)expect(check(event),JSON.stringify(check.errors)).toBe(true);
 expect(events.map(event=>(event as{stage:string}).stage)).toEqual(["inference","tests","edges"]);expect(events.map(event=>(event as{operation:string}).operation)).toEqual(["inference","tests","edges"]);
});

test("a stage that grows without end is stopped at its next yield point, long before the host would run short; the refusal names the stage and the size, the log says so in both languages",async()=>{
 const{state,lines,guard}=harness(1000);let reached=0,error:any;
 try{
  await guard.run(async()=>{guard.mark("environment");guard.mark("tests");for(let step=0;step<500;step++){throwIfOwnerExceededV1();state.bytes+=100*MB;reached=state.bytes;await wait(10);}});
 }catch(reason){error=reason;}
 expect(error?.detail?.stage).toBe("tests");expect(error.detail.operation).toBe("tests");expect(error.detail.limit).toBe(1000);expect(error.detail.megabytes).toBeGreaterThan(1000);expect(reached).toBeLessThan(5000*MB);expect(state.collections).toBeGreaterThanOrEqual(1);
 expect(lines.filter(line=>line.includes("more than the 1000 MB this machine allows for it"))).toHaveLength(1);
 expect(lines.filter(line=>line.includes("die Plan-Stufe tests (während tests)"))).toHaveLength(1);
 expect(lines.at(-2)!).toContain("plan stage tests took");
});

test("growth that no yield point sees is caught at the next stage boundary, and the next boundary after a stop refuses at once",async()=>{
 const{state,guard}=harness(1000);let error:any,afterStop=false;
 try{
  await guard.run(async()=>{guard.mark("files");state.bytes+=2500*MB;guard.mark("edges");afterStop=true;});
 }catch(reason){error=reason;}
 expect(afterStop).toBe(false);expect(error.detail).toEqual({stage:"files",operation:"files",megabytes:2500,limit:1000});
});

test("garbage that a collection frees is no growth; a plan under its limit completes and its value is returned",async()=>{
 const{state,guard,lines}=harness(1000);
 const value=await guard.run(async()=>{guard.mark("metadata");for(let step=0;step<30;step++){throwIfOwnerExceededV1();state.bytes+=100*MB;state.garbage+=100*MB;await wait(5);}guard.mark("shape");return"done";});
 expect(value).toBe("done");expect(state.collections).toBeGreaterThanOrEqual(1);expect(lines.some(line=>line.includes("this machine allows for it"))).toBe(false);
});

test("without a limit nothing is stopped and a quiet serving machine says nothing unless phases are asked for; an error of the work itself passes unchanged; render fails for an unknown key",async()=>{
 const{state,guard,lines}=harness(Infinity);
 await guard.run(async()=>{guard.mark("tests");state.bytes+=90000*MB;guard.mark("edges");});
 expect(lines).toEqual([]);
 const loud:string[]=[],phases=createPlanStageGuardV1({limitBytes:Infinity,ports:{sample:()=>0,collect:()=>undefined},environment:{SEMIO_GRAPH_PHASES:"1"},report:line=>loud.push(line)});
 await phases.run(async()=>{phases.mark("tests");phases.mark("edges");});
 expect(loud.filter(line=>line.startsWith("graph: plan stage"))).toHaveLength(4);
 const failing=harness(1000);
 const failure=await failing.guard.run(async()=>{failing.guard.mark("tests");throw new TypeError("own failure");}).then(()=>undefined,reason=>reason);
 expect(failure).toBeInstanceOf(TypeError);expect(failing.lines.at(-2)!).toContain("plan stage tests took");
 expect(()=>renderPlanStageMessageV1("unknown" as never,{},{})).toThrow("Closed plan stage message required");
});

test("real memory: a plan that holds 32 MB buffers is stopped by the real counters long before a gigabyte, in the stage that allocated",async()=>{
 const held:ArrayBuffer[]=[],lines:string[]=[];let error:any;
 const guard=createPlanStageGuardV1({limitBytes:192*MB,environment:{},report:line=>lines.push(line),refuse:detail=>Object.assign(new Error("plan too heavy"),{detail})});
 try{
  await guard.run(async()=>{guard.mark("edges");for(let step=0;step<64;step++){throwIfOwnerExceededV1();held.push(new ArrayBuffer(32*MB));new Uint8Array(held.at(-1)!).fill(1);await wait(20);}});
 }catch(reason){error=reason;}
 expect(error?.detail?.stage).toBe("edges");expect(error.detail.megabytes).toBeGreaterThan(192);expect(held.length).toBeLessThan(32);expect(error.detail.limit).toBe(192);
 expect(lines.some(line=>line.startsWith("graph: plan stage edges (during edges) grew the plan by"))).toBe(true);
 held.length=0;
});

test("the environment has a budget of its own: an operation that grows past it is stopped at the next operation call, the refusal names the operation, and the detail line says what grew",async()=>{
 const{state,lines,guard}=harness(100000);let error:any,reached=0;
 try{
  await guard.run(async()=>{guard.mark("environment");for(let batch=0;batch<200;batch++){guard.operation(batch<3?"fields":"manifests");state.bytes+=40*MB;reached=state.bytes;await wait(2);}});
 }catch(reason){error=reason;}
 expect(error?.detail).toMatchObject({stage:"environment",operation:"manifests",limit:300});expect(error.detail.megabytes).toBeGreaterThan(300);expect(reached).toBeLessThan(1000*MB);
 expect(lines.filter(line=>line.includes("plan stage environment (during manifests) grew the plan by"))).toHaveLength(1);
 expect(lines.filter(line=>line.includes("Plan-Stufe environment (während manifests)"))).toHaveLength(1);
 const detailLine=lines.find(line=>line.startsWith("graph: plan stop detail "))!,detail=JSON.parse(detailLine.slice("graph: plan stop detail ".length));
 const ajv=new Ajv({strict:false}),check=ajv.compile(schema.definitions.StopDetail);
 expect(check(detail),JSON.stringify(check.errors)).toBe(true);
 expect(detail).toMatchObject({stage:"environment",operation:"manifests",heap:{objectTypes:[{type:"string",count:5},{type:"Array",count:2}]},resources:{FSReqPromise:2,Timeout:1}});
 expect(detail.memory.heapUsed).toBeGreaterThan(300);
});

test("the sampler flags growth that only the timer sees (no operation call, no yield point) and the detail line keeps the recent samples",async()=>{
 const{state,lines,guard}=harness(500);let error:any,spins=0;
 try{
  await guard.run(async()=>{guard.mark("tests");await wait(700);const until=performance.now()+3000;while(performance.now()<until){await wait(20);state.bytes+=60*MB;spins++;if(state.bytes>8000*MB)break;}});
 }catch(reason){error=reason;}
 expect(error).toBeUndefined();
 expect(spins).toBeGreaterThan(0);
 const detailLine=lines.find(line=>line.startsWith("graph: plan stop detail "));expect(detailLine).toBeDefined();
 expect(JSON.parse(detailLine!.slice("graph: plan stop detail ".length)).recent.length).toBeGreaterThan(0);
});

test("a stage without a budget is only held to the plan limit, and a stage budget is not enforced on a machine without limit",async()=>{
 const{state,guard}=harness(100000);
 await guard.run(async()=>{guard.mark("tests");guard.operation("x");state.bytes+=5000*MB;guard.mark("edges");});
 const free=createPlanStageGuardV1({limitBytes:Infinity,ports:{sample:()=>9e12,collect:()=>undefined},environment:{},report:()=>undefined,stageMegabytes:{environment:1}});
 await free.run(async()=>{free.mark("environment");free.operation("fields");free.mark("tests");});
});
