/** 🚏️ Nx daemon admission laws over `./🧫️fixtures/🔣️.json`: limits, admission decisions and the idle rule equal an independent Python model (`./🐍️oracle.py`), the Decision schema is validated by Ajv, and with the real lease library daemon slots enforce the cap (a free slot is taken, a full family refuses, a released slot is reusable, a live namespace daemon is detected). */
import {expect,test} from "bun:test";
import {testPythonV1} from "../../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {spawnSync} from "node:child_process";
import {mkdtempSync,readFileSync,rmSync} from "node:fs";
import {tmpdir} from "node:os";
import {join,resolve} from "node:path";
import Ajv from "ajv";
import {acquireNxDaemonSlotV1,countNxDaemonSlotsV1,decideNxDaemonV1,nxDaemonIdleV1,nxDaemonLimitsV1,nxDaemonResourceHeldV1,nxDaemonSlotResourceV1} from "../🟦️.ts";
import {acquireResourceLease} from "../../../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🔒️leases/🟦️.ts";

const fixturePath=resolve(import.meta.dir,"./🧫️fixtures/🔣️.json"),fixture=JSON.parse(readFileSync(fixturePath,"utf8")) as any;
const schema=JSON.parse(readFileSync(resolve(import.meta.dir,"../🧬️schema/🔣️.json"),"utf8")),validate=new Ajv({strict:false}).compile({...schema.definitions.Decision});
const artifacts=resolve(process.env.SEMIO_TEST_ARTIFACT_DIR||tmpdir());
const model=(()=>{const run=spawnSync(testPythonV1()??"python3",["-I",resolve(import.meta.dir,"./🐍️oracle.py"),fixturePath],{encoding:"utf8"});expect(run.status).toBe(0);return JSON.parse(run.stdout);})();
const signal=new AbortController().signal;

fixture.limits.forEach((row:any,index:number)=>test("limits: "+row.name,()=>{
 const found=nxDaemonLimitsV1(row.environment,row.memoryGiB*fixture.GiB);
 expect({finite:found.finite,watch:found.watch,idleMilliseconds:found.idleMilliseconds}).toEqual(row.expected);expect({finite:found.finite,watch:found.watch,idleMilliseconds:found.idleMilliseconds}).toEqual(model.limits[index]);
}));
fixture.decisions.forEach((row:any,index:number)=>test("decision: "+row.name,()=>{
 const found=decideNxDaemonV1(row.input);expect(found).toEqual(row.expected);expect(found).toEqual(model.decisions[index]);expect(validate(found)).toBe(true);
}));
fixture.idle.forEach((row:any,index:number)=>test("idle: "+row.name,()=>{expect(nxDaemonIdleV1(row.state)).toBe(row.expected);expect(row.expected).toBe(model.idle[index]);}));

test("slots enforce the cap with the real lease library",async()=>{
 const root=mkdtempSync(join(artifacts,"nx-daemon-slots-")),leases=join(root,"lease"),image=join(root,"checkout");
 try{
  expect(await countNxDaemonSlotsV1(leases,image,"finite",signal)).toBe(0);
  const first=await acquireNxDaemonSlotV1(leases,image,"finite",2,signal),second=await acquireNxDaemonSlotV1(leases,image,"finite",2,signal);
  expect(await countNxDaemonSlotsV1(leases,image,"finite",signal)).toBe(2);expect(await countNxDaemonSlotsV1(leases,image,"watch",signal)).toBe(0);
  await expect(acquireNxDaemonSlotV1(leases,image,"finite",2,signal)).rejects.toThrow("slots exhausted");
  first.release();expect(await countNxDaemonSlotsV1(leases,image,"finite",signal)).toBe(1);
  const again=await acquireNxDaemonSlotV1(leases,image,"finite",2,signal);again.release();second.release();
  expect(await countNxDaemonSlotsV1(leases,image,"finite",signal)).toBe(0);
 }finally{rmSync(root,{recursive:true,force:true});}
});

test("a held namespace resource is seen as a live daemon and a free one is not",async()=>{
 const root=mkdtempSync(join(artifacts,"nx-daemon-held-")),leases=join(root,"lease"),resource=nxDaemonSlotResourceV1(root,"finite",0);
 try{
  expect(await nxDaemonResourceHeldV1(leases,resource,signal)).toBe(false);
  const lease=await acquireResourceLease({directory:leases,resource,mode:"exclusive",signal});
  expect(await nxDaemonResourceHeldV1(leases,resource,signal)).toBe(true);lease.release();
  expect(await nxDaemonResourceHeldV1(leases,resource,signal)).toBe(false);
 }finally{rmSync(root,{recursive:true,force:true});}
});

const service=readFileSync(resolve(import.meta.dir,"../../🚦️service/🟦️.ts"),"utf8"),script=readFileSync(resolve(import.meta.dir,"../../../../🚀️bootstrap/📜️script.ts"),"utf8");
test("source: the daemon takes its slot before its service lifetime and gives it back last",()=>{
 const start=service.indexOf("acquireNxDaemonSlotV1(repositoryLeaseDirectoryV1(birth.repositoryRoot)");
 expect(start).toBeGreaterThan(-1);expect(start).toBeLessThan(service.indexOf("let release!:()=>void;"));
 expect(service).toContain("finally{release();slot.release();}");
});
test("source: finite daemons idle out through the pure rule and the timer is cleared with the service",()=>{
 expect(service).toContain("nxDaemonIdleV1({kind,openRequests:activity.open,lastActivity:activity.last");
 expect(service).toContain("idleTimer.unref()");expect(service).toContain("finally{clearInterval(idleTimer);");
 expect(service).toContain("activity.open++;activity.last=Date.now();");expect(service).toContain("activity.open--;activity.last=Date.now();");
});
test("source: a run asks for admission once, after its namespace is bound and never for watch or explicit settings",()=>{
 const bind=script.indexOf("(await bind(toolingIdentity,env.SEMIO_BUILD_GRAPH_DIGEST??null,implementationIdentity)).environment);"),admission=script.indexOf("nxDaemonAdmissionV1(this.root,env,actualPhase.signal)");
 expect(bind).toBeGreaterThan(-1);expect(admission).toBeGreaterThan(bind);
 expect(script).toContain('if(env.NX_DAEMON===undefined&&!invocation.watch&&invocation.args[0]!=="watch")');
});
