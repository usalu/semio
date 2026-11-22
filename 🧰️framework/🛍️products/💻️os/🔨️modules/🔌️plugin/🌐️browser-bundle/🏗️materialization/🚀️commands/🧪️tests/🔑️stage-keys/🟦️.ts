import{afterAll,expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{mkdirSync,mkdtempSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{stageKeyDigestV1}from"../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/🗃️stage-cache/🟦️.ts";
import{artifactFiles,fileDigest}from"../../../../📦️distribution/📋️inventory/🟦️.ts";
import{optimizeStageRequestV1,probeStageRequestV1,transpileStageRequestV1}from"../../🟦️.ts";
const roots:string[]=[],h=(text:string)=>createHash("sha256").update(text).digest("hex");
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
async function treeInputs(root:string,files:Readonly<Record<string,string>>){
 mkdirSync(root,{recursive:true});for(const[name,text]of Object.entries(files)){mkdirSync(join(root,name,".."),{recursive:true});writeFileSync(join(root,name),text);}
 const inputs:Record<string,string>={};for(const[name,path]of artifactFiles(root))inputs[`vendor/${name}`]=await fileDigest(path);return inputs;
}
const checkout=()=>{const root=mkdtempSync(join(tmpdir(),"stage-keys-"));roots.push(root);return root;};
const digest=(request:Parameters<typeof stageKeyDigestV1>[0])=>stageKeyDigestV1(request);
/** 🔑️ A stage key is a function of content digests and tool identities only: the same inputs under two checkout roots give one key, a changed vendor tree, native transpile (jco) digest, wasm-opt binary or probe node version gives another. */
test("identical inputs under two checkout roots give identical stage keys for transpile, optimize and probe",async()=>{
 const files={"a.js":"export const a=1;","sub/b.js":"export const b=2;"},one=await treeInputs(join(checkout(),"deep","checkout-one","vendor"),files),two=await treeInputs(join(checkout(),"other","root","vendor"),files);
 expect(one).toEqual(two);
 expect(digest(transpileStageRequestV1(h("c"),"m","base","n"))).toBe(digest(transpileStageRequestV1(h("c"),"m","base","n")));
 expect(digest(optimizeStageRequestV1(h("core"),"v","-Oz".split(" "),"b"))).toBe(digest(optimizeStageRequestV1(h("core"),"v",["-Oz"],"b")));
 expect(digest(probeStageRequestV1({"m.js":h("x"),...one},"v22"))).toBe(digest(probeStageRequestV1({"m.js":h("x"),...two},"v22")));
 expect(JSON.stringify([transpileStageRequestV1(h("c"),"m","base","n"),optimizeStageRequestV1(h("core"),"v",["-Oz"],"b")])).not.toContain(tmpdir());
});
test("a changed vendor tree, native transpile digest, wasm-opt binary or node version changes the stage key",async()=>{
 const root=checkout(),before=await treeInputs(join(root,"a"),{"a.js":"1"}),after=await treeInputs(join(root,"b"),{"a.js":"2"});
 expect(digest(probeStageRequestV1(before,"v22"))).not.toBe(digest(probeStageRequestV1(after,"v22")));
 expect(digest(probeStageRequestV1(before,"v22"))).not.toBe(digest(probeStageRequestV1(before,"v23")));
 expect(digest(transpileStageRequestV1(h("c"),"m","base","jco-1"))).not.toBe(digest(transpileStageRequestV1(h("c"),"m","base","jco-2")));
 expect(digest(transpileStageRequestV1(h("c"),"m","base","n"))).not.toBe(digest(transpileStageRequestV1(h("c"),"m","base","n",["--map","x"])));
 expect(digest(optimizeStageRequestV1(h("core"),"v",["-Oz"],"b1"))).not.toBe(digest(optimizeStageRequestV1(h("core"),"v",["-Oz"],"b2")));
 expect(transpileStageRequestV1(h("c"),"m","base","n").schemaVersion).toBe(3);expect(optimizeStageRequestV1(h("core"),"v",[],"b").schemaVersion).toBe(2);
});
