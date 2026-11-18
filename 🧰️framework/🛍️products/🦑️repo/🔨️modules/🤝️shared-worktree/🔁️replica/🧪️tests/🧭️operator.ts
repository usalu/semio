import {afterAll,expect,test} from "bun:test";
import {mkdirSync,mkdtempSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {runReplicaCommandV1} from "../⌨️command/🟦️.ts";

const scratch:string[]=[];
const temp=(label:string)=>{const path=mkdtempSync(join(tmpdir(),`operator-${label}-`));scratch.push(path);return path;};
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});

const machine=(store:string,origin:string):string=>{const worktree=temp(origin);mkdirSync(join(worktree,".cache","f"),{recursive:true});writeFileSync(join(worktree,".cache","f","machine.json"),JSON.stringify({version:1,originId:origin,groupId:"g1",storeRoot:store,key:"0123456789abcdef0123456789abcdef"}));return worktree;};
async function run(command:string,store:string,worktree:string,language:"en"|"de"):Promise<Readonly<{lines:string[];code:number}>>{
 const lines:string[]=[],result=await runReplicaCommandV1([command,"--worktree",worktree,"--store",store,"--lang",language],{root:worktree,environment:{},signal:new AbortController().signal,write:()=>{},notice:line=>{lines.push(line);}});
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
 const doc={version:1,origin:"g1-m02",host:"h",platform:"linux",pid:1,state:"backoff",digest:"ab".repeat(32),ready:false,stage:"tools",stageIndex:2,stageCount:5,attempts:3,exit:7,refusal:{code:"stage-failed",en:"Stage tools failed with exit status 7; retrying with back-off.",de:"Stufe tools schlug mit Exit-Status 7 fehl; neuer Versuch mit Wartezeit."},peakMegabytes:2048,tail:[],nextAttemptMs:null,readyMs:null,startedMs:1,updatedMs:Date.now()};
 for(const origin of["g1-m02","g1-m03","g1-m04"])mkdirSync(join(origins,origin),{recursive:true});
 writeFileSync(join(origins,"g1-m02","ready.json"),JSON.stringify(doc));
 writeFileSync(join(origins,"g1-m04","ready.json"),JSON.stringify({...doc,origin:"g1-m04",state:"ready",ready:true,stage:null,stageIndex:0,attempts:0,exit:null,refusal:null,peakMegabytes:null}));
 writeFileSync(join(origins,"g1-m03","ready.json"),"{broken");
 const english=await run("fleet",store,server,"en"),german=await run("fleet",store,server,"de");
 const row=(lines:string[],origin:string)=>lines.filter(line=>line.startsWith(`  ${origin} `)&&(line.includes("no-status")||line.includes("backoff")||line.includes("ready ")||line.includes("kein-Status")));
 expect(english.lines.some(line=>line.startsWith("Readiness of the workers"))).toBe(true);
 expect(german.lines.some(line=>line.startsWith("Bereitschaft der Worker"))).toBe(true);
 expect(row(english.lines,"g1-m02")[0]).toMatch(/^ {2}g1-m02 backoff no 2\/5:tools 3 7 2048 \d+s abababab Stage tools failed with exit status 7/u);
 expect(row(german.lines,"g1-m02")[0]).toMatch(/^ {2}g1-m02 backoff nein 2\/5:tools 3 7 2048 \d+s abababab Stufe tools schlug mit Exit-Status 7 fehl/u);
 expect(row(english.lines,"g1-m03")[0]).toContain("no-status");expect(row(german.lines,"g1-m03")[0]).toContain("kein-Status");
 expect(english.lines.some(line=>line.startsWith("  g1-m04 ready yes"))).toBe(true);
 expect(english.lines.some(line=>line.startsWith("  g1-server "))).toBe(false);
 expect(english.lines.find(line=>line.startsWith("Ready: "))).toBe("Ready: 1 of 3 workers, 1 without a readiness status.");
 expect(german.lines.find(line=>line.startsWith("Bereit: "))).toBe("Bereit: 1 von 3 Workern, 1 ohne Bereitschaftsstatus.");
});
