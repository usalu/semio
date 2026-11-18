/** 📈️ Contract: the cost of one dependency binding is measured, not guessed: the relative-handle layer counts and times every native call by symbol, and the producer prints one profile line per progress step with the wall, snapshot, publish and fence milliseconds. */
import{expect,test}from"bun:test";
import{readFileSync}from"node:fs";
import{fileURLToPath}from"node:url";
import{mkdtempSync,mkdirSync,writeFileSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{takeArtifactRelativeNativeProfileV1,timedArtifactRelativeBodyV1,withArtifactRelativeDirectoryV1}from"../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts";

const source=readFileSync(fileURLToPath(new URL("../../🟦️.ts",import.meta.url)),"utf8");
const layer=readFileSync(fileURLToPath(new URL("../../../../../../../../../../🔨️modules/📁️filesystem/🧾️observation/📁️descriptor/🟦️.ts",import.meta.url)),"utf8");

test("the profile counts calls and milliseconds by name and is drained by every take",()=>{
 takeArtifactRelativeNativeProfileV1();
 expect(timedArtifactRelativeBodyV1("FlushFileBuffers",()=>7)).toBe(7);timedArtifactRelativeBodyV1("FlushFileBuffers",()=>0);
 expect(()=>timedArtifactRelativeBodyV1("NtCreateFile",()=>{throw Error("x");})).toThrow("x");
 const taken=takeArtifactRelativeNativeProfileV1();
 expect(Object.keys(taken).sort()).toEqual(["FlushFileBuffers","NtCreateFile"]);expect(taken.FlushFileBuffers![0]).toBe(2);expect(taken.NtCreateFile![0]).toBe(1);
 expect(takeArtifactRelativeNativeProfileV1()).toEqual({});
});

test("a real POSIX walk through the relative layer reports its native calls by symbol",async()=>{
 const root=mkdtempSync(join(tmpdir(),"semio-r79-"));
 try{mkdirSync(join(root,"d"));writeFileSync(join(root,"d","a.txt"),"a");takeArtifactRelativeNativeProfileV1();
  await withArtifactRelativeDirectoryV1(root,()=>{},async top=>{const names=await top.names();expect(names).toEqual(["d"]);await top.withDirectory("d",async child=>{expect(await child.entry("a.txt")).not.toBeNull();});});
  const taken=takeArtifactRelativeNativeProfileV1(),total=Object.values(taken).reduce((sum,row)=>sum+row[0],0);
  expect(taken.openat![0]).toBeGreaterThanOrEqual(3);expect(total).toBeGreaterThan(8);expect(total).toBeLessThan(200);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("every Windows native symbol of the relative layer is counted and the producer prints the profile line",()=>{
 expect(layer).toContain("symbols:timedArtifactRelativeSymbolsV1(library.symbols");
 expect(layer).toContain('timedArtifactRelativeBodyV1("js:facts"');
 const from=source.indexOf("await publishJavascriptDependencyPreservingSourceDescriptorV1(binding"),to=source.indexOf("await verifyReady();const finals"),loop=source.slice(from,to);
 expect(loop).toContain('purpose:"binding-profile"');expect(loop).toContain("native:takeArtifactRelativeNativeProfileV1()");
 expect(source).toContain('purpose:"payload-profile"');expect(layer).toContain("fstatSync as fstatSyncRaw");expect(layer).toContain("const calls=timedArtifactRelativeSymbolsV1(library.symbols");
 for(const field of["wallMs","snapshotMs","publishMs","fenceMs","changed"])expect(loop).toContain(field+":");
});
