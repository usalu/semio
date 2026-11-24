/** 🌊️ Contract: a directory that vanishes while one owner reads it is a named path, not the end of the process. The follow-up reads only the owners that did not finish (the vanished read left its absence recorded), every other owner answers from its memo, and the read seals against the tree as it is now.
 * @see ../../🧲️membership-follow/🟦️.ts `followedPathsV1` */
import {expect,test} from "bun:test";
import {lstatSync} from "node:fs";
import {mkdir,mkdtemp,rm,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {evictOwnerFactsReferencingV1,observeOwnerFactsMemoizedV1} from "../../🟦️.ts";
import {loadedGraphAdmissionFormatV1} from "../../../🔐️authority/🟦️.ts";
import {followedPathsV1} from "../../🧲️membership-follow/🟦️.ts";
import {concurrentEditV1,staleImplementationV1} from "../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import * as observation from "../../../../👁️observation/🟨️.mjs";

const OWNERS=8,READER=3;
const names=Array.from({length:OWNERS},(_,at)=>"owner-"+at);

/** 🧮️ The independent model: the directories that exist on disk now, asked of the filesystem itself, and the owners that have not finished an observation yet. */
const exists=(path:string)=>{try{return lstatSync(path).isDirectory();}catch{return false;}};

async function world(body:(input:{root:string;corners:string}) =>Promise<void>){
 const root=await mkdtemp(join(tmpdir(),"vanished-directory-")),corners=join(root,"canvas","corners");
 try{
  await mkdir(corners,{recursive:true});
  for(const name of names)await writeFile(join(root,name+".txt"),"own "+name);
  await writeFile(join(corners,"a.txt"),"a");
  await body({root,corners});
 }finally{await rm(root,{recursive:true,force:true});}
}

async function read(root:string,corners:string,scope:string,finished:Set<string>,discovered:string[],vanish:boolean){
 const trust={current:new WeakSet<object>()};
 return observation.captureObservedSourceInputsV1(()=>observation.settleObservedComputationV1(async()=>{
  for(const name of names)await observeOwnerFactsMemoizedV1({
   key:scope+"\0"+loadedGraphAdmissionFormatV1+"\0"+name,
   hold:async rows=>observation.observationRowsHoldV1(rows as never),
   observation,trust:{has:entry=>trust.current.has(entry),add:entry=>trust.current.add(entry)},
   observe:async()=>{
    discovered.push(name);
    observation.observedFs.readFileSync(join(root,name+".txt"),"utf8");
    if(name==="owner-"+READER){
     observation.observedFs.readdirSync(corners,{withFileTypes:true});
     if(vanish){await rm(corners,{recursive:true,force:true});observation.forgetRoundReadsV1();}
     observation.observedFs.readdirSync(corners,{withFileTypes:true});
    }
    finished.add(name);return name;
   },
  });
  return "sealed";
 }),undefined,false);
}

test("a directory that vanishes inside one owner is named, and the follow-up observes only the owners that did not finish",()=>world(async({root,corners})=>{
 const scope="vanished-a-"+Math.random(),finished=new Set<string>(),first:string[]=[];
 const failure=await read(root,corners,scope,finished,first,true).then(()=>undefined,error=>error);
 expect(failure).toBeDefined();
 expect(exists(corners)).toBe(false);
 const named=followedPathsV1(root,failure);
 expect(named).toEqual([corners]);
 expect(named.every(path=>!exists(path))).toBe(true);
 expect(evictOwnerFactsReferencingV1(scope,named)).toBe(0);
 expect([...finished].sort()).toEqual(names.slice(0,READER).sort());
 const modelUnfinished=names.filter(name=>!finished.has(name));
 await mkdir(corners,{recursive:true});
 const third:string[]=[];
 expect(names.filter(name=>!finished.has(name))).toEqual(modelUnfinished);
 const sealed=await read(root,corners,scope,finished,third,false);
 expect(sealed.value).toBe("sealed");
 expect(third.sort()).toEqual(modelUnfinished.sort());
 expect([...new Set(first)]).toEqual(names.slice(0,READER+1));
 const rows=(sealed.observations as {kind:string;path:string}[]).filter(row=>row.path===corners&&(row.kind==="readdir"));
 expect(rows.length).toBeGreaterThan(0);
}));

test("a failure that names no moved path of the tree, a stale implementation and a foreign path stay fatal",()=>world(async({root,corners})=>{
 const denied=Object.assign(new Error("denied"),{code:"EACCES",path:corners});
 const outside=Object.assign(new Error("gone"),{code:"ENOENT",path:join(tmpdir(),"elsewhere-"+Math.random())});
 const stale=Object.assign(staleImplementationV1("moved"),{path:corners,code:staleImplementationV1("x").code});
 expect(followedPathsV1(root,denied)).toEqual([]);
 expect(followedPathsV1(root,outside)).toEqual([]);
 expect(followedPathsV1(root,stale)).toEqual([]);
 expect(followedPathsV1(root,new Error("plain"))).toEqual([]);
 expect(followedPathsV1(root,undefined)).toEqual([]);
 expect(followedPathsV1(root,Object.assign(new Error("wrapped",{cause:concurrentEditV1("vanished",corners)}),{code:"WRAPPER"}))).toEqual([corners]);
 expect(followedPathsV1(root,concurrentEditV1("bytes",join(root,"owner-1.txt")))).toEqual([join(root,"owner-1.txt")]);
 expect(followedPathsV1(root,concurrentEditV1("vanished",join(tmpdir(),"elsewhere")))).toEqual([]);
}));
