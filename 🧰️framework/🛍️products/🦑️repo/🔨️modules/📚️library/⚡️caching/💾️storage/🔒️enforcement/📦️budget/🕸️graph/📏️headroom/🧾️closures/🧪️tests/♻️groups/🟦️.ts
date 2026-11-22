import{nativeImageFreshSpaceV1}from"../../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import{withRepositoryBuildGraphPublicationFixtureV1}from"../../../../../../../../../🏗️build/📥️inputs/💾️storage/🟨️.mjs";
import{compactSourceInputs,cargoLockInput}from"../../../../../../../../../🏗️build/📥️inputs/🟨️.mjs";
import{openRecordStore,recordKey}from"../../../../../../../../🗃️records/🟦️.ts";
import{registerIsolatedBuildTestV1}from"../../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import{createIssuedResourceControlV1}from"../../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{captureBuildGraphClosureProofV1,retireBuildGraphClosureProofV1,invalidateBuildGraphOptionalGroupV1,retireBuildGraphOptionalGroupsHeadroomV1}from"../../🟦️.ts";
import{fileURLToPath}from"node:url";
import{join,dirname}from"node:path";
import{mkdirSync,writeFileSync,readFileSync,readdirSync,existsSync,rmSync,renameSync,lstatSync,chmodSync}from"node:fs";
import{test,expect}from"bun:test";
import Ajv from"ajv";
import{Database}from"bun:sqlite";
import{classifyBuildGraphRetirementStateV1}from"../../🟦️.ts";
import schema from"../../🧬️schema/🔣️.json";
import cases from"../🧫️cases.json";
import{privateRepositoryWorkspaceV1,withPrivateRepositoryBuildTestInvocationV1}from"../../../../../../../../../🧪️testing/🧫️private-repository/🟦️.ts";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
declareLibraryBuildTestEnvelopeV1({suite:"storage-graph-processes"});
const repository=await privateRepositoryWorkspaceV1();
test("graph retirement intent states match the independent SQLite transition oracle",()=>{
 const validate=new Ajv({strict:true}).compile({$schema:schema.$schema,definitions:schema.definitions,$ref:"#/definitions/RetirementState"}),db=new Database(":memory:");try{for(const row of cases.retirementStates){expect(validate(row.input)).toBe(true);const result=classifyBuildGraphRetirementStateV1(row.input),independent=db.query("select case when not ?1 and ?2='bound' and ?3='absent' then 'prepared' when ?2 in ('absent','fresh') and ?3='bound' then 'committed' when ?1 and ?2 in ('absent','fresh') and ?3='absent' and ?4=0 then 'terminal' else 'refuse' end state").get(Number(row.input.committed),row.input.original,row.input.retired,row.input.remaining)as{state:string};expect<unknown>(result).toBe(row.state);expect<unknown>(result).toBe(independent.state);}}finally{db.close();}
});

function retainedFileV1(path:string){
 const identity=lstatSync(path,{bigint:true});
 return {identity:identity.dev+":"+identity.ino,bytes:readFileSync(path)};
}

function retainedGroupV1(directory:string){
 if(!existsSync(directory))return null;
 const identity=lstatSync(directory,{bigint:true});
 return {identity:identity.dev+":"+identity.ino,members:readdirSync(directory).sort().map(name=>({name,...retainedFileV1(join(directory,name))}))};
}

const groupRefusals:Readonly<Partial<Record<string,RegExp>>>={
 "unindexed-pack":/^Complete closure record index required$/u,
 "manifest-changed":/^Closed graph retirement intent required$/u,
 "member-changed":/^Retired graph member changed$/u,
 "ancestor-changed":/^Bound retired graph namespace changed$/u,
 "new-member":/^Retired graph member changed$/u,
 "cancel-after-commit":/^controlled group cancellation$/u,
};
for(const scenario of cases.groups)registerIsolatedBuildTestV1(fileURLToPath(import.meta.url),"graph optional group retirement "+scenario,{bodyMilliseconds:120000,scope:{kind:"body"}},checkpoint=>withPrivateRepositoryBuildTestInvocationV1(async original=>{
 const stop=new AbortController(),control=createIssuedResourceControlV1(original,"test:graph-group",110000,stop.signal),negative=groupRefusals[scenario]!==undefined;let failed=false,failure:unknown,productionFailure:unknown,refused=false,retained=false,commitReady=false,mutationReached=false;
 try{await withRepositoryBuildGraphPublicationFixtureV1(repository.root,original,control,async fixture=>{
  const directory=join(fixture.root,"inference","mr"),key=recordKey("group-retirement"),graph={version:1,projects:{},dependencies:[],namedInputs:{},externalNodes:{}};let token="",lock="";const orphanBody=JSON.stringify({version:1,files:["Cargo.lock"]}),orphanHash=new Bun.CryptoHasher("sha256").update(orphanBody).digest("hex"),orphanPath=join(fixture.root,"build-input-closures",orphanHash+".json");const publish=()=>{mkdirSync(directory,{recursive:true});const records=openRecordStore(directory);records.put(key,Buffer.from(JSON.stringify({...graph,namedInputs:{source:[{env:token},{env:lock}]}})));records.flush();records.close();};
  await fixture.withWriter(async()=>{token=(compactSourceInputs(["{workspaceRoot}/Cargo.toml"],fixture.sourceRoot).find((row:any)=>row?.env?.startsWith("SEMIO_SOURCE_CLOSURE_"))as{env:string}).env;lock=cargoLockInput("Cargo.toml",fixture.sourceRoot).env;publish();});
  const pack=readdirSync(directory).find(name=>name.endsWith(".pack"))!,index=pack.slice(0,-5)+".idx",source=join(fixture.root,"build-input-closures",token.slice("SEMIO_SOURCE_CLOSURE_".length)+".json"),catalog=join(fixture.root,"retired");
  if(scenario==="known-missing-pack"){await fixture.withWriter(async()=>{rmSync(join(directory,pack));const records=openRecordStore(directory);try{expect(records.get(key)).toBeUndefined();}finally{records.close();}});}if(scenario==="unindexed-pack"){await fixture.withWriter(async()=>{rmSync(join(directory,index));});expect(existsSync(join(directory,pack))).toBe(true);expect(existsSync(join(directory,index))).toBe(false);const before={source:retainedFileV1(source),group:retainedGroupV1(directory)};mutationReached=true;try{await fixture.withMaintenance((image,custody)=>captureBuildGraphClosureProofV1(image,custody));}catch(error){if(!(error instanceof Error&&error.message==="Complete closure record index required"))throw error;refused=true;productionFailure=error;}if(!refused)throw Error("Required unindexed pack refusal was not observed");expect({source:retainedFileV1(source),group:retainedGroupV1(directory)}).toEqual(before);retained=!existsSync(catalog);expect(retained).toBe(true);throw productionFailure;}
  const action=async()=>{await fixture.withMaintenance(async(image,custody)=>{const proof=await captureBuildGraphClosureProofV1(image,custody);await invalidateBuildGraphOptionalGroupV1(image,custody,proof);expect(existsSync(directory)).toBe(false);expect(existsSync(source)).toBe(true);});if(scenario==="new-generation")await fixture.withWriter(async()=>publish());if(scenario==="known-missing-pack")await fixture.withWriter(async()=>{writeFileSync(orphanPath,orphanBody);});return fixture.withMaintenance(async(image,custody)=>{const manifests=readdirSync(catalog).filter(name=>name.endsWith(".json"));expect(manifests.length).toBe(1);const intent=join(catalog,manifests[0]!),value=JSON.parse(readFileSync(intent,"utf8")),retired=join(catalog,manifests[0]!.slice(0,-5)+".d");expect(value.committed).toBe(true);if(scenario==="known-missing-pack"){expect(value.plan.retention).toBe("all");expect(value.plan.references).not.toContain("source:"+orphanHash);const db=new Database(":memory:");try{expect((db.query("select count(*) n from json_each(?1) where value=?2").get(JSON.stringify(value.plan.references),"source:"+orphanHash)as{n:number}).n).toBe(0);}finally{db.close();}}const referenceProof=await captureBuildGraphClosureProofV1(image,custody);const referenceSpace=await nativeImageFreshSpaceV1(image.root,image.filesystemBytes,custody.control.signal),closureRetained=await retireBuildGraphClosureProofV1(image,custody,referenceProof,{minimumAvailableBytes:referenceSpace.bavail*referenceSpace.bsize+1});expect(closureRetained.retiredFiles).toBe(0);expect(existsSync(source)).toBe(true);commitReady=true;
   const free=await nativeImageFreshSpaceV1(image.root,image.filesystemBytes,custody.control.signal),minimumAvailableBytes=free.bavail*free.bsize+1048576,invoke=()=>retireBuildGraphOptionalGroupsHeadroomV1(image,custody,{minimumAvailableBytes});
   if(scenario==="manifest-changed")writeFileSync(intent,JSON.stringify({...value,extra:true}));if(scenario==="member-changed")writeFileSync(join(retired,pack),"changed");if(scenario==="ancestor-changed"){renameSync(retired,retired+".old");mkdirSync(retired);}if(scenario==="new-member")writeFileSync(join(retired,"unknown"),"foreign");if(scenario==="cancel-after-commit")stop.abort(Error("controlled group cancellation"));mutationReached=true;
   if(negative){
    const state=()=>({source:retainedFileV1(source),intent:retainedFileV1(intent),groups:[retainedGroupV1(retired),retainedGroupV1(retired+".old")]});
    const before=state();try{await invoke();}catch(error){refused=true;productionFailure=error;}if(!refused)throw Error("Required final group production refusal was not observed");
    expect(state()).toEqual(before);retained=true;throw productionFailure;
   }const result=await invoke();expect(negative).toBe(false);expect(result.retiredFiles).toBeGreaterThan(0);expect(existsSync(intent)).toBe(false);expect(existsSync(retired)).toBe(false);expect(existsSync(source)).toBe(true);if(scenario==="new-generation"){const records=openRecordStore(directory);try{expect(records.get(key)).toBeDefined();}finally{records.close();}}else expect(existsSync(directory)).toBe(false);
  });};
  await action();if(scenario==="known-missing-pack"){expect(existsSync(orphanPath)).toBe(true);await fixture.withMaintenance(async(image,custody)=>{const before=await nativeImageFreshSpaceV1(image.root,image.filesystemBytes,custody.control.signal),candidates=[source,orphanPath,join(fixture.root,"build-lock-closures",lock.slice("SEMIO_CARGO_LOCK_".length)+".json")],bytes=candidates.reduce((sum,path)=>sum+lstatSync(path).blocks*512,0),proof=await captureBuildGraphClosureProofV1(image,custody),result=await retireBuildGraphClosureProofV1(image,custody,proof,{minimumAvailableBytes:before.bavail*before.bsize+bytes});expect(result.retiredFiles).toBe(3);expect(existsSync(orphanPath)).toBe(false);});await fixture.withWriter(async()=>{const records=openRecordStore(directory);try{expect(records.get(key)).toBeUndefined();}finally{records.close();}});}
 });}catch(error){failed=true;failure=error;}if(negative){expect(failed).toBe(true);expect(refused).toBe(true);expect(mutationReached).toBe(true);if(scenario!=="unindexed-pack")expect(commitReady).toBe(true);expect(Object.is(failure,productionFailure)).toBe(true);expect(retained).toBe(true);checkpoint();throw failure;}if(failed)throw failure;
}),groupRefusals[scenario]);

/** 🪜️ A committed group retirement walks every directory from the image root up to the filesystem root, so it settles beneath an image nested deep under unrelated ancestors and still refuses once an ancestor above the image root changed after the proof. */
registerIsolatedBuildTestV1(fileURLToPath(import.meta.url),"graph optional group retirement ancestors above the image root",{bodyMilliseconds:120000,scope:{kind:"body"}},()=>withPrivateRepositoryBuildTestInvocationV1(async original=>{
 const stop=new AbortController(),control=createIssuedResourceControlV1(original,"test:graph-group-ancestry",110000,stop.signal);
 await withRepositoryBuildGraphPublicationFixtureV1(repository.root,original,control,async fixture=>{
  const directory=join(fixture.root,"inference","mr"),key=recordKey("group-ancestry"),graph={version:1,projects:{},dependencies:[],namedInputs:{},externalNodes:{}};let token="",lock="";
  await fixture.withWriter(async()=>{token=(compactSourceInputs(["{workspaceRoot}/Cargo.toml"],fixture.sourceRoot).find((row:any)=>row?.env?.startsWith("SEMIO_SOURCE_CLOSURE_"))as{env:string}).env;lock=cargoLockInput("Cargo.toml",fixture.sourceRoot).env;mkdirSync(directory,{recursive:true});const records=openRecordStore(directory);records.put(key,Buffer.from(JSON.stringify({...graph,namedInputs:{source:[{env:token},{env:lock}]}})));records.flush();records.close();});
  const above=dirname(fixture.root),depth=fixture.root.split("/").filter(Boolean).length;expect(depth).toBeGreaterThan(2);
  await fixture.withMaintenance(async(image,custody)=>{const proof=await captureBuildGraphClosureProofV1(image,custody),mode=lstatSync(above).mode&0o7777;chmodSync(above,mode===0o700?0o750:0o700);try{let refused:unknown;try{await invalidateBuildGraphOptionalGroupV1(image,custody,proof);}catch(error){refused=error;}expect(refused).toBeInstanceOf(Error);expect((refused as Error).message).toMatch(/ancestor changed/iu);expect(existsSync(directory)).toBe(true);}finally{chmodSync(above,mode);}});
  await fixture.withMaintenance(async(image,custody)=>{const proof=await captureBuildGraphClosureProofV1(image,custody);await invalidateBuildGraphOptionalGroupV1(image,custody,proof);expect(existsSync(directory)).toBe(false);});
  await fixture.withMaintenance(async(image,custody)=>{const free=await nativeImageFreshSpaceV1(image.root,image.filesystemBytes,custody.control.signal),result=await retireBuildGraphOptionalGroupsHeadroomV1(image,custody,{minimumAvailableBytes:free.bavail*free.bsize+1048576});expect(result.retiredFiles).toBeGreaterThan(0);expect(existsSync(join(fixture.root,"retired")) ? readdirSync(join(fixture.root,"retired")).filter(name=>name.endsWith(".json")).length : 0).toBe(0);});
 });
}));

/** 🧳️ The owner peaks spill of a graph writer travels with its group into the retirement catalog: the group still settles completely, the spill leaves with it, and a member forged beside it is still refused. */
for(const scenario of ["settles","foreign-member"])registerIsolatedBuildTestV1(fileURLToPath(import.meta.url),"graph optional group retirement with travelled spill "+scenario,{bodyMilliseconds:120000,scope:{kind:"body"}},()=>withPrivateRepositoryBuildTestInvocationV1(async original=>{
 const stop=new AbortController(),control=createIssuedResourceControlV1(original,"test:graph-group-spill",110000,stop.signal);
 await withRepositoryBuildGraphPublicationFixtureV1(repository.root,original,control,async fixture=>{
  const directory=join(fixture.root,"inference","np"),key=recordKey("group-spill"),graph={version:1,projects:{},dependencies:[],namedInputs:{},externalNodes:{}},spill=join(directory,"owner-peaks.json");let token="",lock="";
  await fixture.withWriter(async()=>{token=(compactSourceInputs(["{workspaceRoot}/Cargo.toml"],fixture.sourceRoot).find((row:any)=>row?.env?.startsWith("SEMIO_SOURCE_CLOSURE_"))as{env:string}).env;lock=cargoLockInput("Cargo.toml",fixture.sourceRoot).env;mkdirSync(directory,{recursive:true});const records=openRecordStore(directory);records.put(key,Buffer.from(JSON.stringify({...graph,namedInputs:{source:[{env:token},{env:lock}]}})));records.flush();records.close();writeFileSync(spill,'{"language:cargo":[1,2,3]}');});
  await fixture.withMaintenance(async(image,custody)=>{const proof=await captureBuildGraphClosureProofV1(image,custody);await invalidateBuildGraphOptionalGroupV1(image,custody,proof);expect(existsSync(directory)).toBe(false);});
  const catalog=join(fixture.root,"retired");expect(readdirSync(catalog).filter(name=>name.endsWith(".d")).map(name=>existsSync(join(catalog,name,"owner-peaks.json")))).toEqual([true]);
  if(scenario==="foreign-member")writeFileSync(join(catalog,readdirSync(catalog).find(name=>name.endsWith(".d"))!,"unknown"),"foreign");
  await fixture.withMaintenance(async(image,custody)=>{const free=await nativeImageFreshSpaceV1(image.root,image.filesystemBytes,custody.control.signal),invoke=()=>retireBuildGraphOptionalGroupsHeadroomV1(image,custody,{minimumAvailableBytes:free.bavail*free.bsize+1048576});if(scenario==="foreign-member"){await expect(invoke()).rejects.toThrow(/^Retired graph member changed$/u);return;}const result=await invoke();expect(result.retiredFiles).toBeGreaterThan(0);expect(readdirSync(catalog)).toEqual([]);});
 });
}));
