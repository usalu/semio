/** 📌️ Contract: a run pinned to the implementation it loaded (the slice worker) keeps that implementation's component graph for its whole run: it never resolves the revision of the tree on disk, so a peer moving or half editing a module of the library cannot make it import what that peer left.
 * @see ../../🟨️.mjs `loadCurrentImplementationV1` */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{withImplementationPinV1}from"../../../../📥️inputs/🔏️revision/🟨️.mjs";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"build-inference",scratch:"checkout"});
const inference=await import("../../🟨️.mjs") as any;
await inference.libraryBootstrap;
const text=readFileSync(join(import.meta.dir,"..","..","🟨️.mjs"),"utf8"),slice=readFileSync(join(import.meta.dir,"..","..","..","..","🏡️resident","🏗️composition","🌐️host","👷️slice-pool","👷️worker.ts"),"utf8");

test("pinned, the implementation of the loaded revision answers and the disk revision is not read; unpinned, the disk revision decides",async()=>{
 const body=text.slice(text.indexOf("export async function loadCurrentImplementationV1"),text.indexOf("/** 🏛️ Supplies current roots"));
 expect(body.indexOf("implementationPinnedV1()")).toBeGreaterThan(-1);
 expect(body.indexOf("implementationPinnedV1()")).toBeLessThan(body.indexOf("implementationRevision()"));
 const first=await withImplementationPinV1(()=>inference.loadCurrentImplementationV1()),second=await withImplementationPinV1(()=>inference.loadCurrentImplementationV1());
 expect(second.revision).toBe(first.revision);expect(second.cacheInternals).toBe(first.cacheInternals);
 expect((await inference.loadCurrentImplementationV1()).revision).toBe(inference.currentImplementationRevisionV1());
});

test("the slice worker runs its whole load inside that pin",()=>{
 expect(slice.indexOf("withImplementationPinV1(()=>routing.receiveScriptProcessInvocation(")).toBeGreaterThan(slice.indexOf("useProcessWideImplementationPinV1();"));
});
