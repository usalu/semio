import {afterAll,expect,test} from "bun:test";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,statSync,truncateSync,writeFileSync} from "node:fs";
import {spawnSync} from "node:child_process";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {cacheGrowArgumentsV1,runCacheGrowV1} from "../⌨️command/🟦️.ts";
import {GrowRefusalV1,growPartitionV1,growReceiptTextV1,growTargetBytesV1,growTextV1,readGrowJournalV1,realGrowToolsV1,restampChildReceiptTextV1,restampJournalTextV1,type GrowAttachedV1,type GrowPortsV1,type GrowToolsV1} from "../🟦️.ts";

const MIB=1048576,FROM=64*MIB,TO=256*MIB,roots:string[]=[],oracle=join(import.meta.dir,"🐍️.py");
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const scratch=()=>{const parent=process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir();mkdirSync(parent,{recursive:true});const root=mkdtempSync(join(realpathSync(parent),"gr-"));roots.push(root);return root;};
const ask=(command:"target"|"restamp",cases:readonly unknown[])=>{const file=join(scratch(),"cases.json");writeFileSync(file,JSON.stringify({cases}));const run=spawnSync("python3",["-I",oracle,command,file],{encoding:"utf8",maxBuffer:1<<26});expect(run.status,run.stderr).toBe(0);return JSON.parse(run.stdout) as unknown[];};

type World={attached:boolean;children:string[];device:number;nextDevice:number;holders:string[];detachFails:boolean};
/** 🧪️ A partition directory as the engine leaves it: receipt, backing file, mount directory with the storage journal and the receipt of one nested image. */
function partition(from=FROM){
 const directory=join(scratch(),"fixed"),mount=join(directory,"m"),child=join(mount,"cs","v");mkdirSync(child,{recursive:true});
 writeFileSync(join(directory,"fixed.dmg"),"");truncateSync(join(directory,"fixed.dmg"),from);
 writeFileSync(join(directory,"image.json"),JSON.stringify({version:1,mountDirectory:{dev:"7",ino:"8",mode:16877},bootSession:"B",dev:7,ino:9,size:from,ancestry:["1","2"],filesystemBytes:from,maximumPhysicalBytes:from+MIB}));
 writeFileSync(join(mount,"storage-events.jsonl"),[
  JSON.stringify({type:"reserved",allocation:{id:"a",owner:"o",kind:"compiler-workspace",bytes:4096,phase:"reserved"}}),
  JSON.stringify({type:"reserved",allocation:{id:"c",owner:"p",kind:"compiler-state",bytes:4096,phase:"reserved",child:{directory:child,filesystemBytes:4096,maximumPhysicalBytes:8192,policySha256:"x",version:1,allocation:"c",parentIdentity:`[7,9,${from},100]`}}}),
  JSON.stringify({type:"namespace-created",id:"c",namespace:{directory:join(mount,"cs"),bytes:65536,parent:{dev:"100",ino:"2",mode:16832},entry:{dev:"100",ino:"5",mode:16832}}}),""].join("\n"));
 writeFileSync(join(child,"image.json"),JSON.stringify({version:1,mountDirectory:{dev:"100",ino:"6",mode:16832},dev:100,ino:7,size:4096,ancestry:["5","2"],filesystemBytes:4096,maximumPhysicalBytes:8192}));
 return {directory,mount,child,image:join(directory,"fixed.dmg")};
}
/** 🧪️ A model of the host: the roster, the holders, detach, offline resize and attach with a fresh mount device each time. */
function host(layout:ReturnType<typeof partition>,state:World):GrowToolsV1{
 const rows=():readonly GrowAttachedV1[]=>[...(state.attached?[{imagePath:layout.image,mountPoints:[layout.mount],device:"/dev/main"}]:[]),...state.children.map(path=>({imagePath:path,mountPoints:[],device:"/dev/child"}))];
 return {roster:rows,holders:()=>state.holders,
  detach:row=>{if(state.detachFails)throw Error("busy");if(row.imagePath===layout.image)state.attached=false;else state.children=state.children.filter(path=>path!==row.imagePath);},
  resize:(image,bytes)=>{if(state.attached)throw Error("Resource temporarily unavailable");truncateSync(image,bytes);},
  attach:()=>{state.attached=true;state.device=state.nextDevice++;},
  deviceOf:path=>path===layout.mount&&state.attached?state.device:7,
  volumeBytes:()=>statSync(layout.image).size-MIB};
}
const freshWorld=():World=>({attached:true,children:[],device:100,nextDevice:101,holders:[],detachFails:false});
function ports(layout:ReturnType<typeof partition>,state:World,counts:{released:number},crash?:{name:string;nth:number},seen:string[]=[]):GrowPortsV1{
 const hits=new Map<string,number>();
 return {lease:async()=>({release:()=>{counts.released++;}}),tools:host(layout,state),checkpoint:name=>{seen.push(name);const n=(hits.get(name)??0)+1;hits.set(name,n);if(crash&&crash.name===name&&crash.nth===n)throw Error("crash at "+name);}};
}
const snapshot=(layout:ReturnType<typeof partition>)=>JSON.stringify({receipt:readFileSync(join(layout.directory,"image.json"),"utf8"),events:readFileSync(join(layout.mount,"storage-events.jsonl"),"utf8"),child:readFileSync(join(layout.child,"image.json"),"utf8"),size:statSync(layout.image).size,journal:existsSync(join(layout.directory,"grow.json"))});

test("a growth detaches, resizes offline, attaches again and re-stamps receipt, journal and nested receipt",async()=>{
 const layout=partition(),state=freshWorld(),counts={released:0},lines:string[]=[];
 state.children=[join(layout.mount,"cs","v","fixed.dmg")];
 const outcome=await growPartitionV1({partition:"rust",directory:layout.directory,targetBytes:TO,language:"en"},{...ports(layout,state,counts),report:line=>lines.push(line)});
 expect(outcome).toEqual({partition:"rust",fromBytes:FROM,toBytes:TO,resumed:false,oldDevice:100,newDevice:101});
 expect(state.attached).toBe(true);expect(state.children).toEqual([]);expect(counts.released).toBe(1);
 expect(statSync(layout.image).size).toBe(TO);expect(existsSync(join(layout.directory,"grow.json"))).toBe(false);
 const receipt=JSON.parse(readFileSync(join(layout.directory,"image.json"),"utf8"));expect(receipt).toMatchObject({size:TO,filesystemBytes:TO,maximumPhysicalBytes:TO+MIB,ino:9,dev:7,bootSession:"B"});
 const events=readFileSync(join(layout.mount,"storage-events.jsonl"),"utf8").split("\n").filter(Boolean).map(line=>JSON.parse(line));
 expect(events[1].allocation.child.parentIdentity).toBe(`[7,9,${TO},101]`);expect(events[2].namespace.parent.dev).toBe("101");expect(events[2].namespace.entry.dev).toBe("101");expect(events[0].allocation.bytes).toBe(4096);
 expect(JSON.parse(readFileSync(join(layout.child,"image.json"),"utf8"))).toMatchObject({dev:101,mountDirectory:{dev:"101"},size:4096});
 expect(lines.length).toBeGreaterThan(4);expect(lines.join("\n")).toContain("grown from");
});

test("a growth killed at any effect resumes to exactly the state of an uninterrupted run, releases the lease every time and never leaves the image attached to a stale record",async()=>{
 const reference=partition(),seen:string[]=[];await growPartitionV1({partition:"rust",directory:reference.directory,targetBytes:TO,language:"en"},ports(reference,freshWorld(),{released:0},undefined,seen));
 const expected=snapshot(reference),names=[...new Set(seen)];expect(names.length).toBeGreaterThan(15);
 let crashes=0;
 for(const name of names)for(let nth=1;nth<=seen.filter(item=>item===name).length;nth++){
  const layout=partition(),state=freshWorld(),counts={released:0};
  await expect(growPartitionV1({partition:"rust",directory:layout.directory,targetBytes:TO,language:"en"},ports(layout,state,counts,{name,nth}))).rejects.toThrow("crash at "+name);crashes++;
  const held=name==='lease:before'?0:1;expect(counts.released,name).toBe(held);
  const journal=readGrowJournalV1(layout.directory);
  if(journal)expect(journal).toMatchObject({fromBytes:FROM,toBytes:TO});
  const again=growPartitionV1({partition:"rust",directory:layout.directory,targetBytes:TO,language:"en"},ports(layout,state,counts));
  if(name==="journal:clear:after")await expect(again).rejects.toThrow("already has the declared size");else expect((await again).toBytes,name).toBe(TO);
  expect(counts.released,name).toBe(held+1);
  const final=JSON.parse(snapshot(layout)),want=JSON.parse(expected);
  expect(final.receipt,name).toBe(want.receipt);expect(final.size,name).toBe(want.size);expect(final.journal,name).toBe(false);
  expect(JSON.parse(final.child),name).toMatchObject({dev:state.device,size:4096});
  expect(final.events.includes(`${TO},${state.device}]`),name).toBe(true);
 }
 expect(crashes).toBe(names.reduce((sum,name)=>sum+seen.filter(item=>item===name).length,0));
});

test("a busy image, a nested image that will not detach and a backing file of another size are refused with their reasons and keep the image usable",async()=>{
 const busy=partition(),busyState=freshWorld();busyState.holders=["cargo 123 ueli"];let clock=0;const fast={sleep:async(milliseconds:number)=>{clock+=milliseconds;},now:()=>clock};
 await expect(growPartitionV1({partition:"rust",directory:busy.directory,targetBytes:TO,language:"en"},{...ports(busy,busyState,{released:0}),...fast})).rejects.toThrow("in use");expect(clock).toBeGreaterThanOrEqual(300000);
 const clears=partition(),clearState=freshWorld();clearState.holders=["bun 1 gc.log"];let polls=0;clock=0;const lines:string[]=[];
 const slow={...ports(clears,clearState,{released:0}),sleep:async(milliseconds:number)=>{clock+=milliseconds;if(++polls===3)clearState.holders=[];},now:()=>clock,report:(line:string)=>lines.push(line)};
 await growPartitionV1({partition:"rust",directory:clears.directory,targetBytes:TO,language:"en"},slow);expect(polls).toBe(3);expect(statSync(clears.image).size).toBe(TO);expect(lines.filter(line=>line.includes("still have files open")).length).toBe(3);
 expect(busyState.attached).toBe(true);expect(statSync(busy.image).size).toBe(FROM);expect(existsSync(join(busy.directory,"grow.json"))).toBe(false);
 const stuck=partition(),stuckState=freshWorld();stuckState.children=[join(stuck.mount,"cs","v","fixed.dmg")];stuckState.detachFails=true;
 await expect(growPartitionV1({partition:"rust",directory:stuck.directory,targetBytes:TO,language:"en"},{...ports(stuck,stuckState,{released:0}),sleep:async()=>{}})).rejects.toThrow("cannot be detached");
 expect(statSync(stuck.image).size).toBe(FROM);
 const odd=partition();truncateSync(odd.image,FROM+MIB);
 await expect(growPartitionV1({partition:"rust",directory:odd.directory,targetBytes:TO,language:"en"},ports(odd,freshWorld(),{released:0}))).rejects.toBeInstanceOf(GrowRefusalV1);
 await expect(growPartitionV1({partition:"rust",directory:join(scratch(),"none"),targetBytes:TO,language:"en"},ports(odd,freshWorld(),{released:0}))).rejects.toThrow("no image with a receipt");
 const attachedResize=partition(),world=freshWorld();expect(()=>host(attachedResize,world).resize(attachedResize.image,TO)).toThrow("temporarily unavailable");
});

test("a resumed growth re-reads the mount device and re-checks the holders before it detaches: neither a stale oldDevice nor a stale drained phase is trusted",async()=>{
 const layout=partition(),state=freshWorld();state.device=555;
 const eventsPath=join(layout.mount,"storage-events.jsonl"),childPath=join(layout.child,"image.json");writeFileSync(eventsPath,restampJournalTextV1(readFileSync(eventsPath,"utf8"),{size:FROM,device:100},{size:FROM,device:555}));writeFileSync(childPath,restampChildReceiptTextV1(readFileSync(childPath,"utf8"),100,555));
 writeFileSync(join(layout.directory,"grow.json"),JSON.stringify({version:1,phase:"drained",partition:"rust",fromBytes:FROM,toBytes:TO,oldDevice:100,newDevice:null}));
 state.holders=["bun 9 files"];let clock=0;const calls={holders:0};const tools=host(layout,state);
 await expect(growPartitionV1({partition:"rust",directory:layout.directory,targetBytes:TO,language:"en"},{lease:async()=>({release(){}}),tools:{...tools,holders:()=>{calls.holders++;return state.holders;}},sleep:async(milliseconds:number)=>{clock+=milliseconds;},now:()=>clock})).rejects.toThrow("in use");
 expect(calls.holders).toBeGreaterThan(1);expect(readGrowJournalV1(layout.directory)).toMatchObject({phase:"drained",oldDevice:555});
 state.holders=[];await growPartitionV1({partition:"rust",directory:layout.directory,targetBytes:TO,language:"en"},{lease:async()=>({release(){}}),tools});
 const events=readFileSync(join(layout.mount,"storage-events.jsonl"),"utf8");expect(events).toContain(`${TO},${state.device}]`);
});

test("a detach that is busy for a moment is retried and succeeds, one that stays busy is refused with the open files",async()=>{
 const layout=partition(),state=freshWorld(),tools=host(layout,state);let busy=2,sleeps=0;
 await growPartitionV1({partition:"rust",directory:layout.directory,targetBytes:TO,language:"en"},{lease:async()=>({release(){}}),tools:{...tools,detach:row=>{if(busy-->0)throw Error("Resource busy");tools.detach(row);}},sleep:async()=>{sleeps++;}});
 expect(sleeps).toBe(2);expect(statSync(layout.image).size).toBe(TO);
 const stuck=partition(),stuckState=freshWorld(),stuckTools=host(stuck,stuckState);let calls=0;
 await expect(growPartitionV1({partition:"rust",directory:stuck.directory,targetBytes:TO,language:"en"},{lease:async()=>({release(){}}),tools:{...stuckTools,detach:()=>{throw Error("Resource busy");},holders:()=>(calls++?["cargo 9 x"]:[])},sleep:async()=>{},now:()=>0})).rejects.toThrow("[open: cargo 9 x]");
});

test("a detach blocked only by allowlisted, system-signed scanners is force-unmounted once under the lease, logged in the journal, the outcome and an audit record; any other dissenter, path, signature or an unreadable lsof still refuses",async()=>{
 const SYS="/usr/libexec/syspolicyd";
 const run=async(dissenters:{pid:number;path:string}[],options:{holders?:string[];signed?:boolean;lsofFails?:boolean}={})=>{
  const layout=partition(),state=freshWorld(),tools=host(layout,state);let forced=0,calls=0;
  const pending=growPartitionV1({partition:"rust",directory:layout.directory,targetBytes:TO,language:"en"},{lease:async()=>({release(){}}),tools:{...tools,holders:()=>{if(calls++===0)return[];if(options.lsofFails&&state.attached&&!forced)throw Error("lsof failed");return state.attached&&!forced?options.holders??[]:[];},detach:row=>{if(!forced)throw Error("Resource busy");tools.detach(row);},dissenters:()=>dissenters,signedBySystem:()=>options.signed??true,forceUnmount:()=>{forced++;}},sleep:async()=>{},now:()=>0});pending.catch(()=>undefined);const attempt=pending;
  return{layout,attempt,counts:()=>forced};
 };
 const ok=await run([{pid:274,path:SYS}]);const outcome=await ok.attempt;expect(ok.counts()).toBe(1);expect(statSync(ok.layout.image).size).toBe(TO);
 expect(outcome.forcedUnmount?.[0]).toContain("274:"+SYS);const audit=readFileSync(join(ok.layout.directory,"grow-audit.jsonl"),"utf8").trim().split("\n").map(line=>JSON.parse(line));expect(audit).toHaveLength(1);expect(audit[0].forcedUnmount).toEqual(outcome.forcedUnmount);
 for(const [label,attempt] of [["a name of the allowlist at another path",await run([{pid:9,path:"/tmp/mds"}])],["a mix with another process",await run([{pid:274,path:SYS},{pid:9,path:"/usr/bin/Finder"}])],["no dissenter",await run([])],["an unsigned copy at the allowlisted path",await run([{pid:274,path:SYS}],{signed:false})],["an open file",await run([{pid:274,path:SYS}],{holders:["cargo 9 x"]})],["an lsof that fails",await run([{pid:274,path:SYS}],{lsofFails:true})]] as const){
  await expect(attempt.attempt,label).rejects.toThrow("cannot be detached");expect(attempt.counts(),label).toBe(0);
 }
});

test("a background collector that keeps files open is asked to stop under the lease and the growth goes on once it has exited",async()=>{
 const layout=partition(),state=freshWorld(),stopped:number[]=[];state.holders=["bun 77 gc.log"];const tools=host(layout,state),lines:string[]=[];let clock=0;
 await growPartitionV1({partition:"rust",directory:layout.directory,targetBytes:TO,language:"en"},{lease:async()=>({release(){}}),tools:{...tools,stopCollectors:()=>{stopped.push(77);return[77];},holders:()=>stopped.length&&clock>=10000?[]:state.holders},sleep:async(milliseconds:number)=>{clock+=milliseconds;},now:()=>clock,report:line=>lines.push(line)});
 expect(stopped).toEqual([77]);expect(statSync(layout.image).size).toBe(TO);expect(lines.some(line=>line.includes("collector 77"))).toBe(true);
});

test("receipt, journal and nested receipt rewrites are idempotent and refuse a receipt of another size",()=>{
 const text=JSON.stringify({version:1,size:FROM,filesystemBytes:FROM,maximumPhysicalBytes:FROM+MIB,ino:1}),grown=growReceiptTextV1(text,FROM,TO);
 expect(JSON.parse(grown)).toEqual({version:1,size:TO,filesystemBytes:TO,maximumPhysicalBytes:TO+MIB,ino:1});expect(growReceiptTextV1(grown,FROM,TO)).toBe(grown);
 expect(()=>growReceiptTextV1(text,FROM+MIB,TO)).toThrow("receipt does not describe");expect(()=>growReceiptTextV1("{",FROM,TO)).toThrow("receipt does not describe");
 const line=JSON.stringify({a:{parentIdentity:`[1,2,${FROM},100]`,dev:"100"}}),once=restampJournalTextV1(line,{size:FROM,device:100},{size:TO,device:101});
 expect(restampJournalTextV1(once,{size:FROM,device:100},{size:TO,device:101})).toBe(once);expect(restampJournalTextV1(line,{size:FROM,device:null},{size:TO,device:null})).toContain(`${TO},100`);
 const same=JSON.stringify({dev:"5"});expect(restampJournalTextV1(same,{size:1,device:100},{size:2,device:101})).toBe(same);
 expect(restampChildReceiptTextV1(JSON.stringify({dev:100}),100,100)).toBe(JSON.stringify({dev:100}));expect(restampChildReceiptTextV1("{",100,101)).toBe("{");
 expect(growTextV1("de","done",{partition:"rust",from:1,to:2})).toContain("vergrößert");expect(()=>growTextV1("en","nope")).toThrow("Declared grow text");
});

test("the target size equals the independent Python model on 300 seeded bounds",()=>{
 let seed=11;const random=(n:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%n;};
 const cases=Array.from({length:300},()=>{const current=(1+random(64))*1024*MIB,base={currentBytes:current,declaredBytes:current+(random(4)?random(96)*1024*MIB:0),ruleMaximumBytes:current+random(128)*1024*MIB,hostFreeBytes:random(200)*1024*MIB+random(MIB),hostFreeReservePercent:random(90)};
  return random(3)===0?{...base,requestedBytes:current+(random(5)?random(80)*1024*MIB:0)+(random(7)===0?1:0)}:base;});
 const want=ask("target",cases) as unknown,got=cases.map(row=>{try{return {targetBytes:growTargetBytesV1(row).targetBytes};}catch(error){return {refused:(error as GrowRefusalV1).reason};}});
 expect(got as unknown).toEqual(want);
});

test("the journal re-stamping equals the independent Python model on 200 seeded journals",()=>{
 let seed=23;const random=(n:number)=>{seed=(seed*1103515245+12345)%2147483648;return seed%n;};
 const cases=Array.from({length:200},()=>{const size=(1+random(50))*MIB,device=random(5)?100+random(900):null,next=random(5)?1000+random(900):null,lines:string[]=[];
  for(let i=0;i<1+random(5);i++){const dev=random(2)?100+random(900):device??7;lines.push(JSON.stringify({type:"x",allocation:{child:{parentIdentity:`[${random(9)},${random(9)},${random(2)?size:size+1},${random(2)&&device!==null?device:dev}]`}},namespace:{parent:{dev:String(random(2)&&device!==null?device:dev),ino:"2"},entry:{dev:random(2)&&device!==null?device:dev}},note:"ü"}));}
  return {from:{size,device},to:{size:size*4,device:next},lines};});
 const want=ask("restamp",cases) as string[][];
 cases.forEach((row,index)=>{const got=restampJournalTextV1(row.lines.join("\n"),row.from,row.to).split("\n").map(line=>JSON.parse(line));expect(got).toEqual(want[index]!.map(line=>JSON.parse(line)));});
});

test.skipIf(process.platform!=="darwin")("a real seeded APFS image grows offline: detached, resized, attached again, volume larger, records re-stamped",async()=>{
 const root=scratch(),directory=join(root,"fixed"),mount=join(directory,"m"),seed=join(root,"seed"),image=join(directory,"fixed.dmg");mkdirSync(seed,{recursive:true});mkdirSync(mount,{recursive:true});writeFileSync(join(seed,"seed.txt"),"x");
 const created=spawnSync("/usr/bin/hdiutil",["create","-size","64m","-format","UDRW","-nospotlight","-fs","Case-sensitive APFS","-volname","semio","-srcfolder",seed,image],{encoding:"utf8"});
 expect(created.status,created.stderr).toBe(0);
 const tools=realGrowToolsV1(),from=statSync(image).size;let attachedByTest=false;
 try{
  await tools.attach(image,mount);attachedByTest=true;
  const device=await tools.deviceOf(mount);
  writeFileSync(join(directory,"image.json"),JSON.stringify({version:1,mountDirectory:{dev:"7",ino:"8",mode:16877},bootSession:"B",dev:7,ino:statSync(image).ino,size:from,ancestry:["1"],filesystemBytes:from,maximumPhysicalBytes:from+MIB}));
  writeFileSync(join(mount,"storage-events.jsonl"),JSON.stringify({type:"reserved",allocation:{id:"c",child:{directory:join(mount,"cs","v"),parentIdentity:`[7,${statSync(image).ino},${from},${device}]`}}})+"\n");
  mkdirSync(join(mount,"cs","v"),{recursive:true});writeFileSync(join(mount,"cs","v","image.json"),JSON.stringify({dev:device,mountDirectory:{dev:String(device)}}));
  await expect(Promise.resolve().then(()=>tools.resize(image,TO))).rejects.toThrow();
  const counts={released:0},outcome=await growPartitionV1({partition:"rust",directory,targetBytes:TO,language:"en"},{lease:async()=>({release:()=>{counts.released++;}}),tools});
  expect(outcome).toMatchObject({fromBytes:from,toBytes:TO,resumed:false,oldDevice:device});expect(counts.released).toBe(1);
  expect(statSync(image).size).toBe(TO);expect(await tools.volumeBytes(mount)).toBeGreaterThan(TO*0.9);expect((await tools.holders(mount)).length).toBeGreaterThanOrEqual(0);
  expect(readFileSync(join(mount,"seed.txt"),"utf8")).toBe("x");
  const now=await tools.deviceOf(mount),events=JSON.parse(readFileSync(join(mount,"storage-events.jsonl"),"utf8").trim());
  expect(events.allocation.child.parentIdentity).toBe(`[7,${statSync(image).ino},${TO},${now}]`);expect(JSON.parse(readFileSync(join(mount,"cs","v","image.json"),"utf8")).dev).toBe(now);
  expect(JSON.parse(readFileSync(join(directory,"image.json"),"utf8"))).toMatchObject({size:TO,filesystemBytes:TO,maximumPhysicalBytes:TO+MIB});
  expect(existsSync(join(directory,"grow.json"))).toBe(false);
 }finally{
  const left=(await tools.roster()).filter(entry=>entry.imagePath===image);if(attachedByTest||left.length)for(const row of left){try{await tools.detach(row);}catch{}}
 }
},180000);

test("the command bounds the target by the policy, the host reserve and the partition list, plans in a dry run and grows through the injected lease and tools",async()=>{
 const GIB=1024*MIB,workspace=scratch(),directory=join(workspace,".🧬semio","🦑️repo","⚡️cache","rust","fixed"),mount=join(directory,"m");mkdirSync(mount,{recursive:true});
 const from=40*GIB;writeFileSync(join(directory,"fixed.dmg"),"");truncateSync(join(directory,"fixed.dmg"),from);
 writeFileSync(join(directory,"image.json"),JSON.stringify({version:1,size:from,filesystemBytes:from,maximumPhysicalBytes:from+MIB,ino:1,dev:1}));
 const layout={directory,mount,child:join(mount,"cs","v"),image:join(directory,"fixed.dmg")},state=freshWorld(),lines:string[]=[],counts={released:0};
 const context=(free:number|undefined)=>({workspace,write:(line:string)=>lines.push(line),freeBytes:()=>free,lease:async()=>({release:()=>{counts.released++;}}),tools:host(layout,state)});
 expect(cacheGrowArgumentsV1(["grow","--partition=rust","--bytes=1048576","--dry-run"])).toEqual({partition:"rust",bytes:1048576,dryRun:true,stopCollectors:true});
 expect(()=>cacheGrowArgumentsV1(["grow"])).toThrow("only the partitions");expect(()=>cacheGrowArgumentsV1(["grow","--partition=rust","--partition=rust"])).toThrow();expect(()=>cacheGrowArgumentsV1(["grow","--partition=rust","--bytes=-1"])).toThrow();
 expect(await runCacheGrowV1(["grow","--partition=tools"],context(500*GIB),"en")).toBe(1);expect(lines.at(-1)).toContain("only the partitions");
 expect(await runCacheGrowV1(["grow","--partition=rust"],context(MIB),"en")).toBe(1);expect(lines.at(-1)).toContain("bounded to");
 expect(await runCacheGrowV1(["grow","--partition=rust","--bytes="+String(100*GIB)],context(500*GIB),"en")).toBe(1);expect(lines.at(-1)).toContain("exceeds the declared size");
 lines.length=0;expect(await runCacheGrowV1(["grow","--partition=rust","--dry-run"],context(500*GIB),"de")).toBe(0);expect(lines.join("\n")).toContain("vergrößern");expect(statSync(layout.image).size).toBe(from);expect(counts.released).toBe(0);
 lines.length=0;expect(await runCacheGrowV1(["grow","--partition=rust"],context(500*GIB),"en")).toBe(0);expect(counts.released).toBe(1);
 const declared=statSync(layout.image).size;expect(declared).toBeGreaterThan(from);expect(declared%MIB).toBe(0);expect(JSON.parse(readFileSync(join(directory,"image.json"),"utf8")).filesystemBytes).toBe(declared);
 expect(await runCacheGrowV1(["grow","--partition=rust"],context(500*GIB),"en")).toBe(1);expect(lines.at(-1)).toContain("already has the declared size");
});

test.skipIf(process.platform==="win32")("the production tools are cancellable and time-bounded: an aborted signal kills the running program at once and a tool that outlives its timeout is killed and refused",async()=>{
 const controller=new AbortController(),tools=realGrowToolsV1(controller.signal);
 controller.abort(new Error("cancelled"));
 await expect(Promise.resolve(tools.holders("/nonexistent"))).rejects.toThrow("cancelled");
 const running=new AbortController(),pending=realGrowToolsV1(running.signal).roster();setTimeout(()=>running.abort(new Error("stopped midway")),1);
 await expect(Promise.resolve(pending)).rejects.toThrow("stopped midway");
});
