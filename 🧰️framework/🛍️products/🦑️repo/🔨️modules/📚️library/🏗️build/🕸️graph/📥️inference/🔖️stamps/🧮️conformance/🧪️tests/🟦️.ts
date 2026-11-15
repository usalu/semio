/** 🧮️ Contract: the stamp of a derived inferrer is the digest of what its code answers on a fixed corpus. A body-only edit of any file of its closure moves no stamp and therefore re-infers nothing; an edit that changes one corpus answer, one declared schema or one direct import moves the stamp; a record that is missing, damaged, forged or too slow is never trusted and the inferrer falls back to the content facts of its files. The canonical digests, the stamps and the import lists are recomputed by an independent Python implementation, and the corpus of the real providers is checked against the frozen expectations and the third-party oracles of the existing fixtures. */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {chmodSync,copyFileSync,existsSync,mkdirSync,mkdtempSync,readFileSync,readdirSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,resolve} from "node:path";
import {fileURLToPath} from "node:url";
import Ajv from "ajv";
import {minimatch} from "minimatch";
import schema from "../🧬️schema/🔣️.json";
import {CONFORMANCE_CORPUS_V1,CONFORMANCE_TEXTS_V1,DECLARED_KINDS_V1,auditReportTextV1,canonicalJsonV1,closureFilesV1,conformanceTextV1,coverageViolationsV1,directImportsV1,evaluateProviderV1,readRecordV1,recordHoldsV1,resolveStampV1,runConformanceAuditV1,sampleNamesV1,sha256V1,stampOfV1} from "../🟨️.mjs";
import {RUNNERS_V1,runInputPrecisionV1,runPackageManagersV1} from "../🏃️runners/🟨️.mjs";
import {captureObservedSourceInputsV1,observationsHold,observedInferenceStampsV1,registerInferenceStampsV1} from "../../../../../👁️observation/🟨️.mjs";

const python=testPythonV1(),here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"🐍️.py"),library=resolve(here,"../../../../../.."),precisionTests=join(library,"🏗️build/📥️inputs/🎯️precision/🧪️tests");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"conformance-"));
const ask=(...args:string[])=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,...args]).stdout.toString());
const json=(path:string)=>JSON.parse(readFileSync(path,"utf8"));
const fixtures={main:json(join(precisionTests,"🔣️.json")),dialects:json(join(precisionTests,"🔣️glob-dialects/🔣️.json")),router:json(join(precisionTests,"🛤️router/🔣️.json"))};

test("the corpus manifest, the schema texts and the records are closed under the schema",()=>{
 const ajv=new Ajv({strict:false}),root={definitions:schema.definitions};
 expect(ajv.validate({...root,$ref:"#/definitions/Corpus"},CONFORMANCE_CORPUS_V1),JSON.stringify(ajv.errors)).toBe(true);
 expect(ajv.validate({...root,$ref:"#/definitions/Corpus"},{...CONFORMANCE_CORPUS_V1,surprise:1})).toBe(false);
 const texts=CONFORMANCE_TEXTS_V1;
 expect(texts.languages).toEqual(["en","de"]);
 for(const[key,row]of Object.entries(texts.texts)as[string,Record<string,string>][]){
  expect(Object.keys(row).sort(),key).toEqual(["de","en"]);
  expect(new Set(row.en!.match(/\{\w+\}/gu)),key).toEqual(new Set(row.de!.match(/\{\w+\}/gu)));
 }
 expect(conformanceTextV1("audit.clean","de",{checked:3})).toContain("3");
 expect(()=>conformanceTextV1("audit.clean","fr")).toThrow();
});

test("canonical digests, stamps and import lists equal the independent Python implementation",async()=>{
 const directory=scratch();
 try{
  const result=await runPackageManagersV1({roles:json(join(precisionTests,"🔌️manager-files/🔣️.json")),policy:json(join(library,"⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/🔣️policy.json"))}),outputs=result.cases,file=join(directory,"outputs.json");
  writeFileSync(file,JSON.stringify(outputs));
  expect(ask("digest",file)).toEqual(Object.fromEntries(Object.entries(outputs).map(([key,value])=>[key,sha256V1(canonicalJsonV1(value))])));
  const parts={cases:{a:"0".repeat(64)},schema:"1".repeat(64),imports:"2".repeat(64)},partsFile=join(directory,"parts.json");
  writeFileSync(partsFile,JSON.stringify(parts));
  expect(Bun.spawnSync([python!,"-I",oracle,"stamp","semio.graph-inferrer.x/1",partsFile]).stdout.toString().trim()).toBe(stampOfV1("semio.graph-inferrer.x/1",parts));
  for(const source of[join(library,"🏗️build/📥️inputs/🎯️precision/🟦️.ts"),join(library,"⚡️caching/🚀️bootstrap/📦️dependencies/🔌️managers/📚️catalog/🟦️.ts"),join(here,"..","🟨️.mjs")])expect(ask("imports",source)).toEqual(directImportsV1(readFileSync(source,"utf8")));
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the input precision corpus reproduces the frozen derivations, the router fixture, minimatch and the refusals of the existing fixtures",async()=>{
 const{cases,covered}=await runInputPrecisionV1(fixtures),value=(name:string)=>(cases[name] as any)?.value??cases[name];
 fixtures.main.expect.derive.forEach((expected:unknown,index:number)=>expect(value(`derive/${index}`),`derive ${index}`).toEqual(expected));
 expect(fixtures.main.expect.unselected).toEqual(fixtures.main.unselected.map((_:unknown,index:number)=>cases[`unselected/${index}`]));
 fixtures.main.globs.forEach((row:{pattern:string;path:string;match:boolean},index:number)=>{expect(cases[`glob/${index}`]).toBe(row.match);expect(minimatch(row.path,row.pattern)).toBe(row.match);});
 for(const row of fixtures.router.derived)expect(value(`router/${row.word}`).files).toEqual(row.files);
 fixtures.router.branches.forEach((row:{files:string[]},index:number)=>expect(value(`router-branch/${index}`).files).toEqual(row.files));
 for(const row of fixtures.router.refused)expect((cases[`router-refused/${row.word}`] as any).error).toMatch(new RegExp(row.message,"u"));
 fixtures.main.refusals.forEach((row:{message:string},index:number)=>expect((cases[`refusal/${index}`] as any).error,`refusal ${index}`).toMatch(new RegExp(row.message,"iu")));
 expect(Object.keys(cases).length).toBeGreaterThan(100);
 expect(covered.read).toContain("router");
});

test("the corpus of every real provider covers every kind its schema and tables declare, and a missing kind is reported",async()=>{
 for(const id of Object.keys(CONFORMANCE_CORPUS_V1.providers)){
  const evaluated=await evaluateProviderV1({id,root:library,runners:RUNNERS_V1}),declared=DECLARED_KINDS_V1[id as keyof typeof DECLARED_KINDS_V1](library);
  expect(coverageViolationsV1(declared,evaluated.covered),id).toEqual([]);
  expect(Object.values(declared).flat().length,id).toBeGreaterThan(4);
 }
 expect(coverageViolationsV1({read:["globs","newest"],manager:["bun"]},{read:["globs"]})).toEqual(["read:newest","manager:bun"]);
 const declared=DECLARED_KINDS_V1["input-precision"](library);
 expect(declared.read).toEqual(["cargo","dispatch","globs","input","kinds","manager","owner","profile","program","router","script"]);
});

test("a corpus evaluates in well under two seconds and twice to the same digests",async()=>{
 for(const id of Object.keys(CONFORMANCE_CORPUS_V1.providers)){
  const started=performance.now(),first=await evaluateProviderV1({id,root:library,runners:RUNNERS_V1}),elapsed=performance.now()-started,second=await evaluateProviderV1({id,root:library,runners:RUNNERS_V1});
  expect(elapsed,id).toBeLessThan(2000);
  expect(second,id).toEqual(first);
 }
});

const toy=(root:string)=>{
 mkdirSync(join(root,"code"),{recursive:true});
 writeFileSync(join(root,"code/a.mjs"),'import {double} from "./b.mjs";\nexport const a = double;\n');
 writeFileSync(join(root,"code/b.mjs"),"function double(n) {\n  return n * 2;\n}\n");
 writeFileSync(join(root,"code/schema.json"),JSON.stringify({kinds:["x","y"],version:1}));
 writeFileSync(join(root,"code/fixture.json"),JSON.stringify({numbers:[1,2,3]}));
 let evaluations=0;
 const provider={entries:["code/a.mjs"],schemas:["code/schema.json"],fixtures:{numbers:"code/fixture.json"},runner:"toy"},
  runners={toy:async(fixture:any)=>{evaluations++;const double=new Function(readFileSync(join(root,"code/b.mjs"),"utf8")+"; return double;")();return{cases:{doubled:fixture.numbers.numbers.map(double)},covered:{kinds:["x","y"]}};}};
 const resolveIt=(extra:Record<string,unknown>={})=>resolveStampV1({id:"toy",declared:"semio.graph-inferrer.toy/1",root,directory:join(root,"store"),runners,provider,...extra}) as Promise<any>;
 return{provider,runners,resolveIt,get evaluations(){return evaluations;}};
};
const edit=(root:string,path:string,change:(text:string)=>string)=>writeFileSync(join(root,path),change(readFileSync(join(root,path),"utf8")));

test("a body-only edit of any file of the closure moves no stamp, an edit that changes one answer, one schema or one import moves it",async()=>{
 const root=scratch(),world=toy(root);
 try{
  const base=await world.resolveIt();
  expect(base.reason).toBeUndefined();expect(base.status).toBe("computed");expect(base.stamp).toMatch(/^semio\.graph-inferrer\.toy\/1\+[0-9a-f]{32}$/u);
  edit(root,"code/b.mjs",text=>"// a comment\n"+text+"// trailing comment\n");
  edit(root,"code/a.mjs",text=>text.replace("export const a","\n\nexport   const a")+"\n// more\n");
  edit(root,"code/schema.json",text=>JSON.stringify(JSON.parse(text),null,4));
  const bodyOnly=await world.resolveIt();
  expect(bodyOnly.key,"the closure changed, so the corpus is evaluated again").not.toBe(base.key);
  expect(bodyOnly.status).toBe("computed");
  expect(bodyOnly.stamp,"and answers the same").toBe(base.stamp);
  edit(root,"code/b.mjs",text=>text.replace("n * 2","n * 3"));
  const behaviour=await world.resolveIt();
  expect(behaviour.stamp).not.toBe(base.stamp);
  edit(root,"code/b.mjs",text=>text.replace("n * 3","n * 2"));
  expect((await world.resolveIt()).stamp,"restoring the code restores the stamp").toBe(base.stamp);
  edit(root,"code/schema.json",text=>JSON.stringify({...JSON.parse(text),kinds:["x","y","z"]}));
  const structural=await world.resolveIt();
  expect(structural.stamp).not.toBe(base.stamp);
  edit(root,"code/schema.json",text=>JSON.stringify({...JSON.parse(text),kinds:["x","y"]}));
  edit(root,"code/a.mjs",text=>'import "node:path";\n'+text);
  expect((await world.resolveIt()).stamp,"an added import counts").not.toBe(base.stamp);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a fixture graph of 40 projects stays valid after a body-only edit and goes stale as a whole after a behaviour edit; an unregistered stamp is stale (safe direction)",async()=>{
 const root=scratch(),world=toy(root);
 try{
  const base=await world.resolveIt();
  registerInferenceStampsV1([["inferrer:toy",base.stamp]]);
  const projects:any[][]=[];
  for(let index=0;index<40;index++)projects.push((await captureObservedSourceInputsV1(async()=>{observedInferenceStampsV1(["inferrer:toy"]);},undefined,false)).observations as any[]);
  const stale=async()=>{let count=0;for(const rows of projects)if(!await observationsHold(rows as never))count++;return count;};
  expect(await stale()).toBe(0);
  edit(root,"code/b.mjs",text=>"// comment\n"+text);
  const bodyOnly=await world.resolveIt();
  registerInferenceStampsV1([["inferrer:toy",bodyOnly.stamp]]);
  expect(await stale(),"re-inferred projects after a body-only edit").toBe(0);
  edit(root,"code/b.mjs",text=>text.replace("n * 2","n + n + 1"));
  registerInferenceStampsV1([["inferrer:toy",(await world.resolveIt()).stamp]]);
  expect(await stale(),"re-inferred projects after an edit that changes a corpus answer").toBe(40);
  registerInferenceStampsV1([["inferrer:toy",base.stamp]]);
  expect(await stale()).toBe(0);
  const missing=await world.resolveIt({runners:{toy:async()=>{throw Error("broken");}},directory:join(root,"empty")});
  expect(missing.status).toBe("unavailable");expect(missing.stamp).toBeUndefined();
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the record is kept per closure key, shared through the fleet only after it checks, and never trusted when damaged, forged, slow or unwritable",async()=>{
 const root=scratch(),world=toy(root);
 try{
  const first=await world.resolveIt(),count=world.evaluations;
  expect(first.status).toBe("computed");
  const again=await world.resolveIt();
  expect(again.status).toBe("stored");expect(again.stamp).toBe(first.stamp);expect(world.evaluations).toBe(count);
  const path=join(root,"store","toy",first.key+".json");
  expect(existsSync(path)).toBe(true);
  writeFileSync(path,"{damaged");
  expect((await world.resolveIt()).status).toBe("computed");
  const forged={...JSON.parse(readFileSync(path,"utf8")),stamp:"semio.graph-inferrer.toy/1+"+"f".repeat(32)};
  writeFileSync(path,JSON.stringify(forged));
  expect(readRecordV1(join(root,"store"),"toy",first.key,"semio.graph-inferrer.toy/1")).toBeUndefined();
  expect((await world.resolveIt()).stamp).toBe(first.stamp);
  const good=JSON.parse(readFileSync(path,"utf8")),before=world.evaluations;
  const fromFleet=await world.resolveIt({directory:join(root,"store2"),fleet:{get:async()=>good}});
  expect(fromFleet.status).toBe("fleet");expect(world.evaluations).toBe(before);
  expect(existsSync(join(root,"store2","toy",first.key+".json"))).toBe(true);
  const refused=await world.resolveIt({directory:join(root,"store3"),fleet:{get:async()=>forged}});
  expect(refused.status).toBe("computed");expect(refused.stamp).toBe(first.stamp);
  const crashing=await world.resolveIt({directory:join(root,"store4"),fleet:{get:async()=>{throw Error("offline");},put:async()=>{throw Error("offline");}}});
  expect(crashing.status).toBe("computed");
  const slow=await world.resolveIt({directory:join(root,"store5"),runners:{toy:()=>new Promise(()=>{})},timeoutMilliseconds:50});
  expect(slow.status).toBe("unavailable");expect(slow.reason).toContain("exceeded");
  const controller=new AbortController();controller.abort();
  expect((await world.resolveIt({directory:join(root,"store6"),runners:{toy:()=>new Promise(()=>{})},signal:controller.signal,timeoutMilliseconds:5000})).status).toBe("unavailable");
  writeFileSync(join(root,"blocker"),"file");
  const unwritable=await world.resolveIt({directory:join(root,"blocker","below")});
  expect(unwritable.reason).toBeUndefined();expect(unwritable.status).toBe("computed");
  expect(recordHoldsV1({...good,parts:{...good.parts,cases:{}}},{id:"toy",key:first.key,declared:"semio.graph-inferrer.toy/1"})).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the closure follows relative imports across directories, ignores bare ones and keeps the files stable",()=>{
 const root=scratch();
 try{
  mkdirSync(join(root,"m/n"),{recursive:true});
  writeFileSync(join(root,"m/a.ts"),'import x from "./n/b.ts";\nimport {y} from "../c";\nimport z from "./d.json" with {type:"json"};\nimport fs from "node:fs";\nconst w = await import("./n/🟦️.ts");\n');
  writeFileSync(join(root,"m/n/b.ts"),'export * from "../e.mjs";\n');writeFileSync(join(root,"c.ts"),"export const y = 1;\n");writeFileSync(join(root,"m/d.json"),"{}");writeFileSync(join(root,"m/e.mjs"),"export const e = 1;\n");writeFileSync(join(root,"m/n/🟦️.ts"),"export {};\n");
  expect(closureFilesV1(["m/a.ts"],root)).toEqual(["c.ts","m/a.ts","m/d.json","m/e.mjs","m/n/b.ts","m/n/🟦️.ts"]);
  expect(directImportsV1(readFileSync(join(root,"m/a.ts"),"utf8"))).toEqual(["../c","./d.json","./n/b.ts","./n/🟦️.ts","node:fs"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("the audit compares a sample of fresh inferences with the stored records, reports every difference and failure, shows progress and stops on cancellation",async()=>{
 const names=Array.from({length:50},(_,index)=>"project-"+index);
 const sample=sampleNamesV1(names,10,"seed-1");
 expect(sample).toHaveLength(10);expect(sampleNamesV1(names,10,"seed-1")).toEqual(sample);expect(sampleNamesV1(names,10,"seed-2")).not.toEqual(sample);
 const seen:number[]=[];
 const clean=await runConformanceAuditV1({projects:sample,infer:async(name:string)=>({name}),stored:async(name:string)=>({name}),progress:row=>seen.push(row.checked)});
 expect(clean).toEqual({version:1,checked:10,same:10,differing:[],failed:[]});expect(seen).toHaveLength(10);
 expect(auditReportTextV1(clean,"en")).toContain("10 projects");
 const dirty=await runConformanceAuditV1({projects:sample,infer:async(name:string)=>{if(name===sample[1])throw Error("cannot infer");return{name,rows:name===sample[0]?2:1};},stored:async(name:string)=>({name,rows:1})});
 expect(dirty.differing.map(row=>row.project)).toEqual([sample[0]]);expect(dirty.failed).toEqual([{project:sample[1],message:"cannot infer"}]);expect(dirty.same).toBe(8);
 expect(new Ajv({strict:false}).validate({definitions:schema.definitions,$ref:"#/definitions/AuditReport"},dirty)).toBe(true);
 expect(auditReportTextV1(dirty,"de")).toContain("1 von 10");
 const controller=new AbortController();
 await expect(runConformanceAuditV1({projects:sample,infer:async()=>{controller.abort();return 1;},stored:async()=>1,signal:controller.signal})).rejects.toThrow();
});

test("a body-only edit of each closure file of the real providers re-infers nothing and a corpus-changing edit re-infers every holder",async()=>{
 const workspace=resolve(library,"../../../../.."),copy=scratch(),target=join(copy,"ws");
 try{
  const lines=[] as string[];
  const copyFile=(path:string)=>{mkdirSync(dirname(join(target,path)),{recursive:true});copyFileSync(join(workspace,path),join(target,path));};
  const rel=(path:string)=>join("🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",path);
  const files=new Set<string>();
  for(const[id,provider]of Object.entries(CONFORMANCE_CORPUS_V1.providers))for(const file of[...closureFilesV1(provider.entries,library),...provider.schemas,...Object.values(provider.fixtures)])files.add(file.startsWith("..")?resolve(library,file).slice(workspace.length+1):rel(file));
  const own="🏗️build/🕸️graph/📥️inference/🔖️stamps/🧮️conformance/";
  for(const file of["🟨️.mjs","🔣️.json","🏃️runners/🟨️.mjs","🧬️schema/🔣️.json"])files.add(rel(own+file));
  for(const file of files)copyFile(file);
  const script=join(copy,"evaluate.mjs");
  writeFileSync(script,`const root=${JSON.stringify(join(target,"🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library"))},base=root+"/"+${JSON.stringify(own)};const core=await import(base+"🟨️.mjs"),{RUNNERS_V1}=await import(base+"🏃️runners/🟨️.mjs");const out={};for(const id of Object.keys(core.CONFORMANCE_CORPUS_V1.providers)){const parts=await core.evaluateProviderV1({id,root,runners:RUNNERS_V1});out[id]=core.stampOfV1("x/1",{cases:parts.cases,schema:parts.schema,imports:parts.imports});}console.log(JSON.stringify(out));`);
  const stamps=()=>{const run=Bun.spawnSync([process.execPath,script],{cwd:copy,env:{...process.env}});if(run.exitCode!==0)throw Error(run.stderr.toString().slice(0,2000));return JSON.parse(run.stdout.toString());};
  const base=stamps();
  expect(Object.keys(base).sort()).toEqual(Object.keys(CONFORMANCE_CORPUS_V1.providers).sort());
  const owned=[...files].filter(file=>!file.includes("🧮️conformance/")&&!file.includes("🧪️tests/")&&/\.(?:ts|json)$/u.test(file)&&file.includes("📚️library/")&&(file.includes("🎯️precision/")||file.includes("🔌️managers/")));
  expect(owned.length).toBeGreaterThanOrEqual(19);
  for(const file of owned){
   const original=readFileSync(join(target,file),"utf8");
   writeFileSync(join(target,file),file.endsWith(".json")?JSON.stringify(JSON.parse(original),null,4)+"\n":"// [DEBUG] body-only edit\n"+original+"\n// trailing comment\n");
   const after=stamps();
   lines.push(`${JSON.stringify(after)===JSON.stringify(base)?"same":"MOVED"} ${file.split("/").slice(-3).join("/")}`);
   writeFileSync(join(target,file),original);
  }
  expect(lines.filter(line=>line.startsWith("MOVED")),lines.join("\n")).toEqual([]);
  const edits:[string,string,string,string][]=[
   ["input-precision","🎯️precision/🟦️.ts","export function packageNameV1(manifest: string): string | undefined {","export function packageNameV1(manifest: string): string | undefined {\n  if (manifest.includes(\"crate-a\")) return \"changed\";"],
   ["package-managers","🔌️managers/🍞️bun/🔣️.json","\"bun.lockb\"","\"bun.lockb2\""],
  ];
  for(const[id,fragment,from,to]of edits){
   const file=owned.find(candidate=>candidate.endsWith(fragment))!,original=readFileSync(join(target,file),"utf8");
   expect(original.includes(from),`${fragment} anchor`).toBe(true);
   writeFileSync(join(target,file),original.replace(from,to));
   const after=stamps();
   expect(after[id],`${fragment} changes ${id}`).not.toBe(base[id]);
   for(const other of Object.keys(base).filter(key=>key!==id))expect(after[other],`${fragment} leaves ${other}`).toBe(base[other]);
   writeFileSync(join(target,file),original);
  }
 }finally{rmSync(copy,{recursive:true,force:true});}
},300000);
