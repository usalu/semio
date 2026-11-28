/** 🪜️ Tests for the growth of an image and the sizing of a lane image: the plan and the lane sizing against an independent Python oracle (hand-checked rows and 300 seeded random states), the online Linux stretch against its closed command list, the planned growth with a fake lifecycle (held, busy, grown, never shrinks), and on macOS a real scratch shard that cannot be stretched while attached, waits while a holder keeps a file open and grows when it is quiet. */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{closeSync,lstatSync,mkdirSync,mkdtempSync,openSync,readFileSync,realpathSync,rmSync,writeFileSync,writeSync}from"node:fs";
import{join}from"node:path";
import{testScratchRootV1}from"../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{testPythonV1}from"../../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{createNativeImageDetachPortV1}from"../../🔁️detach/🟦️.ts";
import{appendNativeImageShardEventV1,createNativeImageShardDarwinLifecycleV1,ensureNativeImageShardV1,nativeImageShardFoldV1,nativeImageShardHomeLayoutV1,nativeImageShardMountedV1,nativeImageShardSelectV1,readNativeImageShardLedgerV1,requireNativeImageShardRuleV1,type NativeImageShardRuleV1}from"../../🧩️shard/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
import{NATIVE_IMAGE_GROWTH_V1,growNativeImageOnlineV1,growNativeImageShardPlannedV1,shrinkNativeImageShardPlannedV1,nativeImageGrowthModeV1,nativeImageGrowthPlanV1,nativeImageGrowthTextV1,nativeImageLaneSizingV1}from"../🟦️.ts";
const MiB=1048576,GiB=1073741824,env={PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"};
const rule=corpus.rule as NativeImageShardRuleV1;
const oracle=(...args:string[])=>{
 const python=testPythonV1();if(python===null)throw Error("Python required by the growth oracle");
 const result=spawnSync(python,["-I",join(import.meta.dir,"🐍️.py"),...args],{encoding:"utf8",maxBuffer:64*MiB});
 if(result.status!==0)throw result.error??Error(result.stderr);return result.stdout.trim();
};
const answers=JSON.parse(oracle("answers",join(import.meta.dir,"🔣️.json")));
const plan=(row:{platform:string;facts:Record<string,unknown>})=>nativeImageGrowthPlanV1(row.platform,rule,row.facts as never);
test("the corpus is schema-valid and the sizing rule is closed",()=>{
 expect(validateJsonSchemaSubset(schema.definitions.Corpus,corpus,schema)).toEqual([]);expect(requireNativeImageShardRuleV1(corpus.rule)).toEqual(corpus.rule);
 expect(nativeImageGrowthModeV1("darwin")).toBe("detached");expect(nativeImageGrowthModeV1("linux")).toBe("online");expect(nativeImageGrowthModeV1("win32")).toBe("detached");expect(nativeImageGrowthModeV1("freebsd")).toBe("unsupported");
});
test("the growth plan of every corpus row equals the independent oracle, and the hand-checked rows say what the fleet needs",()=>{
 const ours=corpus.growth.map(plan);expect(ours).toEqual(answers.growth);
 const by=(name:string)=>ours[corpus.growth.findIndex(row=>row.name===name)]!;
 expect(by("m12 cargo shard full, host 15 GB")).toMatchObject({action:"grow",targetBytes:25*GiB,deltaBytes:9*GiB});
 expect(by("same but held by cargo")).toMatchObject({action:"wait",reason:"held",targetBytes:25*GiB});
 expect(by("host keeps 3 GiB")).toMatchObject({action:"refused",reason:"host-short",deltaBytes:0});
 expect(by("at cap")).toMatchObject({action:"none",reason:"at-cap"});expect(by("below threshold").action).toBe("none");
 expect(by("budget room only 2 GiB")).toMatchObject({action:"grow",deltaBytes:2*GiB});expect(by("budget exhausted")).toMatchObject({action:"refused",reason:"budget"});
 expect(by("linux online while held")).toMatchObject({action:"grow",mode:"online"});expect(by("unknown platform")).toMatchObject({action:"refused",reason:"unsupported"});
 expect(by("keep raised by machine floor, clamped").action).toBe("grow");
});
test("a plan never shrinks, never passes the cap, never leaves the host below its keep, and always keeps whole alignment steps",()=>{
 let grows=0;
 for(const row of corpus.growth){
  const facts=row.facts as {capacityBytes:number;usedBytes:number;hostFreeBytes:number;budgetRoomBytes?:number},out=plan(row);
  expect(out.targetBytes).toBeGreaterThanOrEqual(facts.capacityBytes);expect(out.targetBytes).toBeGreaterThanOrEqual(Math.min(facts.usedBytes,out.targetBytes));
  if(out.action==="grow"||out.action==="wait"){
   grows++;expect(out.deltaBytes).toBeGreaterThanOrEqual(GiB);expect(out.targetBytes%GiB).toBe(0);expect(out.targetBytes).toBeLessThanOrEqual(32*GiB*(row.platform==="darwin"?1:4));
   expect(facts.hostFreeBytes-out.deltaBytes-NATIVE_IMAGE_GROWTH_V1.initialBytes).toBeGreaterThanOrEqual(NATIVE_IMAGE_GROWTH_V1.hostKeepBytes);
   if(facts.budgetRoomBytes!==undefined)expect(out.deltaBytes).toBeLessThanOrEqual(facts.budgetRoomBytes);
  }else expect(out.deltaBytes).toBe(0);
 }
 expect(grows).toBeGreaterThan(50);
});
test("the lane sizing equals the oracle, lowers the gate of a small host to the measured need and keeps the preferred size of a roomy host",()=>{
 const ours=corpus.lane.map(row=>{const out=nativeImageLaneSizingV1(rule,row.root,row.samples);return out.measuredNeedBytes===undefined?{changed:false}:{changed:true,needBytes:out.needBytes,measuredNeedBytes:out.measuredNeedBytes,headroomPercent:out.rule.headroomPercent};});
 expect(ours).toEqual(answers.lane);
 const sized=nativeImageLaneSizingV1(rule,0,[5*GiB,6*GiB,6*GiB,7*GiB]);requireNativeImageShardRuleV1(sized.rule);expect(sized.rule.roots[1]).toEqual(rule.roots[1]);expect(sized.needBytes).toBe(7*GiB);
 const choose=(given:NativeImageShardRuleV1,hostFreeBytes:number)=>nativeImageShardSelectV1({rule:given,platform:"darwin",live:new Map(),path:"u/l/lane1/x",hostFreeBytes});
 expect(choose(rule,21*GiB)).toMatchObject({kind:"refused",reason:"host-free"});
 const small=choose(sized.rule,21*GiB);expect(small).toMatchObject({kind:"birth",capacityBytes:13*GiB});
 const roomy=choose(sized.rule,200*GiB);expect(roomy).toMatchObject({kind:"birth",capacityBytes:32*GiB});
 expect(nativeImageLaneSizingV1(rule,0,[]).rule).toBe(rule);
});
test("a lane need above what the cap holds with its headroom is cut to it, so the lane is born at the cap instead of being refused for a size it can never have",()=>{
 const sized=nativeImageLaneSizingV1(rule,0,[30*GiB],"darwin");
 expect(sized.needBytes).toBe(Math.floor(32*GiB*100/(100+NATIVE_IMAGE_GROWTH_V1.laneSizing.headroomPercent)));
 const choose=(need:ReturnType<typeof nativeImageLaneSizingV1>)=>nativeImageShardSelectV1({rule:need.rule,platform:"darwin",live:new Map(),path:"u/l/lane1/x",hostFreeBytes:200*GiB});
 expect(choose(nativeImageLaneSizingV1(rule,0,[30*GiB]))).toMatchObject({kind:"refused",reason:"cap"});
 expect(choose(sized)).toMatchObject({kind:"birth",capacityBytes:32*GiB});
 expect(nativeImageLaneSizingV1(rule,0,[5*GiB,7*GiB],"darwin").needBytes).toBe(7*GiB);
});
test("an online stretch runs the closed Linux commands in order, truncates before the loop device re-reads, and never shrinks",async()=>{
 const calls:string[]=[],image="/x/fixed.img";
 await growNativeImageOnlineV1({image,currentBytes:16*GiB,targetBytes:24*GiB,run:async(program,args)=>{calls.push(program+" "+args.join(" "));return program.endsWith("losetup")&&args[0]==="--associated"?"/dev/loop7\n":"";},extend:(path,bytes)=>{calls.push("extend "+path+" "+bytes);}});
 expect(calls).toEqual(["/usr/sbin/losetup --associated /x/fixed.img --noheadings --output NAME","extend /x/fixed.img "+24*GiB,"/usr/sbin/losetup --set-capacity /dev/loop7","/usr/sbin/resize2fs /dev/loop7"]);
 await expect(growNativeImageOnlineV1({image,currentBytes:16*GiB,targetBytes:16*GiB,run:async()=>"",extend:()=>{}})).rejects.toThrow("Larger aligned");
 await expect(growNativeImageOnlineV1({image,currentBytes:16*GiB,targetBytes:24*GiB,run:async()=>"\n",extend:()=>{calls.push("never");}})).rejects.toThrow("Loop device");
 expect(calls).not.toContain("never");
 expect(nativeImageGrowthTextV1("en","grown",{image:"a",from:1,to:2,free:3})).toContain("grew from 1 MiB to 2 MiB");expect(nativeImageGrowthTextV1("de","wait",{image:"a",to:2})).toContain("ruhigen Moment");
 for(const key of Object.keys(NATIVE_IMAGE_GROWTH_V1.messages)as(keyof typeof NATIVE_IMAGE_GROWTH_V1.messages)[]){expect(NATIVE_IMAGE_GROWTH_V1.messages[key].en.length).toBeGreaterThan(10);expect(NATIVE_IMAGE_GROWTH_V1.messages[key].de.length).toBeGreaterThan(10);}
});
const serial=()=>{let tail=Promise.resolve();return<T>(body:()=>Promise<T>):Promise<T>=>{const next=tail.then(body);tail=next.then(()=>undefined,()=>undefined);return next;};};
const born=(directory:string,capacityBytes:number)=>appendNativeImageShardEventV1(directory,{type:"born",id:"lane1",key:"lane1",root:0,path:"u/l/lane1",capacityBytes,atMs:1});
const fakeLifecycle=(log:string[],fail?:Error)=>({create:async()=>{},attach:async()=>{log.push("attach");},detach:async()=>{},resize:async(_:unknown,bytes:number)=>{log.push("grow "+bytes);if(fail)throw fail;},retire:async()=>{}})as never;
const grower=(directory:string,log:string[],extra:Record<string,unknown>={})=>({directory,rule:{...rule,maximumImageBytes:32*GiB},platform:"darwin",id:"lane1",measure:()=>({usedBytes:15*GiB,hostFreeBytes:100*GiB}),now:()=>5,lifecycle:fakeLifecycle(log),lock:serial(),...extra});
test("a planned growth of a shard is lease-aware: a holder makes it wait and changes nothing, a busy detach waits, a quiet image grows and is logged only afterwards",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("growth"),"plan-"));
 try{
  born(directory,16*GiB);const log:string[]=[];
  expect(await growNativeImageShardPlannedV1(grower(directory,log,{lease:async()=>undefined}) as never)).toEqual({kind:"wait",reason:"held"});expect(log).toEqual([]);
  expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory)).get("lane1")!.capacityBytes).toBe(16*GiB);
  const busy=await growNativeImageShardPlannedV1(grower(directory,log,{lifecycle:fakeLifecycle(log,Error("hdiutil detach x exited with status 16 signal null: Resource busy"))}) as never);expect(busy).toEqual({kind:"wait",reason:"busy"});
  expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory)).get("lane1")!.capacityBytes).toBe(16*GiB);
  const released:string[]=[];log.length=0;
  const done=await growNativeImageShardPlannedV1(grower(directory,log,{lease:async()=>()=>{released.push("lease");}}) as never);expect(done).toEqual({kind:"grown",capacityBytes:30*GiB});expect(log).toEqual(["grow "+30*GiB]);expect(released).toEqual(["lease"]);
  expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory)).get("lane1")!.capacityBytes).toBe(30*GiB);
  expect(await growNativeImageShardPlannedV1(grower(directory,log) as never)).toEqual({kind:"none",reason:"below-threshold"});
 }finally{rmSync(directory,{recursive:true,force:true});}
});
test("a planned growth that fails for another reason raises and logs nothing",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("growth"),"fail-"));
 try{born(directory,16*GiB);const log:string[]=[];await expect(growNativeImageShardPlannedV1(grower(directory,log,{lifecycle:fakeLifecycle(log,Error("boom"))}) as never)).rejects.toThrow("boom");expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory)).get("lane1")!.capacityBytes).toBe(16*GiB);}
 finally{rmSync(directory,{recursive:true,force:true});}
});
test("an online platform grows without any lease while the image stays mounted and held",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("growth"),"online-"));
 try{born(directory,16*GiB);const log:string[]=[];const out=await growNativeImageShardPlannedV1(grower(directory,log,{platform:"linux",lease:async()=>{throw Error("no lease for an online stretch");},online:async(_layout:unknown,from:number,to:number)=>{log.push("online "+from+" "+to);}}) as never);expect(out).toEqual({kind:"grown",capacityBytes:30*GiB});expect(log).toEqual(["online "+16*GiB+" "+30*GiB]);}
 finally{rmSync(directory,{recursive:true,force:true});}
});
const run=(program:string,args:readonly string[])=>{const row=spawnSync(program,[...args],{encoding:"utf8",env});if(row.status!==0)throw Error(program+" "+args.join(" ")+": "+row.stderr+row.stdout);return row.stdout;};
test.skipIf(process.platform!=="darwin")("on macOS a mounted image cannot be stretched (measured), a holder makes the planned growth wait with the image untouched and writable, and a quiet image grows to the planned size with its data and mount intact",async()=>{
 const directory=realpathSync(mkdtempSync(join(testScratchRootV1("growth"),"real-"))),home=nativeImageShardHomeLayoutV1(directory),lifecycle=createNativeImageShardDarwinLifecycleV1({run:async(program,args)=>run(program,args)}),lock=serial();
 const small:NativeImageShardRuleV1={enabled:true,maximumImageBytes:2*GiB,headroomPercent:0,hostFreeReservePercent:90,roots:[{prefix:"u/l",pattern:"^[a-z0-9]{1,12}$",needBytes:256*MiB}]};
 try{
  mkdirSync(home.mountPoint,{recursive:true});
  const seed=join(directory,"seed");mkdirSync(seed);run("/usr/bin/hdiutil",["create","-size","1280m","-format","UDRW","-nospotlight","-fs","Case-sensitive APFS","-volname","semio","-srcfolder",seed,home.image]);rmSync(seed,{recursive:true});
  run("/usr/sbin/diskutil",["image","attach","--nobrowse","--mountOptions","owners","--mountPoint",home.mountPoint,home.image]);
  const made=await ensureNativeImageShardV1({directory,rule:small,platform:"darwin",path:"u/l/lane1/x",hostFreeBytes:()=>64*GiB,now:()=>1,lifecycle,lock});
  if(made.kind!=="birth")throw Error("Birth required");const layout=made.layout;expect(made.capacityBytes).toBe(GiB);
  const attached=spawnSync("/usr/bin/hdiutil",["resize","-size","2048m",layout.image],{encoding:"utf8",env});expect(attached.status).not.toBe(0);expect(attached.stderr).toMatch(/Resource temporarily unavailable|status 35|\(35\)/u);expect(lstatSync(layout.image).size).toBeLessThan(2*GiB);
  const holders=createNativeImageDetachPortV1({cwd:directory,run:async(program,args)=>{const row=spawnSync(program,[...args],{encoding:"utf8",env});if(row.status!==0&&!(program.endsWith("lsof")&&row.status===1))throw Error(program+" exited with status "+row.status+" signal null: "+row.stderr);return row.stdout;},sleep:async()=>{}}),given=(extra:Record<string,unknown>={})=>({directory,rule:small,platform:"darwin",id:"lane1",holders:async(layout:{mountPoint:string})=>(await holders.holders(layout.mountPoint)).length,measure:()=>({usedBytes:900*MiB,hostFreeBytes:64*GiB}),measuredNeedBytes:()=>1536*MiB,now:()=>2,lifecycle,lock,...extra});
  writeFileSync(join(layout.mountPoint,"keep"),Buffer.alloc(MiB,3));const open=openSync(join(layout.mountPoint,"open"),"w"),before=lstatSync(layout.image).size;
  try{
   expect(await growNativeImageShardPlannedV1(given({lease:async()=>undefined}) as never)).toEqual({kind:"wait",reason:"held"});
   const started=Date.now(),busy=await growNativeImageShardPlannedV1(given() as never);expect(busy).toEqual({kind:"wait",reason:"busy"});expect(Date.now()-started).toBeLessThan(10000);
   expect(nativeImageShardMountedV1(layout.mountPoint)).toBe(true);expect(lstatSync(layout.image).size).toBe(before);writeSync(open,"still writable");
  }finally{closeSync(open);}
  const done=await growNativeImageShardPlannedV1(given() as never);expect(done).toEqual({kind:"grown",capacityBytes:2*GiB});
  expect(nativeImageShardMountedV1(layout.mountPoint)).toBe(true);expect(lstatSync(layout.image).size).toBeGreaterThanOrEqual(2*GiB-MiB);expect(lstatSync(layout.image).blocks*512).toBeLessThan(512*MiB);
  expect(readFileSync(join(layout.mountPoint,"keep")).equals(Buffer.alloc(MiB,3))).toBe(true);expect(readFileSync(join(layout.mountPoint,"open"),"utf8")).toBe("still writable");
  expect(JSON.parse(readFileSync(layout.receipt,"utf8")).capacityBytes).toBe(2*GiB);expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory)).get("lane1")!.capacityBytes).toBe(2*GiB);
  const df=run("/bin/df",["-k",layout.mountPoint]).split("\n")[1]!.split(/\s+/u);expect(Number(df[1])*1024).toBeGreaterThan(1.8*GiB);
  expect(await growNativeImageShardPlannedV1(given() as never)).toEqual({kind:"none",reason:"at-cap"});
 }finally{for(const path of[join(home.mountPoint,"u/l/lane1"),home.mountPoint])spawnSync("/usr/bin/hdiutil",["detach",path,"-force"],{env});rmSync(directory,{recursive:true,force:true});}
},600000);

const shrinker=(directory:string,log:string[],extra:Record<string,unknown>={})=>({directory,rule:{...rule,maximumImageBytes:32*GiB},platform:"darwin",id:"lane1",measure:()=>({usedBytes:3*GiB,hostFreeBytes:24*GiB}),available:()=>0,now:()=>7,lifecycle:fakeLifecycle(log),lock:serial(),...extra});
test("a planned shrink keeps the content: a healthy image stays, a held or busy one waits and changes nothing, a clamped one is resized and the smaller capacity is logged only afterwards",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("growth"),"shrink-"));
 try{
  born(directory,30*GiB);const log:string[]=[],capacity=()=>nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory)).get("lane1")!.capacityBytes;
  expect(await shrinkNativeImageShardPlannedV1(shrinker(directory,log,{available:()=>27*GiB}) as never)).toEqual({kind:"none"});expect(log).toEqual([]);
  expect(await shrinkNativeImageShardPlannedV1(shrinker(directory,log,{lease:async()=>undefined}) as never)).toEqual({kind:"wait",reason:"held"});
  expect(await shrinkNativeImageShardPlannedV1(shrinker(directory,log,{holders:async()=>2}) as never)).toEqual({kind:"wait",reason:"busy"});
  expect(await shrinkNativeImageShardPlannedV1(shrinker(directory,log,{lifecycle:fakeLifecycle(log,Error("hdiutil detach x exited with status 16 signal null: Resource busy"))}) as never)).toEqual({kind:"wait",reason:"busy"});
  expect(capacity()).toBe(30*GiB);log.length=0;
  const released:string[]=[];
  expect(await shrinkNativeImageShardPlannedV1(shrinker(directory,log,{lease:async()=>()=>{released.push("lease");},holders:async()=>0}) as never)).toEqual({kind:"shrunk",fromBytes:30*GiB,capacityBytes:16*GiB,usedBytes:3*GiB});
  expect(log).toEqual(["grow "+16*GiB]);expect(released).toEqual(["lease"]);expect(capacity()).toBe(16*GiB);
  expect(await shrinkNativeImageShardPlannedV1(shrinker(directory,log) as never)).toEqual({kind:"none"});
  expect(await shrinkNativeImageShardPlannedV1(shrinker(directory,log,{platform:"linux"}) as never)).toEqual({kind:"none"});
 }finally{rmSync(directory,{recursive:true,force:true});}
});
test("a planned shrink that fails for another reason raises and logs nothing",async()=>{
 const directory=mkdtempSync(join(testScratchRootV1("growth"),"shrink-fail-"));
 try{born(directory,30*GiB);const log:string[]=[];await expect(shrinkNativeImageShardPlannedV1(shrinker(directory,log,{lifecycle:fakeLifecycle(log,Error("boom"))}) as never)).rejects.toThrow("boom");expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory)).get("lane1")!.capacityBytes).toBe(30*GiB);}
 finally{rmSync(directory,{recursive:true,force:true});}
});
test.skipIf(process.platform!=="darwin")("on macOS a real image that declares more than it should is shrunk in place with its files and its mount, and the smaller capacity is logged",async()=>{
 const directory=realpathSync(mkdtempSync(join(testScratchRootV1("growth"),"shrink-real-"))),home=nativeImageShardHomeLayoutV1(directory),lifecycle=createNativeImageShardDarwinLifecycleV1({run:async(program,args)=>run(program,args)}),lock=serial();
 const small:NativeImageShardRuleV1={enabled:true,maximumImageBytes:4*GiB,headroomPercent:0,hostFreeReservePercent:90,roots:[{prefix:"u/l",pattern:"^[a-z0-9]{1,12}$",needBytes:256*MiB}]};
 try{
  mkdirSync(home.mountPoint,{recursive:true});
  const seed=join(directory,"seed");mkdirSync(seed);run("/usr/bin/hdiutil",["create","-size","1280m","-format","UDRW","-nospotlight","-fs","Case-sensitive APFS","-volname","semio","-srcfolder",seed,home.image]);rmSync(seed,{recursive:true});
  run("/usr/sbin/diskutil",["image","attach","--nobrowse","--mountOptions","owners","--mountPoint",home.mountPoint,home.image]);
  const made=await ensureNativeImageShardV1({directory,rule:small,platform:"darwin",path:"u/l/lane1/x",hostFreeBytes:()=>64*GiB,now:()=>1,lifecycle,lock});
  if(made.kind!=="birth")throw Error("Birth required");const layout=made.layout;
  await lifecycle.resize(layout,3*GiB,home);appendNativeImageShardEventV1(directory,{type:"grown",id:"lane1",capacityBytes:3*GiB,atMs:2});
  writeFileSync(join(layout.mountPoint,"keep"),Buffer.alloc(8*MiB,5));
  const df=()=>Number(run("/bin/df",["-k",layout.mountPoint]).split("\n")[1]!.split(/\s+/u)[1])*1024;expect(df()).toBeGreaterThan(2.5*GiB);
  const out=await shrinkNativeImageShardPlannedV1({directory,rule:small,platform:"darwin",id:"lane1",measure:()=>({usedBytes:8*MiB,hostFreeBytes:24*GiB}),available:()=>0,now:()=>3,lifecycle,lock});
  expect(out).toEqual({kind:"shrunk",fromBytes:3*GiB,capacityBytes:GiB,usedBytes:8*MiB});
  expect(nativeImageShardMountedV1(layout.mountPoint)).toBe(true);expect(df()).toBeLessThan(1.1*GiB);expect(df()).toBeGreaterThan(0.8*GiB);
  expect(readFileSync(join(layout.mountPoint,"keep")).equals(Buffer.alloc(8*MiB,5))).toBe(true);
  expect(JSON.parse(readFileSync(layout.receipt,"utf8")).capacityBytes).toBe(GiB);expect(nativeImageShardFoldV1(readNativeImageShardLedgerV1(directory)).get("lane1")!.capacityBytes).toBe(GiB);
  writeFileSync(join(layout.mountPoint,"after"),"writable");
 }finally{for(const path of[join(home.mountPoint,"u/l/lane1"),home.mountPoint])spawnSync("/usr/bin/hdiutil",["detach",path,"-force"],{env});rmSync(directory,{recursive:true,force:true});}
},600000);
