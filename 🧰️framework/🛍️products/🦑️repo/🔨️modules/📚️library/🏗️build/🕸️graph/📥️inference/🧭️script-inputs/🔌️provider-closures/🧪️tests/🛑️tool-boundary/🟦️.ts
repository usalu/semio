/** 🧊️ Contract: the closure of a command that imports a declared tool stops at the tool's entry module, so the freshness of an owner does not hold the files of the tool (the build library): an edit inside a tool, even one that adds an import or a file, leaves the `closure-graph` row (its closure digest is cut at the tool's entry) and every other row of every owner that imports the tool as it was; an edit of an owner's own script moves that owner alone; the declared stamp of an inferrer, and nothing of the bytes of its sources, moves the rows that name it. */
import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import * as esbuild from "esbuild";
import {spawnSync} from "node:child_process";
import {copyFileSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join,relative,resolve} from "node:path";
import corpus from "../🔣️.json" with {type:"json"};

setDefaultTimeout(240000);
const inference=await import("../../../../🟨️.mjs") as any;await inference.libraryBootstrap;
const observation=await import("../../../../../../👁️observation/🟨️.mjs") as any,scriptInputs=await import("../../../🟨️.mjs") as any,stamps=await import("../../../../🔖️stamps/🟨️.mjs") as any;
const designationSchema=join(import.meta.dir,"../../../../../../../🏃️process/🔌️providers/🧬️schema/🔣️.json"),roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const workspace=()=>{const root=realpathSync(mkdtempSync(join(tmpdir(),"tb-law-")));roots.push(root);for(const[path,text]of Object.entries(corpus.files as Record<string,string>)){mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);}mkdirSync(dirname(join(root,corpus.schemaCopy)),{recursive:true});copyFileSync(designationSchema,join(root,corpus.schemaCopy));return root;};
const cache=(root:string,late:boolean)=>scriptInputs.createScriptInputCache({workspaceRoot:root,lateClosures:late,scriptProviderDesignation:join(root,corpus.designation)});
const driver="⚡️caching/🦀️cargo/📜️script.ts";
const infer=<T>(body:()=>Promise<T>)=>observation.observeInferenceSnapshotV1(body) as Promise<T>;
const owner=(root:string,name:string)=>{mkdirSync(join(root,name),{recursive:true});writeFileSync(join(root,name,"own.ts"),"export const own = 1;\n");writeFileSync(join(root,name,"📜️script.ts"),'import "../'+driver+'";\nimport "./own.ts";\nexport const script = 1;\n');return{options:{cwd:name,command:"repo ./📜️script.ts"}};};
const read=async(root:string,target:unknown,projectRoot:string)=>infer(async()=>{const api=scriptInputs.closureApiV1(cache(root,true),root),handle=scriptInputs.closureHandleOfV1(await api.nativeTargetCommandInputs(target,await api.nativeCommandInputs(),projectRoot)),result=await api.resolve([[handle.key,handle.recipe]]);return{rows:result.rows as any[],derived:result.derived as {row:any;support:any[]}[]};});
const holds=async(root:string,entry:{rows:any[];derived:{row:any;support:any[]}[]})=>infer(async()=>{
 const rows=[...entry.rows,...entry.derived.flatMap(item=>item.support)];
 await observation.prepareGlobalFactsV1(rows);
 for(const row of rows)if(!observation.observationRowFreshV1(row))return false;
 for(const{row}of entry.derived){const asked=await scriptInputs.deriveClosureFactV1(row,cache(root,false));if(asked.fact!==row.fact)return false;}
 return true;
});
const toolRows=(root:string,entry:{rows:any[];derived:{row:any;support:any[]}[]})=>[...entry.rows,...entry.derived.flatMap(item=>item.support)].filter(row=>["analysis","file","field"].includes(row.kind)).map(row=>String(row.path)).filter(path=>path.startsWith(join(root,"lib")+"/")&&!path.startsWith(join(root,"lib/providers"))||path.startsWith(join(root,"⚡️caching")));
const edit=(root:string,path:string,change:(text:string)=>string)=>writeFileSync(join(root,path),change(readFileSync(join(root,path),"utf8")));

test("no content row of a declared tool is a row of an owner that imports the tool",async()=>{
 const root=workspace(),entries=[await read(root,owner(root,"app2"),"app2"),await read(root,owner(root,"app3"),"app3")];
 for(const entry of entries){expect(toolRows(root,entry)).toEqual([]);expect(entry.rows.length).toBeGreaterThan(0);}
});

test("an edit inside a declared tool, one that adds a file and an import, leaves every owner that imports the tool fresh",async()=>{
 const root=workspace(),first=owner(root,"app2"),second=owner(root,"app3"),before=[await read(root,first,"app2"),await read(root,second,"app3")];
 for(const entry of before)expect(await holds(root,entry)).toBe(true);
 writeFileSync(join(root,"lib/added.ts"),"export const added = 1;\n");
 edit(root,"lib/util.ts",text=>'import "./added.ts";\n'+text);
 edit(root,"lib/run.ts",text=>text+"// progress reporting\n");
 for(const entry of before)expect(await holds(root,entry)).toBe(true);
});

test("an edit of the own script of one owner stales that owner alone",async()=>{
 const root=workspace(),first=owner(root,"app2"),second=owner(root,"app3"),before=[await read(root,first,"app2"),await read(root,second,"app3")];
 writeFileSync(join(root,"app2/added.ts"),"export const added = 1;\n");
 edit(root,"app2/own.ts",text=>'import "./added.ts";\n'+text);
 expect(await holds(root,before[0]!)).toBe(false);
 expect(await holds(root,before[1]!)).toBe(true);
});

test("an owner closure still names the tool and the tool group still holds the files the owner no longer walks",async()=>{
 const root=workspace(),first=owner(root,"app2");
 const list=await infer(()=>scriptInputs.nativeTargetCommandInputs(first,root,undefined,cache(root,false),"app2"));
 expect(list.filter((item:unknown)=>item!==null&&typeof item==="object"&&(item as any).input==="toolClosure:cargo-command:native").length).toBe(1);
 const expanded=await infer(()=>scriptInputs.expandedCommandInputsV1(list,root,cache(root,false)));
 expect(expanded).toContain("{workspaceRoot}/lib/util.ts");
 expect(expanded).toContain("{workspaceRoot}/app2/own.ts");
 expect(list).not.toContain("{workspaceRoot}/lib/util.ts");
});

test("a row that names the stamp of an inferrer holds while the stamp is declared and stops holding when the declaration is bumped, whatever the bytes of its sources",async()=>{
 const protocol=stamps.inferenceProtocolV1(),restore=()=>observation.registerInferenceStampsV1(stamps.inferenceStampEntriesV1(protocol),stamps.inferenceFragmentsV1(protocol));
 restore();
 try{
  const recorded=await observation.captureObservedSourceInputsV1(async()=>{observation.observedInferenceStampsV1(["inferrer:project-nodes"]);},undefined,false);
  const row=recorded.observations[0];
  expect(row.kind).toBe("stamp");
  expect(observation.observationRowFreshV1(row)).toBe(true);
  observation.registerInferenceStampsV1([["inferrer:project-nodes",protocol.inferrers["project-nodes"].stamp.replace(/\/\d+$/,"/999")]]);
  expect(observation.observationRowFreshV1(row)).toBe(false);
 }finally{restore();}
});

test("the files an owner keeps are those of an independent walk that stops at the tool entry, and with the tool group expanded they cover what esbuild bundles from its script",async()=>{
 const root=workspace(),first=owner(root,"app2"),list=await infer(()=>scriptInputs.nativeTargetCommandInputs(first,root,undefined,cache(root,false),"app2"));
 const kept=list.filter((item:unknown):item is string=>typeof item==="string"&&item.startsWith("{workspaceRoot}/")).map(item=>item.slice(16)).sort();
 const oracle=spawnSync("python3",["-I",join(import.meta.dir,"🐍️oracle/🐍️.py"),root,"app2/📜️script.ts",JSON.stringify([join(root,driver)])],{encoding:"utf8"});
 expect(oracle.status,oracle.stderr).toBe(0);
 expect(kept).toEqual((JSON.parse(oracle.stdout) as string[]).filter(path=>path!==driver).sort());
 const expanded=await infer(()=>scriptInputs.expandedCommandInputsV1(list,root,cache(root,false))),keyed=new Set(expanded.filter((item:unknown):item is string=>typeof item==="string").map((item:string)=>item.slice(16)));
 const built=await esbuild.build({entryPoints:[join(root,"app2/📜️script.ts")],absWorkingDir:root,bundle:true,write:false,platform:"node",format:"esm",packages:"external",metafile:true,logLevel:"silent"});
 for(const input of scriptInputs.withoutSchedulingV1(Object.keys(built.metafile!.inputs).map(input=>"{workspaceRoot}/"+relative(root,resolve(root,input)))))expect(keyed.has((input as string).slice(16)),input as string).toBe(true);
});
