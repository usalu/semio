import{expect,test}from"bun:test";
import{AsyncLocalStorage,AsyncResource}from"node:async_hooks";
import{readFileSync}from"node:fs";
import{join}from"node:path";

/** 🧵️ Law: an inference loop that the farm starts later (its `onReady` callback runs in the farm's own async context) keeps the async context of the graph writer that created it; without the binding the loop's first `ownerCachePathV1` threw "Original bounded graph writer custody required" on every machine whose farm became ready after the first loops (g1-m03, 2026-11-14). */
test("a loop started from a foreign callback keeps the creator's async context when it is bound, and loses it when it is not",async()=>{
 const writer=new AsyncLocalStorage<string>(),callbacks:(()=>void)[]=[],seen:(string|undefined)[]=[];
 writer.run("writer",()=>{const loop=()=>{seen.push(writer.getStore());},grow=AsyncResource.bind(()=>loop());callbacks.push(()=>loop(),()=>grow());});
 await new Promise<void>(resolve=>setTimeout(()=>{for(const callback of callbacks)callback();resolve();},1));
 expect(seen).toEqual([undefined,"writer"]);
});

test("the farm's ready callback starts loops through the bound starter",()=>{
 const source=readFileSync(join(import.meta.dir,"..","..","🟨️.mjs"),"utf8");
 expect(source).toMatch(/grow=AsyncResource\.bind\(\(\)=>startLoop\([^)]*\)\)/u);
 expect(source).toMatch(/farm\?\.onReady\(loops=>\{[^}]*workers\.push\(grow\(\)\)/u);
 expect(source).not.toMatch(/farm\?\.onReady\(loops=>\{[^}]*workers\.push\(loop\(/u);
});
