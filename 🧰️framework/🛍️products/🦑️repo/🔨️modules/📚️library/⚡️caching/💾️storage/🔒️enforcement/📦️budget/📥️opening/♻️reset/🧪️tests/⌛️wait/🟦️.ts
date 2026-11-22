import {test,expect} from "bun:test";
import {Database} from "bun:sqlite";
import {randomBytes} from "node:crypto";
import {lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {join,sep} from "node:path";
import Ajv from "ajv";
import ts from "typescript";
import {declareLibraryBuildTestEnvelopeV1} from "../../../../../../../../🧪️testing/🪪️native-birth/🟦️.ts";
import {withBuildTestInvocationV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/📥️invocation/🟦️.ts";
import * as leases from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import * as routing from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {captureNativeImageBackingAncestryV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🟦️.ts";
import {requireNativeImagePublicationSettledV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {readNativeImageAncestryV1,requireNativeImageAncestryBindingV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🟦️.ts";
import {readNativeImageRetirementFileV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🧱️fixed/🟦️.ts";
import {nativeImageReclaimMechanismV1,reclaimNativeImageV1} from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🟦️.ts";
import nativeStorageSchema from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🧬️schema/🔣️json/🟦️.ts";
import nativeMountSchema from "../../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/📂️mount/🧬️schema/🔣️json/🟦️.ts";
import {validateJsonSchemaSubset} from "../../../../../../../../../../../../🔨️modules/🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import {repositoryRustReclaimWaitDueV1} from "../../../../🟦️.ts";
import settings from "../../🧬️schema/🔣️.json";
import fixture from "./🔣️.json";
import schema from "./🧬️schema/🔣️.json";
import cachePolicy from "../../../../🔣️policy.json";
import pressureCases from "./pressure.json";

declareLibraryBuildTestEnvelopeV1({suite:"storage-opening-processes",maximumElapsedMilliseconds:120000});

function waitRoute(ports:Record<string,unknown>){
 const tree=ts.createSourceFile("reclaim.ts",readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true);
 const names=["captureRepositoryRustReclaimV1","requireRepositoryRustReclaimMappingV1","renderRustReclaimWaitFailureV1","reclaimRepositoryRustImageWithWaitV1"];
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
 const expected=oracle.query(`SELECT $fault IN ('none','mounted-pressure','substitute','mapping','writer-proof','inventory','detach','attach-cleanup-release','cleanup-scratch') AS granted, $fault IN ('none','mounted-pressure') AS reclaimed, CASE
  WHEN $fault IN ('pressure','not-due') THEN 'not-due'
  WHEN $fault='retiring' THEN 'backing-unavailable'
  WHEN $fault='lost-while-waiting' THEN 'opportunity-ended'
  WHEN $fault IN ('substitute','mapping','writer-proof','inventory','detach','attach-cleanup-release','cleanup-scratch') THEN 'reclaim-failed'
  WHEN $fault='timeout' THEN 'lease-timeout'
  WHEN $fault IN ('platform','lease-failed') THEN $fault END AS reason`);
 const expectedNotices=oracle.query(`SELECT value FROM json_each(CASE $fault
  WHEN 'substitute' THEN '["backing-recheck"]' WHEN 'mapping' THEN '["mapping"]'
  WHEN 'writer-proof' THEN '["writer-proof"]' WHEN 'inventory' THEN '["inventory"]'
  WHEN 'detach' THEN '["detach"]' WHEN 'attach-cleanup-release' THEN '["attach","cleanup-detach","lease-release"]'
  WHEN 'cleanup-scratch' THEN '["cleanup-scratch"]' WHEN 'retiring' THEN '["backing-inspection"]'
  WHEN 'lease-failed' THEN '["lease-acquisition"]' ELSE '[]' END) ORDER BY key`);

 const configured=process.env.SEMIO_TEST_ARTIFACT_DIR;if(!configured)throw Error("Private reclaim fixture directory required");const artifacts=realpathSync(configured);
 for(const row of fixture){
  const area=mkdtempSync(artifacts.endsWith(sep)?artifacts:artifacts+sep),directory=join(area,"r"),root=join(directory,"m"),image=join(directory,"fixed.dmg"),catalog=area,resource="native-fixed-image:"+directory;
  mkdirSync(root,{recursive:true});writeFileSync(image,"private reclaim model");
  const backing=lstatSync(image),mounted=lstatSync(root,{bigint:true}),filesystemBytes=42948624384,capacityBytes=42949672960;
  writeFileSync(join(directory,"image.json"),JSON.stringify({version:1,filesystemBytes,maximumPhysicalBytes:capacityBytes,dev:backing.dev,ino:backing.ino,size:backing.size,mountDirectory:{dev:String(backing.dev),ino:String(mounted.ino),mode:Number(mounted.mode)},bootSession:"00000000-0000-4000-8000-000000000001",ancestry:readNativeImageAncestryV1(directory)}));
  const stop=new AbortController(),control=routing.createRequestedIssuedResourceControlV1(routing.createIssuedResourceControlV1(original,"test:rust-wait-"+row.fault,12000),0,stop.signal),waiting=Promise.withResolvers<void>();
  const mountedPressure=row.fault==="mounted-pressure"?pressureCases[0]:undefined,gib=2**30;
  const allocated=mountedPressure?mountedPressure.allocatedGiB*gib:42888224768;
  let free=mountedPressure?mountedPressure.hostFreeGiB*gib:row.fault==="pressure"||row.fault==="not-due"?60000000000:33000000000;
  const used=mountedPressure?mountedPressure.usedGiB!*gib:row.fault==="not-due"?30000000000:7000000000;
  const mountFree=mountedPressure?mountedPressure.mountedFreeGiB*gib:1e11-used;
  let granted=false,reclaimed=false,blocked=false,cleaning=false;const commands:{cleaning:boolean;controlled:boolean}[]=[];
  const notices:string[]=[],releasedLeases=new WeakMap<object,Awaited<ReturnType<typeof leases.acquireQueuedResourceLease>>>();
  const holder=await leases.withLeaseTaskV1(()=>leases.acquireQueuedResourceLease({directory:catalog,resource,mode:"shared",owner:"fixture-live-holder",signal:control.signal,sharedCohort:true}));
  try{
   const route=waitRoute({process:{platform:row.fault==="platform"?"linux":"darwin",env:{}},console:{error:(line:string)=>notices.push(line)},formatStampLanguagesV1:()=>["en"],schema:settings,protocol:settings.$defs.InactivePublicationProtocol.const,join,randomBytes,validateJsonSchemaSubset,nativeStorageSchema,nativeMountSchema,nativeImageReclaimMechanismV1,repositoryRustReclaimWaitDueV1,captureNativeImageBackingAncestryV1,requireNativeImagePublicationSettledV1,readNativeImageAncestryV1,requireNativeImageAncestryBindingV1,readNativeImageRetirementFileV1,
    requireIssuedResourceControlInvocationV1:routing.requireIssuedResourceControlInvocationV1,checkResourceLeaseControl:leases.checkResourceLeaseControl,ResourceLeaseTimeoutErrorV1:leases.ResourceLeaseTimeoutErrorV1,captureResourceLeaseWriterProofV1:(lease:Awaited<ReturnType<typeof leases.acquireQueuedResourceLease>>,...args:Parameters<typeof leases.captureResourceLeaseWriterProofV1> extends [unknown,...infer Rest]?Rest:never)=>{if(row.fault==="writer-proof")throw Error("Private writer proof refused");return leases.captureResourceLeaseWriterProofV1(releasedLeases.get(lease)??lease,...args);},requireResourceLeaseWriterProofV1:leases.requireResourceLeaseWriterProofV1,resourceLeasePathV1:leases.resourceLeasePathV1,
    callerRepositoryRootV1:(cap:unknown)=>{expect(cap).toBe(original.capabilities);return area;},repositoryCacheBudgetV1:()=>({partitions:{rust:{directory,filesystemBytes,capacityBytes}}}),nativeImageRootV1:()=>root,repositoryLeaseDirectoryV1:()=>catalog,nativeRecomputableRetirementPendingV1:()=>row.fault==="retiring",
    lstatSync:(path:string)=>{const stat=lstatSync(path,{bigint:true});if(path===root)stat.dev+=1n;if(path===image)stat.blocks=BigInt(allocated)/512n;return stat;},
    statfsSync:(path:string)=>path===directory?{bsize:1,bavail:free,blocks:1e12}:{bsize:1,blocks:1e11,bfree:1e11-used,bavail:mountFree},
    nativeImageVolumeUsedBytesSyncV1:()=>used,
    acquireQueuedResourceLease:async(options:leases.QueuedLeaseOptions)=>{expect(options.control).toBe(control);expect(options.resource).toBe(resource);expect(options.mode).toBe("exclusive");expect(options.yielding).toBe(false);expect(options.timeoutMs).toBe(120000);if(row.fault==="lease-failed")throw Error("Private catalog read failed");const lease=await leases.acquireQueuedResourceLease({...options,timeoutMs:row.fault==="timeout"?300:options.timeoutMs,onWait:async event=>{blocked=true;if(row.fault==="lost-while-waiting")free=60000000000;waiting.resolve();await options.onWait?.(event);}});granted=true;if(row.fault!=="attach-cleanup-release")return lease;const wrapped={...lease,release(){lease.release();throw Error("Private lease release failed");}};releasedLeases.set(wrapped,lease);return wrapped;},
    captureOwnedCommandV1:async(program:string,args:string[],_directory:string,options:{control?:unknown})=>{
     expect(granted).toBe(true);commands.push({cleaning,controlled:options.control!==undefined});
     if(args[0]==="info"){expect(program).toBe("/usr/bin/hdiutil");if(row.fault==="inventory")throw Error("Private inventory unavailable");}
     if(args[0]==="detach"){expect(program).toBe("/usr/bin/hdiutil");if(row.fault==="detach")throw Error("Private mounted device busy");if(args[1]==="/dev/disk2"&&row.fault==="attach-cleanup-release")throw Error("Private cleanup detach failed");}
     if(args[0]==="image"&&row.fault==="attach-cleanup-release")throw Error("Native attach\u001b refused\n"+"x".repeat(2000)+"\ndisk is busy");
     return "private kernel inventory";
    },
    nativeImageInventoryV1:()=>({images:[{"image-path":image,"system-entities":[{"dev-entry":"/dev/disk1"},{"dev-entry":"/dev/disk1s1","mount-point":row.fault==="mapping"?area:root}]}]}),
    createNativeImageReclaimPortV1:(_platform:string,{run,cleanupRun}:{run:(program:string,args:string[])=>Promise<string>;cleanupRun:(program:string,args:string[])=>Promise<string>})=>{
     let detached=false;
     const phased=async(cleanup:"cleanup"|undefined,program:string,args:string[])=>{cleaning=cleanup!==undefined;try{return await(cleanup?cleanupRun:run)(program,args);}finally{cleaning=false;}};
     return {
      attachedDevice:async(cleanup?:"cleanup")=>{await phased(cleanup,"/usr/bin/hdiutil",["info","-plist"]);return cleanup?"/dev/disk2":"/dev/disk1";},
      detach:async(device:string,cleanup?:"cleanup")=>{await phased(cleanup,"/usr/bin/hdiutil",["detach",device]);detached=true;},
      attach:async()=>{await run("/usr/sbin/diskutil",["image","attach"]);},
      allocatedBytes:()=>detached?800:1000,
      dispose:()=>{if(row.fault==="cleanup-scratch")throw Error("Private scratch removal failed");reclaimed=true;},
     };
    },
    reclaimNativeImageV1:(port:Parameters<typeof reclaimNativeImageV1>[0],hooks:Parameters<typeof reclaimNativeImageV1>[1])=>reclaimNativeImageV1(port,{...hooks,sleep:async()=>{}})
   });
   const attempt=leases.withLeaseTaskV1(()=>route(original,control)).then(value=>({value}),error=>({error}));
   if(!["pressure","not-due","retiring","platform","lease-failed"].includes(row.fault)){
    const first=await Promise.race([waiting.promise.then(()=>"waiting"),attempt.then(value=>JSON.stringify(value,(_,field)=>field instanceof Error?field.message:field))]);expect(first,row.fault).toBe("waiting");
    expect(blocked).toBe(true);expect(granted).toBe(false);expect(reclaimed).toBe(false);
    if(row.fault==="none"){
     expect(await tryPrivateSharedDescendant(catalog,resource,true)).toBe("held");
     expect(await tryPrivateSharedDescendant(catalog,resource,false)).toBe("waited");
     expect(granted).toBe(false);
    }
    if(row.fault==="substitute"){renameSync(image,image+".old");writeFileSync(image,"private reclaim model");}
    if(row.fault==="cancel")stop.abort(Error("private reclaim cancellation"));
    if(["none","mounted-pressure","mapping","substitute","writer-proof","inventory","detach","attach-cleanup-release","cleanup-scratch"].includes(row.fault))holder.release();
   }
   const result=await attempt;
   expect(notices.map(line=>/\(([^)]+)\):/u.exec(line)?.[1]),row.fault).toEqual(expectedNotices.all({$fault:row.fault}).map(row=>(row as {value:string}).value));
   if(row.fault==="none"){
    expect(commands.some(command=>command.cleaning)).toBe(true);
    for(const command of commands)expect(command.controlled,"cleanup commands run without the cancellable control of the cycle").toBe(!command.cleaning);
   }
   if(row.fault==="attach-cleanup-release"){
    expect(notices[0]).toContain("Native attach refused");
    expect(notices[0]).toEndWith("disk is busy");
    expect(notices[0]).not.toMatch(/[\n\u001b]/u);
    expect(notices[0]!.split(": ")[1]!.length).toBeLessThanOrEqual(settings.$defs.RustReclaimWait.const.maximumDiagnosticCharacters);
    expect(notices[1]).toEndWith("Private cleanup detach failed");
    expect(notices[2]).toEndWith("Private lease release failed");
   }
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

test("mounted Rust reclaim checks actual volume pressure and the full material reclaim floor", () => {
 const gib=2**30;
 using oracle=new Database(":memory:");
 const independentlyDue=oracle.query(`
  SELECT $used IS NOT NULL AND (
   ($mounted < $pressure AND $allocated - $used >= $floor)
   OR ($host < min($total, $tight) AND $allocated - $used >= $floor / 8)
  ) AS due
 `);
 const rule=cachePolicy.elastic.rust;
 for(const row of pressureCases){
  const hostFreeBytes=Math.round(row.hostFreeGiB*gib),hostTotalBytes=row.hostTotalGiB*gib,mountedFreeBytes=row.mountedFreeGiB*gib;
  const allocatedBytes=Math.round(row.allocatedGiB*gib),usedBytes=row.usedGiB===null?undefined:row.usedGiB*gib;
  expect(independentlyDue.get({$host:hostFreeBytes,$total:hostTotalBytes,$mounted:mountedFreeBytes,$allocated:allocatedBytes,$used:usedBytes??null,
   $pressure:rule.pressureFreeBytes,$floor:rule.reclaimFloorBytes,$tight:rule.reclaimTightHostFreeBytes}),row.id).toEqual({due:Number(row.due)});
  expect(repositoryRustReclaimWaitDueV1({hostFreeBytes,hostTotalBytes,mountedFreeBytes,allocatedBytes,usedBytes}),row.id).toBe(row.due);
 }
});
