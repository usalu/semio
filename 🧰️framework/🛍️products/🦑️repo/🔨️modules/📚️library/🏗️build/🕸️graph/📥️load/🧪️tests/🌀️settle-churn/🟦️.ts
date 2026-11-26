/** 🌀️ Contract: a path that changes while the owner phase runs holds no unrelated owner hostage. Each owner settles against its own consistent snapshot, the rows of both states stay recorded, a contradiction inside one owner repeats the round for that owner only, and a later read re-checks exactly the owners that read the newer state. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdir,mkdtemp,rm,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {evictOwnerFactsReferencingV1,observeOwnerFactsMemoizedV1} from "../../🟦️.ts";
import {loadedGraphAdmissionFormatV1} from "../../../🔐️authority/🟦️.ts";
import {graphSettleTextV1} from "../../🧲️membership-follow/🟦️.ts";
import * as observation from "../../../../👁️observation/🟨️.mjs";

type Owner={name:string;files:string[]};
type Event={round:number;settled?:true;conflict?:string;paths?:readonly string[]};
const sha=(text:string)=>createHash("sha256").update(text).digest("hex");
const OWNERS=8,DEPENDENT=["owner-2","owner-5"];

/** 🧮️ The independent model: it knows only the declared inputs of each owner, the content each owner saw when it was observed and the content now on disk; an owner must be observed again exactly when one of its inputs differs from what it saw. */
const seen=new Map<string,Map<string,string>>();
const modelStale=(owners:readonly Owner[],contents:ReadonlyMap<string,string[]>)=>owners.filter(owner=>owner.files.some(file=>seen.get(owner.name)?.get(file)!==contents.get(file)!.at(-1))).map(owner=>owner.name).sort();

async function world(body:(input:{root:string;owners:Owner[];contents:Map<string,string[]>;write:(file:string,text:string)=>Promise<void>}) =>Promise<void>){
 const root=await mkdtemp(join(tmpdir(),"settle-churn-")),contents=new Map<string,string[]>();
 const write=async(file:string,text:string)=>{await writeFile(join(root,file),text);const states=contents.get(file)??[];states.push(text);contents.set(file,states);};
 try{
  await mkdir(root,{recursive:true});
  const owners:Owner[]=Array.from({length:OWNERS},(_,at)=>({name:"owner-"+at,files:["own-"+at+".txt",...(DEPENDENT.includes("owner-"+at)?["noise.txt"]:[])]}));
  for(const owner of owners)await write(owner.files[0]!,"own "+owner.name);
  await write("noise.txt","n");
  await body({root,owners,contents,write});
 }finally{await rm(root,{recursive:true,force:true});}
}

/** 🏃️ One read of the owner phase as the loader runs it: the real memoized owner observation inside the real settle, with the loader's follow-up of `onRound`. */
async function read(root:string,owners:readonly Owner[],scope:string,contents:ReadonlyMap<string,string[]>,options:{limit?:number;during?:(owner:Owner,pass:number)=>Promise<void>}={}){
 const discovered:string[]=[],events:Event[]=[],trust={current:new WeakSet<object>()},passes=new Map<string,number>();
 const error=await observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>{
  for(const owner of owners)await observeOwnerFactsMemoizedV1({
   key:scope+"\0"+loadedGraphAdmissionFormatV1+"\0"+owner.name,
   hold:async rows=>observation.observationRowsHoldV1(rows as never),
   observation,trust:{has:entry=>trust.current.has(entry),add:entry=>trust.current.add(entry)},
   observe:async()=>{
    discovered.push(owner.name);seen.set(owner.name,new Map(owner.files.map(file=>[file,contents.get(file)!.at(-1)!])));const pass=(passes.get(owner.name)??0)+1;passes.set(owner.name,pass);
    for(const file of owner.files)observation.observedFs.readFileSync(join(root,file),"utf8");
    await options.during?.(owner,pass);
    return owner.name;
   },
  });
  return "sealed";
 },{limit:options.limit,onRound:event=>{
  events.push(event as Event);
  const paths=event.paths??[];
  if(event.settled!==true&&paths.length>0)evictOwnerFactsReferencingV1(scope,paths);
  else if(event.settled!==true)trust.current=new WeakSet();
 }}),undefined,false).then(captured=>({captured,failure:undefined as unknown}),failure=>({captured:undefined,failure}));
 return{discovered,events,...error};
}
const counts=(names:readonly string[])=>names.reduce<Record<string,number>>((all,name)=>({...all,[name]:(all[name]??0)+1}),{});

test("one unrelated file flipping state every round seals in one round and a later read re-observes only its readers",()=>world(async({root,owners,contents,write})=>{
 const scope="settle-churn-a-"+Math.random(),flip=async(owner:Owner)=>{if(owner.name===DEPENDENT[0])await write("noise.txt","n".repeat(contents.get("noise.txt")!.length+1));};
 seen.clear();
 const first=await read(root,owners,scope,contents,{during:flip});
 expect(first.failure).toBeUndefined();
 expect(first.events.map(event=>event.settled)).toEqual([true]);
 expect(first.events[0]!.paths).toEqual([join(root,"noise.txt")]);
 expect(counts(first.discovered)).toEqual(Object.fromEntries(owners.map(owner=>[owner.name,1])));
 const rows=[...first.captured!.observations].flatMap(row=>row.kind==="file"&&row.path===join(root,"noise.txt")&&"sha256" in row.fact?[row.fact.sha256]:[]).sort();
 expect(rows).toEqual(contents.get("noise.txt")!.slice(0,2).map(sha).sort());
 await write("noise.txt","n".repeat(contents.get("noise.txt")!.length+1));
 const staleBefore=modelStale(owners,contents);
 const second=await read(root,owners,scope,contents,{during:flip});
 expect(second.failure).toBeUndefined();
 expect(second.discovered.sort()).toEqual(staleBefore);
 expect(second.discovered.sort()).toEqual([...DEPENDENT].sort());
 await write("own-3.txt","own owner-3 edited");
 const stale=modelStale(owners,contents),third=await read(root,owners,scope,contents);
 expect(stale).toEqual(["owner-2","owner-3"]);
 expect(third.discovered.sort()).toEqual(stale);
 expect(third.events).toEqual([]);
}));

test("an owner whose own input flips inside its observation is observed again and no other owner is",()=>world(async({root,owners,contents,write})=>{
 const scope="settle-churn-b-"+Math.random(),target=owners[3]!;
 const reread=async(owner:Owner,pass:number)=>{if(owner===target&&pass===1){await write(target.files[0]!,"own "+target.name+" changed mid-observation");observation.observedFs.readFileSync(join(root,target.files[0]!),"utf8");}};
 const outcome=await read(root,owners,scope,contents,{during:reread});
 expect(outcome.failure).toBeUndefined();
 expect(outcome.events.length).toBe(1);
 expect(outcome.events[0]!.settled).toBeUndefined();
 expect(outcome.events[0]!.paths).toEqual([join(root,target.files[0]!)]);
 expect(counts(outcome.discovered)).toEqual({...Object.fromEntries(owners.map(owner=>[owner.name,1])),[target.name]:2});
 const states=[...outcome.captured!.observations].flatMap(row=>row.kind==="file"&&row.path===join(root,target.files[0]!)&&"sha256" in row.fact?[row.fact]:[]);
 expect(states.length).toBe(1);
 expect(states[0]!.sha256).toBe(sha(contents.get(target.files[0]!)!.at(-1)!));
}));

test("a contradiction inside one owner that never settles is refused after the round limit while every other owner is observed once",()=>world(async({root,owners,contents,write})=>{
 const scope="settle-churn-c-"+Math.random(),target=owners[1]!,limit=4;let flips=0;
 const outcome=await read(root,owners,scope,contents,{limit,during:async owner=>{if(owner===target){await write(target.files[0]!,"flip".repeat(++flips));observation.observedFs.readFileSync(join(root,target.files[0]!),"utf8");}}});
 expect(outcome.failure).toBeDefined();
 expect(outcome.events.length).toBe(limit);
 expect(counts(outcome.discovered)).toEqual({...Object.fromEntries(owners.map(owner=>[owner.name,1])),[target.name]:limit});
}));

test("the settle line names the paths in English and German",()=>{
 const churn=graphSettleTextV1(1,3,["a","b","c"],0,true),contradiction=graphSettleTextV1(2,1,["x"],1,false);
 expect(churn.en).toContain("3 path(s)");expect(churn.en).toContain("a, b, c");expect(churn.de).toContain("3 Pfad(e)");
 expect(contradiction.en).toContain("x");expect(contradiction.de).toContain("1 Eigentümer-Merkzettel");
});
