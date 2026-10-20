import{withBuildTestInvocationV1}from"@semio-tech/process/testing/invocation";
import {createHash} from "node:crypto";
import {expect,test} from "bun:test";
import Ajv from "ajv";
import {Database} from "bun:sqlite";
import {mkdtempSync,mkdirSync,readdirSync,symlinkSync,readFileSync,writeFileSync,rmSync} from "node:fs";
import {join,dirname} from "node:path";
import {createRequire} from "node:module";
import {scriptInvocationEnvironment,createIssuedResourceControlV1,requireIssuedResourceControlRefinementV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {captureOwnedCommandV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🎛️execution/🟦️.ts";
import {scriptCompilerAdmissionControlV1} from "../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🏗️build/🎟️capacity/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};
import fixtureSchema from "./🧬️.json" with {type:"json"};
import schema from "../🧬️schema/🔣️.json" with {type:"json"};
import {validateBoundedNxPathsV1,withBoundedNxArtifactWriteV1} from "../🟦️.ts";
import {repositoryStrictStorageV1} from "../../../💾️storage/🔒️enforcement/🟦️.ts";
import {boundedNxNamespaceV1,readAuthoredNxCacheDirectoryV1} from "../../../💾️storage/🔒️enforcement/🔌️nx/🟦️.ts";
import cacheConfigurationSchema from "../../../💾️storage/🔒️enforcement/🔌️nx/🧬️schema/📋️configuration.json" with {type:"json"};
import storagePolicy from "../../../💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import repositoryPolicy from "../../../🔣️policy.json" with {type:"json"};
import repositorySchema from "../../../🧬️schema/🔣️.json" with {type:"json"};
import {ordinaryNativeStoragePortV1,type NativeStoragePortV1} from "@semio-tech/process/artifacts/native-build/storage";
const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;
if(!artifacts)throw Error("Original artifact directory required");
const original=()=>({signal:new AbortController().signal,remainingMilliseconds:()=>20000,publish:async()=>{},yieldContinuation:async()=>{}});
function owned(){const parent=mkdtempSync(join(artifacts!,"nx-bound-")),root=join(parent,"store");mkdirSync(root);mkdirSync(join(parent,"foreign"));symlinkSync(join(parent,"foreign"),join(root,"link"),process.platform==="win32"?"junction":"dir");return{parent,root};}

test("authored storage resolves one fixed16GiB backing with independent envelope accounting",()=>{
 expect(new Ajv({strict:true}).compile(repositorySchema)(repositoryPolicy)).toBe(true);const actual=repositoryStrictStorageV1(process.cwd()),db=new Database(":memory:");
 try{expect(actual.directory).toBe(join(process.cwd(),".🧬semio/🦑️repo/⚡️cache/rust/fixed"));expect(actual.leaseDirectory).toBe(join(process.cwd(),".cache/b/l/m/data"));expect((db.query("SELECT ?1 - ?2 AS reserve,?3 <= ?2 AND ?4 <= ?2 AS kindsFit").get(actual.capacityBytes,actual.filesystemBytes,actual.maximumWorkspaceBytes,actual.maximumArtifactBytes) as {reserve:number;kindsFit:number})).toEqual({reserve:1048576,kindsFit:1});expect(actual.capacityBytes).toBe(17179869184);}finally{db.close();}
});

test("Nx cache database loader and temporary paths share one declared backing namespace",()=>{
 const {root,parent}=owned();try{const namespace=boundedNxNamespaceV1(root,root,"a".repeat(64),parent,null,"d".repeat(64));expect(Object.isFrozen(namespace.environment)).toBe(true);expect(namespace.environment.NX_CACHE_DIRECTORY).toBe(join(root,"nx/artifacts"));expect(namespace.environment.NX_WORKSPACE_DATA_DIRECTORY).toBe(join(root,"nx/graphs", "a".repeat(20),createHash("sha256").update(JSON.stringify([parent.replaceAll("\\","/"),null,"d".repeat(64)])).digest("hex").slice(0,20)));expect(namespace.environment.NX_PROJECT_GRAPH_CACHE_DIRECTORY).toBe(namespace.environment.NX_WORKSPACE_DATA_DIRECTORY);expect(namespace.environment.TMPDIR).toBe(namespace.environment.TMP);expect(namespace.environment.TMPDIR).toBe(namespace.environment.TEMP);for(const path of Object.entries(namespace.environment).filter(([name,path])=>path!==undefined&&!name.startsWith("SEMIO_")).map(([,path])=>path))expect(validateBoundedNxPathsV1(root,[path!])).toEqual([path!]);expect(()=>boundedNxNamespaceV1(root,root,"forged",parent,null,"d".repeat(64))).toThrow("identity");}finally{rmSync(parent,{recursive:true});}
});

test("neutral Nx output confinement agrees with independent SQLite path segments",()=>{
 expect(new Ajv({strict:true}).compile(fixtureSchema)(fixture)).toBe(true);
 const {root,parent}=owned(),db=new Database(":memory:");
 try{for(const row of fixture.cases){const request={version:1,kind:"artifact-publication",owner:"nx:unit",maximumBytes:8,paths:row.paths.map(path=>join(root,path))};
  expect(new Ajv({strict:true}).compile(schema)(request)).toBe(true);
  const oracle=row.paths.every(path=>Boolean((db.query("SELECT CASE WHEN instr('/' || ?1 || '/', '/../') > 0 OR ?1 LIKE 'link/%' THEN 0 ELSE 1 END AS accepted").get(path) as {accepted:number}).accepted));
  expect(oracle).toBe(row.accepted);if(row.accepted)expect(validateBoundedNxPathsV1(root,request.paths)).toEqual(request.paths);else expect(()=>validateBoundedNxPathsV1(root,request.paths)).toThrow();
 }}finally{db.close();rmSync(parent,{recursive:true});}
});

test("ordinary directories and escaping outputs refuse before producer or journal writes",async()=>{
 const {root,parent}=owned();let writes=0,observations=0;const policy={version:1 as const,root,leaseDirectory:join(process.cwd(),".cache/b/l/m/data"),capacityBytes:100,maximumWorkspaceBytes:100,maximumArtifactBytes:100},control=original();
 try{await expect(withBoundedNxArtifactWriteV1({policy,control,port:ordinaryNativeStoragePortV1(),request:{version:1,kind:"artifact-publication",owner:"nx:unit",maximumBytes:8,paths:[join(root,"cache")]}},async()=>{writes++;})).rejects.toThrow("enforcing storage backend");
 const port={...ordinaryNativeStoragePortV1(),observe:async()=>{observations++;return null;}};
 await expect(withBoundedNxArtifactWriteV1({policy,control,port,request:{version:1,kind:"artifact-restore",owner:"nx:unit",maximumBytes:8,paths:[join(parent,"foreign")]}},async()=>{writes++;})).rejects.toThrow("boundary");
 expect(writes).toBe(0);expect(observations).toBe(0);expect(readdirSync(root)).toEqual(["link"]);}finally{rmSync(parent,{recursive:true});}
});

test("publication and failed restore remain conservatively charged in an explicit storage Source model",()=>withBuildTestInvocationV1(async issued=>{
 const {root,parent}=owned(),leaseDirectory=mkdtempSync(join(artifacts!,"q")),control=createIssuedResourceControlV1(issued,"test:bounded-storage-model",0),policy={version:1 as const,root,leaseDirectory,capacityBytes:16,maximumWorkspaceBytes:16,maximumArtifactBytes:16};let retired=0;
 const port:NativeStoragePortV1={withJournal:async(actual,run)=>{requireIssuedResourceControlRefinementV1(control,actual);return run(actual);},withOperation:async(actual,run)=>{requireIssuedResourceControlRefinementV1(control,actual);return run(actual);},observe:async()=>({identity:"law-backing",root,maximumPhysicalBytes:16}),recheck:async()=>{},syncMetadata:async(backing,directory,actual)=>{requireIssuedResourceControlRefinementV1(control,actual);expect(backing.identity).toBe("law-backing");expect(directory).toBe(root);},retire:async()=>{retired++;}};
 try{const first=await withBoundedNxArtifactWriteV1({policy,control,port,request:{version:1,kind:"artifact-publication",owner:"nx:first",maximumBytes:8,paths:[join(root,"cache")]}},async()=>{writeFileSync(join(root,"out"),"artifact");return 7;});expect(first.value).toBe(7);
 await expect(withBoundedNxArtifactWriteV1({policy,control,port,request:{version:1,kind:"artifact-restore",owner:"nx:failed",maximumBytes:8,paths:[join(root,"restore")]}},async()=>{throw Error("copy failed");})).rejects.toThrow("copy failed");
 let attempted=0;await expect(withBoundedNxArtifactWriteV1({policy,control,port,request:{version:1,kind:"artifact-publication",owner:"nx:next",maximumBytes:1,paths:[join(root,"cache")]}},async()=>{attempted++;})).rejects.toThrow("capacity");expect(attempted).toBe(0);expect(retired).toBe(0);
 const events=readFileSync(join(root,"storage-events.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line)),db=new Database(":memory:");db.exec("CREATE TABLE charge(id TEXT PRIMARY KEY,bytes INTEGER NOT NULL)");for(const event of events)if(event.type==="reserved")db.query("INSERT INTO charge VALUES(?,?)").run(event.allocation.id,event.allocation.bytes);expect((db.query("SELECT SUM(bytes) AS total FROM charge").get() as {total:number}).total).toBe(16);db.close();
 }finally{rmSync(parent,{recursive:true});rmSync(leaseDirectory,{recursive:true});}
}));



 test("authored Nx default agrees with finite storage policy and refuses ordinary directory aliases",()=>{
 const validate=new Ajv({strict:true}).compile(cacheConfigurationSchema),workspace=process.cwd();expect(new Ajv({strict:true}).compile(fixtureSchema)(fixture)).toBe(true);
 for(const row of fixture.authoredCacheDirectories){expect(validate(row.directory)).toBe(row.accepted);if(row.accepted)expect(readAuthoredNxCacheDirectoryV1(workspace,row.directory)).toBe(join(workspace,storagePolicy.partitions.rust.directory,"m/nx/artifacts"));else expect(()=>readAuthoredNxCacheDirectoryV1(workspace,row.directory)).toThrow("Authored bounded Nx cache directory");}
 expect(readAuthoredNxCacheDirectoryV1(workspace,JSON.parse(readFileSync(join(workspace,"nx.json"),"utf8")).cacheDirectory)).toBe(join(workspace,storagePolicy.partitions.rust.directory,"m/nx/artifacts"));
 });


 test("actual frozen Nx refuses authored cache initialization without original bounded namespace",async()=>{
 const identity=process.env.SEMIO_NX_TOOLING_IDENTITY;if(!identity)throw Error("Original immutable SDK required");const parent=mkdtempSync(join(artifacts!,"nx-cfg-")),root=join(parent,"workspace"),sdk=createRequire(join(process.cwd(),".cache/b/t/m/nx",identity,"package.json"));mkdirSync(root);writeFileSync(join(root,"nx.json"),JSON.stringify({cacheDirectory:cacheConfigurationSchema.const}));
 const source=`try{require(${JSON.stringify(sdk.resolve("nx/src/utils/cache-directory"))});console.log(JSON.stringify({refused:false}));}catch(error){console.log(JSON.stringify({refused:String(error).includes('Original bounded Nx namespace required')}));}`;
 try{await withBuildTestInvocationV1(async original=>{for(const row of fixture.rawCacheAdmission){const output=await captureOwnedCommandV1("node",["-e",source],root,{control:scriptCompilerAdmissionControlV1(original,"nx:authored-default:node",20000),env:{...scriptInvocationEnvironment(original,process.env),NODE_PATH:join(dirname(dirname(sdk.resolve("nx/package.json")))),NX_WORKSPACE_ROOT:root,NX_WORKSPACE_ROOT_PATH:root,NX_CACHE_DIRECTORY:row.environment==="foreign"?join(parent,"foreign"):undefined,SEMIO_NX_FIXED_STORAGE:undefined,NX_WORKSPACE_DATA_DIRECTORY:join(root,"data")}});expect(JSON.parse(output.trim().split("\n").reverse().find(line=>line.startsWith("{"))!).refused).toBe(row.refused);}});}finally{rmSync(parent,{recursive:true});}
 });
