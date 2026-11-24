/** 🔗️ Tests for the attach that recovers from the sibling volume of a killed in-place kick: the fault is reproduced on a real image and the result is read back with the operating system's own tools. */
import{afterAll,beforeAll,expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,realpathSync,rmSync,statSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{nativeImageInformationV1}from"../../📥️information/🟦️.ts";
import{attachNativeImageV1,removeNativeImageSiblingVolumesV1}from"../🟦️.ts";
const tool=(program:string,args:string[])=>spawnSync(program,args,{encoding:"utf8"});
const plist=(image:string,entities:string)=>`<?xml version="1.0" encoding="UTF-8"?><plist version="1.0"><dict><key>images</key><array><dict><key>image-path</key><string>${image}</string><key>system-entities</key><array>${entities}</array></dict></array></dict></plist>`;
const entity=(device:string,mount?:string)=>`<dict><key>dev-entry</key><string>${device}</string>${mount===undefined?"":`<key>mount-point</key><string>${mount}</string>`}</dict>`;
let root="";
beforeAll(()=>{root=realpathSync(mkdtempSync(join(tmpdir(),"semio-attach-")));});
afterAll(()=>{rmSync(root,{recursive:true,force:true});});

test("a failed attach of an image that is already mounted is never touched and keeps its own error",async()=>{
 const calls:string[]=[],image="/i/fixed.dmg",run=async(program:string,args:readonly string[])=>{
  calls.push(program+" "+args.join(" "));
  if(program==="/usr/sbin/diskutil"&&args[0]==="image")throw Error("Failed to mount the volume");
  if(program==="/usr/bin/hdiutil")return plist(image,entity("/dev/disk4")+entity("/dev/disk4s1","/Volumes/other"));
  throw Error("unexpected "+program);
 };
 await expect(attachNativeImageV1(run,"/i/m",image)).rejects.toThrow("Failed to mount the volume");
 expect(calls.filter(row=>/apfs|detach/u.test(row))).toEqual([]);
 expect(await removeNativeImageSiblingVolumesV1(run,image)).toBe(false);
});

test("a failed attach without a sibling volume detaches what the recovery attached and keeps its own error",async()=>{
 const calls:string[]=[],image="/i/fixed.dmg";let attached=false;
 const run=async(program:string,args:readonly string[])=>{
  calls.push(program+" "+args.join(" "));
  if(program==="/usr/sbin/diskutil"&&args[0]==="image"){if(args.includes("--noMount")){attached=true;return"";}throw Error("Failed to mount the volume");}
  if(program==="/usr/bin/hdiutil"&&args[0]==="info")return attached?plist(image,entity("/dev/disk4")+entity("/dev/disk5")):`<?xml version="1.0"?><plist version="1.0"><dict><key>images</key><array/></dict></plist>`;
  if(program==="/usr/bin/hdiutil"&&args[0]==="detach"){attached=false;return"";}
  if(program==="/usr/sbin/diskutil"&&args[0]==="apfs")return args[2]==="disk5"?`<?xml version="1.0"?><plist version="1.0"><dict><key>Containers</key><array><dict><key>Volumes</key><array><dict><key>Name</key><string>semio</string><key>DeviceIdentifier</key><string>disk5s1</string></dict></array></dict></array></dict></plist>`:(()=>{throw Error("not a container");})();
  throw Error("unexpected "+program);
 };
 await expect(attachNativeImageV1(run,"/i/m",image)).rejects.toThrow("Failed to mount the volume");
 expect(calls.some(row=>row.includes("deleteVolume"))).toBe(false);expect(calls.at(-1)).toBe("/usr/bin/hdiutil detach /dev/disk4");expect(attached).toBe(false);
});

test.skipIf(process.platform!=="darwin")("an image with a leftover semio-trim volume attaches at its mount point after the recovery and ends with one volume",async()=>{
 const image=join(root,"fixed.dmg"),mount=join(root,"m"),run=async(program:string,args:readonly string[])=>{const done=tool(program,[...args]);if(done.status!==0)throw Error(`${program} ${args.join(" ")}: ${done.stdout}${done.stderr}`);return done.stdout;};
 try{
  expect(tool("/usr/bin/hdiutil",["create","-size","600m","-type","UDIF","-layout","GPTSPUD","-fs","Case-sensitive APFS","-volname","semio",image]).status).toBe(0);
  const attached=tool("/usr/sbin/diskutil",["image","attach","--noMount","--nobrowse",image]).stdout,container=/(\/dev\/disk\d+)\s+Apple_APFS_Container/u.exec(attached)?.[1],whole=/(\/dev\/disk\d+)\s+GUID_partition_scheme/u.exec(attached)?.[1];
  expect(container).toBeDefined();
  expect(tool("/usr/sbin/diskutil",["apfs","addVolume",container!.slice(5),"Case-sensitive APFS","semio-trim","-nomount"]).status).toBe(0);
  expect(tool("/usr/bin/hdiutil",["detach",whole!]).status).toBe(0);
  mkdirSync(mount);
  expect(tool("/usr/sbin/diskutil",["image","attach","--nobrowse","--mountOptions","owners","--mountPoint",mount,image]).status).not.toBe(0);
  await attachNativeImageV1(run,mount,image);
  expect(statSync(mount).dev).not.toBe(statSync(root).dev);
  const info=nativeImageInformationV1(tool("/usr/bin/hdiutil",["info","-plist"]).stdout),row=(info.images as Record<string,unknown>[]).find(entry=>entry["image-path"]===image)!,device=(row["system-entities"] as Record<string,unknown>[]).map(entry=>entry["dev-entry"] as string).find(value=>/^\/dev\/disk\d+$/u.test(value)&&value!==whole)!;
  const names=((nativeImageInformationV1(tool("/usr/sbin/diskutil",["apfs","list","-plist",...(row["system-entities"] as Record<string,unknown>[]).map(entry=>entry["dev-entry"] as string).filter(value=>/^\/dev\/disk\d+$/u.test(value)).slice(1,2).map(value=>value.slice(5))]).stdout).Containers as Record<string,unknown>[])??[]).flatMap(entry=>(entry.Volumes as Record<string,unknown>[]).map(volume=>volume.Name));
  expect(device).toBeDefined();expect(names).toEqual(["semio"]);
 }finally{
  const info=tool("/usr/bin/hdiutil",["info","-plist"]).stdout,row=(nativeImageInformationV1(info).images as Record<string,unknown>[]).find(entry=>entry["image-path"]===image);
  const device=(row?.["system-entities"] as Record<string,unknown>[]|undefined)?.map(entry=>entry["dev-entry"] as string).find(value=>/^\/dev\/disk\d+$/u.test(value));
  if(device!==undefined)tool("/usr/bin/hdiutil",["detach","-force",device]);
 }
},240000);
