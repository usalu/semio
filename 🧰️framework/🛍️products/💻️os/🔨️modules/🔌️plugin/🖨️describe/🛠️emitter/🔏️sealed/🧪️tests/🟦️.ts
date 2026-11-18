import {test,expect} from "bun:test";
import {createHash} from "node:crypto";
import {mkdtempSync,writeFileSync,rmSync,readFileSync} from "node:fs";
import {join} from "node:path";
import {tmpdir} from "node:os";
import {assertSealedFilesV1} from "../🟦️.ts";

const sha=(bytes:string)=>createHash("sha256").update(bytes).digest("hex");
const digest=async(path:string)=>{try{return sha(readFileSync(path,"utf8"));}catch{return undefined;}};

/** 🔏️ A sealed descriptor compiler stays valid while unrelated live files move on and refuses only its own drift, naming path and both digests. Target `🛠️emitter/🔏️sealed/🧪️tests/🟦️.ts`. */
test("sealed verification ignores live files, accepts the intact set and names path with both digests on drift",async()=>{
 const root=mkdtempSync(join(tmpdir(),"sealed-")),binary=join(root,"semio-plugin-describe"),index=join(root,"index.json"),live=join(root,"live-source.rs");
 try{
  writeFileSync(binary,"binary");writeFileSync(index,"index");writeFileSync(live,"v1");
  const sealed=[{path:binary,sha256:sha("binary")},{path:index,sha256:sha("index")}];
  await assertSealedFilesV1(sealed,digest);
  writeFileSync(live,"v2");await assertSealedFilesV1(sealed,digest);
  writeFileSync(binary,"tampered");
  const failure=await assertSealedFilesV1(sealed,digest).then(()=>undefined,error=>error as Error);
  expect(failure?.message).toContain(binary);expect(failure?.message).toContain(sha("binary"));expect(failure?.message).toContain(sha("tampered"));expect(failure?.message).not.toContain(index);
  rmSync(index);
  const absent=await assertSealedFilesV1(sealed,digest).then(()=>undefined,error=>error as Error);
  expect(absent?.message).toContain(index+" expected "+sha("index")+" actual absent");expect(absent?.message).toContain(binary);
 }finally{rmSync(root,{recursive:true,force:true});}
});
