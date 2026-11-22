/** ♻️ Tests for image reclaim. */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{Database}from"bun:sqlite";
import{lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{basename,join}from"node:path";
import{testScratchRootV1}from"../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{testPythonV1}from"../../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import imageSchema from"../../../🧬️schema/🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
import{createNativeImageVolumeSilenceSeedV1,nativeImageSeededCreateArgumentsV1}from"../../🤫️silence/🟦️.ts";
import{nativeImageSparseGrowArgumentsV1}from"../../🪶️sparse/🟦️.ts";
import{nativeImageAttachArgumentsV1,nativeImageAttachProgramV1}from"../../👤️owners/🟦️.ts";
import{nativeImageContainerFreeBytesSyncV1,nativeImageContainerFreeBytesV1,nativeImageEffectiveAvailableBytesV1,nativeImageRealAvailableBytesV1}from"../../⚖️availability/🟦️.ts";
import{nativeImageHdiutilCommandV1,nativeImageHdiutilProgramV1}from"../../🛠️hdiutil/🟦️.ts";
import{nativeImageVolumeLatchedV1}from"../../⚖️availability/🟦️.ts";
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
const oracle=(...args:string[])=>{
 const python=testPythonV1();
 if(python===undefined)throw Error("Python required by the reclaim oracle");
 const result=spawnSync(python,["-I",join(import.meta.dir,"🐍️.py"),...args],{encoding:"utf8"});
 if(result.status!==0)throw result.error??Error(result.stderr);
 return result.stdout.trim();
};
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
 const log:string[]=[];let associated=false;
 const failing=createNativeImageLinuxReclaimPortV1({image:"/i/fixed.img",allocated:()=>1,run:async(program,args)=>{
  log.push(basename(program)+" "+args[0]);
  if(basename(program)==="mount")throw Error("mount failed");
  if(args[0]==="--find")associated=true;
  if(args[0]==="--detach")associated=false;
  return associated&&(args[0]==="--find"||args[0]==="--associated")?"/dev/loop1\n":"";
 }});
 await expect(reclaimNativeImageV1(failing,{check:()=>{},sleep:async()=>{}})).rejects.toThrow("mount failed");
 expect(log).toEqual(["losetup --associated","losetup --find","mount -o","losetup --detach","losetup --associated"]);
 const trims:string[]=[];let checks=0;
 const base=createNativeImageLinuxReclaimPortV1({image:"/i/fixed.img",allocated:()=>1,run:async(program,args)=>{trims.push(basename(program)+" "+args[0]);return args[0]==="--find"?"/dev/loop1\n":args[0]==="--source"?"":args[0]==="--associated"&&trims.filter(row=>row==="losetup --find").length?"/dev/loop1\n":"";}});
 await expect(reclaimNativeImageV1(base,{check:()=>{if(++checks>=3)throw Error("cancelled");},sleep:async()=>{}})).rejects.toThrow("cancelled");
 expect(trims.at(-1)).toBe("losetup --detach");expect(trims).not.toContain("fstrim --verbose");
});
test("reclaim cleans up a partial attach and retains both failures when detach refuses",async()=>{
 const db=new Database(":memory:");
 try{
  for(const refuses of [false,true]){
   const expected=db.query("select case when ?1 then 0 else 1 end as disposed").get(Number(refuses)) as {disposed:number};
   const log:string[]=[];
   const primary=Error("attach failed after mounting"),cleanup=Error("device still busy");
   let device:string|null=null;
   const port:NativeImageReclaimPortV1={
    async attachedDevice(){return device;},
    async attach(){device="/dev/owned";log.push("attach");throw primary;},
    async detach(actual){log.push("detach "+actual);if(refuses)throw cleanup;device=null;},
    allocatedBytes(){return 1;},
    dispose(){log.push("dispose");},
   };
   const error=await reclaimNativeImageV1(port,{check:()=>{}}).catch(failure=>failure);
   expect(log.slice(0,2)).toEqual(["attach","detach /dev/owned"]);
   expect(log.includes("dispose")).toBe(Boolean(expected.disposed));
   if(refuses){
    expect(error).toBeInstanceOf(AggregateError);
    expect(error.errors).toEqual([primary,cleanup]);
   }else expect(error).toBe(primary);
  }
 }finally{db.close();}
});

test("Darwin reclaim uses the supplied cleanup runner after a partial attach fails",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("reclaim"),"runner-")),image=join(directory,"fixed.dmg");
 writeFileSync(image,"owned image");
 const primary=Error("attach reported failure"),calls:string[]=[];
 let scratch:string|undefined;
 const plistImage=image.replaceAll("&","&amp;").replaceAll("<","&lt;").replaceAll(">","&gt;");
 const roster=(attached:boolean)=>`<?xml version="1.0" encoding="UTF-8"?><plist version="1.0"><dict><key>images</key><array>${attached?`<dict><key>image-path</key><string>${plistImage}</string><key>system-entities</key><array><dict><key>dev-entry</key><string>/dev/owned</string></dict></array></dict>`:""}</array></dict></plist>`;
 const port=createNativeImageDarwinReclaimPortV1({image,cwd:directory,
  run:async(_program,args)=>{
   calls.push("work "+args[0]);
   if(args[0]==="info")return roster(false);
   scratch=args[args.indexOf("--mountPoint")+1]!;
   throw primary;
  },
  cleanupRun:async(_program,args)=>{
   calls.push("cleanup "+args.join(" "));
   return args[0]==="info"?roster(true):"";
  },
 });
 try{
  expect(await reclaimNativeImageV1(port,{check:()=>{}}).catch(error=>error)).toBe(primary);
  expect(calls).toEqual(["work info","work image","cleanup info -plist","cleanup detach /dev/owned"]);
  expect(lstatSync(scratch!,{throwIfNoEntry:false})).toBeUndefined();
  expect(readFileSync(image,"utf8")).toBe("owned image");
 }finally{
  if(scratch)rmSync(scratch,{recursive:true,force:true});
  rmSync(directory,{recursive:true,force:true});
 }
});

test("Darwin scratch disposal preserves files when a reported mount still holds them",async()=>{
 let scratch:string|undefined;
 const image="/i/fixed.dmg",port=createNativeImageDarwinReclaimPortV1({image,cwd:"/",run:async(_program,args)=>{
  scratch=args[args.indexOf("--mountPoint")+1]!;
  writeFileSync(join(scratch,"payload"),"retained data");
  return "";
 }});
 try{
  await port.attach();
  expect(()=>port.dispose()).toThrow();
  expect(readFileSync(join(scratch!,"payload"),"utf8")).toBe("retained data");
 }finally{if(scratch)rmSync(scratch,{recursive:true,force:true});}
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
test("storage floor corpus: a stale-low volume with a roomy host does not stop, a truly low host does, equal to the independent oracle",()=>{
 const real=fixture.floor.map(row=>nativeImageRealAvailableBytesV1(row.volumeAvailableBytes,(row as{containerFreeBytes?:number}).containerFreeBytes,row.hostFreeBytes));
 expect(real).toEqual(fixture.floor.map(row=>row.realBytes));
 expect(real.map((bytes,index)=>bytes<fixture.floor[index]!.reserveBytes)).toEqual(fixture.floor.map(row=>row.stop));
 expect(JSON.parse(oracle("floor",join(import.meta.dir,"🔣️.json")))).toEqual(fixture.floor.map(row=>[row.realBytes,row.stop]));
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

test("a latched volume is detached with force after a busy refusal and an honestly full or unlatched volume never is",async()=>{
 const gib=1073741824,pure=[[0,100*gib,30*gib,true],[0,100*gib,0,false],[5*gib,100*gib,30*gib,false],[0,undefined,30*gib,false],[0,100,30*gib,false]] as const;
 for(const[volume,container,host,expected]of pure)expect(nativeImageVolumeLatchedV1(volume,container,host)).toBe(expected);
 const plist=`<?xml version="1.0" encoding="UTF-8"?><plist version="1.0"><dict><key>images</key><array><dict><key>image-path</key><string>/i/fixed.dmg</string><key>system-entities</key><array><dict><key>dev-entry</key><string>/dev/d</string><key>mount-point</key><string>/m</string></dict></array></dict></array></dict></plist>`;
 for(const[latched,calls]of[[true,["detach /dev/d","detach -force /dev/d"]],[false,["detach /dev/d"]]] as const){
  const seen:string[]=[],port=createNativeImageDarwinReclaimPortV1({image:"/i/fixed.dmg",cwd:"/",latched:()=>latched,run:async(_program,args)=>{
   if(args[0]==="info")return plist;
   seen.push(args.join(" "));
   if(args.join(" ")==="detach /dev/d")throw Error("hdiutil exited with status 16 couldn't unmount disk15 - Resource busy");
   return "";
  }});
  const device=await port.attachedDevice();
  expect(device).toBe("/dev/d");
  await port.detach(device!).catch(()=>{});
  expect(seen).toEqual(calls);
 }
});

test("the macOS cleanup waits for a late device after a failed attach and forces the detach when the plain one fails, without any wait after a clean attach",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("reclaim"),"late-")),image=join(directory,"fixed.dmg");
 writeFileSync(image,"owned image");
 const empty=`<?xml version="1.0" encoding="UTF-8"?><plist version="1.0"><dict><key>images</key><array/></dict></plist>`;
 const attached=`<?xml version="1.0" encoding="UTF-8"?><plist version="1.0"><dict><key>images</key><array><dict><key>image-path</key><string>${image}</string><key>system-entities</key><array><dict><key>dev-entry</key><string>/dev/d</string><key>mount-point</key><string>/scratch</string></dict></array></dict></array></dict></plist>`;
 try{
  for(const failing of[true,false]){
   const seen:string[]=[],sleeps:number[]=[];let infos=0;
   const port=createNativeImageDarwinReclaimPortV1({image,cwd:directory,sleep:async milliseconds=>{sleeps.push(milliseconds);},
    run:async(_program,args)=>{if(args[0]==="info")return empty;if(failing)throw Error("attach ended by its deadline");return "";},
    cleanupRun:async(_program,args)=>{
     if(args[0]==="info")return ++infos>=(failing?3:1)?attached:empty;
     seen.push(args.join(" "));
     if(args.join(" ")==="detach /dev/d")throw Error("hdiutil exited with status 1 not a busy refusal");
     return "";
    }});
   const outcome=await reclaimNativeImageV1(port,{check:()=>{},sleep:async()=>{}}).catch(error=>error);
   if(failing){expect(outcome.message).toContain("attach ended by its deadline");expect(infos).toBe(3);expect(sleeps).toEqual([protocol.reclaim.settlePollMilliseconds,protocol.reclaim.settlePollMilliseconds]);}
   else{expect(outcome.settled).toBe(true);expect(infos).toBe(1);expect(sleeps).toEqual([]);}
   expect(seen).toEqual(["detach /dev/d","detach -force /dev/d"]);
  }
 }finally{rmSync(directory,{recursive:true,force:true});}
});
test("a cleanup runner that is never cancelled detaches after the work runner was cancelled",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("reclaim"),"cancel-")),image=join(directory,"fixed.dmg");
 writeFileSync(image,"owned image");
 const controller=new AbortController(),calls:string[]=[];
 const roster=(on:boolean)=>`<?xml version="1.0" encoding="UTF-8"?><plist version="1.0"><dict><key>images</key><array>${on?`<dict><key>image-path</key><string>${image}</string><key>system-entities</key><array><dict><key>dev-entry</key><string>/dev/d</string></dict></array></dict>`:""}</array></dict></plist>`;
 try{
  const port=createNativeImageDarwinReclaimPortV1({image,cwd:directory,
   run:async(_program,args)=>{if(controller.signal.aborted)throw Error("cancelled");calls.push("work "+args[0]);if(args[0]==="info")return roster(false);controller.abort();return "";},
   cleanupRun:async(_program,args)=>{calls.push("cleanup "+args.join(" "));return args[0]==="info"?roster(true):"";}});
  await expect(reclaimNativeImageV1(port,{check:()=>{if(controller.signal.aborted)throw Error("cancelled");},sleep:async()=>{}})).rejects.toThrow("cancelled");
  expect(calls).toEqual(["work info","work image","cleanup info -plist","cleanup detach /dev/d"]);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

const canonicalFake=(log:string[],options:{attached:boolean;failFirstAttach?:boolean}):NativeImageReclaimPortV1=>{let attached:string|null=options.attached?"/dev/disk1":null,attaches=0;return{
 async attachedDevice(){return attached;},
 async detach(device,phase){log.push("detach "+device+(phase?" "+phase:""));attached=null;},
 async attach(phase){log.push("attach"+(phase?" "+phase:""));if(options.failFirstAttach&&attaches++===0)throw Error("attach failed");attached="/dev/disk9";},
 allocatedBytes(){return 5;},
 dispose(){log.push("dispose");},
 mountsCanonically:true
};};
test("a port that mounts canonically gives an attached image back attached on every exit path and an inactive image back inactive",async()=>{
 const done:string[]=[];await reclaimNativeImageV1(canonicalFake(done,{attached:true}),{check:()=>{},sleep:async()=>{}});
 expect(done).toEqual(["detach /dev/disk1","attach","dispose"]);
 const cancelled:string[]=[];let polls=0;
 await expect(reclaimNativeImageV1(canonicalFake(cancelled,{attached:true}),{check:()=>{if(++polls>3)throw Error("cancelled");},sleep:async()=>{}})).rejects.toThrow("cancelled");
 expect(cancelled).toEqual(["detach /dev/disk1","attach","dispose"]);
 const failed:string[]=[];
 await expect(reclaimNativeImageV1(canonicalFake(failed,{attached:true,failFirstAttach:true}),{check:()=>{},sleep:async()=>{}})).rejects.toThrow("attach failed");
 expect(failed).toEqual(["detach /dev/disk1","attach","attach cleanup","dispose"]);
 const inactive:string[]=[];await reclaimNativeImageV1(canonicalFake(inactive,{attached:false}),{check:()=>{},sleep:async()=>{}});
 expect(inactive).toEqual(["attach","detach /dev/disk9 cleanup","dispose"]);
});

test("Darwin and Linux ports with a mount point attach there and never create a scratch mount, and a cancelled cycle remounts with the runner no cancellation reaches",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("reclaim"),"canonical-")),image=join(directory,"fixed.dmg"),root=join(directory,"m");
 writeFileSync(image,"owned image");mkdirSync(root);
 const controller=new AbortController(),calls:string[]=[];let attached=true;
 const roster=()=>`<?xml version="1.0" encoding="UTF-8"?><plist version="1.0"><dict><key>images</key><array>${attached?`<dict><key>image-path</key><string>${image}</string><key>system-entities</key><array><dict><key>dev-entry</key><string>/dev/d</string><key>mount-point</key><string>${root}</string></dict></array></dict>`:""}</array></dict></plist>`;
 try{
  const port=createNativeImageDarwinReclaimPortV1({image,cwd:directory,mountPoint:root,
   run:async(_program,args)=>{if(controller.signal.aborted)throw Error("cancelled");calls.push("work "+args.join(" "));if(args[0]==="info")return roster();if(args[0]==="detach"){attached=false;return "";}controller.abort();return "";},
   cleanupRun:async(_program,args)=>{calls.push("cleanup "+args.join(" "));if(args[0]==="info")return roster();attached=true;return "";}});
  expect(port.mountsCanonically).toBe(true);
  await expect(reclaimNativeImageV1(port,{check:()=>{if(controller.signal.aborted)throw Error("cancelled");},sleep:async()=>{}})).rejects.toThrow("cancelled");
  expect(calls).toEqual(["work info -plist","work detach /dev/d",`work image attach --nobrowse --mountOptions owners --mountPoint ${root} ${image}`,"cleanup info -plist",`cleanup image attach --nobrowse --mountOptions owners --mountPoint ${root} ${image}`]);
  expect(attached).toBe(true);
  const linux:string[]=[];
  const linuxPort=createNativeImageLinuxReclaimPortV1({image:"/i/fixed.img",mountPoint:"/cache/r/m",allocated:()=>5,run:async(program,args)=>{linux.push(basename(program)+" "+args.join(" "));return args[0]==="--find"?"/dev/loop7\n":args[0]==="--associated"?"/dev/loop7\n":args[0]==="--source"?"/cache/r/m\n":"";}});
  expect(linuxPort.mountsCanonically).toBe(true);
  await linuxPort.attach();
  expect(linux).toContain("mount -o discard,noatime /dev/loop7 /cache/r/m");
  linuxPort.dispose();
  expect(createNativeImageLinuxReclaimPortV1({image:"/i/fixed.img",run:async()=>""}).mountsCanonically).toBe(false);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test.skipIf(process.platform!=="darwin")("a real image that was attached at its mount point is attached at the same mount point after a cycle and nothing is left at a scratch mount",async()=>{
 const directory=realpathSync(mkdtempSync(join(testScratchRootV1("reclaim"),"canonical-real-"))),image=join(directory,"fixed.dmg"),root=join(directory,"m");
 try{
  mkdirSync(root);
  run(nativeImageHdiutilProgramV1,["create","-size","64m","-fs","APFS","-volname","e14",image]);
  run(nativeImageAttachProgramV1,[...nativeImageAttachArgumentsV1(root,image)]);
  expect(lstatSync(root).dev).not.toBe(lstatSync(directory).dev);
  const outcome=await reclaimNativeImageV1(createNativeImageDarwinReclaimPortV1({image,cwd:directory,mountPoint:root,run:async(program,args)=>run(program,args)}),{check:()=>{}});
  expect(outcome.settled).toBe(true);
  expect(lstatSync(root).dev).not.toBe(lstatSync(directory).dev);
  const listing=run(nativeImageHdiutilProgramV1,["info"]);
  expect(listing).toContain(image);expect(listing).not.toContain("semio-reclaim-");
  run(nativeImageHdiutilProgramV1,["detach",root]);
  const inactive=await reclaimNativeImageV1(createNativeImageDarwinReclaimPortV1({image,cwd:directory,mountPoint:root,run:async(program,args)=>run(program,args)}),{check:()=>{}});
  expect(inactive.settled).toBe(true);expect(lstatSync(root).dev).toBe(lstatSync(directory).dev);expect(run(nativeImageHdiutilProgramV1,["info"])).not.toContain(image);
 }finally{spawnSync(nativeImageHdiutilProgramV1,["detach","-force",root],{env});rmSync(directory,{recursive:true,force:true});}
},180000);

test("a cancellation right after the prior detach still remounts a canonical image",async()=>{
 const log:string[]=[];let checks=0;
 await expect(reclaimNativeImageV1(canonicalFake(log,{attached:true}),{check:()=>{if(++checks>=2)throw Error("cancelled");},sleep:async()=>{}})).rejects.toThrow("cancelled");
 expect(log).toEqual(["detach /dev/disk1","attach cleanup","dispose"]);
});
