import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import fixture from"./🔣️.json"with{type:"json"};
import imageSchema from"../../../🧬️schema/🔣️.json"with{type:"json"};
import{nativeImageInformationV1}from"../../📥️information/🟦️.ts";
import{nativeImageAttachArgumentsV1,nativeImageHostMountOfV1,nativeImageHostMountV1,requireNativeImageHostOwnersV1}from"../🟦️.ts";
const run=(program:string,args:readonly string[])=>{const result=spawnSync(program,[...args],{encoding:"utf8",env:{PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"}});return{status:result.status,stdout:result.stdout,stderr:result.stderr};};
const mountTable=()=>run("/sbin/mount",[]).stdout;
const ownersOracle=(mountPoint:string):boolean=>nativeImageInformationV1(run("/usr/sbin/diskutil",["info","-plist",mountPoint]).stdout).GlobalPermissionsEnabled===true;
test("attach arguments are schema-declared and keep owners on",()=>{
 expect(nativeImageAttachArgumentsV1(fixture.attach.root,fixture.attach.image)).toEqual(fixture.attach.expected);
 expect(imageSchema.definitions.ImageProtocol.const.attachArguments.join(" ")).toContain("-owners on");
});
test("hosting mount resolution and refusal over the language-agnostic corpus",()=>{
 const table=fixture.mountTable.join("\n");
 for(const row of fixture.hosts){const host=nativeImageHostMountOfV1(table,row.directory);expect(host.mountPoint).toBe(row.mountPoint);expect(host.owners).toBe(row.owners);
  if(row.owners)expect(()=>requireNativeImageHostOwnersV1(table,"/")).not.toThrow();}
 expect(()=>nativeImageHostMountOfV1("","/x")).toThrow("Hosting mount");
});
test.skipIf(process.platform!=="darwin")("a nested image attaches under an owners-on parent and the flag agrees with the system oracle",()=>{
 const scratch=realpathSync(mkdtempSync(join(process.env.TMPDIR||tmpdir(),"owners-"))),parentMount=join(scratch,"p"),defaultMount=join(scratch,"d"),childMount=join(parentMount,"nx","cache","m"),attached:string[]=[];
 const create=(image:string,size="24m")=>expect(run("/usr/bin/hdiutil",["create","-size",size,"-type",imageSchema.definitions.ImageProtocol.const.format,"-nospotlight","-fs",imageSchema.definitions.ImageProtocol.const.filesystem,"-volname","semio",image]).status).toBe(0);
 try{
  mkdirSync(parentMount);mkdirSync(defaultMount);
  create(join(scratch,"parent.dmg"),"96m");create(join(scratch,"default.dmg"));
  expect(run("/usr/bin/hdiutil",nativeImageAttachArgumentsV1(parentMount,join(scratch,"parent.dmg"))).status).toBe(0);attached.push(parentMount);
  expect(run("/usr/bin/hdiutil",["attach","-nobrowse","-noautoopen","-mountpoint",defaultMount,join(scratch,"default.dmg")]).status).toBe(0);attached.push(defaultMount);
  const table=mountTable(),owned=nativeImageHostMountV1(table,parentMount),plain=nativeImageHostMountV1(table,defaultMount);
  expect(owned.mountPoint).toBe(parentMount);expect(owned.owners).toBe(true);expect(owned.owners).toBe(ownersOracle(parentMount));
  expect(plain.mountPoint).toBe(defaultMount);expect(plain.owners).toBe(false);expect(plain.owners).toBe(ownersOracle(defaultMount));
  expect(()=>requireNativeImageHostOwnersV1(table,defaultMount)).toThrow("hdiutil detach "+defaultMount);
  expect(()=>requireNativeImageHostOwnersV1(table,parentMount)).not.toThrow();
  const childDirectory=join(parentMount,"nx","cache");mkdirSync(childMount,{recursive:true});create(join(childDirectory,"fixed.dmg"));
  expect(nativeImageHostMountV1(mountTable(),childDirectory).owners).toBe(true);
  const attach=run("/usr/bin/hdiutil",nativeImageAttachArgumentsV1(childMount,join(childDirectory,"fixed.dmg")));
  expect(attach.status).toBe(0);attached.push(childMount);
  expect(mountTable()).toContain(" on "+childMount+" (");
 }finally{
  for(const mountPoint of attached.reverse())run("/usr/bin/hdiutil",["detach","-force",mountPoint]);
  expect(mountTable()).not.toContain(scratch);
  rmSync(scratch,{recursive:true,force:true});
 }
},120000);
