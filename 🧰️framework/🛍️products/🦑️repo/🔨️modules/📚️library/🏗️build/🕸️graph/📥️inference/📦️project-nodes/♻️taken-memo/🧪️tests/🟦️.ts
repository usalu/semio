/** ♻️ Contract: a pass over the owners that follows an earlier pass of the same read takes again exactly the owners the independent model calls stale (an input differs from what the owner saw, or the job changed) and answers every other owner from the memo with its rows retained into the running capture; the memo equals a full take of every owner, pass after pass.
 * @see ../🟨️.mjs */
import {Database} from "bun:sqlite";
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {readFileSync} from "node:fs";
import {mkdir,mkdtemp,readFile,readdir,rm,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import * as observation from "../../../../../👁️observation/🟨️.mjs";
import {createPathRowMatcherV1,createTakenMemoV1} from "../🟨️.mjs";

type Owner={readonly name:string;readonly files:readonly string[];readonly directory:string;job:string};
const OWNERS=40,sha=(text:string|Buffer)=>createHash("sha256").update(text).digest("hex");

/** 🧮️ The independent model: a table of what each owner saw (file content digests and the listing of its directory) when it was taken, the same facts read from disk now, and one SQL query that names the owners to take again; sqlite shares no code with the memo. */
async function facts(root:string,owner:Owner):Promise<Map<string,string>>{
 const rows=new Map<string,string>();
 for(const file of owner.files)rows.set(file,sha(await readFile(join(root,file))));
 rows.set("dir:"+owner.directory,sha((await readdir(join(root,owner.directory))).sort().join("\0")));
 return rows;
}
const oracle=(seen:ReadonlyMap<string,ReadonlyMap<string,string>>,now:ReadonlyMap<string,ReadonlyMap<string,string>>,jobs:ReadonlyMap<string,string>,keys:ReadonlyMap<string,string>)=>{
 const db=new Database(":memory:");
 try{
  db.run("create table seen(owner text,input text,digest text);create table now(owner text,input text,digest text);create table job(owner text,seen text,now text)");
  for(const[owner,rows]of seen)for(const[input,digest]of rows)db.run("insert into seen values(?,?,?)",[owner,input,digest]);
  for(const[owner,rows]of now)for(const[input,digest]of rows)db.run("insert into now values(?,?,?)",[owner,input,digest]);
  for(const[owner,key]of jobs)db.run("insert into job values(?,?,?)",[owner,key,keys.get(owner)!]);
  return (db.query("select distinct owner from (select s.owner from seen s join now n on n.owner=s.owner and n.input=s.input where n.digest<>s.digest union select owner from job where seen<>now) order by owner").all() as {owner:string}[]).map(row=>row.owner);
 }finally{db.close();}
};

async function world(body:(input:{root:string;owners:Owner[];write:(file:string,text:string)=>Promise<void>}) =>Promise<void>){
 const root=await mkdtemp(join(tmpdir(),"taken-memo-")),write=async(file:string,text:string)=>{await writeFile(join(root,file),text);};
 try{
  await mkdir(join(root,"shared"),{recursive:true});await write("shared/common.txt","common");
  const owners:Owner[]=[];
  for(let at=0;at<OWNERS;at++){
   const directory="owner-"+at;await mkdir(join(root,directory),{recursive:true});
   await write(directory+"/src.txt","source "+at);await write(directory+"/more.txt","more "+at);
   owners.push({name:directory,directory,job:"job "+at,files:[directory+"/src.txt",directory+"/more.txt",...at%5===0?["shared/common.txt"]:[]]});
  }
  await body({root,owners,write});
 }finally{await rm(root,{recursive:true,force:true});}
}

const value=async(root:string,owner:Owner)=>({name:owner.name,digest:sha([...(await facts(root,owner)).entries()].join("|")),job:owner.job});

/** 🏃️ One pass of the owner loop as the nodes body runs it: replay the memo, take the rest (each owner reads its inputs in its own capture and inherits the rows into the pass), remember what was taken. */
async function pass(root:string,owners:readonly Owner[],memo:ReturnType<typeof createTakenMemoV1>,seen:Map<string,Map<string,string>>,hint:readonly string[]=[]){
 const took:string[]=[],reusedNames:string[]=[],projects=new Map<string,unknown>();
 const captured=await observation.captureObservedSourceInputsV1(async()=>{
  memo.evictReferencing(hint);
  const reused=await memo.replay(owners.map(owner=>({name:owner.name,key:owner.job})));
  for(const row of reused){reusedNames.push(row.name);projects.set(row.name,row.project);}
  for(const owner of owners){
   if(projects.has(owner.name))continue;
   took.push(owner.name);
   const own=await observation.captureObservedSourceInputsV1(async()=>{
    for(const file of owner.files)observation.observedFs.readFileSync(join(root,file),"utf8");
    observation.observedFs.readdirSync(join(root,owner.directory));
    return value(root,owner);
   },undefined,false);
   observation.retainObservedRowsV1(own.observations);
   seen.set(owner.name,await facts(root,owner));projects.set(owner.name,own.value);
   memo.remember(owner.name,{key:owner.job,project:own.value,rows:own.observations});
  }
  return projects;
 },undefined,false);
 return{took:took.sort(),reused:reusedNames.sort(),projects:captured.value,observations:captured.observations as readonly {kind:string;path:string}[]};
}
const memoOf=()=>createTakenMemoV1({holds:rows=>observation.observationRowsHoldV1(rows as never),retain:rows=>observation.retainObservedRowsV1(rows as never)});
const nowOf=async(root:string,owners:readonly Owner[])=>new Map(await Promise.all(owners.map(async owner=>[owner.name,await facts(root,owner)] as const)));
const keysOf=(owners:readonly Owner[])=>new Map(owners.map(owner=>[owner.name,owner.job]));

test("a repair pass after one stale path takes exactly the owners that read it, and the memo equals a full take",()=>world(async({root,owners,write})=>{
 const memo=memoOf(),seen=new Map<string,Map<string,string>>(),jobs=new Map(owners.map(owner=>[owner.name,owner.job]));
 const first=await pass(root,owners,memo,seen);
 expect(first.took).toHaveLength(OWNERS);expect(first.reused).toHaveLength(0);

 await write("owner-7/src.txt","source 7 edited");
 const model=oracle(seen,await nowOf(root,owners),jobs,keysOf(owners));
 expect(model).toEqual(["owner-7"]);
 const second=await pass(root,owners,memo,seen,[join(root,"owner-7/src.txt")]);
 expect(second.took).toEqual(model);expect(second.reused).toHaveLength(OWNERS-1);
 for(const owner of owners)expect(second.projects.get(owner.name)).toEqual(await value(root,owner));
 const full=await pass(root,owners,memoOf(),new Map());
 expect(new Set(second.observations.map(row=>row.path))).toEqual(new Set(full.observations.map(row=>row.path)));

 await write("shared/common.txt","common edited");
 const shared=oracle(seen,await nowOf(root,owners),jobs,keysOf(owners));
 expect(shared).toEqual(owners.filter((_,at)=>at%5===0).map(owner=>owner.name).sort());
 const third=await pass(root,owners,memo,seen,[join(root,"shared/common.txt")]);
 expect(third.took).toEqual(shared);
 for(const owner of owners)expect(third.projects.get(owner.name)).toEqual(await value(root,owner));
}));

test("a change nobody named is found by the row check, a listing change reaches the owner of the directory, and a changed job is never answered from the memo",()=>world(async({root,owners,write})=>{
 const memo=memoOf(),seen=new Map<string,Map<string,string>>(),jobs=new Map(owners.map(owner=>[owner.name,owner.job]));
 await pass(root,owners,memo,seen);

 await write("owner-3/more.txt","more 3 edited");await write("owner-12/added.txt","new member");
 const model=oracle(seen,await nowOf(root,owners),jobs,keysOf(owners));
 expect(model).toEqual(["owner-12","owner-3"]);
 const second=await pass(root,owners,memo,seen,[]);
 expect(second.took).toEqual(model);expect(second.reused).toHaveLength(OWNERS-2);
 for(const owner of owners)expect(second.projects.get(owner.name)).toEqual(await value(root,owner));
 expect(memo.stats().evictedChanged).toBe(2);

 owners[20]!.job="job 20 changed";
 const keyed=oracle(seen,await nowOf(root,owners),jobs,keysOf(owners));
 expect(keyed).toEqual(["owner-20"]);
 const third=await pass(root,owners,memo,seen);
 expect(third.took).toEqual(keyed);expect(memo.stats().evictedKey).toBe(1);
 expect(third.projects.get("owner-20")).toEqual(await value(root,owners[20]!));
}));

test("several passes in a row stay equal to a full take, and a named path forgets exactly the owners that read it or list its directory",()=>world(async({root,owners,write})=>{
 const memo=memoOf(),seen=new Map<string,Map<string,string>>(),jobs=new Map(owners.map(owner=>[owner.name,owner.job]));
 await pass(root,owners,memo,seen);
 const edits:[string,string][]=[["owner-1/src.txt","a"],["owner-9/more.txt","b"],["shared/common.txt","c"],["owner-30/src.txt","d"]];
 let total=0;
 for(const[file,text]of edits){
  await write(file,text);
  const model=oracle(seen,await nowOf(root,owners),jobs,keysOf(owners)),next=await pass(root,owners,memo,seen,[join(root,file)]);
  expect(next.took).toEqual(model);total+=next.took.length;
  for(const owner of owners)expect(next.projects.get(owner.name)).toEqual(await value(root,owner));
 }
 expect(total).toBeLessThan(OWNERS*edits.length/2);
 const rows=[{kind:"file",path:"/r/a/x"},{kind:"readdir",path:"/r/b"},{kind:"file",path:"/r/b/y"},{kind:"readdirNames",path:"/r/c"}],matches=createPathRowMatcherV1(["/r/b/z","/r/a/x"]);
 expect(rows.filter(matches).map(row=>row.path)).toEqual(["/r/a/x","/r/b"]);
 const own=memoOf();own.remember("a",{key:"k",project:{},rows:[rows[0]!]});own.remember("b",{key:"k",project:{},rows:[rows[1]!]});own.remember("c",{key:"k",project:{},rows:[rows[3]!]});own.remember("empty",{key:"k",project:{},rows:[]});
 expect(own.size).toBe(3);expect(own.evictReferencing(["/r/b/z"])).toBe(1);expect(own.evictReferencing([])).toBe(0);expect(own.size).toBe(2);
}));

test("the owner loop answers from the memo before any look, remembers an owner only after its answer is final, and never in the late-closure mode",()=>{
 const text=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8");
 expect(text).toContain("const takenMemo=late?undefined:context.takenMemo");
 expect(text).toContain("serialize([registryDigest,commandRevision,late,[...projectsByRoot],componentRoots,contracts])");
 expect(text).toContain("try{await preloadTaken();await runTakeBesideInferV1({take:resolveOwners,");
 const finished=text.indexOf("result.projects[name].targets={...inactive,...result.projects[name].targets};noteProjectSizeV1(name,result.projects[name]);"),remembered=text.indexOf("takenMemo.remember(name,{key:memoKey,project:result.projects[name],rows:stored})");
 expect(finished).toBeGreaterThan(-1);expect(remembered).toBeGreaterThan(finished);expect(text.split("takenMemo.remember(").length).toBe(2);
 expect(text.indexOf("if(result.projects[name].unavailable){")).toBeLessThan(remembered);
});
