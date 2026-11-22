import {expect,test} from "bun:test";
import {chmodSync,existsSync,mkdirSync,mkdtempSync,readFileSync,rmSync,writeFileSync} from "node:fs";
import {join} from "node:path";
import {tmpdir} from "node:os";
import {READY_POLICY_V1 as policy,READY_STAGES_V1 as stages,runReadyStageV1} from "../../../🛰️sidecar/🚦️ready/🟦️.ts";
import {READY_STAGE_CLAIM_POLICY_V1 as P,type ReadyStageClaimV1} from "../../../🌡️load/🪑️stage-claim/🟦️.ts";

const posix=test.skipIf(process.platform==="win32");
const pause=(ms:number)=>new Promise<void>(done=>setTimeout(done,ms));

posix("a running stage of a fleet worker publishes its claim and removes it when it ends; a machine outside a fleet publishes nothing",async()=>{
 const base=mkdtempSync(join(process.env.SEMIO_TEST_ARTIFACT_DIR??tmpdir(),"c-")),root=join(base,"r"),home=join(base,"h"),capacity=join(base,"hc"),repo=join(home,".local","bin","repo"),file=join(capacity,P.fileName);
 try{
  mkdirSync(root);mkdirSync(capacity);mkdirSync(join(home,".local","bin"),{recursive:true});
  writeFileSync(repo,"#!/bin/sh\nsleep 3\nexit 0\n");chmodSync(repo,0o755);
  const launch=(environment:Record<string,string>)=>runReadyStageV1({root,home,environment,stage:{...stages.find(row=>row.name==="tools")!,timeoutMilliseconds:20000},signal:new AbortController().signal,jobs:1,onLine:()=>undefined});
  const outside=launch({SEMIO_HOST_CAPACITY_DIRECTORY:capacity});
  await pause(1500);expect(existsSync(file)).toBe(false);await outside;
  const fleet=launch({SEMIO_HOST_CAPACITY_DIRECTORY:capacity,SEMIO_FLEET_STORE_ROOT:join(base,"store")});
  const stop=performance.now()+2000;
  while(!existsSync(file)&&performance.now()<stop)await pause(50);
  expect(existsSync(file)).toBe(true);
  const claim=JSON.parse(readFileSync(file,"utf8")) as ReadyStageClaimV1;
  expect(claim.stage).toBe("tools");expect(claim.entitlementMegabytes).toBeGreaterThanOrEqual(P.minimumMegabytes);expect(claim.observedMegabytes).toBeGreaterThanOrEqual(0);
  await fleet;
  expect(existsSync(file)).toBe(false);
 }finally{rmSync(base,{recursive:true,force:true});}
},60000);
