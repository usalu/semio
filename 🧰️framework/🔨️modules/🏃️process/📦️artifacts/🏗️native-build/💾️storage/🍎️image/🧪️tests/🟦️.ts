import {mkdirSync,lstatSync,rmSync,existsSync,readdirSync,writeFileSync,readFileSync} from "node:fs";
import {copyFile,readdir,mkdir,lstat} from "node:fs/promises";
import {join} from "node:path";
import {resolve,dirname} from "node:path";
import {createHash} from "node:crypto";
import {homedir} from "node:os";
import {captureOwnedCommandV1} from "../../../../../🎛️owned-execution/🟦️.ts";
import type {ResourceLeaseControl} from "../../../../../🔒️leases/🟦️.ts";
import {acquireNativeStorageV1,nativeStorageRetirementPortV1,nativeStorageStateV1,nativeStorageEventV1,nativeStorageUsedBytesV1,recoverNativeStorageReservationV1,requireNativeStorageReservationV1,type NativeStoragePolicyV1} from "../../🟦️.ts";
import {provisionNativeImageStorageV1,openNativeImageStorageV1,retireNativeImageStorageV1} from "../🟦️.ts";
import {requireNativeStoragePathV1,type NativeImageStorageV1} from "../../🌐️platform/📋️contract/🟦️.ts";
import fixture from "./🧫️fixtures/🔣️.json" with {type:"json"};
import leaves from "./🧫️fixtures/📦️leaves.json" with {type:"json"};
import specimens from "./🧫️fixtures/📦️specimens.json" with {type:"json"};
import {buildCargoArtifacts} from "../../../🟦️.ts";
import {originalNativeLibraryEnvironment} from "../../../../../🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";

/** 🧱️ Qualifies actual independent Source and its declared include/test/data inputs under one finite caller backing. */
export async function proveBoundedNativeSpecimensV1(workspace:string,directory:string,leaseDirectory:string,mode:"prototype"|"release",control:ResourceLeaseControl):Promise<void>{
 const image=await provisionNativeImageStorageV1({directory,filesystemBytes:fixture.image.filesystemBytes,maximumPhysicalBytes:fixture.image.maximumPhysicalBytes,control}),owned=requireNativeStoragePathV1(image.root,join(image.root,"o"));let allocation:Awaited<ReturnType<typeof acquireNativeStorageV1>>|undefined;
 try{
  const policy:NativeStoragePolicyV1={version:1,root:image.root,leaseDirectory,capacityBytes:image.maximumPhysicalBytes,maximumWorkspaceBytes:image.maximumPhysicalBytes,maximumArtifactBytes:image.maximumPhysicalBytes},port=nativeStorageRetirementPortV1(image.port,async()=>rmSync(owned,{recursive:true,force:true}));allocation=await acquireNativeStorageV1({policy,control,port,owner:"original-independent-specimens:"+mode,kind:"compiler-workspace",bytes:25165824});mkdirSync(owned);
  const environment={...process.env,TMPDIR:owned,TMP:owned,TEMP:owned},profile=mode==="prototype"?["-C","opt-level=0","-C","debuginfo=1"]:["-C","opt-level=3","-C","lto=thin","-C","codegen-units=1","-C","strip=debuginfo"],toolchain=(await captureOwnedCommandV1("rustc",["--print","target-libdir"],workspace,{control,env:environment})).trim();
  for(const row of specimens.specimens){const files=row.inputs.map(path=>resolve(workspace,path)),digest=()=>files.map(path=>({path,sha256:createHash("sha256").update(readFileSync(path)).digest("hex")})),before=digest(),bytes=files.reduce((sum,path)=>sum+lstatSync(path).size,0),lines=files.filter(path=>path.endsWith(".rs")).reduce((sum,path)=>sum+readFileSync(path,"utf8").split("\n").length,0);if(bytes>262144||lines>4000||files.length>32)throw Error("Original specimen Source exceeds global boundary");await allocation.recheck();const source=files[0]!;
   await captureOwnedCommandV1("rustc",["--crate-name",row.name,"--edition=2024","--crate-type=rlib","--emit=metadata,link","--out-dir",owned,...profile,source],workspace,{control,env:environment});
   if(row.test){const executable=requireNativeStoragePathV1(image.root,join(owned,"native-laws"));await captureOwnedCommandV1("rustc",["--crate-name",row.name,"--edition=2024","--test","-o",executable,...profile,source],workspace,{control,env:environment});console.log(await captureOwnedCommandV1(executable,["--nocapture"],workspace,{control,env:originalNativeLibraryEnvironment(environment,toolchain)}));rmSync(executable);}
   if(JSON.stringify(before)!==JSON.stringify(digest()))throw Error("Original specimen inputs changed during compiler invocation");await allocation.recheck();console.log("[DEBUG] Actual independent original Source qualified "+JSON.stringify({mode,name:row.name,nativeTests:row.test,bytes,lines,inputs:before,imageAllocatedBytes:lstatSync(image.image).blocks*512}));for(const file of readdirSync(owned))rmSync(requireNativeStoragePathV1(image.root,join(owned,file)),{recursive:true});
  }
 }finally{try{await allocation?.retire();}finally{await image.close();}}
}

/** 🪪️ Qualifies the original zero-native-dependency identity Source independently from Cargo resolution. */
export async function proveBoundedNativeIdentityV1(workspace:string,directory:string,leaseDirectory:string,mode:"prototype"|"release",control:ResourceLeaseControl):Promise<void>{
 const image=await provisionNativeImageStorageV1({directory,filesystemBytes:fixture.image.filesystemBytes,maximumPhysicalBytes:fixture.image.maximumPhysicalBytes,control}),source=resolve(workspace,"🧰️framework/🛍️products/💻️os/🔨️modules/🪪️identity/🦀️.rs"),hash=()=>createHash("sha256").update(readFileSync(source)).digest("hex"),before=hash(),owned=join(image.root,"o");let allocation:Awaited<ReturnType<typeof acquireNativeStorageV1>>|undefined;
 try{
  const policy:NativeStoragePolicyV1={version:1,root:image.root,leaseDirectory,capacityBytes:image.maximumPhysicalBytes,maximumWorkspaceBytes:image.maximumPhysicalBytes,maximumArtifactBytes:image.maximumPhysicalBytes},port=nativeStorageRetirementPortV1(image.port,async()=>rmSync(owned,{recursive:true,force:true}));allocation=await acquireNativeStorageV1({policy,control,port,owner:"original-identity:"+mode,kind:"compiler-workspace",bytes:16777216});mkdirSync(owned);
  const environment={...process.env,TMPDIR:owned,TMP:owned,TEMP:owned},profile=mode==="prototype"?["-C","opt-level=0","-C","debuginfo=1","-C","codegen-units=256"]:["-C","opt-level=3","-C","lto=thin","-C","codegen-units=1","-C","strip=debuginfo"],name="semio_framework_os_identity";
  await allocation.recheck();console.log(await captureOwnedCommandV1("rustc",["--crate-name",name,"--edition=2021","--crate-type=rlib","--emit=metadata,link","--out-dir",owned,...profile,source],workspace,{control,env:environment}));
  const test=requireNativeStoragePathV1(image.root,join(owned,"identity-laws"));await captureOwnedCommandV1("rustc",["--crate-name",name,"--edition=2021","--test","-o",test,...profile,source],workspace,{control,env:environment});const directory=(await captureOwnedCommandV1("rustc",["--print","target-libdir"],workspace,{control,env:environment})).trim();if(!existsSync(directory)||!lstatSync(directory).isDirectory())throw Error("Original native identity toolchain directory required");
  console.log(await captureOwnedCommandV1(test,["--nocapture"],workspace,{control,env:originalNativeLibraryEnvironment(environment,directory)}));if(before!==hash())throw Error("Original identity Source changed during compilation");await allocation.recheck();console.log("[DEBUG] Actual bounded original identity library and native laws "+JSON.stringify({mode,source,sourceSha256:before,files:readdirSync(owned),allocatedBytes:lstatSync(image.image).blocks*512}));
 }finally{try{await allocation?.retire();}finally{await image.close();}}
}

/** 🦀️ Compiles the actual selected authored leaf under one finite original image and caller. */
export async function proveBoundedNativeLeavesV1(workspace:string,directory:string,leaseDirectory:string,mode:"prototype"|"release",control:ResourceLeaseControl):Promise<void>{
 const image=await provisionNativeImageStorageV1({directory,filesystemBytes:leaves.filesystemBytes,maximumPhysicalBytes:leaves.maximumPhysicalBytes,control});let allocation:Awaited<ReturnType<typeof acquireNativeStorageV1>>|undefined;
 const policy:NativeStoragePolicyV1={version:1,root:image.root,leaseDirectory,capacityBytes:image.maximumPhysicalBytes,maximumWorkspaceBytes:leaves.reservationBytes,maximumArtifactBytes:leaves.reservationBytes},owned=join(image.root,"o"),port=nativeStorageRetirementPortV1(image.port,async()=>{rmSync(owned,{recursive:true,force:true});});
 try{
  allocation=await acquireNativeStorageV1({policy,control,port,owner:"original-real-leaves:"+mode,kind:"compiler-workspace",bytes:leaves.reservationBytes});mkdirSync(owned);const home=join(owned,"h");mkdirSync(home);
  const external=process.env.CARGO_HOME??join(homedir(),".cargo");let copied=0;
  const copyInputs=async(source:string,target:string):Promise<void>=>{await allocation!.recheck();await mkdir(target,{recursive:true});for(const entry of await readdir(source,{withFileTypes:true})){const input=join(source,entry.name),output=requireNativeStoragePathV1(image.root,join(target,entry.name)),status=await lstat(input);if(status.isSymbolicLink())throw Error("Original registry input must be regular");if(status.isDirectory())await copyInputs(input,output);else if(status.isFile()){if(status.size>leaves.reservationBytes-copied)throw Error("Registry inputs exceed original reserved compiler working set");await allocation!.recheck();await copyFile(input,output);copied+=status.size;}}};
  for(const category of["index","cache"])await copyInputs(join(external,"registry",category),join(home,"registry",category));console.log("[DEBUG] Original measured registry index/archive input copy "+JSON.stringify({bytes:copied,withinReservation:leaves.reservationBytes}));
  for(const relative of leaves.manifests){const manifest=join(workspace,relative),remaining=control.remainingMilliseconds();if(remaining===null||remaining<=30000)throw Error("Original compiler leaves require funded terminal cleanup reserve");await buildCargoArtifacts(manifest,["--lib","--offline"],{version:1,mode,cwd:workspace,compilerConfiguration:join(workspace,".cargo/config.toml"),buildDirectory:join(owned,"b"),captureDirectory:join(owned,"c"),leaseDirectory,budgetMs:Math.floor(remaining-30000)},{control,storage:allocation,sourceBoundary:{version:1,maxSourceBytes:262144,maxSourceFiles:32,maxActiveLines:4000},output:join(owned,"a",createHash("sha256").update(manifest).digest("hex").slice(0,12)),environment:{...process.env,CARGO_HOME:home,SEMIO_TEST_ARTIFACT_DIR:join(owned,"p")}});console.log("[DEBUG] Actual bounded authored Cargo leaf completed "+JSON.stringify({manifest,mode}));}
  await image.recheck();
 }finally{try{await allocation?.retire();}finally{await image.close();}}
}

/** 🧯️ Recovers only the explicitly owned failed Nx fixture after original children and current readers settle. */
export async function recoverOriginalFailedImageV1(directory:string,leaseDirectory:string,control:ResourceLeaseControl):Promise<void>{
 const receipt=JSON.parse(readFileSync(join(directory,"image.json"),"utf8")),image=await openNativeImageStorageV1({directory,leaseDirectory,filesystemBytes:receipt.filesystemBytes,maximumPhysicalBytes:receipt.maximumPhysicalBytes+1048576,control});
 try{await image.withExclusiveMaintenance({version:1,wait:true},async actual=>{
  const processes=await captureOwnedCommandV1("ps",["-axo","pid,ppid,stat"],directory,{control});if(Buffer.byteLength(processes)>=64512)throw Error("Current process inventory exceeds original capture allowance");console.log("[DEBUG] Original failed-image recovery obtained complete process inventory bytes="+Buffer.byteLength(processes));
  let handles:string;try{handles=await captureOwnedCommandV1("/usr/sbin/lsof",["+f","--",image.root],directory,{control});}catch(error){if(!(error instanceof Error)||!error.message.endsWith("exited with status 1\n"))throw error;handles="";}if(handles.trim())throw Error("Original failed image still has current handles");console.log("[DEBUG] Actual lsof entire mounted filesystem contains no handles under exclusive image custody");
  let state=nativeStorageStateV1();for(const line of readFileSync(join(image.root,"storage-events.jsonl"),"utf8").split("\n").filter(Boolean))state=nativeStorageEventV1(state,JSON.parse(line),image.maximumPhysicalBytes);
  if(state.allocations.length!==1||state.allocations[0]?.owner!=="nx:actual:publish"||state.allocations[0]?.kind!=="artifact-publication"||state.allocations[0]?.bytes!==67108864)throw Error("Exact original failed fixture allocation required");
  const policy:NativeStoragePolicyV1={version:1,root:image.root,leaseDirectory,capacityBytes:image.maximumPhysicalBytes,maximumWorkspaceBytes:image.maximumPhysicalBytes,maximumArtifactBytes:image.maximumPhysicalBytes},port=nativeStorageRetirementPortV1(actual,async()=>{for(const name of["workspace","cache","data","tmp","native"]){const path=requireNativeStoragePathV1(image.root,join(image.root,name));rmSync(path,{recursive:true,force:true});}}),allocation=await recoverNativeStorageReservationV1({policy,control,port,id:state.allocations[0]!.id});await allocation.retire();console.log("[DEBUG] Exact original failed Nx allocation physically retired without PID or age inference");
 });}finally{await image.close();}
 await retireNativeImageStorageV1({directory,leaseDirectory,control});
}
/** 🧪️ Qualifies concurrent shared reopen, durable crash charge and original physical release. */
export async function proveSharedNativeImageStorageV1(directory:string,leaseDirectory:string,control:ResourceLeaseControl):Promise<void>{
 const options={directory,leaseDirectory,filesystemBytes:fixture.shared.filesystemBytes,maximumPhysicalBytes:fixture.shared.maximumPhysicalBytes,control};const first=await openNativeImageStorageV1(options);let second:NativeImageStorageV1|undefined;
 try{
  second=await openNativeImageStorageV1(options);if(first.image!==second.image||first.root!==second.root)throw Error("Shared original image identity diverged");
  const policy:NativeStoragePolicyV1={version:1,root:first.root,leaseDirectory,capacityBytes:first.maximumPhysicalBytes,maximumWorkspaceBytes:first.maximumPhysicalBytes,maximumArtifactBytes:first.maximumPhysicalBytes};
  let acknowledged=false;const owned= new Set<string>();
  const port=nativeStorageRetirementPortV1(first.port,async(allocation)=>{if(!acknowledged)throw Error("original writer still held");const path=requireNativeStoragePathV1(first.root,join(first.root,"allocations",allocation.id));if(!owned.has(path))throw Error("Original allocation path not held");rmSync(path,{recursive:true});owned.delete(path);});
  const allocation=await acquireNativeStorageV1({policy,control,port,owner:"original-shared-proof",kind:"compiler-workspace",bytes:fixture.shared.firstReservationBytes});
  const path=requireNativeStoragePathV1(first.root,join(first.root,"allocations",allocation.allocation.id));mkdirSync(path,{recursive:true});owned.add(path);writeFileSync(join(path,"original"),"original finite source");
  let refused=false;try{await acquireNativeStorageV1({policy,control,port:second.port,owner:"original-second-proof",kind:"compiler-workspace",bytes:fixture.shared.contenderBytes});}catch(error){if(!(error instanceof Error)||!error.message.includes("capacity"))throw error;refused=true;}if(!refused)throw Error("Concurrent reservation exceeded capacity");
  try{await allocation.retire();throw Error("Missing physical retirement witness accepted");}catch(error){if(!(error instanceof Error)||!error.message.includes("writer still held"))throw error;}
  let state=nativeStorageStateV1();for(const line of readFileSync(join(first.root,"storage-events.jsonl"),"utf8").split("\n").filter(Boolean))state=nativeStorageEventV1(state,JSON.parse(line),policy.capacityBytes);
  if(nativeStorageUsedBytesV1(state)!==fixture.shared.remainingChargeBytes||state.allocations[0]?.phase!=="retiring")throw Error("Interrupted physical retirement lost charge");const recovered=await recoverNativeStorageReservationV1({policy,control,port,id:allocation.allocation.id});requireNativeStorageReservationV1(recovered,control);const stale=await recoverNativeStorageReservationV1({policy,control,port,id:allocation.allocation.id});acknowledged=true;await recovered.retire();let staleRefused=false;try{await stale.recheck();}catch(error){if(!(error instanceof Error)||!error.message.includes("no longer held"))throw error;staleRefused=true;}if(!staleRefused)throw Error("Retired durable allocation admitted stale writer");if(existsSync(path))throw Error("Original allocation still exists");
  await first.recheck();await second.recheck();console.log("[DEBUG] Shared1GiB image simultaneous original users; excess reservation refused; interrupted retirement retained734003200 bytes until original writer acknowledgement");
 }finally{await second?.close();await first.close();await retireNativeImageStorageV1({directory,leaseDirectory,control});}
}

/** 🧪️ Qualifies real bounded writes and retained host image size using the original controller. */
export async function proveNativeImageStorageV1(directory:string,control:ResourceLeaseControl):Promise<void>{
 const image=await provisionNativeImageStorageV1({directory,filesystemBytes:fixture.image.filesystemBytes,maximumPhysicalBytes:fixture.image.maximumPhysicalBytes,control});
 try{
  const output=await captureOwnedCommandV1(process.execPath,["-e","const fs=require('node:fs');const path=require('node:path');let bytes=0;const fd=fs.openSync(path.join(process.argv[1],'fill'),'wx');const data=Buffer.alloc(1048576,37);try{for(let i=0;i<100;i++){bytes+=fs.writeSync(fd,data);}}catch(error){if(error.code!=='ENOSPC')throw error;console.log('[DEBUG] Real fixed filesystem refused oversized child output '+JSON.stringify({code:error.code,writtenBytes:bytes}));}finally{fs.closeSync(fd);}if(bytes>=33554432)throw Error('fixed limit not enforced');",image.root],image.root,{control,env:{...process.env,TMPDIR:image.root,TMP:image.root,TEMP:image.root}});
  if(!output.includes('"code":"ENOSPC"'))throw Error("Real filesystem cap-plus-one did not refuse");console.log(output.trim());await image.recheck();
  const actual=lstatSync(image.image);if(actual.blocks*512>image.maximumPhysicalBytes)throw Error("Host image allocation exceeded fixed bound");console.log("[DEBUG] Full fixed image host allocation "+JSON.stringify({allocatedBytes:actual.blocks*512,maximumPhysicalBytes:image.maximumPhysicalBytes,files:readdirSync(image.root)}));
 }finally{await image.close();}
}
