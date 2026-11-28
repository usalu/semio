/** 🖼️ Contract: a cache image that is nearly full is told before a stage starts and while it runs (floor and need are shares of the image, clamped, never above a quarter of it), the worst image is named first, a failed write names its image from the compiler's own words, the collectors of the image are the ones below its mount, and a full image is looked at again soon. Cross-checked against an independent Python model whose numbers are written from the schema's rule, not read from the code.
 * @see ../🟦️.ts */
import{expect,test}from"bun:test";
import{spawnSync}from"node:child_process";
import{mkdirSync,mkdtempSync,rmSync}from"node:fs";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{READY_IMAGE_POLICY_V1 as P,readyImageFactsV1,readyImageFloorBytesV1,readyImageGuardV1,readyImageHostLimitedV1,readyImageNeedBytesV1,readyImagePartitionOfLinesV1,readyImagePurposeV1,readyImageRecheckDueV1,readyImageRetainBytesV1,readyImageShortV1,readyImageWritersV1}from"../🟦️.ts";

import{readyImageGrowableBytesV1,readyImageReliefBytesV1,readyImageReliefV1,readyImageRowV1,readyImageStoreV1,readyImageVerdictV1,readyNoSpaceEvidenceV1}from"../🟦️.ts";
import{appendNativeImageShardEventV1}from"../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🍎️image/🧩️shard/🟦️.ts";
import{writeFileSync}from"node:fs";
const MB=1048576,GB=1024*MB;
const sizes=[62,512,1535,2047,8191,12287,30824,36863,40959,98304].map(megabytes=>megabytes*MB);

test("floor and need of an image equal an independent Python model",()=>{
 const script=`
import json,sys
p=json.loads(sys.argv[1]);rows=json.loads(sys.argv[2]);out=[]
MB=1048576
def clamp(v,lo,hi): return max(lo,min(hi,v))
for total in rows:
    floor=min(total*p["floorShareBasisPoints"]//10000,clamp(total*p["floorBasisPoints"]//10000,p["floorMinimumMegabytes"]*MB,p["floorMaximumMegabytes"]*MB))
    need=max(floor,clamp(total*p["needBasisPoints"]//10000,p["needMinimumMegabytes"]*MB,p["needMaximumMegabytes"]*MB))
    out.append([floor,need])
print(json.dumps(out))`;
 const run=spawnSync("python3",["-I","-c",script,JSON.stringify(P),JSON.stringify(sizes)],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const model=JSON.parse(run.stdout) as number[][];
 expect<unknown>(sizes.map(total=>[readyImageFloorBytesV1(total),readyImageNeedBytesV1(total)])).toEqual(model);
 for(const[floor,need]of model)expect(need).toBeGreaterThanOrEqual(floor);
});

test("the real images of the fleet: m12 rust image with 47 MB free is short, a healthy one is not",()=>{
 const image=(partition:string,free:number,total:number)=>({partition,path:`/w/.cache/b/${partition}/m`,freeBytes:free*MB,totalBytes:total*MB});
 const short=readyImageShortV1([image("g",2257,12287),image("n",930,1535),image("r",47,40959)])!;
 expect(short.partition).toBe("r");
 expect(short.needed).toBe(4096);
 expect(short.available).toBe(47);
 expect(short.total).toBe(40959);
 expect(readyImageShortV1([image("r",9267,56319),image("g",5083,10239),image("d",16217,36863)])).toBeNull();
 expect(readyImageShortV1([])).toBeNull();
});

test("the worst image is named first and the guard stops below the floor only",()=>{
 const image=(partition:string,free:number,total:number)=>({partition,path:`/w/.cache/b/${partition}/m`,freeBytes:free*MB,totalBytes:total*MB});
 expect(readyImageShortV1([image("g",600,12287),image("r",3000,40959)])!.partition).toBe("g");
 expect(readyImageShortV1([image("g",1000,12287),image("r",1000,40959)])!.partition).toBe("r");
 const hit=readyImageGuardV1([image("g",2000,12287),image("r",1600,40959)])!;
 expect(hit.partition).toBe("r");
 expect(hit.floor).toBe(1639);
 expect(readyImageGuardV1([image("r",1700,40959)])).toBeNull();
 expect(readyImageGuardV1([])).toBeNull();
});

test("a failed write names its image from the compiler's own words, never a control partition",()=>{
 const noSpace=["No space left on device","os error 28","ENOSPC"];
 const cargo=[
  "error: could not parse/generate dep info at: /Users/ueli/Documents/semio/.cache/b/r/m/u/l/q90be4b/prototype/build/x/out/x.d",
  "Caused by:",
  "  failed to write `/Users/ueli/Documents/semio/.cache/b/r/m/u/l/q90be4b/prototype/build/x/fingerprint/dep-lib-x`",
  "Caused by:",
  "  No space left on device (os error 28)",
 ];
 expect(readyImagePartitionOfLinesV1(cargo,noSpace)).toBe("r");
 expect(readyImagePartitionOfLinesV1(["error: error writing dependencies to `/Users/ueli/Documents/semio/.cache/b/g/m/inference/a.d`: No space left on device (os error 28)"],noSpace)).toBe("g");
 expect(readyImagePartitionOfLinesV1(["error writing `C:\\work\\.cache\\b\\d\\m\\j\\a`: There is not enough space on the disk"],["There is not enough space on the disk"])).toBe("d");
 expect(readyImagePartitionOfLinesV1(["No space left on device (os error 28)"],noSpace)).toBeNull();
 expect(readyImagePartitionOfLinesV1(["failed to write /w/.cache/b/l/m/data/x: No space left on device"],noSpace)).toBeNull();
 expect(readyImagePartitionOfLinesV1(["failed to write /w/.cache/b/r/m/x: permission denied"],noSpace)).toBeNull();
 expect(readyImagePartitionOfLinesV1(["failed to write /w/.cache/b/rr/mx/x: No space left on device"],noSpace)).toBeNull();
});

test("the images of a stage are the mounts the policy names for it, own volumes only, unknown volumes skipped",()=>{
 const root=mkdtempSync(join(tmpdir(),"ready-images-"));
 try{
  for(const name of["r","g","n","t","d","l","p"])mkdirSync(join(root,".cache","b",name,"m"),{recursive:true});
  const facts=(path:string)=>path.includes(`${join("b","g")}`)?null:path.includes(`${join("b","n")}`)?{freeBytes:10*MB,totalBytes:100*MB}:{freeBytes:30*GB,totalBytes:40*GB};
  const mounted=(path:string)=>!path.includes(`${join("b","t")}`);
  const workspace=readyImageFactsV1(root,"workspace",facts,P,mounted);
  expect(workspace.map(row=>row.partition)).toEqual(["r"]);
  expect(workspace[0]!.hostFreeBytes).toBe(30*GB);
  expect(readyImageFactsV1(root,"dependencies",facts,P,mounted).map(row=>row.partition)).toEqual(["d"]);
  expect(readyImageFactsV1(root,"tools",facts,P,mounted)).toEqual([]);
  expect(readyImageFactsV1(root,"native",facts,P,mounted)).toEqual([]);
  expect(readyImageFactsV1(root,"workspace",facts,P,()=>false)).toEqual([]);
  expect(readyImageFactsV1(join(root,"missing"),"workspace",facts)).toEqual([]);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("only the collectors below the mount work for the image, and their relief is the image's",()=>{
 const rows=[{id:"cargo",root:"/w/.cache/b/r/m/cargo",row:{relief:"image"}},{id:"tools",root:"/w/.cache/b/t/m/nx",row:{relief:"image"}},{id:"cas",root:"/h/.cache/semio/cas",row:{relief:"host"}},{id:"mount",root:"/w/.cache/b/r/m",row:{relief:"image"}},{id:"sibling",root:"/w/.cache/b/r/mm",row:{relief:"image"}}];
 const kept=readyImageWritersV1(rows,"/w/.cache/b/r/m");
 expect(kept.map(row=>row.id)).toEqual(["cargo","mount"]);
 expect(kept.every(row=>row.row.relief==="host")).toBe(true);
 expect(rows[0]!.row.relief).toBe("image");
 expect(readyImageWritersV1([{root:"C:\\w\\.cache\\b\\r\\m\\cargo",row:{relief:"image"}}],"C:\\w\\.cache\\b\\r\\m","\\")).toHaveLength(1);
});

test("a full image is looked at again after five minutes or when it gained 256 MB, and the retention frees more than the next write",()=>{
 expect(P.recheckMilliseconds).toBe(300000);
 const said={atMs:1000,availableMegabytes:47};
 expect(readyImageRecheckDueV1(said,47,1000+299999)).toBe(false);
 expect(readyImageRecheckDueV1(said,47,1000+300000)).toBe(true);
 expect(readyImageRecheckDueV1(said,47+255,2000)).toBe(false);
 expect(readyImageRecheckDueV1(said,47+256,2000)).toBe(true);
 expect(readyImageRetainBytesV1({freeBytes:47*MB,totalBytes:40959*MB})).toBe(Math.floor(40959*MB/10)-47*MB);
 expect(readyImageRetainBytesV1({freeBytes:4000*MB,totalBytes:40959*MB})).toBe(512*MB);
});

test("every purpose of the policy is named in English and German",()=>{
 for(const partition of Object.values(P.stages).flat()){
  const name=readyImagePurposeV1(partition);
  expect(name.en.length).toBeGreaterThan(1);
  expect(name.de.length).toBeGreaterThan(1);
 }
 expect(readyImagePurposeV1("zz")).toEqual({en:"zz",de:"zz"});
 expect(P.excluded).toEqual(["c","l"]);
});

test("an image that shows the free bytes of the host is limited by the host, an image with less than the host is full on its own",()=>{
 const image=(free:number,host:number|null)=>({freeBytes:free*MB,hostFreeBytes:host===null?null:host*MB});
 expect(readyImageHostLimitedV1(image(2000,2000))).toBe(true);
 expect(readyImageHostLimitedV1(image(1985,2000))).toBe(true);
 expect(readyImageHostLimitedV1(image(1975,2000))).toBe(false);
 expect(readyImageHostLimitedV1(image(47,19741))).toBe(false);
 expect(readyImageHostLimitedV1(image(2000,null))).toBe(false);
});

test("the low-water mark of an image equals an independent Python model, lies between the need and the declared maximum and names only partitions with a collector",()=>{
 const script=`
import json,sys
p=json.loads(sys.argv[1]);rows=json.loads(sys.argv[2]);out=[]
MB=1048576
def clamp(v,lo,hi): return max(lo,min(hi,v))
for total in rows:
    floor=min(total*p["floorShareBasisPoints"]//10000,clamp(total*p["floorBasisPoints"]//10000,p["floorMinimumMegabytes"]*MB,p["floorMaximumMegabytes"]*MB))
    need=max(floor,clamp(total*p["needBasisPoints"]//10000,p["needMinimumMegabytes"]*MB,p["needMaximumMegabytes"]*MB))
    out.append(max(need,clamp(total*p["reliefBasisPoints"]//10000,p["reliefMinimumMegabytes"]*MB,p["reliefMaximumMegabytes"]*MB)))
print(json.dumps(out))`;
 const run=spawnSync("python3",["-I","-c",script,JSON.stringify(P),JSON.stringify(sizes)],{encoding:"utf8"});
 expect(run.status).toBe(0);
 expect<unknown>(sizes.map(total=>readyImageReliefBytesV1(total))).toEqual(JSON.parse(run.stdout));
 for(const total of sizes){expect(readyImageReliefBytesV1(total)).toBeGreaterThanOrEqual(readyImageNeedBytesV1(total));expect(readyImageReliefBytesV1(total)).toBeGreaterThanOrEqual(readyImageFloorBytesV1(total));}
 const image=(partition:string,free:number,total:number)=>({partition,path:`/w/.cache/b/${partition}/m`,freeBytes:free*MB,totalBytes:total*MB});
 expect(readyImageReliefV1([image("r",5000,40959)])!.needed).toBe(8192);
 expect(readyImageReliefV1([image("r",9000,40959)])).toBeNull();
 expect(readyImageReliefV1([image("g",10,12287),image("n",10,1535)])).toBeNull();
 expect(readyImageReliefV1([image("r",1,40959)])!.available).toBe(1);
});

test("the no-space line of a failed stage is found however many progress lines follow it, ignoring source excerpts and colour codes",()=>{
 const directory=mkdtempSync(join(tmpdir(),"ready-space-")),log=join(directory,"stage.log"),patterns=["No space left on device","os error 28","ENOSPC"];
 try{
  const noise=Array.from({length:150000},(_,index)=>`[repository:command] still running after ${index} s`).join("\n");
  writeFileSync(log,["start","\u001b[31mENOSPC\u001b[0m: no space left on device, open '/w/.cache/b/r/m/cargo/home/.semio-ready/f8.fetched.json.stage'","109 |  if(code==='ENOSPC')retry()",noise,""].join("\n"));
  const found=readyNoSpaceEvidenceV1(log,patterns);
  expect(found).toEqual(["ENOSPC: no space left on device, open '/w/.cache/b/r/m/cargo/home/.semio-ready/f8.fetched.json.stage'"]);
  expect(readyImagePartitionOfLinesV1(found,patterns)).toBe("r");
  writeFileSync(log,["a","b"].join("\n"));
  expect(readyNoSpaceEvidenceV1(log,patterns)).toEqual([]);
  expect(readyNoSpaceEvidenceV1(join(directory,"missing.log"),patterns)).toEqual([]);
 }finally{rmSync(directory,{recursive:true,force:true});}
});

test("the shard images of a partition are guarded like its home image and their collection runs on the home mount",()=>{
 const workspace=mkdtempSync(join(tmpdir(),"ready-shards-")),directory=join(workspace,".cache","b","r");
 try{
  mkdirSync(join(directory,"m","u","l","q90be4b"),{recursive:true});
  appendNativeImageShardEventV1(directory,{type:"born",id:"q90be4b",key:"q90be4b",root:0,path:"u/l/q90be4b",capacityBytes:24*GB,atMs:1});
  const free=(path:string)=>({freeBytes:(path.endsWith("q90be4b")?500:9000)*MB,totalBytes:(path.endsWith("q90be4b")?24576:40959)*MB});
  const rows=readyImageFactsV1(workspace,"workspace",free,P,()=>true);
  expect(rows.map(row=>[row.partition,row.path.endsWith("q90be4b")])).toEqual([["r",false],["r",true]]);
  const hit=readyImageGuardV1(rows)!;
  expect(hit.partition).toBe("r");
  expect(hit.path.endsWith("q90be4b")).toBe(true);
  expect(readyImageStoreV1(join(directory,"m","u","l","q90be4b"))).toBe(join(directory,"m"));
  expect(readyImageStoreV1(join(directory,"m"))).toBe(join(directory,"m"));
  expect(readyImageStoreV1(join(workspace,"elsewhere"))).toBe(join(workspace,"elsewhere"));
 }finally{rmSync(workspace,{recursive:true,force:true});}
});

const fact=(partition:string,freeMegabytes:number,totalMegabytes:number,hostFreeMegabytes:number|null=null)=>({partition,path:`/w/.cache/b/${partition}/m/u/sealed`,freeBytes:freeMegabytes*MB,totalBytes:totalMegabytes*MB,hostFreeBytes:hostFreeMegabytes===null?null:hostFreeMegabytes*MB});

test("the floor is a declared function of the image size: the floors the fleet printed (771 MB and 1311 MB) are 4 % of a 19275 MB and of a 32768 MB image, whatever the lanes hold",()=>{
 expect(readyImageFloorBytesV1(19275*MB)).toBe(771*MB);
 expect(Math.ceil(readyImageFloorBytesV1(32768*MB)/MB)).toBe(1311);
 expect(readyImageFloorBytesV1(512*MB)).toBe(128*MB);
 expect(readyImageFloorBytesV1(200000*MB)).toBe(3072*MB);
 expect(P.floorRule.en).toContain("function of the image size alone");
 expect(P.floorRule.de).toContain("Funktion der Image-Größe");
});

test("m02 and m15 of 11-27: an image 17 to 98 MB under its floor is collected in place before the stage is stopped, and stopped only when the collection ended without room",()=>{
 const idle={belowSinceMs:null,collecting:false,collectedAtMs:0};
 const m02=[fact("r",673,19275,30000)],m15=[fact("r",1294,32768,40000)];
 for(const images of[m02,m15]){
  const first=readyImageVerdictV1(images,idle,1000,true);
  expect(first.action).toBe("collect");
  expect(first.hit).not.toBeNull();
  expect(readyImageVerdictV1(images,idle,1000,false).action).toBe("stop");
  expect(readyImageVerdictV1(images,{belowSinceMs:1000,collecting:true,collectedAtMs:0},20000,true).action).toBe("wait");
  expect(readyImageVerdictV1(images,{belowSinceMs:1000,collecting:true,collectedAtMs:0},1000+P.floorGraceMilliseconds,true).action).toBe("stop");
  expect(readyImageVerdictV1(images,{belowSinceMs:1000,collecting:false,collectedAtMs:30000},31000,true).action).toBe("stop");
  expect(readyImageVerdictV1(images,{belowSinceMs:1000,collecting:false,collectedAtMs:500},31000,true).action).toBe("collect");
 }
 expect(readyImageVerdictV1([fact("r",1294+60,32768,40000)],{belowSinceMs:1000,collecting:false,collectedAtMs:30000},31000,true).action).toBe("ok");
 expect(readyImageVerdictV1([fact("r",300,19275,30000)],idle,1000,true).action).toBe("stop");
 expect(readyImageVerdictV1([],idle,1000,true).action).toBe("ok");
});

test("the verdict and the growable bytes equal an independent Python model over a seeded table",()=>{
 const rows:number[][]=[];let seed=7;const next=(limit:number)=>{seed=(seed*1103515245+12345)%2147483648;return Math.floor(seed/65536)%limit;};
 for(let index=0;index<300;index++){const total=[512,2047,12287,19275,32768,40959][next(6)]!,free=next(Math.floor(total/10)),host=[-1,2000,5000,20000,60000][next(5)]!,since=[-1,0,5000][next(3)]!,collecting=next(2),collected=[0,3000,9000][next(3)]!,now=[10000,70000][next(2)]!,collector=next(2);rows.push([total,free,host,since,collecting,collected,now,collector]);}
 const script=`
import json,sys
p=json.loads(sys.argv[1]);rows=json.loads(sys.argv[2]);out=[];MB=1048576
def clamp(v,lo,hi): return max(lo,min(hi,v))
for total,free,host,since,collecting,collected,now,collector in rows:
    t=total*MB;f=free*MB
    floor=min(t*p["floorShareBasisPoints"]//10000,clamp(t*p["floorBasisPoints"]//10000,p["floorMinimumMegabytes"]*MB,p["floorMaximumMegabytes"]*MB))
    grow=0 if host<0 else max(0,min(p["growCapMegabytes"]*MB-t,host*MB-p["growKeepMegabytes"]*MB))
    if f>=floor: out.append(["ok",0]);continue
    s=now if since<0 else since
    if not collector or f<floor*p["floorHardBasisPoints"]//10000: a="stop"
    elif collecting: a="stop" if now-s>=p["floorGraceMilliseconds"] else "wait"
    else: a="stop" if collected>=s else "collect"
    out.append([a,grow//MB])
print(json.dumps(out))`;
 const run=spawnSync("python3",["-I","-c",script,JSON.stringify(P),JSON.stringify(rows)],{encoding:"utf8"});
 expect(run.status).toBe(0);
 const model=JSON.parse(run.stdout) as[string,number][];
 const mine=rows.map(([total,free,host,since,collecting,collected,now,collector])=>{const verdict=readyImageVerdictV1([fact("r",free!,total!,host!<0?null:host!)],{belowSinceMs:since!<0?null:since!,collecting:collecting===1,collectedAtMs:collected!},now!,collector===1);return[verdict.action,verdict.action==="ok"?0:verdict.growableMegabytes];});
 expect<unknown>(mine).toEqual(model);
 for(const action of["collect","wait","stop"])expect(model.filter(row=>row[0]===action).length,action).toBeGreaterThan(3);
});

test("growable bytes count the sparse capacity the heal can still add: capped at the image maximum, cut to what the host keeps free, zero for an unknown host",()=>{
 expect(readyImageGrowableBytesV1({totalBytes:19275*MB,hostFreeBytes:100*1024*MB})).toBe((32768-19275)*MB);
 expect(readyImageGrowableBytesV1({totalBytes:19275*MB,hostFreeBytes:5000*MB})).toBe((5000-3072)*MB);
 expect(readyImageGrowableBytesV1({totalBytes:19275*MB,hostFreeBytes:1000*MB})).toBe(0);
 expect(readyImageGrowableBytesV1({totalBytes:32768*MB,hostFreeBytes:100*1024*MB})).toBe(0);
 expect(readyImageGrowableBytesV1({totalBytes:19275*MB,hostFreeBytes:null})).toBe(0);
 const row=readyImageRowV1(fact("r",673,19275,30000),readyImageNeedBytesV1(19275*MB));
 expect(row.available).toBe(673);expect(row.total).toBe(19275);expect(row.needed).toBe(1928);
});
