/** 📁️ Rooted writes require read access through ancestors and write access at the selected directory. */
import {expect,test} from "bun:test";
import {mkdtemp,mkdir,rm,readFile,writeFile,lstat,realpath} from "node:fs/promises";
import {join,resolve} from "node:path";
import {existsSync,readdirSync,writeFileSync} from "node:fs";
import glob from "fast-glob";
import Ajv from "ajv";
import {withArtifactRelativeDirectoryV1,withArtifactReadFileSyncV1} from "../../📁️descriptor/🟦️.ts";
import fixture from "./🔣️.json";
import schema from "./📋️schema.json";

test("rooted descriptor corpus follows its closed portable schema",()=>{expect(new Ajv({strict:true}).compile(schema)(fixture)).toBe(true);});

test("synchronous file capabilities read exact Unicode bytes on the native desktop",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit descriptor test output required");const root=await mkdtemp(join(resolve(output),"sync-read-"));
 try{for(const row of fixture.entries){const path=join(root,row.name);await writeFile(path,row.text);let released:(()=>unknown)|undefined;const bytes=withArtifactReadFileSyncV1(path,()=>{},file=>{released=()=>file.stat();const size=Number(file.stat().size),bytes=Buffer.alloc(size);expect(file.read(bytes,0,size,0)).toBe(size);return bytes;});expect(bytes.toString()).toBe(await readFile(path,"utf8"));expect(released).toBeDefined();expect(()=>released!()).toThrow("Exact live");}expect(()=>withArtifactReadFileSyncV1(root,()=>{},()=>{})).toThrow();}finally{await rm(root,{recursive:true,force:true});}
});

test("rooted descriptor writes traverse ordinary desktop ancestors and agree with fast-glob",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit descriptor test output required");await mkdir(output,{recursive:true});const root=await mkdtemp(join(resolve(output),"descriptor-"));
 try{await withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{for(const row of fixture.entries)await directory.withNewFile(row.name,async file=>{const bytes=Buffer.from(row.text);expect(file.write(bytes,0,bytes.length,0)).toBe(bytes.length);file.sync();});expect(await directory.names()).toEqual((await glob("*",{cwd:root})).sort((a,b)=>Buffer.from(a).compare(Buffer.from(b))));});for(const row of fixture.entries)expect(await readFile(join(root,row.name),"utf8")).toBe(row.text);}finally{await rm(root,{recursive:true,force:true});}
});

test("rooted preservation retains original bytes before publishing the directory binding",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit descriptor test output required");
 for(const kind of fixture.preservations){const root=await mkdtemp(join(resolve(output),"preserve-")),source=join(root,"source"),target=join(root,"target"),name="node_modules",original=join(source,name);await mkdir(source);await mkdir(target);if(kind==="directory"){await mkdir(original);await writeFile(join(original,"value.txt"),"original");}else await writeFile(original,"original");const before=await lstat(original,{bigint:true});
  try{const result=await withArtifactRelativeDirectoryV1(source,()=>{},async directory=>{const expected=await directory.capturePreservation(name);return directory.publishPreserving(expected,{kind:"directory",path:target},JSON.stringify({version:1,kind}));});expect(result.binding.target).toBe(target);expect(await realpath(original)).toBe(target);const payload=join(source,result.retained.capsule,result.retained.payload),after=await lstat(payload,{bigint:true});expect(after.ino).toBe(before.ino);expect(await readFile(kind==="directory"?join(payload,"value.txt"):payload,"utf8")).toBe("original");expect((await glob("*",{cwd:join(source,result.retained.capsule),onlyFiles:false})).sort()).toEqual(["i","p","r"]);}finally{await rm(root,{recursive:true,force:true});}
 }
});

test("rooted retention keeps the original directory and proves its old name absent",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit descriptor test output required");const root=await mkdtemp(join(resolve(output),"retention-")),name="node_modules";await mkdir(join(root,name));await writeFile(join(root,name,"original.txt"),"retained");
 try{const result=await withArtifactRelativeDirectoryV1(root,()=>{},async directory=>{const expected=await directory.capturePreservation(name);return directory.retainPreservation(expected,"retain-original");});expect(result.absent).toBe(true);expect(await lstat(join(root,name)).catch(()=>null)).toBeNull();expect(await readFile(join(root,result.retained.capsule,"p/original.txt"),"utf8")).toBe("retained");expect((await glob("*",{cwd:root,dot:true,onlyFiles:false})).sort()).toEqual([result.retained.capsule]);}finally{await rm(root,{recursive:true,force:true});}
});

test("rooted preservation refuses forged expectations and preserves cancellation through partial effects",async()=>{
 const output=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!output)throw Error("Explicit descriptor test output required");
 for(const mode of fixture.refusals){const root=await mkdtemp(join(resolve(output),"refusal-")),source=join(root,"source"),target=join(root,"target"),name="node_modules",marker=mode==="undefined"?undefined:Object.freeze(Error("cancelled preservation"));await mkdir(source);await mkdir(target);await writeFile(join(source,name),"original");let armed=false,mutated=false;
  const check=()=>{if(!armed)return;if(mode==="cancelled"||mode==="undefined")throw marker;const capsule=readdirSync(source).find(value=>value.startsWith(".p"));if(capsule&&mode==="after-capsule")throw marker;if(capsule&&mode==="late-payload"&&existsSync(join(source,capsule,"r"))&&!mutated){mutated=true;writeFileSync(join(source,capsule,"p"),"external mutation");}};
  try{let failed=false,failure:unknown;try{await withArtifactRelativeDirectoryV1(source,check,async directory=>{const expected=await directory.capturePreservation(name);armed=true;return directory.publishPreserving(mode==="copied"?{...expected}:expected,{kind:"directory",path:target},mode);});}catch(error){failed=true;failure=error;}expect(failed,mode).toBe(true);if(["cancelled","undefined","after-capsule"].includes(mode))expect(failure,mode).toBe(marker);if(mode!=="late-payload")expect(await readFile(join(source,name),"utf8")).toBe("original");expect(await glob("*",{cwd:target})).toEqual([]);}finally{await rm(root,{recursive:true,force:true});}
 }
});
