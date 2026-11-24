/** 🧱️ Contract: a target that declares module inputs, module globs or runtime entries is cut at the declared tools like any other target: an edit inside a tool, or of any tool file no owner imports, leaves the entry of every such target fresh; an edit of an owner's own file moves that owner alone; the module a target declares as an input that is itself the entry of a tool is named by one reference to the tool and never walked, and the files it keys on are never fewer than the walk.
 * @see ../🛑️tool-boundary
 * @see ../../../🟨️.mjs `toolCoverDeclaredV1` */
import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import corpus from "../🔣️.json" with {type:"json"};

setDefaultTimeout(240000);
const inference=await import("../../../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../../🟨️.mjs") as any;
const designationSchema=join(import.meta.dir,"../../../../../../../🏃️process/🔌️providers/🧬️schema/🔣️.json"),roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const workspace=()=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"wb-law-")));roots.push(root);for(const[path,text]of Object.entries(corpus.files as Record<string,string>)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}mkdirSync(dirname(join(root,corpus.schemaCopy)),{recursive:true});copyFileSync(designationSchema,join(root,corpus.schemaCopy));return root;};
const cache=(root:string,late:boolean)=>scriptInputs.createScriptInputCache({workspaceRoot:root,lateClosures:late,scriptProviderDesignation:join(root,corpus.designation)});
const driver="⚡️caching/🦀️cargo/📜️script.ts",execution="lib/🦀️cargo/🏃️execution/📜️script.ts";
const infer=<T>(body:()=>Promise<T>)=>observation.observeInferenceSnapshotV1(body) as Promise<T>;
const owner=(root:string,name:string,extra:Record<string,unknown>)=>{mkdirSync(join(root,name),{recursive:true});writeFileSync(join(root,name,"own.ts"),"export const own = 1;\n");writeFileSync(join(root,name,"📜️script.ts"),'import "../'+driver+'";\nimport "./own.ts";\nexport const script = 1;\n');return{options:{cwd:name,command:"repo ./📜️script.ts"},...extra};};
const variants:Record<string,Record<string,unknown>>={
 literal:{inputs:["{workspaceRoot}/"+execution]},
 globs:{metadata:{semio:{closure:{modules:["{projectRoot}/*.ts"]}}}},
 runtime:{metadata:{semio:{runtimeSourceEntries:["{projectRoot}/own.ts"]}}},
};
const read=async(root:string,target:unknown,projectRoot:string)=>infer(async()=>{const api=scriptInputs.closureApiV1(cache(root,true),root),handle=scriptInputs.closureHandleOfV1(await api.nativeTargetCommandInputs(target,await api.nativeCommandInputs(),projectRoot)),result=await api.resolve([[handle.key,handle.recipe]]);return{rows:result.rows as any[],derived:result.derived as {row:any;support:any[]}[]};});
const holds=async(root:string,entry:{rows:any[];derived:{row:any;support:any[]}[]})=>infer(async()=>{
 for(const row of [...entry.rows,...entry.derived.flatMap(item=>item.support)])if(!observation.observationRowFreshV1(row))return false;
 for(const{row}of entry.derived){const asked=await scriptInputs.deriveClosureFactV1(row,cache(root,false));if(asked.fact!==row.fact)return false;}
 return true;
});
const toolRows=(root:string,entry:{rows:any[];derived:{row:any;support:any[]}[]})=>[...entry.rows,...entry.derived.flatMap(item=>item.support)].filter(row=>["analysis","file","field"].includes(row.kind)).map(row=>String(row.path)).filter(path=>path.startsWith(join(root,"lib")+"/")&&!path.startsWith(join(root,"lib/providers"))||path.startsWith(join(root,"⚡️caching")));
const edit=(root:string,path:string,change:(text:string)=>string)=>writeFileSync(join(root,path),change(readFileSync(join(root,path),"utf8")));
const fleet=async(root:string)=>{const found=new Map<string,Awaited<ReturnType<typeof read>>>();for(const[name,extra]of Object.entries(variants))found.set(name,await read(root,owner(root,"app-"+name,extra),"app-"+name));return found;};

test("no content row of a declared tool is a row of a target that declares modules, globs or runtime entries",async()=>{
 const root=workspace(),entries=await fleet(root);
 for(const[name,entry]of entries){expect(toolRows(root,entry),name).toEqual([]);expect(entry.rows.length,name).toBeGreaterThan(0);}
});

test("an edit inside a declared tool, one that adds a file and an import, and the touch of a tool file no owner imports leave every such entry fresh",async()=>{
 const root=workspace(),before=await fleet(root);
 for(const[name,entry]of before)expect(await holds(root,entry),name).toBe(true);
 writeFileSync(join(root,"lib/added.ts"),"export const added = 1;\n");
 edit(root,"lib/util.ts",text=>'import "./added.ts";\n'+text);
 edit(root,"lib/run.ts",text=>text+"// progress reporting\n");
 edit(root,"lib/chain/c5.ts",text=>text+"// unrelated\n");
 edit(root,execution,text=>text+"// toolchain\n");
 let stale=0;for(const[,entry]of before)if(!await holds(root,entry))stale++;
 expect(stale).toBe(0);
});

test("an edit of the own file of one owner stales that owner alone",async()=>{
 const root=workspace(),before=await fleet(root);
 writeFileSync(join(root,"app-literal/added.ts"),"export const added = 1;\n");
 edit(root,"app-literal/own.ts",text=>'import "./added.ts";\n'+text);
 const moved=[];for(const[name,entry]of before)if(!await holds(root,entry))moved.push(name);
 expect(moved).toEqual(["literal"]);
});

test("a module input that is the entry of a tool is named by one reference, held by no file of the target, and keyed on at least the tool files",async()=>{
 const root=workspace(),target=owner(root,"app-literal",variants.literal!);
 const list=await infer(()=>scriptInputs.nativeTargetCommandInputs(target,root,undefined,cache(root,false),"app-literal"));
 const names=list.filter((item:unknown)=>item!==null&&typeof item==="object").map((item:any)=>item.input);
 expect(names).toContain("toolClosure:cargo-execution:native");
 expect(names).toContain("toolClosure:cargo-command:native");
 expect(list).not.toContain("{workspaceRoot}/"+execution);
 const expanded=await infer(()=>scriptInputs.expandedCommandInputsV1(list,root,cache(root,false)));
 expect(expanded).toContain("{workspaceRoot}/"+execution);
 expect(expanded).toContain("{workspaceRoot}/lib/util.ts");
 expect(expanded).toContain("{workspaceRoot}/app-literal/own.ts");
});
