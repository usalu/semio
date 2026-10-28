/** 🪞️ Tests for storage directory fold. */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{existsSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,writeFileSync}from"node:fs";
import{join}from"node:path";
import{testScratchRootV1}from"../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{createNativeImageVolumeSilenceSeedV1,nativeImageSeededCreateArgumentsV1}from"../../../🍎️image/🤫️silence/🟦️.ts";
import{nativeImageAttachArgumentsV1}from"../../../🍎️image/👤️owners/🟦️.ts";
import{allocateNativeDirectoryV1,nativeDirectoryCaseModeV1,nativeDirectoryFoldCollisionsV1,nativeDirectoryProtocolV1,readNativeDirectoryAnchorFileV1,type NativeDirectoryPlatformPortV1}from"../../🟦️.ts";
const protocol=nativeDirectoryProtocolV1(),scratch=()=>realpathSync(mkdtempSync(join(testScratchRootV1("fold"),"fold-")));
const port=(kind:"exchange"|"journaled",prepare:()=>void):NativeDirectoryPlatformPortV1=>({kind,swap:()=>{},flush:()=>{},pending:()=>false,recover:()=>"clean",prepareVolume:prepare});
const place=(base:string,name:string)=>{const directory=join(base,name);mkdirSync(join(directory,protocol.root),{recursive:true});return directory;};
const envelope={filesystemBytes:33554432,maximumPhysicalBytes:33554432};
test("a Windows host without per-directory case sensitivity declares the partition fold-safe, any other refusal still stops the allocation",()=>{
 const base=scratch();try{
  for(const key of protocol.foldSafeRefusals){
   const directory=place(base,key);allocateNativeDirectoryV1({directory,...envelope},()=>false,port("journaled",()=>{throw Error((protocol.refusals as Record<string,string>)[key]);}));
   expect(nativeDirectoryCaseModeV1(directory)).toBe("fold-safe");expect(readNativeDirectoryAnchorFileV1(directory).caseFold).toBe("insensitive");
  }
  for(const key of["caseAccess","caseNotEmpty","caseUnverified"]){
   const directory=place(base,key);expect(()=>allocateNativeDirectoryV1({directory,...envelope},()=>false,port("journaled",()=>{throw Error((protocol.refusals as Record<string,string>)[key]);}))).toThrow();
   expect(existsSync(join(directory,protocol.volume)),key).toBe(false);expect(existsSync(join(directory,protocol.anchor)),key).toBe(false);
  }
  const posix=place(base,"posix");expect(()=>allocateNativeDirectoryV1({directory:posix,...envelope},()=>false,port("exchange",()=>{}))).toThrow(protocol.refusals.caseInsensitive);
  const sensitive=place(base,"sensitive");allocateNativeDirectoryV1({directory:sensitive,...envelope},()=>true,port("journaled",()=>{}));expect(nativeDirectoryCaseModeV1(sensitive)).toBe("sensitive");expect(readNativeDirectoryAnchorFileV1(sensitive).caseFold).toBeUndefined();
 }finally{rmSync(base,{recursive:true,force:true});}
});
test("the collision census groups names that differ by case, normalization or script case, and leaves distinct names alone",()=>{
 expect(nativeDirectoryFoldCollisionsV1(["README.md","readme.md","Readme.MD","src","lib"])).toEqual([["README.md","Readme.MD","readme.md"]]);
 expect(nativeDirectoryFoldCollisionsV1(["caf\u00e9","cafe\u0301","CAF\u00c9"])).toEqual([["caf\u00e9","cafe\u0301","CAF\u00c9"].sort()]);
 expect(nativeDirectoryFoldCollisionsV1(["a","b","c"])).toEqual([]);
 expect(nativeDirectoryFoldCollisionsV1(["a","A","b","B","c"])).toEqual([["A","a"],["B","b"]]);
 expect(nativeDirectoryFoldCollisionsV1(["ß","ss"])).toEqual([]);
});
const run=(args:readonly string[])=>{const row=spawnSync("/usr/bin/hdiutil",[...args],{encoding:"utf8"});if(row.status!==0)throw Error("hdiutil "+args.join(" ")+": "+row.stderr);return row.stdout;};
test.skipIf(process.platform!=="darwin")("every pair a real case-insensitive APFS volume merges is flagged by the census",()=>{
 const base=scratch(),seed=createNativeImageVolumeSilenceSeedV1();let mount="";
 try{
  const image=join(base,"fold.dmg");mount=join(base,"m");mkdirSync(mount);
  const create=[...nativeImageSeededCreateArgumentsV1(67108864,image,seed.directory)];create[create.indexOf("Case-sensitive APFS")]="APFS";run(create);run(nativeImageAttachArgumentsV1(mount,image));
  const names=["a","A","Readme","README","readme","café","café","CAFÉ","Straße","STRASSE","strasse","İ","i","ǅ","ǆ","Ǆ","ω","Ω","ꙁ","Ꙁ","x","y"],survivors=new Map<string,string>();
  for(const name of names){try{writeFileSync(join(mount,name),name,{flag:"wx"});survivors.set(name,name);}catch(error){if((error as NodeJS.ErrnoException).code!=="EEXIST")throw error;}}
  const stored=readdirSync(mount).filter(name=>!name.startsWith("."));expect(stored.length).toBeLessThan(names.length);
  const merged=names.filter(name=>!survivors.has(name)),groups=nativeDirectoryFoldCollisionsV1(names);
  for(const name of merged)expect(groups.some(group=>group.includes(name)),name).toBe(true);
 }finally{seed.dispose();try{run(["detach",mount]);}catch{}rmSync(base,{recursive:true,force:true});}
},60000);
