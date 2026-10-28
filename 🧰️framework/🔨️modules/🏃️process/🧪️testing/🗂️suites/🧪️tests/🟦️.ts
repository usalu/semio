import{test,expect}from"bun:test";
import Ajv from"ajv";
import{mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join,resolve,sep}from"node:path";
import{captureOwnedCommandV1}from"../../../🎛️owned-execution/🟦️.ts";
import{testScratchRootV1}from"../../🏖️sandbox/🟦️.ts";
import{testPythonV1}from"../../🧰️toolchain/🟦️.ts";
import{aggregatedProcessTestFilesV1,discoverProcessTestFilesV1,processTestCoverageV1,processTestProtocolV1,processTestSuiteRunsOnV1,readProcessTestTableV1,type ProcessTestTableV1}from"../🟦️.ts";
import{runProcessTestSuiteV1}from"../🏃️run/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import table from"../🔣️.json"with{type:"json"};
import fixture from"./🧫️fixtures/🔣️.json"with{type:"json"};

const owner=resolve(import.meta.dir,"../../.."),oracle=join(import.meta.dir,"🐍️oracle/🐍️.py"),schemaPath=join(import.meta.dir,"../🧬️schema/🔣️.json");
const python=():string=>{const found=testPythonV1()??Bun.which("python3")??Bun.which("python");if(!found)throw Error("Independent Python oracle required");return found;};
const ask=async(root:string,value:unknown):Promise<Record<string,string[]>>=>{
 const scratch=mkdtempSync(join(testScratchRootV1("suites"),"table-")),path=join(scratch,"table.json");
 try{writeFileSync(path,JSON.stringify(value));return JSON.parse(await captureOwnedCommandV1(python(),[oracle,root,path,schemaPath],owner,{output:{maximumBytes:4194304,maximumLines:16}}));}finally{rmSync(scratch,{recursive:true,force:true});}
};
const sorted=(rows:readonly string[]):string[]=>[...rows].sort();
const asRecord=(coverage:ReturnType<typeof processTestCoverageV1>,discovered:readonly string[])=>({discovered:sorted(discovered),unrouted:sorted(coverage.unrouted),duplicated:sorted(coverage.duplicated),filtered:sorted(coverage.filtered),missing:sorted(coverage.missing),staleHelpers:sorted(coverage.staleHelpers)});

test("the route table is a closed document that Ajv and the module validator both admit",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema);
 expect(ajv.validate({$ref:schema.$id+"#/definitions/Table"},table),JSON.stringify(ajv.errors)).toBe(true);
 expect(readProcessTestTableV1(table).suites.length).toBe(table.suites.length);
 expect(()=>readProcessTestTableV1({...table,suites:[...table.suites,table.suites[0]]})).toThrow();
 expect(()=>readProcessTestTableV1({...table,version:2})).toThrow();
 for(const scratch of["sandbox","checkout"]as const)expect(readProcessTestTableV1({...table,suites:[{...table.suites[0],scratch}as never]}).suites[0]!.scratch).toBe(scratch);
 expect(()=>readProcessTestTableV1({...table,suites:[{...table.suites[0],scratch:"elsewhere"}as never]})).toThrow();
 expect(processTestProtocolV1.checkoutScratchDirectory).toBe(JSON.parse(readFileSync(schemaPath,"utf8")).definitions.Protocol.const.checkoutScratchDirectory);
});

test("discovery and routing agree with the independent Python oracle on every vector",async()=>{
 const base=mkdtempSync(join(testScratchRootV1("suites"),"tree-"));
 try{
  for(const[path,text]of Object.entries(fixture.tree)){mkdirSync(dirname(join(base,path)),{recursive:true});writeFileSync(join(base,path),text);}
  for(const row of fixture.cases){
   const declared=readProcessTestTableV1(row.table),found=discoverProcessTestFilesV1(base),actual=asRecord(processTestCoverageV1(base,declared,found),found),independent=await ask(base,row.table);
   expect(actual,row.name).toEqual(Object.fromEntries(Object.entries(row.expected).map(([key,rows])=>[key,sorted(rows as string[])]))as typeof actual);
   for(const key of Object.keys(row.expected))expect(sorted(independent[key]!),row.name+" "+key).toEqual(sorted((row.expected as Record<string,string[]>)[key]!));
  }
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("every test file of the process module belongs to exactly one suite and every suite has a project target",async()=>{
 const declared=readProcessTestTableV1(),found=discoverProcessTestFilesV1(owner),coverage=processTestCoverageV1(owner,declared,found),actual=asRecord(coverage,found),independent=await ask(owner,declared as unknown as ProcessTestTableV1);
 expect({...actual,discovered:[]}).toEqual({discovered:[],unrouted:[],duplicated:[],filtered:[],missing:[],staleHelpers:[]});
 for(const key of Object.keys(actual))expect(sorted(independent[key]!),key).toEqual(actual[key as keyof typeof actual]);
 const project=JSON.parse(readFileSync(join(owner,"📋️project.json"),"utf8")) as{targets:Record<string,{options?:{command?:string}}>},pattern=/📜️script\.ts test (\S+)$/u,routed=new Map<string,string[]>();
 for(const[name,target]of Object.entries(project.targets)){const match=pattern.exec(target.options?.command??"");if(match)routed.set(match[1]!,[...(routed.get(match[1]!)??[]),name]);}
 const known=new Set([...declared.suites.map(suite=>suite.id),...declared.codeSuites]);
 expect(sorted([...known].filter(id=>!routed.has(id)))).toEqual([]);
 expect(sorted([...routed.keys()].filter(id=>!known.has(id)))).toEqual([]);
});

test("no test file imports another test file, by vectors and across the whole module, agreeing with the Python oracle",async()=>{
 const base=mkdtempSync(join(testScratchRootV1("suites"),"aggregation-"));
 try{
  for(const[path,text]of Object.entries(fixture.aggregation.tree)){mkdirSync(dirname(join(base,path)),{recursive:true});writeFileSync(join(base,path),text);}
  const found=discoverProcessTestFilesV1(base),empty={version:1,suites:[],codeSuites:[],helpers:[]},independent=await ask(base,empty);
  expect(sorted(aggregatedProcessTestFilesV1(base,found))).toEqual(sorted(fixture.aggregation.expected));
  expect(sorted(independent.aggregated!)).toEqual(sorted(fixture.aggregation.expected));
 }finally{rmSync(base,{recursive:true,force:true});}
 const found=discoverProcessTestFilesV1(owner),independent=await ask(owner,readProcessTestTableV1());
 expect(aggregatedProcessTestFilesV1(owner,found)).toEqual([]);
 expect(independent.aggregated).toEqual([]);
});

const hosts=["darwin","linux","win32"]as const,elsewhere=hosts.filter(host=>host!==process.platform);
const tagged=(extra:Record<string,unknown>)=>({...table,suites:[{id:"tagged",budgetMs:1000,typecheck:"none",files:["🧪️testing/🗂️suites/🧪️tests/🟦️.ts"],...extra},...table.suites]});

test("the platform tag and the tool list of a suite are closed: Ajv and the module validator admit the same documents and refuse the same malformed ones",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema);
 const admitted=[{platforms:["darwin"]},{platforms:[...hosts]},{tools:["bazel"]},{platforms:["linux"],tools:["bazel","cargo"]}],refused=[{platforms:[]},{platforms:["freebsd"]},{platforms:["darwin","darwin"]},{tools:[]},{tools:["Bazel"]},{tools:["bazel","bazel"]}];
 for(const row of admitted){expect(ajv.validate({$ref:schema.$id+"#/definitions/Table"},tagged(row)),JSON.stringify(row)+JSON.stringify(ajv.errors)).toBe(true);expect(()=>readProcessTestTableV1(tagged(row)),JSON.stringify(row)).not.toThrow();}
 for(const row of refused){expect(ajv.validate({$ref:schema.$id+"#/definitions/Table"},tagged(row)),JSON.stringify(row)).toBe(false);expect(()=>readProcessTestTableV1(tagged(row)),JSON.stringify(row)).toThrow();}
});

test("a suite runs exactly on the hosts it names, agreeing with an independent Python reading, and a suite declared for other hosts reports SKIPPED in both languages without starting a file",async()=>{
 const reading=async(suite:unknown,host:string):Promise<string>=>(await captureOwnedCommandV1(python(),["-c","import json,sys;s=json.loads(sys.argv[1]);print('run' if 'platforms' not in s or sys.argv[2] in s['platforms'] else 'skip')",JSON.stringify(suite),host],owner,{output:{maximumBytes:64,maximumLines:2}})).trim();
 for(const platforms of[undefined,["darwin"],["linux","win32"],[...hosts]])for(const host of hosts){
  const suite=readProcessTestTableV1(tagged(platforms?{platforms}:{})).suites[0]!;
  expect(processTestSuiteRunsOnV1(suite,host)?"run":"skip",JSON.stringify(platforms)+" on "+host).toBe(await reading(suite,host));
 }
 const declared=readProcessTestTableV1(tagged({platforms:elsewhere})),lines:string[]=[];
 for(const[language,word]of[["en","SKIPPED"],["de","ÜBERSPRUNGEN"]]as const){
  lines.length=0;
  await runProcessTestSuiteV1({invocation:undefined as never,root:owner,checkout:owner,table:declared,suite:"tagged",owner:"suites-law",output:"",environment:{SEMIO_LANGUAGE:language},report:line=>{lines.push(line);}});
  expect(lines.length,language).toBe(1);expect(lines[0]!.startsWith(word),language).toBe(true);expect(lines[0],language).toContain(elsewhere.join(", "));expect(lines[0],language).toContain(process.platform);
 }
});

test("a file that a route table names outside its own module is named by exactly one suite of the three tables and lies below no other table's root, agreeing with an independent Python reading",async()=>{
 const library=resolve(owner,"../../🛍️products/🦑️repo/🔨️modules/📚️library"),roots={process:owner,build:join(library,"🏗️build"),caching:join(library,"⚡️caching")}as const,rows:{table:keyof typeof roots;suite:string;file:string;filtered:boolean}[]=[];
 for(const[name,root]of Object.entries(roots)as[keyof typeof roots,string][]){
  const declared=readProcessTestTableV1(JSON.parse(readFileSync(join(root,name==="process"?"🧪️testing/🗂️suites/🔣️.json":"🗂️suites/🔣️.json"),"utf8")));
  for(const suite of declared.suites)for(const file of suite.files??[])if(file.startsWith(".."))rows.push({table:name,suite:suite.id,file:resolve(root,file),filtered:suite.testNamePattern!==undefined});
 }
 const entire=rows.filter(row=>!row.filtered),byFile=new Map<string,string[]>();
 for(const row of entire)byFile.set(row.file,[...(byFile.get(row.file)??[]),row.table+":"+row.suite]);
 const duplicated=[...byFile].filter(([,owners])=>owners.length>1).map(([file,owners])=>file+" -> "+owners.join(", ")).sort();
 const inside=entire.filter(row=>(Object.entries(roots)as[string,string][]).some(([name,root])=>name!==row.table&&(row.file===root||row.file.startsWith(root+sep)))).map(row=>row.table+":"+row.suite+" -> "+row.file).sort();
 expect({duplicated,inside}).toEqual({duplicated:[],inside:[]});
 const scratch=mkdtempSync(join(testScratchRootV1("suites"),"foreign-")),path=join(scratch,"rows.json");
 try{
  writeFileSync(path,JSON.stringify(entire.map(row=>[row.table,row.file])));
  const independent=JSON.parse(await captureOwnedCommandV1(python(),["-c","import json,sys,collections\nrows=json.load(open(sys.argv[1]))\ncount=collections.Counter(file for _,file in rows)\nprint(json.dumps(sorted(file for file,n in count.items() if n>1)))",path],owner,{output:{maximumBytes:1048576,maximumLines:4}}))as string[];
  expect(independent).toEqual([...byFile].filter(([,owners])=>owners.length>1).map(([file])=>file).sort());
  const probe=[...entire,{...entire[0]!,suite:"probe"}],again=new Map<string,number>();for(const row of probe)again.set(row.file,(again.get(row.file)??0)+1);
  expect([...again.values()].some(count=>count>1)).toBe(true);
 }finally{rmSync(scratch,{recursive:true,force:true});}
});
