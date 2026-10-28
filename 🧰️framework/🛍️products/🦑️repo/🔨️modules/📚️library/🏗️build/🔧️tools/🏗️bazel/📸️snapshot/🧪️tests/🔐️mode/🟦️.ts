/** 🔐️ Law: a captured copy is not reused when only the executable bit of the live file changed (a chmod keeps mtime, inode and size); the copy follows the live file in both directions, and an unchanged file is still reused. */
import {expect,test} from "bun:test";
import {chmodSync,lstatSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {captureBazelSourceSnapshotV1} from "../../🟦️.ts";

test("the executable bit of a live file reaches its captured copy in both directions",async()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"snapshot-mode-"))),root=join(base,"live"),directory=join(base,"ws");
 mkdirSync(join(root,"src"),{recursive:true});mkdirSync(directory);writeFileSync(join(root,"src/tool.sh"),"#!/bin/sh\n");
 const capture=()=>captureBazelSourceSnapshotV1({root,directory,files:["src/tool.sh"],views:{alias:"a",real:"s"},alias:folder=>"d"+folder.length,check:()=>{},progress:async()=>{},now:()=>Date.now()+10_000_000});
 const executable=()=>(lstatSync(join(directory,"a/d3/tool.sh")).mode&0o111)!==0;
 try{
  expect((await capture()).captured).toBe(1);expect(executable()).toBe(false);
  expect(await capture()).toMatchObject({captured:0,reused:1});
  chmodSync(join(root,"src/tool.sh"),0o755);expect(await capture()).toMatchObject({captured:1,reused:0});expect(executable()).toBe(true);
  chmodSync(join(root,"src/tool.sh"),0o644);expect(await capture()).toMatchObject({captured:1});expect(executable()).toBe(false);
 }finally{rmSync(base,{recursive:true,force:true});}
});
