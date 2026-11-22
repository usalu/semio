import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";

const workspaceRoot=join(import.meta.dir,"../../../../../../../../../../..");

test("the roots a farm child maps to owners are the owner roots and the declared native roots, equal to an independent model, so a crate dependency named by a native root has a build project owner",async()=>{
 const inference=await import("../../../🟨️.mjs"),{projectRootsOfV1}=await import("../../🟨️.mjs");
 await inference.libraryBootstrap;
 const source=JSON.parse(inference.readBuildOwnerRegistrySourceV1(workspaceRoot)),rows=projectRootsOfV1(workspaceRoot,source);
 const scratch=mkdtempSync(join(tmpdir(),"e42-roots-")),registry=join(scratch,"registry.json");
 try{
  writeFileSync(registry,JSON.stringify(source));
  const oracle=spawnSync("python3",["-I",join(import.meta.dir,"🐍️.py"),workspaceRoot,registry],{encoding:"utf8",maxBuffer:1<<28});
  expect(oracle.status,oracle.stderr).toBe(0);
  const expected=new Set((JSON.parse(oracle.stdout) as [string,string][]).map(row=>JSON.stringify(row))),actual=new Set(rows.map(row=>JSON.stringify(row)));
  expect(actual).toEqual(expected);
  expect(rows.length,"at least one project declares a native root of its own").toBeGreaterThan(source.owners.length);
 }finally{rmSync(scratch,{recursive:true,force:true});}
},180000);
