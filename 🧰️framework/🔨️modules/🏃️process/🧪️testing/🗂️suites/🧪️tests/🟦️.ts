import{test,expect}from"bun:test";
import Ajv from"ajv";
import{mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{dirname,join,resolve}from"node:path";
import{captureOwnedCommandV1}from"../../../🎛️owned-execution/🟦️.ts";
import{testScratchRootV1}from"../../🏖️sandbox/🟦️.ts";
import{testPythonV1}from"../../🧰️toolchain/🟦️.ts";
import{aggregatedProcessTestFilesV1,discoverProcessTestFilesV1,processTestCoverageV1,processTestProtocolV1,readProcessTestTableV1,type ProcessTestTableV1}from"../🟦️.ts";
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
