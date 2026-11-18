/** ♻️ Tests for image reclaim. */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{lstatSync,mkdirSync,mkdtempSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{basename,join}from"node:path";
import{testScratchRootV1}from"../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import imageSchema from"../../../🧬️schema/🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
import{createNativeImageVolumeSilenceSeedV1,nativeImageSeededCreateArgumentsV1}from"../../🤫️silence/🟦️.ts";
import{nativeImageSparseGrowArgumentsV1}from"../../🪶️sparse/🟦️.ts";
import{nativeImageAttachArgumentsV1,nativeImageAttachProgramV1}from"../../👤️owners/🟦️.ts";
import{nativeImageContainerFreeBytesSyncV1,nativeImageContainerFreeBytesV1,nativeImageEffectiveAvailableBytesV1}from"../../⚖️availability/🟦️.ts";
import{nativeImageHdiutilCommandV1,nativeImageHdiutilProgramV1}from"../../🛠️hdiutil/🟦️.ts";
import{createNativeImageLinuxReclaimPortV1,createNativeImageReclaimPortV1,createNativeImageWindowsReclaimPortV1,createNativeImageDarwinReclaimPortV1,nativeImageReclaimDueV1,nativeImageReclaimMechanismV1,reclaimNativeImageV1,type NativeImageReclaimPortV1}from"../🟦️.ts";
const protocol=imageSchema.definitions.ImageProtocol.const,MiB=1048576;
const env={PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"};
const run=(program:string,args:readonly string[])=>{const row=spawnSync(program,[...args],{encoding:"utf8",env});if(row.status!==0)throw Error(program+" "+args.join(" ")+": "+row.stderr);return row.stdout;};
const scripted=(cycle:typeof fixture.cycles[number],log:string[])=>{
 const seen=new Map<string,number>(),scratch=()=>{const row=log.find(line=>/fstrim|mount -o|Mount-VHD/u.test(line));return row?.match(/(\/[^ ]*semio-reclaim-[^ ]*)$/u)?.[1]??"";};
 return async(program:string,args:readonly string[])=>{
  const name=basename(program),windows=name==="powershell.exe",script=windows?args.at(-1)!:"",key=windows?name+" "+script.split(" ")[0]:name+" "+args[0],values=(cycle.outputs as Record<string,string[]>)[key]??[""],index=seen.get(key)??0;seen.set(key,index+1);
  log.push(name+" "+(windows?script:args.join(" ")));
  return values[Math.min(index,values.length-1)]!.replaceAll("<scratch>",scratch());
 };
};
const oracle=(...args:string[])=>run("/usr/bin/python3",["-I",join(import.meta.dir,"🐍️.py"),...args]).trim();
const fake=(log:string[],sizes:number[],options:{attached?:boolean;busy?:boolean}={}):NativeImageReclaimPortV1=>{let attached=options.attached?"/dev/disk1":null,reads=0;return{
 async attachedDevice(){return attached;},
 async detach(device){log.push("detach "+device);if(options.busy&&attached==="/dev/disk1"&&log.filter(row=>row.startsWith("detach")).length===1)throw Error("busy");attached=null;},
 async attach(){log.push("attach");attached="/dev/disk9";},
 allocatedBytes(){return sizes[Math.min(reads++,sizes.length-1)]!;},
 dispose(){log.push("dispose");}
};};
test("the corpus is schema-valid and the decision equals the independent oracle on every case",()=>{
 expect(validateJsonSchemaSubset(schema.definitions.Corpus,fixture,schema)).toEqual([]);
 const expected=fixture.cases.map(row=>row.due),ours=fixture.cases.map(row=>nativeImageReclaimDueV1(fixture.rule,row.allocatedBytes,row.usedBytes,row.hostFreeBytes));
 expect(ours).toEqual(expected);expect(JSON.parse(oracle("decide",join(import.meta.dir,"🔣️.json")))).toEqual(expected);
});
for(const cycle of fixture.cycles)test("cycle corpus: "+cycle.name,async()=>{
 const log:string[]=[],run=scripted(cycle,log),steps:number[]=[];
 expect(createNativeImageReclaimPortV1(cycle.platform,{image:cycle.image,cwd:"/",run})).not.toBeNull();
 const port=cycle.platform==="linux"?createNativeImageLinuxReclaimPortV1({image:cycle.image,run,allocated:()=>5}):createNativeImageWindowsReclaimPortV1({image:cycle.image,run,allocated:()=>5});
 const outcome=await reclaimNativeImageV1(port,{check:()=>{},progress:done=>steps.push(done),sleep:async()=>{}});
 expect(outcome.settled).toBe(true);expect(steps.at(-1)).toBe(protocol.reclaim.progressSteps);
 const scratch=log.map(line=>line.match(/(\/[^ ]*semio-reclaim-[^ ]*)/u)?.[1]).find(Boolean);
 expect(log.map(line=>scratch?line.replaceAll(scratch,"<scratch>"):line)).toEqual(cycle.expected);
});
test("a failed mount releases the loop device again and cancellation during the trim still detaches",async()=>{
 const log:string[]=[];
 const failing=createNativeImageLinuxReclaimPortV1({image:"/i/fixed.img",allocated:()=>1,run:async(program,args)=>{log.push(basename(program)+" "+args[0]);if(basename(program)==="mount")throw Error("mount failed");return args[0]==="--find"?"/dev/loop1\n":"";}});
 await expect(reclaimNativeImageV1(failing,{check:()=>{},sleep:async()=>{}})).rejects.toThrow("mount failed");
 expect(log).toEqual(["losetup --associated","losetup --find","mount -o","losetup --detach"]);
 const trims:string[]=[];let checks=0;
 const base=createNativeImageLinuxReclaimPortV1({image:"/i/fixed.img",allocated:()=>1,run:async(program,args)=>{trims.push(basename(program)+" "+args[0]);return args[0]==="--find"?"/dev/loop1\n":args[0]==="--source"?"":args[0]==="--associated"&&trims.filter(row=>row==="losetup --find").length?"/dev/loop1\n":"";}});
 await expect(reclaimNativeImageV1(base,{check:()=>{if(++checks>=3)throw Error("cancelled");},sleep:async()=>{}})).rejects.toThrow("cancelled");
 expect(trims.at(-1)).toBe("losetup --detach");expect(trims).not.toContain("fstrim --verbose");
});
test("only the three known platforms get a port and windows is named by either spelling",()=>{
 expect(createNativeImageReclaimPortV1("plan9",{image:"/i",cwd:"/",run:async()=>""})).toBeNull();
 expect(createNativeImageReclaimPortV1("win32",{image:"C:\\i.vhdx",cwd:"C:\\",run:async()=>""})).not.toBeNull();
 expect(nativeImageReclaimMechanismV1("win32")?.imageName).toBe("fixed.vhdx");expect(nativeImageReclaimMechanismV1("linux")?.imageName).toBe("fixed.img");
});
test("availability corpus: the stale-statfs decision equals the independent oracle on every case",()=>{
 const expected=fixture.availability.map(row=>row.effectiveBytes);
 expect(fixture.availability.map(row=>nativeImageEffectiveAvailableBytesV1(row.volumeAvailableBytes,(row as{containerFreeBytes?:number}).containerFreeBytes,(row as{hostFreeBytes?:number}).hostFreeBytes))).toEqual(expected);
 expect(JSON.parse(oracle("effective",join(import.meta.dir,"🔣️.json")))).toEqual(expected);
});
test("the container free bytes come from the diskutil plist and only on macOS",()=>{
 const plist='<?xml version="1.0" encoding="UTF-8"?><!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd"><plist version="1.0"><dict><key>APFSContainerFree</key><integer>81800000000</integer><key>VolumeName</key><string>r</string></dict></plist>';
 expect(nativeImageContainerFreeBytesV1(plist)).toBe(81800000000);expect(nativeImageContainerFreeBytesV1("garbage")).toBeUndefined();
 const calls:string[]=[];expect(nativeImageContainerFreeBytesSyncV1("/m","darwin",(program,args)=>{calls.push(program+" "+args.join(" "));return plist;})).toBe(81800000000);
 expect(calls).toEqual(["/usr/sbin/diskutil info -plist /m"]);expect(nativeImageContainerFreeBytesSyncV1("/m","linux",()=>plist)).toBeUndefined();expect(nativeImageContainerFreeBytesSyncV1("/m","darwin",()=>undefined)).toBeUndefined();
});
test("every platform names its mechanism and an unknown platform names none",()=>{
 for(const platform of["darwin","linux","windows"])expect(nativeImageReclaimMechanismV1(platform)?.mechanism).toBeString();
 expect(nativeImageReclaimMechanismV1("darwin")?.mechanism).toBe("unmap-on-attach");expect(nativeImageReclaimMechanismV1("plan9")).toBeNull();
});
test("every attach goes through the UNMAP-capable stack and nothing else attaches",()=>{
 expect(nativeImageAttachProgramV1).toBe("/usr/sbin/diskutil");
 expect(nativeImageAttachArgumentsV1("/m","/i/fixed.dmg")).toEqual(["image","attach","--nobrowse","--mountOptions","owners","--mountPoint","/m","/i/fixed.dmg"]);
 expect(nativeImageHdiutilCommandV1(nativeImageAttachArgumentsV1("/m","/i/fixed.dmg"),"/").program).toBe(nativeImageAttachProgramV1);
 expect(nativeImageHdiutilCommandV1(["detach","/dev/disk1"],"/").program).toBe(nativeImageHdiutilProgramV1);
});
test("a cycle attaches, settles and detaches again, reporting monotonic progress",async()=>{
 const log:string[]=[],steps:number[]=[];
 const outcome=await reclaimNativeImageV1(fake(log,[900,900,500,400,400,400,400,400,100]),{check:()=>{},progress:(done,total)=>{expect(total).toBe(protocol.reclaim.progressSteps);steps.push(done);},sleep:async()=>{}});
 expect(log).toEqual(["attach","detach /dev/disk9","dispose"]);
 expect(outcome).toEqual({beforeBytes:900,afterBytes:100,settled:true});expect(steps).toEqual([...steps].sort((a,b)=>a-b));expect(steps.at(-1)).toBe(protocol.reclaim.progressSteps);
});
test("an attached image is detached before the cycle and a busy one stops the cycle unchanged",async()=>{
 const log:string[]=[];await reclaimNativeImageV1(fake(log,[5,5,5,5,5,5],{attached:true}),{check:()=>{},sleep:async()=>{}});
 expect(log).toEqual(["detach /dev/disk1","attach","detach /dev/disk9","dispose"]);
 const busy:string[]=[];await expect(reclaimNativeImageV1(fake(busy,[5],{attached:true,busy:true}),{check:()=>{},sleep:async()=>{}})).rejects.toThrow("busy");expect(busy).toEqual(["detach /dev/disk1"]);
});
test("cancellation between polls still detaches the image and releases the scratch mount",async()=>{
 const log:string[]=[];let polls=0;
 await expect(reclaimNativeImageV1(fake(log,[9,8,7,6,5,4]),{check:()=>{if(++polls>3)throw Error("cancelled");},sleep:async()=>{}})).rejects.toThrow("cancelled");
 expect(log).toEqual(["attach","detach /dev/disk9","dispose"]);
});
test("an allocation that never settles ends at the bound instead of waiting forever",async()=>{
 const log:string[]=[];let slept=0,size=0;const port={...fake(log,[1]),allocatedBytes:()=>size+=1};
 const outcome=await reclaimNativeImageV1(port,{check:()=>{},sleep:async milliseconds=>{slept+=milliseconds;}});
 expect(outcome.settled).toBe(false);expect(slept).toBe(protocol.reclaim.maximumSettleMilliseconds);
});
test.skipIf(process.platform!=="darwin")("a real image written by the legacy stack gives its deleted blocks back through one cycle, keeping size and identity",async()=>{
 const directory=realpathSync(mkdtempSync(join(testScratchRootV1("reclaim"),"reclaim-"))),seed=createNativeImageVolumeSilenceSeedV1(),image=join(directory,"fixed.dmg"),root=join(directory,"m");
 try{
  mkdirSync(root);
  run(nativeImageHdiutilProgramV1,nativeImageSeededCreateArgumentsV1(512*MiB,image,seed.directory));const grow=nativeImageSparseGrowArgumentsV1(512*MiB,image);if(grow)run(nativeImageHdiutilProgramV1,grow);
  const file=lstatSync(image);
  run(nativeImageHdiutilProgramV1,["attach","-nobrowse","-noautoopen","-mountpoint",root,image]);
  try{writeFileSync(join(root,"payload"),Buffer.alloc(256*MiB,7));run("/bin/sync",[]);rmSync(join(root,"payload"));run("/bin/sync",[]);}finally{run(nativeImageHdiutilProgramV1,["detach",root]);}
  const stale=lstatSync(image).blocks*512;expect(stale).toBeGreaterThan(200*MiB);
  const outcome=await reclaimNativeImageV1(createNativeImageDarwinReclaimPortV1({image,cwd:directory,run:async(program,args)=>run(program,args)}),{check:()=>{}});
  const after=lstatSync(image);expect(outcome.beforeBytes).toBe(stale);expect(outcome.afterBytes).toBe(after.blocks*512);expect(outcome.afterBytes).toBeLessThan(stale-150*MiB);
  expect(Number(oracle("allocated",image))).toBe(after.blocks*512);expect(after.size).toBe(file.size);expect(after.ino).toBe(file.ino);
  expect(run(nativeImageHdiutilProgramV1,["info"])).not.toContain(image);
 }finally{seed.dispose();spawnSync(nativeImageHdiutilProgramV1,["detach",root],{env});rmSync(directory,{recursive:true,force:true});}
},180000);
