/** 🔥️ Contract: remote-execution headroom counts CPU (load per core below the schema ceiling) and the coordinator keeps reserved slots for its own interactive work.
 * @see ../../🌡️load/🧬️schema/🔣️.json */
import {spawnSync} from "node:child_process";
import {fileURLToPath} from "node:url";
import {expect,test} from "bun:test";
import {requireTestToolV1} from "../../../../../../../../🔨️modules/🏃️process/🧪️testing/🧰️toolchain/🟦️.ts";
import {buildRemoteExecutionLoadReportV1,evaluateRemoteExecutionAdmissionV1,remoteExecutionLoadSaturatedV1,remoteExecutionLoadTextV1,remoteExecutionReservedSlotsV1,REMOTE_EXECUTION_ADMISSION_POLICY_V1 as A,type RemoteExecutionHostViewV1} from "../../🌡️load/🟦️.ts";
import vectors from "./🔣️.json";

const NOW=1_800_000_000_000;
const view=(load:number|null,cores:number):RemoteExecutionHostViewV1=>({host:{version:1,atMs:NOW,platform:"darwin",cores,totalMegabytes:32768,availableMegabytes:14000,pressure:"normal",swapUsedMegabytes:0,swapOutMegabytesPerSecond:0,load,openFiles:1000,openFilesMaximum:200000,attachedImages:0},grants:[],waiters:[]} as never);
const slots=(origin:string,concurrency:number)=>Math.max(1,concurrency-remoteExecutionReservedSlotsV1(origin));

test("the vectors follow the schema numbers and every case answers as written",()=>{
 expect(vectors.ceilingPermille).toBe(A.loadPerCoreCeilingPermille);
 expect(vectors.reservedSlots).toBe(A.coordinatorReservedSlots);
 for(const row of vectors.saturation)expect(remoteExecutionLoadSaturatedV1({load:row.load,cores:row.cores}),row.name).toBe(row.saturated);
 for(const row of vectors.slots)expect(slots(row.origin,row.concurrency),JSON.stringify(row)).toBe(row.registered);
 expect(remoteExecutionLoadTextV1("en",{load:vectors.texts.load,cores:vectors.texts.cores})).toBe(vectors.texts.en);
 expect(remoteExecutionLoadTextV1("de",{load:vectors.texts.load,cores:vectors.texts.cores})).toBe(vectors.texts.de);
});

test("every case equals the independent Python oracle",()=>{
 const run=spawnSync(requireTestToolV1("python"),["-I",fileURLToPath(new URL("./🐍️.py",import.meta.url)),fileURLToPath(new URL("../../🌡️load/🧬️schema/🔣️.json",import.meta.url)),fileURLToPath(new URL("./🔣️.json",import.meta.url))],{encoding:"utf8"});
 expect(run.status,run.stderr).toBe(0);
 const model=JSON.parse(run.stdout) as {saturation:boolean[];slots:number[]};
 expect(model.saturation).toEqual(vectors.saturation.map(row=>remoteExecutionLoadSaturatedV1({load:row.load,cores:row.cores})));
 expect(model.slots).toEqual(vectors.slots.map(row=>slots(row.origin,row.concurrency)));
});

test("a saturated machine refuses with load, reports no headroom and says why in English and German; a calm one is admitted",()=>{
 const busy=evaluateRemoteExecutionAdmissionV1({view:view(37.01,10),running:0,concurrency:10,nowMs:NOW,megabytes:256});
 expect(busy.admission).toEqual({admit:false,reason:"load"});
 expect(evaluateRemoteExecutionAdmissionV1({view:view(2,10),running:0,concurrency:10,nowMs:NOW,megabytes:256}).admission).toEqual({admit:true});
 const report=(one:RemoteExecutionHostViewV1)=>buildRemoteExecutionLoadReportV1({origin:"g1-server",identity:{key:"macos-arm64-none",os:"macos",arch:"arm64",toolchain:null,target:null},view:one,worker:{state:"idle",concurrency:8,running:0,sandbox:null,confinement:null,network:null},counters:{unit:{localHit:0,remoteHit:0,miss:0,stored:0},offload:{out:0,refused:0,fellBack:0}} as never,nowMs:NOW});
 const refused=report(view(37.01,10));
 expect(refused.headroom).toBe(false);
 expect(refused.worker.state).toBe("refusing");
 expect(refused.worker.why?.en).toBe(vectors.texts.en);
 expect(refused.worker.why?.de).toBe(vectors.texts.de);
 expect(report(view(2,10)).headroom).toBe(true);
});
