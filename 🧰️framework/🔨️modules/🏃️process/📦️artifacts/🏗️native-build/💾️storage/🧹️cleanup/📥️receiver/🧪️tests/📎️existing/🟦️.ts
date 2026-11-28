import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {copyFileSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,renameSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {declareBuildTestEnvelopeV1,registerIsolatedBuildTestV1,withBuildTestInvocationV1} from "../../../../../../../🧪️testing/📥️invocation/🟦️.ts";
import {acquireResourceLease,type ResourceLease} from "../../../../../../../🔒️leases/🟦️.ts";
import {createIssuedResourceControlV1} from "../../../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import {acquireNativeStorageOwnerV1,recoverNativeStorageReservationV1} from "../../../../🟦️.ts";
import {captureNativePlatformRecomputableRetirementV1,openExistingNativePlatformImageV1,openNativePlatformStorageV1,retireNativePlatformRecomputableStorageV1} from "../../../../🌐️platform/🟦️.ts";
import fixture from "./🔣️.json" with {type:"json"};

declareBuildTestEnvelopeV1({owner:"test:existing-native-maintenance",maximumElapsedMilliseconds:180000,capabilities:{artifacts:true}});

test.skipIf(process.platform!=="darwin")("existing Native maintenance keeps the exact remounted image through authorized final retirement",()=>withBuildTestInvocationV1(async original=>{
 const scratch=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"er-"))),directory=join(scratch,"i"),leaseDirectory=join(scratch,"l"),root=join(directory,"m"),file=join(directory,"fixed.dmg");
 mkdirSync(leaseDirectory);
 const control=createIssuedResourceControlV1(original,"cache:existing-retirement",120000);
 const options={directory,leaseDirectory,filesystemBytes:fixture.filesystemBytes,maximumPhysicalBytes:fixture.maximumPhysicalBytes,control};
 const policy={version:1 as const,root,leaseDirectory,capacityBytes:fixture.maximumPhysicalBytes,maximumWorkspaceBytes:fixture.filesystemBytes,maximumArtifactBytes:fixture.filesystemBytes};
 const detach=()=>spawnSync("repo",["tools","exec","--light","hdiutil","--","detach",root],{encoding:"utf8"});
 try{
  const born=await openNativePlatformStorageV1(options);
  let allocationId:string;
  try{
   const charge=await acquireNativeStorageOwnerV1({policy,control,port:born.port,owner:fixture.owner,kind:"artifact-publication",bytes:fixture.bytes});
   allocationId=charge.allocation.id;
   writeFileSync(join(root,"owned"),fixture.witness);
  }finally{await born.close();}
  const oracle=()=>spawnSync("repo",["tools","exec","--light","python","--","-c","import json,os,sys; s=os.stat(sys.argv[1]); print(json.dumps([s.st_dev,s.st_ino,s.st_size]))",file],{encoding:"utf8"});
  const before=oracle(),receipt=readFileSync(join(directory,"image.json"));
  expect(before.status).toBe(0);
  const detached=detach();expect(detached.status,detached.stderr).toBe(0);
  expect(lstatSync(root).dev).toBe(lstatSync(file).dev);
  const reopened=await openExistingNativePlatformImageV1(options,"maintenance");
  let retirement:Awaited<ReturnType<typeof captureNativePlatformRecomputableRetirementV1>>;
  try{
   expect(lstatSync(root).dev).not.toBe(lstatSync(file).dev);
   expect(oracle().stdout).toBe(before.stdout);
   expect(readFileSync(join(directory,"image.json"))).toEqual(receipt);
   expect(readFileSync(join(root,"owned"),"utf8")).toBe(fixture.witness);
   const journal=readFileSync(join(root,"storage-events.jsonl"));
   const exact=await reopened.withExclusiveMaintenance({version:1,wait:false},async(port,custody)=>{
    if(custody.kind!=="kernel-ex")throw Error("Actual kernel EX custody required");
    const charge=await recoverNativeStorageReservationV1({policy,control:custody.control,port,id:allocationId});
    expect(charge.allocation.id).toBe(allocationId);
    return captureNativePlatformRecomputableRetirementV1(reopened,custody,charge,custody.control);
   });
   if(!exact.acquired)throw Error("Fixture owns the exact maintenance writer");
   expect(readFileSync(join(root,"storage-events.jsonl"))).toEqual(journal);
   retirement=exact.value;
  }finally{await reopened.close();}
  expect(await retireNativePlatformRecomputableStorageV1(retirement,control,undefined,false)).toBe(true);
  expect(existsSync(directory)).toBe(false);
 }finally{
  if(existsSync(root)&&existsSync(file)&&lstatSync(root).dev!==lstatSync(file).dev)expect(detach().status).toBe(0);
  rmSync(scratch,{recursive:true,force:true});
 }
}),150000);

if(process.platform==="darwin")for(const sample of fixture.negative)registerIsolatedBuildTestV1(import.meta.path,"existing Native maintenance refuses "+sample.kind,{bodyMilliseconds:60000,scope:{kind:"body"}},checkpoint=>withBuildTestInvocationV1(async original=>{
 const artifact=process.env.SEMIO_TEST_ARTIFACT_DIR!,scratch=realpathSync(mkdtempSync(join(artifact,"ef-"))),directory=join(scratch,"i"),leaseDirectory=join(scratch,"l"),root=join(directory,"m"),file=join(directory,"fixed.dmg");
 mkdirSync(leaseDirectory);
 let holder:ResourceLease|undefined;
 try{
  const control=createIssuedResourceControlV1(original,"cache:existing-refusal",40000);
  const options={directory,leaseDirectory,filesystemBytes:fixture.filesystemBytes,maximumPhysicalBytes:fixture.maximumPhysicalBytes,control};
  let inode:number|undefined;
  if(sample.kind==="busy")holder=await acquireResourceLease({directory:leaseDirectory,resource:"native-fixed-image:"+directory,mode:"exclusive",control,signal:control.signal,timeoutMs:0});
  if(sample.kind==="substituted"){
   const born=await openNativePlatformStorageV1(options);await born.close();
   expect(spawnSync("repo",["tools","exec","--light","hdiutil","--","detach",root],{encoding:"utf8"}).status).toBe(0);
   renameSync(file,file+".original");copyFileSync(file+".original",file);inode=lstatSync(file).ino;
   expect(inode).not.toBe(lstatSync(file+".original").ino);
  }
  try{
   const opened=await openExistingNativePlatformImageV1(options,"maintenance");await opened.close();
  }catch(error){
   if(sample.kind==="substituted"){expect(lstatSync(file).ino).toBe(inode!);expect(lstatSync(file).dev).toBe(lstatSync(root).dev);}
   else expect(existsSync(directory)).toBe(false);
   checkpoint();throw error;
  }
  throw Error("Existing Native maintenance accepted "+sample.kind);
 }finally{
  holder?.release();
  if(existsSync(root)&&existsSync(file)&&lstatSync(root).dev!==lstatSync(file).dev)expect(spawnSync("repo",["tools","exec","--light","hdiutil","--","detach",root],{encoding:"utf8"}).status).toBe(0);
  rmSync(scratch,{recursive:true,force:true});
 }
}),new RegExp(sample.failure,"u"));
