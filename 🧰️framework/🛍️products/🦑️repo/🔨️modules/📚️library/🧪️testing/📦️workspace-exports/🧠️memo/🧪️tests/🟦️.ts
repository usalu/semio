/** 🧠️ Law: the validated memo of the workspace ownership walk answers exactly what a fresh walk answers, on every step of an edit corpus (members added, removed, renamed and re-exported, manifests edited, hidden and unrelated entries added, file contents edited, the memo file torn or from another implementation) — and answers from the memo, without walking, whenever nothing it observed changed. Independent oracle: `fast-glob` lists the member directories the corpus implies. The time-sliced yield keeps cancellation: an aborted signal rejects a memo check and a walk alike.
 * @see ../🟦️.ts — the memo */
import {expect,setDefaultTimeout,test} from "bun:test";
import glob from "fast-glob";
import {mkdirSync,mkdtempSync,readFileSync,renameSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {dirname,join,resolve} from "node:path";
import {workspaceTestAliasesV1,type WorkspaceTestAliasV1} from "../../🟦️.ts";
import {ownershipMemoPathV1,OWNERSHIP_MEMO_SETTLE_MILLISECONDS_V1,readOwnershipMemoV1} from "../🟦️.ts";

setDefaultTimeout(120000);
const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw new Error("SEMIO_TEST_ARTIFACT_DIR is required");mkdirSync(output,{recursive:true});

const put=(root:string,path:string,value:unknown):void=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),typeof value==="string"?value:JSON.stringify(value));};
const scope=(members:string[],workspaces:string[]):unknown=>({name:"scope",private:true,workspaces,semio:{workspace:{schemaVersion:1,members,owners:[]}}});
const publish=(root:string,members:string[]):void=>put(root,"own/package.json",scope(members,glob.sync("own/{members,extra}/*/package.json",{cwd:root,dot:false}).map(path=>dirname(path).slice(4)).filter(path=>!/\/target$/u.test(path)).sort()));
const member=(name:string,main="./index.ts"):unknown=>({name,version:"1.0.0",exports:{".":main}});

type Step={name:string;apply:(root:string)=>void;source:"memo"|"walk";implied?:string[]};
const steps:Step[]=[
 {name:"base",apply:()=>{},source:"walk",implied:["own/members/a","own/members/b"]},
 {name:"unchanged",apply:()=>{},source:"memo"},
 {name:"unchanged again",apply:()=>{},source:"memo"},
 {name:"export retargeted",apply:root=>put(root,"own/members/a/package.json",member("@t/a","./other.ts")),source:"walk",implied:["own/members/a","own/members/b"]},
 {name:"retargeted and settled",apply:()=>{},source:"memo"},
 {name:"member added",apply:root=>put(root,"own/members/c/package.json",member("@t/c")),source:"walk",implied:["own/members/a","own/members/b","own/members/c"]},
 {name:"added and settled",apply:()=>{},source:"memo"},
 {name:"file content edited",apply:root=>{put(root,"own/members/a/index.ts","export const x = 1;\n");put(root,"own/members/a/index.ts","export const x = 2;\n");},source:"walk"},
 {name:"content edited again in place",apply:root=>put(root,"own/members/a/index.ts","export const x = 3;\n"),source:"memo"},
 {name:"member removed",apply:root=>rmSync(join(root,"own/members/b"),{recursive:true}),source:"walk",implied:["own/members/a","own/members/c"]},
 {name:"member renamed",apply:root=>renameSync(join(root,"own/members/c"),join(root,"own/members/d")),source:"walk",implied:["own/members/a","own/members/d"]},
 {name:"hidden directory added",apply:root=>put(root,"own/members/.hidden/package.json",member("@t/hidden")),source:"walk",implied:["own/members/a","own/members/d"]},
 {name:"unrelated file added",apply:root=>put(root,"own/members/a/readme.md","# a\n"),source:"walk",implied:["own/members/a","own/members/d"]},
 {name:"output directory added",apply:root=>put(root,"own/members/target/package.json",member("@t/target")),source:"walk"},
 {name:"export name collides",apply:root=>put(root,"own/members/d/package.json",member("@t/a")),source:"walk"},
 {name:"collision resolved",apply:root=>put(root,"own/members/d/package.json",member("@t/d")),source:"walk"},
 {name:"scope declaration widened",apply:()=>{},source:"walk"},
 {name:"new member under the widened pattern",apply:root=>put(root,"own/extra/e/package.json",member("@t/e")),source:"walk"},
 {name:"settled after the last edit",apply:()=>{},source:"memo"},
];
const normal=(rows:readonly WorkspaceTestAliasV1[]):string[]=>rows.map(row=>`${row.find.source}/${row.find.flags}=>${row.replacement.replace(/^\/private/u,"")}`).sort();
async function answer(root:string,cache:string|undefined,extra:{signal?:AbortSignal}={}):Promise<{aliases:string[];source:string;operations:number}>{
 let source="",operations=0;
 const rows=await workspaceTestAliasesV1(root,{owner:"own",...(cache?{cache,settleMilliseconds:-3600000}:{}),report:row=>{source=row.source;operations=row.operations;},...extra});
 return {aliases:normal(rows),source,operations};
}

test("the memo answers what a fresh walk answers on every step of the edit corpus and only skips the walk when nothing observed changed",async()=>{
 const root=mkdtempSync(join(output,"memo-")),cache=mkdtempSync(join(output,"memo-cache-"));
 try{
  put(root,"package.json",{name:"root",private:true,workspaces:[],semio:{workspace:{schemaVersion:1,members:["unused"],owners:["*/package.json"]}}});
  put(root,"own/members/a/package.json",member("@t/a"));put(root,"own/members/b/package.json",member("@t/b"));publish(root,["members/**"]);
  for(const step of steps){
   step.apply(root);publish(root,step.name.startsWith("scope declaration widened")||step.name.startsWith("new member under")||step.name.startsWith("settled after")?["members/**","extra/**"]:["members/**"]);
   if(step.source==="memo")await Bun.sleep(5);
   let failure:string|undefined;let memo:{aliases:string[];source:string;operations:number}|undefined,fresh:{aliases:string[];source:string;operations:number}|undefined;
   try{memo=await answer(root,cache);}catch(error){failure=String((error as Error).message);}
   try{fresh=await answer(root,undefined);expect(failure,`${step.name}: the memo path fails exactly when the walk fails`).toBeUndefined();}catch(error){if(failure===undefined)throw error;expect(["export name collides"],`${step.name}: only the collision step may fail`).toContain(step.name);expect(String((error as Error).message),step.name).toBe(failure);expect(failure).toContain("Unique current workspace public export required");continue;}
   expect(memo!.aliases,`${step.name}: memo equals fresh walk`).toEqual(fresh.aliases);
   expect(fresh.source).toBe("walk");expect(memo!.source,`${step.name}: answered from the ${step.source}`).toBe(step.source);
   if(step.source==="memo")expect(memo!.operations,"a memo check does at least one check per observed directory").toBeGreaterThan(3);
   if(step.implied){
    const independent=(await glob("own/members/*/package.json",{cwd:root,dot:false})).map(path=>dirname(path)).sort();
    expect(independent,`${step.name}: fast-glob lists the implied member directories`).toEqual([...step.implied].filter(path=>!path.includes("/.")).sort());
    const expected=independent.map(path=>JSON.parse(readFileSync(join(root,path,"package.json"),"utf8")).name as string).sort();
    expect(fresh.aliases.map(row=>row.split("/u=>")[0]!.slice(1,-1).replaceAll("\\","")).sort(),`${step.name}: the aliases name the members' packages`).toEqual(expected);
   }
  }
 }finally{rmSync(root,{recursive:true,force:true});rmSync(cache,{recursive:true,force:true});}
});

test("a torn memo file, a memo of another implementation and a memo of another workspace are ignored, a changed manifest refuses the memo",async()=>{
 const root=mkdtempSync(join(output,"memo-")),cache=mkdtempSync(join(output,"memo-cache-"));
 try{
  put(root,"package.json",{name:"root",private:true,workspaces:[],semio:{workspace:{schemaVersion:1,members:["unused"],owners:["*/package.json"]}}});
  put(root,"own/members/a/package.json",member("@t/a"));publish(root,["members/**"]);
  const file=ownershipMemoPathV1(cache,resolve(root),"own"),first=await answer(root,cache);expect(first.source).toBe("walk");expect((await answer(root,cache)).source).toBe("memo");
  const sealed=readOwnershipMemoV1(file)!;expect(sealed.directories.length).toBeGreaterThan(1);expect(sealed.manifests.map(row=>row[0]).sort()).toEqual(["own/members/a/package.json","own/package.json","package.json"]);
  writeFileSync(file,"{ torn");expect((await answer(root,cache)).source,"torn").toBe("walk");expect((await answer(root,cache)).source,"rewritten").toBe("memo");
  writeFileSync(file,JSON.stringify({...sealed,implementation:"0".repeat(64)}));expect((await answer(root,cache)).source,"other implementation").toBe("walk");
  writeFileSync(file,JSON.stringify({...sealed,workspace:"/elsewhere"}));expect((await answer(root,cache)).source,"other workspace").toBe("walk");
  expect((await answer(root,cache)).source).toBe("memo");
  const manifest=join(root,"own/members/a/package.json"),before=readFileSync(manifest);writeFileSync(manifest,JSON.stringify(member("@t/a","./z.ts")));
  const later=new Date(Date.now()+1000);utimesSync(manifest,later,later);
  expect((await answer(root,cache)).source,"a manifest edit that leaves every directory untouched").toBe("walk");
  writeFileSync(manifest,before);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(cache,{recursive:true,force:true});}
});

test("a tree edited within the settle window is never memoized and an aborted signal rejects a memo check",async()=>{
 const root=mkdtempSync(join(output,"memo-")),cache=mkdtempSync(join(output,"memo-cache-"));
 try{
  put(root,"package.json",{name:"root",private:true,workspaces:[],semio:{workspace:{schemaVersion:1,members:["unused"],owners:["*/package.json"]}}});
  put(root,"own/members/a/package.json",member("@t/a"));publish(root,["members/**"]);
  const file=ownershipMemoPathV1(cache,resolve(root),"own");
  expect(OWNERSHIP_MEMO_SETTLE_MILLISECONDS_V1).toBeGreaterThan(0);
  const rows:string[]=[];await workspaceTestAliasesV1(root,{owner:"own",cache,report:row=>rows.push(row.source)});await workspaceTestAliasesV1(root,{owner:"own",cache,report:row=>rows.push(row.source)});
  expect(rows,"files younger than the settle window are walked every time").toEqual(["walk","walk"]);expect(readOwnershipMemoV1(file)).toBeUndefined();
  await answer(root,cache);const controller=new AbortController();controller.abort(new Error("stop"));
  await expect(workspaceTestAliasesV1(root,{owner:"own",cache,settleMilliseconds:-3600000,signal:controller.signal})).rejects.toThrow("stop");
 }finally{rmSync(root,{recursive:true,force:true});rmSync(cache,{recursive:true,force:true});}
});

test("a memo check and a walk both report every operation to the caller and stop after exactly the advance that cancels them, having let the event loop run",async()=>{
 const root=mkdtempSync(join(output,"memo-")),cache=mkdtempSync(join(output,"memo-cache-"));
 try{
  put(root,"package.json",{name:"root",private:true,workspaces:[],semio:{workspace:{schemaVersion:1,members:["unused"],owners:["*/package.json"]}}});
  put(root,"own/members/a/package.json",member("@t/a"));publish(root,["members/**"]);
  const sources:string[]=[];await workspaceTestAliasesV1(root,{owner:"own",cache,settleMilliseconds:-3600000,report:row=>sources.push(row.source)});
  for(const expected of["memo","walk"]){
   const controller=new AbortController(),reason=new Error("cancelled by the third advance");let advances=0,looped=false;
   const heartbeat=new Promise<void>(accept=>setImmediate(()=>{looped=true;accept();}));
   const pending=workspaceTestAliasesV1(root,{owner:"own",cache:expected==="memo"?cache:null,settleMilliseconds:-3600000,signal:controller.signal,advance:()=>{advances++;if(advances===3)controller.abort(reason);}});
   await expect(pending,expected).rejects.toBe(reason);expect(advances,expected).toBe(3);await heartbeat;expect(looped,expected).toBe(true);
  }
  expect(sources).toEqual(["walk"]);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(cache,{recursive:true,force:true});}
});
