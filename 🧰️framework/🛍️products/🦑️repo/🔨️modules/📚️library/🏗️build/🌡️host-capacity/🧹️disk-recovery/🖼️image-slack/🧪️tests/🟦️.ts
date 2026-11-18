/** 🧪️ Contract: the slack of the fixed cache images (host blocks that an image no longer uses and never returns) is measured from the host allocation and the filesystem inside, the gate retires only idle recomputable elastic images above the floor (largest first, only as many as the volume needs, never one that a command holds mapped), the volume is read again after each one, every skip names its reason in both languages, the refusal always names the image slack and the replica store with sizes and the exact command, and on a real synthetic image the retirement returns the host blocks while a held or skipped image keeps its bytes. */
import {afterAll,describe,expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {createHash} from "node:crypto";
import {existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,statSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import budgetPolicy from "../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/🔣️policy.json" with {type:"json"};
import resetSchema from "../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/🧬️schema/🔣️.json" with {type:"json"};
import {DISK_WRITERS_V1,consumersOfV1,refuseDiskRecoveryV1,requireDiskWritersV1,type DiskRecoveryV1,type DiskWriterRowV1,type DiskWriterV1} from "../../🟦️.ts";
import {renderRepoErrorV1} from "../../../../../../../../../🔨️modules/🏃️process/🏗️build/🗣️language/🚨️error/🟦️.ts";
import {IMAGE_SLACK_V1,collectImageSlackV1,decideImageSlackV1,describeImageSlackV1,measureImagesV1,type ImageOutcomeV1,type ImageRowV1} from "../🟦️.ts";

const GIB=1024**3,MIB=1024**2,roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"image-slack-"));roots.push(root);return root;};
const staged=(relative:string):string=>{const mirror=process.env.R172_MIRROR;const live=join("/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library",relative);return mirror&&existsSync(join(mirror,relative))?join(mirror,relative):live;};
const detachUnder=(root:string)=>{if(process.platform!=="darwin")return;for(const block of String(spawnSync("/usr/bin/hdiutil",["info"],{encoding:"utf8"}).stdout).split("================================================"))if(block.includes("image-path      : "+root)){const disk=/^\/dev\/disk\d+/mu.exec(block.slice(block.indexOf("/dev/disk")));if(disk)spawnSync("/usr/bin/hdiutil",["detach",disk[0],"-force","-quiet"]);}};
afterAll(()=>{for(const root of roots){detachUnder(root);rmSync(root,{recursive:true,force:true});}});

const row=(id:string,allocatedGiB:number,usedGiB:number,mounted:boolean=true):ImageRowV1=>({id,directory:"/ws/.cache/b/"+id,mounted,allocatedBytes:allocatedGiB*GIB,usedBytes:usedGiB*GIB,slackBytes:mounted?Math.max(0,(allocatedGiB-usedGiB)*GIB-IMAGE_SLACK_V1.filesystemOverheadBytes):0});
const thisMac=[row("rust",22.6,10.4),row("dependencies",13.75,13.2),row("tools",3.38,3.14),row("graph",2.67,1.91),row("nx",0.52,0.5),row("diagnostics",0.06,0.0),row("products",0.06,0.0)];

describe("the policy is the product's own partition table",()=>{
 test("regrow profiles are exactly the elastic partitions of the budget, each a declared reset profile with a declared regrow target",()=>{
  expect([...IMAGE_SLACK_V1.regrowProfiles].sort()).toEqual(Object.keys(budgetPolicy.elastic).sort());
  for(const profile of IMAGE_SLACK_V1.regrowProfiles){expect(resetSchema.$defs.Profile.enum).toContain(profile);expect(Object.keys(budgetPolicy.partitions)).toContain(profile==="rust"?"rust":profile);}
  const project=JSON.parse(readFileSync(staged("⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📋️project.json"),"utf8")) as {targets:Record<string,{options:{command:string}}>};
  for(const profile of IMAGE_SLACK_V1.regrowProfiles){expect(IMAGE_SLACK_V1.command.replace("{partition}",profile)).toBe("repo run repo-cache-reset:cache-regrow-"+profile);expect(project.targets["cache-regrow-"+profile].options.command).toBe("repo ./📜️script.ts regrow "+profile+" --pressed");}
 });
 test("the registry names the image slack as an automatic image-reset writer after every derivable cache and the replica store before it",()=>{
  requireDiskWritersV1(DISK_WRITERS_V1);
  const slack=DISK_WRITERS_V1.find(item=>item.id==="image-slack")!,store=DISK_WRITERS_V1.find(item=>item.id==="replica-store")!;
  expect(slack.collector).toBe("image-reset");expect(store.collector).toBe("store-compaction");
  expect(slack.priority).toBeGreaterThan(store.priority);
  for(const item of DISK_WRITERS_V1.filter(entry=>entry.class==="derivable"))expect(item.priority).toBeLessThan(store.priority);
  expect(DISK_WRITERS_V1.find(item=>item.id==="cas-lru")!.priority).toBeLessThan(store.priority);
 });
});

describe("the decision",()=>{
 test("on the measured Mac only the rust image is a candidate, the others say why not",()=>{
  const verdicts=decideImageSlackV1(thisMac,10*GIB);
  expect(verdicts.filter(item=>item.action==="retire").map(item=>item.id)).toEqual(["rust"]);
  const by=Object.fromEntries(verdicts.map(item=>[item.id,item.reason]));
  expect(by).toMatchObject({rust:"slack",dependencies:"not-elastic",tools:"not-elastic",graph:"below-floor",nx:"not-elastic",diagnostics:"not-elastic",products:"below-floor"});
 });
 test("only as many images as the volume needs are retired, the largest slack first; a detached image is never judged by a guess",()=>{
  const rows=[row("rust",30,10),row("graph",12,2),row("products",8,1),row("nx",9,1,false)];
  const verdicts=decideImageSlackV1(rows,15*GIB);
  expect(verdicts.map(item=>[item.id,item.action,item.reason])).toEqual([["rust","retire","slack"],["graph","skip","not-needed"],["products","skip","not-needed"],["nx","skip","not-elastic"]]);
  expect(decideImageSlackV1(rows,25*GIB).filter(item=>item.action==="retire").map(item=>item.id)).toEqual(["rust","graph"]);
  expect(decideImageSlackV1([row("rust",30,10,false)],GIB)[0]).toMatchObject({action:"skip",reason:"unmounted"});
  expect(decideImageSlackV1(rows,0).every(item=>item.action==="skip")).toBe(true);
 });
 test("an independent Python implementation decides the same for random machines",()=>{
  let seed=7;const next=()=>{seed=(seed*1103515245+12345)&0x7fffffff;return seed/0x7fffffff;};
  const ids=["rust","graph","products","nx","dependencies","tools"];
  for(let round=0;round<40;round++){
   const rows=ids.filter(()=>next()>0.2).map(id=>{const allocated=Math.round(next()*40*GIB),used=Math.round(next()*allocated),mounted=next()>0.1;return{id,directory:"/d/"+id,mounted,allocatedBytes:allocated,usedBytes:used,slackBytes:mounted?Math.max(0,allocated-used-IMAGE_SLACK_V1.filesystemOverheadBytes):0};});
   const need=Math.round((next()-0.1)*30*GIB),answer=JSON.parse(spawnSync("python3",["-I",join(import.meta.dirname,"oracle.py")],{input:JSON.stringify({policy:IMAGE_SLACK_V1,rows,need}),encoding:"utf8"}).stdout);
   expect(decideImageSlackV1(rows,need).map(item=>({id:item.id,action:item.action,reason:item.reason}))).toEqual(answer);
  }
 });
});

describe("the collection with a fake product",()=>{
 const volume_={free:0};
 const run=async(rows:readonly ImageRowV1[],freeStart:number,target:number,answers:Record<string,ImageOutcomeV1|Error>,gain:Record<string,number>,language:"en"|"de"="en",signal?:AbortSignal)=>{
  volume_.free=freeStart;const lines:string[]=[],calls:string[]=[];
  const result=await collectImageSlackV1({rows,targetFreeBytes:target,freeBytes:()=>volume_.free,language,report:line=>lines.push(line),signal,retire:async id=>{calls.push(id);const answer=answers[id]??{kind:"retired"};if(answer instanceof Error)throw answer;if(answer.kind==="retired")volume_.free+=gain[id]??0;return answer;}});
  return{result,lines,calls};
 };
 test("retires the largest first, reads the volume after each and stops when the target is free",async()=>{
  const rows=[row("graph",12,2),row("rust",30,10),row("products",8,1)];
  const {result,calls,lines}=await run(rows,5*GIB,20*GIB,{},{rust:19*GIB,graph:9*GIB});
  expect(calls).toEqual(["rust"]);expect(result.retired).toEqual(["rust"]);expect(result.freedBytes).toBe(19*GIB);
  expect(lines).toHaveLength(1);expect(lines[0]).toContain("rust");
 });
 test("trusts the volume and not the claim: an image that returned nothing does not satisfy the target",async()=>{
  const rows=[row("rust",30,10),row("graph",12,2)];
  const {result,calls}=await run(rows,5*GIB,20*GIB,{},{rust:0,graph:9*GIB});
  expect(calls).toEqual(["rust","graph"]);expect(result.freedBytes).toBe(9*GIB);
 });
 test("an image a command holds mapped is skipped with its named reason in both languages and the next one is tried",async()=>{
  const rows=[row("rust",30,10),row("graph",12,2)];
  const english=await run(rows,5*GIB,30*GIB,{rust:{kind:"held"}},{graph:9*GIB});
  expect(english.result.held).toEqual(["rust"]);expect(english.result.retired).toEqual(["graph"]);expect(english.lines[0]).toContain("holds it mapped");
  const german=await run(rows,5*GIB,30*GIB,{rust:{kind:"held"}},{graph:9*GIB},"de");
  expect(german.lines[0]).toContain("eingehängt");
  const journal=await run(rows,5*GIB,30*GIB,{rust:{kind:"journal"}},{graph:9*GIB});
  expect(journal.lines[0]).toContain("journal");
 });
 test("a failing retirement never stops the next and cancellation stops between images",async()=>{
  const rows=[row("rust",30,10),row("graph",12,2)];
  const failed=await run(rows,5*GIB,30*GIB,{rust:new Error("boom")},{graph:9*GIB});
  expect(failed.calls).toEqual(["rust","graph"]);expect(failed.lines[0]).toContain("boom");expect(failed.result.retired).toEqual(["graph"]);
  const controller=new AbortController();volume_.free=0;const calls:string[]=[];
  await expect(collectImageSlackV1({rows,targetFreeBytes:99*GIB,freeBytes:()=>volume_.free,language:"en",signal:controller.signal,retire:async id=>{calls.push(id);controller.abort();return{kind:"retired"};}})).rejects.toBeDefined();
  expect(calls).toEqual(["rust"]);
 });
 test("a volume that needs nothing retires nothing",async()=>{
  const {calls}=await run([row("rust",30,10)],50*GIB,20*GIB,{},{});
  expect(calls).toEqual([]);
 });
});

describe("the refusal always names the image slack and the replica store",()=>{
 const writer=(id:string,bytes:number,extra:Partial<DiskWriterV1>={}):DiskWriterV1=>{const base=DISK_WRITERS_V1.find(item=>item.id===id)??({id,priority:1,class:"derivable",relief:"host",root:"/"+id,collector:"age-prune",command:"repo cache gc",bound:{en:"b",de:"b"}} as DiskWriterRowV1);return{row:base,root:"/"+id,measure:()=>bytes,...extra};};
 test("with seven larger consumers the two named ones are listed besides the five largest, with sizes and the exact commands, in English and German",async()=>{
  const writers=[...["cas-lru","compile-caches","dependency-generations","tool-generations","closure-layers","cargo-incremental","leaked-staging"].map((id,index)=>writer(id,(60-index)*GIB)),writer("image-slack",12*GIB,{describe:async()=>({root:"/ws/.cache/b/r (rust 12.0 GiB of 22.6 GiB)",command:"repo run repo-cache-reset:cache-regrow-rust"})}),writer("replica-store",44*GIB,{describe:async()=>({root:"/Users/Shared/semio-store",command:"cd 🧰️framework/🛍️products/🦑️repo/🔨️modules/🤝️shared-worktree/🔁️replica && repo ./📜️script.ts compaction --apply"})})];
  const consumers=await consumersOfV1(writers);
  expect(consumers.slice(0,5).map(item=>item.id)).toEqual(["cas-lru","compile-caches","dependency-generations","tool-generations","closure-layers"]);
  expect(consumers.map(item=>item.id)).toContain("image-slack");expect(consumers.map(item=>item.id)).toContain("replica-store");expect(consumers).toHaveLength(7);
  const outcome:DiskRecoveryV1={verdict:"refuse",facts:{partition:"/",totalBytes:238*GIB,freeBytes:5*GIB},needBytes:6*GIB,mustFreeBytes:7.5*GIB,targetFreeBytes:18*GIB,steps:[],consumers};
  const facts={partition:"/",totalBytes:238*GIB,freeBytes:5*GIB};
  for(const language of ["en","de"] as const){
   const text=renderRepoErrorV1(refuseDiskRecoveryV1(outcome,facts),language);
   expect(text).toContain("12 GiB");expect(text).toContain("44 GiB");expect(text).toContain("repo run repo-cache-reset:cache-regrow-rust");expect(text).toContain("compaction --apply");expect(text).toContain("/Users/Shared/semio-store");
  }
 });
 test("a named consumer without bytes is not invented",async()=>{
  const consumers=await consumersOfV1([writer("cas-lru",GIB),writer("image-slack",0),writer("replica-store",0)]);
  expect(consumers.map(item=>item.id)).toEqual(["cas-lru"]);
 });
});

const darwin=process.platform==="darwin";
const hdiutil=(...args:string[])=>spawnSync("/usr/bin/hdiutil",args,{encoding:"utf8"});
const makeImage=(directory:string,logicalMiB:number)=>{
 mkdirSync(join(directory,"m"),{recursive:true});const image=join(directory,"fixed.dmg");
 expect(hdiutil("create","-size","64m","-type","UDIF","-fs","Case-sensitive APFS","-volname","t","-quiet",image).status).toBe(0);
 const grown=hdiutil("resize","-size",logicalMiB+"m",image);expect(grown.status).toBe(0);
 expect(hdiutil("attach",image,"-nobrowse","-noautoopen","-owners","on","-mountpoint",join(directory,"m"),"-quiet").status).toBe(0);
 return image;
};
const fill=(directory:string,files:number,mib:number)=>{for(let index=0;index<files;index++)expect(spawnSync("/bin/dd",["if=/dev/urandom","of="+join(directory,"m","f"+index),"bs=1m","count="+mib],{stdio:"ignore"}).status).toBe(0);spawnSync("/bin/sync");};
const sha=(path:string)=>createHash("sha256").update(readFileSync(path)).digest("hex");
const partitions={rust:{directory:".cache/b/r"}};
const small:typeof IMAGE_SLACK_V1={...IMAGE_SLACK_V1,minimumSlackBytes:32*MIB,minimumSlackPercent:10};

describe.skipIf(!darwin)("a real synthetic fixed image (hdiutil, the product's own creation recipe)",()=>{
 test("freed blocks inside never return to the host; the retirement does, a held image keeps its bytes (digest law), a killed retirement leaves a usable image or none (kill law)",async()=>{
  const workspace=fresh(),directory=join(workspace,".cache/b/r");mkdirSync(directory,{recursive:true});
  makeImage(directory,256);
  writeFileSync(join(directory,"m","keep"),"keep ".repeat(2_000_000));const digest=sha(join(directory,"m","keep"));
  fill(directory,4,40);
  const full=measureImagesV1(workspace,small,partitions)[0];
  expect(full.mounted).toBe(true);expect(full.usedBytes).toBeGreaterThan(160*MIB);
  for(let index=0;index<4;index++)rmSync(join(directory,"m","f"+index));
  spawnSync("/bin/sync");await new Promise(done=>setTimeout(done,2500));
  const after=measureImagesV1(workspace,small,partitions)[0];
  expect(after.usedBytes).toBeLessThan(40*MIB);
  expect(after.allocatedBytes).toBeGreaterThan(full.allocatedBytes-20*MIB);
  expect(after.slackBytes).toBeGreaterThan(80*MIB);
  const verdict=decideImageSlackV1(measureImagesV1(workspace,small,partitions),50*MIB,small)[0];
  expect(verdict).toMatchObject({id:"rust",action:"retire"});
  const heldRun=await collectImageSlackV1({rows:measureImagesV1(workspace,small,partitions),targetFreeBytes:Number.MAX_SAFE_INTEGER,freeBytes:()=>1,language:"en",policy:small,retire:async()=>({kind:"held"})});
  expect(heldRun.held).toEqual(["rust"]);expect(sha(join(directory,"m","keep"))).toBe(digest);expect(statSync(join(directory,"fixed.dmg")).blocks*512).toBe(after.allocatedBytes);
  const blocks=()=>statSync(join(directory,"fixed.dmg")).blocks*512;
  const retire=async():Promise<ImageOutcomeV1>=>{expect(hdiutil("detach",join(directory,"m"),"-quiet").status).toBe(0);rmSync(join(directory,"fixed.dmg"));return{kind:"retired"};};
  const hostBefore=after.allocatedBytes;let hostFree=0;
  const done=await collectImageSlackV1({rows:measureImagesV1(workspace,small,partitions),targetFreeBytes:Number.MAX_SAFE_INTEGER,freeBytes:()=>hostFree,language:"en",policy:small,retire:async id=>{const outcome=await retire();hostFree+=hostBefore;return outcome;}});
  expect(done.retired).toEqual(["rust"]);expect(existsSync(join(directory,"fixed.dmg"))).toBe(false);
  makeImage(directory,256);expect(blocks()).toBeLessThan(hostBefore-100*MIB);
  const reborn=measureImagesV1(workspace,small,partitions)[0];expect(reborn.slackBytes).toBeLessThan(small.minimumSlackBytes);
  expect(hdiutil("detach",join(directory,"m"),"-quiet").status).toBe(0);
 },240_000);
 test("a retirement killed after the detach leaves an image that attaches with its content intact, killed after the removal leaves none and a new image is born",async()=>{
  for(const killAt of ["after-detach","after-remove"]){
   const workspace=fresh(),directory=join(workspace,".cache/b/r");mkdirSync(directory,{recursive:true});
   makeImage(directory,128);writeFileSync(join(directory,"m","keep"),"keep ".repeat(500_000));const digest=sha(join(directory,"m","keep"));
   const script=join(workspace,"kill.mjs");
   writeFileSync(script,`import {spawnSync} from "node:child_process";import {rmSync} from "node:fs";const [d,at]=process.argv.slice(2);spawnSync("/usr/bin/hdiutil",["detach",d+"/m","-quiet"]);if(at==="after-detach")process.kill(process.pid,"SIGKILL");rmSync(d+"/fixed.dmg");process.kill(process.pid,"SIGKILL");`);
   const child=spawnSync(process.execPath,[script,directory,killAt]);expect(child.signal).toBe("SIGKILL");
   if(killAt==="after-detach"){
    expect(hdiutil("attach",join(directory,"fixed.dmg"),"-nobrowse","-noautoopen","-owners","on","-mountpoint",join(directory,"m"),"-quiet").status).toBe(0);
    expect(sha(join(directory,"m","keep"))).toBe(digest);expect(hdiutil("detach",join(directory,"m"),"-quiet").status).toBe(0);
   }else{
    expect(existsSync(join(directory,"fixed.dmg"))).toBe(false);makeImage(directory,128);expect(readdirOf(join(directory,"m"))).not.toContain("keep");expect(hdiutil("detach",join(directory,"m"),"-quiet").status).toBe(0);
   }
  }
 },240_000);
 test("why the images stay fixed raw images: compaction returns blocks only for sparse formats and only detached, a raw image answers that it is not implemented",async()=>{
  const workspace=fresh(),results:Record<string,{attached:number;detached:number;compacted:number;digest:boolean}>={};
  for(const [kind,file] of [["UDIF","i.dmg"],["SPARSE","i.sparseimage"]] as const){
   const directory=join(workspace,kind);mkdirSync(join(directory,"m"),{recursive:true});const image=join(directory,file);
   expect(hdiutil("create","-size","192m","-type",kind,"-fs","Case-sensitive APFS","-volname","t","-quiet",image).status).toBe(0);
   expect(hdiutil("attach",image,"-nobrowse","-noautoopen","-owners","on","-mountpoint",join(directory,"m"),"-quiet").status).toBe(0);
   writeFileSync(join(directory,"m","keep"),"keep ".repeat(500_000));const digest=sha(join(directory,"m","keep"));
   fill(directory,3,40);for(let index=0;index<3;index++)rmSync(join(directory,"m","f"+index));spawnSync("/bin/sync");await new Promise(done=>setTimeout(done,2000));
   const attached=statSync(image).blocks*512;
   const live=hdiutil("compact",image);expect(live.status).not.toBe(0);
   expect(hdiutil("detach",join(directory,"m"),"-quiet").status).toBe(0);
   const detached=statSync(image).blocks*512,compact=hdiutil("compact",image),compacted=statSync(image).blocks*512;
   expect(hdiutil("attach",image,"-nobrowse","-noautoopen","-owners","on","-mountpoint",join(directory,"m"),"-quiet").status).toBe(0);
   results[kind]={attached,detached,compacted,digest:sha(join(directory,"m","keep"))===digest};
   expect(hdiutil("detach",join(directory,"m"),"-quiet").status).toBe(0);
   if(kind==="UDIF")expect(compact.status).not.toBe(0);else expect(compact.status).toBe(0);
  }
  expect(results.UDIF.compacted).toBeGreaterThanOrEqual(results.UDIF.detached-MIB);
  expect(results.SPARSE.compacted).toBeLessThan(results.SPARSE.detached-80*MIB);
  expect(results.UDIF.digest&&results.SPARSE.digest).toBe(true);
 },240_000);
});
function readdirOf(path:string):string[]{return require("node:fs").readdirSync(path);}
