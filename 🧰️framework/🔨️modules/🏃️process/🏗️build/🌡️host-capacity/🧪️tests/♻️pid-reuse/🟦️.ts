import{afterAll,expect,test}from"bun:test";
import{existsSync,mkdirSync,mkdtempSync,rmSync,utimesSync,writeFileSync}from"node:fs";
import{spawnSync}from"node:child_process";
import{tmpdir}from"node:os";
import{join}from"node:path";
import{readHostCapacityPolicyV1}from"../../🟦️.ts";
import{hostBootAtV1,hostGrantPidReusedV1,prepare,processStartMillisecondsV1,readGrants}from"../../🎫️gate/📖️status/🟦️.ts";
import policyFixture from"../🧫️fixtures/🔣️.json";

/** ♻️ Contract: a grant of a dead process is reclaimed by the next ledger read, and so is the grant whose pid belongs to another process now (the heartbeat stopped for two beats and the process at that pid started after the grant was admitted), long before the six hours a heartbeat may be missed; a live holder that only blocks its event loop is never reclaimed. */
const scratch:string[]=[],policy=readHostCapacityPolicyV1(policyFixture.policy);
afterAll(()=>{for(const path of scratch)rmSync(path,{recursive:true,force:true});});
const beat=policy.wait.heartbeatMilliseconds;

function ledger(){
 const directory=mkdtempSync(join(tmpdir(),"gate-reuse-"));scratch.push(directory);prepare(directory);
 const now=Date.now(),boot=hostBootAtV1(directory,now);
 const add=(id:string,pid:number,admittedAtMs:number,heartbeatAgeMs:number)=>{
  const path=join(directory,"grants",`${id}.json`);
  writeFileSync(path,JSON.stringify({id,operation:"exec",jobs:1,cpu:1,reservedMegabytes:512,admittedAtMs,observedMegabytes:null,pid,bootAtMs:boot}));
  const at=(now-heartbeatAgeMs)/1000;utimesSync(path,at,at);return path;
 };
 return{directory,now,add};
}

test("the verdict: stale heartbeat and a process that started after the admission is another process, a fresh heartbeat or an older process is not",()=>{
 const grant={admittedAtMs:1_000_000};
 expect(hostGrantPidReusedV1({grant,heartbeatAgeMs:2*beat+1,heartbeatMs:beat,startMs:1_100_000})).toBe(true);
 expect(hostGrantPidReusedV1({grant,heartbeatAgeMs:2*beat,heartbeatMs:beat,startMs:1_100_000})).toBe(false);
 expect(hostGrantPidReusedV1({grant,heartbeatAgeMs:10*beat,heartbeatMs:beat,startMs:900_000})).toBe(false);
 expect(hostGrantPidReusedV1({grant,heartbeatAgeMs:10*beat,heartbeatMs:beat,startMs:1_004_000})).toBe(false);
 expect(hostGrantPidReusedV1({grant,heartbeatAgeMs:10*beat,heartbeatMs:beat,startMs:undefined})).toBe(false);
});

test("a ledger read removes the grant of a dead process, the grant of a reused pid and keeps the live ones, however old their heartbeat",()=>{
 const world=ledger(),dead=spawnSync(process.execPath,["--eval","0"]),own=process.pid,ownStart=world.now-Math.round(process.uptime()*1000);
 expect(dead.status).toBe(0);
 const paths={
  dead:world.add("deadgrant",dead.pid!,world.now-60_000,1000),
  fresh:world.add("freshgrant",own,ownStart+1000,1000),
  blocked:world.add("blockedgrant",own,ownStart+1000,10*beat),
  reused:world.add("reusedgrant",own,ownStart-3_600_000,10*beat),
 };
 const rows=readGrants(world.directory,world.now,policy,pid=>pid===own?ownStart:undefined).map(row=>row.id).sort();
 expect(rows).toEqual(["blockedgrant","freshgrant"]);
 expect(existsSync(paths.dead)).toBe(false);expect(existsSync(paths.reused)).toBe(false);expect(existsSync(paths.blocked)).toBe(true);expect(existsSync(paths.fresh)).toBe(true);
});

test("the start of a real process is told to within two seconds, and an unknown pid has none",()=>{
 const started=processStartMillisecondsV1(process.pid);
 if(process.platform==="win32")expect(started).toBeUndefined();
 else expect(Math.abs(started!-(Date.now()-process.uptime()*1000))).toBeLessThan(2500);
 const dead=spawnSync(process.execPath,["--eval","0"]);
 expect(dead.status).toBe(0);expect(processStartMillisecondsV1(dead.pid!)).toBeUndefined();
 expect(processStartMillisecondsV1(0)).toBeUndefined();expect(processStartMillisecondsV1(process.pid,"win32")).toBeUndefined();
});
