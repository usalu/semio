/** 🗂️ Tests for build suites. */
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{existsSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join,resolve}from"node:path";
import{captureOwnedCommandV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️owned-execution/🟦️.ts";
import{testScratchRootV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🎛️execution/🟦️.ts";
import{testPythonV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{aggregatedProcessTestFilesV1,discoverProcessTestFilesV1,expandProcessTestSuiteV1,readProcessTestTableV1,type ProcessTestSuiteV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🗂️suites/🟦️.ts";
import processSchema from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🗂️suites/🧬️schema/🔣️json/🟦️.ts";
import{buildTestAggregationV1,buildTestCoverageV1,buildTestForeignRoutesV1,buildTestRootV1,readBuildTestTableV1}from"../🟦️.ts";
import{cachingTestRootV1,readCachingTestTableV1}from"../../../⚡️caching/🗂️suites/🟦️.ts";
import table from"../🔣️.json"with{type:"json"};
import callerSchema from"../../🧬️schema/📥️caller.json"with{type:"json"};
import{withBuildTestInvocationV1}from"../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};
import{declareLibraryBuildTestEnvelopeV1}from"../../../🧪️testing/🪪️native-birth/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"build-route-table"});

const library=resolve(buildTestRootV1,".."),processRoot=resolve(library,"../../../../🔨️modules/🏃️process"),cachingRoot=join(library,"⚡️caching"),processSuites=join(processRoot,"🧪️testing/🗂️suites"),coverageOracle=join(processSuites,"🧪️tests/🐍️oracle/🐍️.py"),schemaPath=join(processSuites,"🧬️schema/🔣️.json"),oracle=join(import.meta.dir,"🐍️oracle/🐍️.py"),fixturePath=join(import.meta.dir,"🧫️fixtures/🔣️.json"),project=join(cachingRoot,"📋️project.json"),suitesDirectory="🏗️build/🗂️suites";
const checkout=((start:string):string=>{for(let directory=start;;directory=dirname(directory)){if(existsSync(join(directory,"MODULE.bazel")))return directory;if(dirname(directory)===directory)throw Error("Checkout root required");}})(buildTestRootV1);
const python=():string=>{const found=testPythonV1()??Bun.which("python3")??Bun.which("python");if(!found)throw Error("Independent Python oracle required");return found;};
const sorted=(rows:readonly string[]):string[]=>[...rows].sort();
const ask=async(script:string,args:readonly string[]):Promise<unknown>=>JSON.parse(await captureOwnedCommandV1(python(),[script,...args],buildTestRootV1,{output:{maximumBytes:4194304,maximumLines:16}}));
const withFile=async<T>(value:unknown,use:(path:string)=>Promise<T>):Promise<T>=>{const scratch=mkdtempSync(join(testScratchRootV1("build-suites"),"table-")),path=join(scratch,"value.json");try{writeFileSync(path,JSON.stringify(value));return await use(path);}finally{rmSync(scratch,{recursive:true,force:true});}};
const spawns=(source:string):boolean=>fixture.spawnTokens.some(token=>source.includes(token));
const levelOf=(suite:ProcessTestSuiteV1):string|undefined=>Object.entries(fixture.levels).find(([,budget])=>budget===suite.budgetMs)?.[0];
const read=(path:string):unknown=>JSON.parse(readFileSync(path,"utf8"));

test("the build route table is a closed document that Ajv and the module validator both admit",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(processSchema);
 expect(ajv.validate({$ref:processSchema.$id+"#/definitions/Table"},table),JSON.stringify(ajv.errors)).toBe(true);
 expect(readBuildTestTableV1().suites.length).toBe(table.suites.length);
 expect(()=>readBuildTestTableV1({...table,suites:[...table.suites,table.suites[0]]})).toThrow();
});

test("every test file of the build library belongs to exactly one suite, agreeing with the independent Python oracle",async()=>{
 const declared=readBuildTestTableV1(),found=discoverProcessTestFilesV1(buildTestRootV1),coverage=buildTestCoverageV1(declared);
 expect({...coverage}).toEqual({unrouted:[],duplicated:[],filtered:[],missing:[],staleHelpers:[]});
 const independent=await withFile(table,path=>ask(coverageOracle,[buildTestRootV1,path,schemaPath]))as Record<string,string[]>;
 expect(sorted(independent.discovered!)).toEqual(sorted(found));
 for(const key of Object.keys(coverage))expect(sorted(independent[key]!),key).toEqual(sorted(coverage[key as keyof typeof coverage]));
});

test("no build test file imports another test file of the library, so each file receives its own process birth",async()=>{
 const found=discoverProcessTestFilesV1(buildTestRootV1),independent=await withFile(table,path=>ask(coverageOracle,[buildTestRootV1,path,schemaPath]))as Record<string,string[]>,across=await ask(oracle,["aggregation",library,buildTestRootV1,schemaPath])as string[];
 expect(aggregatedProcessTestFilesV1(buildTestRootV1,found)).toEqual([]);
 expect(independent.aggregated).toEqual([]);
 expect(buildTestAggregationV1()).toEqual([]);
 expect(sorted(across)).toEqual([...buildTestAggregationV1()]);
});

test("no other route table of the library runs a file below the build root and no suite identifier is shared",()=>{
 const caching=read(join(cachingRoot,"🗂️suites/🔣️.json")),process_=read(join(processSuites,"🔣️.json")),own=readBuildTestTableV1(),ids=(rows:{suites:readonly{id:string}[];codeSuites:readonly string[]}):string[]=>[...rows.suites.map(suite=>suite.id),...rows.codeSuites];
 const foreign={caching:{root:cachingRoot,table:readProcessTestTableV1(caching)},process:{root:processRoot,table:readProcessTestTableV1(process_)}};
 expect(buildTestForeignRoutesV1(foreign)).toEqual([]);
 const taken=new Set([...ids(foreign.caching.table),...ids(foreign.process.table)]);
 expect(ids(own).filter(id=>taken.has(id))).toEqual([]);
});

test("every suite has exactly one project target and every build route target names a declared suite",()=>{
 const declared=readBuildTestTableV1(),known=new Set(declared.suites.map(suite=>suite.id)),targets=(read(project)as{targets:Record<string,{options?:{command?:string;cwd?:string}}>}).targets,pattern=/^repo \.\/📜️script\.ts test (\S+)$/u,routed=new Map<string,string[]>();
 for(const[name,target]of Object.entries(targets)){
  const match=pattern.exec(target.options?.command??"");
  if(match&&target.options?.cwd?.endsWith(suitesDirectory)===true)routed.set(match[1]!,[...(routed.get(match[1]!)??[]),name]);
 }
 expect(sorted([...known].filter(id=>!routed.has(id)))).toEqual([]);
 expect(sorted([...routed.keys()].filter(id=>!known.has(id)))).toEqual([]);
 for(const[id,names]of routed)expect(names,id).toEqual(["test-"+id]);
 for(const row of Object.values(targets))if(row.options?.cwd?.endsWith(suitesDirectory)===true)expect(existsSync(join(checkout,row.options.cwd,"📜️script.ts")),row.options.cwd).toBe(true);
});

test("no source aggregates test files outside the canonical per-file runner, agreeing with the independent oracle",async()=>{
 const rows=buildTestAggregationV1(true),independent=await ask(oracle,["pure-aggregators",library,buildTestRootV1,schemaPath])as string[];
 expect(rows.map(row=>row.split(" -> ")[0]!).filter((file,index,all)=>all.indexOf(file)===index)).toEqual([...fixture.pureAggregators]);
 expect(sorted(independent)).toEqual([...rows]);
});

test("budget classes follow the independent oracle on every vector and cover every process-spawning file of the build library",async()=>{
 const base=mkdtempSync(join(testScratchRootV1("build-suites"),"vectors-"));
 try{
  const names=fixture.vectors.map((row,index)=>{const name="vector"+index+".ts";writeFileSync(join(base,name),row.source);return name;});
  const independent=await withFile(names,async path=>JSON.parse(await captureOwnedCommandV1(python(),[oracle,"classes",base,path,fixturePath],base,{output:{maximumBytes:65536,maximumLines:16}}))as string[]);
  expect(sorted(independent)).toEqual(sorted(names.filter((_,index)=>fixture.vectors[index]!.spawns)));
  fixture.vectors.forEach(row=>expect(spawns(row.source),row.name).toBe(row.spawns));
 }finally{rmSync(base,{recursive:true,force:true});}
 const declared=readBuildTestTableV1(),found=discoverProcessTestFilesV1(buildTestRootV1),rows=declared.suites.map(suite=>({suite,files:expandProcessTestSuiteV1(suite,found,declared)}));
 for(const{suite}of rows)expect(levelOf(suite),suite.id+" budget "+suite.budgetMs).toBeDefined();
 const all=rows.flatMap(row=>row.files),spawning=new Set(await withFile(all,path=>ask(oracle,["classes",buildTestRootV1,path,fixturePath]))as string[]);
 expect(sorted(all.filter(file=>spawns(readFileSync(join(buildTestRootV1,file),"utf8"))))).toEqual(sorted([...spawning]));
 for(const row of rows){const starts=row.files.filter(file=>spawning.has(file));if(starts.length)expect(row.suite.budgetMs,row.suite.id+" starts processes in "+starts.join(", ")).toBeGreaterThanOrEqual(fixture.levels.quick);}
});

/** 🪪️ The birth a library test file declares names the suite that routes the file, the scratch that suite row declares and an existing project target `test-<suite>`: the declaration cannot drift from the route table. */
test("every declared library test birth names the suite that routes its file, that suite's scratch and an existing target",()=>{
 const rows=discoverProcessTestFilesV1(library).flatMap(file=>[...readFileSync(resolve(library,file),"utf8").matchAll(/declareLibraryBuildTestEnvelopeV1\(\{suite:"([^"]+)"(?:,scratch:"(sandbox|checkout)")?\}\)/gu)].map(match=>({file,suite:match[1]!,scratch:match[2]??"sandbox"})));
 expect(rows.length).toBeGreaterThanOrEqual(15);
 const routed=new Map<string,{files:Set<string>;scratch:string}>();
 for(const[root,declared]of[[buildTestRootV1,readBuildTestTableV1()],[cachingTestRootV1,readCachingTestTableV1()]]as const){const found=discoverProcessTestFilesV1(root);for(const suite of declared.suites)routed.set(suite.id,{files:new Set(expandProcessTestSuiteV1(suite,found,declared).map(file=>resolve(root,file))),scratch:suite.scratch??"sandbox"});}
 const targets=Object.keys((JSON.parse(readFileSync(join(cachingRoot,"📋️project.json"),"utf8")) as{targets:Record<string,unknown>}).targets);
 for(const row of rows){const suite=routed.get(row.suite);expect(suite,row.file+" declares unknown suite "+row.suite).toBeDefined();expect(suite!.files.has(resolve(library,row.file)),row.file+" is not routed by "+row.suite).toBe(true);expect(row.scratch,row.file).toBe(suite!.scratch);expect(targets,row.suite).toContain("test-"+row.suite);}
});
/** 🎫️ A bare library test file is born with a genuine native build caller grant and a sandboxed artifact directory: the capabilities validate against the caller schema with Ajv and name this checkout, a repo run command and no artifacts capability. */
test("a declared library test birth carries a genuine native build caller grant and an existing artifact directory",()=>withBuildTestInvocationV1(async original=>{
 const facts=original.capabilities as Record<string,unknown>,directory=process.env.SEMIO_TEST_ARTIFACT_DIR;
 expect(new Ajv({strict:true}).validate(callerSchema,facts)).toBe(true);
 expect(facts.workspaceRoot).toBe(realpathSync(process.cwd()));expect(["prototype","release"]).toContain(String(facts.buildMode));expect((facts.arguments as string[]).slice(0,2)).toEqual(["repo","run"]);expect(facts.launches).toContainEqual(facts.arguments);expect("artifacts" in facts).toBe(false);
 expect(directory!==undefined&&existsSync(directory)).toBe(true);
}),60000);
