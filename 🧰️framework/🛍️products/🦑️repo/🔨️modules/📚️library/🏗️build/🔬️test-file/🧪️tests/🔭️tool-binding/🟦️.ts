import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve,sep} from "node:path";
import {discoverProcessTestFilesV1,expandProcessTestSuiteV1,processTestSuiteToolsV1,readProcessTestTableV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🗂️suites/🟦️.ts";
import {requireTestToolV1,testToolsUsedByFilesV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import toolchain from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🔣️.json" with {type:"json"};
import {readTestFileTablesV1,routeTestFilesV1} from "../../🟦️.ts";
import policy from "../../🔣️.json" with {type:"json"};

const root=(()=>{let directory=import.meta.dir;while(!existsSync(join(directory,"MODULE.bazel"))&&dirname(directory)!==directory)directory=dirname(directory);return directory;})();
const oracle=(files:readonly string[]):Record<string,string[]>=>{
 const result=spawnSync(requireTestToolV1("python"),["-I",join(import.meta.dir,"🐍️.py")],{encoding:"utf8",input:JSON.stringify({files,hints:toolchain.usage.hints,closure:toolchain.usage.closure}),maxBuffer:1<<28});
 expect(result.status,result.stderr).toBe(0);
 return JSON.parse(result.stdout);
};

test("a suite that does not declare python still binds it for every file that names the Python oracle, directly or through a test helper",()=>{
 const scratch=mkdtempSync(join(tmpdir(),"semio-tool-binding-"));
 try{
  const write=(file:string,text:string)=>{mkdirSync(dirname(join(scratch,file)),{recursive:true});writeFileSync(join(scratch,file),text);};
  write("🧪️tests/🔧️helper.ts","import{requireTestToolV1}from\"x\";export const run=()=>requireTestToolV1(\"python\");");
  write("🧪️tests/🅰️direct/🟦️.ts","import{testPythonV1}from\"x\";import{test}from\"bun:test\";test(\"a\",()=>testPythonV1());");
  write("🧪️tests/🅱️indirect/🟦️.ts","import{run}from\"../🔧️helper.ts\";import{test}from\"bun:test\";test(\"b\",()=>run());");
  write("🧪️tests/🅾️plain/🟦️.ts","import{test}from\"bun:test\";test(\"c\",()=>{});");
  const suite={id:"fixture",budgetMs:15000,typecheck:"none" as const,trees:["🧪️tests"]},table=readProcessTestTableV1({version:1,suites:[suite],codeSuites:[],helpers:[]});
  const bind=(file:string)=>processTestSuiteToolsV1(suite,scratch,[file]);
  expect(bind("🧪️tests/🅰️direct/🟦️.ts")).toEqual(["python"]);
  expect(bind("🧪️tests/🅱️indirect/🟦️.ts")).toEqual(["python"]);
  expect(bind("🧪️tests/🅾️plain/🟦️.ts")).toEqual([]);
  expect(testToolsUsedByFilesV1([join(scratch,"🧪️tests/🅱️indirect/🟦️.ts")])).toEqual(["python"]);
  expect(expandProcessTestSuiteV1(suite,discoverProcessTestFilesV1(scratch),table).length).toBe(3);
  const declared=processTestSuiteToolsV1({...suite,tools:["node"]},scratch,["🧪️tests/🅰️direct/🟦️.ts"]);expect(declared).toEqual(["node","python"]);
 }finally{rmSync(scratch,{recursive:true,force:true});}
});

test("every test file of every route table that names the Python oracle is routed with python bound, as an independent Python scan finds",()=>{
 const tables=readTestFileTablesV1(root,policy),failures:string[]=[];
 let oracleFiles=0;
 for(const owner of policy.tables){
  const table=tables.get(owner.id)!,tableRoot=resolve(root,owner.root),discovered=discoverProcessTestFilesV1(tableRoot).filter(file=>!table.helpers.includes(file)),absolute=discovered.map(file=>join(tableRoot,file)),expected=oracle(absolute);
  const routes=routeTestFilesV1(root,discovered.map(file=>owner.root+"/"+file),tables,policy);
  for(const route of routes){
   if(route.mode!=="suite")continue;
   const module=relative(tableRoot,resolve(root,route.file)).split(sep).join("/"),wants=expected[join(tableRoot,module)]!.includes("python");
   if(wants)oracleFiles+=1;
   if(wants&&!route.python)failures.push(owner.id+":"+route.suite+" lacks python for "+module);
   const suite=table.suites.find(row=>row.id===route.suite)!;
   if(wants&&!processTestSuiteToolsV1(suite,tableRoot,[module]).includes("python"))failures.push(owner.id+":"+route.suite+" binds no python for "+module);
  }
 }
 expect(failures).toEqual([]);
 expect(oracleFiles).toBeGreaterThan(0);
});

test("no runner reads the declared tools of a suite directly: the binding is always derived from the files",()=>{
 const library=resolve(import.meta.dir,"../../../..");
 for(const file of["⚡️caching/📜️script.ts","🏗️build/🗂️suites/📜️script.ts","🏗️build/🔬️test-file/🟦️.ts"])expect(readFileSync(join(library,file),"utf8"),file).not.toMatch(/\.tools\?\.includes\(/u);
});
