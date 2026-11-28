/** 🪫️ Tests for the image relief: the clamp, the right-size decision, the host pressure and the backoff equal an independent oracle and the measured rows of the incident, and a writer that meets a full image relieves it and writes again, or ends in a typed refusal. */
import{test,expect}from"bun:test";
import{spawnSync}from"node:child_process";
import{existsSync,mkdtempSync,readdirSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{testPythonV1}from"../../../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import{validateJsonSchemaSubset}from"../../../../../../../🧬️schema/✅️validator/📦️packages/🟦️typescript/🟦️.ts";
import schema from"../🧬️schema/🔣️.json"with{type:"json"};
import corpus from"./🔣️.json"with{type:"json"};
import{NATIVE_IMAGE_RELIEF_V1,NativeImageFullErrorV1,nativeImageBackoffMillisecondsV1,nativeImageClampV1,nativeImageFullTextV1,nativeImagePressureV1,nativeImageReliefTextV1,nativeImageRightSizeV1,nativeImageShrinkV1,nativeImageWritesRefusedTextV1,NativeImageWritesRefusedErrorV1,probeNativeImageWritesV1,retryOnImageFullV1,type NativeImageReliefPortsV1}from"../🟦️.ts";
const GiB=1073741824;
const oracle=(...args:string[])=>{const python=testPythonV1();if(python===null)throw Error("Python required by the relief oracle");const result=spawnSync(python,["-I",join(import.meta.dir,"🐍️.py"),...args],{encoding:"utf8"});if(result.status!==0)throw result.error??Error(result.stderr);return JSON.parse(result.stdout.trim());};
const answers=oracle("answers",join(import.meta.dir,"🔣️.json"));
const full=()=>Object.assign(Error("ENOSPC: no space left on device, open '/x/.leases/a.json'"),{code:"ENOSPC"});
const ports=(over:Partial<NativeImageReliefPortsV1>={},log:string[]=[]):NativeImageReliefPortsV1=>({image:"sealed",facts:()=>({image:"sealed",partition:"/x",capacityBytes:26*GiB,availableBytes:0,innerFreeBytes:25*GiB,hostFreeBytes:3*GiB}),release:()=>{log.push("release");return false;},reclaim:async demand=>{log.push("reclaim:"+Math.round(demand/GiB));return GiB;},sleep:async ms=>{log.push("sleep:"+ms);},...over});
test("the corpus is schema-valid and the protocol is the policy",()=>{expect(validateJsonSchemaSubset(schema.definitions.Corpus,corpus,schema)).toEqual([]);expect(NATIVE_IMAGE_RELIEF_V1.attempts).toBe(3);});
test("the clamp equals the oracle and the measured rows: sealed and w report a few GiB free of 26 and 27 although they hold under 1 GiB",()=>{
 const ours=corpus.clamp.map(row=>nativeImageClampV1(row));expect(ours).toEqual(answers.clamp);expect(ours).toEqual(corpus.clamp);
 const sealed=nativeImageClampV1({capacityBytes:26*GiB,usedBytes:Math.floor(0.8*GiB),availableBytes:Math.floor(5.4*GiB)});expect(sealed.clamped).toBe(true);expect(sealed.starved).toBe(false);expect(sealed.phantomBytes).toBeGreaterThan(19*GiB);
 expect(nativeImageClampV1({capacityBytes:26*GiB,usedBytes:GiB,availableBytes:0})).toMatchObject({clamped:true,starved:true});
 expect(nativeImageClampV1({capacityBytes:11*GiB,usedBytes:Math.floor(0.3*GiB),availableBytes:Math.floor(10.5*GiB)}).clamped).toBe(false);
});
test("the right-size decision equals the oracle: only a clamped image that is oversized by a whole step is re-created",()=>{
 const ours=corpus.rightSize.map(row=>({capacityBytes:row.capacityBytes,sizedBytes:row.sizedBytes,clamped:row.clamped,retire:nativeImageRightSizeV1(row).retire}));expect(ours).toEqual(answers.rightSize);expect(ours).toEqual(corpus.rightSize);
 expect(nativeImageRightSizeV1({capacityBytes:26*GiB,sizedBytes:8*GiB,clamped:true}).targetBytes).toBe(8*GiB);expect(nativeImageRightSizeV1({capacityBytes:4*GiB,sizedBytes:8*GiB,clamped:true}).targetBytes).toBe(4*GiB);
});
test("the host pressure equals the oracle: hard below the hard watermark or with a starved image, soft below the soft one",()=>{
 const ours=corpus.pressure.map(row=>({hostFreeBytes:row.hostFreeBytes,images:row.images,...nativeImagePressureV1(row)}));expect(JSON.parse(JSON.stringify(ours))).toEqual(answers.pressure);expect(ours.map(row=>row.level)).toEqual(["none","soft","hard","hard","soft","critical","critical"]);
});
test("the backoff equals the oracle and the last step holds",()=>{
 const ours=corpus.backoff.map(row=>({attempt:row.attempt,milliseconds:nativeImageBackoffMillisecondsV1(row.attempt)}));expect(ours).toEqual(answers.backoff);expect(nativeImageBackoffMillisecondsV1(0)).toBe(0);
});
test("a write that meets a full image is relieved and written again instead of failing the task",async()=>{
 const log:string[]=[];let tries=0;
 const value=await retryOnImageFullV1(()=>{if(++tries<3)throw full();return"written";},ports({},log));
 expect(value).toBe("written");expect(tries).toBe(3);expect(log).toEqual(["release","reclaim:37","sleep:0","release","reclaim:37","sleep:2000"]);
});
test("a released reserve file is the whole relief of the first attempt, and an error that is no full disk passes through untouched",async()=>{
 const log:string[]=[];let tries=0;
 expect(await retryOnImageFullV1(()=>{if(++tries<2)throw full();return 7;},ports({release:()=>{log.push("release");return true;}},log))).toBe(7);expect(log[1]).toBe("reclaim:0");
 const other=Error("EACCES");await expect(retryOnImageFullV1(()=>{throw other;},ports())).rejects.toBe(other);
 let calls=0;await expect(retryOnImageFullV1(()=>{calls++;return Promise.reject(Error("database or disk is full"));},ports({reclaim:async()=>0}))).rejects.toBeInstanceOf(NativeImageFullErrorV1);expect(calls).toBe(1+NATIVE_IMAGE_RELIEF_V1.attempts);
});
test("a write that stays refused ends in the typed refusal that names the image, what is usable, what is free inside and the host, in both languages",async()=>{
 const slept:number[]=[];const error=await retryOnImageFullV1(()=>{throw full();},ports({sleep:async ms=>{slept.push(ms);}})).catch(thrown=>thrown as NativeImageFullErrorV1);
 expect(error).toBeInstanceOf(NativeImageFullErrorV1);expect(error.code).toBe("ENOSPC");expect(slept).toEqual([0,2000,8000]);
 expect(error.facts).toMatchObject({image:"sealed",capacityBytes:26*GiB,availableBytes:0,innerFreeBytes:25*GiB,hostFreeBytes:3*GiB,attempts:3,reclaimedBytes:3*GiB});
 expect(nativeImageFullTextV1(error.facts,"en")).toContain("25.0 GiB free inside");expect(nativeImageFullTextV1(error.facts,"de")).toContain("25.0 GiB innen frei");expect(error.message).toContain(" / ");
 expect(nativeImageReliefTextV1("clamped","de",{image:"w",available:"4.7 GiB",inner:"26.0 GiB",host:"31.0 GiB",capacity:"27.0 GiB",target:"8.0 GiB"})).toContain("neu angelegt");
});
test("cancellation stops the relief between attempts",async()=>{
 let checks=0;await expect(retryOnImageFullV1(()=>{throw full();},ports({check:()=>{if(++checks>1)throw new DOMException("cancelled","AbortError");}}))).rejects.toMatchObject({name:"AbortError"});
});
test("below the critical host watermark a start collects once and is then refused with the typed bilingual refusal, above it nothing runs",async()=>{
 const{NativeImageHostCriticalErrorV1,requireHostAboveCriticalV1}=await import("../🟦️.ts");
 let collected=0;expect(await requireHostAboveCriticalV1({partition:"/p",hostFreeBytes:()=>30*GiB,collect:async()=>{collected++;}})).toBe(30*GiB);expect(collected).toBe(0);
 let free=3*GiB;expect(await requireHostAboveCriticalV1({partition:"/p",hostFreeBytes:()=>free,collect:async()=>{collected++;free=12*GiB;}})).toBe(12*GiB);expect(collected).toBe(1);
 const error=await requireHostAboveCriticalV1({partition:"/p",hostFreeBytes:()=>2*GiB,collect:async()=>{collected++;}}).catch(thrown=>thrown);
 expect(error).toBeInstanceOf(NativeImageHostCriticalErrorV1);expect(error.code).toBe("ENOSPC");expect(error.message).toContain("2.0 GiB free");expect(error.message).toContain("Image-Wachstum");expect(collected).toBe(2);
});
test("the in-place shrink equals the oracle: a clamped image keeps its content and shrinks to the sized capacity or twice what it holds, an unclamped one stays",()=>{
 const ours=corpus.shrink.map(row=>nativeImageShrinkV1(row));expect(JSON.parse(JSON.stringify(ours))).toEqual(answers.shrink);expect(ours).toEqual(corpus.shrink);
 expect(nativeImageShrinkV1({capacityBytes:30*GiB,usedBytes:Math.floor(3.7*GiB),sizedBytes:8*GiB,clamped:true})).toMatchObject({shrink:true,targetBytes:8*GiB});
 expect(nativeImageShrinkV1({capacityBytes:32*GiB,usedBytes:14*GiB,sizedBytes:8*GiB,clamped:true}).targetBytes).toBe(28*GiB);
 expect(nativeImageShrinkV1({capacityBytes:27*GiB,usedBytes:GiB,sizedBytes:8*GiB,clamped:false}).shrink).toBe(false);
});
test("a write probe takes a healthy mount and leaves nothing behind, and answers the error code of a mount that refuses",()=>{
 const directory=mkdtempSync(join(tmpdir(),"semio-probe-"));
 try{const healthy=probeNativeImageWritesV1(directory);expect(healthy.writable).toBe(true);expect(healthy.availableBytes).toBeGreaterThan(0);expect(readdirSync(directory)).toEqual([]);}finally{rmSync(directory,{recursive:true,force:true});}
 const removed:string[]=[];
 const refused=probeNativeImageWritesV1("/x",{available:()=>0,write:()=>{throw full();},remove:file=>{removed.push(file);}});
 expect(refused).toMatchObject({writable:false,availableBytes:0,reason:"ENOSPC"});expect(removed.length).toBe(1);expect(existsSync(removed[0]!)).toBe(false);
 expect(probeNativeImageWritesV1("/x",{available:()=>{throw Error("gone");},write:()=>{},remove:()=>{}})).toMatchObject({writable:true,availableBytes:0});
});
test("an image that refuses writes ends in the typed refusal in English and German with the facts that name the repair",()=>{
 const facts={image:"w",reason:"ENOSPC",capacityBytes:30*GiB,availableBytes:0,innerFreeBytes:26*GiB,hostFreeBytes:24*GiB,targetBytes:8*GiB};
 const error=new NativeImageWritesRefusedErrorV1(facts);
 expect(error.code).toBe("ENOSPC");expect(error.facts).toBe(facts);expect(nativeImageWritesRefusedTextV1(facts,"en")).toContain("The cache image w refuses writes before the build (ENOSPC): 0.0 GiB usable of 30.0 GiB declared, 26.0 GiB free inside");
 expect(nativeImageWritesRefusedTextV1(facts,"de")).toContain("lehnt Schreibzugriffe vor dem Build ab");expect(nativeImageWritesRefusedTextV1(facts,"de")).toContain("auf 8.0 GiB verkleinert");expect(error.message).toContain(" / ");
 expect(nativeImageReliefTextV1("shrunk","en",{image:"w",capacity:"30.0 GiB",target:"8.0 GiB",used:"3.7 GiB"})).toContain("were kept");
});
