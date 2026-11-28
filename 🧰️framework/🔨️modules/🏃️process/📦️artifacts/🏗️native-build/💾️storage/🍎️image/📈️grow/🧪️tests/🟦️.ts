/** 📈️ Tests for growing an existing fixed image in place: the pure laws agree with an independent Python oracle, the cycle restores the volume after every refusal, failure and cancellation, and a real image on this machine grows and keeps its mount point (read back with the operating system's own tools). */
import{afterAll,beforeAll,expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{lstatSync,mkdirSync,mkdtempSync,realpathSync,rmSync,statSync,statfsSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import vectors from"./🔣️.json" with {type:"json"};
import{NATIVE_IMAGE_GROW_V1,createNativeImageDarwinGrowPortV1,growNativeImageV1,nativeImageGrowAdmissionV1,nativeImageGrowArgumentsV1,nativeImageGrowTextV1,nativeImageGrownReceiptV1,type NativeImageGrowPortV1}from"../🟦️.ts";
const tool=(program:string,args:string[])=>spawnSync(program,args,{encoding:"utf8"});
const python=Bun.spawnSync(["python3","-I",join(import.meta.dirname,"🐍️.py")],{stdin:new TextEncoder().encode(JSON.stringify(vectors))});
let root="";
beforeAll(()=>{root=realpathSync(mkdtempSync(join(tmpdir(),"semio-grow-")));});
afterAll(()=>{rmSync(root,{recursive:true,force:true});});

test("the admission and the receipt agree with the independent oracle",()=>{
 expect(python.exitCode,python.stderr.toString()).toBe(0);
 const oracle=JSON.parse(python.stdout.toString()) as {admission:string[];receipt:{size:number;maximumPhysicalBytes:number}[]};
 for(const[index,row]of vectors.admission.entries()){
  const actual=nativeImageGrowAdmissionV1({currentBytes:row.currentBytes,targetBytes:row.targetBytes,hostFreeBytes:row.hostFreeBytes??undefined});
  expect<unknown>(actual,row.name).toBe(row.expected);expect(oracle.admission[index],row.name).toBe(row.expected);
 }
 for(const[index,row]of vectors.receipt.entries()){
  const grown=nativeImageGrownReceiptV1({version:1,ino:7,filesystemBytes:1,size:1,maximumPhysicalBytes:1},row.filesystemBytes);
  expect(grown).toEqual({version:1,ino:7,filesystemBytes:row.filesystemBytes,size:row.size,maximumPhysicalBytes:row.maximumPhysicalBytes});expect(oracle.receipt[index]).toEqual({size:row.size,maximumPhysicalBytes:row.maximumPhysicalBytes});
 }
 expect(()=>nativeImageGrowAdmissionV1({currentBytes:1,targetBytes:123,hostFreeBytes:undefined})).toThrow();
 expect(nativeImageGrowArgumentsV1(200*1048576,"/i/fixed.dmg")).toEqual(["resize","-size","200m","/i/fixed.dmg"]);
 expect(()=>nativeImageGrowArgumentsV1(1,"/i")).toThrow();
 for(const language of["en","de"]as const)for(const key of Object.keys(NATIVE_IMAGE_GROW_V1.messages)as(keyof typeof NATIVE_IMAGE_GROW_V1.messages)[])expect(nativeImageGrowTextV1(language,key,{image:"d",from:1,to:2,free:3,needed:4})).toContain("d");
});

const fake=(options:Readonly<{attached:boolean;busy?:boolean;resize?:()=>Promise<void>;canonical?:boolean}>)=>{
 const calls:string[]=[];let attached=options.attached;
 const port:NativeImageGrowPortV1={
  async attachedDevice(){return attached?"/dev/disk9":null;},
  async detach(){calls.push("detach");if(options.busy===true)throw Error("Resource busy");attached=false;},
  async attach(){calls.push("attach");attached=true;},
  async resize(bytes){calls.push("resize "+bytes);await options.resize?.();},
  allocatedBytes:()=>0,dispose(){calls.push("dispose");},mountsCanonically:options.canonical??true,
 };
 return{port,calls,attached:()=>attached};
};
const hooks=(extra:Partial<{check:()=>void;progress:(done:number,total:number)=>void}>={})=>({check:()=>undefined,backingBytes:()=>1,...extra});

test("an attached image is detached, stretched and attached again, with progress",async()=>{
 const{port,calls,attached}=fake({attached:true}),steps:number[]=[];
 await growNativeImageV1(port,hooks({progress:done=>steps.push(done)}),200*1048576);
 expect(calls).toEqual(["detach","resize "+200*1048576,"attach","dispose"]);expect(attached()).toBe(true);expect(steps).toEqual([0,1,2,3,4]);
});

test("a busy image refuses at the detach and nothing else happens",async()=>{
 const{port,calls}=fake({attached:true,busy:true});
 await expect(growNativeImageV1(port,hooks(),200*1048576)).rejects.toThrow("busy");
 expect(calls).toEqual(["detach"]);
});

test("a failed stretch attaches the image again and keeps its own error",async()=>{
 const{port,calls,attached}=fake({attached:true,resize:async()=>{throw Error("hdiutil: resize: failed. Invalid argument (22)");}});
 await expect(growNativeImageV1(port,hooks(),200*1048576)).rejects.toThrow("Invalid argument");
 expect(calls).toEqual(["detach","resize "+200*1048576,"attach","dispose"]);expect(attached()).toBe(true);
});

test("a cancellation after the detach attaches the image again before it stops",async()=>{
 const{port,calls,attached}=fake({attached:true});let checks=0;
 await expect(growNativeImageV1(port,hooks({check:()=>{if(++checks===2)throw Error("cancelled");}}),200*1048576)).rejects.toThrow("cancelled");
 expect(calls).toEqual(["detach","attach","dispose"]);expect(attached()).toBe(true);
});

test("a detached image stays detached and a scratch port never mounts canonically",async()=>{
 const idle=fake({attached:false});
 await growNativeImageV1(idle.port,hooks(),200*1048576);expect(idle.calls).toEqual(["resize "+200*1048576,"dispose"]);expect(idle.attached()).toBe(false);
 const scratch=fake({attached:true,canonical:false});
 await growNativeImageV1(scratch.port,hooks(),200*1048576);expect(scratch.calls).toEqual(["detach","resize "+200*1048576,"dispose"]);expect(scratch.attached()).toBe(false);
});

test.skipIf(process.platform!=="darwin")("a real attached image grows in place, keeps its mount point and shows the larger volume",async()=>{
 const image=join(root,"fixed.dmg"),mount=join(root,"m"),seed=join(root,"seed"),run=async(program:string,args:readonly string[])=>{const done=tool(program,[...args]);if(done.status!==0)throw Error(`${program} ${args.join(" ")}: ${done.stdout}${done.stderr}`);return done.stdout;};
 const MiB=1048576;
 try{
  mkdirSync(seed);mkdirSync(mount);
  expect(tool("/usr/bin/hdiutil",["create","-size","64m","-format","UDRW","-nospotlight","-fs","Case-sensitive APFS","-volname","semio","-srcfolder",seed,image]).status).toBe(0);
  expect(tool("/usr/sbin/diskutil",["image","attach","--nobrowse","--mountOptions","owners","--mountPoint",mount,image]).status).toBe(0);
  const before=statfsSync(mount),volumeBefore=Number(before.blocks)*before.bsize;
  expect(volumeBefore).toBeLessThan(80*MiB);
  const port=createNativeImageDarwinGrowPortV1({image,cwd:root,mountPoint:mount,run}),progress:number[]=[];
  const outcome=await growNativeImageV1(port,{check:()=>undefined,progress:done=>progress.push(done),backingBytes:()=>lstatSync(image).size},200*MiB);
  expect(outcome.afterBytes).toBeGreaterThan(outcome.beforeBytes);expect(lstatSync(image).size).toBe(200*MiB+NATIVE_IMAGE_GROW_V1.trailerBytes);expect(progress.at(-1)).toBe(NATIVE_IMAGE_GROW_V1.progressSteps);
  expect(statSync(mount).dev).not.toBe(statSync(root).dev);
  const after=statfsSync(mount);expect(Number(after.blocks)*after.bsize).toBeGreaterThan(190*MiB);
  const independent=tool("/bin/df",["-k",mount]).stdout.trim().split("\n").at(-1)!.split(/\s+/u);expect(Number(independent[1])*1024).toBeGreaterThan(190*MiB);
 }finally{
  tool("/usr/bin/hdiutil",["detach","-force",mount]);
 }
});
