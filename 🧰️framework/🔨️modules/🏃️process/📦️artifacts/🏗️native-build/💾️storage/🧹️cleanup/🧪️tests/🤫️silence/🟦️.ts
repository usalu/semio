import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{existsSync,mkdirSync,mkdtempSync,readdirSync,realpathSync,rmSync,unlinkSync,writeFileSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{createIssuedResourceControlV1,receiveScriptProcessInvocation}from"../../../../../../📦️packages/🟦️typescript/🧭️routing/🟦️.ts";
import{openNativePlatformStorageV1}from"../../../🌐️platform/🟦️.ts";
import{observeNativeVolumeEventServiceV1}from"../../../📤️publication/📚️witness/🍎️events/🟦️.ts";
import{captureNativeImagePublicationWitnessV1}from"../../../📤️publication/📚️witness/🟦️.ts";
import silence from"../../../🍎️image/🤫️silence/🧬️schema/🔣️.json"with{type:"json"};
const protocol=silence.definitions.Protocol.const;
const run=(program:string,args:readonly string[])=>spawnSync(program,[...args],{encoding:"utf8",env:{PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"}});
const census=(directory:string)=>JSON.parse(run("/usr/bin/python3",["-I","-c","import os,sys,re,json\np=re.compile(sys.argv[2])\nprint(json.dumps(sorted(e.name for e in os.scandir(sys.argv[1]) if p.fullmatch(e.name))))",directory,protocol.logPattern]).stdout)as string[];
const owner=(path:string)=>Number(run("/usr/bin/python3",["-I","-c","import os,sys;print(os.lstat(sys.argv[1]).st_uid)",path]).stdout);
const drift=(root:string,logs:number,marker:boolean)=>{const service=join(root,protocol.serviceDirectory);for(let index=0;index<logs;index++)writeFileSync(join(service,(index+1).toString(16).padStart(16,"0")),"x".repeat(500));if(!marker)unlinkSync(join(service,protocol.serviceMarker));};
/** 🤫️ Proves the product open path silences a fresh image at birth, repairs a drifted mounted image when its exclusive lease is free, and defers while another reader holds it. */
test.skipIf(process.platform!=="darwin")("fresh birth silences the image and a drifted mounted image is repaired at the next open even while another reader holds it",async()=>{
 const scratch=realpathSync(mkdtempSync(join(process.env.TMPDIR||tmpdir(),"sl-"))),directory=join(scratch,"img"),leaseDirectory=join(scratch,"leases"),root=join(directory,"m"),service=join(root,protocol.serviceDirectory);mkdirSync(leaseDirectory);
 const options=(control:ReturnType<typeof createIssuedResourceControlV1>)=>({directory,leaseDirectory,filesystemBytes:33554432,maximumPhysicalBytes:34603008,control});
 try{
  await receiveScriptProcessInvocation(process.env,async original=>{
   const control=(name:string)=>createIssuedResourceControlV1(original,name,0);
   const first=await openNativePlatformStorageV1(options(control("cache:silence-birth")));
   expect(existsSync(join(service,protocol.serviceMarker))).toBe(true);expect(existsSync(join(root,protocol.spotlightMarker))).toBe(true);expect(owner(service)).toBe(process.getuid!());expect(census(service)).toEqual([]);
   const probe=control("cache:silence-witness");
   expect(await observeNativeVolumeEventServiceV1(root,probe,()=>{})).toBeNull();
   const witness=await captureNativeImagePublicationWitnessV1(root,probe,16777216,()=>{},{platform:"posix"});expect(witness.service).toBeNull();expect(witness.files).toBeGreaterThan(0);
   drift(root,12,false);
   const second=await openNativePlatformStorageV1(options(control("cache:silence-busy")));
   expect(census(service)).toEqual([]);expect(existsSync(join(service,protocol.serviceMarker))).toBe(true);expect(readdirSync(root).sort()).toContain(protocol.spotlightMarker);
   drift(root,3,true);
   const third=await openNativePlatformStorageV1(options(control("cache:silence-residue")));
   expect(census(service).length).toBe(3);
   await third.close();await second.close();await first.close();
  });
 }finally{run("/usr/bin/hdiutil",["detach","-force",root]);rmSync(scratch,{recursive:true,force:true});}
},180000);
