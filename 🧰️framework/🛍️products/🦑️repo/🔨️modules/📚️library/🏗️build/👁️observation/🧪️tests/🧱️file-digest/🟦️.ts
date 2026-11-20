import {expect,spyOn,test} from "bun:test";
import fs from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {observationsHold} from "../../🟨️.mjs";
import fixture from "./🔣️.json";
import schema from "../../🧬️schema/🔣️.json";

test("freshly stamped physical files validate the independent SHA-256 vectors with bounded reads",async()=>{
 const directory=fs.mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"file-digest-"));
 const path=resolve(directory,"source"),chunks=spyOn(fs.ReadStream.prototype,"push");
 try{
  for(const {size,digest} of fixture.cases){
   fs.writeFileSync(path,Buffer.alloc(size,120));
   const fact={size,mtimeMs:-1,ctimeMs:-1,ino:-1,sha256:digest};
   expect(await observationsHold([{kind:"file",path,fact}])).toBe(true);
   expect(await observationsHold([{kind:"file",path,fact:{...fact,sha256:"0".repeat(64)}}])).toBe(false);
  }
  const sizes=chunks.mock.calls.map(([chunk])=>chunk?.byteLength??0);
  expect(sizes.reduce((sum,size)=>sum+size,0)).toBe(2*fixture.cases.reduce((sum,row)=>sum+row.size,0));
  expect(Math.max(...sizes)).toBeLessThanOrEqual(schema.$defs.limits.const.fileDigestChunkBytes);
 }finally{chunks.mockRestore();fs.rmSync(directory,{recursive:true,force:true});}
});

for(const event of ["growth","cancel"] as const)test(`a stamp-drift recheck observes ${event} during a physical read`,async()=>{
 const directory=fs.mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"file-digest-"));
 const path=resolve(directory,"source"),{size,digest}=fixture.cases.at(-1)!;
 const controller=new AbortController(),reason=Error("cancel digest validation"),original=fs.ReadStream.prototype.push;
 let touched=false;
 const chunks=spyOn(fs.ReadStream.prototype,"push").mockImplementation(function(this:fs.ReadStream,chunk,encoding){
  if(!touched&&chunk&&this.path===path){
   touched=true;
   if(event==="growth")fs.appendFileSync(path,"y");else controller.abort(reason);
  }
  return original.call(this,chunk,encoding);
 });
 try{
  fs.writeFileSync(path,Buffer.alloc(size,120));
  const pending=observationsHold([{kind:"file",path,fact:{size,mtimeMs:-1,ctimeMs:-1,ino:-1,sha256:digest}}],1,new Set(),new Set(),controller.signal);
  if(event==="growth")expect(await pending).toBe(false);
  else expect(await pending.catch(error=>error)).toBe(reason);
  expect(touched).toBe(true);
 }finally{chunks.mockRestore();fs.rmSync(directory,{recursive:true,force:true});}
});
