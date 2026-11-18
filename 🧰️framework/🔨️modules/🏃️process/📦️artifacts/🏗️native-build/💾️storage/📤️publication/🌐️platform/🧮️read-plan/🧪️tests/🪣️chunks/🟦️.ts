import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash,randomBytes} from "node:crypto";
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {copyProtectedArtifactInputV1,protectedArtifactInputChunkV1,readProtectedArtifactInputV1} from "../../🟦️.ts";
import {withArtifactRelativeDirectoryV1} from "@semio-tech/framework-filesystem/observation/descriptor";
import {localMachineSpeedFactorV1} from "../../../../../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🐢️speed/🟨️.mjs";
import fixture from "./🔣️.json" with {type:"json"};

const MIB=1048576,OLD={encoded:65536,decoded:49152};

const sample=(bytes:number)=>{const buffer=randomBytes(bytes);return{buffer,data:buffer.toString("base64"),sha256:createHash("sha256").update(buffer).digest("hex")};};

class CountingFile{
 writes=0;stats=0;checks=0;steps=0;bytes=0;simulatedMs=0;position=0;
 constructor(readonly path:string,readonly cost:{write:number;stat:number;check:number},readonly limit=Number.POSITIVE_INFINITY){}
 write(buffer:Uint8Array,offset:number,length:number,position:number):number{
  if(position!==this.position)throw Error("contiguous");
  const count=Math.min(length,this.limit);
  this.writes++;this.stats++;this.bytes+=count;this.position+=count;this.simulatedMs+=this.cost.write+this.cost.stat;
  return count;
 }
 get calls(){return this.writes+this.stats+this.checks+this.steps;}
}
const ports=(file:CountingFile,after?:()=>void)=>({check(){file.checks++;file.simulatedMs+=file.cost.check;after?.();},async step(){file.steps++;}});
const POSIX={write:0.05,stat:0.01,check:0.001},WINDOWS={write:20,stat:50,check:3};

test("the chunk of a batch equals the independent Python model",()=>{
 const run=spawnSync("python3",["-I",join(import.meta.dir,"oracle.py"),join(import.meta.dir,"🔣️.json")],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const model=JSON.parse(run.stdout) as {encoded:number;decoded:number}[];
 fixture.plans.forEach((c,at)=>expect({...protectedArtifactInputChunkV1(c.totalBytes,c.freeMemoryBytes)},c.name).toEqual(model[at]!));
});

test("a chunk is megabytes, base64 aligned, and shrinks only with the batch or the memory",()=>{
 const big=protectedArtifactInputChunkV1(165_000_000,34359738368);
 expect(big.decoded).toBeGreaterThanOrEqual(4*MIB);expect(big.decoded).toBeLessThanOrEqual(8*MIB);
 for(const c of fixture.plans){const chunk=protectedArtifactInputChunkV1(c.totalBytes,c.freeMemoryBytes);expect(chunk.encoded%4).toBe(0);expect(chunk.decoded).toBe(chunk.encoded/4*3);expect(chunk.decoded).toBeGreaterThanOrEqual(196608);expect(chunk.decoded).toBeLessThanOrEqual(8*MIB);}
 expect(protectedArtifactInputChunkV1(165_000_000,536870912).decoded).toBeLessThan(big.decoded);
});

test("calls of the file layer per MB of a 64 MiB input stay under a small constant on a POSIX-shaped and a Windows-shaped file",async()=>{
 const input=sample(8*MIB),chunk=protectedArtifactInputChunkV1(64*MIB,34359738368);
 for(const [name,cost,path] of [["posix",POSIX,"/var/store/j/a/w/f.bin"],["windows",WINDOWS,"C:\\\\Users\\\\worker\\\\Store\\\\j\\\\a\\\\w\\\\f.bin"]] as const){
  const files:CountingFile[]=[];
  for(let at=0;at<8;at++){const file=new CountingFile(path,cost);files.push(file);expect(await copyProtectedArtifactInputV1(file,input.data,chunk,ports(file))).toBe(input.buffer.length);}
  const calls=files.reduce((sum,f)=>sum+f.calls,0),writes=files.reduce((sum,f)=>sum+f.writes,0);
  expect(calls/64,name).toBeLessThan(4);expect(writes/64,name).toBeLessThan(1);
 }
});

test("the old 64 KiB step breaks the call-count law and the new one does not (red before the change)",async()=>{
 const input=sample(8*MIB),count=async(chunk:{encoded:number})=>{const file=new CountingFile("C:\\\\w\\\\f.bin",WINDOWS);await copyProtectedArtifactInputV1(file,input.data,chunk,ports(file));return file;};
 const old=await count(OLD),derived=await count(protectedArtifactInputChunkV1(64*MIB,34359738368));
 expect(old.calls/8).toBeGreaterThan(4);expect(derived.calls/8).toBeLessThan(4);
 expect(old.calls).toBeGreaterThan(40*derived.calls);
});

test("165 MB through a Windows-shaped file with a 50 ms stat costs seconds, not hours",async()=>{
 const input=sample(8*MIB),chunk=protectedArtifactInputChunkV1(165_000_000,34359738368);
 let total=0;
 for(let at=0;at<20;at++){const file=new CountingFile("C:\\\\w\\\\f.bin",WINDOWS);await copyProtectedArtifactInputV1(file,input.data,chunk,ports(file));total+=file.simulatedMs;}
 expect(total/1000).toBeLessThan(10);
 const old=new CountingFile("C:\\\\w\\\\f.bin",WINDOWS);await copyProtectedArtifactInputV1(old,input.data,OLD,ports(old));
 expect(old.simulatedMs*20/1000).toBeGreaterThan(60);
});

test("short writes are continued and a write of nothing is refused",async()=>{
 const input=sample(3*MIB),chunk=protectedArtifactInputChunkV1(3*MIB,34359738368),file=new CountingFile("f",POSIX,1*MIB);
 expect(await copyProtectedArtifactInputV1(file,input.data,chunk,ports(file))).toBe(input.buffer.length);expect(file.bytes).toBe(input.buffer.length);
 const stuck=new CountingFile("f",POSIX,0);
 await expect(copyProtectedArtifactInputV1({write:()=>0},input.data,chunk,ports(stuck))).rejects.toThrow("made no progress");
});

test("cancellation is checked before every call of the file and stops at the next step",async()=>{
 const input=sample(8*MIB),chunk={encoded:2**18,decoded:196608},file=new CountingFile("f",POSIX);
 let seen=0;
 await expect(copyProtectedArtifactInputV1(file,input.data,chunk,ports(file,()=>{if(++seen===5)throw Error("cancelled");}))).rejects.toThrow("cancelled");
 expect(file.writes).toBe(4);expect(file.steps).toBe(4);expect(file.bytes).toBe(4*chunk.decoded);
});

test("a run cut off at any step and run again from the start leaves the identical digest",async()=>{
 const input=sample(5*MIB+12345),chunk={encoded:2**18,decoded:196608},steps=Math.ceil(input.buffer.length/chunk.decoded);
 for(const cut of[1,2,steps-1,steps]){
  let seen=0;const partial=new CountingFile("f",POSIX);
  await copyProtectedArtifactInputV1(partial,input.data,chunk,ports(partial,()=>{if(++seen===cut+1)throw Error("killed");})).catch(()=>{});
  expect(partial.bytes).toBe(Math.min(cut*chunk.decoded,input.buffer.length));
  const rerun=new CountingFile("f",POSIX),hash=createHash("sha256");
  const written:Buffer[]=[];
  const sink={write(buffer:Uint8Array,offset:number,length:number,position:number){written.push(Buffer.from(buffer.subarray(offset,offset+length)));return rerun.write(buffer,offset,length,position);}};
  await copyProtectedArtifactInputV1(sink,input.data,chunk,ports(rerun));
  for(const part of written)hash.update(part);
  expect(hash.digest("hex")).toBe(input.sha256);expect(rerun.steps).toBe(steps);
 }
});

test("a real file is written and read back with the same digest at every chunk, in megabyte steps, at the speed of the machine",async()=>{
 const base=mkdtempSync(join(tmpdir(),"r150-")),input=sample(8*MIB),factor=localMachineSpeedFactorV1();
 try{
  const derived=protectedArtifactInputChunkV1(64*MIB,34359738368),checkpoints:Record<string,number>={};
  const results:Record<string,number>={};
  for(const [name,chunk] of [["derived",derived],["old",OLD]] as const){
   let count=0;const t0=performance.now();
   await withArtifactRelativeDirectoryV1(base,()=>{count++;},async root=>{
    for(let at=0;at<8;at++){
     await root.withNewFile(name+at,async file=>{
      const port={check(){},async step(){}};
      expect(await copyProtectedArtifactInputV1(file,input.data,chunk,port)).toBe(input.buffer.length);file.sync();
     });
    }
    await root.sync();
    for(let at=0;at<8;at++)await root.withFile(name+at,async file=>{
     const hash=createHash("sha256");
     expect(await readProtectedArtifactInputV1(file,input.buffer.length,chunk,piece=>hash.update(piece),{check(){},async step(){}})).toBe(input.buffer.length);
     expect(hash.digest("hex")).toBe(input.sha256);
    });
   });
   const ms=performance.now()-t0;checkpoints[name]=count;results[name]=64*2/(ms/1000);
   for(let at=0;at<8;at++)expect(createHash("sha256").update(readFileSync(join(base,name+at))).digest("hex")).toBe(input.sha256);
  }
  expect(checkpoints.derived!/16).toBeLessThan(100);
  expect(checkpoints.old!).toBeGreaterThan(10*checkpoints.derived!);
  expect(results.derived!).toBeGreaterThanOrEqual(50/factor);
 }finally{rmSync(base,{recursive:true,force:true});}
});

test("the platform stages and reads back through the chunk plan and keeps its progress lines",()=>{
 const text=readFileSync(join(import.meta.dir,"..","..","..","🟦️.ts"),"utf8");
 expect(text.split("copyProtectedArtifactInputV1(file,input.data,chunk,").length-1).toBe(2);
 expect(text).toContain("readProtectedArtifactInputV1(file,input.bytes,chunk,");
 expect(text).not.toContain("offset+=limits.chunkBytes");
 expect(text).not.toContain("Buffer.alloc(limits.chunkBytes);let offset=0;while(offset<input.bytes)");
 for(const step of["native-artifact-input-stage:","native-artifact-input-read:","native-artifact-input-verification:"])expect(text).toContain(step);
});
