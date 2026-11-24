/** 🧭️ Law: the first moved row of a stale closure entry is found in its own rows, then its layers, in order, within a time budget, and only a row that really moved is named; the failure text of a closure verification names the recipe and the cause, never only `closure-probe`.
 * @see ../../🟨️.mjs `firstMovedRowOfEntryV1` `closureFailureWhyV1` */
import{expect,test}from"bun:test";
import{mkdtempSync,realpathSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";

const{firstMovedRowOfEntryV1,closureFailureWhyV1}=await import("../../🟨️.mjs");
const observation=await import("../../../../../👁️observation/🟨️.mjs");
const root=mkdtempSync(join(realpathSync(tmpdir()),"moved-row-")),files=["a","b","c","d"].map(name=>join(root,name));
files.forEach((file,at)=>writeFileSync(file,"content "+at));
await new Promise(done=>setTimeout(done,2200));
const rowsOf=async(paths:string[])=>(await observation.observeDetachedV1(async()=>{for(const path of paths)observation.observedFs.readFileSync(path,"utf8");})).rows;

test("an entry whose rows all hold names no row, an edited layer file is named, an edited own row wins over a later layer row",async()=>{
 const own=await rowsOf(files.slice(0,2)),layer=await rowsOf(files.slice(2));
 const entry={rows:own,layers:[layer]};
 expect(firstMovedRowOfEntryV1(entry)).toBeUndefined();
 writeFileSync(files[3],"moved layer file, longer");
 expect(firstMovedRowOfEntryV1(entry)?.path).toBe(files[3]);
 writeFileSync(files[1],"moved own file, longer still");
 expect(firstMovedRowOfEntryV1(entry)?.path).toBe(files[1]);
});

test("a stored entry is searched layer by layer through its store, and a spent budget ends the search without a name",async()=>{
 const fresh=join(root,"e");writeFileSync(fresh,"stored content");await new Promise(done=>setTimeout(done,2200));
 const layer=await rowsOf([fresh]);writeFileSync(fresh,"stored content moved, longer");
 const store={layerBatchesSync:function*(_digest:string,_size:number){yield layer;}},entry={rows:[],digests:["d1"],store};
 expect(firstMovedRowOfEntryV1(entry)?.path).toBe(fresh);
 expect(firstMovedRowOfEntryV1(entry,-1)).toBeUndefined();
 expect(firstMovedRowOfEntryV1({rows:[],digests:["lost"],store:{layerBatchesSync:()=>{throw Object.assign(Error("gone"),{code:"CLOSURE_LAYER_LOST"});}}})).toBeUndefined();
});

test("the failure text names the recipe and the cause: a stale entry of a take with its moved file, any other failure with its code and first line",()=>{
 expect(closureFailureWhyV1(Object.assign(Error("miss"),{code:"BUILD_TAKE_MISS",movedRow:{kind:"file",path:"/w/x/🟦️.ts"}}),"abcdef0123456789")).toBe("closure-entry-stale (recipe abcdef012345, moved file /w/x/🟦️.ts)");
 expect(closureFailureWhyV1(Object.assign(Error("miss"),{code:"BUILD_TAKE_MISS"}),"abcdef0123456789")).toBe("closure-entry-stale (recipe abcdef012345, no moved row found)");
 expect(closureFailureWhyV1(Object.assign(Error("a read parent derives nothing\nsecond line"),{code:"BUILD_READ_PARENT"}),"abcdef0123456789")).toBe("closure-probe BUILD_READ_PARENT: a read parent derives nothing (recipe abcdef012345)");
 expect(closureFailureWhyV1(new TypeError("boom"),"k")).toBe("closure-probe TypeError: boom (recipe k)");
});
