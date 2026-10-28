/** 🕰️ Contract: the boot instant a grant carries does not move when the wall clock is stepped. A grant written before a step and read after it is still a grant of this boot; a grant of an earlier boot (the uptime has run backwards since the anchor was taken) is not. The instant is anchored once per boot in the gate directory, so every process of the boot agrees on it; the old formula `now - uptime` follows the wall clock and judges every running grant foreign after a step, which removed them from the ledger. The oracle is the old formula's drift and a second process reading the anchor. */
import {expect,test} from "bun:test";
import {mkdtempSync,readFileSync,realpathSync,rmSync,writeFileSync} from "node:fs";
import {tmpdir} from "node:os";
import {join} from "node:path";
import {fileURLToPath} from "node:url";
import {hostBootAnchorV1,isCurrentHostGrantV1} from "../../📏️decision/🟦️.ts";
import {hostBootAtV1} from "../../🎫️gate/🟦️.ts";
import schema from "../../🧬️schema/🔣️.json";

const protocol=schema.definitions.Protocol.const,HOUR=3600000;
const wallFormula=(wall:number,uptime:number)=>Math.round((wall-uptime)/1000)*1000;

test("a stepped wall clock leaves the anchor of the boot where it was, an uptime that ran backwards makes a new one",()=>{
 const first=hostBootAnchorV1(undefined,1_700_000_000_000,600_000);
 expect(first).toEqual({bootAtMs:wallFormula(1_700_000_000_000,600_000),uptimeMs:600_000});
 for(const step of[3*HOUR,-2*HOUR,48*HOUR,-90*24*HOUR]){
  const wall=1_700_000_000_000+step+1_000_000,uptime=1_600_000;
  expect(wallFormula(wall,uptime),"the old formula follows the step").not.toBe(first.bootAtMs);
  expect(hostBootAnchorV1(first,wall,uptime)).toBe(first);
 }
 const rebooted=hostBootAnchorV1(first,1_700_000_600_000,30_000);
 expect(rebooted).toEqual({bootAtMs:wallFormula(1_700_000_600_000,30_000),uptimeMs:30_000});
});

test("the grant of an earlier boot is not current and the grant of this boot is, across a step",()=>{
 const policy={wait:{staleMilliseconds:60_000}} as never,anchor=hostBootAnchorV1(undefined,1_700_000_000_000,600_000);
 const facts=(grantBoot:number,boot:number)=>({grant:{id:"a".repeat(22),operation:"run",jobs:1,cpu:1,reservedMegabytes:1,admittedAtMs:1,observedMegabytes:null,pid:1,bootAtMs:grantBoot},expectedId:"a".repeat(22),pidAlive:true,nowMs:1000,bootAtMs:boot,modifiedAtMs:900});
 const after=hostBootAnchorV1(anchor,1_700_000_000_000+5*HOUR,700_000);
 expect(isCurrentHostGrantV1(facts(anchor.bootAtMs,after.bootAtMs),policy)).toBe(true);
 expect(isCurrentHostGrantV1(facts(anchor.bootAtMs,wallFormula(1_700_000_000_000+5*HOUR,700_000)),policy),"what the old formula said after the same step").toBe(false);
 expect(isCurrentHostGrantV1(facts(anchor.bootAtMs,anchor.bootAtMs+10*protocol.bootToleranceMilliseconds),policy)).toBe(false);
});

test("a second process of the same boot reads the anchor of the first, however far the wall clock has been stepped; an anchor from the future is an earlier boot",()=>{
 const directory=realpathSync(mkdtempSync(join(tmpdir(),"boot-anchor-")));
 try{
  const now=Date.now(),first=hostBootAtV1(directory,now);
  expect(Math.abs(first-wallFormula(now,require("node:os").uptime()*1000))).toBeLessThanOrEqual(2000);
  const stored=JSON.parse(readFileSync(join(directory,protocol.bootFile),"utf8")) as {bootAtMs:number;uptimeMs:number};
  expect(stored.bootAtMs).toBe(first);
  const child=Bun.spawnSync([process.execPath,"-e",`const{hostBootAtV1}=await import(${JSON.stringify(fileURLToPath(new URL("../../🎫️gate/🟦️.ts",import.meta.url)))});console.log(hostBootAtV1(${JSON.stringify(directory)},${now+7*HOUR}));`],{stdout:"pipe",stderr:"pipe"});
  expect(child.stderr.toString()).toBe("");expect(Number(child.stdout.toString())).toBe(first);
  writeFileSync(join(directory,protocol.bootFile),JSON.stringify({bootAtMs:first-HOUR,uptimeMs:stored.uptimeMs+10*HOUR}));
  const fresh=Bun.spawnSync([process.execPath,"-e",`const{hostBootAtV1}=await import(${JSON.stringify(fileURLToPath(new URL("../../🎫️gate/🟦️.ts",import.meta.url)))});console.log(hostBootAtV1(${JSON.stringify(directory)},${now}));`],{stdout:"pipe"});
  expect(Math.abs(Number(fresh.stdout.toString())-first)).toBeLessThanOrEqual(2000);
  expect(JSON.parse(readFileSync(join(directory,protocol.bootFile),"utf8")).uptimeMs).toBeLessThan(stored.uptimeMs+HOUR);
 }finally{rmSync(directory,{recursive:true,force:true});}
});
