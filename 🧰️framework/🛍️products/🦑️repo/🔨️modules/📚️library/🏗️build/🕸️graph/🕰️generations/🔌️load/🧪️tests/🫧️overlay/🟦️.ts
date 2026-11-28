/** 🫧️ Contract: a narrow read whose stale owners are all it needs never queues behind a whole read. While a fake whole read holds the builder custody, the narrow read refreshes its own stale owners privately on top of the sealed generation, publishes nothing, answers the graph an independent Python model computes, and ends before the whole read does; without a whole read it publishes as before, and a failing private refresh falls back to the queue. */
import {expect,test} from "bun:test";
import {mkdtempSync,readFileSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {canonicalJsonV1} from "../../../🟨️.mjs";
import {loadGenerationGraphV1} from "../../🟨️.mjs";
import {readGenerationPointerV1} from "../../../🟨️.mjs";
import {GENERATION_POLICY_V1} from "../../../🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"../../../🧪️tests/🐍️.py"),python=Bun.which("python3")??Bun.which("python");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-overlay-")),clean=(dir:string)=>rmSync(dir,{recursive:true,force:true});
const normalized=(value:unknown)=>JSON.parse(canonicalJsonV1(value));
const shape=(graph:any)=>normalized({projects:graph.projects,dependencies:graph.dependencies,externalNodes:graph.externalNodes,unavailable:graph.unavailable??[]});
const pythonGraph=(dir:string,names?:string[])=>{const graph=JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());if(!names)return graph;return{...graph,projects:Object.fromEntries(Object.entries(graph.projects).filter(([name])=>names.includes(name))),dependencies:graph.dependencies.filter((edge:any)=>names.includes(edge.source)&&names.includes(edge.target))};};
const generationOf=(dir:string)=>readGenerationPointerV1(readFileSync(join(dir,".gen",GENERATION_POLICY_V1.pointer))).generation;
const chainFiles=()=>{
 const files:Record<string,string>={};
 for(const[name,imports]of[["a",["@syn/b"]],["b",["@syn/c"]],["c",[]],["d",["@syn/e"]],["e",[]]] as [string,string[]][])files[`pkg/${name}/project.json`]=JSON.stringify({name,package:`@syn/${name}`,uses:[]}),files[`pkg/${name}/src/index.ts`]=imports.map(spec=>`import "${spec}";`).join("\n")+"\nexport const v=1;\n";
 return files;
};
const overlayOf=(mock:ReturnType<typeof createMockHost>,options:{fail?:boolean}={})=>{
 const host=mock.host as Record<string,unknown>;
 host.wholeBusy=async()=>true;
 host.withOverlay=async(body:(store:unknown,control:unknown)=>Promise<unknown>)=>{mock.events.push("overlay");if(options.fail)throw new Error("private refresh refused");return body({root:join(mock.host.workspaceRoot,".gen")},{published:mock.published});};
};

test("a narrow read is served while a whole read holds the custody: no queue, no publication, the Python graph",async()=>{
 const dir=scratch();
 try{
  await seed(dir,chainFiles());
  const custody={tail:Promise.resolve()},first=createMockHost(dir,{custody});(await loadGenerationGraphV1(first.host)).graph;
  expect(generationOf(dir)).toBe(0);
  await applyOps(dir,[{op:"write",path:"pkg/c/src/index.ts",content:"export const v=3;\n"},{op:"write",path:"pkg/e/src/index.ts",content:"export const v=5;\n"}]);
  let open!:()=>void;const gate=new Promise<void>(resolve=>{open=resolve;}),whole=createMockHost(dir,{custody,gate:()=>gate}),wholeRead=loadGenerationGraphV1(whole.host);
  while(!whole.events.includes("builder:held"))await new Promise(resolve=>setTimeout(resolve,5));
  const narrow=createMockHost(dir,{custody});overlayOf(narrow);
  const started=Date.now(),answer=await Promise.race([loadGenerationGraphV1(narrow.host,{closure:["a"]}),new Promise(resolve=>setTimeout(()=>resolve("timeout"),20000))]) as any;
  expect(answer).not.toBe("timeout");expect(Date.now()-started).toBeLessThan(20000);
  expect(narrow.events).toEqual(["shared","overlay"]);
  expect(narrow.log.map(row=>row.full)).toEqual([["c"]]);
  expect(shape(answer.graph)).toEqual(pythonGraph(dir,["a","b","c"]));
  expect(generationOf(dir)).toBe(0);
  expect(narrow.said.map(row=>row.key)).toContain("overlay");
  open();await wholeRead;
  expect(generationOf(dir)).toBe(1);
 }finally{clean(dir);}
},60000);

test("the same read without a whole read in the way builds under the custody and publishes",async()=>{
 const dir=scratch();
 try{
  await seed(dir,chainFiles());
  const first=createMockHost(dir);(await loadGenerationGraphV1(first.host)).graph;
  await applyOps(dir,[{op:"write",path:"pkg/c/src/index.ts",content:"export const v=3;\n"}]);
  const mock=createMockHost(dir);(mock.host as Record<string,unknown>).wholeBusy=async()=>false;(mock.host as Record<string,unknown>).withOverlay=async()=>{throw new Error("no overlay without a whole read");};
  const answer=await loadGenerationGraphV1(mock.host,{closure:["a"]});
  expect(mock.events).toContain("builder:held");expect(generationOf(dir)).toBe(1);
  expect(shape(answer.graph)).toEqual(pythonGraph(dir,["a","b","c"]));
 }finally{clean(dir);}
},60000);

test("a private refresh that fails is told and the read queues as before; a structural change never goes private",async()=>{
 const dir=scratch();
 try{
  await seed(dir,chainFiles());
  const first=createMockHost(dir);(await loadGenerationGraphV1(first.host)).graph;
  await applyOps(dir,[{op:"write",path:"pkg/c/src/index.ts",content:"export const v=3;\n"}]);
  const failing=createMockHost(dir);overlayOf(failing,{fail:true});
  const answer=await loadGenerationGraphV1(failing.host,{closure:["a"]});
  expect(failing.events).toEqual(["shared","overlay","builder:wait","builder:held","builder:released"]);
  expect(failing.said.map(row=>row.key)).toContain("overlayFailed");
  expect(shape(answer.graph)).toEqual(pythonGraph(dir,["a","b","c"]));
  await applyOps(dir,[{op:"write",path:"pkg/f/project.json",content:JSON.stringify({name:"f",package:"@syn/f",uses:[]})},{op:"write",path:"pkg/f/src/index.ts",content:'import "@syn/c";\nexport const v=1;\n'}]);
  const structural=createMockHost(dir);overlayOf(structural);
  await loadGenerationGraphV1(structural.host,{closure:["a"]});
  expect(structural.events).not.toContain("overlay");
 }finally{clean(dir);}
},60000);
