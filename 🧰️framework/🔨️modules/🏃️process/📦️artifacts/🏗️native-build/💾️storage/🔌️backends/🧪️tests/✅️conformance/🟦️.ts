import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {createIssuedResourceControlV1,receiveScriptProcessInvocation} from "../../../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import {nativeImageHdiutilProgramV1} from "../../../🍎️image/🛠️hdiutil/🟦️.ts";
import {nativeImageAttachedV1,nativeImageBackingFilenameV1,nativeImageFilesystemBytesV1,nativeImageRootV1,nativeImageSpaceV1,nativeStatfsV1} from "../../../🌐️platform/📋️contract/🟦️.ts";
import {openNativePlatformStorageV1,retireNativePlatformStorageV1} from "../../../🌐️platform/🟦️.ts";
import {acquireNativeStorageV1} from "../../../🟦️.ts";
import {nativeStorageBackendsV1,nativeStorageSelectionV1,selectedNativeStorageDeclarationV1,withNativeStorageSelectionV1} from "../../📋️contract/🟦️.ts";

const names={backend:"SEMIO_NATIVE_STORAGE_BACKEND",child:"SEMIO_STORAGE_CONFORMANCE_CHILD"};
const child=process.env[names.child]==="1";
const filesystemBytes=33554432,maximumPhysicalBytes=34603008,payloadBytes=1048576;
const tool=(args:string[])=>spawnSync(nativeImageHdiutilProgramV1,args,{encoding:"utf8"});
const python=(code:string,...args:string[])=>spawnSync("python3",["-I","-c",code,...args],{encoding:"utf8"});
const walkedBytes=(root:string)=>Number(python("import os,sys\nt=0\nfor d,_,fs in os.walk(sys.argv[1]):\n for f in fs:\n  p=os.path.join(d,f)\n  if os.path.isfile(p) and not os.path.islink(p): t+=os.lstat(p).st_blocks*512\nprint(t)",root).stdout.trim());

/** ✅️ The storage contract every backend must satisfy, expressed only through the neutral facade, the neutral helpers and the selection; the same body runs once per declared backend. */
async function conform(expected:string){
 const declaration=selectedNativeStorageDeclarationV1();expect(declaration.id).toBe(expected);expect(nativeImageBackingFilenameV1()).toBe(declaration.backingFilename);
 const sandbox=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR||process.env.TMPDIR||tmpdir(),"sc-")));
 const hostNeedsCaseSensitiveVolume=process.platform==="darwin"&&!declaration.capabilities.includes("mounted-volume"),volume=join(sandbox,"cs");
 let scratch=sandbox;
 if(hostNeedsCaseSensitiveVolume){mkdirSync(volume);const created=tool(["create","-size","256m","-fs","Case-sensitive APFS","-volname","semio-sandbox","-ov",join(sandbox,"cs.dmg")]);expect(created.status,created.stdout+created.stderr).toBe(0);const attached=tool(["attach","-nobrowse","-noautoopen","-mountpoint",volume,join(sandbox,"cs.dmg")]);expect(attached.status,attached.stdout+attached.stderr).toBe(0);scratch=realpathSync(volume);}
 const directory=join(scratch,"p"),leaseDirectory=join(scratch,"leases"),root=nativeImageRootV1(directory);
 mkdirSync(leaseDirectory);
 try{
  await receiveScriptProcessInvocation(process.env,async original=>{
   const control=(name:string)=>createIssuedResourceControlV1(original,"storage-conformance:"+name,0),options=(name:string)=>({directory,leaseDirectory,filesystemBytes,maximumPhysicalBytes,control:control(name)});
   const session=control("session"),first=await openNativePlatformStorageV1({directory,leaseDirectory,filesystemBytes,maximumPhysicalBytes,control:session});
   expect(first.root).toBe(root);expect(first.image).toBe(join(directory,declaration.backingFilename));expect(existsSync(first.image)).toBe(true);
   expect(first.maximumPhysicalBytes).toBe(maximumPhysicalBytes);expect(first.filesystemBytes).toBeLessThanOrEqual(filesystemBytes);expect(first.filesystemBytes).toBeGreaterThan(0);
   expect(nativeImageAttachedV1(first.image,first.root)).toBe(true);
   const capacity=nativeImageFilesystemBytesV1(first.root,filesystemBytes);expect(capacity).toBeGreaterThan(0);expect(capacity).toBeLessThanOrEqual(filesystemBytes);
   const space=nativeImageSpaceV1(first.root,filesystemBytes);expect(Number.isInteger(space.bsize)&&space.bsize>0).toBe(true);expect(space.bavail).toBeGreaterThanOrEqual(0);expect(space.bavail*space.bsize).toBeLessThanOrEqual(filesystemBytes);
   const other=nativeStorageBackendsV1.find(row=>row.id!==expected&&row.platforms.includes(nativeStorageSelectionV1().platform!));
   if(other)await expect(withNativeStorageSelectionV1({backend:other.id},()=>openNativePlatformStorageV1(options("conflict")))).rejects.toThrow("belongs to another backend");
   const policy={version:1 as const,root:first.root,leaseDirectory,capacityBytes:maximumPhysicalBytes,maximumWorkspaceBytes:filesystemBytes,maximumArtifactBytes:filesystemBytes};
   const reservation=await acquireNativeStorageV1({policy,control:session,port:first.port,owner:"storage-conformance",kind:"compiler-workspace",bytes:payloadBytes}) as {commit():Promise<void>};
   writeFileSync(join(first.root,"payload.bin"),Buffer.alloc(payloadBytes,7));await reservation.commit();
   const statfs=await nativeStatfsV1(first.root,{fresh:true}),used=(statfs.blocks-statfs.bfree)*statfs.bsize;
   expect(used).toBeGreaterThanOrEqual(payloadBytes);expect(statfs.blocks*statfs.bsize).toBeLessThanOrEqual(filesystemBytes);
   const walked=walkedBytes(first.root);expect(walked).toBeGreaterThanOrEqual(payloadBytes);expect(used).toBeGreaterThanOrEqual(walked);expect(used-walked).toBeLessThanOrEqual(8*1048576);
   expect(readFileSync(join(first.root,"storage-events.jsonl"),"utf8").split("\n").filter(Boolean).length).toBeGreaterThan(0);
   await first.close();
   const second=await openNativePlatformStorageV1(options("reopen"));expect(second.root).toBe(root);expect(nativeImageAttachedV1(second.image,second.root)).toBe(true);
   expect(readFileSync(join(second.root,"payload.bin")).byteLength).toBe(payloadBytes);
   await second.close();
   const spare=join(scratch,"q"),spareOptions=(name:string)=>({...options(name),directory:spare});
   const third=await openNativePlatformStorageV1(spareOptions("spare-open"));expect(existsSync(join(spare,declaration.backingFilename))).toBe(true);await third.close();
   expect(await retireNativePlatformStorageV1(spareOptions("spare-retire"))).toBe(true);
   expect(existsSync(join(spare,declaration.backingFilename))).toBe(false);
  });
 }finally{
  if(declaration.capabilities.includes("mounted-volume"))tool(["detach","-force",root]);
  if(hostNeedsCaseSensitiveVolume)tool(["detach","-force",volume]);
  rmSync(sandbox,{recursive:true,force:true});
 }
}

test.skipIf(!child)("child: the backend selected by the environment satisfies the storage contract",()=>conform(process.env[names.backend]!),240000);

for(const backend of nativeStorageBackendsV1)
 test.skipIf(child||!backend.platforms.includes(process.platform))("backend "+backend.id+" satisfies the storage contract through the neutral facade",()=>{
  const run=spawnSync(process.execPath,["test","--timeout","200000",import.meta.path,"-t","child:"],{env:{...process.env,[names.child]:"1",[names.backend]:backend.id},encoding:"utf8"});
  expect(run.status,run.stdout+run.stderr).toBe(0);expect(run.stdout+run.stderr).toContain("1 pass");
 },280000);
