import{existsSync,lstatSync,readFileSync,readdirSync}from"node:fs";
import{dirname,join,relative,resolve,sep}from"node:path";
import{validateJsonSchemaSubset}from"../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import type{ProcessPlatformCauseV1}from"./🖥️platforms/🟦️.ts";
import schema from"./🧬️schema/🔣️.json"with{type:"json"};
import table from"./🔣️.json"with{type:"json"};
import{refuseProcessV1}from"../../📖️texts/🟦️.ts";

/** 🗂️ Names one suite of the process module's route table: the files and trees it runs, its budget, its typecheck, the capabilities its route authors for every child birth and where its scratch directory lives (a per-run sandbox outside the checkout, or one inside the checkout for tests that read checkout-relative sources), the hosts it can run on (any other host reports it skipped), the host-bound facts that keep a host out and the registered tools whose pinned binary its laws receive. */
export type ProcessTestSuiteV1=Readonly<{id:string;budgetMs:number;typecheck:"none"|"strict"|"strict-indexed";files?:readonly string[];trees?:readonly string[];typecheckFiles?:readonly string[];function?:Readonly<{module:string;export:string}>;testNamePattern?:string;parallel?:number;capabilities?:Readonly<{artifacts?:true;command?:string}>;scratch?:"sandbox"|"checkout";fileLaunchers?:Readonly<Record<string,string>>;platforms?:readonly("darwin"|"linux"|"win32")[];platformCauses?:readonly ProcessPlatformCauseV1[];tools?:readonly string[]}>;
/** 📋️ Carries the closed route table after schema admission. */
export type ProcessTestTableV1=Readonly<{version:1;suites:readonly ProcessTestSuiteV1[];codeSuites:readonly string[];helpers:readonly string[]}>;
const protocol=schema.definitions.Protocol.const,importers=protocol.importPatterns.map(pattern=>new RegExp(pattern,"u"));
/** 📐️ The closed protocol of the route table: how test files are discovered and how much output one suite may carry. */
export const processTestProtocolV1=protocol;

/** 📖️ Reads the route table, refusing documents that break the schema or repeat a suite identifier. */
export function readProcessTestTableV1(value:unknown=table):ProcessTestTableV1{
 if(validateJsonSchemaSubset(schema.definitions.Table,value,schema).length)throw refuseProcessV1("invalid","testing.route-table-invalid");
 const row=value as ProcessTestTableV1,ids=[...row.suites.map(suite=>suite.id),...row.codeSuites];
 if(new Set(ids).size!==ids.length)throw refuseProcessV1("invalid","testing.route-suite-duplicated");
 return row;
}

/** 📂️ Lists every test file below a module root in sorted module-relative form: a source below a `🧪️tests` directory that imports the test API; links are never followed. */
export function discoverProcessTestFilesV1(root:string):readonly string[]{
 const found:string[]=[],walk=(directory:string,inside:boolean):void=>{
  for(const name of readdirSync(directory).sort()){
   const path=join(directory,name),status=lstatSync(path);
   if(status.isSymbolicLink())continue;
   if(status.isDirectory()){if(!protocol.skippedDirectories.includes(name))walk(path,inside||name===protocol.testDirectory);continue;}
   if(inside&&name.endsWith(protocol.extension)&&importers.some(importer=>importer.test(readFileSync(path,"utf8"))))found.push(relative(root,path).split(sep).join("/"));
  }
 };
 walk(resolve(root),false);
 return found;
}

/** 🧭️ Routes every discovered file to its owners: the suites without a name filter that name it explicitly, else the suites owning the deepest tree above it, plus every name-filtered suite that names it; a declared child program belongs to nobody. */
export function routeProcessTestFilesV1(discovered:readonly string[],declared:ProcessTestTableV1=readProcessTestTableV1()):ReadonlyMap<string,readonly ProcessTestSuiteV1[]>{
 const helpers=new Set(declared.helpers),owners=new Map<string,readonly ProcessTestSuiteV1[]>(),trimmed=(tree:string)=>tree.replace(/\/$/u,"");
 for(const file of discovered){
  if(helpers.has(file))continue;
  let chosen=declared.suites.filter(suite=>suite.testNamePattern===undefined&&suite.files?.includes(file));
  if(!chosen.length){
   let depth=-1;
   for(const suite of declared.suites)for(const tree of suite.trees??[]){
    const prefix=trimmed(tree)+"/";if(!file.startsWith(prefix))continue;
    if(prefix.length>depth){depth=prefix.length;chosen=[suite];}else if(prefix.length===depth&&!chosen.includes(suite))chosen.push(suite);
   }
  }
  const all=[...chosen,...declared.suites.filter(suite=>suite.testNamePattern!==undefined&&suite.files?.includes(file))];
  if(all.length)owners.set(file,all);
 }
 return owners;
}

/** 🖥️ Tells whether a suite can run on a host: a suite without a platform tag runs everywhere, a tagged one only on the hosts it names. */
export function processTestSuiteRunsOnV1(suite:ProcessTestSuiteV1,platform:string=process.platform):boolean{return suite.platforms===undefined||(suite.platforms as readonly string[]).includes(platform);}

/** 🧮️ Expands one suite to its sorted unique files: the files it owns among the discovered ones plus its explicit files outside the module. */
export function expandProcessTestSuiteV1(suite:ProcessTestSuiteV1,discovered:readonly string[],declared:ProcessTestTableV1=readProcessTestTableV1()):readonly string[]{
 const owned=[...routeProcessTestFilesV1(discovered,declared)].filter(([,owners])=>owners.includes(suite)).map(([file])=>file),known=new Set(discovered);
 return[...new Set([...owned,...(suite.files??[]).filter(file=>!known.has(file))])].sort();
}

/** ⚖️ Compares the route table with the files of the module: a discovered file no suite runs, a file two suites both run entirely, a file only name-filtered suites run, a declared file that does not exist, and a helper that is not a discovered file. */
export function processTestCoverageV1(root:string,declared:ProcessTestTableV1=readProcessTestTableV1(),discovered:readonly string[]=discoverProcessTestFilesV1(root)):Readonly<{unrouted:readonly string[];duplicated:readonly string[];filtered:readonly string[];missing:readonly string[];staleHelpers:readonly string[]}>{
 const owners=routeProcessTestFilesV1(discovered,declared),helpers=new Set(declared.helpers),found=new Set(discovered),entire=(file:string)=>(owners.get(file)??[]).filter(suite=>suite.testNamePattern===undefined),missing=declared.suites.flatMap(suite=>(suite.files??[]).filter(file=>!found.has(file)&&!existsSync(resolve(root,file))).map(file=>suite.id+":"+file));
 return Object.freeze({unrouted:discovered.filter(file=>!owners.has(file)&&!helpers.has(file)),duplicated:[...owners.keys()].filter(file=>entire(file).length>1).map(file=>file+" -> "+entire(file).map(suite=>suite.id).join(",")),filtered:[...owners.keys()].filter(file=>!entire(file).length),missing,staleHelpers:declared.helpers.filter(file=>!found.has(file))});
}

/** 🚫️ Lists every test file that imports another test file for its side effects: one process receives one Original birth, so a route that is an import chain lets the files collide instead of giving each its own process. */
export function aggregatedProcessTestFilesV1(root:string,discovered:readonly string[]):readonly string[]{
 const known=new Set(discovered),rows:string[]=[];
 for(const file of discovered)for(const match of readFileSync(resolve(root,file),"utf8").matchAll(/(?:^|[;{}])[ \t]*import\s*["']([^"']+)["']/gmu)){
  if(!match[1]!.startsWith("."))continue;
  const target=relative(root,resolve(root,dirname(file),match[1]!)).split(sep).join("/");
  if(known.has(target))rows.push(file+" -> "+target);
 }
 return rows;
}
