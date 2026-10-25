import {afterAll,expect,test} from "bun:test";
import Ajv from "ajv";
import {spawn,spawnSync} from "node:child_process";
import {existsSync,mkdtempSync,readdirSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {setTimeout as sleep} from "node:timers/promises";
import {decideHostHandoverV1,newHostCapacityIdV1,readHostCapacityPolicyV1,withHostCapacityV1,type HostCapacityOptionsV1,type HostSampleV1} from "../../🟦️.ts";
import {offerRestartCustodiesV1,settleRestartCustodiesV1} from "../../../../../📁️filesystem/🧾️observation/🔁️churn/🟦️.ts";
import {systemAncestryV1} from "../../../../🪪️agent-sessions/🧭️ancestry/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";
import decisionFixture from "../🧫️fixtures/🔣️.json";
import fixture from "./🔣️.json";

const policy=readHostCapacityPolicyV1({...decisionFixture.policy,load:{...decisionFixture.policy.load,overrideAfterMilliseconds:150},wait:{...decisionFixture.policy.wait,pollMilliseconds:10,progressMilliseconds:20,lockStaleMilliseconds:5000}});
const roots:string[]=[],fresh=()=>{const root=mkdtempSync(join(tmpdir(),"hh-"));roots.push(root);return root;};
afterAll(async()=>{await settleRestartCustodiesV1();for(const root of roots)rmSync(root,{recursive:true,force:true});});
const host=():HostSampleV1=>({version:1,atMs:Date.now(),platform:"darwin",cores:10,totalMegabytes:32768,availableMegabytes:20000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load:1,openFiles:8000,openFilesMaximum:245760,attachedImages:0});
const control=(signal=new AbortController().signal)=>({signal,remainingMilliseconds:()=>null,publish:async()=>undefined,yieldContinuation:async()=>undefined});
const request=(over:object={})=>({id:newHostCapacityIdV1(),operation:"run",priority:"batch" as const,minimumJobs:1,preferredJobs:1,fixedMegabytes:13000,perJobMegabytes:0,cpuPerJob:1,...over});
const options=(directory:string,over:Partial<HostCapacityOptionsV1>={}):HostCapacityOptionsV1=>({directory,policy,request:request(),control:control(),environment:{},write:()=>undefined,sample:async()=>({host:host(),swappedOutMegabytes:0}),...over});
const STARTED="Sat Oct 24 19:14:40 2026",PARENT=async()=>({pid:process.pid,started:STARTED});
const grants=(directory:string)=>readdirSync(join(directory,"grants")),queue=(directory:string)=>readdirSync(join(directory,"queue"));
const stale=()=>Error("This command runs an outdated implementation");
const park=async(directory:string,lines:string[]=[],environment:Record<string,string>={},ask:object={},real=false)=>{
 await expect(withHostCapacityV1(options(directory,{park:()=>true,...real?{}:{self:async()=>STARTED},write:line=>lines.push(line),environment,request:request(ask)}),async()=>{throw stale();})).rejects.toThrow("outdated implementation");
};
const handover=async(directory:string,lines:string[]=[],environment:Record<string,string>={},ask:object={},real=false)=>{await park(directory,lines,environment,ask,real);return offerRestartCustodiesV1({});};

test("the decision on an offered grant agrees with the closed vectors, the schema and an independent Python oracle",()=>{
 const ajv=new Ajv({strict:true,allowUnionTypes:true}).addSchema(schema),facts=ajv.compile({$ref:schema.$id+"#/definitions/HandoverFacts"}),row=ajv.compile({$ref:schema.$id+"#/definitions/HandoverCase"});
 const oracle=JSON.parse(spawnSync("python3",["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"}).stdout) as Record<string,string>;
 expect(Object.keys(oracle).length).toBe(fixture.cases.length);
 for(const item of fixture.cases){
  expect(Boolean(row(item)),item.id).toBe(true);expect(Boolean(facts(item.input)),item.id).toBe(true);
  const verdict=decideHostHandoverV1(item.input as never);
  expect(verdict.adopt?"adopt":verdict.reason,item.id).toBe(item.expected);expect(oracle[item.id],item.id).toBe(item.expected);
 }
 expect(new Set(fixture.cases.map(item=>item.expected))).toEqual(new Set(["adopt",...schema.definitions.Protocol.const.handoverRefusals]));
});

test("a grant parked by a failing command stays in the ledger and its replacement takes it over in front of the queue, without waiting",async()=>{
 const directory=fresh(),english:string[]=[],german:string[]=[],lines:string[]=[];
 const additions=await handover(directory,lines,{LANG:"en_US.UTF-8"});
 expect(Object.keys(additions).sort()).toEqual(["SEMIO_HOST_CAPACITY_HANDOVER","SEMIO_HOST_CAPACITY_PLAN"]);
 expect(grants(directory).length).toBe(1);
 const parked=JSON.parse(readFileSync(join(directory,"grants",grants(directory)[0]!),"utf8"));
 expect(parked.handover).toMatchObject({fromPid:process.pid,fromStarted:STARTED});
 expect(lines.at(-1)).toMatch(/^\[capacity\] keeping the run grant \(1 job\(s\), 13000 MB reserved\) for the command that replaces this one: it does not queue again$/u);
 const competitor=withHostCapacityV1(options(directory,{environment:{LANG:"de_DE.UTF-8"},write:line=>german.push(line)}),async()=>"competitor");
 await sleep(80);
 expect(queue(directory).length).toBe(1);
 const started=Date.now();
 const result=await withHostCapacityV1(options(directory,{environment:{...additions,LANG:"en_US.UTF-8"},parent:PARENT,write:line=>english.push(line)}),async handle=>{
  expect(handle).toMatchObject({adopted:true,inherited:false,id:JSON.parse(additions.SEMIO_HOST_CAPACITY_PLAN!).id});
  expect(queue(directory).length).toBe(1);
  expect(grants(directory).length).toBe(1);
  expect(JSON.parse(readFileSync(join(directory,"grants",handle.id+".json"),"utf8")).handover).toBeUndefined();
  return "replacement";
 });
 expect(result).toBe("replacement");expect(Date.now()-started).toBeLessThan(1500);
 expect(english).toEqual(["[capacity] took over the run grant of the command this one replaces (1 job(s), 13000 MB reserved): no queue"]);
 expect(await competitor).toBe("competitor");
 expect(german.at(-1)).toMatch(/zugelassen run/u);
 await settleRestartCustodiesV1();
 expect(grants(directory)).toEqual([]);expect(queue(directory)).toEqual([]);
},30000);

test("the takeover is announced in German when German is chosen and the offer is announced in the language of the failing command",async()=>{
 const directory=fresh(),offer:string[]=[],taken:string[]=[];
 const additions=await handover(directory,offer,{LANG:"de_DE.UTF-8"});
 await withHostCapacityV1(options(directory,{environment:{...additions,LANG:"de_DE.UTF-8"},parent:PARENT,write:line=>taken.push(line)}),async()=>undefined);
 expect(offer.at(-1)).toBe("[capacity] die Zusage für run (1 Job(s), 13000 MB reserviert) bleibt für den Befehl, der diesen ersetzt: er reiht sich nicht erneut ein");
 expect(taken).toEqual(["[capacity] Zusage für run des ersetzten Befehls übernommen (1 Job(s), 13000 MB reserviert): keine Warteschlange"]);
 await settleRestartCustodiesV1();
 expect(grants(directory)).toEqual([]);
});

test("a process that is not the one the grant was offered to queues like any other request and is told why",async()=>{
 for(const [name,over,reason] of [
  ["another parent",{parent:async()=>({pid:process.pid+1,started:STARTED})},"this process is not the one the grant was offered to"],
  ["a recycled process id",{parent:async()=>({pid:process.pid,started:"Sat Oct 24 20:00:00 2026"})},"this process is not the one the grant was offered to"],
  ["no known parent",{parent:async()=>undefined},"this process is not the one the grant was offered to"],
  ["another operation",{parent:PARENT,request:request({operation:"cargo"})},"it was granted for another operation"],
 ] as const){
  const directory=fresh(),lines:string[]=[],additions=await handover(directory);
  const controller=new AbortController();setTimeout(()=>controller.abort(Error("gave up")),250);
  await expect(withHostCapacityV1(options(directory,{environment:additions,control:control(controller.signal),write:line=>lines.push(line),...over}),async()=>"never"),name).rejects.toThrow("gave up");
  expect(lines[0],name).toBe(`[capacity] the grant of the command this one replaces cannot be taken over (${reason}): queueing like any other request`);
  expect(grants(directory).length,name).toBe(1);expect(queue(directory),name).toEqual([]);
  await settleRestartCustodiesV1();expect(grants(directory),name).toEqual([]);
 }
});

test("a wrong token, a stale offer and a gone grant are refused with their reason in German too",async()=>{
 const directory=fresh(),additions=await handover(directory),id=additions.SEMIO_HOST_CAPACITY_HANDOVER!.split(".")[0]!;
 const attempt=async(environment:Record<string,string>,policyOver=policy)=>{const lines:string[]=[],controller=new AbortController();setTimeout(()=>controller.abort(Error("gave up")),200);await expect(withHostCapacityV1(options(directory,{policy:policyOver,environment:{LANG:"de_DE.UTF-8",...environment},parent:PARENT,control:control(controller.signal),write:line=>lines.push(line)}),async()=>"never")).rejects.toThrow("gave up");return lines[0];};
 expect(await attempt({...additions,SEMIO_HOST_CAPACITY_HANDOVER:id+".BBBBBBBBBBBBBBBBBBBBBB"})).toBe("[capacity] die Zusage des ersetzten Befehls kann nicht übernommen werden (die Übergabemarke passt nicht): Anfrage reiht sich wie jede andere ein");
 expect(await attempt({...additions,SEMIO_HOST_CAPACITY_HANDOVER:"zzzzzzzzzzzzzzzz."+additions.SEMIO_HOST_CAPACITY_HANDOVER!.split(".")[1]!})).toContain("die Zusage ist erloschen oder nicht mehr aktuell");
 await settleRestartCustodiesV1();expect(grants(directory)).toEqual([]);
});

test("a parked grant nobody offers within its bound is given back, and a body that fails for any other reason never parks",async()=>{
 const directory=fresh();
 await expect(withHostCapacityV1(options(directory,{park:()=>false}),async()=>{throw Error("ordinary failure");})).rejects.toThrow("ordinary failure");
 expect(grants(directory)).toEqual([]);
 await park(directory);expect(grants(directory).length).toBe(1);
 await settleRestartCustodiesV1();expect(grants(directory)).toEqual([]);
 await withHostCapacityV1(options(directory),async()=>undefined);
});

const run=(script:string,environment:Record<string,string>,wrapper?:string[])=>new Promise<{code:number|null;out:string[];err:string}>(accept=>{
 const child=wrapper?spawn(wrapper[0]!,[...wrapper.slice(1),process.execPath,script],{env:{...process.env,...environment},stdio:["ignore","pipe","pipe"]}):spawn(process.execPath,[script],{env:{...process.env,...environment},stdio:["ignore","pipe","pipe"]});
 let out="",err="";child.stdout.on("data",chunk=>{out+=chunk;});child.stderr.on("data",chunk=>{err+=chunk;});
 child.on("close",code=>accept({code,out:out.split("\n").filter(Boolean),err}));
});
const script=fileURLToPath(new URL("./📜️child/🟦️.ts",import.meta.url));

test("a real process that is not the child of the offering process does not take the grant over",async()=>{
 const directory=fresh(),additions=await handover(directory,[],{},{fixedMegabytes:64},true);
 expect(systemAncestryV1(process.platform,process.pid)[0]?.started).toBe(spawnSync("ps",["-o","lstart=","-p",String(process.pid)],{encoding:"utf8"}).stdout.trim());
 const unrelated=await run(script,{...additions,HC_DIRECTORY:directory,HC_MODE:"claim",HC_MEGABYTES:"64",HC_TIMEOUT_MS:"4000",LANG:"en_US.UTF-8"},["sh","-c",'"$@"; :',"sh"]);
 const row=JSON.parse(unrelated.out[0]!);
 expect(unrelated.err).toContain("[capacity] the grant of the command this one replaces cannot be taken over (this process is not the one the grant was offered to): queueing like any other request");
 expect(row.adopted===false||row.failed!==undefined).toBe(true);
 expect(row.grantPid===undefined||row.grantPid===row.pid).toBe(true);
 await settleRestartCustodiesV1();expect(grants(directory)).toEqual([]);
},60000);

test("a real replacement process takes the grant of the process that started it and queues behind nobody",async()=>{
 const directory=fresh(),additions=await handover(directory,[],{},{},true);
 const competitor=withHostCapacityV1(options(directory),async()=>"competitor");
 await sleep(100);
 expect(queue(directory).length).toBe(1);
 const replacement=await run(script,{...additions,HC_DIRECTORY:directory,HC_MODE:"claim",HC_HOLD_MS:"300"});
 const row=JSON.parse(replacement.out[0]!);
 expect(replacement.code,replacement.err).toBe(0);expect(row,replacement.err).toMatchObject({adopted:true,ppid:process.pid,hasHandover:false,queue:1});
 expect(row.grantPid).toBe(row.pid);expect(row.pid).not.toBe(process.pid);
 expect(await competitor).toBe("competitor");
 await settleRestartCustodiesV1();expect(grants(directory)).toEqual([]);
},60000);

test("the gate lock is never held while the process table is read, and a refused claim never leaves it behind",async()=>{
 const directory=fresh(),additions=await handover(directory),seen:boolean[]=[];
 await withHostCapacityV1(options(directory,{environment:{...additions},parent:async()=>{seen.push(existsSync(join(directory,"lock")));return{pid:process.pid,started:STARTED};}}),async handle=>{expect(handle.adopted).toBe(true);});
 expect(seen).toEqual([false]);
 const other=fresh(),again=await handover(other),lines:string[]=[];
 const controller=new AbortController();setTimeout(()=>controller.abort(Error("gave up")),250);
 await expect(withHostCapacityV1(options(other,{environment:{...again},control:control(controller.signal),parent:async()=>{seen.push(existsSync(join(other,"lock")));return{pid:process.pid+1,started:STARTED};},write:line=>lines.push(line)}),async()=>"never")).rejects.toThrow("gave up");
 expect(seen).toEqual([false,false]);expect(existsSync(join(other,"lock"))).toBe(false);expect(lines.some(line=>line.includes("cannot be taken over"))).toBe(true);
 await settleRestartCustodiesV1();
},30000);
