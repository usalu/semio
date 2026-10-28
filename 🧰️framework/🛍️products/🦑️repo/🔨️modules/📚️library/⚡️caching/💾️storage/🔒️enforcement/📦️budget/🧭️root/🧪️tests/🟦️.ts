/** 🧪️ Tests for the checkout root guard of the cache partitions. */
import {spawnSync} from "node:child_process";
import {mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {expect,test} from "bun:test";
import {repositoryCheckoutRootV1,requireRepositoryWorkspaceRootV1} from "../🟦️.ts";

const base=realpathSync(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir());
const git=(cwd:string,...args:string[])=>spawnSync("git",["-c","core.quotepath=off",...args],{cwd,encoding:"utf8"});

test("the guard finds the same checkout root as git and refuses exactly the nested directories",()=>{
 const checkout=realpathSync(mkdtempSync(join(base,"rg-"))),outside=realpathSync(mkdtempSync(join(base,"rg-")));
 try{
  expect(git(checkout,"init","-q").status).toBe(0);
  writeFileSync(join(checkout,"📋️project.json"),"{}");writeFileSync(join(checkout,"package.json"),JSON.stringify({workspaces:["x/*"]}));
  const nested=[join(checkout,"x"),join(checkout,"x","y"),join(checkout,"🧰️framework")],scratch=[join(checkout,".cache","z"),join(checkout,".tmp","q"),join(checkout,".🧬semio","r")];
  for(const path of [...nested,...scratch])mkdirSync(path,{recursive:true});
  mkdirSync(join(checkout,"proj"));writeFileSync(join(checkout,"proj","📋️project.json"),"{}");writeFileSync(join(checkout,"proj","package.json"),JSON.stringify({name:"p"}));
  for(const path of [checkout,...nested,...scratch,join(checkout,"proj")]){
   const independent=realpathSync(git(path,"rev-parse","--show-toplevel").stdout.trim());
   expect(repositoryCheckoutRootV1(path),path).toBe(independent);
  }
  expect(requireRepositoryWorkspaceRootV1(checkout)).toBe(checkout);
  for(const path of scratch)expect(requireRepositoryWorkspaceRootV1(path),path).toBe(path);
  expect(repositoryCheckoutRootV1(outside)).toBeNull();expect(requireRepositoryWorkspaceRootV1(outside)).toBe(outside);
  for(const path of [...nested,join(checkout,"proj")]){
   expect(()=>requireRepositoryWorkspaceRootV1(path),path).toThrow("belongs to the checkout root "+checkout);
   expect(()=>requireRepositoryWorkspaceRootV1(path),path).toThrow("gehört zur Checkout-Wurzel "+checkout);
  }
 }finally{rmSync(checkout,{recursive:true,force:true});rmSync(outside,{recursive:true,force:true});}
});

test("the partition opener calls the guard before it reads the budget or touches a directory",()=>{
 const text=readFileSync(new URL("../../📥️opening/🟦️.ts",import.meta.url),"utf8");
 expect(text).toMatch(/export async function openRepositoryCachePartitionAttemptV1\([^)]*\)[^{]*\{\s*checkResourceLeaseControl\(control\);requireRepositoryWorkspaceRootV1\(workspace\);const budget=/u);
});
