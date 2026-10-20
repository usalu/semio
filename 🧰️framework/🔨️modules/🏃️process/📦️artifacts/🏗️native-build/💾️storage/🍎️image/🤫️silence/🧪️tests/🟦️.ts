import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{chmodSync,existsSync,lstatSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,renameSync,rmSync,symlinkSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import fixture from"./🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import imageSchema from"../../../🧬️schema/🔣️.json"with{type:"json"};
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{nativeImageAttachArgumentsV1}from"../../👤️owners/🟦️.ts";
import{applyNativeImageVolumeSilenceV1,createNativeImageVolumeSilenceSeedV1,decideNativeImageVolumeSilenceV1,hasNativeImageVolumeSilenceMarkerV1,nativeImageSeededCreateArgumentsV1,nativeImageVolumeSilencedV1,observeNativeImageVolumeSilenceV1,type NativeImageVolumeSilenceFactsV1}from"../🟦️.ts";
const protocol=schema.definitions.Protocol.const,hex=(index:number)=>index.toString(16).padStart(16,"0");
const run=(program:string,args:readonly string[])=>{const result=spawnSync(program,[...args],{encoding:"utf8",env:{PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"}});return{status:result.status,stdout:result.stdout,stderr:result.stderr};};
const oracle=(directory:string)=>JSON.parse(run("/usr/bin/python3",["-I","-c","import os,sys,re,json\np=re.compile(sys.argv[2])\nr=[e.stat(follow_symlinks=False).st_size for e in os.scandir(sys.argv[1]) if e.is_file(follow_symlinks=False) and p.fullmatch(e.name)]\nprint(json.dumps({'files':len(r),'bytes':sum(r)}))",directory,protocol.logPattern]).stdout)as{files:number;bytes:number};
const owner=(path:string)=>Number(run("/usr/bin/python3",["-I","-c","import os,sys;print(os.lstat(sys.argv[1]).st_uid)",path]).stdout);
const logs=(root:string,count:number,size=1000)=>{const directory=join(root,protocol.serviceDirectory);mkdirSync(directory,{recursive:true});for(let index=0;index<count;index++)writeFileSync(join(directory,hex(index+1)),"x".repeat(size));};
const settle=async(path:string)=>{for(let attempt=0;attempt<20&&!existsSync(path);attempt++)await new Promise(resolve=>setTimeout(resolve,250));return existsSync(path);};
const scratchRoot=()=>realpathSync(mkdtempSync(join(process.env.TMPDIR||tmpdir(),"silence-")));
test("observation tolerates a discovered log disappearing and retains cancellation",async()=>{
 const root=realpathSync(mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR!,"silence-"))),row=fixture.observationRace,directory=join(root,protocol.serviceDirectory);
 try{
  logs(root,row.logs,row.bytesPerLog);let checks=0;
  const facts=observeNativeImageVolumeSilenceV1(root,()=>{if(++checks===1)rmSync(join(directory,hex(row.removed)));});
  const {default:glob}=await import("fast-glob"),names=glob.sync("*",{cwd:directory,onlyFiles:true,followSymbolicLinks:false}).filter(name=>new RegExp(protocol.logPattern,"u").test(name));
  const census={files:names.length,bytes:names.reduce((sum,name)=>sum+lstatSync(join(directory,name)).size,0)};
  expect(census).toEqual({files:row.files,bytes:row.bytes});expect({files:facts.logFiles,bytes:facts.logBytes}).toEqual(census);
  writeFileSync(join(directory,hex(row.removed)),"x".repeat(row.bytesPerLog));
  const cancelled=Object.assign(Error("cancelled observation"),{code:"ENOENT"});
  let interruption:unknown;try{observeNativeImageVolumeSilenceV1(root,()=>{throw cancelled;});}catch(error){interruption=error;}
  expect(interruption).toBe(cancelled);
  for(const mutation of row.directoryMutations){
   const changed=join(root,mutation),service=join(changed,protocol.serviceDirectory);mkdirSync(changed);logs(changed,row.logs,row.bytesPerLog);let applied=false,refusal:unknown;
   try{observeNativeImageVolumeSilenceV1(changed,()=>{if(applied)return;applied=true;if(mutation==="remove")rmSync(service,{recursive:true});else{renameSync(service,join(changed,"original"));mkdirSync(service);}});}catch(error){refusal=error;}
   expect(applied).toBe(true);expect((refusal as NodeJS.ErrnoException)?.code).toBe("ENOENT");
  }
  console.log("[DEBUG] Native image log census matched the shared rotation vector and propagated original cancellation");
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("the silence corpus is schema-valid and every decision is reproduced",()=>{
 expect(validateJsonSchemaSubset(schema.definitions.Corpus,fixture,schema)).toEqual([]);
 for(const row of fixture.cases){const decision=decideNativeImageVolumeSilenceV1(row.facts as NativeImageVolumeSilenceFactsV1,row.phase as "attach"|"open");expect({name:row.name,actions:[...decision.actions],refusal:decision.refusal}as unknown).toEqual({name:row.name,actions:row.actions,refusal:row.refusal});}
});
test("seeded creation arguments are closed and the seed carries both markers",()=>{
 const seed=createNativeImageVolumeSilenceSeedV1();
 try{
  expect(nativeImageSeededCreateArgumentsV1(66060288,"/x/fixed.dmg",seed.directory)).toEqual(["create","-size","63m","-format","UDRW","-nospotlight","-fs",imageSchema.definitions.ImageProtocol.const.filesystem,"-volname","semio","-srcfolder",seed.directory,"/x/fixed.dmg"]);
  expect(()=>nativeImageSeededCreateArgumentsV1(66060289,"/x",seed.directory)).toThrow("Unit-aligned");
  expect(readdirSync(seed.directory).sort()).toEqual([protocol.spotlightMarker,protocol.serviceDirectory].sort());
  expect(readdirSync(join(seed.directory,protocol.serviceDirectory))).toEqual([protocol.serviceMarker]);
  expect(lstatSync(join(seed.directory,protocol.serviceDirectory)).mode&0o777).toBe(protocol.serviceDirectoryMode);
  expect(nativeImageVolumeSilencedV1(seed.directory,"attach")).toBe(true);
 }finally{seed.dispose();}
 expect(existsSync(seed.directory)).toBe(false);
});
test("observation agrees with an independent census and the policy prunes 400 logs in cancellable batches",async()=>{
 const root=scratchRoot();
 try{
  logs(root,400,1000);writeFileSync(join(root,protocol.serviceDirectory,protocol.uuidFile),"UUID");writeFileSync(join(root,protocol.serviceDirectory,"stray"),"x");
  const facts=observeNativeImageVolumeSilenceV1(root),census=oracle(join(root,protocol.serviceDirectory));
  expect({files:facts.logFiles,bytes:facts.logBytes}).toEqual(census);expect(census.files).toBe(400);
  expect(facts.marker).toBe("absent");expect(nativeImageVolumeSilencedV1(root,"open")).toBe(false);
  const progress:number[][]=[];let checks=0;
  await expect(applyNativeImageVolumeSilenceV1(root,"attach",{check:()=>{if(progress.length>=2)throw Error("cancelled");checks++;},progress:(done,total)=>progress.push([done,total])})).rejects.toThrow("cancelled");
  const partial=oracle(join(root,protocol.serviceDirectory));expect(partial.files).toBeLessThan(400);expect(partial.files).toBeGreaterThan(0);
  const done=await applyNativeImageVolumeSilenceV1(root,"attach",{check:()=>{}});
  expect(done.logFiles).toBe(0);expect(oracle(join(root,protocol.serviceDirectory))).toEqual({files:0,bytes:0});
  expect(existsSync(join(root,protocol.serviceDirectory,protocol.serviceMarker))).toBe(true);expect(existsSync(join(root,protocol.spotlightMarker))).toBe(true);
  expect(existsSync(join(root,protocol.serviceDirectory,protocol.uuidFile))).toBe(true);expect(existsSync(join(root,protocol.serviceDirectory,"stray"))).toBe(true);
  expect(progress.slice(0,2)).toEqual([[64,400],[128,400]]);expect(checks).toBeGreaterThan(0);
  expect(hasNativeImageVolumeSilenceMarkerV1(root)).toBe(true);expect(nativeImageVolumeSilencedV1(root,"attach")).toBe(true);
  await applyNativeImageVolumeSilenceV1(root,"attach",{check:()=>{}});
 }finally{rmSync(root,{recursive:true,force:true});}
});
test("a link as service directory and a service directory the user cannot write are refused with the remedy",async()=>{
 const root=scratchRoot(),linked=scratchRoot();
 try{
  symlinkSync(linked,join(root,protocol.serviceDirectory));
  await expect(applyNativeImageVolumeSilenceV1(root,"attach",{check:()=>{}})).rejects.toThrow("must be a real directory");
  if(process.getuid?.()!==0){
   rmSync(join(root,protocol.serviceDirectory));logs(linked,3);chmodSync(join(linked,protocol.serviceDirectory),0o500);
   await expect(applyNativeImageVolumeSilenceV1(linked,"attach",{check:()=>{}})).rejects.toThrow("hdiutil detach");
   expect(oracle(join(linked,protocol.serviceDirectory)).files).toBe(3);
  }
 }finally{try{chmodSync(join(linked,protocol.serviceDirectory),0o700);}catch{}rmSync(root,{recursive:true,force:true});rmSync(linked,{recursive:true,force:true});}
});
test.skipIf(process.platform!=="darwin")("a seeded image attached with production arguments is user-owned, silenced, unindexed and the same format as the unseeded type",async()=>{
 const scratch=scratchRoot(),attached:string[]=[],seed=createNativeImageVolumeSilenceSeedV1(),protocolImage=imageSchema.definitions.ImageProtocol.const;
 try{
  const mount=join(scratch,"m"),image=join(scratch,"seeded.dmg"),plain=join(scratch,"plain.dmg");mkdirSync(mount);
  expect(run("/usr/bin/hdiutil",nativeImageSeededCreateArgumentsV1(32*1048576,image,seed.directory)).status).toBe(0);
  expect(run("/usr/bin/hdiutil",["create","-size","32m","-type",protocolImage.format,"-nospotlight","-fs",protocolImage.filesystem,"-volname","semio",plain]).status).toBe(0);
  const format=(path:string)=>run("/usr/bin/python3",["-I","-c","import plistlib,subprocess,sys\nprint(plistlib.loads(subprocess.run(['/usr/bin/hdiutil','imageinfo','-plist',sys.argv[1]],capture_output=True).stdout)['Format'])",path]).stdout.trim();
  expect(format(image)).toBe(format(plain));expect(lstatSync(image).size).toBe(lstatSync(plain).size);
  expect(run("/usr/bin/hdiutil",nativeImageAttachArgumentsV1(mount,image)).status).toBe(0);attached.push(mount);
  expect(await settle(join(mount,protocol.serviceDirectory))).toBe(true);
  expect(owner(join(mount,protocol.serviceDirectory))).toBe(process.getuid!());expect(owner(join(mount,protocol.serviceDirectory,protocol.serviceMarker))).toBe(process.getuid!());
  expect(nativeImageVolumeSilencedV1(mount,"attach")).toBe(true);
  await new Promise(resolve=>setTimeout(resolve,2000));
  expect(run("/usr/bin/mdutil",["-s",mount]).stdout).not.toContain("Indexing enabled");
  expect(existsSync(join(mount,".Spotlight-V100"))).toBe(false);
  const facts=await applyNativeImageVolumeSilenceV1(mount,"attach",{check:()=>{}});expect(facts.logFiles).toBe(0);
 }finally{for(const mount of attached.reverse())run("/usr/bin/hdiutil",["detach","-force",mount]);seed.dispose();rmSync(scratch,{recursive:true,force:true});}
},120000);
test.skipIf(process.platform!=="darwin")("a legacy image with an accumulated log directory is repaired at attach and a root-owned service directory is refused",async()=>{
 const scratch=scratchRoot(),attached:string[]=[],protocolImage=imageSchema.definitions.ImageProtocol.const;
 try{
  const legacy=join(scratch,"legacy.dmg"),mine=join(scratch,"mine"),theirs=join(scratch,"theirs");mkdirSync(mine);mkdirSync(theirs);
  expect(run("/usr/bin/hdiutil",["create","-size","32m","-type",protocolImage.format,"-nospotlight","-fs",protocolImage.filesystem,"-volname","semio",legacy]).status).toBe(0);
  expect(run("/usr/bin/hdiutil",["attach","-nobrowse","-noautoopen","-owners","on","-mountpoint",theirs,legacy]).status).toBe(0);attached.push(theirs);
  if(await settle(join(theirs,protocol.serviceDirectory))&&owner(join(theirs,protocol.serviceDirectory))!==process.getuid!()){
   await expect(applyNativeImageVolumeSilenceV1(theirs,"attach",{check:()=>{}})).rejects.toThrow("hdiutil detach");
   expect(existsSync(join(theirs,protocol.serviceDirectory,protocol.serviceMarker))).toBe(false);
  }
  run("/usr/bin/hdiutil",["detach","-force",theirs]);attached.pop();
  expect(run("/usr/bin/hdiutil",["attach","-nobrowse","-noautoopen","-owners","off","-mountpoint",mine,legacy]).status).toBe(0);attached.push(mine);
  await settle(join(mine,protocol.serviceDirectory));logs(mine,250,2000);
  const before=oracle(join(mine,protocol.serviceDirectory));expect(before.files).toBe(250);
  expect(nativeImageVolumeSilencedV1(mine,"open")).toBe(false);
  const facts=await applyNativeImageVolumeSilenceV1(mine,"attach",{check:()=>{}});
  expect(facts.logFiles).toBe(0);expect(oracle(join(mine,protocol.serviceDirectory)).files).toBe(0);
  expect(existsSync(join(mine,protocol.serviceDirectory,protocol.serviceMarker))).toBe(true);expect(nativeImageVolumeSilencedV1(mine,"open")).toBe(true);
 }finally{for(const mount of attached.reverse())run("/usr/bin/hdiutil",["detach","-force",mount]);rmSync(scratch,{recursive:true,force:true});}
},120000);

import "../../../🌐️platform/📋️contract/📂️mount/🧪️tests/🟦️.ts";
