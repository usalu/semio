/** 🕓️ Contract: a crate manifest without a build project owner is pending (named, never stale, never an inference failure); a refused inference names the roots to adopt through the registry reconciler. */
import {expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {adoptPendingBuildOwnersV1,pendingCrateManifestV1,pendingOwnerRootsV1} from "../../../🕸️graph/🟦️.ts";
import {pendingCratePathsV1} from "../../../🕸️graph/🕰️generations/🕓️pending-crates/🟨️.mjs";

const registry=(owners:object[])=>({version:1,owners,catalogProviders:[]}) as never;
const owner={name:"@t/a",root:"a/rust",manifest:"a/rust/Cargo.toml",packages:[],nativeRoots:["a/native"]};
const files:Record<string,string>={"a/rust/Cargo.toml":"[package]\nname = \"a\"\n","a/native/Cargo.toml":"[package]\nname = \"n\"\n","b/rust/Cargo.toml":"[package]\nname = \"b\"\n","Cargo.toml":"[workspace]\nmembers = []\n","b/package.json":"{}"};
const read=async(path:string)=>files[path];

test("only an unowned, non-workspace crate manifest is pending",async()=>{
 const rows=registry([owner]);
 expect(await pendingCrateManifestV1(rows,"b/rust/Cargo.toml",read)).toBe(true);
 expect(await pendingCrateManifestV1(rows,"a/rust/Cargo.toml",read)).toBe(false);
 expect(await pendingCrateManifestV1(rows,"a/native/Cargo.toml",read)).toBe(false);
 expect(await pendingCrateManifestV1(rows,"Cargo.toml",read)).toBe(false);
 expect(await pendingCrateManifestV1(rows,"b/package.json",read)).toBe(false);
 expect(await pendingCrateManifestV1(rows,"b/gone/Cargo.toml",read)).toBe(false);
});

test("a pending change below an owner root belongs to that owner and a directory is never pending",()=>{
 const ownerOf=(path:string)=>path.startsWith("a/")?"a":undefined;
 const changes=[{path:"a/x/Cargo.toml",kind:"added" as const,pending:true},{path:"b/Cargo.toml",kind:"added" as const,pending:true},{path:"b",kind:"added" as const,directory:true,pending:true},{path:"c/project.json",kind:"added" as const}];
 expect([...pendingCratePathsV1(changes,ownerOf)]).toEqual(["b/Cargo.toml"]);
});

test("a refusal that names crates without an owner row yields exactly their roots, through any wrapper",()=>{
 expect(pendingOwnerRootsV1("Native dependency has no build project owner: m/rust (required by p:build; candidates=12)")).toEqual(["m/rust"]);
 expect(pendingOwnerRootsV1("farm: worker threw Error: Canonical native dependency owner required: m/rust\n at x\nNative dependency has no build project owner: n/rust (required by q:test; candidates=3)")).toEqual(["m/rust","n/rust"]);
 expect(pendingOwnerRootsV1("something else")).toEqual([]);
 expect(adoptPendingBuildOwnersV1("/nonexistent",  "something else")).toBe(false);
});

test("the reconciler adopts a crate that has a manifest and no row, once",()=>{
 const root=mkdtempSync(join(realpathSync(tmpdir()),"pending-crate-"));
 try{
  const write=(path:string,text:string)=>{mkdirSync(dirname(join(root,path)),{recursive:true});writeFileSync(join(root,path),text);};
  write("📋️build-owners.json",JSON.stringify({version:1,owners:[],catalogProviders:[]}));
  write("m/rust/Cargo.toml","[package]\nname = \"m-crate\"\n");
  expect(adoptPendingBuildOwnersV1(root,"Native dependency has no build project owner: m/rust (required by p:build; candidates=0)")).toBe(true);
  expect(JSON.parse(readFileSync(join(root,"📋️build-owners.json"),"utf8")).owners.map((row:{root:string})=>row.root)).toEqual(["m/rust"]);
  expect(adoptPendingBuildOwnersV1(root,"Native dependency has no build project owner: m/rust (required by p:build; candidates=1)")).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});
