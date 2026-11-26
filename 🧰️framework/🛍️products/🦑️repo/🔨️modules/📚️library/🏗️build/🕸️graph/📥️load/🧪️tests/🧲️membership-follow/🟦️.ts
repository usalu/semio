import {expect,test} from "bun:test";
import {lstat,mkdir,mkdtemp,rm,writeFile} from "node:fs/promises";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {evictOwnerFactsReferencingV1,selectedAdmissionFactsV1} from "../../🟦️.ts";
import {buildGraphMembershipV1} from "../../../🟦️.ts";
import {followedPathsV1,graphInventoryTextV1,graphRetryTextV1,membershipChangePathsV1,untoleratedFollowedPathsV1,vanishedMemberPathV1} from "../../🧲️membership-follow/🟦️.ts";
import {independentGlobV1} from "../../../../🧪️tests/🕸️graph/🧰️support/🟦️.ts";

const row=(kind:string,path:string)=>({kind,path,fact:kind==="readdir"?"digest":{version:1}});
const owners=(scope:string)=>selectedAdmissionFactsV1.entries(scope).map(entry=>entry.suffix).sort();

test("only the owner memos whose rows name a changed member or its parent listing are forgotten",()=>{
 const scope="scope-evict-"+Math.random();
 selectedAdmissionFactsV1.seed(scope,"reads-member",{},[row("file","/w/a/manifest"),row("file","/w/a/src/x.ts")]);
 selectedAdmissionFactsV1.seed(scope,"lists-parent",{},[row("readdir","/w/a/src")]);
 selectedAdmissionFactsV1.seed(scope,"lists-elsewhere",{},[row("readdir","/w/b")]);
 selectedAdmissionFactsV1.seed(scope,"reads-elsewhere",{},[row("file","/w/b/src/x.ts")]);
 selectedAdmissionFactsV1.seed("other-"+scope,"reads-member",{},[row("file","/w/a/src/x.ts")]);
 expect(evictOwnerFactsReferencingV1(scope,["/w/a/src/x.ts"])).toBe(2);
 expect(owners(scope)).toEqual(["lists-elsewhere","reads-elsewhere"]);
 expect(owners("other-"+scope)).toEqual(["reads-member"]);
 expect(evictOwnerFactsReferencingV1(scope,[])).toBe(0);
 expect(owners(scope)).toEqual(["lists-elsewhere","reads-elsewhere"]);
});

test("a refusal names its paths under the root and unnamed refusals name none",()=>{
 expect(membershipChangePathsV1("/w",{paths:["a/b","/abs/c"]})).toEqual([join("/w","a/b"),"/abs/c"]);
 expect(membershipChangePathsV1("/w",new Error("x"))).toEqual([]);
 expect(membershipChangePathsV1("/w",null)).toEqual([]);
});

test("the follow-up lines are said in both languages with their numbers",()=>{
 const retry=graphRetryTextV1(3,"a/b.ts",5,2,{memo:10,checkpoint:4,discovery:1},12400);
 const settled=graphInventoryTextV1(4,{memo:11,checkpoint:0,discovery:0},61000);
 for(const language of ["en","de"] as const){
  expect(retry[language]).toContain("a/b.ts");expect(retry[language]).toContain("3");expect(retry[language]).toContain("12");expect(retry[language]).not.toContain("{");
  expect(settled[language]).toContain("61");expect(settled[language]).not.toContain("{");
 }
 expect(retry.en).not.toBe(retry.de);
});

test("a membership that moved names every moved member once, as an independent file walk sees them",async()=>{
 const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"membership-follow-"));
 try{
  for(const name of ["a","b","c","d"])await mkdir(join(root,name,"src"),{recursive:true});
  for(const name of ["a","b","c","d"])for(const file of ["manifest","src/x.ts","src/y.ts"])await writeFile(join(root,name,file),name+file);
  const walk=async()=>(await independentGlobV1("**/*",{cwd:root,dot:true})).map((path:string)=>path.replaceAll("\\","/")).sort();
  const files=await walk();
  expect(files).toHaveLength(12);
  const before=await buildGraphMembershipV1(root,files);
  expect(await buildGraphMembershipV1(root,files)).toBe(before);
  await rm(join(root,"a/manifest"));await rm(join(root,"c/src/y.ts"));await rm(join(root,"d/src/x.ts"));
  await rm(join(root,"b/src/y.ts"));await mkdir(join(root,"b/src/y.ts"));
  const alive=new Set(await walk()),moved=files.filter((file:string)=>!alive.has(file)).sort();
  expect(moved).toHaveLength(4);
  const failure=await buildGraphMembershipV1(root,files).then(()=>undefined,error=>error as {code:string;paths:string[]});
  expect(failure?.code).toBe("BUILD_MEMBERSHIP_CHANGED");
  expect([...failure!.paths].sort()).toEqual(moved);
  const rest=files.filter((file:string)=>alive.has(file));
  expect(await buildGraphMembershipV1(root,rest)).not.toBe(before);
  expect(await buildGraphMembershipV1(root,rest)).toBe(await buildGraphMembershipV1(root,rest));
 }finally{await rm(root,{recursive:true,force:true});}
});

test("a path that vanished under the inventory walk names a member below the root, never the root itself",async()=>{
 const root=await mkdtemp(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"membership-vanished-"));
 try{
  const gone=join(root,"modules","scratch");
  const raw=await lstat(gone).then(()=>undefined,error=>error);
  expect(vanishedMemberPathV1(root,raw)).toBe(gone);
  expect(vanishedMemberPathV1(root,Object.assign(new Error("gone"),{code:"ENOENT",path:root}))).toBeUndefined();
  const outside=await lstat(join(tmpdir(),"membership-vanished-"+Math.random())).then(()=>undefined,error=>error);
  expect(vanishedMemberPathV1(root,outside)).toBeUndefined();
  expect(vanishedMemberPathV1(root,Object.assign(new Error("denied"),{code:"EACCES",path:gone}))).toBeUndefined();
  expect(vanishedMemberPathV1(root,null)).toBeUndefined();
  expect(followedPathsV1(root,Object.assign(new Error("wrapped"),{cause:raw}))).toEqual([gone]);
 }finally{await rm(root,{recursive:true,force:true});}
});

test("an owner failure whose paths are all tolerated is not followed any more, a path elsewhere still is",()=>{
 const root=resolve("/w"),gone=Object.assign(new Error("lstat"),{code:"ENOENT",path:join(root,"a","b","Cargo.toml")});
 expect(untoleratedFollowedPathsV1(root,gone,new Set())).toEqual([gone.path]);
 expect(untoleratedFollowedPathsV1(root,gone,new Set(["a/b/Cargo.toml"]))).toEqual([]);
 expect(untoleratedFollowedPathsV1(root,gone,new Set(["a/b"]))).toEqual([]);
 expect(untoleratedFollowedPathsV1(root,gone,new Set(["a/bb","c"]))).toEqual([gone.path]);
 expect(untoleratedFollowedPathsV1(root,new Error("boom"),new Set(["a"]))).toEqual([]);
});
