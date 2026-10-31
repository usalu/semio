import{withRepositoryRuntimeEntriesV1}from"../../🟦️.ts";
import assert from "node:assert/strict";
import{testScratchRootV1}from"../../../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🏖️sandbox/🟦️.ts";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createIssuedResourceControlV1,receiveScriptProcessInvocation} from"../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {nativeImageHdiutilProgramV1} from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🛠️hdiutil/🟦️.ts";
import {nativeImageAttachedV1,nativeImageBackingFilenameV1,nativeImageFilesystemBytesV1,nativeImageRootV1,nativeImageSpaceV1,nativeStatfsV1} from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";
import {openNativePlatformStorageV1,retireNativePlatformStorageV1} from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🟦️.ts";
import {acquireNativeStorageV1} from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🟦️.ts";
import {nativeStorageBackendsV1,nativeStorageSelectionV1,selectedNativeStorageDeclarationV1,withNativeStorageSelectionV1} from"../../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🔌️backends/📋️contract/🟦️.ts";

const names={backend:"SEMIO_NATIVE_STORAGE_BACKEND",child:"SEMIO_STORAGE_CONFORMANCE_CHILD"};

const filesystemBytes=33554432,maximumPhysicalBytes=34603008,payloadBytes=1048576;
const tool=(args:string[])=>spawnSync(nativeImageHdiutilProgramV1,args,{encoding:"utf8"});
const python=(code:string,...args:string[])=>spawnSync("python3",["-I","-c",code,...args],{encoding:"utf8"});
const walkedBytes=(root:string)=>Number(python("import os,sys\nt=0\nfor d,_,fs in os.walk(sys.argv[1]):\n for f in fs:\n  p=os.path.join(d,f)\n  if os.path.isfile(p) and not os.path.islink(p): t+=os.lstat(p).st_blocks*512\nprint(t)",root).stdout.trim());

/** ✅️ The storage contract every backend must satisfy, expressed only through the neutral facade, the neutral helpers and the selection; the same body runs once per declared backend. */
async function conform(expected:string){
 const declaration=selectedNativeStorageDeclarationV1();assert.equal(declaration.id,expected);assert.equal(nativeImageBackingFilenameV1(),declaration.backingFilename);
 const sandbox=realpathSync(mkdtempSync(join(testScratchRootV1("conformance"),"sc-")));
 const hostNeedsCaseSensitiveVolume=process.platform==="darwin"&&!declaration.capabilities.includes("mounted-volume"),volume=join(sandbox,"cs");
 let scratch=sandbox;
 if(hostNeedsCaseSensitiveVolume){mkdirSync(volume);const created=tool(["create","-size","256m","-fs","Case-sensitive APFS","-volname","semio-sandbox","-ov",join(sandbox,"cs.dmg")]);assert.equal(created.status,0,created.stdout+created.stderr);const attached=tool(["attach","-nobrowse","-noautoopen","-mountpoint",volume,join(sandbox,"cs.dmg")]);assert.equal(attached.status,0,attached.stdout+attached.stderr);scratch=realpathSync(volume);}
 const directory=join(scratch,"p"),leaseDirectory=join(scratch,"leases"),root=nativeImageRootV1(directory);
 mkdirSync(leaseDirectory);
 try{
  await receiveScriptProcessInvocation(process.env,async original=>{
   const session=createIssuedResourceControlV1(original,"storage-conformance:session",200000),control=(_name:string)=>session;return withRepositoryRuntimeEntriesV1(process.cwd(),original,session,async()=>{const options=(name:string)=>({directory,leaseDirectory,filesystemBytes,maximumPhysicalBytes,control:control(name)});
   const first=await openNativePlatformStorageV1({directory,leaseDirectory,filesystemBytes,maximumPhysicalBytes,control:session});
   assert.equal(first.root,root);assert.equal(first.image,join(directory,declaration.backingFilename));assert.equal(existsSync(first.image),true);
   assert.equal(first.maximumPhysicalBytes,maximumPhysicalBytes);assert.ok(first.filesystemBytes<=filesystemBytes);assert.ok(first.filesystemBytes>0);
   assert.equal(nativeImageAttachedV1(first.image,first.root),true);
   const capacity=nativeImageFilesystemBytesV1(first.root,filesystemBytes);assert.ok(capacity>0);assert.ok(capacity<=filesystemBytes);
   const space=nativeImageSpaceV1(first.root,filesystemBytes);assert.ok(Number.isInteger(space.bsize)&&space.bsize>0);assert.ok(space.bavail>=0);assert.ok(space.bavail*space.bsize<=filesystemBytes);
   const other=nativeStorageBackendsV1.find(row=>row.id!==expected&&row.platforms.includes(nativeStorageSelectionV1().platform!));
   if(other)await assert.rejects(()=>withNativeStorageSelectionV1({backend:other.id},()=>openNativePlatformStorageV1(options("conflict"))),/belongs to another backend/);
   const policy={version:1 as const,root:first.root,leaseDirectory,capacityBytes:maximumPhysicalBytes,maximumWorkspaceBytes:filesystemBytes,maximumArtifactBytes:filesystemBytes};
   const reservation=await acquireNativeStorageV1({policy,control:session,port:first.port,owner:"storage-conformance",kind:"compiler-workspace",bytes:payloadBytes}) as {commit():Promise<void>};
   writeFileSync(join(first.root,"payload.bin"),Buffer.alloc(payloadBytes,7));await reservation.commit();
   const statfs=await nativeStatfsV1(first.root,{fresh:true}),used=(statfs.blocks-statfs.bfree)*statfs.bsize;
   assert.ok(used>=payloadBytes);assert.ok(statfs.blocks*statfs.bsize<=filesystemBytes);
   const walked=walkedBytes(first.root);assert.ok(walked>=payloadBytes);assert.ok(used>=walked);assert.ok(used-walked<=8*1048576);
   assert.ok(readFileSync(join(first.root,"storage-events.jsonl"),"utf8").split("\n").filter(Boolean).length>0);
   await first.close();
   const second=await openNativePlatformStorageV1(options("reopen"));assert.equal(second.root,root);assert.equal(nativeImageAttachedV1(second.image,second.root),true);
   assert.equal(readFileSync(join(second.root,"payload.bin")).byteLength,payloadBytes);
   await second.close();
   const spare=join(scratch,"q"),spareOptions=(name:string)=>({...options(name),directory:spare});
   const third=await openNativePlatformStorageV1(spareOptions("spare-open"));assert.equal(existsSync(join(spare,declaration.backingFilename)),true);await third.close();
   assert.equal(await retireNativePlatformStorageV1(spareOptions("spare-retire")),true);
   assert.equal(existsSync(join(spare,declaration.backingFilename)),false);
   console.log(JSON.stringify({version:1,backend:expected,completed:true}));
  });});
 }finally{
  if(declaration.capabilities.includes("mounted-volume"))tool(["detach","-force",root]);
  if(hostNeedsCaseSensitiveVolume)tool(["detach","-force",volume]);
  rmSync(sandbox,{recursive:true,force:true});
 }
}

if(import.meta.main)await conform(process.env[names.backend]!);
