import {afterAll,expect,setDefaultTimeout,test} from "bun:test";
import {createHash,randomBytes} from "node:crypto";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {blobPathV1,copyHashedV1,ensureStoreDirectoryV1,largeBlobBytesV1,storeBlobV1,storeFlushBytesV1,storePathsV1} from "../🟦️.ts";

setDefaultTimeout(120_000);
const scratch:string[]=[];
const temp=(label:string):string=>{const path=mkdtempSync(join(tmpdir(),`upload-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const sha=(data:Buffer):string=>createHash("sha256").update(data).digest("hex");

test("the flush interval of an upload is 32 MiB",()=>{expect(storeFlushBytesV1).toBe(33_554_432);});

test("a copy with a flush interval reports progress at every flush, equals the plain copy and hashes like an independent sha256",async()=>{
 const root=temp("copy"),data=randomBytes(5*1024*1024+17),source=join(root,"source.bin");
 writeFileSync(source,data);
 const marks:number[]=[];
 const flushed=await copyHashedV1(source,join(root,"flushed.bin"),{flushBytes:1024*1024,progress:done=>{marks.push(done);}});
 const plain=await copyHashedV1(source,join(root,"plain.bin"),{});
 expect(flushed).toEqual({sha256:sha(data),bytes:data.length,stable:true});
 expect(plain).toEqual(flushed);
 expect(readFileSync(join(root,"flushed.bin")).equals(data)).toBe(true);
 expect(marks.filter((done,index)=>done===marks[index-1]).length).toBeGreaterThanOrEqual(4);
});

test("storing a blob calls alive while it copies and while it verifies a blob that exists, and stores the same bytes as before",async()=>{
 const store=temp("store"),root=temp("source"),data=randomBytes(3*1024*1024+5),source=join(root,"pack.bin");
 await ensureStoreDirectoryV1(store,storePathsV1(store).tmp);
 writeFileSync(source,data);
 let beats=0;
 const first=await storeBlobV1(store,"w1",source,{alive:()=>{beats++;}});
 expect(first).toMatchObject({sha256:sha(data),bytes:data.length,stable:true,created:true});
 expect(beats).toBeGreaterThanOrEqual(3);
 expect(readFileSync(blobPathV1(store,first.sha256)).equals(data)).toBe(true);
 const known=await storeBlobV1(store,"w1",source,{known:{sha256:first.sha256,bytes:data.length},alive:()=>{beats++;}});
 expect(known.created).toBe(false);
 expect(readdirSync(storePathsV1(store).tmp).filter(name=>!name.startsWith("._"))).toEqual([]);
 expect(existsSync(blobPathV1(store,first.sha256))).toBe(true);
});

test("a large file whose blob the store already holds is neither staged nor copied: the store tmp may be unwritable and the call still reports the blob",async()=>{
 const store=temp("store"),root=temp("source"),data=randomBytes(largeBlobBytesV1+3),source=join(root,"pack.bin"),other=join(root,"again.bin");
 await ensureStoreDirectoryV1(store,storePathsV1(store).tmp);
 writeFileSync(source,data);writeFileSync(other,data);
 const first=await storeBlobV1(store,"w1",source);
 expect(first).toMatchObject({sha256:sha(data),bytes:data.length,stable:true,created:true});
 const tmp=storePathsV1(store).tmp;
 chmodSync(tmp,0o555);
 try{
  let beats=0;
  const again=await storeBlobV1(store,"w2",other,{alive:()=>{beats++;}});
  expect(again).toEqual({sha256:first.sha256,bytes:data.length,stable:true,created:false});
  expect(beats).toBeGreaterThanOrEqual(2);
  writeFileSync(other,Buffer.concat([data,Buffer.from("x")]));
  if(process.getuid?.()!==0)await expect(storeBlobV1(store,"w2",other)).rejects.toThrow();
 }finally{chmodSync(tmp,0o755);}
 expect(readdirSync(tmp).filter(name=>!name.startsWith("._"))).toEqual([]);
});
