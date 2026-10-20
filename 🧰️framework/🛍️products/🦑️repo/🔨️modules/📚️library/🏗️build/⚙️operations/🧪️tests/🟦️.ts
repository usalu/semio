import {test,expect} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {spawnSync} from "node:child_process";
import {createRequire} from "node:module";
import {mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createScriptProcessEnvelope,withScriptProcessEnvelope,createIssuedResourceControlV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {buildExecPlanV1,runBuildCacheOperationV1,type BuildCacheToolV1} from "../🟦️.ts";
import {buildHelpForArgumentsV1,readBuildHelpRequestV1,renderBuildHelpV1,resolveBuildHelpLanguageV1,type BuildHelpCatalogV1,type BuildHelpRequestV1} from "../📖️help/🟦️.ts";
import {buildExecEnvironmentV1} from "../🌱️exec-environment/🟦️.ts";
import {validateBuildTaskArgumentsV1} from "../../🧭️selection/🟦️.ts";
import snapshots from "./🧫️fixtures/📖️help/🔣️.json" with {type:"json"};
import catalog from "../🔣️catalog.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import scoping from "./🧫️fixtures/🧭️scoping/🔣️.json" with {type:"json"};

const help=catalog as BuildHelpCatalogV1,taskOperations=["run","run-many","affected","exec","watch"],entry=(name:string)=>name==="root"?[]:name.split(" ");

const root=join(import.meta.dir,"../../../../../../../..").replace(/[\\/]+$/u,""),installed=createRequire(join(root,"package.json")),ajv=new Ajv({strict:true,allowUnionTypes:true}),validator=(definition:string)=>ajv.compile({$schema:schema.$schema,definitions:schema.definitions,$ref:`#/definitions/${definition}`});

/** 🌳️ The installed Nx plans the toy task graph in a clean workspace, independent of this repository's bounded cache contract. */
function nxTaskGraph():unknown{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"nx-plan-")));
 try{
  writeFileSync(join(directory,"nx.json"),"{}\n");
  const source="const {createTaskGraph}=require(process.env.NX_PLAN);const g=JSON.parse(process.env.NX_GRAPH);process.stdout.write(JSON.stringify(createTaskGraph(g.graph,{},[g.project],[g.target],undefined,{})));",
   graph={nodes:fixture.toyGraph.nodes,dependencies:fixture.toyGraph.dependencies},
   result=spawnSync("node",["-e",source],{cwd:directory,encoding:"utf8",env:{...process.env,NX_PLAN:installed.resolve("nx/src/tasks-runner/create-task-graph"),NX_GRAPH:JSON.stringify({graph,project:fixture.toyGraph.project,target:fixture.toyGraph.target}),NX_WORKSPACE_ROOT_PATH:directory,NX_DAEMON:"false"} as NodeJS.ProcessEnv});
  if(result.status!==0)throw Error("nx plan: "+result.stderr);
  return JSON.parse(result.stdout);
 }finally{rmSync(directory,{recursive:true,force:true});}
}

test("the operation catalogue is schema-valid, unique and every synopsis and flag shows in the root help",()=>{
 expect(ajv.compile(schema)(catalog)).toBe(true);
 const names=catalog.operations.map(row=>row.name);
 expect(new Set(names).size).toBe(names.length);
 expect([...names].sort()).toEqual([...schema.definitions.OperationName.enum].sort());
 for(const language of help.languages){const root=renderBuildHelpV1(help,{},language);for(const row of help.operations)expect(root.includes(row.synopsis),language+" "+row.name+": "+row.synopsis).toBe(true);for(const flag of help.flags)for(const name of [flag.name,...(flag.aliases??[])])expect(root.includes(name),language+" "+name).toBe(true);}
});

test("human help and examples use the canonical native frontend in every declared language",()=>{
 const validate=validator("Example"),database=new Database(":memory:");
 try{
  const rows=database.query("SELECT json_extract(example.value, '$.command') AS command FROM json_each(?, '$.operations') operation, json_each(operation.value, '$.examples') example").all(JSON.stringify(catalog)) as {command:string}[];
  expect(rows.length).toBeGreaterThan(0);
  for(const row of rows){
   expect(row.command.startsWith("repo "),row.command).toBe(true);
   expect(row.command.includes("bun repo"),row.command).toBe(false);
   const description={en:"Native frontend",de:"Natives Frontend"};
   expect(validate({...row,description})).toBe(true);
   expect(validate({command:"bun "+row.command,description})).toBe(false);
  }
  for(const language of help.languages)for(const name of ["root",...help.operations.map(row=>row.name)]){
   const rendered=renderBuildHelpV1(help,readBuildHelpRequestV1(help,entry(name)),language);
   expect(rendered.startsWith(name==="root"?"repo - ":"repo "+name+" - ")).toBe(true);
   expect(rendered.includes("bun repo")).toBe(false);
  }
 }finally{database.close();}
});

test("every neutral operation has a tool state, and unimplemented states are honest",async()=>{
 for(const row of catalog.operations){
  expect(["nx","bazel"].every(tool=>["native","adapter","neutral","missing"].includes(row.implementations[tool as "nx"|"bazel"])),row.name).toBe(true);
  if(row.dryRun)expect(row.notes?.some(note=>note.en.includes("--dry-run")||note.en.includes("Changed files"))||row.name==="run-many"||row.name==="affected",row.name).toBe(true);
 }
 const cache=catalog.operations.find(row=>row.name==="cache")!,bazel=(await import("../../🔧️tools/🏗️bazel/⚙️operations/🟦️.ts")).bazelBuildCacheToolV1();
 expect(cache.implementations.bazel).toBe("adapter");
 expect(bazel.tool).toBe("bazel");
 expect([typeof bazel.status,typeof bazel.clear]).toEqual(["function","function"]);
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"build-operations-model",maximumElapsedMilliseconds:30000},{},Date.now()),async original=>{
  const context={root,invocation:original,phaseControl:createIssuedResourceControlV1(original,"test:build-operations",0)};
  await expect(runBuildCacheOperationV1("bazel",["purge"],context,async()=>bazel)).rejects.toThrow("Expected repo cache status|clear");
 });
});

test("cache results and dry-run plans satisfy the neutral result schemas, with Nx's own task graph as the cross-check",()=>{
 expect(validator("CacheStatus")(fixture.cacheStatus)).toBe(true);
 expect(validator("CacheClear")(fixture.cacheClear)).toBe(true);
 expect(validator("CacheStatus")({...fixture.cacheStatus,entries:-1})).toBe(false);
 const plan=buildExecPlanV1(fixture.execProjects),dry=validator("DryRunPlan");
 expect(dry(JSON.parse(JSON.stringify(plan))),JSON.stringify(dry.errors)).toBe(true);
 expect(plan.tasks.roots).toEqual(fixture.execProjects.map(project=>project+":repo-exec"));
 const nx=nxTaskGraph() as {tasks:Record<string,{target:{project:string;target:string}}>;roots:string[]};
 expect(Object.keys(nx.tasks).sort()).toEqual(["app:build","lib:build"]);
 expect(dry({tasks:nx}),JSON.stringify(dry.errors)).toBe(true);
 expect(dry({tasks:{...nx,roots:"app:build"}})).toBe(false);
},60_000);

test("the cache operation validates its arguments and derives the clear result from before and after",async()=>{
 await withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"build-operations-clear-model",maximumElapsedMilliseconds:30000},{},Date.now()),async original=>{
  const context={root,invocation:original,phaseControl:createIssuedResourceControlV1(original,"test:build-operations-clear",0)},calls:string[]=[];
  let cleared=false;
  const tool:BuildCacheToolV1={tool:"nx",status:async()=>({version:1,tool:"nx",available:true,directory:"/cache",...(cleared?fixture.removal.after:fixture.removal.before)}),clear:async()=>{calls.push("clear");cleared=true;}};
  for(const args of fixture.refusedCacheArguments)await expect(runBuildCacheOperationV1("nx",args,context,async()=>tool),args.join(" ")).rejects.toThrow("Expected repo cache status|clear");
  expect(await runBuildCacheOperationV1("nx",["status"],context,async()=>tool)).toEqual({version:1,tool:"nx",available:true,directory:"/cache",...fixture.removal.before});
  expect(calls).toEqual([]);
  const result=await runBuildCacheOperationV1("nx",["clear"],context,async()=>tool);
  expect(result).toEqual({version:1,tool:"nx",cleared:true,removedEntries:fixture.removal.removed.entries,removedBytes:fixture.removal.removed.bytes});
  expect(validator("CacheClear")(result)).toBe(true);
  expect(calls).toEqual(["clear"]);
 });
});

test("the help of every command and subcommand equals the language-agnostic snapshot in every language",()=>{
 expect(help.languages).toEqual(Object.keys(snapshots.snapshots));
 for(const language of help.languages){
  const expected=(snapshots.snapshots as Record<string,Record<string,string[]>>)[language]!,rows=["root",...help.operations.flatMap(row=>[row.name,...(row.subcommands??[]).map(sub=>row.name+" "+sub.name)])];
  expect(Object.keys(expected),language).toEqual(rows);
  for(const row of rows)expect(renderBuildHelpV1(help,readBuildHelpRequestV1(help,entry(row)),language).split("\n"),language+" "+row).toEqual(expected[row]!);
 }
});

test("every command prints its own usage, distinct from the root usage, and each language translates every text",()=>{
 const english=new Set<string>();
 for(const row of help.operations){
  const own=renderBuildHelpV1(help,{operation:row.name},"en");
  expect(own.includes(`repo ${row.name} - `),row.name).toBe(true);
  expect(own.includes(row.synopsis),row.name).toBe(true);
  expect(own.includes("Examples:"),row.name).toBe(true);
  expect(own,row.name).not.toBe(renderBuildHelpV1(help,{},"en"));
  for(const name of row.flags){const flag=help.flags.find(candidate=>candidate.name===name);expect(flag,row.name+" "+name).toBeDefined();expect(own.includes(name),row.name+" "+name).toBe(true);expect(flag!.operations?.includes(row.name),row.name+" "+name).toBe(true);}
  english.add(own);
 }
 expect(english.size).toBe(help.operations.length);
 const texts=(value:unknown):Record<string,string>[]=>Array.isArray(value)?value.flatMap(texts):value&&typeof value==="object"?("en" in value&&"de" in value?[value as Record<string,string>]:Object.values(value).flatMap(texts)):[];
 for(const text of texts(catalog.labels).concat(texts(catalog.flags),texts(catalog.operations)))expect(text.de,text.en).not.toBe(text.en);
});

test("the documented flags are exactly the flags the task validator and the other operations accept",()=>{
 const source=readFileSync(join(import.meta.dir,"../../🧭️selection/🟦️.ts"),"utf8"),validator=source.slice(source.indexOf("export function validateBuildTaskArgumentsV1"));
 const names=(label:string)=>[...validator.slice(validator.indexOf(`${label}=new Set([`)).match(/Set\(\[([^\]]*)\]/u)![1]!.matchAll(/"([^"]+)"/gu)].map(match=>match[1]!);
 const accepted=new Set([...names("const values"),...names("booleans"),...names("optional")]),documented=new Set<string>();
 for(const flag of help.flags)if(flag.operations===undefined?flag.name==="--help":flag.operations.some(operation=>taskOperations.includes(operation)))for(const spelled of [flag.name,...(flag.aliases??[])].filter(candidate=>candidate!=="-h"))documented.add(spelled.replace(/^--?/u,""));
 expect([...accepted].sort()).toEqual([...documented].sort());
 for(const row of help.operations.filter(candidate=>taskOperations.includes(candidate.name)))for(const name of row.flags){
  const flag=help.flags.find(candidate=>candidate.name===name)!,sample=({"stream|stream-without-prefixes|static":"static","prototype|release":"prototype"} as Record<string,string>)[flag.valueName??""]??"x",
   argument=flag.value==="none"?name:`${name}=${sample}`,head=row.name==="run"?["run","app:build"]:[row.name],tail=["exec","watch"].includes(row.name)?["--","echo"]:[];
  expect(()=>validateBuildTaskArgumentsV1([...head,argument,...tail]),row.name+" "+argument).not.toThrow();
 }
});

test("the help language follows --lang, the environment and the system, never assumes one, and refuses unknown choices",()=>{
 const none=()=>[] as string[];
 expect(resolveBuildHelpLanguageV1(help.languages,["--lang=de"],{LANG:"en_US.UTF-8"},none)).toBe("de");
 expect(resolveBuildHelpLanguageV1(help.languages,["--lang","de"],{},none)).toBe("de");
 expect(resolveBuildHelpLanguageV1(help.languages,[],{SEMIO_LANGUAGE:"de",LANG:"en_US.UTF-8"},none)).toBe("de");
 expect(resolveBuildHelpLanguageV1(help.languages,[],{LANG:"de_DE.UTF-8"},none)).toBe("de");
 expect(resolveBuildHelpLanguageV1(help.languages,[],{LC_ALL:"C",LANG:"fr_FR.UTF-8"},()=>["fr-FR","de-DE"])).toBe("de");
 expect(resolveBuildHelpLanguageV1(help.languages,["--","--lang=de"],{LANG:"en_US.UTF-8"},none)).toBe("en");
 for(const [args,environment] of [[[],{}],[["--","--lang=de"],{LANG:"C"}]] as const){
  let message="";try{resolveBuildHelpLanguageV1(help.languages,args,environment,none);}catch(error){message=(error as Error).message;}
  const lines=message.split("\n");
  expect(lines.length).toBe(help.languages.length);
  expect(lines[0]).toContain("No language selected");expect(lines[1]).toContain("Keine Sprache");
 }
 expect(()=>resolveBuildHelpLanguageV1(help.languages,["--lang=fr"],{},none)).toThrow("Unknown language: fr");
 expect(readBuildHelpRequestV1(help,["--tool","bazel","cache","clear","--help"])).toEqual({operation:"cache",subcommand:"clear"});
 expect(readBuildHelpRequestV1(help,["run","a:b","--help"])).toEqual({operation:"run"});
 expect(readBuildHelpRequestV1(help,["--help"])).toEqual({});
 expect(()=>readBuildHelpRequestV1(help,["frobnicate","--help"])).toThrow("Unknown command: frobnicate");
 expect(buildHelpForArgumentsV1(["dependencies","--help","--lang=de"],{})).toContain("Verwendung:");
});

test("exec exports the tool-neutral project environment and the catalogue never names an Nx variable",()=>{
 expect(buildExecEnvironmentV1("app","packages/app")).toEqual({SEMIO_PROJECT_NAME:"app",SEMIO_PROJECT_ROOT:"packages/app"});
 expect(validator("ExecEnvironment")(buildExecEnvironmentV1("app","packages/app"))).toBe(true);
 expect(validator("ExecEnvironment")({NX_PROJECT_NAME:"app",NX_PROJECT_ROOT_PATH:"packages/app"})).toBe(false);
 expect(JSON.stringify(catalog)).not.toMatch(/NX_[A-Z_]+/u);
 expect(JSON.stringify(schema)).not.toMatch(/NX_[A-Z_]+/u);
});

test("the real CLI prints the own help of a command, also for preflight, without running it",()=>{
 const script=join(import.meta.dir,"../../📜️script.ts");
 const run=(args:string[])=>spawnSync(process.execPath,[script,"repo",...args],{cwd:root,encoding:"utf8",env:{...process.env,SEMIO_LANGUAGE:undefined,LANG:"C",LC_ALL:"C"} as NodeJS.ProcessEnv});
 for(const [args,expected] of [[["preflight","--help"],"repo preflight - "],[["cache","clear","--help"],"repo cache clear - "],[["dependencies","lock","--help","--lang=de"],"Verwendung:"]] as const){
  const result=run([...args]);
  expect(result.status,args.join(" ")+result.stderr).toBe(0);
  expect(result.stdout.includes(expected),args.join(" ")+"\n"+result.stdout).toBe(true);
 }
},120_000);

test("the help request reader skips the value of every option with a required value in either spelling",()=>{
 expect(readBuildHelpRequestV1(help,["--workers","on","cache","clear","--help"])).toEqual({operation:"cache",subcommand:"clear"});
 expect(readBuildHelpRequestV1(help,["--configuration","prototype","run","--help"])).toEqual({operation:"run"});
 expect(readBuildHelpRequestV1(help,["run-many","-t","test","-p","a","--help"])).toEqual({operation:"run-many"});
});

test("every note is rendered in every language, on every page of its operation",()=>{
 let rendered=0;
 for(const operation of help.operations){
  for(const note of operation.notes??[]){
   for(const language of help.languages){
    expect(Object.keys(note),operation.name).toEqual([...help.languages]);
    for(const request of [{operation:operation.name},...(operation.subcommands??[]).map(sub=>({operation:operation.name,subcommand:sub.name}))] as readonly BuildHelpRequestV1[]){
     const page=renderBuildHelpV1(help,request,language);
     expect(page.includes(`  - ${note[language]}`),`${operation.name} ${request.subcommand??""} ${language}`).toBe(true);
     expect(page.includes(`${help.labels.notes![language]}:`)).toBe(true);
    }
   }
   rendered++;
  }
 }
 expect(rendered).toBe(scoping.noteCount);
});

test("a subcommand lists only the options it reads and the examples that belong to it",()=>{
 for(const row of scoping.scopedSubcommands){
  const operation=help.operations.find(candidate=>candidate.name===row.operation)!,subcommand=operation.subcommands!.find(candidate=>candidate.name===row.subcommand)!;
  expect(subcommand.flags,row.operation+" "+row.subcommand).toEqual(row.flags);
  for(const name of subcommand.flags??[])expect(operation.flags.includes(name),name).toBe(true);
  for(const language of help.languages){
   const page=renderBuildHelpV1(help,{operation:row.operation,subcommand:row.subcommand},language),optionLines=page.split("\n").filter(line=>/^  (-\w, )?--[a-z]/u.test(line)&&!/^  (--tool|--workers|-h, --help|--lang)/u.test(line));
   expect(optionLines.length,row.operation+" "+row.subcommand+" "+language).toBe(row.flags.length);
   for(const name of row.flags)expect(page.includes(`  ${name}`),name).toBe(true);
   for(const command of row.examples)expect(page.includes(`  ${command}`),command).toBe(true);
   for(const command of row.excludedExamples)expect(page.includes(`  ${command}`),command).toBe(false);
  }
 }
 expect(help.operations.find(row=>row.name==="dependencies")!.synopsis).toBe(scoping.dependenciesSynopsis);
});

test("the neutral help never names a tool environment variable or a tool command line",()=>{
 for(const language of help.languages){
  const pages=[renderBuildHelpV1(help,{},language),...help.operations.flatMap(operation=>[renderBuildHelpV1(help,{operation:operation.name},language),...(operation.subcommands??[]).map(sub=>renderBuildHelpV1(help,{operation:operation.name,subcommand:sub.name},language))])].join("\n");
  expect(pages,language).not.toMatch(/\bNX_[A-Z_]+|\bBAZEL_[A-Z_]+|--remote_[a-z_]+|--nocache_test_results|--build_tests_only|nx reset/u);
 }
});
