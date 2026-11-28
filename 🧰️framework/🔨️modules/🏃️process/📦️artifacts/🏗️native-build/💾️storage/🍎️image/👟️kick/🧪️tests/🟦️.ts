/** 👟️ Tests for the in-place kick that returns freed image blocks to the host while the image stays mounted. */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{closeSync,lstatSync,mkdirSync,mkdtempSync,openSync,readFileSync,realpathSync,rmSync,writeFileSync,writeSync}from"node:fs";
import{basename,join}from"node:path";
import{testScratchRootV1}from"../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{testPythonV1}from"../../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
import{createNativeImageDarwinKickPortV1,createNativeImageKickPortV1,createNativeImageLinuxKickPortV1,kickNativeImageV1,nativeImageKickCanaryReturnedV1,nativeImageKickReturnedBytesV1,nativeImageKickDueV1,nativeImageKickIneffectiveTextV1,nativeImageKickMechanismV1,type NativeImageKickPortV1}from"../🟦️.ts";
const MiB=1048576,env={PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"};
const own=(container:number)=>"<?xml version=\"1.0\"?><plist version=\"1.0\"><dict><key>images</key><array><dict><key>image-path</key><string>/i</string><key>system-entities</key><array><dict><key>dev-entry</key><string>/dev/disk"+container+"</string></dict></array></dict></array></dict></plist>";
const run=(program:string,args:readonly string[])=>{const row=spawnSync(program,[...args],{encoding:"utf8",env});if(row.status!==0)throw Error(program+" "+args.join(" ")+": "+row.stderr);return row.stdout;};
const oracle=(...args:string[])=>{
 const python=testPythonV1();if(python===null)throw Error("Python required by the kick oracle");
 const result=spawnSync(python,["-I",join(import.meta.dir,"🐍️.py"),...args],{encoding:"utf8"});
 if(result.status!==0)throw result.error??Error(result.stderr);return result.stdout.trim();
};
const scripted=(script:typeof fixture.scripts[number],log:string[])=>{
 const seen=new Map<string,number>();
 return async(program:string,args:readonly string[])=>{
  const key=basename(program)+" "+args[0],values=Object.entries(script.outputs).find(([name])=>name===key)?.[1]??[""],index=seen.get(key)??0;seen.set(key,index+1);
  log.push(basename(program)+" "+args.join(" "));return values[Math.min(index,values.length-1)]!;
 };
};
const fake=(log:string[],sizes:number[]):NativeImageKickPortV1=>{let reads=0;return{
 async ensure(){log.push("ensure");},async kick(){log.push("kick");},
 allocatedBytes(){return sizes[Math.min(reads++,sizes.length-1)]!;},
 async dispose(){log.push("dispose");}
};};
test("the corpus is schema-valid and the due decision equals the independent oracle on every case",()=>{
 expect(validateJsonSchemaSubset(schema.definitions.Corpus,fixture,schema)).toEqual([]);
 const expected=fixture.cases.map(row=>row.due),ours=fixture.cases.map(row=>nativeImageKickDueV1(fixture.rule,row,fixture.minimumIntervalMilliseconds));
 expect(ours).toEqual(expected);expect(JSON.parse(oracle("due",join(import.meta.dir,"🔣️.json")))).toEqual(expected);
});
for(const script of fixture.scripts)test("script corpus: "+script.name,async()=>{
 const log:string[]=[],runner=scripted(script,log),steps:number[]=[];
 const mountPoint=script.platform==="darwin"?"/cache/r/m":"/cache/r/m",options={image:script.image,mountPoint,run:runner};
 const port=script.platform==="darwin"?createNativeImageDarwinKickPortV1({...options,allocated:()=>5}):createNativeImageLinuxKickPortV1({...options,allocated:()=>5});
 const outcome=await kickNativeImageV1(port,{check:()=>{},progress:done=>steps.push(done),sleep:async()=>{}});
 expect(outcome.settled).toBe(true);expect(steps).toEqual([0,1,2,3]);
 const scratch=log.map(line=>line.match(/(\/[^ ]*semio-kick-[^ ]*)/u)?.[1]).find(Boolean);
 expect(log.map(line=>scratch?line.replaceAll(scratch,"<scratch>"):line)).toEqual(script.expected);
});
test("every platform names its mechanism, Windows is offline-only and gets no port",()=>{
 expect(nativeImageKickMechanismV1("darwin")?.kind).toBe("sibling-volume");expect(nativeImageKickMechanismV1("linux")?.kind).toBe("fstrim-mounted");
 expect(nativeImageKickMechanismV1("win32")?.kind).toBe("offline-only");expect(nativeImageKickMechanismV1("windows")?.kind).toBe("offline-only");expect(nativeImageKickMechanismV1("freebsd")).toBeNull();
 const options={image:"/i",mountPoint:"/m",run:async()=>""};
 expect(createNativeImageKickPortV1("darwin",options)).not.toBeNull();expect(createNativeImageKickPortV1("linux",options)).not.toBeNull();
 expect(createNativeImageKickPortV1("win32",options)).toBeNull();expect(createNativeImageKickPortV1("freebsd",options)).toBeNull();
});
test("the kick settles when the allocation stops shrinking and reports the allocation before and after",async()=>{
 const log:string[]=[],outcome=await kickNativeImageV1(fake(log,[900,900,500,300,300,300,300,300]),{check:()=>{},sleep:async()=>{}});
 expect(outcome).toEqual({beforeBytes:900,afterBytes:300,settled:true,effective:true});expect(log).toEqual(["ensure","kick","dispose"]);
});
test("an allocation that never settles ends at the bound instead of waiting forever",async()=>{
 let size=1000,slept=0;const port={...fake([],[0]),allocatedBytes:()=>size--};
 const outcome=await kickNativeImageV1(port,{check:()=>{},sleep:async()=>{slept++;}});
 expect(outcome.settled).toBe(false);expect(slept).toBe(schemaBound());
});
const schemaBound=()=>Math.ceil(fixtureProtocol.maximumSettleMilliseconds/fixtureProtocol.settlePollMilliseconds);
const fixtureProtocol=schema.definitions.Protocol.const;
test("cancellation between polls still disposes and a failing kick still disposes, keeping both failures",async()=>{
 const log:string[]=[];let polls=0;
 await expect(kickNativeImageV1(fake(log,[5,4,3,2,1,0]),{check:()=>{if(++polls>3)throw Error("cancelled");},sleep:async()=>{}})).rejects.toThrow("cancelled");
 expect(log.at(-1)).toBe("dispose");
 const failing:NativeImageKickPortV1={...fake([],[1]),async kick(){throw Error("kick failed");},async dispose(){throw Error("dispose failed");}};
 const error=await kickNativeImageV1(failing,{check:()=>{},sleep:async()=>{}}).catch(value=>value);
 expect(error).toBeInstanceOf(AggregateError);expect((error as AggregateError).errors.map(row=>row.message)).toEqual(["kick failed","dispose failed"]);
});
test("a sibling volume that cannot be created is a failure and a mounted sibling is unmounted again by the disposal, forced when the plain unmount refuses",async()=>{
 const log:string[]=[],info="<?xml version=\"1.0\"?><plist version=\"1.0\"><dict><key>APFSContainerReference</key><string>disk7</string></dict></plist>",empty="<?xml version=\"1.0\"?><plist version=\"1.0\"><dict><key>Containers</key><array><dict><key>Volumes</key><array/></dict></array></dict></plist>";
 const refuse=createNativeImageDarwinKickPortV1({image:"/i",mountPoint:"/m",run:async(program,args)=>{log.push(args.join(" "));return program.endsWith("hdiutil")?own(7):args[0]==="info"?info:args[0]==="apfs"&&args[1]==="list"?empty:"";}});
 await expect(refuse.ensure()).rejects.toThrow("Sibling volume semio-trim required in disk7");
 const list="<?xml version=\"1.0\"?><plist version=\"1.0\"><dict><key>Containers</key><array><dict><key>Volumes</key><array><dict><key>DeviceIdentifier</key><string>disk7s2</string><key>Name</key><string>semio-trim</string></dict></array></dict></array></dict></plist>";
 const second:string[]=[];let unmounts=0;
 const port=createNativeImageDarwinKickPortV1({image:"/i",mountPoint:"/m",run:async(program,args)=>{second.push(args.join(" "));return program.endsWith("hdiutil")?own(7):args[0]==="info"?info:args[0]==="apfs"?list:"";},cleanupRun:async(_,args)=>{second.push("cleanup "+args.join(" "));if(args[0]==="unmount"&&args[1]!=="force"&&unmounts++===0)throw Error("busy");return "";}});
 await port.ensure();
 const failing=createNativeImageDarwinKickPortV1({image:"/i",mountPoint:"/m",run:async(program,args)=>{if(args[0]==="unmount")throw Error("unmount failed");return program.endsWith("hdiutil")?own(7):args[0]==="info"?info:args[0]==="apfs"?list:"";},cleanupRun:async(_,args)=>{second.push("cleanup "+args.join(" "));if(args[0]==="unmount"&&args[1]!=="force")throw Error("busy");return "";}});
 await failing.ensure();await expect(failing.kick()).rejects.toThrow("unmount failed");await failing.dispose();
 expect(second.filter(line=>line.startsWith("cleanup"))).toEqual(["cleanup unmount disk7s2","cleanup unmount force disk7s2","cleanup apfs deleteVolume disk7s2"]);
});
test("a kick is effective only when the canary came back, and an ineffective image is named in both languages",async()=>{
 expect(nativeImageKickReturnedBytesV1(1000*MiB,[1000*MiB-128*MiB])).toBe(128*MiB);
 expect(nativeImageKickReturnedBytesV1(1000*MiB,[1030*MiB,1060*MiB,800*MiB,830*MiB])).toBe(260*MiB);
 expect(nativeImageKickReturnedBytesV1(1000*MiB,[1100*MiB,1200*MiB])).toBe(0);
 expect(nativeImageKickCanaryReturnedV1({bytes:256*MiB},128*MiB)).toBe(true);expect(nativeImageKickCanaryReturnedV1({bytes:256*MiB},127*MiB)).toBe(false);
 const busy:NativeImageKickPortV1={...fake([],[900*MiB,930*MiB,960*MiB,700*MiB,730*MiB,760*MiB,790*MiB,820*MiB]),canary:()=>({bytes:256*MiB,allocatedAfterWrite:900*MiB})};
 const growing=await kickNativeImageV1(busy,{check:()=>{},sleep:async()=>{}});expect(growing.effective).toBe(true);expect(growing.canaryReturnedBytes).toBe(260*MiB);
 const gone:NativeImageKickPortV1={...fake([],[900*MiB,900*MiB,900*MiB,900*MiB,900*MiB]),canary:()=>({bytes:256*MiB,allocatedAfterWrite:910*MiB})};
 const stuck=await kickNativeImageV1(gone,{check:()=>{},sleep:async()=>{}});expect(stuck.effective).toBe(false);expect(stuck.canaryReturnedBytes).toBe(10*MiB);
 const returned:NativeImageKickPortV1={...fake([],Array(5).fill(900*MiB)),canary:()=>({bytes:256*MiB,allocatedAfterWrite:1156*MiB})};
 expect((await kickNativeImageV1(returned,{check:()=>{},sleep:async()=>{}})).effective).toBe(true);
 expect((await kickNativeImageV1(fake([],[900*MiB,800*MiB,800*MiB,800*MiB,800*MiB]),{check:()=>{},sleep:async()=>{}})).effective).toBe(true);
 expect((await kickNativeImageV1(fake([],Array(5).fill(900*MiB)),{check:()=>{},sleep:async()=>{}})).effective).toBe(false);
 const values={image:"/i",returned:"0 B",written:"256 MiB"};
 expect(nativeImageKickIneffectiveTextV1("en",values)).toContain("Image /i did not give back a 256 MiB test file after the in-place kick (0 B of 256 MiB returned)");
 expect(nativeImageKickIneffectiveTextV1("de",values)).toContain("Das Image /i gab nach dem direkten Anstoß eine 256-MiB-Testdatei nicht zurück (0 B von 256 MiB zurückgegeben)");
});
test("a container that does not belong to the image is refused before anything is created",async()=>{
 const log:string[]=[],info="<?xml version=\"1.0\"?><plist version=\"1.0\"><dict><key>APFSContainerReference</key><string>disk3</string></dict></plist>",owner="<?xml version=\"1.0\"?><plist version=\"1.0\"><dict><key>images</key><array><dict><key>image-path</key><string>/i</string><key>system-entities</key><array><dict><key>dev-entry</key><string>/dev/disk15</string></dict></array></dict></array></dict></plist>";
 const port=createNativeImageDarwinKickPortV1({image:"/i",mountPoint:"/host",run:async(program,args)=>{log.push(basename(program)+" "+args.join(" "));return args[0]==="info"&&program.endsWith("hdiutil")?owner:args[0]==="info"?info:"";}});
 await expect(port.ensure()).rejects.toThrow("APFS container disk3 does not belong to the image /i");
 expect(log.some(line=>line.includes("addVolume")||line.includes("mount"))).toBe(false);
});
test.skipIf(process.platform!=="darwin")("a real busy image gives its deleted blocks back through the sibling volume while it stays mounted, open and intact",async()=>{
 const directory=realpathSync(mkdtempSync(join(testScratchRootV1("kick"),"kick-"))),image=join(directory,"fixed.dmg"),root=join(directory,"m");
 let device="";
 try{
  mkdirSync(root);
  run("/usr/sbin/diskutil",["image","create","blank","--format","RAW","--size",String(1536*MiB),"--fs","APFS","--volumeName","t",image]);
  run("/usr/sbin/diskutil",["image","attach","--nobrowse","--mountPoint",root,image]);
  device=run("/usr/sbin/diskutil",["info","-plist",root]).match(/<key>DeviceIdentifier<\/key>\s*<string>([^<]+)<\/string>/u)?.[1]??"";
  writeFileSync(join(root,"keep"),Buffer.alloc(4*MiB,3));const open=openSync(join(root,"open"),"w");
  try{
   writeFileSync(join(root,"payload"),Buffer.alloc(512*MiB,7));run("/bin/sync",[]);rmSync(join(root,"payload"));run("/bin/sync",[]);
   const stale=lstatSync(image).blocks*512;expect(stale).toBeGreaterThan(400*MiB);
   const outcome=await kickNativeImageV1(createNativeImageDarwinKickPortV1({image,mountPoint:root,run:async(program,args)=>run(program,args)}),{check:()=>{}});
   const after=lstatSync(image).blocks*512;
   expect(outcome.effective).toBe(true);expect(outcome.canaryBytes).toBe(256*MiB);expect(outcome.beforeBytes).toBe(stale);expect(outcome.afterBytes).toBe(after);expect(after).toBeLessThan(stale-400*MiB);expect(Number(oracle("allocated",image))).toBe(after);
   writeSync(open,"still writable");expect(readFileSync(join(root,"keep")).equals(Buffer.alloc(4*MiB,3))).toBe(true);
   expect(run("/sbin/mount",[])).toContain(" on "+root+" ");expect(run("/sbin/mount",[])).not.toContain("semio-kick-");
   const again=await kickNativeImageV1(createNativeImageDarwinKickPortV1({image,mountPoint:root,run:async(program,args)=>run(program,args)}),{check:()=>{}});
   expect(again.afterBytes).toBeLessThanOrEqual(again.beforeBytes+16*MiB);
   expect(run("/usr/sbin/diskutil",["apfs","list",run("/usr/sbin/diskutil",["info","-plist",root]).match(/<key>APFSContainerReference<\/key>\s*<string>([^<]+)<\/string>/u)?.[1]??""])).not.toContain("semio-trim");
  }finally{closeSync(open);}
 }finally{spawnSync("/usr/bin/hdiutil",["detach",root,"-force"],{env});rmSync(directory,{recursive:true,force:true});void device;}
},240000);
