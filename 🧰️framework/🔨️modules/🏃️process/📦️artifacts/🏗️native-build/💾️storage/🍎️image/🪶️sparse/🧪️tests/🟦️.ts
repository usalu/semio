/** 🪶️ Tests for image sparse. */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{lstatSync,mkdirSync,mkdtempSync,realpathSync,rmSync,statfsSync}from"node:fs";
import{join}from"node:path";
import{testScratchRootV1}from"../../../../../../🧪️testing/🏖️sandbox/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import imageSchema from"../../../🧬️schema/🔣️.json"with{type:"json"};
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import fixture from"./🔣️.json"with{type:"json"};
import{nativeImageVolumeBytesV1}from"../../📏️capacity/🟦️.ts";
import{createNativeImageVolumeSilenceSeedV1,nativeImageSeededCreateArgumentsV1}from"../../🤫️silence/🟦️.ts";
import{nativeImageAttachArgumentsV1}from"../../👤️owners/🟦️.ts";
import{nativeImageHdiutilProgramV1}from"../../🛠️hdiutil/🟦️.ts";
import{nativeImageBackingAllocatedBytesV1}from"../../../🪶️extent/🟦️.ts";
import{nativeImageSparseAdmissionBytesV1,nativeImageSparseGrowArgumentsV1,nativeImageSparseInitialBytesV1}from"../🟦️.ts";
const protocol=schema.definitions.Protocol.const,unit=imageSchema.definitions.ImageProtocol.const.unitBytes,MiB=1048576,GiB=1073741824;
const env={PATH:"/usr/bin:/bin:/usr/sbin:/sbin",LANG:"C",LC_ALL:"C"};
const run=(program:string,args:readonly string[])=>{const row=spawnSync(program,[...args],{encoding:"utf8",env});if(row.status!==0)throw Error(program+" "+args.join(" ")+": "+row.stderr);return row.stdout;};
const oracle=(root:string,probe:string,payload:number)=>JSON.parse(run("/usr/bin/python3",["-I",join(import.meta.dir,"🐍️.py"),root,probe,String(payload)]))as{swap:number;after:string[][];caseDistinct:string[];volumeBytes:number};
test("the corpus is schema-valid and the capacity model equals every measured sparse volume",()=>{
 expect(validateJsonSchemaSubset(schema.definitions.Corpus,fixture,schema)).toEqual([]);
 for(const row of fixture.measured){expect(row.backingBytes-row.imageBytes).toBe(protocol.trailerBytes);expect(row.volumeBytes).toBe(row.imageBytes);expect(nativeImageVolumeBytesV1(row.imageBytes)).toBe(row.volumeBytes);}
});
test("the creation size and the grow arguments are closed and unit-aligned",()=>{
 expect(nativeImageSparseInitialBytesV1(63*MiB)).toBe(63*MiB);expect(nativeImageSparseGrowArgumentsV1(63*MiB,"/x/fixed.dmg")).toBeNull();
 expect(nativeImageSparseGrowArgumentsV1(protocol.initialBytes,"/x/fixed.dmg")).toBeNull();
 expect(nativeImageSparseInitialBytesV1(25*GiB)).toBe(protocol.initialBytes);
 expect(nativeImageSparseGrowArgumentsV1(25*GiB,"/x/fixed.dmg")).toEqual(["resize","-size","25600m","/x/fixed.dmg"]);
 expect(()=>nativeImageSparseInitialBytesV1(25*GiB+1)).toThrow("Unit-aligned");expect(()=>nativeImageSparseInitialBytesV1(MiB)).toThrow("minimum filesystem");
 expect(nativeImageSparseAdmissionBytesV1(25*GiB)).toBe(protocol.admissionFreeBytes+64*MiB);expect(nativeImageSparseAdmissionBytesV1(8*MiB)).toBe(72*MiB);
 expect(nativeImageBackingAllocatedBytesV1({size:7})).toBe(7);expect(nativeImageBackingAllocatedBytesV1({size:7n})).toBe(7n);
 expect(unit).toBe(MiB);
});
test.skipIf(process.platform!=="darwin")("a real sparse image costs its content, keeps one flat file and the swap and case guarantees",()=>{
 const directory=realpathSync(mkdtempSync(join(testScratchRootV1("sparse"),"sparse-"))),seed=createNativeImageVolumeSilenceSeedV1();let root="";
 try{
  for(const row of fixture.measured.filter(item=>item.imageBytes<=2*GiB)){
   const image=join(directory,"fixed"+row.imageBytes+".dmg");root=join(directory,"m"+row.imageBytes);mkdirSync(root);
   run(nativeImageHdiutilProgramV1,nativeImageSeededCreateArgumentsV1(row.imageBytes,image,seed.directory));
   const grow=nativeImageSparseGrowArgumentsV1(row.imageBytes,image);expect(grow).not.toBeNull();run(nativeImageHdiutilProgramV1,grow!);
   const file=lstatSync(image);expect(file.isFile()&&file.nlink===1).toBe(true);expect(file.size).toBe(row.backingBytes);
   expect(file.blocks*512).toBeLessThan(fixture.initialAllocatedLimitBytes);expect(nativeImageBackingAllocatedBytesV1(file)).toBe(row.backingBytes);
   const du=Number(run("/usr/bin/du",["-k",image]).split(/\s/u)[0])*1024;expect(du).toBe(file.blocks*512);
   run(nativeImageHdiutilProgramV1,nativeImageAttachArgumentsV1(root,image));
   try{
    const fs=statfsSync(root);expect(fs.blocks*fs.bsize).toBe(row.volumeBytes);
    const payload=row.imageBytes>=512*MiB?192*MiB:8*MiB,result=oracle(root,join(root,"probe"),payload);expect(result.swap).toBe(0);expect(result.after).toEqual([["y"],["x"]]);expect(result.caseDistinct).toEqual(["Case","case"]);expect(result.volumeBytes).toBe(row.volumeBytes);
    const grown=lstatSync(image);expect(grown.size).toBe(row.backingBytes);expect(grown.ino).toBe(file.ino);expect(grown.blocks*512).toBeGreaterThanOrEqual(file.blocks*512);expect(grown.blocks*512).toBeLessThan(file.blocks*512+payload+64*MiB);if(payload>MiB*100)expect(grown.blocks*512).toBeGreaterThanOrEqual(file.blocks*512+100*MiB);
   }finally{run(nativeImageHdiutilProgramV1,["detach",root]);}
   expect(lstatSync(image).ino).toBe(file.ino);rmSync(image);
  }
 }finally{seed.dispose();spawnSync(nativeImageHdiutilProgramV1,["detach",root],{env});rmSync(directory,{recursive:true,force:true});}
},120000);
