/** 🪤️ Contract: an owner whose Cargo path dependency has no row in the registry the attempt pinned is followed to the physical registry even when that registry is byte-identical to the one read before the attempt (a peer regenerating it exposed a transient version only to the attempt); it is never refused as stable or inconsistent. */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {Database} from "bun:sqlite";
import {clearPinnedAuthorityV1,withImplementationPinV1} from "../../../📥️inputs/🔏️revision/🟨️.mjs";
import {readBuildOwnerRegistrySourceV1} from "../../../🕸️graph/📥️inference/🟨️.mjs";
import {settleOwnerRegistryV1,withOwnerRegistryCustodyV1} from "../../../🕸️graph/📥️load/🧭️registry-follow/🟦️.ts";
import {joinOwnerFailuresV1,ownerFailureV1} from "../../../🕸️graph/📥️load/🛟️owner-isolation/🟦️.ts";

const dependency="🧰️framework/🚚️window-resource/📦️packages/🦀️rust";
const row=(name:string,root:string)=>({name,root,manifest:root+"/Cargo.toml",packages:[]});
const lab=()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"registry-transient-")),write=(path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
 write("a/rust/Cargo.toml",`[package]\nname = "a"\n\n[dependencies]\nwindow-resource = { path = "../../${dependency}" }\n`);
 write(dependency+"/Cargo.toml","[package]\nname = \"window-resource\"\n");
 const put=(owners:object[])=>write("📋️build-owners.json",JSON.stringify({version:1,owners,catalogProviders:[]}));
 return{root,put,read:(path:string)=>readFileSync(join(root,path),"utf8")};
};
const ownersOf=(text:string)=>(JSON.parse(text).owners as {root:string}[]).map(owner=>owner.root);
const pathDependencyOf=(manifest:string)=>(Bun.TOML.parse(manifest) as {dependencies:Record<string,{path:string}>}).dependencies["window-resource"]!.path.replace(/^(\.\.\/)+/u,"");
const unownedBy=(registry:string,manifest:string)=>{
 const oracle=new Database(":memory:");
 try{
  oracle.run("create table owner(root text)");
  for(const root of ownersOf(registry))oracle.run("insert into owner values(?)",[root]);
  return (oracle.query("select ?1 as root where ?1 not in (select root from owner)").all(pathDependencyOf(manifest)) as {root:string}[]).map(entry=>entry.root);
 }finally{oracle.close();}
};
const failureOf=(roots:string[])=>joinOwnerFailuresV1(["@t/a","@t/b","@t/c"].map(name=>ownerFailureV1(name,Object.assign(Error("Canonical native dependency owner required: "+roots[0]),{code:"BUILD_SOURCE_AUTHORITY"}),1,true,[])));
const custody=<T>(body:()=>Promise<T>)=>()=>withOwnerRegistryCustodyV1<T>(new AbortController().signal,contain=>contain(body));

test("a registry that is identical before and after the attempt but lacked the row while pinned is followed",async()=>{
 const l=lab(),complete=[row("@t/a","a/rust"),row("@t/window-resource",dependency)],transient=[row("@t/a","a/rust")];
 try{
  l.put(complete);
  let calls=0;const lines:string[]=[],pinned:string[]=[];
  const read=()=>withImplementationPinV1(()=>settleOwnerRegistryV1(l.root,custody(async()=>{
   calls++;clearPinnedAuthorityV1();
   if(calls===1)l.put(transient);
   const seen=readBuildOwnerRegistrySourceV1(l.root);pinned.push(seen);
   const missing=unownedBy(seen,l.read("a/rust/Cargo.toml"));
   if(calls===1)l.put(complete);
   if(missing.length)throw failureOf(missing);
   return "sealed";
  }),{say:line=>lines.push(line)}));
  expect(await read()).toBe("sealed");
  expect(calls).toBe(2);
  expect(unownedBy(pinned[0]!,l.read("a/rust/Cargo.toml"))).toEqual([dependency]);
  expect(unownedBy(pinned[1]!,l.read("a/rust/Cargo.toml"))).toEqual([]);
  expect(lines.some(line=>line.startsWith("[graph-repair]")&&line.includes('"moved":true'))).toBe(true);
 }finally{rmSync(l.root,{recursive:true,force:true});}
});

test("a physical registry that still lacks the row is refused typed after the bounded rounds, never as a stable owner failure",async()=>{
 const l=lab();
 try{
  l.put([row("@t/a","a/rust")]);
  let calls=0;
  const error=await settleOwnerRegistryV1(l.root,custody(async()=>{calls++;throw failureOf([dependency]);})).catch(value=>value);
  expect(error.code).toBe("BUILD_OWNER_REGISTRY_INCONSISTENT");expect(error.roots).toEqual([dependency]);
  expect(calls).toBeLessThanOrEqual(5);
 }finally{rmSync(l.root,{recursive:true,force:true});}
});
