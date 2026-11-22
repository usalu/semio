import {expect,test} from "bun:test";
import Ajv from "ajv";
import {existsSync,mkdirSync,mkdtempSync,realpathSync,rmSync,utimesSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import schema from "../🧬️schema/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";
import {autoReclaimDemandBytesV1,autoReclaimDueV1,autoReclaimTextV1,collectRustImageV1,requestRustImageReclaimV1,RUST_IMAGE_RECLAIM_POLICY_V1,rustImageReclaimRuleV1,type RustImageReclaimRequestV1} from "../🟦️.ts";
import {cacheLedgerV1,cacheReclaimShareBytesV1} from "../../../../../💾️storage/🧾️ledger/🟦️.ts";
import {admitImageHeadroomV1,createHonestMeasureV1} from "../../🏦️headroom/🟦️.ts";
import {incrementalLockPathV1} from "../../🌱️incremental/🟦️.ts";
import {nativeImageBackingFilenameV1} from "../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/📋️contract/🟦️.ts";

const GIB=1024**3,rule={softAvailableBytes:12*GIB,targetAvailableBytes:20*GIB},hour=3_600_000;
const scratch=():string=>realpathSync(mkdtempSync(join(realpathSync(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"ar-")));
const noSleep=(milliseconds:number,signal?:AbortSignal):Promise<void>=>new Promise((done,fail)=>{if(signal?.aborted){fail(signal.reason);return;}setTimeout(done,Math.min(milliseconds,2));});
const volume=(freeBytes:number)=>({volumeBytes:96*GIB,freeBytes});
const base=(root:string,lines:string[],extra:Partial<RustImageReclaimRequestV1>={}):RustImageReclaimRequestV1=>({root,capacityBytes:96*GIB,rule,language:"en",report:line=>{lines.push(line);},sleep:noSleep,...extra});

test("the due rule and the demand equal the vectors, the schema and an independent Python oracle",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema,"auto.json");
 const validate=ajv.compile({$ref:"auto.json#/definitions/Vectors"});expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 const run=Bun.spawnSync(["python3","-I","-c","import json,sys\nrows=json.load(sys.stdin)\nout=[]\nfor row in rows:\n a=row['availableBytes'];r=row['rule']\n due=a<r['softAvailableBytes']\n out.append({'due':due,'demandBytes':max(0,r['targetAvailableBytes']-a) if due else 0})\nprint(json.dumps(out))"],{stdin:new TextEncoder().encode(JSON.stringify(fixture.vectors))});
 expect(run.exitCode,new TextDecoder().decode(run.stderr)).toBe(0);
 const oracle=JSON.parse(new TextDecoder().decode(run.stdout)) as {due:boolean;demandBytes:number}[];
 fixture.vectors.forEach((row,at)=>{expect(autoReclaimDueV1(row.availableBytes,row.rule),row.name).toBe(row.due);expect(autoReclaimDemandBytesV1(row.availableBytes,row.rule),row.name).toBe(row.demandBytes);expect(oracle[at],row.name).toEqual({due:row.due,demandBytes:row.demandBytes});});
});

test("the share rule of the ledger resolves to the vectors and an independent Python oracle for 30, 64 and 96 GiB partitions",()=>{
 const ajv=new Ajv({strict:true});ajv.addSchema(schema,"auto.json");
 const validate=ajv.compile({$ref:"auto.json#/definitions/Vectors"});expect(validate(fixture),JSON.stringify(validate.errors)).toBe(true);
 const run=Bun.spawnSync(["python3","-I","-c","import json,sys\nrows=json.load(sys.stdin)\nout=[]\nfor row in rows:\n v=row['volumeBytes'];s=row['share']\n def part(bp,fl):\n  x=v*bp//10000\n  return max(x,min(fl,x*3//2))\n out.append({'softAvailableBytes':part(s['softBasisPoints'],s['softFloorBytes']),'targetAvailableBytes':part(s['targetBasisPoints'],s['targetFloorBytes'])})\nprint(json.dumps(out))"],{stdin:new TextEncoder().encode(JSON.stringify(fixture.shares))});
 expect(run.exitCode,new TextDecoder().decode(run.stderr)).toBe(0);
 const oracle=JSON.parse(new TextDecoder().decode(run.stdout)) as {softAvailableBytes:number;targetAvailableBytes:number}[],declared=cacheLedgerV1().areas.find(row=>row.id==="cargo-store")!.reclaimShare!;
 fixture.shares.forEach((row,at)=>{
  const expected={softAvailableBytes:row.softAvailableBytes,targetAvailableBytes:row.targetAvailableBytes};
  expect(cacheReclaimShareBytesV1(row.share,row.volumeBytes),row.name).toEqual(expected);expect(oracle[at],row.name).toEqual(expected);
  expect(row.share,row.name).toEqual(declared);expect(rustImageReclaimRuleV1(row.volumeBytes),row.name).toEqual(expected);
  expect(row.softAvailableBytes).toBeLessThan(row.targetAvailableBytes);expect(row.targetAvailableBytes).toBeLessThanOrEqual(row.volumeBytes/2);
 });
});

test("the ledger declares the soft threshold and the target of the Rust image as a share of the image volume for every image size, every message is bilingual with the same placeholders",()=>{
 const resolved=[1,4,16,48,96,1024].map(size=>[size,rustImageReclaimRuleV1(size*GIB)] as const);
 for(const [size,row] of resolved){expect(row.softAvailableBytes,String(size)).toBeLessThan(row.targetAvailableBytes);expect(row.targetAvailableBytes).toBeLessThanOrEqual(size*GIB/2);}
 expect(rustImageReclaimRuleV1(96*GIB)).toEqual({softAvailableBytes:Math.floor(96*GIB*0.15),targetAvailableBytes:24*GIB});
 expect(rustImageReclaimRuleV1(16*GIB)).toEqual({softAvailableBytes:Math.floor(16*GIB*0.15),targetAvailableBytes:4*GIB});
 for(const [name,row] of Object.entries(schema.definitions.Messages.const)){
  const holes=(text:string)=>[...text.matchAll(/\{(\w+)\}/gu)].map(match=>match[1]).sort();
  expect(holes(row.en),name).toEqual(holes(row.de));expect(row.en).not.toBe(row.de);
 }
 expect(autoReclaimTextV1("done",{freed:"1.0",free:"13.0",target:"20.0"},"de")).toContain("13.0 GiB");
});

test("a request resolves a share rule against the measured volume of its image",async()=>{
 const root=join(scratch(),"u");mkdirSync(root,{recursive:true});
 try{
  const volumes:number[]=[];let called=0,clock=0;
  for(const size of [16,96]){
   const free=Math.floor(size*GIB*0.10),result=await requestRustImageReclaimV1({...base(root,[]),capacityBytes:size*GIB,rule:rustImageReclaimRuleV1,now:()=>clock+=60_000,measure:()=>({volumeBytes:size*GIB,freeBytes:free}),claim:()=>({path:"x",waitedMs:0,release(){}}),collect:async()=>{called++;volumes.push(size);}});
   expect(result.outcome).toBe("nothing");
  }
  expect(volumes).toEqual([16,96]);expect(called).toBe(2);
  expect((await requestRustImageReclaimV1({...base(root,[]),rule:rustImageReclaimRuleV1,measure:()=>({volumeBytes:96*GIB,freeBytes:30*GIB}),collect:async()=>{called++;}})).outcome).toBe("not-due");
 }finally{rmSync(join(root,".."),{recursive:true,force:true});}
});

test("below the soft threshold the reclaim runs once for the demand up to the target and reports in both languages; above it nothing runs",async()=>{
 const root=join(scratch(),"u");mkdirSync(root,{recursive:true});
 try{
  for(const language of ["en","de"] as const){
   let free=2*GIB;const lines:string[]=[],demands:number[]=[];
   const result=await requestRustImageReclaimV1(base(root,lines,{language,measure:()=>volume(free),collect:async request=>{demands.push(request.demandBytes);request.progress("units:remove",1,3);free=25*GIB;}}));
   expect(result).toEqual({outcome:"reclaimed",availableBytes:25*GIB,freedBytes:23*GIB});expect(demands).toEqual([18*GIB]);
   expect(lines[0]).toBe(autoReclaimTextV1("due",{free:"2.0",soft:"12.0",target:"20.0"},language));expect(lines.some(line=>line===autoReclaimTextV1("done",{freed:"23.0",free:"25.0",target:"20.0"},language))).toBe(true);
   let called=0;const idle=await requestRustImageReclaimV1(base(root,[],{measure:()=>volume(free),collect:async()=>{called++;}}));
   expect(idle.outcome).toBe("not-due");expect(called).toBe(0);
  }
  let called=0;expect((await requestRustImageReclaimV1(base(join(root,"x"),[],{measure:()=>volume(GIB),collect:async()=>{called++;}}))).outcome).toBe("not-due");expect(called).toBe(0);
 }finally{rmSync(join(root,".."),{recursive:true,force:true});}
});

test("concurrent requests of many agents in this process share one reclaim",async()=>{
 const root=join(scratch(),"u");mkdirSync(root,{recursive:true});
 try{
  let free=GIB,called=0;const lines:string[]=[],release=Promise.withResolvers<void>();
  const request=()=>requestRustImageReclaimV1(base(root,lines,{measure:()=>volume(free),collect:async()=>{called++;await release.promise;free=30*GIB;}}));
  const all=[request(),request(),request(),request(),request()];
  await Bun.sleep(20);release.resolve();
  const results=await Promise.all(all);
  expect(called).toBe(1);expect(results.every(row=>row.outcome==="reclaimed")).toBe(true);expect(lines.filter(line=>line.includes("already reclaiming")).length).toBe(4);
 }finally{rmSync(join(root,".."),{recursive:true,force:true});}
});

test("another agent's reclaim in a different process is joined: the request waits and runs nothing once the space is back",async()=>{
 const root=join(scratch(),"u");mkdirSync(root,{recursive:true});
 try{
  let free=GIB,polls=0,called=0;const lines:string[]=[];
  const result=await requestRustImageReclaimV1(base(root,lines,{measure:()=>volume(free),claim:()=>undefined,sleep:async()=>{if(++polls===3)free=30*GIB;},collect:async()=>{called++;}}));
  expect(result.outcome).toBe("joined");expect(called).toBe(0);expect(polls).toBe(3);expect(lines.filter(line=>line.includes("already reclaiming")).length).toBe(1);
  let taken=true;free=GIB;
  const later=await requestRustImageReclaimV1(base(root,[],{measure:()=>volume(free),claim:()=>taken?undefined:{path:"x",waitedMs:0,release(){}},sleep:async()=>{taken=false;},collect:async()=>{called++;free=30*GIB;}}));
  expect(later.outcome).toBe("reclaimed");expect(called).toBe(1);
 }finally{rmSync(join(root,".."),{recursive:true,force:true});}
});

test("a reclaim that freed nothing cools instead of rescanning; the claim is always released",async()=>{
 const root=join(scratch(),"u");mkdirSync(root,{recursive:true});
 try{
  let clock=0,called=0,released=0;const lines:string[]=[],claim=()=>({path:"x",waitedMs:0,release(){released++;}});
  const options=base(root,lines,{measure:()=>volume(GIB),now:()=>clock,claim,collect:async()=>{called++;}});
  expect((await requestRustImageReclaimV1(options)).outcome).toBe("nothing");expect(lines.some(line=>line.includes("nothing reclaimable"))).toBe(true);
  expect((await requestRustImageReclaimV1(options)).outcome).toBe("cooling");expect(called).toBe(1);
  clock+=RUST_IMAGE_RECLAIM_POLICY_V1.cooldownMilliseconds;expect((await requestRustImageReclaimV1(options)).outcome).toBe("nothing");expect(called).toBe(2);expect(released).toBe(2);
 }finally{rmSync(join(root,".."),{recursive:true,force:true});}
});

test("cancellation aborts the collector only when every requester cancelled, and the claim is released",async()=>{
 const root=join(scratch(),"u");mkdirSync(root,{recursive:true});
 try{
  let released=0,collectorSignal:AbortSignal|undefined;const started=Promise.withResolvers<void>(),first=new AbortController(),second=new AbortController(),lines:string[]=[];
  const claim=()=>({path:"x",waitedMs:0,release(){released++;}}),collect=(request:{signal:AbortSignal})=>new Promise<void>((_,fail)=>{collectorSignal=request.signal;started.resolve();request.signal.addEventListener("abort",()=>fail(request.signal.reason),{once:true});});
  const one=requestRustImageReclaimV1(base(root,lines,{measure:()=>volume(GIB),claim,collect,signal:first.signal})),two=requestRustImageReclaimV1(base(root,lines,{measure:()=>volume(GIB),claim,collect,signal:second.signal}));
  const outcomes=Promise.allSettled([one,two]);await started.promise;
  first.abort();await Bun.sleep(10);expect(collectorSignal!.aborted).toBe(false);
  second.abort();const [left,right]=await outcomes;
  expect(left.status).toBe("rejected");expect(right.status).toBe("rejected");expect(collectorSignal!.aborted).toBe(true);expect(released).toBe(1);expect(lines.some(line=>line.includes("cancelled"))).toBe(true);
 }finally{rmSync(join(root,".."),{recursive:true,force:true});}
});

function session(store:string,crate:string,name:string,ageMs:number,bytes:number):string{
 const directory=join(store,"b","p","prototype","incremental",crate,name);mkdirSync(directory,{recursive:true});writeFileSync(join(directory,"dep-graph.bin"),Buffer.alloc(bytes,1));
 const time=new Date(Date.now()-ageMs);utimesSync(directory,time,time);const path=incrementalLockPathV1(directory);writeFileSync(path,"");utimesSync(path,time,time);return directory;
}

test("units a running build holds are never removed: the real collectors free the quiet ones and keep the held and the fresh ones",async()=>{
 const store=scratch(),root=join(store,"u");mkdirSync(root,{recursive:true});
 try{
  const old=session(store,"a-1","s-1-aaa-h1",3*hour,40_000),live=session(store,"b-1","s-2-bbb-h2",3*hour,40_000),fresh=session(store,"c-1","s-3-ccc-h3",1000,40_000),lines:string[]=[],progress:string[]=[];
  const measure=()=>volume((existsSync(old)?2:20)*GIB);
  const result=await requestRustImageReclaimV1(base(root,lines,{language:"de",measure,lock:path=>path===incrementalLockPathV1(live)?"held":"missing",collect:request=>collectRustImageV1(store,{dryRun:false,language:"de",demandBytes:request.demandBytes,quotaBytes:0,unitBudgetBytes:96*GIB,measure:()=>measure(),lock:request.lock,write:request.report,control:collector=>({signal:request.signal,progress:(done,total,phase)=>{progress.push(collector+":"+phase);}})})}));
  expect(result.outcome).toBe("reclaimed");expect(existsSync(old)).toBe(false);expect(existsSync(live)).toBe(true);expect(existsSync(fresh)).toBe(true);
  expect(progress.length).toBeGreaterThan(0);expect(lines.some(line=>line.includes("Inkrementsitzungen"))).toBe(true);
 }finally{rmSync(store,{recursive:true,force:true});}
});

test("admission reclaims before it admits: below the soft threshold the reclaim runs once, then the command is admitted without its own cleanup or ENOSPC",async()=>{
 const scratchRoot=scratch(),root=join(scratchRoot,"u");mkdirSync(root,{recursive:true});
 try{
  const events:string[]=[],lines:string[]=[];let free=2*GIB;
  const lease=await admitImageHeadroomV1({root,capacityBytes:96*GIB,classes:["cargo/check"],temperature:"cold",label:"x",language:"en",report:line=>{lines.push(line);},measure:()=>volume(free),sleep:noSleep,softReclaim:rule,reclaim:async()=>{events.push("floor-reclaim");return true;},softPorts:{collect:async()=>{events.push("soft-collect");free=40*GIB;}}});
  events.push("admitted");await lease.release();
  expect(events).toEqual(["soft-collect","admitted"]);expect(lines[0]).toContain("soft threshold");
  free=2*GIB;events.length=0;
  const held=await admitImageHeadroomV1({root,capacityBytes:96*GIB,classes:["cargo/check"],temperature:"cold",label:"y",language:"de",report:()=>{},measure:()=>volume(free),sleep:noSleep,softReclaim:rule,reclaim:async()=>{events.push("floor-reclaim");free=40*GIB;return true;},softPorts:{collect:async()=>{events.push("soft-collect");}}});
  await held.release();expect(events[0]).toBe("soft-collect");
 }finally{rmSync(scratchRoot,{recursive:true,force:true});}
});

test("the honest measure never promises more than the host holds under the image, and leaves an unmounted root alone",()=>{
 const scratchRoot=scratch(),directory=join(scratchRoot,"b","r"),inside=join(directory,"m","u");
 try{
  mkdirSync(inside,{recursive:true});writeFileSync(join(directory,nativeImageBackingFilenameV1()),"");
  const measure=createHonestMeasureV1(()=>volume(50*GIB),{host:()=>7*GIB});
  expect(measure(inside,96*GIB).freeBytes).toBe(7*GIB);expect(measure(scratchRoot,96*GIB).freeBytes).toBe(50*GIB);
  expect(createHonestMeasureV1(()=>volume(5*GIB),{host:()=>50*GIB})(inside,96*GIB).freeBytes).toBe(5*GIB);
 }finally{rmSync(scratchRoot,{recursive:true,force:true});}
});
