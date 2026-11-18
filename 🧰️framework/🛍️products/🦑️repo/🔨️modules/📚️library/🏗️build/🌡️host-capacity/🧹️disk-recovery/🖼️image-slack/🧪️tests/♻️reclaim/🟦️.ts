/** ♻️ Contract: the image-slack collector returns blocks in place first (UNMAP reclaim, nothing recreated), retires an elastic image only when the reclaim could not, never retires an image it just reclaimed, reclaims inelastic images too, names every outcome in both languages and stops at the target; on a real synthetic image the blocks come back and the unit cache content is intact. */
import{afterAll,expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{createHash}from"node:crypto";
import{lstatSync,mkdirSync,mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import project from"../../../../../../⚡️caching/💾️storage/🔒️enforcement/📦️budget/📥️opening/♻️reset/📋️project.json"with{type:"json"};
import{IMAGE_SLACK_V1,collectImageSlackV1,describeImageSlackV1,type ImageOutcomeV1,type ImageReclaimOutcomeV1,type ImageRowV1}from"../../🟦️.ts";
import{createNativeImageDarwinReclaimPortV1,reclaimNativeImageV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🍎️image/♻️reclaim/🟦️.ts";

const GIB=1024**3,MIB=1024**2,roots:string[]=[];
afterAll(()=>{for(const root of roots)rmSync(root,{recursive:true,force:true});});
const row=(id:string,allocatedGiB:number,usedGiB:number,mounted=true):ImageRowV1=>({id,directory:"/ws/.cache/b/"+id,mounted,allocatedBytes:allocatedGiB*GIB,usedBytes:usedGiB*GIB,slackBytes:mounted?Math.max(0,(allocatedGiB-usedGiB)*GIB-IMAGE_SLACK_V1.filesystemOverheadBytes):0});

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
 const answers:Record<string,ImageReclaimOutcomeV1>={rust:{kind:"skipped"},dependencies:{kind:"skipped"},graph:{kind:"failed",message:"hdiutil busy"}};
 const result=await collectImageSlackV1({rows,targetFreeBytes:999*GIB,freeBytes:()=>free,language:"en",report:line=>lines.push(line),
  reclaim:async id=>{calls.push("reclaim "+id);return answers[id]!;},
  retire:async id=>{calls.push("retire "+id);free+=id==="rust"?12*GIB:7*GIB;return{kind:"retired"} as ImageOutcomeV1;}});
 expect(calls).toEqual(["reclaim rust","retire rust","reclaim dependencies","reclaim graph","retire graph"]);
 expect(result.retired).toEqual(["rust","graph"]);expect(result.reclaimed).toEqual([]);
 expect(lines.some(line=>line.includes("dependencies")&&line.includes("in place"))).toBe(true);expect(lines.some(line=>line.includes("hdiutil busy"))).toBe(true);
});

test("the collector stops at the target, skips unmounted and small images, honours cancellation and works without a reclaim port as before",async()=>{
 let free=0;const calls:string[]=[];
 await collectImageSlackV1({rows:[row("rust",22,10),row("graph",9,2)],targetFreeBytes:5*GIB,freeBytes:()=>free,language:"en",reclaim:async id=>{calls.push(id);free+=11*GIB;return{kind:"reclaimed"};},retire:async()=>({kind:"retired"})});
 expect(calls).toEqual(["rust"]);
 const none:string[]=[];
 await collectImageSlackV1({rows:[row("rust",22,10,false),row("nx",0.5,0.45)],targetFreeBytes:99*GIB,freeBytes:()=>0,language:"en",reclaim:async id=>{none.push(id);return{kind:"reclaimed"};},retire:async()=>({kind:"retired"})});
 expect(none).toEqual([]);
 const controller=new AbortController();
 await expect(collectImageSlackV1({rows:[row("rust",22,10),row("graph",9,2)],targetFreeBytes:99*GIB,freeBytes:()=>0,language:"en",signal:controller.signal,reclaim:async()=>{controller.abort();return{kind:"skipped"};},retire:async()=>({kind:"retired"})})).rejects.toThrow();
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
  reclaim:async()=>{const outcome=await reclaimNativeImageV1(createNativeImageDarwinReclaimPortV1({image,cwd:directory,run}),{check:()=>{}});reclaimedBytes=outcome.beforeBytes-outcome.afterBytes;return outcome.afterBytes<outcome.beforeBytes?{kind:"reclaimed"}:{kind:"skipped"};},
  retire:async()=>{throw Error("a reclaimed image must not be retired");}});
 expect(result.reclaimed).toEqual(["rust"]);expect(result.retired).toEqual([]);
 const after=lstatSync(image);expect(after.blocks*512).toBeLessThan(stale-100*MIB);expect(reclaimedBytes).toBeGreaterThan(100*MIB);
 expect(after.size).toBe(before.size);expect(after.ino).toBe(before.ino);
 expect(hdiutil("info").stdout).not.toContain(image);
 attach();
 try{for(const [file,digest] of Object.entries(keep))expect(sha(file)).toBe(digest);}finally{hdiutil("detach",mount,"-quiet");}
},240_000);
