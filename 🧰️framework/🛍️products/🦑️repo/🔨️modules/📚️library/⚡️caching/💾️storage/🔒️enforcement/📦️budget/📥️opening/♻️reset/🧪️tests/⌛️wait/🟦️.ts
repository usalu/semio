import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {randomBytes} from "node:crypto";
import {lstatSync,mkdirSync,mkdtempSync,readFileSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import {withBuildTestInvocationV1,declareBuildTestEnvelopeV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import * as leases from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import * as routing from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {captureNativeImageBackingAncestryV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🟦️.ts";
import {requireNativeImagePublicationSettledV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {readNativeImageAncestryV1,requireNativeImageAncestryBindingV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🟦️.ts";
import {readNativeImageRetirementFileV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts";
import {nativeImageReclaimDueV1,nativeImageReclaimMechanismV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🟦️.ts";
import nativeStorageSchema from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️json/🟦️.ts";
import nativeMountSchema from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🧬️schema/🔣️json/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {repositoryPartitionReclaimRuleV1} from "../../../../🟦️.ts";
import settings from "../../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";

declareBuildTestEnvelopeV1({owner:"test:rust-reclaim-wait",maximumElapsedMilliseconds:120000,capabilities:{artifacts:true}});

function waitRoute(ports:Record<string,unknown>){
 const tree=ts.createSourceFile("reclaim.ts",readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true);
 const names=["captureRepositoryRustReclaimV1","requireRepositoryRustReclaimMappingV1","reclaimRepositoryRustImageWithWaitV1"];
 const code=tree.statements.filter(row=>ts.isFunctionDeclaration(row)&&row.name&&names.includes(row.name.text)).map(row=>row.getText(tree).replace(/^export /u,""));
 expect(code).toHaveLength(names.length);
 const js=ts.transpileModule(code.join("\n"),{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 return new Function(...Object.keys(ports),js+";return reclaimRepositoryRustImageWithWaitV1;")(...Object.values(ports)) as (original:routing.ScriptInvocation,phase:leases.ResourceLeaseControl)=>Promise<unknown>;
}

async function tryPrivateSharedDescendant(catalog:string,resource:string,owned:boolean):Promise<string>{
 const entry=import.meta.resolve("../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts");
 const source=`import{acquireQueuedResourceLease}from${JSON.stringify(entry)};const [directory,resource]=process.argv.slice(1);try{const lease=await acquireQueuedResourceLease({directory,resource,mode:"shared",sharedCohort:true,owner:process.pid.toString(),signal:AbortSignal.timeout(3000),timeoutMs:1000});lease.release();console.log("held");}catch{console.log("waited");}`;
 const child=Bun.spawn([process.execPath,"-e",source,catalog,resource],{stdout:"pipe",stderr:"pipe",signal:AbortSignal.timeout(5000),env:{...process.env,SEMIO_BUILD_INVOCATION_PID:undefined,SEMIO_BUILD_INVOCATION_PIDS:owned?String(process.pid):undefined}});
 const [out,err,status]=await Promise.all([new Response(child.stdout).text(),new Response(child.stderr).text(),child.exited]);
 expect(status,err).toBe(0);
 return out.trim();
}

test("the Rust wait uses the real fair writer, requalifies after release and issues only the proven reclaim cycle",()=>withBuildTestInvocationV1(async original=>{
 const ajv=new Ajv({strict:true,strictRequired:false});
 expect(ajv.validate(schema,fixture)).toBe(true);
 const validateReceipt=ajv.compile(settings.$defs.RustReclaimWaitReceipt);
 using oracle=new Database(":memory:");
 const expected=oracle.query(`SELECT $fault IN ('none','substitute','mapping') AS granted, $fault='none' AS reclaimed, CASE
  WHEN $fault IN ('pressure','not-due') THEN 'not-due'
  WHEN $fault='retiring' THEN 'backing-unavailable'
  WHEN $fault='lost-while-waiting' THEN 'opportunity-ended'
  WHEN $fault IN ('substitute','mapping') THEN 'reclaim-failed'
  WHEN $fault='timeout' THEN 'lease-timeout'
  WHEN $fault IN ('platform','lease-failed') THEN $fault END AS reason`);
 const artifacts=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!artifacts)throw Error("Private reclaim fixture directory required");
 for(const row of fixture){
  const area=mkdtempSync(join(artifacts,"rw-")),directory=join(area,"r"),root=join(directory,"m"),image=join(directory,"fixed.dmg"),catalog=join(area,"l"),resource="native-fixed-image:"+directory;
  mkdirSync(root,{recursive:true});mkdirSync(catalog);writeFileSync(image,"private reclaim model");
  const backing=lstatSync(image),mounted=lstatSync(root,{bigint:true}),filesystemBytes=42948624384,capacityBytes=42949672960;
  writeFileSync(join(directory,"image.json"),JSON.stringify({version:1,filesystemBytes,maximumPhysicalBytes:capacityBytes,dev:backing.dev,ino:backing.ino,size:backing.size,mountDirectory:{dev:String(backing.dev),ino:String(mounted.ino),mode:Number(mounted.mode)},bootSession:"00000000-0000-4000-8000-000000000001",ancestry:readNativeImageAncestryV1(directory)}));
  const stop=new AbortController(),control=routing.createRequestedIssuedResourceControlV1(routing.createIssuedResourceControlV1(original,"test:rust-wait-"+row.fault,12000),0,stop.signal),waiting=Promise.withResolvers<void>();
  let free=row.fault==="pressure"?60000000000:33000000000,used=row.fault==="not-due"?30000000000:7000000000,granted=false,reclaimed=false,blocked=false;
  const holder=await leases.withLeaseTaskV1(()=>leases.acquireQueuedResourceLease({directory:catalog,resource,mode:"shared",owner:"fixture-live-holder",signal:control.signal,sharedCohort:true}));
  try{
   const route=waitRoute({process:{platform:row.fault==="platform"?"linux":"darwin"},schema:settings,protocol:settings.$defs.InactivePublicationProtocol.const,join,randomBytes,validateJsonSchemaSubset,nativeStorageSchema,nativeMountSchema,nativeImageReclaimDueV1,nativeImageReclaimMechanismV1,repositoryPartitionReclaimRuleV1,captureNativeImageBackingAncestryV1,requireNativeImagePublicationSettledV1,readNativeImageAncestryV1,requireNativeImageAncestryBindingV1,readNativeImageRetirementFileV1,
    requireIssuedResourceControlInvocationV1:routing.requireIssuedResourceControlInvocationV1,checkResourceLeaseControl:leases.checkResourceLeaseControl,ResourceLeaseTimeoutErrorV1:leases.ResourceLeaseTimeoutErrorV1,captureResourceLeaseWriterProofV1:leases.captureResourceLeaseWriterProofV1,requireResourceLeaseWriterProofV1:leases.requireResourceLeaseWriterProofV1,resourceLeasePathV1:leases.resourceLeasePathV1,
    callerRepositoryRootV1:(cap:unknown)=>{expect(cap).toBe(original.capabilities);return area;},repositoryCacheBudgetV1:()=>({partitions:{rust:{directory,filesystemBytes,capacityBytes}}}),nativeImageRootV1:()=>root,repositoryLeaseDirectoryV1:()=>catalog,nativeRecomputableRetirementPendingV1:()=>row.fault==="retiring",
    lstatSync:(path:string)=>{const stat=lstatSync(path,{bigint:true});if(path===root)stat.dev+=1n;if(path===image)stat.blocks=42888224768n/512n;return stat;},
    statfsSync:(path:string)=>path===directory?{bsize:1,bavail:free,blocks:1e12}:{bsize:1,blocks:1e11,bfree:1e11-used},
    acquireQueuedResourceLease:async(options:leases.QueuedLeaseOptions)=>{expect(options.control).toBe(control);expect(options.resource).toBe(resource);expect(options.mode).toBe("exclusive");expect(options.yielding).toBe(false);expect(options.timeoutMs).toBe(120000);if(row.fault==="lease-failed")throw Error("Private catalog read failed");const lease=await leases.acquireQueuedResourceLease({...options,timeoutMs:row.fault==="timeout"?300:options.timeoutMs,onWait:async event=>{blocked=true;if(row.fault==="lost-while-waiting")free=60000000000;waiting.resolve();await options.onWait?.(event);}});granted=true;return lease;},
    captureOwnedCommandV1:async(program:string,args:string[])=>{expect(granted).toBe(true);expect(program).toBe("/usr/bin/hdiutil");if(args[0]==="detach"){reclaimed=true;return "";}return "private kernel inventory";},
    nativeImageInventoryV1:()=>({images:[{"image-path":image,"system-entities":[{"dev-entry":"/dev/disk1"},{"dev-entry":"/dev/disk1s1","mount-point":row.fault==="mapping"?area:root}]}]}),
    createNativeImageReclaimPortV1:(_platform:string,options:{run:(program:string,args:string[])=>Promise<string>})=>options,
    reclaimNativeImageV1:async(port:{run:(program:string,args:string[])=>Promise<string>},hooks:{check():void})=>{hooks.check();await port.run("/usr/bin/hdiutil",["info","-plist"]);await port.run("/usr/bin/hdiutil",["detach","/dev/disk1"]);return {beforeBytes:1000,afterBytes:800,settled:true};}
   });
   const attempt=leases.withLeaseTaskV1(()=>route(original,control)).then(value=>({value}),error=>({error}));
   if(!["pressure","not-due","retiring","platform","lease-failed"].includes(row.fault)){
    const first=await Promise.race([waiting.promise.then(()=>"waiting"),attempt.then(()=>"ended")]);expect(first,row.fault).toBe("waiting");
    expect(blocked).toBe(true);expect(granted).toBe(false);expect(reclaimed).toBe(false);
    if(row.fault==="none"){
     expect(await tryPrivateSharedDescendant(catalog,resource,true)).toBe("held");
     expect(await tryPrivateSharedDescendant(catalog,resource,false)).toBe("waited");
     expect(granted).toBe(false);
    }
    if(row.fault==="substitute"){renameSync(image,image+".old");writeFileSync(image,"private reclaim model");}
    if(row.fault==="cancel")stop.abort(Error("private reclaim cancellation"));
    if(["none","mapping","substitute"].includes(row.fault))holder.release();
   }
   const result=await attempt;
   expect({granted:Number(granted),reclaimed:Number(reclaimed),reason:row.reason},row.fault).toEqual(expected.get({$fault:row.fault}));
   expect({granted,reclaimed},row.fault).toEqual({granted:row.granted,reclaimed:row.reclaimed});
   if(row.fault==="cancel")expect(String("error"in result?result.error:"")).toContain("private reclaim cancellation");
   else {
    const value="value"in result?result.value:undefined;
    expect(validateJsonSchemaSubset(settings.$defs.RustReclaimWaitReceipt,value,settings),row.fault).toEqual([]);
    expect(validateReceipt(value),row.fault).toBe(true);
    expect(value,row.fault).toEqual({version:1,kind:"repository-cache-reclaim-wait",partition:"rust",...row.reclaimed?{outcome:"reclaimed",beforeBytes:1000,afterBytes:800,settled:true}:{outcome:"skipped",reason:row.reason,beforeBytes:0,afterBytes:0,settled:false}});
   }
  }finally{holder.release();rmSync(area,{recursive:true,force:true});}
 }
}),60_000);
