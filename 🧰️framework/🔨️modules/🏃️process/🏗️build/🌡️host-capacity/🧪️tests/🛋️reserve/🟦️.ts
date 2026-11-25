/** 🛋️ Contract: host admission keeps a fixed reserve of the machine for the interactive apps (it counts as taken by what is not granted, however little they hold) and caps what one agent session may hold reserved while a second agent session holds or waits; verdicts equal an independent Python oracle.
 * @see ./🐍️.py
 * @see ./🧫️fixtures/🔣️.json */
import {expect,test} from "bun:test";
import {fileURLToPath} from "node:url";
import {testPythonV1} from "../../../../📦️packages/🟦️typescript/🧪️testing/🧰️toolchain/🟦️.ts";
import {evaluateHostCapacityV1,hostInteractiveReserveMegabytesV1,hostSessionCapMegabytesV1,readHostCapacityPolicyV1,type HostGrantV1,type HostWaiterV1} from "../../🟦️.ts";
import buildPolicy from "../../../../../../🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json" with {type:"json"};
import base from "../🧫️fixtures/🔣️.json";
import fixture from "./🧫️fixtures/🔣️.json";

const python=testPythonV1()??Bun.which("python3")??"python3",oracle=fileURLToPath(new URL("./🐍️.py",import.meta.url));
const expected=(()=>{const done=Bun.spawnSync([python,"-I",oracle]);if(done.exitCode!==0)throw Error(new TextDecoder().decode(done.stderr));return JSON.parse(new TextDecoder().decode(done.stdout)) as {reserve:number;sessionCap:number|null;decisions:{verdict:string;reason:string}[]}[];})();
const policyOf=(withPolicy:boolean)=>readHostCapacityPolicyV1({...base.policy,memory:{...base.policy.memory,...withPolicy?fixture.memory:{}}});

test("every verdict, reserve and session cap equals the independent Python oracle",()=>{
 fixture.cases.forEach((row,index)=>{
  const policy=policyOf(row.withPolicy),host={...fixture.host,totalMegabytes:row.totalMegabytes,availableMegabytes:(row as {availableMegabytes?:number}).availableMegabytes??fixture.host.availableMegabytes} as never;
  const ledger=row.ledger.map((grant,at):HostGrantV1=>({id:`ledger${at}xxxx`,operation:"run",jobs:1,cpu:1,reservedMegabytes:grant.reservedMegabytes,admittedAtMs:0,observedMegabytes:grant.reservedMegabytes,pid:1,bootAtMs:0,...grant.session===null?{}:{session:grant.session}}));
  const queue=row.queue.map((entry,at):HostWaiterV1=>({request:{id:`request${at}xxxx`,operation:"run",priority:"batch",minimumJobs:1,preferredJobs:1,fixedMegabytes:entry.fixedMegabytes,perJobMegabytes:0,cpuPerJob:1},arrivedAtMs:1000000-1000+at,pid:1,...entry.session===null?{}:{session:entry.session}}));
  const evaluation=evaluateHostCapacityV1(policy,host,ledger,queue,1000000);
  expect(hostInteractiveReserveMegabytesV1(policy,host),row.name).toBe(expected[index]!.reserve);
  expect(hostSessionCapMegabytesV1(policy,host),row.name).toBe(expected[index]!.sessionCap??undefined);
  expect(queue.map(waiter=>evaluation.decisions.find(decision=>decision.id===waiter.request.id)!).map(decision=>({verdict:decision.verdict==="admit"?"admit":"wait",reason:decision.reason})),row.name).toEqual(expected[index]!.decisions);
 });
});

test("the shipped policy keeps 4096 MB for the interactive apps of a 16 GiB host and caps a session at 40 percent",()=>{
 const policy=readHostCapacityPolicyV1((buildPolicy as unknown as {hostCapacity:unknown}).hostCapacity),host={totalMegabytes:16384};
 expect(hostInteractiveReserveMegabytesV1(policy,host)).toBe(4096);
 expect(hostInteractiveReserveMegabytesV1(policy,{totalMegabytes:131072})).toBe(8192);
 expect(hostInteractiveReserveMegabytesV1(policy,{totalMegabytes:4096})).toBe(2048);
 expect(hostSessionCapMegabytesV1(policy,host)).toBe(6553);
});
