import{test,expect}from"bun:test";
import{readFileSync}from"node:fs";
import{join}from"node:path";
import{parsePatch,applyPatch}from"diff";
import{receiveScriptProcessInvocation}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{acquireNxPackageSourceV1}from"../📥️acquisition/🟦️.ts";
import{applyToolingPatchImagesV1}from"../../../🩹️patch/🟦️.ts";
/** 📦️ The pinned published package and every derived patch post-image have independent oracles. */
test("actual locked Nx archive and exact first-party patch images",async()=>receiveScriptProcessInvocation(process.env,async original=>{
 const root=process.cwd(),recipe="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools",lock=Bun.JSONC.parse(readFileSync(join(root,recipe,"bun.lock"),"utf8")),manifest=JSON.parse(readFileSync(join(root,recipe,"package.json"),"utf8")),archive=await acquireNxPackageSourceV1(root,original,lock),files=new Map(archive.entries.filter(row=>row.kind==="file").map(row=>[row.path,row.bytes]));
 expect(JSON.parse(Buffer.from(files.get("package.json")!).toString()).version).toBe(manifest.dependencies.nx);const patch=readFileSync(join(root,manifest.semio.toolPatches["nx@"+manifest.dependencies.nx]),"utf8"),images=applyToolingPatchImagesV1(files,patch),independent=parsePatch(patch);expect(images.size).toBe(independent.length);for(const row of independent){const path=row.newFileName!.replace(/^b\//u,"");const expected=applyPatch(Buffer.from(files.get(path)!).toString(),row,{fuzzFactor:0});if(expected===false)throw Error("Independent exact SDK patch failed: "+path);expect(Buffer.from(images.get(path)!).toString(),path).toBe(expected);}console.log("[DEBUG] Actual locked Nx source witness "+JSON.stringify({compressed:archive.compressedBytes,inflated:archive.inflatedBytes,integrity:archive.integrity,entries:archive.entries.length,files:files.size,patchFiles:images.size,maximumFile:Math.max(...[...files.values()].map(row=>row.byteLength)),maximumPath:Math.max(...[...files.keys()].map(path=>Buffer.byteLength(path)))}));
}),120000);
