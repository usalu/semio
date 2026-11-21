import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {runReplicaCommandV1} from "../⌨️command/🟦️.ts";
import {readinessReportV1} from "../../🩺️attest/🚦️readiness/🟦️.ts";
import readinessFixture from "../../🩺️attest/🚦️readiness/🧪️tests/🔣️.json" with {type:"json"};

const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`operator-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const machine=(store:string,origin:string):string=>{const worktree=temp(origin);mkdirSync(join(worktree,".cache","f"),{recursive:true});writeFileSync(join(worktree,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:"g1",storeRoot:store,key:"0123456789abcdef0123456789abcdef"}));return worktree;};
async function run(command:string,store:string,worktree:string,language:"en"|"de",flags:readonly string[]=[]):Promise<Readonly<{lines:string[];code:number}>>{
 const lines:string[]=[],result=await runReplicaCommandV1([command,"--worktree",worktree,"--store",store,"--lang",language,...flags],{root:worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:line=>{lines.push(line);}});
 return {lines,code:result.exitCode};
}

test("an origin that is registered but never attested, and a peer store that is not mounted, each get one actionable line in English and in German from fleet and from status on the group server, and no line on a member",async()=>{
 const store=temp("store"),server=machine(store,"g1-server"),worktree=server;
 mkdirSync(join(store,"v1","origins","g1-m09"),{recursive:true});writeFileSync(join(store,"v1","origins","g1-m09","fleet-auth.json"),"{}\n");
 mkdirSync(join(store,"v1","origins","g1-m10"),{recursive:true});
 for(const command of["fleet","status"]){
  const english=await run(command,store,server,"en"),german=await run(command,store,server,"de");
  const never=english.lines.filter(line=>line.includes("g1-m09")&&line.includes("never attested")),nie=german.lines.filter(line=>line.includes("g1-m09")&&line.includes("nie bescheinigt"));
  expect(never,command).toHaveLength(1);expect(never[0]).toContain("retire g1-m09");expect(never[0]).toContain("provisioning script");
  expect(nie,command).toHaveLength(1);expect(nie[0]).toContain("retire g1-m09");expect(nie[0]).toContain("Provisionierungsskript");
  expect(english.lines.some(line=>line.includes("g1-m10")&&line.includes("never attested")),"a folder without a registration is no origin").toBe(false);
  const unreachable=english.lines.filter(line=>line.includes("peer group g2")&&line.includes("not reachable")),unerreichbar=german.lines.filter(line=>line.includes("Partnergruppe g2")&&line.includes("nicht erreichbar"));
  expect(unreachable,command).toHaveLength(1);expect(unreachable[0]).toContain("server-plan --group g1 --part relay-peers");
  expect(unerreichbar,command).toHaveLength(1);expect(unerreichbar[0]).toContain("server-plan --group g1 --part relay-peers");
 }
 const member=await run("fleet",store,machine(store,"g1-m01"),"en");
 expect(member.lines.some(line=>line.includes("not reachable")),"only the relay host mounts the peers").toBe(false);
 expect(member.lines.filter(line=>line.includes("g1-m09")&&line.includes("never attested"))).toHaveLength(1);
});

test("the relay tells the operator once per outage which peer store is missing and what to run, in both languages, and says when it is back",async()=>{
 for(const language of["en","de"] as const){
  const store=temp("relay-store"),server=machine(store,"g1-server"),peers=temp("peers"),peer=join(peers,"g2"),lines:string[]=[],controller=new AbortController();
  mkdirSync(join(store,"v1"),{recursive:true});
  const running=runReplicaCommandV1(["relay","--worktree",server,"--store",store,"--peers",`g2=${peer}`,"--poll","50","--consolidate-minutes","0","--lang",language],{root:server,environment:{},signal:controller.signal,write:()=>{},notice:line=>{lines.push(line);}});
  await Bun.sleep(700);
  mkdirSync(join(peer,"v1"),{recursive:true});
  await Bun.sleep(700);
  controller.abort();await running.catch(()=>{});
  const down=lines.filter(line=>line.includes("g2")&&(language==="en"?line.includes("is not reachable"):line.includes("nicht erreichbar"))),back=lines.filter(line=>language==="en"?line.includes("reachable again"):line.includes("wieder erreichbar"));
  expect(down,language).toHaveLength(1);expect(down[0]).toContain(peer);expect(down[0]).toContain("server-plan --group g1 --part relay-peers");
  expect(back,language).toHaveLength(1);
 }
});

test("fleet shows the readiness table from the ready.json of every worker, with the stored reason in the language asked, names workers without a status, leaves out the group server and does not change the exit code",async()=>{
 const store=temp("ready-store"),server=machine(store,"g1-server"),origins=join(store,"v1","origins");
 const doc={version:1,origin:"g1-m02",host:"h",platform:"linux",pid:1,state:"backoff",digest:"ab".repeat(32),ready:false,stage:"tools",stageIndex:2,stageCount:5,attempts:3,exit:7,refusal:{code:"stage-failed",en:"Stage tools failed with exit status 7; retrying with back-off.",de:"Stufe tools schlug mit Exit-Status 7 fehl; neuer Versuch mit Wartezeit."},reason:{code:"previous-failed",en:"Retry after back-off.",de:"Neuer Versuch nach der Wartezeit."},peakMegabytes:2048,tail:["older line","Graph scope 155/600"," "],nextAttemptMs:null,readyMs:null,startedMs:1,updatedMs:Date.now()};
 for(const origin of["g1-m02","g1-m03","g1-m04","g1-m05","g1-m06","g1-m07","g1-server"])mkdirSync(join(origins,origin),{recursive:true});
 writeFileSync(join(origins,"g1-m02","ready.json"),JSON.stringify(doc));
 writeFileSync(join(origins,"g1-m04","ready.json"),JSON.stringify({...doc,origin:"g1-m04",state:"ready",ready:true,stage:null,stageIndex:0,attempts:0,exit:null,refusal:null,peakMegabytes:null}));
 writeFileSync(join(origins,"g1-m03","ready.json"),"{broken");
 const ready={...doc,state:"ready",ready:true,stage:null,stageIndex:0,attempts:0,exit:null,reason:null,refusal:null,peakMegabytes:null,tail:[]};
 writeFileSync(join(origins,"g1-m05","ready.json"),JSON.stringify({...ready,origin:"g1-m05",updatedMs:doc.updatedMs-3_600_000}));
 writeFileSync(join(origins,"g1-m06","ready.json"),JSON.stringify({...doc,origin:"g1-m06",state:"running",stage:"workspace",stageIndex:5,refusal:null,updatedMs:doc.updatedMs-3_600_000}));
 writeFileSync(join(origins,"g1-m07","ready.json"),JSON.stringify({...ready,origin:"g1-m07",updatedMs:doc.updatedMs+120_000}));
 writeFileSync(join(origins,"g1-server","ready.json"),JSON.stringify({...ready,origin:"g1-server"}));
 const english=await run("fleet",store,server,"en"),german=await run("fleet",store,server,"de");
 const row=(lines:string[],origin:string)=>lines.filter(line=>line.startsWith(`  ${origin} `)&&(line.includes("no-status")||line.includes("backoff")||line.includes("ready ")||line.includes("kein-Status")));
 expect(english.lines.some(line=>line.startsWith("Readiness of the workers"))).toBe(true);
 expect(german.lines.some(line=>line.startsWith("Bereitschaft der Worker"))).toBe(true);
 expect(row(english.lines,"g1-m02")[0]).toMatch(/^ {2}g1-m02 backoff no 2\/5:tools 3 7 2048 \d+s abababab Stage tools failed with exit status 7/u);
 expect(row(german.lines,"g1-m02")[0]).toMatch(/^ {2}g1-m02 backoff nein 2\/5:tools 3 7 2048 \d+s abababab Stufe tools schlug mit Exit-Status 7 fehl/u);
 expect(row(english.lines,"g1-m03")[0]).toContain("no-status");expect(row(german.lines,"g1-m03")[0]).toContain("kein-Status");
 expect(english.lines.some(line=>line.startsWith("  g1-m04 ready yes"))).toBe(true);
 expect(english.lines.some(line=>line.startsWith("  g1-server "))).toBe(false);
 expect(row(english.lines,"g1-m02")[0]).toContain("Retry after back-off. Graph scope 155/600");
 expect(row(german.lines,"g1-m02")[0]).toContain("Neuer Versuch nach der Wartezeit. Graph scope 155/600");
 expect(english.lines.some(line=>line.startsWith("  g1-m05 stale; last reported ready yes"))).toBe(true);
 expect(english.lines.some(line=>line.startsWith("  g1-m06 stale; last reported running no 5/5:workspace"))).toBe(true);
 expect(german.lines.some(line=>line.startsWith("  g1-m06 veraltet; zuletzt gemeldet running nein 5/5:workspace"))).toBe(true);
 expect(english.lines.some(line=>line.startsWith("  g1-m07 future timestamp; last reported ready yes"))).toBe(true);
 const json=await run("fleet",store,server,"en",["--json"]);
 expect([english.code,german.code]).toEqual([json.code,json.code]);
 expect(english.lines.find(line=>line.startsWith("Ready now: "))).toBe("Ready now: 1 of 6 workers, 2 stale, 1 with a future timestamp, 1 without a valid status.");
 expect(german.lines.find(line=>line.startsWith("Jetzt bereit: "))).toBe("Jetzt bereit: 1 von 6 Workern, 2 veraltet, 1 mit künftigem Zeitstempel, 1 ohne gültigen Status.");
});

test("the fleet uses only fresh, origin-bound Ready receipts to report current readiness",async()=>{
 const store=temp("fixture-store"),origins=join(store,"v1","origins");
 for(const item of readinessFixture.cases){
  mkdirSync(join(origins,item.origin),{recursive:true});
  if(item.patch!==null)writeFileSync(join(origins,item.origin,"ready.json"),JSON.stringify({...readinessFixture.receipt,origin:item.origin,...item.patch}));
 }
 const rows=await readinessReportV1(store,readinessFixture.nowMs);
 expect(rows.map(row=>[row.originId,row.freshness,row.currentReady,row.lastReport?.state??null,row.lastReport?.stage??null]))
  .toEqual(readinessFixture.cases.map(item=>[item.origin,...item.expected]));
});
