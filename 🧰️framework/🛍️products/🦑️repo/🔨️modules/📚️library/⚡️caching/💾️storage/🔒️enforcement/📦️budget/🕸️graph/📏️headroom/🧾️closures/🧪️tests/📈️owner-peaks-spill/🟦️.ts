/** 📈️ Contract: the learned owner peaks of the graph writer (`np/owner-peaks.json`, replaced through `owner-peaks.json.<pid>.tmp`) live beside the inference data of a selection or context and are a process-private spill like a row log: the closure proof neither reads nor retires them and never refuses the whole store with "Unknown closure reference publication" because of them (the refusal that stopped `repo show project` at 15:07 UTC). */
import {expect,test} from "bun:test";
import ts from "typescript";
const closuresSource=await Bun.file(new URL("../../🏷️names/🟦️.ts",import.meta.url)).text();

const tree=ts.createSourceFile("closures.ts",String(closuresSource),ts.ScriptTarget.Latest,true,ts.ScriptKind.TS);
const declared=(name:string)=>tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text===name)!;
const load=Function(ts.transpileModule(declared("role").getText(tree)+"\n"+declared("spill").getText(tree),{compilerOptions:{target:ts.ScriptTarget.ES2022,module:ts.ModuleKind.None}}).outputText+";return{role,spill}")() as{role:(name:string)=>string;spill:(name:string)=>boolean};
const hash="8d8f21d7".repeat(8);

test("the exact refusal: the owner peaks file is no closure role and is a spill in every writer directory, also while it is replaced",()=>{
 for(const base of ["inference/selections/"+hash+"/np/","inference/contexts/"+hash+"/np/","inference/np/"])for(const file of ["owner-peaks.json","owner-peaks.json.38258.tmp"]){
  expect(()=>load.role(base+file),base+file).toThrow("Unknown closure reference publication: "+base+file);
  expect(load.spill(base+file),base+file).toBe(true);
 }
});

test("nothing near the owner peaks file is a spill",()=>{
 for(const name of ["inference/selections/"+hash+"/np/owner-peaks.json.bak","inference/selections/"+hash+"/np/owner-peaks.jsonx","inference/selections/"+hash+"/np/owner-peaks.json.x.tmp","inference/selections/"+hash+"/np/sub/owner-peaks.json","inference/selections/short/np/owner-peaks.json","np/owner-peaks.json","inference/selections/"+hash+"/ob/owner-peaks.json","inference/selections/"+hash+"/np/Owner-Peaks.json"])expect(load.spill(name),name).toBe(false);
});

test("the row log spill of the earlier law still holds",()=>{
 expect(load.spill("inference/selections/"+hash+"/rl/rl-38258-mwm347au-0.rows")).toBe(true);
 expect(load.spill("inference/selections/"+hash+"/rl/rl-38258-mwm347au-0.rows.tmp")).toBe(false);
});

test("a crashed directory stamp writer leaves a root temporary that is a spill while the stamps file itself is not",()=>{
 expect(load.spill("directory-stamps.json.3097351.tmp")).toBe(true);
 for(const name of ["directory-stamps.json","directory-stamps.json.tmp","directory-stamps.json.x.tmp","sub/directory-stamps.json.1.tmp","directory-stamps.json.1.tmp.bak"])expect(load.spill(name),name).toBe(false);
});

test("the remote exchange root memos and their temporaries are spills while near names are not",()=>{
 for(const stem of ["generations.closures.json","generations.closures-sent.json","generations.neutral.json"])for(const name of [stem,stem+".4242.tmp"]){
  expect(()=>load.role(name),name).toThrow("Unknown closure reference publication: "+name);
  expect(load.spill(name),name).toBe(true);
 }
 for(const name of ["generations.json","generations.neutral.json.bak","generations.other.json","sub/generations.neutral.json","generations.neutral.json.x.tmp"])expect(load.spill(name),name).toBe(false);
});

test("files of the stores that share the graph image are spills and near names are not",()=>{
 const h=hash,s24="0123456789abcdef01234567";
 for(const name of ["inference/ce/e/"+h+".json.br","inference/selections/"+h+"/ce/travel/n-"+h,"inference/contexts/"+h+"/ce/collected","part-ids/x/rows.dict","task-keys/e5/"+h+".json","generations/a/"+h+".json.br","generations/a/"+h+".json.br.fb538d2b6de055bd.tmp","generations/c/"+s24+".json","selected/"+h+".admission.json"]){
  expect(()=>load.role(name),name).toThrow("Unknown closure reference publication: "+name);
  expect(load.spill(name),name).toBe(true);
 }
 for(const name of ["ce/e/"+h,"task-keys/e5x/"+h+".json","task-keys/e5/"+h+".json.bak","generations/a/"+h+".json","generations/c/"+s24+".json.tmp","selected/"+h+".admission.json.bak","selected/sub/"+h+".admission.json","x/derived-nodes/n/"+h,"derived-nodes"])expect(load.spill(name),name).toBe(false);
});
