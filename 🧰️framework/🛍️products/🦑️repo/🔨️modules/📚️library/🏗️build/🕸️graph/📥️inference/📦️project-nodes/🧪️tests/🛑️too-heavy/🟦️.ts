/** 🛑️ Contract: the project inference of a graph read stops ONE project that outgrows what this machine can give a project, records every owner under its own name, and ends the read with the refusal that names the stopped projects; a serving machine enforces nothing but still records. */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{readFileSync}from"node:fs";
import ts from"typescript";
import{graphOwnerEnforcementV1}from"../../🟨️.mjs";
import{yieldInferenceV1}from"../../../🧭️script-inputs/🟨️.mjs";
import{GraphOwnerTooHeavyErrorV1,createOwnerScopeV1,runInOwnerScopeV1}from"../../../../../🧠️memory/🎯️owner-scope/🟨️.mjs";
import{recordOwnerCostFileV1,readOwnerCostFileV1}from"../../../../../🪪️graph-state/🙋️wanted/💰️cost/🟦️.ts";

const source=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8"),tree=ts.createSourceFile("project-nodes.mjs",source,ts.ScriptTarget.Latest,true,ts.ScriptKind.JS);
const calls=()=>{const found:{text:string;callee:string}[]=[];const visit=(node:ts.Node)=>{if(ts.isCallExpression(node))found.push({text:node.getText(tree),callee:node.expression.getText(tree)});ts.forEachChild(node,visit);};visit(tree);return found;};
const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const MB=1048576;
const withTotal=async<T>(megabytes:number|undefined,work:()=>Promise<T>)=>{const before=process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;if(megabytes===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=String(megabytes);try{return await work();}finally{if(before===undefined)delete process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES;else process.env.SEMIO_HOST_CAPACITY_TOTAL_MEGABYTES=before;}};

test("the governor of the inference is told the limit and the recorder, every owner runs in its own scope under its name, and a stopped owner ends the read with the refusal after the phase",()=>{
 const governor=calls().find(call=>call.callee==="createGraphParentMemoryGovernorV1")!;
 expect(governor.text).toContain("ownerLimitBytes:enforcement.limitBytes");expect(governor.text).toContain("costs:enforcement.recorder");
 expect(source).toMatch(/governor\.run\(async\(\{away,scope\}\)=>/u);expect(source).toContain("runInOwnerScopeV1(scope,");expect(source).toContain("key:job.name");
 expect(source).toContain("catch(error){if(ownerTooHeavyOfV1(error)===undefined)throw error;}");
 expect(source.indexOf("enforcement.recorder?.flush()")).toBeGreaterThan(source.indexOf("graphWaveMeterV1.parent("));
 const refusal=source.indexOf('refuseBuildV1("unserved","graph.owner-too-heavy"');
 expect(refusal).toBeGreaterThan(source.indexOf('phases?.end("inference"'));expect(refusal).toBeLessThan(source.indexOf("if (collector) await collector.finish"));
});

test("the yield points of the inference stop an owner the governor marked, before any other work",()=>{
 const inputs=readFileSync(new URL("../../../🧭️script-inputs/🟨️.mjs",import.meta.url),"utf8");
 expect(inputs).toMatch(/export async function yieldInferenceV1\(context,phase,index,force=false\)\{\n throwIfOwnerExceededV1\(\);\n if\(!context\.invocation\)return;/u);
});

test("a 16 GB machine may give one project 1756 managed MB, an 8 GB machine its parent budget, a serving machine no limit; the recorder is there in every case",async()=>{
 const root=mkdtempSync(join(tmpdir(),"too-heavy-"));scratch.push(root);
 const sixteen=await withTotal(16384,()=>graphOwnerEnforcementV1(root,process.env,1024*MB,()=>7,undefined,()=>16384));
 expect(sixteen.limitBytes).toBe(1756*MB);expect(sixteen.recorder).toBeDefined();
 expect((await withTotal(8192,()=>graphOwnerEnforcementV1(root,process.env,1024*MB,undefined,undefined,()=>8192))).limitBytes).toBe(1024*MB);
 const serving=await withTotal(32768,()=>graphOwnerEnforcementV1(root,process.env,1536*MB,undefined,undefined,()=>32768));
 expect(serving.limitBytes).toBeUndefined();expect(serving.recorder).toBeDefined();
 expect(await withTotal(16384,()=>graphOwnerEnforcementV1(root,{SEMIO_GRAPH_OWNER_LIMIT:"off"},1024*MB))).toEqual({});
 sixteen.recorder!.record({key:"@semio-tech/s-dev-composition",kind:"project",takenBytes:2500*MB,seconds:9,stopped:true});
 expect(readOwnerCostFileV1(join(root,".cache/f/graph-costs/owner-costs.json")).owners["@semio-tech/s-dev-composition"]).toMatchObject({kind:"project",rows:7,samples:[{megabytes:2500,stopped:true}]});
});

test("the real yield point of the inference throws inside the scope of a stopped owner at once and stays silent for the owner beside it and outside any scope",async()=>{
 const heavy=createOwnerScopeV1("heavy"),light=createOwnerScopeV1("light");heavy.exceed({takenBytes:3*MB,limitBytes:2*MB});
 await expect(runInOwnerScopeV1(heavy,()=>yieldInferenceV1({},"build:project-assembly",0))).rejects.toBeInstanceOf(GraphOwnerTooHeavyErrorV1);
 await expect(runInOwnerScopeV1(light,()=>yieldInferenceV1({},"build:project-assembly",0))).resolves.toBeUndefined();
 await expect(yieldInferenceV1({},"build:project-assembly",0)).resolves.toBeUndefined();
 const both=await Promise.allSettled([runInOwnerScopeV1(heavy,async()=>{await new Promise(resolve=>setTimeout(resolve,5));await yieldInferenceV1({},"x",1);}),runInOwnerScopeV1(light,async()=>{await new Promise(resolve=>setTimeout(resolve,5));await yieldInferenceV1({},"x",1);})]);
 expect(both.map(row=>row.status)).toEqual(["rejected","fulfilled"]);
});

test("a limited machine reads alone: the owners whose recorded cost passes the share are named heavy, they get the solo grant of the memory available when they start (never less than the share), a serving machine has no heavy owners",async()=>{
 const root=mkdtempSync(join(tmpdir(),"solo-"));scratch.push(root);
 recordOwnerCostFileV1(join(root,".cache/f/graph-costs/owner-costs.json"),{name:"@semio-tech/framework-os-kernel",kind:"project",megabytes:3492,seconds:90,rows:10,stopped:false,atMs:Date.now()});
 let available=12000*MB;
 const sixteen=await withTotal(16384,()=>graphOwnerEnforcementV1(root,process.env,1024*MB,undefined,()=>available,()=>16384));
 expect(sixteen.heavy!(["@semio-tech/light","@semio-tech/framework-os-kernel","@semio-tech/other"])).toEqual(["@semio-tech/framework-os-kernel"]);
 expect(sixteen.heavy!(["@semio-tech/light"])).toEqual([]);
 expect(sixteen.soloLimitBytes!()).toBe(Math.floor((12000-2048)/2/1.25)*MB);
 available=7599*MB;expect(sixteen.soloLimitBytes!()).toBe(Math.floor((7599-2048)/2/1.25)*MB);
 available=1000*MB;expect(sixteen.soloLimitBytes!()).toBe(1756*MB);
 available=40000*MB;expect(sixteen.soloLimitBytes!()).toBe(Math.floor((16384-2048-3800)/2/1.25)*MB);
 const small=await withTotal(8192,()=>graphOwnerEnforcementV1(root,process.env,1024*MB,undefined,()=>available,()=>8192));
 expect(small.soloLimitBytes!()).toBe(1024*MB);
});

test("the inference runs the heavy owners alone after the light ones: sorted last, one at a time with the solo limit of the moment, a light owner stopped at the share is run again alone when the solo limit is larger, and only a stop at the solo limit ends the read with the refusal",()=>{
 expect(source).toContain("enforcement.heavy?.(jobs.map(job=>job.name))");
 expect(source).toContain("...solo?{limitBytes:enforcement.soloLimitBytes()}:{}");
 expect(source).toContain("enforcement.soloLimitBytes()>scope.exceeded.limitBytes");
 expect(source.indexOf("await awaitGrowingV1(workers);if(!stopped&&cursor<jobs.length){memoryPorts.collect();await loop(Infinity);}")).toBeGreaterThan(source.indexOf("const workers=Array.from({length:limit},()=>loop(lightEnd))"));
 expect(source).toContain("cursor<Math.min(end,jobs.length)");
});
