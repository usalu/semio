/** 🤝️ Contract: the graph child that plans for a loader names the generation it served in a sidecar of its plan file, the loader that reads the plan takes it over, and the runtime hands it to the commands its descendants start only after the graph is pinned; without a served generation there is no sidecar and no pin. */
import{expect,test}from"bun:test";
import{existsSync,mkdtempSync,readFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";
import{BUILD_GRAPH_PLAN_PIN_SUFFIX_V1,writeBuildGraphPlanV1}from"../../../../../🧭️invocation/🕸️graph/🟦️.ts";
import{recordSealedGenerationPinV1,sealedGenerationPinV1}from"../../../../🟦️.ts";

const source=(relative:string)=>readFileSync(fileURLToPath(new URL(relative,import.meta.url)),"utf8");

test("the plan file of a child that served a generation carries its pin, one without carries none",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-plan-pin-"));
 try{
  recordSealedGenerationPinV1(undefined);await writeBuildGraphPlanV1(join(directory,"none.json"),{version:1} as never);
  expect(existsSync(join(directory,"none.json"))).toBe(true);expect(existsSync(join(directory,"none.json"+BUILD_GRAPH_PLAN_PIN_SUFFIX_V1))).toBe(false);
  recordSealedGenerationPinV1('{"pin":1}');await writeBuildGraphPlanV1(join(directory,"some.json"),{version:1} as never);
  expect(readFileSync(join(directory,"some.json"+BUILD_GRAPH_PLAN_PIN_SUFFIX_V1),"utf8")).toBe('{"pin":1}');
  expect(sealedGenerationPinV1()).toBe('{"pin":1}');
 }finally{recordSealedGenerationPinV1(undefined);rmSync(directory,{recursive:true,force:true});}
});

test("only the build tools that issue every task environment export the pin; the others keep the graph of their nested commands",()=>{
 try{
  recordSealedGenerationPinV1('{"pin":2}');
  expect(sealedGenerationPinV1("bazel")).toBe('{"pin":2}');expect(sealedGenerationPinV1("semio")).toBe('{"pin":2}');expect(sealedGenerationPinV1()).toBe('{"pin":2}');
  expect(sealedGenerationPinV1("nx")).toBeUndefined();expect(sealedGenerationPinV1("cargo")).toBeUndefined();
 }finally{recordSealedGenerationPinV1(undefined);}
});

test("the loader takes the pin of the plan it reads and forgets the one of an earlier load",()=>{
 const text=source("../../../../../🧭️invocation/🕸️graph/🟦️.ts");
 expect(text).toContain('recordSealedGenerationPinV1(existsSync(file+BUILD_GRAPH_PLAN_PIN_SUFFIX_V1)?readFileSync(file+BUILD_GRAPH_PLAN_PIN_SUFFIX_V1,"utf8"):undefined);');
});

test("the runtime exports the pin only after the graph is pinned, restores it with the command environment and never overwrites it without a served generation",()=>{
 const text=source("../../../../../⌨️command/⚙️runtime/🟦️.ts"),pinned=text.indexOf('process.env.SEMIO_BUILD_REUSE_GRAPH="true";'),exported=text.indexOf("const generationPin=command===\"watch\"?undefined:sealedGenerationPinV1(selected.tool);"),backend=text.indexOf("loadBuildBackendV1(selected.tool)).execute(");
 expect(pinned).toBeGreaterThan(-1);expect(pinned).toBeLessThan(exported);expect(exported).toBeLessThan(backend);
 expect(text.slice(exported,backend)).toContain("if(generationPin!==undefined){previous.SEMIO_BUILD_GENERATION_PIN=process.env.SEMIO_BUILD_GENERATION_PIN;process.env.SEMIO_BUILD_GENERATION_PIN=generationPin;}");
});

test("a watch re-plans for every change: its children never get a pin",()=>{
 expect(source("../../../../../⌨️command/⚙️runtime/🟦️.ts")).toContain('const generationPin=command==="watch"?undefined:sealedGenerationPinV1(selected.tool);');
});

test("the graph module binds only a pin of its Original and reads both generation paths through it",()=>{
 const text=source("../../../../🟦️.ts");
 expect(text.match(/pin:boundGenerationPinV1\(\)/gu)).toHaveLength(2);
 expect(text).toContain('const state=generationPinStateV1(process.env);return state.state==="bound"?state.pin:undefined;');
});
