/** 🧠️ Contract: the closed owner admission of a selected read and the facts of each of its owners are remembered on the machine, keyed by the scope and the admission format (the hash of the discovery and admission modules and schemas only, never of the whole graph implementation); the admission is served again only while every fact it was derived from holds, a memo that no longer holds still lends the facts of the owners whose own rows hold, and a corrupt, foreign, other-format or oversized memo is a miss, never an error.
 * @see ../🟦️.ts */
import{expect,test}from"bun:test";
import{mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{tmpdir}from"node:os";
import{join}from"node:path";
import ts from"typescript";
import{ownersOfAdmissionMemoV1,packAdmissionMemoV1,readAdmissionMemoV1,writeAdmissionMemoV1,type AdmissionMemoV1}from"../🟦️.ts";
import{graphAdmissionFormatHashV1,graphAdmissionFormatUrlsV1,graphSelfUrlsV1}from"../../../🔐️authority/🟦️.ts";

const format="a".repeat(64);
const row=(path:string,version=1)=>({kind:"file",path,fact:{version}});
const record=(scope="s".repeat(8)):AdmissionMemoV1=>packAdmissionMemoV1({format,scope,admission:{owners:["o"],files:["o/📋️project.json"],targets:{},inputs:{},catalogProviders:[]},observations:[row("/w/o/📋️project.json"),row("/w/shared.json")],owners:[{suffix:"o\0[[],[]]",facts:{closure:1},rows:[row("/w/o/📋️project.json"),row("/w/o/src.ts")]}]});
const accept=(value:AdmissionMemoV1)=>value.format===format;
const directory=()=>mkdtempSync(join(tmpdir(),"admission-memo-"));
const LOAD=fileURLToPath(new URL("../../🟦️.ts",import.meta.url)),tree=ts.createSourceFile("load.ts",readFileSync(process.env.R140_LOAD_SOURCE??LOAD,"utf8"),ts.ScriptTarget.Latest,true);
const extract=(name:string)=>{const node=tree.statements.find((each):each is ts.FunctionDeclaration=>ts.isFunctionDeclaration(each)&&each.name?.text===name);if(!node)throw Error("Actual function required: "+name);return ts.transpileModule(ts.createPrinter().printNode(ts.EmitHint.Unspecified,node,tree).replace("export ",""),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText;};

test("a written memo is served for its format and scope and its observations are the caller's to hold",async()=>{
 const root=directory();try{
  const path=join(root,"s.admission.json");await writeAdmissionMemoV1(path,record(),1<<20);
  const hit=await readAdmissionMemoV1(path,"s".repeat(8),accept);
  expect(hit?.admission).toEqual(record().admission);expect(hit?.observations).toEqual(record().observations);
  expect(ownersOfAdmissionMemoV1(hit!)).toEqual([{suffix:"o\0[[],[]]",facts:{closure:1},rows:[row("/w/o/📋️project.json"),row("/w/o/src.ts")]}]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the rows of an owner are stored once as indexes, an owner row the capture did not retain is appended",()=>{
 const memo=record();
 expect(memo.observations.length).toBe(3);expect(memo.owners[0]!.rows).toEqual([0,2]);
});

test("a memo of another admission format, another scope, a corrupt file or a missing file is a miss",async()=>{
 const root=directory();try{
  const path=join(root,"s.admission.json");await writeAdmissionMemoV1(path,record(),1<<20);
  expect(await readAdmissionMemoV1(path,"t".repeat(8),accept)).toBeUndefined();
  expect(await readAdmissionMemoV1(path,"s".repeat(8),value=>value.format==="b".repeat(64))).toBeUndefined();
  writeFileSync(path,readFileSync(path,"utf8").slice(0,40));
  expect(await readAdmissionMemoV1(path,"s".repeat(8),accept)).toBeUndefined();
  writeFileSync(path,JSON.stringify({...record(),admission:{owners:"not a list"}}));
  expect(await readAdmissionMemoV1(path,"s".repeat(8),accept)).toBeUndefined();
  writeFileSync(path,JSON.stringify({...record(),version:1}));
  expect(await readAdmissionMemoV1(path,"s".repeat(8),accept)).toBeUndefined();
  expect(await readAdmissionMemoV1(join(root,"absent.json"),"s".repeat(8),accept)).toBeUndefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a memo above the limit is not written and a write leaves no temporary file behind",async()=>{
 const root=directory();try{
  const path=join(root,"s.admission.json");
  expect(await writeAdmissionMemoV1(path,record(),50)).toBe(false);expect(readdirSync(root)).toEqual([]);
  expect(await writeAdmissionMemoV1(path,record(),1<<20)).toBe(true);expect(readdirSync(root)).toEqual(["s.admission.json"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the admission format changes with a discovery or admission module and keeps with every other graph module",()=>{
 const hashes=(changed:string[])=>graphAdmissionFormatHashV1(path=>(changed.some(name=>path.endsWith(name))?"changed:":"same:")+path);
 const base=hashes([]),formatPaths=graphAdmissionFormatUrlsV1().map(url=>fileURLToPath(url));
 const unrelated=[...graphSelfUrlsV1().map(url=>fileURLToPath(url)),fileURLToPath(new URL("../../../../👁️observation/🟨️.mjs",import.meta.url))].filter(path=>!formatPaths.includes(path));
 expect(unrelated.length).toBeGreaterThan(5);
 for(const path of unrelated)expect(hashes([path]),path).toBe(base);
 for(const path of formatPaths)expect(hashes([path]),path).not.toBe(base);
 expect(formatPaths.some(path=>path.includes("owner-discovery"))).toBe(true);expect(formatPaths.some(path=>path.endsWith("admission.json"))).toBe(true);
});

test("a memo of another admission format misses after a format change and hits after an unrelated one",async()=>{
 const root=directory();try{
  const path=join(root,"s.admission.json"),at=(changed:string[])=>graphAdmissionFormatHashV1(file=>(changed.some(name=>file.endsWith(name))?"changed:":"same:")+file);
  const [unrelated]=graphSelfUrlsV1().map(url=>fileURLToPath(url)),[format0]=graphAdmissionFormatUrlsV1().map(url=>fileURLToPath(url)),remembered=at([]);
  await writeAdmissionMemoV1(path,{...record(),format:remembered},1<<20);
  expect(await readAdmissionMemoV1(path,"s".repeat(8),value=>value.format===at([unrelated!]))).toBeDefined();
  expect(await readAdmissionMemoV1(path,"s".repeat(8),value=>value.format===at([format0!]))).toBeUndefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("facts lent by a memo that no longer holds re-derive only the owner whose own row changed",async()=>{
 const owners=Array.from({length:6},(_,index)=>"owner-"+index),tree=new Map<string,number>(owners.map(name=>["/w/"+name+"/src",1]));
 const memoOf=new Map<string,{facts:unknown;rows:readonly unknown[]}>(),limits={ownerFactsMemoEntries:100};
 const remember=Function("ownerFactsMemoV1","GRAPH_LIMITS_V1",extract("rememberOwnerFactsV1")+";"+extract("observeOwnerFactsMemoizedV1")+";return observeOwnerFactsMemoizedV1;")(memoOf,limits) as (input:unknown)=>Promise<unknown>;
 const hold=async(rows:readonly {path:string;fact:{version:number}}[])=>rows.every(each=>tree.get(each.path)===each.fact.version);
 const derived:string[]=[];
 const observation={captureObservedSourceInputsV1:async(observe:()=>Promise<unknown>)=>{const value=await observe();return{value,observations:[row("/w/"+(value as {name:string}).name+"/src",tree.get("/w/"+(value as {name:string}).name+"/src"))]};},retainObservedRowsV1:()=>{}};
 const ask=(name:string)=>remember({key:"scope\0f\0"+name,hold,observation,observe:async()=>{derived.push(name);return{name};}});
 for(const name of owners)await ask(name);
 expect(derived).toEqual(owners);
 const memo=packAdmissionMemoV1({format,scope:"s".repeat(8),admission:{owners,files:[],targets:{},inputs:{},catalogProviders:[]},observations:[],owners:[...memoOf].map(([key,entry])=>({suffix:key.slice("scope\0f\0".length),facts:entry.facts,rows:entry.rows}))});
 memoOf.clear();derived.length=0;
 for(const owner of ownersOfAdmissionMemoV1(memo))memoOf.set("scope\0f\0"+owner.suffix,{facts:owner.facts,rows:owner.rows});
 tree.set("/w/owner-3/src",2);
 for(const name of owners)await ask(name);
 expect(derived).toEqual(["owner-3"]);
});

test("the selected inventory checks the registry scope, then asks the memo, derives only on a miss and remembers what it derived, keyed by the admission format",()=>{
 const source=readFileSync(process.env.R140_LOAD_SOURCE??join(import.meta.dir,"..","..","🟦️.ts"),"utf8"),inventory=source.indexOf("const inventory=async()=>");
 const scope=source.indexOf("buildGraphScopeKeyV1(options.selection,readBuildOwnerRegistryV1(",inventory),read=source.indexOf("readAdmissionMemoV1(memoPath,scope",inventory),derive=source.indexOf("captureObservedSourceInputsV1(",read),write=source.indexOf("writeAdmissionMemoV1(memoPath,",derive);
 expect(inventory).toBeGreaterThan(0);expect(scope).toBeGreaterThan(inventory);expect(read).toBeGreaterThan(scope);expect(derive).toBeGreaterThan(read);expect(write).toBeGreaterThan(derive);
 expect(source).toContain("remembered===undefined&&selected&&scope&&captured!==undefined");
 expect(source).toContain("value.format===loadedGraphAdmissionFormatV1");
 expect(source.slice(inventory,write)).not.toContain("loadedGraphModuleAuthorityV1");
});
