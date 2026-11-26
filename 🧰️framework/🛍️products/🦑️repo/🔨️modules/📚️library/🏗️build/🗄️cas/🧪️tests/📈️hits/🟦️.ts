/** 📈️ Contract: the host store counts, one byte per event in its counter directory, the objects it found (`get`, `read`, `link`) and the ones it did not, so a sampler reads the local hits of the machine from the size of a file; `has` and `put` are no lookups.
 * @see ../../🗃️store/🟦️.ts */
import{afterAll,expect,test}from"bun:test";
import{mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{readEventCountsV1}from"../../../📈️counter-ledger/🟦️.ts";
import{casCounterDirectoryV1,createCasStoreV1}from"../../🗃️store/🟦️.ts";

const scratch:string[]=[];
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

test("every lookup that found its object counts a local hit and every one that did not a local miss",async()=>{
 const root=mkdtempSync(join(tmpdir(),"cas-hits-"));scratch.push(root);
 const store=createCasStoreV1({root}),digest=await store.put(new TextEncoder().encode("counted")),absent="0".repeat(64);
 const read=()=>readEventCountsV1(casCounterDirectoryV1(root),["localHit","localMiss"]);
 expect(read().counts).toEqual({localHit:0,localMiss:0});
 expect(await store.has([digest.hash,absent])).toEqual([true,false]);expect(read().counts).toEqual({localHit:0,localMiss:0});
 await store.get(digest.hash);await store.get(digest.hash);await store.read(digest.hash);
 expect(read().counts).toEqual({localHit:3,localMiss:0});
 expect(await store.get(absent)).toBeUndefined();expect(await store.read(absent)).toBeUndefined();expect(await store.link(absent,join(root,"out","a"))).toBeUndefined();
 expect(await store.link(digest.hash,join(root,"out","b"))).toBeDefined();
 expect(read().counts).toEqual({localHit:4,localMiss:3});expect(read().sinceMs).toBeGreaterThan(0);
});
