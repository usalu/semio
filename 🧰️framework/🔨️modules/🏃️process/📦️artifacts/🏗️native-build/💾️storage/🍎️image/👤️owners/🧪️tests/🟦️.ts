import{test,expect}from"bun:test";
import{testScratchRootV1}from"../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{spawnSync}from"node:child_process";
import{lstatSync,mkdirSync,mkdtempSync,realpathSync,rmSync}from"node:fs";
import{join}from"node:path";
import fixture from"./🔣️.json"with{type:"json"};
import imageSchema from"../../../🧬️schema/🔣️.json"with{type:"json"};
import{nativeImageInformationV1}from"../../📥️information/🟦️.ts";
import{openNativeImageStorageV1}from"../../🟦️.ts";
import{createIssuedResourceControlV1,createScriptProcessEnvelope,withScriptProcessEnvelope}from"../../../../../../🧭️routing/📥️invocation/🏃️process/🟦️.ts";
import{nativeImageAttachArgumentsV1,nativeImageAttachProgramV1,nativeImageHostMountOfV1,nativeImageHostMountV1,requireNativeImageHostOwnersV1}from"../🟦️.ts";
const run=(program:string,args:readonly string[])=>{const result=spawnSync(program,[...args],{encoding:"utf8",env:{PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"}});return{status:result.status,stdout:result.stdout,stderr:result.stderr};};
const mountTable=()=>run("/sbin/mount",[]).stdout;
const ownersOracle=(mountPoint:string):boolean=>nativeImageInformationV1(run("/usr/sbin/diskutil",["info","-plist",mountPoint]).stdout).GlobalPermissionsEnabled===true;
test("attach arguments are schema-declared and keep owners on",()=>{
 expect(nativeImageAttachArgumentsV1(fixture.attach.root,fixture.attach.image)).toEqual(fixture.attach.expected);
 expect(imageSchema.definitions.ImageProtocol.const.attachArguments.join(" ")).toContain("--mountOptions owners");
});
test("hosting mount resolution and refusal over the language-agnostic corpus",()=>{
 const table=fixture.mountTable.join("\n");
 for(const row of fixture.hosts){const host=nativeImageHostMountOfV1(table,row.directory);expect(host.mountPoint).toBe(row.mountPoint);expect(host.owners).toBe(row.owners);
  if(row.owners)expect(()=>requireNativeImageHostOwnersV1(table,"/")).not.toThrow();}
 expect(()=>nativeImageHostMountOfV1("","/x")).toThrow("Hosting mount");
});
test("external hosting ownership refusal names the physical mount without requesting detach",()=>{
 const root=realpathSync(".");
 const facts=fixture.hostingOwnership;
 const table="/dev/disk23s1 on "+root+" (apfs, local, "+facts.disabledFlag+")";
 expect(()=>requireNativeImageHostOwnersV1(table,root,root)).toThrow(facts.modeledInstruction);
 expect(()=>requireNativeImageHostOwnersV1(table,root)).toThrow(facts.externalInstruction);
 expect(()=>requireNativeImageHostOwnersV1(table,root,join(root,"other"))).toThrow(facts.externalInstruction);
 expect(()=>requireNativeImageHostOwnersV1(table,root)).toThrow(root);
 expect(()=>requireNativeImageHostOwnersV1(table,root)).not.toThrow(facts.modeledInstruction);
});
test.skipIf(process.platform!=="darwin")("external noowners mount refuses actual top-level activation before producing child association",async()=>{
 const scratch=realpathSync(mkdtempSync(join(testScratchRootV1("image-tests"),"external-")));
 const mount=join(scratch,"p"),image=join(scratch,"parent.sparseimage"),leaseDirectory=join(scratch,"l"),directory=join(mount,"b");
 mkdirSync(mount);mkdirSync(leaseDirectory);
 const bare=lstatSync(mount).dev;
 const facts=fixture.hostingOwnership;
 try{
  expect(run("/usr/bin/hdiutil",["create","-size",facts.outerMiB+"m","-type","SPARSE","-nospotlight","-fs","Case-sensitive APFS","-volname","semio",image]).status).toBe(0);
  expect(run("/usr/bin/hdiutil",["attach","-nobrowse","-owners","off","-mountpoint",mount,image]).status).toBe(0);
  expect(lstatSync(mount).dev).not.toBe(bare);
  expect(ownersOracle(mount)).toBe(false);
  const attempt=withScriptProcessEnvelope(createScriptProcessEnvelope({version:1,owner:"external-native-hosting-fixture",maximumElapsedMilliseconds:25000},{workspaceRoot:scratch},Date.now()),original=>{
   const control=createIssuedResourceControlV1(original,"native:external-hosting-fixture",20000);
   return openNativeImageStorageV1({directory,leaseDirectory,filesystemBytes:facts.filesystemBytes,maximumPhysicalBytes:facts.maximumPhysicalBytes,control});
  });
  await expect(attempt).rejects.toThrow(facts.externalInstruction);
  expect(run("/usr/bin/hdiutil",["info","-plist"]).stdout).not.toContain(join(directory,"fixed.dmg"));
 }finally{
  if(lstatSync(mount).dev!==bare)expect(run("/usr/bin/hdiutil",["detach",mount]).status).toBe(0);
  expect(run("/usr/bin/hdiutil",["info","-plist"]).stdout).not.toContain(scratch);
  rmSync(scratch,{recursive:true});
 }
},45000);
test.skipIf(process.platform!=="darwin")("a nested image attaches under an owners-on parent and the flag agrees with the system oracle",()=>{
 const scratch=realpathSync(mkdtempSync(join(testScratchRootV1("image-tests"),"owners-"))),parentMount=join(scratch,"p"),defaultMount=join(scratch,"d"),childMount=join(parentMount,"nx","cache","m"),attached:string[]=[];
 const create=(image:string,size="24m")=>expect(run("/usr/bin/hdiutil",["create","-size",size,"-type",imageSchema.definitions.ImageProtocol.const.format,"-nospotlight","-fs",imageSchema.definitions.ImageProtocol.const.filesystem,"-volname","semio",image]).status).toBe(0);
 try{
  mkdirSync(parentMount);mkdirSync(defaultMount);
  create(join(scratch,"parent.dmg"),"96m");create(join(scratch,"default.dmg"));
  expect(run(nativeImageAttachProgramV1,nativeImageAttachArgumentsV1(parentMount,join(scratch,"parent.dmg"))).status).toBe(0);attached.push(parentMount);
  expect(run("/usr/bin/hdiutil",["attach","-nobrowse","-noautoopen","-mountpoint",defaultMount,join(scratch,"default.dmg")]).status).toBe(0);attached.push(defaultMount);
  const table=mountTable(),owned=nativeImageHostMountV1(table,parentMount),plain=nativeImageHostMountV1(table,defaultMount);
  expect(owned.mountPoint).toBe(parentMount);expect(owned.owners).toBe(true);expect(owned.owners).toBe(ownersOracle(parentMount));
  expect(plain.mountPoint).toBe(defaultMount);expect(plain.owners).toBe(false);expect(plain.owners).toBe(ownersOracle(defaultMount));
  expect(()=>requireNativeImageHostOwnersV1(table,defaultMount,defaultMount)).toThrow("hdiutil detach "+defaultMount);
  expect(()=>requireNativeImageHostOwnersV1(table,parentMount)).not.toThrow();
  const childDirectory=join(parentMount,"nx","cache");mkdirSync(childMount,{recursive:true});create(join(childDirectory,"fixed.dmg"));
  expect(nativeImageHostMountV1(mountTable(),childDirectory).owners).toBe(true);
  const attach=run(nativeImageAttachProgramV1,nativeImageAttachArgumentsV1(childMount,join(childDirectory,"fixed.dmg")));
  expect(attach.status).toBe(0);attached.push(childMount);
  expect(mountTable()).toContain(" on "+childMount+" (");
 }finally{
  for(const mountPoint of attached.reverse())run("/usr/bin/hdiutil",["detach","-force",mountPoint]);
  expect(mountTable()).not.toContain(scratch);
  rmSync(scratch,{recursive:true,force:true});
 }
},120000);
