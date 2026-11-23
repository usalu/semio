/** 🧠️ Contract: the closed owner admission of a selected read and the facts of each of its owners are remembered on the machine, keyed by the scope and the admission format (the hash of the discovery and admission modules and schemas only, never of the whole graph implementation); the admission is served again only while every fact it was derived from holds, a memo that no longer holds still lends the facts of the owners whose own rows hold, and a corrupt, foreign, other-format or oversized memo is a miss, never an error.
 * @see ../🟦️.ts */
import{expect,test}from"bun:test";
import{mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{tmpdir}from"node:os";
import{join}from"node:path";
import ts from"typescript";
import{Database}from"bun:sqlite";
import cases from"./🧫️cases.json";
import{ownersOfAdmissionMemoV1,packAdmissionMemoV1,readAdmissionMemoV1,writeAdmissionMemoV1,type AdmissionMemoV1}from"../🟦️.ts";
import{observeOwnerWithRetryV1}from"../../🛟️owner-isolation/🟦️.ts";
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
 const remember=Function("ownerFactsMemoV1","GRAPH_LIMITS_V1","observeOwnerWithRetryV1",extract("rememberOwnerFactsV1")+";"+extract("observeOwnerFactsMemoizedV1")+";return observeOwnerFactsMemoizedV1;")(memoOf,limits,observeOwnerWithRetryV1) as (input:unknown)=>Promise<unknown>;
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
 const source=readFileSync(process.env.R140_LOAD_SOURCE??join(import.meta.dir,"..","..","🟦️.ts"),"utf8"),inventory=source.indexOf("const inventory=async(");
 const scope=source.indexOf("ownerRegistryDigestV1(readBuildOwnerRegistryV1(",inventory),read=source.indexOf("readAdmissionMemoV1(memoPath,scope",inventory),derive=source.indexOf("captureObservedSourceInputsV1(",read),write=source.indexOf("writeAdmissionMemoV1(memoPath,",derive);
 expect(inventory).toBeGreaterThan(0);expect(scope).toBeGreaterThan(inventory);expect(read).toBeGreaterThan(scope);expect(derive).toBeGreaterThan(read);expect(write).toBeGreaterThan(derive);
 expect(source).toContain("remembered===undefined&&selected&&scope&&captured!==undefined");
 expect(source).toContain("value.format===loadedGraphAdmissionFormatV1");
 expect(source.slice(inventory,write)).not.toContain("loadedGraphModuleAuthorityV1");
});


test("only a cold owner with integer row positions borrows the corresponding observations",()=>{
 const db=new Database(":memory:");
 try{
  const oracle=db.query(`SELECT json_extract(owner.value,'$.suffix') AS suffix FROM json_each(?) owner
   WHERE owner.type='object' AND json_type(owner.value,'$.rows')='array'
   AND json_extract(owner.value,'$.suffix') NOT IN (SELECT value FROM json_each(?))
   AND NOT EXISTS (SELECT 1 FROM json_each(owner.value,'$.rows') pos WHERE pos.type!='integer' OR pos.value<0 OR pos.value>=?)
   ORDER BY owner.key`).all(JSON.stringify(cases.owners),JSON.stringify(cases.known),cases.observations.length);
  expect(oracle).toEqual(cases.expected.map(({suffix})=>({suffix})));
  let reads=0;
  const observations=new Proxy(cases.observations,{get(target,key,receiver){if(typeof key==="string"&&/^\d+$/.test(key))reads++;return Reflect.get(target,key,receiver);}});
  const memo={...record(),owners:cases.owners,observations} as unknown as AdmissionMemoV1;
  expect(ownersOfAdmissionMemoV1(memo,new Set(cases.known))).toEqual(cases.expected);
  expect(reads).toBe(3);
 }finally{db.close();}
});

test("packing serializes a row at most once and not at all while the owners hold the instances of the capture",()=>{
 let encodings=0;
 const make=(path:string)=>({toJSON(){encodings++;return {kind:"exists",path,fact:true};}}),rows=[make("a"),make("b")];
 const shared=packAdmissionMemoV1({...record(),observations:rows,owners:[{suffix:"x",facts:1,rows:[rows[1]!,rows[0]!]},{suffix:"y",facts:2,rows:[rows[0]!]}]});
 expect(shared.owners.map(owner=>owner.rows)).toEqual([[1,0],[0]]);expect(encodings).toBe(0);
 const other=packAdmissionMemoV1({...record(),observations:rows,owners:[{suffix:"x",facts:1,rows:[make("b"),make("c"),make("b")]}]});
 expect(other.owners[0]!.rows).toEqual([1,2,1]);expect(other.observations.length).toBe(3);expect(encodings).toBe(2+3);
});

test("the row index of a pack equals an independent index by text, for shared instances, equal copies and new rows",()=>{
 const universe=Array.from({length:60},(_,at)=>row("/w/f"+(at%45)+".ts",at%3)),texts=universe.map(each=>JSON.stringify(each));
 let seed=7;const next=(limit:number)=>(seed=(seed*1103515245+12345)%2147483648)%limit;
 const observations=universe.slice(0,40),owners=Array.from({length:30},(_,owner)=>({suffix:"o"+owner,facts:owner,rows:Array.from({length:25},()=>{const pick=next(universe.length);return next(3)===0?JSON.parse(JSON.stringify(universe[pick])):universe[pick]!;})}));
 const memo=packAdmissionMemoV1({...record(),observations,owners});
 const oracle=observations.map(each=>JSON.stringify(each)),index=new Map<string,number>();oracle.forEach((text,at)=>{if(!index.has(text))index.set(text,at);});
 const expected=owners.map(owner=>owner.rows.map(each=>{const text=JSON.stringify(each);let found=index.get(text);if(found===undefined){found=oracle.push(text)-1;index.set(text,found);}return found;}));
 expect(memo.owners.map(owner=>owner.rows)).toEqual(expected);expect(memo.observations.map(each=>JSON.stringify(each))).toEqual(oracle);
 for(const owner of memo.owners)for(const at of owner.rows)expect(at).toBeLessThan(memo.observations.length);
 expect(texts.length).toBe(universe.length);
});

test("a memo is written in chunks to the exact bytes of its JSON, and a memo above the limit stops writing at once",async()=>{
 const root=directory();try{
  const path=join(root,"big.admission.json"),rows=Array.from({length:4000},(_,at)=>row("/w/"+"x".repeat(900)+at,at)),memo=packAdmissionMemoV1({...record(),observations:rows,owners:[{suffix:"o",facts:{closure:"🧬".repeat(10)},rows:rows.slice(0,3000)},{suffix:"p",facts:null,rows:[]}]});
  const text=JSON.stringify(memo);expect(text.length).toBeGreaterThan(3*(1<<20));
  expect(await writeAdmissionMemoV1(path,memo,Buffer.byteLength(text))).toBe(true);
  expect(readFileSync(path,"utf8")).toBe(text);expect(JSON.parse(readFileSync(path,"utf8"))).toEqual(JSON.parse(text));
  rmSync(path);
  expect(await writeAdmissionMemoV1(path,memo,Buffer.byteLength(text)-1)).toBe(false);expect(readdirSync(root)).toEqual([]);
  expect(await writeAdmissionMemoV1(path,memo,1<<20)).toBe(false);expect(readdirSync(root)).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the owner facts a process remembers share one row instance per distinct row, so retention follows the distinct rows and not the owners",async()=>{
 const universe=Array.from({length:4000},(_,at)=>row("/w/dir/"+at+".ts",at)),owners=Array.from({length:200},(_,owner)=>"owner-"+owner);
 const memoOf=new Map<string,{facts:unknown;rows:readonly unknown[]}>(),limits={ownerFactsMemoEntries:100};
 const ask=Function("ownerFactsMemoV1","GRAPH_LIMITS_V1","observeOwnerWithRetryV1",extract("rememberOwnerFactsV1")+";"+extract("observeOwnerFactsMemoizedV1")+";return observeOwnerFactsMemoizedV1;")(memoOf,limits,observeOwnerWithRetryV1) as (input:unknown)=>Promise<unknown>;
 const{shareObservationRowsV1}=await import("../../../../👁️observation/🟨️.mjs");
 const observation={captureObservedSourceInputsV1:async(observe:()=>Promise<unknown>)=>({value:await observe(),observations:[]}),retainObservedRowsV1:()=>{},shareObservationRowsV1};
 const fresh=(owner:number)=>JSON.parse(JSON.stringify(Array.from({length:500},(_,at)=>universe[(owner*37+at)%universe.length])));
 for(const [owner,name] of owners.entries()){
  await ask({key:"scope\0f\0"+name,hold:async()=>true,observation,observe:async()=>({name}),recover:async()=>({facts:{name},rows:fresh(owner)})});
 }
 expect(memoOf.size).toBe(limits.ownerFactsMemoEntries);
 const rows=[...memoOf.values()].flatMap(entry=>entry.rows),distinct=new Set(rows);
 expect(rows.length).toBe(100*500);expect(distinct.size).toBeLessThanOrEqual(universe.length);
 const byText=new Map<string,unknown>();for(const each of rows){const text=JSON.stringify(each);expect(byText.get(text)??each).toBe(each);byText.set(text,each);}
});
