/** 💧️ Tests for the spill of a unit whose objects the host store refuses. */
import {expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readdirSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {dropSpillObjectsV1,publishUnitV1,putSpillObjectV1,readSpillObjectV1} from "../../🟦️.ts";

test("a spilled unit keeps its objects out of the host store, reads back by hash, rejects tampering and is dropped after the upload",()=>{
 const root=mkdtempSync(join(tmpdir(),"spill-")),work=mkdtempSync(join(tmpdir(),"spill-work-"));
 try{
  const lib=join(work,"libx.rlib"),dep=join(work,"x.d");writeFileSync(lib,"compiled bytes");writeFileSync(join(work,"x.rs"),"fn main(){}");
  writeFileSync(dep,`${lib}: ${join(work,"x.rs")}\n`);
  const entry=publishUnitV1(root,{key:"ab"+"0".repeat(62),crate:"x",cwd:work,outDir:work,laneRoot:undefined,outputs:[lib,dep],stdout:"",stderr:"warning: x",stampDirectory:undefined,spill:true});
  expect(entry).toBeDefined();
  const sha=createHash("sha256").update("compiled bytes").digest("hex");
  expect(entry!.outputs.find(row=>row.name==="libx.rlib")!.sha256).toBe(sha);
  expect(readSpillObjectV1(root,sha)!.toString()).toBe("compiled bytes");
  expect(readSpillObjectV1(root,"0".repeat(64))).toBeUndefined();
  const spilled=join(root,"w","spill");writeFileSync(join(spilled,sha),"tampered");
  expect(readSpillObjectV1(root,sha)).toBeUndefined();
  putSpillObjectV1(root,"compiled bytes");expect(readSpillObjectV1(root,sha)).toBeDefined();
  dropSpillObjectsV1(root,entry!);
  expect(existsSync(join(spilled,sha))).toBe(false);expect(readdirSync(spilled).filter(name=>!name.endsWith(".tmp")).length).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});rmSync(work,{recursive:true,force:true});}
});
