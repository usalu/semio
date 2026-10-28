/** 🧊️ Contract: a graph construction that reads the real fact index of the newest generation peaks above a gigabyte, and the process that waits for its slice grows by a fraction of that. The planner of a loader therefore runs as a child that exits; the loader reads only the file it wrote. Skipped when the workspace has no generation yet. Independent measurement: the resident set of both processes read with `ps`. */
import{expect,test}from"bun:test";
import{existsSync,mkdtempSync,readFileSync,rmSync,statSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{fileURLToPath}from"node:url";

const workspace=fileURLToPath(new URL("../../../../../../../../../",import.meta.url)).replace(/\/$/u,""),store=join(workspace,".cache/b/g/m/generations"),generations=new URL("../../../🕸️graph/🕰️generations/🟨️.mjs",import.meta.url).href;
const child=fileURLToPath(new URL("./🟨️.mjs",import.meta.url)),megabytes=(pid:number)=>Number(Bun.spawnSync(["ps","-o","rss=","-p",String(pid)]).stdout.toString().trim()||0)/1024;

test.skipIf(!existsSync(join(store,"current.json")))("the child that plans from the real generation peaks high, the waiting loader grows by less than a third of the child peak",async()=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-isolated-plan-")),file=join(directory,"graph.json");
 try{
  const before=megabytes(process.pid),process_=Bun.spawn([process.execPath,child,generations,store,file,"40"],{stdout:"ignore",stderr:"pipe"});
  let peak=0;const timer=setInterval(()=>{peak=Math.max(peak,megabytes(process_.pid));},100);
  const code=await process_.exited;clearInterval(timer);
  expect(code,await new Response(process_.stderr).text()).toBe(0);
  const slice=JSON.parse(readFileSync(file,"utf8"));
  expect(Object.keys(slice.projects).length).toBeGreaterThanOrEqual(40);
  Bun.gc(true);
  const loader=megabytes(process.pid)-before;
  console.log("MEASURED child peak MB",Math.round(peak),"loader growth MB",Math.round(loader),"slice owners",Object.keys(slice.projects).length,"file MB",Math.round(statSync(file).size/1048576));
  expect(peak,"child peak").toBeGreaterThan(900);
  expect(loader,"loader growth after reading the slice").toBeLessThan(800);
  expect(loader).toBeLessThan(peak/3);
  expect(statSync(file).size).toBeGreaterThan(1000);
 }finally{rmSync(directory,{recursive:true,force:true});}
},120000);

const invocation=await import("../../🕸️graph/🟦️.ts") as typeof import("../../🕸️graph/🟦️.ts");

test("a loader plans in a child unless it reads a pinned graph, is itself the child or the switch is off",()=>{
 expect(invocation.buildGraphPlanInChildV1({},false)).toBe(true);
 expect(invocation.buildGraphPlanInChildV1({},true)).toBe(false);
 expect(invocation.buildGraphPlanInChildV1({[invocation.BUILD_GRAPH_PLAN_OUT_V1]:"/tmp/plan.json"},false)).toBe(false);
 expect(invocation.buildGraphPlanInChildV1({SEMIO_BUILD_GRAPH_ISOLATE:"off"},false)).toBe(false);
 expect(invocation.buildGraphPlanInChildV1({SEMIO_BUILD_GRAPH_ISOLATE:"on"},false)).toBe(true);
});

test("the runtime child writes its plan and returns before the graph is pinned or any backend runs; the loader reads the plan instead of loading",()=>{
 const text=readFileSync(fileURLToPath(new URL("../../../⌨️command/⚙️runtime/🟦️.ts",import.meta.url)),"utf8");
 const planned=text.indexOf("if(planFile!==undefined){"),loaded=text.indexOf("const baseline=await profilePhaseV1(\"graph.load\""),pinned=text.indexOf("withRepositoryBuildGraphArtifactV1(root,graph");
 expect(planned).toBeGreaterThan(-1);expect(planned).toBeLessThan(loaded);expect(loaded).toBeLessThan(pinned);
 expect(text.slice(planned,loaded)).toContain("writeBuildGraphPlanV1(planFile,await loadHere(),phaseControl.signal));return;}");
 expect(text.slice(loaded,pinned)).toContain("buildGraphPlanInChildV1(process.env,process.env.SEMIO_BUILD_REUSE_GRAPH===\"true\")?loadBuildGraphInChildV1(root,args,invocation,phaseControl):loadHere()");
});

test("the child heap is the declared graph child size, never the capacity grant of the run",async()=>{
 const text=readFileSync(fileURLToPath(new URL("../../🕸️graph/🟦️.ts",import.meta.url)),"utf8"),start=text.indexOf("export async function loadBuildGraphInChildV1("),body=text.slice(start,text.indexOf("\n}\n",start));
 expect(start).toBeGreaterThan(-1);
 expect(body).toContain("BUN_JSC_forceRAMSize:String(graphChildHeapBytesV1())");
 expect(body.match(/BUN_JSC_forceRAMSize/gu)?.length).toBe(1);
 expect("buildGraphChildHeapBytesV1" in invocation).toBe(false);
 const memory=await import("../../../🧠️memory/🟨️.mjs");
 expect(memory.graphChildHeapBytesV1()).toBe(memory.GRAPH_MEMORY_POLICY_V1.childHeapMegabytes*1048576);
});
