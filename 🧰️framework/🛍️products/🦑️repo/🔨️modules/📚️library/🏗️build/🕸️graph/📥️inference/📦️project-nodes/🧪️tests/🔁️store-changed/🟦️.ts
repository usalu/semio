/** 🫏️ Contract: an owner whose inference met a store that changed under it is requeued once and set aside with its reason when it changes again; every other failure is not the reader's to absorb. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{ownerStoreChangeV1}from"../../🟨️.mjs";

const source=readFileSync(new URL("../../🟨️.mjs",import.meta.url),"utf8");

test("the first change requeues the owner, the second sets it aside, other owners are judged on their own",()=>{
 const seen=new Set<string>(),layer=Object.assign(Error("layers changed"),{storeChanged:true}),inventory=Object.assign(Error("absence changed"),{code:"BUILD_STORE_CHANGED"});
 expect(ownerStoreChangeV1(layer,"a",seen)).toBe("requeue");
 expect(ownerStoreChangeV1(inventory,"b",seen)).toBe("requeue");
 expect(ownerStoreChangeV1(inventory,"a",seen)).toBe("aside");
 expect(ownerStoreChangeV1(layer,"b",seen)).toBe("aside");
 expect([...seen].sort()).toEqual(["a","b"]);
});

test("any other failure is not absorbed",()=>{
 const seen=new Set<string>();
 for(const error of[Error("disk on fire"),Object.assign(Error("x"),{code:"graph.farm-job-failed"}),undefined,null,"text"])expect(ownerStoreChangeV1(error,"a",seen)).toBeUndefined();
 expect(seen.size).toBe(0);
});

test("the owner loop absorbs the change with the owner it runs, in both languages, and sets it aside like a failed farm job",()=>{
 expect(source).toContain("deferOwner(error)||failedOwner(error)||changedOwner(error,job)||");
 expect(source).toContain("const verdict=ownerStoreChangeV1(error,owner.name,changedOnce)");
 expect(source).toContain("it is requeued once / Projekt ");expect(source).toContain("), der Lauf geht weiter und nennt es am Ende");
 expect(source).toContain(".catch(error=>{if(!setAside(error,job))throw error;});");
});
