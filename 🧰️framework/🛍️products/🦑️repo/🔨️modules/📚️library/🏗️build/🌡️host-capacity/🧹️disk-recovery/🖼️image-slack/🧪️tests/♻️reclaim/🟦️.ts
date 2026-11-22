/** ♻️ Contract: the image-slack collector returns blocks in place first (UNMAP reclaim, nothing recreated), retires an elastic image only when the reclaim could not, never retires an image it just reclaimed, reclaims inelastic images too, names every outcome in both languages and stops at the target; on a real synthetic image the blocks come back and the unit cache content is intact. */
import{afterAll,expect,spyOn,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{createHash}from"node:crypto";
import{existsSync,lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,truncateSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{basename,join,parse}from"node:path";
import fixture from "./🔣️receipts.json" with{type:"json"};
import{readImageReclaimReceiptV1,realDiskWritersV1}from"../../../🔌️real/🟦️.ts";
import{DISK_WRITERS_V1}from"../../../🟦️.ts";
import*as imageSlack from"../../🟦️.ts";
import project from"../../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📋️project.json"with{type:"json"};
import{IMAGE_SLACK_V1,collectImageSlackV1,describeImageSlackV1,type ImageOutcomeV1,type ImageReclaimOutcomeV1,type ImageRowV1}from"../../🟦️.ts";
import{createNativeImageDarwinReclaimPortV1,nativeImageReclaimDueV1,nativeImageReclaimMechanismV1,reclaimNativeImageV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🟦️.ts";

import {Database} from "bun:sqlite";
import {nativeImageVolumeUsedBytesSyncV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/⚖️availability/🟦️.ts";
import ts from "typescript";
import {ResourceLeaseTimeoutErrorV1} from "../../../../../../../../../../🔨️modules/🏃️process/🔒️leases/🟦️.ts";
import {imageRecoverySkipReasonV1} from "../../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts";
import {repositoryPartitionReclaimRuleV1} from "../../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🟦️.ts";

const GIB=1024**3,MIB=1024**2,roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const row=(id:string,allocatedGiB:number,usedGiB:number,mounted=true):ImageRowV1=>({id,directory:"/ws/.cache/b/"+id,mounted,allocatedBytes:allocatedGiB*GIB,usedBytes:usedGiB*GIB,slackBytes:mounted?Math.max(0,(allocatedGiB-usedGiB)*GIB-IMAGE_SLACK_V1.filesystemOverheadBytes):0});

test("mounted APFS occupancy is independent of host-limited statfs availability",async()=>{
 const tree=ts.createSourceFile("measure.ts",readFileSync(new URL("../../🟦️.ts",import.meta.url),"utf8"),ts.ScriptTarget.Latest,true);
 const method=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="measureImagesV1")!.getText(tree).replace(/^export /u,"");
 const code=ts.transpileModule(method,{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 const measure=new Function("machinePathV1","lstatSync","statfsSync","nativeImageVolumeUsedBytesSyncV1","join",code+";return measureImagesV1;");
 const data=fixture.apfsOccupancy,root="/cache/r/m",directory="/cache/r";
 const statfs={bsize:GIB,blocks:data.volumeGiB,bfree:data.hostFreeGiB,bavail:data.hostFreeGiB};
 expect(nativeImageVolumeUsedBytesSyncV1(root,statfs,"linux",()=>{throw Error("non-Darwin must use native statfs");})).toBe((data.volumeGiB-data.hostFreeGiB)*GIB);
 using oracle=new Database(":memory:");
 const expected=oracle.query("select coalesce(?1=?2 and ?3='apfs' and ?4 between 1 and ?5 and ?6-?4-0.0625>=max(1,?6/10.0),0) accepted");
 for(const sample of data.cases){
  const plist=sample.reportedMount===null?undefined:'<?xml version="1.0"?><plist version="1.0"><dict>'
   +'<key>MountPoint</key><string>'+sample.reportedMount+'</string><key>FilesystemType</key><string>'+sample.filesystem+'</string>'
   +'<key>CapacityInUse</key><integer>'+String(sample.usedGiB*GIB)+'</integer></dict></plist>';
  const occupied=(path:string,volume:typeof statfs)=>nativeImageVolumeUsedBytesSyncV1(path,volume,"darwin",(program,args)=>{
   expect(program).toBe("/usr/sbin/diskutil");expect(args).toEqual(["info","-plist",root]);return plist;
  });
  const observations=measure((_workspace:string,path:string)=>path,(path:string)=>({
   dev:path===root?9:4,nlink:1,blocks:data.allocatedGiB*GIB/512,isDirectory:()=>!path.endsWith("fixed.dmg"),isFile:()=>path.endsWith("fixed.dmg"),isSymbolicLink:()=>false,
  }),()=>statfs,occupied,join)("/repo",IMAGE_SLACK_V1,{rust:{directory}}) as ImageRowV1[];
  const sql=expected.get(sample.reportedMount,root,sample.filesystem,sample.usedGiB,data.volumeGiB,data.allocatedGiB) as {accepted:number};
  expect(Boolean(sql.accepted)).toBe(sample.expected);
  expect(observations[0]?.mounted).toBe(true);
  let effects=0,free=data.hostFreeGiB*GIB;
  const result=await collectImageSlackV1({rows:observations,targetFreeBytes:data.pressureTargetGiB*GIB,freeBytes:()=>free,language:"en",pressureDevice:4,reclaim:async()=>{effects++;free=data.pressureTargetGiB*GIB;return{kind:"reclaimed"};},retire:async()=>({kind:"held"})});
  expect(effects).toBe(sql.accepted);
  expect(result.reclaimed).toEqual(sample.expected?["rust"]:[]);
 }
});

test("the pressured planner and non-waiting executor agree on current mounted slack",async()=>{
 const source=readFileSync(new URL("../../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🟦️.ts",import.meta.url),"utf8");
 const tree=ts.createSourceFile("reclaim.ts",source,ts.ScriptTarget.Latest,true);
 const body=tree.statements.find(node=>ts.isFunctionDeclaration(node)&&node.name?.text==="reclaimImageIfQuiescentV1")!.getText(tree).replace(/^export /u,"");
 const code=ts.transpileModule(body,{compilerOptions:{target:ts.ScriptTarget.ES2022}}).outputText;
 using oracle=new Database(":memory:");
 const admissible=oracle.query("select case when ?1 then ?2 < 24 and ?3 - ?4 - 0.0625 >= max(1,?3/10.0) and not (?5 or ?6 or ?7 or ?8 or ?9 or ?10) else (?2 < 48 or ?3 >= 3*?4) and ?3 - ?4 >= (case when ?2 < 48 then 1 else 8 end) end as yes");
 const directory="/cache/r",root=directory+"/m",image=directory+"/fixed.dmg";
 for(const sample of fixture.pressureCases){
  const {pressure,allocated,used,free}=sample;
  const expected=admissible.get(+pressure,free,allocated,used,+!!sample.busy,+!!sample.wrongDevice,+!!sample.detached,+!!sample.retiring,+!!sample.cancel,+!!(sample.leaseFailed||sample.occupancyUnavailable||sample.cycleFailed||sample.releaseFailed)) as {yes:number};
  expect(Boolean(expected.yes),sample.name).toBe(sample.effect);
  const stop=new AbortController(),control={signal:stop.signal};
  const backingDevice=sample.wrongDevice?7:4,mountDevice=sample.detached?backingDevice:9;
  const entry=(dev:number,file:boolean)=>({dev,nlink:1,blocks:allocated*GIB/512,isFile:()=>file,isDirectory:()=>!file,isSymbolicLink:()=>false});
  let effects=0,held=false,requests=0,released=0,routine=0,reported: number=14*GIB;
  const ports={
   process:{platform:"darwin"},join,nativeImageReclaimMechanismV1,imageRecoverySkipReasonV1,ResourceLeaseTimeoutErrorV1,
   imageSlack:IMAGE_SLACK_V1,requireIssuedResourceControlInvocationV1:()=>undefined,checkResourceLeaseControl:()=>stop.signal.throwIfAborted(),
   callerRepositoryRootV1:()=>"/repo",repositoryCacheBudgetV1:()=>({partitions:{rust:{directory}}}),repositoryLeaseDirectoryV1:()=>"/leases",
   nativeRecomputableRetirementPendingV1:()=>held&&sample.retiring,repositoryPartitionReclaimDueV1:()=>{
    routine++;return nativeImageReclaimDueV1(repositoryPartitionReclaimRuleV1("rust"),allocated*GIB,used*GIB,free*GIB);
   },
   acquireResourceLease:async(options:Record<string,unknown>)=>{
    requests++;expect(options).toMatchObject({resource:"native-fixed-image:"+directory,mode:"exclusive",timeoutMs:0,signal:stop.signal});
    if(sample.busy)throw new ResourceLeaseTimeoutErrorV1({directory:"/leases",resource:"native-fixed-image:"+directory},0);
     if(sample.leaseFailed)throw Error("lease store could not be read");
    held=true;if(sample.cancel)stop.abort();
    return{release(){held=false;released++;if(sample.releaseFailed)throw Error("release failed");}};
   },
   nativeImageRootV1:()=>root,lstatSync:(name:string)=>name===image?entry(backingDevice,true):entry(name===root?mountDevice:backingDevice,false),
   statfsSync:(name:string)=>name===root?{blocks:100,bfree:free,bsize:GIB}:{blocks:100,bavail:free,bsize:GIB},
   nativeImageVolumeUsedBytesSyncV1:(name:string,volume:{blocks:number;bfree:number;bsize:number})=>nativeImageVolumeUsedBytesSyncV1(name,volume,"darwin",()=>sample.occupancyUnavailable?undefined:
    `<plist version="1.0"><dict><key>MountPoint</key><string>${root}</string><key>FilesystemType</key><string>apfs</string><key>CapacityInUse</key><integer>${used*GIB}</integer></dict></plist>`),
   existsSync:()=>true,createNativeImageReclaimPortV1:()=>({}),
   reclaimNativeImageV1:async()=>{
    expect(held).toBe(true);if(sample.cycleFailed)throw Error("cycle failed");effects++;if(!sample.releaseFailed)reported=24*GIB;
    return{beforeBytes:allocated*GIB,afterBytes:used*GIB,settled:true};
   },
  };
  const run=new Function(...Object.keys(ports),code+";return reclaimImageIfQuiescentV1;")(...Object.values(ports));
  const invoke=async(id:string)=>readImageReclaimReceiptV1(await run({},control,id,undefined,pressure?{version:1,device:4,requiredFreeBytes:24*GIB}:undefined),id);
   const expectedOutcome=sample.effect?{kind:"reclaimed"}:{kind:"skipped",reason:sample.reason};
   let observed:ImageReclaimOutcomeV1|undefined;
   if(pressure){
   const result=collectImageSlackV1({
    rows:[{...row("rust",66,35),device:4}],pressureDevice:4,targetFreeBytes:24*GIB,freeBytes:()=>reported,language:"en",signal:stop.signal,
    reclaim:async id=>observed=await invoke(id),
    retire:async()=>({kind:"held"}),
   });
   if(sample.cancel)await expect(result).rejects.toThrow();
   else{expect((await result).reclaimed).toEqual(sample.effect?["rust"]:[]);expect(observed,sample.name).toEqual(expectedOutcome);}
  }else expect(await invoke("rust"),sample.name).toEqual(expectedOutcome);
  expect(effects,sample.name).toBe(Number(sample.effect||!!sample.releaseFailed));
  expect(routine,sample.name).toBe(+!pressure);
  expect(released,sample.name).toBe(requests-(sample.busy||sample.leaseFailed?1:0));
  expect(held).toBe(false);
 }
});

test("the policy names a declared reclaim target for every reclaim profile and the retire profiles are among them",()=>{
 for(const profile of IMAGE_SLACK_V1.reclaimProfiles){expect(IMAGE_SLACK_V1.reclaimCommand.replace("{partition}",profile)).toBe("repo run repo-cache-reset:cache-reclaim-"+profile);expect(Object.keys(project.targets)).toContain("cache-reclaim-"+profile);}
 for(const profile of IMAGE_SLACK_V1.regrowProfiles)expect(IMAGE_SLACK_V1.reclaimProfiles).toContain(profile);
});

test("reclaim comes first for every bloated image, an inelastic image is reclaimed and never retired, a reclaimed image is never retired",async()=>{
 const rows=[row("rust",22,10),row("dependencies",14,6),row("tools",3.4,3.1)],calls:string[]=[],lines:string[]=[];let free=0;
 const result=await collectImageSlackV1({rows,targetFreeBytes:30*GIB,freeBytes:()=>free,language:"both",report:line=>lines.push(line),
  reclaim:async id=>{calls.push("reclaim "+id);free+=id==="rust"?11*GIB:7*GIB;return{kind:"reclaimed"};},
  retire:async id=>{calls.push("retire "+id);return{kind:"retired"};}});
 expect(calls).toEqual(["reclaim rust","reclaim dependencies"]);expect(result.reclaimed).toEqual(["rust","dependencies"]);expect(result.retired).toEqual([]);expect(result.freedBytes).toBe(18*GIB);
 expect(lines.filter(line=>line.includes("rust")&&line.includes(" / ")).length).toBe(1);
});

test("when the reclaim cannot help, an elastic image is retired next, an inelastic one is skipped with the reason named, a failed reclaim is named and retire still follows",async()=>{
 const rows=[row("rust",22,10),row("dependencies",14,6),row("graph",9,2)],calls:string[]=[],lines:string[]=[];let free=0;
 const answers:Record<string,ImageReclaimOutcomeV1>={rust:{kind:"skipped",reason:"lease-timeout"},dependencies:{kind:"skipped",reason:"retirement-pending"},graph:{kind:"failed",message:"hdiutil busy"}};
 const result=await collectImageSlackV1({rows,targetFreeBytes:999*GIB,freeBytes:()=>free,language:"both",report:line=>lines.push(line),
  reclaim:async id=>{calls.push("reclaim "+id);return answers[id]!;},
  retire:async id=>{calls.push("retire "+id);free+=id==="rust"?12*GIB:7*GIB;return{kind:"retired"} as ImageOutcomeV1;}});
 expect(calls).toEqual(["reclaim rust","retire rust","reclaim dependencies","reclaim graph","retire graph"]);
 expect(result.retired).toEqual(["rust","graph"]);expect(result.reclaimed).toEqual([]);
 const missed=lines.find(line=>line.includes("dependencies"))!.split(" / ");
 expect(missed[0]).toContain("retirement is pending");expect(missed[1]).toContain("steht aus");
 expect(lines.some(line=>line.includes("hdiutil busy"))).toBe(true);
});

test("settled zero block gain permits only the existing eligible fallback while still short",async()=>{
 const oracle=spawnSync("repo",["tools","exec","--light","python","--","-c",`import json,sys
fixture=json.load(sys.stdin)
for row in fixture["cases"]:
 receipt=row["receipt"]
 if receipt and receipt["outcome"]=="skipped":
  print("skipped:"+receipt["reason"] if receipt["partition"]=="rust" and receipt.get("reason") in fixture["reasons"] and not receipt["settled"] and receipt["beforeBytes"]==0 and receipt["afterBytes"]==0 else "unconfirmed")
  continue
 trustworthy=receipt and "reason" not in receipt and receipt["partition"]=="rust" and receipt["settled"] and receipt["beforeBytes"]>=receipt["afterBytes"]>=0
 if not trustworthy:
  print("unconfirmed")
 else:
  print("unchanged" if receipt["beforeBytes"]==receipt["afterBytes"] else "reclaimed")
`],{input:JSON.stringify(fixture),encoding:"utf8"});
 expect(oracle.status).toBe(0);
 expect(oracle.stdout.trim().split("\n")).toEqual(fixture.cases.map(row=>row.kind==="skipped"?row.kind+":"+row.receipt?.reason:row.kind));
 for(const sample of fixture.cases){
  const answer=readImageReclaimReceiptV1(sample.receipt,"rust");
  expect(answer).toEqual(sample.kind==="skipped"?{kind:"skipped",reason:sample.receipt?.reason}:{kind:sample.kind});
  for(const [id,short,cancel] of [["rust",true,false],["rust",false,false],["dependencies",true,false],["rust",true,true]] as const){
   let free=0,retire=0;const controller=new AbortController();
   const collected=collectImageSlackV1({rows:[row(id,22,10)],targetFreeBytes:GIB,freeBytes:()=>free,language:"both",signal:controller.signal,
    reclaim:async()=>{if(!short)free=GIB;if(cancel)controller.abort();return answer;},retire:async()=>{retire++;return{kind:"journal"};}});
   if(cancel)await expect(collected).rejects.toThrow();else await collected;
   expect(retire).toBe(Number(sample.retire&&id==="rust"&&short&&!cancel));
  }
 }
});

test("detached nomination is bounded to eligible registered backing files on the pressured device",async()=>{
 const incoming=[{...row("rust",22,10,false),device:4},{...row("graph",9,2,false),device:9},{...row("dependencies",20,1,false),device:4},{...row("products",0.5,0.2,false),device:4},{...row("tools",20,1,true),device:9}];
 for(const device of [undefined,4,9]){
  const calls:string[]=[];
  await collectImageSlackV1({rows:incoming,targetFreeBytes:GIB,pressureDevice:device,freeBytes:()=>0,language:"both",reclaim:async id=>{calls.push("reclaim "+id);return{kind:"skipped",reason:"not-due"};},retire:async id=>{calls.push("inspect "+id);return{kind:"skipped"};}});
  expect(calls).toEqual(device===undefined?["reclaim tools"]:device===4?["inspect rust"]:["reclaim tools","inspect graph"]);
 }
});

test("the collector stops at the target, skips unmounted and small images, honours cancellation and works without a reclaim port as before",async()=>{
 let free=0;const calls:string[]=[];
 await collectImageSlackV1({rows:[row("rust",22,10),row("graph",9,2)],targetFreeBytes:5*GIB,freeBytes:()=>free,language:"en",reclaim:async id=>{calls.push(id);free+=11*GIB;return{kind:"reclaimed"};},retire:async()=>({kind:"retired"})});
 expect(calls).toEqual(["rust"]);
 const none:string[]=[];
 await collectImageSlackV1({rows:[row("rust",22,10,false),row("nx",0.5,0.45)],targetFreeBytes:99*GIB,freeBytes:()=>0,language:"en",reclaim:async id=>{none.push(id);return{kind:"reclaimed"};},retire:async()=>({kind:"retired"})});
 expect(none).toEqual([]);
 const controller=new AbortController();
 await expect(collectImageSlackV1({rows:[row("rust",22,10),row("graph",9,2)],targetFreeBytes:99*GIB,freeBytes:()=>0,language:"en",signal:controller.signal,reclaim:async()=>{controller.abort();return{kind:"skipped",reason:"not-due"};},retire:async()=>({kind:"retired"})})).rejects.toThrow();
 const old:string[]=[];
 const legacy=await collectImageSlackV1({rows:[row("rust",22,10)],targetFreeBytes:99*GIB,freeBytes:()=>0,language:"en",retire:async id=>{old.push(id);return{kind:"retired"};}});
 expect(old).toEqual(["rust"]);expect(legacy.reclaimed).toEqual([]);
});

test("the refusal names the in-place command for an inelastic image and both commands for the largest elastic one",()=>{
 const only=describeImageSlackV1([row("dependencies",14,6),row("tools",3.4,3.1)]);
 expect(only.command).toBe("repo run repo-cache-reset:cache-reclaim-dependencies");expect(only.bytes).toBeGreaterThan(7*GIB);expect(only.root).toContain("/ws/.cache/b/dependencies");
 const both=describeImageSlackV1([row("rust",22,10),row("dependencies",14,6)]);
 expect(both.command).toBe("repo run repo-cache-reset:cache-reclaim-rust or/oder repo run repo-cache-reset:cache-regrow-rust");
 expect(both.bytes).toBeGreaterThan(18*GIB);
});

test("the image measurement and collector child agree on the machine state directory",()=>{
 const base=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"rm-"));roots.push(base);
 const workspace=join(base,"w"),state=join(parse(base).root,"semio-state-"+basename(base)),relative=".cache/b/r",directory=join(workspace,relative),backing=join(directory,"fixed.dmg");
 expect(existsSync(state)).toBe(false);
 mkdirSync(directory,{recursive:true});
 writeFileSync(backing,Buffer.alloc(4096,5));truncateSync(backing,16*MIB);
 const result=spawnSync(process.execPath,["--eval",`
 import {measureImagesV1} from ${JSON.stringify(new URL("../../🟦️.ts",import.meta.url).href)};
 import {graphCollectorEnvironmentV1} from ${JSON.stringify(new URL("../../../../../🧹️cache-gc/🚀️launch/🟦️.ts",import.meta.url).href)};
 const env=graphCollectorEnvironmentV1(process.env);
 const rows=measureImagesV1(${JSON.stringify(workspace)},undefined,{rust:{directory:${JSON.stringify(relative)}}});
 delete process.env.SEMIO_MACHINE_STATE_DIRECTORY;
 const local=measureImagesV1(${JSON.stringify(workspace)},undefined,{rust:{directory:${JSON.stringify(relative)}}});
 console.log(JSON.stringify({state:env.SEMIO_MACHINE_STATE_DIRECTORY,rows,local}));
 `],{env:{...process.env,SEMIO_MACHINE_STATE_DIRECTORY:state},encoding:"utf8"});
 expect(result.stderr).toBe("");expect(result.status).toBe(0);
 const output=JSON.parse(result.stdout);expect(output.state).toBe(state);
 expect(output.rows).toEqual([]);
 const oracle=spawnSync("repo",["tools","exec","--light","python","--","-c","import json, os, sys; s=os.stat(sys.argv[1]); print(json.dumps([getattr(s,'st_blocks',0)*512,s.st_size]))",backing],{encoding:"utf8"});
 expect(oracle.status).toBe(0);
 const [allocated,size]=JSON.parse(oracle.stdout);
 expect(output.local).toEqual([{id:"rust",directory,device:lstatSync(backing).dev,mounted:false,allocatedBytes:allocated,usedBytes:0,slackBytes:0}]);
 expect(size).toBe(16*MIB);
 if(process.platform!=="win32")expect(allocated).toBeLessThan(size);
});

const hdiutil=(...args:string[])=>spawnSync("/usr/bin/hdiutil",args,{encoding:"utf8"});
const sha=(path:string)=>createHash("sha256").update(readFileSync(path)).digest("hex");
test.skipIf(process.platform!=="darwin")("a real synthetic image gives its deleted blocks back through the reclaim cycle the collector drives, keeping size, identity and the unit cache content",async()=>{
 const base=realpathSync(mkdtempSync(join(tmpdir(),"reclaim-law-")));roots.push(base);
 const directory=join(base,".cache","b","r"),mount=join(directory,"m"),image=join(directory,"fixed.dmg");mkdirSync(mount,{recursive:true});
 expect(hdiutil("create","-size","64m","-type","UDIF","-fs","Case-sensitive APFS","-volname","t","-quiet",image).status).toBe(0);
 expect(hdiutil("resize","-size","384m",image).status).toBe(0);
 const attach=()=>expect(hdiutil("attach",image,"-nobrowse","-noautoopen","-owners","on","-mountpoint",mount,"-quiet").status).toBe(0);
 attach();
 mkdirSync(join(mount,"u"));const keep:Record<string,string>={};
 for(let index=0;index<6;index++){const file=join(mount,"u","unit"+index+".rlib");writeFileSync(file,Buffer.from(("unit "+index+" ").repeat(150_000)));keep[file]=sha(file);}
 for(let index=0;index<4;index++)expect(spawnSync("/bin/dd",["if=/dev/urandom","of="+join(mount,"f"+index),"bs=1m","count=40"],{stdio:"ignore"}).status).toBe(0);
 spawnSync("/bin/sync");for(let index=0;index<4;index++)rmSync(join(mount,"f"+index));spawnSync("/bin/sync");await new Promise(done=>setTimeout(done,2500));
 const before=lstatSync(image);const stale=before.blocks*512;expect(stale).toBeGreaterThan(140*MIB);
 expect(hdiutil("detach",mount,"-quiet").status).toBe(0);
 const run=async(program:string,args:readonly string[])=>{const result=spawnSync(program,[...args],{encoding:"utf8"});if(result.status!==0)throw Error(program+": "+result.stderr);return result.stdout;};
 let reclaimedBytes=0;
 const result=await collectImageSlackV1({rows:[{id:"rust",directory,mounted:true,allocatedBytes:stale,usedBytes:10*MIB,slackBytes:stale-10*MIB-IMAGE_SLACK_V1.filesystemOverheadBytes}],targetFreeBytes:Number.MAX_SAFE_INTEGER,freeBytes:()=>reclaimedBytes,language:"en",policy:{...IMAGE_SLACK_V1,minimumSlackBytes:32*MIB,minimumSlackPercent:10},
  reclaim:async()=>{const outcome=await reclaimNativeImageV1(createNativeImageDarwinReclaimPortV1({image,cwd:directory,run}),{check:()=>{}});reclaimedBytes=outcome.beforeBytes-outcome.afterBytes;return outcome.afterBytes<outcome.beforeBytes?{kind:"reclaimed"}:{kind:"unchanged"};},
  retire:async()=>{throw Error("a reclaimed image must not be retired");}});
 expect(result.reclaimed).toEqual(["rust"]);expect(result.retired).toEqual([]);
 const after=lstatSync(image);expect(after.blocks*512).toBeLessThan(stale-100*MIB);expect(reclaimedBytes).toBeGreaterThan(100*MIB);
 expect(after.size).toBe(before.size);expect(after.ino).toBe(before.ino);
 expect(hdiutil("info").stdout).not.toContain(image);
 attach();
 try{for(const [file,digest] of Object.entries(keep))expect(sha(file)).toBe(digest);}finally{hdiutil("detach",mount,"-quiet");}
},240_000);


test("build intake limits mounted reclaim and ordinary fallback to the pressured device, without detached inspection",async()=>{
 const sample=fixture.buildIngress,calls:string[]=[],rows=sample.images.map(image=>({...row(image.id,image.allocatedGiB,image.usedGiB,image.mounted),device:image.device}));
 using oracle=new Database(":memory:");
 const expected=oracle.query(`with eligible as (
  select key,json_extract(value,'$.id') id,json_extract(value,'$.busy') busy from json_each(?1)
  where json_extract(value,'$.device')=?2 and json_extract(value,'$.mounted'))
 select 'reclaim ' || id action,key*2 position from eligible union all
 select 'retire ' || id,key*2+1 from eligible where busy order by position`).all(JSON.stringify(sample.images),sample.device) as {action:string}[];
 await collectImageSlackV1({rows,pressureDevice:sample.device,inspectDetached:false,targetFreeBytes:30*GIB,freeBytes:()=>0,language:"en",
  reclaim:async id=>{calls.push("reclaim "+id);return sample.images.some(image=>image.id===id&&image.busy)?{kind:"skipped",reason:"lease-timeout"}:{kind:"reclaimed"};},
  retire:async id=>{calls.push("retire "+id);return{kind:"held"};}});
 expect(calls).toEqual(expected.map(row=>row.action));
});

test.skipIf(process.platform!=="darwin")("the real image writer sends only the in-place child the measured device and pressure",async()=>{
 const sample=fixture.buildIngress,workspace=mkdtempSync(join(tmpdir(),"image-pressure-"));roots.push(workspace);
 const device=lstatSync(workspace).dev,target=30*GIB,calls:string[]=[];
 let free=sample.startGiB*GIB;
 const measure=spyOn(imageSlack,"measureImagesV1").mockReturnValue(sample.images.map(image=>({
  ...row(image.id,image.allocatedGiB,image.usedGiB,image.mounted),device:image.device===sample.device?device:device+1,
 })));
 try{
  const row=DISK_WRITERS_V1.find(item=>item.id==="image-slack")!;
  const [writer]=realDiskWritersV1({workspace,home:workspace,tmp:workspace,uid:0,environment:{},
   reclaimPressure:{path:workspace,requiredFreeBytes:target,freeBytes:()=>free},
   reclaim:async(id,_signal,recovery)=>{
    expect(recovery).toEqual({version:1,device,requiredFreeBytes:target});
    calls.push("reclaim "+id);
    if(sample.images.some(image=>image.id===id&&image.busy))return{kind:"skipped",reason:"lease-timeout"};
    free=target;
    return{kind:"reclaimed"};
   },
   retire:async(id,_signal,recovery)=>{
    expect(recovery).toBeUndefined();
    calls.push("retire "+id);
    return{kind:"held"};
   },
  },[row]);
  expect(await writer!.collect!({needBytes:target-free})).toBe(target-sample.startGiB*GIB);
  expect(calls).toEqual(["reclaim rust","retire rust","reclaim dependencies"]);
 }finally{measure.mockRestore();}
});
