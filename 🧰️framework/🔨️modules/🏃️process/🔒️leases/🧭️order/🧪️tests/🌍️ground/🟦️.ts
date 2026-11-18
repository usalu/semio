import {afterAll,expect,test} from "bun:test";
import {createHash} from "node:crypto";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import authored from "../../🔣️policy.json";
import schema from "../../🧬️schema/🔣️.json";
import {checkGroundWaitV1,checkHoldAndWaitV1,classifyLeaseResourceV1,formatLeaseViolationV1,groundWaitBoundV1,readLeaseOrderPolicyV1,registerLeaseWaitScaleV1,type LeaseViolationV1} from "../../🟦️.ts";
import {machineSpeedFactorV1,localMachineSpeedFactorV1} from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🐢️speed/🟨️.mjs";
import {readLeaseRegistryEntriesV1} from "../../🗂️registry/🟦️.ts";

const root=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"lg-"));
afterAll(()=>rmSync(root,{recursive:true,force:true}));
const worker=resolve(import.meta.dir,"../📥️worker/🟦️.ts");
const mapping="native-storage-mapping:"+createHash("sha256").update("/scratch/checkout/.cache/b/g/m").digest("hex"),facts="repository-graph-facts:/Users/ueli/Documents/semio:shard:53";
const live=readLeaseOrderPolicyV1(authored);
const directory="/scratch/checkout/.cache/b/g/data";
const hold=(resource:string,mode:"shared"|"exclusive"="exclusive")=>({directory,resource,mode,atMs:1,task:1});
const request=(resource:string,mode:"shared"|"exclusive"="shared")=>({directory,resource,mode,nonblocking:false});

test("the mapping of a native image is a declared ground class: no held exclusive lease of any class makes waiting for it a hold-and-wait, and every other unclassified resource still is",()=>{
 expect(classifyLeaseResourceV1(live,mapping)).toMatchObject({id:"image-mapping",rank:7});
 expect(classifyLeaseResourceV1(live,mapping).ground?.reason.en.length).toBeGreaterThan(40);expect(classifyLeaseResourceV1(live,mapping).ground?.reason.de.length).toBeGreaterThan(40);
 const refused:string[]=[];
 for(const row of live.classes){
  const sample=row.match[0]!.kind==="exact"?row.match[0]!.value:row.match[0]!.value+"x";
  if(checkHoldAndWaitV1(live,[hold(sample)],request(mapping),600000)!==undefined)refused.push(row.id);
 }
 expect(refused).toEqual([]);
 expect(checkHoldAndWaitV1(live,[hold(facts)],request("not-declared:"+"a".repeat(8)),600000)?.kind).toBe("hold-and-wait");
 expect(checkHoldAndWaitV1(live,[hold(facts)],request(mapping),600000)).toBeUndefined();
});

test("the ground wait bound is the declared base times the machine speed factor, equal to the independent R135 formula on 4 cores and 8 GB, and is refused with the holder named",()=>{
 const small=machineSpeedFactorV1({cores:4,totalMegabytes:8192}),python=Math.ceil(Math.max(1,8/4)*Math.max(1,16384/8192)**0.5*100-1e-9)/100;
 expect(small).toBe(python);expect(small).toBe(2.83);
 registerLeaseWaitScaleV1(()=>small);
 try{
  const bound=groundWaitBoundV1(live)!;expect(bound.factor).toBe(2.83);expect(bound.boundMilliseconds).toBe(Math.ceil(1800000*2.83));
  expect(checkGroundWaitV1(live,[hold(facts)],request(mapping),bound.boundMilliseconds-1)).toBeUndefined();
  const row=checkGroundWaitV1(live,[hold(facts)],request(mapping),bound.boundMilliseconds,{pid:4242,heldSeconds:61,command:"repo graph read --slice 9"})!;
  expect(row).toMatchObject({kind:"hold-and-wait",held:facts,requested:mapping,requestedClass:"image-mapping",boundMilliseconds:bound.boundMilliseconds,speedFactor:2.83,blockerPid:4242,blockerHeldSeconds:61,blockerCommand:"repo graph read --slice 9"});
  for(const language of ["en","de"] as const){const text=formatLeaseViolationV1(language,row);expect(text).toContain("4242");expect(text).toContain("repo graph read --slice 9");expect(text).toContain("61");expect(text).toContain("2.83");expect(text).toContain(String(Math.floor(bound.boundMilliseconds/1000)));}
  expect(formatLeaseViolationV1("en",row)).toContain("Refusing to wait any longer");expect(formatLeaseViolationV1("de",row)).toContain("Warten nicht länger zugelassen");
  const self=checkGroundWaitV1(live,[],request(mapping),bound.boundMilliseconds,{pid:process.pid,heldSeconds:3,command:"x",self:true})!;
  expect(formatLeaseViolationV1("en",self)).toContain("another task of this very process");expect(formatLeaseViolationV1("de",self)).toContain("anderen Task desselben Prozesses");
  expect(self.held).toBe(self.requested);expect(formatLeaseViolationV1("en",self)).toContain("holds nothing");
  const unknown=checkGroundWaitV1(live,[hold(facts)],request(mapping),bound.boundMilliseconds)!;
  expect(formatLeaseViolationV1("en",unknown)).toContain("no process of the lease registry");expect(formatLeaseViolationV1("de",unknown)).toContain("keinem Prozess der Lease-Registrierung");
  expect(checkGroundWaitV1(live,[hold(facts)],{...request(mapping),nonblocking:true},bound.boundMilliseconds*10)).toBeUndefined();
  expect(checkGroundWaitV1(live,[hold(facts)],request("rustup:home"),bound.boundMilliseconds*10)).toBeUndefined();
 }finally{registerLeaseWaitScaleV1(()=>1);}
 expect(groundWaitBoundV1(live)).toMatchObject({factor:1,boundMilliseconds:1800000});
 expect(typeof localMachineSpeedFactorV1()).toBe("number");expect(groundWaitBoundV1(live)!.factor).toBe(localMachineSpeedFactorV1());
});

type Child={pid:number;send(text:string):void;next(prefix:string,milliseconds?:number):Promise<string>;stop():void;done:Promise<number>};
function start(spec:Record<string,unknown>,env:Record<string,string>):Child{
 const child=Bun.spawn([process.execPath,...(process.env.SEMIO_TEST_CHILD_BUN_ARGUMENTS?.split("|")??[]),worker,JSON.stringify(spec)],{stdin:"pipe",stdout:"pipe",stderr:"pipe",env});
 const lines:string[]=[],waiting:{prefix:string;resolve:(line:string)=>void}[]=[];let buffer="";
 const push=(line:string)=>{lines.push(line);for(const entry of [...waiting])if(line.startsWith(entry.prefix)){waiting.splice(waiting.indexOf(entry),1);entry.resolve(line);}};
 (async()=>{const decoder=new TextDecoder();for await(const chunk of child.stdout){buffer+=decoder.decode(chunk);for(let at=buffer.indexOf("\n");at>=0;at=buffer.indexOf("\n")){push(buffer.slice(0,at));buffer=buffer.slice(at+1);}}})();
 const errors:string[]=[];(async()=>{for await(const chunk of child.stderr)errors.push(new TextDecoder().decode(chunk));})();
 return {pid:child.pid,send:text=>{child.stdin.write(text+"\n");child.stdin.flush();},stop:()=>{try{child.kill();}catch{}},done:child.exited,
  next:(prefix,milliseconds=15000)=>{const found=lines.find(line=>line.startsWith(prefix));if(found)return Promise.resolve(found);return new Promise<string>((resolve,reject)=>{waiting.push({prefix,resolve});setTimeout(()=>reject(Error(`child ${child.pid} never printed ${prefix}; stderr: ${errors.join("").slice(0,400)}`)),milliseconds).unref();});}};
}
let counter=0;
function scenario(policy?:Record<string,unknown>){
 const lease=join(root,`l${++counter}`),registry=join(root,`r${counter}`),state=join(root,`s${counter}`);for(const path of [lease,registry,state])mkdirSync(path,{recursive:true});
 const env:Record<string,string>={...process.env as Record<string,string>,SEMIO_LEASE_REGISTRY_DIRECTORY:registry,SEMIO_MACHINE_STATE_DIRECTORY:state,SEMIO_LEASE_ORDER:"",SEMIO_LEASE_ORDER_POLICY:""};
 delete env.SEMIO_LEASE_ORDER;delete env.SEMIO_LEASE_ORDER_POLICY;
 if(policy){const file=join(root,`p${counter}.json`);writeFileSync(file,JSON.stringify({...authored,...policy}));env.SEMIO_LEASE_ORDER_POLICY=file;}
 return {lease,registry,env};
}
const tick=(milliseconds:number)=>new Promise(resolve=>setTimeout(resolve,milliseconds));

test("two real processes: a graph reader that holds the facts shard exclusively waits 18 s for a mapping held exclusively elsewhere and then proceeds, with its wait published",async()=>{
 const {lease,registry,env}=scenario();
 const holder=start({mode:"hold",directory:lease,resource:mapping,access:"exclusive"},env);await holder.next("HELD");
 const reader=start({mode:"chain",queued:true,directory:lease,first:{resource:facts,access:"exclusive"},second:{resource:mapping,access:"shared"}},env);await reader.next("HELD");
 const began=Date.now();reader.send("go");
 await tick(3000);
 const entry=readLeaseRegistryEntriesV1(registry,lease).find(row=>row.pid===reader.pid);
 expect(entry?.holds.some(row=>row.resource===facts&&row.mode==="exclusive")).toBe(true);
 expect(entry?.waits.some(row=>row.resource===mapping&&row.mode==="shared")).toBe(true);
 await tick(Math.max(0,18000-(Date.now()-began)));
 holder.send("release");await holder.next("DONE");
 const result=JSON.parse((await reader.next("RESULT",30000)).slice("RESULT ".length));
 expect(result.message??"").toBe("");
 expect(result.ok).toBe(true);expect(result.elapsedMs).toBeGreaterThanOrEqual(17500);
 reader.send("done");await reader.next("DONE");
},120000);

test("two real processes: a wait for the mapping past the declared bound is a typed refusal that names the holder process, its command and its age, in both languages",async()=>{
 const {lease,env}=scenario({groundWait:{boundMilliseconds:4000}});
 const holder=start({mode:"hold",directory:lease,resource:mapping,access:"exclusive"},env);await holder.next("HELD");
 const reader=start({mode:"chain",queued:true,directory:lease,first:{resource:facts,access:"exclusive"},second:{resource:mapping,access:"shared"}},env);await reader.next("HELD");
 reader.send("go");
 const result=JSON.parse((await reader.next("RESULT",60000)).slice("RESULT ".length));
 expect(result.ok).toBe(false);expect(result.code).toBe("SEMIO_LEASE_ORDER");
 expect(result.elapsedMs).toBeGreaterThanOrEqual(3800);expect(result.elapsedMs).toBeLessThan(12000);
 expect(result.message).toContain("Refusing to wait any longer");expect(result.message).toContain("Warten nicht länger zugelassen");
 expect(result.message.match(new RegExp("process "+holder.pid,"gu"))?.length).toBe(1);expect(result.message).toContain("Prozess "+holder.pid);expect(result.message).toContain("worker");
 reader.send("done");holder.send("release");await holder.next("DONE");
},120000);

test("the schema declares the ground fields closed and the messages in both languages",()=>{
 const messages=schema.definitions.Messages.const;
 for(const key of ["waitBound","blockerCommand","blockerSelf","blockerUnknown","holdsNothing"] as const){expect(messages[key].en.length).toBeGreaterThan(2);expect(messages[key].de.length).toBeGreaterThan(2);}
 expect(()=>readLeaseOrderPolicyV1({...authored,groundWait:{boundMilliseconds:5}})).toThrow("Closed lease order Policy");
 expect(()=>readLeaseOrderPolicyV1({...authored,classes:[...authored.classes,{...authored.classes[0]!,id:"extra",match:[{kind:"prefix",value:"zz:"}],ground:{reason:{en:"x"}}}]})).toThrow("Closed lease order Policy");
});
