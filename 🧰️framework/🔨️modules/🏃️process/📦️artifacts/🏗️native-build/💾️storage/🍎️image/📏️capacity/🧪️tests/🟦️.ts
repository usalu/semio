import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,realpathSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import fixture from"./🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import{nativeImageVolumeBytesV1,nativeImageVolumeMetadataBytesV1,nativeImageUsableBytesV1}from"../🟦️.ts";
import{createNativeImageVolumeSilenceSeedV1,nativeImageSeededCreateArgumentsV1}from"../../🤫️silence/🟦️.ts";
const run=(program:string,args:readonly string[])=>{const result=spawnSync(program,[...args],{encoding:"utf8",env:{PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"}});return{status:result.status,stdout:result.stdout,stderr:result.stderr};};
const statvfs=(path:string):{total:number;used:number}=>JSON.parse(run("/usr/bin/python3",["-I","-c","import os,sys,json;s=os.statvfs(sys.argv[1]);print(json.dumps({'total':s.f_blocks*s.f_frsize,'used':(s.f_blocks-s.f_bfree)*s.f_frsize}))",path]).stdout);
test("the capacity corpus is schema-valid and the model matches every measured volume",()=>{
 expect(validateJsonSchemaSubset(schema.definitions.Corpus,fixture,schema)).toEqual([]);
 for(const row of fixture.measured){expect(nativeImageVolumeBytesV1(row.imageBytes)).toBe(row.filesystemBytes);expect(nativeImageVolumeMetadataBytesV1(row.imageBytes)).toBeGreaterThanOrEqual(row.usedAtEmptyBytes);}
 for(const row of fixture.usable){expect(nativeImageUsableBytesV1(row.imageBytes)).toBe(row.usableBytes);expect(row.usableBytes).toBeLessThan(nativeImageVolumeBytesV1(row.imageBytes));}
 expect(()=>nativeImageUsableBytesV1(1024)).toThrow("minimum filesystem");
});
test("a 63 MiB lease-sized image keeps 63.14 MB usable instead of the nominal 66.06 MB",()=>{
 expect(nativeImageUsableBytesV1(66060288)).toBeLessThan(66060288-2000000);
});
test.skipIf(process.platform!=="darwin")("the model agrees with statvfs and the used bytes of real seeded images of three sizes",()=>{
 const scratch=realpathSync(mkdtempSync(join(process.env.TMPDIR||tmpdir(),"capacity-"))),attached:string[]=[],seed=createNativeImageVolumeSilenceSeedV1();
 try{
  for(const megabytes of[32,63,128]){
   const imageBytes=megabytes*1048576,image=join(scratch,megabytes+".dmg"),mount=join(scratch,"m"+megabytes);mkdirSync(mount);
   expect(run("/usr/bin/hdiutil",nativeImageSeededCreateArgumentsV1(imageBytes,image,seed.directory)).status).toBe(0);
   expect(run("/usr/bin/hdiutil",["attach","-nobrowse","-noautoopen","-owners","on","-mountpoint",mount,image]).status).toBe(0);attached.push(mount);
   const observed=statvfs(mount);expect(observed.total).toBe(nativeImageVolumeBytesV1(imageBytes));expect(observed.used).toBeLessThanOrEqual(nativeImageVolumeMetadataBytesV1(imageBytes));
  }
 }finally{
  for(const mount of attached.reverse())run("/usr/bin/hdiutil",["detach","-force",mount]);
  seed.dispose();rmSync(scratch,{recursive:true,force:true});
 }
},120000);
