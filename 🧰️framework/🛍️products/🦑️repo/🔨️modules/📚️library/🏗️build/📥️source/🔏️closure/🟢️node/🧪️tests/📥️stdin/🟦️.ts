import{validateJsonSchemaSubset}from"../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import entrySchema from"../../../../../../../../../../🔨️modules/🏃️process/🎛️owned-execution/📥️capture/🚪️entry/🧬️schema/🔣️.json";
import{nativeDirectoryLayoutV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/📁️directory/🟦️.ts";
import{captureNxAuthorizerSourcesV1,nxAuthorizerCapturedBytesV1,nxAuthorizerCapturedImportsV1}from"../../../../../../⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts";
import{nodeNativeLinkerEnvironmentV1}from"../../../../../../../../../../🔨️modules/⚙️runtime/🟢️node/🧲️native/🏗️addon/🟦️.ts";
import{nativeSystemFromProviderV1,nodeNativeChildEnvironmentV1}from"../../../../../../../../../../🔨️modules/⚙️runtime/🟢️node/🧲️native/🟦️.ts";
import{toolForCommandV1,resolveRegisteredToolV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import{declareLibraryBuildTestEnvelopeV1}from"../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import{withBuildTestInvocationV1}from"../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/📥️invocation/🟦️.ts";
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{resolve,join}from"node:path";
import{mkdir,writeFile,lstat}from"node:fs/promises";
import{createHash}from"node:crypto";
import{buildSourceClosureNodeArgumentsV1,buildSourceClosureNodeStdinLoaderV1}from"../../🟨️.mjs";
import policy from"../../../🧬️schema/🔣️.json";
import fixture from"../🧫️stdin.json";
import schema from"../🧬️schema/📥️stdin.json";
import{createIssuedResourceControlV1,scriptResourceProcessEnvironmentV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{runOwnedCommand,captureOwnedCommandResultV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"build-source-closure",maximumElapsedMilliseconds:180000,scratch:"checkout"});
const hash=(value:string)=>createHash("sha256").update(value).digest("hex");
/** 📥️ The actual pinned Node executes the same immutable stdin kernel and its closed helper proof. */
test("Node stdin helper preserves admitted bytes, proof and closed arguments",async()=>withBuildTestInvocationV1(async original=>{
 expect(new Ajv({strict:true}).validate(schema,fixture)).toBe(true);
 const root=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??process.env.SEMIO_TICKET_DIR??".","node-stdin"),control=createIssuedResourceControlV1(original,"test:node-stdin",60000),loader=buildSourceClosureNodeStdinLoaderV1(policy.definitions.Policy.const),db=new Database(":memory:");await mkdir(root,{recursive:true});
 try{for(const row of fixture.cases){const source='import{answer}from"./answer.ts";export async function runCapturedBuildSourceV1(args,proof){proof.recheck();console.log(JSON.stringify({identity:import.meta.semioCapturedImplementationIdentity===proof.sha256,loader:import.meta.semioCapturedLoaderIdentity===proof.loaderSha256,snapshot:proof.snapshot,args}));}',answer="export const answer=42;",files=[{path:"entry.ts",sha256:hash(source),base64:Buffer.from(source).toString("base64"),imports:{"./answer.ts":"answer.ts"} as Record<string,string>},{path:"answer.ts",sha256:hash(answer),base64:Buffer.from(answer).toString("base64"),imports:{}}];if(row.mutation==="module")files[1]!.sha256="0".repeat(64);if(row.mutation==="import")delete files[0]!.imports["./answer.ts"];const snapshot={version:1,root,entry:"entry.ts",files},input=JSON.stringify(snapshot),digest=row.mutation==="digest"?"0".repeat(64):hash(input);await writeFile(join(root,"answer.ts"),"throw Error('mutable Source executed')");const result=await captureOwnedCommandResultV1("node",["--eval",loader,"--",digest,...fixture.args],root,{control,env:scriptResourceProcessEnvironmentV1(control,process.env),input:{version:1,text:input},output:{maximumBytes:65536,maximumLines:256}});if(!row.accepted){expect(result.status,row.id).not.toBe(0);continue;}expect(result.status,result.stderr).toBe(0);expect(result.signal).toBeNull();const actual=JSON.parse(result.stdout);expect(actual).toEqual(fixture.expected);const oracle=db.query("select json_extract(?1,'$.identity')=1 and json_extract(?1,'$.loader')=1 and json_type(?1,'$.snapshot')='null' and json_extract(?1,'$.args')=json_extract(?2,'$.args') as same").get(JSON.stringify(actual),JSON.stringify(fixture.expected))as{same:number};expect(oracle.same).toBe(1);}}finally{db.close();}
}),60000);

/** 🐧️ The real declared directory helper runs through the same Node loader and hands back the original filesystem identities. */
test("actual Node captured directory helper allocates activates observes and deactivates",async()=>withBuildTestInvocationV1(async original=>{
 const workspace=process.cwd(),area=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!,"node-native-helper"),directory=join(area,"partition"),control=createIssuedResourceControlV1(original,"test:node-native-helper",120000);await mkdir(area,{recursive:true});const layout=nativeDirectoryLayoutV1(directory);await mkdir(layout.root,{recursive:true});const before=await lstat(layout.root);
 const compiler=toolForCommandV1("cc"),registered=compiler?resolveRegisteredToolV1(compiler,workspace,{env:process.env}):undefined,environment=registered?nodeNativeLinkerEnvironmentV1({...process.env,SEMIO_RUNTIME_CACHE:join(area,"runtime")},{...registered,suffix:registered.suffix??[]}):process.env;
 nativeSystemFromProviderV1(environment);const nativeEnvironment=nodeNativeChildEnvironmentV1(),entry=fixture.helper.entry,captured=await captureNxAuthorizerSourcesV1(workspace,entry,control.signal,{installed:true}),bytes=nxAuthorizerCapturedBytesV1(captured),imports=nxAuthorizerCapturedImportsV1(workspace,captured),snapshot={version:1 as const,root:workspace,entry,files:captured.manifest.sources.map(row=>({...row,base64:Buffer.from(bytes.get(row.path)!).toString("base64"),imports:imports.get(row.path)!}))},input=JSON.stringify(snapshot),loader=buildSourceClosureNodeStdinLoaderV1(policy.definitions.Policy.const),seen:string[]=[];
 for(const action of fixture.helper.actions){const args=action==="allocate"?[action,directory,String(fixture.helper.filesystemBytes),String(fixture.helper.maximumPhysicalBytes)]:[action,directory],output:Buffer[]=[];expect(Buffer.byteLength(input)).toBeLessThanOrEqual(4194304);await runOwnedCommand("node",["--eval",loader,"--",hash(input),...args],area,"test:captured-node-directory",120000,{control,signal:control.signal,env:scriptResourceProcessEnvironmentV1(control,{...process.env,...nativeEnvironment}),input,stdout:"ignore",outputStyle:"static",output:{maximumBytes:65536,maximumLines:256},outputConsumer:(bytes,stream)=>{if(stream==="stdout")output.push(Buffer.from(bytes));}});seen.push(Buffer.concat(output).toString("utf8"));}
 expect(seen).toEqual(fixture.helper.outputs);const after=await lstat(layout.root);expect([after.dev,after.ino]).toEqual([before.dev,before.ino]);
}),120000);

/** 🔁️ The unchanged interactive kernel retains readiness and immutable execution beside the Native stdin variant. */
test("existing interactive Node kernel keeps its readiness contract",async()=>withBuildTestInvocationV1(async original=>{
 const root=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR!,"node-interactive"),control=createIssuedResourceControlV1(original,"test:node-interactive",60000),source='export async function runCapturedBuildSourceV1(){console.log(JSON.stringify({answer:42}));}';await mkdir(root,{recursive:true});const snapshot={version:1 as const,root,entry:"entry.ts",files:[{path:"entry.ts",sha256:hash(source),base64:Buffer.from(source).toString("base64"),imports:{}}]},prepared=buildSourceClosureNodeArgumentsV1(snapshot,policy.definitions.Policy.const,128,"node"),result=await captureOwnedCommandResultV1("node",prepared.argv,root,{control,input:{version:1,text:prepared.bootstrap},env:scriptResourceProcessEnvironmentV1(control,process.env),output:{maximumBytes:65536,maximumLines:256}});expect(result.status,result.stderr).toBe(0);expect(result.stdout.trim().split("\n").map(row=>JSON.parse(row))).toEqual([{version:1,source:prepared.digest},fixture.interactive]);
}),60000);

/** 🔒️ The owned validator and Ajv agree on the exact one-implementation Native child handoff. */
test("Native child environment remains closed and rejects opposite implementations",()=>{
 const contract=entrySchema.definitions.Command.properties.environment,validate=new Ajv({strict:false}).compile(contract);
 expect(fixture.environments.map(row=>({id:row.id,accepted:validateJsonSchemaSubset(contract,row.value).length===0}))).toEqual(fixture.environments.map(({id,accepted})=>({id,accepted})));
 expect(fixture.environments.map(row=>({id:row.id,accepted:validate(row.value)}))).toEqual(fixture.environments.map(({id,accepted})=>({id,accepted})));
});
