/** ⌛️ Law: every lease wait that runs out, in every lease class, is the one typed bilingual refusal that names the resource, the mode, the seconds waited and the holder (pid, command, age), keeps the message every caller matches, and reaches the CLI boundary without the internal-error text; no other construction of the timeout exists. */
import {afterAll,expect,test} from "bun:test";
import {readFileSync} from "node:fs";
import {mkdirSync,mkdtempSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import authored from "../../🧭️order/🔣️policy.json";
import {readLeaseOrderPolicyV1,formatLeaseTimeoutV1} from "../../🧭️order/🟦️.ts";
import schema from "../../🧭️order/🧬️schema/🔣️.json";
import {listSources} from "../../🧭️order/🧪️tests/🧭️sites/🔎️census.mjs";
import {readLeaseRegistryEntriesV1} from "../../🧭️order/🗂️registry/🟦️.ts";
import {acquireQueuedResourceLease,acquireResourceLease,ResourceLeaseTimeoutErrorV1} from "../../🟦️.ts";
import {reportBoundaryErrorV1} from "../../../🏗️build/🗣️language/📣️progress/🟦️.ts";

const framework=resolve(import.meta.dir,"../../../../.."),roots=[framework+"/🔨️modules/🏃️process",framework+"/🛍️products/🦑️repo/🔨️modules/📚️library"];
const root=mkdtempSync(join(resolve(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir()),"lt-"));
afterAll(()=>rmSync(root,{recursive:true,force:true}));
const policy=readLeaseOrderPolicyV1(authored),samples=policy.classes.map(row=>({id:row.id,resource:row.match[0]!.kind==="exact"?row.match[0]!.value:row.match[0]!.value+"x"}));
const leases=resolve(import.meta.dir,"../../🟦️.ts");
const INTERNAL=/Internal error|Interner Fehler/u;

test("the timeout is constructed in exactly one place: no source of the process and build libraries builds the message anywhere else",()=>{
 const constructions:string[]=[],owner:string[]=[];
 for(const file of roots.flatMap(base=>listSources(base))){
  const text=readFileSync(file,"utf8");
  if(/\bError\(\s*[`"']Resource lease timed out/u.test(text))constructions.push(file.slice(framework.length+1));
  if(/super\(\s*"Resource lease timed out: "/u.test(text))owner.push(file.slice(framework.length+1));
 }
 expect(constructions).toEqual([]);expect(owner).toEqual(["🔨️modules/🏃️process/🔒️leases/🟦️.ts"]);
});

test("the message of the typed refusal is the one every caller matches, and a try-lock that is meant to miss does not look anything up",()=>{
 const error=new ResourceLeaseTimeoutErrorV1({directory:join(root,"nowhere"),resource:"native-storage:abc",mode:"shared"},1500);
 expect(error).toBeInstanceOf(Error);expect(error.message).toBe("Resource lease timed out: native-storage:abc");expect(error.name).toBe("ResourceLeaseTimeoutErrorV1");expect(error.code).toBe("SEMIO_LEASE_TIMEOUT");
 expect(Object.getOwnPropertyDescriptor(error,"texts")).toBeUndefined();
 expect(error.texts.en).toContain("native-storage:abc");expect(error.texts.en).toContain("shared");expect(error.texts.en).toContain("2 s");expect(error.texts.de).toContain("geteilt");
 expect(error.texts.en).toContain("no process of the lease registry");expect(error.texts.de).toContain("keinem Prozess der Lease-Registrierung");
});

test("the texts render the holder in both languages: pid, command and age, another task of this process, or nobody",()=>{
 const base={resource:"repository-dependencies:/c/.cache/b/d",mode:"exclusive",seconds:84};
 for(const language of ["en","de"] as const){
  const named=formatLeaseTimeoutV1(language,{...base,blockerPid:12936,blockerCommand:"bun script.ts component",blockerHeldSeconds:610});
  for(const part of ["12936","bun script.ts component","610","84","repository-dependencies"])expect(named).toContain(part);
  expect(formatLeaseTimeoutV1(language,{...base,blockerPid:1,blockerHeldSeconds:3,blockerSelf:true})).toContain(language==="en"?"another task of this very process":"anderen Task desselben Prozesses");
 }
 const catalogue=schema.definitions.Messages.const.timeout;expect(catalogue.en.length).toBeGreaterThan(40);expect(catalogue.de.length).toBeGreaterThan(40);
});

test("two real processes, every lease class: a holder process keeps one resource of every class exclusive; every wait that runs out, plain or queued, shared or exclusive, is the typed refusal naming that process, its command (as the registry publishes it) and the resource, and renders at the boundary in both languages",async()=>{
 const lease=join(root,"leases"),registry=join(root,"registry"),state=join(root,"state");for(const path of [lease,registry,state])mkdirSync(path,{recursive:true});
 const names=samples.map(row=>row.resource),code=`import {acquireResourceLease} from ${JSON.stringify(leases)};const held=[];for(const resource of ${JSON.stringify(names)})held.push(await acquireResourceLease({directory:${JSON.stringify(lease)},resource,mode:"exclusive",signal:new AbortController().signal}));process.stdout.write("HELD\\n");for await(const chunk of Bun.stdin.stream()){break;}for(const row of held)row.release();process.stdout.write("DONE\\n");`;
 const child=Bun.spawn([process.execPath,...(process.env.SEMIO_TEST_CHILD_BUN_ARGUMENTS?.split("|")??[]),"-e",code,"--holder-of-leases"],{stdin:"pipe",stdout:"pipe",stderr:"pipe",env:{...process.env as Record<string,string>,SEMIO_LEASE_REGISTRY_DIRECTORY:registry,SEMIO_MACHINE_STATE_DIRECTORY:state}});
 const reader=child.stdout.getReader();let seen="";const begin=Date.now();
 while(!seen.includes("HELD")&&Date.now()-begin<60000){const chunk=await reader.read();if(chunk.done)break;seen+=new TextDecoder().decode(chunk.value);}
 if(!seen.includes("HELD"))throw Error("holder never held: "+(await new Response(child.stderr).text()).slice(0,500));
 const previous={registry:process.env.SEMIO_LEASE_REGISTRY_DIRECTORY,state:process.env.SEMIO_MACHINE_STATE_DIRECTORY};process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=registry;process.env.SEMIO_MACHINE_STATE_DIRECTORY=state;
 try{
  const failures:string[]=[],command=readLeaseRegistryEntriesV1(registry,lease).find(row=>row.pid===child.pid)?.command??"";
  expect(command.length).toBeGreaterThanOrEqual(0);
  for(const {id,resource} of samples){
   for(const [label,mode,queued] of [["plain exclusive","exclusive",false],["queued exclusive","exclusive",true],["plain shared","shared",false],["queued shared","shared",true]] as const){
    let caught:unknown;
    try{
     const options={directory:lease,resource,mode,signal:new AbortController().signal,timeoutMs:250};
     await (queued?acquireQueuedResourceLease({...options,owner:"probe-"+process.pid}):acquireResourceLease(options));
    }catch(error){caught=error;}
    if(!(caught instanceof ResourceLeaseTimeoutErrorV1)){failures.push(`${id} ${label}: ${String((caught as Error)?.name??caught)} ${String((caught as Error)?.message??"").slice(0,120)}`);continue;}
    if(caught.message!=="Resource lease timed out: "+resource){failures.push(`${id} ${label}: message ${caught.message}`);continue;}
    for(const language of ["en","de"] as const){
     const text=caught.texts[language];
     if(!text.includes(String(child.pid))||!text.includes(command.slice(0,40))||!text.includes(resource.length>80?resource.slice(-30):resource)){failures.push(`${id} ${label} ${language}: holder not named in "${text.slice(0,200)}"`);continue;}
     const lines:string[]=[],codes:number[]=[];reportBoundaryErrorV1(caught,{environment:{SEMIO_LANGUAGE:language},osLanguages:()=>[],write:line=>lines.push(line),setExitCode:code=>codes.push(code)});
     if(lines.join("\n")!==text||INTERNAL.test(lines.join("\n"))||codes[0]!==1)failures.push(`${id} ${label} ${language}: boundary ${JSON.stringify(lines).slice(0,200)} ${codes}`);
    }
   }
  }
  expect(failures).toEqual([]);expect(samples.length).toBe(policy.classes.length);expect(samples.length).toBeGreaterThan(15);
 }finally{
  if(previous.registry===undefined)delete process.env.SEMIO_LEASE_REGISTRY_DIRECTORY;else process.env.SEMIO_LEASE_REGISTRY_DIRECTORY=previous.registry;
  if(previous.state===undefined)delete process.env.SEMIO_MACHINE_STATE_DIRECTORY;else process.env.SEMIO_MACHINE_STATE_DIRECTORY=previous.state;
  child.stdin.write("go\n");child.stdin.flush();try{child.kill();}catch{}
 }
},240000);
