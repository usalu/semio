/** 🔒️ Contract: a question about every project under the graph policy `current` is answered from the sealed generation of the whole workspace with its age stated, and never starts, queues for or joins a build; without a sealed generation it refuses with a typed bilingual text that names how far the whole read of this machine is. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import ts from "typescript";
import {writeJsonStreamV1} from "../../../🌊️json-stream/🟨️.mjs";
import {boundGenerationPinV1} from "../../../../📍️pin/🟦️.ts";
import {buildInvocationGraphSelectionV1,readBuildOwnerRegistryV1} from "../../../../🚪️admission/🟦️.ts";
import {selectLiteralBuildProjectsV1} from "../../../../../🧭️selection/📥️options/🟦️.ts";
import {canonicalJsonV1} from "../../../🟨️.mjs";
import {extractGraphPolicyV1,generationAgeTextV1,loadGenerationGraphV1,renderGenerationMessageV1,renderWholeReadProgressV1} from "../../🟨️.mjs";
import {applyOps,createMockHost,ringFiles,seed} from "../../../🧪️tests/🏃️world/🟦️.ts";
import {reportBoundaryErrorV1} from "../../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/📣️progress/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"../../../🧪️tests/🐍️.py"),python=Bun.which("python3")??Bun.which("python");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-current-never-builds-")),clean=(dir:string)=>rmSync(dir,{recursive:true,force:true});
const normalized=(value:unknown)=>JSON.parse(canonicalJsonV1(value));
const shape=(graph:any)=>normalized({projects:graph.projects,dependencies:graph.dependencies,externalNodes:graph.externalNodes,unavailable:graph.unavailable??[]});
const pythonGraph=(dir:string)=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());

test("without a sealed generation the policy current refuses with the typed text, builds nothing and names the progress of the whole read",async()=>{
 for(const language of ["en","de"] as const){
  const dir=scratch();
  try{
   await seed(dir,ringFiles(6));
   const mock=createMockHost(dir,{language}),host=mock.host as Record<string,unknown>;
   host.wholeReadProgress=async()=>renderWholeReadProgressV1({phase:"owners",done:89,total:4785,heldSeconds:1200},language);
   const failure=await loadGenerationGraphV1(mock.host,{policy:"current"}).then(()=>undefined,error=>error as Error&{code?:string});
   expect(failure?.code).toBe("GENERATION_NONE_SEALED");
   expect(failure!.message).toContain(language==="en"?"89 of 4785":"89 von 4785");
   expect(failure!.message).toContain(language==="en"?"never starts":"startet nie");
   expect(failure!.message).toContain("repo show project");
   expect(failure!.message).toContain("--graph-policy fresh");
   expect(mock.events).toEqual(["shared"]);
   expect(mock.log).toEqual([]);
  }finally{clean(dir);}
 }
});

test("the refusal is a typed refusal: the CLI boundary renders it as itself in both languages, with exit status 1 and without the internal-error text",async()=>{
 for(const language of ["en","de"] as const){
  const dir=scratch();
  try{
   await seed(dir,ringFiles(6));
   const mock=createMockHost(dir,{language});
   const failure=await loadGenerationGraphV1(mock.host,{policy:"current"}).then(()=>undefined,error=>error as Error);
   const lines:string[]=[];let status=-1;
   reportBoundaryErrorV1(failure,{environment:{SEMIO_LANGUAGE:language},osLanguages:()=>[],write:line=>lines.push(line),setExitCode:code=>{status=code;}});
   expect(lines).toEqual([failure!.message]);expect(status).toBe(1);expect(lines.join("\n")).not.toMatch(/Internal error|Interner Fehler/u);
  }finally{clean(dir);}
 }
});

test("a sealed generation answers under the policy current without inference or custody, states its age, and equals the Python model",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));
  const built=createMockHost(dir);
  await loadGenerationGraphV1(built.host);
  await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:'import "@syn/p4";\nexport const v=2;\n'}]);
  for(const language of ["en","de"] as const){
   const mock=createMockHost(dir,{language}),before=mock.log.length;
   const{graph}=await loadGenerationGraphV1(mock.host,{policy:"current"});
   expect(mock.events).toEqual(["shared"]);
   expect(mock.log.length).toBe(before);
   const age=mock.said.find(row=>row.key==="sealedAge");
   expect(age).toBeDefined();
   expect(age!.text).toMatch(language==="en"?/6 projects\), \d+ s old/u:/6 Projekte\), \d+ s alt/u);
   expect(Object.keys((graph as any).projects).sort()).toEqual(Object.keys(pythonGraph(dir).projects).sort());
  }
 }finally{clean(dir);}
});

test("a closure read under the policy current keeps its way: no whole-workspace refusal and no age line",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));
  await loadGenerationGraphV1(createMockHost(dir).host);
  const mock=createMockHost(dir);
  const{graph}=await loadGenerationGraphV1(mock.host,{policy:"current",closure:["p1"]});
  expect(Object.keys((graph as any).projects)).toContain("p1");
  expect(mock.said.some(row=>row.key==="sealedAge")).toBe(false);
 }finally{clean(dir);}
});

test("a current project query reads the sealed graph without selected facts custody or any fallback writer",async()=>{
 const dir=scratch(),name="@semio-tech/framework-os-dev",files=ringFiles(6),manifest="pkg/p2/project.json";
 try{
  files[manifest]=JSON.stringify({...JSON.parse(files[manifest]!),name});
  await seed(dir,files);
  const runtime=new URL("../../../../../⌨️command/⚙️runtime/🟦️.ts",import.meta.url),source=await Bun.file(runtime).text(),tree=ts.createSourceFile(runtime.pathname,source,ts.ScriptTarget.Latest,true);
  const names=["runOwnedBuildInLeasesV1","runBuildGraphQueryV1"],functions=names.map(name=>tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name));
  const program=ts.transpileModule(functions.map(node=>node?.getText(tree)??"").join("\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.ESNext}}).outputText.replace(/\bimport\(/gu,"load(").replaceAll("import.meta.dirname","''").replace(/\bexport \{\};\s*$/u,"");
  const args=["show","project",name,"--graph-policy=current","--json"],selected=extractGraphPolicyV1(args,{}),invocation={control:{signal:new AbortController().signal}},phase={},packets:string[]=[],preGraph:string[]=[];
  const refuse=()=>{throw Error("Current project query attempted writer admission");};
  async function query(mock:ReturnType<typeof createMockHost>){
   Object.assign(mock.host,{withBuilder:refuse,membership:refuse,exchange:{ahead:refuse,upload:true,publish:refuse},settleClosure:refuse,sealed:refuse});
   const load=async(specifier:string)=>{
    if(specifier.endsWith("/🚀️bootstrap/⌨️cli/🟦️.ts"))return {runNativeRepoRecoveryV1:async()=>false};
    if(specifier.endsWith("/🚀️bootstrap/⌨️cli/🔌️ports/🟦️.ts"))return {};
    if(specifier.endsWith("/🕸️graph/📍️pin/🟦️.ts"))return {boundGenerationPinV1};
    if(specifier.endsWith("/📥️opening/🏪️store/🟦️.ts"))return {withRepositoryCacheStoreV1:async(_root:string,_partition:string,_invocation:unknown,_control:unknown,body:(store:unknown,phase:unknown)=>unknown)=>body({},phase)};
    if(specifier.endsWith("/🕸️graph/🚪️admission/🟦️.ts"))return {buildInvocationGraphSelectionV1,readBuildOwnerRegistryV1};
    if(specifier.endsWith("/📥️options/🟦️.ts"))return {selectLiteralBuildProjectsV1};
    if(specifier.endsWith("/📥️inference/🟨️.mjs"))return {readBuildOwnerRegistrySourceV1:()=>JSON.stringify({version:1,owners:[{name,root:"pkg/p2",manifest:"pkg/p2/📋️project.json",packages:[]}],catalogProviders:[]})};
    if(specifier.endsWith("/🕸️graph/🟦️.ts"))return {loadBuildGraphV1:async(_root:string,_original:unknown,_control:unknown,options:{selection:unknown;graph:Parameters<typeof loadGenerationGraphV1>[1]})=>{
     if(JSON.stringify(options.selection)!=='{"kind":"all"}')throw Error("Current project query requested selected graph facts");
     return (await loadGenerationGraphV1(mock.host,options.graph)).graph;
    }};
    if(specifier.endsWith("/🧭️selection/🟦️.ts"))return {buildOptionV1:()=>undefined};
    if(specifier.endsWith("/🌊️json-stream/🟨️.mjs"))return {writeJsonStreamV1};
    throw Error("Unexpected current query import "+specifier);
   };
   const ports={load,process:{env:{}},profilePhaseV1:(_name:string,fn:()=>unknown)=>fn(),profileMarkV1:()=>{},markBuildCapacityPhaseV1:async()=>{},ensureTaskDependenciesV1:async()=>{preGraph.push("dependencies");},maintainTaskStoresV1:async()=>{preGraph.push("maintenance");},withRepositoryCommandEnvironmentV1:(_root:string,_args:unknown,_original:unknown,_phase:unknown,body:(context:unknown)=>unknown)=>body({selected:{graphPolicy:selected.policy},resolved:{args:selected.args},invocation}),machineOutputV1:{write:(text:string,callback?:(error?:Error)=>void)=>{packets.push(text);callback?.();}}};
   const run=new Function(...Object.keys(ports),program+";return runOwnedBuildInLeasesV1")(...Object.values(ports));
   await run(dir,args,invocation,phase);
  }
  const empty=createMockHost(dir);
  await expect(query(empty)).rejects.toMatchObject({code:"GENERATION_NONE_SEALED"});
  expect(empty.events).toEqual(["shared"]);expect(empty.holdsLog).toEqual([]);expect(empty.log).toEqual([]);expect(packets).toEqual([]);
  await loadGenerationGraphV1(createMockHost(dir).host);
  const expected=pythonGraph(dir).projects[name],pointer=join(dir,".gen","current.json"),before=await Bun.file(pointer).text();
  await applyOps(dir,[{op:"write",path:"pkg/p2/src/index.ts",content:'import "@syn/p4";\nexport const changed=2;\n'}]);
  expect(pythonGraph(dir).projects[name]).not.toEqual(expected);
  const current=createMockHost(dir);
  await query(current);
  expect(JSON.parse(packets.join(""))).toEqual(expected);
  expect(current.events).toEqual(["shared"]);expect(current.holdsLog).toEqual([]);expect(current.log).toEqual([]);
  const broken=createMockHost(dir),key="a".repeat(64);
  broken.host.expand=async()=>{throw Object.assign(Error("ENOENT: lstat 'build-input-closures/"+key+".json'"),{code:"ENOENT"});};
  await expect(query(broken)).rejects.toMatchObject({code:"closure-missing",missingClosure:key});
  expect(broken.events).toEqual(["shared"]);expect(broken.holdsLog).toEqual([]);expect(broken.log).toEqual([]);
  expect(await Bun.file(pointer).text()).toBe(before);expect(preGraph).toEqual([]);
 }finally{clean(dir);}
});

test("a whole current assembly with a missing closure neither joins inference nor modifies the sealed generation",async()=>{
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));
  await loadGenerationGraphV1(createMockHost(dir).host);
  const mock=createMockHost(dir),pointer=join(dir,".gen","current.json"),before=await Bun.file(pointer).text(),key="a".repeat(64),effects:string[]=[];
  mock.host.expand=async()=>{throw Object.assign(Error("ENOENT: lstat 'build-input-closures/"+key+".json'"),{code:"ENOENT"});};
  Object.assign(mock.host,{fleetRead:async()=>{effects.push("fleet");throw Error("unexpected fleet inference");},settleClosure:async()=>{effects.push("settle");return"re-derived";}});
  const failure=await loadGenerationGraphV1(mock.host,{policy:"current"}).catch(error=>error);
  expect({code:failure.code,key:failure.missingClosure}).toEqual({code:"closure-missing",key});
  expect(mock.events).toEqual(["shared"]);expect(effects).toEqual([]);expect(mock.log).toEqual([]);
  expect(await Bun.file(pointer).text()).toBe(before);
 }finally{clean(dir);}
});

test("ages are worded in days, hours and minutes and the progress of the whole read in both languages",()=>{
 expect(generationAgeTextV1(0)).toBe("0 s");
 expect(generationAgeTextV1(59)).toBe("59 s");
 expect(generationAgeTextV1(60)).toBe("1 min");
 expect(generationAgeTextV1(3600)).toBe("1 h 0 min");
 expect(generationAgeTextV1(86400+3600*4+60*12+7)).toBe("1 d 4 h 12 min");
 expect(generationAgeTextV1(86400)).toBe("1 d 0 h 0 min");
 expect(renderWholeReadProgressV1(null,"en")).toBe("No whole read is running on this machine.");
 expect(renderWholeReadProgressV1(null,"de")).toBe("Auf dieser Maschine läuft kein ganzes Lesen.");
 expect(renderWholeReadProgressV1({phase:"owners",done:3,total:9,heldSeconds:125},"en")).toBe("The whole read of this machine is at owners, 3 of 9, running for 2 min.");
 expect(renderWholeReadProgressV1({phase:"owners",done:3,total:9,heldSeconds:125},"de")).toBe("Das ganze Lesen dieser Maschine steht bei owners, 3 von 9, seit 2 min.");
 expect(renderGenerationMessageV1("fleetHolds",{},"en")).toContain("fleet cache");
 expect(renderGenerationMessageV1("fleetHolds",{},"de")).toContain("Flotten-Cache");
});
