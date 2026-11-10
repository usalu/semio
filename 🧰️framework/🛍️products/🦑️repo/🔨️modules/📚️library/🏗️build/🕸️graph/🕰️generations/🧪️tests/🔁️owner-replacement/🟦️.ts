/** 🔁️ Contract: a config added at the root an owner already holds through a different config replaces that owner (either direction); a config added in a nested directory leaves the parent owner alone. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {applyOps,createWorld,seed} from "../🏃️world/🟦️.ts";

const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-replace-"));
const config=(name:string,primary=false)=>JSON.stringify({name,package:`@syn/${name}`,uses:[],...(primary?{primary}:{})});
const source=(root:string)=>({[`${root}/src/index.ts`]:"export const v=1;\n"});
const names=async(world:ReturnType<typeof createWorld>)=>Object.keys((await world.refresh()).current!.manifest.owners).sort();

test("a project.json added over a module.json owner replaces it",async()=>{
 const dir=scratch();
 try{
  await seed(dir,{"pkg/a/module.json":config("a"),...source("pkg/a"),"pkg/k/project.json":config("k"),...source("pkg/k")});const world=createWorld(dir);await world.refresh();
  await applyOps(dir,[{op:"write",path:"pkg/a/project.json",content:config("a2",true)}]);
  expect(await names(world)).toEqual(["a2","k"]);
 }finally{rmSync(dir,{recursive:true,force:true});}
},30000);

test("a module.json added over a project.json owner replaces it",async()=>{
 const dir=scratch();
 try{
  await seed(dir,{"pkg/b/project.json":config("b"),...source("pkg/b"),"pkg/k/project.json":config("k"),...source("pkg/k")});const world=createWorld(dir);await world.refresh();
  await applyOps(dir,[{op:"write",path:"pkg/b/module.json",content:config("b2",true)}]);
  expect(await names(world)).toEqual(["b2","k"]);
 }finally{rmSync(dir,{recursive:true,force:true});}
},30000);

test("a nested crate keeps its parent owner",async()=>{
 const dir=scratch();
 try{
  await seed(dir,{"pkg/p/project.json":config("p"),...source("pkg/p"),"pkg/k/project.json":config("k"),...source("pkg/k")});const world=createWorld(dir);await world.refresh();
  await applyOps(dir,[{op:"write",path:"pkg/p/q/module.json",content:config("q")},{op:"write",path:"pkg/p/q/src/index.ts",content:"export const v=3;\n"}]);
  expect(await names(world)).toEqual(["k","p","q"]);
 }finally{rmSync(dir,{recursive:true,force:true});}
},30000);
