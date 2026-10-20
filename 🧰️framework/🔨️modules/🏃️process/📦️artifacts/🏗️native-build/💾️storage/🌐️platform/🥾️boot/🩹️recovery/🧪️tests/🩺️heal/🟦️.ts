import{expect,test}from"bun:test";
import{createHash}from"node:crypto";
import{chmodSync,existsSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{dirname,join,resolve}from"node:path";
import{acquireResourceLease,type ResourceLeaseControl}from"../../../../../../../../🔒️leases/🟦️.ts";
import{healNativeLeakedImagesV1,healNativeProcessBootV1,nativeBootImageRosterV1,nativeBootLedgerDirectoryV1,settleNativePartitionPublicationV1,type NativeBootHostV1,type NativeBootRosterImageV1}from"../../🩺️heal/🟦️.ts";
import{loadNativeBootLedgerV1,nativeBootLedgerPathV1}from"../../🟦️.ts";

const BOOT="1E4A0CBE-3710-481F-9614-F8F1FCBD61D0",DAY=86400000,STEM="a".repeat(64);
const scratch=()=>{const base=process.env.SEMIO_TEST_ARTIFACT_DIR?resolve(process.env.SEMIO_TEST_ARTIFACT_DIR):tmpdir();mkdirSync(base,{recursive:true});return realpathSync(mkdtempSync(join(base,"hl-")));};
const sha=(text:string|Buffer)=>createHash("sha256").update(text).digest("hex");
const b36=(n:number,w:number)=>n.toString(36).padStart(w,"0");
function control(published:string[]=[]):ResourceLeaseControl{return{signal:new AbortController().signal,remainingMilliseconds:()=>null,publish:progress=>{published.push(progress.resource);},yieldContinuation:async()=>{}};}
const age=(path:string,ms:number)=>{const at=(Date.now()-ms)/1000;utimesSync(path,at,at);};
const image=(root:string,partition:string)=>{const path=join(root,".cache","b",partition,"fixed.dmg");mkdirSync(dirname(path),{recursive:true});writeFileSync(path,"x");return path;};

test("dead tickets and abandoned scratch images are recovered once per process through the injected host",async()=>{
 const root=scratch();try{
  const workspace=join(root,"ws"),leaseDirectory=join(workspace,".cache","b","c"),queue=join(leaseDirectory,STEM+".queue");mkdirSync(join(leaseDirectory,"claims"),{recursive:true});mkdirSync(queue);
  const dead=`${b36(Date.now()-3*DAY,10)}-${b36(8903,7)}-Qk3v8XhT2mZpLw0aYcR1dg`,live=`${b36(Date.now()+60000,10)}-${b36(4410,7)}-n1HfT9sLd0QwYb7xKa2VcA`;
  writeFileSync(join(queue,dead),"{}");writeFileSync(join(queue,live),"{}");
  const leak=join(root,"leak"),leakLeases=join(leak,".cache","b","c");mkdirSync(leakLeases,{recursive:true});
  const idle=image(leak,"l"),fresh=image(leak,"t"),held=image(leak,"o"),inside=image(workspace,"d"),loose=join(root,"loose","fixed.dmg"),gone=join(root,"gone",".cache","b","p","fixed.dmg");
  mkdirSync(dirname(loose),{recursive:true});writeFileSync(loose,"x");
  for(const path of[idle,held,inside,loose])age(path,DAY);
  const attached=new Map<string,string>([[idle,"/dev/disk91"],[fresh,"/dev/disk92"],[held,"/dev/disk93"],[inside,"/dev/disk94"],[loose,"/dev/disk95"],[gone,"/dev/disk96"]]),detached:string[]=[];
  const host:NativeBootHostV1={clock:async()=>({bootSession:BOOT,bootEpochMs:Date.now()}),capture:async(_program,args)=>{if(args[0]==="detach"){for(const[path,device]of attached)if(device===args[1])attached.delete(path);detached.push(args[1]!);}return"";}};
  const roster=async():Promise<readonly NativeBootRosterImageV1[]>=>[...attached].map(([imagePath,device])=>({imagePath,device}));
  const published:string[]=[],owner=control(published),holder=await acquireResourceLease({directory:leakLeases,resource:"native-fixed-image:"+dirname(held),mode:"exclusive",signal:owner.signal,control:owner,timeoutMs:0});
  try{await healNativeProcessBootV1({leaseDirectory,control:owner,host,roster});}finally{holder.release();}
  expect(detached.sort()).toEqual(["/dev/disk91","/dev/disk96"]);
  expect(readdirSync(queue)).toEqual([live]);
  expect(published.some(row=>row.startsWith("boot-recovery:commit:"))).toBe(true);
  expect(loadNativeBootLedgerV1(nativeBootLedgerPathV1(nativeBootLedgerDirectoryV1(leaseDirectory),leaseDirectory+"#images")).state.state).toBe("committed");
  attached.set(idle,"/dev/disk97");age(idle,2*DAY);
  await healNativeProcessBootV1({leaseDirectory,control:owner,host,roster});
  expect(detached.length,"the second call of one process observes nothing").toBe(2);
  expect(await healNativeLeakedImagesV1({leaseDirectory,control:owner,host,roster}),"a second process within the interval skips the sweep").toBe("recent");expect(detached.length).toBe(2);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a failing recovery never blocks the caller but a cancelled one always propagates",async()=>{
 const root=scratch();try{
  const leaseDirectory=join(root,"ws",".cache","b","c");mkdirSync(join(leaseDirectory,"claims"),{recursive:true});
  const published:string[]=[],host:NativeBootHostV1={clock:async()=>{throw Error("clock unavailable");},capture:async()=>""};
  await healNativeProcessBootV1({leaseDirectory,control:control(published),host,roster:async()=>[]});
  expect(published.some(row=>row.startsWith("boot-recovery:failed:clock unavailable"))).toBe(true);
  const other=join(root,"other",".cache","b","c");mkdirSync(join(other,"claims"),{recursive:true});const controller=new AbortController();controller.abort(Error("stop"));
  await expect(healNativeProcessBootV1({leaseDirectory:other,control:{...control(),signal:controller.signal},host,roster:async()=>[]})).rejects.toThrow("stop");
 }finally{rmSync(root,{recursive:true,force:true});}
});

function publication(root:string,stage:"rolled-back"|"unknown"){
 mkdirSync(root,{recursive:true});const receipt=Buffer.from('{"version":1,"generation":"old"}\n'),base=join(root,".publication");mkdirSync(base,{mode:0o700});chmodSync(base,0o700);
 const put=(name:string,bytes:Buffer|string)=>{writeFileSync(join(base,name),bytes,{mode:0o600});chmodSync(join(base,name),0o600);};
 writeFileSync(join(root,"image.json"),receipt,{mode:0o600});chmodSync(join(root,"image.json"),0o600);put("intent.json",JSON.stringify({version:1,receiptSha256:sha(receipt)})+"\n");
 if(stage==="unknown")put("stranger.txt","x");
 return receipt;
}

test("publication residue of a killed publisher is settled under the held lease and unknown residue is refused with its reason",async()=>{
 const root=scratch();try{
  const leaseDirectory=join(root,".cache","b","c"),directory=join(root,"p"),published:string[]=[],host:NativeBootHostV1={clock:async()=>({bootSession:BOOT,bootEpochMs:Date.now()}),capture:async()=>""};
  mkdirSync(leaseDirectory,{recursive:true});const old=publication(directory,"rolled-back");
  await settleNativePartitionPublicationV1({directory,leaseDirectory,control:control(published),host});
  expect(existsSync(join(directory,".publication"))).toBe(false);expect(published.some(row=>row.startsWith("boot-recovery:step:"))).toBe(true);expect(readdirSync(directory)).toEqual(["image.json"]);expect(old.length).toBeGreaterThan(0);
  const unknown=join(root,"q");publication(unknown,"unknown");
  await expect(settleNativePartitionPublicationV1({directory:unknown,leaseDirectory,control:control(),host})).rejects.toThrow("retained publication residue of "+unknown+": unknown-shape");
  expect(readdirSync(join(unknown,".publication")).sort()).toEqual(["intent.json","stranger.txt"]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

const run=(program:string,args:readonly string[])=>{const row=Bun.spawnSync([program,...args],{stdout:"pipe",stderr:"pipe"});if(row.exitCode!==0)throw Error(`${program} ${args.join(" ")} failed: ${row.stderr.toString().slice(0,300)}`);return row.stdout.toString();};
test.skipIf(process.platform!=="darwin")("a really attached abandoned scratch image is detached by the kernel roster sweep and a recently used one is not",async()=>{
 const root=scratch();const mounts:string[]=[];
 try{
  const workspace=join(root,"ws"),leaseDirectory=join(workspace,".cache","b","c"),leak=join(root,"leak");mkdirSync(leaseDirectory,{recursive:true});mkdirSync(join(leak,".cache","b","c"),{recursive:true});
  const make=(partition:string)=>{const path=join(leak,".cache","b",partition,"fixed.dmg"),mount=join(leak,".cache","b",partition,"m");mkdirSync(mount,{recursive:true});run("/usr/bin/hdiutil",["create","-size","16m","-fs","APFS","-volname",partition,"-ov",path]);run("/usr/bin/hdiutil",["attach","-nobrowse","-mountpoint",mount,path]);mounts.push(mount);return path;};
  const abandoned=make("l"),recent=make("t");age(abandoned,DAY);
  const host:NativeBootHostV1={clock:async()=>({bootSession:BOOT,bootEpochMs:Date.now()}),capture:async(program,args)=>run(program,args)},owner=control();
  const real=nativeBootImageRosterV1(owner,host,root),roster=async()=>(await real()).filter(row=>row.imagePath===abandoned||row.imagePath===recent);
  expect((await roster()).map(row=>row.imagePath).sort()).toEqual([abandoned,recent].sort());
  await healNativeProcessBootV1({leaseDirectory,control:owner,host,roster});
  expect((await roster()).map(row=>row.imagePath)).toEqual([recent]);
 }finally{for(const mount of mounts)try{run("/usr/bin/hdiutil",["detach","-force",mount]);}catch{}rmSync(root,{recursive:true,force:true});}
},120000);
