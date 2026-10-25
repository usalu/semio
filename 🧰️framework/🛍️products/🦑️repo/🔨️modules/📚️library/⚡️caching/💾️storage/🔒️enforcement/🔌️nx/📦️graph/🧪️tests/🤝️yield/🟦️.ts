/** 🤝️ The yield protocol between an eviction and the daemon that holds a namespace, over real leases: an idle daemon hands its namespace over, every other holder keeps it; the outcomes equal an independent Python model (`./🐍️.py`). */
import {expect,test} from "bun:test";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import {testPythonV1} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {acquireResourceLease,type ResourceLease} from "../../../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";
import {nxDaemonYieldRequestedV1,nxDaemonYieldV1} from "../../../../../../🕸️graph/🏁️startup/🚏️admission/🟦️.ts";
import {nxGraphCapacityWaitMillisecondsV1,acquireYieldedNxGraphNamespaceV1,nxGraphNamespaceResourceV1,nxGraphYieldResourceV1} from "../../🟦️.ts";

const fixturePath=resolve(import.meta.dir,"./🔣️.json"),fixture=JSON.parse(readFileSync(fixturePath,"utf8")) as {waitMilliseconds:number;capacityWait:{name:string;environment:Record<string,string>;expected:number}[];scenarios:{name:string;daemon:null|"idle"|"active";loader:boolean;peerEviction:boolean;expected:boolean}[]};
const model=(()=>{const run=spawnSync(testPythonV1()??"python3",["-I",resolve(import.meta.dir,"./🐍️.py"),fixturePath],{encoding:"utf8"});expect(run.status).toBe(0);return JSON.parse(run.stdout) as {outcomes:boolean[];capacityWait:number[]};})();
const artifacts=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR||tmpdir()),signal=new AbortController().signal;

fixture.scenarios.forEach((row,index)=>test("yield: "+row.name,async()=>{
 const root=mkdtempSync(join(artifacts,"nx-yield-")),leases=join(root,"lease"),namespace=join(root,"graphs","a","b"),held:ResourceLease[]=[];let poll:ReturnType<typeof setInterval>|undefined,daemonLease:ResourceLease|undefined;
 try{
  if(row.loader)held.push(await acquireResourceLease({directory:leases,resource:nxGraphNamespaceResourceV1(namespace),mode:"shared",signal}));
  if(row.peerEviction)held.push(await acquireResourceLease({directory:leases,resource:nxGraphYieldResourceV1(namespace),mode:"exclusive",signal}));
  if(row.daemon!==null){
   daemonLease=await acquireResourceLease({directory:leases,resource:nxGraphNamespaceResourceV1(namespace),mode:"shared",signal});
   poll=setInterval(()=>{void nxDaemonYieldRequestedV1(leases,namespace,signal).then(requested=>{if(daemonLease&&nxDaemonYieldV1({kind:"finite",openRequests:row.daemon==="active"?1:0,requested})){daemonLease.release();daemonLease=undefined;}});},10);
  }
  const lease=await acquireYieldedNxGraphNamespaceV1(leases,namespace,fixture.waitMilliseconds,{signal});
  expect(lease!==undefined).toBe(row.expected);expect(lease!==undefined).toBe(model.outcomes[index]);lease?.release();
 }finally{if(poll)clearInterval(poll);daemonLease?.release();for(const lease of held)lease.release();rmSync(root,{recursive:true,force:true});}
}));

fixture.capacityWait.forEach((row,index)=>test("capacity wait: "+row.name,()=>{const found=nxGraphCapacityWaitMillisecondsV1(row.environment);expect(found).toBe(row.expected);expect(found).toBe(model.capacityWait[index]);}));
