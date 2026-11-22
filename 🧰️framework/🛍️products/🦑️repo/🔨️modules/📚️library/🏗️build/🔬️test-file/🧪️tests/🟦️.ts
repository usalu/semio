import {expect,test} from "bun:test";
import Ajv from "ajv";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,symlinkSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {CATALOG_LANGUAGES_V1,placeholdersV1,textV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🗣️language/📖️catalog/🟦️.ts";
import {discoverProcessTestFilesV1} from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🗂️suites/🟦️.ts";
import {requireTestToolV1} from "../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import policy from "../🔣️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import texts from "../📖️texts/🔣️.json" with {type:"json"};
import corpus from "./🧫️fixtures/🔣️.json" with {type:"json"};
import catalogue from "../../⚙️operations/🔣️catalog.json" with {type:"json"};
import {readTestFilePolicyV1,readTestFileTablesV1,renderTestFilePlanV1,renderTestFileResultV1,resolveTestFilesV1,routeTestFilesV1,testFileArgumentsV1,testFileBunCountsV1,testFileOptionNamesV1,testFileResultV1,testFileTailV1,testFileWantsPythonV1,validateTestFileDocumentV1} from "../🟦️.ts";
import {TEST_FILE_TEXTS_V1} from "../📖️texts/🟦️.ts";
import {providedTestEnvironmentV1} from "../🏃️run/🟦️.ts";
import storageSchema from "../../../🗂️workspaces/🦀️cargo/🛠️preparation/📦️storage/🧬️schema/🔣️.json" with {type:"json"};
import {isAbsolute} from "node:path";
import {validateJsonSchemaSubset} from "../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {runTestFileCliV1} from "../⌨️command/🟦️.ts";

const oracle=join(import.meta.dir,"🐍️oracle","🐍️.py"),root=(()=>{let directory=import.meta.dir;while(!existsSync(join(directory,"MODULE.bazel"))&&dirname(directory)!==directory)directory=dirname(directory);return directory;})();
const ajv=new Ajv({strict:false,allowUnionTypes:true}),definitions=schema.definitions,validator=(name:string)=>ajv.compile({$schema:schema.$schema,definitions,$ref:`#/definitions/${name}`});
const python=(args:readonly string[],input:unknown):unknown=>{
 const result=spawnSync(requireTestToolV1("python"),[oracle,...args],{encoding:"utf8",input:JSON.stringify(input),maxBuffer:1<<28});
 expect(result.status,result.stderr).toBe(0);
 return JSON.parse(result.stdout);
};
const refusal=(run:()=>unknown):string|undefined=>{try{run();return undefined;}catch(error){return String((error as {key?:string}).key).replace(/^test-file\./u,"");}};
const here="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔬️test-file/🧪️tests/🧫️fixtures";
const run=async(args:readonly string[])=>{const lines:string[]=[],code=await runTestFileCliV1(root,args,{...process.env,SEMIO_LANGUAGE:"en"},line=>{lines.push(line);},{notice:()=>{}});return {code,json:JSON.parse(lines.join("\n")) as ReturnType<typeof testFileResultV1>};};

test("the policy satisfies its closed schema under the in-repo validator and under Ajv, and both reject the same mutations",()=>{
 const check=validator("Policy");
 expect(check(policy),JSON.stringify(check.errors?.[0])).toBe(true);
 expect(validateTestFileDocumentV1("Policy",policy)).toEqual([]);
 expect(readTestFilePolicyV1()).toBe(policy as never);
 const mutations:Record<string,(value:any)=>void>={extra:value=>{value.extra=1;},noTables:value=>{value.tables=[];},badTable:value=>{value.tables[0].project="nope";},badLimit:value=>{value.limits.maximumFiles=0;},badMode:value=>{value.defaultBuildMode="debug";},missing:value=>{delete value.scrub;},badProvider:value=>{value.providers[0].provider="nope";},noProviders:value=>{delete value.providers;}};
 for(const [name,mutate] of Object.entries(mutations)){
  const copy=JSON.parse(JSON.stringify(policy));mutate(copy);
  expect(check(copy),name+" ajv").toBe(false);
  expect(validateTestFileDocumentV1("Policy",copy).length,name+" in-repo").toBeGreaterThan(0);
  expect(()=>readTestFilePolicyV1(copy),name).toThrow();
 }
 for(const row of policy.tables){expect(existsSync(join(root,row.root)),row.id).toBe(true);expect(existsSync(join(root,row.table)),row.id).toBe(true);}
 expect(existsSync(join(root,policy.launcherScript))).toBe(true);
 for(const row of policy.providers){
  expect(existsSync(join(root,row.root)),row.root).toBe(true);
  expect(policy.tables.some(table=>row.root.startsWith(table.root+"/")||table.root.startsWith(row.root+"/")),"a provider root lies outside every suite table: "+row.root).toBe(false);
  const files=discoverProcessTestFilesV1(join(root,row.root));expect(files.length,row.root).toBeGreaterThan(0);
 }
});

test("every text exists in both languages with exactly its placeholders, differs between them unless language-neutral, is used by the code and every key the code names exists",()=>{
 const neutral=["run.start"],keys=Object.keys(texts.messages),code=["🟦️.ts","⌨️command/🟦️.ts","🏃️run/🟦️.ts"].map(file=>readFileSync(join(import.meta.dir,"..",file),"utf8")).join("\n");
 expect(TEST_FILE_TEXTS_V1.id).toBe("test-file");
 for(const [key,leaf] of Object.entries(texts.messages)){
  const names=placeholdersV1(leaf.en),values=Object.fromEntries(names.map(name=>[name,"v"+name]));
  for(const language of CATALOG_LANGUAGES_V1){const text=textV1(TEST_FILE_TEXTS_V1.catalog,key,language,values);for(const name of names)expect(text,key+" "+language).toContain("v"+name);}
  expect([...placeholdersV1(leaf.de)].sort(),key).toEqual([...names].sort());
  expect(leaf.en===leaf.de,key).toBe(neutral.includes(key));
  expect(code.includes(JSON.stringify(key)),key+" is unused").toBe(true);
 }
 for(const match of code.matchAll(/(?:refuseTestFileV1\(\s*"[a-z]+"\s*,|testFileTextV1\(\s*[A-Za-z]+\s*,)\s*("(?:[^"\\]|\\.)*")/gu))expect(keys,match[1]).toContain(JSON.parse(match[1]!));
});

test("the argument vectors parse as the corpus says, and the independent Python reading agrees on every one",()=>{
 const vectors=corpus.arguments as readonly {id:string;args:string[];expected?:Record<string,unknown>;refusal?:string}[],oracleRows=python(["arguments"],vectors.map(row=>row.args)) as Record<string,unknown>[];
 for(const [at,row] of vectors.entries()){
  const parsed=row.refusal===undefined?testFileArgumentsV1(row.args):undefined,failed=refusal(()=>testFileArgumentsV1(row.args));
  expect(failed,row.id).toBe(row.refusal);
  if(row.expected!==undefined)expect({...parsed,language:parsed!.language??null},row.id).toEqual(row.expected as never);
  expect(oracleRows[at],row.id).toEqual(row.refusal!==undefined?{refusal:row.refusal}:row.expected as never);
 }
 expect(refusal(()=>testFileArgumentsV1(Array.from({length:policy.limits.maximumFiles+1},(_,at)=>"f"+at+".ts")))).toBe("files.too-many");
 expect(testFileOptionNamesV1()).toEqual({valued:["--lang"],plain:["--json","--bail","--dry-run"]});
});

test("the catalogue row of the operation names exactly the options the parser accepts and a synopsis that starts with its name",()=>{
 const row=catalogue.operations.find(operation=>operation.name==="test-file") as {name:string;requires:string;synopsis:string;dryRun:boolean;progress:boolean;cancellable:boolean;flags:string[]}|undefined;
 expect(row,"catalogue row").toBeDefined();
 const accepted=testFileOptionNamesV1(),names=[...accepted.plain].sort(),declared=[...(row?.flags??[])].sort();
 expect(declared).toEqual(names);
 expect(row?.synopsis.startsWith("test-file ")).toBe(true);
 expect(row?.requires).toBe("none");
 expect([row?.dryRun,row?.progress,row?.cancellable]).toEqual([true,true,true]);
 for(const flag of names)expect((catalogue.flags as {name:string;operations?:string[]}[]).find(entry=>entry.name===flag)?.operations,flag).toContain("test-file");
});

test("every test file of the three suite tables routes to the same suites as the independent Python reading",()=>{
 const tables=readTestFileTablesV1(root),files=policy.tables.flatMap(row=>discoverProcessTestFilesV1(join(root,row.root)).map(file=>row.root+"/"+file));
 expect(files.length).toBeGreaterThan(500);
 const theirs=python(["routes",root,join(import.meta.dir,"..","🔣️.json")],files) as Record<string,"helper"|{table:string|null;suite:string}[]>,helpers=files.filter(file=>theirs[file]==="helper"),tests=files.filter(file=>theirs[file]!=="helper");
 expect(helpers.length,"the tables declare helpers").toBeGreaterThan(0);
 for(const file of helpers.slice(0,10))expect(refusal(()=>routeTestFilesV1(root,[file],tables,undefined,process.platform,()=>"")),file).toBe("file.helper");
 const routes=routeTestFilesV1(root,tests,tables,undefined,process.platform,()=>""),mine=new Map<string,string[]>();
 for(const row of routes)mine.set(row.file,[...(mine.get(row.file)??[]),(row.table??"null")+":"+row.suite]);
 const differing=tests.filter(file=>JSON.stringify([...(mine.get(file)??[])].sort())!==JSON.stringify((theirs[file] as {table:string|null;suite:string}[]).map(row=>(row.table??"null")+":"+row.suite).sort()));
 expect(differing.slice(0,5)).toEqual([]);
});

test("the corpus routes land on the suite, strictness, launcher and Python binding their tables declare, and an unowned file runs plain",()=>{
 const tables=readTestFileTablesV1(root),expected=Object.fromEntries((corpus.routes as {id:string;file:string}[]).map(row=>[row.id,routeTestFilesV1(root,[row.file],tables,undefined,"darwin")]));
 expect(expected["build-file-in-tree"]!.map(row=>row.table+":"+row.suite)).toEqual(["build:build-test-file"]);
 expect(expected["build-door"]![0]).toMatchObject({mode:"suite",table:"build",owner:"build",python:false,launcher:null,runsOnHost:true});
 expect(expected["caching-suite-table"]![0]).toMatchObject({mode:"suite",table:"caching",owner:"cache"});
 expect(expected["process-routing"]![0]).toMatchObject({mode:"suite",table:"process",owner:"process"});
 expect(expected["plain-law"]).toEqual([expect.objectContaining({mode:"plain",table:null,suite:"plain",python:true,typecheck:"none",budgetMs:policy.plain.budgetMs})]);
 const fake=new Map([["build",{version:1 as const,codeSuites:[],helpers:[],suites:[{id:"strict",budgetMs:1000,typecheck:"strict-indexed" as const,files:["x/🧪️tests/🟦️.ts"],tools:["python"],platforms:["win32" as const],fileLaunchers:{"x/🧪️tests/🟦️.ts":"genuine"}}]}]]);
 const row=routeTestFilesV1(root,[policy.tables[0]!.root+"/x/🧪️tests/🟦️.ts"],fake as never,undefined,"darwin",()=>"")[0]!;
 expect(row).toMatchObject({mode:"suite",suite:"strict",typecheck:"strict-indexed",python:true,launcher:"genuine",runsOnHost:false});
 expect(testFileWantsPythonV1("const x = 1")).toBe(false);
 expect(testFileWantsPythonV1("requireTestToolV1(\"python\")")).toBe(true);
});

test("files resolve against the caller's directory first and the checkout second, and refuse missing, outside, non-regular and non-test paths",()=>{
 const base=mkdtempSync(join(tmpdir(),"test-file-")),outside=mkdtempSync(join(tmpdir(),"test-file-outside-"));
 try{
  mkdirSync(join(base,"pkg","🧪️tests"),{recursive:true});writeFileSync(join(base,"pkg","🧪️tests","a.ts"),"");writeFileSync(join(base,"pkg","note.md"),"");writeFileSync(join(outside,"o.ts"),"");symlinkSync(join(outside,"o.ts"),join(base,"pkg","link.ts"));
  expect(resolveTestFilesV1(base,join(base,"pkg"),["🧪️tests/a.ts"])).toEqual(["pkg/🧪️tests/a.ts"]);
  expect(resolveTestFilesV1(base,outside,["pkg/🧪️tests/a.ts",join(base,"pkg","🧪️tests","a.ts")])).toEqual(["pkg/🧪️tests/a.ts"]);
  expect(refusal(()=>resolveTestFilesV1(base,base,["pkg/nope.ts"]))).toBe("file.missing");
  expect(refusal(()=>resolveTestFilesV1(base,base,[join(outside,"o.ts")]))).toBe("file.outside");
  expect(refusal(()=>resolveTestFilesV1(base,base,["pkg/link.ts"]))).toBe("file.outside");
  expect(refusal(()=>resolveTestFilesV1(base,base,["pkg"]))).toBe("file.not-file");
  expect(refusal(()=>resolveTestFilesV1(base,base,["pkg/note.md"]))).toBe("file.not-test");
 }finally{rmSync(base,{recursive:true,force:true});rmSync(outside,{recursive:true,force:true});}
});

test("bun's counts are those of the last summary, a missing summary yields none, and a tail keeps whole last lines",()=>{
 expect(testFileBunCountsV1("bun test\n\n 9 pass\n 1 fail\n 452 expect() calls\nRan 10 tests across 1 file.")).toEqual({pass:9,fail:1});
 expect(testFileBunCountsV1(" 2 pass\n 0 fail\n 3 pass\n 1 fail\n")).toEqual({pass:3,fail:1});
 expect(testFileBunCountsV1(" 4 pass\n")).toEqual({pass:4,fail:0});
 expect(testFileBunCountsV1("nothing here")).toBeUndefined();
 expect(testFileTailV1("a\nb\nc\nd",5)).toBe("c\nd");
 expect(testFileTailV1("short",50)).toBe("short");
});

test("a result and a plan satisfy their schemas under both validators, render in both languages and the verdict follows the failures",()=>{
 const rows=[{file:"a.ts",mode:"suite" as const,table:"build",suite:"s",status:"passed" as const,seconds:1.2,pass:3,fail:0},{file:"b.ts",mode:"plain" as const,table:null,suite:"plain",status:"failed" as const,seconds:0.4,pass:0,fail:1,error:"boom",tail:"x"},{file:"c.ts",mode:"suite" as const,table:"build",suite:"s",status:"skipped" as const,seconds:0},{file:"d.ts",mode:"suite" as const,table:"build",suite:"s",status:"not-run" as const,seconds:0}];
 const result=testFileResultV1(rows,2,true),check=validator("Result");
 expect(result.status).toBe("failed");
 expect(result.counts).toEqual({passed:1,failed:1,skipped:1,notRun:1});
 expect(check(result),JSON.stringify(check.errors?.[0])).toBe(true);
 expect(validateTestFileDocumentV1("Result",result)).toEqual([]);
 expect(testFileResultV1(rows.slice(0,1),1,false).status).toBe("passed");
 for(const mutate of [(value:any)=>{value.status="maybe";},(value:any)=>{value.files[0].seconds=-1;},(value:any)=>{value.counts.passed="1";},(value:any)=>{delete value.bail;},(value:any)=>{value.files[0].status="done";}]){
  const copy=JSON.parse(JSON.stringify(result));mutate(copy);
  expect(check(copy)).toBe(false);
  expect(validateTestFileDocumentV1("Result",copy).length).toBeGreaterThan(0);
 }
 const english=renderTestFileResultV1(result,"en"),german=renderTestFileResultV1(result,"de");
 expect(english).toContain("FAILED  b.ts");
 expect(german).toContain("FEHLGESCHLAGEN  b.ts");
 expect(english).toContain("SKIPPED (not passed)");
 expect(english).not.toEqual(german);
 const plan={version:1 as const,files:[{file:"a.ts",mode:"suite" as const,table:"build",suite:"s",owner:"build",typecheck:"strict" as const,budgetMs:300000,python:true,launcher:"g",runsOnHost:false}]},planCheck=validator("Plan");
 expect(planCheck(plan),JSON.stringify(planCheck.errors?.[0])).toBe(true);
 expect(validateTestFileDocumentV1("Plan",plan)).toEqual([]);
 expect(renderTestFilePlanV1(plan,"en")).toContain("managed Python, launcher g, skipped on this host");
 expect(renderTestFilePlanV1(plan,"de")).toContain("verwaltetes Python, Starter g, auf diesem Rechner übersprungen");
});

test("a dry run prints the plan and runs nothing, in JSON and in text, and a refusal exits 2 with the text in the resolved language",async()=>{
 const lines:string[]=[],notices:string[]=[],file=here+"/🟢️passing/🟦️.ts",call=(args:string[],language:string)=>runTestFileCliV1(root,args,{...process.env,SEMIO_LANGUAGE:language},line=>{lines.push(line);},{notice:line=>{notices.push(line);}});
 expect(await call(["--dry-run","--json",file],"en")).toBe(0);
 expect(validateTestFileDocumentV1("Plan",JSON.parse(lines[0]!))).toEqual([]);
 expect(JSON.parse(lines[0]!).files[0]).toMatchObject({mode:"suite",table:"build",suite:"build-test-file"});
 expect(await call(["--dry-run",file],"de")).toBe(0);
 expect(lines[1]).toContain("Typprüfung none");
 expect(await call(["--dry-run"],"de")).toBe(2);
 expect(notices[0]).toContain("Erwartet: repo test-file");
 expect(await call(["--dry-run","nope.ts"],"en")).toBe(2);
 expect(notices[1]).toContain("No such file: nope.ts");
});

test("files run one by one as their suite runner runs them: a passing and a failing fixture give a passed and a failed row with counts and the failure's tail, bail leaves the rest unrun, and the sequential envelopes do not interfere",async()=>{
 const passing=here+"/🟢️passing/🟦️.ts",failing=here+"/🔴️failing/🟦️.ts";
 const both=await run(["--json",passing,failing]);
 expect(both.code).toBe(1);
 expect(validateTestFileDocumentV1("Result",both.json)).toEqual([]);
 expect(both.json.files.map(row=>[row.status,row.suite])).toEqual([["passed","build-test-file"],["failed","build-test-file"]]);
 expect(both.json.files[0]).toMatchObject({pass:1,fail:0});
 expect(both.json.files[1]!.tail).toContain("the failing fixture fails on purpose");
 expect(both.json.files[1]!.error).toContain("build-test-file");
 expect(both.json.counts).toEqual({passed:1,failed:1,skipped:0,notRun:0});
 const bailed=await run(["--json","--bail",failing,passing]);
 expect(bailed.code).toBe(1);
 expect(bailed.json.files.map(row=>row.status)).toEqual(["failed","not-run"]);
 expect(bailed.json.bail).toBe(true);
 const ok=await run(["--json",passing]);
 expect(ok.code).toBe(0);
 expect(ok.json.status).toBe("passed");
},600000);

test.skipIf(process.platform==="win32")("cancelling a run with SIGINT ends it with exit code 130, prints no result and leaves no test child behind",async()=>{
 const sleeping=join(root,here,"😴️sleeping","🟦️.ts"),driver=join(root,here,"🚀️drive","🟦️.ts"),child=Bun.spawn([process.execPath,driver,root,"--json",sleeping],{stdout:"pipe",stderr:"pipe",env:{...process.env,SEMIO_LANGUAGE:"en"}});
 const decoder=new TextDecoder(),reader=child.stderr.getReader();
 let seen="";
 while(!seen.includes("[test-file] 1/1")){const chunk=await reader.read();if(chunk.done)break;seen+=decoder.decode(chunk.value);}
 await Bun.sleep(4000);
 child.kill("SIGINT");
 const code=await child.exited,output=await new Response(child.stdout).text();
 expect(code).toBe(130);
 expect(output).toBe("");
 if(process.platform!=="win32")expect(spawnSync("pgrep",["-f",sleeping],{encoding:"utf8"}).status).toBe(1);
},120000);

test("a file below a provider root runs the plain route and the provider supplies the Cargo preparation custody storage of the checkout it is given, the one the project scripts' contract targets run under: valid under the storage schema in both validators and inside that checkout's own cache and lease partitions",async()=>{
 const file=policy.providers[0]!.root+"/🧪️tests/🟦️.ts",route=routeTestFilesV1(root,[file],readTestFileTablesV1(root))[0]!;
 expect(route.mode).toBe("plain");
 const checkout=mkdtempSync(join(tmpdir(),"tf-provider-")),provided=await providedTestEnvironmentV1(policy.providers[0]!.provider,checkout),value=JSON.parse(provided.SEMIO_CARGO_PREPARATION_STORAGE!);
 try{
  expect(Object.keys(provided)).toEqual(["SEMIO_CARGO_PREPARATION_STORAGE"]);
  expect(new Ajv({strict:false}).compile(storageSchema)(value)).toBe(true);
  expect(validateJsonSchemaSubset(storageSchema,value)).toEqual([]);
  expect(value.version).toBe(1);
  for(const key of ["directory","leaseDirectory","program"]){expect(isAbsolute(value[key]),key).toBe(true);expect(value[key].startsWith(checkout+"/"),key).toBe(true);}
  expect(value.directory.startsWith(join(checkout,".cache")),"the storage lies in the checkout's cache partition").toBe(true);
  expect(value.program.endsWith("📜️script.ts")).toBe(true);
 }finally{rmSync(checkout,{recursive:true,force:true});}
});

test("a file that outlives the deadline of its envelope is a failed row with the deadline as its error, not a crash of the run, and the rest of the run goes on",async()=>{
 const shortPolicy={...policy,plain:{...policy.plain,budgetMs:1000},elapsedSlackMs:0} as never,sleeping=here+"/😴️sleeping/🟦️.ts",passing=here+"/🟢️passing/🟦️.ts",tables=new Map() as ReturnType<typeof readTestFileTablesV1>;
 const {runTestFileRoutesV1}=await import("../🏃️run/🟦️.ts"),routes=routeTestFilesV1(root,[sleeping,passing],tables);
 const result=await runTestFileRoutesV1(root,routes,{bail:false,json:true,language:"en"},{policy:shortPolicy,tables,notice:()=>{}});
 expect(result.files.map(row=>row.status)).toEqual(["failed","passed"]);
 expect(result.files[0]!.error).toContain("deadline");
 expect(result.status).toBe("failed");
},120000);
