import"../../../../../🧪️tests/📐️phases/🟦️.ts";
import"../../🎟️capacity/🧪️tests/📤️derivative/🟦️.ts";
import"../../🎟️capacity/🧪️tests/📤️derivative/⚠️failure/🟦️.ts";
import"../../📥️source/🔗️derivative/🧪️tests/🟦️.ts";
import {expect,test} from "bun:test";
import {mkdtemp,mkdir,writeFile,readFile,rm,symlink,readlink,lstat} from "node:fs/promises";
import {join,dirname} from "node:path";
import Ajv from "ajv";
import {build} from "esbuild";
import {parsePatch,applyPatch} from "diff";
import {assertToolingPatchV1,toolPatchReceipt} from "../../../🩹️patch/🟦️.ts";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {cloneNxSdkSnapshotV1,nxSdkSnapshotIdentityV1,nxSdkSnapshotReceiptHoldsV1} from "../🟦️.ts";

test("tooling patch preflight agrees with independent unified diff syntax and application",()=>{
 for(const row of fixture.patches){
  let oracle=false;try{const parsed=parsePatch(row.patch);oracle=parsed.length===new Set(parsed.map(value=>value.newFileName)).size&&parsed.length===1&&applyPatch(row.before,parsed[0]!)===row.after;}catch{}
  expect(oracle,row.id).toBe(row.accept);if(row.accept)expect(()=>assertToolingPatchV1(row.patch),row.id).not.toThrow();else expect(()=>assertToolingPatchV1(row.patch),row.id).toThrow();
 }
});

test("SDK snapshot identity and receipt are closed over pinned package bytes",()=>{
 const files=new Map(Object.entries(fixture.identity.files).map(([path,body])=>[path,new TextEncoder().encode(body)])),environment=fixture.identity.environment,identity=nxSdkSnapshotIdentityV1(environment,files),oracle=new Bun.CryptoHasher("sha256").update("nx-sdk-snapshot-v1\0immutable-binaries-v1\0"+environment.platform+"\0"+environment.architecture+"\0"+environment.libc+"\0");
 for(const[path,body]of Object.entries(fixture.identity.files).sort(([a],[b])=>a<b?-1:a>b?1:0))oracle.update(path+"\0").update(body).update("\0");
 expect(identity).toBe(fixture.identity.expected);expect(identity).toBe(oracle.digest("hex"));expect(nxSdkSnapshotIdentityV1(environment,new Map([...files].reverse()))).toBe(identity);
 const receipt={version:1,digest:identity,patches:{},binaries:{version:1,entry:".semio-nx-sdk-binaries.json",digest:"0".repeat(64)}},ajv=new Ajv({strict:true});ajv.addSchema(schema);expect(ajv.validate({$ref:schema.$id+"#/$defs/Receipt"},receipt)).toBe(true);expect(nxSdkSnapshotReceiptHoldsV1(receipt,identity)).toBe(true);
 expect(nxSdkSnapshotReceiptHoldsV1({...receipt,unknown:true},identity)).toBe(false);expect(nxSdkSnapshotReceiptHoldsV1(receipt,"0".repeat(64))).toBe(false);
 files.set(fixture.identity.invalidSource,new Uint8Array());expect(()=>nxSdkSnapshotIdentityV1(environment,files)).toThrow("Closed pinned SDK input");
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
  const result=await cloneNxSdkSnapshotV1(source,destination,control,{version:1,kind:"copy"}),validate=new Ajv({strict:true}).compile(schema);
  expect(validate(result),JSON.stringify(validate.errors)).toBe(true);expect(result.files).toBe(fixture.expected.files);expect(result.links).toBe(fixture.expected.links);
  const output=await build({absWorkingDir:destination,stdin:{contents:'import {value} from "probe";export {value};',resolveDir:destination},bundle:true,platform:"node",format:"esm",write:false,logLevel:"silent"});
  expect(output.outputFiles[0]!.text).toContain("value = "+fixture.expected.value);
  expect(await readlink(join(destination,fixture.aliases[0]!.path))).toBe(fixture.aliases[0]!.target);
  expect((await lstat(join(destination,fixture.mutation.path))).ino).not.toBe((await lstat(join(source,fixture.mutation.path))).ino);
  await writeFile(join(destination,fixture.mutation.path),fixture.mutation.bytes);expect(await readFile(join(source,fixture.mutation.path),"utf8")).toBe(fixture.files[fixture.mutation.path as keyof typeof fixture.files]);
  expect(progress).toBeGreaterThan(0);expect(continuations).toBe(progress);
  abort.abort(Error("Original snapshot cancelled"));await expect(cloneNxSdkSnapshotV1(source,join(root,"cancelled"),control,{version:1,kind:"copy"})).rejects.toThrow("Original snapshot cancelled");
  await rm(join(source,fixture.aliases[0]!.path));await symlink(fixture.unsafeTarget,join(source,fixture.aliases[0]!.path),"file");
  await mkdir(join(root,"unsafe"));await expect(cloneNxSdkSnapshotV1(source,join(root,"unsafe"),{...control,signal:new AbortController().signal},{version:1,kind:"copy"})).rejects.toThrow("contained");
 }finally{await rm(root,{recursive:true,force:true});}
});

/** 🩹️ Receipt authority requires every requested post-hunk at its exact SDK position and original context. */
test("SDK patch receipt refuses misplaced additions and changed hunk context",async()=>{
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Original patch artifact directory required");const root=await mkdtemp(join(artifacts,"pp-")),oracle=new Ajv({strict:true}).addSchema(schema).compile({$ref:schema.$id+"#/$defs/PatchPositionFixture"});
 try{for(const row of fixture.positions){expect(oracle(row),row.id).toBe(true);expect(applyPatch(row.before,row.patch)===row.after,row.id).toBe(row.accept);await writeFile(join(root,"source.js"),row.after);let accepted=true;try{toolPatchReceipt(root,row.patch)}catch{accepted=false}expect(accepted,row.id).toBe(row.accept)}}finally{await rm(root,{recursive:true,force:true})}
});

/** 📐️ Registry dependency artifacts do not acquire first-party patch revision identity. */
test("pristine dependency inputs ignore patch edits and retain all locked compiler inputs",async()=>{const{nxDependencyInstallFilesV1}=await import("../../📥️source/📐️projection/🟦️.ts"),rows=(await import("../../📥️source/📐️projection/🧪️tests/🔣️.json")).default,{Database}=await import("bun:sqlite"),db=new Database(":memory:"),environment={platform:"darwin",architecture:"arm64",libc:""},identity=(recipe:unknown,lock:unknown)=>nxSdkSnapshotIdentityV1(environment,nxDependencyInstallFilesV1(recipe,lock)),baseline=identity(rows.recipe,rows.lock);try{for(const row of rows.cases){const recipe=structuredClone(rows.recipe),lock=structuredClone(rows.lock);if(row.field==="patch"){recipe.semio.toolPatches["nx@23.2.0"]="patches/changed.patch";lock.patchedDependencies["nx@23.2.0"]="patches/changed.patch";}else if(row.field==="dependency"){recipe.dependencies.nx="23.2.1";lock.workspaces[""].dependencies.nx="23.2.1";}else if(row.field==="integrity")lock.packages.nx[3]="sha512-changed";else if(row.field==="bun")recipe.packageManager="bun@1.3.15";else throw Error("Closed dependency mutation required");const actual=identity(recipe,lock)===baseline,oracle=db.query("select json_remove(?1,'$.semio')=json_remove(?2,'$.semio') and json_remove(?3,'$.patchedDependencies')=json_remove(?4,'$.patchedDependencies') as same").get(JSON.stringify(recipe),JSON.stringify(rows.recipe),JSON.stringify(lock),JSON.stringify(rows.lock)) as {same:number};expect(Boolean(oracle.same),row.name).toBe(row.same);expect(actual,row.name).toBe(row.same);}}finally{db.close();}});

/** 🩹️ Derived SDK patch bytes match the independent exact unified-diff emitter. */
test("first-party SDK patch emitter owns exact original bytes and terminal newlines",async()=>{const {applyToolingPatchImagesV1}=await import("../../../🩹️patch/🟦️.ts"),cases=(await import("../../../🩹️patch/🧪️tests/🔣️.json")).default,contract=(await import("../../../🩹️patch/🧬️schema/🔣️.json")).default,oracle=new Ajv({strict:true}).compile(contract.definitions.Case);for(const row of cases){expect(oracle(row),row.name).toBe(true);const independent=applyPatch(row.before,row.patch,{fuzzFactor:0});expect(independent!==false,row.name).toBe(row.accept);if(row.accept){expect(independent,row.name).toBe(row.after);expect(Buffer.from(applyToolingPatchImagesV1(new Map([["source.js",Buffer.from(row.before)]]),row.patch).get("source.js")!).toString(),row.name).toBe(row.after);}else expect(()=>applyToolingPatchImagesV1(new Map([["source.js",Buffer.from(row.before)]]),row.patch)).toThrow();}});

/** 📋️ Shared binary retention follows closed observations and independent relational classification. */
test("pristine file disposition refuses missing and foreign inode shapes",async()=>{const{nxPristineFileDispositionV1}=await import("../../📥️source/🧊️pristine/📋️plan/🟦️.ts"),rows=(await import("../../📥️source/🧊️pristine/📋️plan/🧪️tests/🔣️.json")).default,contract=(await import("../../📥️source/🧊️pristine/📋️plan/🧬️schema/🔣️.json")).default,validate=new Ajv({strict:true}).compile(contract.definitions.Observation),{Database}=await import("bun:sqlite"),db=new Database(":memory:");try{for(const row of rows){expect(validate(row.observation),row.name).toBe(true);const oracle=db.query("select case when json_extract(?1,'$.path') like 'node_modules/%' then 'nested' when not json_extract(?1,'$.present') then null when json_extract(?1,'$.shared') then case when json_extract(?1,'$.archive') and json_extract(?1,'$.matches') then 'preserve' else null end when json_extract(?1,'$.links') != 1 then null when json_extract(?1,'$.archive') then 'replace' else 'remove' end disposition").get(JSON.stringify(row.observation))as{disposition:string|null};expect(oracle.disposition,row.name).toBe(row.expected);if(row.expected===null)expect(()=>nxPristineFileDispositionV1(row.observation),row.name).toThrow();else expect(String(nxPristineFileDispositionV1(row.observation)),row.name).toBe(row.expected);}}finally{db.close();}});


test("SDK snapshot permanently registers captured-input and custody laws",async()=>{
 const {readFileSync}=await import("node:fs"),ts=await import("typescript"),text=readFileSync(import.meta.path,"utf8"),expected=fixture.capacityRegistration.imports,actual=ts.preProcessFile(text,true,true).importedFiles.map(row=>row.fileName),scan=new Bun.Transpiler({loader:"ts"}).scan(text).imports.map(row=>{const path=row.path;if(!/[\u0080-\u00ff]/u.test(path)||/[\u0100-\uffff]/u.test(path))return path;return Buffer.from(path,"latin1").toString("utf8");});
 for(const path of expected){expect(actual.filter(value=>value===path)).toHaveLength(1);expect(scan.filter(value=>value===path)).toHaveLength(1);const bytes=readFileSync(new URL(path,import.meta.url),"utf8");expect(bytes).toContain("withBuildTestInvocationV1");}expect(readFileSync(new URL(expected[1]!,import.meta.url),"utf8")).toContain("scriptInvocationEnvironment(original");
});
