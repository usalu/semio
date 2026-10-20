import{createIssuedResourceControlV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {testPythonV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import{requireToolingPublicationValueV1}from"../../../../../🧹️retention/📏️headroom/🟦️.ts";
import{withBuildTestInvocationV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import{test,expect}from"bun:test";
import{join,dirname,basename}from"node:path";
import{lstat,mkdtemp,realpath,rm}from"node:fs/promises";
import type{BigIntStats}from"node:fs";
import corpus from"./🔣️.json"with{type:"json"};
import{captureNxSdkPublicationInputsV1,withNxSdkDerivativePublicationV1,requireNxSdkClonePublicationV1,type NxSdkClonePublicationV1}from"../../../📥️physical/🟦️.ts";
import{withNxSdkDerivativeProjectionV1,requireNxSdkDerivativeProjectionV1,copyNxSdkDerivativeV1}from"../../../../📥️source/🔗️derivative/🟦️.ts";
import type{NxSdkDerivativeProjectionV1}from"../../../../📥️source/🔗️derivative/📋️projection/🟦️.ts";
import{receiveScriptProcessInvocation}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{withResourceLeases,resourceLeasePathV1}from"@semio-tech/process/leases";
import{readPhysicalCargoSourceV1}from"../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/📥️source/🎟️admission/📖️read/🟦️.ts";
import{withRepositoryToolingStoreV1,type RepositoryToolingStoreV1}from"../../../../../../../💾️storage/🔒️enforcement/🛠️tools/🟦️.ts";
import{readNativeCallerAuthorityV1}from"../../../../../../../../📦️packages/🟦️typescript/📦️artifacts/🟦️.ts";
import{withManagedToolingProvisionV1}from"../../../../../🧹️retention/📥️physical/🟦️.ts";
import{captureOwnedCommandV1}from"@semio-tech/process/execution";

/** 🔬️ Asks independent SQLite connections whether the exact registry and base inodes remain excluded. */
async function exclusions(workspace:string,store:RepositoryToolingStoreV1,snapshot:string):Promise<{registry:boolean;base:boolean}>{
 const python=testPythonV1()??Bun.which("python");if(!python)throw Error("Independent system SQLite oracle required");
 const probe="import sqlite3,sys,json\nresult={}\nfor key,path,statement in [('registry',sys.argv[1],'BEGIN'),('base',sys.argv[2],'BEGIN EXCLUSIVE')]:\n c=sqlite3.connect('file:'+path+'?mode=rw',uri=True,timeout=0)\n try:\n  c.execute(statement);c.execute('SELECT version,resource FROM semio_resource_lease').fetchall();result[key]=False\n except sqlite3.OperationalError as e:\n  if str(e)!='database is locked':raise\n  result[key]=True\n finally:c.close()\nprint(json.dumps(result))";
 const directory=join(workspace,".cache/b/l/m/data");return JSON.parse(await captureOwnedCommandV1(python,["-c",probe,resourceLeasePathV1(directory,"nx-tooling-provision"),resourceLeasePathV1(directory,"nx-sdk-snapshot:"+snapshot)],workspace,{control:store.control,env:store.environment}));
}

/** 🧫️ Each failure pattern receives one real carried Original and drains before testing its rejection outside the receiver. */
async function failureLaw(kind:"publication"|"issuer"):Promise<void>{
 const events:string[]=[],marker=Error("fixture derivative "+kind+" failure");let accepted:Promise<unknown>|undefined,copySettled=false,failedScope:unknown,projection:NxSdkDerivativeProjectionV1|undefined,quote:NxSdkClonePublicationV1|undefined;
 const received=receiveScriptProcessInvocation(process.env,original=>{
  const workspace=readNativeCallerAuthorityV1(original.capabilities).repositoryRoot;
  return withRepositoryToolingStoreV1(workspace,original,createIssuedResourceControlV1(original,"cache:entry",0),async store=>{
   const directory=join(workspace,".cache/b/l/m/data"),read=async(file:string)=>{const value=await readPhysicalCargoSourceV1(file,{expectedBytes:null,version:1,root:store.root,maximumBytes:16777216,remainingFiles:1},store.control);if(!value)throw Error("Bounded actual SDK input required");return Buffer.from(value);},generation=await realpath(join(workspace,".nx/installation")),identity=basename(generation);if(dirname(generation)!==join(store.root,"nx")||!/^([a-f0-9]{64})$/u.test(identity))throw Error("Actual canonical SDK generation required");
   const snapshot=await withResourceLeases({directory,resources:[{resource:"nx-tooling-provision",mode:"shared"},{resource:"nx-tooling:"+identity,mode:"shared"}],control:store.control,signal:store.control.signal},async()=>{const material=JSON.parse((await read(join(generation,".semio-sdk-material.json"))).toString());if(!/^([a-f0-9]{64})$/u.test(material.snapshot))throw Error("Exact actual SDK base selection required");return material.snapshot as string;}),source=join(store.root,"sdk",snapshot);
   const inputs=await withResourceLeases({directory,resources:[{resource:"nx-sdk-snapshot:"+snapshot,mode:"shared"}],control:store.control,signal:store.control.signal},async()=>{const result=new Map<string,Uint8Array>();for(const name of["package.json","bun.lock"])result.set(name,await read(join(source,name)));const manifest=JSON.parse(Buffer.from(result.get("package.json")!).toString());for(const name of Object.values(manifest.patchedDependencies??{})){if(typeof name!=="string"||!/^patches\/[a-f0-9]{64}[.]patch$/u.test(name)||result.size>=32)throw Error("Closed actual SDK patch input required");result.set(name,await read(join(source,name)));}if([...result.values()].reduce((sum,row)=>sum+row.byteLength,0)>16777216)throw Error("Actual SDK input byte ceiling exceeded");return result;});
   let destination:string|undefined,owned:BigIntStats|undefined;
   try{
    await withNxSdkDerivativeProjectionV1(workspace,store,store.control,source,inputs,async current=>{projection=current;
     return withManagedToolingProvisionV1(workspace,store,store.control,{version:1,minimumAvailableBytes:1},(publish,control)=>withNxSdkDerivativePublicationV1(workspace,store,current,captureNxSdkPublicationInputsV1(inputs),store.control,async issued=>{quote=issued;let ready!:(value:void)=>void,decline!:(error:unknown)=>void;const entered=new Promise<void>((accept,reject)=>{ready=accept;decline=reject;});
      const publication=publish(async()=>{destination=await mkdtemp(join(store.root,"nx",".stage-drv-failure-"));owned=await lstat(destination,{bigint:true});accepted=issued.clone(destination,control);events.push("copy-accepted");void accepted.then(()=>{copySettled=true;events.push("copy-settled");},()=>{copySettled=true;events.push("copy-rejected");});expect(await exclusions(workspace,store,snapshot)).toEqual(corpus.held);expect(copySettled).toBe(false);events.push(kind+"-thrown");ready();if(kind==="publication")throw marker;},issued).then(requireToolingPublicationValueV1);void publication.catch(decline);
      if(kind==="publication")await publication;else{await entered;throw marker;}
     },{requireNxSdkDerivativeProjectionV1,copyNxSdkDerivativeV1}).finally(()=>{events.push("quote-released");})).finally(()=>{events.push("registry-released");});
    }).finally(()=>{events.push("base-released");});
   }catch(error){failedScope=error;expect(copySettled).toBe(true);expect(error).toBe(marker);expect(await exclusions(workspace,store,snapshot)).toEqual(corpus.released);await expect(requireNxSdkClonePublicationV1(store,quote)).rejects.toThrow("released");await expect(requireNxSdkDerivativeProjectionV1(workspace,store,projection)).rejects.toThrow("released");expect((await lstat(join(destination!,"node_modules/nx/package.json"))).isFile()).toBe(true);throw error;}
   finally{await accepted?.catch(()=>{});if(destination&&owned)await withResourceLeases({directory,resources:[{resource:"nx-tooling-provision",mode:"exclusive"}],control:store.control,signal:store.control.signal},async()=>{const actual=await lstat(destination!,{bigint:true});if(dirname(destination!)!==join(store.root,"nx")||!basename(destination!).startsWith(".stage-drv-failure-")||!actual.isDirectory()||actual.isSymbolicLink()||actual.dev!==owned!.dev||actual.ino!==owned!.ino)throw Error("Exact owned failure fixture staging identity required");await rm(destination!,{recursive:true});});}
  }).finally(()=>{events.push("store-released");});
 });
 await expect(received).rejects.toBe(marker);expect(failedScope).toBe(marker);expect(events.filter(event=>corpus.terminalOrder.includes(event))).toEqual(corpus.terminalOrder);
}

for(const row of corpus.cases)test(row.pattern,()=>{if(row.kind!=="publication"&&row.kind!=="issuer")throw Error("Closed derivative failure kind required");if(process.env.SEMIO_DERIVATIVE_FAILURE_CHILD===row.kind)return failureLaw(row.kind);return withBuildTestInvocationV1(async original=>{const {captureOwnedCommandResultV1}=await import("@semio-tech/process/execution"),{scriptCompilerAdmissionControlV1}=await import("@semio-tech/process/compiler-capacity"),{scriptInvocationEnvironment}=await import("../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts"),result=await captureOwnedCommandResultV1(process.execPath,["test",import.meta.path,"--test-name-pattern",row.pattern],process.cwd(),{control:scriptCompilerAdmissionControlV1(original,"sdk:derivative-failure:"+row.kind,corpus.maximumElapsedMilliseconds),env:scriptInvocationEnvironment(original,{...process.env,SEMIO_DERIVATIVE_FAILURE_CHILD:row.kind}),output:{maximumBytes:1048576,maximumLines:8192}});process.stdout.write(result.stdout);process.stderr.write(result.stderr);expect(result.status,result.stderr).toBe(0);expect(result.signal).toBe(null);});},corpus.maximumElapsedMilliseconds);
