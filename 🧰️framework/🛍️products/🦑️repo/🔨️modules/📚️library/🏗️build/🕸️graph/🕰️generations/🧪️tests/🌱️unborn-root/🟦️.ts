/** 🌱️ Contract: a project the reader names that the sealed generation does not know yet (a project created after the generation was built, as every registry change shows) is built into the generation incrementally and then verified in the same call; a name that exists nowhere refuses and builds nothing. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {applyOps,createWorld,seed} from "../🏃️world/🟦️.ts";

const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-unborn-"));
const project=(name:string)=>({[`pkg/${name}/project.json`]:JSON.stringify({name,package:`@syn/${name}`,uses:[]}),[`pkg/${name}/src/index.ts`]:"export const v=1;\n"});

test("a root the generation does not know is built incrementally and the closure is verified in the same call",async()=>{
 const dir=scratch();
 try{
  await seed(dir,{...project("a"),...project("b")});const world=createWorld(dir);await world.refresh();
  await applyOps(dir,[{op:"write",path:"pkg/n/project.json",content:JSON.stringify({name:"n",package:"@syn/n",uses:[]})},{op:"write",path:"pkg/n/src/index.ts",content:"export const v=2;\n"}]);
  const before=world.log.length,result=await world.refreshClosure(["a","n"]);
  expect(result.built&&result.published).toBe(true);expect([...result.names].sort()).toEqual(["a","n"]);
  expect(world.log.slice(before).map(row=>row.full)).toEqual([["n"]]);
  expect(Object.keys((await world.graph(result.current,[...result.names]) as any).projects).sort()).toEqual(["a","n"]);
 }finally{rmSync(dir,{recursive:true,force:true});}
},30000);

test("a root that exists nowhere refuses and builds nothing",async()=>{
 const dir=scratch();
 try{
  await seed(dir,project("a"));const world=createWorld(dir);await world.refresh();
  const before=world.log.length;
  await expect(world.refreshClosure(["a","ghost"])).rejects.toMatchObject({code:"GENERATION_UNKNOWN_OWNER"});
  expect(world.log.length).toBe(before);
 }finally{rmSync(dir,{recursive:true,force:true});}
},30000);
