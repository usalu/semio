import {expect,test} from "bun:test";
import {linkSync,mkdirSync,mkdtempSync,symlinkSync,writeFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {readArtifactBytesV1} from "../../🟦️.ts";

/** 🧾️ Target `📦️distribution/📋️inventory/🧪️tests/🧾️refusal/🟦️.ts`. A refused artifact file names its actual reason: over the limit with size and limit, symbolic link, not a regular file, hard links. */
test("an artifact read names the actual refusal reason",async()=>{
 const root=mkdtempSync(join(tmpdir(),"refusal-"));
 try{
  writeFileSync(join(root,"ok"),"abc");writeFileSync(join(root,"big"),"x".repeat(100));symlinkSync(join(root,"ok"),join(root,"link"));mkdirSync(join(root,"dir"));linkSync(join(root,"ok"),join(root,"twin"));
  expect(Buffer.from(await readArtifactBytesV1(join(root,"big"),100)).length).toBe(100);
  await expect(readArtifactBytesV1(join(root,"big"),99)).rejects.toThrow("has 100 bytes, over the limit of 99");
  await expect(readArtifactBytesV1(join(root,"link"),100)).rejects.toThrow("is a symbolic link");
  await expect(readArtifactBytesV1(join(root,"dir"),100)).rejects.toThrow("is not a regular file");
  await expect(readArtifactBytesV1(join(root,"ok"),100)).rejects.toThrow("has 2 hard links");
 }finally{rmSync(root,{recursive:true,force:true});}
});
