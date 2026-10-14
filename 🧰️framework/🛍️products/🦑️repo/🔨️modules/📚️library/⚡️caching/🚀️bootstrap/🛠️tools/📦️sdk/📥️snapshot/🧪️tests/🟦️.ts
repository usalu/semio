import {expect,test} from "bun:test";
import {mkdtemp,mkdir,writeFile,readFile,rm,symlink,readlink,lstat} from "node:fs/promises";
import {join,dirname} from "node:path";
import Ajv from "ajv";
import {build} from "esbuild";
import {parsePatch,applyPatch} from "diff";
import {assertToolingPatchV1} from "../../../🩹️patch/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {cloneNxSdkSnapshotV1,nxSdkSnapshotIdentityV1,nxSdkSnapshotReceiptHoldsV1} from "../🟦️.ts";

test("tooling patch preflight agrees with independent unified diff syntax and application",()=>{
 for(const row of fixture.patches){
  let oracle=false;try{const parsed=parsePatch(row.patch);oracle=parsed.length===new Set(parsed.map(value=>value.newFileName)).size&&parsed.length===1&&applyPatch(row.before,parsed[0]!)===row.after;}catch{}
  expect(oracle,row.id).toBe(row.accept);if(row.accept)expect(()=>assertToolingPatchV1(row.patch),row.id).not.toThrow();else expect(()=>assertToolingPatchV1(row.patch),row.id).toThrow();
 }
 console.log("[DEBUG] SDK patch exactHunks=true repeatedFileRefused=true independentDiff=true");
});

test("SDK snapshot identity and receipt are closed over pinned package bytes",()=>{
 const files=new Map(Object.entries(fixture.identity.files).map(([path,body])=>[path,new TextEncoder().encode(body)])),environment=fixture.identity.environment,identity=nxSdkSnapshotIdentityV1(environment,files),oracle=new Bun.CryptoHasher("sha256").update("nx-sdk-snapshot-v1\0"+environment.platform+"\0"+environment.architecture+"\0"+environment.libc+"\0");
 for(const[path,body]of Object.entries(fixture.identity.files).sort(([a],[b])=>a<b?-1:a>b?1:0))oracle.update(path+"\0").update(body).update("\0");
 expect(identity).toBe(fixture.identity.expected);expect(identity).toBe(oracle.digest("hex"));expect(nxSdkSnapshotIdentityV1(environment,new Map([...files].reverse()))).toBe(identity);
 const receipt={version:1,digest:identity,patches:{}},ajv=new Ajv({strict:true});ajv.addSchema(schema);expect(ajv.validate({$ref:schema.$id+"#/$defs/Receipt"},receipt)).toBe(true);expect(nxSdkSnapshotReceiptHoldsV1(receipt,identity)).toBe(true);
 expect(nxSdkSnapshotReceiptHoldsV1({...receipt,unknown:true},identity)).toBe(false);expect(nxSdkSnapshotReceiptHoldsV1(receipt,"0".repeat(64))).toBe(false);
 files.set(fixture.identity.invalidSource,new Uint8Array());expect(()=>nxSdkSnapshotIdentityV1(environment,files)).toThrow("Closed pinned SDK input");
 console.log("[DEBUG] SDK pinned-byte identity independentHasher=true sourceScopeClosed=true");
});

test("SDK snapshot clone preserves relocatable imports and isolated writable bytes",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original ticket artifact directory required");
 const root=await mkdtemp(join(artifacts,"sk-")),source=join(root,"src"),destination=join(root,"dst"),abort=new AbortController();let progress=0,continuations=0;
 const control={signal:abort.signal,remainingMilliseconds:()=>120000,publish:async()=>{progress++;},yieldContinuation:async()=>{continuations++;}};
 try{
  for(const[path,bytes]of Object.entries(fixture.files)){await mkdir(dirname(join(source,path)),{recursive:true});await writeFile(join(source,path),bytes);}
  await mkdir(join(source,"node_modules/.bin"),{recursive:true});
  for(const row of fixture.aliases)await symlink(row.target,join(source,row.path),"file");
  await mkdir(destination);
  const result=await cloneNxSdkSnapshotV1(source,destination,control),validate=new Ajv({strict:true}).compile(schema);
  expect(validate(result),JSON.stringify(validate.errors)).toBe(true);expect(result.files).toBe(fixture.expected.files);expect(result.links).toBe(fixture.expected.links);
  const output=await build({absWorkingDir:destination,stdin:{contents:'import {value} from "probe";export {value};',resolveDir:destination},bundle:true,platform:"node",format:"esm",write:false,logLevel:"silent"});
  expect(output.outputFiles[0]!.text).toContain("value = "+fixture.expected.value);
  expect(await readlink(join(destination,fixture.aliases[0]!.path))).toBe(fixture.aliases[0]!.target);
  expect((await lstat(join(destination,fixture.mutation.path))).ino).not.toBe((await lstat(join(source,fixture.mutation.path))).ino);
  await writeFile(join(destination,fixture.mutation.path),fixture.mutation.bytes);expect(await readFile(join(source,fixture.mutation.path),"utf8")).toBe(fixture.files[fixture.mutation.path as keyof typeof fixture.files]);
  expect(progress).toBeGreaterThan(0);expect(continuations).toBe(progress);
  abort.abort(Error("Original snapshot cancelled"));await expect(cloneNxSdkSnapshotV1(source,join(root,"cancelled"),control)).rejects.toThrow("Original snapshot cancelled");
  await rm(join(source,fixture.aliases[0]!.path));await symlink(fixture.unsafeTarget,join(source,fixture.aliases[0]!.path),"file");
  await mkdir(join(root,"unsafe"));await expect(cloneNxSdkSnapshotV1(source,join(root,"unsafe"),{...control,signal:new AbortController().signal})).rejects.toThrow("contained");
  console.log("[DEBUG] SDK snapshot esbuild value="+fixture.expected.value+" independentWrites=true originalCancellation=true unsafeAliasRefused=true");
 }finally{await rm(root,{recursive:true,force:true});}
});
