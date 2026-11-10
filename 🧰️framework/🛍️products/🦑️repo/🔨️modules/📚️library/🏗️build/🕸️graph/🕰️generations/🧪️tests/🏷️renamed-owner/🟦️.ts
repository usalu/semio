/** 🏷️ Contract: a project root belongs to exactly one project. When an edit gives the project of a root another name, the generation publishes the new name and drops the owner of the old one, whatever the environment did: an owner that is neither inferred again nor refreshed never survives with the edges of an older environment. */
import {expect,test} from "bun:test";
import {mkdtempSync,realpathSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {dirname,join} from "node:path";
import {fileURLToPath} from "node:url";
import {GENERATION_POLICY_V1,canonicalJsonV1} from "../../🟨️.mjs";
import {applyOps,createWorld,ringFiles,seed} from "../🏃️world/🟦️.ts";

const here=dirname(fileURLToPath(import.meta.url)),oracle=join(here,"../🐍️.py"),python=Bun.which("python3")??Bun.which("python");
const scratch=()=>mkdtempSync(join(realpathSync(tmpdir()),"graph-renamed-owner-"));
const pythonGraph=(dir:string)=>JSON.parse(Bun.spawnSync([python!,"-I",oracle,"graph",dir]).stdout.toString());
const normalized=(value:unknown)=>JSON.parse(canonicalJsonV1(value));

test("the project of a root renamed by an edit replaces the owner of the old name, and the generation equals a full inference",async()=>{
 expect(GENERATION_POLICY_V1).toBeDefined();
 const dir=scratch();
 try{
  await seed(dir,ringFiles(6));
  const world=createWorld(dir),cold=await world.refresh();
  expect(cold.published).toBe(true);
  await applyOps(dir,[{op:"write",path:"pkg/p2/project.json",content:JSON.stringify({name:"p2-renamed",package:"@syn/p2",uses:[]})}]);
  const warm=await world.refresh();
  expect(warm.published).toBe(true);
  const names=Object.keys(warm.current.manifest.owners).sort();
  expect(names).toContain("p2-renamed");expect(names).not.toContain("p2");
  expect(await world.graph(warm.current)).toEqual(normalized((({generation:_g,membership:_m,...rest}:any)=>rest)(pythonGraph(dir))));
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);

test("a crate that only had a manifest and gets a project.json after the generation is published under the new name, the old crate-named owner is dropped, and no owner keeps the edges of an older environment",async()=>{
 const dir=scratch();
 try{
  await seed(dir,{...ringFiles(5),"pkg/crate/Cargo.toml":'[package]\nname = "crate-original"\n'});
  const world=createWorld(dir),cold=await world.refresh();
  expect(Object.keys(cold.current.manifest.owners)).toContain("crate-original");
  await applyOps(dir,[{op:"write",path:"pkg/crate/project.json",content:JSON.stringify({name:"@syn/crate-rs",uses:[]})}]);
  const warm=await world.refresh();
  expect(warm.published).toBe(true);
  const names=Object.keys(warm.current.manifest.owners).sort();
  expect(names).toContain("@syn/crate-rs");expect(names).not.toContain("crate-original");
  for(const row of Object.values(warm.current.manifest.owners))expect((row as {edgesEnv:string}).edgesEnv).toBe(warm.current.manifest.env);
  expect(await world.graph(warm.current)).toEqual(normalized((({generation:_g,membership:_m,...rest}:any)=>rest)(pythonGraph(dir))));
 }finally{rmSync(dir,{recursive:true,force:true});}
},60000);
